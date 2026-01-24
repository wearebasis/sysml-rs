# sysml-rs Development Roadmap

This document outlines the phased development plan for completing the sysml-rs ecosystem. Each phase has clear success criteria and test examples that demonstrate completion.

## Overview

| Phase | Focus | Status | Est. Effort |
|-------|-------|--------|-------------|
| 0a | Foundation Hardening + ElementKind Codegen | ✅ Complete | 1-2 weeks |
| 0b | Shapes Codegen (Typed Properties) | ✅ Complete | 1-2 weeks |
| 0c | Type Hierarchy & Enumerations Codegen | ✅ Complete | 2-3 days |
| 1 | Core Model Integration | ✅ Complete | 1 week |
| 2 | Text Parsing (Pest Parser) | ✅ **MVP Complete** | 3-4 weeks |
| 2a | ↳ Grammar & Syntax | ✅ Complete | 1 week |
| 2b | ↳ Semantic Model Construction | ✅ Complete | 1-2 weeks |
| 2c | ↳ Coverage & Validation | ✅ MVP (corpus parses) | 1 week |
| 2d | ↳ Name Resolution | ✅ Complete | 8-12 sessions |
| 3 | Query & Analysis | ✅ Basic | 1-2 weeks |
| 4 | Visualization | ✅ Basic | 1 week |
| 5 | State Machine Execution | 🟡 Basic | 2 weeks |
| 6 | Constraint Evaluation | 🔴 Stub | 2 weeks |
| 7 | Storage & Persistence | ✅ Basic | 1-2 weeks |
| 8 | REST API + Contract Tests | 🟡 Basic | 1 week |
| 9 | LSP Server | 🟡 Stub | 2-3 weeks |
| 10 | Integration & Polish | 🔴 Not Started | 2 weeks |

### Current Milestone: MVP Parser Complete 🎉

The pest parser successfully parses the SysML v2 standard library corpus. The pipeline works:

```
.sysml files → PestParser → ModelGraph → query/visualize/serialize/execute
```

**What works now:**
- Parse any valid .sysml file to ModelGraph
- Query elements by name, kind, relationships
- Export to DOT, PlantUML, Cytoscape JSON
- Serialize/deserialize models (JSON)
- Basic state machine execution
- REST API for model storage

**Key limitations (for future phases):**
- ✅ Name resolution now available (via `ParseResult::into_resolved()`)
- No expression evaluation
- No standard library pre-loaded
- LSP uses stub parser (not pest)

### Suggested Next Steps (Pick One)

| Option | Description | Effort | Unlocks |
|--------|-------------|--------|---------|
| **A. CLI Tool** | Build `sysml` CLI to parse files and output diagrams | Small | Immediate usability, demo-able |
| **B. Name Resolution** | ✅ DONE - `ParseResult::into_resolved()` | Medium | Semantic queries, go-to-definition |
| **C. LSP Integration** | Wire pest parser into LSP server | Medium | IDE experience with real parsing |
| **D. Expression Eval** | Parse and evaluate constraint expressions | Medium-Large | Constraint checking |
| **E. Web Viewer** | Simple web UI to visualize parsed models | Medium | Demo-able product |

**Recommendation:** Option A (CLI Tool) provides the fastest path to a usable product. Example:
```bash
sysml parse vehicle.sysml --format json > model.json
sysml diagram vehicle.sysml --view bdd --format svg > diagram.svg
sysml query vehicle.sysml "requirements where unverified"
```

> **Coverage Strategy**: This project uses **build-time spec validation**—the build FAILS if generated code doesn't match the spec. This guarantees 100% coverage of types, enums, and relationships. See [Appendix B: Codegen & Coverage Strategy](#appendix-b-codegen--coverage-strategy) for details.

---

## Phase 0: Foundation Hardening + Codegen Infrastructure

**Goal**: Ensure the foundation crates are production-ready AND set up the codegen infrastructure that guarantees spec coverage.

### Success Criteria

- [ ] All public APIs have doc comments with examples
- [ ] Error types are comprehensive and user-friendly
- [ ] Serde serialization/deserialization roundtrips perfectly
- [ ] 100% test coverage on public APIs
- [ ] Benchmarks for critical paths (ID generation, parsing)
- [ ] **Codegen**: `codegen/` crate can parse TTL vocab files
- [ ] **Codegen**: `ElementKind` enum is generated from `SysML-vocab.ttl` + `Kerml-Vocab.ttl` (266 types)
- [ ] **Codegen**: `RelationshipKind` enum is generated from shapes files
- [ ] **Codegen**: Build fails if generated code is out of sync with spec files

### Test Examples

```rust
// sysml-id: Qualified name edge cases
#[test]
fn qualified_name_unicode() {
    let qn: QualifiedName = "パッケージ::部品::属性".parse().unwrap();
    assert_eq!(qn.segments().len(), 3);
    assert_eq!(qn.to_string(), "パッケージ::部品::属性");
}

#[test]
fn qualified_name_escaping() {
    // Names with special characters
    let qn = QualifiedName::from_segments(vec![
        "My Package".to_string(),
        "Part-1".to_string(),
    ]);
    // Should handle display/parse roundtrip
}

// sysml-span: Multi-file diagnostics
#[test]
fn diagnostic_with_related_locations() {
    let primary = Span::with_location("main.sysml", 100, 110, 10, 5);
    let related = Span::with_location("types.sysml", 50, 60, 5, 1);

    let diag = Diagnostic::error("type mismatch")
        .with_span(primary)
        .with_related(related, "type defined here");

    assert_eq!(diag.related.len(), 1);
}

// sysml-meta: Value type coercion
#[test]
fn value_arithmetic() {
    let a = Value::Int(10);
    let b = Value::Float(2.5);

    // Should be able to compare/operate across numeric types
    assert!(a.as_float().unwrap() > b.as_float().unwrap());
}
```

### Deliverables
- [ ] `sysml-id/src/lib.rs` - Add unicode support, escaping
- [ ] `sysml-span/src/lib.rs` - Add related diagnostic locations
- [ ] `sysml-meta/src/lib.rs` - Add Value comparison, arithmetic helpers
- [ ] Benchmark suite in `benches/`

### Codegen Deliverables

> **Context**: The SysML v2 spec defines exactly 266 element types (84 KerML + 182 SysML) in TTL vocabulary files. Rather than hand-writing these enums (error-prone, drift-prone), we generate them from the spec files at build time.

- [ ] `codegen/Cargo.toml` - New crate for build-time code generation
- [ ] `codegen/src/lib.rs` - Public API for other crates' build.rs
- [ ] `codegen/src/ttl_parser.rs` - Parse TTL vocab files to extract type names
- [ ] `codegen/src/enum_generator.rs` - Generate Rust enum source code
- [ ] `sysml-core/build.rs` - Invoke codegen to generate `element_kind.generated.rs`

**Codegen Test Example:**
```rust
// codegen/src/lib.rs
#[test]
fn parses_sysml_vocab() {
    let types = parse_ttl_vocab("../sysmlv2-references/SysML-vocab.ttl");

    // Should find all 182 SysML types
    assert!(types.contains("PartDefinition"));
    assert!(types.contains("PartUsage"));
    assert!(types.contains("ActionDefinition"));
    assert!(types.contains("RequirementDefinition"));
    assert_eq!(types.len(), 182);
}

#[test]
fn parses_kerml_vocab() {
    let types = parse_ttl_vocab("../sysmlv2-references/Kerml-Vocab.ttl");

    // Should find all 84 KerML types
    assert!(types.contains("Element"));
    assert!(types.contains("Relationship"));
    assert!(types.contains("Feature"));
    assert_eq!(types.len(), 84);
}

#[test]
fn generates_element_kind_enum() {
    let kerml = parse_ttl_vocab("../sysmlv2-references/Kerml-Vocab.ttl");
    let sysml = parse_ttl_vocab("../sysmlv2-references/SysML-vocab.ttl");

    let code = generate_enum("ElementKind", &kerml, &sysml);

    // Generated code should compile and have all variants
    assert!(code.contains("pub enum ElementKind"));
    assert!(code.contains("PartDefinition,"));
    assert!(code.contains("Element,"));
}
```

**How Codegen Links to sysml-core:**
```
sysmlv2-references/
├── Kerml-Vocab.ttl          ─┐
└── SysML-vocab.ttl          ─┼─► codegen/src/ttl_parser.rs
                               │
codegen/                       │
├── src/enum_generator.rs    ◄─┘
│
sysml-core/
├── build.rs                 ─► calls codegen::generate_element_kind()
└── src/
    └── element_kind.generated.rs  ◄─ output written by build.rs
```

---

## Phase 0b: Shapes Codegen (Typed Properties)

**Goal**: Generate typed property accessors and validation from OSLC shapes files, providing type-safe access to element properties.

### Success Criteria

- [x] Parse OSLC shapes from `KerML-shapes.ttl` and `SysML-shapes.ttl`
- [x] Extract property definitions with names, types, and cardinalities
- [x] Resolve property inheritance through type hierarchy
- [x] Generate typed property accessors for each element type
- [x] Generate validation methods based on cardinality constraints
- [x] All shapes parsed (257 shapes: 83 KerML + 174 SysML) with 302 shared properties

### What We Generate

From the shapes files, we can extract:

| OSLC Property | Rust Mapping |
|---------------|--------------|
| `oslc:name "owningType"` | Method name: `owning_type()` |
| `oslc:occurs oslc:Zero-or-one` | Return type: `Option<ElementId>` |
| `oslc:occurs oslc:Zero-or-many` | Return type: `impl Iterator<Item = ElementId>` |
| `oslc:occurs oslc:Exactly-one` | Return type: `T` (required) |
| `oslc:range oslc_sysml:Type` | Value type: `ElementId` (reference to Type) |
| `dcterms:description "..."` | Doc comment |

### Generated Code Example

```rust
// Generated from SysML-shapes.ttl - PartUsageShape
impl PartUsage {
    /// The Type that is the owningType of the owningFeatureMembership of this Feature.
    pub fn owning_type(&self) -> Option<ElementId> {
        self.props.get("owningType").and_then(|v| v.as_element_id())
    }

    /// Whether this Usage is for a variation point or not.
    /// Required property (Exactly-one cardinality).
    pub fn is_variation(&self) -> bool {
        self.props.get("isVariation").and_then(|v| v.as_bool()).unwrap_or(false)
    }

    /// The usages of this Usage that are directedFeatures.
    pub fn directed_usages(&self) -> impl Iterator<Item = ElementId> + '_ {
        self.props.get("directedUsage")
            .and_then(|v| v.as_list())
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_element_id())
    }

    /// Validate this element against OSLC shape constraints.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        // Check required properties (Exactly-one)
        if self.props.get("isVariation").is_none() {
            errors.push(ValidationError::missing_required("isVariation"));
        }
        errors
    }
}
```

### Test Examples

```rust
// codegen/tests/shapes_parser_test.rs

#[test]
fn parse_sysml_shapes() {
    let shapes = parse_oslc_shapes("../sysmlv2-references/SysML-shapes.ttl");

    // Should find all SysML shapes
    assert!(shapes.contains_key("PartUsageShape"));
    assert!(shapes.contains_key("RequirementUsageShape"));

    // PartUsageShape should have properties
    let part_shape = &shapes["PartUsageShape"];
    assert!(part_shape.properties.iter().any(|p| p.name == "owningType"));
    assert!(part_shape.properties.iter().any(|p| p.name == "isVariation"));
}

#[test]
fn property_cardinality_parsed() {
    let shapes = parse_oslc_shapes("../sysmlv2-references/SysML-shapes.ttl");
    let part_shape = &shapes["PartUsageShape"];

    let is_variation = part_shape.properties.iter()
        .find(|p| p.name == "isVariation").unwrap();
    assert_eq!(is_variation.cardinality, Cardinality::ExactlyOne);

    let directed_usage = part_shape.properties.iter()
        .find(|p| p.name == "directedUsage").unwrap();
    assert_eq!(directed_usage.cardinality, Cardinality::ZeroOrMany);
}

#[test]
fn inheritance_resolved() {
    let shapes = parse_oslc_shapes("../sysmlv2-references/SysML-shapes.ttl");
    let kerml = parse_oslc_shapes("../sysmlv2-references/KerML-shapes.ttl");

    // Resolve inheritance: PartUsage -> ItemUsage -> Usage -> Feature -> Type -> Element
    let resolved = resolve_inheritance(&shapes, &kerml);
    let part_props = &resolved["PartUsage"];

    // Should have inherited properties from Element
    assert!(part_props.iter().any(|p| p.name == "elementId"));
    assert!(part_props.iter().any(|p| p.name == "name"));
}

// sysml-core/tests/typed_properties.rs

#[test]
fn typed_property_access() {
    let mut graph = ModelGraph::new();

    let part = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("engine")
        .with_prop("isVariation", false)
        .with_prop("isIndividual", true);

    let id = graph.add_element(part);
    let elem = graph.get_element(&id).unwrap();

    // Use generated typed accessors
    let part_usage = elem.as_part_usage().unwrap();
    assert_eq!(part_usage.is_variation(), false);
    assert_eq!(part_usage.is_individual(), true);
}

#[test]
fn validation_catches_missing_required() {
    let part = Element::new_with_kind(ElementKind::PartUsage)
        .with_name("engine");
        // Missing: isVariation (required)

    let part_usage = part.as_part_usage().unwrap();
    let errors = part_usage.validate();

    assert!(errors.iter().any(|e| e.property == "isVariation"));
}
```

### Deliverables

- [x] `codegen/src/shapes_parser.rs` - Parse OSLC shapes TTL format
- [x] `codegen/src/inheritance.rs` - Resolve property inheritance through type hierarchy
- [x] `codegen/src/accessor_generator.rs` - Generate typed property accessors
- [x] `codegen/src/validation_generator.rs` - Generate validation methods
- [x] `sysml-core/build.rs` - Extend to also generate property accessors
- [x] `sysml-core/src/validation.rs` - ValidationError and ValidationResult types
- [x] Generated `properties.generated.rs` - 174 accessor structs (filtered to ElementKind variants)

### Architecture (Implemented)

```
sysmlv2-references/
├── KerML-shapes.ttl        ─┐
└── SysML-shapes.ttl        ─┼─► codegen/src/shapes_parser.rs
                              │
codegen/                      │
├── src/shapes_parser.rs    ◄─┘  Parses OSLC shapes, extracts property info
├── src/inheritance.rs           Builds type hierarchy, resolves inherited props
├── src/accessor_generator.rs    Generates XxxProps<'a> structs + cast methods
└── src/validation_generator.rs  Generates validate() methods
│
sysml-core/
├── build.rs                 ─► Calls all codegen functions
├── src/validation.rs            ValidationError, ValidationResult types
└── target/.../out/
    ├── element_kind.generated.rs   (from Phase 0a)
    └── properties.generated.rs     (174 accessor structs + validation)
```

**Usage Example:**
```rust
let elem = Element::new_with_kind(ElementKind::PartUsage);
let props = elem.as_part_usage().unwrap();  // Returns PartUsageProps<'_>
let owner = props.owning_type();            // Returns Option<ElementId>
let errors = props.validate();              // Returns ValidationResult
```

---

## Phase 0c: Type Hierarchy & Enumerations Codegen ✅ COMPLETE

**Goal**: Complete TTL-based code generation by adding static type hierarchy information and value enumerations extracted from the spec files.

### Success Criteria

- [x] Generate static supertype chains for all 175 unique element types (266 raw = 84 KerML + 182 SysML with overlaps)
- [x] Generate category predicates (`is_definition()`, `is_usage()`, `is_relationship()`, `is_feature()`, `is_classifier()`)
- [x] Generate Definition↔Usage pair mappings
- [x] Generate 7 value enumeration types (FeatureDirectionKind, VisibilityKind, PortionKind, RequirementConstraintKind, StateSubactionKind, TransitionFeatureKind, TriggerKind)
- [x] Generate relationship source/target type constraints for all 16 relationship types
- [x] **Build-time spec validation**: Build FAILS if coverage checks don't pass

### What Was Generated

The codegen now produces three files at build time:

| File | Contents |
|------|----------|
| `element_kind.generated.rs` | ElementKind enum (175 variants), type hierarchy methods, relationship constraint methods |
| `enums.generated.rs` | 7 value enumeration types with iter(), as_str(), FromStr |
| `properties.generated.rs` | 175 typed property accessor structs + validation methods |

### Spec Coverage Validation (Build-Time)

The build validates 100% spec coverage by cross-checking authoritative sources:

| Check | Sources Compared | Result |
|-------|------------------|--------|
| **Type coverage** | TTL vocabulary vs XMI metamodel | **100%** (175 types) |
| **Enum coverage** | TTL vocabulary vs JSON schema | **100%** (7 enums) |
| **Relationship constraints** | XMI metamodel + fallback | **100%** (16 types) |

**Build output:**
```
Type coverage: 257 TTL, 175 XMI, 175 matched
Enum coverage: 7 TTL enums, 7 JSON enums
Spec coverage validation PASSED
Relationship constraints: 15/16 from XMI, 1 from fallback
```

### Files Created/Modified

- `codegen/src/hierarchy_generator.rs` - Supertype chains, predicates, def↔usage mapping
- `codegen/src/enum_value_generator.rs` - Value enumeration generation
- `codegen/src/relationship_generator.rs` - Relationship constraint methods
- `codegen/src/spec_validation.rs` - Type and enum coverage validation
- `codegen/src/xmi_class_parser.rs` - XMI class extraction for validation
- `codegen/src/json_schema_parser.rs` - JSON enum extraction for validation
- `sysml-core/build.rs` - Integrated all validation + generation

### What We Generate

**1. Static Supertype Chains** (from `rdfs:subClassOf` in vocab TTL):
```rust
impl ElementKind {
    /// Returns the direct and transitive supertypes of this element kind.
    pub const fn supertypes(&self) -> &'static [ElementKind] {
        match self {
            ElementKind::PartUsage => &[
                ElementKind::ItemUsage,
                ElementKind::OccurrenceUsage,
                ElementKind::Usage,
                ElementKind::Feature,
                ElementKind::Type,
                ElementKind::Namespace,
                ElementKind::Element,
            ],
            // ... 265 more
        }
    }

    /// Check if this type is a subtype of another.
    pub const fn is_subtype_of(&self, other: ElementKind) -> bool { ... }
}
```

**2. Category Predicates** (from naming patterns and inheritance):
```rust
impl ElementKind {
    /// True for all *Definition types (PartDefinition, ActionDefinition, etc.)
    pub const fn is_definition(&self) -> bool { ... }

    /// True for all *Usage types (PartUsage, ActionUsage, etc.)
    pub const fn is_usage(&self) -> bool { ... }

    /// True for all Relationship subtypes
    pub const fn is_relationship(&self) -> bool { ... }

    /// True for all Classifier subtypes
    pub const fn is_classifier(&self) -> bool { ... }
}
```

**3. Definition↔Usage Mapping** (from naming conventions):
```rust
impl ElementKind {
    /// For a Definition type, returns the corresponding Usage type.
    pub const fn corresponding_usage(&self) -> Option<ElementKind> {
        match self {
            ElementKind::PartDefinition => Some(ElementKind::PartUsage),
            ElementKind::ActionDefinition => Some(ElementKind::ActionUsage),
            // ... 22+ more pairs
            _ => None,
        }
    }

    /// For a Usage type, returns the corresponding Definition type.
    pub const fn corresponding_definition(&self) -> Option<ElementKind> { ... }
}
```

**4. Value Enumerations** (from shapes TTL enum constraints):
```rust
/// Direction of a feature (in, out, inout).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureDirectionKind {
    In,
    Out,
    InOut,
}

/// Visibility of a member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisibilityKind {
    Public,
    Private,
    Protected,
}

/// Kind of portion for occurrence usages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PortionKind {
    Snapshot,
    Timeslice,
}

// Plus 6 more: RequirementConstraintKind, StateSubactionKind,
// TransitionFeatureKind, TriggerKind, etc.
```

### Test Examples

```rust
// codegen/tests/hierarchy_test.rs

#[test]
fn supertype_chains_correct() {
    let supertypes = ElementKind::PartUsage.supertypes();

    assert!(supertypes.contains(&ElementKind::ItemUsage));
    assert!(supertypes.contains(&ElementKind::Usage));
    assert!(supertypes.contains(&ElementKind::Feature));
    assert!(supertypes.contains(&ElementKind::Element));

    // Element has no supertypes
    assert!(ElementKind::Element.supertypes().is_empty());
}

#[test]
fn is_subtype_of_works() {
    assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Feature));
    assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Element));
    assert!(!ElementKind::PartUsage.is_subtype_of(ElementKind::Relationship));
}

#[test]
fn category_predicates_correct() {
    assert!(ElementKind::PartDefinition.is_definition());
    assert!(!ElementKind::PartDefinition.is_usage());

    assert!(ElementKind::PartUsage.is_usage());
    assert!(!ElementKind::PartUsage.is_definition());

    assert!(ElementKind::Specialization.is_relationship());
    assert!(!ElementKind::PartUsage.is_relationship());
}

#[test]
fn definition_usage_pairs() {
    assert_eq!(
        ElementKind::PartDefinition.corresponding_usage(),
        Some(ElementKind::PartUsage)
    );
    assert_eq!(
        ElementKind::PartUsage.corresponding_definition(),
        Some(ElementKind::PartDefinition)
    );

    // Element has no corresponding type
    assert_eq!(ElementKind::Element.corresponding_usage(), None);
}

// sysml-core/tests/enums_test.rs

#[test]
fn feature_direction_kind() {
    assert_eq!(FeatureDirectionKind::In.as_str(), "in");
    assert_eq!(FeatureDirectionKind::from_str("out"), Some(FeatureDirectionKind::Out));

    // Serde roundtrip
    let json = serde_json::to_string(&FeatureDirectionKind::InOut).unwrap();
    assert_eq!(json, "\"inout\"");
}

#[test]
fn visibility_kind() {
    for kind in VisibilityKind::iter() {
        let s = kind.as_str();
        assert_eq!(VisibilityKind::from_str(s), Some(kind));
    }
}
```

### Deliverables

- [ ] `codegen/src/hierarchy_generator.rs` - Generate supertype chains and predicates
- [ ] `codegen/src/enum_value_generator.rs` - Generate value enumerations from shapes
- [ ] `codegen/src/ttl_parser.rs` - Extend to extract inheritance relationships
- [ ] `sysml-core/build.rs` - Extend to call new generators
- [ ] Generated code in `element_kind.generated.rs` - Add methods to ElementKind
- [ ] Generated `enums.generated.rs` - Value enumeration types

### Architecture

```
sysmlv2-references/
├── Kerml-Vocab.ttl        ─┬─► codegen/src/hierarchy_generator.rs
└── SysML-vocab.ttl        ─┘   (extracts rdfs:subClassOf relationships)
                              │
├── KerML-shapes.ttl       ─┬─► codegen/src/enum_value_generator.rs
└── SysML-shapes.ttl       ─┘   (extracts enum value sets)
                              │
sysml-core/
├── build.rs               ─► Calls hierarchy + enum generators
└── target/.../out/
    ├── element_kind.generated.rs  (extended with supertypes, predicates)
    └── enums.generated.rs         (NEW: FeatureDirectionKind, etc.)
```

---

## Phase 1: Core Model Integration ✅ COMPLETE

**Goal**: Wire up the generated code into a working ModelGraph that can represent full SysML models with proper ownership and relationships.

> **Note**: With Phase 0 complete, most of the "type coverage" work is now generated. Phase 1 focuses on **integrating** the generated code into a usable model layer.

### What's Already Done (via Codegen)

| Component | Status | Source |
|-----------|--------|--------|
| All 175 element types | ✅ Generated | `ElementKind` enum |
| Type hierarchy (supertypes, is_subtype_of) | ✅ Generated | `element_kind.generated.rs` |
| Category predicates (is_definition, is_usage, etc.) | ✅ Generated | `element_kind.generated.rs` |
| Definition↔Usage mapping | ✅ Generated | `element_kind.generated.rs` |
| Relationship constraints | ✅ Generated | `element_kind.generated.rs` |
| Value enumerations | ✅ Generated | `enums.generated.rs` |
| Typed property accessors | ✅ Generated | `properties.generated.rs` |
| Property validation | ✅ Generated | `properties.generated.rs` |

### What Was Implemented

- [x] **Membership-based ownership**: SysML v2 compliant ownership via OwningMembership elements
- [x] **Ownership hierarchy**: Parent/child relationships via owning_membership → membershipOwningNamespace
- [x] **Qualified name resolution**: resolve_name(), resolve_qname(), resolve_path()
- [x] **Visibility filtering**: visible_members() returns only public members
- [x] **Structural validation**: Detects orphans, cycles, dangling memberships
- [x] **Element factory**: 40+ factory methods with type-appropriate defaults

### Test Examples

```rust
// Generated code works - these are now trivial given codegen
#[test]
fn generated_element_kinds_work() {
    // 175 unique types (84 KerML + 182 SysML, deduplicated)
    assert_eq!(ElementKind::count(), 266); // Count includes all raw

    // Type hierarchy is generated
    assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Feature));
    assert!(ElementKind::PartUsage.supertypes().contains(&ElementKind::ItemUsage));

    // Category predicates are generated
    assert!(ElementKind::PartDefinition.is_definition());
    assert!(ElementKind::PartUsage.is_usage());
    assert!(ElementKind::Specialization.is_relationship());

    // Definition↔Usage mapping is generated
    assert_eq!(
        ElementKind::PartDefinition.corresponding_usage(),
        Some(ElementKind::PartUsage)
    );
}

#[test]
fn relationship_constraints_are_validated() {
    // Relationship constraints are generated from XMI
    assert_eq!(
        ElementKind::FeatureTyping.relationship_source_type(),
        Some(ElementKind::Feature)
    );
    assert_eq!(
        ElementKind::FeatureTyping.relationship_target_type(),
        Some(ElementKind::Type)
    );
}

// Qualified name resolution
#[test]
fn resolve_qualified_name() {
    let mut graph = ModelGraph::new();

    // Build: Package1::Package2::PartDef1
    let pkg1 = graph.add_element(
        Element::new_with_kind(ElementKind::Package).with_name("Package1")
    );
    let pkg2 = graph.add_element(
        Element::new_with_kind(ElementKind::Package)
            .with_name("Package2")
            .with_owner(pkg1.clone())
    );
    let part = graph.add_element(
        Element::new_with_kind(ElementKind::PartDef)
            .with_name("PartDef1")
            .with_owner(pkg2.clone())
    );

    // Resolve by qualified name
    let resolved = graph.resolve_qname("Package1::Package2::PartDef1").unwrap();
    assert_eq!(resolved.id, part);
}

// Type specialization
#[test]
fn type_specialization_chain() {
    let mut graph = ModelGraph::new();

    let vehicle = graph.add_element(
        Element::new_with_kind(ElementKind::PartDef).with_name("Vehicle")
    );
    let car = graph.add_element(
        Element::new_with_kind(ElementKind::PartDef).with_name("Car")
    );
    let sedan = graph.add_element(
        Element::new_with_kind(ElementKind::PartDef).with_name("Sedan")
    );

    // Car specializes Vehicle
    graph.add_relationship(Relationship::new(
        RelationshipKind::Specialize, car.clone(), vehicle.clone()
    ));
    // Sedan specializes Car
    graph.add_relationship(Relationship::new(
        RelationshipKind::Specialize, sedan.clone(), car.clone()
    ));

    // Check transitive specialization
    assert!(graph.is_subtype_of(&sedan, &vehicle));
    assert!(graph.all_supertypes(&sedan).contains(&vehicle));
}
```

### Deliverables

**Already Complete (via Codegen):**
- [x] `element_kind.generated.rs` - ElementKind enum with 175 types
- [x] `element_kind.generated.rs` - Type hierarchy methods (supertypes, is_subtype_of)
- [x] `element_kind.generated.rs` - Category predicates (is_definition, is_usage, etc.)
- [x] `element_kind.generated.rs` - Relationship constraint methods
- [x] `enums.generated.rs` - 7 value enumeration types
- [x] `properties.generated.rs` - Typed property accessors for all 175 types

**Implemented Files:**
- [x] `sysml-core/src/membership.rs` - MembershipView, OwningMembershipView, MembershipBuilder
- [x] `sysml-core/src/ownership.rs` - create_owning_membership, add_owned_element, owner_of, ancestors
- [x] `sysml-core/src/namespace.rs` - owned_members, visible_members, resolve_name, resolve_qname, resolve_path
- [x] `sysml-core/src/structural_validation.rs` - StructuralError, validate_structure()
- [x] `sysml-core/src/factory.rs` - ElementFactory with 40+ factory methods

**Runnable Examples:**
- `cargo run --example basic_ownership -p sysml-core`
- `cargo run --example namespace_resolution -p sysml-core`
- `cargo run --example structural_validation -p sysml-core`
- `cargo run --example vehicle_model -p sysml-core`

**Coverage Tests:**
- `sysml-core/tests/coverage_tests.rs` - 25 tests verifying all ElementKind variants, enums, and factory methods

### Spec Coverage: Now Build-Time

> **Note**: Spec coverage validation moved from test-time to **build-time**. The build itself FAILS if the generated code doesn't match the spec. No separate `sysml-spec-tests` crate is needed for type/enum coverage.

**Build-time validation includes:**
- Type coverage: TTL vocabulary vs XMI metamodel (175 types)
- Enum coverage: TTL vocabulary vs JSON schema (7 enums)
- Relationship constraints: XMI metamodel (16 relationship types)

**To verify coverage:**
```bash
cargo build -p sysml-core  # Will FAIL if coverage checks don't pass
```

---

## Phase 2: Text Parsing (Pest Parser) ✅ MVP COMPLETE

**Goal**: Parse real SysML v2 text files into ModelGraph using the pest parser, verified by grammar coverage tests.

### Current Status (January 2026)

| Component | Status | Details |
|-----------|--------|---------|
| **Pest Grammar** | ✅ Complete | 569 rules, 174 auto-generated keywords |
| **Grammar Codegen** | ✅ Complete | build.rs extracts keywords/operators/enums from xtext |
| **Basic Parsing** | ✅ Complete | All unit/integration tests pass |
| **AST Converter** | ✅ Complete | Ownership ✅, Relationships ✅, Properties ✅ |
| **Semantic Model** | ✅ MVP | OwningMembership ✅, Relationships (unresolved) ✅ |
| **Corpus Tests** | ✅ MVP | Standard library corpus parses successfully |
| **Doc Comments** | ✅ Fixed | ML_COMMENT excluded from implicit COMMENT rule |

**Key Accomplishment**: Auto-generation of 174 keywords, 18 operators, and 7 enums from xtext spec files at build time.

**Current Status**: The parser constructs a semantic model with canonical ownership (via OwningMembership) and creates relationship elements. Relationship targets are stored as unresolved qualified name strings initially, then resolved to concrete ElementIds via `ParseResult::into_resolved()` (Phase 2d complete). Property extraction (multiplicity, values, direction) uses grammar rules; boolean flags still use `text.contains()`.

---

### Phase 2a: Grammar & Syntax ✅ COMPLETE

**Goal**: Parse SysML v2 textual notation into a parse tree with correct syntax validation.

#### What's Generated
| Component | Source | Output |
|-----------|--------|--------|
| 174 keyword rules (`KW_*`) | xtext files | `sysml.pest` |
| 18 operator rules | KerMLExpressions.xtext | `sysml.pest` |
| 7 enum rules | SysML.xtext | `sysml.pest` |

#### What's Hand-Written
| Component | Reason |
|-----------|--------|
| Grammar fragments (12 files) | Complex PEG semantics differ from xtext CFG |
| AST converter skeleton | Maps pest rules to ElementKind |

#### Verification ✅
```bash
cargo test -p sysml-text-pest        # 12/12 pass
cargo test -p sysml-codegen          # 75/75 pass
```

#### Known Issues (to fix in 2b/2c)
- ~~Documentation comment precedence (1 ignored test)~~ ✅ Fixed in header.pest
- 77 xtext rules not yet in pest grammar

---

### Phase 2b: Semantic Model Construction ✅ COMPLETE

**Goal**: Convert parse tree into semantically valid SysML v2 ModelGraph with proper relationships, ownership, and typed data.

#### Sub-stages

**2b.1: Canonical Ownership** ✅ COMPLETE
- [x] Replace `graph.add_element()` with `graph.add_owned_element()`
- [x] Create `OwningMembership` for all owned elements
- [x] Enable `visible_members`, `resolve_qname`, namespace semantics
- Tests: `ownership_tests.rs` (14 tests pass)

**2b.2: Relationship Construction** ✅ COMPLETE (with caveats)
- [x] Create `Specialization` from `:>` / `specializes` syntax
- [x] Create `FeatureTyping` from `:` / `typed by` syntax
- [x] Create `Subsetting` from `:>` / `subsets` syntax
- [x] Create `Redefinition` from `:>>` / `redefines` syntax
- [x] Create `ReferenceSubsetting` from `::>` / `references` syntax
- [x] Create `Dependency` with proper source/target
- Tests: `relationship_tests.rs` (19 tests pass)

**Note on Architecture:** Relationships are created as `Element` objects with relationship `ElementKind`s (e.g., `ElementKind::Specialization`). This follows the SysML v2 metamodel where relationships ARE elements. The `ModelGraph.relationships` field (which holds `Relationship` structs) is not populated - this is a legacy/alternative representation.

**Note on Resolution:** ✅ Phase 2d is now complete. Relationship targets are initially stored as unresolved qualified name strings (e.g., `unresolved_general`, `unresolved_type`). After calling `ParseResult::into_resolved()`, these are converted to concrete `ElementId`s.

**2b.3: Property Extraction** ✅ COMPLETE
- [x] Extract multiplicities (`[n]`, `[n..m]`, `[*]`, `[n..*]`) - uses grammar rules
- [x] Extract values (`= expr`, `:= expr`, `default = expr`) - uses grammar rules
- [x] Extract feature directions (`in`, `out`, `inout`) - uses grammar rules
- [x] Flag extraction (`abstract`, `variation`, `readonly`, `derived`, `end`, `ref`) - uses grammar rules via `extract_flags()`
- Tests: `property_tests.rs` (18 pass, 2 ignored for grammar gaps)

**Note:** All property extraction now uses proper grammar rule traversal via dedicated `extract_*()` methods. The `ExtractedFlags` struct and `extract_flags()` method look for specific grammar rules (`Rule::Abstract`, `Rule::Readonly`, etc.) rather than crude `text.contains()` matching, avoiding false positives (e.g., `attribute abstract_value` does NOT trigger `isAbstract`). False positive prevention tests added.

**2b.4: ElementKind Completeness** 🔴 NOT STARTED
- [ ] Remove ElementKind collapsing (Flow→Connection, etc.)
- [ ] Map all 77 constructible kinds correctly

#### Generation Opportunities

| Component | Source | Approach |
|-----------|--------|----------|
| Rule→ElementKind mapping | xtext rule names | Generate lookup table from xtext |
| Relationship patterns | xtext grammar | Generate extractor patterns |
| Property extractors | OSLC shapes | Generate from shape constraints |
| Semantic test assertions | xtext + shapes | Generate expected model structure |

#### Verification Criteria

```bash
# Ownership verification
cargo test -p sysml-text-pest test_ownership_creates_membership
# Expected: Every element has OwningMembership in graph

# Relationship verification
cargo test -p sysml-text-pest test_specialization_created
# Expected: `part def A :> B` creates Specialization relationship

# Property verification
cargo test -p sysml-text-pest test_multiplicity_extracted
# Expected: `part wheels[4]` has multiplicity lower=4, upper=4

# ElementKind verification
cargo test -p sysml-text-pest test_all_element_kinds
# Expected: All 77 constructible kinds can be produced
```

#### Deliverables

**Generated:**
- [ ] `codegen/src/rule_mapping_generator.rs` - Generate Rule→ElementKind table
- [ ] `codegen/src/relationship_extractor_generator.rs` - Generate relationship patterns
- [ ] `sysml-text-pest/src/extractors.generated.rs` - Property extraction code

**Hand-Written:**
- [x] `sysml-text-pest/src/ast/mod.rs` - Canonical ownership (`add_with_ownership()`)
- [x] `sysml-text-pest/src/ast/mod.rs` - Relationship construction (`create_*()` methods)
- [x] `sysml-text-pest/src/ast/mod.rs` - Property extraction (`extract_*()` methods)
- [x] `sysml-core/src/membership.rs` - Membership-based ownership helpers
- [x] `sysml-core/src/ownership.rs` - Ownership graph operations

---

### Phase 2c: Coverage & Validation ✅ MVP COMPLETE

**Goal**: Verify parser produces correct semantic models for all SysML v2 constructs.

#### Sub-stages

**2c.1: Corpus Coverage**
- [x] Fix documentation comment parsing (`ML_COMMENT` excluded from implicit `COMMENT` rule)
- [ ] Parse all corpus files without errors (currently ~56 in allow-list)
- [ ] Shrink allow-list to zero

**2c.2: ElementKind Coverage**
- Wire actual ElementKind tracking into corpus tests
- Verify all 77 constructible kinds are produced
- Generate coverage report

**2c.3: Semantic Correctness Tests**
- Generate test cases from xtext grammar patterns
- Verify relationships, ownership, properties for each pattern
- Add regression tests for semantic mapping

#### Generation Opportunities

| Component | Source | Approach |
|-----------|--------|----------|
| Test file patterns | xtext grammar | Generate minimal .sysml for each rule |
| Expected ElementKinds | xtext rule names | Generate expected kinds per file |
| Relationship assertions | xtext patterns | Generate expected relationships |
| Coverage reports | Generated tests | Auto-generate coverage dashboard |

#### Verification Criteria

```bash
# Corpus coverage
SYSML_CORPUS_PATH=... cargo test -p sysml-spec-tests -- --ignored
# Expected: 0 unexpected failures, allow-list empty

# ElementKind coverage
cargo test -p sysml-spec-tests element_kind_coverage -- --ignored
# Expected: 77/77 kinds produced (100% coverage)

# Semantic correctness (generated tests)
cargo test -p sysml-text-pest semantic_
# Expected: All generated semantic tests pass
```

#### Deliverables

**Generated:**
- [ ] `sysml-spec-tests/data/generated_test_patterns/` - Minimal .sysml files
- [ ] `sysml-spec-tests/src/generated_assertions.rs` - Expected model structure
- [ ] Coverage dashboard in test output

**Hand-Written:**
- [ ] Fix for documentation comment grammar precedence
- [ ] `sysml-spec-tests/tests/semantic_tests.rs` - Semantic verification harness

---

### Phase 2d: Name Resolution ✅ COMPLETE

**Goal**: Resolve all `unresolved_*` string references to concrete `ElementId`s.

**Status**: ✅ COMPLETE - Resolution infrastructure is fully implemented and wired to the parse pipeline.

**Architecture**: Implemented in `sysml-core/src/resolution/` with:
- `ScopeTable` - Import visibility cache per namespace
- `ResolutionContext` - Per-element resolution context
- `ResolutionResult` - Collected errors/diagnostics
- Multiple scoping strategies (owning, non-expression, relative, chaining, transition, global)

**Pipeline Integration**:
```rust
// One-liner for full parse → resolve pipeline
let result = parser.parse(&files).into_resolved();

// Or get detailed statistics
let mut result = parser.parse(&files);
let res = result.resolve();
println!("Resolved {} references", res.resolved_count);
```

#### Sub-stages

**2d.1: Scope Infrastructure** ✅
- [x] Create `sysml-core/src/resolution/mod.rs` module
- [x] Implement `ScopeTable` with local member lookup
- [x] Implement `resolve_name()` without imports/inheritance
- [x] Add tests for simple package/member resolution

**2d.2: Import Expansion** ✅
- [x] Expand `NamespaceImport` (::*) to visible members
- [x] Expand `MembershipImport` for specific elements
- [x] Handle recursive imports (::**)
- [x] Respect visibility (public/private/protected)
- [x] Add tests with import scenarios

**2d.3: Inheritance Resolution** ✅
- [x] Resolve inherited members via Specialization chain
- [x] Handle redefinition shadowing
- [x] Add tests with type hierarchies

**2d.4: Resolution Pass** ✅
- [x] Implement `resolve_references()` function
- [x] Resolve each relationship kind:
  - Specialization → `general` property
  - FeatureTyping → `type` property
  - Subsetting → `subsettedFeature` property
  - Redefinition → `redefinedFeature` property
  - ReferenceSubsetting → `referencedFeature` property
  - Dependency → `source/target` properties
  - Plus 20+ additional cross-reference types
- [x] Collect unresolved references as diagnostics

**2d.5: Standard Library** ✅ (Infrastructure)
- [x] Implement library loading mechanism (`ModelGraph::register_library_package()`)
- [x] Library package merging (`ModelGraph::merge(other, as_library)`)
- [ ] Pre-load standard library packages (future enhancement)

**2d.6: Pipeline Wiring** ✅ (NEW)
- [x] Add `ParseResult::into_resolved()` convenience method
- [x] Add `ParseResult::resolve()` for detailed statistics
- [x] Integration tests for parse → resolve pipeline
- [x] Corpus resolution test for quality measurement

#### Resolution Precedence

Per xtext KerMLScope.xtend, resolution follows this order:
1. **LOCAL**: Owned members of current namespace
2. **INHERITED**: Members via Specialization chain (for Types)
3. **IMPORTED**: Members from Import statements
4. **PARENT**: Walk up to parent namespace
5. **GLOBAL**: Root packages

#### Property Convention

- Keep `unresolved_*` for diagnostics/debugging
- Add resolved property alongside (e.g., `general` = ElementId)

#### What Exists (Starting Point)

**In `sysml-core/src/namespace.rs`:**
- `resolve_name(namespace_id, name)` - resolve within a namespace
- `resolve_qname(qname)` - resolve from root (e.g., "Pkg::Part")
- `resolve_path(namespace_id, path)` - resolve relative path

**Parser stores (in `sysml-text-pest`):**
- `unresolved_general` - Specialization supertype
- `unresolved_type` - FeatureTyping type
- `unresolved_subsettedFeature` - Subsetting
- `unresolved_redefinedFeature` - Redefinition
- `unresolved_referencedFeature` - ReferenceSubsetting
- `unresolved_sources/targets` - Dependency
- `importedReference`, `isNamespace`, `isRecursive` - Import

#### Test Examples

```rust
// 2d.1: Basic scope resolution
#[test]
fn resolve_local_member() {
    let source = r#"
        package Pkg {
            part def Engine;
            part engine : Engine;
        }
    "#;
    let graph = parse(source);
    graph.resolve_references();

    let engine_usage = graph.find_by_name(Some(&ElementKind::PartUsage), "engine").next().unwrap();
    let typing = graph.typing_of(&engine_usage.id).unwrap();

    // Should resolve to the local PartDefinition
    assert_eq!(typing.type_id, graph.find_by_name(Some(&ElementKind::PartDefinition), "Engine").next().unwrap().id);
}

// 2d.2: Import resolution
#[test]
fn resolve_imported_member() {
    let source = r#"
        package Types { part def Engine; }
        package Vehicle {
            import Types::*;
            part engine : Engine;  // Should resolve via import
        }
    "#;
    let graph = parse(source);
    graph.resolve_references();

    // Engine should resolve to Types::Engine
    let engine_def = graph.find_by_qname("Types::Engine").unwrap();
    let engine_usage = graph.find_by_name(Some(&ElementKind::PartUsage), "engine").next().unwrap();
    let typing = graph.typing_of(&engine_usage.id).unwrap();
    assert_eq!(typing.type_id, engine_def.id);
}

// 2d.3: Inheritance resolution
#[test]
fn resolve_inherited_member() {
    let source = r#"
        part def Base { part component; }
        part def Derived :> Base;
        part d : Derived {
            :>> component = something;  // Inherited from Base
        }
    "#;
    let graph = parse(source);
    graph.resolve_references();

    // component redefinition should link to Base::component
}
```

#### Verification Criteria

```bash
# Basic resolution tests
cargo test -p sysml-core resolution_

# Integration: parse + resolve
cargo test -p sysml-text-pest resolved_

# Unresolved reference diagnostics
cargo test -p sysml-text-pest unresolved_diagnostics
```

#### Deliverables

- [x] `sysml-core/src/resolution/mod.rs` - Resolution module (1100+ lines)
- [x] `sysml-core/src/resolution/scoping/` - Multiple scoping strategies
  - `owning.rs` - Owning namespace scope
  - `non_expression.rs` - Skip expression scopes
  - `relative.rs` - Relative to element
  - `chaining.rs` - Feature chain expressions
  - `transition.rs` - State machine transitions
  - `global.rs` - Root packages and library
- [x] `ResolutionContext` - Per-element resolution context
- [x] `sysml-text/src/lib.rs` - Pipeline integration (`into_resolved()`, `resolve()`)
- [x] `sysml-text-pest/tests/integration_tests.rs` - Resolution integration tests
- [x] `sysml-spec-tests/tests/corpus_tests.rs` - Corpus resolution test

**Actual Effort:** ~10 sessions (within estimate)

**Coverage Statistics** (from corpus test):
- 55 files parsed successfully
- 1181 references resolved
- 42.9% resolution rate (without standard library loaded)
- **66.0% resolution rate** (with standard library loaded via `load_standard_library()`)
- Library loading now available via `sysml_text::library` module

---

### Overall Phase 2 Success Criteria

| Stage | Criteria | Status |
|-------|----------|--------|
| **2a** | Parse valid .sysml files, helpful error messages | ✅ Complete |
| **2b** | Semantic model with relationships, ownership, properties | ✅ Complete |
| **2c** | Corpus coverage, ElementKind coverage | ✅ MVP |
| **2d** | Resolve `unresolved_*` refs to concrete ElementIds | ✅ Complete |

**Phase 2 Complete:**
- [x] Standard library corpus parses without errors
- [x] All relationships (Specialization, Typing, Subsetting, Redefinition, ReferenceSubsetting, Dependency) created
- [x] Multiplicity, values, directions extracted via grammar rules
- [x] Boolean flags extracted via grammar rules
- [x] Canonical ownership via OwningMembership
- [x] Name resolution via `ParseResult::into_resolved()`
- [x] Resolution diagnostics for unresolved references

**Remaining for full completion:**
- [ ] All 77 constructible ElementKinds verified (2c)
- [x] Relationship targets resolved to concrete ElementIds (Phase 2d) ✅
- [ ] Performance benchmarks: 10K lines in < 1 second
- [ ] Standard library preloading for better resolution rates

### Test Examples

```rust
// Basic parsing
#[test]
fn parse_simple_package() {
    let parser = SySideParser::new();
    let source = r#"
        package Vehicle {
            part def Engine;
            part def Wheel;

            part vehicle : Vehicle {
                part engine : Engine;
                part wheels : Wheel[4];
            }
        }
    "#;

    let result = parser.parse(&[SysmlFile::new("vehicle.sysml", source)]);

    assert!(!result.has_errors());
    assert_eq!(result.graph.element_count(), 5); // package + 2 defs + 1 usage + parts

    let pkg = result.graph.find_by_name(Some(&ElementKind::Package), "Vehicle").next().unwrap();
    assert_eq!(result.graph.children_of(&pkg.id).count(), 3);
}

// Error recovery
#[test]
fn parse_with_syntax_error() {
    let parser = SySideParser::new();
    let source = r#"
        package Test {
            part engine  // missing semicolon
            part wheels;
        }
    "#;

    let result = parser.parse(&[SysmlFile::new("test.sysml", source)]);

    assert!(result.has_errors());
    assert_eq!(result.diagnostics[0].span.as_ref().unwrap().line, Some(3));

    // Should still parse what it can
    assert!(result.graph.element_count() > 0);
}

// Multi-file parsing with imports
#[test]
fn parse_with_imports() {
    let parser = SySideParser::new();

    let types_file = SysmlFile::new("types.sysml", r#"
        package Types {
            part def Engine {
                attribute power : Real;
            }
        }
    "#);

    let vehicle_file = SysmlFile::new("vehicle.sysml", r#"
        package Vehicle {
            import Types::*;

            part car {
                part engine : Engine {
                    :>> power = 200.0;
                }
            }
        }
    "#);

    let result = parser.parse(&[types_file, vehicle_file]);

    assert!(!result.has_errors());

    // Verify the reference is resolved
    let engine_usage = result.graph.find_by_name(
        Some(&ElementKind::PartUsage), "engine"
    ).next().unwrap();

    let type_rel = result.graph.outgoing(&engine_usage.id)
        .find(|r| r.kind == RelationshipKind::TypeOf)
        .unwrap();

    let engine_def = result.graph.get_element(&type_rel.target).unwrap();
    assert_eq!(engine_def.name.as_deref(), Some("Engine"));
}

// Spans preserved
#[test]
fn spans_track_source_location() {
    let parser = SySideParser::new();
    let source = "package Test {\n    part engine;\n}";

    let result = parser.parse(&[SysmlFile::new("test.sysml", source)]);
    let engine = result.graph.find_by_name(None, "engine").next().unwrap();

    let span = &engine.spans[0];
    assert_eq!(span.line, Some(2));
    assert_eq!(span.col, Some(10)); // "part engine" starts at col 10
}
```

### Deliverables

**Completed:**
- [x] `codegen/src/pest_generator.rs` - Generate pest grammar from xtext
- [x] `codegen/src/xtext_parser.rs` - Parse xtext keywords, operators, enums
- [x] `sysml-text-pest/build.rs` - Orchestrate grammar generation
- [x] `sysml-text-pest/src/grammar/fragments/` - 12 manual grammar fragments
- [x] `sysml-text-pest/src/grammar/sysml.pest` - Generated grammar (1527 lines)
- [x] `sysml-text-pest/src/ast/mod.rs` - AST converter (807 lines)
- [x] `sysml-text-pest/src/lib.rs` - Parser trait implementation
- [x] `sysml-spec-tests/` - Coverage testing infrastructure

**Remaining:**
- [ ] Fix documentation comment precedence in grammar
- [ ] Expand AST converter to produce all 77 element kinds
- [ ] Add missing grammar rules for full spec coverage
- [ ] Integration test suite with real SysML files

### Grammar Coverage Test Deliverables

> **Context**: The SysML v2 Pilot Implementation includes extensive test files. Parsing ALL of them proves our parser handles the full grammar. If any file fails, we know exactly what syntax we're missing.

- [ ] `sysml-spec-tests/tests/grammar_coverage.rs` - Parse all Pilot test files

**Grammar Coverage Test Example:**
```rust
// sysml-spec-tests/tests/grammar_coverage.rs

/// Parse every .sysml file from the Pilot test suite
/// This is the definitive test that our parser handles the full SysML v2 grammar
#[test]
fn parser_handles_all_pilot_examples() {
    let test_dirs = [
        "../sysmlv2-references/SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.systems",
        "../sysmlv2-references/SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.kernel",
    ];

    let parser = create_parser();
    let mut failures: Vec<(PathBuf, String)> = vec![];
    let mut success_count = 0;

    for dir in test_dirs {
        for entry in glob(&format!("{}/**/*.sysml", dir)).unwrap() {
            let path = entry.unwrap();
            match parser.parse_file(&path) {
                Ok(result) if !result.has_errors() => success_count += 1,
                Ok(result) => {
                    let errors: Vec<_> = result.diagnostics.iter()
                        .map(|d| d.message.clone())
                        .collect();
                    failures.push((path, errors.join("; ")));
                }
                Err(e) => {
                    failures.push((path, e.to_string()));
                }
            }
        }
    }

    // Report results
    println!("Parsed {} files successfully", success_count);

    if !failures.is_empty() {
        println!("\nFailed to parse {} files:", failures.len());
        for (path, error) in &failures {
            println!("  {}: {}", path.display(), error);
        }
        panic!("Grammar coverage incomplete: {} failures", failures.len());
    }
}

/// Track which grammar constructs we've seen across all test files
#[test]
fn grammar_construct_coverage() {
    let test_files = collect_pilot_test_files();
    let mut seen_constructs: HashSet<&str> = HashSet::new();

    for file in test_files {
        let result = parse_file(&file);
        for elem in result.graph.elements() {
            seen_constructs.insert(elem.kind.as_str());
        }
    }

    // Verify we've seen examples of key constructs
    let required_constructs = [
        "Package", "PartDefinition", "PartUsage",
        "ActionDefinition", "ActionUsage",
        "StateDefinition", "StateUsage", "TransitionUsage",
        "RequirementDefinition", "RequirementUsage",
        "ConstraintDefinition", "ConstraintUsage",
        // ... add more as needed
    ];

    let missing: Vec<_> = required_constructs.iter()
        .filter(|c| !seen_constructs.contains(*c))
        .collect();

    assert!(missing.is_empty(),
        "Test files don't cover these constructs: {:?}", missing);
}
```

**How Grammar Tests Link to Specs:**
```
sysmlv2-references/SysML-v2-Pilot-Implementation/
└── org.omg.sysml.xpect.tests/
    ├── library.systems/        ─┐
    │   ├── Parts.sysml          │
    │   ├── Ports.sysml          ├─► sysml-spec-tests/tests/grammar_coverage.rs
    │   ├── Actions.sysml        │   (parse every file, track success/failure)
    │   ├── States.sysml         │
    │   └── ...                  │
    └── library.kernel/         ─┘
```

---

## Phase 3: Query & Analysis

**Goal**: Provide powerful query capabilities for analyzing models.

### Success Criteria

- [ ] Graph traversal queries (ancestors, descendants, reachable)
- [ ] Pattern matching queries
- [ ] Impact analysis (what depends on X?)
- [ ] Coverage analysis (which requirements are satisfied?)
- [ ] Cycle detection

### Test Examples

```rust
// Impact analysis
#[test]
fn impact_analysis() {
    let graph = load_test_model(); // Vehicle with parts and requirements

    let engine_def = graph.find_by_name(Some(&ElementKind::PartDef), "Engine")
        .next().unwrap();

    // What would be affected if we change Engine?
    let impact = query::impact_of(&graph, &engine_def.id);

    // Should include all usages, subtypes, and requirements that reference it
    assert!(impact.usages.len() > 0);
    assert!(impact.requirements.len() > 0);
}

// Requirement coverage
#[test]
fn requirement_coverage_analysis() {
    let graph = load_test_model();

    let coverage = query::requirement_coverage(&graph);

    assert!(coverage.total_requirements > 0);
    assert!(coverage.satisfied_count <= coverage.total_requirements);
    assert!(coverage.verified_count <= coverage.satisfied_count);

    // Get unsatisfied requirements
    for req in coverage.unsatisfied() {
        println!("Unsatisfied: {}", req.name.as_deref().unwrap_or("unnamed"));
    }
}

// Cycle detection
#[test]
fn detect_specialization_cycles() {
    let mut graph = ModelGraph::new();

    let a = graph.add_element(Element::new_with_kind(ElementKind::PartDef).with_name("A"));
    let b = graph.add_element(Element::new_with_kind(ElementKind::PartDef).with_name("B"));
    let c = graph.add_element(Element::new_with_kind(ElementKind::PartDef).with_name("C"));

    // Create cycle: A -> B -> C -> A
    graph.add_relationship(Relationship::new(RelationshipKind::Specialize, a.clone(), b.clone()));
    graph.add_relationship(Relationship::new(RelationshipKind::Specialize, b.clone(), c.clone()));
    graph.add_relationship(Relationship::new(RelationshipKind::Specialize, c.clone(), a.clone()));

    let cycles = query::find_cycles(&graph, &[RelationshipKind::Specialize]);

    assert_eq!(cycles.len(), 1);
    assert_eq!(cycles[0].len(), 3);
}

// Pattern query
#[test]
fn query_by_pattern() {
    let graph = load_test_model();

    // Find all parts that have a port but no interface
    let pattern = query::Pattern::new()
        .element(ElementKind::PartUsage)
        .has_child(ElementKind::PortUsage)
        .not_has_relationship(RelationshipKind::TypeOf, ElementKind::InterfaceDef);

    let matches = query::find_matching(&graph, &pattern);

    for elem in matches {
        println!("Part without interface: {}", elem.name.as_deref().unwrap_or("?"));
    }
}
```

### Deliverables
- [ ] `sysml-query/src/traversal.rs` - Graph traversal functions
- [ ] `sysml-query/src/impact.rs` - Impact analysis
- [ ] `sysml-query/src/coverage.rs` - Requirement coverage
- [ ] `sysml-query/src/cycles.rs` - Cycle detection
- [ ] `sysml-query/src/pattern.rs` - Pattern matching DSL

---

## Phase 4: Visualization

**Goal**: Generate high-quality diagrams from models.

### Success Criteria

- [ ] BDD (Block Definition Diagrams)
- [ ] IBD (Internal Block Diagrams)
- [ ] Requirements diagrams
- [ ] State machine diagrams
- [ ] Sequence diagrams
- [ ] Custom styling/theming

### Test Examples

```rust
// Block Definition Diagram
#[test]
fn generate_bdd() {
    let graph = load_vehicle_model();

    let pkg = graph.find_by_name(Some(&ElementKind::Package), "Vehicle").next().unwrap();

    let bdd = vis::BlockDefinitionDiagram::new(&graph, &pkg.id)
        .include_attributes(true)
        .include_operations(true)
        .depth(2);

    let dot = bdd.to_dot();

    // Verify structure
    assert!(dot.contains("Vehicle"));
    assert!(dot.contains("Engine"));
    assert!(dot.contains("Wheel"));
    assert!(dot.contains("->"));  // Has relationships

    // Save for visual inspection
    std::fs::write("target/vehicle_bdd.dot", &dot).unwrap();
}

// Internal Block Diagram
#[test]
fn generate_ibd() {
    let graph = load_vehicle_model();

    let vehicle = graph.find_by_name(Some(&ElementKind::PartUsage), "vehicle")
        .next().unwrap();

    let ibd = vis::InternalBlockDiagram::new(&graph, &vehicle.id)
        .show_ports(true)
        .show_flows(true);

    let svg = ibd.to_svg(); // Requires graphviz

    assert!(svg.contains("<svg"));
    assert!(svg.contains("engine"));
}

// State machine diagram
#[test]
fn generate_state_diagram() {
    let graph = load_traffic_light_model();

    let sm = graph.elements_by_kind(&ElementKind::StateDef).next().unwrap();

    let diagram = vis::StateMachineDiagram::new(&graph, &sm.id)
        .highlight_state("Green");

    let plantuml = diagram.to_plantuml();

    assert!(plantuml.contains("@startuml"));
    assert!(plantuml.contains("[*] --> Red"));  // Initial state
    assert!(plantuml.contains("Red --> Green"));
}
```

### Deliverables
- [ ] `sysml-vis/src/bdd.rs` - Block Definition Diagram
- [ ] `sysml-vis/src/ibd.rs` - Internal Block Diagram
- [ ] `sysml-vis/src/req.rs` - Requirements diagram
- [ ] `sysml-vis/src/stm.rs` - State machine diagram
- [ ] `sysml-vis/src/seq.rs` - Sequence diagram
- [ ] `sysml-vis/src/style.rs` - Styling/theming

---

## Phase 5: State Machine Execution

**Goal**: Execute state machines with proper semantics.

### Success Criteria

- [ ] Hierarchical states (composite states)
- [ ] Parallel regions (orthogonal states)
- [ ] History states (shallow and deep)
- [ ] Guard conditions evaluated
- [ ] Entry/exit/do actions executed
- [ ] Event queuing and priority
- [ ] Trace/debug output

### Test Examples

```rust
// Hierarchical states
#[test]
fn hierarchical_state_machine() {
    let graph = parse_sysml(r#"
        state def TrafficLight {
            state Operating {
                entry; do sendHeartbeat;

                state Red { entry; do turnOnRed; }
                state Yellow { entry; do turnOnYellow; }
                state Green { entry; do turnOnGreen; }

                transition Red to Green when timer;
                transition Green to Yellow when timer;
                transition Yellow to Red when timer;
            }

            state Fault {
                entry; do flashAllLights;
            }

            transition Operating to Fault when sensorFailure;
            transition Fault to Operating.Red when reset;
        }
    "#);

    let mut runner = StateMachineRunner::from_graph(&graph).unwrap();

    // Initial state should be Operating.Red
    assert_eq!(runner.current_state_path(), vec!["Operating", "Red"]);

    // Cycle through lights
    runner.step(Some("timer"));
    assert_eq!(runner.current_state_path(), vec!["Operating", "Green"]);

    // Fault while in Green
    runner.step(Some("sensorFailure"));
    assert_eq!(runner.current_state_path(), vec!["Fault"]);

    // Reset goes to Operating.Red (entry point)
    runner.step(Some("reset"));
    assert_eq!(runner.current_state_path(), vec!["Operating", "Red"]);
}

// Parallel regions
#[test]
fn orthogonal_regions() {
    let graph = parse_sysml(r#"
        state def Phone {
            parallel {
                region Display {
                    state Off;
                    state On;
                    transition Off to On when powerButton;
                    transition On to Off when powerButton;
                }
                region Sound {
                    state Silent;
                    state Ring;
                    transition Silent to Ring when incomingCall;
                    transition Ring to Silent when answerCall;
                }
            }
        }
    "#);

    let mut runner = StateMachineRunner::from_graph(&graph).unwrap();

    // Both regions start in initial states
    assert!(runner.is_in_state("Display.Off"));
    assert!(runner.is_in_state("Sound.Silent"));

    // Events affect only relevant region
    runner.step(Some("incomingCall"));
    assert!(runner.is_in_state("Display.Off"));
    assert!(runner.is_in_state("Sound.Ring"));

    runner.step(Some("powerButton"));
    assert!(runner.is_in_state("Display.On"));
    assert!(runner.is_in_state("Sound.Ring"));
}

// Guard conditions
#[test]
fn guard_conditions() {
    let graph = parse_sysml(r#"
        state def Thermostat {
            attribute temperature : Real;

            state Idle;
            state Heating;
            state Cooling;

            transition Idle to Heating when checkTemp [temperature < 68];
            transition Idle to Cooling when checkTemp [temperature > 72];
            transition Heating to Idle when checkTemp [temperature >= 70];
            transition Cooling to Idle when checkTemp [temperature <= 70];
        }
    "#);

    let mut runner = StateMachineRunner::from_graph(&graph).unwrap();
    runner.set_attribute("temperature", 65.0);

    runner.step(Some("checkTemp"));
    assert_eq!(runner.current_state(), "Heating");

    runner.set_attribute("temperature", 71.0);
    runner.step(Some("checkTemp"));
    assert_eq!(runner.current_state(), "Idle");
}

// Execution trace
#[test]
fn execution_trace() {
    let mut runner = StateMachineRunner::from_graph(&graph).unwrap();
    runner.enable_tracing();

    runner.step(Some("event1"));
    runner.step(Some("event2"));

    let trace = runner.trace();

    assert_eq!(trace.len(), 2);
    assert_eq!(trace[0].event, Some("event1".to_string()));
    assert_eq!(trace[0].transition.from, "StateA");
    assert_eq!(trace[0].actions_executed, vec!["exitA", "transitionAction", "entryB"]);
}
```

### Deliverables
- [ ] `sysml-run-statemachine/src/ir.rs` - Enhanced IR with hierarchy
- [ ] `sysml-run-statemachine/src/compiler.rs` - Full compiler
- [ ] `sysml-run-statemachine/src/runner.rs` - Full runner with regions
- [ ] `sysml-run-statemachine/src/guards.rs` - Guard evaluation
- [ ] `sysml-run-statemachine/src/trace.rs` - Execution tracing

---

## Phase 6: Constraint Evaluation

**Goal**: Evaluate constraint expressions at runtime.

### Success Criteria

- [ ] Parse constraint expressions (KerML expression language)
- [ ] Type checking of expressions
- [ ] Evaluate boolean constraints
- [ ] Evaluate numeric constraints
- [ ] Support for OCL-like operations (forAll, exists, select)
- [ ] Constraint violation reporting

### Test Examples

```rust
// Boolean constraints
#[test]
fn evaluate_boolean_constraint() {
    let ctx = EvaluationContext::new()
        .with("isActive", true)
        .with("hasPermission", false);

    let result = evaluate("isActive and hasPermission", &ctx);
    assert_eq!(result, Value::Bool(false));

    let result = evaluate("isActive or hasPermission", &ctx);
    assert_eq!(result, Value::Bool(true));
}

// Numeric constraints
#[test]
fn evaluate_numeric_constraint() {
    let ctx = EvaluationContext::new()
        .with("speed", 50.0)
        .with("maxSpeed", 100.0)
        .with("minSpeed", 0.0);

    let result = evaluate("speed >= minSpeed and speed <= maxSpeed", &ctx);
    assert_eq!(result, Value::Bool(true));

    let result = evaluate("speed / maxSpeed", &ctx);
    assert_eq!(result, Value::Float(0.5));
}

// Collection operations
#[test]
fn evaluate_collection_constraints() {
    let ctx = EvaluationContext::new()
        .with("items", vec![1, 2, 3, 4, 5]);

    // All items positive
    let result = evaluate("items->forAll(x | x > 0)", &ctx);
    assert_eq!(result, Value::Bool(true));

    // At least one even
    let result = evaluate("items->exists(x | x % 2 == 0)", &ctx);
    assert_eq!(result, Value::Bool(true));

    // Filter
    let result = evaluate("items->select(x | x > 3)", &ctx);
    assert_eq!(result, Value::List(vec![Value::Int(4), Value::Int(5)]));
}

// Model-level constraint checking
#[test]
fn check_model_constraints() {
    let graph = load_constrained_model();

    let results = constraints::check_all(&graph);

    assert_eq!(results.total_constraints, 10);
    assert_eq!(results.satisfied, 8);
    assert_eq!(results.violated, 2);

    for violation in results.violations() {
        println!("Violated: {} at {:?}",
            violation.constraint.expr,
            violation.element.name);
    }
}

// Constraint from parsed model
#[test]
fn constraints_from_parsed_model() {
    let graph = parse_sysml(r#"
        part def Engine {
            attribute power : Real;
            attribute efficiency : Real;

            constraint powerConstraint {
                power >= 100.0 and power <= 500.0
            }

            constraint efficiencyConstraint {
                efficiency > 0.0 and efficiency <= 1.0
            }
        }

        part myEngine : Engine {
            :>> power = 250.0;
            :>> efficiency = 0.35;
        }
    "#);

    let my_engine = graph.find_by_name(None, "myEngine").next().unwrap();
    let results = constraints::check_element(&graph, &my_engine.id);

    assert!(results.all_satisfied());
}
```

### Deliverables
- [ ] `sysml-run-constraints/src/parser.rs` - Expression parser
- [ ] `sysml-run-constraints/src/typecheck.rs` - Type checking
- [ ] `sysml-run-constraints/src/evaluator.rs` - Expression evaluator
- [ ] `sysml-run-constraints/src/collections.rs` - Collection operations
- [ ] `sysml-run-constraints/src/checker.rs` - Model constraint checker

---

## Phase 7: Storage & Persistence

**Goal**: Reliable model storage with version history.

### Success Criteria

- [ ] PostgreSQL backend fully working
- [ ] Efficient delta storage (don't duplicate unchanged elements)
- [ ] Branching support
- [ ] Diff between commits
- [ ] Merge with conflict detection
- [ ] Export/import to files

### Test Examples

```rust
// Basic CRUD with PostgreSQL
#[tokio::test]
async fn postgres_crud() {
    let store = PostgresStore::new(&get_test_db_url()).await.unwrap();
    store.init_schema().await.unwrap();

    let project = ProjectId::new("test-project");
    let mut graph = ModelGraph::new();
    graph.add_element(Element::new_with_kind(ElementKind::Package).with_name("Test"));

    // Create
    let meta = SnapshotMeta::new(CommitId::new("v1"), "Initial commit");
    store.put_snapshot_async(&project, meta, &graph).await.unwrap();

    // Read
    let snapshot = store.get_snapshot_async(&project, &CommitId::new("v1")).await.unwrap().unwrap();
    let restored = snapshot.graph().unwrap();
    assert_eq!(restored.element_count(), 1);
}

// Version history
#[tokio::test]
async fn version_history() {
    let store = setup_store().await;
    let project = ProjectId::new("versioned-project");

    // Create 3 versions
    for i in 1..=3 {
        let mut graph = ModelGraph::new();
        for j in 0..i {
            graph.add_element(
                Element::new_with_kind(ElementKind::Package)
                    .with_name(format!("Package{}", j))
            );
        }

        let meta = SnapshotMeta::new(
            CommitId::new(format!("v{}", i)),
            format!("Version {}", i)
        ).with_parent(if i > 1 {
            Some(CommitId::new(format!("v{}", i-1)))
        } else {
            None
        });

        store.put_snapshot_async(&project, meta, &graph).await.unwrap();
    }

    // List history
    let commits = store.list_commits(&project).await.unwrap();
    assert_eq!(commits.len(), 3);
    assert_eq!(commits[0].commit.as_str(), "v3"); // Most recent first
}

// Diff between versions
#[tokio::test]
async fn diff_versions() {
    let store = setup_store_with_history().await;

    let diff = store.diff(
        &ProjectId::new("project"),
        &CommitId::new("v1"),
        &CommitId::new("v2")
    ).await.unwrap();

    assert_eq!(diff.added_elements.len(), 2);
    assert_eq!(diff.removed_elements.len(), 0);
    assert_eq!(diff.modified_elements.len(), 1);
}

// Branching
#[tokio::test]
async fn branching() {
    let store = setup_store().await;
    let project = ProjectId::new("branched-project");

    // Main branch
    store.create_branch(&project, "main").await.unwrap();
    store.put_to_branch(&project, "main", meta1, &graph1).await.unwrap();

    // Create feature branch from main
    store.create_branch_from(&project, "feature", "main").await.unwrap();
    store.put_to_branch(&project, "feature", meta2, &graph2).await.unwrap();

    // Branches have different heads
    let main_head = store.branch_head(&project, "main").await.unwrap();
    let feature_head = store.branch_head(&project, "feature").await.unwrap();

    assert_ne!(main_head, feature_head);
}
```

### Deliverables
- [ ] `sysml-store-postgres/src/schema.sql` - Full database schema
- [ ] `sysml-store-postgres/src/postgres.rs` - Complete PostgreSQL impl
- [ ] `sysml-store/src/diff.rs` - Model diffing
- [ ] `sysml-store/src/branch.rs` - Branching support
- [ ] `sysml-store/src/merge.rs` - Merging with conflicts
- [ ] `sysml-store/src/export.rs` - File export/import

---

## Phase 8: REST API + Contract Tests

**Goal**: Complete REST API for all operations, verified against the official SysML v2 API specification.

### Success Criteria

- [ ] OpenAPI 3.0 specification
- [ ] All CRUD operations
- [ ] Query endpoints
- [ ] Visualization endpoints
- [ ] WebSocket for real-time updates
- [ ] Authentication/authorization hooks
- [ ] **Coverage**: All responses validate against `SysmlAPISchema.json`
- [ ] **Coverage**: All endpoints match `OpenAPI.json` spec

### Test Examples

```rust
// OpenAPI spec generation
#[test]
fn generates_openapi_spec() {
    let spec = api::openapi_spec();

    assert!(spec.paths.contains_key("/projects"));
    assert!(spec.paths.contains_key("/projects/{id}/commits"));
    assert!(spec.paths.contains_key("/projects/{id}/commits/{commit}/model"));
    assert!(spec.paths.contains_key("/projects/{id}/commits/{commit}/query"));
    assert!(spec.paths.contains_key("/projects/{id}/commits/{commit}/visualize"));
}

// Query endpoint
#[tokio::test]
async fn query_endpoint() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/projects/test/commits/v1/query")
                .header("content-type", "application/json")
                .body(Body::from(r#"{
                    "type": "find_by_name",
                    "kind": "PartDef",
                    "name": "Engine"
                }"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: QueryResponse = parse_body(response).await;
    assert_eq!(body.results.len(), 1);
}

// Visualization endpoint
#[tokio::test]
async fn visualization_endpoint() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects/test/commits/v1/visualize?format=dot&type=bdd&root=Vehicle")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/vnd.graphviz"
    );
}

// WebSocket real-time updates
#[tokio::test]
async fn websocket_notifications() {
    let (app, mut rx) = setup_test_app_with_ws().await;

    // Subscribe to project changes
    let ws = connect_ws(&app, "/ws/projects/test").await;

    // Make a change via REST
    app.post("/projects/test/commits/v2/model", model_json).await;

    // Should receive notification
    let notification: Notification = ws.recv().await.unwrap();
    assert_eq!(notification.event, "commit_created");
    assert_eq!(notification.commit, "v2");
}
```

### Deliverables
- [ ] `sysml-api/src/openapi.rs` - OpenAPI spec generation
- [ ] `sysml-api/src/routes/query.rs` - Query endpoints
- [ ] `sysml-api/src/routes/visualize.rs` - Visualization endpoints
- [ ] `sysml-api/src/routes/diff.rs` - Diff endpoints
- [ ] `sysml-api/src/ws.rs` - WebSocket support
- [ ] `sysml-api/src/auth.rs` - Auth middleware hooks

### API Contract Test Deliverables

> **Context**: The SysML v2 spec includes both a JSON Schema (`SysmlAPISchema.json`) and OpenAPI spec (`OpenAPI.json`). Our API must conform to these exactly for interoperability with other SysML v2 tools.

- [ ] `sysml-spec-tests/tests/schema_validation.rs` - Validate JSON responses against schema
- [ ] `sysml-spec-tests/tests/openapi_contract.rs` - Verify endpoints match OpenAPI spec

**API Contract Test Examples:**
```rust
// sysml-spec-tests/tests/schema_validation.rs

use jsonschema::JSONSchema;

/// Verify all our serialized elements validate against the official JSON Schema
#[test]
fn element_serialization_matches_schema() {
    // Load the official SysML v2 API schema
    let schema_json = std::fs::read_to_string(
        "../sysmlv2-references/SysmlAPISchema.json"
    ).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema_json).unwrap();
    let compiled_schema = JSONSchema::compile(&schema).unwrap();

    // Test serialization for every element type
    for kind in ElementKind::iter() {
        let element = create_test_element(kind);
        let json = serde_json::to_value(&element).unwrap();

        let result = compiled_schema.validate(&json);
        if let Err(errors) = result {
            let error_msgs: Vec<_> = errors.map(|e| e.to_string()).collect();
            panic!(
                "Element {:?} failed JSON Schema validation:\n{}",
                kind,
                error_msgs.join("\n")
            );
        }
    }
}

/// Verify our query responses match the schema
#[test]
fn query_response_matches_schema() {
    let schema = load_schema_def("QueryResponse");

    let response = QueryResponse {
        results: vec![/* test elements */],
        total: 10,
        offset: 0,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(schema.validate(&json).is_ok());
}

// sysml-spec-tests/tests/openapi_contract.rs

/// Verify our endpoint paths match the official OpenAPI spec
#[test]
fn endpoints_match_openapi_spec() {
    // Load official OpenAPI spec
    let openapi: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("../sysmlv2-references/OpenAPI.json").unwrap()
    ).unwrap();

    let spec_paths: HashSet<String> = openapi["paths"]
        .as_object().unwrap()
        .keys().cloned().collect();

    // Get our implemented endpoints
    let our_endpoints = get_implemented_endpoints();

    // Every endpoint we implement should be in the spec
    for endpoint in &our_endpoints {
        assert!(
            spec_paths.contains(endpoint),
            "Endpoint {} not in OpenAPI spec", endpoint
        );
    }

    // Warn about unimplemented spec endpoints (not a failure, just tracking)
    let unimplemented: Vec<_> = spec_paths.difference(&our_endpoints).collect();
    if !unimplemented.is_empty() {
        println!("Note: {} spec endpoints not yet implemented", unimplemented.len());
    }
}

/// Verify request/response schemas match for each endpoint
#[test]
fn endpoint_schemas_match_openapi() {
    let openapi = load_openapi_spec();

    for (path, methods) in &openapi.paths {
        for (method, operation) in methods {
            if let Some(request_body) = &operation.request_body {
                // Verify our request types match the spec schema
                verify_request_schema(path, method, request_body);
            }

            for (status, response) in &operation.responses {
                // Verify our response types match the spec schema
                verify_response_schema(path, method, status, response);
            }
        }
    }
}
```

**How API Tests Link to Specs:**
```
sysmlv2-references/
├── SysmlAPISchema.json    ─► sysml-spec-tests/tests/schema_validation.rs
│                              (validates JSON serialization of all types)
│
└── OpenAPI.json           ─► sysml-spec-tests/tests/openapi_contract.rs
                               (validates endpoint paths and schemas)
```

---

## Phase 9: LSP Server

**Goal**: Full-featured language server for IDE integration.

### Success Criteria

- [ ] Diagnostics (syntax errors, semantic errors)
- [ ] Go to definition
- [ ] Find references
- [ ] Hover information
- [ ] Code completion
- [ ] Document symbols (outline)
- [ ] Rename refactoring
- [ ] Code actions (quick fixes)
- [ ] Formatting

### Test Examples

```rust
// Go to definition
#[tokio::test]
async fn goto_definition() {
    let server = setup_test_server().await;

    let doc = r#"
        package Types {
            part def Engine;
        }
        package Vehicle {
            import Types::*;
            part car {
                part engine : Engine;
            }
        }
    "#;

    server.open_document("test.sysml", doc).await;

    // Request definition of "Engine" in the usage
    let result = server.goto_definition("test.sysml", Position { line: 7, character: 30 }).await;

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].range.start.line, 2); // Line of "part def Engine"
}

// Find references
#[tokio::test]
async fn find_references() {
    let server = setup_test_server().await;

    server.open_document("test.sysml", doc).await;

    // Find all references to Engine (position of definition)
    let refs = server.find_references("test.sysml", Position { line: 2, character: 22 }).await;

    assert_eq!(refs.len(), 2); // Definition + 1 usage
}

// Code completion
#[tokio::test]
async fn code_completion() {
    let server = setup_test_server().await;

    let doc = r#"
        package Vehicle {
            part def Engine;
            part def Wheel;

            part car {
                part e
            }
        }
    "#;

    server.open_document("test.sysml", doc).await;

    // Request completion at "part e|"
    let completions = server.completion(
        "test.sysml",
        Position { line: 6, character: 22 }
    ).await;

    // Should suggest "Engine" as type
    assert!(completions.iter().any(|c| c.label == "Engine"));
}

// Hover information
#[tokio::test]
async fn hover_info() {
    let server = setup_test_server().await;

    server.open_document("test.sysml", doc).await;

    let hover = server.hover("test.sysml", Position { line: 7, character: 30 }).await;

    assert!(hover.contents.contains("part def Engine"));
    assert!(hover.contents.contains("Defined in: Types"));
}

// Rename refactoring
#[tokio::test]
async fn rename() {
    let server = setup_test_server().await;

    server.open_document("types.sysml", types_doc).await;
    server.open_document("vehicle.sysml", vehicle_doc).await;

    // Rename "Engine" to "Motor"
    let edits = server.rename(
        "types.sysml",
        Position { line: 2, character: 22 },
        "Motor"
    ).await;

    // Should have edits in both files
    assert!(edits.contains_key("types.sysml"));
    assert!(edits.contains_key("vehicle.sysml"));
}

// Code action - quick fix
#[tokio::test]
async fn code_action_quick_fix() {
    let server = setup_test_server().await;

    let doc = r#"
        package Vehicle {
            part car {
                part engine : Enigne;  // typo
            }
        }
    "#;

    server.open_document("test.sysml", doc).await;

    // Get diagnostics first
    let diags = server.diagnostics("test.sysml").await;
    assert!(diags[0].message.contains("undefined"));

    // Get code actions for the error
    let actions = server.code_actions("test.sysml", diags[0].range).await;

    // Should suggest "Did you mean: Engine"
    assert!(actions.iter().any(|a| a.title.contains("Engine")));
}
```

### Deliverables
- [ ] `sysml-lsp-server/src/definition.rs` - Go to definition
- [ ] `sysml-lsp-server/src/references.rs` - Find references
- [ ] `sysml-lsp-server/src/completion.rs` - Code completion
- [ ] `sysml-lsp-server/src/hover.rs` - Hover information
- [ ] `sysml-lsp-server/src/rename.rs` - Rename refactoring
- [ ] `sysml-lsp-server/src/actions.rs` - Code actions
- [ ] `sysml-lsp-server/src/format.rs` - Formatting

---

## Phase 10: Integration & Polish

**Goal**: End-to-end integration, documentation, and release preparation.

### Success Criteria

- [ ] End-to-end integration tests
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete (API docs, user guide)
- [ ] Example projects
- [ ] CI/CD pipeline
- [ ] Published to crates.io

### Test Examples

```rust
// End-to-end: Parse → Store → Query → Visualize
#[tokio::test]
async fn end_to_end_workflow() {
    // 1. Parse SysML files
    let parser = SySideParser::new();
    let result = parser.parse(&load_test_files("examples/vehicle"));
    assert!(!result.has_errors());

    // 2. Store in database
    let store = PostgresStore::new(&db_url()).await.unwrap();
    let project = ProjectId::new("vehicle");
    let meta = SnapshotMeta::new(CommitId::new("v1"), "Initial import");
    store.put_snapshot_async(&project, meta, &result.graph).await.unwrap();

    // 3. Query via API
    let client = ApiClient::new("http://localhost:3000");
    let reqs = client.query(&project, "v1", Query::UnverifiedRequirements).await.unwrap();

    // 4. Generate visualization
    let bdd = client.visualize(&project, "v1", VisualizeRequest {
        format: Format::Svg,
        diagram_type: DiagramType::Bdd,
        root: "Vehicle".to_string(),
    }).await.unwrap();

    std::fs::write("target/vehicle.svg", bdd).unwrap();

    // 5. Run state machine
    let sm = result.graph.elements_by_kind(&ElementKind::StateDef).next().unwrap();
    let mut runner = StateMachineRunner::new(
        StateMachineCompiler::compile_element(&result.graph, &sm.id).unwrap()
    );

    runner.step(Some("start"));
    assert_eq!(runner.current_state(), "Running");
}

// Performance: Parse large model
#[test]
fn parse_performance() {
    let parser = SySideParser::new();
    let large_model = generate_test_model(10_000); // 10K elements

    let start = std::time::Instant::now();
    let result = parser.parse(&[SysmlFile::new("large.sysml", &large_model)]);
    let elapsed = start.elapsed();

    assert!(!result.has_errors());
    assert!(elapsed.as_secs() < 2, "Parse took too long: {:?}", elapsed);
}

// Performance: Query large graph
#[test]
fn query_performance() {
    let graph = load_large_test_model(); // 100K elements

    let start = std::time::Instant::now();
    let results: Vec<_> = query::find_by_name_contains(&graph, None, "Engine").collect();
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 100, "Query took too long: {:?}", elapsed);
}
```

### Deliverables
- [ ] `tests/integration/` - End-to-end test suite
- [ ] `benches/` - Comprehensive benchmarks
- [ ] `docs/` - User documentation
- [ ] `examples/` - Example projects
- [ ] `.github/workflows/` - CI/CD configuration
- [ ] Release to crates.io

---

## Appendix A: File Locations

```
sysml-rs/
├── Cargo.toml
├── README.md
├── ROADMAP.md (this file)
├── DEPENDENCY.md
├── codegen/                    # Build-time code generation
│   ├── Cargo.toml
│   ├── README.md               # Comprehensive codegen documentation
│   └── src/
│       ├── lib.rs              # Public API exports
│       ├── ttl_parser.rs       # TTL vocabulary parsing (types + enums)
│       ├── enum_generator.rs   # ElementKind enum generation
│       ├── hierarchy_generator.rs    # Type hierarchy methods
│       ├── enum_value_generator.rs   # Value enum generation
│       ├── relationship_generator.rs # Relationship constraint methods
│       ├── shapes_parser.rs    # OSLC shapes parsing
│       ├── inheritance.rs      # Property inheritance resolution
│       ├── accessor_generator.rs    # Property accessor generation
│       ├── validation_generator.rs  # Validation method generation
│       ├── spec_validation.rs  # Spec coverage validation
│       ├── xmi_class_parser.rs # XMI class extraction
│       ├── xmi_relationship_parser.rs # XMI relationship constraints
│       └── json_schema_parser.rs    # JSON enum extraction
├── sysml-core/
│   ├── build.rs               # Invokes codegen, validates spec coverage
│   └── src/
│       └── lib.rs             # Includes generated code via include!()
├── target/.../build/sysml-core-*/out/  # Generated code (build output)
│   ├── element_kind.generated.rs   # ElementKind enum + hierarchy
│   ├── enums.generated.rs          # Value enumeration types
│   └── properties.generated.rs     # Property accessors + validation
├── benches/
│   ├── parsing.rs
│   ├── query.rs
│   └── serialization.rs
├── docs/
│   ├── user-guide.md
│   └── api-reference.md
├── examples/
│   ├── vehicle/           # Complete vehicle model example
│   ├── traffic-light/     # State machine example
│   └── requirements/      # Requirements tracing example
├── tests/
│   └── integration/
│       ├── parse_and_store.rs
│       ├── query_and_visualize.rs
│       └── execute_statemachine.rs
└── [other crates...]
```

---

## Appendix B: Codegen & Coverage Strategy

This project uses **build-time code generation with integrated spec validation** to ensure 100% coverage of the SysML v2 specification.

### Current Architecture

The codegen system generates code from multiple spec sources and validates coverage at build time:

| Source File | What We Generate | Validation |
|-------------|------------------|------------|
| `Kerml-Vocab.ttl` + `SysML-vocab.ttl` | ElementKind enum, type hierarchy | Cross-check against XMI |
| `KerML-shapes.ttl` + `SysML-shapes.ttl` | Typed property accessors, validation methods | Property inheritance resolution |
| `KerML.xmi` + `SysML.xmi` | Relationship constraints | Type-checked source/target |
| `*Kind.json` schema files | Value enumerations | Cross-check against TTL enums |

### What Gets Generated

| Component | Generated? | Source | Lines of Code |
|-----------|------------|--------|---------------|
| `ElementKind` enum (175 variants) | ✅ Yes | TTL vocab | ~800 |
| Type hierarchy methods | ✅ Yes | TTL vocab | ~2000 |
| Category predicates | ✅ Yes | TTL vocab | ~200 |
| Definition↔Usage mapping | ✅ Yes | TTL vocab | ~100 |
| Relationship constraints | ✅ Yes | XMI metamodel | ~200 |
| Value enumerations (7 types) | ✅ Yes | TTL vocab + JSON | ~400 |
| Typed property accessors | ✅ Yes | OSLC shapes | ~5000 |
| Validation methods | ✅ Yes | OSLC shapes | ~3000 |
| **Pest grammar keywords** | ✅ Yes | xtext | ~200 |
| **Pest grammar operators** | ✅ Yes | xtext | ~25 |
| **Pest grammar enums** | ✅ Yes | xtext | ~15 |

### What's Hand-Written

| Component | Why |
|-----------|-----|
| `Element` struct | Core data structure with idiomatic Rust API |
| `ModelGraph` | Graph operations, ownership, resolution |
| Parser | tree-sitter grammar, CST→AST conversion |
| JSON serialization | Custom @type discrimination, serde attributes |
| API handlers | Business logic, authentication |

### Build-Time Validation

The build **FAILS** if any coverage check fails:

```
cargo build -p sysml-core

# On success:
Type coverage: 257 TTL, 175 XMI, 175 matched
Enum coverage: 7 TTL enums, 7 JSON enums
Spec coverage validation PASSED
Relationship constraints: 15/16 from XMI, 1 from fallback

# On failure:
TYPE COVERAGE VALIDATION FAILED: 1 errors
TTL-only types: []
XMI-only types: ["MissingType"]
error: build failed
```

### Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                    sysmlv2-references/                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Kerml-Vocab.ttl ──┐                                                │
│  SysML-vocab.ttl ──┼──► ttl_parser.rs ──┬──► element_kind.generated.rs
│                    │                    │    (ElementKind enum)     │
│                    │                    │                           │
│  KerML.xmi ────────┼──► xmi_class_parser.rs                         │
│  SysML.xmi ────────┤    xmi_relationship_parser.rs                  │
│                    │         │                                      │
│                    │         └──► spec_validation.rs                │
│                    │              (BUILD FAILS if mismatch)         │
│                                                                     │
│  KerML-shapes.ttl ─┼──► shapes_parser.rs ──► properties.generated.rs
│  SysML-shapes.ttl ─┤    inheritance.rs        (175 accessor structs)│
│                    │                                                │
│  *Kind.json ───────┼──► json_schema_parser.rs ──► enums.generated.rs
│                    │    (enum values)             (7 enum types)    │
│                                                                     │
│  SysML.xtext ──────────┬─► codegen/src/xtext_parser.rs              │
│  KerML.xtext ──────────┼─► codegen/src/pest_generator.rs            │
│  KerMLExpressions.xtext┘     │                                      │
│                              └─► sysml-text-pest/src/grammar/sysml.pest
│                                  (174 keywords, 18 operators, 7 enums)
│                                                                     │
│  *.sysml test files ──► sysml-spec-tests (corpus coverage tests)    │
│                                                                     │
│  SysmlAPISchema.json ─► (future) API response validation            │
│  OpenAPI.json ────────► (future) endpoint contract tests            │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Commands

```bash
# Build and validate spec coverage
cargo build -p sysml-core

# Run codegen tests
cargo test -p sysml-codegen

# Build pest grammar (triggers generation from xtext)
cargo build -p sysml-text-pest

# Run parser tests
cargo test -p sysml-text-pest

# Run corpus coverage tests (requires sysmlv2-references)
SYSML_CORPUS_PATH=/path/to/sysmlv2-references cargo test -p sysml-spec-tests -- --ignored

# See generated code
ls target/debug/build/sysml-core-*/out/*.generated.rs

# See generated grammar
cat sysml-text-pest/src/grammar/sysml.pest | head -100
```

### When Validation Fails

| Failure | Cause | Fix |
|---------|-------|-----|
| "Types in XMI but not TTL" | XMI has new type we're not parsing | Update ttl_parser.rs |
| "Types in TTL but not XMI" | TTL has type that's not in metamodel | Check if it's an enum (filter it out) |
| "Enum values don't match" | TTL and JSON disagree on enum values | Check which source is authoritative |
| "Relationship type without constraint" | New relationship type added | Add to fallback or parse from XMI |

---

## Getting Started with Phase 1

Phase 0 (codegen infrastructure) is complete. To verify:

```bash
cd sysml-rs

# Build sysml-core - this runs all codegen and spec validation
cargo build -p sysml-core

# You should see:
# warning: sysml-core@0.1.0: Type coverage: 257 TTL, 175 XMI, 175 matched
# warning: sysml-core@0.1.0: Enum coverage: 7 TTL enums, 7 JSON enums
# warning: sysml-core@0.1.0: Spec coverage validation PASSED
# warning: sysml-core@0.1.0: Relationship constraints: 15/16 from XMI, 1 from fallback

# Run codegen tests
cargo test -p sysml-codegen

# See what was generated
ls target/debug/build/sysml-core-*/out/
# element_kind.generated.rs  - ElementKind enum + hierarchy + relationships
# enums.generated.rs         - 7 value enumeration types
# properties.generated.rs    - 175 typed property accessors
```

To start Phase 1 (Core Model Integration), the next tasks are:

1. **Wire ModelGraph to use generated accessors** - Connect `Element` to use the generated `as_*()` methods
2. **Implement ownership hierarchy** - Track parent/child relationships
3. **Add qualified name resolution** - Resolve `Package::Part::Feature` paths
4. **Add import handling** - Package imports affect name visibility

Pick a test example from Phase 1 and implement it!
