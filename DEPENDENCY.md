# Dependency Rules

This document describes the strict layering and dependency rules for the sysml-rs workspace.

## Why Layers?

The crates in this workspace are organized into **layers** — lower layers don't know about higher layers. This design provides several benefits:

| Benefit | What It Means |
|---------|--------------|
| **Faster builds** | Change a high-level crate? Only rebuild that layer and above. |
| **Easier testing** | Test `sysml-core` without starting an HTTP server. |
| **Flexible deployment** | Use just the parser without the REST API. |
| **Clear contracts** | Each layer has a defined interface. |
| **No circular deps** | The build always succeeds in one pass. |

## The Layers (Simple View)

```
    Layer 5: API          ← Brings it all together (HTTP server)
              │
    Layer 4: Storage      ← Saves models to databases
              │
    Layer 3: Features     ← Visualization, execution, IDE support
              │
    Layer 2: Text         ← Reads .sysml files
              │
    Layer 1: Core         ← The model database (elements, relationships)
              │
    Layer 0: Foundations  ← Basic building blocks (IDs, spans, metadata)
              │
            (std)         ← Just the Rust standard library
```

**The Rule:** Each layer can only depend on layers below it. Never above. Never sideways (mostly).

## Layer-by-Layer Explanation

### Layer 0: Foundations

**Crates:** `sysml-id`, `sysml-span`, `sysml-meta`

**What they do:**
- `sysml-id` — Creates unique identifiers for things
- `sysml-span` — Tracks source locations for error messages
- `sysml-meta` — Holds metadata values (strings, numbers, booleans)

**Why they're at the bottom:** Everything needs IDs and error locations. These crates are tiny and compile fast.

**Dependencies:** Only `std` (and optionally `serde` for serialization).

---

### Layer 1: Semantic Core

**Crates:** `sysml-core`, `sysml-query`, `sysml-canon`

**What they do:**
- `sysml-core` — The main model: Elements, Relationships, ModelGraph
- `sysml-query` — Functions to search the model
- `sysml-canon` — Converts models to JSON

**Why they're here:** Once you have IDs and spans, you can define what a "model" actually is.

**Dependencies:**
- `sysml-core` → `sysml-id`, `sysml-span`, `sysml-meta`
- `sysml-query` → `sysml-core`
- `sysml-canon` → `sysml-core`, `serde`, `serde_json`

---

### Layer 2: Text Frontend

**Crates:** `sysml-text`, `sysml-text-pest`, sidecar adapters

**What they do:**
- `sysml-text` — Defines the `Parser` interface
- `sysml-text-pest` — Native parser implementation
- Sidecars — Adapters for external parsers (Java, Node.js)

**Why they're here:** Parsers produce ModelGraphs, so they need `sysml-core`.

**Dependencies:**
- `sysml-text` → `sysml-core`, `sysml-span`
- Sidecars → `sysml-text` only

---

### Layer 2 (Parallel): IDE

**Crates:** `sysml-ts`, `sysml-lsp`, `sysml-lsp-server`

**What they do:**
- `sysml-ts` — Fast CST parsing for editors
- `sysml-lsp` — LSP protocol types
- `sysml-lsp-server` — Full editor integration

**Why they're parallel:** IDE tools need speed over completeness. `sysml-ts` intentionally does NOT depend on `sysml-core` — it's just for syntax highlighting and fast feedback.

---

### Layer 3: Features

**Crates:** `sysml-vis`, `sysml-run`, `sysml-run-statemachine`, `sysml-run-constraints`

**What they do:**
- `sysml-vis` — Export diagrams
- `sysml-run` — Define what "running" a model means
- Runners — Actually execute state machines and constraints

**Why they're here:** They operate on complete models from `sysml-core`.

---

### Layer 4: Storage

**Crates:** `sysml-store`, `sysml-store-postgres`

**What they do:** Save and load models from databases.

**Why they're here:** Storage needs serialization (`sysml-canon`) and the model types (`sysml-core`).

---

### Layer 5: API

**Crates:** `sysml-api`

**What it does:** HTTP REST server that ties everything together.

**Why it's at the top:** It can use anything below — parsing, querying, storage, visualization.

---

## Dependency Diagram (Detailed)

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

## External Dependency Policy

| Dependency | Allowed In | Why |
|------------|------------|-----|
| `serde` | All crates | Standard serialization (feature-gated in foundations) |
| `serde_json` | `sysml-canon`, `sysml-vis`, `sysml-api` | JSON output |
| `uuid` | `sysml-id` | Unique identifiers (feature-gated) |
| `tower-lsp` | `sysml-lsp-server` only | LSP protocol implementation |
| `axum` | `sysml-api` only | HTTP server framework |
| `tokio` | `sysml-api`, `sysml-store-postgres`, `sysml-lsp-server` | Async runtime |
| `sqlx` | `sysml-store-postgres` only | Database access (feature-gated) |
| `thiserror` | All crates | Clean error types |

## Adding New Crates

When adding a new crate:

1. **Identify the layer** — What does it need? What provides it?
2. **Only depend downward** — Never on crates above your layer
3. **Minimize externals** — Each new dependency is a build cost
4. **Feature-gate optionals** — Don't force users to compile what they don't use
5. **Update this document** — Keep the rules documented

## Common Mistakes to Avoid

| Mistake | Why It's Bad | Solution |
|---------|-------------|----------|
| `sysml-ts` depending on `sysml-core` | Defeats the "fast IDE" purpose | Keep CST-only for speed |
| Foundation crates depending on `serde` unconditionally | Forces everyone to compile serde | Feature-gate it |
| Circular dependencies between siblings | Breaks the build | Extract shared types to lower layer |
| `sysml-core` depending on `sysml-text` | Inverts the natural flow | Parser produces core, not vice versa |
