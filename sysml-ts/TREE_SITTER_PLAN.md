# Tree-sitter Grammar Plan (sysml-ts)

This plan documents how we can reuse existing **codegen + pest assets** to build a
Tree-sitter grammar for SysML v2. It does **not** attempt a direct conversion
from pest to Tree-sitter (PEG → GLR); instead it uses the current pipeline to
auto-generate token tables, operator precedence, and coverage checks, then
incrementally implements the CST grammar by hand.

## What We Can Reuse Today

1) **Xtext-derived tokens + operators + enums**
   - `codegen/src/xtext_parser.rs` already extracts:
     - keywords (all quoted strings)
     - operators (with precedence ordering)
     - enums
   - `codegen/src/pest_generator.rs` shows how those become grammar rules.
   - We can reuse the same extraction to generate Tree-sitter keyword/operator
     tables and enum token lists.

2) **Grammar ↔ ElementKind linkage**
   - `codegen/src/grammar_element_validator.rs` builds a mapping from Xtext rule
     names → element types (ElementKind).
   - This can become a coverage/validation tool for Tree-sitter, ensuring that
     the CST grammar exposes at least the element types we expect to be parseable.

3) **Pest grammar fragments as canonical structure reference**
   - `sysml-text-pest/src/grammar/fragments/*.pest` is the most complete current
     syntax implementation. We can use it as the “source of truth” for rule
     grouping and naming, but not for direct mechanical conversion.

## What We *Cannot* Directly Reuse

Tree-sitter is not PEG. Many pest constructs (ordering, implicit backtracking,
negative lookaheads, etc.) cannot be translated verbatim. The grammar must be
authored to handle conflicts, precedence, and incremental error recovery.

So: **use pest for coverage and mapping, not for direct generation.**

## Proposed Plan (Phased)

### Phase 0 — Generator Scaffolding (sysml-ts) ✅
Goal: make grammar *data-driven* (keywords/operators/enums) using existing codegen.

- [x] Add a generator under `sysml-ts/` that:
  - [x] Reads Xtext files via existing codegen functions.
  - [x] Produces `generated/keywords.js`, `generated/operators.js`,
    `generated/enums.js` for Tree-sitter grammar consumption.
- [x] Keep generated files committed for editors (no build-time dependency).
- [x] Add a lightweight coverage validator for tree-sitter rule names.

### Phase 1 — Minimal Tree-sitter Grammar (CST Core)
Goal: parse the “spine” of SysML files for IDE basics.

- Implement grammar for:
  - [x] `source_file`, `package_decl`, `import`, basic blocks
  - [x] `part/attribute` definitions/usages and simple typings
  - [x] identifiers, qualified names, literals, comments
  - [ ] minimal expression/literal usage sites (optional)
- Ensure stable node names (snake_case) and a clear CST layout.
  - [x] stable nodes for package/part/attribute/typing
- Add a tiny corpus in `sysml-ts` for parse sanity checks.
  - [x] minimal corpus (2–3 files)

### Phase 2 — Definition/Usage Expansion
Goal: cover the main SysML/KerML structural constructs.

- Implement:
  - definitions/usages in `definitions.pest` + `usages.pest`
  - features/relationships/annotations blocks
- Use grammar-element linkage tooling to track which element kinds have a CST rule.

### Phase 3 — Expression Grammar (Operators)
Goal: precise operator precedence using the Xtext-derived precedence map.

- Translate `KerMLExpressions.xtext` operator tiers into Tree-sitter
  precedence rules.
- Add error recovery for expression parsing (common edit-time failures).

### Phase 4 — Validation + Coverage
Goal: verify grammar coverage against real syntax expectations.

- Add a validation script that:
  - reads Tree-sitter rule names
  - compares to Xtext rule names + ElementKind mapping
  - reports missing CST rules (non-blocking at first)
- Create corpus tests mirroring `sysml-text-pest` examples.

### Phase 5 — Editor Queries
Goal: full IDE experience (highlighting, folding, etc.).

- [x] Generate baseline `highlights.scm` from keyword + enum lists.
- [x] Add `folds.scm`, `brackets.scm`, `indents.scm`, `locals.scm`.
- Keep these queries versioned in `sysml-ts` for reuse in Zed/VS Code.

## Where This Hooks In

- **Inputs**:
  - `references/sysmlv2/.../*.xtext` (Xtext sources)
  - `sysml-text-pest/src/grammar/fragments/*.pest` (structure reference)
  - `codegen` utilities (keywords/operators/enums/precedence)

- **Outputs**:
  - `sysml-ts` Tree-sitter grammar + generated token tables
  - CST mapping for `SyntaxNode` + outline extraction
  - Editor query files shared by Zed/VS Code extensions

## Open Questions

- Should Tree-sitter rule names mirror Xtext rule names or Rust model names?
- Do we want strict coverage gating (build fail) or only warnings?
- How much error recovery is needed for “live edit” scenarios?

## Validation Steps (for this phase)

1) Regenerate token tables (requires references/sysmlv2):
```
cargo run -p sysml-ts --bin generate_ts_tokens --features codegen
```

2) Run coverage report (shows how many Xtext/ElementKinds are mapped by the grammar):
```
cargo run -p sysml-ts --bin validate_ts_coverage --features codegen
```

Set `SYSML_TS_SHOW_MISSING=1` to list missing rules in detail.

## Validation Steps (Phase 1)

1) Regenerate token tables (if xtext sources changed):
```
cargo run -p sysml-ts --bin generate_ts_tokens --features codegen
```

2) Generate the Tree-sitter parser (requires Node + tree-sitter CLI):
```
cd sysml-ts/tree-sitter
tree-sitter generate --abi=14
```

3) Run coverage report to confirm rule growth:
```
cargo run -p sysml-ts --bin validate_ts_coverage --features codegen
```

4) Run tree-sitter corpus tests:
```
cd sysml-ts/tree-sitter
tree-sitter test
```
