use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind, VisibilityKind};
use sysml_vis::to_dot_requirements_view;

fn main() {
    let mut graph = ModelGraph::new();

    let root = graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Spec"));

    let reqs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Requirements"),
        root.clone(),
        VisibilityKind::Public,
    );

    let verification = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Verification"),
        root.clone(),
        VisibilityKind::Public,
    );

    let design = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Design"),
        root,
        VisibilityKind::Public,
    );

    let safety_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("SafetyRequirement"),
        reqs.clone(),
        VisibilityKind::Public,
    );

    let braking_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("BrakingRequirement"),
        reqs.clone(),
        VisibilityKind::Public,
    );

    let range_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("RangeRequirement"),
        reqs,
        VisibilityKind::Public,
    );

    let brake_test = graph.add_owned_element(
        Element::new_with_kind(ElementKind::VerificationCaseUsage).with_name("BrakeTest"),
        verification.clone(),
        VisibilityKind::Public,
    );

    let range_test = graph.add_owned_element(
        Element::new_with_kind(ElementKind::VerificationCaseUsage).with_name("RangeTest"),
        verification,
        VisibilityKind::Public,
    );

    let brake_system = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("BrakeSystem"),
        design.clone(),
        VisibilityKind::Public,
    );

    let battery_pack = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("BatteryPack"),
        design,
        VisibilityKind::Public,
    );

    graph.add_relationship(Relationship::new(
        RelationshipKind::Derive,
        braking_req.clone(),
        safety_req.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Trace,
        range_req.clone(),
        braking_req.clone(),
    ));

    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        brake_system,
        braking_req.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        battery_pack,
        range_req.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Verify,
        brake_test,
        braking_req,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Verify,
        range_test,
        range_req,
    ));

    let dot = to_dot_requirements_view(&graph);
    println!("{}", dot);
}
