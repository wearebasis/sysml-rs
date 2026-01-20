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
