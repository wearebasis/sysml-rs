//! Validation code generator.
//!
//! This module generates validation methods for property accessors
//! based on shape constraints.

use crate::inheritance::ResolvedShape;
use crate::shapes_parser::{Cardinality, PropertyInfo, PropertyType};
use std::collections::HashMap;

/// Convert a type name to the accessor struct name.
fn accessor_struct_name(element_type: &str) -> String {
    format!("{}Props", element_type)
}

/// Get the expected type name for error messages.
fn expected_type_name(prop_type: &PropertyType) -> &'static str {
    match prop_type {
        PropertyType::Bool => "bool",
        PropertyType::String => "string",
        PropertyType::DateTime => "datetime",
        PropertyType::ElementRef(_) => "ElementId",
        PropertyType::Any => "any",
    }
}

/// Generate validation methods for all shapes.
pub fn generate_validation_methods(resolved: &HashMap<String, ResolvedShape>) -> String {
    let mut output = String::new();

    // Sort shapes for consistent output
    let mut shapes: Vec<_> = resolved.values().collect();
    shapes.sort_by(|a, b| a.element_type.cmp(&b.element_type));

    for shape in shapes {
        generate_validation_impl(&mut output, shape);
    }

    output
}

/// Generate validation impl for a single shape.
fn generate_validation_impl(output: &mut String, shape: &ResolvedShape) {
    let struct_name = accessor_struct_name(&shape.element_type);

    output.push_str(&format!("impl<'a> {}<'a> {{\n", struct_name));
    output.push_str("    /// Validate this element against its shape constraints.\n");
    output.push_str("    ///\n");
    output.push_str("    /// Returns a list of validation errors, empty if valid.\n");
    output.push_str("    pub fn validate(&self) -> ValidationResult {\n");
    output.push_str("        let mut result = ValidationResult::new();\n\n");

    // Generate validation for each property
    for prop in &shape.properties {
        generate_property_validation(output, prop);
    }

    output.push_str("        result\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
}

/// Generate validation code for a single property.
fn generate_property_validation(output: &mut String, prop: &PropertyInfo) {
    let prop_name = &prop.name;

    match prop.cardinality {
        Cardinality::ExactlyOne => {
            // Required property - check it exists and has correct type
            if matches!(prop.property_type, PropertyType::Bool) {
                // Booleans are always present (default to false), no validation needed
                output.push_str(&format!(
                    "        // {} is a boolean, always present\n\n",
                    prop_name
                ));
            } else {
                output.push_str(&format!(
                    "        // Check required property: {}\n",
                    prop_name
                ));
                output.push_str(&format!(
                    "        if self.0.props.get(\"{}\").is_none() {{\n",
                    prop_name
                ));
                output.push_str(&format!(
                    "            result.add_error(ValidationError::missing_required(\"{}\"));\n",
                    prop_name
                ));
                output.push_str("        }\n");

                // Type check
                generate_type_check(output, prop);

                // MaxCardinality check - exactly one means at most 1 value
                generate_max_cardinality_check(output, prop_name);
                output.push_str("\n");
            }
        }
        Cardinality::ZeroOrOne => {
            // Optional property - check type if present
            output.push_str(&format!(
                "        // Check optional property type: {}\n",
                prop_name
            ));
            generate_type_check(output, prop);

            // MaxCardinality check - zero or one means at most 1 value
            generate_max_cardinality_check(output, prop_name);
            output.push_str("\n");
        }
        Cardinality::OneOrMany => {
            // Must have at least one value
            output.push_str(&format!(
                "        // Check one-or-many property: {}\n",
                prop_name
            ));
            output.push_str(&format!(
                "        if let Some(v) = self.0.props.get(\"{}\") {{\n",
                prop_name
            ));
            output.push_str("            if let Some(list) = v.as_list() {\n");
            output.push_str("                if list.is_empty() {\n");
            output.push_str(&format!(
                "                    result.add_error(ValidationError::min_cardinality(\"{}\"));\n",
                prop_name
            ));
            output.push_str("                }\n");
            output.push_str("            } else {\n");
            output.push_str("                // Single value is acceptable\n");
            output.push_str("            }\n");
            output.push_str("        } else {\n");
            output.push_str(&format!(
                "            result.add_error(ValidationError::min_cardinality(\"{}\"));\n",
                prop_name
            ));
            output.push_str("        }\n\n");
        }
        Cardinality::ZeroOrMany => {
            // No cardinality constraints, just type check
            output.push_str(&format!(
                "        // Check zero-or-many property type: {}\n",
                prop_name
            ));
            generate_list_type_check(output, prop);
            output.push_str("\n");
        }
    }
}

/// Generate type check code for a property value.
fn generate_type_check(output: &mut String, prop: &PropertyInfo) {
    let prop_name = &prop.name;
    let expected = expected_type_name(&prop.property_type);

    output.push_str(&format!(
        "        if let Some(v) = self.0.props.get(\"{}\") {{\n",
        prop_name
    ));

    match &prop.property_type {
        PropertyType::Bool => {
            output.push_str("            if v.as_bool().is_none() && !v.is_null() {\n");
        }
        PropertyType::String | PropertyType::DateTime => {
            output.push_str("            if v.as_str().is_none() && !v.is_null() {\n");
        }
        PropertyType::ElementRef(_) => {
            output.push_str("            if v.as_ref().is_none() && !v.is_null() {\n");
        }
        PropertyType::Any => {
            // Any type is valid
            output.push_str("            if false { // Any type is valid\n");
        }
    }

    output.push_str(&format!(
        "                result.add_error(ValidationError::wrong_type(\"{}\", \"{}\", v.type_name()));\n",
        prop_name, expected
    ));
    output.push_str("            }\n");
    output.push_str("        }\n");
}

/// Generate type check for list property values.
fn generate_list_type_check(output: &mut String, prop: &PropertyInfo) {
    let prop_name = &prop.name;
    let expected = expected_type_name(&prop.property_type);

    output.push_str(&format!(
        "        if let Some(v) = self.0.props.get(\"{}\") {{\n",
        prop_name
    ));
    output.push_str("            if let Some(list) = v.as_list() {\n");
    output.push_str("                for item in list {\n");

    match &prop.property_type {
        PropertyType::Bool => {
            output.push_str("                    if item.as_bool().is_none() && !item.is_null() {\n");
        }
        PropertyType::String | PropertyType::DateTime => {
            output.push_str("                    if item.as_str().is_none() && !item.is_null() {\n");
        }
        PropertyType::ElementRef(_) => {
            output.push_str("                    if item.as_ref().is_none() && !item.is_null() {\n");
        }
        PropertyType::Any => {
            output.push_str("                    if false { // Any type is valid\n");
        }
    }

    output.push_str(&format!(
        "                        result.add_error(ValidationError::wrong_type(\"{}\", \"{}\", item.type_name()));\n",
        prop_name, expected
    ));
    output.push_str("                        break; // Report only first type error\n");
    output.push_str("                    }\n");
    output.push_str("                }\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
}

/// Generate MaxCardinality check code for ZeroOrOne and ExactlyOne properties.
///
/// Validates that if the property value is a list, it has at most 1 element.
fn generate_max_cardinality_check(output: &mut String, prop_name: &str) {
    output.push_str(&format!(
        "        if let Some(sysml_meta::Value::List(list)) = self.0.props.get(\"{}\") {{\n",
        prop_name
    ));
    output.push_str("            if list.len() > 1 {\n");
    output.push_str(&format!(
        "                result.add_error(ValidationError::max_cardinality(\"{}\"));\n",
        prop_name
    ));
    output.push_str("            }\n");
    output.push_str("        }\n");
}

/// Generate validation methods only for shapes that exist in ElementKind.
pub fn generate_validation_methods_filtered(
    resolved: &HashMap<String, ResolvedShape>,
    valid_element_kinds: &[String],
) -> String {
    let valid_set: std::collections::HashSet<&str> =
        valid_element_kinds.iter().map(|s| s.as_str()).collect();

    let filtered: HashMap<String, ResolvedShape> = resolved
        .iter()
        .filter(|(k, _)| valid_set.contains(k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    generate_validation_methods(&filtered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes_parser::Cardinality;

    #[test]
    fn test_generate_validation() {
        let mut resolved = HashMap::new();
        resolved.insert(
            "TestType".to_string(),
            ResolvedShape {
                element_type: "TestType".to_string(),
                properties: vec![
                    PropertyInfo {
                        name: "requiredProp".to_string(),
                        cardinality: Cardinality::ExactlyOne,
                        property_type: PropertyType::ElementRef("Type".to_string()),
                        read_only: false,
                        description: None,
                    },
                    PropertyInfo {
                        name: "optionalProp".to_string(),
                        cardinality: Cardinality::ZeroOrOne,
                        property_type: PropertyType::String,
                        read_only: false,
                        description: None,
                    },
                ],
                supertypes: vec![],
                description: None,
            },
        );

        let code = generate_validation_methods(&resolved);

        // Check method is generated
        assert!(code.contains("impl<'a> TestTypeProps<'a>"));
        assert!(code.contains("pub fn validate(&self) -> ValidationResult"));

        // Check required property validation
        assert!(code.contains("missing_required(\"requiredProp\")"));

        // Check optional property type check
        assert!(code.contains("Check optional property type: optionalProp"));
    }
}
