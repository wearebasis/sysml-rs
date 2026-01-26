# sysml-text-pest

## What This Does (One Sentence)

Reads SysML text files and converts them into structured model data using a fast, native Rust parser.

## The Problem It Solves

SysML files are text — humans can read them, but computers need structure. This crate is the **translator** that converts:

```sysml
package VehicleModel {
    part def Engine;
    part car {
        part engine : Engine;
    }
}
```

Into a structured model that other tools can query, visualize, and execute.

Think of it like **Google Translate for SysML** — it reads the language and converts it into something the rest of the system understands.

## How It Works

```
         ┌────────────────────────────────────────────┐
         │              Your .sysml file              │
         │                                            │
         │  package VehicleModel {                    │
         │      part def Engine;                      │
         │      part car : Vehicle {                  │
         │          part engine : Engine;             │
         │      }                                     │
         │  }                                         │
         └────────────────────┬───────────────────────┘
                              │
                              ▼
         ┌────────────────────────────────────────────┐
         │         Pest Grammar (the rules)           │
         │  ════════════════════════════════════════  │
         │                                            │
         │  "package" must be followed by a name      │
         │  names are letters/numbers/underscores     │
         │  "part def" creates a definition           │
         │  "part" creates a usage                    │
         │  ":" means "typed by"                      │
         │  "{" and "}" group things together         │
         └────────────────────┬───────────────────────┘
                              │
                              ▼
         ┌────────────────────────────────────────────┐
         │              Parse Tree                    │
         │  (raw structure, not yet meaningful)       │
         │                                            │
         │  ├─ Package                                │
         │  │  ├─ name: "VehicleModel"                │
         │  │  └─ body                                │
         │  │     ├─ PartDefinition                   │
         │  │     │  └─ name: "Engine"                │
         │  │     └─ PartUsage                        │
         │  │        ├─ name: "car"                   │
         │  │        └─ ...                           │
         └────────────────────┬───────────────────────┘
                              │
                              ▼
         ┌────────────────────────────────────────────┐
         │              ModelGraph                    │
         │  (structured, queryable model)             │
         │                                            │
         │  Elements:                                 │
         │    [Package "VehicleModel"]                │
         │    [PartDefinition "Engine"]               │
         │    [PartUsage "car"]                       │
         │    [PartUsage "engine"]                    │
         │                                            │
         │  Relationships:                            │
         │    VehicleModel owns Engine, car           │
         │    car owns engine                         │
         │    engine typed by Engine                  │
         └────────────────────────────────────────────┘
```

## How It Fits Into the System

```
    ┌─────────────────────────────────────────────────────┐
    │                Your Application                      │
    │   (API server, LSP, CLI, tests)                     │
    └───────────────────────────┬─────────────────────────┘
                                │
                    uses        │
                                ▼
                        ┌───────────────┐
                        │  sysml-text   │  (Parser interface)
                        └───────┬───────┘
                                │
                    implemented by
                                │
           ┌────────────────────┼────────────────────┐
           ▼                    ▼                    ▼
    ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
    │ sysml-text- │      │   pilot     │      │  other      │
    │    pest     │      │  sidecar    │      │ sidecars    │
    │             │      │             │      │             │
    │  ← YOU ARE  │      │  (Java)     │      │             │
    │    HERE     │      │             │      │             │
    └──────┬──────┘      └─────────────┘      └─────────────┘
           │
           │ produces
           ▼
    ┌─────────────┐
    │ sysml-core  │  (ModelGraph)
    │ ModelGraph  │
    └─────────────┘
```

## Key Concepts

| Concept | What It Is | Analogy |
|---------|-----------|---------|
| **Grammar** | Rules for what valid SysML looks like | Dictionary + grammar book |
| **PEG** | Parsing Expression Grammar — how the rules work | A recipe for reading text |
| **Parse Tree** | Raw structure from reading the text | A diagram of sentence parts |
| **AST Conversion** | Turning parse tree into Elements | Translating diagram to meaning |

### Why "Pest"?

Pest is a parser library for Rust. It's:
- **Fast** — compiles grammar rules into efficient code
- **Safe** — catches grammar bugs at compile time
- **Readable** — grammar files look similar to the language they parse

## For Developers

<details>
<summary>API Reference (click to expand)</summary>

### Basic Usage

```rust
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

let parser = PestParser::new();

let files = vec![SysmlFile::new("example.sysml", r#"
    package Example {
        part def Vehicle;
        part car : Vehicle;
    }
"#)];

let result = parser.parse(&files);
assert!(result.is_ok());
println!("Parsed {} elements", result.graph.element_count());
```

### Build Requirements

This crate needs the official SysML spec files to build:

```bash
# Option 1: Environment variable
export SYSML_REFS_DIR=/path/to/references/sysmlv2
# (legacy name still supported)
export SYSMLV2_REFS_DIR=/path/to/sysmlv2-references

# Option 2: In-repo references (recommended)
ls references/sysmlv2/  # Should exist
```

### Grammar Location

The pest grammar files are in:
```
src/grammar/
├── sysml.pest           # Main entry point
└── fragments/
    ├── definitions.pest  # part def, action def, etc.
    ├── usages.pest       # part, action, flow, etc.
    ├── expressions.pest  # operators, literals
    ├── structure.pest    # package, import
    └── tokens.pest       # keywords, identifiers
```

### Coverage Testing

```bash
# Enable coverage feature for grammar rule tracking
cargo test -p sysml-spec-tests --features coverage
```

### Status

The grammar is broad but AST conversion is still expanding:
- Most element types parse successfully
- Relationship emission is in progress
- Property mapping is being extended

</details>
