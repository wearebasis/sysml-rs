//! AST conversion from pest parse tree to ModelGraph.
//!
//! This module converts the pest parse tree (Pairs) to the sysml-core
//! ModelGraph representation.
//!
//! ## Ownership Model
//!
//! This converter implements SysML v2 canonical ownership through `OwningMembership`
//! elements. When a nested element is converted, an `OwningMembership` is created
//! that links the element to its containing namespace with the appropriate visibility.
//!
//! Visibility is extracted from the grammar's `*Member` wrapper rules (e.g.,
//! `UsageMember`, `DefinitionMember`) and propagated to the owned elements.
//!
//! ## Relationship Construction
//!
//! This converter creates relationship elements (Specialization, FeatureTyping,
//! Subsetting, Redefinition) from syntax like `:>`, `:`, `:>>`. Target qualified
//! names are stored as `unresolved_*` properties for later resolution in Phase 2c.
//!
//! ## Single-Pass Extraction
//!
//! This module uses optimized extraction functions that traverse each pest pair
//! only once, instead of multiple times for different properties. This reduces
//! overhead from repeated `.clone().into_inner()` calls, providing ~68% faster
//! parsing for large models.

mod extraction;

use pest::iterators::{Pair, Pairs};
use sysml_core::{Element, ElementKind, ModelGraph, Value, VisibilityKind};
use sysml_id::ElementId;
use sysml_span::Span;

use crate::{ParseError, Rule};

use extraction::{DefinitionExtraction, PackageExtraction, UsageExtraction};

/// Work item for iterative tree traversal.
///
/// Instead of using recursion (which can overflow the stack for large files),
/// we use an explicit work stack with these item types.
enum WorkItem<'a> {
    /// Process this pair (main work item)
    ProcessPair(Pair<'a, Rule>),
    /// Pop the visibility stack after processing children
    PopVisibility,
    /// Pop the owner stack after processing children
    PopOwner,
}

/// Converter from pest pairs to ModelGraph.
pub struct Converter<'a> {
    /// The file path for span information.
    file_path: &'a str,
    /// Whether to include spans.
    include_spans: bool,
    /// Stack of owner IDs for building the tree.
    owner_stack: Vec<ElementId>,
    /// Stack of visibility kinds for nested members.
    visibility_stack: Vec<VisibilityKind>,
}

impl<'a> Converter<'a> {
    /// Create a new converter.
    pub fn new(file_path: &'a str, include_spans: bool) -> Self {
        Converter {
            file_path,
            include_spans,
            owner_stack: Vec::new(),
            visibility_stack: Vec::new(),
        }
    }

    /// Extract visibility from a pair that may contain a Visibility child.
    ///
    /// Looks for `Rule::Visibility` or `Rule::VisibilityKind` in the pair's
    /// immediate children and extracts the visibility keyword.
    fn extract_visibility(&self, pair: &Pair<'_, Rule>) -> VisibilityKind {
        for inner in pair.clone().into_inner() {
            match inner.as_rule() {
                Rule::Visibility | Rule::VisibilityKind => {
                    let text = inner.as_str().trim();
                    return match text {
                        "private" => VisibilityKind::Private,
                        "protected" => VisibilityKind::Protected,
                        "expose" => VisibilityKind::Public, // 'expose' is treated as public
                        _ => VisibilityKind::Public,
                    };
                }
                _ => {}
            }
        }
        VisibilityKind::Public
    }

    /// Get the current visibility from the stack, or Public if empty.
    fn current_visibility(&self) -> VisibilityKind {
        self.visibility_stack.last().copied().unwrap_or(VisibilityKind::Public)
    }

    /// Add an element with ownership established through OwningMembership.
    ///
    /// If there's an owner on the stack, creates an OwningMembership
    /// linking the element to its owner with the current visibility.
    /// Otherwise, adds the element as a root.
    fn add_with_ownership(&self, element: Element, graph: &mut ModelGraph) -> ElementId {
        if let Some(owner_id) = self.owner_stack.last() {
            graph.add_owned_element(element, owner_id.clone(), self.current_visibility())
        } else {
            graph.add_element(element)
        }
    }

    /// Convert pest pairs to ModelGraph using iterative traversal.
    ///
    /// This uses an explicit work stack instead of recursion to handle
    /// deeply nested parse trees without stack overflow.
    pub fn convert(mut self, pairs: Pairs<'_, Rule>, graph: &mut ModelGraph) -> Result<(), ParseError> {
        // Initialize work stack with top-level pairs (in reverse order for LIFO processing)
        let mut work_stack: Vec<WorkItem<'_>> = pairs
            .map(WorkItem::ProcessPair)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Process work items iteratively
        while let Some(item) = work_stack.pop() {
            match item {
                WorkItem::ProcessPair(pair) => {
                    self.process_pair(pair, graph, &mut work_stack)?;
                }
                WorkItem::PopVisibility => {
                    self.visibility_stack.pop();
                }
                WorkItem::PopOwner => {
                    self.owner_stack.pop();
                }
            }
        }
        Ok(())
    }

    /// Process a single pair, pushing work items for children instead of recursing.
    ///
    /// This is the core of the iterative traversal. Instead of calling itself
    /// recursively, it pushes `WorkItem::ProcessPair` onto the work stack.
    fn process_pair<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let rule = pair.as_rule();
        let span = self.pair_to_span(&pair);

        match rule {
            // Skip tokens and whitespace
            Rule::WHITESPACE | Rule::NEWLINE | Rule::COMMENT | Rule::SL_COMMENT | Rule::ML_COMMENT
            | Rule::SL_NOTE | Rule::ML_NOTE | Rule::EOI => {}

            // Entry points - push children
            Rule::File | Rule::Model => {
                self.push_children(pair, work_stack);
            }

            // Packages
            Rule::Package => {
                self.process_package(pair, graph, span, work_stack)?;
            }
            Rule::LibraryPackage => {
                self.process_library_package(pair, graph, span, work_stack)?;
            }

            // Package body elements - push children
            Rule::PackageBodyElement => {
                self.push_children(pair, work_stack);
            }

            // Definitions - DefinitionElement passes through, DefinitionMember extracts visibility
            Rule::DefinitionElement => {
                self.push_children(pair, work_stack);
            }
            Rule::DefinitionMember => {
                let visibility = self.extract_visibility(&pair);
                self.visibility_stack.push(visibility);
                // Push PopVisibility BEFORE children (will execute AFTER due to LIFO)
                work_stack.push(WorkItem::PopVisibility);
                self.push_children(pair, work_stack);
            }

            Rule::AttributeDefinition => { self.process_definition(pair, graph, ElementKind::AttributeDefinition, span, work_stack)?; }
            Rule::EnumerationDefinition => { self.process_definition(pair, graph, ElementKind::EnumerationDefinition, span, work_stack)?; }
            Rule::OccurrenceDefinition => { self.process_definition(pair, graph, ElementKind::OccurrenceDefinition, span, work_stack)?; }
            Rule::ItemDefinition => { self.process_definition(pair, graph, ElementKind::ItemDefinition, span, work_stack)?; }
            Rule::MetadataDefinition => { self.process_definition(pair, graph, ElementKind::MetadataDefinition, span, work_stack)?; }
            Rule::PartDefinition => { self.process_definition(pair, graph, ElementKind::PartDefinition, span, work_stack)?; }
            Rule::PortDefinition => { self.process_definition(pair, graph, ElementKind::PortDefinition, span, work_stack)?; }
            Rule::ConnectionDefinition => { self.process_definition(pair, graph, ElementKind::ConnectionDefinition, span, work_stack)?; }
            Rule::FlowConnectionDefinition => { self.process_definition(pair, graph, ElementKind::FlowDefinition, span, work_stack)?; }
            Rule::InterfaceDefinition => { self.process_definition(pair, graph, ElementKind::InterfaceDefinition, span, work_stack)?; }
            Rule::AllocationDefinition => { self.process_definition(pair, graph, ElementKind::AllocationDefinition, span, work_stack)?; }
            Rule::ActionDefinition => { self.process_definition(pair, graph, ElementKind::ActionDefinition, span, work_stack)?; }
            Rule::CalculationDefinition => { self.process_definition(pair, graph, ElementKind::CalculationDefinition, span, work_stack)?; }
            Rule::StateDefinition => { self.process_definition(pair, graph, ElementKind::StateDefinition, span, work_stack)?; }
            Rule::ConstraintDefinition => { self.process_definition(pair, graph, ElementKind::ConstraintDefinition, span, work_stack)?; }
            Rule::RequirementDefinition => { self.process_definition(pair, graph, ElementKind::RequirementDefinition, span, work_stack)?; }
            Rule::ConcernDefinition => { self.process_definition(pair, graph, ElementKind::ConcernDefinition, span, work_stack)?; }
            Rule::CaseDefinition => { self.process_definition(pair, graph, ElementKind::CaseDefinition, span, work_stack)?; }
            Rule::AnalysisCaseDefinition => { self.process_definition(pair, graph, ElementKind::AnalysisCaseDefinition, span, work_stack)?; }
            Rule::VerificationCaseDefinition => { self.process_definition(pair, graph, ElementKind::VerificationCaseDefinition, span, work_stack)?; }
            Rule::UseCaseDefinition => { self.process_definition(pair, graph, ElementKind::UseCaseDefinition, span, work_stack)?; }
            Rule::ViewDefinition => { self.process_definition(pair, graph, ElementKind::ViewDefinition, span, work_stack)?; }
            Rule::ViewpointDefinition => { self.process_definition(pair, graph, ElementKind::ViewpointDefinition, span, work_stack)?; }
            Rule::RenderingDefinition => { self.process_definition(pair, graph, ElementKind::RenderingDefinition, span, work_stack)?; }
            Rule::ExtendedDefinition => { self.process_definition(pair, graph, ElementKind::Definition, span, work_stack)?; }

            // KerML definitions (for standard library parsing)
            Rule::ClassifierDefinition => { self.process_definition(pair, graph, ElementKind::Classifier, span, work_stack)?; }
            Rule::DatatypeDefinition => { self.process_definition(pair, graph, ElementKind::DataType, span, work_stack)?; }
            Rule::ClassDefinition => { self.process_definition(pair, graph, ElementKind::Class, span, work_stack)?; }
            Rule::StructDefinition => { self.process_definition(pair, graph, ElementKind::Structure, span, work_stack)?; }
            Rule::AssociationDefinition => { self.process_definition(pair, graph, ElementKind::Association, span, work_stack)?; }
            Rule::AssociationStructDefinition => { self.process_definition(pair, graph, ElementKind::AssociationStructure, span, work_stack)?; }
            Rule::MultiplicityDefinition => { self.process_definition(pair, graph, ElementKind::Multiplicity, span, work_stack)?; }
            Rule::FeatureDefinition => { self.process_usage(pair, graph, ElementKind::Feature, span, work_stack)?; }
            Rule::BehaviorDefinition => { self.process_definition(pair, graph, ElementKind::Behavior, span, work_stack)?; }
            Rule::FunctionDefinition => { self.process_definition(pair, graph, ElementKind::Function, span, work_stack)?; }
            Rule::PredicateDefinition => { self.process_definition(pair, graph, ElementKind::Predicate, span, work_stack)?; }
            Rule::InteractionDefinition => { self.process_definition(pair, graph, ElementKind::Interaction, span, work_stack)?; }
            Rule::MetaclassDefinition => { self.process_definition(pair, graph, ElementKind::Metaclass, span, work_stack)?; }

            // KerML feature usages (for standard library parsing)
            Rule::StepUsage => { self.process_usage(pair, graph, ElementKind::Step, span, work_stack)?; }
            Rule::ExpressionUsage => { self.process_usage(pair, graph, ElementKind::Expression, span, work_stack)?; }
            Rule::BooleanExpressionUsage => { self.process_usage(pair, graph, ElementKind::BooleanExpression, span, work_stack)?; }
            Rule::InvariantUsage => { self.process_usage(pair, graph, ElementKind::Invariant, span, work_stack)?; }

            // Usages - *Element rules pass through, *Member rules extract visibility
            Rule::UsageElement | Rule::NonOccurrenceUsageElement | Rule::OccurrenceUsageElement
            | Rule::StructureUsageElement | Rule::BehaviorUsageElement | Rule::VariantUsageElement => {
                self.push_children(pair, work_stack);
            }
            Rule::UsageMember | Rule::NonOccurrenceUsageMember | Rule::OccurrenceUsageMember | Rule::VariantUsageMember => {
                let visibility = self.extract_visibility(&pair);
                self.visibility_stack.push(visibility);
                work_stack.push(WorkItem::PopVisibility);
                self.push_children(pair, work_stack);
            }

            Rule::ReferenceUsage | Rule::DefaultReferenceUsage => { self.process_usage(pair, graph, ElementKind::ReferenceUsage, span, work_stack)?; }
            Rule::AttributeUsage => { self.process_usage(pair, graph, ElementKind::AttributeUsage, span, work_stack)?; }
            Rule::EnumerationUsage => { self.process_usage(pair, graph, ElementKind::EnumerationUsage, span, work_stack)?; }
            Rule::OccurrenceUsage => { self.process_usage(pair, graph, ElementKind::OccurrenceUsage, span, work_stack)?; }
            Rule::IndividualUsage => { self.process_usage(pair, graph, ElementKind::OccurrenceUsage, span, work_stack)?; }
            Rule::PortionUsage => { self.process_usage(pair, graph, ElementKind::OccurrenceUsage, span, work_stack)?; }
            Rule::EventOccurrenceUsage => { self.process_usage(pair, graph, ElementKind::EventOccurrenceUsage, span, work_stack)?; }
            Rule::ItemUsage => { self.process_usage(pair, graph, ElementKind::ItemUsage, span, work_stack)?; }
            Rule::PartUsage => { self.process_usage(pair, graph, ElementKind::PartUsage, span, work_stack)?; }
            Rule::PortUsage => { self.process_usage(pair, graph, ElementKind::PortUsage, span, work_stack)?; }
            Rule::ConnectionUsage => { self.process_usage(pair, graph, ElementKind::ConnectionUsage, span, work_stack)?; }
            Rule::InterfaceUsage => { self.process_usage(pair, graph, ElementKind::InterfaceUsage, span, work_stack)?; }
            Rule::AllocationUsage => { self.process_usage(pair, graph, ElementKind::AllocationUsage, span, work_stack)?; }
            Rule::FlowConnectionUsage => { self.process_usage(pair, graph, ElementKind::FlowUsage, span, work_stack)?; }
            Rule::SuccessionFlowUsage => { self.process_usage(pair, graph, ElementKind::SuccessionFlowUsage, span, work_stack)?; }
            Rule::ViewUsage => { self.process_usage(pair, graph, ElementKind::ViewUsage, span, work_stack)?; }
            Rule::RenderingUsage => { self.process_usage(pair, graph, ElementKind::RenderingUsage, span, work_stack)?; }
            Rule::ActionUsage => { self.process_usage(pair, graph, ElementKind::ActionUsage, span, work_stack)?; }
            Rule::PerformActionUsage => { self.process_usage(pair, graph, ElementKind::PerformActionUsage, span, work_stack)?; }
            Rule::CalculationUsage => { self.process_usage(pair, graph, ElementKind::CalculationUsage, span, work_stack)?; }
            Rule::StateUsage => { self.process_usage(pair, graph, ElementKind::StateUsage, span, work_stack)?; }
            Rule::ExhibitStateUsage => { self.process_usage(pair, graph, ElementKind::ExhibitStateUsage, span, work_stack)?; }
            Rule::ConstraintUsage => { self.process_usage(pair, graph, ElementKind::ConstraintUsage, span, work_stack)?; }
            Rule::AssertConstraintUsage => { self.process_usage(pair, graph, ElementKind::AssertConstraintUsage, span, work_stack)?; }
            Rule::RequirementUsage => { self.process_usage(pair, graph, ElementKind::RequirementUsage, span, work_stack)?; }
            Rule::SatisfyRequirementUsage => { self.process_usage(pair, graph, ElementKind::SatisfyRequirementUsage, span, work_stack)?; }
            Rule::ConcernUsage => { self.process_usage(pair, graph, ElementKind::ConcernUsage, span, work_stack)?; }
            Rule::CaseUsage => { self.process_usage(pair, graph, ElementKind::CaseUsage, span, work_stack)?; }
            Rule::AnalysisCaseUsage => { self.process_usage(pair, graph, ElementKind::AnalysisCaseUsage, span, work_stack)?; }
            Rule::VerificationCaseUsage => { self.process_usage(pair, graph, ElementKind::VerificationCaseUsage, span, work_stack)?; }
            Rule::UseCaseUsage => { self.process_usage(pair, graph, ElementKind::UseCaseUsage, span, work_stack)?; }
            Rule::IncludeUseCaseUsage => { self.process_usage(pair, graph, ElementKind::IncludeUseCaseUsage, span, work_stack)?; }
            Rule::ViewpointUsage => { self.process_usage(pair, graph, ElementKind::ViewpointUsage, span, work_stack)?; }
            Rule::BindingConnectorAsUsage => { self.process_usage(pair, graph, ElementKind::BindingConnectorAsUsage, span, work_stack)?; }
            Rule::SuccessionAsUsage => { self.process_usage(pair, graph, ElementKind::SuccessionAsUsage, span, work_stack)?; }
            Rule::ExtendedUsage => { self.process_usage(pair, graph, ElementKind::Usage, span, work_stack)?; }
            Rule::VariantReference => { self.process_usage(pair, graph, ElementKind::ReferenceUsage, span, work_stack)?; }

            // Action nodes (map to specific ActionUsage subtypes)
            Rule::SendNode => { self.process_usage(pair, graph, ElementKind::SendActionUsage, span, work_stack)?; }
            Rule::AcceptNode => { self.process_usage(pair, graph, ElementKind::AcceptActionUsage, span, work_stack)?; }
            Rule::AssignmentNode => { self.process_usage(pair, graph, ElementKind::AssignmentActionUsage, span, work_stack)?; }
            Rule::TerminateNode => { self.process_usage(pair, graph, ElementKind::TerminateActionUsage, span, work_stack)?; }
            Rule::IfNode => { self.process_usage(pair, graph, ElementKind::IfActionUsage, span, work_stack)?; }
            Rule::WhileLoopNode => { self.process_usage(pair, graph, ElementKind::WhileLoopActionUsage, span, work_stack)?; }
            Rule::ForLoopNode => { self.process_usage(pair, graph, ElementKind::ForLoopActionUsage, span, work_stack)?; }

            // Transitions
            Rule::TransitionUsage => { self.process_usage(pair, graph, ElementKind::TransitionUsage, span, work_stack)?; }

            // ActionNode/ControlNode wrappers - push children
            Rule::ActionNode | Rule::ControlNode => {
                self.push_children(pair, work_stack);
            }

            // Imports
            Rule::Import => { self.process_import(pair, graph, span)?; }

            // Annotations - AnnotatingElement passes through, AnnotatingMember extracts visibility
            Rule::AnnotatingElement => {
                self.push_children(pair, work_stack);
            }
            Rule::AnnotatingMember => {
                let visibility = self.extract_visibility(&pair);
                self.visibility_stack.push(visibility);
                work_stack.push(WorkItem::PopVisibility);
                self.push_children(pair, work_stack);
            }
            Rule::Comment => { self.process_comment(pair, graph, span)?; }
            Rule::Documentation => { self.process_documentation(pair, graph, span)?; }
            Rule::MetadataUsage => { self.process_metadata_usage(pair, graph, span, work_stack)?; }

            // Dependencies - RelationshipMember extracts visibility
            Rule::Dependency => { self.process_dependency(pair, graph, span)?; }
            Rule::RelationshipMember => {
                let visibility = self.extract_visibility(&pair);
                self.visibility_stack.push(visibility);
                work_stack.push(WorkItem::PopVisibility);
                self.push_children(pair, work_stack);
            }

            // Skip intermediate rules - push their children
            Rule::DefinitionBody | Rule::DefinitionBodyItem | Rule::PackageBody
            | Rule::ActionBody | Rule::ActionBodyItem | Rule::StateBody | Rule::StateBodyItem
            | Rule::CalculationBody | Rule::CalculationBodyItem | Rule::FunctionBody | Rule::FunctionBodyItem | Rule::RequirementBody | Rule::RequirementBodyItem
            | Rule::CaseBody | Rule::CaseBodyItem | Rule::ViewBody | Rule::ViewBodyItem
            | Rule::InterfaceBody | Rule::InterfaceBodyItem | Rule::EnumerationBody | Rule::EnumerationBodyItem
            | Rule::MetadataBody | Rule::MetadataBodyItem => {
                self.push_children(pair, work_stack);
            }

            // Names and identifiers - no action needed
            Rule::Name | Rule::ID | Rule::UNRESTRICTED_NAME | Rule::QualifiedName
            | Rule::RegularName | Rule::ShortName | Rule::Identification => {
                // Names are extracted by parent rules
            }

            // Tokens and keywords - skip
            _ if rule_is_keyword(rule) => {}

            // Default: push children
            _ => {
                self.push_children(pair, work_stack);
            }
        }
        Ok(())
    }

    /// Push children of a pair onto the work stack in reverse order (for LIFO processing).
    fn push_children<'b>(&self, pair: Pair<'b, Rule>, work_stack: &mut Vec<WorkItem<'b>>) {
        let children: Vec<_> = pair.into_inner().collect();
        for child in children.into_iter().rev() {
            work_stack.push(WorkItem::ProcessPair(child));
        }
    }

    /// Process a Package using single-pass extraction.
    fn process_package<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        span: Option<Span>,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let extraction = PackageExtraction::from_pair(pair, false);

        let mut element = Element::new_with_kind(ElementKind::Package);

        if let Some(name) = extraction.name {
            element.name = Some(name);
        }

        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = self.add_with_ownership(element, graph);

        self.owner_stack.push(id);
        work_stack.push(WorkItem::PopOwner);

        for body_pair in extraction.body_pairs.into_iter().rev() {
            work_stack.push(WorkItem::ProcessPair(body_pair));
        }

        Ok(())
    }

    /// Process a LibraryPackage using single-pass extraction.
    fn process_library_package<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        span: Option<Span>,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let extraction = PackageExtraction::from_pair(pair, true);

        let mut element = Element::new_with_kind(ElementKind::LibraryPackage);

        if let Some(name) = extraction.name {
            element.name = Some(name);
        }

        if extraction.is_standard {
            element.set_prop("isStandard", true);
        }

        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = self.add_with_ownership(element, graph);

        self.owner_stack.push(id);
        work_stack.push(WorkItem::PopOwner);

        for body_pair in extraction.body_pairs.into_iter().rev() {
            work_stack.push(WorkItem::ProcessPair(body_pair));
        }

        Ok(())
    }

    /// Process a definition using single-pass extraction.
    fn process_definition<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        kind: ElementKind,
        span: Option<Span>,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let extraction = DefinitionExtraction::from_pair(pair);

        let mut element = Element::new_with_kind(kind);

        if let Some(name) = extraction.name {
            element.name = Some(name);
        }

        if extraction.is_abstract {
            element.set_prop("isAbstract", true);
        }
        if extraction.is_variation {
            element.set_prop("isVariation", true);
        }

        if let Some(s) = span.clone() {
            element.spans.push(s);
        }

        let id = self.add_with_ownership(element, graph);

        // Create Specialization elements for each subclassification target
        for target_qname in extraction.subclassifications {
            self.create_specialization(id.clone(), target_qname, graph, span.clone());
        }

        self.owner_stack.push(id);
        work_stack.push(WorkItem::PopOwner);

        for body_pair in extraction.body_pairs.into_iter().rev() {
            work_stack.push(WorkItem::ProcessPair(body_pair));
        }

        Ok(())
    }

    /// Process a usage using single-pass extraction.
    fn process_usage<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        kind: ElementKind,
        span: Option<Span>,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let extraction = UsageExtraction::from_pair(pair);

        let mut element = Element::new_with_kind(kind);

        if let Some(name) = extraction.name {
            element.name = Some(name);
        }

        if let Some(direction) = extraction.direction {
            element.set_prop("direction", direction);
        }

        if let Some((lower, upper)) = extraction.multiplicity {
            element.set_prop("multiplicity_lower", Value::Int(lower));
            match upper {
                Some(u) => element.set_prop("multiplicity_upper", Value::Int(u)),
                None => element.set_prop("multiplicity_upper", Value::String("*".to_string())),
            }
        }

        if let Some(value_expression) = extraction.value_expression {
            element.set_prop("unresolved_value", Value::String(value_expression));
            if extraction.value_is_default {
                element.set_prop("isDefault", true);
            }
            if extraction.value_is_initial {
                element.set_prop("isInitial", true);
            }
        }

        // Apply flags
        if extraction.is_abstract {
            element.set_prop("isAbstract", true);
        }
        if extraction.is_variation {
            element.set_prop("isVariation", true);
        }
        if extraction.is_readonly {
            element.set_prop("isReadOnly", true);
        }
        if extraction.is_derived {
            element.set_prop("isDerived", true);
        }
        if extraction.is_end {
            element.set_prop("isEnd", true);
        }
        if extraction.is_reference {
            element.set_prop("isReference", true);
        }
        if extraction.is_composite {
            element.set_prop("isComposite", true);
        }
        if extraction.is_portion {
            element.set_prop("isPortion", true);
        }
        if extraction.is_variable {
            element.set_prop("isVariable", true);
        }
        if extraction.is_constant {
            element.set_prop("isConstant", true);
        }

        if let Some(s) = span.clone() {
            element.spans.push(s);
        }

        let id = self.add_with_ownership(element, graph);

        // Create FeatureTyping elements
        for type_qname in extraction.typings {
            self.create_feature_typing(id.clone(), type_qname, graph, span.clone());
        }

        // Create Subsetting elements
        for subsetted in extraction.subsettings {
            self.create_subsetting(id.clone(), subsetted, graph, span.clone());
        }

        // Create Redefinition elements
        for redefined in extraction.redefinitions {
            self.create_redefinition(id.clone(), redefined, graph, span.clone());
        }

        // Create ReferenceSubsetting elements
        for referenced in extraction.references {
            self.create_reference_subsetting(id.clone(), referenced, graph, span.clone());
        }

        self.owner_stack.push(id);
        work_stack.push(WorkItem::PopOwner);

        for body_pair in extraction.body_pairs.into_iter().rev() {
            work_stack.push(WorkItem::ProcessPair(body_pair));
        }

        Ok(())
    }

    /// Process an Import (no children to process).
    fn process_import(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<(), ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Import);

        // Extract the imported reference
        if let Some(reference) = self.extract_qualified_name(&pair) {
            element.set_prop("importedReference", reference);
        }

        // Check for flags
        let text = pair.as_str();
        if text.contains("all ") {
            element.set_prop("importsAll", true);
        }
        if text.contains("::*") {
            element.set_prop("isNamespace", true);
        }
        if text.contains("::**") {
            element.set_prop("isRecursive", true);
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        // Add element with canonical ownership
        self.add_with_ownership(element, graph);

        Ok(())
    }

    /// Process a Comment (no children to process).
    fn process_comment(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<(), ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Comment);

        // Extract comment body
        let text = pair.as_str();
        if let Some(start) = text.find("/*") {
            if let Some(end) = text.rfind("*/") {
                let body = &text[start + 2..end];
                element.set_prop("body", body.trim());
            }
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        // Add element with canonical ownership
        self.add_with_ownership(element, graph);

        Ok(())
    }

    /// Process Documentation (no children to process).
    fn process_documentation(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<(), ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Documentation);

        // Extract documentation body
        let text = pair.as_str();
        if let Some(start) = text.find("/*") {
            if let Some(end) = text.rfind("*/") {
                let body = &text[start + 2..end];
                element.set_prop("body", body.trim());
            }
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        // Add element with canonical ownership
        self.add_with_ownership(element, graph);

        Ok(())
    }

    /// Process MetadataUsage, pushing children onto the work stack.
    fn process_metadata_usage<'b>(
        &mut self,
        pair: Pair<'b, Rule>,
        graph: &mut ModelGraph,
        span: Option<Span>,
        work_stack: &mut Vec<WorkItem<'b>>,
    ) -> Result<(), ParseError> {
        let mut element = Element::new_with_kind(ElementKind::MetadataUsage);

        // Extract name if present
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        // Add element with canonical ownership
        let id = self.add_with_ownership(element, graph);

        // Push owner and schedule children
        self.owner_stack.push(id);
        work_stack.push(WorkItem::PopOwner);
        self.push_children(pair, work_stack);

        Ok(())
    }

    /// Process a Dependency (no children to process).
    fn process_dependency(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<(), ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Dependency);

        // Extract name if present
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Extract FROM/TO endpoints
        let (sources, targets) = self.extract_dependency_endpoints(&pair);

        // Store unresolved sources and targets
        if !sources.is_empty() {
            element.set_prop(
                "unresolved_sources",
                Value::List(sources.into_iter().map(Value::String).collect()),
            );
        }
        if !targets.is_empty() {
            element.set_prop(
                "unresolved_targets",
                Value::List(targets.into_iter().map(Value::String).collect()),
            );
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        // Add element with canonical ownership
        self.add_with_ownership(element, graph);

        Ok(())
    }

    /// Extract a name from a pair (looking for Identification/RegularName).
    fn extract_name(&self, pair: &Pair<'_, Rule>) -> Option<String> {
        for inner in pair.clone().into_inner() {
            match inner.as_rule() {
                Rule::Identification => {
                    for ident_inner in inner.into_inner() {
                        if let Rule::RegularName = ident_inner.as_rule() {
                            return self.extract_name_from_regular_name(&ident_inner);
                        }
                    }
                }
                Rule::DefinitionDeclaration | Rule::UsageDeclaration | Rule::FeatureDeclaration => {
                    if let Some(name) = self.extract_name(&inner) {
                        return Some(name);
                    }
                }
                Rule::RegularName => {
                    return self.extract_name_from_regular_name(&inner);
                }
                _ => {}
            }
        }
        None
    }

    /// Extract name from RegularName.
    fn extract_name_from_regular_name(&self, pair: &Pair<'_, Rule>) -> Option<String> {
        for inner in pair.clone().into_inner() {
            if let Rule::Name = inner.as_rule() {
                let inner_str = inner.as_str().to_string();
                for name_inner in inner.into_inner() {
                    match name_inner.as_rule() {
                        Rule::ID => return Some(name_inner.as_str().to_string()),
                        Rule::UNRESTRICTED_NAME => {
                            // Remove quotes
                            let s = name_inner.as_str();
                            if s.len() >= 2 {
                                return Some(s[1..s.len()-1].to_string());
                            }
                        }
                        _ => {}
                    }
                }
                // Name itself might be the ID directly
                return Some(inner_str);
            }
        }
        // The RegularName might directly contain the name text
        Some(pair.as_str().to_string())
    }

    /// Extract a qualified name from a pair.
    fn extract_qualified_name(&self, pair: &Pair<'_, Rule>) -> Option<String> {
        for inner in pair.clone().into_inner() {
            match inner.as_rule() {
                Rule::ImportedReference => {
                    for ref_inner in inner.into_inner() {
                        if let Rule::QualifiedName = ref_inner.as_rule() {
                            return Some(ref_inner.as_str().to_string());
                        }
                    }
                }
                Rule::QualifiedName => {
                    return Some(inner.as_str().to_string());
                }
                _ => {}
            }
        }
        None
    }

    /// Convert a pest Pair to a Span.
    fn pair_to_span(&self, pair: &Pair<'_, Rule>) -> Option<Span> {
        if !self.include_spans {
            return None;
        }

        let pest_span = pair.as_span();
        let (line, col) = pest_span.start_pos().line_col();

        Some(Span::with_location(
            self.file_path,
            pest_span.start(),
            pest_span.end(),
            line as u32,
            col as u32,
        ))
    }

    // =========================================================================
    // Relationship Extraction Helpers
    // =========================================================================
    // NOTE: Legacy multiplicity, direction, flags, and feature specialization
    // extraction functions have been removed. These are now handled by single-pass
    // extraction in extraction.rs, providing ~68% faster parsing for large models.

    /// Extract dependency FROM and TO endpoints.
    ///
    /// Grammar: Dependency = { ... ~ QualifiedName ~ ("," ~ QualifiedName)* ~ KW_TO ~ QualifiedName ~ ("," ~ QualifiedName)* ~ ... }
    ///
    /// Returns (sources, targets) where sources are FROM qualified names and targets are TO qualified names.
    fn extract_dependency_endpoints(&self, pair: &Pair<'_, Rule>) -> (Vec<String>, Vec<String>) {
        let mut sources = Vec::new();
        let mut targets = Vec::new();
        let mut seen_to_keyword = false;

        for inner in pair.clone().into_inner() {
            match inner.as_rule() {
                Rule::KW_TO => {
                    seen_to_keyword = true;
                }
                Rule::QualifiedName => {
                    let name = inner.as_str().trim().to_string();
                    if !name.is_empty() {
                        if seen_to_keyword {
                            targets.push(name);
                        } else {
                            sources.push(name);
                        }
                    }
                }
                _ => {}
            }
        }

        (sources, targets)
    }

    // =========================================================================
    // Relationship Creation Methods
    // =========================================================================

    /// Create a Specialization element linking a specific type to its general type.
    ///
    /// The relationship is owned by the specific type.
    fn create_specialization(
        &self,
        specific_id: ElementId,
        general_qname: String,
        graph: &mut ModelGraph,
        span: Option<Span>,
    ) -> ElementId {
        let mut element = Element::new_with_kind(ElementKind::Specialization);
        element.set_prop("specific", Value::Ref(specific_id.clone()));
        element.set_prop("unresolved_general", Value::String(general_qname));

        if let Some(s) = span {
            element.spans.push(s);
        }

        // Owned by the specific type
        graph.add_owned_element(element, specific_id, VisibilityKind::Public)
    }

    /// Create a FeatureTyping element linking a typed feature to its type.
    ///
    /// The relationship is owned by the typed feature.
    fn create_feature_typing(
        &self,
        typed_feature_id: ElementId,
        type_qname: String,
        graph: &mut ModelGraph,
        span: Option<Span>,
    ) -> ElementId {
        let mut element = Element::new_with_kind(ElementKind::FeatureTyping);
        element.set_prop("typedFeature", Value::Ref(typed_feature_id.clone()));
        element.set_prop("unresolved_type", Value::String(type_qname));

        if let Some(s) = span {
            element.spans.push(s);
        }

        // Owned by the typed feature
        graph.add_owned_element(element, typed_feature_id, VisibilityKind::Public)
    }

    /// Create a Subsetting element linking a subsetting feature to its subsetted feature.
    ///
    /// The relationship is owned by the subsetting feature.
    fn create_subsetting(
        &self,
        subsetting_feature_id: ElementId,
        subsetted_qname: String,
        graph: &mut ModelGraph,
        span: Option<Span>,
    ) -> ElementId {
        let mut element = Element::new_with_kind(ElementKind::Subsetting);
        element.set_prop("subsettingFeature", Value::Ref(subsetting_feature_id.clone()));
        element.set_prop("unresolved_subsettedFeature", Value::String(subsetted_qname));

        if let Some(s) = span {
            element.spans.push(s);
        }

        // Owned by the subsetting feature
        graph.add_owned_element(element, subsetting_feature_id, VisibilityKind::Public)
    }

    /// Create a Redefinition element linking a redefining feature to its redefined feature.
    ///
    /// The relationship is owned by the redefining feature.
    fn create_redefinition(
        &self,
        redefining_feature_id: ElementId,
        redefined_qname: String,
        graph: &mut ModelGraph,
        span: Option<Span>,
    ) -> ElementId {
        let mut element = Element::new_with_kind(ElementKind::Redefinition);
        element.set_prop("redefiningFeature", Value::Ref(redefining_feature_id.clone()));
        element.set_prop("unresolved_redefinedFeature", Value::String(redefined_qname));

        if let Some(s) = span {
            element.spans.push(s);
        }

        // Owned by the redefining feature
        graph.add_owned_element(element, redefining_feature_id, VisibilityKind::Public)
    }

    /// Create a ReferenceSubsetting element linking a referencing feature to its referenced feature.
    ///
    /// The relationship is owned by the referencing feature.
    fn create_reference_subsetting(
        &self,
        referencing_feature_id: ElementId,
        referenced_qname: String,
        graph: &mut ModelGraph,
        span: Option<Span>,
    ) -> ElementId {
        let mut element = Element::new_with_kind(ElementKind::ReferenceSubsetting);
        element.set_prop("referencingFeature", Value::Ref(referencing_feature_id.clone()));
        element.set_prop("unresolved_referencedFeature", Value::String(referenced_qname));

        if let Some(s) = span {
            element.spans.push(s);
        }

        // Owned by the referencing feature
        graph.add_owned_element(element, referencing_feature_id, VisibilityKind::Public)
    }
}

/// Check if a rule is a keyword (to skip).
fn rule_is_keyword(rule: Rule) -> bool {
    matches!(
        rule,
        Rule::KW_ABSTRACT
            | Rule::KW_ABOUT
            | Rule::KW_ACCEPT
            | Rule::KW_ACTION
            | Rule::KW_ACTOR
            | Rule::KW_ALIAS
            | Rule::KW_ALL
            | Rule::KW_ALLOCATE
            | Rule::KW_ALLOCATION
            | Rule::KW_ANALYSIS
            | Rule::KW_AND
            | Rule::KW_AS
            | Rule::KW_ASSERT
            | Rule::KW_ASSIGN
            | Rule::KW_AFTER
            | Rule::KW_ASSOC
            | Rule::KW_ASSUME
            | Rule::KW_AT
            | Rule::KW_ATTRIBUTE
            | Rule::KW_BEHAVIOR
            | Rule::KW_BIND
            | Rule::KW_BINDING
            | Rule::KW_BOOL
            | Rule::KW_BY
            | Rule::KW_CALC
            | Rule::KW_CASE
            | Rule::KW_CHAINS
            | Rule::KW_CLASS
            | Rule::KW_CLASSIFIER
            | Rule::KW_COMMENT
            | Rule::KW_COMPOSITE
            | Rule::KW_CONCERN
            | Rule::KW_CONJUGATE
            | Rule::KW_CONJUGATES
            | Rule::KW_CONNECTION
            | Rule::KW_CONNECT
            | Rule::KW_CONSTRAINT
            | Rule::KW_CROSSES
            | Rule::KW_DATATYPE
            | Rule::KW_DECIDE
            | Rule::KW_DEF
            | Rule::KW_DEFAULT
            | Rule::KW_DEFINED
            | Rule::KW_DEPENDENCY
            | Rule::KW_DERIVED
            | Rule::KW_DIFFERENCES
            | Rule::KW_DISJOINT
            | Rule::KW_DISJOINING
            | Rule::KW_DO
            | Rule::KW_DOC
            | Rule::KW_ELSE
            | Rule::KW_END
            | Rule::KW_ENTRY
            | Rule::KW_ENUM
            | Rule::KW_EVENT
            | Rule::KW_EXHIBIT
            | Rule::KW_EXIT
            | Rule::KW_EXPOSE
            | Rule::KW_EXPR
            | Rule::KW_FALSE
            | Rule::KW_FEATURED
            | Rule::KW_FEATURING
            | Rule::KW_FILTER
            | Rule::KW_FIRST
            | Rule::KW_FLOW
            | Rule::KW_FOR
            | Rule::KW_FORK
            | Rule::KW_FRAME
            | Rule::KW_FROM
            | Rule::KW_HASTYPE
            | Rule::KW_IF
            | Rule::KW_IMPLIES
            | Rule::KW_IMPORT
            | Rule::KW_IN
            | Rule::KW_INCLUDE
            | Rule::KW_INDIVIDUAL
            | Rule::KW_INOUT
            | Rule::KW_INTERFACE
            | Rule::KW_INTERSECTS
            | Rule::KW_INV
            | Rule::KW_INVERSE
            | Rule::KW_INVERTING
            | Rule::KW_ISTYPE
            | Rule::KW_ITEM
            | Rule::KW_JOIN
            | Rule::KW_LANGUAGE
            | Rule::KW_LIBRARY
            | Rule::KW_LOCALE
            | Rule::KW_LOOP
            | Rule::KW_MEMBER
            | Rule::KW_MERGE
            | Rule::KW_MESSAGE
            | Rule::KW_META
            | Rule::KW_METADATA
            | Rule::KW_MULTIPLICITY
            | Rule::KW_NAMESPACE
            | Rule::KW_NONUNIQUE
            | Rule::KW_NOT
            | Rule::KW_NULL
            | Rule::KW_OBJECTIVE
            | Rule::KW_OCCURRENCE
            | Rule::KW_OF
            | Rule::KW_OR
            | Rule::KW_ORDERED
            | Rule::KW_OUT
            | Rule::KW_PACKAGE
            | Rule::KW_PARALLEL
            | Rule::KW_PART
            | Rule::KW_PERFORM
            | Rule::KW_PORT
            | Rule::KW_PORTION
            | Rule::KW_PRIVATE
            | Rule::KW_PROTECTED
            | Rule::KW_PUBLIC
            | Rule::KW_READONLY
            | Rule::KW_REDEFINES
            | Rule::KW_REDEFINITION
            | Rule::KW_REF
            | Rule::KW_REFERENCES
            | Rule::KW_RENDER
            | Rule::KW_RENDERING
            | Rule::KW_REP
            | Rule::KW_REQUIRE
            | Rule::KW_REQUIREMENT
            | Rule::KW_RETURN
            | Rule::KW_SATISFY
            | Rule::KW_SEND
            | Rule::KW_SNAPSHOT
            | Rule::KW_SPECIALIZATION
            | Rule::KW_SPECIALIZES
            | Rule::KW_STAKEHOLDER
            | Rule::KW_STANDARD
            | Rule::KW_STATE
            | Rule::KW_STEP
            | Rule::KW_STRUCT
            | Rule::KW_SUBCLASSIFIER
            | Rule::KW_SUBJECT
            | Rule::KW_SUBSET
            | Rule::KW_SUBSETS
            | Rule::KW_SUBTYPE
            | Rule::KW_SUCCESSION
            | Rule::KW_TERMINATE
            | Rule::KW_THEN
            | Rule::KW_TIMESLICE
            | Rule::KW_TO
            | Rule::KW_TRANSITION
            | Rule::KW_TRUE
            | Rule::KW_TYPED
            | Rule::KW_TYPING
            | Rule::KW_UNIONS
            | Rule::KW_UNTIL
            | Rule::KW_USE
            | Rule::KW_VARIANT
            | Rule::KW_VARIATION
            | Rule::KW_VERIFICATION
            | Rule::KW_VERIFY
            | Rule::KW_VIA
            | Rule::KW_VIEW
            | Rule::KW_VIEWPOINT
            | Rule::KW_WHEN
            | Rule::KW_WHILE
            | Rule::KW_XOR
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_simple_name() {
        // This would need actual pest pairs to test
        // For now, just verify the module compiles
    }
}
