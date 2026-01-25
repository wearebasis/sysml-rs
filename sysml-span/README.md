# sysml-span

## What This Does (One Sentence)

Tracks where things are in your source files and reports problems to users with helpful error messages.

## The Problem It Solves

When something goes wrong while reading a SysML file, users need to know **exactly where** the problem is. Without this crate, error messages would say things like "invalid syntax" with no location. With it, they say:

```
error[E0001]: undefined element 'Vehicle'
  --> model.sysml:42:12
  = note: did you mean 'Vehicles'?
```

Think of it like **GPS for error messages** — it pinpoints the exact file, line, and column where problems occur.

## How It Works

```
Your SysML file (model.sysml)
─────────────────────────────────
line 41: part engine : Engine;
line 42: part car : Vehicle;      ← Problem here!
                     ^^^^^^^^
                     cols 12-19

                        │
                        ▼
               ┌─────────────────┐
               │      Span       │
               │ ─────────────── │
               │ file: model.sysml
               │ start: byte 847 │
               │ end: byte 854   │
               │ line: 42        │
               │ column: 12      │
               └────────┬────────┘
                        │
                        ▼
    ┌────────────────────────────────────────┐
    │ Diagnostic                              │
    │ ──────────────────────────────────────  │
    │ severity: Error                         │
    │ message: "undefined element 'Vehicle'"  │
    │ code: E0001                             │
    │ span: model.sysml:42:12-19              │
    │ note: "did you mean 'Vehicles'?"        │
    └────────────────────────────────────────┘
                        │
                        ▼
            Shown to the user in their
            terminal or IDE
```

## How It Fits Into the System

This crate sits at the very bottom of the stack — almost every other crate uses it:

```
              ┌─────────────┐
              │  Your Tool  │  (API, LSP, CLI)
              └──────┬──────┘
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
    ┌─────────┐            ┌─────────┐
    │ Parser  │            │  Query  │
    └────┬────┘            └────┬────┘
         │                      │
         │  Both create         │
         │  Diagnostics         │
         └──────────┬───────────┘
                    ▼
              ┌───────────┐
              │ sysml-span│  ← You are here
              └───────────┘
```

## Key Concepts

| Concept | What It Is | Analogy |
|---------|-----------|---------|
| **Span** | A range in a source file (start to end position) | Highlighting text with a marker |
| **Severity** | How serious a problem is (Error, Warning, Info) | Traffic lights (red, yellow, green) |
| **Diagnostic** | A complete problem report with message, location, and hints | A detailed doctor's report |
| **Diagnostics** | A collection of multiple diagnostics | A folder of reports |

## For Developers

<details>
<summary>API Reference (click to expand)</summary>

### Span

```rust
// Create with byte offsets
let span = Span::new("file.sysml", 10, 20);

// Create with line/column info
let span = Span::with_location("file.sysml", 10, 20, 5, 3);

// Point span (single position)
let span = Span::point("file.sysml", 15);

// Operations
span.len();           // byte length
span.contains(15);    // check if offset in range
span.merge(&other);   // combine two spans
```

### Diagnostic

```rust
let diag = Diagnostic::error("parse error")
    .with_code("E001")
    .with_span(span)
    .with_note("hint: check syntax");
```

### Pretty Rendering (feature: pretty)

```rust
use sysml_span::{Diagnostic, HashMapSourceProvider, Span};

let source = "package Demo {\n  part def Engine;\n}\n";
let span = Span::with_location("demo.sysml", 17, 23, 2, 8);
let diag = Diagnostic::error("unexpected token")
    .with_code("E002")
    .with_span(span)
    .with_note("expected 'part def'");

let mut provider = HashMapSourceProvider::new();
provider.insert("demo.sysml", source);

let rendered = diag.render_snippet(&provider);
println!("{rendered}");
```

### Diagnostics Collection

```rust
let mut diags = Diagnostics::new();
diags.error("error 1");
diags.warning("warning 1");
diags.has_errors();   // true
```

### Features

- `serde`: Enable serialization support

</details>
