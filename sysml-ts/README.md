# sysml-ts

Tree-sitter based CST (Concrete Syntax Tree) parsing for SysML v2.

## Purpose

This crate provides fast, incremental parsing suitable for IDE use cases. It produces a CST rather than a semantic model, making it useful for:

- Syntax highlighting
- Bracket matching
- Code folding
- Basic outline view
- Error recovery during editing

**Important**: This crate intentionally does NOT depend on sysml-core. It deals only with syntax, not semantics.

## Public API

### SyntaxNode

```rust
let node = SyntaxNode::new("identifier", span);
let error = SyntaxNode::error(span);

// Navigation
node.child_by_kind("identifier");  // first child of kind
node.find_by_kind("identifier");   // all descendants of kind
node.text(&source);                // get source text

// Error checking
node.has_errors();
node.errors();
```

### FastParser Trait

```rust
pub trait FastParser {
    fn parse_cst(&self, file: &SysmlFile) -> SyntaxNode;
    fn supports_incremental(&self) -> bool;
}
```

### StubTreeSitterParser

```rust
let parser = StubTreeSitterParser::new();
let file = SysmlFile::new("model.sysml", "package Test {}");
let cst = parser.parse_cst(&file);
```

### Outline Extraction

```rust
let outline = extract_outline(&cst, &source);
for item in outline {
    println!("{}: {} at {:?}", item.kind, item.name, item.span);
}
```

## Dependencies

- `sysml-span`: For Span type only

## Example

```rust
use sysml_ts::{SysmlFile, StubTreeSitterParser, FastParser, extract_outline};

let parser = StubTreeSitterParser::new();
let source = "package Vehicle { part engine; }";
let file = SysmlFile::new("model.sysml", source);

let cst = parser.parse_cst(&file);

// Check for errors
if cst.has_errors() {
    for error in cst.errors() {
        eprintln!("Syntax error at {:?}", error.span);
    }
}

// Extract outline for IDE
let outline = extract_outline(&cst, source);
for item in outline {
    println!("{}: {}", item.kind, item.name);
}
```

## Future Work

When tree-sitter grammar for SysML v2 is available, this crate will integrate with it for full incremental parsing support.

## Generated token tables

`sysml-ts` includes generated keyword/operator/enum tables derived from the Xtext specs to
keep Tree-sitter grammar data in sync with the parser pipeline.

To regenerate:

```bash
cargo run -p sysml-ts --bin generate_ts_tokens --features codegen
```

This writes `sysml-ts/generated/*.js` using `sysmlv2-references` as the source of truth.

## Tree-sitter stub + coverage check

The current Tree-sitter stub grammar lives at `sysml-ts/tree-sitter/grammar.js` and
consumes the generated keyword/operator/enum tables.

To run a basic coverage report against Xtext rules + ElementKinds:

```bash
cargo run -p sysml-ts --bin validate_ts_coverage --features codegen
```

Set `SYSML_TS_SHOW_MISSING=1` to list missing rules.
