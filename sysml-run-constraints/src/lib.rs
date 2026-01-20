//! # sysml-run-constraints
//!
//! Constraint compilation and evaluation for SysML v2.
//!
//! This crate provides:
//! - Extraction of constraints from ModelGraph
//! - A simple constraint evaluator (stub)

use std::collections::HashMap;
use sysml_core::{ElementKind, ModelGraph, Value};
use sysml_run::ConstraintIR;
use sysml_span::Diagnostic;

/// A compiled set of constraints.
#[derive(Debug, Clone)]
pub struct ConstraintSet {
    /// The constraints in this set.
    pub constraints: Vec<ConstraintIR>,
}

impl ConstraintSet {
    /// Create a new empty constraint set.
    pub fn new() -> Self {
        ConstraintSet {
            constraints: Vec::new(),
        }
    }

    /// Add a constraint.
    pub fn add(&mut self, constraint: ConstraintIR) {
        self.constraints.push(constraint);
    }

    /// Get the number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

impl Default for ConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract constraints from a ModelGraph.
///
/// Looks for elements with a "constraint" property or elements
/// of kind that have constraint expressions.
pub fn extract_constraints(graph: &ModelGraph) -> ConstraintSet {
    let mut set = ConstraintSet::new();

    // Look for elements with constraint property
    for element in graph.elements.values() {
        if let Some(constraint_expr) = element.get_prop("constraint").and_then(|v| v.as_str()) {
            let constraint = ConstraintIR::new(constraint_expr)
                .with_description(element.name.clone().unwrap_or_default());
            set.add(constraint);
        }

        // Also check for "expr" property (common pattern)
        if let Some(expr) = element.get_prop("expr").and_then(|v| v.as_str()) {
            let constraint = ConstraintIR::new(expr)
                .with_description(element.name.clone().unwrap_or_default());
            set.add(constraint);
        }
    }

    set
}

/// The result of evaluating a constraint.
#[derive(Debug, Clone)]
pub struct EvaluationResult {
    /// The constraint that was evaluated.
    pub constraint: ConstraintIR,
    /// Whether the constraint is satisfied.
    pub satisfied: bool,
    /// Any diagnostics from evaluation.
    pub diagnostics: Vec<Diagnostic>,
}

/// A context for constraint evaluation.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    /// Variable bindings.
    pub variables: HashMap<String, Value>,
}

impl EvaluationContext {
    /// Create a new empty context.
    pub fn new() -> Self {
        EvaluationContext {
            variables: HashMap::new(),
        }
    }

    /// Set a variable value.
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.variables.insert(name.into(), value.into());
    }

    /// Get a variable value.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

/// Evaluate a single constraint.
///
/// This is a stub implementation that only handles simple comparisons.
pub fn evaluate(
    constraint: &ConstraintIR,
    context: &EvaluationContext,
) -> EvaluationResult {
    let expr = &constraint.expr;

    // Stub: very simple expression evaluation
    let satisfied = evaluate_simple_expr(expr, context);

    EvaluationResult {
        constraint: constraint.clone(),
        satisfied,
        diagnostics: Vec::new(),
    }
}

/// Evaluate all constraints in a set.
pub fn evaluate_all(
    constraints: &ConstraintSet,
    context: &EvaluationContext,
) -> Vec<EvaluationResult> {
    constraints
        .constraints
        .iter()
        .map(|c| evaluate(c, context))
        .collect()
}

/// Check if all constraints are satisfied.
pub fn all_satisfied(results: &[EvaluationResult]) -> bool {
    results.iter().all(|r| r.satisfied)
}

/// Get failed constraints.
pub fn failed_constraints(results: &[EvaluationResult]) -> Vec<&EvaluationResult> {
    results.iter().filter(|r| !r.satisfied).collect()
}

// Simple expression evaluator (stub)
fn evaluate_simple_expr(expr: &str, context: &EvaluationContext) -> bool {
    // Handle simple comparisons like "x < 10" or "y == 5"
    let expr = expr.trim();

    // Try "var < num"
    if let Some((var, rest)) = expr.split_once('<') {
        let var = var.trim();
        let num: f64 = rest.trim().parse().unwrap_or(0.0);
        if let Some(val) = context.get(var).and_then(|v| v.as_float()) {
            return val < num;
        }
    }

    // Try "var > num"
    if let Some((var, rest)) = expr.split_once('>') {
        if !rest.starts_with('=') {
            let var = var.trim();
            let num: f64 = rest.trim().parse().unwrap_or(0.0);
            if let Some(val) = context.get(var).and_then(|v| v.as_float()) {
                return val > num;
            }
        }
    }

    // Try "var == num"
    if let Some((var, rest)) = expr.split_once("==") {
        let var = var.trim();
        let num: f64 = rest.trim().parse().unwrap_or(0.0);
        if let Some(val) = context.get(var).and_then(|v| v.as_float()) {
            return (val - num).abs() < f64::EPSILON;
        }
    }

    // Try "var <= num"
    if let Some((var, rest)) = expr.split_once("<=") {
        let var = var.trim();
        let num: f64 = rest.trim().parse().unwrap_or(0.0);
        if let Some(val) = context.get(var).and_then(|v| v.as_float()) {
            return val <= num;
        }
    }

    // Try "var >= num"
    if let Some((var, rest)) = expr.split_once(">=") {
        let var = var.trim();
        let num: f64 = rest.trim().parse().unwrap_or(0.0);
        if let Some(val) = context.get(var).and_then(|v| v.as_float()) {
            return val >= num;
        }
    }

    // Unknown expression - assume true (stub behavior)
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_core::Element;

    #[test]
    fn constraint_set_creation() {
        let mut set = ConstraintSet::new();
        set.add(ConstraintIR::new("x < 10"));
        set.add(ConstraintIR::new("y > 0"));

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn extract_constraints_from_graph() {
        let mut graph = ModelGraph::new();

        let elem = Element::new_with_kind(ElementKind::AttributeUsage)
            .with_name("SpeedLimit")
            .with_prop("constraint", "speed < 100");
        graph.add_element(elem);

        let constraints = extract_constraints(&graph);
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints.constraints[0].expr, "speed < 100");
    }

    #[test]
    fn evaluate_less_than() {
        let constraint = ConstraintIR::new("x < 10");
        let mut context = EvaluationContext::new();
        context.set("x", 5.0f64);

        let result = evaluate(&constraint, &context);
        assert!(result.satisfied);

        context.set("x", 15.0f64);
        let result = evaluate(&constraint, &context);
        assert!(!result.satisfied);
    }

    #[test]
    fn evaluate_greater_than() {
        let constraint = ConstraintIR::new("x > 0");
        let mut context = EvaluationContext::new();
        context.set("x", 5.0f64);

        let result = evaluate(&constraint, &context);
        assert!(result.satisfied);

        context.set("x", -1.0f64);
        let result = evaluate(&constraint, &context);
        assert!(!result.satisfied);
    }

    #[test]
    fn evaluate_equals() {
        let constraint = ConstraintIR::new("x == 10");
        let mut context = EvaluationContext::new();
        context.set("x", 10.0f64);

        let result = evaluate(&constraint, &context);
        assert!(result.satisfied);

        context.set("x", 11.0f64);
        let result = evaluate(&constraint, &context);
        assert!(!result.satisfied);
    }

    #[test]
    fn evaluate_all_constraints() {
        let mut set = ConstraintSet::new();
        set.add(ConstraintIR::new("x < 100"));
        set.add(ConstraintIR::new("y > 0"));

        let mut context = EvaluationContext::new();
        context.set("x", 50.0f64);
        context.set("y", 10.0f64);

        let results = evaluate_all(&set, &context);
        assert!(all_satisfied(&results));
    }

    #[test]
    fn failed_constraints_detection() {
        let mut set = ConstraintSet::new();
        set.add(ConstraintIR::new("x < 100"));
        set.add(ConstraintIR::new("y > 0"));

        let mut context = EvaluationContext::new();
        context.set("x", 50.0f64);
        context.set("y", -5.0f64); // This will fail

        let results = evaluate_all(&set, &context);
        assert!(!all_satisfied(&results));

        let failed = failed_constraints(&results);
        assert_eq!(failed.len(), 1);
    }
}
