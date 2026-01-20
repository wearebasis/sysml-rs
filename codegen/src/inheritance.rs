//! Property inheritance resolution for SysML/KerML types.
//!
//! This module resolves property inheritance by walking the type hierarchy
//! and collecting properties from all supertypes.

use crate::shapes_parser::{PropertyInfo, ShapeInfo};
use crate::ttl_parser::TypeInfo;
use std::collections::{HashMap, HashSet};

/// Build a type hierarchy map from vocab type information.
///
/// Returns a map from type name to its direct supertypes.
pub fn build_type_hierarchy(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
) -> HashMap<String, Vec<String>> {
    let mut hierarchy: HashMap<String, Vec<String>> = HashMap::new();

    for type_info in kerml_types.iter().chain(sysml_types.iter()) {
        hierarchy.insert(type_info.name.clone(), type_info.supertypes.clone());
    }

    hierarchy
}

/// Get all supertypes of a type (including transitive supertypes).
fn get_all_supertypes(
    type_name: &str,
    hierarchy: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> Vec<String> {
    if visited.contains(type_name) {
        return Vec::new();
    }
    visited.insert(type_name.to_string());

    let mut all_supertypes = Vec::new();

    if let Some(direct_supertypes) = hierarchy.get(type_name) {
        for supertype in direct_supertypes {
            all_supertypes.push(supertype.clone());
            // Recursively get supertypes of this supertype
            let transitive = get_all_supertypes(supertype, hierarchy, visited);
            all_supertypes.extend(transitive);
        }
    }

    all_supertypes
}

/// Resolved properties for a type, including inherited ones.
#[derive(Debug, Clone)]
pub struct ResolvedShape {
    /// The element type name.
    pub element_type: String,
    /// All properties for this type (own + inherited).
    pub properties: Vec<PropertyInfo>,
    /// Direct supertypes.
    pub supertypes: Vec<String>,
    /// Description from the shape.
    pub description: Option<String>,
}

/// Resolve property inheritance for all shapes.
///
/// For each shape, collects properties from all supertypes in the hierarchy.
/// Properties from subtypes override properties from supertypes with the same name.
pub fn resolve_inheritance(
    shapes: &[ShapeInfo],
    type_hierarchy: &HashMap<String, Vec<String>>,
) -> HashMap<String, ResolvedShape> {
    // Build a map from element type to shape
    let shape_by_type: HashMap<&str, &ShapeInfo> = shapes
        .iter()
        .map(|s| (s.element_type.as_str(), s))
        .collect();

    let mut resolved: HashMap<String, ResolvedShape> = HashMap::new();

    for shape in shapes {
        let mut all_props: HashMap<String, PropertyInfo> = HashMap::new();
        let mut visited = HashSet::new();

        // Get all supertypes
        let supertypes = get_all_supertypes(&shape.element_type, type_hierarchy, &mut visited);

        // Process supertypes in reverse order (most general first)
        // so that more specific types override general ones
        for supertype in supertypes.iter().rev() {
            if let Some(super_shape) = shape_by_type.get(supertype.as_str()) {
                for prop in &super_shape.properties {
                    all_props.insert(prop.name.clone(), prop.clone());
                }
            }
        }

        // Add own properties (override inherited)
        for prop in &shape.properties {
            all_props.insert(prop.name.clone(), prop.clone());
        }

        // Sort properties by name for consistent output
        let mut properties: Vec<PropertyInfo> = all_props.into_values().collect();
        properties.sort_by(|a, b| a.name.cmp(&b.name));

        // Get direct supertypes
        let direct_supertypes = type_hierarchy
            .get(&shape.element_type)
            .cloned()
            .unwrap_or_default();

        resolved.insert(
            shape.element_type.clone(),
            ResolvedShape {
                element_type: shape.element_type.clone(),
                properties,
                supertypes: direct_supertypes,
                description: shape.description.clone(),
            },
        );
    }

    resolved
}

/// Find the common base properties that are shared by all elements.
///
/// Returns properties that are defined on the Element type.
pub fn get_element_base_properties(resolved: &HashMap<String, ResolvedShape>) -> Vec<PropertyInfo> {
    if let Some(element_shape) = resolved.get("Element") {
        element_shape.properties.clone()
    } else {
        Vec::new()
    }
}

/// Count total unique properties across all shapes.
pub fn count_total_properties(resolved: &HashMap<String, ResolvedShape>) -> usize {
    let mut unique_props: HashSet<&str> = HashSet::new();
    for shape in resolved.values() {
        for prop in &shape.properties {
            unique_props.insert(&prop.name);
        }
    }
    unique_props.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes_parser::{Cardinality, PropertyType};

    fn make_type(name: &str, supertypes: &[&str]) -> TypeInfo {
        TypeInfo {
            name: name.to_string(),
            supertypes: supertypes.iter().map(|s| s.to_string()).collect(),
            comment: None,
        }
    }

    fn make_prop(name: &str) -> PropertyInfo {
        PropertyInfo {
            name: name.to_string(),
            cardinality: Cardinality::ZeroOrOne,
            property_type: PropertyType::Any,
            read_only: false,
            description: None,
        }
    }

    fn make_shape(element_type: &str, props: &[&str]) -> ShapeInfo {
        ShapeInfo {
            element_type: element_type.to_string(),
            shape_name: format!("{}Shape", element_type),
            properties: props.iter().map(|p| make_prop(p)).collect(),
            property_refs: Vec::new(),
            description: None,
        }
    }

    #[test]
    fn test_build_type_hierarchy() {
        let kerml = vec![
            make_type("Element", &[]),
            make_type("Namespace", &["Element"]),
            make_type("Type", &["Namespace"]),
        ];
        let sysml = vec![
            make_type("Usage", &["Feature"]),
            make_type("PartUsage", &["ItemUsage"]),
        ];

        let hierarchy = build_type_hierarchy(&kerml, &sysml);

        assert_eq!(hierarchy.get("Element"), Some(&vec![]));
        assert_eq!(
            hierarchy.get("Namespace"),
            Some(&vec!["Element".to_string()])
        );
        assert_eq!(hierarchy.get("Type"), Some(&vec!["Namespace".to_string()]));
        assert_eq!(hierarchy.get("Usage"), Some(&vec!["Feature".to_string()]));
    }

    #[test]
    fn test_get_all_supertypes() {
        let mut hierarchy = HashMap::new();
        hierarchy.insert("Element".to_string(), vec![]);
        hierarchy.insert("Namespace".to_string(), vec!["Element".to_string()]);
        hierarchy.insert("Type".to_string(), vec!["Namespace".to_string()]);
        hierarchy.insert("Feature".to_string(), vec!["Type".to_string()]);

        let mut visited = HashSet::new();
        let supertypes = get_all_supertypes("Feature", &hierarchy, &mut visited);

        assert!(supertypes.contains(&"Type".to_string()));
        assert!(supertypes.contains(&"Namespace".to_string()));
        assert!(supertypes.contains(&"Element".to_string()));
    }

    #[test]
    fn test_resolve_inheritance() {
        let shapes = vec![
            make_shape("Element", &["elementId", "name"]),
            make_shape("Namespace", &["member", "ownedMember"]),
            make_shape("Type", &["feature", "ownedFeature"]),
        ];

        let mut hierarchy = HashMap::new();
        hierarchy.insert("Element".to_string(), vec![]);
        hierarchy.insert("Namespace".to_string(), vec!["Element".to_string()]);
        hierarchy.insert("Type".to_string(), vec!["Namespace".to_string()]);

        let resolved = resolve_inheritance(&shapes, &hierarchy);

        // Element should have only its own properties
        let element = &resolved["Element"];
        assert_eq!(element.properties.len(), 2);

        // Namespace should have Element's properties + its own
        let namespace = &resolved["Namespace"];
        assert_eq!(namespace.properties.len(), 4); // elementId, name, member, ownedMember

        // Type should have all properties from the hierarchy
        let type_shape = &resolved["Type"];
        assert_eq!(type_shape.properties.len(), 6);
        let prop_names: Vec<&str> = type_shape.properties.iter().map(|p| p.name.as_str()).collect();
        assert!(prop_names.contains(&"elementId"));
        assert!(prop_names.contains(&"name"));
        assert!(prop_names.contains(&"member"));
        assert!(prop_names.contains(&"feature"));
    }

    #[test]
    fn test_property_override() {
        // When a subtype defines a property with the same name,
        // it should override the supertype's property
        let shapes = vec![
            make_shape("Base", &["prop1"]),
            ShapeInfo {
                element_type: "Derived".to_string(),
                shape_name: "DerivedShape".to_string(),
                properties: vec![PropertyInfo {
                    name: "prop1".to_string(),
                    cardinality: Cardinality::ExactlyOne, // Different from base
                    property_type: PropertyType::Bool,
                    read_only: false,
                    description: Some("Overridden".to_string()),
                }],
                property_refs: Vec::new(),
                description: None,
            },
        ];

        let mut hierarchy = HashMap::new();
        hierarchy.insert("Base".to_string(), vec![]);
        hierarchy.insert("Derived".to_string(), vec!["Base".to_string()]);

        let resolved = resolve_inheritance(&shapes, &hierarchy);

        let derived = &resolved["Derived"];
        assert_eq!(derived.properties.len(), 1);
        assert_eq!(derived.properties[0].cardinality, Cardinality::ExactlyOne);
        assert_eq!(
            derived.properties[0].description,
            Some("Overridden".to_string())
        );
    }
}
