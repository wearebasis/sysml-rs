//! Generator for relationship source/target type constraint methods.
//!
//! This module generates methods on `ElementKind` that expose the expected
//! source and target types for relationship elements.

use crate::json_schema_parser::JsonRelationshipConstraint;
use crate::ttl_parser::TypeInfo;
use crate::xmi_relationship_parser::XmiRelationshipConstraint;
use std::collections::{HashMap, HashSet};

/// Relationship constraint with source and target types.
pub struct RelationshipConstraint {
    pub source_type: String,
    pub target_type: String,
}

impl RelationshipConstraint {
    pub fn new(source_type: impl Into<String>, target_type: impl Into<String>) -> Self {
        Self {
            source_type: source_type.into(),
            target_type: target_type.into(),
        }
    }
}

impl From<JsonRelationshipConstraint> for (String, RelationshipConstraint) {
    fn from(jrc: JsonRelationshipConstraint) -> Self {
        (
            jrc.relationship_type,
            RelationshipConstraint::new(jrc.source_type, jrc.target_type),
        )
    }
}

/// Build constraints map from JSON-parsed constraints.
pub fn build_constraints_map(
    json_constraints: Vec<JsonRelationshipConstraint>,
) -> HashMap<String, RelationshipConstraint> {
    json_constraints.into_iter().map(Into::into).collect()
}

/// Get fallback constraints for relationships not in JSON or as defaults.
/// These are used when JSON parsing doesn't yield specific constraints.
fn get_fallback_constraints() -> HashMap<&'static str, RelationshipConstraint> {
    let mut constraints = HashMap::new();

    // Base Relationship type - defines source/target as Element
    // This is not a redefinition, it's the base definition that subclasses specialize
    constraints.insert("Relationship", RelationshipConstraint::new("Element", "Element"));

    // Specialization relationships - defaults for subtypes
    constraints.insert("Specialization", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("FeatureTyping", RelationshipConstraint::new("Feature", "Type"));
    constraints.insert("Subsetting", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("Redefinition", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("ReferenceSubsetting", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("Subclassification", RelationshipConstraint::new("Classifier", "Classifier"));
    constraints.insert("Conjugation", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("Disjoining", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("Differencing", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("Intersecting", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("Unioning", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("CrossSubsetting", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("ConjugatedPortTyping", RelationshipConstraint::new("Feature", "ConjugatedPortDefinition"));

    // Feature relationships
    constraints.insert("FeatureChaining", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("FeatureInverting", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("TypeFeaturing", RelationshipConstraint::new("Feature", "Type"));

    // Membership relationships
    constraints.insert("Membership", RelationshipConstraint::new("Namespace", "Element"));
    constraints.insert("OwningMembership", RelationshipConstraint::new("Namespace", "Element"));
    constraints.insert("FeatureMembership", RelationshipConstraint::new("Type", "Feature"));
    constraints.insert("EndFeatureMembership", RelationshipConstraint::new("Type", "Feature"));
    constraints.insert("ParameterMembership", RelationshipConstraint::new("Type", "Feature"));
    constraints.insert("ResultExpressionMembership", RelationshipConstraint::new("Function", "Expression"));
    constraints.insert("ReturnParameterMembership", RelationshipConstraint::new("Type", "Feature"));
    constraints.insert("ObjectiveMembership", RelationshipConstraint::new("CaseDefinition", "RequirementUsage"));
    constraints.insert("SubjectMembership", RelationshipConstraint::new("Type", "Usage"));
    constraints.insert("ActorMembership", RelationshipConstraint::new("Type", "PartUsage"));
    constraints.insert("StakeholderMembership", RelationshipConstraint::new("Type", "PartUsage"));
    constraints.insert("RequirementConstraintMembership", RelationshipConstraint::new("RequirementDefinition", "ConstraintUsage"));
    constraints.insert("RequirementVerificationMembership", RelationshipConstraint::new("RequirementDefinition", "RequirementUsage"));
    constraints.insert("FramedConcernMembership", RelationshipConstraint::new("Type", "ConcernUsage"));
    constraints.insert("ViewRenderingMembership", RelationshipConstraint::new("ViewDefinition", "RenderingUsage"));
    constraints.insert("ElementFilterMembership", RelationshipConstraint::new("Namespace", "Expression"));
    constraints.insert("VariantMembership", RelationshipConstraint::new("Type", "Usage"));
    constraints.insert("StateSubactionMembership", RelationshipConstraint::new("StateDefinition", "ActionUsage"));
    constraints.insert("TransitionFeatureMembership", RelationshipConstraint::new("Type", "Step"));

    // Import relationships
    constraints.insert("Import", RelationshipConstraint::new("Namespace", "Element"));
    constraints.insert("MembershipImport", RelationshipConstraint::new("Namespace", "Membership"));
    constraints.insert("NamespaceImport", RelationshipConstraint::new("Namespace", "Namespace"));
    constraints.insert("Expose", RelationshipConstraint::new("ViewUsage", "Membership"));

    // Annotation relationships
    constraints.insert("Annotation", RelationshipConstraint::new("AnnotatingElement", "Element"));

    // Dependency
    constraints.insert("Dependency", RelationshipConstraint::new("Element", "Element"));

    // Port relationships
    constraints.insert("PortConjugation", RelationshipConstraint::new("ConjugatedPortDefinition", "PortDefinition"));

    // Connection and flow relationships (use generic ends)
    constraints.insert("ItemFlow", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("FlowConnectionUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("SuccessionFlowConnectionUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("Succession", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("SuccessionAsUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("ConnectionUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("InterfaceUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("BindingConnectorAsUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("AllocationUsage", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("BindingConnector", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("Connector", RelationshipConstraint::new("Feature", "Feature"));
    constraints.insert("Association", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("AssociationStructure", RelationshipConstraint::new("Type", "Type"));
    constraints.insert("Interaction", RelationshipConstraint::new("Type", "Type"));

    constraints
}

/// Generate relationship constraint methods for ElementKind.
///
/// This generates:
/// - `relationship_source_type()` - Expected source type for relationship kinds
/// - `relationship_target_type()` - Expected target type for relationship kinds
///
/// If `json_constraints` is provided, those take precedence over fallback constraints.
pub fn generate_relationship_methods(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
) -> String {
    generate_relationship_methods_with_constraints(kerml_types, sysml_types, Vec::new())
}

/// Generate relationship constraint methods with JSON-parsed constraints.
///
/// JSON constraints are merged with fallback constraints, with JSON taking precedence.
pub fn generate_relationship_methods_with_constraints(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
    json_constraints: Vec<JsonRelationshipConstraint>,
) -> String {
    let mut output = String::new();

    output.push_str("\n// === Relationship Type Constraint Methods ===\n\n");

    // Build hierarchy to identify all relationship types
    let hierarchy = build_type_hierarchy(kerml_types, sysml_types);

    // Get all type names
    let all_types: HashSet<&str> = kerml_types
        .iter()
        .chain(sysml_types.iter())
        .map(|t| t.name.as_str())
        .collect();

    // Find all relationship types (deduplicated)
    let relationship_set: HashSet<&str> = kerml_types
        .iter()
        .chain(sysml_types.iter())
        .filter(|t| {
            t.name == "Relationship"
                || hierarchy
                    .get(&t.name)
                    .map_or(false, |s| s.contains(&"Relationship".to_string()))
        })
        .map(|t| t.name.as_str())
        .collect();

    // Convert to sorted Vec for consistent output
    let mut relationship_types: Vec<&str> = relationship_set.into_iter().collect();
    relationship_types.sort();

    // Build merged constraints map: JSON constraints override fallbacks
    let json_map = build_constraints_map(json_constraints);
    let fallback_map = get_fallback_constraints();

    // Helper to get constraint for a relationship type
    let get_constraint = |rel_type: &str| -> (&str, &str) {
        if let Some(c) = json_map.get(rel_type) {
            (c.source_type.as_str(), c.target_type.as_str())
        } else if let Some(c) = fallback_map.get(rel_type) {
            (c.source_type.as_str(), c.target_type.as_str())
        } else {
            ("Element", "Element")
        }
    };

    output.push_str("impl ElementKind {\n");

    // relationship_source_type()
    output.push_str("    /// For relationship types, returns the expected source element type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// The source type indicates what kind of element can be the source of this relationship.\n");
    output.push_str("    /// Returns `None` for non-relationship types.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::FeatureTyping.relationship_source_type(),\n");
    output.push_str("    ///     Some(ElementKind::Feature)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.relationship_source_type(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_source_type(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");

    for rel_type in &relationship_types {
        let (source_type, _) = get_constraint(rel_type);

        // Only include if the source type exists in our type list
        if all_types.contains(source_type) {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::{}),\n",
                rel_type, source_type
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::Element),\n",
                rel_type
            ));
        }
    }

    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    // relationship_target_type()
    output.push_str("    /// For relationship types, returns the expected target element type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// The target type indicates what kind of element can be the target of this relationship.\n");
    output.push_str("    /// Returns `None` for non-relationship types.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::FeatureTyping.relationship_target_type(),\n");
    output.push_str("    ///     Some(ElementKind::Type)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.relationship_target_type(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_target_type(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");

    for rel_type in &relationship_types {
        let (_, target_type) = get_constraint(rel_type);

        // Only include if the target type exists in our type list
        if all_types.contains(target_type) {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::{}),\n",
                rel_type, target_type
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::Element),\n",
                rel_type
            ));
        }
    }

    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    output.push_str("}\n");

    output
}

/// Generate relationship constraint methods with XMI-derived constraints.
///
/// XMI constraints are the authoritative source. Fallback constraints are used
/// for any types not found in XMI (if any).
///
/// # Priority Order
/// 1. XMI constraints (authoritative, from metamodel)
/// 2. Fallback constraints (for types not in XMI)
/// 3. Default to Element (only if neither source has the type)
pub fn generate_relationship_methods_with_xmi(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
    xmi_constraints: &HashMap<String, XmiRelationshipConstraint>,
) -> String {
    let mut output = String::new();

    output.push_str("\n// === Relationship Type Constraint Methods ===\n");
    output.push_str("// Generated from XMI metamodel files\n\n");

    // Build hierarchy to identify all relationship types
    let hierarchy = build_type_hierarchy(kerml_types, sysml_types);

    // Get all type names
    let all_types: HashSet<&str> = kerml_types
        .iter()
        .chain(sysml_types.iter())
        .map(|t| t.name.as_str())
        .collect();

    // Find all relationship types (deduplicated)
    let relationship_set: HashSet<&str> = kerml_types
        .iter()
        .chain(sysml_types.iter())
        .filter(|t| {
            t.name == "Relationship"
                || hierarchy
                    .get(&t.name)
                    .map_or(false, |s| s.contains(&"Relationship".to_string()))
        })
        .map(|t| t.name.as_str())
        .collect();

    // Convert to sorted Vec for consistent output
    let mut relationship_types: Vec<&str> = relationship_set.into_iter().collect();
    relationship_types.sort();

    // Get fallback constraints for types not in XMI
    let fallback_map = get_fallback_constraints();

    // Helper to get constraint for a relationship type
    // Priority: XMI > Fallback > Default (Element)
    let get_constraint = |rel_type: &str| -> (&str, &str) {
        // Priority 1: XMI constraint (authoritative)
        if let Some(c) = xmi_constraints.get(rel_type) {
            return (c.source_type.as_str(), c.target_type.as_str());
        }
        // Priority 2: Fallback constraint
        if let Some(c) = fallback_map.get(rel_type) {
            return (c.source_type.as_str(), c.target_type.as_str());
        }
        // Default: Element (only if neither source has it)
        ("Element", "Element")
    };

    output.push_str("impl ElementKind {\n");

    // relationship_source_type()
    output.push_str("    /// For relationship types, returns the expected source element type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// The source type indicates what kind of element can be the source of this relationship.\n");
    output.push_str("    /// Returns `None` for non-relationship types.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::FeatureTyping.relationship_source_type(),\n");
    output.push_str("    ///     Some(ElementKind::Feature)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.relationship_source_type(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_source_type(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");

    for rel_type in &relationship_types {
        let (source_type, _) = get_constraint(rel_type);

        // Only include if the source type exists in our type list
        if all_types.contains(source_type) {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::{}),\n",
                rel_type, source_type
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::Element),\n",
                rel_type
            ));
        }
    }

    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    // relationship_target_type()
    output.push_str("    /// For relationship types, returns the expected target element type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// The target type indicates what kind of element can be the target of this relationship.\n");
    output.push_str("    /// Returns `None` for non-relationship types.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::FeatureTyping.relationship_target_type(),\n");
    output.push_str("    ///     Some(ElementKind::Type)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.relationship_target_type(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_target_type(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");

    for rel_type in &relationship_types {
        let (_, target_type) = get_constraint(rel_type);

        // Only include if the target type exists in our type list
        if all_types.contains(target_type) {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::{}),\n",
                rel_type, target_type
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => Some(ElementKind::Element),\n",
                rel_type
            ));
        }
    }

    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    output.push_str("}\n");

    output
}

/// Get the list of fallback constraint type names.
///
/// This is useful for coverage validation.
pub fn get_fallback_constraint_names() -> Vec<&'static str> {
    get_fallback_constraints().keys().copied().collect()
}

/// Information about a relationship's target property.
#[derive(Debug, Clone)]
pub struct RelationshipTargetProperty {
    /// The property name containing the target (e.g., "general", "type").
    pub property: String,
    /// Whether this is a multi-valued (list) property.
    pub is_multi: bool,
}

/// Source property names that represent the "source" side of a relationship.
/// These are the owning features, not the targets we want to validate.
const SOURCE_PROPERTY_PATTERNS: &[&str] = &[
    "specific",           // Specialization - the specializing type
    "subsettingFeature",  // Subsetting - the subsetting feature
    "redefiningFeature",  // Redefinition - the redefining feature
    "featureInverted",    // FeatureInverting - the inverted feature
    "typedFeature",       // FeatureTyping - the typed feature
    "typeDisjoined",      // Disjoining - the disjoined type
    "subclassifier",      // Subclassification - the subclassifier
    "featureOfType",      // TypeFeaturing - the featured type
    "annotatingElement",  // Annotation - the annotating element (source)
    "importOwningNamespace", // Import - the importing namespace
    "owningRelatedElement", // Generic - owner element
    "membershipOwningNamespace", // Membership - owning namespace
];

/// Map grammar rule names to element type names where they differ.
///
/// The Xtext grammar often uses fragment names or abbreviated rule names
/// that differ from the actual element type names.
fn normalize_rule_to_element_type(rule_name: &str) -> String {
    // First strip "Owned" prefix if present
    let base = rule_name.strip_prefix("Owned").unwrap_or(rule_name);

    // Map grammar fragment/rule names to ElementKind names
    match base {
        // FeatureType fragment returns FeatureTyping
        "FeatureType" => "FeatureTyping".to_string(),
        // MetadataTyping returns FeatureTyping (but for metadata)
        "MetadataTyping" => "FeatureTyping".to_string(),
        // ConjugatedPortTyping rule -> ConjugatedPortTyping element
        "ConjugatedPortTyping" => "ConjugatedPortTyping".to_string(),
        // CrossSubsetting is a feature chain subsetting
        "CrossSubsetting" => "CrossSubsetting".to_string(),
        // Keep the base name for most rules
        _ => base.to_string(),
    }
}

/// Build a map from relationship type names to their target property info.
///
/// Uses cross-reference data from Xtext grammar to identify which property
/// contains the target element reference.
pub fn build_relationship_target_properties(
    cross_refs: &[crate::xtext_crossref_parser::CrossReference],
) -> HashMap<String, RelationshipTargetProperty> {
    let mut result: HashMap<String, RelationshipTargetProperty> = HashMap::new();

    for cr in cross_refs {
        // Skip source-side properties (these are the owning/source features)
        if is_source_property(&cr.property) {
            continue;
        }

        // Normalize rule name to element type name
        let element_type = normalize_rule_to_element_type(&cr.containing_rule);

        // Only add if we don't already have an entry (first occurrence wins)
        result.entry(element_type).or_insert_with(|| RelationshipTargetProperty {
            property: cr.property.clone(),
            is_multi: cr.is_multi,
        });
    }

    result
}

/// Check if a property name represents the source side of a relationship.
fn is_source_property(property: &str) -> bool {
    SOURCE_PROPERTY_PATTERNS.contains(&property)
}

/// Coverage report for relationship target property mappings.
#[derive(Debug)]
pub struct RelationshipPropertyCoverageReport {
    /// Total number of relationship types.
    pub total_relationships: usize,
    /// Number of relationships with target property mappings.
    pub with_mapping: usize,
    /// Relationship types without mappings.
    pub without_mapping: Vec<String>,
    /// Coverage percentage.
    pub coverage_percent: f64,
}

/// Validate coverage of relationship target property mappings.
pub fn validate_relationship_property_coverage(
    relationship_types: &[&str],
    property_map: &HashMap<String, RelationshipTargetProperty>,
) -> RelationshipPropertyCoverageReport {
    let mut without_mapping = Vec::new();
    let mut with_mapping = 0;

    for rel_type in relationship_types {
        if property_map.contains_key(*rel_type) {
            with_mapping += 1;
        } else {
            without_mapping.push(rel_type.to_string());
        }
    }

    let total = relationship_types.len();
    let coverage_percent = if total > 0 {
        (with_mapping as f64 / total as f64) * 100.0
    } else {
        100.0
    };

    RelationshipPropertyCoverageReport {
        total_relationships: total,
        with_mapping,
        without_mapping,
        coverage_percent,
    }
}

/// Generate relationship target property methods for ElementKind.
///
/// Generates:
/// - `relationship_target_property()` - Returns the property name containing the target
/// - `relationship_target_is_list()` - Returns whether the target property is a list
pub fn generate_relationship_property_methods(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
    property_map: &HashMap<String, RelationshipTargetProperty>,
) -> String {
    let mut output = String::new();

    output.push_str("\n// === Relationship Target Property Methods ===\n");
    output.push_str("// Generated from Xtext cross-reference registry\n\n");

    // Build hierarchy to identify all relationship types
    let hierarchy = build_type_hierarchy(kerml_types, sysml_types);

    // Find all relationship types (deduplicated)
    let relationship_set: HashSet<&str> = kerml_types
        .iter()
        .chain(sysml_types.iter())
        .filter(|t| {
            t.name == "Relationship"
                || hierarchy
                    .get(&t.name)
                    .map_or(false, |s| s.contains(&"Relationship".to_string()))
        })
        .map(|t| t.name.as_str())
        .collect();

    // Convert to sorted Vec for consistent output
    let mut relationship_types: Vec<&str> = relationship_set.into_iter().collect();
    relationship_types.sort();

    output.push_str("impl ElementKind {\n");

    // relationship_target_property()
    output.push_str("    /// For relationship types, returns the property name containing the target element.\n");
    output.push_str("    ///\n");
    output.push_str("    /// This property name can be used to look up the target ElementId in the\n");
    output.push_str("    /// relationship element's props map after name resolution.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(ElementKind::Specialization.relationship_target_property(), Some(\"general\"));\n");
    output.push_str("    /// assert_eq!(ElementKind::FeatureTyping.relationship_target_property(), Some(\"type\"));\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.relationship_target_property(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_target_property(&self) -> Option<&'static str> {\n");
    output.push_str("        match self {\n");

    for rel_type in &relationship_types {
        if let Some(prop_info) = property_map.get(*rel_type) {
            output.push_str(&format!(
                "            ElementKind::{} => Some(\"{}\"),\n",
                rel_type, prop_info.property
            ));
        }
    }

    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    // relationship_target_is_list()
    output.push_str("    /// For relationship types, returns whether the target property is a list.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Most relationships have a single target, but some (like Dependency.supplier)\n");
    output.push_str("    /// can have multiple targets.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(ElementKind::Dependency.relationship_target_is_list(), true);\n");
    output.push_str("    /// assert_eq!(ElementKind::Specialization.relationship_target_is_list(), false);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn relationship_target_is_list(&self) -> bool {\n");
    output.push_str("        match self {\n");

    // Only output entries for list properties to keep the match arm small
    for rel_type in &relationship_types {
        if let Some(prop_info) = property_map.get(*rel_type) {
            if prop_info.is_multi {
                output.push_str(&format!(
                    "            ElementKind::{} => true,\n",
                    rel_type
                ));
            }
        }
    }

    output.push_str("            _ => false,\n");
    output.push_str("        }\n");
    output.push_str("    }\n");

    output.push_str("}\n");

    output
}

/// Build a map from type name to all its supertypes (direct + transitive).
fn build_type_hierarchy(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
) -> HashMap<String, Vec<String>> {
    // First, build direct supertype map
    let mut direct_supertypes: HashMap<String, Vec<String>> = HashMap::new();

    for type_info in kerml_types.iter().chain(sysml_types.iter()) {
        direct_supertypes.insert(type_info.name.clone(), type_info.supertypes.clone());
    }

    // Now compute transitive closure
    let mut hierarchy: HashMap<String, Vec<String>> = HashMap::new();

    for type_info in kerml_types.iter().chain(sysml_types.iter()) {
        let mut all_supertypes = Vec::new();
        let mut visited = HashSet::new();
        collect_supertypes(
            &type_info.name,
            &direct_supertypes,
            &mut all_supertypes,
            &mut visited,
        );
        hierarchy.insert(type_info.name.clone(), all_supertypes);
    }

    hierarchy
}

/// Recursively collect all supertypes.
fn collect_supertypes(
    type_name: &str,
    direct_map: &HashMap<String, Vec<String>>,
    result: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    if visited.contains(type_name) {
        return;
    }
    visited.insert(type_name.to_string());

    if let Some(direct) = direct_map.get(type_name) {
        for supertype in direct {
            if !result.contains(supertype) {
                result.push(supertype.clone());
            }
            collect_supertypes(supertype, direct_map, result, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_type(name: &str, supertypes: &[&str]) -> TypeInfo {
        TypeInfo {
            name: name.to_string(),
            supertypes: supertypes.iter().map(|s| s.to_string()).collect(),
            comment: None,
        }
    }

    #[test]
    fn test_generates_relationship_methods() {
        let kerml = vec![
            make_type("Element", &[]),
            make_type("Type", &["Element"]),
            make_type("Feature", &["Type"]),
            make_type("Classifier", &["Type"]),
            make_type("Relationship", &["Element"]),
            make_type("Specialization", &["Relationship"]),
            make_type("FeatureTyping", &["Specialization"]),
        ];
        let sysml = vec![];

        let code = generate_relationship_methods(&kerml, &sysml);

        // Check method signatures
        assert!(code.contains("pub const fn relationship_source_type(&self)"));
        assert!(code.contains("pub const fn relationship_target_type(&self)"));

        // Check known constraints are applied
        assert!(code.contains("ElementKind::FeatureTyping => Some(ElementKind::Feature)"));
        assert!(code.contains("ElementKind::Specialization => Some(ElementKind::Type)"));

        // Non-relationship types should return None
        assert!(code.contains("_ => None,"));
    }

    #[test]
    fn test_identifies_relationship_subtypes() {
        let kerml = vec![
            make_type("Element", &[]),
            make_type("Relationship", &["Element"]),
            make_type("Annotation", &["Relationship"]),
        ];
        let sysml = vec![];

        let code = generate_relationship_methods(&kerml, &sysml);

        // Annotation should be included as it's a Relationship subtype
        assert!(code.contains("ElementKind::Annotation"));
    }
}
