//! Xtext grammar parser for extracting operators, keywords, and rules.
//!
//! This module parses Xtext grammar files (`.xtext`) to extract:
//! - Operators with precedence information
//! - Keywords defined in the grammar
//! - Enum definitions
//! - Grammar rules
//!
//! The primary source is `KerMLExpressions.xtext` for operators and `SysML.xtext`
//! for keywords and domain-specific rules.

use std::collections::HashMap;

/// Information about an operator extracted from xtext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorInfo {
    /// The rule name (e.g., "EqualityOperator").
    pub name: String,
    /// The operator symbols (e.g., ["==", "!=", "===", "!=="]).
    pub symbols: Vec<String>,
    /// Category derived from name (e.g., "equality").
    pub category: String,
    /// Precedence level (higher = binds tighter).
    pub precedence: u8,
}

/// Information about a keyword extracted from xtext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordInfo {
    /// The rule name (e.g., "StateKeyword").
    pub name: String,
    /// The keyword string (e.g., "state").
    pub keyword: String,
}

/// Information about an enum defined in xtext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtextEnumInfo {
    /// The enum name (e.g., "FeatureDirection").
    pub name: String,
    /// The returns type (e.g., "SysML::FeatureDirectionKind").
    pub returns_type: String,
    /// The values as (variant_name, keyword) pairs.
    pub values: Vec<(String, String)>,
}

/// Information about a grammar rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtextRule {
    /// The rule name.
    pub name: String,
    /// The returns type, if specified.
    pub returns_type: Option<String>,
    /// Whether this is a fragment rule.
    pub is_fragment: bool,
    /// Whether this is a terminal rule.
    pub is_terminal: bool,
}

/// Precedence levels for operators (from lowest to highest binding).
/// These values come from the structure of KerMLExpressions.xtext.
const PRECEDENCE_MAP: &[(&str, u8)] = &[
    ("ConditionalOperator", 1),
    ("NullCoalescingOperator", 2),
    ("ImpliesOperator", 3),
    ("OrOperator", 4),
    ("ConditionalOrOperator", 4),
    ("XorOperator", 5),
    ("AndOperator", 6),
    ("ConditionalAndOperator", 6),
    ("EqualityOperator", 7),
    ("ClassificationTestOperator", 8),
    ("MetaClassificationTestOperator", 9),
    ("CastOperator", 10),
    ("MetaCastOperator", 10),
    ("RelationalOperator", 11),
    ("RangeOperator", 12),
    ("AdditiveOperator", 13),
    ("MultiplicativeOperator", 14),
    ("ExponentiationOperator", 15),
    ("UnaryOperator", 16),
];

/// Parse operators from an xtext grammar file (typically KerMLExpressions.xtext).
///
/// Looks for patterns like:
/// ```text
/// EqualityOperator :
///     '==' | '!=' | '===' | '!=='
/// ;
/// ```
///
/// # Arguments
///
/// * `content` - The xtext file content as a string
///
/// # Returns
///
/// A vector of `OperatorInfo` structs with precedence information.
pub fn parse_xtext_operators(content: &str) -> Vec<OperatorInfo> {
    let mut operators = Vec::new();
    let precedence_lookup: HashMap<&str, u8> = PRECEDENCE_MAP.iter().cloned().collect();

    // Match operator rule definitions
    // Pattern: OperatorName : 'symbol' | 'symbol' ... ;
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is an operator rule definition
        if line.ends_with("Operator :") || line.ends_with("Operator:") {
            let name = line
                .trim_end_matches(':')
                .trim_end_matches(" :")
                .trim()
                .to_string();

            // Collect the operator body (may span multiple lines)
            let mut body = String::new();
            i += 1;
            while i < lines.len() {
                let body_line = lines[i].trim();
                body.push_str(body_line);
                body.push(' ');
                if body_line.ends_with(';') {
                    break;
                }
                i += 1;
            }

            // Extract symbols from the body
            let symbols = extract_symbols(&body);

            if !symbols.is_empty() {
                let category = derive_category(&name);
                let precedence = precedence_lookup.get(name.as_str()).copied().unwrap_or(0);

                operators.push(OperatorInfo {
                    name,
                    symbols,
                    category,
                    precedence,
                });
            }
        }
        i += 1;
    }

    // Sort by precedence
    operators.sort_by_key(|op| op.precedence);
    operators
}

/// Parse keywords from an xtext grammar file (typically SysML.xtext).
///
/// Looks for patterns like:
/// ```text
/// StateKeyword :
///     'state'
/// ;
/// ```
///
/// # Arguments
///
/// * `content` - The xtext file content as a string
///
/// # Returns
///
/// A vector of `KeywordInfo` structs.
pub fn parse_xtext_keywords(content: &str) -> Vec<KeywordInfo> {
    let mut keywords = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a keyword rule definition
        if line.ends_with("Keyword :") || line.ends_with("Keyword:") {
            let name = line
                .trim_end_matches(':')
                .trim_end_matches(" :")
                .trim()
                .to_string();

            // Collect the keyword body (may span multiple lines)
            let mut body = String::new();
            i += 1;
            while i < lines.len() {
                let body_line = lines[i].trim();
                body.push_str(body_line);
                body.push(' ');
                if body_line.ends_with(';') {
                    break;
                }
                i += 1;
            }

            // Extract the keyword from the body
            if let Some(keyword) = extract_first_keyword(&body) {
                keywords.push(KeywordInfo { name, keyword });
            }
        }
        i += 1;
    }

    keywords.sort_by(|a, b| a.name.cmp(&b.name));
    keywords
}

/// Extract all unique quoted keyword-like strings from xtext content.
///
/// This function scans the entire xtext file for quoted strings that look
/// like keywords (alphabetic identifiers) and returns them. This is useful
/// for finding keywords that are used inline rather than in dedicated rules.
///
/// # Arguments
///
/// * `content` - The xtext file content as a string
///
/// # Returns
///
/// A vector of unique keyword strings, sorted alphabetically.
pub fn extract_all_keyword_strings(content: &str) -> Vec<String> {
    let mut keywords = std::collections::HashSet::new();

    // Extract all single-quoted strings that are alphabetic identifiers
    let mut in_quote = false;
    let mut current = String::new();

    for ch in content.chars() {
        if ch == '\'' {
            if in_quote {
                // Check if the extracted string is a valid keyword (alphabetic)
                if !current.is_empty()
                    && current.chars().all(|c| c.is_ascii_alphabetic())
                    && current.len() > 1
                {
                    keywords.insert(current.clone());
                }
                current.clear();
            }
            in_quote = !in_quote;
        } else if in_quote {
            current.push(ch);
        }
    }

    let mut result: Vec<_> = keywords.into_iter().collect();
    result.sort();
    result
}

/// Parse enum definitions from an xtext grammar file.
///
/// Looks for patterns like:
/// ```text
/// enum FeatureDirection returns SysML::FeatureDirectionKind :
///     in | out | inout | ...
/// ;
/// ```
///
/// # Arguments
///
/// * `content` - The xtext file content as a string
///
/// # Returns
///
/// A vector of `XtextEnumInfo` structs.
pub fn parse_xtext_enums(content: &str) -> Vec<XtextEnumInfo> {
    let mut enums = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is an enum definition
        if line.starts_with("enum ") {
            // Parse: enum Name returns Type :
            // Find the colon that separates the header from the body (not ::)
            if let Some(colon_pos) = find_rule_colon(line) {
                let header = &line[5..colon_pos].trim(); // skip "enum "
                let parts: Vec<&str> = header.split_whitespace().collect();

                if parts.len() >= 3 && parts[1] == "returns" {
                    let name = parts[0].to_string();
                    let returns_type = parts[2..].join(" ");

                    // Get anything after the colon on the same line
                    let mut body = line[colon_pos + 1..].to_string();

                    // Collect the rest of the enum body
                    i += 1;
                    while i < lines.len() {
                        let body_line = lines[i].trim();
                        body.push_str(body_line);
                        body.push(' ');
                        if body_line.ends_with(';') {
                            break;
                        }
                        i += 1;
                    }

                    // Parse enum values
                    let values = parse_enum_values(&body);

                    enums.push(XtextEnumInfo {
                        name,
                        returns_type,
                        values,
                    });
                }
            }
        }
        i += 1;
    }

    enums
}

/// Parse grammar rules from an xtext grammar file.
///
/// Extracts rule names, return types, and whether they are fragments or terminals.
///
/// # Arguments
///
/// * `content` - The xtext file content as a string
///
/// # Returns
///
/// A vector of `XtextRule` structs.
pub fn parse_xtext_rules(content: &str) -> Vec<XtextRule> {
    let mut rules = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }

        // Terminal rule: terminal NAME ...
        if trimmed.starts_with("terminal ") {
            let name = trimmed
                .strip_prefix("terminal ")
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            if !name.is_empty() {
                rules.push(XtextRule {
                    name,
                    returns_type: None,
                    is_fragment: false,
                    is_terminal: true,
                });
            }
            continue;
        }

        // Fragment rule: fragment NAME returns TYPE :
        if trimmed.starts_with("fragment ") {
            if let Some((name, returns_type)) = parse_rule_header(trimmed.strip_prefix("fragment ").unwrap()) {
                rules.push(XtextRule {
                    name,
                    returns_type,
                    is_fragment: true,
                    is_terminal: false,
                });
            }
            continue;
        }

        // Regular rule: NAME returns TYPE : or NAME :
        if let Some((name, returns_type)) = parse_rule_header(trimmed) {
            // Skip operator and keyword rules (they're handled separately)
            if name.ends_with("Operator") || name.ends_with("Keyword") {
                continue;
            }
            rules.push(XtextRule {
                name,
                returns_type,
                is_fragment: false,
                is_terminal: false,
            });
        }
    }

    rules
}

/// Extract quoted symbols from a rule body.
fn extract_symbols(body: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    let mut in_quote = false;
    let mut current = String::new();

    for ch in body.chars() {
        if ch == '\'' {
            if in_quote {
                if !current.is_empty() {
                    symbols.push(current.clone());
                }
                current.clear();
            }
            in_quote = !in_quote;
        } else if in_quote {
            current.push(ch);
        }
    }

    symbols
}

/// Extract the first quoted keyword from a rule body.
fn extract_first_keyword(body: &str) -> Option<String> {
    let symbols = extract_symbols(body);
    symbols.into_iter().next()
}

/// Derive category from operator name.
fn derive_category(name: &str) -> String {
    // Remove "Operator" suffix and convert to snake_case
    let base = name.trim_end_matches("Operator");
    let mut category = String::new();
    for (i, ch) in base.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            category.push('_');
        }
        category.push(ch.to_lowercase().next().unwrap_or(ch));
    }
    category
}

/// Parse enum values from the body of an enum definition.
fn parse_enum_values(body: &str) -> Vec<(String, String)> {
    let mut values = Vec::new();

    // Remove semicolon (may have spaces around it) and split by |
    let cleaned = body.trim().trim_end_matches(';').trim();
    for part in cleaned.split('|') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // Parse: variant = 'keyword' or just 'keyword' or just identifier
        if part.contains('=') {
            let parts: Vec<&str> = part.split('=').collect();
            if parts.len() == 2 {
                let variant = parts[0].trim().to_string();
                // Try quoted keyword first, then fall back to identifier
                let value = extract_first_keyword(parts[1])
                    .unwrap_or_else(|| parts[1].trim().to_string());
                if !value.is_empty() {
                    values.push((variant, value));
                }
            }
        } else if let Some(keyword) = extract_first_keyword(part) {
            // Quoted keyword: use as both variant and value
            values.push((keyword.clone(), keyword));
        } else {
            // Bare identifier: use as both variant and value
            let ident = part.trim().to_string();
            if !ident.is_empty() && ident.chars().all(|c| c.is_alphanumeric() || c == '_') {
                values.push((ident.clone(), ident));
            }
        }
    }

    values
}

/// Parse a rule header to extract name and optional returns type.
fn parse_rule_header(line: &str) -> Option<(String, Option<String>)> {
    let trimmed = line.trim();

    // Must contain : to be a rule definition (but not ::)
    // Find the first standalone : (not part of ::)
    let colon_pos = find_rule_colon(trimmed)?;
    let before_colon = &trimmed[..colon_pos];

    // Check for "returns" clause
    if before_colon.contains(" returns ") {
        let parts: Vec<&str> = before_colon.split(" returns ").collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let returns_type = Some(parts[1].trim().to_string());
            if is_valid_rule_name(&name) {
                return Some((name, returns_type));
            }
        }
    } else {
        // Simple rule without returns clause
        let name = before_colon.split_whitespace().next()?.to_string();
        if is_valid_rule_name(&name) {
            return Some((name, None));
        }
    }

    None
}

/// Find the position of the rule definition colon (not part of ::).
fn find_rule_colon(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        if ch == ':' {
            // Check if this is part of :: (namespace separator)
            let is_double_colon = (i > 0 && chars.get(i - 1) == Some(&':'))
                || chars.get(i + 1) == Some(&':');
            if !is_double_colon {
                return Some(i);
            }
        }
    }
    None
}

/// Check if a string is a valid rule name.
fn is_valid_rule_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().next().map_or(false, |c| c.is_uppercase())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_symbols() {
        let body = "'==' | '!=' | '===' | '!=='";
        let symbols = extract_symbols(body);
        assert_eq!(symbols, vec!["==", "!=", "===", "!=="]);
    }

    #[test]
    fn test_extract_symbols_single() {
        let body = "'if'";
        let symbols = extract_symbols(body);
        assert_eq!(symbols, vec!["if"]);
    }

    #[test]
    fn test_derive_category() {
        assert_eq!(derive_category("EqualityOperator"), "equality");
        assert_eq!(derive_category("NullCoalescingOperator"), "null_coalescing");
        assert_eq!(derive_category("ConditionalAndOperator"), "conditional_and");
    }

    #[test]
    fn test_parse_operators() {
        let content = r#"
ConditionalOperator :
    'if'
;

EqualityOperator :
    '==' | '!=' | '===' | '!=='
;
"#;
        let operators = parse_xtext_operators(content);
        assert_eq!(operators.len(), 2);

        // Sorted by precedence
        assert_eq!(operators[0].name, "ConditionalOperator");
        assert_eq!(operators[0].symbols, vec!["if"]);
        assert_eq!(operators[0].category, "conditional");
        assert_eq!(operators[0].precedence, 1);

        assert_eq!(operators[1].name, "EqualityOperator");
        assert_eq!(operators[1].symbols, vec!["==", "!=", "===", "!=="]);
        assert_eq!(operators[1].category, "equality");
        assert_eq!(operators[1].precedence, 7);
    }

    #[test]
    fn test_parse_keywords() {
        let content = r#"
StateKeyword :
    'state'
;

ActionKeyword :
    'action'
;
"#;
        let keywords = parse_xtext_keywords(content);
        assert_eq!(keywords.len(), 2);

        assert_eq!(keywords[0].name, "ActionKeyword");
        assert_eq!(keywords[0].keyword, "action");

        assert_eq!(keywords[1].name, "StateKeyword");
        assert_eq!(keywords[1].keyword, "state");
    }

    #[test]
    fn test_parse_enums() {
        let content = r#"
enum FeatureDirection returns SysML::FeatureDirectionKind :
    in | out | inout
;
"#;
        let enums = parse_xtext_enums(content);
        assert_eq!(enums.len(), 1);

        assert_eq!(enums[0].name, "FeatureDirection");
        assert_eq!(enums[0].returns_type, "SysML::FeatureDirectionKind");
        assert_eq!(
            enums[0].values,
            vec![
                ("in".to_string(), "in".to_string()),
                ("out".to_string(), "out".to_string()),
                ("inout".to_string(), "inout".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_rules() {
        let content = r#"
Package returns SysML::Package :
    PackageDeclaration PackageBody
;

terminal DECIMAL_VALUE returns Ecore::EInt:
    '0'..'9' ('0'..'9')*;

fragment Identification returns SysML::Element :
    '<' declaredShortName = Name '>'
;
"#;
        let rules = parse_xtext_rules(content);

        // Find the rules by name
        let package = rules.iter().find(|r| r.name == "Package");
        assert!(package.is_some());
        let package = package.unwrap();
        assert_eq!(package.returns_type, Some("SysML::Package".to_string()));
        assert!(!package.is_fragment);
        assert!(!package.is_terminal);

        let decimal = rules.iter().find(|r| r.name == "DECIMAL_VALUE");
        assert!(decimal.is_some());
        let decimal = decimal.unwrap();
        assert!(decimal.is_terminal);

        let ident = rules.iter().find(|r| r.name == "Identification");
        assert!(ident.is_some());
        let ident = ident.unwrap();
        assert!(ident.is_fragment);
    }

    #[test]
    fn test_parse_rule_header() {
        let (name, returns) = parse_rule_header("Package returns SysML::Package :").unwrap();
        assert_eq!(name, "Package");
        assert_eq!(returns, Some("SysML::Package".to_string()));

        let (name, returns) = parse_rule_header("SimpleRule :").unwrap();
        assert_eq!(name, "SimpleRule");
        assert_eq!(returns, None);

        assert!(parse_rule_header("// comment").is_none());
        assert!(parse_rule_header("no colon here").is_none());
    }
}
