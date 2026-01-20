//! # sysml-id
//!
//! Element identifiers, qualified names, and project/commit IDs for SysML v2.
//!
//! This crate provides the fundamental identification types used throughout
//! the sysml-rs ecosystem.
//!
//! ## Features
//!
//! - `uuid` (default): Use UUID v4 for ElementId
//! - `serde`: Enable serde serialization support
//!
//! ## Examples
//!
//! ```
//! use sysml_id::{ElementId, QualifiedName, ProjectId};
//!
//! // Create element IDs
//! let id = ElementId::new_v4();
//! let id2 = ElementId::from_string("my-element");
//!
//! // Create qualified names
//! let qn: QualifiedName = "Package::Part::Attribute".parse().unwrap();
//! assert_eq!(qn.simple_name(), Some("Attribute"));
//!
//! // Create project IDs
//! let project = ProjectId::new("my-sysml-project");
//! ```

use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type for ID parsing failures.
///
/// # Examples
///
/// ```
/// use sysml_id::{IdError, QualifiedName};
///
/// let result: Result<QualifiedName, IdError> = "A::::B".parse();
/// assert!(matches!(result, Err(IdError::InvalidQualifiedName(_))));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    /// Invalid UUID format
    InvalidUuid(String),
    /// Invalid qualified name format
    InvalidQualifiedName(String),
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::InvalidUuid(s) => write!(f, "invalid UUID: {}", s),
            IdError::InvalidQualifiedName(s) => write!(f, "invalid qualified name: {}", s),
        }
    }
}

impl std::error::Error for IdError {}

/// A unique identifier for a model element.
///
/// When the `uuid` feature is enabled (default), this wraps a UUID v4.
/// Otherwise, it uses a compact string representation.
///
/// # Examples
///
/// ```
/// use sysml_id::ElementId;
///
/// // Create a new random ID
/// let id = ElementId::new_v4();
///
/// // Create from a string (deterministic for same input)
/// let id1 = ElementId::from_string("my-element");
/// let id2 = ElementId::from_string("my-element");
/// assert_eq!(id1, id2);
///
/// // Convert to string and back
/// let s = id.to_string();
/// let parsed: ElementId = s.parse().unwrap();
/// assert_eq!(id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ElementId(
    #[cfg(feature = "uuid")] uuid::Uuid,
    #[cfg(not(feature = "uuid"))] String,
);

impl ElementId {
    /// Create a new random ElementId (UUID v4 when uuid feature enabled).
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ElementId;
    ///
    /// let id1 = ElementId::new_v4();
    /// let id2 = ElementId::new_v4();
    /// assert_ne!(id1, id2); // Each call generates a unique ID
    /// ```
    #[cfg(feature = "uuid")]
    pub fn new_v4() -> Self {
        ElementId(uuid::Uuid::new_v4())
    }

    /// Create a new random ElementId using a simple counter-based ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ElementId;
    ///
    /// let id1 = ElementId::new_v4();
    /// let id2 = ElementId::new_v4();
    /// assert_ne!(id1, id2); // Each call generates a unique ID
    /// ```
    #[cfg(not(feature = "uuid"))]
    pub fn new_v4() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        ElementId(format!("elem_{:016x}", id))
    }

    /// Create an ElementId from a string representation.
    ///
    /// If the string is a valid UUID, it will be parsed. Otherwise, a deterministic
    /// UUID will be generated from the string content.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ElementId;
    ///
    /// // Same input produces same ID
    /// let id1 = ElementId::from_string("my-unique-element");
    /// let id2 = ElementId::from_string("my-unique-element");
    /// assert_eq!(id1, id2);
    ///
    /// // Different input produces different ID
    /// let id3 = ElementId::from_string("another-element");
    /// assert_ne!(id1, id3);
    ///
    /// // Valid UUID strings are parsed directly
    /// let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    /// let id = ElementId::from_string(uuid_str);
    /// assert_eq!(id.to_string(), uuid_str);
    /// ```
    pub fn from_string(s: impl Into<String>) -> Self {
        #[cfg(feature = "uuid")]
        {
            let s = s.into();
            match uuid::Uuid::parse_str(&s) {
                Ok(uuid) => ElementId(uuid),
                Err(_) => {
                    // Create a deterministic UUID from the string using a simple hash
                    // We use a basic approach: hash the string and create a UUID from the bytes
                    let mut bytes = [0u8; 16];
                    let s_bytes = s.as_bytes();
                    for (i, &b) in s_bytes.iter().enumerate() {
                        bytes[i % 16] ^= b;
                        bytes[(i + 7) % 16] = bytes[(i + 7) % 16].wrapping_add(b);
                    }
                    // Set version 4 (random) and variant bits
                    bytes[6] = (bytes[6] & 0x0f) | 0x40;
                    bytes[8] = (bytes[8] & 0x3f) | 0x80;
                    ElementId(uuid::Uuid::from_bytes(bytes))
                }
            }
        }
        #[cfg(not(feature = "uuid"))]
        {
            ElementId(s.into())
        }
    }

    /// Get the string representation of this ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ElementId;
    ///
    /// let id = ElementId::from_string("test");
    /// let s = id.as_str();
    /// assert!(!s.is_empty());
    /// ```
    pub fn as_str(&self) -> String {
        #[cfg(feature = "uuid")]
        {
            self.0.to_string()
        }
        #[cfg(not(feature = "uuid"))]
        {
            self.0.clone()
        }
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ElementId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "uuid")]
        {
            uuid::Uuid::parse_str(s)
                .map(ElementId)
                .map_err(|e| IdError::InvalidUuid(e.to_string()))
        }
        #[cfg(not(feature = "uuid"))]
        {
            Ok(ElementId(s.to_string()))
        }
    }
}

/// A unique identifier for a project.
///
/// # Examples
///
/// ```
/// use sysml_id::ProjectId;
///
/// let project = ProjectId::new("my-sysml-project");
/// assert_eq!(project.as_str(), "my-sysml-project");
/// assert_eq!(project.to_string(), "my-sysml-project");
///
/// // Parse from string
/// let parsed: ProjectId = "another-project".parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ProjectId(String);

impl ProjectId {
    /// Create a new ProjectId from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ProjectId;
    ///
    /// let id = ProjectId::new("my-project");
    /// assert_eq!(id.as_str(), "my-project");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        ProjectId(id.into())
    }

    /// Get the string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::ProjectId;
    ///
    /// let id = ProjectId::new("test-project");
    /// assert_eq!(id.as_str(), "test-project");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ProjectId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProjectId(s.to_string()))
    }
}

/// A unique identifier for a commit/snapshot.
///
/// # Examples
///
/// ```
/// use sysml_id::CommitId;
///
/// let commit = CommitId::new("abc123def456");
/// assert_eq!(commit.as_str(), "abc123def456");
///
/// // Parse from string
/// let parsed: CommitId = "commit-hash".parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CommitId(String);

impl CommitId {
    /// Create a new CommitId from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::CommitId;
    ///
    /// let id = CommitId::new("abc123");
    /// assert_eq!(id.as_str(), "abc123");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        CommitId(id.into())
    }

    /// Get the string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::CommitId;
    ///
    /// let id = CommitId::new("commit-hash");
    /// assert_eq!(id.as_str(), "commit-hash");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CommitId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CommitId(s.to_string()))
    }
}

/// A qualified name representing a hierarchical path (e.g., "Package::Part::Attribute").
///
/// Qualified names support Unicode characters in segments and can properly escape
/// special characters like `::` and whitespace when needed.
///
/// # Examples
///
/// ```
/// use sysml_id::QualifiedName;
///
/// // Parse from string
/// let qn: QualifiedName = "Package::Part::Attribute".parse().unwrap();
/// assert_eq!(qn.len(), 3);
/// assert_eq!(qn.simple_name(), Some("Attribute"));
///
/// // Build programmatically
/// let qn = QualifiedName::from_single("Root").child("Level1").child("Level2");
/// assert_eq!(qn.to_string(), "Root::Level1::Level2");
///
/// // Unicode support
/// let qn: QualifiedName = "包::部品::属性".parse().unwrap();
/// assert_eq!(qn.simple_name(), Some("属性"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QualifiedName {
    segments: Vec<String>,
}

impl QualifiedName {
    /// Create an empty qualified name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn = QualifiedName::empty();
    /// assert!(qn.is_empty());
    /// assert_eq!(qn.len(), 0);
    /// ```
    pub fn empty() -> Self {
        QualifiedName {
            segments: Vec::new(),
        }
    }

    /// Create a qualified name from segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn = QualifiedName::from_segments(vec![
    ///     "Package".to_string(),
    ///     "Part".to_string(),
    /// ]);
    /// assert_eq!(qn.to_string(), "Package::Part");
    /// ```
    pub fn from_segments(segments: Vec<String>) -> Self {
        QualifiedName { segments }
    }

    /// Create a qualified name with a single segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn = QualifiedName::from_single("TopLevel");
    /// assert_eq!(qn.len(), 1);
    /// assert_eq!(qn.simple_name(), Some("TopLevel"));
    /// ```
    pub fn from_single(name: impl Into<String>) -> Self {
        QualifiedName {
            segments: vec![name.into()],
        }
    }

    /// Get the segments of this qualified name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn: QualifiedName = "A::B::C".parse().unwrap();
    /// assert_eq!(qn.segments(), &["A", "B", "C"]);
    /// ```
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Get the last segment (the simple name).
    ///
    /// Returns `None` if the qualified name is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn: QualifiedName = "Package::Part::Attribute".parse().unwrap();
    /// assert_eq!(qn.simple_name(), Some("Attribute"));
    ///
    /// let empty = QualifiedName::empty();
    /// assert_eq!(empty.simple_name(), None);
    /// ```
    pub fn simple_name(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// Get the parent qualified name (all but the last segment).
    ///
    /// Returns `None` if the qualified name has one or fewer segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn: QualifiedName = "A::B::C".parse().unwrap();
    /// let parent = qn.parent().unwrap();
    /// assert_eq!(parent.to_string(), "A::B");
    ///
    /// let single: QualifiedName = "Root".parse().unwrap();
    /// assert!(single.parent().is_none());
    /// ```
    pub fn parent(&self) -> Option<QualifiedName> {
        if self.segments.len() > 1 {
            Some(QualifiedName {
                segments: self.segments[..self.segments.len() - 1].to_vec(),
            })
        } else {
            None
        }
    }

    /// Append a segment to create a child qualified name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let parent: QualifiedName = "Package::Part".parse().unwrap();
    /// let child = parent.child("Attribute");
    /// assert_eq!(child.to_string(), "Package::Part::Attribute");
    /// ```
    pub fn child(&self, name: impl Into<String>) -> QualifiedName {
        let mut segments = self.segments.clone();
        segments.push(name.into());
        QualifiedName { segments }
    }

    /// Check if this qualified name is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// assert!(QualifiedName::empty().is_empty());
    ///
    /// let qn: QualifiedName = "A".parse().unwrap();
    /// assert!(!qn.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the number of segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn: QualifiedName = "A::B::C".parse().unwrap();
    /// assert_eq!(qn.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Check if this qualified name starts with another qualified name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let full: QualifiedName = "A::B::C::D".parse().unwrap();
    /// let prefix: QualifiedName = "A::B".parse().unwrap();
    /// assert!(full.starts_with(&prefix));
    ///
    /// let other: QualifiedName = "X::Y".parse().unwrap();
    /// assert!(!full.starts_with(&other));
    /// ```
    pub fn starts_with(&self, prefix: &QualifiedName) -> bool {
        if prefix.segments.len() > self.segments.len() {
            return false;
        }
        self.segments[..prefix.segments.len()] == prefix.segments[..]
    }

    /// Escape a segment for display, handling special characters.
    ///
    /// Escapes backslash as `\\` and colon as `\:`.
    fn escape_segment(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '\\' => result.push_str("\\\\"),
                ':' => result.push_str("\\:"),
                _ => result.push(c),
            }
        }
        result
    }

    /// Display with escaping for segments that contain special characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// // Normal segments display normally
    /// let qn = QualifiedName::from_segments(vec!["A".to_string(), "B".to_string()]);
    /// assert_eq!(qn.to_escaped_string(), "A::B");
    ///
    /// // Segments with colons are escaped
    /// let qn = QualifiedName::from_segments(vec!["A:B".to_string(), "C".to_string()]);
    /// assert_eq!(qn.to_escaped_string(), "A\\:B::C");
    /// ```
    pub fn to_escaped_string(&self) -> String {
        self.segments
            .iter()
            .map(|s| Self::escape_segment(s))
            .collect::<Vec<_>>()
            .join("::")
    }

    /// Parse a qualified name with escaped segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_id::QualifiedName;
    ///
    /// let qn = QualifiedName::parse_escaped("A\\:B::C").unwrap();
    /// assert_eq!(qn.segments(), &["A:B", "C"]);
    /// ```
    pub fn parse_escaped(s: &str) -> Result<Self, IdError> {
        if s.is_empty() {
            return Ok(QualifiedName::empty());
        }

        let mut segments = Vec::new();
        let mut current = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                // Handle escape sequences
                if let Some(&next) = chars.peek() {
                    match next {
                        '\\' | ':' => {
                            current.push(chars.next().unwrap());
                        }
                        _ => current.push(c),
                    }
                } else {
                    current.push(c);
                }
            } else if c == ':' {
                // Check for ::
                if chars.peek() == Some(&':') {
                    chars.next(); // consume second :
                    if current.is_empty() {
                        return Err(IdError::InvalidQualifiedName(format!(
                            "empty segment in '{}'",
                            s
                        )));
                    }
                    segments.push(current);
                    current = String::new();
                } else {
                    current.push(c);
                }
            } else {
                current.push(c);
            }
        }

        if current.is_empty() && !segments.is_empty() {
            return Err(IdError::InvalidQualifiedName(format!(
                "trailing separator in '{}'",
                s
            )));
        }

        if !current.is_empty() {
            segments.push(current);
        }

        Ok(QualifiedName { segments })
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.segments.join("::"))
    }
}

impl FromStr for QualifiedName {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(QualifiedName::empty());
        }

        let segments: Vec<String> = s.split("::").map(|s| s.trim().to_string()).collect();

        // Validate that no segment is empty
        for seg in &segments {
            if seg.is_empty() {
                return Err(IdError::InvalidQualifiedName(format!(
                    "empty segment in '{}'",
                    s
                )));
            }
        }

        Ok(QualifiedName { segments })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_id_roundtrip() {
        let id = ElementId::new_v4();
        let s = id.to_string();
        let parsed: ElementId = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn element_id_from_string() {
        let id1 = ElementId::from_string("test-element");
        let id2 = ElementId::from_string("test-element");
        assert_eq!(id1, id2);
    }

    #[test]
    fn project_id_roundtrip() {
        let id = ProjectId::new("my-project");
        assert_eq!(id.as_str(), "my-project");
        assert_eq!(id.to_string(), "my-project");
    }

    #[test]
    fn commit_id_roundtrip() {
        let id = CommitId::new("abc123");
        assert_eq!(id.as_str(), "abc123");
        assert_eq!(id.to_string(), "abc123");
    }

    #[test]
    fn qualified_name_from_str() {
        let qn: QualifiedName = "A::B::C".parse().unwrap();
        assert_eq!(qn.segments(), &["A", "B", "C"]);
        assert_eq!(qn.to_string(), "A::B::C");
    }

    #[test]
    fn qualified_name_display() {
        let qn = QualifiedName::from_segments(vec![
            "Package".to_string(),
            "Part".to_string(),
            "Attr".to_string(),
        ]);
        assert_eq!(qn.to_string(), "Package::Part::Attr");
    }

    #[test]
    fn qualified_name_simple_name() {
        let qn: QualifiedName = "A::B::C".parse().unwrap();
        assert_eq!(qn.simple_name(), Some("C"));
    }

    #[test]
    fn qualified_name_parent() {
        let qn: QualifiedName = "A::B::C".parse().unwrap();
        let parent = qn.parent().unwrap();
        assert_eq!(parent.to_string(), "A::B");
    }

    #[test]
    fn qualified_name_child() {
        let qn: QualifiedName = "A::B".parse().unwrap();
        let child = qn.child("C");
        assert_eq!(child.to_string(), "A::B::C");
    }

    #[test]
    fn qualified_name_empty() {
        let qn = QualifiedName::empty();
        assert!(qn.is_empty());
        assert_eq!(qn.len(), 0);
    }

    #[test]
    fn qualified_name_invalid() {
        let result: Result<QualifiedName, _> = "A::::B".parse();
        assert!(result.is_err());
    }

    #[test]
    fn qualified_name_unicode() {
        // Japanese
        let qn: QualifiedName = "パッケージ::部品::属性".parse().unwrap();
        assert_eq!(qn.len(), 3);
        assert_eq!(qn.simple_name(), Some("属性"));
        assert_eq!(qn.to_string(), "パッケージ::部品::属性");

        // Chinese
        let qn: QualifiedName = "包::部件::属性".parse().unwrap();
        assert_eq!(qn.simple_name(), Some("属性"));

        // Mixed
        let qn: QualifiedName = "Package::部品::Attribute".parse().unwrap();
        assert_eq!(qn.segments(), &["Package", "部品", "Attribute"]);
    }

    #[test]
    fn qualified_name_starts_with() {
        let full: QualifiedName = "A::B::C::D".parse().unwrap();
        let prefix: QualifiedName = "A::B".parse().unwrap();
        assert!(full.starts_with(&prefix));

        let other: QualifiedName = "X::Y".parse().unwrap();
        assert!(!full.starts_with(&other));

        // Self always starts with self
        assert!(full.starts_with(&full));

        // Empty prefix
        assert!(full.starts_with(&QualifiedName::empty()));
    }

    #[test]
    fn qualified_name_escaping() {
        // Segment with colon
        let qn = QualifiedName::from_segments(vec!["A:B".to_string(), "C".to_string()]);
        assert_eq!(qn.to_escaped_string(), "A\\:B::C");

        // Segment with backslash
        let qn = QualifiedName::from_segments(vec!["A\\B".to_string(), "C".to_string()]);
        assert_eq!(qn.to_escaped_string(), "A\\\\B::C");

        // Parse escaped
        let qn = QualifiedName::parse_escaped("A\\:B::C").unwrap();
        assert_eq!(qn.segments(), &["A:B", "C"]);

        // Parse with escaped backslash
        let qn = QualifiedName::parse_escaped("A\\\\B::C").unwrap();
        assert_eq!(qn.segments(), &["A\\B", "C"]);
    }

    #[test]
    fn qualified_name_with_whitespace() {
        // Whitespace is trimmed in normal parsing
        let qn: QualifiedName = "A :: B :: C".parse().unwrap();
        assert_eq!(qn.segments(), &["A", "B", "C"]);

        // Segments can contain whitespace in the middle
        let qn = QualifiedName::from_segments(vec!["A B".to_string(), "C D".to_string()]);
        assert_eq!(qn.segments(), &["A B", "C D"]);
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

        #[test]
        fn element_id_serde_roundtrip() {
            let id = ElementId::new_v4();
            let json = serde_json::to_string(&id).unwrap();
            let parsed: ElementId = serde_json::from_str(&json).unwrap();
            assert_eq!(id, parsed);
        }

        #[test]
        fn project_id_serde_roundtrip() {
            let id = ProjectId::new("test-project");
            let json = serde_json::to_string(&id).unwrap();
            let parsed: ProjectId = serde_json::from_str(&json).unwrap();
            assert_eq!(id, parsed);
        }

        #[test]
        fn commit_id_serde_roundtrip() {
            let id = CommitId::new("abc123");
            let json = serde_json::to_string(&id).unwrap();
            let parsed: CommitId = serde_json::from_str(&json).unwrap();
            assert_eq!(id, parsed);
        }

        #[test]
        fn qualified_name_serde_roundtrip() {
            let qn: QualifiedName = "A::B::C".parse().unwrap();
            let json = serde_json::to_string(&qn).unwrap();
            let parsed: QualifiedName = serde_json::from_str(&json).unwrap();
            assert_eq!(qn, parsed);
        }

        #[test]
        fn qualified_name_unicode_serde_roundtrip() {
            let qn: QualifiedName = "パッケージ::部品".parse().unwrap();
            let json = serde_json::to_string(&qn).unwrap();
            let parsed: QualifiedName = serde_json::from_str(&json).unwrap();
            assert_eq!(qn, parsed);
        }
    }
}
