use sysml_core::{Element, ElementKind, ModelGraph, VisibilityKind};
use sysml_vis::to_dot_browser_view;

fn main() {
    let mut graph = ModelGraph::new();

    let root =
        graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("VehicleProgram"));

    let library = graph.add_owned_element(
        Element::new_with_kind(ElementKind::LibraryPackage).with_name("StandardLibrary"),
        root.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Wheel"),
        library,
        VisibilityKind::Public,
    );

    let defs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Definitions"),
        root.clone(),
        VisibilityKind::Public,
    );

    let powertrain = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Powertrain"),
        defs.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine"),
        powertrain.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Battery"),
        powertrain,
        VisibilityKind::Private,
    );

    let vehicle = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Vehicle"),
        defs.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("engine"),
        vehicle.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("battery"),
        vehicle.clone(),
        VisibilityKind::Private,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("wheel"),
        vehicle,
        VisibilityKind::Public,
    );

    let reqs = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Requirements"),
        root.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("SafetyRequirement"),
        reqs.clone(),
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::RequirementUsage).with_name("RangeRequirement"),
        reqs,
        VisibilityKind::Public,
    );

    let instances = graph.add_owned_element(
        Element::new_with_kind(ElementKind::Package).with_name("Instances"),
        root,
        VisibilityKind::Public,
    );

    graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("myVehicle"),
        instances,
        VisibilityKind::Public,
    );

    let dot = to_dot_browser_view(&graph);
    println!("{}", dot);
}
