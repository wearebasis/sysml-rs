//! # sysml-query
//!
//! Query functions for SysML v2 ModelGraph.
//!
//! This crate provides higher-level query functions built on top of
//! the core ModelGraph type.

use sysml_core::{Element, ElementId, ElementKind, ModelGraph, Relationship, RelationshipKind, Value};

/// Find elements by name, optionally filtered by kind.
///
/// # Arguments
///
/// * `graph` - The model graph to search
/// * `kind` - Optional element kind filter
/// * `name` - The name to search for (exact match)
///
/// # Returns
///
/// An iterator over matching elements.
pub fn find_by_name<'a>(
    graph: &'a ModelGraph,
    kind: Option<&'a ElementKind>,
    name: &'a str,
) -> impl Iterator<Item = &'a Element> {
    graph.elements.values().filter(move |e| {
        let name_matches = e.name.as_deref() == Some(name);
        let kind_matches = kind.map_or(true, |k| &e.kind == k);
        name_matches && kind_matches
    })
}

/// Find elements by name pattern (contains).
///
/// # Arguments
///
/// * `graph` - The model graph to search
/// * `kind` - Optional element kind filter
/// * `pattern` - The pattern to search for (substring match)
pub fn find_by_name_contains<'a>(
    graph: &'a ModelGraph,
    kind: Option<&'a ElementKind>,
    pattern: &'a str,
) -> impl Iterator<Item = &'a Element> {
    graph.elements.values().filter(move |e| {
        let name_matches = e.name.as_ref().map_or(false, |n| n.contains(pattern));
        let kind_matches = kind.map_or(true, |k| &e.kind == k);
        name_matches && kind_matches
    })
}

/// Find all requirements that are applicable.
///
/// Looks for requirements with an "applicability" property set to "applicable".
pub fn requirements_applicable(graph: &ModelGraph) -> impl Iterator<Item = &Element> {
    graph
        .elements_by_kind(&ElementKind::RequirementUsage)
        .filter(|e| {
            e.get_prop("applicability")
                .and_then(|v| v.as_str())
                .map_or(true, |s| s == "applicable" || s == "Applicable")
        })
}

/// Find all requirements that are not yet verified.
///
/// A requirement is considered unverified if there are no Verify relationships
/// targeting it.
pub fn requirements_unverified(graph: &ModelGraph) -> impl Iterator<Item = &Element> {
    let verified_ids: std::collections::HashSet<_> = graph
        .relationships_by_kind(&RelationshipKind::Verify)
        .map(|r| r.target.clone())
        .collect();

    graph
        .elements_by_kind(&ElementKind::RequirementUsage)
        .filter(move |e| !verified_ids.contains(&e.id))
}

/// A row in a trace matrix.
#[derive(Debug, Clone)]
pub struct TraceMatrixRow {
    /// The source element.
    pub source: ElementId,
    /// The source element name.
    pub source_name: Option<String>,
    /// The target element.
    pub target: ElementId,
    /// The target element name.
    pub target_name: Option<String>,
    /// The relationship id.
    pub relationship: ElementId,
}

/// Generate a trace matrix between two element kinds via a relationship kind.
///
/// # Arguments
///
/// * `graph` - The model graph
/// * `source_kind` - The kind of source elements
/// * `rel_kind` - The relationship kind to trace
/// * `target_kind` - The kind of target elements
///
/// # Returns
///
/// A vector of trace matrix rows.
pub fn trace_matrix(
    graph: &ModelGraph,
    source_kind: &ElementKind,
    rel_kind: &RelationshipKind,
    target_kind: &ElementKind,
) -> Vec<TraceMatrixRow> {
    let mut rows = Vec::new();

    for rel in graph.relationships_by_kind(rel_kind) {
        let source = graph.get_element(&rel.source);
        let target = graph.get_element(&rel.target);

        if let (Some(src), Some(tgt)) = (source, target) {
            if &src.kind == source_kind && &tgt.kind == target_kind {
                rows.push(TraceMatrixRow {
                    source: src.id.clone(),
                    source_name: src.name.clone(),
                    target: tgt.id.clone(),
                    target_name: tgt.name.clone(),
                    relationship: rel.id.clone(),
                });
            }
        }
    }

    rows
}

/// Find elements that satisfy a given requirement.
pub fn elements_satisfying<'a>(
    graph: &'a ModelGraph,
    requirement_id: &'a ElementId,
) -> impl Iterator<Item = &'a Element> {
    graph
        .incoming(requirement_id)
        .filter(|r| matches!(r.kind, RelationshipKind::Satisfy))
        .filter_map(move |r| graph.get_element(&r.source))
}

/// Find elements that verify a given requirement.
pub fn elements_verifying<'a>(
    graph: &'a ModelGraph,
    requirement_id: &'a ElementId,
) -> impl Iterator<Item = &'a Element> {
    graph
        .incoming(requirement_id)
        .filter(|r| matches!(r.kind, RelationshipKind::Verify))
        .filter_map(move |r| graph.get_element(&r.source))
}

/// Find requirements satisfied by a given element.
pub fn requirements_satisfied_by<'a>(
    graph: &'a ModelGraph,
    element_id: &'a ElementId,
) -> impl Iterator<Item = &'a Element> {
    graph
        .outgoing(element_id)
        .filter(|r| matches!(r.kind, RelationshipKind::Satisfy))
        .filter_map(move |r| graph.get_element(&r.target))
}

/// Find all ancestors of an element (owner chain).
pub fn ancestors<'a>(graph: &'a ModelGraph, element_id: &'a ElementId) -> Vec<&'a Element> {
    let mut result = Vec::new();
    let mut current_id = element_id;

    while let Some(element) = graph.get_element(current_id) {
        if let Some(owner_id) = &element.owner {
            if let Some(owner) = graph.get_element(owner_id) {
                result.push(owner);
                current_id = owner_id;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    result
}

/// Find all descendants of an element (recursive children).
pub fn descendants<'a>(graph: &'a ModelGraph, element_id: &'a ElementId) -> Vec<&'a Element> {
    let mut result = Vec::new();
    let mut stack = vec![element_id.clone()];

    while let Some(id) = stack.pop() {
        for child in graph.children_of(&id) {
            result.push(child);
            stack.push(child.id.clone());
        }
    }

    result
}

/// Find elements by property value.
pub fn find_by_property<'a>(
    graph: &'a ModelGraph,
    key: &'a str,
    value: &'a Value,
) -> impl Iterator<Item = &'a Element> {
    graph
        .elements
        .values()
        .filter(move |e| e.get_prop(key) == Some(value))
}

/// Count relationships by kind.
pub fn count_relationships_by_kind(graph: &ModelGraph) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();

    for rel in graph.relationships.values() {
        *counts.entry(rel.kind.as_str().to_string()).or_insert(0) += 1;
    }

    counts
}

/// Count elements by kind.
pub fn count_elements_by_kind(graph: &ModelGraph) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();

    for elem in graph.elements.values() {
        *counts.entry(elem.kind.as_str().to_string()).or_insert(0) += 1;
    }

    counts
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

        // Requirements
        let req1 = Element::new_with_kind(ElementKind::RequirementUsage)
            .with_name("SafetyReq")
            .with_owner(pkg_id.clone())
            .with_prop("applicability", "applicable");
        let req1_id = graph.add_element(req1);

        let req2 = Element::new_with_kind(ElementKind::RequirementUsage)
            .with_name("PerformanceReq")
            .with_owner(pkg_id.clone())
            .with_prop("applicability", "not_applicable");
        let req2_id = graph.add_element(req2);

        // Parts
        let part1 = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("Engine")
            .with_owner(pkg_id.clone());
        let part1_id = graph.add_element(part1);

        // Verification case
        let vc = Element::new_with_kind(ElementKind::VerificationCaseUsage)
            .with_name("SafetyTest")
            .with_owner(pkg_id.clone());
        let vc_id = graph.add_element(vc);

        // Relationships
        let satisfy = Relationship::new(RelationshipKind::Satisfy, part1_id.clone(), req1_id.clone());
        graph.add_relationship(satisfy);

        let verify = Relationship::new(RelationshipKind::Verify, vc_id, req1_id);
        graph.add_relationship(verify);

        graph
    }

    #[test]
    fn test_find_by_name() {
        let graph = create_test_graph();
        let results: Vec<_> = find_by_name(&graph, None, "SafetyReq").collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, Some("SafetyReq".to_string()));
    }

    #[test]
    fn test_find_by_name_with_kind() {
        let graph = create_test_graph();
        let results: Vec<_> =
            find_by_name(&graph, Some(&ElementKind::RequirementUsage), "SafetyReq").collect();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_by_name_contains() {
        let graph = create_test_graph();
        let results: Vec<_> = find_by_name_contains(&graph, None, "Req").collect();
        assert_eq!(results.len(), 2); // SafetyReq and PerformanceReq
    }

    #[test]
    fn test_requirements_applicable() {
        let graph = create_test_graph();
        let results: Vec<_> = requirements_applicable(&graph).collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, Some("SafetyReq".to_string()));
    }

    #[test]
    fn test_requirements_unverified() {
        let graph = create_test_graph();
        let results: Vec<_> = requirements_unverified(&graph).collect();
        assert_eq!(results.len(), 1); // PerformanceReq is not verified
        assert_eq!(results[0].name, Some("PerformanceReq".to_string()));
    }

    #[test]
    fn test_trace_matrix() {
        let graph = create_test_graph();
        let matrix = trace_matrix(
            &graph,
            &ElementKind::PartUsage,
            &RelationshipKind::Satisfy,
            &ElementKind::RequirementUsage,
        );
        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0].source_name, Some("Engine".to_string()));
        assert_eq!(matrix[0].target_name, Some("SafetyReq".to_string()));
    }

    #[test]
    fn test_ancestors() {
        let graph = create_test_graph();
        let part = find_by_name(&graph, Some(&ElementKind::PartUsage), "Engine")
            .next()
            .unwrap();
        let ancestors = ancestors(&graph, &part.id);
        assert_eq!(ancestors.len(), 1);
        assert_eq!(ancestors[0].name, Some("TestPackage".to_string()));
    }

    #[test]
    fn test_descendants() {
        let graph = create_test_graph();
        let pkg = find_by_name(&graph, Some(&ElementKind::Package), "TestPackage")
            .next()
            .unwrap();
        let descendants = descendants(&graph, &pkg.id);
        assert_eq!(descendants.len(), 4); // 2 requirements, 1 part, 1 verification case
    }

    #[test]
    fn test_count_elements_by_kind() {
        let graph = create_test_graph();
        let counts = count_elements_by_kind(&graph);
        assert_eq!(counts.get("Package"), Some(&1));
        assert_eq!(counts.get("RequirementUsage"), Some(&2));
        assert_eq!(counts.get("PartUsage"), Some(&1));
    }

    #[test]
    fn test_count_relationships_by_kind() {
        let graph = create_test_graph();
        let counts = count_relationships_by_kind(&graph);
        assert_eq!(counts.get("Satisfy"), Some(&1));
        assert_eq!(counts.get("Verify"), Some(&1));
    }
}
