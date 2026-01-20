# sysml-text-monticore-sidecar

MontiCore parser adapter for SysML v2 (JVM/HTTP sidecar).

## Purpose

This crate provides an adapter that integrates with the MontiCore SysML parser via either JVM execution or HTTP service.

## Integration Points

### HTTP Mode

Calls a running MontiCore HTTP service:

```bash
export MONTICORE_ENABLED=1
export MONTICORE_MODE=http
export MONTICORE_URL=http://localhost:8081
```

### JVM Mode

Calls a MontiCore JAR file via subprocess:

```bash
export MONTICORE_ENABLED=1
export MONTICORE_MODE=jvm
export MONTICORE_JAR=/path/to/monticore.jar
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
let transport = HttpTransport::new("http://localhost:8081");

// Command/JVM transport
let transport = CommandTransport::new("/path/to/monticore.jar");
```

### Parser

```rust
use sysml_text_monticore_sidecar::{MontiCoreParser, HttpTransport, create_from_env};

// With explicit transport
let parser = MontiCoreParser::new(HttpTransport::new("http://localhost:8081"));

// From environment variables
let parser = create_from_env();

let result = parser.parse(&files);
```

## Current Status

This crate is a stub implementation. The transports are not yet functional. To enable the parser for testing, set `MONTICORE_ENABLED=1`.

## Dependencies

- `sysml-text`: Parser trait
- `sysml-core`: ModelGraph
- `sysml-span`: Diagnostic

## Example

```rust
use sysml_text::{Parser, SysmlFile};
use sysml_text_monticore_sidecar::create_from_env;

std::env::set_var("MONTICORE_ENABLED", "1");
std::env::set_var("MONTICORE_URL", "http://localhost:8081");

let parser = create_from_env();
let files = vec![SysmlFile::new("model.sysml", "package Model {}")];

let result = parser.parse(&files);
```
