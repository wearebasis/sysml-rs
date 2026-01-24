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

mod classify;
mod cytoscape;
mod dot;
mod graphviz;
mod plantuml;

pub use cytoscape::to_cytoscape_json;
pub use dot::{
    to_dot, to_dot_browser_view, to_dot_general_view, to_dot_interconnection_view,
    to_dot_requirements_view,
};
pub use graphviz::{
    render_dot, render_dot_to_pdf, render_dot_to_png, render_dot_to_svg, GraphvizEngine,
    GraphvizFormat, GraphvizOptions, VisError,
};
pub use plantuml::{to_plantuml, to_plantuml_state_view};

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind};

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
