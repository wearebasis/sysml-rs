//! AU Hybrid Installation State Machine Simulation
//!
//! This example parses the gen2-architecture SysML model and runs
//! the composed state machines from the `auHybridInstallation` part.
//!
//! # Usage
//!
//! ```sh
//! cargo run -p sysml-run-statemachine --example au_hybrid_sim
//! ```

use std::path::PathBuf;
use sysml_core::{ElementKind, ModelGraph};
use sysml_run::{RegionIR, StateIR, StateMachineIR, TransitionIR};
use sysml_run_statemachine::{ParallelStateMachineRunner, StateMachineCompiler};
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

/// SysML files containing state definitions (these should parse cleanly).
const STATE_DEF_FILES: &[&str] = &[
    "Libraries/Types.sysml",
    "Libraries/GenericParts.sysml",
    "ProblemSpace/OperationalScenarios/GridInterface.sysml",
    "ProblemSpace/OperationalScenarios/InvertersInterface.sysml",
    "SolutionSpace/Behaviour/PanelBehaviors.sysml",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find the gen2-architecture directory
    let gen2_path = find_gen2_architecture_path()?;
    println!("Loading SysML model from: {}", gen2_path.display());

    // Load state definition files (avoiding files with parse issues)
    let mut files = Vec::new();
    for f in STATE_DEF_FILES {
        let path = gen2_path.join(f);
        match std::fs::read_to_string(&path) {
            Ok(text) => files.push(SysmlFile::new(path.to_string_lossy(), text)),
            Err(e) => eprintln!("Warning: Could not read {}: {}", path.display(), e),
        }
    }

    println!("Parsing {} SysML files...", files.len());

    // Parse files
    let parser = PestParser::new();
    let result = parser.parse(&files);

    // Report but don't fail on parse errors
    let has_errors = result.has_errors();
    if has_errors {
        eprintln!("\nParse warnings/errors:");
        for diag in &result.diagnostics {
            eprintln!("  {}", diag);
        }
    }

    let graph = result.graph;
    println!(
        "Parsed {} elements, {} relationships",
        graph.element_count(),
        graph.relationship_count()
    );

    // Show what state definitions we found
    println!("\n=== State Definitions Found ===");
    print_state_definitions(&graph);

    // Try to compile from parsed state definitions
    println!("\n=== Attempting State Machine Compilation ===");

    // First try: use any part with exhibit states we can find
    let exhibit_states: Vec<_> = graph
        .elements
        .values()
        .filter(|e| e.kind == ElementKind::ExhibitStateUsage)
        .collect();

    if !exhibit_states.is_empty() {
        println!("Found {} exhibit state usages", exhibit_states.len());

        // Find a suitable root part
        for part in graph.elements.values() {
            if matches!(part.kind, ElementKind::PartUsage | ElementKind::PartDefinition) {
                if let Some(name) = &part.name {
                    if name.contains("Installation") || name.contains("Panel") || name.contains("Grid") {
                        println!("Trying to compile from part: {}", name);
                        match StateMachineCompiler::compile_from_part(&graph, &part.id) {
                            Ok(ir) => {
                                // Only use if we got a meaningful state machine
                                let total_transitions: usize = ir.regions.iter().map(|r| r.transitions.len()).sum();
                                if ir.regions.len() >= 2 && total_transitions > 0 {
                                    run_simulation(ir);
                                    return Ok(());
                                }
                                println!("  (insufficient state machine data, trying next)");
                            }
                            Err(_) => continue,
                        }
                    }
                }
            }
        }
    }

    // Fallback: Build state machine manually from parsed state definitions
    println!("\n=== Building State Machine Manually ===");
    let ir = build_hybrid_state_machine_manually(&graph);

    if ir.regions.is_empty() {
        eprintln!("Could not build state machine - no state definitions found");
        return Err("No state definitions found".into());
    }

    run_simulation(ir);

    Ok(())
}

/// Build the hybrid installation state machine manually from parsed state definitions.
///
/// Note: The current parser doesn't extract transition source/target from `first`/`then` syntax.
/// So we manually specify the transitions based on the SysML model until the parser is enhanced.
fn build_hybrid_state_machine_manually(_graph: &ModelGraph) -> StateMachineIR {
    let mut ir = StateMachineIR::parallel("AUHybridInstallation");

    // Build GridStates region manually
    let grid_region = RegionIR::new("grid", "energized")
        .with_state(StateIR::new("energized"))
        .with_state(StateIR::new("deEnergized"))
        .with_state(StateIR::new("fault"))
        // Transitions from GridStates in GridInterface.sysml
        .with_transition(TransitionIR::new("energized", "deEnergized").with_event("gridFail"))
        .with_transition(TransitionIR::new("energized", "deEnergized").with_event("deEnergize"))
        .with_transition(TransitionIR::new("deEnergized", "energized").with_event("energize"))
        .with_transition(TransitionIR::new("deEnergized", "energized").with_event("gridRestore"))
        .with_transition(TransitionIR::new("energized", "fault").with_event("fault"))
        .with_transition(TransitionIR::new("fault", "deEnergized").with_event("fault_cleared"));
    ir = ir.with_region(grid_region);

    // Build HybridInverterStates region
    let inverter_region = RegionIR::new("inverter", "gridTied")
        .with_state(StateIR::new("gridTied"))
        .with_state(StateIR::new("islanding"))
        .with_state(StateIR::new("islanded"))
        .with_state(StateIR::new("reconnecting"))
        // Transitions from HybridInverterStates in InvertersInterface.sysml
        .with_transition(TransitionIR::new("gridTied", "islanding").with_event("gridFail"))
        .with_transition(TransitionIR::new("gridTied", "islanding").with_event("begin_island"))
        .with_transition(TransitionIR::new("islanding", "islanded").with_event("island_complete"))
        .with_transition(TransitionIR::new("islanding", "gridTied").with_event("island_fault"))
        .with_transition(TransitionIR::new("islanded", "reconnecting").with_event("gridRestore"))
        .with_transition(TransitionIR::new("islanded", "reconnecting").with_event("begin_reconnect"))
        .with_transition(TransitionIR::new("reconnecting", "gridTied").with_event("reconnect_complete"));
    ir = ir.with_region(inverter_region);

    // Build MainSwitchControllerStates region
    let controller_region = RegionIR::new("controller", "monitoring")
        .with_state(StateIR::new("monitoring"))
        .with_state(StateIR::new("waitingRelayConfirm"))
        .with_state(StateIR::new("verifyingRelayOpen"))
        .with_state(StateIR::new("isolated"))
        .with_state(StateIR::new("fault"))
        // Transitions from MainSwitchControllerStates in PanelBehaviors.sysml
        .with_transition(TransitionIR::new("monitoring", "waitingRelayConfirm").with_event("gridFail"))
        .with_transition(TransitionIR::new("monitoring", "waitingRelayConfirm").with_event("detectFaultAndOpenRelay"))
        .with_transition(TransitionIR::new("waitingRelayConfirm", "verifyingRelayOpen").with_event("auxContactChanged"))
        .with_transition(TransitionIR::new("verifyingRelayOpen", "isolated").with_event("verifyComplete"))
        .with_transition(TransitionIR::new("verifyingRelayOpen", "fault").with_event("verifyTimedOut"))
        .with_transition(TransitionIR::new("isolated", "monitoring").with_event("gridRestore"))
        .with_transition(TransitionIR::new("isolated", "monitoring").with_event("gridRestoredFromIsolated"));
    ir = ir.with_region(controller_region);

    // Build LatchingRelayStates region
    let relay_region = RegionIR::new("relay", "closed")
        .with_state(StateIR::new("closed"))
        .with_state(StateIR::new("opening"))
        .with_state(StateIR::new("open"))
        .with_state(StateIR::new("closing"))
        // Transitions from LatchingRelayStates in GenericParts.sysml
        .with_transition(TransitionIR::new("closed", "opening").with_event("startOpen"))
        .with_transition(TransitionIR::new("closed", "opening").with_event("gridFail"))
        .with_transition(TransitionIR::new("opening", "open").with_event("completeOpen"))
        .with_transition(TransitionIR::new("open", "closing").with_event("startClose"))
        .with_transition(TransitionIR::new("open", "closing").with_event("gridRestore"))
        .with_transition(TransitionIR::new("closing", "closed").with_event("completeClose"));
    ir = ir.with_region(relay_region);

    ir
}

/// Find a state definition by name.
#[allow(dead_code)]
fn find_state_def_by_name(graph: &ModelGraph, name: &str) -> Option<sysml_core::ElementId> {
    graph
        .elements_by_kind(&ElementKind::StateDefinition)
        .find(|e| e.name.as_deref() == Some(name))
        .map(|e| e.id.clone())
}

/// Convert a state definition to a RegionIR.
#[allow(dead_code)]
fn state_def_to_region(
    graph: &ModelGraph,
    state_def_id: &sysml_core::ElementId,
    region_name: &str,
) -> Option<RegionIR> {
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
        let name = state.name.clone().unwrap_or_else(|| state.id.to_string());
        region = region.with_state(StateIR::new(&name));
    }

    // Find transitions (TransitionUsage children)
    for child in graph.children_of(state_def_id) {
        if child.kind == ElementKind::TransitionUsage {
            // Get transition source and target
            let source = child
                .props
                .get("unresolved_source")
                .and_then(|v| v.as_str())
                .map(String::from);

            let target = child
                .props
                .get("unresolved_target")
                .and_then(|v| v.as_str())
                .map(String::from);

            if let (Some(from), Some(to)) = (source, target) {
                let mut transition = TransitionIR::new(&from, &to);

                // Extract event/trigger
                if let Some(trigger) = child.props.get("trigger").and_then(|v| v.as_str()) {
                    transition = transition.with_event(trigger);
                }

                region = region.with_transition(transition);
            }
        }
    }

    Some(region)
}

/// Run the state machine simulation.
fn run_simulation(ir: StateMachineIR) {
    println!("\nCompiled state machine: {}", ir.name);
    println!("Regions: {}", ir.regions.len());

    for region in &ir.regions {
        println!(
            "  - {} (initial: {}, states: {}, transitions: {})",
            region.name,
            region.initial,
            region.states.len(),
            region.transitions.len()
        );
    }

    // Create runner and simulate
    let mut runner = ParallelStateMachineRunner::new(ir);

    // Set timing parameters based on ComponentDatasheets.sysml
    // Grid relay timing: 20ms operate time
    runner.set_context("relay_operate_time", 20.0);

    println!("\n========================================");
    println!("=== AU Hybrid Installation Simulation ===");
    println!("========================================");

    println!("\n--- Initial State ---");
    for (region, state) in runner.region_states() {
        println!("  {}: {}", region, state);
    }
    println!("  t = {}ms", runner.t_ms());

    // Scenario: Grid failure and recovery
    println!("\n--- SCENARIO: Grid Failure ---");
    println!("The grid experiences a fault and loses power.\n");

    // Step 1: Grid fails
    println!("Event: gridFail");
    runner.send("gridFail");
    print_states(&runner);

    // Step 2: Relay completes opening (after 20ms operate time)
    println!("\nEvent: completeOpen (relay finishes opening)");
    runner.send("completeOpen");
    print_states(&runner);

    // Step 3: Controller detects relay open via aux contact
    println!("\nEvent: auxContactChanged (aux contact confirms relay open)");
    runner.send("auxContactChanged");
    print_states(&runner);

    // Step 4: Verification period completes
    println!("\nEvent: verifyComplete (verification timer elapsed)");
    runner.send("verifyComplete");
    print_states(&runner);

    // Step 5: Inverter finishes islanding sequence
    println!("\nEvent: island_complete (inverter now operating in island mode)");
    runner.send("island_complete");
    print_states(&runner);

    println!("\n--- System is now ISLANDED ---");
    println!("The home is powered by the battery/inverter while grid is down.\n");

    // Scenario: Grid restoration
    println!("--- SCENARIO: Grid Restoration ---");
    println!("The grid is restored and the system reconnects.\n");

    // Step 1: Grid power returns
    println!("Event: gridRestore");
    runner.send("gridRestore");
    print_states(&runner);

    // Step 2: Relay closes
    println!("\nEvent: completeClose (relay finishes closing)");
    runner.send("completeClose");
    print_states(&runner);

    // Step 3: Inverter completes reconnection
    println!("\nEvent: reconnect_complete (inverter synchronized with grid)");
    runner.send("reconnect_complete");
    print_states(&runner);

    println!("\n--- System is now GRID-CONNECTED ---");
    println!("Normal operation resumed.\n");

    println!("========================================");
    println!("Final state:");
    for (region, state) in runner.region_states() {
        println!("  {}: {}", region, state);
    }
    println!("Simulation time: {}ms", runner.t_ms());
}

fn print_states(runner: &ParallelStateMachineRunner) {
    print!("  State: ");
    let state_strs: Vec<_> = runner.region_states()
        .iter()
        .map(|(r, s)| format!("{}={}", r, s))
        .collect();
    println!("{}", state_strs.join(", "));
}

/// Find the gen2-architecture-sysml2 directory.
fn find_gen2_architecture_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Try common locations
    let candidates = [
        // Relative to the project
        PathBuf::from("../gen2-architecture-sysml2"),
        // Absolute path for the configured additional working directory
        PathBuf::from("/Users/david/Documents/gen2-architecture-sysml2"),
        // Try current directory
        PathBuf::from("gen2-architecture-sysml2"),
    ];

    for candidate in &candidates {
        if candidate.exists() && candidate.is_dir() {
            return Ok(candidate.clone());
        }
    }

    Err("Could not find gen2-architecture-sysml2 directory".into())
}

/// Print all state definitions found in the graph.
fn print_state_definitions(graph: &ModelGraph) {
    for element in graph.elements_by_kind(&ElementKind::StateDefinition) {
        let name = element.name.as_deref().unwrap_or("<unnamed>");
        let states: Vec<_> = graph
            .children_of(&element.id)
            .filter(|c| c.kind == ElementKind::StateUsage)
            .filter_map(|c| c.name.clone())
            .collect();
        let transitions: Vec<_> = graph
            .children_of(&element.id)
            .filter(|c| c.kind == ElementKind::TransitionUsage)
            .filter_map(|c| c.name.clone())
            .collect();
        println!("  {} (states: {:?}, transitions: {:?})", name, states, transitions);
    }
}
