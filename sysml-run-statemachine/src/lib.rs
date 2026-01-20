//! # sysml-run-statemachine
//!
//! State machine compilation and execution for SysML v2.
//!
//! This crate provides:
//! - Compilation from ModelGraph state machines to StateMachineIR
//! - A simple runner that executes the IR

use sysml_core::{ElementKind, ModelGraph, RelationshipKind};
use sysml_query::find_by_name;
use sysml_run::{CompileToIR, Runner, StateIR, StateMachineIR, StepResult, TransitionIR};
use sysml_span::Diagnostic;

/// Compiler for state machines.
pub struct StateMachineCompiler;

impl CompileToIR<StateMachineIR> for StateMachineCompiler {
    fn compile(graph: &ModelGraph) -> Result<StateMachineIR, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        // Find the first state machine element
        let sm_element = graph
            .elements_by_kind(&ElementKind::StateDefinition)
            .next();

        let sm = match sm_element {
            Some(e) => e,
            None => {
                diagnostics.push(Diagnostic::error("No state machine found in model"));
                return Err(diagnostics);
            }
        };

        let sm_name = sm.name.clone().unwrap_or_else(|| "StateMachine".to_string());

        // Find all states that are children of this state machine
        let states: Vec<_> = graph
            .children_of(&sm.id)
            .filter(|e| matches!(e.kind, ElementKind::StateUsage))
            .collect();

        if states.is_empty() {
            diagnostics.push(Diagnostic::error("State machine has no states"));
            return Err(diagnostics);
        }

        // Find initial state (first state or one marked as initial)
        let initial_state = states
            .iter()
            .find(|s| {
                s.get_prop("initial")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            })
            .or_else(|| states.first())
            .unwrap();

        let initial_name = initial_state
            .name
            .clone()
            .unwrap_or_else(|| "initial".to_string());

        // Build the IR
        let mut ir = StateMachineIR::new(sm_name, initial_name);

        // Add states
        for state in &states {
            let name = state.name.clone().unwrap_or_else(|| state.id.to_string());
            let is_final = state
                .get_prop("final")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let mut state_ir = StateIR::new(&name);

            if let Some(entry) = state.get_prop("entry").and_then(|v| v.as_str()) {
                state_ir = state_ir.with_entry(entry);
            }

            if let Some(exit) = state.get_prop("exit").and_then(|v| v.as_str()) {
                state_ir = state_ir.with_exit(exit);
            }

            if is_final {
                state_ir = state_ir.final_state();
            }

            ir = ir.with_state(state_ir);
        }

        // Find transitions
        for rel in graph.relationships_by_kind(&RelationshipKind::Transition) {
            let source = graph.get_element(&rel.source);
            let target = graph.get_element(&rel.target);

            if let (Some(src), Some(tgt)) = (source, target) {
                let from = src.name.clone().unwrap_or_else(|| src.id.to_string());
                let to = tgt.name.clone().unwrap_or_else(|| tgt.id.to_string());

                let mut transition = TransitionIR::new(from, to);

                if let Some(event) = rel.props.get("event").and_then(|v| v.as_str()) {
                    transition = transition.with_event(event);
                }

                if let Some(guard) = rel.props.get("guard").and_then(|v| v.as_str()) {
                    transition = transition.with_guard(guard);
                }

                if let Some(action) = rel.props.get("action").and_then(|v| v.as_str()) {
                    transition = transition.with_action(action);
                }

                ir = ir.with_transition(transition);
            }
        }

        if !diagnostics.is_empty() {
            Err(diagnostics)
        } else {
            Ok(ir)
        }
    }
}

/// A simple state machine runner.
pub struct StateMachineRunner {
    ir: StateMachineIR,
    current_state: String,
    completed: bool,
}

impl StateMachineRunner {
    /// Create a new runner from IR.
    pub fn new(ir: StateMachineIR) -> Self {
        let initial = ir.initial.clone();
        StateMachineRunner {
            ir,
            current_state: initial,
            completed: false,
        }
    }

    /// Create a runner by compiling a model graph.
    pub fn from_graph(graph: &ModelGraph) -> Result<Self, Vec<Diagnostic>> {
        let ir = StateMachineCompiler::compile(graph)?;
        Ok(Self::new(ir))
    }
}

impl Runner for StateMachineRunner {
    fn reset(&mut self) {
        self.current_state = self.ir.initial.clone();
        self.completed = false;
    }

    fn step(&mut self, event: Option<&str>) -> StepResult {
        if self.completed {
            return StepResult::new(&self.current_state).completed();
        }

        let mut outputs = Vec::new();

        // Find a matching transition
        let transitions = self.ir.transitions_from(&self.current_state);
        let matching = transitions.iter().find(|t| t.matches(event));

        if let Some(transition) = matching {
            // Execute exit action of current state
            if let Some(state) = self.ir.find_state(&self.current_state) {
                if let Some(exit) = &state.exit_action {
                    outputs.push(format!("exit: {}", exit));
                }
            }

            // Execute transition action
            if let Some(action) = &transition.action {
                outputs.push(format!("action: {}", action));
            }

            // Move to new state
            self.current_state = transition.to.clone();

            // Execute entry action of new state
            if let Some(state) = self.ir.find_state(&self.current_state) {
                if let Some(entry) = &state.entry_action {
                    outputs.push(format!("entry: {}", entry));
                }

                if state.is_final {
                    self.completed = true;
                }
            }
        }

        let mut result = StepResult::new(&self.current_state).with_outputs(outputs);

        if self.completed {
            result = result.completed();
        }

        result
    }

    fn current_state(&self) -> &str {
        &self.current_state
    }

    fn is_completed(&self) -> bool {
        self.completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::{Element, Relationship};

    fn create_traffic_light_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();

        // Create state machine
        let sm = Element::new_with_kind(ElementKind::StateDefinition).with_name("TrafficLight");
        let sm_id = graph.add_element(sm);

        // Create states
        let red = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("Red")
            .with_owner(sm_id.clone())
            .with_prop("initial", true);
        let red_id = graph.add_element(red);

        let green = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("Green")
            .with_owner(sm_id.clone());
        let green_id = graph.add_element(green);

        let yellow = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("Yellow")
            .with_owner(sm_id.clone());
        let yellow_id = graph.add_element(yellow);

        // Create transitions
        let t1 = Relationship::new(RelationshipKind::Transition, red_id.clone(), green_id.clone())
            .with_prop("event", "timer");
        graph.add_relationship(t1);

        let t2 = Relationship::new(RelationshipKind::Transition, green_id, yellow_id.clone())
            .with_prop("event", "timer");
        graph.add_relationship(t2);

        let t3 = Relationship::new(RelationshipKind::Transition, yellow_id, red_id)
            .with_prop("event", "timer");
        graph.add_relationship(t3);

        graph
    }

    #[test]
    fn compile_state_machine() {
        let graph = create_traffic_light_graph();
        let ir = StateMachineCompiler::compile(&graph).unwrap();

        assert_eq!(ir.name, "TrafficLight");
        assert_eq!(ir.states.len(), 3);
        assert_eq!(ir.transitions.len(), 3);
        assert_eq!(ir.initial, "Red");
    }

    #[test]
    fn runner_initial_state() {
        let graph = create_traffic_light_graph();
        let runner = StateMachineRunner::from_graph(&graph).unwrap();

        assert_eq!(runner.current_state(), "Red");
        assert!(!runner.is_completed());
    }

    #[test]
    fn runner_step() {
        let graph = create_traffic_light_graph();
        let mut runner = StateMachineRunner::from_graph(&graph).unwrap();

        // Step with timer event
        let result = runner.step(Some("timer"));
        assert_eq!(result.state, "Green");

        // Another step
        let result = runner.step(Some("timer"));
        assert_eq!(result.state, "Yellow");

        // Back to red
        let result = runner.step(Some("timer"));
        assert_eq!(result.state, "Red");
    }

    #[test]
    fn runner_no_matching_event() {
        let graph = create_traffic_light_graph();
        let mut runner = StateMachineRunner::from_graph(&graph).unwrap();

        // Step with non-matching event
        let result = runner.step(Some("unknown"));
        assert_eq!(result.state, "Red"); // Should stay in Red
    }

    #[test]
    fn runner_reset() {
        let graph = create_traffic_light_graph();
        let mut runner = StateMachineRunner::from_graph(&graph).unwrap();

        runner.step(Some("timer"));
        assert_eq!(runner.current_state(), "Green");

        runner.reset();
        assert_eq!(runner.current_state(), "Red");
    }

    #[test]
    fn compile_no_state_machine() {
        let graph = ModelGraph::new();
        let result = StateMachineCompiler::compile(&graph);

        assert!(result.is_err());
        let diags = result.unwrap_err();
        assert!(diags[0].message.contains("No state machine"));
    }
}
