# Dependency Rules

This document describes the strict layering and dependency rules for the sysml-rs workspace.

## Dependency Diagram

```
                                 ┌─────────────┐
                                 │  sysml-api  │
                                 └──────┬──────┘
                                        │
                    ┌───────────────────┼───────────────────┐
                    │                   │                   │
                    ▼                   ▼                   ▼
        ┌───────────────────┐  ┌───────────────┐  ┌───────────────┐
        │ sysml-store-      │  │ sysml-run-    │  │ sysml-run-    │
        │   postgres        │  │  statemachine │  │  constraints  │
        └─────────┬─────────┘  └───────┬───────┘  └───────┬───────┘
                  │                    │                   │
                  ▼                    ├───────────────────┘
        ┌───────────────────┐          │
        │   sysml-store     │          ▼
        └─────────┬─────────┘  ┌───────────────┐  ┌───────────────┐
                  │            │  sysml-query  │  │   sysml-run   │
                  │            └───────┬───────┘  └───────┬───────┘
                  │                    │                   │
                  ▼                    └─────────┬─────────┘
        ┌───────────────────┐                    │
        │   sysml-canon     │◄───────────────────┤
        └─────────┬─────────┘                    │
                  │                              │
                  │           ┌───────────────┐  │
                  │           │   sysml-vis   │──┤
                  │           └───────────────┘  │
                  │                              │
                  │    ┌────────────────────────┐│
                  │    │    sysml-lsp-server    ││
                  │    └────────────┬───────────┘│
                  │                 │            │
                  │    ┌────────────┼────────────┤
                  │    │            │            │
                  │    ▼            ▼            ▼
                  │  ┌──────┐  ┌─────────┐  ┌─────────────┐
                  │  │sysml-│  │sysml-lsp│  │ sysml-text  │
                  │  │  ts  │  └────┬────┘  └──────┬──────┘
                  │  └──────┘       │              │
                  │                 │    ┌─────────┼─────────┐
                  │                 │    │         │         │
                  │                 │    ▼         ▼         ▼
                  │                 │  pilot   monticore  syside
                  │                 │ sidecar   sidecar   sidecar
                  │                 │
                  ▼                 │
        ┌───────────────────┐       │
        │    sysml-core     │◄──────┴───────────────────────────────
        └─────────┬─────────┘
                  │
    ┌─────────────┼─────────────┐
    │             │             │
    ▼             ▼             ▼
┌─────────┐  ┌─────────┐  ┌─────────┐
│sysml-id │  │sysml-   │  │sysml-   │
│         │  │  span   │  │  meta   │
└─────────┘  └─────────┘  └─────────┘
    │             │             │
    └─────────────┼─────────────┘
                  │
                  ▼
              std only
              (+ optional serde)
```

## Layering Rules

### Layer 0: Foundations (Zero External Dependencies)

**Crates:** `sysml-id`, `sysml-span`, `sysml-meta`

**Rules:**
- May only depend on `std`
- May optionally depend on `serde` behind a feature flag
- No internal crate dependencies

### Layer 1: Semantic Core

**Crates:** `sysml-core`, `sysml-query`, `sysml-canon`

**Rules:**
- `sysml-core`: depends only on `sysml-id`, `sysml-span`, `sysml-meta`
- `sysml-query`: depends only on `sysml-core`
- `sysml-canon`: depends on `sysml-core`, `serde`, `serde_json`

### Layer 2: Text Frontend

**Crates:** `sysml-text`, `sysml-text-*-sidecar`

**Rules:**
- `sysml-text`: depends on `sysml-core`, `sysml-span`
- Sidecar crates: depend only on `sysml-text`

### Layer 2: IDE (Parallel to Text Frontend)

**Crates:** `sysml-ts`, `sysml-lsp`, `sysml-lsp-server`

**Rules:**
- `sysml-ts`: **NO** dependency on `sysml-core` (CST only)
- `sysml-ts`: depends only on `sysml-span`
- `sysml-lsp`: depends on `sysml-span`, `sysml-id`, optionally `sysml-core`
- `sysml-lsp-server`: depends on `sysml-lsp`, `sysml-text`, `sysml-ts`, `sysml-span`, `tower-lsp`, `tokio`

### Layer 3: Visualization & Execution

**Crates:** `sysml-vis`, `sysml-run`, `sysml-run-statemachine`, `sysml-run-constraints`

**Rules:**
- `sysml-vis`: depends on `sysml-core` (plus `serde_json` for export formats)
- `sysml-run`: depends on `sysml-core`, `sysml-span`
- `sysml-run-statemachine`: depends on `sysml-run`, `sysml-query`, `sysml-core`, `sysml-span`
- `sysml-run-constraints`: depends on `sysml-run`, `sysml-query`, `sysml-core`, `sysml-span`

### Layer 4: Storage

**Crates:** `sysml-store`, `sysml-store-postgres`

**Rules:**
- `sysml-store`: depends on `sysml-core`, `sysml-canon`
- `sysml-store-postgres`: depends on `sysml-store`, optionally `sqlx`

### Layer 5: API

**Crates:** `sysml-api`

**Rules:**
- May depend on any lower layer
- Primary dependencies: `sysml-store`, `sysml-query`

## External Dependency Policy

| Dependency | Allowed In | Notes |
|------------|------------|-------|
| `serde` | All crates | Feature-gated in foundations |
| `serde_json` | `sysml-canon`, `sysml-vis`, `sysml-api` | Serialization layer |
| `uuid` | `sysml-id` | Feature-gated |
| `tower-lsp` | `sysml-lsp-server` only | LSP protocol |
| `axum` | `sysml-api` only | HTTP server |
| `tokio` | `sysml-api`, `sysml-store-postgres`, `sysml-lsp-server` | Async runtime |
| `sqlx` | `sysml-store-postgres` only | Feature-gated |
| `thiserror` | All crates | Error handling |

## Adding New Crates

When adding a new crate:

1. Identify which layer it belongs to
2. Only depend on crates from lower layers
3. Keep external dependencies minimal
4. Feature-gate optional dependencies
5. Update this document

## Circular Dependency Prevention

The layered architecture prevents circular dependencies:
- Lower layers never depend on higher layers
- Sibling crates at the same layer should avoid mutual dependencies
- If two crates need to share types, extract to a lower layer
