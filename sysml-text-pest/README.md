# sysml-text-pest

Native Rust parser for SysML v2 textual notation using pest.

## Purpose

- Implements the `sysml-text` Parser trait in pure Rust
- Grammar is generated from the SysML/KerML Xtext specs at build time
- Produces a ModelGraph with elements and basic properties

## Build Requirements

This crate depends on the official spec Xtext files and will fail to build if they are missing.
Provide the references directory in one of these ways:

- Set `SYSMLV2_REFS_DIR=/path/to/sysmlv2-references`
- Place `sysmlv2-references/` as a sibling of the workspace root

## Usage

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

## Coverage Support

Enable the `coverage` feature to collect grammar rule hits for coverage tests:

```bash
cargo test -p sysml-spec-tests
```

## Status

The grammar is broad, but AST conversion is still expanding. Relationship emission,
ownership membership wiring, and full property mapping are in progress.
