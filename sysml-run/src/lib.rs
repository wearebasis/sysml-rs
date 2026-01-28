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
use std::collections::HashMap;

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

/// Extended result for parallel state machine execution.
#[derive(Debug, Clone)]
pub struct ParallelStepResult {
    /// Current state of each region (region name -> state name).
    pub region_states: HashMap<String, String>,
    /// Any outputs produced by the step.
    pub outputs: Vec<String>,
    /// Internal events generated during this step.
    pub internal_events: Vec<String>,
    /// Whether execution has completed.
    pub completed: bool,
    /// Timing and other context variables.
    pub context: HashMap<String, f64>,
}

impl ParallelStepResult {
    /// Create a new parallel step result.
    pub fn new() -> Self {
        ParallelStepResult {
            region_states: HashMap::new(),
            outputs: Vec::new(),
            internal_events: Vec::new(),
            completed: false,
            context: HashMap::new(),
        }
    }

    /// Set the state for a region.
    pub fn with_region_state(mut self, region: impl Into<String>, state: impl Into<String>) -> Self {
        self.region_states.insert(region.into(), state.into());
        self
    }

    /// Add an output.
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.outputs.push(output.into());
        self
    }

    /// Add an internal event.
    pub fn with_internal_event(mut self, event: impl Into<String>) -> Self {
        self.internal_events.push(event.into());
        self
    }

    /// Mark as completed.
    pub fn completed(mut self) -> Self {
        self.completed = true;
        self
    }
}

impl Default for ParallelStepResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Assignment operator for structured actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOp {
    /// Direct assignment (=)
    Set,
    /// Addition assignment (+=)
    Add,
    /// Subtraction assignment (-=)
    Subtract,
}

/// A variable assignment in a structured action.
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentIR {
    /// The variable name being assigned.
    pub variable: String,
    /// The assignment operator.
    pub operator: AssignmentOp,
    /// The value being assigned.
    pub value: f64,
}

impl AssignmentIR {
    /// Create a new assignment.
    pub fn new(variable: impl Into<String>, operator: AssignmentOp, value: f64) -> Self {
        AssignmentIR {
            variable: variable.into(),
            operator,
            value,
        }
    }

    /// Create a set assignment (x = value).
    pub fn set(variable: impl Into<String>, value: f64) -> Self {
        Self::new(variable, AssignmentOp::Set, value)
    }

    /// Create an add assignment (x += value).
    pub fn add(variable: impl Into<String>, value: f64) -> Self {
        Self::new(variable, AssignmentOp::Add, value)
    }

    /// Create a subtract assignment (x -= value).
    pub fn subtract(variable: impl Into<String>, value: f64) -> Self {
        Self::new(variable, AssignmentOp::Subtract, value)
    }
}

/// Action IR that can be simple text or structured with assignments and sends.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionIR {
    /// Simple action as a string (backward compatible).
    Simple(String),
    /// Structured action with variable assignments and send events.
    Structured {
        /// Variable assignments (e.g., t += 10).
        assignments: Vec<AssignmentIR>,
        /// Events to send to the event queue.
        sends: Vec<String>,
    },
}

impl ActionIR {
    /// Create a simple action from a string.
    pub fn simple(action: impl Into<String>) -> Self {
        ActionIR::Simple(action.into())
    }

    /// Create a structured action.
    pub fn structured(assignments: Vec<AssignmentIR>, sends: Vec<String>) -> Self {
        ActionIR::Structured { assignments, sends }
    }

    /// Check if this is a simple action.
    pub fn is_simple(&self) -> bool {
        matches!(self, ActionIR::Simple(_))
    }

    /// Get the simple action string if this is a simple action.
    pub fn as_simple(&self) -> Option<&str> {
        match self {
            ActionIR::Simple(s) => Some(s),
            ActionIR::Structured { .. } => None,
        }
    }
}

impl From<String> for ActionIR {
    fn from(s: String) -> Self {
        ActionIR::Simple(s)
    }
}

impl From<&str> for ActionIR {
    fn from(s: &str) -> Self {
        ActionIR::Simple(s.to_string())
    }
}

/// Parallel region within a composite state machine.
#[derive(Debug, Clone)]
pub struct RegionIR {
    /// The region name.
    pub name: String,
    /// All states in this region.
    pub states: Vec<StateIR>,
    /// All transitions in this region.
    pub transitions: Vec<TransitionIR>,
    /// The initial state name for this region.
    pub initial: String,
}

impl RegionIR {
    /// Create a new region.
    pub fn new(name: impl Into<String>, initial: impl Into<String>) -> Self {
        RegionIR {
            name: name.into(),
            states: Vec::new(),
            transitions: Vec::new(),
            initial: initial.into(),
        }
    }

    /// Add a state to this region.
    pub fn with_state(mut self, state: StateIR) -> Self {
        self.states.push(state);
        self
    }

    /// Add a transition to this region.
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
    /// All states in the machine (for simple, non-parallel state machines).
    pub states: Vec<StateIR>,
    /// All transitions in the machine (for simple, non-parallel state machines).
    pub transitions: Vec<TransitionIR>,
    /// The initial state name (for simple, non-parallel state machines).
    pub initial: String,
    /// Parallel regions (for composite state machines with concurrent regions).
    pub regions: Vec<RegionIR>,
}

impl StateMachineIR {
    /// Create a new state machine IR.
    pub fn new(name: impl Into<String>, initial: impl Into<String>) -> Self {
        StateMachineIR {
            name: name.into(),
            states: Vec::new(),
            transitions: Vec::new(),
            initial: initial.into(),
            regions: Vec::new(),
        }
    }

    /// Create a parallel state machine with regions.
    pub fn parallel(name: impl Into<String>) -> Self {
        StateMachineIR {
            name: name.into(),
            states: Vec::new(),
            transitions: Vec::new(),
            initial: String::new(),
            regions: Vec::new(),
        }
    }

    /// Add a region to this state machine.
    pub fn with_region(mut self, region: RegionIR) -> Self {
        self.regions.push(region);
        self
    }

    /// Check if this is a parallel state machine (has regions).
    pub fn is_parallel(&self) -> bool {
        !self.regions.is_empty()
    }

    /// Get a region by name.
    pub fn find_region(&self, name: &str) -> Option<&RegionIR> {
        self.regions.iter().find(|r| r.name == name)
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
    pub entry_action: Option<ActionIR>,
    /// Exit action (optional).
    pub exit_action: Option<ActionIR>,
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

    /// Set entry action (accepts string or ActionIR).
    pub fn with_entry(mut self, action: impl Into<ActionIR>) -> Self {
        self.entry_action = Some(action.into());
        self
    }

    /// Set exit action (accepts string or ActionIR).
    pub fn with_exit(mut self, action: impl Into<ActionIR>) -> Self {
        self.exit_action = Some(action.into());
        self
    }

    /// Set a structured entry action.
    pub fn with_entry_action(mut self, action: ActionIR) -> Self {
        self.entry_action = Some(action);
        self
    }

    /// Set a structured exit action.
    pub fn with_exit_action(mut self, action: ActionIR) -> Self {
        self.exit_action = Some(action);
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
    pub action: Option<ActionIR>,
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

    /// Set the action (accepts string or ActionIR).
    pub fn with_action(mut self, action: impl Into<ActionIR>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Set a structured action.
    pub fn with_action_ir(mut self, action: ActionIR) -> Self {
        self.action = Some(action);
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

        assert_eq!(state.entry_action.as_ref().and_then(|a| a.as_simple()), Some("onEnter()"));
        assert_eq!(state.exit_action.as_ref().and_then(|a| a.as_simple()), Some("onExit()"));
    }

    #[test]
    fn parallel_step_result() {
        let result = ParallelStepResult::new()
            .with_region_state("grid", "energized")
            .with_region_state("relay", "closed")
            .with_output("initialized")
            .with_internal_event("gridReady");

        assert_eq!(result.region_states.get("grid"), Some(&"energized".to_string()));
        assert_eq!(result.region_states.get("relay"), Some(&"closed".to_string()));
        assert_eq!(result.outputs.len(), 1);
        assert_eq!(result.internal_events.len(), 1);
    }

    #[test]
    fn region_ir() {
        let region = RegionIR::new("grid", "energized")
            .with_state(StateIR::new("energized"))
            .with_state(StateIR::new("deEnergized"))
            .with_transition(TransitionIR::new("energized", "deEnergized").with_event("gridFail"));

        assert_eq!(region.name, "grid");
        assert_eq!(region.initial, "energized");
        assert_eq!(region.states.len(), 2);
        assert_eq!(region.transitions.len(), 1);
        assert!(region.find_state("energized").is_some());
        assert_eq!(region.transitions_from("energized").len(), 1);
    }

    #[test]
    fn action_ir_types() {
        let simple = ActionIR::simple("doSomething()");
        assert!(simple.is_simple());
        assert_eq!(simple.as_simple(), Some("doSomething()"));

        let structured = ActionIR::structured(
            vec![AssignmentIR::add("t", 10.0)],
            vec!["eventA".to_string()],
        );
        assert!(!structured.is_simple());
        assert_eq!(structured.as_simple(), None);
    }

    #[test]
    fn assignment_ir() {
        let set = AssignmentIR::set("x", 5.0);
        assert_eq!(set.variable, "x");
        assert_eq!(set.operator, AssignmentOp::Set);
        assert_eq!(set.value, 5.0);

        let add = AssignmentIR::add("t", 10.0);
        assert_eq!(add.operator, AssignmentOp::Add);
    }

    #[test]
    fn parallel_state_machine_ir() {
        let ir = StateMachineIR::parallel("HybridSystem")
            .with_region(RegionIR::new("grid", "energized"))
            .with_region(RegionIR::new("relay", "closed"));

        assert!(ir.is_parallel());
        assert_eq!(ir.regions.len(), 2);
        assert!(ir.find_region("grid").is_some());
        assert!(ir.find_region("unknown").is_none());
    }

    #[test]
    fn constraint_ir() {
        let constraint = ConstraintIR::new("speed < 100")
            .with_description("Speed limit constraint");

        assert_eq!(constraint.expr, "speed < 100");
        assert!(constraint.description.is_some());
    }
}
