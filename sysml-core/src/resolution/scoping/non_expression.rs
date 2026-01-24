//! Non-expression namespace scoping strategy.
//!
//! This strategy is used for cross-references that should skip expression scopes
//! when walking up the namespace hierarchy. It's used for:
//! - FeatureTyping
//! - Conjugation
//! - Other type-related references
//!
//! The key difference from OwningNamespace is that expression scopes (like
//! InvocationExpression, OperatorExpression) are skipped when walking up.

use super::ScopedResolution;
use crate::ElementId;
use crate::ModelGraph;

/// Resolve a name, skipping expression scopes.
///
/// Expression scopes include:
/// - InvocationExpression
/// - OperatorExpression
/// - FeatureChainExpression
///
/// When we encounter these while walking up the namespace hierarchy,
/// we skip them and continue to their parent.
pub fn resolve_in_non_expression_namespace(
    graph: &ModelGraph,
    scope_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // TODO: Implement non-expression namespace scoping
    // For now, fall back to owning namespace
    // The difference is skipping expression scopes when walking up

    // Find the non-expression namespace for this scope
    let effective_scope = find_non_expression_namespace(graph, scope_id);

    super::owning::resolve_in_owning_namespace(graph, &effective_scope, name)
}

/// Find the nearest non-expression namespace for a given element.
fn find_non_expression_namespace(graph: &ModelGraph, element_id: &ElementId) -> ElementId {
    use crate::ElementKind;

    let element = match graph.get_element(element_id) {
        Some(e) => e,
        None => return element_id.clone(),
    };

    // Check if this element is an expression type that should be skipped
    let is_expression = matches!(
        element.kind,
        ElementKind::InvocationExpression
            | ElementKind::OperatorExpression
            | ElementKind::FeatureChainExpression
            | ElementKind::FeatureReferenceExpression
            | ElementKind::SelectExpression
            | ElementKind::CollectExpression
            | ElementKind::IndexExpression
    );

    if is_expression {
        // Get the owner and recurse
        if let Some(owner) = graph.owner_of(element_id) {
            return find_non_expression_namespace(graph, &owner.id);
        }
    }

    element_id.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_non_expression_not_found() {
        let graph = ModelGraph::new();
        let scope = ElementId::new_v4();

        let result = resolve_in_non_expression_namespace(&graph, &scope, "NonExistent");
        assert!(matches!(result, ScopedResolution::NotFound));
    }
}
