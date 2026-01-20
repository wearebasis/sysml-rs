//! Generator for ElementKind type hierarchy methods.
//!
//! This module generates methods on `ElementKind` that expose type hierarchy information:
//! - `supertypes()` - All supertypes in inheritance order
//! - `direct_supertypes()` - Immediate parent types only
//! - `is_subtype_of()` - Subtype checking
//! - Category predicates (`is_definition()`, `is_usage()`, etc.)
//! - Definition↔Usage mappings

use crate::ttl_parser::TypeInfo;
use std::collections::{HashMap, HashSet};

/// Generate all hierarchy-related methods for the ElementKind enum.
///
/// This generates:
/// - `supertypes()` - Returns all supertypes (direct + transitive)
/// - `direct_supertypes()` - Returns only immediate parents
/// - `is_subtype_of()` - Checks if type is a subtype of another
/// - Category predicates: `is_definition()`, `is_usage()`, `is_relationship()`, etc.
/// - `corresponding_usage()` / `corresponding_definition()` - Definition↔Usage pairs
pub fn generate_hierarchy_methods(
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
) -> String {
    let mut output = String::new();

    output.push_str("\n// === Type Hierarchy Methods ===\n\n");

    // Build the type hierarchy
    let hierarchy = build_type_hierarchy(kerml_types, sysml_types);

    // Get all type names (deduplicated)
    let all_types: Vec<&str> = get_all_type_names(kerml_types, sysml_types);

    // Generate supertypes method
    output.push_str(&generate_supertypes_method(&all_types, &hierarchy));

    // Generate direct_supertypes method
    output.push_str(&generate_direct_supertypes_method(&all_types, kerml_types, sysml_types));

    // Generate is_subtype_of method
    output.push_str(&generate_is_subtype_of_method());

    // Generate category predicates
    output.push_str(&generate_category_predicates(&all_types, &hierarchy));

    // Generate definition/usage mappings
    output.push_str(&generate_def_usage_mappings(&all_types));

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
        collect_supertypes(&type_info.name, &direct_supertypes, &mut all_supertypes, &mut visited);
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

/// Get all unique type names from both vocabularies.
fn get_all_type_names<'a>(
    kerml_types: &'a [TypeInfo],
    sysml_types: &'a [TypeInfo],
) -> Vec<&'a str> {
    let mut names: Vec<&str> = kerml_types.iter().map(|t| t.name.as_str()).collect();
    for t in sysml_types {
        if !names.contains(&t.name.as_str()) {
            names.push(&t.name);
        }
    }
    names.sort();
    names
}

/// Generate the `supertypes()` method.
fn generate_supertypes_method(
    all_types: &[&str],
    hierarchy: &HashMap<String, Vec<String>>,
) -> String {
    let mut output = String::new();

    output.push_str("impl ElementKind {\n");
    output.push_str("    /// Returns all supertypes (direct + transitive) in inheritance order.\n");
    output.push_str("    ///\n");
    output.push_str("    /// The supertypes are ordered from most specific to most general.\n");
    output.push_str("    /// For example, `PartUsage.supertypes()` returns `[ItemUsage, OccurrenceUsage, Usage, ...]`.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// let supertypes = ElementKind::Feature.supertypes();\n");
    output.push_str("    /// assert!(supertypes.contains(&ElementKind::Type));\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub fn supertypes(&self) -> &'static [ElementKind] {\n");
    output.push_str("        match self {\n");

    for type_name in all_types {
        let supertypes = hierarchy.get(*type_name).cloned().unwrap_or_default();
        // Filter to only include types that exist in our type list
        let valid_supertypes: Vec<&str> = supertypes
            .iter()
            .filter(|s| all_types.contains(&s.as_str()))
            .map(|s| s.as_str())
            .collect();

        if valid_supertypes.is_empty() {
            output.push_str(&format!(
                "            ElementKind::{} => &[],\n",
                type_name
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => &[\n",
                type_name
            ));
            for supertype in &valid_supertypes {
                output.push_str(&format!(
                    "                ElementKind::{},\n",
                    supertype
                ));
            }
            output.push_str("            ],\n");
        }
    }

    output.push_str("        }\n");
    output.push_str("    }\n\n");
    output.push_str("}\n\n");

    output
}

/// Generate the `direct_supertypes()` method.
fn generate_direct_supertypes_method(
    all_types: &[&str],
    kerml_types: &[TypeInfo],
    sysml_types: &[TypeInfo],
) -> String {
    let mut output = String::new();

    // Build direct supertypes map
    let mut direct_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for type_info in kerml_types.iter().chain(sysml_types.iter()) {
        let valid_supertypes: Vec<&str> = type_info
            .supertypes
            .iter()
            .filter(|s| all_types.contains(&s.as_str()))
            .map(|s| s.as_str())
            .collect();
        direct_map.insert(&type_info.name, valid_supertypes);
    }

    output.push_str("impl ElementKind {\n");
    output.push_str("    /// Returns the direct supertypes (immediate parents) of this element kind.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Unlike `supertypes()`, this only returns immediate parents, not the full\n");
    output.push_str("    /// transitive closure.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// let direct = ElementKind::Feature.direct_supertypes();\n");
    output.push_str("    /// assert!(direct.contains(&ElementKind::Type));\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub fn direct_supertypes(&self) -> &'static [ElementKind] {\n");
    output.push_str("        match self {\n");

    for type_name in all_types {
        let supertypes = direct_map.get(type_name).cloned().unwrap_or_default();

        if supertypes.is_empty() {
            output.push_str(&format!(
                "            ElementKind::{} => &[],\n",
                type_name
            ));
        } else {
            output.push_str(&format!(
                "            ElementKind::{} => &[\n",
                type_name
            ));
            for supertype in &supertypes {
                output.push_str(&format!(
                    "                ElementKind::{},\n",
                    supertype
                ));
            }
            output.push_str("            ],\n");
        }
    }

    output.push_str("        }\n");
    output.push_str("    }\n\n");
    output.push_str("}\n\n");

    output
}

/// Generate the `is_subtype_of()` method.
fn generate_is_subtype_of_method() -> String {
    let mut output = String::new();

    output.push_str("impl ElementKind {\n");
    output.push_str("    /// Check if this type is a subtype of another (including transitively).\n");
    output.push_str("    ///\n");
    output.push_str("    /// Returns `true` if `other` appears anywhere in this type's supertype chain.\n");
    output.push_str("    /// Note: A type is NOT considered a subtype of itself.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert!(ElementKind::Feature.is_subtype_of(ElementKind::Type));\n");
    output.push_str("    /// assert!(ElementKind::Feature.is_subtype_of(ElementKind::Element));\n");
    output.push_str("    /// assert!(!ElementKind::Feature.is_subtype_of(ElementKind::Feature));\n");
    output.push_str("    /// assert!(!ElementKind::Element.is_subtype_of(ElementKind::Feature));\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub fn is_subtype_of(&self, other: ElementKind) -> bool {\n");
    output.push_str("        self.supertypes().contains(&other)\n");
    output.push_str("    }\n\n");
    output.push_str("}\n\n");

    output
}

/// Generate category predicates.
fn generate_category_predicates(
    all_types: &[&str],
    hierarchy: &HashMap<String, Vec<String>>,
) -> String {
    let mut output = String::new();

    // Identify types by category
    let definitions: Vec<&str> = all_types
        .iter()
        .filter(|t| t.ends_with("Definition"))
        .copied()
        .collect();

    let usages: Vec<&str> = all_types
        .iter()
        .filter(|t| t.ends_with("Usage"))
        .copied()
        .collect();

    let relationships: Vec<&str> = all_types
        .iter()
        .filter(|t| {
            **t == "Relationship" || hierarchy.get(**t).map_or(false, |s| s.contains(&"Relationship".to_string()))
        })
        .copied()
        .collect();

    let classifiers: Vec<&str> = all_types
        .iter()
        .filter(|t| {
            **t == "Classifier" || hierarchy.get(**t).map_or(false, |s| s.contains(&"Classifier".to_string()))
        })
        .copied()
        .collect();

    let features: Vec<&str> = all_types
        .iter()
        .filter(|t| {
            **t == "Feature" || hierarchy.get(**t).map_or(false, |s| s.contains(&"Feature".to_string()))
        })
        .copied()
        .collect();

    output.push_str("impl ElementKind {\n");

    // is_definition()
    output.push_str("    /// Returns `true` if this is a Definition type (e.g., PartDefinition, ActionDefinition).\n");
    output.push_str("    ///\n");
    output.push_str("    /// Definition types define reusable element templates that can be instantiated\n");
    output.push_str("    /// as Usage types.\n");
    output.push_str("    pub const fn is_definition(&self) -> bool {\n");
    output.push_str("        matches!(self,\n");
    for (i, def_type) in definitions.iter().enumerate() {
        if i == definitions.len() - 1 {
            output.push_str(&format!("            ElementKind::{}\n", def_type));
        } else {
            output.push_str(&format!("            ElementKind::{} |\n", def_type));
        }
    }
    output.push_str("        )\n");
    output.push_str("    }\n\n");

    // is_usage()
    output.push_str("    /// Returns `true` if this is a Usage type (e.g., PartUsage, ActionUsage).\n");
    output.push_str("    ///\n");
    output.push_str("    /// Usage types are instantiations or references to Definition types.\n");
    output.push_str("    pub const fn is_usage(&self) -> bool {\n");
    output.push_str("        matches!(self,\n");
    for (i, usage_type) in usages.iter().enumerate() {
        if i == usages.len() - 1 {
            output.push_str(&format!("            ElementKind::{}\n", usage_type));
        } else {
            output.push_str(&format!("            ElementKind::{} |\n", usage_type));
        }
    }
    output.push_str("        )\n");
    output.push_str("    }\n\n");

    // is_relationship()
    output.push_str("    /// Returns `true` if this is a Relationship type or any of its subtypes.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Relationship types connect elements together (e.g., Specialization, FeatureTyping).\n");
    output.push_str("    pub const fn is_relationship(&self) -> bool {\n");
    output.push_str("        matches!(self,\n");
    for (i, rel_type) in relationships.iter().enumerate() {
        if i == relationships.len() - 1 {
            output.push_str(&format!("            ElementKind::{}\n", rel_type));
        } else {
            output.push_str(&format!("            ElementKind::{} |\n", rel_type));
        }
    }
    output.push_str("        )\n");
    output.push_str("    }\n\n");

    // is_classifier()
    output.push_str("    /// Returns `true` if this is a Classifier type or any of its subtypes.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Classifiers are Types that classify their instances (e.g., Class, DataType).\n");
    output.push_str("    pub const fn is_classifier(&self) -> bool {\n");
    output.push_str("        matches!(self,\n");
    for (i, cls_type) in classifiers.iter().enumerate() {
        if i == classifiers.len() - 1 {
            output.push_str(&format!("            ElementKind::{}\n", cls_type));
        } else {
            output.push_str(&format!("            ElementKind::{} |\n", cls_type));
        }
    }
    output.push_str("        )\n");
    output.push_str("    }\n\n");

    // is_feature()
    output.push_str("    /// Returns `true` if this is a Feature type or any of its subtypes.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Features are typed structural and/or behavioral elements.\n");
    output.push_str("    pub const fn is_feature(&self) -> bool {\n");
    output.push_str("        matches!(self,\n");
    for (i, feat_type) in features.iter().enumerate() {
        if i == features.len() - 1 {
            output.push_str(&format!("            ElementKind::{}\n", feat_type));
        } else {
            output.push_str(&format!("            ElementKind::{} |\n", feat_type));
        }
    }
    output.push_str("        )\n");
    output.push_str("    }\n\n");

    output.push_str("}\n\n");

    output
}

/// Generate Definition↔Usage mapping methods.
fn generate_def_usage_mappings(all_types: &[&str]) -> String {
    let mut output = String::new();

    // Find matching pairs
    let mut pairs: Vec<(&str, &str)> = Vec::new();

    for def_type in all_types.iter().filter(|t| t.ends_with("Definition")) {
        // Try to find matching Usage type
        let base_name = def_type.strip_suffix("Definition").unwrap();
        let usage_name = format!("{}Usage", base_name);
        if all_types.contains(&usage_name.as_str()) {
            pairs.push((def_type, all_types.iter().find(|&&t| t == usage_name).unwrap()));
        }
    }

    output.push_str("impl ElementKind {\n");

    // corresponding_usage()
    output.push_str("    /// For Definition types, returns the corresponding Usage type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// For example, `PartDefinition.corresponding_usage()` returns `Some(PartUsage)`.\n");
    output.push_str("    /// Returns `None` for non-Definition types or Definitions without a matching Usage.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::PartDefinition.corresponding_usage(),\n");
    output.push_str("    ///     Some(ElementKind::PartUsage)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.corresponding_usage(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn corresponding_usage(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");
    for (def_type, usage_type) in &pairs {
        output.push_str(&format!(
            "            ElementKind::{} => Some(ElementKind::{}),\n",
            def_type, usage_type
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    // corresponding_definition()
    output.push_str("    /// For Usage types, returns the corresponding Definition type.\n");
    output.push_str("    ///\n");
    output.push_str("    /// For example, `PartUsage.corresponding_definition()` returns `Some(PartDefinition)`.\n");
    output.push_str("    /// Returns `None` for non-Usage types or Usages without a matching Definition.\n");
    output.push_str("    ///\n");
    output.push_str("    /// # Examples\n");
    output.push_str("    ///\n");
    output.push_str("    /// ```\n");
    output.push_str("    /// use sysml_core::ElementKind;\n");
    output.push_str("    ///\n");
    output.push_str("    /// assert_eq!(\n");
    output.push_str("    ///     ElementKind::PartUsage.corresponding_definition(),\n");
    output.push_str("    ///     Some(ElementKind::PartDefinition)\n");
    output.push_str("    /// );\n");
    output.push_str("    /// assert_eq!(ElementKind::Element.corresponding_definition(), None);\n");
    output.push_str("    /// ```\n");
    output.push_str("    pub const fn corresponding_definition(&self) -> Option<ElementKind> {\n");
    output.push_str("        match self {\n");
    for (def_type, usage_type) in &pairs {
        output.push_str(&format!(
            "            ElementKind::{} => Some(ElementKind::{}),\n",
            usage_type, def_type
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    output.push_str("}\n");

    output
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
    fn test_build_type_hierarchy() {
        let kerml = vec![
            make_type("Element", &[]),
            make_type("Namespace", &["Element"]),
            make_type("Type", &["Namespace"]),
            make_type("Feature", &["Type"]),
        ];
        let sysml = vec![];

        let hierarchy = build_type_hierarchy(&kerml, &sysml);

        // Element has no supertypes
        assert_eq!(hierarchy.get("Element").unwrap().len(), 0);

        // Namespace has Element
        assert!(hierarchy.get("Namespace").unwrap().contains(&"Element".to_string()));

        // Type has Namespace and Element
        let type_supers = hierarchy.get("Type").unwrap();
        assert!(type_supers.contains(&"Namespace".to_string()));
        assert!(type_supers.contains(&"Element".to_string()));

        // Feature has Type, Namespace, and Element
        let feature_supers = hierarchy.get("Feature").unwrap();
        assert!(feature_supers.contains(&"Type".to_string()));
        assert!(feature_supers.contains(&"Namespace".to_string()));
        assert!(feature_supers.contains(&"Element".to_string()));
    }

    #[test]
    fn test_generates_methods() {
        let kerml = vec![
            make_type("Element", &[]),
            make_type("Type", &["Element"]),
            make_type("Feature", &["Type"]),
            make_type("Relationship", &["Element"]),
            make_type("Specialization", &["Relationship"]),
            make_type("Classifier", &["Type"]),
        ];
        let sysml = vec![
            make_type("PartDefinition", &["Classifier"]),
            make_type("PartUsage", &["Feature"]),
        ];

        let code = generate_hierarchy_methods(&kerml, &sysml);

        // Check method signatures
        assert!(code.contains("pub fn supertypes(&self)"));
        assert!(code.contains("pub fn direct_supertypes(&self)"));
        assert!(code.contains("pub fn is_subtype_of(&self"));
        assert!(code.contains("pub const fn is_definition(&self)"));
        assert!(code.contains("pub const fn is_usage(&self)"));
        assert!(code.contains("pub const fn is_relationship(&self)"));
        assert!(code.contains("pub const fn is_classifier(&self)"));
        assert!(code.contains("pub const fn is_feature(&self)"));
        assert!(code.contains("pub const fn corresponding_usage(&self)"));
        assert!(code.contains("pub const fn corresponding_definition(&self)"));

        // Check some specific entries
        assert!(code.contains("ElementKind::PartDefinition => Some(ElementKind::PartUsage)"));
        assert!(code.contains("ElementKind::PartUsage => Some(ElementKind::PartDefinition)"));
    }

    #[test]
    fn test_handles_cycles() {
        // Even with cycles, should not infinite loop
        let kerml = vec![
            make_type("A", &["B"]),
            make_type("B", &["A"]), // Cycle
        ];
        let sysml = vec![];

        let hierarchy = build_type_hierarchy(&kerml, &sysml);

        // Should complete without hanging
        assert!(hierarchy.contains_key("A"));
        assert!(hierarchy.contains_key("B"));
    }
}
