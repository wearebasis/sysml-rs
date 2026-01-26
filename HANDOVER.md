# Handover Summary (Jan 26, 2026)

This handover focuses on **architecture + crate status** across the sysml-rs workspace.

## Architecture Overview (End-to-End)

**Core pipeline (ready for real-world use/testing):**
```
.sysml/.kerml → sysml-text-pest → sysml-core::ModelGraph → (resolve/query/serialize/visualize/run)
```

**Key layers**
- **Parsing**: `sysml-text-pest` builds a parse tree and constructs `ModelGraph`.
- **Semantic model**: `sysml-core` provides element types, relationships, and the graph model.
- **Resolution**: name resolution hooks live in `sysml-core` and are exposed via parse results.
- **Diagnostics**: `sysml-span` provides error spans + pretty rendering.
- **Output / Use**: query (`sysml-query`), visualization (`sysml-vis`, `sysml-web-vis`), storage (`sysml-store`), runners (`sysml-run-*`).
- **IDE layer**: `sysml-lsp-server` (LSP) + `sysml-ts` (Tree-sitter CST/queries) + editor extension(s).

## Crate Status Summary

**Core + Parsing**
- `sysml-core`: ✅ **Stable core**. Generated ElementKind + relationships; ModelGraph is the canonical semantic model.
- `sysml-text`: ✅ **Stable API layer** for text parsing; wraps parser variants.
- `sysml-text-pest`: ✅ **MVP complete**. Parses SysML v2 corpus and builds ModelGraph.
- `sysml-text-*sidecar`: 🟡 Early. Alternate parsers (pilot/monticore/syside) present but not primary.

**Codegen + Spec Validation**
- `sysml-codegen`: ✅ Stable. Builds element kinds, enums, properties, relationships; spec coverage validated at build time.
- `sysml-spec-tests`: 🟡 Basic. Corpus coverage for parser; more coverage can be added.

**Diagnostics + IDs**
- `sysml-span`: ✅ Stable. Pretty/colored diagnostics with spans + related locations.
- `sysml-id`: ✅ Stable. IDs and qualified names.
- `sysml-meta`: ✅ Stable. Values and metadata helpers.

**Query, Storage, Execution**
- `sysml-query`: 🟡 Basic but useful.
- `sysml-store` / `sysml-store-postgres`: 🟡 Basic persistence + Postgres storage.
- `sysml-run`: 🟡 Basic runner framework.
- `sysml-run-statemachine`: 🟡 Basic implementation.
- `sysml-run-constraints`: 🔴 Stub.

**Visualization**
- `sysml-vis`: 🟡 Basic (DOT/PlantUML/Cytoscape outputs).
- `sysml-web-vis`: 🟡 Basic web viewer.

**IDE / Tooling**
- `sysml-lsp`: 🟡 Early (integration helpers).
- `sysml-lsp-server`: 🟡 Early (works in Zed; more features to add).
- `sysml-ts`: 🟡 Early (Tree-sitter grammar + queries; minimal CST coverage).
- `sysml-lsp-zed-extension`: 🟡 Early (dev extension wired to local grammar).

**Other**
- `sysml-api`: 🟡 Basic REST API.
- `sysml-canon`: 🟡 Early/experimental canonicalization tools.

## What’s Ready vs Early

**Ready for real-world use/testing**
- `sysml-text-pest` parser pipeline → `sysml-core::ModelGraph`
- `sysml-span` diagnostics
- build-time spec validation via `sysml-codegen`

**Early / MVP**
- LSP server + editor integration
- Tree-sitter grammar + queries
- Runners (state machine, constraints)
- Web visualization

## How To Validate / Run

Parser pipeline:
```
cargo test -p sysml-text-pest
```

Tree-sitter:
```
cd sysml-ts/tree-sitter
tree-sitter generate --abi=14
tree-sitter test
```

LSP server:
```
cargo build -p sysml-lsp-server
```

## What Can Be Built Next

- **CLI**: parse/validate/visualize/query entrypoints.
- **Runners**: expand state machine execution, implement constraints.
- **IDE features**: document symbols, semantic tokens, quick fixes, go-to-def.
- **Visualization**: richer diagrams, diffing, interactive web UI.
- **Storage**: indexing, query APIs, cross-file resolution.

## Known Caveats

- Tree-sitter grammar coverage is still low (minimal CST).
- LSP lifecycle handling is still being hardened.
