//! Feature chaining scoping strategy.
//!
//! This strategy is used for FeatureChainExpression and related constructs
//! where each element in a chain affects the scope for the next element.
//!
//! For example, in `vehicle.engine.pistons`:
//! - `vehicle` is resolved in the current scope
//! - `engine` is resolved in the type of `vehicle`
//! - `pistons` is resolved in the type of `engine`

use super::ScopedResolution;
use crate::ElementId;
use crate::ElementKind;
use crate::ModelGraph;

/// Resolve a name in a feature chaining context.
///
/// The `scope_id` should be the feature or type from the previous element in the chain.
/// Resolution looks in the type's features and inherited features.
///
/// # Arguments
///
/// * `graph` - The model graph
/// * `scope_id` - The element to scope from (feature or type)
/// * `name` - The name to resolve
///
/// # Returns
///
/// The resolved element ID, or `NotFound` if not found.
pub fn resolve_with_feature_chaining(
    graph: &ModelGraph,
    scope_id: &ElementId,
    name: &str,
) -> ScopedResolution {
    // Get the element at scope_id
    let scope = match graph.get_element(scope_id) {
        Some(e) => e,
        None => return ScopedResolution::NotFound,
    };

    // If the scope is a Feature, we need to look at its type
    // If it's a Type, we look directly at its features
    let type_id = if scope.kind.is_feature() {
        // Get the feature's type
        find_feature_type(graph, scope_id)
    } else if scope.kind == ElementKind::Type || scope.kind.is_subtype_of(ElementKind::Type) {
        Some(scope_id.clone())
    } else {
        None
    };

    match type_id {
        Some(tid) => resolve_feature_in_type(graph, &tid, name, 0),
        None => ScopedResolution::NotFound,
    }
}

/// Find the type of a feature by looking for FeatureTyping relationships.
///
/// A feature's type is determined by FeatureTyping elements that have:
/// - `typedFeature` pointing to this feature
/// - `type` (resolved) pointing to the type
///
/// If the type is not yet resolved (only `unresolved_type` exists), this
/// returns None.
///
/// Performance: O(1) lookup using reverse index `typed_feature_to_typings`.
pub fn find_feature_type(graph: &ModelGraph, feature_id: &ElementId) -> Option<ElementId> {
    // Use reverse index for O(1) lookup instead of O(n) scan
    if let Some(typing_ids) = graph.typed_feature_to_typings.get(feature_id) {
        for typing_id in typing_ids {
            if let Some(element) = graph.get_element(typing_id) {
                // Try to get the resolved type
                if let Some(type_ref) = element.props.get("type") {
                    if let Some(type_id) = type_ref.as_ref() {
                        return Some(type_id.clone());
                    }
                }
                // Type not resolved yet - could try unresolved_type but
                // that would require another resolution pass
            }
        }
    }

    None
}

/// Find all types of a feature (a feature can have multiple typings).
///
/// Performance: O(k) where k is the number of typings for this feature,
/// using reverse index `typed_feature_to_typings`.
pub fn find_feature_types(graph: &ModelGraph, feature_id: &ElementId) -> Vec<ElementId> {
    let mut types = Vec::new();

    // Use reverse index for O(1) lookup instead of O(n) scan
    if let Some(typing_ids) = graph.typed_feature_to_typings.get(feature_id) {
        for typing_id in typing_ids {
            if let Some(element) = graph.get_element(typing_id) {
                if let Some(type_ref) = element.props.get("type") {
                    if let Some(type_id) = type_ref.as_ref() {
                        types.push(type_id.clone());
                    }
                }
            }
        }
    }

    types
}

/// Maximum depth for inheritance traversal to prevent infinite loops.
const MAX_INHERITANCE_DEPTH: usize = 20;

/// Resolve a feature name within a type.
///
/// This looks in:
/// 1. The type's directly owned features
/// 2. Inherited features from supertypes (via Specialization chain)
///
/// Does NOT walk up parent namespaces - this is specific to feature lookup.
fn resolve_feature_in_type(
    graph: &ModelGraph,
    type_id: &ElementId,
    name: &str,
    depth: usize,
) -> ScopedResolution {
    // Prevent infinite recursion
    if depth > MAX_INHERITANCE_DEPTH {
        return ScopedResolution::NotFound;
    }

    // 1. Look in type's owned features (direct children that are features)
    for child in graph.children_of(type_id) {
        if child.name.as_deref() == Some(name) && child.kind.is_feature() {
            return ScopedResolution::Found(child.id.clone());
        }
    }

    // 2. Look in inherited features via Specialization chain
    for general_id in find_general_types(graph, type_id) {
        let inherited = resolve_feature_in_type(graph, &general_id, name, depth + 1);
        if !matches!(inherited, ScopedResolution::NotFound) {
            return inherited;
        }
    }

    ScopedResolution::NotFound
}

/// Find the general (super) types of a type via Specialization relationships.
///
/// A type's generals are determined by Specialization elements that have:
/// - `specific` pointing to this type
/// - `general` (resolved) pointing to the supertype
///
/// Performance: O(k) where k is the number of specializations for this type,
/// using reverse index `specific_to_specializations`.
fn find_general_types(graph: &ModelGraph, type_id: &ElementId) -> Vec<ElementId> {
    let mut generals = Vec::new();

    // Use reverse index for O(1) lookup instead of O(n) scan
    if let Some(spec_ids) = graph.specific_to_specializations.get(type_id) {
        for spec_id in spec_ids {
            if let Some(element) = graph.get_element(spec_id) {
                // Try to get the resolved general
                if let Some(general_ref) = element.props.get("general") {
                    if let Some(general_id) = general_ref.as_ref() {
                        generals.push(general_id.clone());
                    }
                }
            }
        }
    }

    generals
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Element;
    use sysml_meta::Value;

    #[test]
    fn test_resolve_chaining_not_found() {
        let graph = ModelGraph::new();
        let scope = ElementId::new_v4();

        let result = resolve_with_feature_chaining(&graph, &scope, "NonExistent");
        assert!(matches!(result, ScopedResolution::NotFound));
    }

    #[test]
    fn test_find_feature_type_basic() {
        let mut graph = ModelGraph::new();

        // Create a PartDefinition (the type)
        let type_elem = Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine");
        let type_id = graph.add_element(type_elem);

        // Create a PartUsage (the feature)
        let feature_elem = Element::new_with_kind(ElementKind::PartUsage).with_name("engine");
        let feature_id = graph.add_element(feature_elem);

        // Create a FeatureTyping linking them (with resolved type)
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop("typedFeature", Value::Ref(feature_id.clone()));
        typing.set_prop("type", Value::Ref(type_id.clone()));
        graph.add_element(typing);

        // Should find the type
        let found_type = find_feature_type(&graph, &feature_id);
        assert_eq!(found_type, Some(type_id));
    }

    #[test]
    fn test_find_feature_type_not_found() {
        let mut graph = ModelGraph::new();

        // Create a feature without any typing
        let feature_elem = Element::new_with_kind(ElementKind::PartUsage).with_name("orphan");
        let feature_id = graph.add_element(feature_elem);

        // Should not find any type
        let found_type = find_feature_type(&graph, &feature_id);
        assert_eq!(found_type, None);
    }

    #[test]
    fn test_resolve_feature_in_type() {
        let mut graph = ModelGraph::new();

        // Create a PartDefinition (the type)
        let type_elem = Element::new_with_kind(ElementKind::PartDefinition).with_name("Vehicle");
        let type_id = graph.add_element(type_elem);

        // Create a nested feature owned by the type
        let feature_elem = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("engine")
            .with_owner(type_id.clone());
        let feature_id = graph.add_element(feature_elem);

        // Should resolve the feature within the type
        let result = resolve_feature_in_type(&graph, &type_id, "engine", 0);
        assert!(matches!(result, ScopedResolution::Found(id) if id == feature_id));
    }

    #[test]
    fn test_resolve_inherited_feature() {
        let mut graph = ModelGraph::new();

        // Create base type with a feature
        let base_type = Element::new_with_kind(ElementKind::PartDefinition).with_name("Base");
        let base_id = graph.add_element(base_type);

        let base_feature = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("component")
            .with_owner(base_id.clone());
        let base_feature_id = graph.add_element(base_feature);

        // Create derived type that specializes base
        let derived_type = Element::new_with_kind(ElementKind::PartDefinition).with_name("Derived");
        let derived_id = graph.add_element(derived_type);

        // Create Specialization with resolved general
        let mut spec = Element::new_with_kind(ElementKind::Specialization);
        spec.set_prop("specific", Value::Ref(derived_id.clone()));
        spec.set_prop("general", Value::Ref(base_id.clone()));
        graph.add_element(spec);

        // Should resolve inherited feature in derived type
        let result = resolve_feature_in_type(&graph, &derived_id, "component", 0);
        assert!(matches!(result, ScopedResolution::Found(id) if id == base_feature_id));
    }

    #[test]
    fn test_feature_chaining_with_type() {
        let mut graph = ModelGraph::new();

        // Create Engine type with a 'pistons' feature
        let engine_type = Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine");
        let engine_type_id = graph.add_element(engine_type);

        let pistons = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("pistons")
            .with_owner(engine_type_id.clone());
        let pistons_id = graph.add_element(pistons);

        // Create Vehicle with an 'engine' feature typed by Engine
        let vehicle_type =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("Vehicle");
        let vehicle_id = graph.add_element(vehicle_type);

        let engine_feature = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("engine")
            .with_owner(vehicle_id.clone());
        let engine_feature_id = graph.add_element(engine_feature);

        // FeatureTyping: engine : Engine
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop("typedFeature", Value::Ref(engine_feature_id.clone()));
        typing.set_prop("type", Value::Ref(engine_type_id.clone()));
        graph.add_element(typing);

        // Now resolve 'pistons' starting from 'engine' feature
        // This should:
        // 1. Find engine's type (Engine)
        // 2. Look for 'pistons' in Engine's features
        let result = resolve_with_feature_chaining(&graph, &engine_feature_id, "pistons");
        assert!(matches!(result, ScopedResolution::Found(id) if id == pistons_id));
    }
}
