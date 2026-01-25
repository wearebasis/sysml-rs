//! # sysml-text-pest
//!
//! Native Rust parser for SysML v2 textual notation using pest.
//!
//! This crate provides a pure Rust implementation of the SysML v2 parser,
//! based on the SySide Langium grammar as a reference. It uses the pest
//! PEG parser generator.
//!
//! ## Example
//!
//! ```
//! use sysml_text::{Parser, SysmlFile};
//! use sysml_text_pest::PestParser;
//!
//! let parser = PestParser::new();
//! let files = vec![SysmlFile::new("example.sysml", r#"
//!     package Example {
//!         part def Vehicle;
//!         part car : Vehicle;
//!     }
//! "#)];
//!
//! let result = parser.parse(&files);
//! if result.is_ok() {
//!     println!("Parsed {} elements", result.graph.element_count());
//! }
//! ```

use pest::Parser as PestParserTrait;
use pest_derive::Parser;
use rayon::prelude::*;
use sysml_core::ModelGraph;
use sysml_span::{Diagnostic, Span};
use sysml_text::{ParseResult, Parser, SysmlFile};

pub mod ast;

/// The pest parser generated from the grammar file.
///
/// The grammar is generated at build time by concatenating manual fragments
/// with auto-generated rules for keywords, operators, and enums extracted
/// from the SysML v2 xtext specification files.
#[derive(Parser)]
#[grammar = "grammar/sysml.pest"]
pub struct SysmlGrammar;

/// Native Rust parser for SysML v2 using pest.
#[derive(Debug, Clone, Default)]
pub struct PestParser {
    /// Whether to include detailed span information.
    include_spans: bool,
}

impl PestParser {
    /// Create a new pest parser.
    pub fn new() -> Self {
        PestParser {
            include_spans: true,
        }
    }

    /// Create a parser without span tracking (faster for large files).
    pub fn without_spans() -> Self {
        PestParser {
            include_spans: false,
        }
    }

    /// Parse source and collect visited grammar rule names.
    ///
    /// This method is only available when the `coverage` feature is enabled.
    /// It returns a set of rule names (as strings) that were visited during parsing,
    /// useful for coverage analysis.
    #[cfg(feature = "coverage")]
    pub fn parse_for_rule_coverage(
        &self,
        source: &str,
    ) -> Result<std::collections::HashSet<String>, String> {
        use pest::iterators::Pairs;
        use std::collections::HashSet;

        fn collect_rules(pairs: Pairs<'_, Rule>, rules: &mut HashSet<String>) {
            for pair in pairs {
                rules.insert(format!("{:?}", pair.as_rule()));
                collect_rules(pair.into_inner(), rules);
            }
        }

        match SysmlGrammar::parse(Rule::File, source) {
            Ok(pairs) => {
                let mut rules = HashSet::new();
                collect_rules(pairs, &mut rules);
                Ok(rules)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Parse a single file and convert to ModelGraph.
    fn parse_file(&self, file: &SysmlFile) -> (ModelGraph, Vec<Diagnostic>) {
        let mut graph = ModelGraph::new();
        let mut diagnostics = Vec::new();

        // Parse using pest
        match SysmlGrammar::parse(Rule::File, &file.text) {
            Ok(pairs) => {
                // Convert pest pairs to ModelGraph
                // Pass source text for O(log n) line/column lookups via LineIndex
                let converter =
                    ast::Converter::new(&file.path, self.include_spans, Some(&file.text));
                match converter.convert(pairs, &mut graph) {
                    Ok(()) => {}
                    Err(e) => {
                        let diagnostic = Diagnostic::error(format!("Conversion error: {}", e))
                            .with_note(format!("file: {}", file.path))
                            .with_note("AST conversion failed after parsing succeeded");
                        diagnostics.push(diagnostic);
                    }
                }
            }
            Err(e) => {
                // Convert pest error to diagnostic with richer context
                let diagnostic = self.pest_error_to_diagnostic(&file.path, &file.text, e);
                diagnostics.push(diagnostic);
            }
        }

        (graph, diagnostics)
    }

    /// Convert a pest parsing error to a Diagnostic.
    fn pest_error_to_diagnostic(
        &self,
        file: &str,
        source: &str,
        error: pest::error::Error<Rule>,
    ) -> Diagnostic {
        let (line, col) = match error.line_col {
            pest::error::LineColLocation::Pos((line, col)) => (line as u32, col as u32),
            pest::error::LineColLocation::Span((line, col), _) => (line as u32, col as u32),
        };

        let (mut start, mut end) = match error.location {
            pest::error::InputLocation::Pos(pos) => (pos, pos.saturating_add(1)),
            pest::error::InputLocation::Span((start, end)) => (start, end),
        };

        let source_len = source.len();
        start = start.min(source_len);
        end = end.min(source_len);
        if end <= start && start < source_len {
            end = start + 1;
        }

        let mut diagnostic = match &error.variant {
            pest::error::ErrorVariant::ParsingError { .. } => Diagnostic::error("syntax error"),
            pest::error::ErrorVariant::CustomError { message } => {
                Diagnostic::error(message.clone())
            }
        };

        diagnostic = diagnostic
            .with_span(Span::with_location(file, start, end, line, col))
            .with_code("E001");

        if let pest::error::ErrorVariant::ParsingError {
            positives,
            negatives,
        } = &error.variant
        {
            let expected = format_rule_list(positives);
            let unexpected = format_rule_list(negatives);

            if !expected.is_empty() {
                diagnostic = diagnostic.with_note(format!("expected: {}", expected));
            }
            if !unexpected.is_empty() {
                diagnostic = diagnostic.with_note(format!("unexpected: {}", unexpected));
            }
        }

        let line_text = error.line().trim();
        if !line_text.is_empty() {
            diagnostic = diagnostic.with_note(format!("near: {}", line_text));
        }

        diagnostic
    }
}

fn format_rule_list(rules: &[Rule]) -> String {
    if rules.is_empty() {
        return String::new();
    }

    let mut names: Vec<String> = rules.iter().map(|r| format!("{:?}", r)).collect();
    names.sort();
    names.dedup();

    const MAX_RULES: usize = 6;
    if names.len() > MAX_RULES {
        let remaining = names.len() - MAX_RULES;
        names.truncate(MAX_RULES);
        format!("{} (+{} more)", names.join(", "), remaining)
    } else {
        names.join(", ")
    }
}

impl PestParser {
    /// Parse files and run structural validation.
    ///
    /// This is equivalent to calling `parse()` followed by
    /// `validate_structure()` and `validate_relationships()`.
    ///
    /// # Example
    /// ```ignore
    /// let parser = PestParser::new();
    /// let files = vec![SysmlFile::new("test.sysml", "...")];
    /// let result = parser.parse_with_validation(&files);
    /// if result.has_errors() {
    ///     for diag in &result.diagnostics {
    ///         eprintln!("{}", diag);
    ///     }
    /// }
    /// ```
    pub fn parse_with_validation(&self, files: &[SysmlFile]) -> ParseResult {
        let mut result = self.parse(files);
        result.validate_structure();
        result.validate_relationships();
        result
    }
}

impl Parser for PestParser {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Threshold for parallel parsing - overhead not worth it for small batches
        const PARALLEL_THRESHOLD: usize = 2;

        let results: Vec<(ModelGraph, Vec<Diagnostic>)> = if inputs.len() >= PARALLEL_THRESHOLD {
            // Parse files in parallel using rayon
            inputs
                .par_iter()
                .map(|file| self.parse_file(file))
                .collect()
        } else {
            // Sequential parsing for single files (avoids rayon overhead)
            inputs.iter().map(|file| self.parse_file(file)).collect()
        };

        // Sequential merge phase (unavoidable - mutates single graph)
        let mut combined_graph = ModelGraph::new();
        let mut all_diagnostics = Vec::new();

        for (graph, diagnostics) in results {
            // Merge graphs - copy elements and relationships
            for (_, element) in graph.elements {
                combined_graph.add_element(element);
            }
            for (_, rel) in graph.relationships {
                combined_graph.add_relationship(rel);
            }

            all_diagnostics.extend(diagnostics);
        }

        // Rebuild indexes after merging to ensure namespace_to_memberships
        // and element_to_owning_membership indexes are populated
        combined_graph.rebuild_indexes();

        ParseResult::new(combined_graph, all_diagnostics)
    }

    fn name(&self) -> &str {
        "pest"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Error type for parser operations.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Syntax error from pest.
    #[error("Syntax error: {0}")]
    Syntax(String),

    /// Conversion error during AST building.
    #[error("Conversion error: {0}")]
    Conversion(String),

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creates() {
        let parser = PestParser::new();
        assert_eq!(parser.name(), "pest");
    }

    #[test]
    fn parse_empty_file() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("test.sysml", "")];
        let result = parser.parse(&files);
        // Empty file should parse successfully
        assert!(result.is_ok());
    }

    #[test]
    fn parse_simple_package() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("test.sysml", "package TestPackage { }")];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok(), "Expected successful parse");
        assert!(
            result.graph.element_count() >= 1,
            "Expected at least one element"
        );
    }

    #[test]
    fn parse_package_with_part() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                part def Vehicle;
                part car : Vehicle;
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok(), "Expected successful parse");
    }

    #[test]
    fn parse_comment() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                /* This is a comment */
            }
            "#,
        )];
        let result = parser.parse(&files);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_import() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                import OtherPackage::*;
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    #[test]
    fn parse_attribute_def() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                attribute def Mass;
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    #[test]
    fn parse_action_def() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                action def Drive;
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    #[test]
    fn parse_requirement_def() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                requirement def MaxSpeed;
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    // Documentation parsing with /* */ comment body.
    // Fixed in 2c.1: ML_COMMENT removed from implicit COMMENT rule.
    #[test]
    fn parse_documentation() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                doc /* This is documentation for the package. */
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    #[test]
    fn parse_state_def() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new(
            "test.sysml",
            r#"
            package TestPackage {
                state def VehicleState {
                    entry;
                    do;
                    exit;
                }
            }
            "#,
        )];
        let result = parser.parse(&files);

        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }

        assert!(result.is_ok());
    }

    #[test]
    fn parse_syntax_error() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("test.sysml", "package { invalid syntax")];
        let result = parser.parse(&files);

        // Should have parse errors
        assert!(result.has_errors());
    }

    // === Validation Integration Tests (Phase 5) ===

    #[test]
    fn parse_with_validation_valid_package() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("test.sysml", "package TestPackage { }")];
        let result = parser.parse_with_validation(&files);

        // Valid package should have no errors
        if result.has_errors() {
            for d in &result.diagnostics {
                eprintln!("Error: {}", d);
            }
        }
        assert!(result.is_ok(), "Valid package should pass validation");
    }

    #[test]
    fn parse_with_validation_runs_validation() {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("test.sysml", "package TestPackage { }")];

        // parse_with_validation should complete without panic
        let result = parser.parse_with_validation(&files);
        // Just verify it ran - specific validation behavior is tested elsewhere
        let _ = result.diagnostics.len();
    }
}
