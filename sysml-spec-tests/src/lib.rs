//! # sysml-spec-tests
//!
//! Parser coverage validation for the SysML v2 parser.
//!
//! This crate provides comprehensive coverage testing to track how much of the
//! SysML v2 language specification is covered by the `sysml-text-pest` parser.
//!
//! ## Coverage Dimensions
//!
//! - **Corpus Files**: Parse real-world .sysml files from reference materials
//! - **Grammar Rules**: Track which pest rules are exercised
//! - **ElementKinds**: Verify all parseable types are produced
//! - **Operators**: Test expression parsing
//!
//! ## Usage
//!
//! Tests are `#[ignore]` by default and enabled via environment variable:
//!
//! ```bash
//! # Enable corpus tests
//! SYSML_CORPUS_PATH=/path/to/sysmlv2-references cargo test -p sysml-spec-tests -- --ignored
//! ```

pub mod corpus;
pub mod element_coverage;
pub mod grammar_rules;
pub mod operators;
pub mod report;
pub mod rule_coverage;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Locate the sysmlv2-references directory.
///
/// Searches in order:
/// 1. SYSML_CORPUS_PATH environment variable
/// 2. Common relative paths (../sysmlv2-references, ../../sysmlv2-references, etc.)
///
/// # Panics
///
/// Panics if the references directory cannot be found. This ensures we fail fast
/// with a clear error message rather than silently using stale fallback data.
pub fn find_references_dir() -> PathBuf {
    // Check environment variable first
    if let Ok(env_path) = std::env::var("SYSML_CORPUS_PATH") {
        let path = PathBuf::from(&env_path);
        if path.exists() {
            return path;
        }
    }

    // Try common relative paths
    let candidates = [
        "../sysmlv2-references",
        "../../sysmlv2-references",
        "sysmlv2-references",
        "../spec",
        "spec",
    ];

    for candidate in candidates {
        let path = Path::new(candidate);
        if path.exists() && path.join("SysML-vocab.ttl").exists() {
            return path.to_path_buf();
        }
    }

    panic!(
        "Could not find sysmlv2-references directory.\n\
         Set SYSML_CORPUS_PATH environment variable or ensure sysmlv2-references is available.\n\
         Searched: {:?}",
        candidates
    );
}

/// Check if the references directory is available (without panicking).
///
/// Use this in tests that want to skip gracefully when spec files aren't available.
pub fn try_find_references_dir() -> Option<PathBuf> {
    // Check environment variable first
    if let Ok(env_path) = std::env::var("SYSML_CORPUS_PATH") {
        let path = PathBuf::from(&env_path);
        if path.exists() {
            return Some(path);
        }
    }

    // Try common relative paths
    let candidates = [
        "../sysmlv2-references",
        "../../sysmlv2-references",
        "sysmlv2-references",
        "../spec",
        "spec",
    ];

    for candidate in candidates {
        let path = Path::new(candidate);
        if path.exists() && path.join("SysML-vocab.ttl").exists() {
            return Some(path.to_path_buf());
        }
    }

    None
}

/// Configuration for coverage tests.
#[derive(Debug, Clone)]
pub struct CoverageConfig {
    /// Path to the sysmlv2-references directory.
    pub corpus_path: PathBuf,
    /// Paths within corpus to search for .sysml files.
    pub corpus_subdirs: Vec<&'static str>,
}

impl CoverageConfig {
    /// Create a new configuration from environment variable.
    ///
    /// Returns `None` if `SYSML_CORPUS_PATH` is not set.
    pub fn from_env() -> Option<Self> {
        let corpus_path = std::env::var("SYSML_CORPUS_PATH").ok()?;
        Some(CoverageConfig {
            corpus_path: PathBuf::from(corpus_path),
            corpus_subdirs: vec![
                // Standard library (21 files)
                "SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.systems",
                // Example models
                "SysML-v2-Models/models",
            ],
        })
    }

    /// Create configuration for local development (relative path).
    pub fn local_dev() -> Self {
        CoverageConfig {
            corpus_path: PathBuf::from("../sysmlv2-references"),
            corpus_subdirs: vec![
                "SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.systems",
                "SysML-v2-Models/models",
            ],
        }
    }
}

/// Result of parsing a single corpus file.
#[derive(Debug, Clone)]
pub struct FileParseResult {
    /// Relative path to the file.
    pub path: String,
    /// Whether parsing succeeded.
    pub success: bool,
    /// Error messages if parsing failed.
    pub errors: Vec<String>,
    /// Number of elements produced.
    pub element_count: usize,
}

/// Summary of corpus coverage.
#[derive(Debug, Clone, Default)]
pub struct CoverageSummary {
    /// Total files attempted.
    pub total_files: usize,
    /// Successfully parsed files.
    pub passed_files: usize,
    /// Failed files.
    pub failed_files: usize,
    /// Files in allow-list that failed (expected).
    pub expected_failures: usize,
    /// Files NOT in allow-list that failed (unexpected).
    pub unexpected_failures: usize,
    /// Grammar rules exercised.
    pub rules_exercised: HashSet<String>,
    /// Element kinds produced.
    pub element_kinds_produced: HashSet<String>,
}

impl CoverageSummary {
    /// Get the pass percentage.
    pub fn pass_percentage(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.passed_files as f64 / self.total_files as f64) * 100.0
        }
    }
}

/// Load the allow-list of expected failures from a file.
pub fn load_allow_list(content: &str) -> HashSet<String> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect()
}

/// Load the list of constructible kinds from a file.
pub fn load_constructible_kinds(content: &str) -> HashSet<String> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect()
}

/// Operator definition for coverage testing.
///
/// **Deprecated**: Use [`operators::OperatorInfo`] from codegen instead,
/// which is derived from the xtext grammar specification.
#[derive(Debug, Clone)]
#[deprecated(
    since = "0.1.0",
    note = "Use `operators::load_operators()` and `sysml_codegen::OperatorInfo` instead"
)]
pub struct OperatorDef {
    /// The operator symbol or keyword.
    pub operator: String,
    /// Category (logical, equality, relational, etc.).
    pub category: String,
    /// Example usage.
    pub example: String,
}

/// Load operator definitions from a file.
///
/// **Deprecated**: Use [`operators::load_operators()`] instead,
/// which derives operators from the xtext grammar specification.
#[deprecated(
    since = "0.1.0",
    note = "Use `operators::load_operators()` which derives from xtext spec"
)]
#[allow(deprecated)]
pub fn load_operators(content: &str) -> Vec<OperatorDef> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                Some(OperatorDef {
                    operator: parts[0].to_string(),
                    category: parts[1].to_string(),
                    example: parts[2].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_allow_list_basic() {
        let content = r#"
# This is a comment
file1.sysml
file2.sysml

# Another comment
file3.sysml
"#;
        let list = load_allow_list(content);
        assert_eq!(list.len(), 3);
        assert!(list.contains("file1.sysml"));
        assert!(list.contains("file2.sysml"));
        assert!(list.contains("file3.sysml"));
    }

    #[test]
    #[allow(deprecated)]
    fn load_operators_basic() {
        let content = r#"
# Operators
implies | logical | a implies b
== | equality | a == b
"#;
        let ops = load_operators(content);
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].operator, "implies");
        assert_eq!(ops[0].category, "logical");
        assert_eq!(ops[1].operator, "==");
    }
}
