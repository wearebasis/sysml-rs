# sysml-meta

Metadata types for SysML v2: applicability, clause references, and values.

## Purpose

This crate provides types for representing metadata about model elements:

- **Applicability**: Status of whether an element applies (Applicable, NotApplicable, TBD)
- **ClauseKind**: Purpose of a clause (Operational, Test, Informative)
- **ClauseRef**: Reference to a clause in a standard document
- **Value**: Flexible value type for element properties

## Public API

### Applicability

```rust
pub enum Applicability {
    Applicable,
    NotApplicable,
    TBD,
}

let app = Applicability::Applicable;
app.is_applicable();      // true
app.is_not_applicable();  // false
app.is_tbd();             // false
```

### ClauseRef

```rust
// Without edition
let clause = ClauseRef::new("ISO 26262", "5.4.3");
// With edition
let clause = ClauseRef::with_edition("ISO 26262", "2018", "5.4.3");

println!("{}", clause);  // "ISO 26262 (2018) §5.4.3"
```

### Value

```rust
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Enum(String),
    Ref(ElementId),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Null,
}

// From conversions
let v: Value = true.into();
let v: Value = 42i64.into();
let v: Value = "hello".into();

// Accessors
v.as_bool();   // Option<bool>
v.as_int();    // Option<i64>
v.as_str();    // Option<&str>
v.as_list();   // Option<&Vec<Value>>
v.type_name(); // "bool", "int", etc.
```

## Features

- `serde`: Enable serde serialization support

## Dependencies

- `sysml-id`: For ElementId in Value::Ref
- `serde` (optional): Serialization support

## Example

```rust
use sysml_meta::{Applicability, ClauseRef, Value};
use std::collections::BTreeMap;

// Track requirement applicability
let app = Applicability::Applicable;

// Reference a standard clause
let clause = ClauseRef::with_edition("ISO 26262", "2018", "5.4.3");

// Build property values
let mut props = BTreeMap::new();
props.insert("priority".to_string(), Value::Int(1));
props.insert("verified".to_string(), Value::Bool(false));
props.insert("rationale".to_string(), Value::String("Safety critical".into()));
```
