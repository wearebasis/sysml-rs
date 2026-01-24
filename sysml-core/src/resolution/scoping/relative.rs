//! Relative namespace scoping strategy.
//!
//! This strategy resolves names relative to a specific element's namespace
//! rather than walking up the ownership hierarchy. It's used when the scope
//! is explicitly specified or for feature chain resolution.

use super::ScopedResolution;
use crate::ElementId;
use crate::ModelGraph;

/// Resolve a name relative to a specific namespace.
///
/// Unlike owning namespace resolution, this doesn't walk up to parent
/// namespaces. It only looks in:
/// 1. The namespace's owned members
/// 2. The namespace's inherited members (if it's a Type)
/// 3. The namespace's imported members
pub fn resolve_in_relative_namespace(
    graph: &ModelGraph,
    namespace_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // TODO: Implement relative namespace scoping
    // This should NOT walk up to parent namespaces

    // For now, fall back to owning namespace
    // This is incorrect but provides basic functionality
    super::owning::resolve_in_owning_namespace(graph, namespace_id, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_relative_not_found() {
        let graph = ModelGraph::new();
        let scope = ElementId::new_v4();

        let result = resolve_in_relative_namespace(&graph, &scope, "NonExistent");
        assert!(matches!(result, ScopedResolution::NotFound));
    }
}
