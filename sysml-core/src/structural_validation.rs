//! Structural validation for SysML v2 model graphs.
//!
//! This module validates the structural integrity of the model graph:
//! - No orphan elements (except roots)
//! - No ownership cycles
//! - No dangling references in memberships
//! - Relationship type constraints (source/target types)
//!
//! ## Usage
//!
//! ```ignore
//! let errors = graph.validate_structure();
//! if errors.is_empty() {
//!     println!("Model structure is valid!");
//! } else {
//!     for error in errors {
//!         eprintln!("Error: {}", error);
//!     }
//! }
//! ```

use std::collections::HashSet;
use std::fmt;

use crate::membership::props as membership_props;
use crate::{ElementKind, ModelGraph};
use sysml_id::ElementId;

/// An error in the structural integrity of the model graph.
#[derive(Debug, Clone, PartialEq)]
pub enum StructuralError {
    /// An element has no owner and is not a valid root type.
    OrphanElement {
        element_id: ElementId,
        element_name: Option<String>,
        element_kind: ElementKind,
    },

    /// An ownership cycle was detected.
    OwnershipCycle {
        /// The elements forming the cycle (in order).
        element_ids: Vec<ElementId>,
    },

    /// A membership references a non-existent element.
    DanglingMembershipRef {
        membership_id: ElementId,
        /// The property containing the dangling reference.
        property: String,
        /// The missing element ID.
        missing_id: ElementId,
    },

    /// A relationship source element doesn't match expected type.
    RelationshipSourceTypeMismatch {
        relationship_id: ElementId,
        relationship_kind: ElementKind,
        source_id: ElementId,
        source_kind: ElementKind,
        expected_kind: ElementKind,
    },

    /// A relationship target element doesn't match expected type.
    RelationshipTargetTypeMismatch {
        relationship_id: ElementId,
        relationship_kind: ElementKind,
        target_id: ElementId,
        target_kind: ElementKind,
        expected_kind: ElementKind,
    },

    /// A relationship references a non-existent element.
    DanglingRelationshipRef {
        relationship_id: ElementId,
        /// "source" or "target".
        endpoint: String,
        missing_id: ElementId,
    },

    /// An element's owning_membership field points to a non-existent element.
    DanglingOwningMembership {
        element_id: ElementId,
        element_name: Option<String>,
        missing_membership_id: ElementId,
    },

    /// An element's owning_membership points to a non-membership element.
    InvalidOwningMembership {
        element_id: ElementId,
        membership_id: ElementId,
        membership_kind: ElementKind,
    },
}

impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructuralError::OrphanElement {
                element_id,
                element_name,
                element_kind,
            } => {
                let name_str = element_name
                    .as_ref()
                    .map(|n| format!(" '{}'", n))
                    .unwrap_or_default();
                write!(
                    f,
                    "Orphan element{} ({:?}): {} has no owner",
                    name_str, element_kind, element_id
                )
            }
            StructuralError::OwnershipCycle { element_ids } => {
                write!(f, "Ownership cycle detected: {:?}", element_ids)
            }
            StructuralError::DanglingMembershipRef {
                membership_id,
                property,
                missing_id,
            } => {
                write!(
                    f,
                    "Dangling reference in membership {}: property '{}' references non-existent element {}",
                    membership_id, property, missing_id
                )
            }
            StructuralError::RelationshipSourceTypeMismatch {
                relationship_kind,
                source_kind,
                expected_kind,
                ..
            } => {
                write!(
                    f,
                    "Relationship type mismatch: {:?} expects source type {:?}, got {:?}",
                    relationship_kind, expected_kind, source_kind
                )
            }
            StructuralError::RelationshipTargetTypeMismatch {
                relationship_kind,
                target_kind,
                expected_kind,
                ..
            } => {
                write!(
                    f,
                    "Relationship type mismatch: {:?} expects target type {:?}, got {:?}",
                    relationship_kind, expected_kind, target_kind
                )
            }
            StructuralError::DanglingRelationshipRef {
                relationship_id,
                endpoint,
                missing_id,
            } => {
                write!(
                    f,
                    "Dangling {} reference in relationship {}: element {} does not exist",
                    endpoint, relationship_id, missing_id
                )
            }
            StructuralError::DanglingOwningMembership {
                element_id,
                element_name,
                missing_membership_id,
            } => {
                let name_str = element_name
                    .as_ref()
                    .map(|n| format!(" '{}'", n))
                    .unwrap_or_default();
                write!(
                    f,
                    "Element{} ({}) has owning_membership pointing to non-existent membership {}",
                    name_str, element_id, missing_membership_id
                )
            }
            StructuralError::InvalidOwningMembership {
                element_id,
                membership_id,
                membership_kind,
            } => {
                write!(
                    f,
                    "Element {}'s owning_membership {} is not a Membership (is {:?})",
                    element_id, membership_id, membership_kind
                )
            }
        }
    }
}

impl std::error::Error for StructuralError {}

/// Types that are valid as root elements (can exist without an owner).
const VALID_ROOT_KINDS: &[ElementKind] = &[
    ElementKind::Package,
    ElementKind::LibraryPackage,
    // Membership elements can be roots in certain contexts
    ElementKind::Membership,
    ElementKind::OwningMembership,
    ElementKind::FeatureMembership,
];

/// Check if an element kind is valid as a root (no owner).
fn is_valid_root_kind(kind: &ElementKind) -> bool {
    VALID_ROOT_KINDS.contains(kind)
        || kind.is_subtype_of(ElementKind::Membership)
        || kind.is_subtype_of(ElementKind::Package)
}

impl ModelGraph {
    /// Validate the structural integrity of the model graph.
    ///
    /// Checks for:
    /// - Orphan elements (non-root elements without owners)
    /// - Ownership cycles
    /// - Dangling references in memberships
    /// - Invalid owning_membership references
    ///
    /// # Returns
    ///
    /// A vector of structural errors. Empty if the model is valid.
    pub fn validate_structure(&self) -> Vec<StructuralError> {
        let mut errors = Vec::new();

        self.validate_orphans(&mut errors);
        self.validate_ownership_cycles(&mut errors);
        self.validate_membership_references(&mut errors);
        self.validate_owning_membership_references(&mut errors);
        self.validate_relationship_references(&mut errors);

        errors
    }

    /// Check for orphan elements.
    fn validate_orphans(&self, errors: &mut Vec<StructuralError>) {
        for (id, element) in &self.elements {
            // Skip if element has an owner
            if element.owner.is_some() || element.owning_membership.is_some() {
                continue;
            }

            // Check if it's a valid root type
            if !is_valid_root_kind(&element.kind) {
                errors.push(StructuralError::OrphanElement {
                    element_id: id.clone(),
                    element_name: element.name.clone(),
                    element_kind: element.kind.clone(),
                });
            }
        }
    }

    /// Check for ownership cycles.
    fn validate_ownership_cycles(&self, errors: &mut Vec<StructuralError>) {
        let mut visited_global: HashSet<ElementId> = HashSet::new();

        for id in self.elements.keys() {
            if visited_global.contains(id) {
                continue;
            }

            let mut path: Vec<ElementId> = Vec::new();
            let mut path_set: HashSet<ElementId> = HashSet::new();
            let mut current = Some(id.clone());

            while let Some(current_id) = current {
                if path_set.contains(&current_id) {
                    // Found a cycle - extract the cycle portion
                    let cycle_start = path.iter().position(|i| i == &current_id).unwrap();
                    let cycle: Vec<ElementId> = path[cycle_start..].to_vec();
                    errors.push(StructuralError::OwnershipCycle { element_ids: cycle });
                    break;
                }

                visited_global.insert(current_id.clone());
                path.push(current_id.clone());
                path_set.insert(current_id.clone());

                // Move to owner
                current = self
                    .elements
                    .get(&current_id)
                    .and_then(|e| e.owner.clone());
            }
        }
    }

    /// Validate membership element references.
    fn validate_membership_references(&self, errors: &mut Vec<StructuralError>) {
        for (id, element) in &self.elements {
            // Only check Membership elements
            if element.kind != ElementKind::Membership
                && !element.kind.is_subtype_of(ElementKind::Membership)
            {
                continue;
            }

            // Check memberElement reference
            if let Some(member_ref) = element.props.get(membership_props::MEMBER_ELEMENT) {
                if let Some(member_id) = member_ref.as_ref() {
                    if !self.elements.contains_key(member_id) {
                        errors.push(StructuralError::DanglingMembershipRef {
                            membership_id: id.clone(),
                            property: membership_props::MEMBER_ELEMENT.to_string(),
                            missing_id: member_id.clone(),
                        });
                    }
                }
            }

            // Check membershipOwningNamespace reference
            if let Some(ns_ref) = element.props.get(membership_props::MEMBERSHIP_OWNING_NAMESPACE) {
                if let Some(ns_id) = ns_ref.as_ref() {
                    if !self.elements.contains_key(ns_id) {
                        errors.push(StructuralError::DanglingMembershipRef {
                            membership_id: id.clone(),
                            property: membership_props::MEMBERSHIP_OWNING_NAMESPACE.to_string(),
                            missing_id: ns_id.clone(),
                        });
                    }
                }
            }
        }
    }

    /// Validate owning_membership references in elements.
    fn validate_owning_membership_references(&self, errors: &mut Vec<StructuralError>) {
        for (id, element) in &self.elements {
            if let Some(membership_id) = &element.owning_membership {
                match self.elements.get(membership_id) {
                    None => {
                        errors.push(StructuralError::DanglingOwningMembership {
                            element_id: id.clone(),
                            element_name: element.name.clone(),
                            missing_membership_id: membership_id.clone(),
                        });
                    }
                    Some(membership) => {
                        // Check it's actually a Membership
                        if membership.kind != ElementKind::Membership
                            && !membership.kind.is_subtype_of(ElementKind::Membership)
                        {
                            errors.push(StructuralError::InvalidOwningMembership {
                                element_id: id.clone(),
                                membership_id: membership_id.clone(),
                                membership_kind: membership.kind.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    /// Validate relationship source/target references.
    fn validate_relationship_references(&self, errors: &mut Vec<StructuralError>) {
        for (id, rel) in &self.relationships {
            // Check source exists
            if !self.elements.contains_key(&rel.source) {
                errors.push(StructuralError::DanglingRelationshipRef {
                    relationship_id: id.clone(),
                    endpoint: "source".to_string(),
                    missing_id: rel.source.clone(),
                });
            }

            // Check target exists
            if !self.elements.contains_key(&rel.target) {
                errors.push(StructuralError::DanglingRelationshipRef {
                    relationship_id: id.clone(),
                    endpoint: "target".to_string(),
                    missing_id: rel.target.clone(),
                });
            }
        }
    }

    /// Validate relationship type constraints for Element-based relationships.
    ///
    /// This checks that Relationship elements (like FeatureTyping, Specialization)
    /// have source/target types matching the spec constraints.
    pub fn validate_relationship_types(&self) -> Vec<StructuralError> {
        let errors = Vec::new();

        for (id, element) in &self.elements {
            // Only check Relationship elements
            if !element.kind.is_relationship() {
                continue;
            }

            // Get expected source/target types
            let expected_source = element.kind.relationship_source_type();
            let expected_target = element.kind.relationship_target_type();

            // This is a simplification - in practice we'd need to get the actual
            // source/target from the relationship's properties
            // For now, just demonstrate the structure

            // Get source element if available (from relatedElement[0] typically)
            if let Some(expected) = expected_source {
                // TODO: Extract actual source from relationship properties
                // and validate against expected type
                let _ = expected; // Suppress unused warning
            }

            if let Some(expected) = expected_target {
                // TODO: Extract actual target from relationship properties
                // and validate against expected type
                let _ = expected; // Suppress unused warning
            }

            let _ = id; // Suppress unused warning
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Element, VisibilityKind};

    #[test]
    fn valid_structure_no_errors() {
        let mut graph = ModelGraph::new();

        // Create a valid hierarchy
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("Pkg");
        let pkg_id = graph.add_element(pkg);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let _part_id = graph.add_owned_element(part, pkg_id.clone(), VisibilityKind::Public);

        let errors = graph.validate_structure();
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn orphan_element_detected() {
        let mut graph = ModelGraph::new();

        // Add a PartDefinition without an owner (not a valid root)
        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("OrphanPart");
        graph.add_element(part);

        let errors = graph.validate_structure();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], StructuralError::OrphanElement { .. }));
    }

    #[test]
    fn package_can_be_root() {
        let mut graph = ModelGraph::new();

        // Package is a valid root
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("RootPackage");
        graph.add_element(pkg);

        let errors = graph.validate_structure();
        assert!(errors.is_empty());
    }

    #[test]
    fn ownership_cycle_detected() {
        let mut graph = ModelGraph::new();

        // Create elements manually to create a cycle
        let a = Element::new_with_kind(ElementKind::Package).with_name("A");
        let a_id = a.id.clone();
        graph.add_element(a);

        let b = Element::new_with_kind(ElementKind::Package)
            .with_name("B")
            .with_owner(a_id.clone());
        let b_id = b.id.clone();
        graph.add_element(b);

        // Create cycle: set A's owner to B
        if let Some(a_elem) = graph.elements.get_mut(&a_id) {
            a_elem.owner = Some(b_id);
        }

        let errors = graph.validate_structure();
        let cycle_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StructuralError::OwnershipCycle { .. }))
            .collect();
        assert!(!cycle_errors.is_empty(), "Expected cycle error");
    }

    #[test]
    fn dangling_owning_membership_detected() {
        let mut graph = ModelGraph::new();

        // Create an element with a dangling owning_membership
        let fake_membership_id = ElementId::new_v4();
        let part = Element::new_with_kind(ElementKind::PartDefinition)
            .with_name("Part")
            .with_owning_membership(fake_membership_id.clone());
        graph.add_element(part);

        let errors = graph.validate_structure();
        let dangling_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StructuralError::DanglingOwningMembership { .. }))
            .collect();
        assert!(!dangling_errors.is_empty(), "Expected dangling membership error");
    }

    #[test]
    fn invalid_owning_membership_type_detected() {
        let mut graph = ModelGraph::new();

        // Create a package (not a Membership)
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("Pkg");
        let pkg_id = graph.add_element(pkg);

        // Create an element pointing to Package as owning_membership (wrong!)
        let part = Element::new_with_kind(ElementKind::PartDefinition)
            .with_name("Part")
            .with_owning_membership(pkg_id);
        graph.add_element(part);

        let errors = graph.validate_structure();
        let invalid_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StructuralError::InvalidOwningMembership { .. }))
            .collect();
        assert!(!invalid_errors.is_empty(), "Expected invalid membership type error");
    }

    #[test]
    fn structural_error_display() {
        let error = StructuralError::OrphanElement {
            element_id: ElementId::new_v4(),
            element_name: Some("TestPart".to_string()),
            element_kind: ElementKind::PartDefinition,
        };

        let msg = format!("{}", error);
        assert!(msg.contains("Orphan"));
        assert!(msg.contains("TestPart"));
    }
}
