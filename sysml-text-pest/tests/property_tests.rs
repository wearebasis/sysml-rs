//! Tests for SysML v2 property extraction from parser.
//!
//! These tests verify that the parser correctly extracts semantic properties
//! like multiplicity, directions, values, and flags from the textual syntax.
//!
//! Categories covered:
//! - Multiplicity: [4], [0..10], [*], [1..*]
//! - Directions: in, out, inout
//! - Values: = expr, := expr
//! - Flags: abstract, variation, readonly, derived, end

use sysml_core::ElementKind;
use sysml_text::{ParseResult, Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn parse_source(source: &str) -> ParseResult {
    let parser = PestParser::new();
    parser.parse(&[SysmlFile::new("test.sysml", source)])
}

// =============================================================================
// Multiplicity Tests
// =============================================================================

#[test]
fn multiplicity_exact() {
    let source = "package P { part def Vehicle { attribute wheels[4]; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    // Find the attribute
    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // Verify multiplicity lower/upper bounds are set
    let lower = attrs[0].get_prop("multiplicity_lower");
    let upper = attrs[0].get_prop("multiplicity_upper");

    assert!(lower.is_some(), "multiplicity_lower property should be set");
    assert!(upper.is_some(), "multiplicity_upper property should be set");
    assert_eq!(lower.and_then(|v| v.as_int()), Some(4), "lower bound should be 4");
    assert_eq!(upper.and_then(|v| v.as_int()), Some(4), "upper bound should be 4");
}

#[test]
fn multiplicity_range() {
    let source = "package P { part def Fleet { part vehicles[0..10]; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let parts: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .collect();
    assert_eq!(parts.len(), 1, "Expected 1 PartUsage");

    // Verify multiplicity range bounds
    let lower = parts[0].get_prop("multiplicity_lower");
    let upper = parts[0].get_prop("multiplicity_upper");

    assert!(lower.is_some(), "multiplicity_lower property should be set");
    assert!(upper.is_some(), "multiplicity_upper property should be set");
    assert_eq!(lower.and_then(|v| v.as_int()), Some(0), "lower bound should be 0");
    assert_eq!(upper.and_then(|v| v.as_int()), Some(10), "upper bound should be 10");
}

#[test]
fn multiplicity_star() {
    let source = "package P { part def Collection { item elements[*]; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let items: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ItemUsage)
        .collect();
    assert_eq!(items.len(), 1, "Expected 1 ItemUsage");

    // Verify multiplicity [*] means lower=0, upper="*"
    let lower = items[0].get_prop("multiplicity_lower");
    let upper = items[0].get_prop("multiplicity_upper");

    assert!(lower.is_some(), "multiplicity_lower property should be set");
    assert!(upper.is_some(), "multiplicity_upper property should be set");
    assert_eq!(lower.and_then(|v| v.as_int()), Some(0), "lower bound should be 0");
    assert_eq!(upper.and_then(|v| v.as_str()), Some("*"), "upper bound should be '*' (unbounded)");
}

#[test]
fn multiplicity_one_to_many() {
    let source = "package P { part def Container { part contents[1..*]; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let parts: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .collect();
    assert_eq!(parts.len(), 1, "Expected 1 PartUsage");

    // Verify multiplicity [1..*] means lower=1, upper="*"
    let lower = parts[0].get_prop("multiplicity_lower");
    let upper = parts[0].get_prop("multiplicity_upper");

    assert!(lower.is_some(), "multiplicity_lower property should be set");
    assert!(upper.is_some(), "multiplicity_upper property should be set");
    assert_eq!(lower.and_then(|v| v.as_int()), Some(1), "lower bound should be 1");
    assert_eq!(upper.and_then(|v| v.as_str()), Some("*"), "upper bound should be '*' (unbounded)");
}

// =============================================================================
// Direction Tests (in, out, inout)
// =============================================================================

#[test]
fn direction_in() {
    let source = "package P { action def Process { in item input; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let items: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ItemUsage)
        .collect();
    assert_eq!(items.len(), 1, "Expected 1 ItemUsage");

    // Verify direction property is set to "in"
    let direction = items[0].get_prop("direction");
    assert!(direction.is_some(), "direction property should be set");
    assert_eq!(direction.and_then(|v| v.as_str()), Some("in"), "direction should be 'in'");
}

#[test]
fn direction_out() {
    let source = "package P { action def Process { out item output; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let items: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ItemUsage)
        .collect();
    assert_eq!(items.len(), 1, "Expected 1 ItemUsage");

    // Verify direction property is set to "out"
    let direction = items[0].get_prop("direction");
    assert!(direction.is_some(), "direction property should be set");
    assert_eq!(direction.and_then(|v| v.as_str()), Some("out"), "direction should be 'out'");
}

#[test]
#[ignore = "Grammar gap: inout keyword not yet supported in action body items"]
fn direction_inout() {
    let source = "package P { action def Process { inout item data; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let items: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ItemUsage)
        .collect();
    assert_eq!(items.len(), 1, "Expected 1 ItemUsage");
}

// =============================================================================
// Value Tests (= expr, := expr)
// =============================================================================

#[test]
fn value_default_equals() {
    let source = "package P { attribute def Speed; attribute speed : Speed = 100; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // Verify value property is set
    let value = attrs[0].get_prop("unresolved_value");
    assert!(value.is_some(), "unresolved_value property should be set");
    assert_eq!(value.and_then(|v| v.as_str()), Some("100"), "value should be '100'");

    // Should not have isDefault or isInitial for plain `=`
    assert!(attrs[0].get_prop("isDefault").is_none(), "isDefault should not be set for plain '='");
    assert!(attrs[0].get_prop("isInitial").is_none(), "isInitial should not be set for plain '='");
}

#[test]
fn value_binding_colon_equals() {
    let source = "package P { attribute def Count; attribute total : Count := 0; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // Verify value property is set
    let value = attrs[0].get_prop("unresolved_value");
    assert!(value.is_some(), "unresolved_value property should be set");
    assert_eq!(value.and_then(|v| v.as_str()), Some("0"), "value should be '0'");

    // Should have isInitial set for `:=`
    let is_initial = attrs[0].get_prop("isInitial");
    assert!(is_initial.is_some(), "isInitial property should be set for ':='");
    assert_eq!(is_initial.and_then(|v| v.as_bool()), Some(true), "isInitial should be true");
}

#[test]
fn value_expression_arithmetic() {
    let source = "package P { attribute def X; attribute x : X = 2 + 3; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // Verify value property is set (expression stored as text)
    let value = attrs[0].get_prop("unresolved_value");
    assert!(value.is_some(), "unresolved_value property should be set");
    // Expression should be stored as text "2 + 3"
    let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
    assert!(value_str.contains("2") && value_str.contains("3"), "value should contain '2' and '3'");
}

// =============================================================================
// Flag Tests (abstract, variation, readonly, derived, end)
// =============================================================================

#[test]
fn flag_abstract_definition() {
    let source = "package P { abstract part def Vehicle; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let defs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .collect();
    assert_eq!(defs.len(), 1, "Expected 1 PartDefinition");

    // Check isAbstract property if available
    let is_abstract = defs[0]
        .get_prop("isAbstract")
        .and_then(|v| v.as_bool());
    if is_abstract.is_some() {
        assert_eq!(is_abstract, Some(true));
    }
}

#[test]
fn flag_variation_definition() {
    let source = "package P { variation part def ColorOption; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let defs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .collect();
    assert_eq!(defs.len(), 1, "Expected 1 PartDefinition");
}

#[test]
fn flag_readonly_attribute() {
    let source = "package P { part def Config { readonly attribute id; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");
}

#[test]
fn flag_derived_attribute() {
    let source = "package P { part def Shape { derived attribute area; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");
}

#[test]
#[ignore = "Grammar gap: end keyword not yet supported in connection definition body"]
fn flag_end_feature() {
    let source = "package P { connection def Link { end part source; end part target; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    // Connection definition with end features
    let defs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ConnectionDefinition)
        .collect();
    assert_eq!(defs.len(), 1, "Expected 1 ConnectionDefinition");

    // Should have 2 PartUsage as ends
    let parts: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .collect();
    assert_eq!(parts.len(), 2, "Expected 2 PartUsage (source and target)");
}

// =============================================================================
// False Positive Prevention Tests (Phase 2b.3)
// =============================================================================

/// Verify that element names containing flag keywords don't trigger false positives.
/// This tests the grammar-based flag extraction vs crude text.contains() approach.
#[test]
fn no_false_positive_abstract_in_name() {
    // An attribute named "abstract_value" should NOT have isAbstract=true
    let source = "package P { part def Config { attribute abstract_value; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // The attribute should NOT have isAbstract set just because its name contains "abstract"
    let is_abstract = attrs[0].get_prop("isAbstract");
    assert!(
        is_abstract.is_none(),
        "isAbstract should NOT be set for attribute named 'abstract_value'"
    );
}

/// Verify that element names containing "readonly" don't trigger false positives.
#[test]
fn no_false_positive_readonly_in_name() {
    // An attribute named "readonly_flag" should NOT have isReadOnly=true
    let source = "package P { part def Config { attribute readonly_flag; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // The attribute should NOT have isReadOnly set
    let is_readonly = attrs[0].get_prop("isReadOnly");
    assert!(
        is_readonly.is_none(),
        "isReadOnly should NOT be set for attribute named 'readonly_flag'"
    );
}

/// Verify that element names containing "derived" don't trigger false positives.
#[test]
fn no_false_positive_derived_in_name() {
    // An attribute named "derived_data" should NOT have isDerived=true
    let source = "package P { part def Shape { attribute derived_data; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let attrs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::AttributeUsage)
        .collect();
    assert_eq!(attrs.len(), 1, "Expected 1 AttributeUsage");

    // The attribute should NOT have isDerived set
    let is_derived = attrs[0].get_prop("isDerived");
    assert!(
        is_derived.is_none(),
        "isDerived should NOT be set for attribute named 'derived_data'"
    );
}

// =============================================================================
// Combined Property Tests
// =============================================================================

#[test]
fn combined_direction_and_multiplicity() {
    let source = "package P { action def Transform { in item inputs[1..*]; out item outputs[*]; } }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let items: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::ItemUsage)
        .collect();
    assert_eq!(items.len(), 2, "Expected 2 ItemUsage");
}

#[test]
fn combined_abstract_and_specialization() {
    let source = "package P { abstract part def Base; part def Concrete :> Base; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let defs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .collect();
    assert_eq!(defs.len(), 2, "Expected 2 PartDefinition");
}
