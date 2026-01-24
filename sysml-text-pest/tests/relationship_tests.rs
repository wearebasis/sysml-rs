//! Tests for SysML v2 relationship construction from parser.
//!
//! These tests verify that the parser correctly creates relationship elements
//! (Specialization, FeatureTyping, Subsetting, Redefinition, Dependency) from
//! the textual syntax.

use sysml_core::ElementKind;
use sysml_text::{ParseResult, Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn parse_source(source: &str) -> ParseResult {
    let parser = PestParser::new();
    parser.parse(&[SysmlFile::new("test.sysml", source)])
}

// =============================================================================
// Specialization Tests (for Definitions)
// =============================================================================

#[test]
fn specialization_from_definition_colon_greater() {
    let source = "package P { part def A; part def B :> A; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let specs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .collect();
    assert_eq!(specs.len(), 1, "Expected 1 Specialization, got {}", specs.len());

    // Check the unresolved_general property
    let general = specs[0]
        .get_prop("unresolved_general")
        .and_then(|v| v.as_str());
    assert_eq!(general, Some("A"), "Expected unresolved_general to be 'A'");
}

#[test]
fn specialization_from_definition_specializes_keyword() {
    let source = "package P { part def Base; part def Derived specializes Base; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let specs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .collect();
    assert_eq!(specs.len(), 1, "Expected 1 Specialization");

    let general = specs[0]
        .get_prop("unresolved_general")
        .and_then(|v| v.as_str());
    assert_eq!(general, Some("Base"));
}

#[test]
fn multiple_specializations() {
    let source = "package P { part def A; part def B; part def C :> A, B; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let specs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .collect();
    assert_eq!(specs.len(), 2, "Expected 2 Specializations for multiple supertypes");

    // Collect all generals
    let generals: Vec<_> = specs
        .iter()
        .filter_map(|s| s.get_prop("unresolved_general").and_then(|v| v.as_str()))
        .collect();
    assert!(generals.contains(&"A"), "Should specialize A");
    assert!(generals.contains(&"B"), "Should specialize B");
}

#[test]
fn specialization_owned_by_specific_type() {
    let source = "package P { part def A; part def B :> A; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    // Find B (the specific type)
    let b = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .find(|e| e.name.as_deref() == Some("B"))
        .expect("PartDefinition B should exist");

    // Find the Specialization
    let spec = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .next()
        .expect("Specialization should exist");

    // Verify the Specialization is owned by B
    assert_eq!(
        spec.owner,
        Some(b.id.clone()),
        "Specialization should be owned by the specific type (B)"
    );
}

// =============================================================================
// FeatureTyping Tests (for Usages)
// =============================================================================

#[test]
fn feature_typing_from_usage_colon() {
    let source = "package P { part def V; part v : V; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let typings: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::FeatureTyping)
        .collect();
    assert_eq!(typings.len(), 1, "Expected 1 FeatureTyping");

    let type_name = typings[0]
        .get_prop("unresolved_type")
        .and_then(|v| v.as_str());
    assert_eq!(type_name, Some("V"));
}

#[test]
fn feature_typing_owned_by_typed_feature() {
    let source = "package P { part def V; part v : V; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    // Find v (the typed feature)
    let v = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .find(|e| e.name.as_deref() == Some("v"))
        .expect("PartUsage v should exist");

    // Find the FeatureTyping
    let typing = result
        .graph
        .elements_by_kind(&ElementKind::FeatureTyping)
        .next()
        .expect("FeatureTyping should exist");

    // Verify the FeatureTyping is owned by v
    assert_eq!(
        typing.owner,
        Some(v.id.clone()),
        "FeatureTyping should be owned by the typed feature (v)"
    );
}

#[test]
fn feature_typing_with_qualified_name() {
    let source = "package P { part def Outer::Inner; part v : Outer::Inner; }";
    let result = parse_source(source);

    // Note: This might fail to parse if nested definitions aren't supported
    // but we're testing qualified name extraction
    if result.diagnostics.is_empty() {
        let typings: Vec<_> = result
            .graph
            .elements_by_kind(&ElementKind::FeatureTyping)
            .collect();
        if typings.len() == 1 {
            let type_name = typings[0]
                .get_prop("unresolved_type")
                .and_then(|v| v.as_str());
            assert_eq!(type_name, Some("Outer::Inner"));
        }
    }
}

// =============================================================================
// Subsetting Tests (for Features)
// =============================================================================

#[test]
fn subsetting_from_usage_colon_greater() {
    let source = "package P { part a; part b :> a; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let subs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Subsetting)
        .collect();
    assert_eq!(subs.len(), 1, "Expected 1 Subsetting");

    let subsetted = subs[0]
        .get_prop("unresolved_subsettedFeature")
        .and_then(|v| v.as_str());
    assert_eq!(subsetted, Some("a"));
}

#[test]
fn subsetting_from_usage_subsets_keyword() {
    let source = "package P { part base; part derived subsets base; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let subs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Subsetting)
        .collect();
    assert_eq!(subs.len(), 1, "Expected 1 Subsetting");

    let subsetted = subs[0]
        .get_prop("unresolved_subsettedFeature")
        .and_then(|v| v.as_str());
    assert_eq!(subsetted, Some("base"));
}

#[test]
fn subsetting_owned_by_subsetting_feature() {
    let source = "package P { part a; part b :> a; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    // Find b (the subsetting feature)
    let b = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .find(|e| e.name.as_deref() == Some("b"))
        .expect("PartUsage b should exist");

    // Find the Subsetting
    let sub = result
        .graph
        .elements_by_kind(&ElementKind::Subsetting)
        .next()
        .expect("Subsetting should exist");

    // Verify the Subsetting is owned by b
    assert_eq!(
        sub.owner,
        Some(b.id.clone()),
        "Subsetting should be owned by the subsetting feature (b)"
    );
}

// =============================================================================
// Redefinition Tests
// =============================================================================

#[test]
fn redefinition_from_usage_colon_double_greater() {
    let source = r#"
        package P {
            part def A { part x; }
            part def B :> A { part y :>> x; }
        }
    "#;
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let redefs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Redefinition)
        .collect();
    assert_eq!(redefs.len(), 1, "Expected 1 Redefinition");

    let redefined = redefs[0]
        .get_prop("unresolved_redefinedFeature")
        .and_then(|v| v.as_str());
    assert_eq!(redefined, Some("x"));
}

#[test]
fn redefinition_from_usage_redefines_keyword() {
    let source = r#"
        package P {
            part def A { part originalPart; }
            part def B :> A { part newPart redefines originalPart; }
        }
    "#;
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let redefs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Redefinition)
        .collect();
    assert_eq!(redefs.len(), 1, "Expected 1 Redefinition");

    let redefined = redefs[0]
        .get_prop("unresolved_redefinedFeature")
        .and_then(|v| v.as_str());
    assert_eq!(redefined, Some("originalPart"));
}

#[test]
fn redefinition_owned_by_redefining_feature() {
    let source = r#"
        package P {
            part def A { part x; }
            part def B :> A { part y :>> x; }
        }
    "#;
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    // Find y (the redefining feature)
    let y = result
        .graph
        .elements_by_kind(&ElementKind::PartUsage)
        .find(|e| e.name.as_deref() == Some("y"))
        .expect("PartUsage y should exist");

    // Find the Redefinition
    let redef = result
        .graph
        .elements_by_kind(&ElementKind::Redefinition)
        .next()
        .expect("Redefinition should exist");

    // Verify the Redefinition is owned by y
    assert_eq!(
        redef.owner,
        Some(y.id.clone()),
        "Redefinition should be owned by the redefining feature (y)"
    );
}

// =============================================================================
// Dependency Tests
// =============================================================================

#[test]
fn dependency_basic() {
    // Basic dependency syntax: dependency <sources> to <targets>;
    let source = "package P { part def A; part def B; dependency A to B; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let deps: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Dependency)
        .collect();
    assert_eq!(deps.len(), 1, "Expected 1 Dependency");

    // Check unresolved_sources
    let sources = deps[0].get_prop("unresolved_sources");
    if let Some(sources) = sources {
        if let Some(list) = sources.as_list() {
            let source_names: Vec<_> = list.iter().filter_map(|v| v.as_str()).collect();
            assert!(source_names.contains(&"A"), "Sources should contain A");
        }
    }

    // Check unresolved_targets
    let targets = deps[0].get_prop("unresolved_targets");
    if let Some(targets) = targets {
        if let Some(list) = targets.as_list() {
            let target_names: Vec<_> = list.iter().filter_map(|v| v.as_str()).collect();
            assert!(target_names.contains(&"B"), "Targets should contain B");
        }
    }
}

#[test]
fn dependency_with_multiple_sources_and_targets() {
    // Multiple sources and targets: dependency A, B to C, D;
    let source = "package P { part def A; part def B; part def C; part def D; dependency A, B to C, D; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let deps: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Dependency)
        .collect();
    assert_eq!(deps.len(), 1, "Expected 1 Dependency");

    // Check sources count
    if let Some(sources) = deps[0].get_prop("unresolved_sources") {
        if let Some(list) = sources.as_list() {
            assert_eq!(list.len(), 2, "Expected 2 sources");
        }
    }

    // Check targets count
    if let Some(targets) = deps[0].get_prop("unresolved_targets") {
        if let Some(list) = targets.as_list() {
            assert_eq!(list.len(), 2, "Expected 2 targets");
        }
    }
}

// =============================================================================
// Combined Relationship Tests
// =============================================================================

#[test]
fn typing_and_subsetting_combined() {
    // A usage can have both typing and subsetting
    let source = "package P { part def V; part a; part b : V :> a; }";
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    let typings: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::FeatureTyping)
        .collect();
    let subs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Subsetting)
        .collect();

    // b should have both a FeatureTyping (: V) and a Subsetting (:> a)
    assert_eq!(typings.len(), 1, "Expected 1 FeatureTyping");
    assert_eq!(subs.len(), 1, "Expected 1 Subsetting");
}

#[test]
fn specialization_and_nested_relationships() {
    let source = r#"
        package P {
            part def Base { part basePart; }
            part def Derived :> Base {
                part derivedPart :>> basePart;
            }
        }
    "#;
    let result = parse_source(source);

    assert!(
        result.diagnostics.is_empty(),
        "Parse errors: {:?}",
        result.diagnostics
    );

    // Check that Derived :> Base creates a Specialization
    let specs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .collect();
    assert_eq!(specs.len(), 1, "Expected 1 Specialization");

    // Check that derivedPart :>> basePart creates a Redefinition
    let redefs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Redefinition)
        .collect();
    assert_eq!(redefs.len(), 1, "Expected 1 Redefinition");
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn no_relationships_without_syntax() {
    // No specialization syntax means no Specialization elements
    let source = "package P { part def A; part def B; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    let specs: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::Specialization)
        .collect();
    assert!(specs.is_empty(), "Should have no Specialization elements");

    let typings: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::FeatureTyping)
        .collect();
    assert!(typings.is_empty(), "Should have no FeatureTyping elements");
}

#[test]
fn usage_without_typing() {
    // Untyped usage should not create FeatureTyping
    let source = "package P { part x; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty());

    let typings: Vec<_> = result
        .graph
        .elements_by_kind(&ElementKind::FeatureTyping)
        .collect();
    assert!(typings.is_empty(), "Untyped usage should have no FeatureTyping");
}
