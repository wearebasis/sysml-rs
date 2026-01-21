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

use pest::iterators::{Pair, Pairs};
use sysml_core::{Element, ElementKind, ModelGraph, VisibilityKind};
use sysml_id::ElementId;
use sysml_span::Span;

use crate::{ParseError, Rule};

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

    /// Convert pest pairs to ModelGraph.
    pub fn convert(mut self, pairs: Pairs<'_, Rule>, graph: &mut ModelGraph) -> Result<(), ParseError> {
        for pair in pairs {
            self.convert_pair(pair, graph)?;
        }
        Ok(())
    }

    /// Convert a single pair and its children.
    fn convert_pair(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph) -> Result<Option<ElementId>, ParseError> {
        let rule = pair.as_rule();
        let span = self.pair_to_span(&pair);

        match rule {
            // Skip tokens and whitespace
            Rule::WHITESPACE | Rule::NEWLINE | Rule::COMMENT | Rule::SL_COMMENT | Rule::ML_COMMENT
            | Rule::SL_NOTE | Rule::ML_NOTE | Rule::EOI => Ok(None),

            // Entry points
            Rule::File | Rule::Model => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            // Packages
            Rule::Package => self.convert_package(pair, graph, span),
            Rule::LibraryPackage => self.convert_library_package(pair, graph, span),

            // Package body elements
            Rule::PackageBodyElement => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            // Definitions
            Rule::DefinitionElement | Rule::DefinitionMember => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            Rule::AttributeDefinition => self.convert_definition(pair, graph, ElementKind::AttributeDefinition, span),
            Rule::EnumerationDefinition => self.convert_definition(pair, graph, ElementKind::EnumerationDefinition, span),
            Rule::OccurrenceDefinition => self.convert_definition(pair, graph, ElementKind::OccurrenceDefinition, span),
            Rule::ItemDefinition => self.convert_definition(pair, graph, ElementKind::ItemDefinition, span),
            Rule::MetadataDefinition => self.convert_definition(pair, graph, ElementKind::MetadataDefinition, span),
            Rule::PartDefinition => self.convert_definition(pair, graph, ElementKind::PartDefinition, span),
            Rule::PortDefinition => self.convert_definition(pair, graph, ElementKind::PortDefinition, span),
            Rule::ConnectionDefinition => self.convert_definition(pair, graph, ElementKind::ConnectionDefinition, span),
            Rule::FlowConnectionDefinition => self.convert_definition(pair, graph, ElementKind::ConnectionDefinition, span),
            Rule::InterfaceDefinition => self.convert_definition(pair, graph, ElementKind::InterfaceDefinition, span),
            Rule::AllocationDefinition => self.convert_definition(pair, graph, ElementKind::AllocationDefinition, span),
            Rule::ActionDefinition => self.convert_definition(pair, graph, ElementKind::ActionDefinition, span),
            Rule::CalculationDefinition => self.convert_definition(pair, graph, ElementKind::CalculationDefinition, span),
            Rule::StateDefinition => self.convert_definition(pair, graph, ElementKind::StateDefinition, span),
            Rule::ConstraintDefinition => self.convert_definition(pair, graph, ElementKind::ConstraintDefinition, span),
            Rule::RequirementDefinition => self.convert_definition(pair, graph, ElementKind::RequirementDefinition, span),
            Rule::ConcernDefinition => self.convert_definition(pair, graph, ElementKind::ConcernDefinition, span),
            Rule::CaseDefinition => self.convert_definition(pair, graph, ElementKind::CaseDefinition, span),
            Rule::AnalysisCaseDefinition => self.convert_definition(pair, graph, ElementKind::AnalysisCaseDefinition, span),
            Rule::VerificationCaseDefinition => self.convert_definition(pair, graph, ElementKind::VerificationCaseDefinition, span),
            Rule::UseCaseDefinition => self.convert_definition(pair, graph, ElementKind::UseCaseDefinition, span),
            Rule::ViewDefinition => self.convert_definition(pair, graph, ElementKind::ViewDefinition, span),
            Rule::ViewpointDefinition => self.convert_definition(pair, graph, ElementKind::ViewpointDefinition, span),
            Rule::RenderingDefinition => self.convert_definition(pair, graph, ElementKind::RenderingDefinition, span),
            Rule::ExtendedDefinition => self.convert_definition(pair, graph, ElementKind::Definition, span),

            // Usages
            Rule::UsageElement | Rule::UsageMember | Rule::NonOccurrenceUsageElement | Rule::OccurrenceUsageElement
            | Rule::StructureUsageElement | Rule::BehaviorUsageElement | Rule::VariantUsageElement
            | Rule::NonOccurrenceUsageMember | Rule::OccurrenceUsageMember | Rule::VariantUsageMember => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            Rule::ReferenceUsage | Rule::DefaultReferenceUsage => self.convert_usage(pair, graph, ElementKind::ReferenceUsage, span),
            Rule::AttributeUsage => self.convert_usage(pair, graph, ElementKind::AttributeUsage, span),
            Rule::EnumerationUsage => self.convert_usage(pair, graph, ElementKind::EnumerationUsage, span),
            Rule::OccurrenceUsage => self.convert_usage(pair, graph, ElementKind::OccurrenceUsage, span),
            Rule::IndividualUsage => self.convert_usage(pair, graph, ElementKind::OccurrenceUsage, span),
            Rule::PortionUsage => self.convert_usage(pair, graph, ElementKind::OccurrenceUsage, span),
            Rule::EventOccurrenceUsage => self.convert_usage(pair, graph, ElementKind::EventOccurrenceUsage, span),
            Rule::ItemUsage => self.convert_usage(pair, graph, ElementKind::ItemUsage, span),
            Rule::PartUsage => self.convert_usage(pair, graph, ElementKind::PartUsage, span),
            Rule::PortUsage => self.convert_usage(pair, graph, ElementKind::PortUsage, span),
            Rule::ConnectionUsage => self.convert_usage(pair, graph, ElementKind::ConnectionUsage, span),
            Rule::InterfaceUsage => self.convert_usage(pair, graph, ElementKind::InterfaceUsage, span),
            Rule::AllocationUsage => self.convert_usage(pair, graph, ElementKind::AllocationUsage, span),
            Rule::FlowConnectionUsage => self.convert_usage(pair, graph, ElementKind::ConnectionUsage, span),
            Rule::ViewUsage => self.convert_usage(pair, graph, ElementKind::ViewUsage, span),
            Rule::RenderingUsage => self.convert_usage(pair, graph, ElementKind::RenderingUsage, span),
            Rule::ActionUsage => self.convert_usage(pair, graph, ElementKind::ActionUsage, span),
            Rule::PerformActionUsage => self.convert_usage(pair, graph, ElementKind::PerformActionUsage, span),
            Rule::CalculationUsage => self.convert_usage(pair, graph, ElementKind::CalculationUsage, span),
            Rule::StateUsage => self.convert_usage(pair, graph, ElementKind::StateUsage, span),
            Rule::ExhibitStateUsage => self.convert_usage(pair, graph, ElementKind::ExhibitStateUsage, span),
            Rule::ConstraintUsage => self.convert_usage(pair, graph, ElementKind::ConstraintUsage, span),
            Rule::AssertConstraintUsage => self.convert_usage(pair, graph, ElementKind::AssertConstraintUsage, span),
            Rule::RequirementUsage => self.convert_usage(pair, graph, ElementKind::RequirementUsage, span),
            Rule::SatisfyRequirementUsage => self.convert_usage(pair, graph, ElementKind::SatisfyRequirementUsage, span),
            Rule::ConcernUsage => self.convert_usage(pair, graph, ElementKind::ConcernUsage, span),
            Rule::CaseUsage => self.convert_usage(pair, graph, ElementKind::CaseUsage, span),
            Rule::AnalysisCaseUsage => self.convert_usage(pair, graph, ElementKind::AnalysisCaseUsage, span),
            Rule::VerificationCaseUsage => self.convert_usage(pair, graph, ElementKind::VerificationCaseUsage, span),
            Rule::UseCaseUsage => self.convert_usage(pair, graph, ElementKind::UseCaseUsage, span),
            Rule::IncludeUseCaseUsage => self.convert_usage(pair, graph, ElementKind::IncludeUseCaseUsage, span),
            Rule::ViewpointUsage => self.convert_usage(pair, graph, ElementKind::ViewpointUsage, span),
            Rule::BindingConnectorAsUsage => self.convert_usage(pair, graph, ElementKind::BindingConnectorAsUsage, span),
            Rule::SuccessionAsUsage => self.convert_usage(pair, graph, ElementKind::SuccessionAsUsage, span),
            Rule::ExtendedUsage => self.convert_usage(pair, graph, ElementKind::Usage, span),
            Rule::VariantReference => self.convert_usage(pair, graph, ElementKind::ReferenceUsage, span),

            // Imports
            Rule::Import => self.convert_import(pair, graph, span),

            // Annotations
            Rule::AnnotatingMember | Rule::AnnotatingElement => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }
            Rule::Comment => self.convert_comment(pair, graph, span),
            Rule::Documentation => self.convert_documentation(pair, graph, span),
            Rule::MetadataUsage => self.convert_metadata_usage(pair, graph, span),

            // Dependencies
            Rule::Dependency => self.convert_dependency(pair, graph, span),
            Rule::RelationshipMember => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            // Skip intermediate rules - process their children
            Rule::DefinitionBody | Rule::DefinitionBodyItem | Rule::PackageBody
            | Rule::ActionBody | Rule::ActionBodyItem | Rule::StateBody | Rule::StateBodyItem
            | Rule::CalculationBody | Rule::CalculationBodyItem | Rule::RequirementBody | Rule::RequirementBodyItem
            | Rule::CaseBody | Rule::CaseBodyItem | Rule::ViewBody | Rule::ViewBodyItem
            | Rule::InterfaceBody | Rule::InterfaceBodyItem | Rule::EnumerationBody | Rule::EnumerationBodyItem
            | Rule::MetadataBody | Rule::MetadataBodyItem => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }

            // Names and identifiers - return name as string (for extraction)
            Rule::Name | Rule::ID | Rule::UNRESTRICTED_NAME | Rule::QualifiedName
            | Rule::RegularName | Rule::ShortName | Rule::Identification => {
                // Names are extracted by parent rules
                Ok(None)
            }

            // Tokens and keywords - skip
            _ if rule_is_keyword(rule) => Ok(None),

            // Default: process children
            _ => {
                for inner in pair.into_inner() {
                    self.convert_pair(inner, graph)?;
                }
                Ok(None)
            }
        }
    }

    /// Convert a Package to an Element.
    fn convert_package(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Package);

        // Extract name from identification
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        // Process children with this package as owner
        self.owner_stack.push(id.clone());
        for inner in pair.into_inner() {
            self.convert_pair(inner, graph)?;
        }
        self.owner_stack.pop();

        Ok(Some(id))
    }

    /// Convert a LibraryPackage to an Element.
    fn convert_library_package(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::LibraryPackage);

        // Extract name from identification
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Check for 'standard' flag
        let text = pair.as_str();
        if text.starts_with("standard") {
            element.set_prop("isStandard", true);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        // Process children with this package as owner
        self.owner_stack.push(id.clone());
        for inner in pair.into_inner() {
            self.convert_pair(inner, graph)?;
        }
        self.owner_stack.pop();

        Ok(Some(id))
    }

    /// Convert a definition to an Element.
    fn convert_definition(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, kind: ElementKind, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(kind);

        // Extract name from identification
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Check for 'abstract' flag
        let text = pair.as_str();
        if text.contains("abstract") {
            element.set_prop("isAbstract", true);
        }

        // Check for 'variation' flag
        if text.contains("variation") {
            element.set_prop("isVariation", true);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        // Process children with this definition as owner
        self.owner_stack.push(id.clone());
        for inner in pair.into_inner() {
            self.convert_pair(inner, graph)?;
        }
        self.owner_stack.pop();

        Ok(Some(id))
    }

    /// Convert a usage to an Element.
    fn convert_usage(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, kind: ElementKind, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(kind);

        // Extract name from identification
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Check for directional flags
        let text = pair.as_str();
        if text.starts_with("in ") || text.contains(" in ") {
            element.set_prop("direction", "in");
        } else if text.starts_with("out ") || text.contains(" out ") {
            element.set_prop("direction", "out");
        } else if text.starts_with("inout ") || text.contains(" inout ") {
            element.set_prop("direction", "inout");
        }

        // Check for other flags
        if text.contains("abstract") {
            element.set_prop("isAbstract", true);
        }
        if text.contains("variation") {
            element.set_prop("isVariation", true);
        }
        if text.contains("readonly") {
            element.set_prop("isReadOnly", true);
        }
        if text.contains("derived") {
            element.set_prop("isDerived", true);
        }
        if text.contains("end") {
            element.set_prop("isEnd", true);
        }
        if text.contains("ref ") {
            element.set_prop("isReference", true);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        // Process children with this usage as owner
        self.owner_stack.push(id.clone());
        for inner in pair.into_inner() {
            self.convert_pair(inner, graph)?;
        }
        self.owner_stack.pop();

        Ok(Some(id))
    }

    /// Convert an Import to an Element.
    fn convert_import(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
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

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        Ok(Some(id))
    }

    /// Convert a Comment to an Element.
    fn convert_comment(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Comment);

        // Extract comment body
        let text = pair.as_str();
        if let Some(start) = text.find("/*") {
            if let Some(end) = text.rfind("*/") {
                let body = &text[start + 2..end];
                element.set_prop("body", body.trim());
            }
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        Ok(Some(id))
    }

    /// Convert Documentation to an Element.
    fn convert_documentation(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Documentation);

        // Extract documentation body
        let text = pair.as_str();
        if let Some(start) = text.find("/*") {
            if let Some(end) = text.rfind("*/") {
                let body = &text[start + 2..end];
                element.set_prop("body", body.trim());
            }
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        Ok(Some(id))
    }

    /// Convert MetadataUsage to an Element.
    fn convert_metadata_usage(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::MetadataUsage);

        // Extract name if present
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        // Process children with this metadata as owner
        self.owner_stack.push(id.clone());
        for inner in pair.into_inner() {
            self.convert_pair(inner, graph)?;
        }
        self.owner_stack.pop();

        Ok(Some(id))
    }

    /// Convert a Dependency to an Element and Relationships.
    fn convert_dependency(&mut self, pair: Pair<'_, Rule>, graph: &mut ModelGraph, span: Option<Span>) -> Result<Option<ElementId>, ParseError> {
        let mut element = Element::new_with_kind(ElementKind::Dependency);

        // Extract name if present
        if let Some(name) = self.extract_name(&pair) {
            element.name = Some(name);
        }

        // Set owner if we have one
        if let Some(owner_id) = self.owner_stack.last() {
            element.owner = Some(owner_id.clone());
        }

        // Add span
        if let Some(s) = span {
            element.spans.push(s);
        }

        let id = element.id.clone();
        graph.add_element(element);

        Ok(Some(id))
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
            | Rule::KW_FEATURE
            | Rule::KW_FEATURED
            | Rule::KW_FEATURING
            | Rule::KW_FILTER
            | Rule::KW_FIRST
            | Rule::KW_FLOW
            | Rule::KW_FOR
            | Rule::KW_FORK
            | Rule::KW_FRAME
            | Rule::KW_FROM
            | Rule::KW_FUNCTION
            | Rule::KW_HASTYPE
            | Rule::KW_IF
            | Rule::KW_IMPLIES
            | Rule::KW_IMPORT
            | Rule::KW_IN
            | Rule::KW_INCLUDE
            | Rule::KW_INDIVIDUAL
            | Rule::KW_INOUT
            | Rule::KW_INTERACTION
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
            | Rule::KW_METACLASS
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
            | Rule::KW_PREDICATE
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
            | Rule::KW_TYPE
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
