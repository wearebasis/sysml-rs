# sysml-run-statemachine

State machine compilation and execution for SysML v2.

## Purpose

This crate provides:
- Compilation from ModelGraph state machines to StateMachineIR
- A simple runner that executes the IR

## Public API

### Compilation

```rust
use sysml_run_statemachine::StateMachineCompiler;
use sysml_run::CompileToIR;

let ir = StateMachineCompiler::compile(&graph)?;
```

### Running

```rust
use sysml_run_statemachine::StateMachineRunner;
use sysml_run::Runner;

let mut runner = StateMachineRunner::from_graph(&graph)?;

// Or from pre-compiled IR
let mut runner = StateMachineRunner::new(ir);

// Execute steps
runner.step(Some("startEvent"));
runner.step(Some("nextEvent"));

// Check state
println!("Current: {}", runner.current_state());
println!("Completed: {}", runner.is_completed());

// Reset to initial
runner.reset();
```

## Model Requirements

The compiler expects:
- At least one `StateMachine` element
- `State` elements as children of the state machine
- `Transition` relationships between states
- Optional `initial: true` property on initial state
- Optional `final: true` property on final states
- Optional `event`, `guard`, `action` properties on transitions

## Dependencies

- `sysml-run`: Runner trait and IR types
- `sysml-core`: ModelGraph
- `sysml-query`: Query functions
- `sysml-span`: Diagnostics

## Example

```rust
use sysml_core::{ModelGraph, Element, ElementKind, Relationship, RelationshipKind};
use sysml_run_statemachine::StateMachineRunner;
use sysml_run::Runner;

// Build a state machine model
let mut graph = ModelGraph::new();

let sm = Element::new_with_kind(ElementKind::StateMachine)
    .with_name("DoorController");
let sm_id = graph.add_element(sm);

let closed = Element::new_with_kind(ElementKind::State)
    .with_name("Closed")
    .with_owner(sm_id.clone())
    .with_prop("initial", true);
let closed_id = graph.add_element(closed);

let open = Element::new_with_kind(ElementKind::State)
    .with_name("Open")
    .with_owner(sm_id.clone());
let open_id = graph.add_element(open);

let t1 = Relationship::new(RelationshipKind::Transition, closed_id.clone(), open_id.clone())
    .with_prop("event", "open");
graph.add_relationship(t1);

let t2 = Relationship::new(RelationshipKind::Transition, open_id, closed_id)
    .with_prop("event", "close");
graph.add_relationship(t2);

// Run the state machine
let mut runner = StateMachineRunner::from_graph(&graph).unwrap();
assert_eq!(runner.current_state(), "Closed");

runner.step(Some("open"));
assert_eq!(runner.current_state(), "Open");

runner.step(Some("close"));
assert_eq!(runner.current_state(), "Closed");
```
