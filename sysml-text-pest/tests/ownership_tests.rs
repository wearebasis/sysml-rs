//! Tests for SysML v2 canonical ownership via OwningMembership elements.
//!
//! These tests verify that the parser correctly creates OwningMembership elements
//! to establish ownership relationships between elements, following the SysML v2
//! specification.

use sysml_core::{ElementKind, VisibilityKind};
use sysml_text::{ParseResult, Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn parse_source(source: &str) -> ParseResult {
    let parser = PestParser::new();
    parser.parse(&[SysmlFile::new("test.sysml", source)])
}

#[test]
fn ownership_creates_owning_membership() {
    let source = "package P { part def X; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    // Find the PartDefinition
    let x = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .find(|e| e.name.as_deref() == Some("X"))
        .expect("PartDefinition X should exist");

    // Verify it has an owning_membership
    assert!(
        x.owning_membership.is_some(),
        "PartDefinition should have owning_membership"
    );

    // Verify the OwningMembership exists and has the correct kind
    let membership = result
        .graph
        .get_element(x.owning_membership.as_ref().unwrap())
        .expect("OwningMembership should exist");
    assert_eq!(
        membership.kind,
        ElementKind::OwningMembership,
        "Membership should be OwningMembership"
    );

    // Verify owner is set correctly
    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");
    assert_eq!(
        x.owner,
        Some(pkg.id.clone()),
        "PartDefinition owner should be the Package"
    );
}

#[test]
fn visibility_extracted_public() {
    let source = "package P { public part def Pub; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    // Check that visible_members returns the public element
    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert_eq!(visible.len(), 1, "Should have 1 visible member");
    assert_eq!(
        visible[0].name,
        Some("Pub".to_string()),
        "Visible member should be 'Pub'"
    );
}

#[test]
fn visibility_extracted_private() {
    let source = "package P { private part def Priv; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    // visible_members should return empty (no public members)
    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert!(
        visible.is_empty(),
        "Private member should not be in visible_members"
    );

    // But owned_members should still include it
    let owned: Vec<_> = result.graph.owned_members(&pkg.id).collect();
    assert_eq!(owned.len(), 1, "Should have 1 owned member");
    assert_eq!(
        owned[0].name,
        Some("Priv".to_string()),
        "Owned member should be 'Priv'"
    );
}

#[test]
fn visibility_mixed_public_private() {
    let source = "package P { public part def Pub; private part def Priv; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    // visible_members should return only public
    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert_eq!(visible.len(), 1, "Should have 1 visible member");
    assert_eq!(
        visible[0].name,
        Some("Pub".to_string()),
        "Visible member should be 'Pub'"
    );

    // owned_members should return both
    let owned: Vec<_> = result.graph.owned_members(&pkg.id).collect();
    assert_eq!(owned.len(), 2, "Should have 2 owned members");
}

#[test]
fn visibility_default_is_public() {
    // No visibility modifier means public by default
    let source = "package P { part def X; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    // visible_members should return the element (default public)
    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert_eq!(
        visible.len(),
        1,
        "Default visibility should be public"
    );
}

#[test]
fn root_has_no_owner() {
    let source = "package Root;";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    assert!(
        pkg.owner.is_none(),
        "Root package should have no owner"
    );
    assert!(
        pkg.owning_membership.is_none(),
        "Root package should have no owning_membership"
    );
}

#[test]
fn nested_package_ownership() {
    let source = "package Outer { package Inner { part def X; } }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    // Find elements
    let outer = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .find(|e| e.name.as_deref() == Some("Outer"))
        .expect("Outer package should exist");

    let inner = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .find(|e| e.name.as_deref() == Some("Inner"))
        .expect("Inner package should exist");

    let x = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .find(|e| e.name.as_deref() == Some("X"))
        .expect("PartDefinition X should exist");

    // Verify ownership chain
    assert!(outer.owner.is_none(), "Outer should have no owner (root)");
    assert_eq!(
        inner.owner,
        Some(outer.id.clone()),
        "Inner owner should be Outer"
    );
    assert_eq!(
        x.owner,
        Some(inner.id.clone()),
        "X owner should be Inner"
    );

    // Verify owning_memberships exist
    assert!(outer.owning_membership.is_none(), "Outer should have no owning_membership (root)");
    assert!(inner.owning_membership.is_some(), "Inner should have owning_membership");
    assert!(x.owning_membership.is_some(), "X should have owning_membership");
}

#[test]
fn structural_validation_passes() {
    let source = "package P { part def V { attribute m; } part v : V; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    // Run structural validation - it should pass without errors
    let errors = result.graph.validate_structure();
    assert!(
        errors.is_empty(),
        "Structural validation should pass. Errors: {:?}",
        errors
    );
}

#[test]
fn qualified_name_resolution_works() {
    let source = "package P { part def X; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    // Resolve qualified name
    let resolved = result.graph.resolve_qname("P::X");
    assert!(
        resolved.is_some(),
        "Should resolve P::X"
    );
    assert_eq!(
        resolved.unwrap().kind,
        ElementKind::PartDefinition,
        "Resolved element should be PartDefinition"
    );
}

#[test]
fn owner_of_follows_membership() {
    let source = "package P { part def X; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let x = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .next()
        .expect("PartDefinition should exist");

    let owner = result.graph.owner_of(&x.id);
    assert!(owner.is_some(), "Should find owner via owner_of");
    assert_eq!(
        owner.unwrap().kind,
        ElementKind::Package,
        "Owner should be Package"
    );
}

#[test]
fn usage_visibility() {
    let source = "package P { public part x; private part y; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert_eq!(visible.len(), 1, "Should have 1 visible usage");
    assert_eq!(
        visible[0].name,
        Some("x".to_string()),
        "Visible usage should be 'x'"
    );
}

#[test]
fn import_has_ownership() {
    let source = "package P { import Other::*; part def X; }";
    let result = parse_source(source);

    // There may be resolution warnings for 'Other' but we don't care about that
    // We just want to verify the import has ownership

    let import = result
        .graph
        .elements_by_kind(&ElementKind::Import)
        .next()
        .expect("Import should exist");

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    assert_eq!(
        import.owner,
        Some(pkg.id.clone()),
        "Import owner should be Package"
    );
    assert!(
        import.owning_membership.is_some(),
        "Import should have owning_membership"
    );
}

#[test]
fn protected_visibility() {
    let source = "package P { protected part def X; }";
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let pkg = result
        .graph
        .elements_by_kind(&ElementKind::Package)
        .next()
        .expect("Package should exist");

    // protected is not public, so visible_members should not include it
    let visible: Vec<_> = result.graph.visible_members(&pkg.id).collect();
    assert!(
        visible.is_empty(),
        "Protected member should not be in visible_members"
    );

    // But members_with_visibility should find it
    let protected: Vec<_> = result
        .graph
        .members_with_visibility(&pkg.id, VisibilityKind::Protected)
        .collect();
    assert_eq!(protected.len(), 1, "Should have 1 protected member");
}

#[test]
fn definition_with_nested_usages() {
    let source = r#"
        package P {
            part def V {
                attribute a;
                part p;
            }
        }
    "#;
    let result = parse_source(source);

    assert!(result.diagnostics.is_empty(), "Parse errors: {:?}", result.diagnostics);

    let v = result
        .graph
        .elements_by_kind(&ElementKind::PartDefinition)
        .find(|e| e.name.as_deref() == Some("V"))
        .expect("PartDefinition V should exist");

    // Check that nested usages are owned by V
    let owned: Vec<_> = result.graph.owned_members(&v.id).collect();
    assert_eq!(
        owned.len(),
        2,
        "PartDefinition should have 2 owned members"
    );

    let names: Vec<_> = owned.iter().filter_map(|e| e.name.as_deref()).collect();
    assert!(names.contains(&"a"), "Should contain attribute 'a'");
    assert!(names.contains(&"p"), "Should contain part 'p'");
}
