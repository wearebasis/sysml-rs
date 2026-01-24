# sysml-vis

Visualization exporters for SysML v2 ModelGraph.

## Purpose

This crate provides exporters for various visualization formats:

- **DOT (Graphviz)**: For static diagrams rendered with Graphviz
- **PlantUML**: For diagrams rendered with PlantUML
- **Cytoscape JSON**: For interactive web-based visualization

For the detailed plan and milestones, see `sysml-vis/ROADMAP.md`.

## Planned Views (Spec-Based)

The SysML v2 standard library defines view types in:
- `sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/StandardViewDefinitions.sysml`
- `sysmlv2-references/SysML-v2-Pilot-Implementation/sysml.library/Systems Library/Views.sysml`

These are the views we plan to support from the ModelGraph:

- **GeneralView**: Generic graph view of elements and relationships
- **BrowserView**: Tree view of membership/ownership
- **GridView**: Tabular/relationship matrix views
- **InterconnectionView**: Structure and connectivity (parts/ports/connectors)
- **ActionFlowView**: Action flow/activity views
- **StateTransitionView**: State machine views
- **SequenceView**: Lifelines and message sequencing
- **GeometryView**: Spatial/geometry rendering (deferred)

## Implementation Order

1) **BrowserView (Tree)**: purely membership-based, minimal prerequisites
2) **GeneralView (Graph)**: generic nodes/edges (builds on relationships)
3) **GridView (Tables)**: lists and matrices (good for requirements/trace)
4) **InterconnectionView**: parts/ports/connectors/flows
5) **ActionFlowView**: actions, parameters, control nodes, flows
6) **StateTransitionView**: states, transitions, guards, triggers
7) **SequenceView**: lifelines, event occurrences, messages
8) **GeometryView**: spatial rendering (needs geometry model support)

## Textual SysML v2 to View Mapping

This maps common textual constructs to the standard views they appear in:

| View | Typical textual constructs | Notes |
|------|----------------------------|-------|
| **GeneralView** | `package`, `part def`, `part`, `requirement`, `relationship` | Default graph of any elements/relationships |
| **BrowserView** | `package`, owned members, `import` | Hierarchical ownership tree |
| **GridView** | `satisfy`, `verify`, `allocate`, `dependency` | Tables and matrices (requirements/traceability) |
| **InterconnectionView** | `part def`, `part`, `port def`, `port`, `connection`, `flow` | Structure + connectivity diagrams |
| **ActionFlowView** | `action def`, `action`, `first/then`, `fork/join/decision/merge` | Activity/action flow diagrams |
| **StateTransitionView** | `state def`, `state`, `transition` | State machine diagrams |
| **SequenceView** | `event`, `send`, `accept`, `succession` | Lifelines and message ordering |
| **GeometryView** | spatial items, shapes, coordinate frames | 2D/3D spatial renderings (deferred) |

## Public API

### DOT (Graphviz)

```rust
use sysml_vis::{
    to_dot,
    to_dot_browser_view,
    to_dot_general_view,
    to_dot_interconnection_view,
    to_dot_requirements_view,
};

let dot = to_dot(&graph);                // alias for general view
let general = to_dot_general_view(&graph);
let browser = to_dot_browser_view(&graph);
let requirements = to_dot_requirements_view(&graph);
let interconnection = to_dot_interconnection_view(&graph);
// digraph sysml {
//   "id1" [label="{Package | VehicleModel}"];
//   "id2" [label="{PartDefinition | Engine}"];
//   "id2" -> "id3" [label="Satisfy"];
// }
```

### Graphviz Helpers

```rust
use sysml_vis::{render_dot_to_svg, render_dot_to_png};

let svg = render_dot_to_svg(&dot)?;
render_dot_to_png(&dot, "model.png")?;
```

### PlantUML

```rust
use sysml_vis::{to_plantuml, to_plantuml_state_view};

let puml = to_plantuml(&graph);
let states = to_plantuml_state_view(&graph);
// @startuml
// package "VehicleModel" {
//   class "Engine" as id2
// }
// id2 ..> id3 : Satisfy
// @enduml
```

### Cytoscape JSON

```rust
use sysml_vis::to_cytoscape_json;

let json = to_cytoscape_json(&graph);
// {
//   "elements": {
//     "nodes": [...],
//     "edges": [...]
//   }
// }
```

## Output Styling

### Element Shapes (DOT)

| Element Kind | Shape |
|--------------|-------|
| Package | folder |
| PartDefinition / PartUsage | record |
| RequirementDefinition / RequirementUsage | note |
| VerificationCaseDefinition / VerificationCaseUsage | diamond |
| StateDefinition / StateUsage | ellipse |
| ActionDefinition / ActionUsage | box |

### Relationship Styles (DOT)

| Relationship Kind | Style | Color |
|-------------------|-------|-------|
| Owning | solid | black |
| Satisfy | dashed | green |
| Verify | dashed | purple |
| Derive | dotted | orange |
| Trace | dotted | gray |
| Flow | bold | red |
| Transition | bold | red |

## Dependencies

- `sysml-core`: Core model types
- `serde_json`: JSON serialization for Cytoscape

## Example

```rust
use sysml_core::{Element, ElementKind, ModelGraph, Relationship, RelationshipKind, VisibilityKind};
use sysml_vis::{to_dot, to_plantuml, to_cytoscape_json};

// Create a model
let mut graph = ModelGraph::new();
let pkg = Element::new_with_kind(ElementKind::Package).with_name("VehicleModel");
let pkg_id = graph.add_element(pkg);

let engine = Element::new_with_kind(ElementKind::PartDefinition)
    .with_name("Engine");
let engine_id = graph.add_owned_element(engine, pkg_id.clone(), VisibilityKind::Public);

let req = Element::new_with_kind(ElementKind::RequirementUsage)
    .with_name("SafetyRequirement");
let req_id = graph.add_owned_element(req, pkg_id.clone(), VisibilityKind::Public);

let satisfy = Relationship::new(RelationshipKind::Satisfy, engine_id, req_id);
graph.add_relationship(satisfy);

// Export to different formats
let dot = to_dot(&graph);
let plantuml = to_plantuml(&graph);
let json = to_cytoscape_json(&graph);

// Write to files
std::fs::write("model.dot", &dot).unwrap();
std::fs::write("model.puml", &plantuml).unwrap();
std::fs::write("model.json", &json).unwrap();
```

## Examples to Run

```bash
cargo run -p sysml-vis --example browser_view
cargo run -p sysml-vis --example general_view
cargo run -p sysml-vis --example render_dot_svg
cargo run -p sysml-vis --example requirements_view
cargo run -p sysml-vis --example state_view
cargo run -p sysml-vis --example interconnection_view
```
