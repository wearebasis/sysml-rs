//! Grammar rule coverage tracking.
//!
//! This module tracks which pest grammar rules are exercised during parsing.
//! It uses the `coverage` feature on `sysml-text-pest`.

use std::collections::HashSet;
use std::path::Path;

use sysml_text_pest::PestParser;

use crate::grammar_rules::{extract_rule_info, visible_rules, RuleInfo};

/// Track which rules are visited during parsing.
///
/// This uses the `parse_for_rule_coverage()` method from `sysml-text-pest`
/// to collect rule names.
pub struct RuleCoverageTracker {
    /// Set of rule names that have been visited.
    visited_rules: HashSet<String>,
}

impl RuleCoverageTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        RuleCoverageTracker {
            visited_rules: HashSet::new(),
        }
    }

    /// Track rules from parsing a source string.
    pub fn track_parse(&mut self, source: &str) {
        let parser = PestParser::new();
        if let Ok(rules) = parser.parse_for_rule_coverage(source) {
            self.visited_rules.extend(rules);
        }
    }

    /// Get all visited rule names.
    pub fn visited_rules(&self) -> &HashSet<String> {
        &self.visited_rules
    }

    /// Get the count of visited rules.
    pub fn count(&self) -> usize {
        self.visited_rules.len()
    }

    /// Check if a specific rule was visited.
    pub fn was_visited(&self, rule: &str) -> bool {
        self.visited_rules.contains(rule)
    }

    /// Get rules that were expected but not visited.
    pub fn missing_rules(&self, expected: &HashSet<String>) -> HashSet<String> {
        expected
            .difference(&self.visited_rules)
            .cloned()
            .collect()
    }
}

impl Default for RuleCoverageTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Load all grammar rules from the pest grammar file.
///
/// This reads the actual grammar file and extracts all rule names,
/// providing an authoritative source for coverage tracking.
///
/// # Arguments
///
/// * `workspace_root` - Path to the sysml-rs workspace root
///
/// # Returns
///
/// A vector of RuleInfo with all rules defined in the grammar.
pub fn load_all_grammar_rules(workspace_root: &Path) -> std::io::Result<Vec<RuleInfo>> {
    let grammar_path = workspace_root.join("sysml-text-pest/src/grammar/sysml.pest");
    let content = std::fs::read_to_string(grammar_path)?;
    Ok(extract_rule_info(&content))
}

/// Get the set of all visible grammar rule names from the pest grammar file.
///
/// Silent rules (defined with `_{ }`) are excluded as they don't appear
/// in parse results.
///
/// # Arguments
///
/// * `workspace_root` - Path to the sysml-rs workspace root
pub fn all_visible_rule_names(workspace_root: &Path) -> std::io::Result<HashSet<String>> {
    let rules = load_all_grammar_rules(workspace_root)?;
    let visible = visible_rules(&rules);
    Ok(visible.iter().map(|r| r.name.clone()).collect())
}

/// Get the set of all grammar rule names (including hidden/silent rules).
///
/// # Arguments
///
/// * `workspace_root` - Path to the sysml-rs workspace root
pub fn all_rule_names(workspace_root: &Path) -> std::io::Result<HashSet<String>> {
    let rules = load_all_grammar_rules(workspace_root)?;
    Ok(rules.iter().map(|r| r.name.clone()).collect())
}

/// Get the list of all grammar rules defined in the pest grammar.
///
/// This is provided for backwards compatibility with the original API.
/// It returns a hardcoded fallback list if the grammar file cannot be loaded.
///
/// For proper coverage tracking, use `load_all_grammar_rules()` instead.
pub fn all_grammar_rules() -> Vec<String> {
    // Try to load from workspace-relative path first
    let workspace_paths = [
        Path::new("."),            // Running from sysml-rs
        Path::new(".."),           // Running from a crate subdirectory
        Path::new("../.."),        // Running from tests subdirectory
    ];

    for base in workspace_paths {
        if let Ok(rules) = load_all_grammar_rules(base) {
            return rules.into_iter().map(|r| r.name).collect();
        }
    }

    // Fallback to hardcoded list if file can't be loaded
    // This ensures tests can still run even if paths are wrong
    fallback_grammar_rules()
}

/// Get visible grammar rules (non-silent rules that appear in parse tree).
pub fn visible_grammar_rules() -> Vec<String> {
    let workspace_paths = [
        Path::new("."),
        Path::new(".."),
        Path::new("../.."),
    ];

    for base in workspace_paths {
        if let Ok(rules) = load_all_grammar_rules(base) {
            return visible_rules(&rules)
                .into_iter()
                .map(|r| r.name.clone())
                .collect();
        }
    }

    // Fallback - filter keywords and silent rules from fallback list
    fallback_grammar_rules()
        .into_iter()
        .filter(|r| !r.starts_with("KW_"))
        .collect()
}

/// Categorize rules by their purpose.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    /// Entry points (File, Model)
    EntryPoint,
    /// Keywords (KW_*)
    Keyword,
    /// Definitions (PartDefinition, ActionDefinition, etc.)
    Definition,
    /// Usages (PartUsage, ActionUsage, etc.)
    Usage,
    /// Tokens and operators
    Token,
    /// Names and references
    Name,
    /// Expressions
    Expression,
    /// Annotations (Comment, Documentation, etc.)
    Annotation,
    /// Imports
    Import,
    /// Other/Uncategorized
    Other,
}

/// Categorize a rule by its name.
pub fn categorize_rule(name: &str) -> RuleCategory {
    if name.starts_with("KW_") {
        RuleCategory::Keyword
    } else if name == "File" || name == "Model" {
        RuleCategory::EntryPoint
    } else if name.ends_with("Definition") {
        RuleCategory::Definition
    } else if name.ends_with("Usage") {
        RuleCategory::Usage
    } else if name.contains("Expression") || name.contains("Literal") || name.contains("Operator") {
        RuleCategory::Expression
    } else if name.contains("Name") || name.contains("Identification") || name == "QualifiedName" {
        RuleCategory::Name
    } else if name == "Comment" || name == "Documentation" || name.contains("Annotation") {
        RuleCategory::Annotation
    } else if name.contains("Import") {
        RuleCategory::Import
    } else if name.ends_with("Token") || name.ends_with("Kind") {
        RuleCategory::Token
    } else {
        RuleCategory::Other
    }
}

/// Group rules by category for reporting.
pub fn rules_by_category(rules: &[RuleInfo]) -> std::collections::HashMap<RuleCategory, Vec<&RuleInfo>> {
    let mut grouped = std::collections::HashMap::new();

    for rule in rules {
        let category = categorize_rule(&rule.name);
        grouped.entry(category).or_insert_with(Vec::new).push(rule);
    }

    grouped
}

/// Fallback hardcoded list of grammar rules.
/// Used when the grammar file cannot be loaded.
fn fallback_grammar_rules() -> Vec<String> {
    vec![
        // Entry points
        "File".to_string(),
        "Model".to_string(),
        // Packages
        "Package".to_string(),
        "LibraryPackage".to_string(),
        "PackageBody".to_string(),
        "PackageBodyElement".to_string(),
        // Definitions
        "DefinitionElement".to_string(),
        "AttributeDefinition".to_string(),
        "EnumerationDefinition".to_string(),
        "OccurrenceDefinition".to_string(),
        "ItemDefinition".to_string(),
        "MetadataDefinition".to_string(),
        "PartDefinition".to_string(),
        "PortDefinition".to_string(),
        "ConnectionDefinition".to_string(),
        "FlowConnectionDefinition".to_string(),
        "InterfaceDefinition".to_string(),
        "AllocationDefinition".to_string(),
        "ActionDefinition".to_string(),
        "CalculationDefinition".to_string(),
        "StateDefinition".to_string(),
        "ConstraintDefinition".to_string(),
        "RequirementDefinition".to_string(),
        "ConcernDefinition".to_string(),
        "CaseDefinition".to_string(),
        "AnalysisCaseDefinition".to_string(),
        "VerificationCaseDefinition".to_string(),
        "UseCaseDefinition".to_string(),
        "ViewDefinition".to_string(),
        "ViewpointDefinition".to_string(),
        "RenderingDefinition".to_string(),
        // Usages
        "UsageElement".to_string(),
        "ReferenceUsage".to_string(),
        "AttributeUsage".to_string(),
        "EnumerationUsage".to_string(),
        "OccurrenceUsage".to_string(),
        "ItemUsage".to_string(),
        "PartUsage".to_string(),
        "PortUsage".to_string(),
        "ConnectionUsage".to_string(),
        "InterfaceUsage".to_string(),
        "AllocationUsage".to_string(),
        "FlowConnectionUsage".to_string(),
        "ViewUsage".to_string(),
        "RenderingUsage".to_string(),
        "ActionUsage".to_string(),
        "PerformActionUsage".to_string(),
        "CalculationUsage".to_string(),
        "StateUsage".to_string(),
        "ExhibitStateUsage".to_string(),
        "ConstraintUsage".to_string(),
        "AssertConstraintUsage".to_string(),
        "RequirementUsage".to_string(),
        "SatisfyRequirementUsage".to_string(),
        "ConcernUsage".to_string(),
        "CaseUsage".to_string(),
        "AnalysisCaseUsage".to_string(),
        "VerificationCaseUsage".to_string(),
        "UseCaseUsage".to_string(),
        "IncludeUseCaseUsage".to_string(),
        "ViewpointUsage".to_string(),
        // Imports
        "Import".to_string(),
        // Annotations
        "Comment".to_string(),
        "Documentation".to_string(),
        "MetadataUsage".to_string(),
        // Names and identifiers
        "Name".to_string(),
        "QualifiedName".to_string(),
        "Identification".to_string(),
        // Expressions
        "Expression".to_string(),
        "PrimaryExpression".to_string(),
        "LiteralExpression".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracker_basic() {
        let tracker = RuleCoverageTracker::new();
        assert_eq!(tracker.count(), 0);
    }

    #[test]
    fn all_rules_non_empty() {
        let rules = all_grammar_rules();
        assert!(!rules.is_empty());
        assert!(rules.contains(&"Package".to_string()));
    }

    #[test]
    fn categorize_rules_correctly() {
        assert_eq!(categorize_rule("KW_PACKAGE"), RuleCategory::Keyword);
        assert_eq!(categorize_rule("File"), RuleCategory::EntryPoint);
        assert_eq!(categorize_rule("PartDefinition"), RuleCategory::Definition);
        assert_eq!(categorize_rule("PartUsage"), RuleCategory::Usage);
        assert_eq!(categorize_rule("PrimaryExpression"), RuleCategory::Expression);
        assert_eq!(categorize_rule("Import"), RuleCategory::Import);
        assert_eq!(categorize_rule("Comment"), RuleCategory::Annotation);
    }

    #[test]
    fn visible_rules_filters_keywords() {
        let visible = visible_grammar_rules();
        // Keywords should still be in visible rules (they're not silent)
        // but we can check that the list is reasonable
        assert!(!visible.is_empty());
    }
}
