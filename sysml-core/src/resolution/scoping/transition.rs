//! Transition-specific scoping strategy.
//!
//! This strategy is used for resolving references within state machine
//! transitions, particularly for:
//! - Trigger events
//! - Guard conditions
//! - Effect actions
//!
//! The scoping rules for transitions consider the containing state definition
//! and the states involved in the transition.

use super::ScopedResolution;
use crate::ElementId;
use crate::ModelGraph;

/// Resolve a name in a transition context.
///
/// Transition scoping considers:
/// 1. The source state's features
/// 2. The containing state definition's features
/// 3. The containing package's members
pub fn resolve_in_transition_context(
    graph: &ModelGraph,
    scope_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // TODO: Implement transition-specific scoping
    // For now, fall back to owning namespace

    // Get the transition element
    let transition = match graph.get_element(scope_id) {
        Some(e) => e,
        None => return ScopedResolution::NotFound,
    };

    // Check if this is actually a transition-related element
    use crate::ElementKind;
    let is_transition_related = matches!(
        transition.kind,
        ElementKind::TransitionUsage
            | ElementKind::AcceptActionUsage
            | ElementKind::SendActionUsage
            | ElementKind::TriggerInvocationExpression
            | ElementKind::StateSubactionMembership
    );

    if is_transition_related {
        // TODO: Implement proper transition scoping
        // For now, use owning namespace
        super::owning::resolve_in_owning_namespace(graph, scope_id, name)
    } else {
        // Not a transition context, fall back to owning namespace
        super::owning::resolve_in_owning_namespace(graph, scope_id, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_transition_not_found() {
        let graph = ModelGraph::new();
        let scope = ElementId::new_v4();

        let result = resolve_in_transition_context(&graph, &scope, "NonExistent");
        assert!(matches!(result, ScopedResolution::NotFound));
    }
}
