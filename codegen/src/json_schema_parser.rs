//! Parser for JSON Schema files from the SysML v2 specification.
//!
//! This module extracts:
//! - Enum values from `*Kind.json` files
//! - Relationship source/target type constraints from `$comment` fields

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parsed enum from JSON schema.
#[derive(Debug, Clone)]
pub struct JsonEnumInfo {
    pub name: String,
    pub values: Vec<String>,
}

/// Relationship constraint parsed from JSON schema.
#[derive(Debug, Clone)]
pub struct JsonRelationshipConstraint {
    pub relationship_type: String,
    pub source_type: String,
    pub target_type: String,
}

/// Intermediate structure for parsing JSON schema.
#[derive(Debug, Deserialize)]
struct JsonSchema {
    title: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
    #[serde(rename = "enum")]
    enum_values: Option<Vec<String>>,
    #[serde(rename = "anyOf")]
    any_of: Option<Vec<JsonSchemaVariant>>,
    properties: Option<HashMap<String, PropertyDef>>,
}

#[derive(Debug, Deserialize)]
struct JsonSchemaVariant {
    #[allow(dead_code)]
    #[serde(rename = "type")]
    type_field: Option<String>,
    properties: Option<HashMap<String, PropertyDef>>,
}

#[derive(Debug, Deserialize)]
struct PropertyDef {
    #[serde(rename = "$comment")]
    comment: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "$ref")]
    ref_field: Option<String>,
    #[serde(rename = "oneOf")]
    one_of: Option<Vec<PropertyVariant>>,
}

#[derive(Debug, Deserialize)]
struct PropertyVariant {
    #[serde(rename = "$comment")]
    comment: Option<String>,
}

/// Known property name mappings for relationship source/target.
///
/// Maps relationship type to (source_property_name, target_property_name).
/// These are stable from the spec and define which JSON property represents
/// the source vs target of the relationship.
fn get_source_target_properties() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut map = HashMap::new();

    // Specialization hierarchy
    map.insert("Specialization", ("specific", "general"));
    map.insert("FeatureTyping", ("typedFeature", "type"));
    map.insert("Subsetting", ("subsettingFeature", "subsettedFeature"));
    map.insert("Redefinition", ("redefiningFeature", "redefinedFeature"));
    map.insert("ReferenceSubsetting", ("referencingFeature", "referencedFeature"));
    map.insert("Subclassification", ("subclassifier", "superclassifier"));
    map.insert("ConjugatedPortTyping", ("typedFeature", "conjugatedPortDefinition"));
    map.insert("CrossSubsetting", ("crossingFeature", "crossedFeature"));

    // Type-level relationships
    map.insert("Conjugation", ("conjugatedType", "originalType"));
    map.insert("Disjoining", ("disjoiningType", "disjoinedType"));
    map.insert("Differencing", ("differencingType", ""));  // target inherited
    map.insert("Intersecting", ("intersectingType", ""));   // target inherited
    map.insert("Unioning", ("unioningType", ""));           // target inherited
    map.insert("PortConjugation", ("conjugatedPortDefinition", "originalPortDefinition"));

    // Feature relationships
    map.insert("FeatureChaining", ("chainingFeature", ""));  // part of chain
    map.insert("FeatureInverting", ("featureInverted", "invertingFeature"));
    map.insert("TypeFeaturing", ("featureOfType", "featuringType"));

    // Membership hierarchy
    map.insert("Membership", ("membershipOwningNamespace", "memberElement"));
    map.insert("OwningMembership", ("membershipOwningNamespace", "ownedMemberElement"));
    map.insert("FeatureMembership", ("owningType", "ownedMemberFeature"));
    map.insert("EndFeatureMembership", ("owningType", "ownedMemberFeature"));
    map.insert("ParameterMembership", ("owningType", "ownedMemberParameter"));
    map.insert("ResultExpressionMembership", ("owningType", "ownedResultExpression"));
    map.insert("ReturnParameterMembership", ("owningType", "ownedMemberParameter"));
    map.insert("ObjectiveMembership", ("owningType", "ownedObjectiveRequirement"));
    map.insert("SubjectMembership", ("owningType", "ownedSubjectParameter"));
    map.insert("ActorMembership", ("owningType", "ownedActorParameter"));
    map.insert("StakeholderMembership", ("owningType", "ownedStakeholderParameter"));
    map.insert("RequirementConstraintMembership", ("owningType", "ownedConstraint"));
    map.insert("RequirementVerificationMembership", ("owningType", "ownedRequirement"));
    map.insert("FramedConcernMembership", ("owningType", "ownedConcern"));
    map.insert("ViewRenderingMembership", ("owningType", "ownedRendering"));
    map.insert("ElementFilterMembership", ("owningType", "condition"));
    map.insert("VariantMembership", ("owningType", "ownedVariantUsage"));
    map.insert("StateSubactionMembership", ("owningType", "action"));
    map.insert("TransitionFeatureMembership", ("owningType", "transitionFeature"));

    // Import relationships
    map.insert("Import", ("importOwningNamespace", "importedElement"));
    map.insert("MembershipImport", ("importOwningNamespace", "importedMembership"));
    map.insert("NamespaceImport", ("importOwningNamespace", "importedNamespace"));

    // Annotation
    map.insert("Annotation", ("annotatingElement", "annotatedElement"));

    // Dependency
    map.insert("Dependency", ("client", "supplier"));

    // Expose
    map.insert("Expose", ("importOwningNamespace", "importedMembership"));

    // Connection/flow relationships
    map.insert("ItemFlow", ("itemFlowEnd", "itemFlowEnd")); // bidirectional
    map.insert("FlowConnectionUsage", ("", "")); // uses ends
    map.insert("SuccessionFlowConnectionUsage", ("", ""));
    map.insert("Succession", ("", "")); // uses ends
    map.insert("ConnectionUsage", ("", "")); // uses ends
    map.insert("InterfaceUsage", ("", "")); // uses ends
    map.insert("BindingConnectorAsUsage", ("", "")); // uses ends
    map.insert("AllocationUsage", ("", "")); // uses ends

    map
}

/// Extract the type name from a SysML URL.
/// e.g., "https://www.omg.org/spec/SysML/20250201/Feature" -> "Feature"
fn extract_type_from_url(url: &str) -> Option<String> {
    url.rsplit('/').next().map(|s| s.to_string())
}

/// Parse a JSON enum file (e.g., FeatureDirectionKind.json).
pub fn parse_enum_json(content: &str) -> Option<JsonEnumInfo> {
    let schema: JsonSchema = serde_json::from_str(content).ok()?;

    // Must be a string type with enum values
    if schema.type_field.as_deref() != Some("string") {
        return None;
    }

    let values = schema.enum_values?;
    let name = schema.title?;

    Some(JsonEnumInfo { name, values })
}

/// Parse a relationship JSON schema to extract source/target constraints.
pub fn parse_relationship_json(
    type_name: &str,
    content: &str,
) -> Option<JsonRelationshipConstraint> {
    let property_map = get_source_target_properties();

    // Get the source/target property names for this relationship type
    let (source_prop, target_prop) = property_map.get(type_name)?;

    // Skip relationships without specific property mappings
    if source_prop.is_empty() && target_prop.is_empty() {
        return None;
    }

    let schema: JsonSchema = serde_json::from_str(content).ok()?;

    // Get properties - either directly or from first anyOf variant
    let properties = if let Some(props) = schema.properties {
        props
    } else if let Some(any_of) = schema.any_of {
        any_of.into_iter()
            .find_map(|v| v.properties)?
    } else {
        return None;
    };

    // Extract source type
    let source_type = if !source_prop.is_empty() {
        extract_type_from_property(&properties, source_prop)
            .unwrap_or_else(|| "Element".to_string())
    } else {
        "Element".to_string()
    };

    // Extract target type
    let target_type = if !target_prop.is_empty() {
        extract_type_from_property(&properties, target_prop)
            .unwrap_or_else(|| "Element".to_string())
    } else {
        "Element".to_string()
    };

    Some(JsonRelationshipConstraint {
        relationship_type: type_name.to_string(),
        source_type,
        target_type,
    })
}

/// Extract the type from a property's $comment field.
fn extract_type_from_property(
    properties: &HashMap<String, PropertyDef>,
    prop_name: &str,
) -> Option<String> {
    let prop = properties.get(prop_name)?;

    // Direct $comment
    if let Some(ref comment) = prop.comment {
        return extract_type_from_url(comment);
    }

    // Check oneOf variants for $comment
    if let Some(ref one_of) = prop.one_of {
        for variant in one_of {
            if let Some(ref comment) = variant.comment {
                return extract_type_from_url(comment);
            }
        }
    }

    None
}

/// Parse all enum JSON files from a schema directory.
///
/// Discovers all *Kind.json files in the directory and parses them.
pub fn parse_all_enums_from_json(schema_dir: &Path) -> Vec<JsonEnumInfo> {
    // Discover all *Kind.json files in the directory
    let mut enums = Vec::new();

    if let Ok(entries) = fs::read_dir(schema_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                // Only process files ending with Kind.json
                if filename.ends_with("Kind.json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(enum_info) = parse_enum_json(&content) {
                            enums.push(enum_info);
                        }
                    }
                }
            }
        }
    }

    // Sort by name for consistent output
    enums.sort_by(|a, b| a.name.cmp(&b.name));
    enums
}

/// Get the list of expected enum types.
///
/// These are the known enum types in the SysML v2 specification.
/// Used for validation to ensure we're not missing any.
pub fn expected_enum_types() -> Vec<&'static str> {
    vec![
        "FeatureDirectionKind",
        "PortionKind",
        "RequirementConstraintKind",
        "StateSubactionKind",
        "TransitionFeatureKind",
        "TriggerKind",
        "VisibilityKind",
    ]
}

/// Parse relationship constraints from JSON schemas.
///
/// Takes a list of relationship type names and the schema directory,
/// returns parsed constraints for each relationship.
pub fn parse_relationship_constraints_from_json(
    schema_dir: &Path,
    relationship_types: &[&str],
) -> Vec<JsonRelationshipConstraint> {
    relationship_types.iter().filter_map(|rel_type| {
        let path = schema_dir.join(format!("{}.json", rel_type));
        let content = fs::read_to_string(&path).ok()?;
        parse_relationship_json(rel_type, &content)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enum_json() {
        let content = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://www.omg.org/spec/SysML/20250201/FeatureDirectionKind",
            "title": "FeatureDirectionKind",
            "type": "string",
            "enum": ["in", "inout", "out"]
        }"#;

        let result = parse_enum_json(content).unwrap();
        assert_eq!(result.name, "FeatureDirectionKind");
        assert_eq!(result.values, vec!["in", "inout", "out"]);
    }

    #[test]
    fn test_parse_relationship_json() {
        let content = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://www.omg.org/spec/SysML/20250201/FeatureTyping",
            "title": "FeatureTyping",
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "typedFeature": {
                            "$ref": "https://www.omg.org/spec/SysML/20250201/Identified",
                            "$comment": "https://www.omg.org/spec/SysML/20250201/Feature"
                        },
                        "type": {
                            "$ref": "https://www.omg.org/spec/SysML/20250201/Identified",
                            "$comment": "https://www.omg.org/spec/SysML/20250201/Type"
                        }
                    }
                }
            ]
        }"#;

        let result = parse_relationship_json("FeatureTyping", content).unwrap();
        assert_eq!(result.relationship_type, "FeatureTyping");
        assert_eq!(result.source_type, "Feature");
        assert_eq!(result.target_type, "Type");
    }

    #[test]
    fn test_parse_subclassification() {
        let content = r#"{
            "title": "Subclassification",
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "subclassifier": {
                            "$ref": "https://www.omg.org/spec/SysML/20250201/Identified",
                            "$comment": "https://www.omg.org/spec/SysML/20250201/Classifier"
                        },
                        "superclassifier": {
                            "$ref": "https://www.omg.org/spec/SysML/20250201/Identified",
                            "$comment": "https://www.omg.org/spec/SysML/20250201/Classifier"
                        }
                    }
                }
            ]
        }"#;

        let result = parse_relationship_json("Subclassification", content).unwrap();
        assert_eq!(result.source_type, "Classifier");
        assert_eq!(result.target_type, "Classifier");
    }

    #[test]
    fn test_extract_type_from_url() {
        assert_eq!(
            extract_type_from_url("https://www.omg.org/spec/SysML/20250201/Feature"),
            Some("Feature".to_string())
        );
        assert_eq!(
            extract_type_from_url("https://www.omg.org/spec/SysML/20250201/Type"),
            Some("Type".to_string())
        );
    }
}
