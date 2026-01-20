//! Example: Exporting ModelGraph to various visualization formats
//!
//! This example demonstrates how to export a SysML model to DOT (Graphviz),
//! PlantUML, and Cytoscape JSON formats.
//!
//! Run with: cargo run --example visualization

use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind};
use sysml_vis::{to_cytoscape_json, to_dot, to_plantuml};

fn create_sample_model() -> ModelGraph {
    let mut graph = ModelGraph::new();

    // Package
    let pkg = Element::new_with_kind(ElementKind::Package).with_name("TrafficControl");
    let pkg_id = graph.add_element(pkg);

    // Parts
    let controller = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("Controller")
        .with_owner(pkg_id.clone());
    let controller_id = graph.add_element(controller);

    let sensor = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("VehicleSensor")
        .with_owner(pkg_id.clone());
    let sensor_id = graph.add_element(sensor);

    let light = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("TrafficLight")
        .with_owner(pkg_id.clone());
    let light_id = graph.add_element(light);

    // Requirements
    let safety_req = Element::new_with_kind(ElementKind::RequirementUsage)
        .with_name("SafetyRequirement")
        .with_owner(pkg_id.clone());
    let safety_req_id = graph.add_element(safety_req);

    let timing_req = Element::new_with_kind(ElementKind::RequirementUsage)
        .with_name("TimingRequirement")
        .with_owner(pkg_id);
    let timing_req_id = graph.add_element(timing_req);

    // Relationships
    let flow1 = Relationship::new(RelationshipKind::Flow, sensor_id.clone(), controller_id.clone());
    graph.add_relationship(flow1);

    let flow2 = Relationship::new(RelationshipKind::Flow, controller_id.clone(), light_id.clone());
    graph.add_relationship(flow2);

    let satisfy1 = Relationship::new(RelationshipKind::Satisfy, controller_id.clone(), safety_req_id);
    graph.add_relationship(satisfy1);

    let satisfy2 = Relationship::new(RelationshipKind::Satisfy, controller_id, timing_req_id);
    graph.add_relationship(satisfy2);

    graph
}

fn main() {
    let graph = create_sample_model();

    println!("=== DOT (Graphviz) Output ===\n");
    let dot = to_dot(&graph);
    println!("{}", dot);

    println!("\n=== PlantUML Output ===\n");
    let plantuml = to_plantuml(&graph);
    println!("{}", plantuml);

    println!("\n=== Cytoscape JSON Output ===\n");
    let json = to_cytoscape_json(&graph);
    println!("{}", json);

    println!("\n=== Usage Instructions ===");
    println!("To render the DOT output:");
    println!("  1. Save the DOT output to a file (e.g., model.dot)");
    println!("  2. Run: dot -Tpng model.dot -o model.png");
    println!();
    println!("To render the PlantUML output:");
    println!("  1. Save the PlantUML output to a file (e.g., model.puml)");
    println!("  2. Run: plantuml model.puml");
    println!();
    println!("To use the Cytoscape JSON:");
    println!("  1. Save the JSON to a file (e.g., model.json)");
    println!("  2. Load it in Cytoscape.js: cy.json(jsonData)");
}
