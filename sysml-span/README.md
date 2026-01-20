# sysml-span

Source locations, diagnostics, and severity levels for SysML v2.

## Purpose

This crate provides types for tracking source code locations and reporting diagnostics:

- **Span**: A range in a source file (byte offsets, optional line/column)
- **Severity**: Error, Warning, or Info
- **Diagnostic**: A message with severity, optional code, span, and notes
- **Diagnostics**: A collection of diagnostics with helper methods

## Public API

### Span

```rust
// Create with byte offsets
let span = Span::new("file.sysml", 10, 20);

// Create with line/column info
let span = Span::with_location("file.sysml", 10, 20, 5, 3);

// Point span
let span = Span::point("file.sysml", 15);

// Synthetic (no real location)
let span = Span::synthetic();

// Operations
span.len();           // byte length
span.contains(15);    // check if offset in range
span.merge(&other);   // combine two spans
```

### Severity

```rust
pub enum Severity {
    Info,
    Warning,
    Error,
}

severity.is_error();
severity.is_warning_or_error();
```

### Diagnostic

```rust
// Create diagnostics
let diag = Diagnostic::error("parse error")
    .with_code("E001")
    .with_span(span)
    .with_note("hint: check syntax");

// Convenience constructors
Diagnostic::warning("deprecated feature");
Diagnostic::info("suggestion");
```

### Diagnostics Collection

```rust
let mut diags = Diagnostics::new();
diags.error("error 1");
diags.warning("warning 1");

diags.has_errors();   // true
diags.error_count();  // 1
diags.len();          // 2
```

## Features

- `serde`: Enable serde serialization support

## Dependencies

- `serde` (optional): Serialization support

## Example

```rust
use sysml_span::{Span, Diagnostic, Severity, Diagnostics};

let span = Span::with_location("model.sysml", 100, 110, 10, 5);

let diag = Diagnostic::error("undefined element 'foo'")
    .with_code("E0001")
    .with_span(span)
    .with_note("did you mean 'bar'?");

println!("{}", diag);
// error[E0001]: undefined element 'foo'
//   --> model.sysml:10:5
//   = note: did you mean 'bar'?
```
