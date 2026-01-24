use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind, VisibilityKind};
use sysml_vis::{to_dot_general_view, to_plantuml};

fn main() {
    let mut graph = ModelGraph::new();

    let root = graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("System"));

    let defs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Definitions"),
        root.clone(),
        VisibilityKind::Public,
    );

    let instances = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Instances"),
        root.clone(),
        VisibilityKind::Public,
    );

    let reqs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Requirements"),
        root.clone(),
        VisibilityKind::Public,
    );

    let verification = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Verification"),
        root,
        VisibilityKind::Public,
    );

    let vehicle_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Vehicle"),
        defs.clone(),
        VisibilityKind::Public,
    );

    let car_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Car"),
        defs.clone(),
        VisibilityKind::Public,
    );

    let engine_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine"),
        defs.clone(),
        VisibilityKind::Public,
    );

    let brake_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Brake"),
        defs,
        VisibilityKind::Public,
    );

    let car_usage = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("myCar"),
        instances.clone(),
        VisibilityKind::Public,
    );

    let engine_usage = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("myEngine"),
        instances,
        VisibilityKind::Public,
    );

    let safety_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("SafetyRequirement"),
        reqs.clone(),
        VisibilityKind::Public,
    );

    let emissions_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("EmissionsRequirement"),
        reqs,
        VisibilityKind::Public,
    );

    let brake_test = graph.add_owned_element(
        Element::new_with_kind(ElementKind::VerificationCaseUsage).with_name("BrakeTest"),
        verification,
        VisibilityKind::Public,
    );

    graph.add_relationship(Relationship::new(
        RelationshipKind::Specialize,
        car_def.clone(),
        vehicle_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        car_usage.clone(),
        car_def.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        engine_usage,
        engine_def.clone(),
    ));

    graph.add_relationship(Relationship::new(
        RelationshipKind::Reference,
        car_usage.clone(),
        engine_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Reference,
        car_usage,
        brake_def.clone(),
    ));

    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        car_def,
        safety_req.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        brake_def,
        emissions_req,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Verify,
        brake_test,
        safety_req,
    ));

    let dot = to_dot_general_view(&graph);
    let plantuml = to_plantuml(&graph);

    println!("--- DOT ---\n{}\n", dot);
    println!("--- PlantUML ---\n{}", plantuml);
}
