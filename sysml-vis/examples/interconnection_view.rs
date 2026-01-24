use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind, VisibilityKind};
use sysml_vis::to_dot_interconnection_view;

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
        root,
        VisibilityKind::Public,
    );

    let sensor_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Sensor"),
        defs.clone(),
        VisibilityKind::Public,
    );
    let controller_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Controller"),
        defs.clone(),
        VisibilityKind::Public,
    );
    let bus_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartDefinition).with_name("Bus"),
        defs.clone(),
        VisibilityKind::Public,
    );

    let data_port_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortDefinition).with_name("DataPort"),
        defs.clone(),
        VisibilityKind::Public,
    );
    let control_port_def = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortDefinition).with_name("ControlPort"),
        defs,
        VisibilityKind::Public,
    );

    let sensor = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("sensor1"),
        instances.clone(),
        VisibilityKind::Public,
    );
    let controller = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("controller1"),
        instances.clone(),
        VisibilityKind::Public,
    );
    let bus = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PartUsage).with_name("bus1"),
        instances,
        VisibilityKind::Public,
    );

    // Port usages owned by their parts
    let sensor_out = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortUsage).with_name("sensorOut"),
        sensor.clone(),
        VisibilityKind::Public,
    );
    let controller_in = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortUsage).with_name("controllerIn"),
        controller.clone(),
        VisibilityKind::Public,
    );
    let controller_cmd = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortUsage).with_name("commandOut"),
        controller.clone(),
        VisibilityKind::Public,
    );
    let bus_in = graph.add_owned_element(
        Element::new_with_kind(ElementKind::PortUsage).with_name("busIn"),
        bus.clone(),
        VisibilityKind::Public,
    );

    // Typing relationships (usage -> definition)
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        sensor.clone(),
        sensor_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        controller.clone(),
        controller_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        bus,
        bus_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        sensor_out.clone(),
        data_port_def.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        controller_in.clone(),
        data_port_def.clone(),
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        bus_in.clone(),
        data_port_def,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::TypeOf,
        controller_cmd.clone(),
        control_port_def,
    ));

    // Flow relationships (port -> port)
    graph.add_relationship(Relationship::new(
        RelationshipKind::Flow,
        sensor_out,
        controller_in,
    ));
    graph.add_relationship(Relationship::new(
        RelationshipKind::Flow,
        controller_cmd,
        bus_in,
    ));

    let dot = to_dot_interconnection_view(&graph);
    println!("{}", dot);
}
