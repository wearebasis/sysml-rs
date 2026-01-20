# sysml-lsp

LSP protocol types and conversions for SysML v2.

## Purpose

This crate provides minimal LSP-compatible types without pulling in the full lsp-types crate. It handles conversions from sysml-span diagnostics to LSP format.

## Public API

### Diagnostic Conversion

```rust
use sysml_lsp::LspDiagnostic;
use sysml_span::Diagnostic;

let sysml_diag = Diagnostic::error("undefined symbol")
    .with_span(span)
    .with_code("E001");

let lsp_diag = LspDiagnostic::from_sysml(&sysml_diag, &source_text);
```

### Position and Range

```rust
use sysml_lsp::{Position, Range};

let pos = Position::new(10, 5);  // line 10, character 5
let range = Range::new(start, end);
let range = Range::from_span(&span, &source);
```

### Document Symbols

```rust
use sysml_lsp::{DocumentSymbol, SymbolKind};

let symbol = DocumentSymbol {
    name: "MyPackage".to_string(),
    detail: None,
    kind: SymbolKind::Package,
    range,
    selection_range,
    children: vec![],
};
```

### Symbol Kinds

```rust
pub enum SymbolKind {
    File, Module, Namespace, Package, Class, Method,
    Property, Field, Constructor, Enum, Interface,
    Function, Variable, Constant, String, Number,
    Boolean, Array, Object, Key, Null, EnumMember,
    Struct, Event, Operator, TypeParameter,
}
```

## Features

- `linking`: Enable sysml-core integration for semantic symbol linking

## Dependencies

- `sysml-span`: For Span and Diagnostic types
- `sysml-id`: For ElementId
- `sysml-core` (optional, with `linking` feature): For element kind mapping

## Example

```rust
use sysml_span::{Span, Diagnostic, Severity};
use sysml_lsp::{LspDiagnostic, Range};

let source = "package Test { error here }";
let span = Span::with_location("test.sysml", 15, 20, 1, 16);

let diag = Diagnostic::error("unexpected token")
    .with_span(span)
    .with_code("E0001");

let lsp_diag = LspDiagnostic::from_sysml(&diag, source);
println!("Line {}: {}", lsp_diag.range.start.line, lsp_diag.message);
```
