//! Minimal test cases for scoping strategies.
//!
//! These tests verify that each scoping strategy resolves names correctly
//! without requiring the full corpus. Each test focuses on a specific
//! scoping behavior.
//!
//! Run with:
//! ```bash
//! cargo test -p sysml-spec-tests scoping -- --nocapture
//! ```

use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

/// Parse SysML code and run resolution.
fn parse_and_resolve(code: &str) -> (usize, usize) {
    let parser = PestParser::new();
    let file = SysmlFile::new("test.sysml", code);
    let mut result = parser.parse(&[file]);
    let res = result.resolve();
    // Debug output for failed resolutions
    if res.unresolved_count > 0 {
        eprintln!("Unresolved references:");
        for diag in res.diagnostics.iter().filter(|d| d.is_error()) {
            eprintln!("  - {}", diag);
        }
    }
    (res.resolved_count, res.unresolved_count)
}

/// Check that a piece of code has no unresolved references.
fn assert_resolves(code: &str) {
    let (resolved, unresolved) = parse_and_resolve(code);
    assert_eq!(
        unresolved, 0,
        "Expected all references to resolve, but {} unresolved out of {}",
        unresolved, resolved + unresolved
    );
}

/// Check that a piece of code has the expected resolution rate.
fn assert_resolution_rate(code: &str, expected_resolved: usize, expected_unresolved: usize) {
    let (resolved, unresolved) = parse_and_resolve(code);
    assert_eq!(
        (resolved, unresolved),
        (expected_resolved, expected_unresolved),
        "Expected {}/{} resolved/unresolved, got {}/{}",
        expected_resolved, expected_unresolved, resolved, unresolved
    );
}

// =============================================================================
// OwningNamespace Strategy Tests
// =============================================================================

mod owning_namespace {
    use super::*;

    #[test]
    fn local_type_reference() {
        // A part usage referencing a local type
        let code = r#"
            package Test {
                part def Vehicle;
                part car : Vehicle;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn nested_type_reference() {
        // Reference to a type in parent namespace
        let code = r#"
            package Test {
                part def Engine;
                package Inner {
                    part motor : Engine;
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn specialization_reference() {
        // Specialization using :>
        let code = r#"
            package Test {
                part def Vehicle;
                part def Car :> Vehicle;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    #[ignore = "Subsetting inherited features requires inheritance resolution"]
    fn subsetting_reference() {
        // Subsetting using :>
        let code = r#"
            package Test {
                part def Vehicle {
                    part engine;
                }
                part def Car :> Vehicle {
                    part motor :> engine;
                }
            }
        "#;
        assert_resolves(code);
    }
}

// =============================================================================
// NonExpressionNamespace Strategy Tests
// =============================================================================

mod non_expression_namespace {
    use super::*;

    #[test]
    fn feature_typing_in_part() {
        // FeatureTyping should skip expression scopes
        let code = r#"
            package Test {
                part def Engine;
                part car {
                    part engine : Engine;
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn nested_feature_typing() {
        // Typing in deeply nested structure
        let code = r#"
            package Test {
                part def Cylinder;
                part def Engine {
                    part cylinders : Cylinder[4];
                }
                part car {
                    part engine : Engine {
                        part c : Cylinder;
                    }
                }
            }
        "#;
        assert_resolves(code);
    }
}

// =============================================================================
// RelativeNamespace Strategy Tests
// =============================================================================

mod relative_namespace {
    use super::*;

    #[test]
    #[ignore = "Relative namespace not yet implemented"]
    fn feature_chain_simple() {
        // Feature chain: vehicle.engine
        let code = r#"
            package Test {
                part def Engine;
                part def Vehicle {
                    part engine : Engine;
                }
                part car : Vehicle;

                // Reference to car.engine should work
                alias myEngine = car.engine;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    #[ignore = "Relative namespace not yet implemented"]
    fn feature_chain_multiple() {
        // Longer chain: vehicle.engine.cylinders
        let code = r#"
            package Test {
                part def Cylinder;
                part def Engine {
                    part cylinders : Cylinder[4];
                }
                part def Vehicle {
                    part engine : Engine;
                }
                part car : Vehicle;

                // Reference to car.engine.cylinders
                alias myCylinders = car.engine.cylinders;
            }
        "#;
        assert_resolves(code);
    }
}

// =============================================================================
// TransitionSpecific Strategy Tests
// =============================================================================

mod transition_specific {
    use super::*;

    #[test]
    #[ignore = "Transition scoping not yet implemented"]
    fn state_transition_trigger() {
        // Transition trigger should resolve in state context
        let code = r#"
            package Test {
                state def VehicleStates {
                    entry; then off;
                    state off;
                    state on;

                    transition off_to_on
                        first off
                        accept start
                        then on;
                }
            }
        "#;
        // This will fail until transition scoping is implemented
        assert_resolves(code);
    }
}

// =============================================================================
// Import Resolution Tests
// =============================================================================

mod imports {
    use super::*;

    #[test]
    fn import_single_element() {
        let code = r#"
            package Lib {
                part def Engine;
            }
            package Test {
                import Lib::Engine;
                part myEngine : Engine;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn import_namespace() {
        let code = r#"
            package Lib {
                part def Engine;
                part def Wheel;
            }
            package Test {
                import Lib::*;
                part myEngine : Engine;
                part myWheel : Wheel;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn qualified_reference() {
        let code = r#"
            package Lib {
                part def Engine;
            }
            package Test {
                part myEngine : Lib::Engine;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn cross_package_namespace_import() {
        // Package A defines types, Package B imports from A with ::*
        let code = r#"
            package MeasurementRefs {
                attribute def DerivedUnit;
                attribute def SimpleUnit;
            }
            package Quantities {
                import MeasurementRefs::*;
                attribute def MyUnit :> DerivedUnit;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn library_package_cross_import() {
        // Simulates standard library package cross-reference
        let code = r#"
            standard library package MeasurementRefs {
                attribute def DerivedUnit;
                attribute def DimensionOneValue;
            }
            standard library package ISQThermo {
                import MeasurementRefs::*;
                attribute def ThermalUnit :> DerivedUnit;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn private_import_namespace() {
        // Private imports should still make names visible within the package
        let code = r#"
            package Lib {
                attribute def BaseUnit;
            }
            package Test {
                private import Lib::*;
                attribute def MyUnit :> BaseUnit;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn nested_specialization_chain() {
        // Test inheritance chain where redefinition needs to find inherited feature
        let code = r#"
            package MeasurementRefs {
                abstract attribute def VectorMeasurementRef {
                    attribute dimensions;
                }
                abstract attribute def ScalarMeasurementRef :> VectorMeasurementRef {
                    attribute :>> dimensions = ();
                    attribute quantityDimension;
                }
                abstract attribute def MeasurementUnit :> ScalarMeasurementRef;
                abstract attribute def DerivedUnit :> MeasurementUnit;
            }
            package ISQThermo {
                import MeasurementRefs::*;
                attribute def ThermalResistanceUnit :> DerivedUnit {
                    attribute :>> quantityDimension;
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn library_package_with_import_chain() {
        // Standard library packages with import chains
        let code = r#"
            standard library package Quantities {
                attribute def QuantityDimension;
            }
            standard library package MeasurementRefs {
                private import Quantities::*;
                abstract attribute def ScalarMeasurementRef {
                    attribute quantityDimension : QuantityDimension;
                }
            }
            standard library package ISQBase {
                private import MeasurementRefs::*;
                attribute def LengthUnit :> ScalarMeasurementRef;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn three_level_inheritance_with_redefinition() {
        // Simulates ISQ pattern: A <- B <- C <- D, redefine feature from A
        // Using plain `package` to test resolution (library packages have different lookup path)
        let code = r#"
            package MeasurementRefs {
                abstract attribute def VectorMeasurementRef {
                    attribute dimensions;
                    attribute isOrthogonal;
                }
                abstract attribute def ScalarMeasurementRef :> VectorMeasurementRef {
                    attribute :>> dimensions = ();
                    attribute :>> isOrthogonal = true;
                    attribute quantityDimension;
                    attribute mRefs;
                }
                abstract attribute def MeasurementUnit :> ScalarMeasurementRef;
                abstract attribute def DerivedUnit :> MeasurementUnit;
                abstract attribute def DimensionOneValue;
            }
            package ISQThermo {
                private import MeasurementRefs::*;
                attribute def ThermalUnit :> DerivedUnit {
                    attribute :>> quantityDimension;
                    attribute :>> mRefs;
                }
                attribute def ThermalValue :> DimensionOneValue;
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn three_level_inheritance_with_redefinition_library() {
        // Same as above but with standard library package
        // This tests the library package resolution path
        let code = r#"
            standard library package MeasurementRefs {
                abstract attribute def VectorMeasurementRef {
                    attribute dimensions;
                    attribute isOrthogonal;
                }
                abstract attribute def ScalarMeasurementRef :> VectorMeasurementRef {
                    attribute :>> dimensions = ();
                    attribute :>> isOrthogonal = true;
                    attribute quantityDimension;
                    attribute mRefs;
                }
                abstract attribute def MeasurementUnit :> ScalarMeasurementRef;
                abstract attribute def DerivedUnit :> MeasurementUnit;
                abstract attribute def DimensionOneValue;
            }
            standard library package ISQThermo {
                private import MeasurementRefs::*;
                attribute def ThermalUnit :> DerivedUnit {
                    attribute :>> quantityDimension;
                    attribute :>> mRefs;
                }
                attribute def ThermalValue :> DimensionOneValue;
            }
        "#;
        assert_resolves(code);
    }
}

// =============================================================================
// Inheritance Resolution Tests
// =============================================================================

mod inheritance {
    use super::*;

    #[test]
    #[ignore = "Inherited feature visibility requires inheritance resolution"]
    fn inherited_feature_visible() {
        // Features from supertypes should be visible
        let code = r#"
            package Test {
                part def Vehicle {
                    part engine;
                }
                part def Car :> Vehicle {
                    // Should see 'engine' from Vehicle
                    part turbocharged :> engine;
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    #[ignore = "Redefinition hiding requires inheritance resolution"]
    fn redefinition_hides_inherited() {
        // Redefined feature should shadow inherited one
        let code = r#"
            package Test {
                part def Vehicle {
                    part engine;
                }
                part def ElectricCar :> Vehicle {
                    part motor :>> engine;  // Redefines engine
                }
            }
        "#;
        assert_resolves(code);
    }
}

// =============================================================================
// Edge Cases and Regression Tests
// =============================================================================

mod library_loading {
    use super::*;

    #[test]
    fn parse_measurement_references_minimal() {
        // Test parsing MeasurementReferences.sysml patterns
        let code = r#"
            standard library package MeasurementReferences {
                doc
                /*
                 * This is documentation.
                 */

                abstract attribute def ScalarMeasurementReference {
                    attribute :>> dimensions = ();
                    attribute :>> isOrthogonal = true;
                    attribute quantityDimension;
                }
                abstract attribute def MeasurementUnit :> ScalarMeasurementReference;
                abstract attribute def DerivedUnit :> MeasurementUnit;
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());

        // Check DerivedUnit exists
        let derived_unit = result.graph.elements.values()
            .find(|e| e.name.as_deref() == Some("DerivedUnit"));
        assert!(derived_unit.is_some(), "DerivedUnit not found");
    }

    #[test]
    fn parse_default_value() {
        // Test "default false" and "default true" syntax from MeasurementReferences.sysml
        let code = r#"
            package Test {
                attribute def A {
                    attribute isBound: Boolean[1] default false;
                    attribute isOrthogonal: Boolean[1] default true;
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    fn parse_mref_self() {
        // Test "attribute :>> mRefs = self;" syntax
        let code = r#"
            package Test {
                attribute def A {
                    attribute :>> mRefs = self;
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    fn parse_assert_constraint() {
        // Test "assert constraint { expr }" syntax from MeasurementReferences.sysml
        let code = r#"
            package Test {
                attribute def A {
                    attribute dims;
                    assert constraint { dims == 3 }
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    fn parse_sequence_index_operator() {
        // Test sequence indexing with # operator
        let code = r#"
            package Test {
                attribute def A {
                    attribute dims;
                    assert constraint { dims#(1) == 3 }
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    fn parse_arrow_for_all() {
        // Test ->forAll expression
        let code = r#"
            package Test {
                attribute def A {
                    attribute items;
                    assert constraint { items->forAll { in item; item == 0 } }
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    fn parse_or_expression() {
        // Test "or" keyword in expressions (line 152 of MeasurementReferences.sysml)
        let code = r#"
            package Test {
                attribute def A {
                    attribute x;
                    attribute y;
                    assert constraint { x == 0 or y == 1 }
                }
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    #[ignore = "Parser does not yet support 'new' constructor expressions - see KerMLExpressions.xtext ConstructorExpression"]
    fn parse_new_expression() {
        // Test "= new TypeName()" syntax (line 490 of MeasurementReferences.sysml)
        // This is the root cause why MeasurementReferences.sysml fails to parse
        let code = r#"
            package Test {
                attribute def DimensionOneUnit;
                attribute one : DimensionOneUnit[1] = new DimensionOneUnit();
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        if result.has_errors() {
            for diag in result.diagnostics.iter() {
                eprintln!("Parse error: {}", diag);
            }
        }
        assert!(!result.has_errors(), "Parse failed with {} errors", result.error_count());
    }

    #[test]
    #[ignore = "Requires corpus path"]
    fn parse_actual_measurement_references() {
        // Test parsing the actual MeasurementReferences.sysml file
        let Some(root) = sysml_spec_tests::try_find_references_dir() else {
            eprintln!("Skipping test: references directory not found");
            return;
        };
        let path = root.join(
            "SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.domain/Quantities and Units/MeasurementReferences.sysml",
        );
        if !path.exists() {
            eprintln!("Skipping test: MeasurementReferences.sysml not found at {:?}", path);
            return;
        }
        let content = std::fs::read_to_string(&path).expect("Failed to read file");

        let parser = PestParser::new();
        let file = SysmlFile::new("MeasurementReferences.sysml", &content);
        let result = parser.parse(&[file]);

        println!("Elements: {}", result.graph.element_count());
        if result.has_errors() {
            for diag in result.diagnostics.iter().take(10) {
                eprintln!("Parse error: {}", diag);
            }
        }

        // Check for MeasurementReferences package
        let meas_ref = result.graph.elements.values()
            .find(|e| e.name.as_deref() == Some("MeasurementReferences"));
        println!("MeasurementReferences found: {}", meas_ref.is_some());

        // Don't assert - just report
        println!("Error count: {}", result.error_count());
    }

    #[test]
    fn parse_standard_library_package() {
        // Test that we can parse a standard library package
        let code = r#"
            standard library package MeasurementReferences {
                abstract attribute def TensorMeasurementReference;
                abstract attribute def DerivedUnit;
            }
        "#;
        let parser = PestParser::new();
        let file = SysmlFile::new("test.sysml", code);
        let result = parser.parse(&[file]);

        assert!(!result.has_errors(), "Parse errors: {:?}", result.error_count());

        // Check root package exists
        // Note: The parser creates some OwningMembership elements without owners,
        // which is a known parser issue. Filter for just packages.
        let root_packages: Vec<_> = result.graph.elements.values()
            .filter(|e| e.owner.is_none() &&
                matches!(e.kind, sysml_core::ElementKind::Package | sysml_core::ElementKind::LibraryPackage))
            .collect();
        assert_eq!(root_packages.len(), 1, "Expected 1 root package element");
        assert_eq!(root_packages[0].name.as_deref(), Some("MeasurementReferences"));

        // Check it's a LibraryPackage
        assert!(
            root_packages[0].kind == sysml_core::ElementKind::Package
                || root_packages[0].kind == sysml_core::ElementKind::LibraryPackage,
            "Expected Package or LibraryPackage, got {:?}",
            root_packages[0].kind
        );
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn empty_package() {
        let code = r#"
            package Empty {
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn self_reference() {
        // A part that references itself (recursive structure)
        let code = r#"
            package Test {
                part def Node {
                    part children : Node[*];
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn mutual_reference() {
        // Two types that reference each other
        let code = r#"
            package Test {
                part def A {
                    part b : B;
                }
                part def B {
                    part a : A;
                }
            }
        "#;
        assert_resolves(code);
    }

    #[test]
    fn shadowing() {
        // Local definition shadows outer one
        let code = r#"
            package Outer {
                part def Thing;
                package Inner {
                    part def Thing;  // Shadows Outer::Thing
                    part t : Thing;  // Should resolve to Inner::Thing
                }
            }
        "#;
        assert_resolves(code);
    }
}
