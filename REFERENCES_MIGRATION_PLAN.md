# References Migration Plan (sysmlv2-references → in-repo + LFS)

This plan captures **where external references are used today** and a concrete path to
move them **inside the repo** using Git LFS. Goal: make the repo self‑contained for
contributors and CI while preserving provenance.

## 1) Inventory: What depends on sysmlv2-references today

### Hard requirements (build/runtime)
- **Xtext grammars** (build-time codegen + validation)
  - `SysML-v2-Pilot-Implementation/org.omg.sysml.xtext/.../SysML.xtext`
  - `SysML-v2-Pilot-Implementation/org.omg.kerml.xtext/.../KerML.xtext`
  - `SysML-v2-Pilot-Implementation/org.omg.kerml.expressions.xtext/.../KerMLExpressions.xtext`
  - Used by:
    - `sysml-text-pest/build.rs`
    - `sysml-core/build.rs`
    - `sysml-ts` generators (`generate_ts_tokens`, `validate_ts_coverage`)
    - `sysml-spec-tests` (operator coverage)

- **TTL vocab + shapes**
  - `Kerml-Vocab.ttl`, `SysML-vocab.ttl`
  - `KerML-shapes.ttl`, `SysML-shapes.ttl`
  - Used by:
    - `sysml-core/build.rs`
    - `codegen` tests
    - `sysml-spec-tests`
  - NOTE: copies already exist in `spec/` but code still points at sysmlv2-references.

### Test corpus + library usage
- **Corpus tests + examples**
  - `SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/**`
  - `SysML-v2-Models/models/**`
  - Used by `sysml-spec-tests`, `sysml-text-pest` tests, and examples.

- **Standard library**
  - `SysML-v2-Pilot-Implementation/sysml.library/**`
  - Portions already vendored under `libraries/standard/**`.

### Docs/scripts
Numerous README/ROADMAP references to `../sysmlv2-references` (paths and examples).

## 2) Target Layout (in-repo)

Proposed top-level folder:
```
references/
  sysmlv2/
    README.md               # provenance + upstream commit/tag
    SysML-vocab.ttl
    Kerml-Vocab.ttl
    SysML-shapes.ttl
    KerML-shapes.ttl
    SysML-v2-Pilot-Implementation/
      org.omg.sysml.xtext/...
      org.omg.kerml.xtext/...
      org.omg.kerml.expressions.xtext/...
      org.omg.sysml.xpect.tests/...
      sysml.library/...
    SysML-v2-Models/
      models/...
```

## 3) Git LFS strategy

Use LFS for the heavy corpora + libraries. Proposed `.gitattributes`:
```
references/sysmlv2/** filter=lfs diff=lfs merge=lfs -text
```

If you prefer to keep small spec files in normal git, scope it tighter:
```
references/sysmlv2/SysML-v2-Pilot-Implementation/** filter=lfs diff=lfs merge=lfs -text
references/sysmlv2/SysML-v2-Models/** filter=lfs diff=lfs merge=lfs -text
references/sysmlv2/**/sysml.library/** filter=lfs diff=lfs merge=lfs -text
```

## 4) Code Changes (path resolution)

Add a single resolver function (shared helper) that:
1) Checks `SYSMLV2_REFS_DIR` (backward compat)
2) Checks `SYSML_REFS_DIR` (new name)
3) Checks `references/sysmlv2` (repo local)
4) Falls back to `../sysmlv2-references` (legacy)

Apply to:
- `sysml-text-pest/build.rs`
- `sysml-core/build.rs`
- `sysml-spec-tests/src/lib.rs`
- `sysml-ts` token/coverage tools
- Bench/scripts (`scripts/bench.sh`)
- Examples that hardcode absolute paths

## 5) Documentation Updates

- Update `README.md`, `ROADMAP.md`, and spec/test docs to:
  - mention `references/sysmlv2`
  - keep `SYSMLV2_REFS_DIR` fallback for compatibility
- Add `references/sysmlv2/README.md` with:
  - upstream source URL(s)
  - commit/tag used
  - license/attribution notes

## 6) Migration Steps (Implementation Order)

1) Add `references/` folder + `.gitattributes` (LFS rules).
2) Copy data into `references/sysmlv2` (keep provenance).
3) Update path resolution helpers + tests to use new path.
4) Update docs and scripts.
5) CI: add a check that the directory exists (or document `SYSMLV2_REFS_DIR`).

## 7) Acceptance Criteria

- Clean checkout + `git lfs pull` gives a working build and tests.
- No hardcoded absolute paths remain.
- All parser/codegen/coverage tests run without external folders.
- Docs show the new location and provenance.
