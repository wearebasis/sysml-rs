//! # sysml-lsp
//!
//! LSP protocol types and conversions for SysML v2.
//!
//! This crate provides minimal LSP-compatible types without pulling in
//! the full lsp-types crate. It handles conversions from sysml-span
//! diagnostics to LSP format.
//!
//! ## Features
//!
//! - `linking`: Enable sysml-core integration for semantic symbol linking

use sysml_id::ElementId;
use sysml_span::{Diagnostic as SysmlDiagnostic, Severity as SysmlSeverity, Span};

/// LSP diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

impl From<SysmlSeverity> for DiagnosticSeverity {
    fn from(severity: SysmlSeverity) -> Self {
        match severity {
            SysmlSeverity::Error => DiagnosticSeverity::Error,
            SysmlSeverity::Warning => DiagnosticSeverity::Warning,
            SysmlSeverity::Info => DiagnosticSeverity::Information,
        }
    }
}

/// A position in a text document (0-indexed line and character).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: u32,
    /// Character offset (0-indexed, UTF-16 code units).
    pub character: u32,
}

impl Position {
    /// Create a new position.
    pub fn new(line: u32, character: u32) -> Self {
        Position { line, character }
    }
}

/// A range in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Range {
    /// The start position.
    pub start: Position,
    /// The end position.
    pub end: Position,
}

impl Range {
    /// Create a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Range { start, end }
    }

    /// Create a range from a span and source text.
    ///
    /// Converts byte offsets to line/character positions.
    pub fn from_span(span: &Span, source: &str) -> Self {
        let start = offset_to_position(span.start, source);
        let end = offset_to_position(span.end, source);
        Range { start, end }
    }
}

/// Convert a byte offset to a line/character position.
fn offset_to_position(offset: usize, source: &str) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;

    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Position::new(line, col)
}

/// An LSP diagnostic.
#[derive(Debug, Clone)]
pub struct LspDiagnostic {
    /// The range where the diagnostic applies.
    pub range: Range,
    /// The diagnostic severity.
    pub severity: Option<DiagnosticSeverity>,
    /// The diagnostic code.
    pub code: Option<String>,
    /// The diagnostic source (e.g., "sysml").
    pub source: Option<String>,
    /// The diagnostic message.
    pub message: String,
    /// Related information.
    pub related_information: Vec<DiagnosticRelatedInformation>,
}

impl LspDiagnostic {
    /// Convert from a SysML diagnostic.
    pub fn from_sysml(diag: &SysmlDiagnostic, source_text: &str) -> Self {
        let range = diag
            .span
            .as_ref()
            .map(|s| Range::from_span(s, source_text))
            .unwrap_or_default();

        LspDiagnostic {
            range,
            severity: Some(diag.severity.into()),
            code: diag.code.clone(),
            source: Some("sysml".to_string()),
            message: diag.message.clone(),
            related_information: diag
                .notes
                .iter()
                .map(|note| DiagnosticRelatedInformation {
                    location: Location {
                        uri: diag
                            .span
                            .as_ref()
                            .map(|s| s.file.clone())
                            .unwrap_or_default(),
                        range,
                    },
                    message: note.clone(),
                })
                .collect(),
        }
    }
}

/// Related information for a diagnostic.
#[derive(Debug, Clone)]
pub struct DiagnosticRelatedInformation {
    /// The location of the related information.
    pub location: Location,
    /// The message.
    pub message: String,
}

/// A location in a document.
#[derive(Debug, Clone)]
pub struct Location {
    /// The document URI.
    pub uri: String,
    /// The range within the document.
    pub range: Range,
}

/// A document symbol for outline/navigation.
#[derive(Debug, Clone)]
pub struct DocumentSymbol {
    /// The name of this symbol.
    pub name: String,
    /// More detail for this symbol (optional).
    pub detail: Option<String>,
    /// The kind of this symbol.
    pub kind: SymbolKind,
    /// The range of this symbol.
    pub range: Range,
    /// The range that should be selected when navigating to this symbol.
    pub selection_range: Range,
    /// Children of this symbol.
    pub children: Vec<DocumentSymbol>,
}

/// Symbol kinds (subset of LSP spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/// A symbol information for workspace-wide symbol search.
#[derive(Debug, Clone)]
pub struct SymbolInformation {
    /// The name of this symbol.
    pub name: String,
    /// The kind of this symbol.
    pub kind: SymbolKind,
    /// The location of this symbol.
    pub location: Location,
    /// The name of the symbol containing this symbol (optional).
    pub container_name: Option<String>,
}

/// Linked symbol information (when linking feature is enabled).
#[cfg(feature = "linking")]
#[derive(Debug, Clone)]
pub struct LinkedSymbol {
    /// The symbol information.
    pub symbol: DocumentSymbol,
    /// The element ID if resolved.
    pub element_id: Option<ElementId>,
}

/// Convert SysML element kind to LSP symbol kind.
#[cfg(feature = "linking")]
pub fn element_kind_to_symbol_kind(kind: &sysml_core::ElementKind) -> SymbolKind {
    use sysml_core::ElementKind;
    match kind {
        ElementKind::Package => SymbolKind::Package,
        ElementKind::PartUsage | ElementKind::PartDefinition => SymbolKind::Class,
        ElementKind::RequirementUsage | ElementKind::RequirementDefinition => SymbolKind::Interface,
        ElementKind::VerificationCaseUsage | ElementKind::VerificationCaseDefinition => SymbolKind::Method,
        ElementKind::StateDefinition => SymbolKind::Class,
        ElementKind::StateUsage => SymbolKind::Enum,
        ElementKind::TransitionUsage => SymbolKind::Event,
        ElementKind::ActionUsage | ElementKind::ActionDefinition => SymbolKind::Function,
        ElementKind::AttributeUsage | ElementKind::AttributeDefinition => SymbolKind::Property,
        ElementKind::Documentation => SymbolKind::File,
        _ => SymbolKind::Object,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_span::Diagnostic;

    #[test]
    fn severity_conversion() {
        assert_eq!(
            DiagnosticSeverity::from(SysmlSeverity::Error),
            DiagnosticSeverity::Error
        );
        assert_eq!(
            DiagnosticSeverity::from(SysmlSeverity::Warning),
            DiagnosticSeverity::Warning
        );
        assert_eq!(
            DiagnosticSeverity::from(SysmlSeverity::Info),
            DiagnosticSeverity::Information
        );
    }

    #[test]
    fn offset_to_position_simple() {
        let source = "line1\nline2\nline3";
        let pos = offset_to_position(0, source);
        assert_eq!(pos.line, 0);
        assert_eq!(pos.character, 0);
    }

    #[test]
    fn offset_to_position_second_line() {
        let source = "line1\nline2\nline3";
        let pos = offset_to_position(6, source); // Start of "line2"
        assert_eq!(pos.line, 1);
        assert_eq!(pos.character, 0);
    }

    #[test]
    fn offset_to_position_middle() {
        let source = "line1\nline2\nline3";
        let pos = offset_to_position(8, source); // "ne2" in line2
        assert_eq!(pos.line, 1);
        assert_eq!(pos.character, 2);
    }

    #[test]
    fn range_from_span() {
        let source = "package Test {\n  part x;\n}";
        let span = Span::new("test.sysml", 8, 12); // "Test"
        let range = Range::from_span(&span, source);
        assert_eq!(range.start.line, 0);
        assert_eq!(range.start.character, 8);
    }

    #[test]
    fn lsp_diagnostic_from_sysml() {
        let source = "package Test {}";
        let diag = Diagnostic::error("test error")
            .with_span(Span::new("test.sysml", 0, 7))
            .with_code("E001");

        let lsp_diag = LspDiagnostic::from_sysml(&diag, source);
        assert_eq!(lsp_diag.message, "test error");
        assert_eq!(lsp_diag.severity, Some(DiagnosticSeverity::Error));
        assert_eq!(lsp_diag.code, Some("E001".to_string()));
    }
}
