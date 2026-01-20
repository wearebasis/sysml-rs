//! # sysml-span
//!
//! Source locations, diagnostics, and severity levels for SysML v2.
//!
//! This crate provides types for tracking source code locations and
//! reporting diagnostics (errors, warnings, etc.).
//!
//! ## Features
//!
//! - `serde`: Enable serde serialization support
//!
//! ## Examples
//!
//! ```
//! use sysml_span::{Span, Diagnostic, Severity};
//!
//! // Create a span for a source location
//! let span = Span::with_location("example.sysml", 100, 150, 5, 10);
//!
//! // Create an error diagnostic
//! let error = Diagnostic::error("unexpected token")
//!     .with_span(span)
//!     .with_code("E001")
//!     .with_note("expected ';' here");
//!
//! assert!(error.is_error());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A span representing a range in a source file.
///
/// # Examples
///
/// ```
/// use sysml_span::Span;
///
/// // Create a span with byte offsets
/// let span = Span::new("file.sysml", 10, 20);
/// assert_eq!(span.len(), 10);
/// assert!(span.contains(15));
///
/// // Create a span with line/column info
/// let span = Span::with_location("file.sysml", 10, 20, 5, 3);
/// assert_eq!(span.line, Some(5));
/// assert_eq!(span.col, Some(3));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    /// The file path or URI.
    pub file: String,
    /// Start byte offset (0-indexed).
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Start line number (1-indexed, optional).
    pub line: Option<u32>,
    /// Start column number (1-indexed, optional).
    pub col: Option<u32>,
}

impl Span {
    /// Create a new span with byte offsets only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::new("file.sysml", 0, 100);
    /// assert_eq!(span.file, "file.sysml");
    /// assert_eq!(span.start, 0);
    /// assert_eq!(span.end, 100);
    /// ```
    pub fn new(file: impl Into<String>, start: usize, end: usize) -> Self {
        Span {
            file: file.into(),
            start,
            end,
            line: None,
            col: None,
        }
    }

    /// Create a new span with line and column information.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::with_location("file.sysml", 10, 20, 5, 3);
    /// assert_eq!(span.to_string(), "file.sysml:5:3");
    /// ```
    pub fn with_location(
        file: impl Into<String>,
        start: usize,
        end: usize,
        line: u32,
        col: u32,
    ) -> Self {
        Span {
            file: file.into(),
            start,
            end,
            line: Some(line),
            col: Some(col),
        }
    }

    /// Create a span at a single point.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::point("file.sysml", 50);
    /// assert_eq!(span.start, 50);
    /// assert_eq!(span.end, 50);
    /// assert!(span.is_empty());
    /// ```
    pub fn point(file: impl Into<String>, offset: usize) -> Self {
        Span::new(file, offset, offset)
    }

    /// Create a synthetic span (no real source location).
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::synthetic();
    /// assert_eq!(span.file, "<synthetic>");
    /// ```
    pub fn synthetic() -> Self {
        Span {
            file: "<synthetic>".to_string(),
            start: 0,
            end: 0,
            line: None,
            col: None,
        }
    }

    /// Get the length of this span in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::new("file.sysml", 10, 20);
    /// assert_eq!(span.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Check if this span is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::point("file.sysml", 10);
    /// assert!(span.is_empty());
    ///
    /// let span = Span::new("file.sysml", 10, 20);
    /// assert!(!span.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Check if this span contains a byte offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let span = Span::new("file.sysml", 10, 20);
    /// assert!(span.contains(15));
    /// assert!(!span.contains(25));
    /// assert!(span.contains(10));  // Inclusive start
    /// assert!(!span.contains(20)); // Exclusive end
    /// ```
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }

    /// Merge two spans into one covering both.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Span;
    ///
    /// let a = Span::new("file.sysml", 10, 20);
    /// let b = Span::new("file.sysml", 15, 30);
    /// let merged = a.merge(&b);
    /// assert_eq!(merged.start, 10);
    /// assert_eq!(merged.end, 30);
    /// ```
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            file: self.file.clone(),
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.or(other.line),
            col: self.col.or(other.col),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(line), Some(col)) = (self.line, self.col) {
            write!(f, "{}:{}:{}", self.file, line, col)
        } else {
            write!(f, "{}:{}-{}", self.file, self.start, self.end)
        }
    }
}

/// Severity level for diagnostics.
///
/// # Examples
///
/// ```
/// use sysml_span::Severity;
///
/// assert!(Severity::Error.is_error());
/// assert!(Severity::Warning.is_warning_or_error());
/// assert!(!Severity::Info.is_error());
///
/// // Severities are ordered
/// assert!(Severity::Info < Severity::Warning);
/// assert!(Severity::Warning < Severity::Error);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Severity {
    /// Informational message.
    Info,
    /// Warning that doesn't prevent processing.
    Warning,
    /// Error that prevents successful processing.
    Error,
}

impl Severity {
    /// Check if this is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Severity;
    ///
    /// assert!(Severity::Error.is_error());
    /// assert!(!Severity::Warning.is_error());
    /// ```
    pub fn is_error(&self) -> bool {
        matches!(self, Severity::Error)
    }

    /// Check if this is a warning or error.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Severity;
    ///
    /// assert!(Severity::Error.is_warning_or_error());
    /// assert!(Severity::Warning.is_warning_or_error());
    /// assert!(!Severity::Info.is_warning_or_error());
    /// ```
    pub fn is_warning_or_error(&self) -> bool {
        matches!(self, Severity::Warning | Severity::Error)
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "info"),
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
        }
    }
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Error
    }
}

/// A related location in a diagnostic, used to provide additional context.
///
/// # Examples
///
/// ```
/// use sysml_span::{RelatedLocation, Span};
///
/// let related = RelatedLocation {
///     span: Span::with_location("other.sysml", 50, 60, 10, 5),
///     message: "defined here".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelatedLocation {
    /// The source location.
    pub span: Span,
    /// A message describing the relationship.
    pub message: String,
}

impl RelatedLocation {
    /// Create a new related location.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::{RelatedLocation, Span};
    ///
    /// let span = Span::with_location("file.sysml", 10, 20, 5, 3);
    /// let related = RelatedLocation::new(span, "first defined here");
    /// ```
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        RelatedLocation {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for RelatedLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.span, self.message)
    }
}

/// A diagnostic message with location and severity.
///
/// # Examples
///
/// ```
/// use sysml_span::{Diagnostic, Span};
///
/// let error = Diagnostic::error("unexpected token")
///     .with_code("E001")
///     .with_span(Span::with_location("file.sysml", 10, 20, 5, 3))
///     .with_note("expected identifier");
///
/// assert!(error.is_error());
/// assert_eq!(error.code, Some("E001".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    /// The severity of this diagnostic.
    pub severity: Severity,
    /// An optional error code.
    pub code: Option<String>,
    /// The main message.
    pub message: String,
    /// The source location (optional).
    pub span: Option<Span>,
    /// Additional notes or suggestions.
    pub notes: Vec<String>,
    /// Related locations that provide additional context.
    pub related: Vec<RelatedLocation>,
}

impl Diagnostic {
    /// Create a new error diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let error = Diagnostic::error("syntax error");
    /// assert!(error.is_error());
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            related: Vec::new(),
        }
    }

    /// Create a new warning diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let warning = Diagnostic::warning("unused variable");
    /// assert!(!warning.is_error());
    /// ```
    pub fn warning(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            related: Vec::new(),
        }
    }

    /// Create a new info diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let info = Diagnostic::info("parsing file");
    /// assert!(!info.is_error());
    /// ```
    pub fn info(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Info,
            code: None,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            related: Vec::new(),
        }
    }

    /// Add a span to this diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::{Diagnostic, Span};
    ///
    /// let error = Diagnostic::error("error")
    ///     .with_span(Span::with_location("file.sysml", 10, 20, 5, 3));
    /// assert!(error.span.is_some());
    /// ```
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Add an error code to this diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let error = Diagnostic::error("error").with_code("E001");
    /// assert_eq!(error.code, Some("E001".to_string()));
    /// ```
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Add a note to this diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let error = Diagnostic::error("error")
    ///     .with_note("try this instead");
    /// assert_eq!(error.notes.len(), 1);
    /// ```
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add multiple notes to this diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// let error = Diagnostic::error("error")
    ///     .with_notes(["hint 1", "hint 2"]);
    /// assert_eq!(error.notes.len(), 2);
    /// ```
    pub fn with_notes(mut self, notes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.notes.extend(notes.into_iter().map(|n| n.into()));
        self
    }

    /// Add a related location to this diagnostic.
    ///
    /// Related locations provide additional context about the diagnostic,
    /// such as where a symbol was first defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::{Diagnostic, Span};
    ///
    /// let error = Diagnostic::error("duplicate definition")
    ///     .with_span(Span::with_location("file.sysml", 100, 110, 10, 5))
    ///     .with_related(
    ///         Span::with_location("file.sysml", 50, 60, 5, 5),
    ///         "first defined here"
    ///     );
    /// assert_eq!(error.related.len(), 1);
    /// ```
    pub fn with_related(mut self, span: Span, message: impl Into<String>) -> Self {
        self.related.push(RelatedLocation::new(span, message));
        self
    }

    /// Add a related location struct to this diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::{Diagnostic, RelatedLocation, Span};
    ///
    /// let related = RelatedLocation::new(
    ///     Span::with_location("file.sysml", 50, 60, 5, 5),
    ///     "first defined here"
    /// );
    /// let error = Diagnostic::error("duplicate definition")
    ///     .with_related_location(related);
    /// assert_eq!(error.related.len(), 1);
    /// ```
    pub fn with_related_location(mut self, related: RelatedLocation) -> Self {
        self.related.push(related);
        self
    }

    /// Check if this is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostic;
    ///
    /// assert!(Diagnostic::error("error").is_error());
    /// assert!(!Diagnostic::warning("warning").is_error());
    /// ```
    pub fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: severity[code]: message
        write!(f, "{}", self.severity)?;
        if let Some(code) = &self.code {
            write!(f, "[{}]", code)?;
        }
        write!(f, ": {}", self.message)?;

        // Add location if available
        if let Some(span) = &self.span {
            write!(f, "\n  --> {}", span)?;
        }

        // Add notes
        for note in &self.notes {
            write!(f, "\n  = note: {}", note)?;
        }

        // Add related locations
        for related in &self.related {
            write!(f, "\n  = related: {} ({})", related.span, related.message)?;
        }

        Ok(())
    }
}

/// A collection of diagnostics with helper methods.
///
/// # Examples
///
/// ```
/// use sysml_span::{Diagnostics, Diagnostic};
///
/// let mut diags = Diagnostics::new();
/// diags.error("error 1");
/// diags.warning("warning 1");
/// diags.error("error 2");
///
/// assert!(diags.has_errors());
/// assert_eq!(diags.error_count(), 2);
/// assert_eq!(diags.len(), 3);
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Create an empty diagnostics collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let diags = Diagnostics::new();
    /// assert!(diags.is_empty());
    /// ```
    pub fn new() -> Self {
        Diagnostics { items: Vec::new() }
    }

    /// Add a diagnostic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::{Diagnostics, Diagnostic};
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.push(Diagnostic::error("something went wrong"));
    /// assert_eq!(diags.len(), 1);
    /// ```
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    /// Add an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.error("something went wrong");
    /// assert!(diags.has_errors());
    /// ```
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(Diagnostic::error(message));
    }

    /// Add a warning.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.warning("potential issue");
    /// assert!(!diags.has_errors());
    /// assert!(!diags.is_empty());
    /// ```
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(Diagnostic::warning(message));
    }

    /// Check if there are any errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// assert!(!diags.has_errors());
    /// diags.error("error");
    /// assert!(diags.has_errors());
    /// ```
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.is_error())
    }

    /// Get the number of errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.error("error 1");
    /// diags.warning("warning");
    /// diags.error("error 2");
    /// assert_eq!(diags.error_count(), 2);
    /// ```
    pub fn error_count(&self) -> usize {
        self.items.iter().filter(|d| d.is_error()).count()
    }

    /// Get all diagnostics.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.error("error");
    /// for d in diags.iter() {
    ///     println!("{}", d);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }

    /// Get all diagnostics as a vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.error("error");
    /// let vec = diags.into_vec();
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }

    /// Check if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let diags = Diagnostics::new();
    /// assert!(diags.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of diagnostics.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_span::Diagnostics;
    ///
    /// let mut diags = Diagnostics::new();
    /// diags.error("error");
    /// diags.warning("warning");
    /// assert_eq!(diags.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for Diagnostics {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl FromIterator<Diagnostic> for Diagnostics {
    fn from_iter<T: IntoIterator<Item = Diagnostic>>(iter: T) -> Self {
        Diagnostics {
            items: iter.into_iter().collect(),
        }
    }
}

impl Extend<Diagnostic> for Diagnostics {
    fn extend<T: IntoIterator<Item = Diagnostic>>(&mut self, iter: T) {
        self.items.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_basic() {
        let span = Span::new("test.sysml", 10, 20);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
        assert!(span.contains(15));
        assert!(!span.contains(25));
    }

    #[test]
    fn span_with_location() {
        let span = Span::with_location("test.sysml", 10, 20, 5, 3);
        assert_eq!(span.line, Some(5));
        assert_eq!(span.col, Some(3));
        assert_eq!(span.to_string(), "test.sysml:5:3");
    }

    #[test]
    fn span_merge() {
        let a = Span::new("test.sysml", 10, 20);
        let b = Span::new("test.sysml", 15, 30);
        let merged = a.merge(&b);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 30);
    }

    #[test]
    fn diagnostic_error() {
        let diag = Diagnostic::error("something went wrong")
            .with_code("E001")
            .with_span(Span::with_location("test.sysml", 0, 10, 1, 1))
            .with_note("try doing X instead");

        assert!(diag.is_error());
        assert_eq!(diag.code, Some("E001".to_string()));
        assert_eq!(diag.notes.len(), 1);
    }

    #[test]
    fn diagnostic_display() {
        let diag = Diagnostic::error("parse error")
            .with_code("E001")
            .with_span(Span::with_location("test.sysml", 0, 10, 1, 1));

        let s = diag.to_string();
        assert!(s.contains("error[E001]"));
        assert!(s.contains("parse error"));
        assert!(s.contains("test.sysml:1:1"));
    }

    #[test]
    fn diagnostics_collection() {
        let mut diags = Diagnostics::new();
        diags.error("error 1");
        diags.warning("warning 1");
        diags.error("error 2");

        assert!(diags.has_errors());
        assert_eq!(diags.error_count(), 2);
        assert_eq!(diags.len(), 3);
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn diagnostic_with_related() {
        let error = Diagnostic::error("duplicate definition")
            .with_span(Span::with_location("file.sysml", 100, 110, 10, 5))
            .with_related(
                Span::with_location("file.sysml", 50, 60, 5, 5),
                "first defined here",
            );

        assert_eq!(error.related.len(), 1);
        assert_eq!(error.related[0].message, "first defined here");

        let s = error.to_string();
        assert!(s.contains("related:"));
        assert!(s.contains("first defined here"));
    }

    #[test]
    fn related_location_display() {
        let related = RelatedLocation::new(
            Span::with_location("file.sysml", 10, 20, 5, 3),
            "defined here",
        );
        let s = related.to_string();
        assert!(s.contains("file.sysml:5:3"));
        assert!(s.contains("defined here"));
    }
}
