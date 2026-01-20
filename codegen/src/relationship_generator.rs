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
