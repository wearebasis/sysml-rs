# sysml-core

Core model types for SysML v2: Element, Relationship, and ModelGraph.

## Purpose

This crate provides the fundamental data structures for representing SysML v2 models:

- **ElementKind**: Types of model elements (Package, Part, Requirement, etc.)
- **RelationshipKind**: Types of relationships (Owning, Satisfy, Verify, etc.)
- **Element**: A model element with id, kind, name, owner, properties, and spans
- **Relationship**: A directed relationship between two elements
- **ModelGraph**: A graph containing elements and relationships with indexes

## Public API

### ElementKind

```rust
pub enum ElementKind {
    Package, Part, Requirement, VerificationCase,
    StateMachine, State, Transition, Action,
    Attribute, Document, Unknown(String),
}
```

### RelationshipKind

```rust
pub enum RelationshipKind {
    Owning, TypeOf, Satisfy, Verify, Derive,
    Trace, Reference, Specialize, Redefine,
    Subsetting, Flow, Transition,
}
```

### Element

```rust
let element = Element::new_with_kind(ElementKind::Part)
    .with_name("Engine")
    .with_owner(pkg_id)
    .with_prop("mass", 150.0)
    .with_span(span);

element.get_prop("mass");  // Option<&Value>
```

### Relationship

```rust
let rel = Relationship::new(RelationshipKind::Satisfy, part_id, req_id)
    .with_prop("rationale", "Design satisfies safety requirement");
```

### ModelGraph

```rust
let mut graph = ModelGraph::new();

// Add elements
let pkg_id = graph.add_element(package);
let part_id = graph.add_element(part);

// Add relationships
graph.add_relationship(relationship);

// Query
graph.get_element(&id);
graph.children_of(&owner_id);
graph.outgoing(&source_id);
graph.incoming(&target_id);
graph.elements_by_kind(&ElementKind::Part);
graph.relationships_by_kind(&RelationshipKind::Satisfy);
graph.roots();
```

## Features

- `serde`: Enable serde serialization support (propagates to dependencies)

## Dependencies

- `sysml-id`: For ElementId and QualifiedName
- `sysml-span`: For Span type
- `sysml-meta`: For Value type
- `serde` (optional): Serialization support

## Example

```rust
use sysml_core::{ModelGraph, Element, Relationship, ElementKind, RelationshipKind};

let mut graph = ModelGraph::new();

// Create a package
let pkg = Element::new_with_kind(ElementKind::Package)
    .with_name("VehicleModel");
let pkg_id = graph.add_element(pkg);

// Create a part
let engine = Element::new_with_kind(ElementKind::Part)
    .with_name("Engine")
    .with_owner(pkg_id.clone());
let engine_id = graph.add_element(engine);

// Create a requirement
let req = Element::new_with_kind(ElementKind::Requirement)
    .with_name("PowerRequirement")
    .with_owner(pkg_id.clone());
let req_id = graph.add_element(req);

// Create satisfaction relationship
let satisfy = Relationship::new(RelationshipKind::Satisfy, engine_id, req_id);
graph.add_relationship(satisfy);

// Query the graph
for child in graph.children_of(&pkg_id) {
    println!("Child: {:?}", child.name);
}
```
