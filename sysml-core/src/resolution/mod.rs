//! Name resolution for SysML v2 model graphs.
//!
//! This module implements name resolution following the SysML v2/KerML scoping rules.
//! Resolution follows this precedence order:
//!
//! 1. **OWNED**: Local owned memberships of the namespace
//! 2. **INHERITED**: Members via Specialization chain (for Types)
//! 3. **IMPORTED**: Members from Import statements
//! 4. **PARENT**: Walk up to parent namespace
//! 5. **GLOBAL**: Root packages
//! 6. **LIBRARY**: Standard library package members (implicit)
//!
//! ## Usage
//!
//! ```ignore
//! use sysml_core::resolution::{ResolutionContext, resolve_name_in_scope};
//!
//! let mut ctx = ResolutionContext::new(&graph);
//! let result = ctx.resolve_name(&namespace_id, "SomeName");
//! ```
//!
//! ## Standard Library Support
//!
//! Library packages are registered with `ModelGraph::register_library_package()` and
//! are available globally during name resolution without requiring explicit imports.
//! This enables resolution of standard library types like `Anything`, `DataValue`, etc.
//!
//! ```ignore
//! // Register a library package
//! graph.register_library_package(base_pkg_id);
//!
//! // Now "Anything" can be resolved from any namespace
//! let resolved = ctx.resolve_name(&user_ns_id, "Anything");
//! ```
//!
//! ## Unresolved References
//!
//! The parser stores unresolved references as properties like:
//! - `unresolved_general` - Specialization supertype
//! - `unresolved_type` - FeatureTyping type
//! - `unresolved_subsettedFeature` - Subsetting
//! - `unresolved_redefinedFeature` - Redefinition
//! - `unresolved_referencedFeature` - ReferenceSubsetting
//! - `unresolved_sources/targets` - Dependency
//!
//! The resolution pass converts these to resolved `ElementId` references.
//!
//! ## Scoping Strategies
//!
//! Different cross-references require different scoping strategies:
//! - **OwningNamespace**: Most common, walks up the namespace hierarchy
//! - **NonExpressionNamespace**: Skips expression scopes (for FeatureTyping)
//! - **RelativeNamespace**: Relative to a specific element
//! - **FeatureChaining**: For feature chain expressions
//! - **TransitionSpecific**: For state machine transitions
//! - **Global**: For imports and root-level references
//!
//! See the `scoping` submodule for implementations.
//!
//! ## Resolution Tracing
//!
//! Enable detailed resolution tracing by compiling with the `resolution-tracing` feature:
//!
//! ```bash
//! cargo build --features resolution-tracing
//! ```
//!
//! This will print detailed information about each resolution step.

pub mod scoping;

// Resolution tracing macros - enabled with the `resolution-tracing` feature
#[cfg(feature = "resolution-tracing")]
macro_rules! res_trace {
    ($($arg:tt)*) => {
        eprintln!("[RESOLVE] {}", format!($($arg)*));
    };
}

#[cfg(not(feature = "resolution-tracing"))]
macro_rules! res_trace {
    ($($arg:tt)*) => {};
}

// Allow the macro to be used in submodules
pub(crate) use res_trace;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use sysml_id::ElementId;
use sysml_span::{Diagnostic, Diagnostics};

use crate::membership::MembershipView;
use crate::{ElementKind, ModelGraph, VisibilityKind};

/// Property keys for unresolved references (as stored by parser).
pub mod unresolved_props {
    /// Unresolved supertype in Specialization.
    pub const GENERAL: &str = "unresolved_general";
    /// Unresolved type in FeatureTyping.
    pub const TYPE: &str = "unresolved_type";
    /// Unresolved subsetted feature in Subsetting.
    pub const SUBSETTED_FEATURE: &str = "unresolved_subsettedFeature";
    /// Unresolved redefined feature in Redefinition.
    pub const REDEFINED_FEATURE: &str = "unresolved_redefinedFeature";
    /// Unresolved referenced feature in ReferenceSubsetting.
    pub const REFERENCED_FEATURE: &str = "unresolved_referencedFeature";
    /// Unresolved sources in Dependency.
    pub const SOURCES: &str = "unresolved_sources";
    /// Unresolved targets in Dependency.
    pub const TARGETS: &str = "unresolved_targets";
    /// Unresolved value expression.
    pub const VALUE: &str = "unresolved_value";

    // === Phase B: Additional cross-references ===

    /// Unresolved superclassifier in Subclassification.
    pub const SUPERCLASSIFIER: &str = "unresolved_superclassifier";
    /// Unresolved conjugated type in Conjugation.
    pub const CONJUGATED_TYPE: &str = "unresolved_conjugatedType";
    /// Unresolved original type in Conjugation.
    pub const ORIGINAL_TYPE: &str = "unresolved_originalType";
    /// Unresolved featuring type in TypeFeaturing.
    pub const FEATURING_TYPE: &str = "unresolved_featuringType";
    /// Unresolved disjoining type in Disjoining.
    pub const DISJOINING_TYPE: &str = "unresolved_disjoiningType";
    /// Unresolved unioning type in Unioning.
    pub const UNIONING_TYPE: &str = "unresolved_unioningType";
    /// Unresolved intersecting type in Intersecting.
    pub const INTERSECTING_TYPE: &str = "unresolved_intersectingType";
    /// Unresolved differencing type in Differencing.
    pub const DIFFERENCING_TYPE: &str = "unresolved_differencingType";
    /// Unresolved inverting feature in FeatureInverting.
    pub const INVERTING_FEATURE: &str = "unresolved_invertingFeature";
    /// Unresolved crossed feature in FeatureChaining.
    pub const CROSSED_FEATURE: &str = "unresolved_crossedFeature";
    /// Unresolved annotated element in Annotation.
    pub const ANNOTATED_ELEMENT: &str = "unresolved_annotatedElement";
    /// Unresolved member element in Membership.
    pub const MEMBER_ELEMENT: &str = "unresolved_memberElement";
    /// Unresolved client in Dependency.
    pub const CLIENT: &str = "unresolved_client";
    /// Unresolved supplier in Dependency.
    pub const SUPPLIER: &str = "unresolved_supplier";
    /// Unresolved conjugated port definition in ConjugatedPortDefinition.
    pub const CONJUGATED_PORT_DEFINITION: &str = "unresolved_conjugatedPortDefinition";
}

/// Property keys for Import elements (as stored by parser).
pub mod import_props {
    /// The qualified name of the imported reference.
    pub const IMPORTED_REFERENCE: &str = "importedReference";
    /// Whether this is a namespace import (::*).
    pub const IS_NAMESPACE: &str = "isNamespace";
    /// Whether this is a recursive import (::**).
    pub const IS_RECURSIVE: &str = "isRecursive";
    /// Whether 'all' keyword was used.
    pub const IMPORTS_ALL: &str = "importsAll";
}

/// Static map of primitive type aliases to their canonical names.
///
/// These are common shorthand aliases used in SysML that should resolve
/// to their canonical library types.
fn primitive_type_alias(name: &str) -> Option<&'static str> {
    match name {
        "float" => Some("Real"),
        "int" => Some("Integer"),
        _ => None,
    }
}

/// Well-known standard library package names.
///
/// These are the package names defined in the SysML v2 standard library.
/// Use these with `ModelGraph::register_library_package()` to enable
/// automatic resolution of library types.
pub mod library_packages {
    // === KerML Kernel Libraries ===
    /// Base types: Anything, DataValue, things, dataValues.
    pub const BASE: &str = "Base";
    /// Core element links.
    pub const LINKS: &str = "Links";
    /// Occurrence types.
    pub const OCCURRENCES: &str = "Occurrences";
    /// Object types.
    pub const OBJECTS: &str = "Objects";
    /// Performance types.
    pub const PERFORMANCES: &str = "Performances";
    /// Transfer types.
    pub const TRANSFERS: &str = "Transfers";
    /// Control performances.
    pub const CONTROL_PERFORMANCES: &str = "ControlPerformances";
    /// Transition performances.
    pub const TRANSITION_PERFORMANCES: &str = "TransitionPerformances";
    /// State performances.
    pub const STATE_PERFORMANCES: &str = "StatePerformances";
    /// Triggers.
    pub const TRIGGERS: &str = "Triggers";
    /// Scalar values: Boolean, String, Integer, Real, Complex, etc.
    pub const SCALAR_VALUES: &str = "ScalarValues";
    /// Vector values.
    pub const VECTOR_VALUES: &str = "VectorValues";
    /// Collections.
    pub const COLLECTIONS: &str = "Collections";
    /// Clocks.
    pub const CLOCKS: &str = "Clocks";
    /// Spatial frames.
    pub const SPATIAL_FRAMES: &str = "SpatialFrames";
    /// Observation.
    pub const OBSERVATION: &str = "Observation";
    /// Metaobjects.
    pub const METAOBJECTS: &str = "Metaobjects";
    /// KerML top-level library.
    pub const KERML: &str = "KerML";

    // === SysML Systems Libraries ===
    /// SysML top-level library.
    pub const SYSML: &str = "SysML";
    /// Items library.
    pub const ITEMS: &str = "Items";
    /// Parts library.
    pub const PARTS: &str = "Parts";
    /// Ports library.
    pub const PORTS: &str = "Ports";
    /// Actions library.
    pub const ACTIONS: &str = "Actions";
    /// States library.
    pub const STATES: &str = "States";
    /// Connections library.
    pub const CONNECTIONS: &str = "Connections";
    /// Interfaces library.
    pub const INTERFACES: &str = "Interfaces";
    /// Allocations library.
    pub const ALLOCATIONS: &str = "Allocations";
    /// Flows library.
    pub const FLOWS: &str = "Flows";
    /// Attributes library.
    pub const ATTRIBUTES: &str = "Attributes";
    /// Calculations library.
    pub const CALCULATIONS: &str = "Calculations";
    /// Constraints library.
    pub const CONSTRAINTS: &str = "Constraints";
    /// Requirements library.
    pub const REQUIREMENTS: &str = "Requirements";
    /// Cases library.
    pub const CASES: &str = "Cases";
    /// Analysis cases library.
    pub const ANALYSIS_CASES: &str = "AnalysisCases";
    /// Verification cases library.
    pub const VERIFICATION_CASES: &str = "VerificationCases";
    /// Use cases library.
    pub const USE_CASES: &str = "UseCases";
    /// Views library.
    pub const VIEWS: &str = "Views";
    /// Metadata library.
    pub const METADATA: &str = "Metadata";

    /// All KerML kernel library package names.
    pub const KERML_PACKAGES: &[&str] = &[
        BASE,
        LINKS,
        OCCURRENCES,
        OBJECTS,
        PERFORMANCES,
        TRANSFERS,
        CONTROL_PERFORMANCES,
        TRANSITION_PERFORMANCES,
        STATE_PERFORMANCES,
        TRIGGERS,
        SCALAR_VALUES,
        VECTOR_VALUES,
        COLLECTIONS,
        CLOCKS,
        SPATIAL_FRAMES,
        OBSERVATION,
        METAOBJECTS,
        KERML,
    ];

    /// All SysML systems library package names.
    pub const SYSML_PACKAGES: &[&str] = &[
        SYSML,
        ITEMS,
        PARTS,
        PORTS,
        ACTIONS,
        STATES,
        CONNECTIONS,
        INTERFACES,
        ALLOCATIONS,
        FLOWS,
        ATTRIBUTES,
        CALCULATIONS,
        CONSTRAINTS,
        REQUIREMENTS,
        CASES,
        ANALYSIS_CASES,
        VERIFICATION_CASES,
        USE_CASES,
        VIEWS,
        METADATA,
    ];

    /// All standard library package names (KerML + SysML).
    pub const ALL_PACKAGES: &[&str] = &[
        // KerML
        BASE,
        LINKS,
        OCCURRENCES,
        OBJECTS,
        PERFORMANCES,
        TRANSFERS,
        CONTROL_PERFORMANCES,
        TRANSITION_PERFORMANCES,
        STATE_PERFORMANCES,
        TRIGGERS,
        SCALAR_VALUES,
        VECTOR_VALUES,
        COLLECTIONS,
        CLOCKS,
        SPATIAL_FRAMES,
        OBSERVATION,
        METAOBJECTS,
        KERML,
        // SysML
        SYSML,
        ITEMS,
        PARTS,
        PORTS,
        ACTIONS,
        STATES,
        CONNECTIONS,
        INTERFACES,
        ALLOCATIONS,
        FLOWS,
        ATTRIBUTES,
        CALCULATIONS,
        CONSTRAINTS,
        REQUIREMENTS,
        CASES,
        ANALYSIS_CASES,
        VERIFICATION_CASES,
        USE_CASES,
        VIEWS,
        METADATA,
    ];
}

/// Property keys for resolved references.
pub mod resolved_props {
    /// Resolved supertype in Specialization.
    pub const GENERAL: &str = "general";
    /// Resolved type in FeatureTyping.
    pub const TYPE: &str = "type";
    /// Resolved subsetted feature in Subsetting.
    pub const SUBSETTED_FEATURE: &str = "subsettedFeature";
    /// Resolved redefined feature in Redefinition.
    pub const REDEFINED_FEATURE: &str = "redefinedFeature";
    /// Resolved referenced feature in ReferenceSubsetting.
    pub const REFERENCED_FEATURE: &str = "referencedFeature";
    /// Resolved sources in Dependency.
    pub const SOURCES: &str = "source";
    /// Resolved targets in Dependency.
    pub const TARGETS: &str = "target";

    // === Phase B: Additional cross-references ===

    /// Resolved superclassifier in Subclassification.
    pub const SUPERCLASSIFIER: &str = "superclassifier";
    /// Resolved conjugated type in Conjugation.
    pub const CONJUGATED_TYPE: &str = "conjugatedType";
    /// Resolved original type in Conjugation.
    pub const ORIGINAL_TYPE: &str = "originalType";
    /// Resolved featuring type in TypeFeaturing.
    pub const FEATURING_TYPE: &str = "featuringType";
    /// Resolved disjoining type in Disjoining.
    pub const DISJOINING_TYPE: &str = "disjoiningType";
    /// Resolved unioning type in Unioning.
    pub const UNIONING_TYPE: &str = "unioningType";
    /// Resolved intersecting type in Intersecting.
    pub const INTERSECTING_TYPE: &str = "intersectingType";
    /// Resolved differencing type in Differencing.
    pub const DIFFERENCING_TYPE: &str = "differencingType";
    /// Resolved inverting feature in FeatureInverting.
    pub const INVERTING_FEATURE: &str = "invertingFeature";
    /// Resolved crossed feature in FeatureChaining.
    pub const CROSSED_FEATURE: &str = "crossedFeature";
    /// Resolved annotated element in Annotation.
    pub const ANNOTATED_ELEMENT: &str = "annotatedElement";
    /// Resolved member element in Membership.
    pub const MEMBER_ELEMENT: &str = "memberElement";
    /// Resolved client in Dependency.
    pub const CLIENT: &str = "client";
    /// Resolved supplier in Dependency.
    pub const SUPPLIER: &str = "supplier";
    /// Resolved conjugated port definition in ConjugatedPortDefinition.
    pub const CONJUGATED_PORT_DEFINITION: &str = "conjugatedPortDefinition";
}

/// Cached scope information for a namespace.
///
/// This caches the expanded set of visible names in a namespace,
/// including members imported from other namespaces.
#[derive(Debug, Default, Clone)]
pub struct ScopeTable {
    /// Direct owned members: name -> element ID.
    owned: HashMap<String, ElementId>,
    /// Short names of owned members: short_name -> element ID.
    owned_short: HashMap<String, ElementId>,
    /// Imported members: name -> (element ID, visibility).
    imported: HashMap<String, (ElementId, VisibilityKind)>,
    /// Imported members by short name.
    imported_short: HashMap<String, (ElementId, VisibilityKind)>,
    /// Inherited members (via Specialization): name -> element ID.
    inherited: HashMap<String, ElementId>,
    /// Whether this scope table has been fully populated (owned members).
    populated: bool,
    /// Whether inherited members have been populated.
    inherited_populated: bool,
    /// Whether imported members have been populated.
    imported_populated: bool,
}

impl ScopeTable {
    /// Create a new empty scope table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an owned member.
    pub fn add_owned(&mut self, name: String, id: ElementId) {
        self.owned.insert(name, id);
    }

    /// Add an owned member by short name.
    pub fn add_owned_short(&mut self, short_name: String, id: ElementId) {
        self.owned_short.insert(short_name, id);
    }

    /// Add an imported member.
    pub fn add_imported(&mut self, name: String, id: ElementId, visibility: VisibilityKind) {
        self.imported.insert(name, (id, visibility));
    }

    /// Add an imported member by short name.
    pub fn add_imported_short(
        &mut self,
        short_name: String,
        id: ElementId,
        visibility: VisibilityKind,
    ) {
        self.imported_short.insert(short_name, (id, visibility));
    }

    /// Add an inherited member.
    pub fn add_inherited(&mut self, name: String, id: ElementId) {
        self.inherited.insert(name, id);
    }

    /// Try looking up a name in a HashMap, also checking quoted/unquoted variants.
    fn lookup_with_quote_variants<'a, V>(
        map: &'a HashMap<String, V>,
        name: &str,
    ) -> Option<&'a V> {
        // Try exact match first
        if let Some(v) = map.get(name) {
            return Some(v);
        }
        // Try with quotes stripped
        let stripped = name.trim_matches('\'');
        if stripped != name {
            if let Some(v) = map.get(stripped) {
                return Some(v);
            }
        }
        // Try with quotes added
        let quoted = format!("'{}'", stripped);
        if quoted != name {
            map.get(&quoted)
        } else {
            None
        }
    }

    /// Look up a name in this scope (owned only).
    pub fn lookup_owned(&self, name: &str) -> Option<&ElementId> {
        Self::lookup_with_quote_variants(&self.owned, name)
            .or_else(|| Self::lookup_with_quote_variants(&self.owned_short, name))
    }

    /// Look up a name in inherited members.
    pub fn lookup_inherited(&self, name: &str) -> Option<&ElementId> {
        Self::lookup_with_quote_variants(&self.inherited, name)
    }

    /// Look up a name in imported members.
    pub fn lookup_imported(&self, name: &str) -> Option<&ElementId> {
        Self::lookup_with_quote_variants(&self.imported, name)
            .or_else(|| Self::lookup_with_quote_variants(&self.imported_short, name))
            .map(|(id, _)| id)
    }

    /// Look up a name with visibility check for imports.
    pub fn lookup_imported_visible(
        &self,
        name: &str,
        check_visibility: bool,
    ) -> Option<&ElementId> {
        let result = Self::lookup_with_quote_variants(&self.imported, name)
            .or_else(|| Self::lookup_with_quote_variants(&self.imported_short, name));
        match result {
            Some((id, visibility)) => {
                if !check_visibility || *visibility == VisibilityKind::Public {
                    Some(id)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Mark this scope table as fully populated.
    pub fn set_populated(&mut self) {
        self.populated = true;
    }

    /// Check if this scope table has been fully populated.
    pub fn is_populated(&self) -> bool {
        self.populated
    }

    /// Mark inherited members as populated.
    pub fn set_inherited_populated(&mut self) {
        self.inherited_populated = true;
    }

    /// Check if inherited members have been populated.
    pub fn has_inherited_populated(&self) -> bool {
        self.inherited_populated
    }

    /// Mark imported members as populated.
    pub fn set_imported_populated(&mut self) {
        self.imported_populated = true;
    }

    /// Check if imported members have been populated.
    pub fn has_imported_populated(&self) -> bool {
        self.imported_populated
    }

    /// Clear the scope table.
    pub fn clear(&mut self) {
        self.owned.clear();
        self.owned_short.clear();
        self.imported.clear();
        self.imported_short.clear();
        self.inherited.clear();
        self.populated = false;
        self.inherited_populated = false;
        self.imported_populated = false;
    }
}

/// Maximum depth for inheritance traversal.
/// This prevents infinite recursion in case of cycles not caught by the visited set.
const MAX_INHERITANCE_DEPTH: usize = 50;

/// Pre-computed inheritance index: maps types to their direct supertypes.
///
/// This is built lazily and provides O(1) lookup of supertypes,
/// avoiding repeated iteration over owned members during inheritance expansion.
#[derive(Debug)]
struct InheritanceIndex {
    /// Maps type ElementId -> list of direct supertype ElementIds
    direct_supertypes: HashMap<ElementId, Vec<ElementId>>,
}

impl InheritanceIndex {
    /// Build the inheritance index from a ModelGraph.
    ///
    /// Iterates over all elements once to find Specialization relationships
    /// and pre-computes the direct supertype mapping.
    fn build(graph: &ModelGraph) -> Self {
        let mut map: HashMap<ElementId, Vec<ElementId>> = HashMap::new();

        for (_, elem) in &graph.elements {
            // Look for Specialization elements
            if elem.kind == ElementKind::Specialization
                || elem.kind.is_subtype_of(ElementKind::Specialization)
            {
                // The owner of the Specialization is the specific (subtype)
                // The "general" property is the general (supertype)
                if let Some(specific_id) = &elem.owner {
                    if let Some(general_id) = elem
                        .props
                        .get(resolved_props::GENERAL)
                        .and_then(|v| v.as_ref())
                    {
                        map.entry(specific_id.clone())
                            .or_default()
                            .push(general_id.clone());
                    }
                }
            }
        }

        Self {
            direct_supertypes: map,
        }
    }

    /// Get the direct supertypes for a type.
    fn supertypes(&self, type_id: &ElementId) -> &[ElementId] {
        self.direct_supertypes
            .get(type_id)
            .map(|v| &v[..])
            .unwrap_or(&[])
    }
}

/// Context for name resolution.
///
/// Tracks state during resolution to prevent cycles and provide context
/// for visibility checks.
#[derive(Debug)]
pub struct ResolutionContext<'a> {
    /// The model graph being resolved.
    graph: &'a ModelGraph,
    /// Cached scope tables per namespace.
    scope_tables: HashMap<ElementId, ScopeTable>,
    /// Elements currently being visited (cycle detection).
    visiting: HashSet<ElementId>,
    /// Whether we're inside a scope (affects private member visibility).
    inside_scope: Option<ElementId>,
    /// Whether we're inheriting (affects protected member visibility).
    inheriting: bool,
    /// Collected diagnostics.
    diagnostics: Diagnostics,
    /// Cache for resolved import targets (qualified name -> resolved ElementId).
    /// Uses RefCell for interior mutability since resolve_import_target is called from &self contexts.
    import_cache: RefCell<HashMap<String, Option<ElementId>>>,
    /// Negative lookup cache: (namespace_id, name) pairs that have already failed resolution.
    /// Avoids redundant parent-walking for names that don't exist anywhere.
    failed_lookups: RefCell<HashSet<(ElementId, String)>>,
    /// Pre-computed inheritance index for O(1) supertype lookup.
    /// Lazily built on first use.
    inheritance_index: Option<InheritanceIndex>,
}

impl<'a> ResolutionContext<'a> {
    /// Create a new resolution context.
    pub fn new(graph: &'a ModelGraph) -> Self {
        ResolutionContext {
            graph,
            scope_tables: HashMap::new(),
            visiting: HashSet::new(),
            inside_scope: None,
            inheriting: false,
            diagnostics: Diagnostics::new(),
            import_cache: RefCell::new(HashMap::new()),
            failed_lookups: RefCell::new(HashSet::new()),
            inheritance_index: None,
        }
    }

    /// Ensure the inheritance index is built.
    ///
    /// This is called lazily on first use to avoid building the index
    /// if inheritance expansion is never needed.
    fn ensure_inheritance_index(&mut self) {
        if self.inheritance_index.is_none() {
            self.inheritance_index = Some(InheritanceIndex::build(self.graph));
        }
    }

    /// Get the underlying graph.
    pub fn graph(&self) -> &'a ModelGraph {
        self.graph
    }

    /// Get collected diagnostics.
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    /// Take the collected diagnostics.
    pub fn take_diagnostics(self) -> Diagnostics {
        self.diagnostics
    }

    /// Add a diagnostic.
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Set the "inside scope" context for visibility checks.
    pub fn set_inside_scope(&mut self, namespace_id: Option<ElementId>) {
        self.inside_scope = namespace_id;
    }

    /// Set the "inheriting" flag for visibility checks.
    pub fn set_inheriting(&mut self, inheriting: bool) {
        self.inheriting = inheriting;
    }

    /// Check if a member with given visibility is visible.
    /// Used in Phase 2d.2 for import expansion.
    #[allow(dead_code)]
    fn is_visible(&self, visibility: VisibilityKind, namespace_id: &ElementId) -> bool {
        match visibility {
            VisibilityKind::Public => true,
            VisibilityKind::Protected => self.inheriting,
            VisibilityKind::Private => self.inside_scope.as_ref() == Some(namespace_id),
        }
    }

    /// Get or create a scope table for a namespace (owned members only).
    pub fn get_scope_table(&mut self, namespace_id: &ElementId) -> &ScopeTable {
        if !self.scope_tables.contains_key(namespace_id) {
            let table = self.build_scope_table(namespace_id);
            self.scope_tables.insert(namespace_id.clone(), table);
        }
        self.scope_tables.get(namespace_id).unwrap()
    }

    /// Get or create a FULL scope table for a namespace.
    ///
    /// This includes:
    /// - Owned members (populated immediately)
    /// - Inherited members (populated on first call)
    /// - Imported members (populated on first call)
    ///
    /// This is the main entry point for name resolution - using this cached
    /// table avoids rebuilding inherited/imported lookups on every call.
    pub fn get_full_scope_table(&mut self, namespace_id: &ElementId) -> &ScopeTable {
        // Check if we need to populate inherited/imported
        let needs_inherited = self.scope_tables.get(namespace_id)
            .map(|t| !t.has_inherited_populated())
            .unwrap_or(true);
        let needs_imported = self.scope_tables.get(namespace_id)
            .map(|t| !t.has_imported_populated())
            .unwrap_or(true);

        if needs_inherited || needs_imported {
            // Build or get owned members first
            if !self.scope_tables.contains_key(namespace_id) {
                let table = self.build_scope_table(namespace_id);
                self.scope_tables.insert(namespace_id.clone(), table);
            }

            // Now expand inherited and imported into the cached table
            // We need to remove the table, modify it, and reinsert due to borrow rules
            let mut table = self.scope_tables.remove(namespace_id).unwrap();

            if needs_inherited {
                let redefined = self.collect_redefined_names(namespace_id);
                let mut visited = HashSet::new();
                self.expand_inherited(namespace_id, &mut table, &mut visited, &redefined, 0);
                table.set_inherited_populated();
            }

            if needs_imported {
                let mut visited = HashSet::new();
                self.expand_imports(namespace_id, &mut table, &mut visited);
                table.set_imported_populated();
            }

            self.scope_tables.insert(namespace_id.clone(), table);
        }

        self.scope_tables.get(namespace_id).unwrap()
    }

    /// Build a scope table for a namespace by collecting owned members.
    fn build_scope_table(&self, namespace_id: &ElementId) -> ScopeTable {
        let mut table = ScopeTable::new();

        // Add owned members
        for membership in self.graph.memberships(namespace_id) {
            if let Some(view) = MembershipView::try_from_element(membership) {
                if let Some(member_id) = view.member_element() {
                    // Get the member name (from membership or element)
                    let member_name = view.member_name().map(|s| s.to_string()).or_else(|| {
                        self.graph
                            .get_element(member_id)
                            .and_then(|e| e.name.clone())
                    });

                    if let Some(name) = member_name {
                        table.add_owned(name, member_id.clone());
                    }

                    // Also add by short name if available
                    if let Some(short_name) = view.member_short_name() {
                        table.add_owned_short(short_name.to_string(), member_id.clone());
                    }
                }
            }
        }

        table.set_populated();
        table
    }

    /// Expand imports for a namespace and add them to a mutable scope table.
    ///
    /// This processes all Import elements owned by the namespace and adds
    /// the imported members to the scope table.
    fn expand_imports(
        &self,
        namespace_id: &ElementId,
        table: &mut ScopeTable,
        visited_imports: &mut HashSet<ElementId>,
    ) {
        // Find all Import elements owned by this namespace
        let imports: Vec<_> = self
            .graph
            .owned_members(namespace_id)
            .filter(|e| {
                e.kind == ElementKind::Import
                    || e.kind == ElementKind::NamespaceImport
                    || e.kind == ElementKind::MembershipImport
                    || e.kind.is_subtype_of(ElementKind::Import)
            })
            .collect();

        for import in imports {
            // Skip if already visited (cycle prevention)
            if visited_imports.contains(&import.id) {
                continue;
            }
            visited_imports.insert(import.id.clone());

            // Get import properties
            let imported_ref = import
                .props
                .get(import_props::IMPORTED_REFERENCE)
                .and_then(|v| v.as_str());

            let is_namespace = import
                .props
                .get(import_props::IS_NAMESPACE)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let is_recursive = import
                .props
                .get(import_props::IS_RECURSIVE)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if let Some(ref_name) = imported_ref {
                // Try to resolve the imported reference
                if let Some(target_id) = self.resolve_import_target(ref_name) {
                    if is_namespace || is_recursive {
                        // Namespace import: import all public members
                        self.import_namespace_members(
                            &target_id,
                            table,
                            is_recursive,
                            visited_imports,
                        );
                    } else {
                        // Membership import: import the specific element
                        if let Some(target) = self.graph.get_element(&target_id) {
                            if let Some(name) = &target.name {
                                table.add_imported(
                                    name.clone(),
                                    target_id.clone(),
                                    VisibilityKind::Public,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// Resolve the target of an import reference.
    ///
    /// Results are cached in `import_cache` to avoid redundant qualified name resolution
    /// when the same import target is referenced multiple times.
    fn resolve_import_target(&self, ref_name: &str) -> Option<ElementId> {
        // Check cache first
        {
            let cache = self.import_cache.borrow();
            if let Some(cached) = cache.get(ref_name) {
                return cached.clone();
            }
        }

        // Cache miss - perform resolution
        let result = self.resolve_import_target_uncached(ref_name);

        // Cache the result (including None for negative caching)
        self.import_cache
            .borrow_mut()
            .insert(ref_name.to_string(), result.clone());

        result
    }

    /// Inner implementation of import target resolution (uncached).
    fn resolve_import_target_uncached(&self, ref_name: &str) -> Option<ElementId> {
        // Try to resolve as a qualified name from root
        let segments = Self::parse_qualified_name_segments(ref_name);
        if segments.is_empty() {
            return None;
        }

        // Find the root element
        let first = segments[0];
        let mut current = self
            .graph
            .roots()
            .find(|r| {
                r.name
                    .as_ref()
                    .map(|n| Self::names_match(n, first))
                    .unwrap_or(false)
            })?
            .id
            .clone();

        // Resolve each subsequent segment by checking owned members directly
        for segment in segments.iter().skip(1) {
            let mut found = false;
            for member in self.graph.owned_members(&current) {
                if member
                    .name
                    .as_ref()
                    .map(|n| Self::names_match(n, segment))
                    .unwrap_or(false)
                {
                    current = member.id.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
        }

        Some(current)
    }

    /// Import all public members from a namespace.
    fn import_namespace_members(
        &self,
        namespace_id: &ElementId,
        table: &mut ScopeTable,
        recursive: bool,
        visited: &mut HashSet<ElementId>,
    ) {
        // Skip if already processed
        if visited.contains(namespace_id) {
            return;
        }
        visited.insert(namespace_id.clone());

        // Add all public members
        for membership in self.graph.memberships(namespace_id) {
            if let Some(view) = MembershipView::try_from_element(membership) {
                // Only import public members
                if view.visibility() != VisibilityKind::Public {
                    continue;
                }

                if let Some(member_id) = view.member_element() {
                    // Get the member name
                    let member_name = view.member_name().map(|s| s.to_string()).or_else(|| {
                        self.graph
                            .get_element(member_id)
                            .and_then(|e| e.name.clone())
                    });

                    if let Some(name) = member_name {
                        table.add_imported(name, member_id.clone(), VisibilityKind::Public);
                    }

                    // If recursive, also import from nested namespaces
                    if recursive {
                        if let Some(member) = self.graph.get_element(member_id) {
                            // Check if member is a namespace (Package, etc.)
                            if member.kind == ElementKind::Package
                                || member.kind == ElementKind::Namespace
                                || member.kind.is_subtype_of(ElementKind::Namespace)
                            {
                                self.import_namespace_members(member_id, table, true, visited);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Expand inherited members for a Type and add them to the scope table.
    ///
    /// This processes all Specialization elements that have this Type as the
    /// specific type and adds the general type's members to the scope table.
    ///
    /// The `depth` parameter prevents infinite recursion in pathological cases.
    fn expand_inherited(
        &self,
        type_id: &ElementId,
        table: &mut ScopeTable,
        visited: &mut HashSet<ElementId>,
        redefined: &HashSet<String>,
        depth: usize,
    ) {
        // Safety limit to prevent infinite recursion
        if depth > MAX_INHERITANCE_DEPTH {
            return;
        }

        // Check if this is a Type (only Types can have specializations)
        let type_element = match self.graph.get_element(type_id) {
            Some(e) => e,
            None => return,
        };

        // Only process Types (Definition, Usage, Classifier, etc.)
        if !type_element.kind.is_subtype_of(ElementKind::Type)
            && type_element.kind != ElementKind::Type
        {
            return;
        }

        // Skip if already processed (cycle prevention)
        if visited.contains(type_id) {
            return;
        }
        visited.insert(type_id.clone());

        // Find all Specialization elements owned by this type
        let specializations: Vec<_> = self
            .graph
            .owned_members(type_id)
            .filter(|e| {
                e.kind == ElementKind::Specialization
                    || e.kind.is_subtype_of(ElementKind::Specialization)
            })
            .collect();

        for spec in specializations {
            // FI-2 FIX: Prioritize already-resolved ElementId to avoid losing package context.
            // When `PackageB::Derived :> PackageA::Base` is resolved, we should use the
            // resolved ElementId directly instead of extracting "Base" and re-resolving it
            // (which would fail without an import from PackageB to PackageA).
            let general_id: Option<ElementId> = spec
                .props
                .get(resolved_props::GENERAL)
                .and_then(|v| v.as_ref())
                .cloned()
                // Fallback: resolve unresolved name if not yet resolved
                .or_else(|| {
                    let ref_name = spec
                        .props
                        .get(unresolved_props::GENERAL)
                        .and_then(|v| v.as_str())?;

                    // Try qualified name resolution, then library packages
                    self.resolve_import_target(ref_name)
                        .or_else(|| self.resolve_in_library_packages(ref_name))
                });

            if let Some(gid) = general_id {
                // Add inherited members from the general type
                self.add_inherited_members(&gid, table, visited, redefined, depth);
            }
        }
    }
    /// Add members from a supertype to the inherited section of the scope table.
    ///
    /// The `depth` parameter is passed through to prevent infinite recursion.
    fn add_inherited_members(
        &self,
        supertype_id: &ElementId,
        table: &mut ScopeTable,
        visited: &mut HashSet<ElementId>,
        redefined: &HashSet<String>,
        depth: usize,
    ) {
        // Get public and protected members from supertype
        for membership in self.graph.memberships(supertype_id) {
            if let Some(view) = MembershipView::try_from_element(membership) {
                // Inherit public and protected members (not private)
                let visibility = view.visibility();
                if visibility == VisibilityKind::Private {
                    continue;
                }

                if let Some(member_id) = view.member_element() {
                    // Get the member name
                    let member_name = view.member_name().map(|s| s.to_string()).or_else(|| {
                        self.graph
                            .get_element(member_id)
                            .and_then(|e| e.name.clone())
                    });

                    if let Some(name) = member_name {
                        // Skip if this name is redefined
                        if redefined.contains(&name) {
                            continue;
                        }
                        table.add_inherited(name, member_id.clone());
                    }
                }
            }
        }

        // Recursively add inherited members from supertype's supertypes
        self.expand_inherited(supertype_id, table, visited, redefined, depth + 1);
    }

    /// Collect redefined feature names from a type.
    fn collect_redefined_names(&self, type_id: &ElementId) -> HashSet<String> {
        let mut redefined = HashSet::new();

        // Find all Redefinition elements
        for member in self.graph.owned_members(type_id) {
            if member.kind == ElementKind::Redefinition
                || member.kind.is_subtype_of(ElementKind::Redefinition)
            {
                // Get the redefined feature name
                if let Some(name) = member
                    .props
                    .get(unresolved_props::REDEFINED_FEATURE)
                    .and_then(|v| v.as_str())
                {
                    // Extract just the name part (last segment of qualified name)
                    let name_part = name.rsplit("::").next().unwrap_or(name);
                    redefined.insert(name_part.to_string());
                }
            }
        }

        redefined
    }

    /// Resolve a simple name within a namespace.
    ///
    /// Follows the precedence: OWNED → INHERITED → IMPORTED → PARENT → GLOBAL
    pub fn resolve_name(&mut self, namespace_id: &ElementId, name: &str) -> Option<ElementId> {
        // Check for cycles
        if self.visiting.contains(namespace_id) {
            return None;
        }
        self.visiting.insert(namespace_id.clone());

        let result = self.resolve_name_inner(namespace_id, name);

        self.visiting.remove(namespace_id);
        result
    }

    /// Inner resolution logic.
    ///
    /// Uses negative lookup caching to avoid re-walking parent hierarchies
    /// for names that have already failed resolution from a given namespace.
    fn resolve_name_inner(&mut self, namespace_id: &ElementId, name: &str) -> Option<ElementId> {
        // Check negative cache first - avoid redundant parent walking for known failures
        {
            let cache = self.failed_lookups.borrow();
            if cache.contains(&(namespace_id.clone(), name.to_string())) {
                return None;
            }
        }

        // 0. PRIMITIVE ALIASES: Check if this is a primitive type alias
        // e.g., "float" -> "Real", "int" -> "Integer"
        if let Some(canonical) = primitive_type_alias(name) {
            let result = self.resolve_name_inner(namespace_id, canonical);
            // If the canonical name failed, also cache the alias as failed
            if result.is_none() {
                self.failed_lookups
                    .borrow_mut()
                    .insert((namespace_id.clone(), name.to_string()));
            }
            return result;
        }

        // Use the cached full scope table (owned + inherited + imported)
        // This avoids rebuilding the table on every lookup - critical for performance!
        // We do lookups first, then extract parent_id to avoid keeping borrow alive
        let (_found_in_table, parent_id) = {
            let table = self.get_full_scope_table(namespace_id);

            // 1. OWNED: Check local owned members
            if let Some(id) = table.lookup_owned(name) {
                return Some(id.clone());
            }

            // 2. INHERITED: Check inherited members (for Types)
            if let Some(id) = table.lookup_inherited(name) {
                return Some(id.clone());
            }

            // 3. IMPORTED: Check imported members
            if let Some(id) = table.lookup_imported(name) {
                return Some(id.clone());
            }

            // Get parent ID while we have immutable borrow
            (false, self.graph.owner_of(namespace_id).map(|e| e.id.clone()))
        };

        // 4. PARENT: Walk up to parent namespace
        if let Some(owner_id) = parent_id {
            if let Some(id) = self.resolve_name(&owner_id, name) {
                return Some(id);
            }
        }

        // 5. GLOBAL: Check root packages (user packages first)
        for root in self.graph.roots() {
            if root.name.as_ref().map(|n| n == name).unwrap_or(false) {
                return Some(root.id.clone());
            }
        }

        // 6. LIBRARY: Check library package members
        // This allows implicit access to standard library types without imports
        if let Some(id) = self.resolve_in_library_packages(name) {
            return Some(id);
        }

        // Cache this failure to avoid re-walking from this namespace for this name
        self.failed_lookups
            .borrow_mut()
            .insert((namespace_id.clone(), name.to_string()));

        None
    }

    /// Resolve a name by searching library package contents.
    ///
    /// This searches the public members of all registered library packages,
    /// including nested packages recursively.
    /// For example, resolving "Anything" will search in "Base" library package.
    fn resolve_in_library_packages(&self, name: &str) -> Option<ElementId> {
        for lib_pkg_id in self.graph.library_packages() {
            // Check if the name matches the library package itself
            if let Some(lib_pkg) = self.graph.get_element(lib_pkg_id) {
                if lib_pkg.name.as_ref().map(|n| n == name).unwrap_or(false) {
                    return Some(lib_pkg_id.clone());
                }
            }

            // Search recursively in nested packages
            if let Some(id) = self.search_library_recursively(lib_pkg_id, name, 0) {
                return Some(id);
            }
        }
        None
    }

    /// Recursively search a library namespace for a name.
    ///
    /// This checks direct members first, then recurses into nested namespaces.
    /// A depth limit prevents infinite recursion in case of cycles.
    fn search_library_recursively(
        &self,
        namespace_id: &ElementId,
        name: &str,
        depth: usize,
    ) -> Option<ElementId> {
        const MAX_DEPTH: usize = 10;
        if depth > MAX_DEPTH {
            return None;
        }

        // Check public members of this namespace
        for membership in self.graph.memberships(namespace_id) {
            if let Some(view) = MembershipView::try_from_element(membership) {
                // Only check public members
                if view.visibility() != crate::VisibilityKind::Public {
                    continue;
                }

                if let Some(member_id) = view.member_element() {
                    // Check member name (use membership name or element name)
                    let member_name = view.member_name().or_else(|| {
                        self.graph
                            .get_element(member_id)
                            .and_then(|e| e.name.as_deref())
                    });

                    // Use names_match to handle quoted names
                    if let Some(mname) = member_name {
                        if Self::names_match(mname, name) {
                            return Some(member_id.clone());
                        }
                    }

                    // Recurse into nested namespaces (packages, definitions, etc.)
                    if let Some(member) = self.graph.get_element(member_id) {
                        if member.kind.is_subtype_of(ElementKind::Namespace)
                            || member.kind == ElementKind::Namespace
                            || member.kind == ElementKind::Package
                        {
                            if let Some(id) =
                                self.search_library_recursively(member_id, name, depth + 1)
                            {
                                return Some(id);
                            }
                        }
                    }
                }
            }
        }

        // Also follow public imports to find re-exported types
        // e.g., KerML has "public import Kernel::*" which re-exports Expression, Occurrence, etc.
        for element in self.graph.owned_members(namespace_id) {
            // Check if this is an import
            let is_import = element.kind == ElementKind::Import
                || element.kind == ElementKind::NamespaceImport
                || element.kind == ElementKind::MembershipImport
                || element.kind.is_subtype_of(ElementKind::Import);

            if !is_import {
                continue;
            }

            // Check visibility - only follow public imports
            let is_public = element
                .props
                .get("visibility")
                .and_then(|v| v.as_str())
                .map(|v| v == "public")
                .unwrap_or(false);

            if !is_public {
                continue;
            }

            // Get the imported reference and resolve it
            let imported_ref = element
                .props
                .get(import_props::IMPORTED_REFERENCE)
                .and_then(|v| v.as_str());

            if let Some(ref_name) = imported_ref {
                // Check if it's a namespace import (::*)
                let is_namespace = element
                    .props
                    .get(import_props::IS_NAMESPACE)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if is_namespace {
                    // Resolve the import target and search in it
                    if let Some(target_id) = self.resolve_import_target(ref_name) {
                        if let Some(found) =
                            self.search_library_recursively(&target_id, name, depth + 1)
                        {
                            return Some(found);
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if a name is a pure feature chain (contains '.' but not '::').
    ///
    /// Feature chains are dot-separated paths like `vehicle.engine.pistons` that
    /// require feature chaining resolution (each segment is resolved in the type
    /// of the previous segment).
    ///
    /// Names containing `::` are qualified names and should be handled by
    /// `resolve_qualified_name` even if they also contain dots (e.g., `A::B.c`).
    ///
    /// # Examples
    /// - `"a.b"` → true (pure feature chain)
    /// - `"a.b.c"` → true (pure feature chain)
    /// - `"A::B"` → false (qualified name)
    /// - `"A::B.c"` → false (qualified name with feature access - not pure chain)
    /// - `"simple"` → false (simple name)
    /// - `"'a.b'"` → false (dot is inside quotes)
    fn is_feature_chain(name: &str) -> bool {
        // If it contains ::, it's a qualified name, not a pure feature chain
        // (even if it also has dots like A::B.c)
        if name.contains("::") {
            return false;
        }

        let mut in_quotes = false;
        for c in name.chars() {
            match c {
                '\'' => in_quotes = !in_quotes,
                '.' if !in_quotes => return true,
                _ => {}
            }
        }
        false
    }

    /// Split a feature chain into segments by '.', respecting quoted names.
    ///
    /// Returns an iterator to avoid allocation in the common case.
    ///
    /// # Examples
    /// - `"a.b.c"` → `["a", "b", "c"]`
    /// - `"'a.b'.c"` → `["'a.b'", "c"]`
    fn split_feature_chain_segments(chain: &str) -> impl Iterator<Item = &str> {
        struct ChainIter<'a> {
            remaining: &'a str,
        }

        impl<'a> Iterator for ChainIter<'a> {
            type Item = &'a str;

            fn next(&mut self) -> Option<Self::Item> {
                if self.remaining.is_empty() {
                    return None;
                }

                let mut in_quotes = false;
                let mut end = 0;

                for (i, c) in self.remaining.char_indices() {
                    match c {
                        '\'' => in_quotes = !in_quotes,
                        '.' if !in_quotes => {
                            let segment = &self.remaining[..i];
                            self.remaining = &self.remaining[i + 1..];
                            return Some(segment);
                        }
                        _ => {}
                    }
                    end = i + c.len_utf8();
                }

                // Last segment
                let segment = &self.remaining[..end];
                self.remaining = "";
                Some(segment)
            }
        }

        ChainIter { remaining: chain }
    }

    /// Parse a qualified name into segments, respecting quoted names.
    ///
    /// Handles cases like "DataFunctions::'/'" where the segment contains `::`.
    /// Returns segments with quotes preserved.
    ///
    /// # Examples
    /// - `"Package::Element"` → `["Package", "Element"]`
    /// - `"DataFunctions::'/'"` → `["DataFunctions", "'/'"]`
    /// - `"ScalarFunctions::'-'"` → `["ScalarFunctions", "'-'"]`
    fn parse_qualified_name_segments(qname: &str) -> Vec<&str> {
        let mut segments = Vec::new();
        let mut start = 0;
        let mut in_quotes = false;
        let bytes = qname.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'\'' {
                in_quotes = !in_quotes;
                i += 1;
            } else if !in_quotes
                && i + 1 < bytes.len()
                && bytes[i] == b':'
                && bytes[i + 1] == b':'
            {
                if i > start {
                    segments.push(&qname[start..i]);
                }
                i += 2;
                start = i;
            } else {
                i += 1;
            }
        }
        if start < qname.len() {
            segments.push(&qname[start..]);
        }
        segments
    }

    /// Check if two names match, stripping quotes if present.
    ///
    /// Handles quoted operator names like `'/'` matching `'/'` or `/`.
    fn names_match(name: &str, target: &str) -> bool {
        let name = name.trim_matches('\'');
        let target = target.trim_matches('\'');
        name == target
    }

    /// Resolve a feature chain reference (dot-separated path like "a.b.c").
    ///
    /// Feature chains are used in expressions like `vehicle.engine.pistons` where
    /// each segment after the first is resolved in the type of the previous segment.
    ///
    /// # Algorithm
    /// 1. Split chain into segments by '.'
    /// 2. Resolve first segment in current scope (normal name resolution)
    /// 3. For each subsequent segment, use feature chaining strategy
    ///
    /// # Performance
    /// O(m * (k + d)) where:
    /// - m = number of segments in chain
    /// - k = average owned features per type
    /// - d = average inheritance depth
    ///
    /// Uses O(1) reverse indexes for type and specialization lookups.
    pub fn resolve_feature_chain(
        &mut self,
        namespace_id: &ElementId,
        chain: &str,
    ) -> Option<ElementId> {
        let mut segments = Self::split_feature_chain_segments(chain);

        // Step 1: Resolve first segment in normal scope
        let first_segment = segments.next()?;
        let mut current_id = self.resolve_name(namespace_id, first_segment)?;

        // Step 2: For each subsequent segment, use feature chaining
        for segment in segments {
            let resolution =
                scoping::resolve_with_feature_chaining(self.graph, &current_id, segment);
            match resolution {
                scoping::ScopedResolution::Found(id) => {
                    current_id = id;
                }
                _ => {
                    return None;
                }
            }
        }

        Some(current_id)
    }

    /// Resolve a qualified name (e.g., "Package::SubPackage::Element") or feature chain (e.g., "a.b.c").
    ///
    /// Starts from the given namespace and resolves each segment.
    /// If local resolution fails for the first segment, falls back to global resolution.
    ///
    /// # Feature Chains
    /// If the name contains '.' (outside of quotes), it's treated as a feature chain
    /// and resolved using feature chaining strategy.
    pub fn resolve_qualified_name(
        &mut self,
        namespace_id: &ElementId,
        qname: &str,
    ) -> Option<ElementId> {
        // Check for feature chain (contains '.' not in quotes)
        if Self::is_feature_chain(qname) {
            return self.resolve_feature_chain(namespace_id, qname);
        }

        let segments = Self::parse_qualified_name_segments(qname);
        if segments.is_empty() {
            return None;
        }

        // First segment: resolve in the current scope
        let first = segments[0];
        let mut current = self.resolve_name(namespace_id, first);

        // If local resolution failed, try global resolution (roots)
        if current.is_none() {
            current = self
                .graph
                .roots()
                .find(|r| {
                    r.name
                        .as_ref()
                        .map(|n| Self::names_match(n, first))
                        .unwrap_or(false)
                })
                .map(|r| r.id.clone());
        }

        // Also search library packages if still not found
        if current.is_none() {
            current = self.resolve_in_library_packages(first);
        }

        let mut current = current?;

        // Subsequent segments: resolve in the resolved element's scope
        for segment in segments.iter().skip(1) {
            current = self.resolve_name(&current, segment)?;
        }

        Some(current)
    }

    /// Resolve a qualified name from root (global).
    ///
    /// The first segment must be a root package name.
    pub fn resolve_qualified_name_global(&mut self, qname: &str) -> Option<ElementId> {
        let segments = Self::parse_qualified_name_segments(qname);
        if segments.is_empty() {
            return None;
        }

        // First segment: find root package or library package
        let first = segments[0];
        let mut current = self
            .graph
            .roots()
            .find(|r| {
                r.name
                    .as_ref()
                    .map(|n| Self::names_match(n, first))
                    .unwrap_or(false)
            })
            .map(|r| r.id.clone())
            // Also search library packages for the first segment
            .or_else(|| self.resolve_in_library_packages(first))?;

        // Subsequent segments: resolve in the resolved element's scope
        for segment in segments.iter().skip(1) {
            current = self.resolve_name(&current, segment)?;
        }

        Some(current)
    }
}

/// Result of resolving references in a model graph.
#[derive(Debug, Default)]
pub struct ResolutionResult {
    /// Number of references successfully resolved.
    pub resolved_count: usize,
    /// Number of references that could not be resolved.
    pub unresolved_count: usize,
    /// Diagnostics collected during resolution.
    pub diagnostics: Diagnostics,
}

impl ResolutionResult {
    /// Create a new empty resolution result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all references were resolved.
    pub fn is_complete(&self) -> bool {
        self.unresolved_count == 0
    }

    /// Check if there were any errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }
}

/// Extension trait for ModelGraph to provide resolution methods.
impl ModelGraph {
    /// Create a resolution context for this graph.
    pub fn resolution_context(&self) -> ResolutionContext<'_> {
        ResolutionContext::new(self)
    }

    /// Resolve a name within a namespace using a fresh context.
    ///
    /// This is a convenience method for simple resolution.
    /// For multiple resolutions, create a `ResolutionContext` instead.
    pub fn resolve_name_in(&self, namespace_id: &ElementId, name: &str) -> Option<ElementId> {
        let mut ctx = self.resolution_context();
        ctx.resolve_name(namespace_id, name)
    }

    /// Resolve a qualified name from root.
    ///
    /// This is a convenience method for simple resolution.
    pub fn resolve_qualified(&self, qname: &str) -> Option<ElementId> {
        let mut ctx = self.resolution_context();
        ctx.resolve_qualified_name_global(qname)
    }
}

/// Resolve all unresolved references in a model graph.
///
/// This function resolves all `unresolved_*` properties to concrete `ElementId`s
/// and stores the resolved references in the corresponding resolved properties.
///
/// # Arguments
///
/// * `graph` - The model graph to resolve (mutable)
///
/// # Returns
///
/// A `ResolutionResult` containing statistics and diagnostics.
pub fn resolve_references(graph: &mut ModelGraph) -> ResolutionResult {
    let mut result = ResolutionResult::new();

    // Collect elements that need resolution (to avoid borrowing issues)
    let elements_to_resolve: Vec<(ElementId, ElementKind)> = graph
        .elements
        .iter()
        .filter(|(_, e)| has_unresolved_refs(e))
        .map(|(id, e)| (id.clone(), e.kind.clone()))
        .collect();

    // Create a resolution context (needs immutable borrow)
    // We'll resolve references and collect them, then apply updates
    let mut updates: Vec<(ElementId, String, ElementId)> = Vec::new();
    let mut unresolved: Vec<(ElementId, String, String)> = Vec::new();

    {
        let ctx_graph = &*graph; // Immutable borrow for context
        let mut ctx = ResolutionContext::new(ctx_graph);

        for (element_id, kind) in &elements_to_resolve {
            // Get the element's owner for scoping
            let scope_id = ctx_graph
                .get_element(element_id)
                .and_then(|e| e.owner.clone())
                .unwrap_or_else(|| element_id.clone());

            // Get the element to check its properties
            let element = match ctx_graph.get_element(element_id) {
                Some(e) => e,
                None => continue,
            };

            // Resolve based on element kind
            // NOTE: Order matters! More specific subtypes must come before more general supertypes.
            // In KerML: Redefinition/ReferenceSubsetting <: Subsetting <: Specialization
            //           FeatureTyping <: Specialization
            match kind {
                // Most specific subtypes first
                k if k == &ElementKind::Redefinition
                    || k.is_subtype_of(ElementKind::Redefinition) =>
                {
                    resolve_redefinition(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::ReferenceSubsetting
                    || k.is_subtype_of(ElementKind::ReferenceSubsetting) =>
                {
                    resolve_reference_subsetting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Subsetting
                    || k.is_subtype_of(ElementKind::Subsetting) =>
                {
                    resolve_subsetting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::FeatureTyping
                    || k.is_subtype_of(ElementKind::FeatureTyping) =>
                {
                    resolve_feature_typing(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                // Most general supertype last
                k if k == &ElementKind::Specialization
                    || k.is_subtype_of(ElementKind::Specialization) =>
                {
                    resolve_specialization(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                // Dependency is a separate hierarchy
                k if k == &ElementKind::Dependency
                    || k.is_subtype_of(ElementKind::Dependency) =>
                {
                    resolve_dependency(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // === Phase B: Additional cross-reference resolution ===

                // Subclassification (superclassifier)
                k if k == &ElementKind::Subclassification
                    || k.is_subtype_of(ElementKind::Subclassification) =>
                {
                    resolve_subclassification(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Conjugation (conjugatedType, originalType)
                k if k == &ElementKind::Conjugation
                    || k.is_subtype_of(ElementKind::Conjugation) =>
                {
                    resolve_conjugation(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // TypeFeaturing (featuringType)
                k if k == &ElementKind::TypeFeaturing
                    || k.is_subtype_of(ElementKind::TypeFeaturing) =>
                {
                    resolve_type_featuring(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Disjoining (disjoiningType)
                k if k == &ElementKind::Disjoining
                    || k.is_subtype_of(ElementKind::Disjoining) =>
                {
                    resolve_disjoining(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Unioning (unioningType)
                k if k == &ElementKind::Unioning
                    || k.is_subtype_of(ElementKind::Unioning) =>
                {
                    resolve_unioning(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Intersecting (intersectingType)
                k if k == &ElementKind::Intersecting
                    || k.is_subtype_of(ElementKind::Intersecting) =>
                {
                    resolve_intersecting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Differencing (differencingType)
                k if k == &ElementKind::Differencing
                    || k.is_subtype_of(ElementKind::Differencing) =>
                {
                    resolve_differencing(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // FeatureInverting (invertingFeature)
                k if k == &ElementKind::FeatureInverting
                    || k.is_subtype_of(ElementKind::FeatureInverting) =>
                {
                    resolve_feature_inverting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // FeatureChaining (crossedFeature)
                k if k == &ElementKind::FeatureChaining
                    || k.is_subtype_of(ElementKind::FeatureChaining) =>
                {
                    resolve_feature_chaining(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Annotation (annotatedElement)
                k if k == &ElementKind::Annotation
                    || k.is_subtype_of(ElementKind::Annotation) =>
                {
                    resolve_annotation(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // Membership (memberElement) - only for elements that have unresolved memberElement
                k if (k == &ElementKind::Membership
                    || k == &ElementKind::OwningMembership
                    || k == &ElementKind::FeatureMembership
                    || k.is_subtype_of(ElementKind::Membership))
                    && element.props.contains_key(unresolved_props::MEMBER_ELEMENT) =>
                {
                    resolve_membership(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                // ConjugatedPortDefinition (conjugatedPortDefinition)
                k if k == &ElementKind::ConjugatedPortDefinition
                    || k.is_subtype_of(ElementKind::ConjugatedPortDefinition) =>
                {
                    resolve_conjugated_port_definition(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }

                _ => {}
            }
        }

        // Take diagnostics from context
        result.diagnostics = ctx.take_diagnostics();
    }

    // Apply updates to the graph
    for (element_id, prop_name, resolved_id) in updates {
        if let Some(element) = graph.elements.get_mut(&element_id) {
            element.set_prop(&prop_name, crate::Value::Ref(resolved_id));
            result.resolved_count += 1;
        }
    }

    // Record unresolved references as diagnostics
    for (element_id, prop_name, unresolved_name) in unresolved {
        let span = graph
            .get_element(&element_id)
            .and_then(|e| e.spans.first().cloned());

        let mut diag = Diagnostic::error(format!(
            "Unresolved reference '{}' for property '{}'",
            unresolved_name, prop_name
        ));

        if let Some(s) = span {
            diag = diag.with_span(s);
        }

        result.diagnostics.push(diag);
        result.unresolved_count += 1;
    }

    result
}

/// Resolve all cross-references in a model graph, excluding specified elements.
///
/// This is useful when resolving user-defined elements while excluding library
/// elements that have already been resolved.
///
/// # Arguments
///
/// * `graph` - The model graph to resolve (mutable)
/// * `exclude_ids` - Set of element IDs to skip during resolution
///
/// # Returns
///
/// A `ResolutionResult` containing statistics and diagnostics.
pub fn resolve_references_excluding(
    graph: &mut ModelGraph,
    exclude_ids: &std::collections::HashSet<ElementId>,
) -> ResolutionResult {
    let mut result = ResolutionResult::new();

    // Collect elements that need resolution, excluding specified IDs
    let elements_to_resolve: Vec<(ElementId, ElementKind)> = graph
        .elements
        .iter()
        .filter(|(id, e)| !exclude_ids.contains(*id) && has_unresolved_refs(e))
        .map(|(id, e)| (id.clone(), e.kind.clone()))
        .collect();

    // Create a resolution context
    let mut updates: Vec<(ElementId, String, ElementId)> = Vec::new();
    let mut unresolved: Vec<(ElementId, String, String)> = Vec::new();

    {
        let ctx_graph = &*graph;
        let mut ctx = ResolutionContext::new(ctx_graph);

        for (element_id, kind) in &elements_to_resolve {
            let scope_id = ctx_graph
                .get_element(element_id)
                .and_then(|e| e.owner.clone())
                .unwrap_or_else(|| element_id.clone());

            let element = match ctx_graph.get_element(element_id) {
                Some(e) => e,
                None => continue,
            };

            // Resolve based on element kind (same logic as resolve_references)
            match kind {
                k if k == &ElementKind::Redefinition
                    || k.is_subtype_of(ElementKind::Redefinition) =>
                {
                    resolve_redefinition(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::ReferenceSubsetting
                    || k.is_subtype_of(ElementKind::ReferenceSubsetting) =>
                {
                    resolve_reference_subsetting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Subsetting
                    || k.is_subtype_of(ElementKind::Subsetting) =>
                {
                    resolve_subsetting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::FeatureTyping
                    || k.is_subtype_of(ElementKind::FeatureTyping) =>
                {
                    resolve_feature_typing(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Specialization
                    || k.is_subtype_of(ElementKind::Specialization) =>
                {
                    resolve_specialization(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Dependency
                    || k.is_subtype_of(ElementKind::Dependency) =>
                {
                    resolve_dependency(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Subclassification
                    || k.is_subtype_of(ElementKind::Subclassification) =>
                {
                    resolve_subclassification(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Conjugation
                    || k.is_subtype_of(ElementKind::Conjugation) =>
                {
                    resolve_conjugation(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::TypeFeaturing
                    || k.is_subtype_of(ElementKind::TypeFeaturing) =>
                {
                    resolve_type_featuring(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Disjoining
                    || k.is_subtype_of(ElementKind::Disjoining) =>
                {
                    resolve_disjoining(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Unioning
                    || k.is_subtype_of(ElementKind::Unioning) =>
                {
                    resolve_unioning(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Intersecting
                    || k.is_subtype_of(ElementKind::Intersecting) =>
                {
                    resolve_intersecting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Differencing
                    || k.is_subtype_of(ElementKind::Differencing) =>
                {
                    resolve_differencing(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::FeatureInverting
                    || k.is_subtype_of(ElementKind::FeatureInverting) =>
                {
                    resolve_feature_inverting(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::FeatureChaining
                    || k.is_subtype_of(ElementKind::FeatureChaining) =>
                {
                    resolve_feature_chaining(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Annotation
                    || k.is_subtype_of(ElementKind::Annotation) =>
                {
                    resolve_annotation(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::Membership
                    || k.is_subtype_of(ElementKind::Membership) =>
                {
                    resolve_membership(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                k if k == &ElementKind::ConjugatedPortDefinition
                    || k.is_subtype_of(ElementKind::ConjugatedPortDefinition) =>
                {
                    resolve_conjugated_port_definition(element, &scope_id, &mut ctx, &mut updates, &mut unresolved);
                }
                _ => {}
            }
        }

        result.diagnostics = ctx.take_diagnostics();
    }

    // Apply updates
    for (element_id, prop_name, resolved_id) in updates {
        if let Some(element) = graph.elements.get_mut(&element_id) {
            element.set_prop(&prop_name, crate::Value::Ref(resolved_id));
            result.resolved_count += 1;
        }
    }

    // Record unresolved references
    for (element_id, prop_name, unresolved_name) in unresolved {
        let span = graph
            .get_element(&element_id)
            .and_then(|e| e.spans.first().cloned());

        let mut diag = Diagnostic::error(format!(
            "Unresolved reference '{}' for property '{}'",
            unresolved_name, prop_name
        ));

        if let Some(s) = span {
            diag = diag.with_span(s);
        }

        result.diagnostics.push(diag);
        result.unresolved_count += 1;
    }

    result
}

/// Check if an element has any unresolved references.
fn has_unresolved_refs(element: &crate::Element) -> bool {
    element.props.contains_key(unresolved_props::GENERAL)
        || element.props.contains_key(unresolved_props::TYPE)
        || element.props.contains_key(unresolved_props::SUBSETTED_FEATURE)
        || element.props.contains_key(unresolved_props::REDEFINED_FEATURE)
        || element.props.contains_key(unresolved_props::REFERENCED_FEATURE)
        || element.props.contains_key(unresolved_props::SOURCES)
        || element.props.contains_key(unresolved_props::TARGETS)
        // Phase B: Additional cross-references
        || element.props.contains_key(unresolved_props::SUPERCLASSIFIER)
        || element.props.contains_key(unresolved_props::CONJUGATED_TYPE)
        || element.props.contains_key(unresolved_props::ORIGINAL_TYPE)
        || element.props.contains_key(unresolved_props::FEATURING_TYPE)
        || element.props.contains_key(unresolved_props::DISJOINING_TYPE)
        || element.props.contains_key(unresolved_props::UNIONING_TYPE)
        || element.props.contains_key(unresolved_props::INTERSECTING_TYPE)
        || element.props.contains_key(unresolved_props::DIFFERENCING_TYPE)
        || element.props.contains_key(unresolved_props::INVERTING_FEATURE)
        || element.props.contains_key(unresolved_props::CROSSED_FEATURE)
        || element.props.contains_key(unresolved_props::ANNOTATED_ELEMENT)
        || element.props.contains_key(unresolved_props::MEMBER_ELEMENT)
        || element.props.contains_key(unresolved_props::CLIENT)
        || element.props.contains_key(unresolved_props::SUPPLIER)
        || element.props.contains_key(unresolved_props::CONJUGATED_PORT_DEFINITION)
}

/// Resolve a Specialization element's general property.
fn resolve_specialization(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(general_ref) = element
        .props
        .get(unresolved_props::GENERAL)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, general_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::GENERAL.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::GENERAL.to_string(),
                general_ref.to_string(),
            ));
        }
    }
}

/// Resolve a FeatureTyping element's type property.
fn resolve_feature_typing(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(type_ref) = element
        .props
        .get(unresolved_props::TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, type_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::TYPE.to_string(),
                type_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Subsetting element's subsettedFeature property.
fn resolve_subsetting(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(subsetted_ref) = element
        .props
        .get(unresolved_props::SUBSETTED_FEATURE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, subsetted_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::SUBSETTED_FEATURE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::SUBSETTED_FEATURE.to_string(),
                subsetted_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Redefinition element's redefinedFeature property.
fn resolve_redefinition(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(redefined_ref) = element
        .props
        .get(unresolved_props::REDEFINED_FEATURE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, redefined_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::REDEFINED_FEATURE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::REDEFINED_FEATURE.to_string(),
                redefined_ref.to_string(),
            ));
        }
    }
}

/// Resolve a ReferenceSubsetting element's referencedFeature property.
fn resolve_reference_subsetting(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(referenced_ref) = element
        .props
        .get(unresolved_props::REFERENCED_FEATURE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, referenced_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::REFERENCED_FEATURE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::REFERENCED_FEATURE.to_string(),
                referenced_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Dependency element's source and target properties.
fn resolve_dependency(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    // Resolve sources
    if let Some(sources) = element
        .props
        .get(unresolved_props::SOURCES)
        .and_then(|v| v.as_list())
    {
        for source_val in sources {
            if let Some(source_ref) = source_val.as_str() {
                if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, source_ref) {
                    updates.push((
                        element.id.clone(),
                        resolved_props::SOURCES.to_string(),
                        resolved_id,
                    ));
                } else {
                    unresolved.push((
                        element.id.clone(),
                        resolved_props::SOURCES.to_string(),
                        source_ref.to_string(),
                    ));
                }
            }
        }
    }

    // Resolve targets
    if let Some(targets) = element
        .props
        .get(unresolved_props::TARGETS)
        .and_then(|v| v.as_list())
    {
        for target_val in targets {
            if let Some(target_ref) = target_val.as_str() {
                if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, target_ref) {
                    updates.push((
                        element.id.clone(),
                        resolved_props::TARGETS.to_string(),
                        resolved_id,
                    ));
                } else {
                    unresolved.push((
                        element.id.clone(),
                        resolved_props::TARGETS.to_string(),
                        target_ref.to_string(),
                    ));
                }
            }
        }
    }

    // Also resolve client/supplier if present (alternative properties for Dependency)
    if let Some(client_ref) = element
        .props
        .get(unresolved_props::CLIENT)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, client_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::CLIENT.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::CLIENT.to_string(),
                client_ref.to_string(),
            ));
        }
    }

    if let Some(supplier_ref) = element
        .props
        .get(unresolved_props::SUPPLIER)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, supplier_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::SUPPLIER.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::SUPPLIER.to_string(),
                supplier_ref.to_string(),
            ));
        }
    }
}

// === Phase B: Additional cross-reference resolution handlers ===

/// Resolve a Subclassification element's superclassifier property.
fn resolve_subclassification(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(superclassifier_ref) = element
        .props
        .get(unresolved_props::SUPERCLASSIFIER)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, superclassifier_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::SUPERCLASSIFIER.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::SUPERCLASSIFIER.to_string(),
                superclassifier_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Conjugation element's conjugatedType and originalType properties.
fn resolve_conjugation(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    // Resolve conjugatedType
    if let Some(conjugated_ref) = element
        .props
        .get(unresolved_props::CONJUGATED_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, conjugated_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::CONJUGATED_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::CONJUGATED_TYPE.to_string(),
                conjugated_ref.to_string(),
            ));
        }
    }

    // Resolve originalType
    if let Some(original_ref) = element
        .props
        .get(unresolved_props::ORIGINAL_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, original_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::ORIGINAL_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::ORIGINAL_TYPE.to_string(),
                original_ref.to_string(),
            ));
        }
    }
}

/// Resolve a TypeFeaturing element's featuringType property.
fn resolve_type_featuring(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(featuring_ref) = element
        .props
        .get(unresolved_props::FEATURING_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, featuring_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::FEATURING_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::FEATURING_TYPE.to_string(),
                featuring_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Disjoining element's disjoiningType property.
fn resolve_disjoining(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(disjoining_ref) = element
        .props
        .get(unresolved_props::DISJOINING_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, disjoining_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::DISJOINING_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::DISJOINING_TYPE.to_string(),
                disjoining_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Unioning element's unioningType property.
fn resolve_unioning(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(unioning_ref) = element
        .props
        .get(unresolved_props::UNIONING_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, unioning_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::UNIONING_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::UNIONING_TYPE.to_string(),
                unioning_ref.to_string(),
            ));
        }
    }
}

/// Resolve an Intersecting element's intersectingType property.
fn resolve_intersecting(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(intersecting_ref) = element
        .props
        .get(unresolved_props::INTERSECTING_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, intersecting_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::INTERSECTING_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::INTERSECTING_TYPE.to_string(),
                intersecting_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Differencing element's differencingType property.
fn resolve_differencing(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(differencing_ref) = element
        .props
        .get(unresolved_props::DIFFERENCING_TYPE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, differencing_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::DIFFERENCING_TYPE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::DIFFERENCING_TYPE.to_string(),
                differencing_ref.to_string(),
            ));
        }
    }
}

/// Resolve a FeatureInverting element's invertingFeature property.
fn resolve_feature_inverting(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(inverting_ref) = element
        .props
        .get(unresolved_props::INVERTING_FEATURE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, inverting_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::INVERTING_FEATURE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::INVERTING_FEATURE.to_string(),
                inverting_ref.to_string(),
            ));
        }
    }
}

/// Resolve a FeatureChaining element's crossedFeature property.
fn resolve_feature_chaining(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    // Note: Feature chaining resolution is more complex and may need
    // the FeatureChaining scoping strategy for proper resolution.
    // For now, we use the standard qualified name resolution.
    if let Some(crossed_ref) = element
        .props
        .get(unresolved_props::CROSSED_FEATURE)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, crossed_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::CROSSED_FEATURE.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::CROSSED_FEATURE.to_string(),
                crossed_ref.to_string(),
            ));
        }
    }
}

/// Resolve an Annotation element's annotatedElement property.
fn resolve_annotation(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(annotated_ref) = element
        .props
        .get(unresolved_props::ANNOTATED_ELEMENT)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, annotated_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::ANNOTATED_ELEMENT.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::ANNOTATED_ELEMENT.to_string(),
                annotated_ref.to_string(),
            ));
        }
    }
}

/// Resolve a Membership element's memberElement property.
fn resolve_membership(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(member_ref) = element
        .props
        .get(unresolved_props::MEMBER_ELEMENT)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, member_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::MEMBER_ELEMENT.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::MEMBER_ELEMENT.to_string(),
                member_ref.to_string(),
            ));
        }
    }
}

/// Resolve a ConjugatedPortDefinition element's conjugatedPortDefinition property.
fn resolve_conjugated_port_definition(
    element: &crate::Element,
    scope_id: &ElementId,
    ctx: &mut ResolutionContext<'_>,
    updates: &mut Vec<(ElementId, String, ElementId)>,
    unresolved: &mut Vec<(ElementId, String, String)>,
) {
    if let Some(port_def_ref) = element
        .props
        .get(unresolved_props::CONJUGATED_PORT_DEFINITION)
        .and_then(|v| v.as_str())
    {
        if let Some(resolved_id) = ctx.resolve_qualified_name(scope_id, port_def_ref) {
            updates.push((
                element.id.clone(),
                resolved_props::CONJUGATED_PORT_DEFINITION.to_string(),
                resolved_id,
            ));
        } else {
            unresolved.push((
                element.id.clone(),
                resolved_props::CONJUGATED_PORT_DEFINITION.to_string(),
                port_def_ref.to_string(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Element, ElementKind, VisibilityKind};

    fn create_test_hierarchy() -> (ModelGraph, ElementId, ElementId, ElementId) {
        let mut graph = ModelGraph::new();

        // Create TestPackage::SubPackage::PartDef
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPackage");
        let pkg_id = graph.add_element(pkg);

        let sub = Element::new_with_kind(ElementKind::Package).with_name("SubPackage");
        let sub_id = graph.add_owned_element(sub, pkg_id.clone(), VisibilityKind::Public);

        let part = Element::new_with_kind(ElementKind::PartDefinition).with_name("PartDef");
        let part_id = graph.add_owned_element(part, sub_id.clone(), VisibilityKind::Public);

        (graph, pkg_id, sub_id, part_id)
    }

    #[test]
    fn scope_table_basic() {
        let mut table = ScopeTable::new();
        let id1 = ElementId::new_v4();
        let id2 = ElementId::new_v4();

        table.add_owned("foo".to_string(), id1.clone());
        table.add_owned_short("f".to_string(), id2.clone());

        assert_eq!(table.lookup_owned("foo"), Some(&id1));
        assert_eq!(table.lookup_owned("f"), Some(&id2));
        assert_eq!(table.lookup_owned("bar"), None);
    }

    #[test]
    fn resolve_name_owned_member() {
        let (graph, pkg_id, sub_id, _) = create_test_hierarchy();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&pkg_id, "SubPackage");

        assert_eq!(resolved, Some(sub_id));
    }

    #[test]
    fn resolve_name_not_found() {
        let (graph, pkg_id, _, _) = create_test_hierarchy();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&pkg_id, "NonExistent");

        assert!(resolved.is_none());
    }

    #[test]
    fn resolve_name_via_parent() {
        let (graph, _, sub_id, _) = create_test_hierarchy();

        // Create another root package
        let mut graph = graph;
        let other = Element::new_with_kind(ElementKind::Package).with_name("OtherPackage");
        let other_id = graph.add_element(other);

        // From SubPackage, resolve OtherPackage (via parent -> global)
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&sub_id, "OtherPackage");

        assert_eq!(resolved, Some(other_id));
    }

    #[test]
    fn resolve_qualified_name_simple() {
        let (graph, pkg_id, _, part_id) = create_test_hierarchy();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_qualified_name(&pkg_id, "SubPackage::PartDef");

        assert_eq!(resolved, Some(part_id));
    }

    #[test]
    fn resolve_qualified_name_global() {
        let (graph, _, _, part_id) = create_test_hierarchy();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_qualified_name_global("TestPackage::SubPackage::PartDef");

        assert_eq!(resolved, Some(part_id));
    }

    #[test]
    fn resolve_qualified_name_global_not_found() {
        let (graph, _, _, _) = create_test_hierarchy();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_qualified_name_global("NonExistent::Path");

        assert!(resolved.is_none());
    }

    #[test]
    fn resolution_context_diagnostics() {
        let graph = ModelGraph::new();
        let mut ctx = graph.resolution_context();

        ctx.add_diagnostic(Diagnostic::error("test error"));
        ctx.add_diagnostic(Diagnostic::warning("test warning"));

        assert!(ctx.diagnostics().has_errors());
        assert_eq!(ctx.diagnostics().len(), 2);
    }

    #[test]
    fn convenience_methods() {
        let (graph, pkg_id, sub_id, part_id) = create_test_hierarchy();

        // Test resolve_name_in
        assert_eq!(
            graph.resolve_name_in(&pkg_id, "SubPackage"),
            Some(sub_id.clone())
        );

        // Test resolve_qualified
        assert_eq!(
            graph.resolve_qualified("TestPackage::SubPackage::PartDef"),
            Some(part_id)
        );
    }

    #[test]
    fn scope_table_imported() {
        let mut table = ScopeTable::new();
        let id = ElementId::new_v4();

        table.add_imported("Imported".to_string(), id.clone(), VisibilityKind::Public);

        assert_eq!(table.lookup_imported("Imported"), Some(&id));
        assert_eq!(table.lookup_imported_visible("Imported", true), Some(&id));

        // Add a private import
        let id2 = ElementId::new_v4();
        table.add_imported("Private".to_string(), id2.clone(), VisibilityKind::Private);

        assert_eq!(table.lookup_imported("Private"), Some(&id2));
        assert_eq!(table.lookup_imported_visible("Private", true), None);
        assert_eq!(table.lookup_imported_visible("Private", false), Some(&id2));
    }

    #[test]
    fn resolution_result_tracking() {
        let mut result = ResolutionResult::new();
        assert!(result.is_complete());
        assert!(!result.has_errors());

        result.resolved_count = 5;
        result.unresolved_count = 2;
        result.diagnostics.error("unresolved reference");

        assert!(!result.is_complete());
        assert!(result.has_errors());
    }

    // === Import Expansion Tests (Phase 2d.2) ===

    /// Helper to create an import element with properties.
    fn create_import(
        graph: &mut ModelGraph,
        owner_id: &ElementId,
        imported_ref: &str,
        is_namespace: bool,
        is_recursive: bool,
    ) -> ElementId {
        use crate::Value;

        let mut import = Element::new_with_kind(ElementKind::Import);
        import.set_prop(
            import_props::IMPORTED_REFERENCE,
            Value::String(imported_ref.to_string()),
        );
        if is_namespace {
            import.set_prop(import_props::IS_NAMESPACE, Value::Bool(true));
        }
        if is_recursive {
            import.set_prop(import_props::IS_RECURSIVE, Value::Bool(true));
        }

        graph.add_owned_element(import, owner_id.clone(), VisibilityKind::Public)
    }

    #[test]
    fn resolve_via_membership_import() {
        let mut graph = ModelGraph::new();

        // Create LibPackage::Helper
        let lib = Element::new_with_kind(ElementKind::Package).with_name("LibPackage");
        let lib_id = graph.add_element(lib);

        let helper = Element::new_with_kind(ElementKind::PartDefinition).with_name("Helper");
        let helper_id = graph.add_owned_element(helper, lib_id.clone(), VisibilityKind::Public);

        // Create UserPackage with import LibPackage::Helper
        let user = Element::new_with_kind(ElementKind::Package).with_name("UserPackage");
        let user_id = graph.add_element(user);

        // Add a membership import (specific element)
        create_import(&mut graph, &user_id, "LibPackage::Helper", false, false);

        // From UserPackage, resolve "Helper" via import
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "Helper");

        assert_eq!(resolved, Some(helper_id));
    }

    #[test]
    fn resolve_via_namespace_import() {
        let mut graph = ModelGraph::new();

        // Create LibPackage with multiple members
        let lib = Element::new_with_kind(ElementKind::Package).with_name("LibPackage");
        let lib_id = graph.add_element(lib);

        let part_a = Element::new_with_kind(ElementKind::PartDefinition).with_name("PartA");
        let part_a_id = graph.add_owned_element(part_a, lib_id.clone(), VisibilityKind::Public);

        let part_b = Element::new_with_kind(ElementKind::PartDefinition).with_name("PartB");
        let part_b_id = graph.add_owned_element(part_b, lib_id.clone(), VisibilityKind::Public);

        // Create UserPackage with namespace import (::*)
        let user = Element::new_with_kind(ElementKind::Package).with_name("UserPackage");
        let user_id = graph.add_element(user);

        // Add a namespace import: import LibPackage::*
        create_import(&mut graph, &user_id, "LibPackage", true, false);

        // From UserPackage, resolve both "PartA" and "PartB" via import
        let mut ctx = graph.resolution_context();
        let resolved_a = ctx.resolve_name(&user_id, "PartA");
        let resolved_b = ctx.resolve_name(&user_id, "PartB");

        assert_eq!(resolved_a, Some(part_a_id));
        assert_eq!(resolved_b, Some(part_b_id));
    }

    #[test]
    fn namespace_import_respects_visibility() {
        let mut graph = ModelGraph::new();

        // Create LibPackage with public and private members
        let lib = Element::new_with_kind(ElementKind::Package).with_name("LibPackage");
        let lib_id = graph.add_element(lib);

        let public_part =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("PublicPart");
        let public_id =
            graph.add_owned_element(public_part, lib_id.clone(), VisibilityKind::Public);

        let private_part =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("PrivatePart");
        let _private_id =
            graph.add_owned_element(private_part, lib_id.clone(), VisibilityKind::Private);

        // Create UserPackage with namespace import
        let user = Element::new_with_kind(ElementKind::Package).with_name("UserPackage");
        let user_id = graph.add_element(user);

        create_import(&mut graph, &user_id, "LibPackage", true, false);

        // From UserPackage, only public members should be resolvable
        let mut ctx = graph.resolution_context();
        let resolved_public = ctx.resolve_name(&user_id, "PublicPart");
        let resolved_private = ctx.resolve_name(&user_id, "PrivatePart");

        assert_eq!(resolved_public, Some(public_id));
        assert!(
            resolved_private.is_none(),
            "Private members should not be imported"
        );
    }

    #[test]
    fn resolve_via_recursive_import() {
        let mut graph = ModelGraph::new();

        // Create LibPackage::Sub::DeepPart
        let lib = Element::new_with_kind(ElementKind::Package).with_name("LibPackage");
        let lib_id = graph.add_element(lib);

        let sub = Element::new_with_kind(ElementKind::Package).with_name("Sub");
        let sub_id = graph.add_owned_element(sub, lib_id.clone(), VisibilityKind::Public);

        let deep_part = Element::new_with_kind(ElementKind::PartDefinition).with_name("DeepPart");
        let deep_part_id =
            graph.add_owned_element(deep_part, sub_id.clone(), VisibilityKind::Public);

        // Also add a direct member
        let top_part = Element::new_with_kind(ElementKind::PartDefinition).with_name("TopPart");
        let top_part_id = graph.add_owned_element(top_part, lib_id.clone(), VisibilityKind::Public);

        // Create UserPackage with recursive import (::**)
        let user = Element::new_with_kind(ElementKind::Package).with_name("UserPackage");
        let user_id = graph.add_element(user);

        create_import(&mut graph, &user_id, "LibPackage", true, true);

        // From UserPackage, resolve both top-level and nested members
        let mut ctx = graph.resolution_context();
        let resolved_top = ctx.resolve_name(&user_id, "TopPart");
        let resolved_sub = ctx.resolve_name(&user_id, "Sub");
        let resolved_deep = ctx.resolve_name(&user_id, "DeepPart");

        assert_eq!(resolved_top, Some(top_part_id));
        assert_eq!(resolved_sub, Some(sub_id));
        assert_eq!(resolved_deep, Some(deep_part_id));
    }

    #[test]
    fn owned_takes_precedence_over_imported() {
        let mut graph = ModelGraph::new();

        // Create LibPackage::Part
        let lib = Element::new_with_kind(ElementKind::Package).with_name("LibPackage");
        let lib_id = graph.add_element(lib);

        let lib_part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let _lib_part_id =
            graph.add_owned_element(lib_part, lib_id.clone(), VisibilityKind::Public);

        // Create UserPackage::Part (same name as imported)
        let user = Element::new_with_kind(ElementKind::Package).with_name("UserPackage");
        let user_id = graph.add_element(user);

        let user_part = Element::new_with_kind(ElementKind::PartDefinition).with_name("Part");
        let user_part_id =
            graph.add_owned_element(user_part, user_id.clone(), VisibilityKind::Public);

        // Add namespace import
        create_import(&mut graph, &user_id, "LibPackage", true, false);

        // Resolve "Part" - should get the owned one, not imported
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "Part");

        assert_eq!(
            resolved,
            Some(user_part_id),
            "Owned members take precedence over imports"
        );
    }

    // === Inheritance Resolution Tests (Phase 2d.3) ===

    /// Helper to create a Specialization element.
    fn create_specialization(
        graph: &mut ModelGraph,
        owner_id: &ElementId,
        general_ref: &str,
    ) -> ElementId {
        use crate::Value;

        let mut spec = Element::new_with_kind(ElementKind::Specialization);
        spec.set_prop(
            unresolved_props::GENERAL,
            Value::String(general_ref.to_string()),
        );

        graph.add_owned_element(spec, owner_id.clone(), VisibilityKind::Public)
    }

    /// Helper to create a Redefinition element.
    fn create_redefinition(
        graph: &mut ModelGraph,
        owner_id: &ElementId,
        redefined_ref: &str,
    ) -> ElementId {
        use crate::Value;

        let mut redef = Element::new_with_kind(ElementKind::Redefinition);
        redef.set_prop(
            unresolved_props::REDEFINED_FEATURE,
            Value::String(redefined_ref.to_string()),
        );

        graph.add_owned_element(redef, owner_id.clone(), VisibilityKind::Public)
    }

    #[test]
    fn resolve_inherited_member() {
        let mut graph = ModelGraph::new();

        // Create Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Create BaseDef with a member
        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let inherited_part =
            Element::new_with_kind(ElementKind::PartUsage).with_name("inheritedPart");
        let inherited_id =
            graph.add_owned_element(inherited_part, base_id.clone(), VisibilityKind::Public);

        // Create DerivedDef that specializes BaseDef
        let derived_def =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id =
            graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add specialization: DerivedDef :> BaseDef
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // From DerivedDef, resolve "inheritedPart" via inheritance
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&derived_id, "inheritedPart");

        assert_eq!(resolved, Some(inherited_id));
    }

    #[test]
    fn owned_takes_precedence_over_inherited() {
        let mut graph = ModelGraph::new();

        // Create Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Create BaseDef with a member
        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let base_part = Element::new_with_kind(ElementKind::PartUsage).with_name("part");
        let _base_part_id =
            graph.add_owned_element(base_part, base_id.clone(), VisibilityKind::Public);

        // Create DerivedDef with its own "part" member
        let derived_def =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id =
            graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        let derived_part = Element::new_with_kind(ElementKind::PartUsage).with_name("part");
        let derived_part_id =
            graph.add_owned_element(derived_part, derived_id.clone(), VisibilityKind::Public);

        // Add specialization
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // Resolve "part" - should get the owned one, not inherited
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&derived_id, "part");

        assert_eq!(
            resolved,
            Some(derived_part_id),
            "Owned members take precedence over inherited"
        );
    }

    #[test]
    fn redefinition_shadows_inherited() {
        let mut graph = ModelGraph::new();

        // Create Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Create BaseDef with a member
        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let base_part = Element::new_with_kind(ElementKind::PartUsage).with_name("part");
        let _base_part_id =
            graph.add_owned_element(base_part, base_id.clone(), VisibilityKind::Public);

        // Create DerivedDef with a redefinition
        let derived_def =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id =
            graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add specialization
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // Add redefinition of "part"
        create_redefinition(&mut graph, &derived_id, "TestPkg::BaseDef::part");

        // Resolve "part" - should NOT find it (redefined but not re-declared)
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&derived_id, "part");

        // The redefinition shadows the inherited member, but since we haven't
        // added a new "part" member, it should not be found
        assert!(resolved.is_none(), "Redefined member should be shadowed");
    }

    #[test]
    fn inherited_member_multiple_levels() {
        let mut graph = ModelGraph::new();

        // Create Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Create GrandBase with a member
        let grand_base = Element::new_with_kind(ElementKind::PartDefinition).with_name("GrandBase");
        let grand_id = graph.add_owned_element(grand_base, pkg_id.clone(), VisibilityKind::Public);

        let grand_part = Element::new_with_kind(ElementKind::PartUsage).with_name("grandPart");
        let grand_part_id =
            graph.add_owned_element(grand_part, grand_id.clone(), VisibilityKind::Public);

        // Create BaseDef that specializes GrandBase
        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        create_specialization(&mut graph, &base_id, "TestPkg::GrandBase");

        // Create DerivedDef that specializes BaseDef
        let derived_def =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id =
            graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // From DerivedDef, resolve "grandPart" via transitive inheritance
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&derived_id, "grandPart");

        assert_eq!(
            resolved,
            Some(grand_part_id),
            "Should resolve through inheritance chain"
        );
    }

    #[test]
    fn private_members_not_inherited() {
        let mut graph = ModelGraph::new();

        // Create Package
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Create BaseDef with a private member
        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let private_part = Element::new_with_kind(ElementKind::PartUsage).with_name("privatePart");
        let _private_id =
            graph.add_owned_element(private_part, base_id.clone(), VisibilityKind::Private);

        // Also add a public member
        let public_part = Element::new_with_kind(ElementKind::PartUsage).with_name("publicPart");
        let public_id =
            graph.add_owned_element(public_part, base_id.clone(), VisibilityKind::Public);

        // Create DerivedDef that specializes BaseDef
        let derived_def =
            Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id =
            graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        let mut ctx = graph.resolution_context();

        // Public member should be inherited
        let resolved_public = ctx.resolve_name(&derived_id, "publicPart");
        assert_eq!(resolved_public, Some(public_id));

        // Private member should NOT be inherited
        let resolved_private = ctx.resolve_name(&derived_id, "privatePart");
        assert!(
            resolved_private.is_none(),
            "Private members should not be inherited"
        );
    }

    // === Main Resolution Pass Tests (Phase 2d.4) ===

    #[test]
    fn resolve_references_specialization() {
        let mut graph = ModelGraph::new();

        // Create Package with BaseDef and DerivedDef
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let derived_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id = graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add specialization with unresolved reference
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // Run resolution
        let result = resolve_references(&mut graph);

        // Should have resolved the specialization
        assert_eq!(result.resolved_count, 1);
        assert_eq!(result.unresolved_count, 0);
        assert!(!result.has_errors());

        // Check that the resolved property was set
        let specs: Vec<_> = graph
            .owned_members(&derived_id)
            .filter(|e| e.kind == ElementKind::Specialization)
            .collect();
        assert_eq!(specs.len(), 1);

        let general = specs[0].props.get(resolved_props::GENERAL);
        assert!(general.is_some(), "general property should be set");
        assert_eq!(general.and_then(|v| v.as_ref()), Some(&base_id));
    }

    #[test]
    fn resolve_references_unresolved_reports_error() {
        let mut graph = ModelGraph::new();

        // Create Package with only DerivedDef (no BaseDef to resolve to)
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        let derived_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id = graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add specialization referencing non-existent type
        create_specialization(&mut graph, &derived_id, "TestPkg::NonExistent");

        // Run resolution
        let result = resolve_references(&mut graph);

        // Should have unresolved reference
        assert_eq!(result.resolved_count, 0);
        assert_eq!(result.unresolved_count, 1);
        assert!(result.has_errors());
    }

    #[test]
    fn resolve_references_feature_typing() {
        use crate::Value;

        let mut graph = ModelGraph::new();

        // Create Package with TypeDef and a feature with typing
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        let type_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("TypeDef");
        let type_id = graph.add_owned_element(type_def, pkg_id.clone(), VisibilityKind::Public);

        // Create a FeatureTyping element with unresolved type
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop(unresolved_props::TYPE, Value::String("TestPkg::TypeDef".to_string()));
        let _typing_id = graph.add_owned_element(typing, pkg_id.clone(), VisibilityKind::Public);

        // Run resolution
        let result = resolve_references(&mut graph);

        // Should have resolved the typing
        assert_eq!(result.resolved_count, 1);
        assert_eq!(result.unresolved_count, 0);

        // Find the FeatureTyping element and check resolved property
        let typings: Vec<_> = graph
            .owned_members(&pkg_id)
            .filter(|e| e.kind == ElementKind::FeatureTyping)
            .collect();
        assert_eq!(typings.len(), 1);

        let resolved_type = typings[0].props.get(resolved_props::TYPE);
        assert!(resolved_type.is_some());
        assert_eq!(resolved_type.and_then(|v| v.as_ref()), Some(&type_id));
    }

    #[test]
    fn resolve_references_multiple_elements() {
        use crate::Value;

        let mut graph = ModelGraph::new();

        // Create Package with multiple types and relationships
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        let base_id = graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let type_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("TypeDef");
        let type_id = graph.add_owned_element(type_def, pkg_id.clone(), VisibilityKind::Public);

        let derived_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id = graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add specialization
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // Add FeatureTyping
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop(unresolved_props::TYPE, Value::String("TestPkg::TypeDef".to_string()));
        graph.add_owned_element(typing, pkg_id.clone(), VisibilityKind::Public);

        // Run resolution
        let result = resolve_references(&mut graph);

        // Should have resolved both
        assert_eq!(result.resolved_count, 2);
        assert_eq!(result.unresolved_count, 0);
        assert!(!result.has_errors());

        // Verify Specialization was resolved
        let specs: Vec<_> = graph
            .owned_members(&derived_id)
            .filter(|e| e.kind == ElementKind::Specialization)
            .collect();
        assert_eq!(specs[0].props.get(resolved_props::GENERAL).and_then(|v| v.as_ref()), Some(&base_id));

        // Verify FeatureTyping was resolved
        let typings: Vec<_> = graph
            .owned_members(&pkg_id)
            .filter(|e| e.kind == ElementKind::FeatureTyping)
            .collect();
        assert_eq!(typings[0].props.get(resolved_props::TYPE).and_then(|v| v.as_ref()), Some(&type_id));
    }

    #[test]
    fn resolve_references_partial_success() {
        use crate::Value;

        let mut graph = ModelGraph::new();

        // Create Package with one resolvable and one unresolvable reference
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        let base_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("BaseDef");
        graph.add_owned_element(base_def, pkg_id.clone(), VisibilityKind::Public);

        let derived_def = Element::new_with_kind(ElementKind::PartDefinition).with_name("DerivedDef");
        let derived_id = graph.add_owned_element(derived_def, pkg_id.clone(), VisibilityKind::Public);

        // Add resolvable specialization
        create_specialization(&mut graph, &derived_id, "TestPkg::BaseDef");

        // Add unresolvable FeatureTyping
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop(unresolved_props::TYPE, Value::String("TestPkg::NonExistent".to_string()));
        graph.add_owned_element(typing, pkg_id.clone(), VisibilityKind::Public);

        // Run resolution
        let result = resolve_references(&mut graph);

        // Should have one resolved and one unresolved
        assert_eq!(result.resolved_count, 1);
        assert_eq!(result.unresolved_count, 1);
        assert!(result.has_errors());
    }

    // === Standard Library Tests (Phase 2d.5) ===

    #[test]
    fn register_library_package() {
        let mut graph = ModelGraph::new();

        // Create a library package (like Base)
        let lib_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let lib_id = graph.add_element(lib_pkg);

        // Register it as a library package
        assert!(graph.register_library_package(lib_id.clone()));
        assert!(graph.is_library_package(&lib_id));
        assert_eq!(graph.library_packages().len(), 1);

        // Unregister
        assert!(graph.unregister_library_package(&lib_id));
        assert!(!graph.is_library_package(&lib_id));
    }

    #[test]
    fn register_non_root_fails() {
        let mut graph = ModelGraph::new();

        // Create a parent package
        let parent = Element::new_with_kind(ElementKind::Package).with_name("Parent");
        let parent_id = graph.add_element(parent);

        // Create a child package
        let child = Element::new_with_kind(ElementKind::Package).with_name("Child");
        let child_id = graph.add_owned_element(child, parent_id.clone(), VisibilityKind::Public);

        // Registering a non-root package should fail
        assert!(!graph.register_library_package(child_id.clone()));
        assert!(!graph.is_library_package(&child_id));
    }

    #[test]
    fn resolve_from_library_package() {
        let mut graph = ModelGraph::new();

        // Create library package "Base" with member "Anything"
        let base_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let base_id = graph.add_element(base_pkg);
        graph.register_library_package(base_id.clone());

        let anything = Element::new_with_kind(ElementKind::Classifier).with_name("Anything");
        let anything_id =
            graph.add_owned_element(anything, base_id.clone(), VisibilityKind::Public);

        // Create a user package
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = graph.add_element(user_pkg);

        // Resolve "Anything" from user package - should find it in library
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "Anything");

        assert_eq!(resolved, Some(anything_id));
    }

    #[test]
    fn user_takes_precedence_over_library() {
        let mut graph = ModelGraph::new();

        // Create library package with "MyType"
        let lib_pkg = Element::new_with_kind(ElementKind::Package).with_name("Lib");
        let lib_id = graph.add_element(lib_pkg);
        graph.register_library_package(lib_id.clone());

        let lib_type = Element::new_with_kind(ElementKind::Classifier).with_name("MyType");
        let _lib_type_id =
            graph.add_owned_element(lib_type, lib_id.clone(), VisibilityKind::Public);

        // Create user package with its own "MyType"
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = graph.add_element(user_pkg);

        let user_type = Element::new_with_kind(ElementKind::Classifier).with_name("MyType");
        let user_type_id =
            graph.add_owned_element(user_type, user_id.clone(), VisibilityKind::Public);

        // Resolve "MyType" from user package - should get user's, not library's
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "MyType");

        assert_eq!(
            resolved,
            Some(user_type_id),
            "User definitions take precedence over library"
        );
    }

    #[test]
    fn resolve_library_package_by_name() {
        let mut graph = ModelGraph::new();

        // Create library package "Base"
        let base_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let base_id = graph.add_element(base_pkg);
        graph.register_library_package(base_id.clone());

        // Create a user package
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = graph.add_element(user_pkg);

        // Resolve "Base" from user package - should find the library package itself
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "Base");

        assert_eq!(resolved, Some(base_id));
    }

    #[test]
    fn library_private_members_not_visible() {
        let mut graph = ModelGraph::new();

        // Create library package with private member
        let lib_pkg = Element::new_with_kind(ElementKind::Package).with_name("Lib");
        let lib_id = graph.add_element(lib_pkg);
        graph.register_library_package(lib_id.clone());

        let private_type = Element::new_with_kind(ElementKind::Classifier).with_name("PrivateType");
        let _private_id =
            graph.add_owned_element(private_type, lib_id.clone(), VisibilityKind::Private);

        let public_type = Element::new_with_kind(ElementKind::Classifier).with_name("PublicType");
        let public_id =
            graph.add_owned_element(public_type, lib_id.clone(), VisibilityKind::Public);

        // Create a user package
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = graph.add_element(user_pkg);

        let mut ctx = graph.resolution_context();

        // Public member should be visible
        let resolved_public = ctx.resolve_name(&user_id, "PublicType");
        assert_eq!(resolved_public, Some(public_id));

        // Private member should NOT be visible
        let resolved_private = ctx.resolve_name(&user_id, "PrivateType");
        assert!(
            resolved_private.is_none(),
            "Library private members should not be visible"
        );
    }

    #[test]
    fn add_library_package_convenience() {
        let mut graph = ModelGraph::new();

        // Use convenience method to add library package
        let lib_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let lib_id = graph.add_library_package(lib_pkg);

        assert!(graph.is_library_package(&lib_id));
        assert!(graph.get_element(&lib_id).is_some());
    }

    #[test]
    fn merge_as_library() {
        // Create library graph
        let mut lib_graph = ModelGraph::new();
        let base_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let base_id = lib_graph.add_element(base_pkg);

        let anything = Element::new_with_kind(ElementKind::Classifier).with_name("Anything");
        let anything_id =
            lib_graph.add_owned_element(anything, base_id.clone(), VisibilityKind::Public);

        // Create user graph
        let mut user_graph = ModelGraph::new();
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = user_graph.add_element(user_pkg);

        // Merge library into user graph
        let count = user_graph.merge(lib_graph, true);
        assert!(count > 0);

        // Rebuild indexes after merge
        user_graph.rebuild_indexes();

        // Verify library package was registered
        assert!(user_graph.is_library_package(&base_id));

        // Resolve "Anything" from user package
        let mut ctx = user_graph.resolution_context();
        let resolved = ctx.resolve_name(&user_id, "Anything");

        assert_eq!(resolved, Some(anything_id));
    }

    #[test]
    fn multiple_library_packages() {
        let mut graph = ModelGraph::new();

        // Create multiple library packages
        let base_pkg = Element::new_with_kind(ElementKind::Package).with_name("Base");
        let base_id = graph.add_element(base_pkg);
        graph.register_library_package(base_id.clone());

        let anything = Element::new_with_kind(ElementKind::Classifier).with_name("Anything");
        let anything_id =
            graph.add_owned_element(anything, base_id.clone(), VisibilityKind::Public);

        let scalar_pkg = Element::new_with_kind(ElementKind::Package).with_name("ScalarValues");
        let scalar_id = graph.add_element(scalar_pkg);
        graph.register_library_package(scalar_id.clone());

        let integer = Element::new_with_kind(ElementKind::DataType).with_name("Integer");
        let integer_id =
            graph.add_owned_element(integer, scalar_id.clone(), VisibilityKind::Public);

        // Create user package
        let user_pkg = Element::new_with_kind(ElementKind::Package).with_name("UserPkg");
        let user_id = graph.add_element(user_pkg);

        // Resolve from both library packages
        let mut ctx = graph.resolution_context();
        assert_eq!(ctx.resolve_name(&user_id, "Anything"), Some(anything_id));
        assert_eq!(ctx.resolve_name(&user_id, "Integer"), Some(integer_id));
        assert_eq!(ctx.resolve_name(&user_id, "Base"), Some(base_id));
        assert_eq!(ctx.resolve_name(&user_id, "ScalarValues"), Some(scalar_id));
    }

    // === Feature Chain Resolution Tests ===

    #[test]
    fn test_is_feature_chain() {
        // Pure feature chains (contain '.' outside quotes, no '::')
        assert!(ResolutionContext::is_feature_chain("a.b"));
        assert!(ResolutionContext::is_feature_chain("a.b.c"));
        assert!(ResolutionContext::is_feature_chain("vehicle.engine.pistons"));

        // Not feature chains
        assert!(!ResolutionContext::is_feature_chain("A::B"));
        assert!(!ResolutionContext::is_feature_chain("A::B::C"));
        assert!(!ResolutionContext::is_feature_chain("simple"));
        assert!(!ResolutionContext::is_feature_chain("'a.b'")); // Dot inside quotes
        assert!(!ResolutionContext::is_feature_chain("'some.path'")); // All inside quotes
        // Mixed qualified name with dot - NOT a pure feature chain
        assert!(!ResolutionContext::is_feature_chain("A::B.c"));
        assert!(!ResolutionContext::is_feature_chain("Package::Type.feature"));
    }

    #[test]
    fn test_split_feature_chain_segments() {
        // Simple cases
        let segments: Vec<_> = ResolutionContext::split_feature_chain_segments("a.b.c").collect();
        assert_eq!(segments, vec!["a", "b", "c"]);

        let segments: Vec<_> = ResolutionContext::split_feature_chain_segments("x.y").collect();
        assert_eq!(segments, vec!["x", "y"]);

        // Single segment (no dots)
        let segments: Vec<_> = ResolutionContext::split_feature_chain_segments("single").collect();
        assert_eq!(segments, vec!["single"]);

        // Quoted names with dots inside
        let segments: Vec<_> =
            ResolutionContext::split_feature_chain_segments("'a.b'.c").collect();
        assert_eq!(segments, vec!["'a.b'", "c"]);

        let segments: Vec<_> =
            ResolutionContext::split_feature_chain_segments("a.'b.c'.d").collect();
        assert_eq!(segments, vec!["a", "'b.c'", "d"]);
    }

    #[test]
    fn test_resolve_feature_chain_simple() {
        use crate::Value;

        let mut graph = ModelGraph::new();

        // Create Engine type with a 'pistons' feature
        let engine_type = Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine");
        let engine_type_id = graph.add_element(engine_type);

        // Use with_owner() + add_element() to populate owner_to_children index
        // (required for children_of() which is used by resolve_feature_in_type)
        let pistons = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("pistons")
            .with_owner(engine_type_id.clone());
        let pistons_id = graph.add_element(pistons);

        // Create Vehicle package with an 'engine' feature typed by Engine
        let vehicle_pkg = Element::new_with_kind(ElementKind::Package).with_name("VehiclePkg");
        let vehicle_pkg_id = graph.add_element(vehicle_pkg);

        // Use add_owned_element for engine feature (creates membership for scope resolution)
        let engine_feature = Element::new_with_kind(ElementKind::PartUsage).with_name("engine");
        let engine_feature_id =
            graph.add_owned_element(engine_feature, vehicle_pkg_id.clone(), VisibilityKind::Public);

        // FeatureTyping: engine : Engine
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop("typedFeature", Value::Ref(engine_feature_id.clone()));
        typing.set_prop("type", Value::Ref(engine_type_id.clone()));
        graph.add_element(typing);

        // Resolve "engine.pistons" from VehiclePkg
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_feature_chain(&vehicle_pkg_id, "engine.pistons");

        assert_eq!(resolved, Some(pistons_id));
    }

    #[test]
    fn test_resolve_feature_chain_via_qualified_name() {
        use crate::Value;

        let mut graph = ModelGraph::new();

        // Create Engine type with a 'pistons' feature
        let engine_type = Element::new_with_kind(ElementKind::PartDefinition).with_name("Engine");
        let engine_type_id = graph.add_element(engine_type);

        // Use with_owner() + add_element() to populate owner_to_children index
        let pistons = Element::new_with_kind(ElementKind::PartUsage)
            .with_name("pistons")
            .with_owner(engine_type_id.clone());
        let pistons_id = graph.add_element(pistons);

        // Create Vehicle package with an 'engine' feature typed by Engine
        let vehicle_pkg = Element::new_with_kind(ElementKind::Package).with_name("VehiclePkg");
        let vehicle_pkg_id = graph.add_element(vehicle_pkg);

        // Use add_owned_element for engine feature (creates membership for scope resolution)
        let engine_feature = Element::new_with_kind(ElementKind::PartUsage).with_name("engine");
        let engine_feature_id =
            graph.add_owned_element(engine_feature, vehicle_pkg_id.clone(), VisibilityKind::Public);

        // FeatureTyping: engine : Engine
        let mut typing = Element::new_with_kind(ElementKind::FeatureTyping);
        typing.set_prop("typedFeature", Value::Ref(engine_feature_id.clone()));
        typing.set_prop("type", Value::Ref(engine_type_id.clone()));
        graph.add_element(typing);

        // resolve_qualified_name should automatically use feature chaining for "engine.pistons"
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_qualified_name(&vehicle_pkg_id, "engine.pistons");

        assert_eq!(resolved, Some(pistons_id));
    }

    #[test]
    fn test_resolve_feature_chain_not_found() {
        let mut graph = ModelGraph::new();

        // Create a simple package with a feature
        let pkg = Element::new_with_kind(ElementKind::Package).with_name("TestPkg");
        let pkg_id = graph.add_element(pkg);

        // Use add_owned_element to create proper membership relationship
        let feature = Element::new_with_kind(ElementKind::PartUsage).with_name("myFeature");
        graph.add_owned_element(feature, pkg_id.clone(), VisibilityKind::Public);

        // Try to resolve a non-existent chain
        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_feature_chain(&pkg_id, "myFeature.nonExistent");

        // Should fail because myFeature has no type
        assert!(resolved.is_none());
    }

    #[test]
    fn test_resolve_feature_chain_first_segment_not_found() {
        let graph = ModelGraph::new();
        let fake_id = ElementId::new_v4();

        let mut ctx = graph.resolution_context();
        let resolved = ctx.resolve_feature_chain(&fake_id, "nonExistent.field");

        assert!(resolved.is_none());
    }
}
