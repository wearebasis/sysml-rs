# sysml-run-constraints

Constraint compilation and evaluation for SysML v2.

## Purpose

This crate provides:
- Extraction of constraints from ModelGraph
- A simple constraint evaluator

**Note**: The current evaluator is a stub that handles only simple comparisons. A full implementation would need an expression parser and evaluator.

## Public API

### ConstraintSet

```rust
use sysml_run_constraints::ConstraintSet;
use sysml_run::ConstraintIR;

let mut set = ConstraintSet::new();
set.add(ConstraintIR::new("speed < 100"));
set.add(ConstraintIR::new("fuel > 0"));
```

### Extract from ModelGraph

```rust
use sysml_run_constraints::extract_constraints;

let constraints = extract_constraints(&graph);
```

### Evaluation

```rust
use sysml_run_constraints::{evaluate, evaluate_all, EvaluationContext};

let mut context = EvaluationContext::new();
context.set("speed", 50.0);
context.set("fuel", 75.0);

// Evaluate single constraint
let result = evaluate(&constraint, &context);
println!("Satisfied: {}", result.satisfied);

// Evaluate all
let results = evaluate_all(&constraints, &context);
```

### Check Results

```rust
use sysml_run_constraints::{all_satisfied, failed_constraints};

if all_satisfied(&results) {
    println!("All constraints satisfied");
} else {
    for failed in failed_constraints(&results) {
        println!("Failed: {}", failed.constraint.expr);
    }
}
```

## Supported Expressions (Stub)

The current stub evaluator supports:
- `var < num`
- `var > num`
- `var == num`
- `var <= num`
- `var >= num`

## Dependencies

- `sysml-run`: ConstraintIR type
- `sysml-core`: ModelGraph and Value types
- `sysml-query`: Query functions
- `sysml-span`: Diagnostic types

## Example

```rust
use sysml_core::{ModelGraph, Element, ElementKind};
use sysml_run_constraints::{extract_constraints, evaluate_all, EvaluationContext, all_satisfied};

// Build model with constraints
let mut graph = ModelGraph::new();

let speed_limit = Element::new_with_kind(ElementKind::Attribute)
    .with_name("MaxSpeed")
    .with_prop("constraint", "speed < 120");
graph.add_element(speed_limit);

let fuel_check = Element::new_with_kind(ElementKind::Attribute)
    .with_name("FuelRequired")
    .with_prop("constraint", "fuel > 10");
graph.add_element(fuel_check);

// Extract constraints
let constraints = extract_constraints(&graph);

// Set up evaluation context
let mut context = EvaluationContext::new();
context.set("speed", 100.0);
context.set("fuel", 50.0);

// Evaluate
let results = evaluate_all(&constraints, &context);
assert!(all_satisfied(&results));
```
