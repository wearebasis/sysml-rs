# sysml-lsp-server

LSP server implementation for SysML v2.

## Purpose

This crate provides a Language Server Protocol server that supports IDE features for SysML v2:

- Document synchronization (open, change, close)
- Parse + resolution diagnostics
- Document symbols (outline view)

## Architecture

The server uses:
- `sysml-text-pest` for full parsing + resolution diagnostics
- `sysml-text` for parser trait + standard library loading
- `sysml-ts` for fast CST parsing (outline)
- `sysml-lsp` for LSP protocol type conversions

## Public API

### Creating a Service

```rust
use sysml_lsp_server::create_service;

let (service, socket) = create_service();
```

### Running on stdin/stdout

```rust
use sysml_lsp_server::run_stdio;

#[tokio::main]
async fn main() {
    run_stdio().await;
}
```

## Supported Capabilities

| Capability | Status |
|------------|--------|
| Text document sync | ✅ Full sync |
| Document symbols | ✅ Basic outline |
| Diagnostics | ✅ Parse + resolution diagnostics |
| Completion | 🚧 Planned |
| Go to definition | 🚧 Planned |
| Hover | 🚧 Planned |
| References | 🚧 Planned |

## Dependencies

- `sysml-lsp`: Protocol types
- `sysml-text`: Parser trait + library loader
- `sysml-text-pest`: Semantic parser
- `sysml-ts`: CST parsing (outline)
- `sysml-span`: Diagnostic types
- `tower-lsp`: LSP framework
- `tokio`: Async runtime

## Example: Running as Binary

```rust
// main.rs
use sysml_lsp_server::run_stdio;

#[tokio::main]
async fn main() {
    run_stdio().await;
}
```

## IDE Integration

### VS Code

Configure your VS Code extension to use this server:

```json
{
  "sysml.serverPath": "/path/to/sysml-lsp-server"
}
```

## Standard Library

The server loads the standard library for resolution using `SYSML_LIBRARY_PATH`,
falling back to `./libraries/standard` when present.

### Neovim (nvim-lspconfig)

```lua
require('lspconfig').sysml.setup{
  cmd = { '/path/to/sysml-lsp-server' }
}
```
