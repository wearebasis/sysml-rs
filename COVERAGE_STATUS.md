# Coverage Status

This file tracks parsing and resolution coverage metrics over time. Update after significant changes.

---

## Latest Status (2026-01-25)

### Parsing Coverage

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Corpus files parsed | 55/57 | 57/57 | 96.5% |
| Grammar-Element linkage | 122/182 | 164/182 | 67.0% |
| Property validation | 5/5 | 5/5 | 100% |

**Parsing Failures (2 files):**
- `VehicleModel.sysml` - Expression operator parsing gap
- `EIT_System_Use_Cases.sysml` - Feature specialization parsing gap

### Resolution Coverage

| Test | Resolved | Unresolved | Rate | Notes |
|------|----------|------------|------|-------|
| Single-file (no library) | 919 | 1,227 | 42.8% | Baseline, no cross-file refs |
| Multi-file (with library) | 893 | 56 | **94.1%** | Best result - all files in one graph |
| With library (per-file) | 595 | 348 | **63.1%** | Per-file mode (cross-file refs can't resolve) |
| Quick check | 40 | 7 | **85.1%** | Single file with library |

**Phase 2e.4 Literal Fix COMPLETE:** Numeric literals like `5.0` no longer create false unresolved references.

**Phase 2e.3 Iterative Resolution:** Fixed-point iteration for cross-file inheritance chains.

**eVehicleLibrary Stub Created:** Added stub library for missing HVAC model dependency.

### Unhandled AST Rules

As of latest run, these rules still trigger `[AST] Unhandled rule` warnings:
- `TypeResultMember`, `TypeReferenceFeature` (type expressions)
- `SubclassificationRelationship`, `SpecializesToken` (specialization)

---

## Known Issues

### Library Index Rebuild Bug - FIXED (2026-01-25)

**Symptom:** Resolution rate was dropping from 74.4% → 7.1% when using per-file library merge.

**Root Cause:** Two issues:
1. `merge()` was not copying library's pre-built indexes - now fixed with index merging
2. Library unresolved references were being counted N times (once per file) in per-file mode - now fixed by excluding library elements from resolution counting

**Fix Applied:**
- Added index merging to `ModelGraph::merge()` in `sysml-core/src/lib.rs`
- Added `resolve_references_excluding()` in `sysml-core/src/resolution/mod.rs`
- Modified `into_resolved_with_library()` and `resolve_with_library()` in `sysml-text/src/lib.rs` to track and exclude library elements

### Most Common Unresolved References (80 total from multi-file test)

**Category 1: Cross-File Redefinitions (58 refs)**

These are features being redefined from other model files:
| Name | Count | Source File |
|------|-------|-------------|
| pilotPod | 12 | UseCases*.sysml |
| powerUsage | 5 | COTS.sysml |
| hostileShip | 3 | DomainModel |
| station | 3 | DomainModel |
| fleetSize, age, securityEscortCoverage, miningRateLS, etc. | 35 | Various |

**Root Cause:** Redefinition resolution requires finding the original feature in the supertype's namespace. When the supertype is defined in another file, the full inheritance chain may not be available.

**Category 2: Missing Import Targets (22 refs)**

These reference types from packages that don't exist in the corpus:
| Name | Count | Missing Package |
|------|-------|-----------------|
| Percentage | 9 | eVehicleLibrary |
| Temperature | 6 | eVehicleLibrary |
| Distance | 2 | eVehicleLibrary |
| Power | 2 | eVehicleLibrary |
| Decibel | 2 | eVehicleLibrary |
| ShapeItems::Cylinder, Box | 1 | ShapeItems |

**Root Cause:** Model files import from `eVehicleLibrary::*` which doesn't exist in the corpus. These are placeholder imports - the models are intentionally incomplete.

---

## Known Limitations (Phase 2e.2 Analysis)

The 91.8% multi-file resolution rate represents the **practical maximum for this corpus**. The remaining ~8% unresolved references are due to:

### 1. Incomplete Model Files

The corpus includes model files that import from packages not present in the corpus:
- `eVehicleLibrary` - Referenced by Drone model files but not provided
- `ShapeItems` - Referenced but not defined

These are placeholder imports in intentionally incomplete demonstration models.

### 2. Cross-File Redefinition Resolution

Feature redefinitions like `redefines pilotPod` require traversing the supertype's namespace to find the original feature. When that supertype is defined in a different file:
- The inheritance chain must be fully resolved first
- This is working correctly for most cases (91.8% rate)
- Remaining failures are edge cases with deep cross-file inheritance

### 3. What IS Working ✅

- KerML base types (Expression, Object, Interaction, etc.) - NOW RESOLVING
- Standard library types (ISQ units, SI units, base definitions)
- Cross-file type references (imports, qualified names)
- Intra-file redefinitions and specializations

### Potential Future Improvements

| Improvement | Impact | Effort |
|-------------|--------|--------|
| Cross-file redefinition traversal | +3-5% | High |
| Missing import diagnostics | 0% (info only) | Low |
| Add missing eVehicleLibrary stub | +2% | Low |

**Recommendation:** Accept current rates and proceed to Phase 3 (LSP/IDE integration).

---

## History

### 2026-01-25 - Phase 2e.4 Complete (Literal Detection Fix)

**Changes:**
- Added `is_literal_expression()` detection in `extraction.rs` to identify simple literal values
- Modified `process_usage()` to skip setting `unresolved_value` for literal values
- Added literal handlers in `ast/mod.rs` for future literal element creation
- Created `eVehicleLibrary.sysml` stub for missing HVAC model dependency

**Problem Solved:**
Numeric literals like `5.0`, `100`, `true` were being stored in `unresolved_value` property, causing false "unresolved reference '5.0'" errors during resolution.

**Solution:**
Detect when a value expression is a simple literal (not a complex expression like `2 + 3`) and skip storing it as an unresolved reference.

**Metrics:**
- Multi-file resolution: 92.0% → **94.1%** (+2.1%)
- Total unresolved references: 75 → 56 (-19)

### 2026-01-25 - Phase 2e.3 Complete (Iterative Resolution)

**Changes:**
- Implemented fixed-point iterative resolution in `resolve_references_excluding()`
- Extracted single-pass logic to `resolve_references_single_pass()` helper
- Simplified `resolve_references()` to delegate to `resolve_references_excluding()`
- Resolution now runs multiple passes until no more references resolve

**How It Works:**
- Each iteration rebuilds scope tables with newly resolved Specializations
- This makes inherited members visible for subsequent redefinition resolution
- Typical completion: 2 iterations (iteration 2 resolves 0 → fixed-point reached)
- Safety limit: 10 iterations maximum

**Metrics:**
- Multi-file resolution: 91.8% → **92.0%** (+0.2%)
- Iteration count: 2 passes
- All existing unit tests pass

**Analysis:**
The remaining 75 unresolved references are NOT cross-file redefinition issues:
- Feature chain expressions (e.g., `evaluateData.verdict`)
- References to custom types not in library (`Temperature`, `Percentage`)
- Numeric literals being parsed as references

These require different fixes (expression parsing, custom library support).

### 2026-01-25 - Phase 2e.2 Complete (Analysis of Remaining Unresolved References)

**Findings:**
- KerML base types (Expression, Object, etc.) ARE resolving correctly after 2e.1 fix
- Remaining 80 unresolved references are NOT KerML types
- Two categories: cross-file redefinitions (58) and missing import targets (22)
- 91.8% resolution is the practical maximum for this corpus

**Changes:**
- Updated ROADMAP.md Phase 2e.2 section with analysis
- Added "Known Limitations" section to COVERAGE_STATUS.md
- No code changes required

**Conclusion:** Phase 2e complete. Ready to proceed to Phase 3.

### 2026-01-25 - Phase 2e.1 Complete (Library Index Bug Fix)

**Changes:**
- Fixed index merging in `ModelGraph::merge()` - library indexes now preserved
- Added `resolve_references_excluding()` to skip library elements during resolution counting
- Updated `into_resolved_with_library()` and `resolve_with_library()` functions

**Metrics Before:**
- Multi-file: 74.4%
- Per-file with library: 7.1%
- Quick check: 15.0%

**Metrics After:**
- Multi-file: **91.8%** (+17.4%)
- Per-file with library: **63.1%** (+56%)
- Quick check: **85.1%** (+70.1%)

**Remaining Work:** Cross-file references (pilotPod, station, etc.) can't resolve in per-file mode - this is expected behavior.

### 2026-01-25 - Phase 6 & 7 Complete

**Changes:**
- Added 16+ missing AST handlers for pest rules
- Implemented MaxCardinality validation
- Marked ReadOnly as covered (documentation only)
- Added handlers for intermediate rules (OccurrenceUsageMemberWithSuccession, etc.)

**Metrics Before:**
- Property validation: 3/5 (60%)
- Many `[AST] Unhandled rule` warnings

**Metrics After:**
- Property validation: 5/5 (100%)
- Most unhandled rule warnings resolved
- Parsing coverage unchanged at 55/57

---

## Running Coverage Tests

```bash
# Parsing coverage only
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
cargo test -p sysml-spec-tests corpus_coverage -- --ignored --nocapture

# Full resolution tests (slow, ~3 minutes)
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
cargo test -p sysml-spec-tests corpus_resolution -- --ignored --nocapture

# Quick check (faster)
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
cargo test -p sysml-spec-tests corpus_resolution_quick -- --ignored --nocapture
```

---

## Target Metrics

| Metric | Current | Target | Priority | Status |
|--------|---------|--------|----------|--------|
| Parsing | 96.5% | 100% | Medium | 2 files remaining |
| Resolution (multi-file) | **94.1%** | 90%+ | HIGH | ✅ ACHIEVED |
| Resolution (with library) | **63.1%** | 70%+ | Medium | Near target |
| Resolution (quick) | **85.1%** | 80%+ | - | ✅ ACHIEVED |
| Grammar-Element linkage | 67% | 90% | Low | Future work |

**Note:** Per-file resolution (63.1%) is inherently lower than multi-file (94.1%) because cross-file references cannot resolve when files are parsed independently.

**Note:** The 94.1% rate represents near-maximum coverage for this corpus. Remaining 56 unresolved references are:
- Feature chain expressions (e.g., `evaluateData.verdict`)
- Cross-file redefinitions (e.g., `pilotPod`)
- These require deeper expression parsing or cross-file traversal.
