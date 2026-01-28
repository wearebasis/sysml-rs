//! # sysml-run-statemachine
//!
//! State machine compilation and execution for SysML v2.
//!
//! This crate provides:
//! - Compilation from ModelGraph state machines to StateMachineIR
//! - A simple runner that executes the IR
//! - Parallel state machine runner for composite state machines with concurrent regions

pub mod action_parser;
pub mod parallel;

pub use action_parser::parse_action;
pub use parallel::ParallelStateMachineRunner;

use sysml_core::{Element, ElementId, ElementKind, ModelGraph, RelationshipKind};
use sysml_run::{ActionIR, CompileToIR, RegionIR, Runner, StateIR, StateMachineIR, StepResult, TransitionIR};
use sysml_span::Diagnostic;
use std::collections::HashSet;

/// Compiler for state machines.
pub struct StateMachineCompiler;

impl StateMachineCompiler {
    /// Compile a simple (non-parallel) state machine.
    fn compile_simple(
        graph: &ModelGraph,
        sm: &Element,
        sm_name: String,
    ) -> Result<StateMachineIR, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

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
            let state_ir = Self::compile_state(state);
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
                    transition = transition.with_action(parse_action(action));
                }

                ir = ir.with_transition(transition);
            }
        }

        Ok(ir)
    }

    /// Compile a parallel state machine with multiple concurrent regions.
    fn compile_parallel(
        graph: &ModelGraph,
        _sm: &Element,
        sm_name: String,
        region_elements: Vec<&Element>,
    ) -> Result<StateMachineIR, Vec<Diagnostic>> {
        let mut ir = StateMachineIR::parallel(sm_name);

        for region_elem in region_elements {
            let region_name = region_elem
                .name
                .clone()
                .unwrap_or_else(|| region_elem.id.to_string());

            // Find states within this region
            let states: Vec<_> = graph
                .children_of(&region_elem.id)
                .filter(|e| matches!(e.kind, ElementKind::StateUsage))
                .collect();

            if states.is_empty() {
                continue; // Skip regions with no states
            }

            // Find initial state for this region
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

            let mut region = RegionIR::new(&region_name, initial_name);

            // Add states to region
            for state in &states {
                let state_ir = Self::compile_state(state);
                region = region.with_state(state_ir);
            }

            // Find transitions within this region
            // Collect state IDs for this region
            let region_state_ids: std::collections::HashSet<_> =
                states.iter().map(|s| &s.id).collect();

            for rel in graph.relationships_by_kind(&RelationshipKind::Transition) {
                // Only include transitions where source is in this region
                if !region_state_ids.contains(&rel.source) {
                    continue;
                }

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
                        transition = transition.with_action(parse_action(action));
                    }

                    region = region.with_transition(transition);
                }
            }

            ir = ir.with_region(region);
        }

        Ok(ir)
    }

    /// Compile a single state element into StateIR.
    fn compile_state(state: &Element) -> StateIR {
        let name = state.name.clone().unwrap_or_else(|| state.id.to_string());
        let is_final = state
            .get_prop("final")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut state_ir = StateIR::new(&name);

        if let Some(entry) = state.get_prop("entry").and_then(|v| v.as_str()) {
            state_ir = state_ir.with_entry_action(parse_action(entry));
        }

        if let Some(exit) = state.get_prop("exit").and_then(|v| v.as_str()) {
            state_ir = state_ir.with_exit_action(parse_action(exit));
        }

        if is_final {
            state_ir = state_ir.final_state();
        }

        state_ir
    }

    /// Check if a state machine should be compiled as parallel.
    /// Returns the region elements if parallel, None otherwise.
    fn detect_parallel_regions<'a>(
        graph: &'a ModelGraph,
        sm: &'a Element,
    ) -> Option<Vec<&'a Element>> {
        // Check isParallel property
        let is_parallel = sm
            .get_prop("isParallel")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_parallel {
            // Top-level StateUsage children are regions
            let regions: Vec<_> = graph
                .children_of(&sm.id)
                .filter(|e| matches!(e.kind, ElementKind::StateUsage))
                .collect();
            if !regions.is_empty() {
                return Some(regions);
            }
        }

        // Check for multiple top-level states that each have their own substates
        // This indicates a parallel structure
        let top_level_states: Vec<_> = graph
            .children_of(&sm.id)
            .filter(|e| matches!(e.kind, ElementKind::StateUsage))
            .collect();

        // If multiple top-level states each have child states, treat as parallel
        let regions_with_substates: Vec<_> = top_level_states
            .iter()
            .filter(|s| {
                graph
                    .children_of(&s.id)
                    .any(|c| matches!(c.kind, ElementKind::StateUsage))
            })
            .copied()
            .collect();

        if regions_with_substates.len() > 1 {
            return Some(regions_with_substates);
        }

        None
    }

    /// Compile a composite state machine from a part that contains
    /// sub-parts exhibiting state machines.
    ///
    /// This traverses the part hierarchy to find all `exhibit state` declarations,
    /// extracts the state definitions they reference, and builds a parallel
    /// state machine with one region per exhibited state machine.
    ///
    /// # Arguments
    ///
    /// * `graph` - The model graph containing the parsed SysML model
    /// * `part_id` - The ID of the root part (e.g., the installation)
    ///
    /// # Returns
    ///
    /// A `StateMachineIR` with one region per exhibited state machine, or
    /// diagnostics if compilation fails.
    pub fn compile_from_part(
        graph: &ModelGraph,
        part_id: &ElementId,
    ) -> Result<StateMachineIR, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        let part = match graph.get_element(part_id) {
            Some(e) => e,
            None => {
                diagnostics.push(Diagnostic::error("Part not found"));
                return Err(diagnostics);
            }
        };

        let part_name = part.name.clone().unwrap_or_else(|| "CompositeStateMachine".to_string());

        // Find all descendants with exhibit state declarations
        let mut exhibit_states = Vec::new();
        let mut visited = HashSet::new();
        Self::collect_exhibit_states(graph, part_id, "", &mut exhibit_states, &mut visited);

        if exhibit_states.is_empty() {
            diagnostics.push(Diagnostic::error("No exhibit state declarations found in part hierarchy"));
            return Err(diagnostics);
        }

        // Build parallel state machine
        let mut ir = StateMachineIR::parallel(part_name);

        for (region_name, exhibit_id) in exhibit_states {
            // Find the type of this exhibit state (the state definition it references)
            if let Some(state_def_id) = Self::find_exhibit_state_type(graph, &exhibit_id) {
                if let Some(region) = Self::state_def_to_region(graph, &state_def_id, &region_name) {
                    ir = ir.with_region(region);
                }
            }
        }

        if ir.regions.is_empty() {
            diagnostics.push(Diagnostic::error("No valid state machines found"));
            return Err(diagnostics);
        }

        Ok(ir)
    }

    /// Recursively collect all exhibit state usages in the part tree.
    ///
    /// Builds a list of (region_name, exhibit_id) pairs, where region_name
    /// is a simplified name based on the containing part.
    fn collect_exhibit_states(
        graph: &ModelGraph,
        element_id: &ElementId,
        path_prefix: &str,
        result: &mut Vec<(String, ElementId)>,
        visited: &mut HashSet<ElementId>,
    ) {
        if !visited.insert(element_id.clone()) {
            return;
        }

        // Get the element's name for path building
        let element_name = graph
            .get_element(element_id)
            .and_then(|e| e.name.clone())
            .unwrap_or_default();

        // Check all children
        for child in graph.children_of(element_id) {
            match child.kind {
                ElementKind::ExhibitStateUsage => {
                    // Found an exhibit state - use the element name or the exhibit's name
                    let region_name = if let Some(name) = &child.name {
                        // If exhibit has a name, use it
                        name.clone()
                    } else if !element_name.is_empty() {
                        // Use the containing part's name
                        element_name.clone()
                    } else {
                        // Fallback to path-based name
                        format!("{}region_{}", path_prefix, result.len())
                    };
                    result.push((region_name, child.id.clone()));
                }
                ElementKind::PartUsage | ElementKind::PartDefinition => {
                    // Recurse into parts
                    let new_prefix = if path_prefix.is_empty() {
                        element_name.clone()
                    } else if !element_name.is_empty() {
                        format!("{}.{}", path_prefix, element_name)
                    } else {
                        path_prefix.to_string()
                    };
                    Self::collect_exhibit_states(graph, &child.id, &new_prefix, result, visited);
                }
                _ => {
                    // Recurse into other structural elements
                    Self::collect_exhibit_states(graph, &child.id, path_prefix, result, visited);
                }
            }
        }
    }

    /// Find the state definition type for an exhibit state usage.
    ///
    /// Looks for FeatureTyping children with a resolved `type` property
    /// or an `unresolved_type` property that we can try to resolve.
    fn find_exhibit_state_type(graph: &ModelGraph, exhibit_id: &ElementId) -> Option<ElementId> {
        // Look for FeatureTyping children
        for child in graph.children_of(exhibit_id) {
            if child.kind == ElementKind::FeatureTyping
                || child.kind.is_subtype_of(ElementKind::FeatureTyping)
            {
                // Check for resolved type
                if let Some(type_ref) = child.props.get("type") {
                    if let Some(type_id) = type_ref.as_ref() {
                        return Some(type_id.clone());
                    }
                }

                // Check for unresolved type and try to find it
                if let Some(unresolved) = child.props.get("unresolved_type") {
                    if let Some(type_name) = unresolved.as_str() {
                        // Try to find the state definition by name
                        if let Some(state_def) = Self::find_state_definition_by_name(graph, type_name) {
                            return Some(state_def);
                        }
                    }
                }
            }
        }

        None
    }

    /// Find a state definition by name (potentially qualified).
    fn find_state_definition_by_name(graph: &ModelGraph, name: &str) -> Option<ElementId> {
        // Extract the simple name (last part of qualified name)
        let simple_name = name.rsplit("::").next().unwrap_or(name);

        // Search for StateDefinition elements with this name
        for element in graph.elements_by_kind(&ElementKind::StateDefinition) {
            if let Some(elem_name) = &element.name {
                if elem_name == simple_name || elem_name == name {
                    return Some(element.id.clone());
                }
            }
        }

        None
    }

    /// Convert a state definition to a RegionIR.
    fn state_def_to_region(
        graph: &ModelGraph,
        state_def_id: &ElementId,
        region_name: &str,
    ) -> Option<RegionIR> {
        let _state_def = graph.get_element(state_def_id)?;

        // Find all states within this state definition
        let states: Vec<_> = graph
            .children_of(state_def_id)
            .filter(|e| matches!(e.kind, ElementKind::StateUsage))
            .collect();

        if states.is_empty() {
            return None;
        }

        // Find initial state (first state or one marked initial)
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

        let mut region = RegionIR::new(region_name, &initial_name);

        // Add states
        for state in &states {
            let state_ir = Self::compile_state(state);
            region = region.with_state(state_ir);
        }

        // Build a map of state names to IDs for transition lookup
        let state_ids: HashSet<_> = states.iter().map(|s| s.id.clone()).collect();

        // Find transitions within this state definition
        // Look for TransitionUsage elements owned by the state definition
        for child in graph.children_of(state_def_id) {
            if child.kind == ElementKind::TransitionUsage {
                if let Some(transition) = Self::compile_transition_usage(graph, &child, &state_ids) {
                    region = region.with_transition(transition);
                }
            }
        }

        // Also check for transitions as relationships
        for rel in graph.relationships_by_kind(&RelationshipKind::Transition) {
            if state_ids.contains(&rel.source) {
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
                        transition = transition.with_action(parse_action(action));
                    }

                    region = region.with_transition(transition);
                }
            }
        }

        Some(region)
    }

    /// Compile a TransitionUsage element to TransitionIR.
    fn compile_transition_usage(
        graph: &ModelGraph,
        transition: &Element,
        _state_ids: &HashSet<ElementId>,
    ) -> Option<TransitionIR> {
        // Get source and target from properties
        let source_name = transition.props.get("source").and_then(|v| {
            // Could be a Ref or a String
            v.as_ref()
                .and_then(|id| graph.get_element(id))
                .and_then(|e| e.name.clone())
                .or_else(|| v.as_str().map(String::from))
        });

        let target_name = transition.props.get("target").and_then(|v| {
            v.as_ref()
                .and_then(|id| graph.get_element(id))
                .and_then(|e| e.name.clone())
                .or_else(|| v.as_str().map(String::from))
        });

        // Try unresolved properties
        let source_name = source_name.or_else(|| {
            transition.props.get("unresolved_source").and_then(|v| v.as_str().map(String::from))
        });

        let target_name = target_name.or_else(|| {
            transition.props.get("unresolved_target").and_then(|v| v.as_str().map(String::from))
        });

        let (from, to) = match (source_name, target_name) {
            (Some(f), Some(t)) => (f, t),
            _ => {
                // Can't determine source/target without explicit properties
                // Future: could try parsing from transition name patterns
                return None;
            }
        };

        let mut ir = TransitionIR::new(from, to);

        // Extract event from trigger
        if let Some(trigger) = transition.props.get("trigger").and_then(|v| v.as_str()) {
            ir = ir.with_event(trigger);
        }

        // Extract guard
        if let Some(guard) = transition.props.get("guard").and_then(|v| v.as_str()) {
            ir = ir.with_guard(guard);
        }

        // Extract action
        if let Some(action) = transition.props.get("effect").and_then(|v| v.as_str()) {
            ir = ir.with_action(parse_action(action));
        }

        Some(ir)
    }
}

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

        // Check if this should be compiled as a parallel state machine
        if let Some(regions) = Self::detect_parallel_regions(graph, sm) {
            Self::compile_parallel(graph, sm, sm_name, regions)
        } else {
            Self::compile_simple(graph, sm, sm_name)
        }
    }
}

/// Format an action for output.
fn format_action(action: &ActionIR) -> String {
    match action {
        ActionIR::Simple(s) => s.clone(),
        ActionIR::Structured { assignments, sends } => {
            let mut parts = Vec::new();
            for assign in assignments {
                let op = match assign.operator {
                    sysml_run::AssignmentOp::Set => "=",
                    sysml_run::AssignmentOp::Add => "+=",
                    sysml_run::AssignmentOp::Subtract => "-=",
                };
                parts.push(format!("{} {} {}", assign.variable, op, assign.value));
            }
            for send in sends {
                parts.push(format!("send('{}')", send));
            }
            parts.join("; ")
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
                    outputs.push(format!("exit: {}", format_action(exit)));
                }
            }

            // Execute transition action
            if let Some(action) = &transition.action {
                outputs.push(format!("action: {}", format_action(action)));
            }

            // Move to new state
            self.current_state = transition.to.clone();

            // Execute entry action of new state
            if let Some(state) = self.ir.find_state(&self.current_state) {
                if let Some(entry) = &state.entry_action {
                    outputs.push(format!("entry: {}", format_action(entry)));
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

    fn create_parallel_state_machine_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();

        // Create parallel state machine
        let sm = Element::new_with_kind(ElementKind::StateDefinition)
            .with_name("HybridSystem")
            .with_prop("isParallel", true);
        let sm_id = graph.add_element(sm);

        // Region 1: Grid
        let grid = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("grid")
            .with_owner(sm_id.clone());
        let grid_id = graph.add_element(grid);

        let grid_energized = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("energized")
            .with_owner(grid_id.clone())
            .with_prop("initial", true);
        let grid_energized_id = graph.add_element(grid_energized);

        let grid_deenergized = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("deEnergized")
            .with_owner(grid_id.clone());
        let grid_deenergized_id = graph.add_element(grid_deenergized);

        // Grid transitions
        let t1 = Relationship::new(
            RelationshipKind::Transition,
            grid_energized_id.clone(),
            grid_deenergized_id.clone(),
        )
        .with_prop("event", "gridFail");
        graph.add_relationship(t1);

        let t2 = Relationship::new(
            RelationshipKind::Transition,
            grid_deenergized_id,
            grid_energized_id,
        )
        .with_prop("event", "gridRestore");
        graph.add_relationship(t2);

        // Region 2: Relay
        let relay = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("relay")
            .with_owner(sm_id.clone());
        let relay_id = graph.add_element(relay);

        let relay_closed = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("closed")
            .with_owner(relay_id.clone())
            .with_prop("initial", true);
        let relay_closed_id = graph.add_element(relay_closed);

        let relay_open = Element::new_with_kind(ElementKind::StateUsage)
            .with_name("open")
            .with_owner(relay_id.clone());
        let relay_open_id = graph.add_element(relay_open);

        // Relay transitions
        let t3 = Relationship::new(
            RelationshipKind::Transition,
            relay_closed_id.clone(),
            relay_open_id.clone(),
        )
        .with_prop("event", "gridFail")
        .with_prop("action", "t += 20");
        graph.add_relationship(t3);

        let t4 = Relationship::new(
            RelationshipKind::Transition,
            relay_open_id,
            relay_closed_id,
        )
        .with_prop("event", "gridRestore");
        graph.add_relationship(t4);

        graph
    }

    #[test]
    fn compile_parallel_state_machine() {
        let graph = create_parallel_state_machine_graph();
        let ir = StateMachineCompiler::compile(&graph).unwrap();

        assert_eq!(ir.name, "HybridSystem");
        assert!(ir.is_parallel());
        assert_eq!(ir.regions.len(), 2);

        // Check grid region
        let grid_region = ir.find_region("grid").unwrap();
        assert_eq!(grid_region.initial, "energized");
        assert_eq!(grid_region.states.len(), 2);
        assert!(grid_region.find_state("energized").is_some());
        assert!(grid_region.find_state("deEnergized").is_some());

        // Check relay region
        let relay_region = ir.find_region("relay").unwrap();
        assert_eq!(relay_region.initial, "closed");
        assert_eq!(relay_region.states.len(), 2);
    }

    #[test]
    fn parallel_runner_from_compiled_graph() {
        let graph = create_parallel_state_machine_graph();
        let ir = StateMachineCompiler::compile(&graph).unwrap();
        let mut runner = ParallelStateMachineRunner::new(ir);

        // Initial states
        assert_eq!(runner.region_state("grid"), Some("energized"));
        assert_eq!(runner.region_state("relay"), Some("closed"));

        // Send gridFail event
        runner.send("gridFail");

        // Both regions should transition
        assert_eq!(runner.region_state("grid"), Some("deEnergized"));
        assert_eq!(runner.region_state("relay"), Some("open"));

        // Check timing context was updated
        assert_eq!(runner.get_context("t"), Some(20.0));
    }

    #[test]
    fn parallel_runner_restore() {
        let graph = create_parallel_state_machine_graph();
        let ir = StateMachineCompiler::compile(&graph).unwrap();
        let mut runner = ParallelStateMachineRunner::new(ir);

        // Fail then restore
        runner.send("gridFail");
        runner.send("gridRestore");

        assert_eq!(runner.region_state("grid"), Some("energized"));
        assert_eq!(runner.region_state("relay"), Some("closed"));
    }

    #[test]
    fn action_parsing_in_compiled_transitions() {
        let graph = create_parallel_state_machine_graph();
        let ir = StateMachineCompiler::compile(&graph).unwrap();

        // Find relay region and check transition action
        let relay_region = ir.find_region("relay").unwrap();
        let transitions = relay_region.transitions_from("closed");
        assert_eq!(transitions.len(), 1);

        let transition = transitions[0];
        assert!(transition.action.is_some());

        // The action should be parsed as structured
        if let Some(ActionIR::Structured { assignments, .. }) = &transition.action {
            assert_eq!(assignments.len(), 1);
            assert_eq!(assignments[0].variable, "t");
            assert_eq!(assignments[0].value, 20.0);
        } else {
            panic!("Expected structured action");
        }
    }
}
