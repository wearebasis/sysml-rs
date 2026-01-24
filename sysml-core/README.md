# sysml-core

## What This Does (One Sentence)

Stores all the parts, connections, and requirements from your SysML models in an organized structure that other tools can query and analyze.

## The Problem It Solves

A SysML model describes complex systems — vehicles with engines, software with requirements, factories with processes. After reading the text file, we need somewhere to **store all that information** in a way that:

- Lets us find things quickly ("show me all parts called 'engine'")
- Tracks how things connect ("what does this requirement apply to?")
- Preserves the structure ("this part is inside that package")

Without this crate, every tool would need to re-parse the original file every time. With it, we have a **central model database** that everything can use.

Think of it like a **blueprint filing system** — it holds all the pieces of your design and remembers how they connect.

## How It Works

```
                   SysML Text File
    ┌─────────────────────────────────────────┐
    │  package VehicleModel {                 │
    │      part def Engine;                   │
    │      part def Wheel;                    │
    │      part car {                         │
    │          part engine : Engine;          │
    │          part wheels : Wheel[4];        │
    │      }                                  │
    │  }                                      │
    └─────────────────────────────────────────┘
                        │
                        │  Parser reads this
                        ▼
    ┌─────────────────────────────────────────┐
    │              ModelGraph                  │
    │  ═══════════════════════════════════════ │
    │                                          │
    │  ELEMENTS (the things):                  │
    │  ┌────────────┬─────────────────────┐   │
    │  │ ID         │ What it is          │   │
    │  ├────────────┼─────────────────────┤   │
    │  │ #001       │ Package "VehicleModel"│  │
    │  │ #002       │ PartDefinition "Engine"│ │
    │  │ #003       │ PartDefinition "Wheel"│  │
    │  │ #004       │ PartUsage "car"       │  │
    │  │ #005       │ PartUsage "engine"    │  │
    │  │ #006       │ PartUsage "wheels"    │  │
    │  └────────────┴─────────────────────┘   │
    │                                          │
    │  RELATIONSHIPS (how they connect):       │
    │  ┌───────────────────────────────────┐  │
    │  │ #001 owns #002, #003, #004        │  │
    │  │ #004 owns #005, #006              │  │
    │  │ #005 is typed by #002             │  │
    │  │ #006 is typed by #003             │  │
    │  └───────────────────────────────────┘  │
    └─────────────────────────────────────────┘
```

## How It Fits Into the System

This crate is the **heart of the system** — almost everything depends on it:

```
    ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
    │  Query  │  │   API   │  │Visualize│  │ Execute │
    │ "find X"│  │  REST   │  │  export │  │  state  │
    └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘
         │            │            │            │
         └────────────┴────────────┴────────────┘
                           │
                           ▼
                   ┌───────────────┐
                   │  sysml-core   │  ← You are here
                   │  ModelGraph   │
                   └───────┬───────┘
                           │
              filled by    │
                           │
         ┌─────────────────┴─────────────────┐
         ▼                                   ▼
    ┌─────────┐                        ┌─────────┐
    │ Parser  │  (reads .sysml files)  │  JSON   │ (loads saved models)
    └─────────┘                        └─────────┘
```

## Key Concepts

| Concept | What It Is | Real-World Analogy |
|---------|-----------|-------------------|
| **Element** | A thing in your model (part, requirement, action, etc.) | A component in a blueprint |
| **ElementKind** | The type of element (266 official types!) | Categories like "electrical", "mechanical" |
| **Relationship** | A connection between elements | Lines connecting components |
| **ModelGraph** | Container holding all elements and relationships | The complete blueprint folder |

### Element Kinds (Examples)

SysML v2 defines 266 element kinds. Here are some common ones:

- `Package` — A folder for organizing things
- `PartDefinition` — A blueprint for a part ("Engine" as a concept)
- `PartUsage` — An actual part ("the engine in my car")
- `RequirementDefinition` — A rule the system must follow
- `ActionDefinition` — A behavior or process

## For Developers

<details>
<summary>API Reference (click to expand)</summary>

### Creating Elements

```rust
use sysml_core::{Element, ElementKind, ModelGraph, VisibilityKind};

let mut graph = ModelGraph::new();

// Create a package
let pkg = Element::new_with_kind(ElementKind::Package)
    .with_name("VehicleModel");
let pkg_id = graph.add_element(pkg);

// Create a part definition owned by the package
let engine = Element::new_with_kind(ElementKind::PartDefinition)
    .with_name("Engine");
let engine_id = graph.add_owned_element(engine, pkg_id, VisibilityKind::Public);
```

### Querying Elements

```rust
// By ID
let elem = graph.get_element(&some_id);

// By kind
let all_parts = graph.elements_by_kind(ElementKind::PartUsage);
```

### ElementKind Operations

```rust
let kind: ElementKind = "PartUsage".parse().unwrap();
kind.is_usage();       // true
kind.is_definition();  // false
kind.is_subtype_of(ElementKind::Feature);  // true
```

### Features

- `serde`: Enable serialization support

### Dependencies

- `sysml-id`: For element identifiers
- `sysml-span`: For source locations
- `sysml-meta`: For metadata values

</details>
