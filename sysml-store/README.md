# sysml-store

Storage trait and types for SysML v2 model snapshots.

## Purpose

This crate defines the interface for storing and retrieving model snapshots with version control:

- **Store**: Trait for storage backends
- **Snapshot**: A stored model with metadata
- **SnapshotMeta**: Commit information (id, parent, message, timestamp)
- **InMemoryStore**: Built-in in-memory implementation

## Public API

### Store Trait

```rust
pub trait Store {
    fn put_snapshot(&mut self, project: &ProjectId, meta: SnapshotMeta, graph: &ModelGraph) -> Result<(), StoreError>;
    fn get_snapshot(&self, project: &ProjectId, commit: &CommitId) -> Result<Option<Snapshot>, StoreError>;
    fn latest(&self, project: &ProjectId) -> Result<Option<CommitId>, StoreError>;
    fn list_commits(&self, project: &ProjectId) -> Result<Vec<SnapshotMeta>, StoreError>;
    fn list_projects(&self) -> Result<Vec<ProjectId>, StoreError>;
}
```

### SnapshotMeta

```rust
let meta = SnapshotMeta::new(CommitId::new("v1.0"), "Initial release")
    .with_parent(CommitId::new("v0.9"))
    .with_timestamp(1234567890);
```

### InMemoryStore

```rust
use sysml_store::{InMemoryStore, Store, SnapshotMeta};

let mut store = InMemoryStore::new();

// Store a snapshot
let meta = SnapshotMeta::new(CommitId::new("v1"), "First commit");
store.put_snapshot(&project_id, meta, &graph)?;

// Retrieve a snapshot
let snapshot = store.get_snapshot(&project_id, &commit_id)?;
if let Some(snap) = snapshot {
    let graph = snap.graph()?;
}

// Get latest commit
let latest = store.latest(&project_id)?;

// List all commits
let commits = store.list_commits(&project_id)?;
```

## Error Handling

```rust
pub enum StoreError {
    ProjectNotFound(String),
    CommitNotFound(String),
    SerializationError(String),
    DeserializationError(String),
    DatabaseError(String),
    Conflict(String),
}
```

## Dependencies

- `sysml-core`: ModelGraph
- `sysml-canon`: JSON serialization
- `sysml-id`: ProjectId, CommitId
- `thiserror`: Error handling

## Example

```rust
use sysml_store::{Store, InMemoryStore, SnapshotMeta};
use sysml_id::{ProjectId, CommitId};
use sysml_core::{ModelGraph, Element, ElementKind};

// Create store
let mut store = InMemoryStore::new();
let project = ProjectId::new("my-project");

// Create and store a model
let mut graph = ModelGraph::new();
graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Root"));

let meta = SnapshotMeta::new(CommitId::new("v1"), "Initial model");
store.put_snapshot(&project, meta, &graph)?;

// Later: add more elements and create new commit
let meta2 = SnapshotMeta::new(CommitId::new("v2"), "Added requirements")
    .with_parent(CommitId::new("v1"));
store.put_snapshot(&project, meta2, &graph)?;

// List history
for commit in store.list_commits(&project)? {
    println!("{}: {}", commit.commit, commit.message);
}
```
