# Plan: Achieve 100% Name Resolution

## Executive Summary

**Current State:** 42.9% resolution rate (1,181 resolved / 2,752 total)
**Target:** 100% resolution rate

**Root Causes (in priority order):**
1. **Standard library not loaded** (~40-50% of failures)
2. **Feature chaining incomplete** (~15-20% of failures)
3. **Cross-file references** (~10-15% of failures)
4. **Incomplete scoping strategies** (~5-10% of failures)

---

## Phase A: Standard Library Loading (Biggest Win)

**Goal:** Parse and pre-load all standard library files so types like `Anything`, `Real`, `Boolean` resolve.

**Estimated Impact:** 42.9% → ~70-80%

### A.1: Create Library Loader Module

**File:** `sysml-text/src/library.rs` (new)

```rust
//! Standard library loading for name resolution.

use std::path::Path;
use crate::{Parser, SysmlFile, ParseResult};
use sysml_core::ModelGraph;

/// Load and parse all standard library files from a directory.
pub fn load_standard_library(library_path: &Path) -> Result<ModelGraph, LibraryLoadError> {
    let mut combined_graph = ModelGraph::new();

    // Load KerML kernel libraries first (they're foundational)
    load_kerml_libraries(&mut combined_graph, library_path)?;

    // Then load SysML systems libraries
    load_sysml_libraries(&mut combined_graph, library_path)?;

    // Register all root packages as library packages
    register_library_packages(&mut combined_graph);

    Ok(combined_graph)
}

fn load_kerml_libraries(graph: &mut ModelGraph, base: &Path) -> Result<(), LibraryLoadError> {
    let kerml_dir = base.join("library.kernel");
    load_files_from_dir(graph, &kerml_dir, "kerml")
}

fn load_sysml_libraries(graph: &mut ModelGraph, base: &Path) -> Result<(), LibraryLoadError> {
    let sysml_dir = base.join("library.systems");
    load_files_from_dir(graph, &sysml_dir, "sysml")
}

fn register_library_packages(graph: &mut ModelGraph) {
    // Find all root packages and register them
    for element in graph.roots() {
        if element.kind == ElementKind::Package || element.kind == ElementKind::LibraryPackage {
            graph.register_library_package(element.id.clone());
        }
    }
}
```

### A.2: Add Library-Aware Resolution Methods

**File:** `sysml-text/src/lib.rs` (extend)

```rust
impl ParseResult {
    /// Resolve references with standard library loaded.
    ///
    /// This merges the library graph into the parsed graph before resolution,
    /// enabling resolution of standard types like Anything, Real, etc.
    pub fn into_resolved_with_library(mut self, library: ModelGraph) -> Self {
        // Merge library into our graph
        self.graph.merge(library, true);

        // Now resolve
        self.into_resolved()
    }
}
```

### A.3: Add Library Path Configuration

**File:** `sysml-text/src/config.rs` (new)

```rust
/// Parser configuration including library paths.
pub struct ParserConfig {
    /// Path to standard library directory.
    /// Should contain `library.kernel/` and `library.systems/` subdirs.
    pub library_path: Option<PathBuf>,

    /// Whether to automatically load and merge standard library.
    pub auto_load_library: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            library_path: None,
            auto_load_library: true,
        }
    }
}
```

### A.4: Environment Variable Support

Support `SYSML_LIBRARY_PATH` environment variable for library location.

```rust
impl ParserConfig {
    pub fn from_env() -> Self {
        Self {
            library_path: std::env::var("SYSML_LIBRARY_PATH")
                .ok()
                .map(PathBuf::from),
            auto_load_library: true,
        }
    }
}
```

### A.5: Verification Test

```rust
#[test]
fn resolve_with_standard_library() {
    let source = r#"
        package Test {
            part def Vehicle :> Items::Item;  // Should resolve to Items::Item
            attribute mass : ScalarValues::Real;  // Should resolve
        }
    "#;

    let library = load_standard_library(Path::new("/path/to/library")).unwrap();
    let result = parser.parse(&files).into_resolved_with_library(library);

    assert!(result.is_ok(), "All references should resolve with library");
}
```

---

## Phase B: Complete Feature Chaining

**Goal:** Implement `find_feature_type()` so feature chains like `vehicle.engine.pistons` resolve.

**Estimated Impact:** +10-15%

### B.1: Implement find_feature_type()

**File:** `sysml-core/src/resolution/scoping/chaining.rs`

```rust
/// Find the type of a feature by looking for FeatureTyping relationship.
fn find_feature_type(graph: &ModelGraph, feature_id: &ElementId) -> Option<ElementId> {
    // Look for FeatureTyping element that types this feature
    for element in graph.elements.values() {
        if element.kind == ElementKind::FeatureTyping {
            // Check if this typing's typedFeature matches our feature
            if let Some(typed_feature) = element.props.get("typedFeature") {
                if let Some(tf_id) = typed_feature.as_ref() {
                    if tf_id == feature_id {
                        // Found it! Return the type
                        if let Some(type_ref) = element.props.get("type") {
                            return type_ref.as_ref().cloned();
                        }
                    }
                }
            }
        }
    }
    None
}
```

### B.2: Implement resolve_feature_in_type()

```rust
/// Resolve a feature name within a type.
fn resolve_feature_in_type(
    graph: &ModelGraph,
    type_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // 1. Look in type's owned features (direct children)
    for child in graph.children_of(type_id) {
        if child.name.as_deref() == Some(name) && child.kind.is_feature() {
            return ScopedResolution::Found(child.id.clone());
        }
    }

    // 2. Look in inherited features (via Specialization chain)
    if let Some(general_id) = find_general_type(graph, type_id) {
        let inherited = resolve_feature_in_type(graph, &general_id, name);
        if !matches!(inherited, ScopedResolution::NotFound) {
            return inherited;
        }
    }

    ScopedResolution::NotFound
}

/// Find the general (supertype) of a type.
fn find_general_type(graph: &ModelGraph, type_id: &ElementId) -> Option<ElementId> {
    for element in graph.elements.values() {
        if element.kind == ElementKind::Specialization {
            if let Some(specific) = element.props.get("specific") {
                if specific.as_ref() == Some(type_id) {
                    return element.props.get("general").and_then(|g| g.as_ref().cloned());
                }
            }
        }
    }
    None
}
```

---

## Phase C: Multi-File Parsing Support

**Goal:** Support parsing multiple files together so cross-file imports resolve.

**Estimated Impact:** +5-10%

### C.1: Extend Parser Trait

```rust
impl PestParser {
    /// Parse multiple files into a single model graph.
    ///
    /// This allows cross-file references to resolve correctly.
    pub fn parse_multi(&self, files: &[SysmlFile]) -> ParseResult {
        let mut combined = ParseResult::default();

        for file in files {
            let result = self.parse(&[file.clone()]);

            // Merge graphs
            for (id, element) in result.graph.elements {
                combined.graph.elements.insert(id, element);
            }
            for (id, rel) in result.graph.relationships {
                combined.graph.relationships.insert(id, rel);
            }

            // Merge diagnostics
            combined.diagnostics.extend(result.diagnostics);
        }

        // Rebuild indexes after merge
        combined.graph.rebuild_indexes();

        combined
    }
}
```

### C.2: Corpus Test Update

```rust
#[test]
fn corpus_resolution_multi_file() {
    // Parse ALL corpus files together, then resolve
    let all_files: Vec<SysmlFile> = corpus_files.iter()
        .map(|f| SysmlFile::new(&f.relative_path, &f.content))
        .collect();

    let result = parser.parse_multi(&all_files).into_resolved();

    // Should have much higher resolution rate
}
```

---

## Phase D: Fix Incomplete Scoping Strategies

**Goal:** Complete the TODO implementations in scoping modules.

**Estimated Impact:** +3-5%

### D.1: Fix NonExpressionNamespace

**File:** `sysml-core/src/resolution/scoping/non_expression.rs`

The current implementation falls back instead of properly skipping expression scopes.

```rust
pub fn resolve_in_non_expression_namespace(
    graph: &ModelGraph,
    start_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    let mut current = start_id.clone();

    loop {
        let element = match graph.get_element(&current) {
            Some(e) => e,
            None => return ScopedResolution::NotFound,
        };

        // Skip if this is an expression scope
        if is_expression_scope(&element.kind) {
            if let Some(owner) = &element.owner {
                current = owner.clone();
                continue;
            }
            return ScopedResolution::NotFound;
        }

        // Try to resolve in this namespace
        let result = super::owning::resolve_in_owning_namespace(graph, &current, name);
        if !matches!(result, ScopedResolution::NotFound) {
            return result;
        }

        // Walk up
        match &element.owner {
            Some(owner) => current = owner.clone(),
            None => return ScopedResolution::NotFound,
        }
    }
}

fn is_expression_scope(kind: &ElementKind) -> bool {
    matches!(kind,
        ElementKind::Expression |
        ElementKind::InvocationExpression |
        ElementKind::OperatorExpression |
        ElementKind::FeatureChainExpression |
        // ... other expression kinds
    )
}
```

### D.2: Fix RelativeNamespace

**File:** `sysml-core/src/resolution/scoping/relative.rs`

Should stay within the relative namespace instead of walking up.

```rust
pub fn resolve_in_relative_namespace(
    graph: &ModelGraph,
    namespace_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // Only look in this namespace's members - don't walk up
    let namespace = match graph.get_element(namespace_id) {
        Some(e) => e,
        None => return ScopedResolution::NotFound,
    };

    // Look in direct children only
    for child in graph.children_of(namespace_id) {
        if child.name.as_deref() == Some(name) {
            return ScopedResolution::Found(child.id.clone());
        }
    }

    // Look in inherited members (for types)
    if namespace.kind.is_subtype_of(ElementKind::Type) {
        if let Some(result) = resolve_inherited(graph, namespace_id, name) {
            return ScopedResolution::Found(result);
        }
    }

    ScopedResolution::NotFound
}
```

### D.3: Fix TransitionSpecific

**File:** `sysml-core/src/resolution/scoping/transition.rs`

Implement state machine specific scoping for transitions.

---

## Phase E: Verification & Testing

### E.1: Resolution Rate Milestones

| Phase | Target Rate | Verification Command |
|-------|-------------|---------------------|
| A (Library) | 70-80% | `cargo test corpus_resolution_with_library` |
| B (Chaining) | 80-90% | `cargo test feature_chaining_resolution` |
| C (Multi-file) | 90-95% | `cargo test corpus_resolution_multi_file` |
| D (Scoping) | 95-100% | `cargo test corpus_resolution` |

### E.2: Integration Test Suite

```rust
// Test standard library types
#[test]
fn resolve_base_anything() { /* Anything type */ }

#[test]
fn resolve_scalar_real() { /* ScalarValues::Real */ }

// Test feature chaining
#[test]
fn resolve_vehicle_engine_pistons() { /* a.b.c chain */ }

// Test imports
#[test]
fn resolve_namespace_import() { /* import Pkg::* */ }

#[test]
fn resolve_membership_import() { /* import Pkg::Type */ }

// Test inheritance
#[test]
fn resolve_inherited_feature() { /* :> Base, use Base::feature */ }
```

---

## Implementation Order

1. **Phase A.1-A.3** (1-2 sessions): Library loader infrastructure
2. **Phase A.4-A.5** (1 session): Environment config and tests
3. **Phase B** (1-2 sessions): Feature chaining
4. **Phase C** (1 session): Multi-file parsing
5. **Phase D** (1-2 sessions): Scoping fixes
6. **Phase E** (1 session): Final verification

**Total Estimated Effort:** 6-9 sessions

---

## Success Criteria

```bash
# Must achieve 95%+ resolution on corpus
SYSML_CORPUS_PATH=... SYSML_LIBRARY_PATH=... \
cargo test -p sysml-spec-tests corpus_resolution -- --ignored --nocapture

# Output should show:
# Resolution rate: 95%+ (ideally 100%)
```

---

## Files to Create/Modify

| File | Change |
|------|--------|
| `sysml-text/src/library.rs` | NEW: Library loading |
| `sysml-text/src/config.rs` | NEW: Parser configuration |
| `sysml-text/src/lib.rs` | ADD: `into_resolved_with_library()` |
| `sysml-core/src/resolution/scoping/chaining.rs` | FIX: `find_feature_type()` |
| `sysml-core/src/resolution/scoping/non_expression.rs` | FIX: Skip expressions |
| `sysml-core/src/resolution/scoping/relative.rs` | FIX: Stay within namespace |
| `sysml-core/src/resolution/scoping/transition.rs` | FIX: State machine scoping |
| `sysml-spec-tests/tests/corpus_tests.rs` | ADD: Library-aware tests |
