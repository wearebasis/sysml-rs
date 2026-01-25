//! Grammar-Element linkage validation.
//!
//! This module validates at build-time that every pest grammar rule producing an element
//! has a matching ElementKind variant, and vice versa. This prevents:
//! 1. Orphan rules that silently create invalid elements
//! 2. Missing ElementKind variants for element-producing rules
//! 3. Runtime failures when resolution expects certain element types
//!
//! ## How it works
//!
//! The validator extracts:
//! - Xtext rules with `returns SysML::TypeName` or `returns KerML::TypeName`
//! - Pest grammar rule names
//! - ElementKind variants from TTL vocabularies
//!
//! It then validates the linkage between these three sources.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::xtext_parser::XtextRule;

/// Result of grammar-element linkage validation.
#[derive(Debug, Clone, Default)]
pub struct GrammarElementLinkageResult {
    /// Xtext rules that return element types, mapped to their return type.
    pub xtext_element_rules: BTreeMap<String, String>,

    /// Pest rules found in the grammar that appear to produce elements.
    /// These are rules whose names match ElementKind variants.
    pub pest_element_rules: BTreeSet<String>,

    /// ElementKind variants from TTL vocabularies.
    pub element_kinds: BTreeSet<String>,

    /// Pest rules that produce elements but don't have a matching ElementKind.
    /// These would cause runtime errors when creating elements.
    pub missing_element_kinds: Vec<String>,

    /// ElementKind variants that have no corresponding pest grammar rule.
    /// These are types that can't be parsed from textual syntax.
    pub element_kinds_without_rules: Vec<String>,

    /// Xtext rules with return types that don't match any ElementKind.
    pub xtext_rules_with_unknown_types: Vec<(String, String)>,

    /// Successfully validated linkages (rule name -> element kind).
    pub validated_linkages: Vec<(String, String)>,
}

impl GrammarElementLinkageResult {
    /// Returns true if validation passed with no critical errors.
    ///
    /// Note: element_kinds_without_rules is not considered an error because
    /// many element types are created programmatically, not parsed.
    pub fn is_valid(&self) -> bool {
        self.missing_element_kinds.is_empty()
    }

    /// Get the count of critical errors.
    pub fn error_count(&self) -> usize {
        self.missing_element_kinds.len()
    }

    /// Get coverage statistics.
    pub fn coverage_stats(&self) -> (usize, usize, f64) {
        let total_element_kinds = self.element_kinds.len();
        let covered = self.validated_linkages.len();
        let percent = if total_element_kinds > 0 {
            (covered as f64 / total_element_kinds as f64) * 100.0
        } else {
            0.0
        };
        (covered, total_element_kinds, percent)
    }

    /// Format the validation result as a report string.
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        let (covered, total, percent) = self.coverage_stats();

        report.push_str(&format!(
            "Grammar-Element Linkage: {}/{} ElementKinds have grammar rules ({:.1}%)\n",
            covered, total, percent
        ));

        if self.is_valid() {
            report.push_str("  Status: PASSED\n");
        } else {
            report.push_str("  Status: FAILED\n");
        }

        if !self.missing_element_kinds.is_empty() {
            report.push_str(&format!(
                "\nCRITICAL - Pest rules without matching ElementKind ({}):\n",
                self.missing_element_kinds.len()
            ));
            for rule in &self.missing_element_kinds {
                report.push_str(&format!("  - {}\n", rule));
            }
            report.push_str("  These rules will fail at runtime when creating elements.\n");
        }

        if !self.xtext_rules_with_unknown_types.is_empty()
            && self.xtext_rules_with_unknown_types.len() < 20
        {
            report.push_str(&format!(
                "\nWARNING - Xtext rules with unknown return types ({}):\n",
                self.xtext_rules_with_unknown_types.len()
            ));
            for (rule, ret_type) in &self.xtext_rules_with_unknown_types {
                report.push_str(&format!("  - {} returns {}\n", rule, ret_type));
            }
        }

        // Don't report element_kinds_without_rules by default since it's noisy
        // and many types are created programmatically

        report
    }
}

/// Extract the simple type name from an xtext return type.
///
/// Converts "SysML::PartUsage" -> "PartUsage"
/// Converts "KerML::Feature" -> "Feature"
fn extract_type_name(return_type: &str) -> Option<String> {
    // Handle both "SysML::TypeName" and "KerML::TypeName" formats
    if let Some(pos) = return_type.rfind("::") {
        let name = &return_type[pos + 2..];
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    // Also handle simple type names
    if !return_type.is_empty()
        && return_type.chars().next().map_or(false, |c| c.is_uppercase())
    {
        return Some(return_type.to_string());
    }
    None
}

/// Build a mapping from xtext rules to their return element types.
///
/// Only includes rules that return SysML:: or KerML:: types.
pub fn build_xtext_element_rules(xtext_rules: &[XtextRule]) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();

    for rule in xtext_rules {
        // Skip fragments and terminals
        if rule.is_fragment || rule.is_terminal {
            continue;
        }

        // Check if rule has a return type
        if let Some(ref returns_type) = rule.returns_type {
            // Only consider SysML and KerML types
            if returns_type.starts_with("SysML::") || returns_type.starts_with("KerML::") {
                if let Some(type_name) = extract_type_name(returns_type) {
                    mapping.insert(rule.name.clone(), type_name);
                }
            }
        }
    }

    mapping
}

/// Extract pest rule names from grammar content that might produce elements.
///
/// This looks for PascalCase rule names that could correspond to ElementKind variants.
/// Rules ending in "Usage" or "Definition" are strong candidates.
pub fn extract_pest_element_rule_names(grammar_content: &str) -> BTreeSet<String> {
    let mut rules = BTreeSet::new();

    for line in grammar_content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        // Look for rule definitions: Name = { or Name = @{ or Name = !{
        if let Some(eq_pos) = trimmed.find('=') {
            let before_eq = trimmed[..eq_pos].trim();

            // Check if it's a valid PascalCase rule name (potential element producer)
            if is_pascal_case_rule(before_eq) {
                rules.insert(before_eq.to_string());
            }
        }
    }

    rules
}

/// Check if a rule name is PascalCase (potential element type).
fn is_pascal_case_rule(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();

    // Must start with uppercase letter
    if !first_char.is_ascii_uppercase() {
        return false;
    }

    // Must not be all uppercase (those are typically keywords like KW_PART)
    if name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
        return false;
    }

    // Must be alphanumeric
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Filter pest rules to only those likely producing elements.
///
/// This is a heuristic based on naming conventions:
/// - Rules ending in "Usage", "Definition", "Relationship", etc.
/// - Rules matching known ElementKind patterns
pub fn filter_element_producing_rules(
    pest_rules: &BTreeSet<String>,
    element_kinds: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut element_rules = BTreeSet::new();

    // Element-producing suffixes
    let element_suffixes = [
        "Usage",
        "Definition",
        "Membership",
        "Relationship",
        "Annotation",
        "Comment",
        "Documentation",
        "Import",
        "Package",
        "Namespace",
        "Dependency",
        "Typing",
        "Subsetting",
        "Redefinition",
        "Specialization",
        "Expression",
    ];

    for rule in pest_rules {
        // Check if rule name matches an ElementKind
        if element_kinds.contains(rule) {
            element_rules.insert(rule.clone());
            continue;
        }

        // Check if rule has an element-producing suffix
        for suffix in &element_suffixes {
            if rule.ends_with(suffix) {
                element_rules.insert(rule.clone());
                break;
            }
        }
    }

    element_rules
}

/// Validate grammar-element linkage.
///
/// This is the main validation function that checks:
/// 1. Every pest rule that produces elements has a matching ElementKind
/// 2. Xtext rules return types that exist as ElementKinds
/// 3. Reports coverage statistics
///
/// # Arguments
///
/// * `xtext_rules` - Parsed xtext rules with return types
/// * `grammar_content` - The pest grammar content
/// * `element_kind_names` - Names of ElementKind variants from TTL
///
/// # Returns
///
/// A `GrammarElementLinkageResult` with validation results.
pub fn validate_grammar_element_linkage(
    xtext_rules: &[XtextRule],
    grammar_content: &str,
    element_kind_names: &[String],
) -> GrammarElementLinkageResult {
    let mut result = GrammarElementLinkageResult::default();

    // Build element kinds set
    result.element_kinds = element_kind_names.iter().cloned().collect();

    // Build xtext element rules mapping
    result.xtext_element_rules = build_xtext_element_rules(xtext_rules);

    // Extract all pest rules
    let all_pest_rules = extract_pest_element_rule_names(grammar_content);

    // Filter to likely element-producing rules
    result.pest_element_rules = filter_element_producing_rules(&all_pest_rules, &result.element_kinds);

    // Validate xtext return types against ElementKinds
    for (rule_name, return_type) in &result.xtext_element_rules {
        if !result.element_kinds.contains(return_type) {
            result
                .xtext_rules_with_unknown_types
                .push((rule_name.clone(), return_type.clone()));
        }
    }

    // Check for pest rules that should have ElementKinds but don't
    for pest_rule in &result.pest_element_rules {
        if result.element_kinds.contains(pest_rule) {
            result
                .validated_linkages
                .push((pest_rule.clone(), pest_rule.clone()));
        } else {
            // Check if this might map to a different ElementKind
            // Some rules have different names than their ElementKind (e.g., FlowConnectionUsage -> FlowUsage)
            let potential_mappings = find_potential_element_kind(pest_rule, &result.element_kinds);
            if let Some(element_kind) = potential_mappings {
                result
                    .validated_linkages
                    .push((pest_rule.clone(), element_kind));
            } else if is_likely_element_rule(pest_rule) {
                result.missing_element_kinds.push(pest_rule.clone());
            }
        }
    }

    // Check for ElementKinds without corresponding pest rules
    let pest_rule_targets: HashSet<String> = result
        .validated_linkages
        .iter()
        .map(|(_, kind)| kind.clone())
        .collect();

    for kind in &result.element_kinds {
        if !pest_rule_targets.contains(kind) {
            result.element_kinds_without_rules.push(kind.clone());
        }
    }

    // Sort results for consistent output
    result.missing_element_kinds.sort();
    result.element_kinds_without_rules.sort();
    result.xtext_rules_with_unknown_types.sort();
    result.validated_linkages.sort();

    result
}

/// Find potential ElementKind matches for a pest rule name.
///
/// Handles common naming discrepancies:
/// - FlowConnectionUsage -> FlowUsage
/// - FlowConnectionDefinition -> FlowDefinition
/// - DefaultReferenceUsage -> ReferenceUsage
fn find_potential_element_kind(rule_name: &str, element_kinds: &BTreeSet<String>) -> Option<String> {
    // Known mappings
    let mappings: &[(&str, &str)] = &[
        ("FlowConnectionUsage", "FlowUsage"),
        ("FlowConnectionDefinition", "FlowDefinition"),
        ("DefaultReferenceUsage", "ReferenceUsage"),
        ("IndividualUsage", "OccurrenceUsage"),
        ("PortionUsage", "OccurrenceUsage"),
        ("SuccessionFlowUsage", "SuccessionFlowUsage"),
        ("ExtendedDefinition", "Definition"),
        ("ExtendedUsage", "Usage"),
        ("VariantReference", "ReferenceUsage"),
        ("LibraryPackage", "LibraryPackage"),
        // Action nodes
        ("SendNode", "SendActionUsage"),
        ("AcceptNode", "AcceptActionUsage"),
        ("AssignmentNode", "AssignmentActionUsage"),
        ("TerminateNode", "TerminateActionUsage"),
        ("IfNode", "IfActionUsage"),
        ("WhileLoopNode", "WhileLoopActionUsage"),
        ("ForLoopNode", "ForLoopActionUsage"),
        // KerML definitions
        ("ClassifierDefinition", "Classifier"),
        ("DatatypeDefinition", "DataType"),
        ("ClassDefinition", "Class"),
        ("StructDefinition", "Structure"),
        ("AssociationDefinition", "Association"),
        ("AssociationStructDefinition", "AssociationStructure"),
        ("MultiplicityDefinition", "Multiplicity"),
        ("FeatureDefinition", "Feature"),
        ("BehaviorDefinition", "Behavior"),
        ("FunctionDefinition", "Function"),
        ("PredicateDefinition", "Predicate"),
        ("InteractionDefinition", "Interaction"),
        ("MetaclassDefinition", "Metaclass"),
        // KerML usages
        ("StepUsage", "Step"),
        ("ExpressionUsage", "Expression"),
        ("BooleanExpressionUsage", "BooleanExpression"),
        ("InvariantUsage", "Invariant"),
    ];

    for (rule, kind) in mappings {
        if rule_name == *rule {
            if element_kinds.contains(*kind) {
                return Some(kind.to_string());
            }
        }
    }

    None
}

/// Check if a rule name is likely an element-producing rule.
///
/// This is a heuristic to avoid false positives for helper rules.
fn is_likely_element_rule(rule_name: &str) -> bool {
    // Definite element patterns
    if rule_name.ends_with("Usage")
        || rule_name.ends_with("Definition")
        || rule_name.ends_with("Membership")
    {
        return true;
    }

    // Skip known helper/intermediate rules
    let helper_patterns = [
        "Member",
        "Element",
        "Body",
        "Item",
        "Part",
        "Prefix",
        "Declaration",
        "Completion",
        "Node",
    ];

    for pattern in helper_patterns {
        if rule_name.ends_with(pattern) && rule_name != pattern {
            return false;
        }
    }

    // Specific element types
    matches!(
        rule_name,
        "Package" | "Namespace" | "Import" | "Comment" | "Documentation" | "Dependency"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_type_name() {
        assert_eq!(
            extract_type_name("SysML::PartUsage"),
            Some("PartUsage".to_string())
        );
        assert_eq!(
            extract_type_name("KerML::Feature"),
            Some("Feature".to_string())
        );
        assert_eq!(
            extract_type_name("PartUsage"),
            Some("PartUsage".to_string())
        );
        assert_eq!(extract_type_name("::"), None);
        assert_eq!(extract_type_name("lowercase"), None);
    }

    #[test]
    fn test_is_pascal_case_rule() {
        assert!(is_pascal_case_rule("PartUsage"));
        assert!(is_pascal_case_rule("ActionDefinition"));
        assert!(is_pascal_case_rule("Package"));
        assert!(!is_pascal_case_rule("KW_PART")); // All caps
        assert!(!is_pascal_case_rule("lowercase"));
        assert!(!is_pascal_case_rule("_underscore"));
        assert!(!is_pascal_case_rule(""));
    }

    #[test]
    fn test_extract_pest_element_rule_names() {
        let grammar = r#"
// Comment
PartUsage = { KW_PART }
ActionDefinition = @{ KW_ACTION ~ KW_DEF }
KW_PART = { "part" }
_helper_rule = { "test" }
WHITESPACE = _{ " " }
"#;
        let rules = extract_pest_element_rule_names(grammar);

        assert!(rules.contains("PartUsage"));
        assert!(rules.contains("ActionDefinition"));
        assert!(!rules.contains("KW_PART")); // All caps
        assert!(!rules.contains("_helper_rule")); // Starts with underscore
        assert!(!rules.contains("WHITESPACE")); // All caps
    }

    #[test]
    fn test_build_xtext_element_rules() {
        let xtext_rules = vec![
            XtextRule {
                name: "PartUsage".to_string(),
                returns_type: Some("SysML::PartUsage".to_string()),
                is_fragment: false,
                is_terminal: false,
            },
            XtextRule {
                name: "Feature".to_string(),
                returns_type: Some("KerML::Feature".to_string()),
                is_fragment: false,
                is_terminal: false,
            },
            XtextRule {
                name: "Identification".to_string(),
                returns_type: Some("SysML::Element".to_string()),
                is_fragment: true, // Should be skipped
                is_terminal: false,
            },
            XtextRule {
                name: "DECIMAL_VALUE".to_string(),
                returns_type: Some("Ecore::EInt".to_string()),
                is_fragment: false,
                is_terminal: true, // Should be skipped
            },
        ];

        let mapping = build_xtext_element_rules(&xtext_rules);

        assert_eq!(mapping.get("PartUsage"), Some(&"PartUsage".to_string()));
        assert_eq!(mapping.get("Feature"), Some(&"Feature".to_string()));
        assert!(!mapping.contains_key("Identification")); // Fragment
        assert!(!mapping.contains_key("DECIMAL_VALUE")); // Terminal
    }

    #[test]
    fn test_validate_grammar_element_linkage() {
        let xtext_rules = vec![XtextRule {
            name: "PartUsage".to_string(),
            returns_type: Some("SysML::PartUsage".to_string()),
            is_fragment: false,
            is_terminal: false,
        }];

        let grammar = "PartUsage = { KW_PART }\nActionUsage = { KW_ACTION }";

        let element_kinds = vec!["PartUsage".to_string(), "ActionUsage".to_string()];

        let result = validate_grammar_element_linkage(&xtext_rules, grammar, &element_kinds);

        assert!(result.is_valid());
        assert!(result
            .validated_linkages
            .iter()
            .any(|(r, _)| r == "PartUsage"));
        assert!(result
            .validated_linkages
            .iter()
            .any(|(r, _)| r == "ActionUsage"));
    }
}
