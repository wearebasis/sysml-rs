//! # sysml-ts
//!
//! Tree-sitter based CST (Concrete Syntax Tree) parsing for SysML v2.
//!
//! This crate provides fast, incremental parsing suitable for IDE use cases.
//! It produces a CST rather than a semantic model, making it useful for:
//!
//! - Syntax highlighting
//! - Bracket matching
//! - Code folding
//! - Basic outline view
//! - Error recovery during editing
//!
//! **Note**: This crate intentionally does NOT depend on sysml-core.
//! It deals only with syntax, not semantics.

use sysml_span::Span;

/// A file to be parsed.
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

/// A node in the concrete syntax tree.
#[derive(Debug, Clone)]
pub struct SyntaxNode {
    /// The kind of this node (e.g., "package_declaration", "identifier").
    pub kind: String,
    /// The source span of this node.
    pub span: Span,
    /// Child nodes.
    pub children: Vec<SyntaxNode>,
    /// Whether this node represents an error.
    pub is_error: bool,
}

impl SyntaxNode {
    /// Create a new syntax node.
    pub fn new(kind: impl Into<String>, span: Span) -> Self {
        SyntaxNode {
            kind: kind.into(),
            span,
            children: Vec::new(),
            is_error: false,
        }
    }

    /// Create an error node.
    pub fn error(span: Span) -> Self {
        SyntaxNode {
            kind: "ERROR".to_string(),
            span,
            children: Vec::new(),
            is_error: true,
        }
    }

    /// Add a child node.
    pub fn with_child(mut self, child: SyntaxNode) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple child nodes.
    pub fn with_children(mut self, children: Vec<SyntaxNode>) -> Self {
        self.children = children;
        self
    }

    /// Get the text content of this node given the source.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        if self.span.end <= source.len() {
            &source[self.span.start..self.span.end]
        } else {
            ""
        }
    }

    /// Find all nodes of a given kind in the tree.
    pub fn find_by_kind(&self, kind: &str) -> Vec<&SyntaxNode> {
        let mut result = Vec::new();
        self.find_by_kind_recursive(kind, &mut result);
        result
    }

    fn find_by_kind_recursive<'a>(&'a self, kind: &str, result: &mut Vec<&'a SyntaxNode>) {
        if self.kind == kind {
            result.push(self);
        }
        for child in &self.children {
            child.find_by_kind_recursive(kind, result);
        }
    }

    /// Find the first child of a given kind.
    pub fn child_by_kind(&self, kind: &str) -> Option<&SyntaxNode> {
        self.children.iter().find(|c| c.kind == kind)
    }

    /// Check if this node or any descendant has errors.
    pub fn has_errors(&self) -> bool {
        if self.is_error {
            return true;
        }
        self.children.iter().any(|c| c.has_errors())
    }

    /// Get all error nodes.
    pub fn errors(&self) -> Vec<&SyntaxNode> {
        let mut result = Vec::new();
        self.collect_errors(&mut result);
        result
    }

    fn collect_errors<'a>(&'a self, result: &mut Vec<&'a SyntaxNode>) {
        if self.is_error {
            result.push(self);
        }
        for child in &self.children {
            child.collect_errors(result);
        }
    }
}

/// Trait for fast CST parsers.
pub trait FastParser {
    /// Parse a file and return its CST.
    fn parse_cst(&self, file: &SysmlFile) -> SyntaxNode;

    /// Check if incremental parsing is supported.
    fn supports_incremental(&self) -> bool {
        false
    }
}

/// A stub tree-sitter parser that returns a minimal CST.
#[derive(Debug, Clone, Default)]
pub struct StubTreeSitterParser;

impl StubTreeSitterParser {
    /// Create a new stub parser.
    pub fn new() -> Self {
        StubTreeSitterParser
    }
}

impl FastParser for StubTreeSitterParser {
    fn parse_cst(&self, file: &SysmlFile) -> SyntaxNode {
        // Return a simple root node representing the entire file
        let span = Span::new(&file.path, 0, file.text.len());

        // Try to extract some basic structure
        let mut root = SyntaxNode::new("source_file", span.clone());

        // Very basic: look for "package" keyword
        if let Some(pkg_start) = file.text.find("package") {
            // Find the package name (simplified)
            let after_pkg = &file.text[pkg_start + 7..];
            let name_start = pkg_start + 7 + after_pkg.chars().take_while(|c| c.is_whitespace()).count();

            if let Some(name_end_offset) = after_pkg.trim_start().find(|c: char| !c.is_alphanumeric() && c != '_') {
                let name_end = name_start + name_end_offset;

                let pkg_node = SyntaxNode::new(
                    "package_declaration",
                    Span::new(&file.path, pkg_start, file.text.len()),
                )
                .with_child(SyntaxNode::new(
                    "identifier",
                    Span::new(&file.path, name_start, name_end),
                ));

                root = root.with_child(pkg_node);
            }
        }

        root
    }

    fn supports_incremental(&self) -> bool {
        false
    }
}

/// An outline item for IDE navigation.
#[derive(Debug, Clone)]
pub struct OutlineItem {
    /// The name/label of this item.
    pub name: String,
    /// The kind of this item (e.g., "package", "part").
    pub kind: String,
    /// The source span.
    pub span: Span,
    /// Child items.
    pub children: Vec<OutlineItem>,
}

impl OutlineItem {
    /// Create a new outline item.
    pub fn new(name: impl Into<String>, kind: impl Into<String>, span: Span) -> Self {
        OutlineItem {
            name: name.into(),
            kind: kind.into(),
            span,
            children: Vec::new(),
        }
    }

    /// Add a child item.
    pub fn with_child(mut self, child: OutlineItem) -> Self {
        self.children.push(child);
        self
    }
}

/// Extract an outline from a CST.
pub fn extract_outline(root: &SyntaxNode, source: &str) -> Vec<OutlineItem> {
    let mut items = Vec::new();

    for pkg in root.find_by_kind("package_declaration") {
        if let Some(id) = pkg.child_by_kind("identifier") {
            let name = id.text(source).to_string();
            items.push(OutlineItem::new(name, "package", pkg.span.clone()));
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_node_creation() {
        let span = Span::new("test.sysml", 0, 10);
        let node = SyntaxNode::new("identifier", span);
        assert_eq!(node.kind, "identifier");
        assert!(!node.is_error);
    }

    #[test]
    fn syntax_node_children() {
        let span = Span::new("test.sysml", 0, 20);
        let child = SyntaxNode::new("identifier", Span::new("test.sysml", 8, 12));
        let parent = SyntaxNode::new("package_declaration", span).with_child(child);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].kind, "identifier");
    }

    #[test]
    fn syntax_node_text() {
        let source = "package Test {}";
        let span = Span::new("test.sysml", 8, 12);
        let node = SyntaxNode::new("identifier", span);
        assert_eq!(node.text(source), "Test");
    }

    #[test]
    fn find_by_kind() {
        let span = Span::new("test.sysml", 0, 20);
        let id1 = SyntaxNode::new("identifier", Span::new("test.sysml", 8, 12));
        let id2 = SyntaxNode::new("identifier", Span::new("test.sysml", 14, 18));
        let root = SyntaxNode::new("source_file", span)
            .with_child(id1)
            .with_child(id2);

        let found = root.find_by_kind("identifier");
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn error_nodes() {
        let span = Span::new("test.sysml", 0, 20);
        let error = SyntaxNode::error(Span::new("test.sysml", 5, 10));
        let root = SyntaxNode::new("source_file", span).with_child(error);

        assert!(root.has_errors());
        assert_eq!(root.errors().len(), 1);
    }

    #[test]
    fn stub_parser() {
        let parser = StubTreeSitterParser::new();
        let file = SysmlFile::new("test.sysml", "package Test {}");
        let cst = parser.parse_cst(&file);

        assert_eq!(cst.kind, "source_file");
        assert!(!parser.supports_incremental());
    }

    #[test]
    fn stub_parser_extracts_package() {
        let parser = StubTreeSitterParser::new();
        let file = SysmlFile::new("test.sysml", "package MyPackage { }");
        let cst = parser.parse_cst(&file);

        let packages = cst.find_by_kind("package_declaration");
        assert!(!packages.is_empty());
    }

    #[test]
    fn outline_extraction() {
        let parser = StubTreeSitterParser::new();
        let source = "package MyPackage { }";
        let file = SysmlFile::new("test.sysml", source);
        let cst = parser.parse_cst(&file);
        let outline = extract_outline(&cst, source);

        assert!(!outline.is_empty());
        assert_eq!(outline[0].kind, "package");
    }
}
