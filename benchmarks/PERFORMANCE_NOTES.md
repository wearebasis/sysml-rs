# Performance Analysis Notes

This document tracks performance investigations and optimization opportunities for sysml-rs.

## 2025-01-25: LineIndex Fix (Phase 2f)

### Problem: O(n²) Line/Column Calculation

**Symptom**: Corpus resolution tests hanging, taking excessive time.

**Flamegraph Analysis** (`/tmp/corpus_resolution_big.svg`):

| Function | % Time | Issue |
|----------|--------|-------|
| `pest::position::Position::line_col` | **79.49%** | THE BOTTLENECK |
| `pair_to_span` | 79.55% | Calls line_col for every AST node |
| `process_pair` | 79.57% | AST conversion |
| Resolution functions | <1% | Previous caching fixes worked |

**Root Cause**: Pest's `Position::line_col()` is O(n) - it scans from byte 0 to the current position, counting newlines. For a file with N tokens:
- Token 1: scan 0→1
- Token 2: scan 0→2
- Token N: scan 0→N
- **Total: O(N²) operations**

For the standard library (~36k elements), this means billions of character scans.

### Solution: Pre-computed LineIndex

Added `LineIndex` struct to `sysml-span`:
- **Build**: O(n) one-time scan to record byte offset of each line start
- **Lookup**: O(log n) binary search to convert byte offset to (line, col)

```rust
// sysml-span/src/lib.rs
pub struct LineIndex {
    line_offsets: Vec<usize>,  // line_offsets[0] = 0, etc.
}

impl LineIndex {
    pub fn new(source: &str) -> Self { ... }  // O(n)
    pub fn line_col(&self, offset: usize) -> (u32, u32) { ... }  // O(log n)
}
```

### Results After Fix

**Flamegraph** (`/tmp/corpus_resolution_big_v2.svg`):

| Function | % Time | Notes |
|----------|--------|-------|
| `[unknown]` (kernel) | 44.8% | Background/kernel overhead |
| `pest::ParserState::rule` | 18.6% | **Now top** - actual parsing |
| `pest::ParserState::match_string` | 14.3% | String matching in parser |
| `Position::match_string` | 8.6% | Low-level string compare |
| `SlicePartialEq::equal` | 7.2% | Slice comparisons |
| `BTreeMap::get/insert` | ~3% | Graph lookups |
| `uuid::getrandom` | 0.82% | UUID generation |
| `has_unresolved_refs` | 0.81% | Resolution checking |
| `create_owning_membership` | 0.66% | Graph building |

**`line_col` is no longer visible** - the fix eliminated the 79.5% bottleneck.

---

## Remaining Optimization Opportunities

These are lower priority since the major bottleneck is fixed, but documented for future reference.

### 1. UUID Generation (0.82%)

**Current**: Using `uuid::Uuid::new_v4()` which calls `getrandom` syscall.

**Options**:
- Use thread-local `SmallRng` seeded once
- Use sequential IDs (fastest, but not globally unique)
- Use `uuid::Uuid::new_v7()` which is partially time-based

**Impact**: Minor, only ~0.8% of time.

### 2. BTreeMap Operations (~3%)

**Current**: `ModelGraph` uses `BTreeMap<ElementId, Element>` for ordered iteration.

**Options**:
- Switch to `HashMap` for O(1) lookups (loses ordering)
- Switch to `IndexMap` for O(1) lookups with insertion order
- Use `slotmap` for dense storage with stable keys

**Impact**: Moderate, ~3% of time.

### 3. `has_unresolved_refs` (0.81%)

**Current**: Iterates through properties checking for `unresolved_*` prefixes.

**Options**:
- Add a boolean flag on Element tracking if any unresolved refs exist
- Use a bitset for property categories

**Impact**: Minor, only ~0.8% of time.

### 4. Pest Parsing Overhead (18.6%)

This is now the dominant cost after the LineIndex fix, which is expected for a parser.

**Options for further improvement**:
- Grammar optimization (reduce backtracking)
- Consider tree-sitter for incremental parsing (already have sysml-ts)
- Parallel file parsing (already implemented via rayon)

**Impact**: Would require significant grammar work for diminishing returns.

---

## Benchmark Commands

```bash
# Generate flamegraph for multi-file corpus resolution
sudo sysctl kernel.perf_event_paranoid=-1
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
CARGO_PROFILE_RELEASE_DEBUG=true \
cargo flamegraph -p sysml-spec-tests --test corpus_tests \
  -o /tmp/corpus_resolution_multi_file.svg -- corpus_resolution_multi_file --ignored --nocapture

# Quick corpus coverage test
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
cargo test -p sysml-spec-tests corpus_coverage -- --ignored --nocapture

# Full corpus resolution test (multi-file + library)
SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
cargo test -p sysml-spec-tests corpus_resolution_multi_file -- --ignored --nocapture
```

---

## Flamegraph Archive

| Date | File | Description |
|------|------|-------------|
| 2025-01-25 | `/tmp/corpus_resolution_big.svg` | Before LineIndex fix (79.5% in line_col) |
| 2025-01-25 | `/tmp/corpus_resolution_big_v2.svg` | After LineIndex fix (line_col eliminated) |
