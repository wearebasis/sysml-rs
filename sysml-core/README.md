# sysml-core

Core model types for SysML v2: ElementKind, Element, Relationship, and ModelGraph.

## Purpose

This crate provides the fundamental data structures for representing SysML v2 models:

- **ElementKind**: Generated from the official vocab files (266 kinds)
- **Value enums**: FeatureDirectionKind, VisibilityKind, and other spec enums
- **Element**: A model element with id, kind, name, ownership, properties, and spans
- **Relationship**: A directed relationship between two elements
- **ModelGraph**: A graph containing elements and relationships with indexes
- **Ownership helpers**: Membership-based ownership helpers and validation
- **ElementFactory**: Convenience constructors for common elements

## Public API

### ElementKind (generated)

```rust
let kind: ElementKind = "PartUsage".parse().unwrap();
assert!(kind.is_usage());
assert!(kind.is_subtype_of(ElementKind::Feature));
```

### Element and ModelGraph

```rust
use sysml_core::{Element, ElementKind, ModelGraph, VisibilityKind};

let mut graph = ModelGraph::new();

let pkg = Element::new_with_kind(ElementKind::Package)
    .with_name("VehicleModel");
let pkg_id = graph.add_element(pkg);

let engine = Element::new_with_kind(ElementKind::PartDefinition)
    .with_name("Engine");
let engine_id = graph.add_owned_element(engine, pkg_id.clone(), VisibilityKind::Public);

let req = Element::new_with_kind(ElementKind::RequirementUsage)
    .with_name("PowerRequirement");
let req_id = graph.add_owned_element(req, pkg_id.clone(), VisibilityKind::Public);
```

### Relationship

```rust
use sysml_core::{Relationship, RelationshipKind};

let rel = Relationship::new(RelationshipKind::Satisfy, engine_id, req_id);
```

### Property accessors (generated)

```rust
let engine = graph.get_element(&engine_id).unwrap();
let props = engine.as_part_definition().unwrap();
let name = props.declared_name();
```

## RelationshipKind

`RelationshipKind` is a lightweight set of labels used for queries and visualization.
It is not a full mirror of the SysML v2 relationship taxonomy.

## Features

- `serde`: Enable serde serialization support (propagates to dependencies)

## Dependencies

- `sysml-id`: For ElementId and QualifiedName
- `sysml-span`: For Span type
- `sysml-meta`: For Value type
- `serde` (optional): Serialization support
