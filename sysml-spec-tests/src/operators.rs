//! Operator coverage tracking derived from xtext grammar.
//!
//! This module loads operators from the xtext grammar files using the codegen
//! crate's xtext parser. It provides the authoritative list of operators
//! defined in the SysML v2 specification.
//!
//! **Note**: This module requires the SysML v2 references directory to be available.
//! Set SYSML_CORPUS_PATH or ensure the references are in a standard location.

use std::path::Path;

use sysml_codegen::{parse_xtext_operators, OperatorInfo};

use crate::find_references_dir;

/// Path to KerMLExpressions.xtext relative to the references root.
pub const KERML_EXPRESSIONS_PATH: &str =
    "SysML-v2-Pilot-Implementation/org.omg.kerml.expressions.xtext/\
     src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext";

/// Load operators from the xtext grammar file.
///
/// This uses the codegen crate's xtext parser to get the authoritative list
/// of operators from the KerMLExpressions.xtext file.
///
/// # Arguments
///
/// * `references_path` - Path to the SysML v2 references directory
///
/// # Returns
///
/// A vector of `OperatorInfo` structs with precedence information.
pub fn load_operators_from_spec(references_path: &Path) -> std::io::Result<Vec<OperatorInfo>> {
    let path = references_path.join(KERML_EXPRESSIONS_PATH);
    let content = std::fs::read_to_string(&path).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Failed to read {}: {}", path.display(), e),
        )
    })?;
    Ok(parse_xtext_operators(&content))
}

/// Load operators from the spec files.
///
/// This function locates the SysML v2 references directory and loads operators
/// from the KerMLExpressions.xtext file.
///
/// # Panics
///
/// Panics if the references directory cannot be found or the xtext file cannot be parsed.
/// This ensures we fail fast with a clear error rather than using stale data.
pub fn load_operators() -> Vec<OperatorInfo> {
    let refs_dir = find_references_dir();
    load_operators_from_spec(&refs_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to load operators from spec: {}\n\
             Ensure {} exists in the references directory.",
            e, KERML_EXPRESSIONS_PATH
        )
    })
}

/// Get all unique operator symbols.
pub fn all_operator_symbols() -> Vec<String> {
    let operators = load_operators();
    let mut symbols: Vec<String> = operators
        .into_iter()
        .flat_map(|op| op.symbols)
        .collect();
    symbols.sort();
    symbols.dedup();
    symbols
}

/// Get operator symbols by category.
pub fn operators_by_category() -> std::collections::HashMap<String, Vec<String>> {
    let operators = load_operators();
    let mut by_category = std::collections::HashMap::new();

    for op in operators {
        by_category
            .entry(op.category)
            .or_insert_with(Vec::new)
            .extend(op.symbols);
    }

    by_category
}

#[cfg(test)]
mod tests {
    use super::load_operators_from_spec;
    use crate::try_find_references_dir;

    /// Skip test if spec files not available.
    fn require_spec_files() -> std::path::PathBuf {
        try_find_references_dir().expect(
            "Spec files required for this test. Set SYSML_CORPUS_PATH or \
             ensure references/sysmlv2 is available.",
        )
    }

    #[test]
    fn test_load_operators_from_spec() {
        let refs_dir = require_spec_files();
        let operators =
            load_operators_from_spec(&refs_dir).expect("Should load operators from spec");

        // Verify we got operators
        assert!(!operators.is_empty(), "Should have loaded operators");
        assert!(
            operators.len() >= 15,
            "Expected at least 15 operators, got {}",
            operators.len()
        );

        // Verify known operators are present
        let names: Vec<_> = operators.iter().map(|o| o.name.as_str()).collect();
        assert!(names.contains(&"EqualityOperator"));
        assert!(names.contains(&"RelationalOperator"));
        assert!(names.contains(&"AdditiveOperator"));
        assert!(names.contains(&"UnaryOperator"));

        // Verify precedence ordering
        for i in 1..operators.len() {
            assert!(
                operators[i - 1].precedence <= operators[i].precedence,
                "Operators should be sorted by precedence"
            );
        }
    }

    #[test]
    fn operators_have_symbols() {
        let refs_dir = require_spec_files();
        let operators =
            load_operators_from_spec(&refs_dir).expect("Should load operators from spec");

        for op in &operators {
            assert!(
                !op.symbols.is_empty(),
                "Operator {} should have at least one symbol",
                op.name
            );
        }

        // Check specific operators have expected symbols
        let equality = operators.iter().find(|o| o.name == "EqualityOperator");
        assert!(equality.is_some());
        let equality = equality.unwrap();
        assert!(equality.symbols.contains(&"==".to_string()));
        assert!(equality.symbols.contains(&"!=".to_string()));
    }

    #[test]
    fn all_symbols_extracted() {
        let refs_dir = require_spec_files();
        let operators =
            load_operators_from_spec(&refs_dir).expect("Should load operators from spec");

        let all_symbols: Vec<_> = operators.iter().flat_map(|o| &o.symbols).collect();

        // Should have common operators
        assert!(all_symbols.contains(&&"==".to_string()));
        assert!(all_symbols.contains(&&"+".to_string()));
        assert!(all_symbols.contains(&&"-".to_string()));
        assert!(all_symbols.contains(&&"*".to_string()));
        assert!(all_symbols.contains(&&"<".to_string()));
    }

    #[test]
    fn operators_by_category_works() {
        let refs_dir = require_spec_files();
        let operators = load_operators_from_spec(&refs_dir).expect("Should load operators");

        let mut by_category = std::collections::HashMap::new();
        for op in operators {
            by_category
                .entry(op.category)
                .or_insert_with(Vec::new)
                .extend(op.symbols);
        }

        assert!(!by_category.is_empty());
        assert!(by_category.contains_key("equality"));
        assert!(by_category.contains_key("relational"));
        assert!(by_category.contains_key("additive"));
    }
}
