# sysml-store-postgres

PostgreSQL storage backend for SysML v2 model snapshots.

## Purpose

This crate provides a PostgreSQL implementation of the Store trait. When the `postgres` feature is disabled, it provides an in-memory fallback.

## Features

- `postgres`: Enable PostgreSQL support via sqlx (disabled by default)

## Public API

### Without postgres feature (default)

```rust
use sysml_store_postgres::create_in_memory_store;

let store = create_in_memory_store();
```

### With postgres feature

```rust
use sysml_store_postgres::PostgresStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = PostgresStore::new("postgres://user:pass@localhost/sysml").await?;
    store.init_schema().await?;

    // Store a snapshot
    store.put_snapshot_async(&project, meta, &graph).await?;

    // Retrieve a snapshot
    let snapshot = store.get_snapshot_async(&project, &commit).await?;

    // Get latest commit
    let latest = store.latest_async(&project).await?;

    Ok(())
}
```

## Database Schema

When using the postgres feature, the following schema is created:

```sql
CREATE TABLE snapshots (
    project_id TEXT NOT NULL,
    commit_id TEXT NOT NULL,
    parent_id TEXT,
    message TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    data JSONB NOT NULL,
    PRIMARY KEY (project_id, commit_id)
);

CREATE INDEX idx_snapshots_project
ON snapshots (project_id, timestamp DESC);
```

## Dependencies

- `sysml-store`: Store trait and types
- `sysml-core`: ModelGraph
- `sysml-id`: ProjectId, CommitId
- `sysml-canon`: JSON serialization
- `sqlx` (optional): PostgreSQL driver
- `tokio` (optional): Async runtime

## Usage

### Cargo.toml

```toml
# Without PostgreSQL
sysml-store-postgres = "0.1"

# With PostgreSQL
sysml-store-postgres = { version = "0.1", features = ["postgres"] }
```

## Example

```rust
use sysml_store_postgres::create_in_memory_store;
use sysml_store::{Store, SnapshotMeta};
use sysml_id::{ProjectId, CommitId};
use sysml_core::{ModelGraph, Element, ElementKind};

// Create store (in-memory fallback)
let mut store = create_in_memory_store();

// Create and store a model
let project = ProjectId::new("vehicle-model");
let mut graph = ModelGraph::new();
graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Vehicle"));

let meta = SnapshotMeta::new(CommitId::new("v1.0.0"), "Initial vehicle model");
store.put_snapshot(&project, meta, &graph).unwrap();

// Retrieve
let snapshot = store.get_snapshot(&project, &CommitId::new("v1.0.0")).unwrap();
```
