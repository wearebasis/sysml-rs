//! Integration tests for the pest parser.

use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

#[test]
fn parse_simple_package() {
    let parser = PestParser::new();
    let source = r#"
package A {
    part def A;
}
"#;
    let files = vec![SysmlFile::new("a.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
    assert!(result.graph.element_count() >= 2); // Package + PartDefinition
}

#[test]
fn parse_package_with_attribute() {
    let parser = PestParser::new();
    let source = r#"
package LibTest {
    attribute desc = "Just testing";
}
"#;
    let files = vec![SysmlFile::new("libtest.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_vehicle_model() {
    let parser = PestParser::new();
    let source = r#"
package VehicleModel {
    part def Vehicle {
        attribute mass : Real;
    }

    part def Wheel;

    part car : Vehicle {
        part wheels : Wheel[4];
    }
}
"#;
    let files = vec![SysmlFile::new("vehicle.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_action_with_flow() {
    let parser = PestParser::new();
    let source = r#"
package Actions {
    action def Drive;
    action def Park;

    action def VehicleActions {
        action drive : Drive;
        action park : Park;

        first drive then park;
    }
}
"#;
    let files = vec![SysmlFile::new("actions.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_state_machine() {
    let parser = PestParser::new();
    let source = r#"
package StateMachines {
    state def VehicleStates {
        entry;
        do;
        exit;
    }

    state parked : VehicleStates;
    state driving : VehicleStates;
}
"#;
    let files = vec![SysmlFile::new("states.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_requirements() {
    let parser = PestParser::new();
    let source = r#"
package Requirements {
    requirement def MaxSpeedRequirement;
    requirement def SafetyRequirement;

    requirement speed : MaxSpeedRequirement;
    requirement safety : SafetyRequirement;
}
"#;
    let files = vec![SysmlFile::new("requirements.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_ports_and_connections() {
    let parser = PestParser::new();
    let source = r#"
package Interfaces {
    port def FuelPort;
    port def ElectricPort;

    part def Engine {
        port fuelIn : FuelPort;
    }

    part def FuelTank {
        port fuelOut : FuelPort;
    }
}
"#;
    let files = vec![SysmlFile::new("interfaces.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_imports() {
    let parser = PestParser::new();
    let source = r#"
package Main {
    import OtherPackage::*;
    import SI::*;

    part def MainPart;
}
"#;
    let files = vec![SysmlFile::new("main.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_visibility() {
    let parser = PestParser::new();
    let source = r#"
package Visibility {
    public part def PublicPart;
    private part def PrivatePart;
    protected part def ProtectedPart;
}
"#;
    let files = vec![SysmlFile::new("visibility.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_specialization() {
    let parser = PestParser::new();
    let source = r#"
package Specialization {
    part def Vehicle;
    part def Car :> Vehicle;
    part def Truck :> Vehicle;

    part myCar : Car;
}
"#;
    let files = vec![SysmlFile::new("specialization.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_calculation() {
    let parser = PestParser::new();
    let source = r#"
package Calculations {
    calc def TotalMass;
    calc def Speed;

    calc totalMass : TotalMass;
}
"#;
    let files = vec![SysmlFile::new("calculations.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_constraints() {
    let parser = PestParser::new();
    let source = r#"
package Constraints {
    constraint def SpeedLimit;
    constraint def WeightLimit;

    constraint speed : SpeedLimit;
}
"#;
    let files = vec![SysmlFile::new("constraints.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_enumerations() {
    let parser = PestParser::new();
    let source = r#"
package Enumerations {
    enum def Color {
        enum red;
        enum green;
        enum blue;
    }

    attribute myColor : Color;
}
"#;
    let files = vec![SysmlFile::new("enumerations.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_abstract_definitions() {
    let parser = PestParser::new();
    let source = r#"
package AbstractDefs {
    abstract part def AbstractVehicle;

    part def ConcreteVehicle :> AbstractVehicle;
}
"#;
    let files = vec![SysmlFile::new("abstract.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_library_package() {
    let parser = PestParser::new();
    let source = r#"
standard library package SI {
    part def Unit;
}
"#;
    let files = vec![SysmlFile::new("si.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_metadata() {
    let parser = PestParser::new();
    let source = r#"
package MetadataExample {
    #Safety
    part def SafePart;
}
"#;
    let files = vec![SysmlFile::new("metadata.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}

#[test]
fn parse_expressions() {
    let parser = PestParser::new();
    let source = r#"
package Expressions {
    attribute x = 1;
    attribute y = 2;
    attribute z = 3;
}
"#;
    let files = vec![SysmlFile::new("expressions.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok());
}
