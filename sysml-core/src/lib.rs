//! # sysml-core
//!
//! Core model types for SysML v2: Element, Relationship, and ModelGraph.
//!
//! This crate provides the fundamental data structures for representing
//! SysML v2 models in memory.
//!
//! ## Features
//!
//! - `serde`: Enable serde serialization support
//!
//! ## ElementKind
//!
//! The `ElementKind` enum is generated at build time from the official SysML v2
//! specification TTL vocabulary files. It contains all 266 element types defined
//! in the KerML and SysML specifications.
//!
//! ## Typed Property Accessors
//!
//! This crate also provides typed property accessors generated from OSLC shapes.
//! Use `element.as_part_usage()` to get a typed accessor for PartUsage properties.

use std::collections::{BTreeMap, HashMap, HashSet};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use sysml_id::{ElementId, QualifiedName};
pub use sysml_meta::Value;
pub use sysml_span::Span;

mod validation;
pub use validation::{ValidationError, ValidationErrorKind, ValidationResult};

// Membership-based ownership modules (SysML v2 compliant)
mod membership;
mod ownership;
mod namespace;
mod structural_validation;
mod factory;

pub use membership::{MembershipBuilder, MembershipView, OwningMembershipView};
pub use structural_validation::StructuralError;
pub use factory::ElementFactory;

// Include the generated ElementKind enum (with hierarchy, predicates, and relationship methods)
include!(concat!(env!("OUT_DIR"), "/element_kind.generated.rs"));

// Include the generated value enumeration types (FeatureDirectionKind, etc.)
include!(concat!(env!("OUT_DIR"), "/enums.generated.rs"));

// Include the generated property accessors
include!(concat!(env!("OUT_DIR"), "/properties.generated.rs"));

/// The kind of a relationship between elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum RelationshipKind {
    /// Ownership relationship (container -> contained).
    Owning,
    /// Type relationship (instance -> type).
    TypeOf,
    /// Satisfaction relationship (design -> requirement).
    Satisfy,
    /// Verification relationship (test -> requirement).
    Verify,
    /// Derivation relationship (derived -> source).
    Derive,
    /// Traceability relationship.
    Trace,
    /// Reference relationship.
    Reference,
    /// Specialization relationship (subtype -> supertype).
    Specialize,
    /// Redefinition relationship.
    Redefine,
    /// Subsetting relationship.
    Subsetting,
    /// Flow relationship.
    Flow,
    /// Transition relationship.
    Transition,
}

impl RelationshipKind {
    /// Get the string representation of this kind.
    pub fn as_str(&self) -> &str {
        match self {
            RelationshipKind::Owning => "Owning",
            RelationshipKind::TypeOf => "TypeOf",
            RelationshipKind::Satisfy => "Satisfy",
            RelationshipKind::Verify => "Verify",
            RelationshipKind::Derive => "Derive",
            RelationshipKind::Trace => "Trace",
            RelationshipKind::Reference => "Reference",
            RelationshipKind::Specialize => "Specialize",
            RelationshipKind::Redefine => "Redefine",
            RelationshipKind::Subsetting => "Subsetting",
            RelationshipKind::Flow => "Flow",
            RelationshipKind::Transition => "Transition",
        }
    }
}

impl Default for RelationshipKind {
    fn default() -> Self {
        RelationshipKind::Reference
    }
}

impl std::fmt::Display for RelationshipKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A model element.
///
/// ## Ownership Model (SysML v2 Compliant)
///
/// In SysML v2, ownership is established through Membership elements:
/// - `owning_membership` points to the OwningMembership element that owns this element
/// - `owner` is derived from `owning_membership.membershipOwningNamespace`
///
/// For backward compatibility, you can set `owner` directly and an implicit
/// OwningMembership will be created when added to a ModelGraph.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Element {
    /// Unique identifier for this element.
    pub id: ElementId,
    /// The kind of this element.
    pub kind: ElementKind,
    /// The name of this element (optional).
    pub name: Option<String>,
    /// The OwningMembership that owns this element (SysML v2 canonical ownership).
    /// This points to a Membership element, not directly to the owning namespace.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub owning_membership: Option<ElementId>,
    /// The owning element (cached/derived from owning_membership).
    /// This is a convenience field derived from `owning_membership.membershipOwningNamespace`.
    pub owner: Option<ElementId>,
    /// The qualified name of this element (optional, computed).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub qname: Option<QualifiedName>,
    /// Additional properties.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub props: BTreeMap<String, Value>,
    /// Source locations for this element.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub spans: Vec<Span>,
}

impl Element {
    /// Create a new element with the given id and kind.
    pub fn new(id: ElementId, kind: ElementKind) -> Self {
        Element {
            id,
            kind,
            name: None,
            owning_membership: None,
            owner: None,
            qname: None,
            props: BTreeMap::new(),
            spans: Vec::new(),
        }
    }

    /// Create a new element with a generated id.
    pub fn new_with_kind(kind: ElementKind) -> Self {
        Element::new(ElementId::new_v4(), kind)
    }

    /// Set the name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the owner.
    pub fn with_owner(mut self, owner: ElementId) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Set the owning membership (SysML v2 canonical ownership).
    ///
    /// This sets the OwningMembership element that owns this element.
    /// Note: You typically don't need to call this directly - use
    /// `ModelGraph::add_owned_element()` which creates the membership for you.
    pub fn with_owning_membership(mut self, membership_id: ElementId) -> Self {
        self.owning_membership = Some(membership_id);
        self
    }

    /// Set the qualified name.
    pub fn with_qname(mut self, qname: QualifiedName) -> Self {
        self.qname = Some(qname);
        self
    }

    /// Add a property.
    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }

    /// Add a span.
    pub fn with_span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    /// Get a property value.
    pub fn get_prop(&self, key: &str) -> Option<&Value> {
        self.props.get(key)
    }

    /// Set a property value.
    pub fn set_prop(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.props.insert(key.into(), value.into());
    }
}

/// A relationship between two elements.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Relationship {
    /// Unique identifier for this relationship.
    pub id: ElementId,
    /// The kind of this relationship.
    pub kind: RelationshipKind,
    /// The source element.
    pub source: ElementId,
    /// The target element.
    pub target: ElementId,
    /// Additional properties.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub props: BTreeMap<String, Value>,
}

impl Relationship {
    /// Create a new relationship.
    pub fn new(kind: RelationshipKind, source: ElementId, target: ElementId) -> Self {
        Relationship {
            id: ElementId::new_v4(),
            kind,
            source,
            target,
            props: BTreeMap::new(),
        }
    }

    /// Create a relationship with a specific id.
    pub fn with_id(id: ElementId, kind: RelationshipKind, source: ElementId, target: ElementId) -> Self {
        Relationship {
            id,
            kind,
            source,
            target,
            props: BTreeMap::new(),
        }
    }

    /// Add a property.
    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }
}

/// A graph of model elements and relationships.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModelGraph {
    /// All elements in the graph, keyed by id.
    pub elements: BTreeMap<ElementId, Element>,
    /// All relationships in the graph, keyed by id.
    pub relationships: BTreeMap<ElementId, Relationship>,

    // Indexes (built lazily, not serialized)
    #[cfg_attr(feature = "serde", serde(skip))]
    owner_to_children: HashMap<ElementId, HashSet<ElementId>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    source_to_rels: HashMap<ElementId, HashSet<ElementId>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    target_to_rels: HashMap<ElementId, HashSet<ElementId>>,

    // NEW: Membership-based ownership indexes
    /// Maps namespace ID to its membership element IDs.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) namespace_to_memberships: HashMap<ElementId, HashSet<ElementId>>,
    /// Maps element ID to its owning membership element ID.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) element_to_owning_membership: HashMap<ElementId, ElementId>,

    #[cfg_attr(feature = "serde", serde(skip))]
    indexes_dirty: bool,
}

impl ModelGraph {
    /// Create a new empty model graph.
    pub fn new() -> Self {
        ModelGraph {
            elements: BTreeMap::new(),
            relationships: BTreeMap::new(),
            owner_to_children: HashMap::new(),
            source_to_rels: HashMap::new(),
            target_to_rels: HashMap::new(),
            namespace_to_memberships: HashMap::new(),
            element_to_owning_membership: HashMap::new(),
            indexes_dirty: false,
        }
    }

    /// Add an element to the graph.
    pub fn add_element(&mut self, element: Element) -> ElementId {
        let id = element.id.clone();

        // Update owner index
        if let Some(owner) = &element.owner {
            self.owner_to_children
                .entry(owner.clone())
                .or_default()
                .insert(id.clone());
        }

        self.elements.insert(id.clone(), element);
        id
    }

    /// Add a relationship to the graph.
    pub fn add_relationship(&mut self, relationship: Relationship) -> ElementId {
        let id = relationship.id.clone();

        // Update source index
        self.source_to_rels
            .entry(relationship.source.clone())
            .or_default()
            .insert(id.clone());

        // Update target index
        self.target_to_rels
            .entry(relationship.target.clone())
            .or_default()
            .insert(id.clone());

        self.relationships.insert(id.clone(), relationship);
        id
    }

    /// Get an element by id.
    pub fn get_element(&self, id: &ElementId) -> Option<&Element> {
        self.elements.get(id)
    }

    /// Get a mutable element by id.
    pub fn get_element_mut(&mut self, id: &ElementId) -> Option<&mut Element> {
        self.elements.get_mut(id)
    }

    /// Get a relationship by id.
    pub fn get_relationship(&self, id: &ElementId) -> Option<&Relationship> {
        self.relationships.get(id)
    }

    /// Get the children of an owner element.
    pub fn children_of(&self, owner: &ElementId) -> impl Iterator<Item = &Element> {
        self.owner_to_children
            .get(owner)
            .into_iter()
            .flat_map(|children| children.iter())
            .filter_map(move |id| self.elements.get(id))
    }

    /// Get outgoing relationships from a source element.
    pub fn outgoing(&self, source: &ElementId) -> impl Iterator<Item = &Relationship> {
        self.source_to_rels
            .get(source)
            .into_iter()
            .flat_map(|rels| rels.iter())
            .filter_map(move |id| self.relationships.get(id))
    }

    /// Get incoming relationships to a target element.
    pub fn incoming(&self, target: &ElementId) -> impl Iterator<Item = &Relationship> {
        self.target_to_rels
            .get(target)
            .into_iter()
            .flat_map(|rels| rels.iter())
            .filter_map(move |id| self.relationships.get(id))
    }

    /// Get all elements of a specific kind.
    pub fn elements_by_kind<'a>(&'a self, kind: &'a ElementKind) -> impl Iterator<Item = &'a Element> {
        self.elements.values().filter(move |e| &e.kind == kind)
    }

    /// Get all relationships of a specific kind.
    pub fn relationships_by_kind<'a>(&'a self, kind: &'a RelationshipKind) -> impl Iterator<Item = &'a Relationship> {
        self.relationships.values().filter(move |r| &r.kind == kind)
    }

    /// Get all root elements (elements without an owner).
    pub fn roots(&self) -> impl Iterator<Item = &Element> {
        self.elements.values().filter(|e| e.owner.is_none())
    }

    /// Get the number of elements.
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Get the number of relationships.
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    /// Check if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty() && self.relationships.is_empty()
    }

    /// Rebuild indexes after deserialization.
    pub fn rebuild_indexes(&mut self) {
        self.owner_to_children.clear();
        self.source_to_rels.clear();
        self.target_to_rels.clear();
        self.namespace_to_memberships.clear();
        self.element_to_owning_membership.clear();

        for (id, element) in &self.elements {
            if let Some(owner) = &element.owner {
                self.owner_to_children
                    .entry(owner.clone())
                    .or_default()
                    .insert(id.clone());
            }

            // Rebuild owning_membership index
            if let Some(membership_id) = &element.owning_membership {
                self.element_to_owning_membership
                    .insert(id.clone(), membership_id.clone());
            }
        }

        // Rebuild namespace_to_memberships index from Membership elements
        for (id, element) in &self.elements {
            // Check if this is a Membership element
            if element.kind == ElementKind::Membership
                || element.kind.is_subtype_of(ElementKind::Membership)
            {
                // Get the membershipOwningNamespace from props
                if let Some(ns_ref) = element.props.get("membershipOwningNamespace") {
                    if let Some(ns_id) = ns_ref.as_ref() {
                        self.namespace_to_memberships
                            .entry(ns_id.clone())
                            .or_default()
                            .insert(id.clone());
                    }
                }
            }
        }

        for (id, rel) in &self.relationships {
            self.source_to_rels
                .entry(rel.source.clone())
                .or_default()
                .insert(id.clone());
            self.target_to_rels
                .entry(rel.target.clone())
                .or_default()
                .insert(id.clone());
        }

        self.indexes_dirty = false;
    }

    /// Clear the graph.
    pub fn clear(&mut self) {
        self.elements.clear();
        self.relationships.clear();
        self.owner_to_children.clear();
        self.source_to_rels.clear();
        self.target_to_rels.clear();
        self.namespace_to_memberships.clear();
        self.element_to_owning_membership.clear();
        self.indexes_dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> ModelGraph {
        let mut graph = ModelGraph::new();

        // Create a package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPackage");
        let pkg_id = graph.add_element(pkg);

        // Create a part usage owned by the package
        let part = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("TestPart")
            .with_owner(pkg_id.clone());
        let part_id = graph.add_element(part);

        // Create a requirement usage
        let req = Element::new_with_kind(ElementKind::RequirementUsage)
            .with_name("TestReq")
            .with_owner(pkg_id.clone());
        let req_id = graph.add_element(req);

        // Create a satisfy relationship
        let satisfy = Relationship::new(RelationshipKind::Satisfy, part_id, req_id);
        graph.add_relationship(satisfy);

        graph
    }

    #[test]
    fn add_and_get_element() {
        let mut graph = ModelGraph::new();
        let element = Element::new_with_kind(ElementKind::PartUsage).with_name("MyPart");
        let id = element.id.clone();
        graph.add_element(element);

        let retrieved = graph.get_element(&id).unwrap();
        assert_eq!(retrieved.name, Some("MyPart".to_string()));
    }

    #[test]
    fn add_and_get_relationship() {
        let mut graph = ModelGraph::new();
        let e1 = Element::new_with_kind(ElementKind::PartUsage);
        let e2 = Element::new_with_kind(ElementKind::RequirementUsage);
        let id1 = graph.add_element(e1);
        let id2 = graph.add_element(e2);

        let rel = Relationship::new(RelationshipKind::Satisfy, id1.clone(), id2.clone());
        let rel_id = rel.id.clone();
        graph.add_relationship(rel);

        let retrieved = graph.get_relationship(&rel_id).unwrap();
        assert_eq!(retrieved.source, id1);
        assert_eq!(retrieved.target, id2);
    }

    #[test]
    fn children_of() {
        let graph = create_test_graph();
        let pkg = graph.elements_by_kind(&ElementKind::Package).next().unwrap();
        let children: Vec<_> = graph.children_of(&pkg.id).collect();
        assert_eq!(children.len(), 2); // PartUsage and RequirementUsage
    }

    #[test]
    fn outgoing_relationships() {
        let graph = create_test_graph();
        let part = graph.elements_by_kind(&ElementKind::PartUsage).next().unwrap();
        let outgoing: Vec<_> = graph.outgoing(&part.id).collect();
        assert_eq!(outgoing.len(), 1);
        assert!(matches!(outgoing[0].kind, RelationshipKind::Satisfy));
    }

    #[test]
    fn elements_by_kind() {
        let graph = create_test_graph();
        let packages: Vec<_> = graph.elements_by_kind(&ElementKind::Package).collect();
        assert_eq!(packages.len(), 1);

        let parts: Vec<_> = graph.elements_by_kind(&ElementKind::PartUsage).collect();
        assert_eq!(parts.len(), 1);
    }

    #[test]
    fn roots() {
        let graph = create_test_graph();
        let roots: Vec<_> = graph.roots().collect();
        assert_eq!(roots.len(), 1);
        assert!(matches!(roots[0].kind, ElementKind::Package));
    }

    #[test]
    fn element_with_props() {
        let element = Element::new_with_kind(ElementKind::RequirementUsage)
            .with_name("Req1")
            .with_prop("priority", 1i64)
            .with_prop("verified", false);

        assert_eq!(element.get_prop("priority").and_then(|v| v.as_int()), Some(1));
        assert_eq!(element.get_prop("verified").and_then(|v| v.as_bool()), Some(false));
    }

    #[test]
    fn graph_counts() {
        let graph = create_test_graph();
        assert_eq!(graph.element_count(), 3);
        assert_eq!(graph.relationship_count(), 1);
        assert!(!graph.is_empty());
    }

    #[test]
    fn element_kind_from_str() {
        assert_eq!(ElementKind::from_str("Package"), Some(ElementKind::Package));
        assert_eq!(ElementKind::from_str("PartUsage"), Some(ElementKind::PartUsage));
        assert_eq!(ElementKind::from_str("InvalidType"), None);
    }

    #[test]
    fn element_kind_has_all_types() {
        // Verify the enum has the expected number of types
        // The count is the unique types after deduplication between KerML and SysML
        let count = ElementKind::count();
        // At least 150 unique types (some are duplicated between KerML and SysML)
        assert!(count >= 150, "Expected at least 150 types, got {}", count);
        assert!(count <= 300, "Expected at most 300 types, got {}", count);
    }

    #[test]
    fn element_kind_iter() {
        let count = ElementKind::iter().count();
        assert_eq!(count, ElementKind::count());
    }

    #[test]
    fn typed_property_accessor_cast() {
        // Test that we can cast an element to a typed accessor
        let element = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("TestPart")
            .with_prop("isVariation", false)
            .with_prop("isComposite", true);

        // Cast to PartUsageProps
        let part_props = element.as_part_usage();
        assert!(part_props.is_some());
        let part = part_props.unwrap();

        // Access underlying element
        assert_eq!(part.element().name, Some("TestPart".to_string()));
        assert_eq!(part.element().kind, ElementKind::PartUsage);
    }

    #[test]
    fn typed_property_accessor_wrong_kind() {
        // Test that casting fails for wrong element kind
        let element = Element::new_with_kind(ElementKind::Package);

        // Should not cast to PartUsageProps
        assert!(element.as_part_usage().is_none());
        // Should cast to PackageProps
        assert!(element.as_package().is_some());
    }

    #[test]
    fn property_accessor_validation() {
        // Test validation on a typed accessor
        let element = Element::new_with_kind(ElementKind::PartUsage);

        let part = element.as_part_usage().unwrap();
        let result = part.validate();

        // Validation runs without panicking
        // There may be missing required properties, but the point is it doesn't panic
        let _ = result.error_count();
    }

    // === Tests for Phase 0c: Type Hierarchy & Enumerations ===

    #[test]
    fn test_supertypes() {
        let supertypes = ElementKind::PartUsage.supertypes();
        assert!(supertypes.contains(&ElementKind::ItemUsage));
        assert!(supertypes.contains(&ElementKind::Usage));
        assert!(supertypes.contains(&ElementKind::Feature));
        assert!(supertypes.contains(&ElementKind::Type));
        assert!(supertypes.contains(&ElementKind::Element));
        // Should not contain itself or unrelated types
        assert!(!supertypes.contains(&ElementKind::PartUsage));
        assert!(!supertypes.contains(&ElementKind::Relationship));
    }

    #[test]
    fn test_direct_supertypes() {
        // PartUsage's direct supertype should be ItemUsage
        let direct = ElementKind::PartUsage.direct_supertypes();
        assert!(direct.contains(&ElementKind::ItemUsage));
        // Should not include transitive supertypes
        assert!(!direct.contains(&ElementKind::Element));
    }

    #[test]
    fn test_is_subtype_of() {
        assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Feature));
        assert!(ElementKind::PartUsage.is_subtype_of(ElementKind::Element));
        assert!(ElementKind::Feature.is_subtype_of(ElementKind::Type));
        // A type is not a subtype of itself
        assert!(!ElementKind::Feature.is_subtype_of(ElementKind::Feature));
        // Element is the root, not a subtype of anything
        assert!(!ElementKind::Element.is_subtype_of(ElementKind::Feature));
    }

    #[test]
    fn test_is_definition_predicate() {
        assert!(ElementKind::PartDefinition.is_definition());
        assert!(ElementKind::ActionDefinition.is_definition());
        assert!(!ElementKind::PartUsage.is_definition());
        assert!(!ElementKind::Element.is_definition());
    }

    #[test]
    fn test_is_usage_predicate() {
        assert!(ElementKind::PartUsage.is_usage());
        assert!(ElementKind::ActionUsage.is_usage());
        assert!(!ElementKind::PartDefinition.is_usage());
        assert!(!ElementKind::Element.is_usage());
    }

    #[test]
    fn test_is_relationship_predicate() {
        assert!(ElementKind::Relationship.is_relationship());
        assert!(ElementKind::Specialization.is_relationship());
        assert!(ElementKind::FeatureTyping.is_relationship());
        assert!(!ElementKind::Element.is_relationship());
        assert!(!ElementKind::PartUsage.is_relationship());
    }

    #[test]
    fn test_is_classifier_predicate() {
        assert!(ElementKind::Classifier.is_classifier());
        assert!(ElementKind::Class.is_classifier());
        assert!(!ElementKind::Element.is_classifier());
        assert!(!ElementKind::Feature.is_classifier());
    }

    #[test]
    fn test_is_feature_predicate() {
        assert!(ElementKind::Feature.is_feature());
        assert!(ElementKind::PartUsage.is_feature());
        assert!(ElementKind::Connector.is_feature());
        assert!(!ElementKind::Element.is_feature());
        assert!(!ElementKind::Relationship.is_feature());
    }

    #[test]
    fn test_corresponding_usage() {
        assert_eq!(
            ElementKind::PartDefinition.corresponding_usage(),
            Some(ElementKind::PartUsage)
        );
        assert_eq!(
            ElementKind::ActionDefinition.corresponding_usage(),
            Some(ElementKind::ActionUsage)
        );
        assert_eq!(ElementKind::Element.corresponding_usage(), None);
        assert_eq!(ElementKind::PartUsage.corresponding_usage(), None);
    }

    #[test]
    fn test_corresponding_definition() {
        assert_eq!(
            ElementKind::PartUsage.corresponding_definition(),
            Some(ElementKind::PartDefinition)
        );
        assert_eq!(
            ElementKind::ActionUsage.corresponding_definition(),
            Some(ElementKind::ActionDefinition)
        );
        assert_eq!(ElementKind::Element.corresponding_definition(), None);
        assert_eq!(ElementKind::PartDefinition.corresponding_definition(), None);
    }

    #[test]
    fn test_relationship_source_type() {
        assert_eq!(
            ElementKind::FeatureTyping.relationship_source_type(),
            Some(ElementKind::Feature)
        );
        assert_eq!(
            ElementKind::Specialization.relationship_source_type(),
            Some(ElementKind::Type)
        );
        assert_eq!(
            ElementKind::Relationship.relationship_source_type(),
            Some(ElementKind::Element)
        );
        // Non-relationships return None
        assert_eq!(ElementKind::Element.relationship_source_type(), None);
        assert_eq!(ElementKind::PartUsage.relationship_source_type(), None);
    }

    #[test]
    fn test_relationship_target_type() {
        assert_eq!(
            ElementKind::FeatureTyping.relationship_target_type(),
            Some(ElementKind::Type)
        );
        assert_eq!(
            ElementKind::Subsetting.relationship_target_type(),
            Some(ElementKind::Feature)
        );
        // Non-relationships return None
        assert_eq!(ElementKind::Element.relationship_target_type(), None);
    }

    #[test]
    fn test_feature_direction_kind() {
        assert_eq!(FeatureDirectionKind::In.as_str(), "in");
        assert_eq!(FeatureDirectionKind::Out.as_str(), "out");
        assert_eq!(FeatureDirectionKind::Inout.as_str(), "inout");

        assert_eq!(FeatureDirectionKind::from_str("in"), Some(FeatureDirectionKind::In));
        assert_eq!(FeatureDirectionKind::from_str("out"), Some(FeatureDirectionKind::Out));
        assert_eq!(FeatureDirectionKind::from_str("invalid"), None);

        assert_eq!(FeatureDirectionKind::count(), 3);
        assert_eq!(FeatureDirectionKind::iter().count(), 3);
    }

    #[test]
    fn test_visibility_kind() {
        assert_eq!(VisibilityKind::Public.as_str(), "public");
        assert_eq!(VisibilityKind::Private.as_str(), "private");
        assert_eq!(VisibilityKind::Protected.as_str(), "protected");

        assert_eq!(VisibilityKind::from_str("public"), Some(VisibilityKind::Public));
        assert_eq!(VisibilityKind::count(), 3);
    }

    #[test]
    fn test_state_subaction_kind() {
        // "do" is a reserved keyword, so the variant is Do_
        assert_eq!(StateSubactionKind::Entry.as_str(), "entry");
        assert_eq!(StateSubactionKind::Do_.as_str(), "do");
        assert_eq!(StateSubactionKind::Exit.as_str(), "exit");

        assert_eq!(StateSubactionKind::from_str("do"), Some(StateSubactionKind::Do_));
        assert_eq!(StateSubactionKind::count(), 3);
    }

    #[test]
    fn test_all_value_enums_exist() {
        // Verify all 7 value enums are available
        let _ = FeatureDirectionKind::default();
        let _ = VisibilityKind::default();
        let _ = PortionKind::default();
        let _ = RequirementConstraintKind::default();
        let _ = StateSubactionKind::default();
        let _ = TransitionFeatureKind::default();
        let _ = TriggerKind::default();
    }
}
