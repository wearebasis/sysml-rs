//! # sysml-text
//!
//! Parser trait and result types for SysML v2 text parsing.
//!
//! This crate defines the interface for parsing SysML v2 textual notation.
//! Actual parsing implementations are provided by sidecar crates that
//! wrap external parsers (Pilot, MontiCore, SySide).
//!
//! ## Standard Library Loading
//!
//! For full name resolution, you can load the standard library:
//!
//! ```ignore
//! use sysml_text::library::{load_standard_library, LibraryConfig};
//!
//! // Load library from environment variable SYSML_LIBRARY_PATH
//! let config = LibraryConfig::from_env()?;
//! let library = load_standard_library(&parser, &config)?;
//!
//! // Parse and resolve with library
//! let result = parser.parse(&files).into_resolved_with_library(library);
//! ```

pub mod library;

use sysml_core::resolution::{resolve_references, ResolutionResult};
use sysml_core::ModelGraph;
use sysml_span::Diagnostic;

/// A SysML source file to be parsed.
#[derive(Debug, Clone)]
pub struct SysmlFile {
    /// The file path or URI.
    pub path: String,
    /// The file contents.
    pub text: String,
}

impl SysmlFile {
    /// Create a new SysML file.
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        SysmlFile {
            path: path.into(),
            text: text.into(),
        }
    }
}

/// The result of parsing SysML files.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// The parsed model graph (may be partial if there were errors).
    pub graph: ModelGraph,
    /// Any diagnostics (errors, warnings) from parsing.
    pub diagnostics: Vec<Diagnostic>,
}

impl ParseResult {
    /// Create a new parse result.
    pub fn new(graph: ModelGraph, diagnostics: Vec<Diagnostic>) -> Self {
        ParseResult { graph, diagnostics }
    }

    /// Create a successful parse result with no diagnostics.
    pub fn success(graph: ModelGraph) -> Self {
        ParseResult {
            graph,
            diagnostics: Vec::new(),
        }
    }

    /// Create an empty parse result with a single error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        ParseResult {
            graph: ModelGraph::new(),
            diagnostics: vec![Diagnostic::error(message)],
        }
    }

    /// Check if parsing succeeded (no errors).
    pub fn is_ok(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.is_error())
    }

    /// Check if parsing had errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }

    /// Get the number of errors.
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }

    /// Resolve all unresolved references in the parsed model.
    ///
    /// This converts `unresolved_*` string properties to resolved `ElementId`
    /// references. Any references that cannot be resolved are added to diagnostics.
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Example
    /// ```ignore
    /// let parser = PestParser::new();
    /// let result = parser.parse(&files).into_resolved();
    /// assert!(result.is_ok()); // Both parse AND resolution succeeded
    /// ```
    pub fn into_resolved(mut self) -> Self {
        let res = resolve_references(&mut self.graph);

        // Merge resolution diagnostics into parse diagnostics
        for diag in res.diagnostics.iter() {
            self.diagnostics.push(diag.clone());
        }

        self
    }

    /// Resolve references and return the detailed resolution result.
    ///
    /// Use this when you need access to resolution statistics
    /// (resolved_count, unresolved_count).
    ///
    /// # Example
    /// ```ignore
    /// let parser = PestParser::new();
    /// let mut result = parser.parse(&files);
    /// let res = result.resolve();
    /// println!("Resolved {} references", res.resolved_count);
    /// ```
    pub fn resolve(&mut self) -> ResolutionResult {
        resolve_references(&mut self.graph)
    }

    /// Resolve references with a pre-loaded standard library.
    ///
    /// This merges the library graph into the parsed graph before resolution,
    /// enabling resolution of standard library types like `Anything`, `Real`,
    /// `Item`, etc.
    ///
    /// # Arguments
    ///
    /// * `library` - A ModelGraph containing the standard library elements,
    ///   typically created via `load_standard_library()`.
    ///
    /// # Example
    /// ```ignore
    /// use sysml_text::library::{load_standard_library, LibraryConfig};
    ///
    /// let config = LibraryConfig::from_env()?;
    /// let library = load_standard_library(&parser, &config)?;
    ///
    /// let result = parser.parse(&files).into_resolved_with_library(library);
    /// assert!(result.is_ok()); // Standard library types now resolve
    /// ```
    pub fn into_resolved_with_library(mut self, library: ModelGraph) -> Self {
        // Merge library into our graph (as_library=true registers root packages)
        self.graph.merge(library, true);

        // Rebuild indexes after merge
        self.graph.rebuild_indexes();

        // Now resolve
        self.into_resolved()
    }

    /// Resolve references with library and return detailed statistics.
    ///
    /// Like `into_resolved_with_library`, but returns the `ResolutionResult`
    /// with statistics instead of consuming self.
    pub fn resolve_with_library(&mut self, library: ModelGraph) -> ResolutionResult {
        // Merge library into our graph
        self.graph.merge(library, true);

        // Rebuild indexes after merge
        self.graph.rebuild_indexes();

        // Now resolve
        resolve_references(&mut self.graph)
    }
}

impl Default for ParseResult {
    fn default() -> Self {
        ParseResult::success(ModelGraph::new())
    }
}

/// Trait for SysML v2 text parsers.
///
/// Implementations of this trait wrap external parsers and convert
/// their output to the common ModelGraph representation.
pub trait Parser {
    /// Parse one or more SysML files.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The source files to parse
    ///
    /// # Returns
    ///
    /// A `ParseResult` containing the parsed model and any diagnostics.
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult;

    /// Get the name of this parser implementation.
    fn name(&self) -> &str;

    /// Get the version of this parser implementation.
    fn version(&self) -> &str {
        "unknown"
    }
}

/// Trait for SysML v2 text formatters.
///
/// Implementations can format/pretty-print a ModelGraph back to
/// SysML v2 textual notation.
pub trait Formatter {
    /// Format a model graph to SysML v2 text.
    ///
    /// # Arguments
    ///
    /// * `graph` - The model to format
    ///
    /// # Returns
    ///
    /// The formatted SysML v2 text.
    fn format(&self, graph: &ModelGraph) -> String;
}

/// A no-op parser that returns an empty graph with a "not implemented" diagnostic.
///
/// This is useful as a default or placeholder.
#[derive(Debug, Clone, Default)]
pub struct NoopParser;

impl NoopParser {
    /// Create a new no-op parser.
    pub fn new() -> Self {
        NoopParser
    }
}

impl Parser for NoopParser {
    fn parse(&self, _inputs: &[SysmlFile]) -> ParseResult {
        ParseResult::error("NoopParser: parsing not implemented")
    }

    fn name(&self) -> &str {
        "noop"
    }

    fn version(&self) -> &str {
        "0.0.0"
    }
}

/// A stub parser that returns an empty graph with success.
///
/// Useful for testing when you need a parser that doesn't fail.
#[derive(Debug, Clone, Default)]
pub struct StubParser;

impl StubParser {
    /// Create a new stub parser.
    pub fn new() -> Self {
        StubParser
    }
}

impl Parser for StubParser {
    fn parse(&self, _inputs: &[SysmlFile]) -> ParseResult {
        ParseResult::success(ModelGraph::new())
    }

    fn name(&self) -> &str {
        "stub"
    }

    fn version(&self) -> &str {
        "0.0.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sysml_file_creation() {
        let file = SysmlFile::new("test.sysml", "package Test {}");
        assert_eq!(file.path, "test.sysml");
        assert_eq!(file.text, "package Test {}");
    }

    #[test]
    fn parse_result_success() {
        let result = ParseResult::success(ModelGraph::new());
        assert!(result.is_ok());
        assert!(!result.has_errors());
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn parse_result_error() {
        let result = ParseResult::error("test error");
        assert!(!result.is_ok());
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn noop_parser() {
        let parser = NoopParser::new();
        let files = vec![SysmlFile::new("test.sysml", "")];
        let result = parser.parse(&files);

        assert!(result.has_errors());
        assert_eq!(parser.name(), "noop");
    }

    #[test]
    fn stub_parser() {
        let parser = StubParser::new();
        let files = vec![SysmlFile::new("test.sysml", "")];
        let result = parser.parse(&files);

        assert!(result.is_ok());
        assert_eq!(parser.name(), "stub");
    }
}
