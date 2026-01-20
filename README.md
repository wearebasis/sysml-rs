# sysml-rs

A Unix-philosophy SysML v2 ecosystem implemented in Rust.

## Architecture

This workspace follows a layered architecture with clean crate boundaries and minimal dependencies. Each crate has a single responsibility and can be used independently.

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
│  sysml-span   │   │ + sidecar adapters  │   │     sysml-lsp       │
│  sysml-meta   │   └─────────────────────┘   │  sysml-lsp-server   │
└───────────────┘                             └─────────────────────┘
```

## Crate Map

### Foundations
| Crate | Purpose |
|-------|---------|
| `sysml-id` | Element identifiers, qualified names, project/commit IDs |
| `sysml-span` | Source locations, diagnostics, severity levels |
| `sysml-meta` | Metadata types: applicability, clause references, values |

### Semantic Core
| Crate | Purpose |
|-------|---------|
| `sysml-core` | Core model types: Element, Relationship, ModelGraph |
| `sysml-query` | Query functions over ModelGraph |
| `sysml-canon` | Canonical JSON serialization with stable ordering |

### Text Frontend
| Crate | Purpose |
|-------|---------|
| `sysml-text` | Parser trait and result types |
| `sysml-text-pilot-sidecar` | Adapter for Pilot parser (JVM/HTTP) |
| `sysml-text-monticore-sidecar` | Adapter for MontiCore parser (JVM/HTTP) |
| `sysml-text-syside-sidecar` | Adapter for SySide parser (Node.js) |

### IDE Support
| Crate | Purpose |
|-------|---------|
| `sysml-ts` | Tree-sitter based CST parsing (fast, IDE-focused) |
| `sysml-lsp` | LSP protocol types and conversions |
| `sysml-lsp-server` | Full LSP server implementation |

### Visualization
| Crate | Purpose |
|-------|---------|
| `sysml-vis` | Export to DOT, PlantUML, Cytoscape JSON |

### Execution
| Crate | Purpose |
|-------|---------|
| `sysml-run` | Runner trait and IR base types |
| `sysml-run-statemachine` | State machine compilation and execution |
| `sysml-run-constraints` | Constraint evaluation |

### Storage & API
| Crate | Purpose |
|-------|---------|
| `sysml-store` | Storage trait for model snapshots |
| `sysml-store-postgres` | PostgreSQL implementation |
| `sysml-api` | REST/GraphQL API server |

## Getting Started

```bash
# Build all crates
cargo build

# Run all tests
cargo test

# Build with specific features
cargo build --features sysml-core/serde
```

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
