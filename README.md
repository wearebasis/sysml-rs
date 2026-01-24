# sysml-rs

A Unix-philosophy SysML v2 ecosystem implemented in Rust.

## What is SysML?

**SysML v2** (Systems Modeling Language) is a standard language for describing complex systems — think vehicles, spacecraft, factories, or software architectures. Engineers write SysML models to:

- Define **parts** and how they connect
- Capture **requirements** and trace them to designs
- Describe **behaviors** like state machines and processes
- Share designs between teams and tools

This project provides Rust libraries and tools to **read, store, query, execute, and visualize** SysML v2 models.

## How the Pieces Fit Together

```
                          Your SysML Files
                     ┌─────────────────────────┐
                     │  package Vehicle {      │
                     │    part engine;         │
                     │    requirement speed;   │
                     │  }                      │
                     └───────────┬─────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                        PARSE (read the text)                     │
│                                                                  │
│  sysml-text-pest reads the file and creates a structured model   │
└─────────────────────────────────┬───────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                     STORE (hold the model)                       │
│                                                                  │
│  sysml-core keeps all elements and relationships in a ModelGraph │
└─────────────────────────────────┬───────────────────────────────┘
                                  │
            ┌─────────────────────┼─────────────────────┐
            │                     │                     │
            ▼                     ▼                     ▼
    ┌───────────────┐     ┌───────────────┐     ┌───────────────┐
    │    QUERY      │     │    EXECUTE    │     │   VISUALIZE   │
    │               │     │               │     │               │
    │  Find parts,  │     │ Run state     │     │ Export to     │
    │  trace reqs,  │     │ machines,     │     │ diagrams,     │
    │  filter by    │     │ evaluate      │     │ DOT, JSON     │
    │  properties   │     │ constraints   │     │               │
    │               │     │               │     │               │
    │  sysml-query  │     │  sysml-run-*  │     │  sysml-vis    │
    └───────────────┘     └───────────────┘     └───────────────┘
            │                     │                     │
            └─────────────────────┼─────────────────────┘
                                  │
                                  ▼
                    ┌─────────────────────────┐
                    │         SERVE           │
                    │                         │
                    │  REST API for external  │
                    │  tools and integrations │
                    │                         │
                    │       sysml-api         │
                    └─────────────────────────┘
```

## Quick Start

### For Users (Conceptual Overview)

1. **Write your model** in `.sysml` files using the SysML v2 textual syntax
2. **Parse it** using `sysml-text-pest` to get a structured `ModelGraph`
3. **Query it** using `sysml-query` to find elements, trace requirements, etc.
4. **Execute it** using `sysml-run-statemachine` to simulate behaviors
5. **Visualize it** using `sysml-vis` to export diagrams
6. **Serve it** using `sysml-api` to make it available over HTTP

### For Developers

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Build with serialization support
cargo build --features sysml-core/serde
```

## Architecture

This workspace follows a layered architecture with clean crate boundaries. Each crate has a single responsibility and can be used independently.

```
┌─────────────────────────────────────────────────────────────────────┐
│                            API LAYER                                │
│                           sysml-api                                 │
└─────────────────────────────────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
┌───────────────┐   ┌─────────────────────┐   ┌─────────────────────┐
│    STORAGE    │   │      EXECUTION      │   │    VISUALIZATION    │
│ sysml-store   │   │     sysml-run       │   │     sysml-vis       │
│ sysml-store-  │   │ sysml-run-state-    │   └─────────────────────┘
│   postgres    │   │   machine           │
└───────────────┘   │ sysml-run-          │
        │           │   constraints       │
        │           └─────────────────────┘
        │                    │
        ▼                    ▼
┌───────────────────────────────────────────────────────────────────────┐
│                          SEMANTIC CORE                                │
│              sysml-core    sysml-query    sysml-canon                 │
└───────────────────────────────────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
┌───────────────┐   ┌─────────────────────┐   ┌─────────────────────┐
│  FOUNDATIONS  │   │   TEXT FRONTEND     │   │        IDE          │
│  sysml-id     │   │   sysml-text        │   │     sysml-ts        │
│  sysml-span   │   │ sysml-text-pest     │   │     sysml-lsp       │
│  sysml-meta   │   │ + sidecar adapters  │   │  sysml-lsp-server   │
└───────────────┘   └─────────────────────┘   └─────────────────────┘
```

## Crate Overview

### Foundations (Bottom Layer)

These crates have minimal dependencies and provide basic building blocks.

| Crate | What It Does |
|-------|-------------|
| `sysml-id` | Unique identifiers for elements, projects, and commits |
| `sysml-span` | Source locations and error reporting (see [README](sysml-span/README.md)) |
| `sysml-meta` | Metadata types like applicability and clause references |

### Semantic Core (Middle Layer)

These crates define how SysML models are represented and manipulated.

| Crate | What It Does |
|-------|-------------|
| `sysml-core` | The model database — elements, relationships, and the ModelGraph (see [README](sysml-core/README.md)) |
| `sysml-query` | Functions to search and filter the ModelGraph |
| `sysml-canon` | Canonical JSON serialization with stable ordering |

### Text Frontend

These crates handle reading SysML text files.

| Crate | What It Does |
|-------|-------------|
| `sysml-text` | Parser interface that all parsers implement (see [README](sysml-text/README.md)) |
| `sysml-text-pest` | Native Rust parser using pest grammar (see [README](sysml-text-pest/README.md)) |
| `sysml-text-*-sidecar` | Adapters for external parsers (Pilot, MontiCore, SySide) |

### IDE Support

These crates enable editor integration.

| Crate | What It Does |
|-------|-------------|
| `sysml-ts` | Tree-sitter CST parsing for fast IDE feedback |
| `sysml-lsp` | LSP protocol types |
| `sysml-lsp-server` | Full Language Server Protocol implementation |

### Higher Layers

| Crate | What It Does |
|-------|-------------|
| `sysml-vis` | Export to DOT, PlantUML, Cytoscape JSON |
| `sysml-run` | Runner trait for executables |
| `sysml-run-statemachine` | State machine compilation and execution |
| `sysml-run-constraints` | Constraint evaluation |
| `sysml-store` | Storage trait for model snapshots |
| `sysml-store-postgres` | PostgreSQL backend |
| `sysml-api` | REST API server |

### Testing

| Crate | What It Does |
|-------|-------------|
| `codegen` | Build-time code generation from spec files |
| `sysml-spec-tests` | Parser validation against official corpus (see [README](sysml-spec-tests/README.md)) |

## Extending

### Adding a New Parser Backend

1. Create a new crate depending on `sysml-text`
2. Implement the `Parser` trait
3. Wire up your transport (HTTP, CLI, etc.)

### Adding a New Storage Backend

1. Create a new crate depending on `sysml-store`
2. Implement the `Store` trait
3. Use `sysml-canon` for serialization

### Adding a New Visualization Format

1. Add a new export function to `sysml-vis`
2. Or create a new crate depending on `sysml-core`

## Feature Flags

| Flag | Crate | Description |
|------|-------|-------------|
| `serde` | `sysml-id`, `sysml-span`, `sysml-meta`, `sysml-core` | Enable serde serialization |
| `uuid` | `sysml-id` | Use UUID for ElementId (default) |
| `postgres` | `sysml-store-postgres` | Enable PostgreSQL support |

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
