# sysml-codegen

Build-time code generator for SysML v2 Rust types, derived from the official OMG specification files.

## Overview

This crate generates Rust code from the SysML v2 specification at compile time. It parses multiple authoritative specification files and produces:

- **`ElementKind` enum** - All 266 SysML/KerML element types
- **Type hierarchy methods** - Supertypes, predicates, definition/usage mappings
- **Relationship constraints** - Source/target type constraints for relationships
- **Value enumerations** - 7 enumeration types (FeatureDirectionKind, etc.)
- **Property accessors** - Typed property access for each element type
- **Validation methods** - Property validation per element type

## Spec Coverage Validation

The build automatically validates that the generated code has **100% coverage** of the SysML v2 / KerML specifications by cross-checking multiple authoritative sources:

| Check | Sources Compared | Status |
|-------|------------------|--------|
| **Type coverage** | TTL vocabulary vs XMI metamodel | **100%** (175 types) |
| **Enum coverage** | TTL vocabulary vs JSON schema | **100%** (7 enums) |
| **Relationship constraints** | XMI metamodel + fallback definitions | **100%** (16 types) |

**The build will FAIL if any coverage check fails**, ensuring the generated code always matches the specification.

## Data Source Breakdown

The codegen uses three types of specification files:

### TTL (Turtle) Files - Primary Type Source

| File | Purpose | Data Extracted |
|------|---------|----------------|
| `Kerml-Vocab.ttl` | KerML vocabulary | 84 types, type hierarchy, 2 enums |
| `SysML-vocab.ttl` | SysML vocabulary | 182 types, type hierarchy, 7 enums |
| `KerML-shapes.ttl` | KerML OSLC shapes | Property definitions for KerML types |
| `SysML-shapes.ttl` | SysML OSLC shapes | Property definitions for SysML types |

**What TTL provides:**
- Complete type hierarchy with `rdfs:subClassOf` relationships
- Enum type definitions with values
- Property shapes (names, types, cardinality, descriptions)

### XMI Files - Relationship Constraints & Validation

| File | Purpose | Data Extracted |
|------|---------|----------------|
| `KerML/20250201/KerML.xmi` | KerML metamodel | Class definitions, relationship source/target constraints |
| `SysML/20250201/SysML.xmi` | SysML metamodel | Class definitions, relationship source/target constraints |

**What XMI provides:**
- Authoritative relationship source/target type constraints (via `redefinedProperty`)
- Complete list of metamodel classes for cross-validation against TTL

### JSON Schema Files - Enum Validation

| Directory | Purpose | Data Extracted |
|-----------|---------|----------------|
| `metamodel/*Kind.json` | JSON schemas | Enum values for cross-validation |

**What JSON provides:**
- Enum values for validation against TTL-parsed enums
- 7 enum files: FeatureDirectionKind, VisibilityKind, PortionKind, RequirementConstraintKind, StateSubactionKind, TransitionFeatureKind, TriggerKind

## Coverage Analysis

### Element Types: 100% Coverage

| Source | Types | Status |
|--------|-------|--------|
| KerML | 84 | ✅ Complete |
| SysML | 182 | ✅ Complete |
| **Unique Total** | **175** | ✅ (validated against XMI) |

Note: KerML and SysML share many types, resulting in 175 unique types.

### Value Enumerations: 100% Coverage

| Enum | Values | Validated |
|------|--------|-----------|
| FeatureDirectionKind | in, out, inout | ✅ TTL = JSON |
| VisibilityKind | public, private, protected | ✅ TTL = JSON |
| PortionKind | timeslice, snapshot | ✅ TTL = JSON |
| RequirementConstraintKind | assumption, requirement | ✅ TTL = JSON |
| StateSubactionKind | entry, do, exit | ✅ TTL = JSON |
| TransitionFeatureKind | trigger, guard, effect | ✅ TTL = JSON |
| TriggerKind | when, at, after | ✅ TTL = JSON |

### Relationship Constraints: 100% Coverage

| Source | Count | Description |
|--------|-------|-------------|
| XMI | 15 | Constraints from `redefinedProperty` in XMI |
| Fallback | 1 | Base `Relationship` type (Element → Element) |
| **Total** | **16** | All relationship types covered |

**Specific constraint examples:**
- `FeatureTyping`: Feature → Type
- `Subsetting`: Feature → Feature
- `Specialization`: Type → Type
- `Membership`: Namespace → Element
- `Relationship`: Element → Element (base type)

### Property Accessors: 175 Types

Property accessors are generated for 175 element types that have:
- A valid `ElementKind` variant
- Property definitions in the shapes files

## Architecture

```
codegen/src/
├── lib.rs                    # Public API exports
├── ttl_parser.rs             # TTL vocabulary parsing (types + enums)
├── xmi_class_parser.rs       # XMI class extraction (for type validation)
├── xmi_relationship_parser.rs # XMI relationship constraint extraction
├── json_schema_parser.rs     # JSON schema parsing (enum validation)
├── spec_validation.rs        # Spec coverage validation (type + enum checks)
├── enum_generator.rs         # ElementKind enum generation
├── enum_value_generator.rs   # Value enum generation (FeatureDirectionKind, etc.)
├── hierarchy_generator.rs    # Type hierarchy methods
├── relationship_generator.rs # Relationship constraint methods
├── shapes_parser.rs          # OSLC shapes parsing (properties)
├── inheritance.rs            # Property inheritance resolution
├── accessor_generator.rs     # Property accessor generation
└── validation_generator.rs   # Validation method generation
```

## Generated Code Location

When `sysml-core` builds, the generated files are placed in:

```
target/debug/build/sysml-core-*/out/
├── element_kind.generated.rs   # ElementKind enum + hierarchy + relationships
├── enums.generated.rs          # Value enumeration types
└── properties.generated.rs     # Property accessors + validation
```

## Build Output

A successful build shows validation results:

```
warning: sysml-core@0.1.0: Type coverage: 257 TTL, 175 XMI, 175 matched
warning: sysml-core@0.1.0: Enum coverage: 7 TTL enums, 7 JSON enums
warning: sysml-core@0.1.0: Spec coverage validation PASSED
warning: sysml-core@0.1.0: Relationship constraints: 15/16 from XMI, 1 from fallback
warning: sysml-core@0.1.0: Generated ElementKind enum with 84 KerML and 182 SysML types
warning: sysml-core@0.1.0: Generated 7 value enumeration types
```

If validation fails, the build will error with details:

```
TYPE COVERAGE VALIDATION FAILED: 1 errors
TTL-only types: []
XMI-only types: ["MissingType"]
```

## Usage Examples

### ElementKind - Type Identification

```rust
use sysml_core::ElementKind;

// Parse from string
let kind = "PartUsage".parse::<ElementKind>().unwrap();
assert_eq!(kind, ElementKind::PartUsage);

// Convert to string
assert_eq!(ElementKind::PartUsage.as_str(), "PartUsage");

// Iterate all types
for kind in ElementKind::iter() {
    println!("{}", kind.as_str());
}

// Count of all types
assert_eq!(ElementKind::count(), 266);
```

### Type Hierarchy - Inheritance Queries

```rust
use sysml_core::ElementKind;

// Check supertypes
let supers = ElementKind::PartUsage.supertypes();
assert!(supers.contains(&ElementKind::ItemUsage));
assert!(supers.contains(&ElementKind::Usage));
assert!(supers.contains(&ElementKind::Feature));

// Direct supertypes only
let direct = ElementKind::PartUsage.direct_supertypes();
assert!(direct.contains(&ElementKind::ItemUsage));

// Subtype checking
assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Feature));
assert!(!ElementKind::PartUsage.is_subtype_of(ElementKind::ActionUsage));
```

### Category Predicates

```rust
use sysml_core::ElementKind;

// Definition vs Usage
assert!(ElementKind::PartDefinition.is_definition());
assert!(ElementKind::PartUsage.is_usage());

// Other categories
assert!(ElementKind::Relationship.is_relationship());
assert!(ElementKind::Specialization.is_relationship());
assert!(ElementKind::Feature.is_feature());
assert!(ElementKind::Class.is_classifier());
```

### Definition ↔ Usage Mapping

```rust
use sysml_core::ElementKind;

// Get corresponding usage for a definition
assert_eq!(
    ElementKind::PartDefinition.corresponding_usage(),
    Some(ElementKind::PartUsage)
);

// Get corresponding definition for a usage
assert_eq!(
    ElementKind::PartUsage.corresponding_definition(),
    Some(ElementKind::PartDefinition)
);
```

### Relationship Constraints

```rust
use sysml_core::ElementKind;

// Get expected source/target types for relationships
assert_eq!(
    ElementKind::FeatureTyping.relationship_source_type(),
    Some(ElementKind::Feature)
);
assert_eq!(
    ElementKind::FeatureTyping.relationship_target_type(),
    Some(ElementKind::Type)
);

// Specialization: Type → Type
assert_eq!(
    ElementKind::Specialization.relationship_source_type(),
    Some(ElementKind::Type)
);

// Non-relationships return None
assert_eq!(ElementKind::PartUsage.relationship_source_type(), None);
```

### Value Enumerations

```rust
use sysml_core::{FeatureDirectionKind, VisibilityKind, StateSubactionKind};

// Parse from string
let dir = FeatureDirectionKind::from_str("in").unwrap();
assert_eq!(dir, FeatureDirectionKind::In);

// Convert to string
assert_eq!(FeatureDirectionKind::Out.as_str(), "out");

// Iterate values
for kind in StateSubactionKind::iter() {
    println!("{}", kind.as_str()); // entry, do, exit
}
```

### Property Accessors

```rust
use sysml_core::{Element, ElementKind};

// Create an element
let mut element = Element::new(ElementKind::PartUsage);
element.set_property("declaredName", "engine".into());
element.set_property("isAbstract", false.into());

// Type-safe property access
let props = element.as_part_usage().unwrap();
assert_eq!(props.declared_name(), Some(&"engine".into()));
assert_eq!(props.is_abstract(), Some(&false.into()));
```

## Building

The codegen runs automatically when building `sysml-core`:

```bash
cargo build -p sysml-core
```

To test the codegen crate directly:

```bash
cargo test -p sysml-codegen
```

## Reference Files

Codegen reads spec files from the repo-local `spec/` directory, with a fallback to
`../sysmlv2-references` if you keep the full OMG reference bundle outside the repo.

```
spec/
├── Kerml-Vocab.ttl                              # KerML types + enums
├── SysML-vocab.ttl                              # SysML types + enums
├── KerML-shapes.ttl                             # KerML property shapes
├── SysML-shapes.ttl                             # SysML property shapes
├── KerML/20250201/KerML.xmi                     # KerML metamodel (XMI)
├── SysML/20250201/SysML.xmi                     # SysML metamodel (XMI)
└── metamodel/                                   # JSON schemas
    ├── FeatureDirectionKind.json                # Enum schema
    ├── VisibilityKind.json                      # Enum schema
    └── ...                                      # 7 *Kind.json files total
```
