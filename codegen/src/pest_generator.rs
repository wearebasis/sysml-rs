//! Pest grammar generation from xtext specifications.
//!
//! This module generates pest grammar rules from parsed xtext data,
//! specifically for keywords, operators, and enums.
//!
//! The generated pest rules are designed to be concatenated with manually
//! written grammar fragments to produce the complete sysml.pest grammar.

use crate::xtext_parser::{KeywordInfo, OperatorInfo, XtextEnumInfo};

/// Generate pest keyword rules from xtext keyword definitions.
///
/// Each keyword is converted to an atomic rule with word boundary checking:
/// ```pest
/// KW_ABSTRACT = @{ "abstract" ~ !(ASCII_ALPHANUMERIC | "_") }
/// ```
///
/// The word boundary check prevents partial matching (e.g., "in" matching
/// the start of "interface" or "inout").
///
/// # Arguments
///
/// * `keywords` - Keyword information extracted from xtext files
///
/// # Returns
///
/// A string containing pest grammar rules for all keywords.
pub fn generate_pest_keywords(keywords: &[KeywordInfo]) -> String {
    let mut output = String::new();
    output.push_str("// =============================================================================\n");
    output.push_str("// KEYWORDS (Generated from SysML.xtext)\n");
    output.push_str("// Keywords use word boundary checking to prevent partial matching.\n");
    output.push_str("// =============================================================================\n\n");

    // Collect unique keywords (some may be defined in multiple rules)
    let mut unique_keywords: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();

    for kw in keywords {
        // Transform rule name like "StateKeyword" to "KW_STATE"
        let pest_name = keyword_to_pest_name(&kw.name);
        unique_keywords.insert(pest_name, kw.keyword.clone());
    }

    // Generate the rules, sorted alphabetically by pest name
    for (pest_name, keyword) in &unique_keywords {
        // Use atomic rule (@) with word boundary check to prevent partial matching
        output.push_str(&format!(
            "{} = @{{ \"{}\" ~ !(ASCII_ALPHANUMERIC | \"_\") }}\n",
            pest_name, keyword
        ));
    }

    output
}

/// Generate pest keyword rules from a simple list of keyword strings.
///
/// This is a more comprehensive alternative to `generate_pest_keywords` that
/// works with keyword strings directly, useful when keywords are extracted
/// from all quoted strings in xtext rather than just dedicated keyword rules.
///
/// Each keyword is converted to an atomic rule with word boundary checking:
/// ```pest
/// KW_ABSTRACT = @{ "abstract" ~ !(ASCII_ALPHANUMERIC | "_") }
/// ```
///
/// The word boundary check prevents partial matching (e.g., "in" matching
/// the start of "interface" or "inout").
///
/// # Arguments
///
/// * `keywords` - A list of keyword strings
///
/// # Returns
///
/// A string containing pest grammar rules for all keywords.
pub fn generate_pest_keywords_from_strings(keywords: &[String]) -> String {
    let mut output = String::new();
    output.push_str("// =============================================================================\n");
    output.push_str("// KEYWORDS (Generated from SysML/KerML xtext files)\n");
    output.push_str("// Keywords use word boundary checking to prevent partial matching.\n");
    output.push_str("// =============================================================================\n\n");

    // Generate rules sorted alphabetically
    let mut sorted_keywords = keywords.to_vec();
    sorted_keywords.sort();

    for keyword in &sorted_keywords {
        let pest_name = format!("KW_{}", keyword.to_uppercase());
        // Use atomic rule (@) with word boundary check to prevent partial matching
        // e.g., "in" should not match the start of "interface" or "inout"
        output.push_str(&format!(
            "{} = @{{ \"{}\" ~ !(ASCII_ALPHANUMERIC | \"_\") }}\n",
            pest_name, keyword
        ));
    }

    output
}

/// Generate pest operator rules from xtext operator definitions.
///
/// Operators are grouped into rules with alternatives, sorted longest-first
/// for proper PEG matching. For example:
/// ```pest
/// EqualityOperator = { "===" | "!==" | "==" | "!=" }
/// ```
///
/// Alphabetic operators (like "not", "as", "meta") use keyword references
/// instead of string literals for consistency with the keyword system.
///
/// # Arguments
///
/// * `operators` - Operator information extracted from xtext files
///
/// # Returns
///
/// A string containing pest grammar rules for all operators.
pub fn generate_pest_operators(operators: &[OperatorInfo]) -> String {
    let mut output = String::new();
    output.push_str("// =============================================================================\n");
    output.push_str("// OPERATORS (Generated from KerMLExpressions.xtext)\n");
    output.push_str("// =============================================================================\n\n");

    for op in operators {
        // Sort symbols longest-first for proper PEG matching
        let mut symbols = op.symbols.clone();
        symbols.sort_by(|a, b| b.len().cmp(&a.len()));

        // Format alternatives - use keyword refs for alphabetic symbols
        let alternatives: Vec<String> = symbols
            .iter()
            .map(|s| {
                if s.chars().all(|c| c.is_ascii_alphabetic()) {
                    // Alphabetic symbol - use keyword reference
                    format!("KW_{}", s.to_uppercase())
                } else {
                    // Symbol with special characters - use string literal
                    format!("\"{}\"", escape_pest_string(s))
                }
            })
            .collect();

        output.push_str(&format!(
            "{} = {{ {} }}\n",
            op.name,
            alternatives.join(" | ")
        ));
    }

    output
}

/// Generate pest enum rules from xtext enum definitions.
///
/// Enums are converted to rules with keyword alternatives. For example:
/// ```pest
/// VisibilityKind = { KW_PUBLIC | KW_PRIVATE | KW_PROTECTED }
/// ```
///
/// Multiple enums with the same return type are merged into a single rule.
/// Non-keyword values (like brackets or other symbols) are filtered out.
///
/// # Arguments
///
/// * `enums` - Enum information extracted from xtext files
///
/// # Returns
///
/// A string containing pest grammar rules for all enums.
pub fn generate_pest_enums(enums: &[XtextEnumInfo]) -> String {
    let mut output = String::new();
    output.push_str("// =============================================================================\n");
    output.push_str("// ENUMS (Generated from SysML.xtext)\n");
    output.push_str("// =============================================================================\n\n");

    // Group enums by return type (to merge duplicates)
    let mut type_to_keywords: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> =
        std::collections::BTreeMap::new();

    for enum_info in enums {
        // Extract the simple type name from the full path
        let type_name = extract_type_name(&enum_info.returns_type);

        let keywords = type_to_keywords.entry(type_name).or_default();

        for (_, keyword) in &enum_info.values {
            // Filter out non-keyword values (must be alphabetic identifiers)
            if is_valid_keyword(keyword) {
                keywords.insert(format!("KW_{}", keyword.to_uppercase()));
            }
        }
    }

    // Generate rules for each unique type
    for (type_name, keywords) in type_to_keywords {
        if !keywords.is_empty() {
            // Sort keywords by length (descending) for proper PEG matching
            // This ensures "KW_INOUT" comes before "KW_IN" since "in" is a prefix of "inout"
            let mut alternatives: Vec<_> = keywords.into_iter().collect();
            alternatives.sort_by(|a, b| b.len().cmp(&a.len()));
            output.push_str(&format!(
                "{} = {{ {} }}\n",
                type_name,
                alternatives.join(" | ")
            ));
        }
    }

    output
}

/// Check if a string is a valid keyword (alphabetic identifier).
fn is_valid_keyword(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}

/// Transform xtext keyword rule name to pest KW_* naming convention.
///
/// Examples:
/// - "StateKeyword" -> "KW_STATE"
/// - "PartDefKeyword" -> "KW_PART_DEF" (actually just extracts the keyword)
fn keyword_to_pest_name(xtext_name: &str) -> String {
    // Remove "Keyword" suffix
    let base = xtext_name.trim_end_matches("Keyword");

    // Convert PascalCase to SCREAMING_SNAKE_CASE
    let mut result = String::from("KW_");
    for (i, ch) in base.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_uppercase().next().unwrap_or(ch));
    }
    result
}

/// Extract simple type name from fully qualified xtext type.
///
/// Examples:
/// - "SysML::VisibilityKind" -> "VisibilityKind"
/// - "SysML::FeatureDirectionKind" -> "FeatureDirectionKind"
fn extract_type_name(full_type: &str) -> String {
    full_type
        .rsplit("::")
        .next()
        .unwrap_or(full_type)
        .to_string()
}

/// Escape special characters in pest string literals.
fn escape_pest_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Extract all keywords from xtext content.
///
/// This function finds all rules ending in "Keyword" and extracts
/// the quoted string they define.
pub fn extract_all_keywords(sysml_xtext: &str, kerml_xtext: &str) -> Vec<KeywordInfo> {
    let mut keywords = Vec::new();

    // Parse both SysML and KerML xtext files
    keywords.extend(crate::xtext_parser::parse_xtext_keywords(sysml_xtext));
    keywords.extend(crate::xtext_parser::parse_xtext_keywords(kerml_xtext));

    // Remove duplicates based on keyword string
    let mut seen = std::collections::HashSet::new();
    keywords.retain(|k| seen.insert(k.keyword.clone()));

    // Sort by keyword name
    keywords.sort_by(|a, b| a.keyword.cmp(&b.keyword));

    keywords
}

/// Extract all operators from xtext content.
pub fn extract_all_operators(kerml_expr_xtext: &str) -> Vec<OperatorInfo> {
    crate::xtext_parser::parse_xtext_operators(kerml_expr_xtext)
}

/// Extract all enums from xtext content.
pub fn extract_all_enums(sysml_xtext: &str) -> Vec<XtextEnumInfo> {
    crate::xtext_parser::parse_xtext_enums(sysml_xtext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_to_pest_name() {
        assert_eq!(keyword_to_pest_name("StateKeyword"), "KW_STATE");
        assert_eq!(keyword_to_pest_name("ActionKeyword"), "KW_ACTION");
        assert_eq!(keyword_to_pest_name("PartDefKeyword"), "KW_PART_DEF");
        assert_eq!(keyword_to_pest_name("UseCaseKeyword"), "KW_USE_CASE");
    }

    #[test]
    fn test_extract_type_name() {
        assert_eq!(extract_type_name("SysML::VisibilityKind"), "VisibilityKind");
        assert_eq!(
            extract_type_name("SysML::FeatureDirectionKind"),
            "FeatureDirectionKind"
        );
        assert_eq!(extract_type_name("SimpleType"), "SimpleType");
    }

    #[test]
    fn test_generate_pest_keywords() {
        let keywords = vec![
            KeywordInfo {
                name: "StateKeyword".to_string(),
                keyword: "state".to_string(),
            },
            KeywordInfo {
                name: "ActionKeyword".to_string(),
                keyword: "action".to_string(),
            },
        ];

        let output = generate_pest_keywords(&keywords);
        // Keywords should have word boundary checking with atomic rule (@)
        assert!(output.contains("KW_ACTION = @{ \"action\" ~ !(ASCII_ALPHANUMERIC | \"_\") }"));
        assert!(output.contains("KW_STATE = @{ \"state\" ~ !(ASCII_ALPHANUMERIC | \"_\") }"));
    }

    #[test]
    fn test_generate_pest_operators() {
        let operators = vec![OperatorInfo {
            name: "EqualityOperator".to_string(),
            symbols: vec![
                "==".to_string(),
                "!=".to_string(),
                "===".to_string(),
                "!==".to_string(),
            ],
            category: "equality".to_string(),
            precedence: 7,
        }];

        let output = generate_pest_operators(&operators);
        // Should have longest symbols first
        assert!(output.contains("\"===\""));
        assert!(output.contains("\"!==\""));
    }

    #[test]
    fn test_generate_pest_enums() {
        let enums = vec![XtextEnumInfo {
            name: "FeatureDirection".to_string(),
            returns_type: "SysML::FeatureDirectionKind".to_string(),
            values: vec![
                ("in".to_string(), "in".to_string()),
                ("out".to_string(), "out".to_string()),
                ("inout".to_string(), "inout".to_string()),
            ],
        }];

        let output = generate_pest_enums(&enums);
        // Enum values are sorted by length descending for proper PEG matching
        // (longer keywords first to prevent prefix matching issues)
        assert!(output.contains("FeatureDirectionKind = { KW_INOUT | KW_OUT | KW_IN }"));
    }
}
