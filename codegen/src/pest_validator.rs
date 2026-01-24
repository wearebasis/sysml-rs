//! Validation of keyword coverage in pest grammar.
//!
//! This module provides build-time validation to ensure every usage/definition keyword
//! has a corresponding grammar rule, preventing missing rules like `MessageUsage` from
//! going undetected.
//!
//! ## Architecture
//!
//! Keywords are auto-extracted from xtext specs, but grammar rules are manually written.
//! This validation connects KW_* keywords to their expected grammar rules.
//!
//! ## Keyword Classification
//!
//! Keywords are classified into categories:
//! - **UsageDefinition**: Keywords that require both XxxUsage and XxxDefinition rules
//! - **UsageOnly**: Keywords that only require XxxUsage rules (no definition variant)
//! - **Modifier**: Prefix modifiers like `abstract`, `derived`, etc.
//! - **Operator**: Expression operators like `and`, `or`, `implies`
//! - **Control**: Control flow keywords like `if`, `then`, `else`
//! - **Contextual**: Context-specific keywords like `do`, `entry`, `exit`

use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Classification of keyword types for grammar validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordType {
    /// Keywords that typically have both XxxUsage and XxxDefinition grammar rules.
    /// The strings are the expected rule name suffixes (e.g., "Part" -> PartUsage, PartDefinition).
    UsageDefinition {
        /// Expected usage rule name (e.g., "PartUsage")
        usage_rule: String,
        /// Expected definition rule name (e.g., "PartDefinition"), if applicable
        def_rule: Option<String>,
    },
    /// Keywords that only have XxxUsage rules (no definition variant).
    UsageOnly {
        /// Expected usage rule name
        usage_rule: String,
    },
    /// Modifier keywords used in prefixes (abstract, derived, readonly, etc.)
    Modifier,
    /// Operator keywords used in expressions (and, or, implies, etc.)
    Operator,
    /// Control flow keywords (if, then, else, while, etc.)
    Control,
    /// Context-specific keywords (do, entry, exit, first, etc.)
    Contextual,
    /// Miscellaneous keywords that don't fit other categories
    Other,
}

/// Classify a keyword into its type.
///
/// This determines what grammar rules (if any) are expected for each keyword.
///
/// # Arguments
///
/// * `kw` - The keyword string (e.g., "part", "abstract", "and")
///
/// # Returns
///
/// The classification of the keyword.
pub fn classify_keyword(kw: &str) -> KeywordType {
    match kw {
        // ===== Usage/Definition keywords (require both rules) =====
        "action" => KeywordType::UsageDefinition {
            usage_rule: "ActionUsage".to_string(),
            def_rule: Some("ActionDefinition".to_string()),
        },
        "part" => KeywordType::UsageDefinition {
            usage_rule: "PartUsage".to_string(),
            def_rule: Some("PartDefinition".to_string()),
        },
        "state" => KeywordType::UsageDefinition {
            usage_rule: "StateUsage".to_string(),
            def_rule: Some("StateDefinition".to_string()),
        },
        "constraint" => KeywordType::UsageDefinition {
            usage_rule: "ConstraintUsage".to_string(),
            def_rule: Some("ConstraintDefinition".to_string()),
        },
        "requirement" => KeywordType::UsageDefinition {
            usage_rule: "RequirementUsage".to_string(),
            def_rule: Some("RequirementDefinition".to_string()),
        },
        "item" => KeywordType::UsageDefinition {
            usage_rule: "ItemUsage".to_string(),
            def_rule: Some("ItemDefinition".to_string()),
        },
        "port" => KeywordType::UsageDefinition {
            usage_rule: "PortUsage".to_string(),
            def_rule: Some("PortDefinition".to_string()),
        },
        "connection" => KeywordType::UsageDefinition {
            usage_rule: "ConnectionUsage".to_string(),
            def_rule: Some("ConnectionDefinition".to_string()),
        },
        "interface" => KeywordType::UsageDefinition {
            usage_rule: "InterfaceUsage".to_string(),
            def_rule: Some("InterfaceDefinition".to_string()),
        },
        "allocation" => KeywordType::UsageDefinition {
            usage_rule: "AllocationUsage".to_string(),
            def_rule: Some("AllocationDefinition".to_string()),
        },
        "view" => KeywordType::UsageDefinition {
            usage_rule: "ViewUsage".to_string(),
            def_rule: Some("ViewDefinition".to_string()),
        },
        "viewpoint" => KeywordType::UsageDefinition {
            usage_rule: "ViewpointUsage".to_string(),
            def_rule: Some("ViewpointDefinition".to_string()),
        },
        "rendering" => KeywordType::UsageDefinition {
            usage_rule: "RenderingUsage".to_string(),
            def_rule: Some("RenderingDefinition".to_string()),
        },
        "calc" => KeywordType::UsageDefinition {
            usage_rule: "CalculationUsage".to_string(),
            def_rule: Some("CalculationDefinition".to_string()),
        },
        "attribute" => KeywordType::UsageDefinition {
            usage_rule: "AttributeUsage".to_string(),
            def_rule: Some("AttributeDefinition".to_string()),
        },
        "flow" => KeywordType::UsageDefinition {
            usage_rule: "FlowConnectionUsage".to_string(),
            def_rule: Some("FlowConnectionDefinition".to_string()),
        },
        "message" => KeywordType::UsageDefinition {
            usage_rule: "MessageUsage".to_string(),
            def_rule: None, // MessageDefinition doesn't exist in SysML v2
        },
        "occurrence" => KeywordType::UsageDefinition {
            usage_rule: "OccurrenceUsage".to_string(),
            def_rule: Some("OccurrenceDefinition".to_string()),
        },
        "case" => KeywordType::UsageDefinition {
            usage_rule: "CaseUsage".to_string(),
            def_rule: Some("CaseDefinition".to_string()),
        },
        "concern" => KeywordType::UsageDefinition {
            usage_rule: "ConcernUsage".to_string(),
            def_rule: Some("ConcernDefinition".to_string()),
        },
        "analysis" => KeywordType::UsageDefinition {
            usage_rule: "AnalysisCaseUsage".to_string(),
            def_rule: Some("AnalysisCaseDefinition".to_string()),
        },
        "verification" => KeywordType::UsageDefinition {
            usage_rule: "VerificationCaseUsage".to_string(),
            def_rule: Some("VerificationCaseDefinition".to_string()),
        },
        "enum" => KeywordType::UsageDefinition {
            usage_rule: "EnumerationUsage".to_string(),
            def_rule: Some("EnumerationDefinition".to_string()),
        },

        // ===== Usage-only keywords (no definition variant) =====
        "ref" => KeywordType::UsageOnly {
            usage_rule: "ReferenceUsage".to_string(),
        },
        "event" => KeywordType::UsageOnly {
            usage_rule: "EventOccurrenceUsage".to_string(),
        },
        "succession" => KeywordType::UsageOnly {
            usage_rule: "SuccessionAsUsage".to_string(),
        },
        "bind" | "binding" => KeywordType::UsageOnly {
            usage_rule: "BindingConnectorAsUsage".to_string(),
        },
        "perform" => KeywordType::UsageOnly {
            usage_rule: "PerformActionUsage".to_string(),
        },
        "exhibit" => KeywordType::UsageOnly {
            usage_rule: "ExhibitStateUsage".to_string(),
        },
        "include" => KeywordType::UsageOnly {
            usage_rule: "IncludeUseCaseUsage".to_string(),
        },
        "satisfy" => KeywordType::UsageOnly {
            usage_rule: "SatisfyRequirementUsage".to_string(),
        },
        "assert" => KeywordType::UsageOnly {
            usage_rule: "AssertConstraintUsage".to_string(),
        },
        "connect" => KeywordType::UsageOnly {
            usage_rule: "ConnectionUsage".to_string(),
        },
        "allocate" => KeywordType::UsageOnly {
            usage_rule: "AllocationUsage".to_string(),
        },

        // ===== Modifiers (used in prefixes, no dedicated rules) =====
        "abstract" | "derived" | "readonly" | "ordered" | "nonunique" | "individual"
        | "portion" | "snapshot" | "timeslice" | "variant" | "variation" | "end"
        | "composite" | "conjugate" | "conjugated" | "disjoint" | "specializes"
        | "conjugates" | "featured" | "parallel" => KeywordType::Modifier,

        // ===== Operators (used in expressions) =====
        "and" | "or" | "xor" | "implies" | "not" | "as" | "hastype" | "istype" | "meta"
        | "all" | "null" | "true" | "false" => KeywordType::Operator,

        // ===== Control flow =====
        "if" | "then" | "else" | "while" | "loop" | "until" | "accept" => KeywordType::Control,

        // ===== Contextual keywords =====
        "do" | "entry" | "exit" | "first" | "via" | "to" | "from" | "of" | "by"
        | "subject" | "actor" | "stakeholder" | "objective" | "assume" | "require"
        | "frame" | "render" | "filter" | "expose" | "import" | "alias" | "language"
        | "doc" | "comment" | "about" | "rep" | "return" | "decide" | "merge" | "join"
        | "fork" | "send" | "transition" | "private" | "protected" | "public"
        | "dependency" | "use" | "package" | "library" | "standard" | "metadata"
        | "id" | "default" | "def" | "in" | "out" | "inout" => KeywordType::Contextual,

        // Everything else
        _ => KeywordType::Other,
    }
}

/// Result of validating keyword coverage against grammar rules.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Usage rules that should exist but are missing from the grammar.
    pub missing_usage_rules: Vec<String>,
    /// Definition rules that should exist but are missing from the grammar.
    pub missing_definition_rules: Vec<String>,
    /// Keywords that don't have any expected rules (informational).
    pub unclassified_keywords: Vec<String>,
    /// Rules that were found and validated.
    pub validated_rules: Vec<String>,
}

impl ValidationResult {
    /// Returns true if validation passed with no missing rules.
    pub fn is_valid(&self) -> bool {
        self.missing_usage_rules.is_empty() && self.missing_definition_rules.is_empty()
    }

    /// Format the validation result as a report string.
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        if self.is_valid() {
            report.push_str("Keyword-to-grammar validation PASSED\n");
            report.push_str(&format!(
                "  Validated {} rules\n",
                self.validated_rules.len()
            ));
        } else {
            report.push_str("Keyword-to-grammar validation FAILED\n");

            if !self.missing_usage_rules.is_empty() {
                report.push_str("\nMissing usage rules:\n");
                for rule in &self.missing_usage_rules {
                    report.push_str(&format!("  - {}\n", rule));
                }
            }

            if !self.missing_definition_rules.is_empty() {
                report.push_str("\nMissing definition rules:\n");
                for rule in &self.missing_definition_rules {
                    report.push_str(&format!("  - {}\n", rule));
                }
            }
        }

        if !self.unclassified_keywords.is_empty() && self.unclassified_keywords.len() < 50 {
            report.push_str(&format!(
                "\nUnclassified keywords ({}): {:?}\n",
                self.unclassified_keywords.len(),
                self.unclassified_keywords
            ));
        }

        report
    }
}

/// Validate that all usage/definition keywords have corresponding grammar rules.
///
/// This function checks that for each keyword classified as UsageDefinition or UsageOnly,
/// the expected grammar rules exist in the provided grammar content.
///
/// # Arguments
///
/// * `keywords` - List of keyword strings extracted from xtext files
/// * `grammar_content` - The generated pest grammar content to validate against
///
/// # Returns
///
/// A `ValidationResult` containing any missing rules.
pub fn validate_keyword_coverage(keywords: &[String], grammar_content: &str) -> ValidationResult {
    let mut result = ValidationResult::default();

    // Extract all rule names from the grammar
    let grammar_rules = extract_grammar_rules(grammar_content);

    // Track which rules we expect
    let mut expected_usage_rules: BTreeSet<String> = BTreeSet::new();
    let mut expected_def_rules: BTreeSet<String> = BTreeSet::new();

    // Classify each keyword and collect expected rules
    for kw in keywords {
        match classify_keyword(kw) {
            KeywordType::UsageDefinition {
                usage_rule,
                def_rule,
            } => {
                expected_usage_rules.insert(usage_rule);
                if let Some(def) = def_rule {
                    expected_def_rules.insert(def);
                }
            }
            KeywordType::UsageOnly { usage_rule } => {
                expected_usage_rules.insert(usage_rule);
            }
            KeywordType::Other => {
                result.unclassified_keywords.push(kw.clone());
            }
            _ => {
                // Modifiers, operators, control, contextual don't need dedicated rules
            }
        }
    }

    // Check which expected rules are missing
    for rule in &expected_usage_rules {
        if grammar_rules.contains(rule) {
            result.validated_rules.push(rule.clone());
        } else {
            result.missing_usage_rules.push(rule.clone());
        }
    }

    for rule in &expected_def_rules {
        if grammar_rules.contains(rule) {
            result.validated_rules.push(rule.clone());
        } else {
            result.missing_definition_rules.push(rule.clone());
        }
    }

    // Sort for consistent output
    result.missing_usage_rules.sort();
    result.missing_definition_rules.sort();
    result.unclassified_keywords.sort();
    result.validated_rules.sort();

    result
}

/// Extract all rule names from pest grammar content.
///
/// Looks for patterns like `RuleName = { ... }` or `RuleName = @{ ... }`.
fn extract_grammar_rules(grammar_content: &str) -> HashSet<String> {
    let mut rules = HashSet::new();

    for line in grammar_content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        // Look for rule definitions: Name = { or Name = @{ or Name = !{ or Name = _{
        if let Some(eq_pos) = trimmed.find('=') {
            let before_eq = trimmed[..eq_pos].trim();

            // Validate it's a proper rule name (PascalCase or SCREAMING_SNAKE_CASE)
            if is_valid_pest_rule_name(before_eq) {
                rules.insert(before_eq.to_string());
            }
        }
    }

    rules
}

/// Check if a string is a valid pest rule name.
fn is_valid_pest_rule_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();

    // Rule names start with uppercase letter or underscore
    if !first_char.is_ascii_uppercase() && first_char != '_' {
        return false;
    }

    // Rest can be alphanumeric or underscore
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Get a summary of keyword classifications for debugging.
pub fn get_keyword_classification_summary(keywords: &[String]) -> BTreeMap<String, Vec<String>> {
    let mut summary: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for kw in keywords {
        let category = match classify_keyword(kw) {
            KeywordType::UsageDefinition { .. } => "UsageDefinition",
            KeywordType::UsageOnly { .. } => "UsageOnly",
            KeywordType::Modifier => "Modifier",
            KeywordType::Operator => "Operator",
            KeywordType::Control => "Control",
            KeywordType::Contextual => "Contextual",
            KeywordType::Other => "Other",
        };

        summary
            .entry(category.to_string())
            .or_default()
            .push(kw.clone());
    }

    // Sort keywords within each category
    for keywords in summary.values_mut() {
        keywords.sort();
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_usage_definition_keywords() {
        match classify_keyword("part") {
            KeywordType::UsageDefinition {
                usage_rule,
                def_rule,
            } => {
                assert_eq!(usage_rule, "PartUsage");
                assert_eq!(def_rule, Some("PartDefinition".to_string()));
            }
            _ => panic!("Expected UsageDefinition"),
        }

        match classify_keyword("action") {
            KeywordType::UsageDefinition {
                usage_rule,
                def_rule,
            } => {
                assert_eq!(usage_rule, "ActionUsage");
                assert_eq!(def_rule, Some("ActionDefinition".to_string()));
            }
            _ => panic!("Expected UsageDefinition"),
        }

        // message has no definition
        match classify_keyword("message") {
            KeywordType::UsageDefinition {
                usage_rule,
                def_rule,
            } => {
                assert_eq!(usage_rule, "MessageUsage");
                assert_eq!(def_rule, None);
            }
            _ => panic!("Expected UsageDefinition"),
        }
    }

    #[test]
    fn test_classify_usage_only_keywords() {
        match classify_keyword("ref") {
            KeywordType::UsageOnly { usage_rule } => {
                assert_eq!(usage_rule, "ReferenceUsage");
            }
            _ => panic!("Expected UsageOnly"),
        }

        match classify_keyword("perform") {
            KeywordType::UsageOnly { usage_rule } => {
                assert_eq!(usage_rule, "PerformActionUsage");
            }
            _ => panic!("Expected UsageOnly"),
        }
    }

    #[test]
    fn test_classify_modifier_keywords() {
        assert_eq!(classify_keyword("abstract"), KeywordType::Modifier);
        assert_eq!(classify_keyword("derived"), KeywordType::Modifier);
        assert_eq!(classify_keyword("readonly"), KeywordType::Modifier);
    }

    #[test]
    fn test_classify_operator_keywords() {
        assert_eq!(classify_keyword("and"), KeywordType::Operator);
        assert_eq!(classify_keyword("or"), KeywordType::Operator);
        assert_eq!(classify_keyword("implies"), KeywordType::Operator);
    }

    #[test]
    fn test_classify_control_keywords() {
        assert_eq!(classify_keyword("if"), KeywordType::Control);
        assert_eq!(classify_keyword("then"), KeywordType::Control);
        assert_eq!(classify_keyword("else"), KeywordType::Control);
    }

    #[test]
    fn test_classify_contextual_keywords() {
        assert_eq!(classify_keyword("entry"), KeywordType::Contextual);
        assert_eq!(classify_keyword("exit"), KeywordType::Contextual);
        assert_eq!(classify_keyword("do"), KeywordType::Contextual);
    }

    #[test]
    fn test_extract_grammar_rules() {
        let grammar = r#"
// Comment line
WHITESPACE = _{ " " | "\t" }

PartUsage = {
    UsagePrefix ~ KW_PART ~ UsageDeclaration?
}

ActionUsage = @{ "action" }

StateDefinition = !{
    OccurrenceDefinitionPrefix
}

_hidden_rule = { "test" }
"#;

        let rules = extract_grammar_rules(grammar);

        assert!(rules.contains("WHITESPACE"));
        assert!(rules.contains("PartUsage"));
        assert!(rules.contains("ActionUsage"));
        assert!(rules.contains("StateDefinition"));
        assert!(rules.contains("_hidden_rule"));
    }

    #[test]
    fn test_validate_keyword_coverage_success() {
        let keywords = vec!["part".to_string(), "abstract".to_string()];

        let grammar = r#"
PartUsage = { KW_PART }
PartDefinition = { KW_PART ~ KW_DEF }
"#;

        let result = validate_keyword_coverage(&keywords, grammar);

        assert!(result.is_valid());
        assert!(result.missing_usage_rules.is_empty());
        assert!(result.missing_definition_rules.is_empty());
    }

    #[test]
    fn test_validate_keyword_coverage_missing_usage() {
        let keywords = vec!["part".to_string()];

        let grammar = r#"
// PartUsage is missing!
PartDefinition = { KW_PART ~ KW_DEF }
"#;

        let result = validate_keyword_coverage(&keywords, grammar);

        assert!(!result.is_valid());
        assert!(result.missing_usage_rules.contains(&"PartUsage".to_string()));
    }

    #[test]
    fn test_validate_keyword_coverage_missing_definition() {
        let keywords = vec!["part".to_string()];

        let grammar = r#"
PartUsage = { KW_PART }
// PartDefinition is missing!
"#;

        let result = validate_keyword_coverage(&keywords, grammar);

        assert!(!result.is_valid());
        assert!(result
            .missing_definition_rules
            .contains(&"PartDefinition".to_string()));
    }

    #[test]
    fn test_validate_usage_only_keyword() {
        let keywords = vec!["ref".to_string()];

        let grammar = r#"
ReferenceUsage = { KW_REF }
"#;

        let result = validate_keyword_coverage(&keywords, grammar);

        // Should pass - ref only needs usage rule, not definition
        assert!(result.is_valid());
    }

    #[test]
    fn test_message_has_no_definition_expected() {
        let keywords = vec!["message".to_string()];

        let grammar = r#"
MessageUsage = { KW_MESSAGE }
// No MessageDefinition needed
"#;

        let result = validate_keyword_coverage(&keywords, grammar);

        // Should pass - message doesn't require a definition rule
        assert!(result.is_valid());
    }

    #[test]
    fn test_validation_report_format() {
        let keywords = vec!["part".to_string()];

        let grammar = ""; // Empty grammar - everything missing

        let result = validate_keyword_coverage(&keywords, grammar);
        let report = result.format_report();

        assert!(report.contains("FAILED"));
        assert!(report.contains("PartUsage"));
        assert!(report.contains("PartDefinition"));
    }

    #[test]
    fn test_get_keyword_classification_summary() {
        let keywords = vec![
            "part".to_string(),
            "action".to_string(),
            "abstract".to_string(),
            "and".to_string(),
            "if".to_string(),
            "entry".to_string(),
            "unknown_kw".to_string(),
        ];

        let summary = get_keyword_classification_summary(&keywords);

        assert!(summary.get("UsageDefinition").unwrap().contains(&"part".to_string()));
        assert!(summary.get("Modifier").unwrap().contains(&"abstract".to_string()));
        assert!(summary.get("Operator").unwrap().contains(&"and".to_string()));
        assert!(summary.get("Control").unwrap().contains(&"if".to_string()));
        assert!(summary.get("Contextual").unwrap().contains(&"entry".to_string()));
        assert!(summary.get("Other").unwrap().contains(&"unknown_kw".to_string()));
    }
}
