use std::collections::{BTreeMap, BTreeSet, HashMap};

use sysml_core::{ElementId, ElementKind, ModelGraph, RelationshipKind};

use crate::classify::{
    is_interconnection_kind, is_membership_kind, is_part_kind, is_port_kind,
    is_requirement_kind, is_requirement_relationship,
};

/// Export a ModelGraph to DOT (Graphviz) format.
///
/// This is an alias for the general view.
pub fn to_dot(graph: &ModelGraph) -> String {
    to_dot_general_view(graph)
}

/// Export a ModelGraph to a general DOT (Graphviz) view.
pub fn to_dot_general_view(graph: &ModelGraph) -> String {
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
            id,
            kind,
            escape_dot(name),
            shape,
            color
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

/// Export a ModelGraph to a browser-style DOT view (ownership tree).
///
/// This view only shows ownership edges (namespace -> owned members).
pub fn to_dot_browser_view(graph: &ModelGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph sysml_browser {\n");
    output.push_str("  rankdir=TB;\n");
    output.push_str("  node [shape=record, fontname=\"Helvetica\"];\n");
    output.push_str("  edge [fontname=\"Helvetica\", fontsize=10];\n");
    output.push('\n');

    // Export elements as nodes (skip membership elements for clarity)
    for (id, element) in &graph.elements {
        if is_membership_kind(&element.kind) {
            continue;
        }
        let name = element.name.as_deref().unwrap_or("unnamed");
        let kind = element.kind.as_str();
        let shape = element_shape(&element.kind);
        let color = element_color(&element.kind);

        output.push_str(&format!(
            "  \"{}\" [label=\"{{{} | {}}}\", shape={}, fillcolor=\"{}\", style=filled];\n",
            id,
            kind,
            escape_dot(name),
            shape,
            color
        ));
    }

    output.push('\n');

    // Ownership edges (namespace -> owned member)
    for owner in graph.elements.values() {
        if is_membership_kind(&owner.kind) {
            continue;
        }
        for member in graph.owned_members(&owner.id) {
            if is_membership_kind(&member.kind) {
                continue;
            }
            output.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"owns\", style=solid, color=\"black\"];\n",
                owner.id, member.id
            ));
        }
    }

    output.push_str("}\n");
    output
}

/// Export a ModelGraph to a requirements-focused DOT view.
///
/// This view includes requirement/verification elements and any nodes
/// involved in satisfy/verify/derive/trace relationships.
pub fn to_dot_requirements_view(graph: &ModelGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph sysml_requirements {\n");
    output.push_str("  rankdir=LR;\n");
    output.push_str("  node [shape=record, fontname=\"Helvetica\"];\n");
    output.push_str("  edge [fontname=\"Helvetica\", fontsize=10];\n");
    output.push('\n');

    let mut nodes = BTreeSet::new();

    for element in graph.elements.values() {
        if is_requirement_kind(&element.kind) {
            nodes.insert(element.id.clone());
        }
    }

    for rel in graph.relationships.values() {
        if is_requirement_relationship(&rel.kind) {
            nodes.insert(rel.source.clone());
            nodes.insert(rel.target.clone());
        }
    }

    for id in &nodes {
        if let Some(element) = graph.get_element(id) {
            if is_membership_kind(&element.kind) {
                continue;
            }
            let name = element.name.as_deref().unwrap_or("unnamed");
            let kind = element.kind.as_str();
            let shape = element_shape(&element.kind);
            let color = element_color(&element.kind);

            output.push_str(&format!(
                "  \"{}\" [label=\"{{{} | {}}}\", shape={}, fillcolor=\"{}\", style=filled];\n",
                id,
                kind,
                escape_dot(name),
                shape,
                color
            ));
        }
    }

    output.push('\n');

    for rel in graph.relationships.values() {
        if !is_requirement_relationship(&rel.kind) {
            continue;
        }
        if !nodes.contains(&rel.source) || !nodes.contains(&rel.target) {
            continue;
        }
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

/// Export a ModelGraph to an interconnection-focused DOT view.
///
/// This view includes parts, ports, connections, flows, and related elements.
pub fn to_dot_interconnection_view(graph: &ModelGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph sysml_interconnection {\n");
    output.push_str("  rankdir=LR;\n");
    output.push_str("  node [fontname=\"Helvetica\"];\n");
    output.push_str("  edge [fontname=\"Helvetica\", fontsize=10];\n");
    output.push('\n');

    let mut nodes = BTreeSet::new();
    let mut port_anchors: HashMap<ElementId, (ElementId, String)> = HashMap::new();
    let mut part_ports: BTreeMap<ElementId, Vec<(String, String)>> = BTreeMap::new();
    let mut embedded_ports = BTreeSet::new();

    for element in graph.elements.values() {
        if is_interconnection_kind(&element.kind) {
            nodes.insert(element.id.clone());
        }
    }

    // Collect owned ports for parts so we can render them as node ports.
    for element in graph.elements.values() {
        if !is_part_kind(&element.kind) {
            continue;
        }
        let mut ports: Vec<(String, ElementId)> = graph
            .owned_members(&element.id)
            .filter(|p| is_port_kind(&p.kind))
            .map(|p| (p.name.clone().unwrap_or_else(|| "port".to_string()), p.id.clone()))
            .collect();
        ports.sort_by(|a, b| a.0.cmp(&b.0));

        if ports.is_empty() {
            continue;
        }

        let mut rows = Vec::new();
        for (idx, (label, port_id)) in ports.into_iter().enumerate() {
            let anchor = format!("p{}", idx);
            port_anchors.insert(port_id.clone(), (element.id.clone(), anchor.clone()));
            embedded_ports.insert(port_id);
            rows.push((anchor, label));
        }
        part_ports.insert(element.id.clone(), rows);
    }

    for id in &nodes {
        if let Some(element) = graph.get_element(id) {
            if is_membership_kind(&element.kind) || embedded_ports.contains(id) {
                continue;
            }
            let name = element.name.as_deref().unwrap_or("unnamed");
            let kind = element.kind.as_str();
            let color = element_color(&element.kind);

            if let Some(ports) = part_ports.get(id) {
                let label = build_part_table_label(kind, name, color, ports);
                output.push_str(&format!(
                    "  \"{}\" [shape=plaintext, label=<{}>];\n",
                    id, label
                ));
            } else {
                let shape = element_shape(&element.kind);
                output.push_str(&format!(
                    "  \"{}\" [label=\"{{{} | {}}}\", shape={}, fillcolor=\"{}\", style=filled];\n",
                    id,
                    kind,
                    escape_dot(name),
                    shape,
                    color
                ));
            }
        }
    }

    output.push('\n');

    // Ownership edges (skip embedded ports to avoid ghost nodes)
    for owner in graph.elements.values() {
        if !nodes.contains(&owner.id) {
            continue;
        }
        for member in graph.owned_members(&owner.id) {
            if !nodes.contains(&member.id) {
                continue;
            }
            if embedded_ports.contains(&member.id) {
                continue;
            }
            output.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"owns\", style=dashed, color=\"gray\"];\n",
                owner.id, member.id
            ));
        }
    }

    // Relationship edges among interconnection nodes
    for rel in graph.relationships.values() {
        if !is_interconnection_relationship(&rel.kind) {
            continue;
        }
        let source = dot_endpoint(&rel.source, &port_anchors);
        let target = dot_endpoint(&rel.target, &port_anchors);
        let label = rel.kind.as_str();
        let style = relationship_style(&rel.kind);
        let color = relationship_color(&rel.kind);

        output.push_str(&format!(
            "  {} -> {} [label=\"{}\", style={}, color=\"{}\"];\n",
            source, target, label, style, color
        ));
    }

    output.push_str("}\n");
    output
}

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

fn is_interconnection_relationship(kind: &RelationshipKind) -> bool {
    matches!(
        kind,
        RelationshipKind::Flow | RelationshipKind::TypeOf | RelationshipKind::Reference
    )
}

fn dot_endpoint(id: &ElementId, port_anchors: &HashMap<ElementId, (ElementId, String)>) -> String {
    if let Some((parent, anchor)) = port_anchors.get(id) {
        format!("\"{}\":{}", parent, anchor)
    } else {
        format!("\"{}\"", id)
    }
}

fn build_part_table_label(
    kind: &str,
    name: &str,
    color: &str,
    ports: &[(String, String)],
) -> String {
    let mut out = String::new();
    out.push_str("<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">");
    out.push_str(&format!(
        "<TR><TD BGCOLOR=\"{}\"><B>{}</B></TD><TD BGCOLOR=\"{}\">{}</TD></TR>",
        color,
        escape_html(kind),
        color,
        escape_html(name)
    ));
    for (anchor, label) in ports {
        out.push_str(&format!(
            "<TR><TD PORT=\"{}\" COLSPAN=\"2\">{}</TD></TR>",
            anchor,
            escape_html(label)
        ));
    }
    out.push_str("</TABLE>");
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
