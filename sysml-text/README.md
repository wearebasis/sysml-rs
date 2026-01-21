# sysml-text

Parser trait and result types for SysML v2 text parsing.

## Purpose

This crate defines the interface for parsing SysML v2 textual notation:

- **SysmlFile**: Input file with path and text content
- **ParseResult**: Output containing ModelGraph and diagnostics
- **Parser**: Trait for parser implementations
- **Formatter**: Trait for formatter implementations

Parsing implementations live in separate crates:

- `sysml-text-pest`: native Rust parser using pest
- `sysml-text-*-sidecar`: adapters for external parsers (Pilot, MontiCore, SySide)

## Public API

### Types

```rust
// Input file
let file = SysmlFile::new("model.sysml", "package MyModel {}");

// Parse result
let result = ParseResult::success(graph);
let result = ParseResult::error("parse failed");

result.is_ok();       // true if no errors
result.has_errors();  // true if any errors
result.error_count(); // number of errors
```

### Parser Trait

```rust
pub trait Parser {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}
```

### Formatter Trait

```rust
pub trait Formatter {
    fn format(&self, graph: &ModelGraph) -> String;
}
```

### Built-in Parsers

```rust
// No-op parser (returns error)
let parser = NoopParser::new();

// Stub parser (returns empty success)
let parser = StubParser::new();
```

## Dependencies

- `sysml-core`: For ModelGraph
- `sysml-span`: For Diagnostic

## Implementing a Parser

To implement a parser sidecar:

```rust
use sysml_text::{Parser, ParseResult, SysmlFile};
use sysml_core::ModelGraph;

struct MyParser;

impl Parser for MyParser {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Call external parser (JVM, HTTP, etc.)
        // Convert results to ModelGraph
        // Return ParseResult with graph and diagnostics
        todo!()
    }

    fn name(&self) -> &str {
        "my-parser"
    }
}
```

## Example

```rust
use sysml_text::{Parser, SysmlFile, StubParser};

let parser = StubParser::new();
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
