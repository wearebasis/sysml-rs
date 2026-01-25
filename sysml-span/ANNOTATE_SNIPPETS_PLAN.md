# Annotate-Snippets Integration Plan (sysml-span)

## Goal
Provide rustc-style, ASCII diagnostic rendering for `sysml_span::Diagnostic` using the `annotate-snippets` crate, without changing parsing or resolution behavior. This is **presentation only**: it should format existing diagnostics (spans, notes, related locations) into readable snippets.

## Non-Goals
- No changes to parsing logic (pest, tree-sitter, or LSP).
- No changes to diagnostic generation semantics in `sysml-core` or `sysml-text`.
- No new CLI tooling in this phase (rendering is an API; CLI can adopt later).

## Why `annotate-snippets`
It produces output close to rustc diagnostics (header + source excerpt + caret underline + notes). It is a focused renderer, not a parser or diagnostic engine.

## Proposed Surface API (no implementation yet)

### Option A: Standalone renderer module (recommended)
```rust
use sysml_span::{Diagnostic, DiagnosticRenderer, SourceProvider};

let renderer = DiagnosticRenderer::new(); // defaults: color auto/off
let output = renderer.render(&diagnostic, &provider);
println!("{output}");
```

### Option B: Convenience method on `Diagnostic`
```rust
let output = diagnostic.render_snippet(&provider);
```

### SourceProvider
We need a way to fetch source text for each file referenced in `Span.file`:
```rust
trait SourceProvider {
    fn source(&self, file: &str) -> Option<&str>;
}
```
This avoids doing file I/O inside `sysml-span`. Callers can pass:
- an in-memory map (ideal for editors/LSP),
- or a simple file loader wrapper for CLI tools.

## Expected Output Examples

### 1) Unresolved Type (from resolution diagnostics)
```
error[E010]: unresolved reference: MissingPart
  --> demo.sysml:7:29
   |
7  |     part ghostAttachment : MissingPart;
   |                             ^^^^^^^^^ not found in scope
   |
   = note: declare MissingPart or import a package that defines it
```

### 2) Relationship Type Mismatch (structural validation)
```
error[E005]: Relationship type mismatch: FeatureTyping expects Type, got Package
  --> demo.sysml:12:12
   |
12 |     part myCar : Demo;
   |            ^^^ invalid type reference
```

### 3) Related Location (definition site)
```
error[E020]: name already defined
  --> model.sysml:4:5
   |
4  |     part def Engine;
   |     ^^^^^^^^^^^^^^^ redefinition
   |
note: first defined here
  --> model.sysml:2:5
   |
2  |     part def Engine;
   |     ^^^^^^^^^^^^^^^
```

## Codebase Touchpoints (for reference)

### sysml-span (target crate)
- `sysml-span/src/lib.rs`  
  - `Span` (source location)  
  - `Diagnostic` (message, code, span, notes, related)  
  - `Diagnostics` (collection)

### sysml-core
- `sysml-core/src/structural_validation.rs`  
  - `impl From<StructuralError> for sysml_span::Diagnostic`  
  - This determines `message` + `code` values that will be rendered.

### sysml-text
- `sysml-text/src/lib.rs`  
  - `ParseResult` collects diagnostics and already merges resolution/validation errors.  
  - Any rendering should work directly on `ParseResult.diagnostics`.

### sysml-lsp (+ server)
- `sysml-lsp/src/lib.rs`  
  - Converts `sysml_span::Diagnostic` to LSP diagnostics; stays untouched.
- `sysml-lsp-server` can optionally print pretty snippets on CLI later.

## Tasks & Tracking

### Phase 1 — API & Feature Flag
- [x] Add optional dependency on `annotate-snippets` in `sysml-span/Cargo.toml`.
- [x] Add `pretty` (or `annotate-snippets`) feature flag in `sysml-span`.
- [x] Decide on renderer API (module + trait + helpers).

**Phase 1 decisions**
- Feature name: `pretty` (gates annotate-snippets integration).
- API shape: `DiagnosticRenderer` + `SourceProvider` (module-level types, feature-gated), with `render()`/`render_all()` helpers.

### Phase 2 — Renderer Implementation (sysml-span)
- [x] Implement `SourceProvider` trait and a simple `HashMap` provider.
- [x] Implement `DiagnosticRenderer` that:
  - [x] Maps `Diagnostic.severity` to annotate-snippets level.
  - [x] Uses `Diagnostic.span` for main label.
  - [x] Renders `Diagnostic.notes` as footers.
  - [x] Renders `RelatedLocation` entries as secondary slices.
- [x] Implement `render_all(diagnostics)` helper to batch render.

### Phase 3 — Examples + Docs
- [x] Add a small example to `sysml-span/README.md`.
- [x] Add a new example under `sysml-span/examples/` that renders a few diagnostics.
- [x] Show how to use it from `sysml-text` ParseResult.

### Phase 4 — Integration Opt-In (Optional)
- [ ] In `sysml-text-pest` example(s), optionally render diagnostics via the new renderer.
- [ ] Provide CLI output toggle for pretty printing (future).

## Open Questions
- Do we want colors enabled by default, or controlled by caller?
- Should renderer handle missing source gracefully (fallback to `file:line:col: message`)?
- Should `sysml-span` expose a `SourceMap` type, or keep it trait-only?

## Completion Criteria
- A caller can pass `ParseResult.diagnostics` + in-memory sources and get rustc-style ASCII output.
- No change to diagnostics generation logic in other crates.
- No feature flag? no extra dependencies in default builds.
