# sysml-api

REST API server for SysML v2 models.

## Purpose

This crate provides a minimal HTTP API for model storage and retrieval using axum.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/projects` | List all projects |
| GET | `/projects/:id/commits` | List commits for a project |
| GET | `/projects/:id/commits/:commit/model` | Get model snapshot (JSON) |
| POST | `/projects/:id/commits/:commit/model` | Store model snapshot |

## Public API

### Running the Server

```rust
use sysml_api::run_server;

#[tokio::main]
async fn main() {
    run_server("0.0.0.0:3000").await.unwrap();
}
```

### Creating a Router

```rust
use sysml_api::{create_router, AppState};
use std::sync::Arc;

let state = Arc::new(AppState::new());
let router = create_router(state);
```

### With Existing Store

```rust
use sysml_api::AppState;
use sysml_store::InMemoryStore;

let store = InMemoryStore::new();
// ... populate store ...
let state = AppState::with_store(store);
```

## Request/Response Examples

### Health Check

```http
GET /health

{
  "status": "ok",
  "version": "0.1.0"
}
```

### List Projects

```http
GET /projects

{
  "projects": ["project-a", "project-b"]
}
```

### List Commits

```http
GET /projects/my-project/commits

{
  "commits": [
    {
      "id": "v2",
      "parent": "v1",
      "message": "Added requirements",
      "timestamp": 1234567890
    },
    {
      "id": "v1",
      "parent": null,
      "message": "Initial commit",
      "timestamp": 1234567800
    }
  ]
}
```

### Get Model

```http
GET /projects/my-project/commits/v1/model

{
  "version": "1.0",
  "elements": [...],
  "relationships": [...]
}
```

### Store Model

```http
POST /projects/my-project/commits/v2/model
Content-Type: application/json

{
  "message": "Added engine part",
  "parent": "v1",
  "model": {
    "version": "1.0",
    "elements": [...],
    "relationships": [...]
  }
}
```

## Dependencies

- `sysml-store`: Storage trait and InMemoryStore
- `sysml-query`: Query functions
- `sysml-core`: ModelGraph
- `sysml-canon`: JSON serialization
- `sysml-id`: ProjectId, CommitId
- `axum`: HTTP framework
- `tokio`: Async runtime

## Example

```rust
use sysml_api::{create_router, AppState};
use sysml_store::{Store, SnapshotMeta};
use sysml_core::{ModelGraph, Element, ElementKind};
use sysml_id::{ProjectId, CommitId};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create state with pre-populated store
    let mut store = sysml_store::InMemoryStore::new();

    let mut graph = ModelGraph::new();
    graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Demo"));

    store.put_snapshot(
        &ProjectId::new("demo"),
        SnapshotMeta::new(CommitId::new("v1"), "Initial"),
        &graph,
    ).unwrap();

    let state = Arc::new(AppState::with_store(store));
    let app = create_router(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```
