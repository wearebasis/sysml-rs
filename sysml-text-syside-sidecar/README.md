# sysml-text-syside-sidecar

SySide parser adapter for SysML v2 (Node.js sidecar).

## Purpose

This crate provides an adapter that integrates with the SySide SysML parser via Node.js HTTP service.

## Integration Points

### HTTP Mode

Calls a running SySide Node.js HTTP service:

```bash
export SYSIDE_ENABLED=1
export SYSIDE_URL=http://localhost:8082
```

## Public API

### Transport Trait

```rust
pub trait Transport {
    fn send(&self, files: &[SysmlFile]) -> Result<TransportResult, TransportError>;
}
```

### Built-in Transports

```rust
// HTTP transport
let transport = HttpTransport::new("http://localhost:8082");
```

### Parser

```rust
use sysml_text_syside_sidecar::{SySideParser, HttpTransport, create_from_env};

// With explicit transport
let parser = SySideParser::new(HttpTransport::new("http://localhost:8082"));

// From environment variables
let parser = create_from_env();

let result = parser.parse(&files);
```

## Current Status

This crate is a stub implementation. The transports are not yet functional. To enable the parser for testing, set `SYSIDE_ENABLED=1`.

## Dependencies

- `sysml-text`: Parser trait
- `sysml-core`: ModelGraph
- `sysml-span`: Diagnostic

## Example

```rust
use sysml_text::{Parser, SysmlFile};
use sysml_text_syside_sidecar::create_from_env;

std::env::set_var("SYSIDE_ENABLED", "1");
std::env::set_var("SYSIDE_URL", "http://localhost:8082");

let parser = create_from_env();
let files = vec![SysmlFile::new("model.sysml", "package Model {}")];

let result = parser.parse(&files);
```
