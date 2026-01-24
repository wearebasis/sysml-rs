use sysml_core::ModelGraph;

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
