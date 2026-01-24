# sysml-text

## What This Does (One Sentence)

Defines a standard interface so that different SysML parsers can plug into the system interchangeably.

## The Problem It Solves

There are multiple ways to parse SysML files:

- A fast native Rust parser (for production)
- The official Java "Pilot" parser (for compatibility)
- Other parsers for specific use cases

Without a common interface, each tool would need to know about every parser's quirks. With this crate, **any parser can plug in** as long as it speaks the same language.

Think of it like a **universal power adapter** — it doesn't matter if the parser is built in Rust, Java, or JavaScript; if it implements this interface, it works.

## How It Works

```
                    ┌─────────────────────────────────────┐
                    │           Your Application          │
                    │                                     │
                    │   parser.parse(&files) ─────────┐   │
                    │                                 │   │
                    └─────────────────────────────────│───┘
                                                      │
                                                      ▼
                         ┌────────────────────────────────────┐
                         │        Parser Trait (sysml-text)   │
                         │  ════════════════════════════════  │
                         │  parse(&[SysmlFile]) → ParseResult │
                         │  name() → &str                     │
                         │  version() → &str                  │
                         └───────────────────┬────────────────┘
                                             │
             Implements                      │           Implements
                  │                          │                │
    ┌─────────────┴────┐     ┌──────────────┴───┐    ┌──────┴───────────┐
    │                  │     │                  │    │                  │
    ▼                  ▼     ▼                  ▼    ▼                  ▼
┌────────────┐    ┌────────────┐          ┌────────────┐         ┌────────────┐
│  Pest      │    │   Pilot    │          │ MontiCore  │         │   SySide   │
│ (Native)   │    │  (Java)    │          │  (Java)    │         │ (Node.js)  │
│            │    │            │          │            │         │            │
│ Fast,      │    │ Official   │          │ Academic   │         │ VS Code    │
│ embedded   │    │ reference  │          │ research   │         │ extension  │
└────────────┘    └────────────┘          └────────────┘         └────────────┘
```

## How It Fits Into the System

```
    ┌─────────────────────────────────────────────────────┐
    │                Higher-Level Tools                    │
    │   (API server, LSP server, CLI tools, tests)        │
    └───────────────────────────┬─────────────────────────┘
                                │
                    They call   │  parser.parse(files)
                                │
                                ▼
                        ┌───────────────┐
                        │  sysml-text   │  ← You are here
                        │  Parser trait │
                        └───────┬───────┘
                                │
              implementations   │
                                │
         ┌──────────────────────┼──────────────────────┐
         ▼                      ▼                      ▼
    ┌─────────┐           ┌─────────┐           ┌─────────┐
    │  pest   │           │  pilot  │           │ others  │
    │ parser  │           │ sidecar │           │   ...   │
    └─────────┘           └─────────┘           └─────────┘
```

## Key Concepts

| Concept | What It Is | Analogy |
|---------|-----------|---------|
| **SysmlFile** | A file to parse (path + content) | A document handed to a translator |
| **ParseResult** | What the parser returns (model + errors) | The translation plus any notes about problems |
| **Parser** | Something that reads SysML text | A translator who speaks SysML |
| **Sidecar** | A wrapper around an external parser | A translator who calls another translator |

## For Developers

<details>
<summary>API Reference (click to expand)</summary>

### Using a Parser

```rust
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;  // or any other parser

let parser = PestParser::new();

let files = vec![
    SysmlFile::new("a.sysml", "package A {}"),
    SysmlFile::new("b.sysml", "package B {}"),
];

let result = parser.parse(&files);

if result.is_ok() {
    println!("Parsed {} elements", result.graph.element_count());
} else {
    for diag in &result.diagnostics {
        eprintln!("{}", diag);
    }
}
```

### Implementing a Parser

```rust
use sysml_text::{Parser, ParseResult, SysmlFile};

struct MyParser;

impl Parser for MyParser {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Your parsing logic here
        // Return ParseResult with graph and diagnostics
        todo!()
    }

    fn name(&self) -> &str {
        "my-parser"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}
```

### Built-in Test Parsers

```rust
// Returns an error (for testing error handling)
let parser = NoopParser::new();

// Returns empty success (for testing happy path)
let parser = StubParser::new();
```

### ParseResult

```rust
// Check for success
result.is_ok();
result.has_errors();
result.error_count();

// Access the model
let graph = result.graph;

// Access diagnostics
for diag in result.diagnostics {
    println!("{}", diag);
}
```

### Dependencies

- `sysml-core`: For ModelGraph
- `sysml-span`: For Diagnostic types

</details>
