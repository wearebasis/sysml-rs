//! Grammar rule extraction from pest files.
//!
//! This module parses the pest grammar file to extract all rule names,
//! providing an authoritative source for grammar coverage testing.

use std::collections::HashSet;

/// Extract all rule names from a pest grammar file content.
///
/// This parses the grammar file and extracts rule names by looking for
/// patterns like `RuleName = { ... }` or `RuleName = @{ ... }` etc.
///
/// # Arguments
///
/// * `content` - The pest grammar file content
///
/// # Returns
///
/// A set of all rule names defined in the grammar.
pub fn extract_rule_names(content: &str) -> HashSet<String> {
    let mut rules = HashSet::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Match rule definitions:
        // RuleName = { ... }
        // RuleName = @{ ... }
        // RuleName = _{ ... }
        // RuleName = ${ ... }
        // RuleName = !{ ... }
        if let Some(rule_name) = extract_rule_name_from_line(line) {
            rules.insert(rule_name);
        }
    }

    rules
}

/// Extract a rule name from a line if it's a rule definition.
fn extract_rule_name_from_line(line: &str) -> Option<String> {
    // Look for patterns like "NAME = {" or "NAME = @{" etc.
    // The rule name is everything before " = "
    let equals_pos = line.find(" = ")?;
    let after_equals = &line[equals_pos + 3..].trim();

    // Check if what follows is a valid rule body opener
    if after_equals.starts_with('{')
        || after_equals.starts_with("@{")
        || after_equals.starts_with("_{")
        || after_equals.starts_with("${")
        || after_equals.starts_with("!{")
    {
        let name = line[..equals_pos].trim();
        // Validate it's a valid identifier
        if is_valid_rule_name(name) {
            return Some(name.to_string());
        }
    }

    None
}

/// Check if a string is a valid pest rule name.
fn is_valid_rule_name(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // Must start with letter or underscore
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Categorize rules by their type (hidden, atomic, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleType {
    /// Normal rule: `Name = { ... }`
    Normal,
    /// Silent/Hidden rule: `Name = _{ ... }`
    Silent,
    /// Atomic rule: `Name = @{ ... }`
    Atomic,
    /// Compound atomic rule: `Name = ${ ... }`
    CompoundAtomic,
    /// Non-atomic rule: `Name = !{ ... }`
    NonAtomic,
}

/// Information about a grammar rule.
#[derive(Debug, Clone)]
pub struct RuleInfo {
    /// The name of the rule.
    pub name: String,
    /// The type of rule (normal, silent, atomic, etc.).
    pub rule_type: RuleType,
}

/// Extract rule information including type from a pest grammar file.
pub fn extract_rule_info(content: &str) -> Vec<RuleInfo> {
    let mut rules = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some(info) = extract_rule_info_from_line(line) {
            rules.push(info);
        }
    }

    // Sort by name for consistent ordering
    rules.sort_by(|a, b| a.name.cmp(&b.name));
    rules
}

/// Extract rule info from a line if it's a rule definition.
fn extract_rule_info_from_line(line: &str) -> Option<RuleInfo> {
    let equals_pos = line.find(" = ")?;
    let after_equals = line[equals_pos + 3..].trim();
    let name = line[..equals_pos].trim();

    if !is_valid_rule_name(name) {
        return None;
    }

    let rule_type = if after_equals.starts_with("_{") {
        RuleType::Silent
    } else if after_equals.starts_with("@{") {
        RuleType::Atomic
    } else if after_equals.starts_with("${") {
        RuleType::CompoundAtomic
    } else if after_equals.starts_with("!{") {
        RuleType::NonAtomic
    } else if after_equals.starts_with('{') {
        RuleType::Normal
    } else {
        return None;
    };

    Some(RuleInfo {
        name: name.to_string(),
        rule_type,
    })
}

/// Filter rules to get only non-silent (visible) rules.
///
/// Silent rules (defined with `_{ }`) are not part of the parse tree
/// and cannot be tracked during parsing.
pub fn visible_rules(rules: &[RuleInfo]) -> Vec<&RuleInfo> {
    rules
        .iter()
        .filter(|r| r.rule_type != RuleType::Silent)
        .collect()
}

/// Get the path to the grammar file relative to the references directory.
pub const GRAMMAR_PATH: &str = "sysml-text-pest/src/grammar/sysml.pest";

/// Load and parse the grammar file from a workspace root path.
pub fn load_grammar_rules(workspace_root: &std::path::Path) -> std::io::Result<Vec<RuleInfo>> {
    let grammar_path = workspace_root.join(GRAMMAR_PATH);
    let content = std::fs::read_to_string(grammar_path)?;
    Ok(extract_rule_info(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_simple_rules() {
        let content = r#"
// Comment
Name = { ID | UNRESTRICTED_NAME }
Package = { KW_PACKAGE ~ Identification }
"#;
        let rules = extract_rule_names(content);
        assert!(rules.contains("Name"));
        assert!(rules.contains("Package"));
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn extract_atomic_rules() {
        let content = r#"
ID = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
STRING_VALUE = @{ "\"" ~ CHAR* ~ "\"" }
"#;
        let rules = extract_rule_names(content);
        assert!(rules.contains("ID"));
        assert!(rules.contains("STRING_VALUE"));
    }

    #[test]
    fn extract_silent_rules() {
        let content = r#"
WHITESPACE = _{ " " | "\t" | NEWLINE }
COMMENT = _{ ML_COMMENT | SL_COMMENT }
"#;
        let rules = extract_rule_names(content);
        assert!(rules.contains("WHITESPACE"));
        assert!(rules.contains("COMMENT"));
    }

    #[test]
    fn rule_type_detection() {
        let content = r#"
Normal = { "normal" }
Silent = _{ "silent" }
Atomic = @{ "atomic" }
CompoundAtomic = ${ "compound" }
NonAtomic = !{ "nonatomic" }
"#;
        let rules = extract_rule_info(content);
        assert_eq!(rules.len(), 5);

        let by_name: std::collections::HashMap<&str, &RuleInfo> =
            rules.iter().map(|r| (r.name.as_str(), r)).collect();

        assert_eq!(by_name["Normal"].rule_type, RuleType::Normal);
        assert_eq!(by_name["Silent"].rule_type, RuleType::Silent);
        assert_eq!(by_name["Atomic"].rule_type, RuleType::Atomic);
        assert_eq!(by_name["CompoundAtomic"].rule_type, RuleType::CompoundAtomic);
        assert_eq!(by_name["NonAtomic"].rule_type, RuleType::NonAtomic);
    }

    #[test]
    fn filters_visible_rules() {
        let content = r#"
Normal = { "normal" }
Silent = _{ "silent" }
Atomic = @{ "atomic" }
"#;
        let rules = extract_rule_info(content);
        let visible = visible_rules(&rules);
        assert_eq!(visible.len(), 2);
        assert!(visible.iter().any(|r| r.name == "Normal"));
        assert!(visible.iter().any(|r| r.name == "Atomic"));
        assert!(!visible.iter().any(|r| r.name == "Silent"));
    }

    #[test]
    fn skips_comments_and_empty() {
        let content = r#"
// This is a comment
Name = { ID }

// Another comment
// More comments

Package = { KW_PACKAGE }
"#;
        let rules = extract_rule_names(content);
        assert_eq!(rules.len(), 2);
    }
}
