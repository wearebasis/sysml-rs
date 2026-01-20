# sysml-run

Execution runtime traits and IR (Intermediate Representation) types for SysML v2.

## Purpose

This crate defines the core abstractions for executing SysML models:

- **Runner**: Trait for stepping through execution
- **CompileToIR**: Trait for compiling ModelGraph to executable IR
- **IR structs**: State machines, constraints, etc.

Actual implementations are in sub-crates:
- `sysml-run-statemachine`: State machine execution
- `sysml-run-constraints`: Constraint evaluation

## Public API

### Runner Trait

```rust
pub trait Runner {
    fn reset(&mut self);
    fn step(&mut self, event: Option<&str>) -> StepResult;
    fn current_state(&self) -> &str;
    fn is_completed(&self) -> bool;
}
```

### CompileToIR Trait

```rust
pub trait CompileToIR<T> {
    fn compile(graph: &ModelGraph) -> Result<T, Vec<Diagnostic>>;
}
```

### StepResult

```rust
let result = StepResult::new("running")
    .with_output("action executed")
    .completed();

result.state;      // "running"
result.outputs;    // ["action executed"]
result.completed;  // true
```

### StateMachineIR

```rust
let ir = StateMachineIR::new("TrafficLight", "red")
    .with_state(StateIR::new("red"))
    .with_state(StateIR::new("yellow"))
    .with_state(StateIR::new("green"))
    .with_transition(TransitionIR::new("red", "green").with_event("timer"))
    .with_transition(TransitionIR::new("green", "yellow").with_event("timer"))
    .with_transition(TransitionIR::new("yellow", "red").with_event("timer"));
```

### TransitionIR

```rust
let transition = TransitionIR::new("idle", "running")
    .with_event("start")
    .with_guard("fuel > 0")
    .with_action("startEngine()");

transition.matches(Some("start"));  // true
```

### ConstraintIR

```rust
let constraint = ConstraintIR::new("speed < maxSpeed")
    .with_description("Speed limit constraint");
```

## Dependencies

- `sysml-core`: ModelGraph and element types
- `sysml-span`: Diagnostic types

## Example

```rust
use sysml_run::{Runner, StepResult, StateMachineIR, StateIR, TransitionIR};

// Create a simple state machine IR
let ir = StateMachineIR::new("Door", "closed")
    .with_state(StateIR::new("closed"))
    .with_state(StateIR::new("open"))
    .with_transition(TransitionIR::new("closed", "open").with_event("open"))
    .with_transition(TransitionIR::new("open", "closed").with_event("close"));

// Find transitions from current state
for t in ir.transitions_from("closed") {
    println!("Can transition to {} via {:?}", t.to, t.event);
}
```
