//! # sysml-canon
//!
//! Canonical JSON serialization for SysML v2 ModelGraph with stable ordering.
//!
//! This crate provides deterministic JSON serialization for ModelGraph,
//! ensuring that the same graph always produces the same JSON output.
//! This is essential for:
//!
//! - Content-addressable storage
//! - Diffing and comparison
//! - Reproducible builds
//! - Testing

use serde::{Deserialize, Serialize};
use sysml_core::{Element, ModelGraph, Relationship};

/// Error type for serialization/deserialization failures.
#[derive(Debug)]
pub enum CanonError {
    /// JSON serialization error.
    SerializeError(String),
    /// JSON deserialization error.
    DeserializeError(String),
}

impl std::fmt::Display for CanonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonError::SerializeError(s) => write!(f, "serialization error: {}", s),
            CanonError::DeserializeError(s) => write!(f, "deserialization error: {}", s),
        }
    }
}

impl std::error::Error for CanonError {}

impl From<serde_json::Error> for CanonError {
    fn from(e: serde_json::Error) -> Self {
        CanonError::DeserializeError(e.to_string())
    }
}

/// Canonical representation of a ModelGraph for serialization.
///
/// Elements and relationships are stored in sorted order by ID string
/// to ensure deterministic output.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalGraph {
    /// Schema version for forward compatibility.
    #[serde(default = "default_version")]
    version: String,
    /// Elements sorted by ID.
    elements: Vec<Element>,
    /// Relationships sorted by ID.
    relationships: Vec<Relationship>,
}

fn default_version() -> String {
    "1.0".to_string()
}

impl From<&ModelGraph> for CanonicalGraph {
    fn from(graph: &ModelGraph) -> Self {
        // Collect and sort elements by ID string
        let mut elements: Vec<Element> = graph.elements.values().cloned().collect();
        elements.sort_by(|a, b| a.id.as_str().cmp(&b.id.as_str()));

        // Collect and sort relationships by ID string
        let mut relationships: Vec<Relationship> = graph.relationships.values().cloned().collect();
        relationships.sort_by(|a, b| a.id.as_str().cmp(&b.id.as_str()));

        CanonicalGraph {
            version: "1.0".to_string(),
            elements,
            relationships,
        }
    }
}

impl From<CanonicalGraph> for ModelGraph {
    fn from(canon: CanonicalGraph) -> Self {
        let mut graph = ModelGraph::new();

        for element in canon.elements {
            graph.add_element(element);
        }

        for relationship in canon.relationships {
            graph.add_relationship(relationship);
        }

        graph
    }
}

/// Serialize a ModelGraph to canonical JSON string.
///
/// The output is deterministic: the same graph will always produce
/// the same JSON string. Elements and relationships are sorted by
/// their ID strings.
///
/// # Arguments
///
/// * `graph` - The model graph to serialize
///
/// # Returns
///
/// A JSON string representation of the graph.
///
/// # Example
///
/// ```
/// use sysml_core::ModelGraph;
/// use sysml_canon::to_json_string;
///
/// let graph = ModelGraph::new();
/// let json = to_json_string(&graph);
/// ```
pub fn to_json_string(graph: &ModelGraph) -> String {
    let canon = CanonicalGraph::from(graph);
    // Use serde_json with sorted keys to ensure deterministic output
    serde_json::to_string(&canon).expect("ModelGraph should always be serializable")
}

/// Serialize a ModelGraph to pretty-printed canonical JSON string.
///
/// Like `to_json_string`, but with indentation for readability.
pub fn to_json_string_pretty(graph: &ModelGraph) -> String {
    let canon = CanonicalGraph::from(graph);
    serde_json::to_string_pretty(&canon).expect("ModelGraph should always be serializable")
}

/// Deserialize a ModelGraph from a JSON string.
///
/// # Arguments
///
/// * `json` - The JSON string to parse
///
/// # Returns
///
/// A ModelGraph on success, or an error on parse failure.
///
/// # Example
///
/// ```
/// use sysml_canon::from_json_str;
///
/// let json = r#"{"version":"1.0","elements":[],"relationships":[]}"#;
/// let graph = from_json_str(json).unwrap();
/// ```
pub fn from_json_str(json: &str) -> Result<ModelGraph, CanonError> {
    let canon: CanonicalGraph = serde_json::from_str(json)?;
    Ok(ModelGraph::from(canon))
}

/// Serialize a ModelGraph to a JSON value.
pub fn to_json_value(graph: &ModelGraph) -> serde_json::Value {
    let canon = CanonicalGraph::from(graph);
    serde_json::to_value(canon).expect("ModelGraph should always be serializable")
}

/// Deserialize a ModelGraph from a JSON value.
pub fn from_json_value(value: serde_json::Value) -> Result<ModelGraph, CanonError> {
    let canon: CanonicalGraph = serde_json::from_value(value)?;
    Ok(ModelGraph::from(canon))
}

/// Compute a hash of the canonical JSON representation.
///
/// This can be used for content-addressable storage or change detection.
/// Uses a simple FNV-1a hash for demonstration; in production, consider
/// using SHA-256 or similar.
pub fn content_hash(graph: &ModelGraph) -> u64 {
    let json = to_json_string(graph);
    // FNV-1a hash
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in json.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, ElementKind, Relationship, RelationshipKind};

    fn create_test_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();

        let elem1 = Element::new_with_kind(ElementKind::Package).with_name("A");
        let elem2 = Element::new_with_kind(ElementKind::PartUsage).with_name("B");
        let id1 = graph.add_element(elem1);
        let id2 = graph.add_element(elem2);

        let rel = Relationship::new(RelationshipKind::Owning, id1, id2);
        graph.add_relationship(rel);

        graph
    }

    #[test]
    fn roundtrip() {
        let graph = create_test_graph();
        let json = to_json_string(&graph);
        let restored = from_json_str(&json).unwrap();

        assert_eq!(graph.element_count(), restored.element_count());
        assert_eq!(graph.relationship_count(), restored.relationship_count());
    }

    #[test]
    fn deterministic_output() {
        let graph = create_test_graph();

        let json1 = to_json_string(&graph);
        let json2 = to_json_string(&graph);

        assert_eq!(json1, json2, "Output should be deterministic");
    }

    #[test]
    fn deterministic_after_roundtrip() {
        let graph = create_test_graph();
        let json1 = to_json_string(&graph);

        let restored = from_json_str(&json1).unwrap();
        let json2 = to_json_string(&restored);

        assert_eq!(json1, json2, "Output should be deterministic after roundtrip");
    }

    #[test]
    fn content_hash_deterministic() {
        let graph = create_test_graph();

        let hash1 = content_hash(&graph);
        let hash2 = content_hash(&graph);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn content_hash_changes_with_content() {
        let mut graph = create_test_graph();
        let hash1 = content_hash(&graph);

        // Add another element
        let elem = Element::new_with_kind(ElementKind::RequirementUsage).with_name("C");
        graph.add_element(elem);
        let hash2 = content_hash(&graph);

        assert_ne!(hash1, hash2, "Hash should change with content");
    }

    #[test]
    fn empty_graph_roundtrip() {
        let graph = ModelGraph::new();
        let json = to_json_string(&graph);
        let restored = from_json_str(&json).unwrap();

        assert!(restored.is_empty());
    }

    #[test]
    fn json_contains_version() {
        let graph = ModelGraph::new();
        let json = to_json_string(&graph);

        assert!(json.contains("\"version\":\"1.0\""));
    }

    #[test]
    fn pretty_print() {
        let graph = create_test_graph();
        let json = to_json_string_pretty(&graph);

        assert!(json.contains('\n'), "Pretty output should have newlines");
    }

    #[test]
    fn to_and_from_value() {
        let graph = create_test_graph();
        let value = to_json_value(&graph);
        let restored = from_json_value(value).unwrap();

        assert_eq!(graph.element_count(), restored.element_count());
    }
}
