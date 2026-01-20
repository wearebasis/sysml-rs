# sysml-vis

Visualization exporters for SysML v2 ModelGraph.

## Purpose

This crate provides exporters for various visualization formats:

- **DOT (Graphviz)**: For static diagrams rendered with Graphviz
- **PlantUML**: For diagrams rendered with PlantUML
- **Cytoscape JSON**: For interactive web-based visualization

## Public API

### DOT (Graphviz)

```rust
use sysml_vis::to_dot;

let dot = to_dot(&graph);
// digraph sysml {
//   "id1" [label="{Package | MyPackage}"];
//   "id2" [label="{Part | Engine}"];
//   "id2" -> "id3" [label="Satisfy"];
// }
```

### PlantUML

```rust
use sysml_vis::to_plantuml;

let puml = to_plantuml(&graph);
// @startuml
// package "MyPackage" {
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
| Part | record |
| Requirement | note |
| VerificationCase | diamond |
| StateMachine | ellipse |
| State | ellipse |
| Action | box |

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
use sysml_core::{ModelGraph, Element, ElementKind, Relationship, RelationshipKind};
use sysml_vis::{to_dot, to_plantuml, to_cytoscape_json};

// Create a model
let mut graph = ModelGraph::new();
let pkg = Element::new_with_kind(ElementKind::Package).with_name("Vehicle");
let pkg_id = graph.add_element(pkg);

let engine = Element::new_with_kind(ElementKind::Part)
    .with_name("Engine")
    .with_owner(pkg_id);
graph.add_element(engine);

// Export to different formats
let dot = to_dot(&graph);
let plantuml = to_plantuml(&graph);
let json = to_cytoscape_json(&graph);

// Write to files
std::fs::write("model.dot", &dot).unwrap();
std::fs::write("model.puml", &plantuml).unwrap();
std::fs::write("model.json", &json).unwrap();

// Render with Graphviz
// dot -Tpng model.dot -o model.png
```
