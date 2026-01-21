# sysml-canon

Canonical JSON serialization for SysML v2 ModelGraph with stable ordering.

## Purpose

This crate provides deterministic JSON serialization for ModelGraph, ensuring that the same graph always produces the same JSON output. This is essential for:

- Content-addressable storage
- Diffing and comparison
- Reproducible builds
- Testing

## Public API

### Serialization

```rust
use sysml_canon::{to_json_string, to_json_string_pretty, to_json_value};

// Compact JSON
let json = to_json_string(&graph);

// Pretty-printed JSON
let json = to_json_string_pretty(&graph);

// JSON value
let value = to_json_value(&graph);
```

### Deserialization

```rust
use sysml_canon::{from_json_str, from_json_value};

// From string
let graph = from_json_str(&json)?;

// From value
let graph = from_json_value(value)?;
```

### Content Hashing

```rust
use sysml_canon::content_hash;

// Compute a hash for change detection
let hash = content_hash(&graph);
```

## Canonical Format

The JSON output is structured as:

```json
{
  "version": "1.0",
  "elements": [...],
  "relationships": [...]
}
```

Elements and relationships are sorted by their ID strings to ensure deterministic ordering.

## Spec-Backed Canonical JSON-LD (Planned)

Implementation notes for moving from the internal format above to a spec-aligned JSON-LD representation:

- **Source of truth**: `spec/metamodel/schemas.json` and the per-type schemas in `spec/metamodel/*.json`.
- **Identifiers**: map `ElementId` -> `@id` (UUID), `ElementKind` -> `@type` (SysML type name).
- **Properties**: emit only schema-defined properties (e.g. `ownedElement`, `ownedRelationship`, `featureMembership`, `documentation`) and validate them against the schema.
- **Relationships**: serialize Relationship and Membership subtypes as first-class elements, not a separate `relationships` array.
- **Ownership**: derive `owner` and `owningMembership` from membership elements to match the SysML v2 ownership model.
- **Determinism**: sort element arrays and property arrays by `@id`; consider JSON canonicalization (RFC 8785) for stable hashing.
- **Validation**: validate JSON output against the schema and fail serialization if required fields are missing or any unexpected fields appear.

## Dependencies

- `sysml-core`: Core model types (with serde feature)
- `serde`: Serialization framework
- `serde_json`: JSON format

## Example

```rust
use sysml_core::{ModelGraph, Element, ElementKind};
use sysml_canon::{to_json_string, from_json_str, content_hash};

// Create a graph
let mut graph = ModelGraph::new();
let elem = Element::new_with_kind(ElementKind::Package).with_name("MyPackage");
graph.add_element(elem);

// Serialize to JSON
let json = to_json_string(&graph);

// Deserialize back
let restored = from_json_str(&json)?;

// Content hashing for change detection
let hash = content_hash(&graph);
println!("Content hash: {:016x}", hash);
```
