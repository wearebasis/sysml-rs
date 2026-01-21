# sysml-query

Query functions for SysML v2 ModelGraph.

## Purpose

This crate provides higher-level query functions built on top of the core ModelGraph type:

- Element search by name and kind
- Requirement filtering (applicable, unverified)
- Trace matrix generation
- Ancestor/descendant traversal
- Property-based search
- Statistics and counting

## Public API

### Search Functions

```rust
// Find by exact name
let elements = find_by_name(&graph, Some(&ElementKind::PartUsage), "Engine");

// Find by name pattern (contains)
let elements = find_by_name_contains(&graph, None, "Req");

// Find by property value
let elements = find_by_property(&graph, "priority", &Value::Int(1));
```

### Requirement Queries

```rust
// Find applicable requirements
let reqs = requirements_applicable(&graph);

// Find unverified requirements
let reqs = requirements_unverified(&graph);

// Find elements that satisfy a requirement
let satisfiers = elements_satisfying(&graph, &req_id);

// Find elements that verify a requirement
let verifiers = elements_verifying(&graph, &req_id);

// Find requirements satisfied by an element
let reqs = requirements_satisfied_by(&graph, &part_id);
```

### Trace Matrix

```rust
let matrix = trace_matrix(
    &graph,
    &ElementKind::PartUsage,
    &RelationshipKind::Satisfy,
    &ElementKind::RequirementUsage,
);

for row in matrix {
    println!("{:?} satisfies {:?}", row.source_name, row.target_name);
}
```

### Traversal

```rust
// Get all ancestors (owner chain)
let ancestors = ancestors(&graph, &element_id);

// Get all descendants (recursive children)
let descendants = descendants(&graph, &package_id);
```

### Statistics

```rust
// Count elements by kind
let counts = count_elements_by_kind(&graph);
// {"Package": 1, "PartUsage": 5, "RequirementUsage": 10, ...}

// Count relationships by kind
let counts = count_relationships_by_kind(&graph);
// {"Satisfy": 8, "Verify": 3, ...}
```

## Dependencies

- `sysml-core`: Core model types

## Example

```rust
use sysml_core::{ElementKind, RelationshipKind};
use sysml_query::{find_by_name, requirements_unverified, trace_matrix};

// Find all unverified requirements
for req in requirements_unverified(&graph) {
    println!("Unverified: {:?}", req.name);
}

// Generate a satisfaction trace matrix
let matrix = trace_matrix(
    &graph,
    &ElementKind::PartUsage,
    &RelationshipKind::Satisfy,
    &ElementKind::RequirementUsage,
);

println!("Satisfaction coverage: {} relationships", matrix.len());
```
