//! # sysml-vis
//!
//! Visualization exporters for SysML v2 ModelGraph.
//!
//! This crate provides exporters for various visualization formats:
//! - DOT (Graphviz)
//! - PlantUML
//! - Cytoscape JSON
//!
//! ## Example
//!
//! ```
//! use sysml_core::ModelGraph;
//! use sysml_vis::{to_dot, to_plantuml, to_cytoscape_json};
//!
//! let graph = ModelGraph::new();
//! let dot = to_dot(&graph);
//! let plantuml = to_plantuml(&graph);
//! let json = to_cytoscape_json(&graph);
//! ```

use sysml_core::{ElementKind, ModelGraph, RelationshipKind};

/// Export a ModelGraph to DOT (Graphviz) format.
///
/// # Arguments
///
/// * `graph` - The model graph to export
///
/// # Returns
///
/// A DOT format string that can be rendered with Graphviz.
pub fn to_dot(graph: &ModelGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph sysml {\n");
    output.push_str("  rankdir=TB;\n");
    output.push_str("  node [shape=record, fontname=\"Helvetica\"];\n");
    output.push_str("  edge [fontname=\"Helvetica\", fontsize=10];\n");
    output.push('\n');

    // Export elements as nodes
    for (id, element) in &graph.elements {
        let name = element.name.as_deref().unwrap_or("unnamed");
        let kind = element.kind.as_str();
        let shape = element_shape(&element.kind);
        let color = element_color(&element.kind);

        output.push_str(&format!(
            "  \"{}\" [label=\"{{{} | {}}}\", shape={}, fillcolor=\"{}\", style=filled];\n",
            id, kind, escape_dot(name), shape, color
        ));
    }

    output.push('\n');

    // Export relationships as edges
    for (_id, rel) in &graph.relationships {
        let label = rel.kind.as_str();
        let style = relationship_style(&rel.kind);
        let color = relationship_color(&rel.kind);

        output.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{}\", style={}, color=\"{}\"];\n",
            rel.source, rel.target, label, style, color
        ));
    }

    output.push_str("}\n");
    output
}

/// Export a ModelGraph to PlantUML format.
///
/// # Arguments
///
/// * `graph` - The model graph to export
///
/// # Returns
///
/// A PlantUML format string.
pub fn to_plantuml(graph: &ModelGraph) -> String {
    let mut output = String::new();
    output.push_str("@startuml\n");
    output.push_str("skinparam linetype ortho\n");
    output.push('\n');

    // Export packages first
    for element in graph.elements_by_kind(&ElementKind::Package) {
        let name = element.name.as_deref().unwrap_or("unnamed");
        output.push_str(&format!("package \"{}\" {{\n", name));

        // Add children
        for child in graph.children_of(&element.id) {
            let child_name = child.name.as_deref().unwrap_or("unnamed");
            let stereotype = plantuml_stereotype(&child.kind);
            output.push_str(&format!("  {} \"{}\" as {}\n", stereotype, child_name, child.id));
        }

        output.push_str("}\n\n");
    }

    // Export top-level elements not in packages
    for element in graph.elements.values() {
        if element.owner.is_none() && !matches!(element.kind, ElementKind::Package) {
            let name = element.name.as_deref().unwrap_or("unnamed");
            let stereotype = plantuml_stereotype(&element.kind);
            output.push_str(&format!("{} \"{}\" as {}\n", stereotype, name, element.id));
        }
    }

    output.push('\n');

    // Export relationships
    for rel in graph.relationships.values() {
        let arrow = plantuml_arrow(&rel.kind);
        let label = rel.kind.as_str();
        output.push_str(&format!("{} {} {} : {}\n", rel.source, arrow, rel.target, label));
    }

    output.push_str("@enduml\n");
    output
}

/// Export a ModelGraph to Cytoscape JSON format.
///
/// # Arguments
///
/// * `graph` - The model graph to export
///
/// # Returns
///
/// A JSON string compatible with Cytoscape.js.
pub fn to_cytoscape_json(graph: &ModelGraph) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Export elements as nodes
    for (id, element) in &graph.elements {
        let name = element.name.as_deref().unwrap_or("unnamed");
        let kind = element.kind.as_str();

        nodes.push(serde_json::json!({
            "data": {
                "id": id.to_string(),
                "label": name,
                "kind": kind,
                "parent": element.owner.as_ref().map(|o| o.to_string())
            }
        }));
    }

    // Export relationships as edges
    for (id, rel) in &graph.relationships {
        edges.push(serde_json::json!({
            "data": {
                "id": id.to_string(),
                "source": rel.source.to_string(),
                "target": rel.target.to_string(),
                "kind": rel.kind.as_str()
            }
        }));
    }

    let result = serde_json::json!({
        "elements": {
            "nodes": nodes,
            "edges": edges
        }
    });

    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}

// Helper functions

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('|', "\\|")
}

fn element_shape(kind: &ElementKind) -> &'static str {
    match kind {
        ElementKind::Package => "folder",
        ElementKind::PartUsage | ElementKind::PartDefinition => "record",
        ElementKind::RequirementUsage | ElementKind::RequirementDefinition => "note",
        ElementKind::VerificationCaseUsage | ElementKind::VerificationCaseDefinition => "diamond",
        ElementKind::StateDefinition => "ellipse",
        ElementKind::StateUsage => "ellipse",
        ElementKind::TransitionUsage => "point",
        ElementKind::ActionUsage | ElementKind::ActionDefinition => "box",
        ElementKind::AttributeUsage | ElementKind::AttributeDefinition => "record",
        ElementKind::Documentation => "note",
        _ => "box",
    }
}

fn element_color(kind: &ElementKind) -> &'static str {
    match kind {
        ElementKind::Package => "#E8F4EA",
        ElementKind::PartUsage | ElementKind::PartDefinition => "#E3F2FD",
        ElementKind::RequirementUsage | ElementKind::RequirementDefinition => "#FFF3E0",
        ElementKind::VerificationCaseUsage | ElementKind::VerificationCaseDefinition => "#F3E5F5",
        ElementKind::StateDefinition => "#E8EAF6",
        ElementKind::StateUsage => "#E1F5FE",
        ElementKind::TransitionUsage => "#FAFAFA",
        ElementKind::ActionUsage | ElementKind::ActionDefinition => "#FBE9E7",
        ElementKind::AttributeUsage | ElementKind::AttributeDefinition => "#F1F8E9",
        ElementKind::Documentation => "#FFFDE7",
        _ => "#FAFAFA",
    }
}

fn relationship_style(kind: &RelationshipKind) -> &'static str {
    match kind {
        RelationshipKind::Owning => "solid",
        RelationshipKind::TypeOf => "solid",
        RelationshipKind::Satisfy => "dashed",
        RelationshipKind::Verify => "dashed",
        RelationshipKind::Derive => "dotted",
        RelationshipKind::Trace => "dotted",
        RelationshipKind::Reference => "solid",
        RelationshipKind::Specialize => "solid",
        RelationshipKind::Redefine => "solid",
        RelationshipKind::Subsetting => "dashed",
        RelationshipKind::Flow => "bold",
        RelationshipKind::Transition => "bold",
    }
}

fn relationship_color(kind: &RelationshipKind) -> &'static str {
    match kind {
        RelationshipKind::Owning => "black",
        RelationshipKind::TypeOf => "blue",
        RelationshipKind::Satisfy => "green",
        RelationshipKind::Verify => "purple",
        RelationshipKind::Derive => "orange",
        RelationshipKind::Trace => "gray",
        RelationshipKind::Reference => "black",
        RelationshipKind::Specialize => "blue",
        RelationshipKind::Redefine => "blue",
        RelationshipKind::Subsetting => "blue",
        RelationshipKind::Flow => "red",
        RelationshipKind::Transition => "red",
    }
}

fn plantuml_stereotype(kind: &ElementKind) -> &'static str {
    match kind {
        ElementKind::Package => "package",
        ElementKind::PartUsage | ElementKind::PartDefinition => "class",
        ElementKind::RequirementUsage | ElementKind::RequirementDefinition => "class <<requirement>>",
        ElementKind::VerificationCaseUsage | ElementKind::VerificationCaseDefinition => "class <<verification>>",
        ElementKind::StateDefinition => "state",
        ElementKind::StateUsage => "state",
        ElementKind::TransitionUsage => "state",
        ElementKind::ActionUsage | ElementKind::ActionDefinition => "class <<action>>",
        ElementKind::AttributeUsage | ElementKind::AttributeDefinition => "class <<attribute>>",
        ElementKind::Documentation => "note",
        _ => "class",
    }
}

fn plantuml_arrow(kind: &RelationshipKind) -> &'static str {
    match kind {
        RelationshipKind::Owning => "*--",
        RelationshipKind::TypeOf => "--|>",
        RelationshipKind::Satisfy => "..>",
        RelationshipKind::Verify => "..>",
        RelationshipKind::Derive => "..>",
        RelationshipKind::Trace => "..>",
        RelationshipKind::Reference => "-->",
        RelationshipKind::Specialize => "--|>",
        RelationshipKind::Redefine => "--|>",
        RelationshipKind::Subsetting => "..|>",
        RelationshipKind::Flow => "-->",
        RelationshipKind::Transition => "-->",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, Relationship};

    fn create_test_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();

        // Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPackage");
        let pkg_id = graph.add_element(pkg);

        // Part
        let part = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("Engine")
            .with_owner(pkg_id.clone());
        let part_id = graph.add_element(part);

        // Requirement
        let req = Element::new_with_kind(ElementKind::RequirementUsage)
            .with_name("SafetyReq")
            .with_owner(pkg_id);
        let req_id = graph.add_element(req);

        // Satisfy relationship
        let satisfy = Relationship::new(RelationshipKind::Satisfy, part_id, req_id);
        graph.add_relationship(satisfy);

        graph
    }

    #[test]
    fn dot_output_structure() {
        let graph = create_test_graph();
        let dot = to_dot(&graph);

        assert!(dot.starts_with("digraph sysml {"));
        assert!(dot.ends_with("}\n"));
        assert!(dot.contains("->"));
        assert!(dot.contains("label="));
    }

    #[test]
    fn dot_contains_elements() {
        let graph = create_test_graph();
        let dot = to_dot(&graph);

        assert!(dot.contains("TestPackage"));
        assert!(dot.contains("Engine"));
        assert!(dot.contains("SafetyReq"));
    }

    #[test]
    fn dot_contains_relationships() {
        let graph = create_test_graph();
        let dot = to_dot(&graph);

        assert!(dot.contains("Satisfy"));
    }

    #[test]
    fn plantuml_output_structure() {
        let graph = create_test_graph();
        let puml = to_plantuml(&graph);

        assert!(puml.starts_with("@startuml"));
        assert!(puml.ends_with("@enduml\n"));
    }

    #[test]
    fn plantuml_contains_elements() {
        let graph = create_test_graph();
        let puml = to_plantuml(&graph);

        assert!(puml.contains("TestPackage"));
        assert!(puml.contains("Engine"));
        assert!(puml.contains("SafetyReq"));
    }

    #[test]
    fn cytoscape_json_structure() {
        let graph = create_test_graph();
        let json = to_cytoscape_json(&graph);

        assert!(json.contains("\"elements\""));
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
    }

    #[test]
    fn cytoscape_json_valid() {
        let graph = create_test_graph();
        let json = to_cytoscape_json(&graph);

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["elements"]["nodes"].is_array());
        assert!(parsed["elements"]["edges"].is_array());
    }

    #[test]
    fn cytoscape_json_contains_elements() {
        let graph = create_test_graph();
        let json = to_cytoscape_json(&graph);

        assert!(json.contains("TestPackage"));
        assert!(json.contains("Engine"));
        assert!(json.contains("SafetyReq"));
    }

    #[test]
    fn empty_graph() {
        let graph = ModelGraph::new();

        let dot = to_dot(&graph);
        assert!(dot.contains("digraph sysml"));

        let puml = to_plantuml(&graph);
        assert!(puml.contains("@startuml"));

        let json = to_cytoscape_json(&graph);
        assert!(json.contains("\"nodes\": []"));
    }
}
