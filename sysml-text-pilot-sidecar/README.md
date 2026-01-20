# sysml-text-pilot-sidecar

Pilot parser adapter for SysML v2 (JVM/HTTP sidecar).

## Purpose

This crate provides an adapter that integrates with the Pilot SysML parser via either JVM execution or HTTP service.

## Integration Points

### HTTP Mode

Calls a running Pilot HTTP service:

```bash
export PILOT_ENABLED=1
export PILOT_MODE=http
export PILOT_URL=http://localhost:8080
```

### JVM Mode

Calls a Pilot JAR file via subprocess:

```bash
export PILOT_ENABLED=1
export PILOT_MODE=jvm
export PILOT_JAR=/path/to/pilot.jar
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
let transport = HttpTransport::new("http://localhost:8080");

// Command/JVM transport
let transport = CommandTransport::new("/path/to/pilot.jar");
```

### Parser

```rust
use sysml_text_pilot_sidecar::{PilotParser, HttpTransport, create_from_env};

// With explicit transport
let parser = PilotParser::new(HttpTransport::new("http://localhost:8080"));

// From environment variables
let parser = create_from_env();

let result = parser.parse(&files);
```

## Current Status

This crate is a stub implementation. The transports are not yet functional. To enable the parser for testing, set `PILOT_ENABLED=1`.

## Dependencies

- `sysml-text`: Parser trait
- `sysml-core`: ModelGraph
- `sysml-span`: Diagnostic

## Example

```rust
use sysml_text::{Parser, SysmlFile};
use sysml_text_pilot_sidecar::create_from_env;

std::env::set_var("PILOT_ENABLED", "1");
std::env::set_var("PILOT_URL", "http://localhost:8080");

let parser = create_from_env();
let files = vec![SysmlFile::new("model.sysml", "package Model {}")];

let result = parser.parse(&files);
```
