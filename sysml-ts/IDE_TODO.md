# sysml-ts IDE TODO (Tree-sitter path)

This list captures the remaining work needed for **full IDE support** based on Tree-sitter.
`sysml-ts` should own the Tree-sitter grammar + CST pipeline used by editors (Zed, VS Code,
and any LSP semantic token integration).

## Phase 0 (done)
- [x] Generate keyword/operator/enum tables from Xtext (`generated/*.js`)
- [x] Add tree-sitter stub grammar that consumes generated tables
- [x] Add coverage validator (`validate_ts_coverage`)

## Phase 1 (in progress)
- [x] Add stable nodes for package/part/attribute/typing
- [x] Add part/attribute definitions + usages with `def` disambiguation
- [x] Add literals (string/number/boolean/null) and qualified names
- [x] Add minimal corpus tests (2–3 files)
- [x] Ensure `tree-sitter generate` succeeds from `sysml-ts/tree-sitter`

## Grammar
- [ ] Implement a real Tree-sitter grammar for SysML v2 (replace stub)
- [ ] Define stable node names for key constructs (package/part/attribute/typing/etc.)
- [ ] Add error recovery rules to keep CST usable during edits
- [ ] Validate grammar against existing parser test corpus

## Queries (editor features)
- [x] `highlights.scm` for keywords, types, identifiers, literals
- [x] `folds.scm` for blocks/packages
- [x] `brackets.scm` for bracket matching
- [x] `indents.scm` for auto-indentation
- [x] `locals.scm` for local scope highlighting (if needed)

## CST / API
- [ ] Map Tree-sitter nodes into `SyntaxNode` with spans
- [ ] Preserve exact byte offsets for LSP range conversion
- [ ] Provide incremental parse support via Tree-sitter edits
- [ ] Expose lightweight outline extraction from CST

## LSP / IDE integration
- [ ] Add semantic token mapping from CST (optional)
- [ ] Provide fast document symbol extraction
- [ ] Provide quick syntax error diagnostics from CST (before full parse)

## Testing
- [ ] Corpus tests for grammar coverage
- [ ] Keep coverage validator output in CI (informational)
- [ ] Snapshot tests for highlights/outline output
- [ ] Incremental parse tests (edits + reparsing)

## Packaging
- [ ] Publish Tree-sitter grammar for editors
- [ ] Update Zed extension to point to sysml-ts grammar
