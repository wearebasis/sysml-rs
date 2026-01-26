# sysml-lsp integration map

This crate is the **thin LSP shim** between the SysML parser/diagnostic pipeline and editor
clients. It intentionally avoids pulling in `lsp-types` and instead provides small, stable
structures used by the server.

## What this crate does

- Converts `sysml-span` diagnostics into LSP-friendly diagnostics
- Converts `sysml-span::Span` byte offsets into LSP ranges
- Defines a minimal subset of LSP structures used by `sysml-lsp-server`
- Optionally maps `sysml-core::ElementKind` to LSP symbol kinds (feature: `linking`)

## What this crate does NOT do

- It does **not** parse SysML
- It does **not** perform resolution or validation
- It does **not** depend on `tower-lsp` or `lsp-types`

Those responsibilities live in the server and parser crates.

## Dependencies and why they exist

- `sysml-span` (required)
  - Source locations (`Span`) and diagnostic structures (`Diagnostic`)
  - Used in `Range::from_span` and `LspDiagnostic::from_sysml`

- `sysml-id` (required)
  - `ElementId` used when the optional `linking` feature is enabled

- `sysml-core` (optional, feature: `linking`)
  - Provides `ElementKind` so LSP symbols can be mapped to semantic kinds

## How data flows (end to end)

1) **Parsing** happens in `sysml-text-pest`
   - Produces `sysml-span::Diagnostic` with spans
2) **Resolution / validation** happens in `sysml-core`
   - Produces additional `sysml-span::Diagnostic` entries
3) **LSP conversion** happens here (`sysml-lsp`)
   - `LspDiagnostic::from_sysml` converts to LSP-friendly form
4) **Transport** happens in `sysml-lsp-server`
   - Uses `tower-lsp` to send diagnostics to editors

## Important functions and their purpose

- `Range::from_span(span, source)`
  - Converts byte offsets to LSP ranges using the given source text

- `LspDiagnostic::from_sysml(diag, source)`
  - Converts a `sysml-span::Diagnostic` to an LSP diagnostic
  - Carries through severity, code, and message
  - Attaches related information from `diag.notes`

- `element_kind_to_symbol_kind(kind)` (feature: `linking`)
  - Maps `sysml-core` element kinds to LSP symbol kinds
  - Used for outline/document symbols in the server

## Why the server depends on this crate

`sysml-lsp-server` depends on this crate for **stable conversion logic** so that:

- diagnostic formatting stays consistent even if the server changes
- LSP types can be swapped or minimized without reworking server logic

## Extension points

If you want richer IDE behavior, extend in these places:

- Add related spans or notes in `sysml-core` diagnostics
- Add new LSP conversions here (e.g., code actions, semantic tokens)
- Add LSP handlers in `sysml-lsp-server` to publish new data
