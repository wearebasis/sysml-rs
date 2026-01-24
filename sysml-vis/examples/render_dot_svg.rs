use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind, VisibilityKind};
use sysml_vis::{render_dot_to_svg, to_dot_general_view};

fn main() {
    let mut graph = ModelGraph::new();

    let root = graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Vehicle"));

    let defs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Definitions"),
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

    let engine = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine"),
        defs.clone(),
        VisibilityKind::Public,
    );

    let battery = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Battery"),
        defs,
        VisibilityKind::Public,
    );

    let req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("SafetyRequirement"),
        reqs.clone(),
        VisibilityKind::Public,
    );

    let range_req = graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("RangeRequirement"),
        reqs,
        VisibilityKind::Public,
    );

    let test = graph.add_owned_element(
        Element::new_with_kind(ElementKind::VerificationCaseUsage).with_name("BatteryTest"),
        verification,
        VisibilityKind::Public,
    );

    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        engine,
        req.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Satisfy,
        battery,
        range_req,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Verify,
        test,
        req,
    ));

    let dot = to_dot_general_view(&graph);

    match render_dot_to_svg(&dot) {
        Ok(svg) => {
            std::fs::write("model.svg", svg).expect("write model.svg");
            println!("Wrote model.svg");
        }
        Err(err) => {
            eprintln!("Graphviz render failed: {}", err);
        }
    }
}
