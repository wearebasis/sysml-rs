//! Relative namespace scoping strategy.
//!
//! This strategy resolves names relative to a specific element's namespace
//! WITHOUT walking up the ownership hierarchy. It's used for:
//! - Feature chain expressions (e.g., `vehicle.engine.cylinders`)
//! - Assignment targets (e.g., `assign obj.field := value`)
//!
//! ## Key Difference from OwningNamespace
//!
//! Relative namespace scoping sets `isInsideScope = false` in the Xtext pilot,
//! meaning it does NOT include parent scopes. This prevents polluting the scope
//! with unrelated context when accessing members through feature chains.
//!
//! ## Algorithm (based on Xtext pilot implementation)
//!
//! 1. Look in the namespace's owned members
//! 2. Look in inherited members (for Types)
//! 3. Look in imported members
//! 4. DO NOT walk up to parent namespaces

use super::ScopedResolution;
use crate::resolution::res_trace;
use crate::resolution::ResolutionContext;
use crate::ElementId;
use crate::ModelGraph;

/// Resolve a name relative to a specific namespace.
///
/// Unlike owning namespace resolution, this doesn't walk up to parent
/// namespaces. It only looks in:
/// 1. The namespace's owned members
/// 2. The namespace's inherited members (if it's a Type)
/// 3. The namespace's imported members
///
/// This is used for feature chain resolution where we want to find
/// members of a specific type, not lexically visible names.
pub fn resolve_in_relative_namespace(
    graph: &ModelGraph,
    namespace_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    #[cfg(feature = "resolution-tracing")]
    let ns_name = graph.get_element(namespace_id)
        .and_then(|e| e.name.clone())
        .unwrap_or_else(|| format!("{:.8}", namespace_id));
    res_trace!("Strategy: RelativeNamespace for '{}' in {}", name, ns_name);

    // Create a resolution context and get the scope table
    let mut ctx = ResolutionContext::new(graph);
    let table = ctx.get_scope_table(namespace_id).clone();

    // Look in OWNED members only (no parent walking)
    if let Some(id) = table.lookup_owned(name) {
        res_trace!("  OWNED: FOUND {:?}", id);
        return ScopedResolution::Found(id.clone());
    }
    res_trace!("  OWNED: None");

    // Look in INHERITED members
    if let Some(id) = table.lookup_inherited(name) {
        res_trace!("  INHERITED: FOUND {:?}", id);
        return ScopedResolution::Found(id.clone());
    }
    res_trace!("  INHERITED: None");

    // Look in IMPORTED members (but don't recurse into parents)
    if let Some(id) = table.lookup_imported(name) {
        res_trace!("  IMPORTED: FOUND {:?}", id);
        return ScopedResolution::Found(id.clone());
    }
    res_trace!("  IMPORTED: None");

    // Do NOT walk up to parent namespaces!
    // This is the key difference from owning namespace scoping.
    res_trace!("  NOT FOUND (no parent walking in relative scope)");

    ScopedResolution::NotFound
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
