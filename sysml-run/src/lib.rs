//! # sysml-run
//!
//! Execution runtime traits and IR (Intermediate Representation) types for SysML v2.
//!
//! This crate defines the core abstractions for executing SysML models:
//! - Runner trait for stepping through execution
//! - CompileToIR trait for compiling ModelGraph to executable IR
//! - IR structs for state machines, constraints, etc.
//!
//! Actual implementations are in sub-crates (sysml-run-statemachine, etc.).

use sysml_core::ModelGraph;
use sysml_span::Diagnostic;

/// The result of a single execution step.
#[derive(Debug, Clone)]
pub struct StepResult {
    /// The current state after the step.
    pub state: String,
    /// Any outputs produced by the step.
    pub outputs: Vec<String>,
    /// Whether execution has completed.
    pub completed: bool,
}

impl StepResult {
    /// Create a new step result.
    pub fn new(state: impl Into<String>) -> Self {
        StepResult {
            state: state.into(),
            outputs: Vec::new(),
            completed: false,
        }
    }

    /// Mark this result as completed.
    pub fn completed(mut self) -> Self {
        self.completed = true;
        self
    }

    /// Add an output.
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.outputs.push(output.into());
        self
    }

    /// Add multiple outputs.
    pub fn with_outputs(mut self, outputs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.outputs.extend(outputs.into_iter().map(|o| o.into()));
        self
    }
}

/// Trait for executable runners.
///
/// A runner maintains state and can be stepped through execution
/// by providing optional events.
pub trait Runner {
    /// Reset the runner to its initial state.
    fn reset(&mut self);

    /// Execute a single step, optionally triggered by an event.
    ///
    /// # Arguments
    ///
    /// * `event` - An optional event name that triggers the step
    ///
    /// # Returns
    ///
    /// The result of the step including the new state and any outputs.
    fn step(&mut self, event: Option<&str>) -> StepResult;

    /// Get the current state name.
    fn current_state(&self) -> &str;

    /// Check if execution is complete.
    fn is_completed(&self) -> bool;
}

/// Trait for compiling a ModelGraph to an IR type.
pub trait CompileToIR<T> {
    /// Compile a model graph to the target IR.
    ///
    /// # Arguments
    ///
    /// * `graph` - The model graph to compile
    ///
    /// # Returns
    ///
    /// The compiled IR on success, or diagnostics on failure.
    fn compile(graph: &ModelGraph) -> Result<T, Vec<Diagnostic>>;
}

/// IR for a state machine.
#[derive(Debug, Clone)]
pub struct StateMachineIR {
    /// The name of this state machine.
    pub name: String,
    /// All states in the machine.
    pub states: Vec<StateIR>,
    /// All transitions in the machine.
    pub transitions: Vec<TransitionIR>,
    /// The initial state name.
    pub initial: String,
}

impl StateMachineIR {
    /// Create a new state machine IR.
    pub fn new(name: impl Into<String>, initial: impl Into<String>) -> Self {
        StateMachineIR {
            name: name.into(),
            states: Vec::new(),
            transitions: Vec::new(),
            initial: initial.into(),
        }
    }

    /// Add a state.
    pub fn with_state(mut self, state: StateIR) -> Self {
        self.states.push(state);
        self
    }

    /// Add a transition.
    pub fn with_transition(mut self, transition: TransitionIR) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Find a state by name.
    pub fn find_state(&self, name: &str) -> Option<&StateIR> {
        self.states.iter().find(|s| s.name == name)
    }

    /// Get all transitions from a given state.
    pub fn transitions_from(&self, state: &str) -> Vec<&TransitionIR> {
        self.transitions.iter().filter(|t| t.from == state).collect()
    }
}

/// IR for a state within a state machine.
#[derive(Debug, Clone)]
pub struct StateIR {
    /// The state name.
    pub name: String,
    /// Entry action (optional).
    pub entry_action: Option<String>,
    /// Exit action (optional).
    pub exit_action: Option<String>,
    /// Whether this is a final state.
    pub is_final: bool,
}

impl StateIR {
    /// Create a new state IR.
    pub fn new(name: impl Into<String>) -> Self {
        StateIR {
            name: name.into(),
            entry_action: None,
            exit_action: None,
            is_final: false,
        }
    }

    /// Set entry action.
    pub fn with_entry(mut self, action: impl Into<String>) -> Self {
        self.entry_action = Some(action.into());
        self
    }

    /// Set exit action.
    pub fn with_exit(mut self, action: impl Into<String>) -> Self {
        self.exit_action = Some(action.into());
        self
    }

    /// Mark as final state.
    pub fn final_state(mut self) -> Self {
        self.is_final = true;
        self
    }
}

/// IR for a transition between states.
#[derive(Debug, Clone)]
pub struct TransitionIR {
    /// The source state name.
    pub from: String,
    /// The target state name.
    pub to: String,
    /// The triggering event (optional).
    pub event: Option<String>,
    /// The guard condition (optional, as string expression).
    pub guard: Option<String>,
    /// The action to execute (optional).
    pub action: Option<String>,
}

impl TransitionIR {
    /// Create a new transition IR.
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        TransitionIR {
            from: from.into(),
            to: to.into(),
            event: None,
            guard: None,
            action: None,
        }
    }

    /// Set the triggering event.
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// Set the guard condition.
    pub fn with_guard(mut self, guard: impl Into<String>) -> Self {
        self.guard = Some(guard.into());
        self
    }

    /// Set the action.
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Check if this transition matches an event.
    pub fn matches(&self, event: Option<&str>) -> bool {
        match (&self.event, event) {
            (None, _) => true, // Auto-transition
            (Some(e), Some(ev)) => e == ev,
            (Some(_), None) => false,
        }
    }
}

/// IR for a constraint.
#[derive(Debug, Clone)]
pub struct ConstraintIR {
    /// The constraint expression as a string.
    pub expr: String,
    /// Human-readable description.
    pub description: Option<String>,
}

impl ConstraintIR {
    /// Create a new constraint IR.
    pub fn new(expr: impl Into<String>) -> Self {
        ConstraintIR {
            expr: expr.into(),
            description: None,
        }
    }

    /// Set the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_result_creation() {
        let result = StepResult::new("initial")
            .with_output("started")
            .with_output("ready");

        assert_eq!(result.state, "initial");
        assert_eq!(result.outputs.len(), 2);
        assert!(!result.completed);
    }

    #[test]
    fn step_result_completed() {
        let result = StepResult::new("final").completed();
        assert!(result.completed);
    }

    #[test]
    fn state_machine_ir_creation() {
        let ir = StateMachineIR::new("TestMachine", "initial")
            .with_state(StateIR::new("initial"))
            .with_state(StateIR::new("running"))
            .with_state(StateIR::new("final").final_state())
            .with_transition(TransitionIR::new("initial", "running").with_event("start"))
            .with_transition(TransitionIR::new("running", "final").with_event("stop"));

        assert_eq!(ir.name, "TestMachine");
        assert_eq!(ir.states.len(), 3);
        assert_eq!(ir.transitions.len(), 2);
        assert_eq!(ir.initial, "initial");
    }

    #[test]
    fn find_state() {
        let ir = StateMachineIR::new("Test", "s1")
            .with_state(StateIR::new("s1"))
            .with_state(StateIR::new("s2"));

        assert!(ir.find_state("s1").is_some());
        assert!(ir.find_state("s3").is_none());
    }

    #[test]
    fn transitions_from() {
        let ir = StateMachineIR::new("Test", "s1")
            .with_transition(TransitionIR::new("s1", "s2").with_event("e1"))
            .with_transition(TransitionIR::new("s1", "s3").with_event("e2"))
            .with_transition(TransitionIR::new("s2", "s3").with_event("e3"));

        let from_s1 = ir.transitions_from("s1");
        assert_eq!(from_s1.len(), 2);
    }

    #[test]
    fn transition_matching() {
        let t1 = TransitionIR::new("s1", "s2").with_event("click");
        let t2 = TransitionIR::new("s1", "s2"); // Auto-transition

        assert!(t1.matches(Some("click")));
        assert!(!t1.matches(Some("hover")));
        assert!(!t1.matches(None));

        assert!(t2.matches(Some("anything")));
        assert!(t2.matches(None));
    }

    #[test]
    fn state_with_actions() {
        let state = StateIR::new("running")
            .with_entry("onEnter()")
            .with_exit("onExit()");

        assert_eq!(state.entry_action, Some("onEnter()".to_string()));
        assert_eq!(state.exit_action, Some("onExit()".to_string()));
    }

    #[test]
    fn constraint_ir() {
        let constraint = ConstraintIR::new("speed < 100")
            .with_description("Speed limit constraint");

        assert_eq!(constraint.expr, "speed < 100");
        assert!(constraint.description.is_some());
    }
}
