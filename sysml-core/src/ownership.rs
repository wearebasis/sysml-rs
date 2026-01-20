//! Ownership operations for SysML v2 compliant model graphs.
//!
//! This module provides operations for establishing and querying ownership
//! relationships through Memberships, following the SysML v2 specification.
//!
//! ## Key Operations
//!
//! - `create_owning_membership`: Create an OwningMembership to establish ownership
//! - `add_owned_element`: Add an element with ownership in one call
//! - `owner_of`: Get the owner by following owning_membership
//! - `ancestors`: Get all ancestors (owner chain to root)
//! - `build_qualified_name`: Build qualified name from ownership chain

use crate::membership::{props as membership_props, MembershipBuilder};
use crate::{Element, ModelGraph, VisibilityKind};
use sysml_id::{ElementId, QualifiedName};

impl ModelGraph {
    /// Create an OwningMembership to establish ownership between a namespace and an element.
    ///
    /// This creates a new OwningMembership element that:
    /// - Has the namespace as its `membershipOwningNamespace`
    /// - Has the owned element as its `ownedMemberElement`
    /// - Sets the given visibility and optional member name
    ///
    /// The owned element's `owning_membership` and `owner` fields are updated automatically.
    ///
    /// # Arguments
    ///
    /// * `namespace_id` - The ID of the owning namespace element
    /// * `owned_element_id` - The ID of the element to be owned
    /// * `visibility` - The visibility of the member (public/private/protected)
    /// * `member_name` - Optional name for the member in the namespace
    ///
    /// # Returns
    ///
    /// The ElementId of the created OwningMembership
    ///
    /// # Panics
    ///
    /// Panics if the namespace_id or owned_element_id don't exist in the graph.
    pub fn create_owning_membership(
        &mut self,
        namespace_id: ElementId,
        owned_element_id: ElementId,
        visibility: VisibilityKind,
        member_name: Option<String>,
    ) -> ElementId {
        // Verify both elements exist
        assert!(
            self.elements.contains_key(&namespace_id),
            "Namespace element does not exist: {:?}",
            namespace_id
        );
        assert!(
            self.elements.contains_key(&owned_element_id),
            "Owned element does not exist: {:?}",
            owned_element_id
        );

        // Build the OwningMembership
        let mut builder = MembershipBuilder::owning()
            .owning_namespace(namespace_id.clone())
            .member_element(owned_element_id.clone())
            .visibility(visibility);

        if let Some(name) = member_name {
            builder = builder.member_name(name);
        }

        let membership = builder.build();
        let membership_id = membership.id.clone();

        // Add the membership to the graph
        self.add_element(membership);

        // Update the owned element's owning_membership and owner
        if let Some(owned) = self.elements.get_mut(&owned_element_id) {
            owned.owning_membership = Some(membership_id.clone());
            owned.owner = Some(namespace_id.clone());
        }

        // Update the namespace_to_memberships index
        self.namespace_to_memberships
            .entry(namespace_id)
            .or_default()
            .insert(membership_id.clone());

        // Update the element_to_owning_membership index
        self.element_to_owning_membership
            .insert(owned_element_id, membership_id.clone());

        membership_id
    }

    /// Add an element with ownership established through an OwningMembership.
    ///
    /// This is a convenience method that:
    /// 1. Adds the element to the graph
    /// 2. Creates an OwningMembership from the owner to the element
    ///
    /// If the element has a `name`, it's used as the `memberName` in the membership.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to add (should not already have owner set)
    /// * `owner_id` - The ID of the owning namespace element
    /// * `visibility` - The visibility of the member
    ///
    /// # Returns
    ///
    /// The ElementId of the added element
    pub fn add_owned_element(
        &mut self,
        element: Element,
        owner_id: ElementId,
        visibility: VisibilityKind,
    ) -> ElementId {
        let element_id = element.id.clone();
        let member_name = element.name.clone();

        // Add the element first
        self.add_element(element);

        // Create the owning membership
        self.create_owning_membership(owner_id, element_id.clone(), visibility, member_name);

        element_id
    }

    /// Get the owner of an element by following its owning_membership.
    ///
    /// This is the SysML v2 compliant way to get the owner - by dereferencing
    /// the element's `owning_membership` and then getting its `membershipOwningNamespace`.
    ///
    /// # Returns
    ///
    /// The owner Element, or None if the element has no owning membership.
    pub fn owner_of(&self, element_id: &ElementId) -> Option<&Element> {
        let element = self.elements.get(element_id)?;

        // First try the cached owner field
        if let Some(owner_id) = &element.owner {
            return self.elements.get(owner_id);
        }

        // Fall back to dereferencing owning_membership
        if let Some(membership_id) = &element.owning_membership {
            let membership = self.elements.get(membership_id)?;
            let namespace_id = membership.props.get(membership_props::MEMBERSHIP_OWNING_NAMESPACE)?.as_ref()?;
            return self.elements.get(namespace_id);
        }

        None
    }

    /// Get all ancestors of an element (owner chain to root).
    ///
    /// Returns ancestors in order from immediate parent to root.
    /// Does not include the element itself.
    ///
    /// # Example
    ///
    /// For a hierarchy `Package::SubPackage::Part`, calling `ancestors(&part_id)` returns
    /// `[SubPackage, Package]`.
    pub fn ancestors(&self, element_id: &ElementId) -> Vec<&Element> {
        let mut result = Vec::new();
        let mut current_id = element_id.clone();

        while let Some(owner) = self.owner_of(&current_id) {
            result.push(owner);
            current_id = owner.id.clone();
        }

        result
    }

    /// Build the qualified name for an element from its ownership chain.
    ///
    /// The qualified name is built by concatenating the names of all ancestors
    /// and the element itself, separated by `::`.
    ///
    /// # Returns
    ///
    /// The qualified name, or None if the element or any ancestor has no name.
    pub fn build_qualified_name(&self, element_id: &ElementId) -> Option<QualifiedName> {
        let element = self.elements.get(element_id)?;
        let element_name = element.name.as_ref()?;

        // Collect ancestor names (in reverse order, root to element)
        let ancestors = self.ancestors(element_id);
        let mut segments: Vec<String> = Vec::with_capacity(ancestors.len() + 1);

        // Ancestors are in order [parent, grandparent, ...], so reverse
        for ancestor in ancestors.into_iter().rev() {
            if let Some(name) = &ancestor.name {
                segments.push(name.clone());
            } else {
                // Ancestor has no name, can't build qualified name
                return None;
            }
        }

        // Add the element's own name
        segments.push(element_name.clone());

        Some(QualifiedName::from_segments(segments))
    }

    /// Get the owning membership element for an element.
    ///
    /// # Returns
    ///
    /// The OwningMembership element, or None if not found.
    pub fn owning_membership_of(&self, element_id: &ElementId) -> Option<&Element> {
        // First check the index
        if let Some(membership_id) = self.element_to_owning_membership.get(element_id) {
            return self.elements.get(membership_id);
        }

        // Fall back to the element's field
        let element = self.elements.get(element_id)?;
        let membership_id = element.owning_membership.as_ref()?;
        self.elements.get(membership_id)
    }

    /// Check if an element is a root (has no owner).
    pub fn is_root(&self, element_id: &ElementId) -> bool {
        self.owner_of(element_id).is_none()
    }

    /// Get the depth of an element in the ownership hierarchy.
    ///
    /// Root elements have depth 0. Returns None if element doesn't exist.
    pub fn depth_of(&self, element_id: &ElementId) -> Option<usize> {
        if !self.elements.contains_key(element_id) {
            return None;
        }
        Some(self.ancestors(element_id).len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Element, ElementKind, VisibilityKind};

    #[test]
    fn create_owning_membership_basic() {
        let mut graph = ModelGraph::new();

        // Create a package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPackage");
        let pkg_id = graph.add_element(pkg);

        // Create a part definition
        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("MyPart");
        let part_id = graph.add_element(part);

        // Create the owning membership
        let membership_id = graph.create_owning_membership(
            pkg_id.clone(),
            part_id.clone(),
            VisibilityKind::Public,
            Some("MyPart".to_string()),
        );

        // Verify the membership was created correctly
        let membership = graph.get_element(&membership_id).unwrap();
        assert_eq!(membership.kind, ElementKind::OwningMembership);

        // Verify the part's ownership was updated
        let part = graph.get_element(&part_id).unwrap();
        assert_eq!(part.owning_membership, Some(membership_id));
        assert_eq!(part.owner, Some(pkg_id.clone()));

        // Verify owner_of works
        let owner = graph.owner_of(&part_id).unwrap();
        assert_eq!(owner.id, pkg_id);
    }

    #[test]
    fn add_owned_element_convenience() {
        let mut graph = ModelGraph::new();

        // Create a package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPackage");
        let pkg_id = graph.add_element(pkg);

        // Add an owned part definition
        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("MyPart");
        let part_id = graph.add_owned_element(part, pkg_id.clone(), VisibilityKind::Public);

        // Verify ownership
        let part = graph.get_element(&part_id).unwrap();
        assert!(part.owning_membership.is_some());
        assert_eq!(part.owner, Some(pkg_id.clone()));

        // Verify we can get the owner
        let owner = graph.owner_of(&part_id).unwrap();
        assert_eq!(owner.id, pkg_id);
    }

    #[test]
    fn ancestors_chain() {
        let mut graph = ModelGraph::new();

        // Create Package1::Package2::Part
        let pkg1 = Element::new_with_kind(ElementKind::Package).with_name("Package1");
        let pkg1_id = graph.add_element(pkg1);

        let pkg2 = Element::new_with_kind(ElementKind::Package).with_name("Package2");
        let pkg2_id = graph.add_owned_element(pkg2, pkg1_id.clone(), VisibilityKind::Public);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let part_id = graph.add_owned_element(part, pkg2_id.clone(), VisibilityKind::Public);

        // Get ancestors of Part
        let ancestors = graph.ancestors(&part_id);
        assert_eq!(ancestors.len(), 2);
        assert_eq!(ancestors[0].id, pkg2_id); // Immediate parent
        assert_eq!(ancestors[1].id, pkg1_id); // Grandparent
    }

    #[test]
    fn build_qualified_name_simple() {
        let mut graph = ModelGraph::new();

        // Create Package1::Part
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("Package1");
        let pkg_id = graph.add_element(pkg);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("MyPart");
        let part_id = graph.add_owned_element(part, pkg_id.clone(), VisibilityKind::Public);

        let qname = graph.build_qualified_name(&part_id).unwrap();
        assert_eq!(qname.to_string(), "Package1::MyPart");
    }

    #[test]
    fn build_qualified_name_deep_hierarchy() {
        let mut graph = ModelGraph::new();

        // Create A::B::C::D
        let a = Element::new_with_kind(ElementKind::Package).with_name("A");
        let a_id = graph.add_element(a);

        let b = Element::new_with_kind(ElementKind::Package).with_name("B");
        let b_id = graph.add_owned_element(b, a_id.clone(), VisibilityKind::Public);

        let c = Element::new_with_kind(ElementKind::Package).with_name("C");
        let c_id = graph.add_owned_element(c, b_id.clone(), VisibilityKind::Public);

        let d = Element::new_with_kind(ElementKind::PartDefinition).with_name("D");
        let d_id = graph.add_owned_element(d, c_id.clone(), VisibilityKind::Public);

        let qname = graph.build_qualified_name(&d_id).unwrap();
        assert_eq!(qname.to_string(), "A::B::C::D");
    }

    #[test]
    fn build_qualified_name_missing_name_returns_none() {
        let mut graph = ModelGraph::new();

        // Package with no name
        let pkg = Element::new_with_kind(ElementKind::Package);
        let pkg_id = graph.add_element(pkg);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let part_id = graph.add_owned_element(part, pkg_id.clone(), VisibilityKind::Public);

        // Should return None because ancestor has no name
        assert!(graph.build_qualified_name(&part_id).is_none());
    }

    #[test]
    fn is_root_and_depth() {
        let mut graph = ModelGraph::new();

        let pkg = Element::new_with_kind(ElementKind::Package).with_name("Pkg");
        let pkg_id = graph.add_element(pkg);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let part_id = graph.add_owned_element(part, pkg_id.clone(), VisibilityKind::Public);

        assert!(graph.is_root(&pkg_id));
        assert!(!graph.is_root(&part_id));

        assert_eq!(graph.depth_of(&pkg_id), Some(0));
        assert_eq!(graph.depth_of(&part_id), Some(1));
    }
}
