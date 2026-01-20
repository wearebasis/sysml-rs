# sysml-id

Element identifiers, qualified names, and project/commit IDs for SysML v2.

## Purpose

This crate provides the fundamental identification types used throughout the sysml-rs ecosystem:

- **ElementId**: Unique identifier for model elements (UUID v4 by default)
- **ProjectId**: Identifier for projects
- **CommitId**: Identifier for commits/snapshots
- **QualifiedName**: Hierarchical path names (e.g., "Package::Part::Attribute")

## Public API

### Types

```rust
pub struct ElementId;
pub struct ProjectId;
pub struct CommitId;
pub struct QualifiedName;
pub enum IdError;
```

### ElementId

```rust
// Create a new random ElementId
let id = ElementId::new_v4();

// Create from string (deterministic)
let id = ElementId::from_string("my-element");

// Parse from UUID string
let id: ElementId = "550e8400-e29b-41d4-a716-446655440000".parse()?;

// Display
println!("{}", id);
```

### QualifiedName

```rust
// Parse from string
let qn: QualifiedName = "Package::Part::Attribute".parse()?;

// Build programmatically
let qn = QualifiedName::from_segments(vec!["A".into(), "B".into()]);
let qn = QualifiedName::from_single("Name");

// Navigation
let simple = qn.simple_name();  // "Attribute"
let parent = qn.parent();       // "Package::Part"
let child = qn.child("Sub");    // "Package::Part::Attribute::Sub"
```

## Features

- `uuid` (default): Use UUID v4 for ElementId
- `serde`: Enable serde serialization support

## Dependencies

- `uuid` (optional, default): UUID generation and parsing
- `serde` (optional): Serialization support

## Example

```rust
use sysml_id::{ElementId, QualifiedName, ProjectId, CommitId};

let element_id = ElementId::new_v4();
let project_id = ProjectId::new("my-project");
let commit_id = CommitId::new("v1.0.0");

let qname: QualifiedName = "Vehicle::Engine::Cylinder".parse().unwrap();
assert_eq!(qname.simple_name(), Some("Cylinder"));
```
