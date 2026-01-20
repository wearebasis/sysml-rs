//! Example: Building a ModelGraph programmatically
//!
//! This example demonstrates how to create a simple SysML model using
//! the sysml-core API.
//!
//! Run with: cargo run --example model_builder

use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind};

fn main() {
    // Create a new model graph
    let mut graph = ModelGraph::new();

    // Create a top-level package
    let vehicle_pkg = Element::new_with_kind(ElementKind::Package)
        .with_name("VehicleModel")
        .with_prop("description", "Top-level vehicle model package");
    let vehicle_pkg_id = graph.add_element(vehicle_pkg);

    println!("Created package: VehicleModel");

    // Create a part for the engine
    let engine = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("Engine")
        .with_owner(vehicle_pkg_id.clone())
        .with_prop("mass", 150.0f64)
        .with_prop("power", 200.0f64);
    let engine_id = graph.add_element(engine);

    println!("Created part: Engine");

    // Create a part for the transmission
    let transmission = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("Transmission")
        .with_owner(vehicle_pkg_id.clone())
        .with_prop("mass", 50.0f64)
        .with_prop("gears", 6i64);
    let transmission_id = graph.add_element(transmission);

    println!("Created part: Transmission");

    // Create some requirements
    let power_req = Element::new_with_kind(ElementKind::RequirementUsage)
        .with_name("PowerRequirement")
        .with_owner(vehicle_pkg_id.clone())
        .with_prop("text", "The engine shall produce at least 180 horsepower");
    let power_req_id = graph.add_element(power_req);

    let efficiency_req = Element::new_with_kind(ElementKind::RequirementUsage)
        .with_name("EfficiencyRequirement")
        .with_owner(vehicle_pkg_id.clone())
        .with_prop("text", "The vehicle shall achieve at least 30 mpg");
    let efficiency_req_id = graph.add_element(efficiency_req);

    println!("Created requirements");

    // Create satisfaction relationships
    let satisfy1 = Relationship::new(RelationshipKind::Satisfy, engine_id.clone(), power_req_id)
        .with_prop("rationale", "Engine produces 200 hp, exceeding requirement");
    graph.add_relationship(satisfy1);

    let satisfy2 = Relationship::new(
        RelationshipKind::Satisfy,
        transmission_id.clone(),
        efficiency_req_id,
    )
    .with_prop("rationale", "6-speed transmission optimizes fuel efficiency");
    graph.add_relationship(satisfy2);

    println!("Created satisfaction relationships");

    // Print summary
    println!("\n=== Model Summary ===");
    println!("Elements: {}", graph.element_count());
    println!("Relationships: {}", graph.relationship_count());

    println!("\n=== Package Contents ===");
    for child in graph.children_of(&vehicle_pkg_id) {
        println!(
            "  - {} ({})",
            child.name.as_deref().unwrap_or("unnamed"),
            child.kind
        );
    }

    println!("\n=== Requirements ===");
    for req in graph.elements_by_kind(&ElementKind::RequirementUsage) {
        let text = req
            .get_prop("text")
            .and_then(|v| v.as_str())
            .unwrap_or("no text");
        println!(
            "  - {}: {}",
            req.name.as_deref().unwrap_or("unnamed"),
            text
        );
    }

    println!("\n=== Satisfaction Relationships ===");
    for rel in graph.relationships_by_kind(&RelationshipKind::Satisfy) {
        let source = graph.get_element(&rel.source);
        let target = graph.get_element(&rel.target);
        if let (Some(src), Some(tgt)) = (source, target) {
            println!(
                "  - {} satisfies {}",
                src.name.as_deref().unwrap_or("unnamed"),
                tgt.name.as_deref().unwrap_or("unnamed")
            );
        }
    }
}
