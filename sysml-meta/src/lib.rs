//! # sysml-meta
//!
//! Metadata types for SysML v2: applicability, clause references, and values.
//!
//! This crate provides types for representing metadata about model elements,
//! including applicability status, references to standards clauses, and
//! a flexible value type for property values.
//!
//! ## Features
//!
//! - `serde`: Enable serde serialization support
//!
//! ## Examples
//!
//! ```
//! use sysml_meta::{Applicability, ClauseRef, Value};
//!
//! // Check applicability
//! let status = Applicability::Applicable;
//! assert!(status.is_applicable());
//!
//! // Reference a clause in a standard
//! let clause = ClauseRef::new("ISO 26262", "5.4.3");
//!
//! // Create and compare values
//! let v1 = Value::from(10);
//! let v2 = Value::from(20);
//! assert!(v1.partial_cmp_value(&v2) == Some(std::cmp::Ordering::Less));
//! ```

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use sysml_id::ElementId;

/// Applicability status of a requirement or element.
///
/// # Examples
///
/// ```
/// use sysml_meta::Applicability;
///
/// let status = Applicability::Applicable;
/// assert!(status.is_applicable());
///
/// let tbd = Applicability::default();
/// assert!(tbd.is_tbd());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Applicability {
    /// The element is applicable.
    Applicable,
    /// The element is not applicable.
    NotApplicable,
    /// Applicability is to be determined.
    #[default]
    TBD,
}

impl Applicability {
    /// Check if this is applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Applicability;
    ///
    /// assert!(Applicability::Applicable.is_applicable());
    /// assert!(!Applicability::NotApplicable.is_applicable());
    /// ```
    pub fn is_applicable(&self) -> bool {
        matches!(self, Applicability::Applicable)
    }

    /// Check if this is not applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Applicability;
    ///
    /// assert!(Applicability::NotApplicable.is_not_applicable());
    /// assert!(!Applicability::Applicable.is_not_applicable());
    /// ```
    pub fn is_not_applicable(&self) -> bool {
        matches!(self, Applicability::NotApplicable)
    }

    /// Check if this is to be determined.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Applicability;
    ///
    /// assert!(Applicability::TBD.is_tbd());
    /// assert!(!Applicability::Applicable.is_tbd());
    /// ```
    pub fn is_tbd(&self) -> bool {
        matches!(self, Applicability::TBD)
    }
}

impl fmt::Display for Applicability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Applicability::Applicable => write!(f, "applicable"),
            Applicability::NotApplicable => write!(f, "not applicable"),
            Applicability::TBD => write!(f, "TBD"),
        }
    }
}

/// The kind/purpose of a clause.
///
/// # Examples
///
/// ```
/// use sysml_meta::ClauseKind;
///
/// let kind = ClauseKind::Operational;
/// assert_eq!(kind.to_string(), "operational");
///
/// let default = ClauseKind::default();
/// assert!(matches!(default, ClauseKind::Operational));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum ClauseKind {
    /// Operational clause (normative).
    #[default]
    Operational,
    /// Test clause.
    Test,
    /// Informative clause (non-normative).
    Informative,
}

impl fmt::Display for ClauseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClauseKind::Operational => write!(f, "operational"),
            ClauseKind::Test => write!(f, "test"),
            ClauseKind::Informative => write!(f, "informative"),
        }
    }
}

/// A reference to a clause in a standard document.
///
/// # Examples
///
/// ```
/// use sysml_meta::ClauseRef;
///
/// // Simple reference
/// let clause = ClauseRef::new("ISO 26262", "5.4.3");
/// assert_eq!(clause.to_string(), "ISO 26262 §5.4.3");
///
/// // With edition
/// let clause = ClauseRef::with_edition("ISO 26262", "2018", "5.4.3");
/// assert_eq!(clause.to_string(), "ISO 26262 (2018) §5.4.3");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClauseRef {
    /// The name/identifier of the standard (e.g., "ISO 26262").
    pub standard: String,
    /// The edition or version of the standard (optional).
    pub edition: Option<String>,
    /// The clause identifier (e.g., "5.4.3").
    pub clause_id: String,
}

impl ClauseRef {
    /// Create a new clause reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::ClauseRef;
    ///
    /// let clause = ClauseRef::new("ISO 26262", "5.4.3");
    /// assert_eq!(clause.standard, "ISO 26262");
    /// assert_eq!(clause.clause_id, "5.4.3");
    /// assert!(clause.edition.is_none());
    /// ```
    pub fn new(standard: impl Into<String>, clause_id: impl Into<String>) -> Self {
        ClauseRef {
            standard: standard.into(),
            edition: None,
            clause_id: clause_id.into(),
        }
    }

    /// Create a new clause reference with edition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::ClauseRef;
    ///
    /// let clause = ClauseRef::with_edition("ISO 26262", "2018", "5.4.3");
    /// assert_eq!(clause.edition, Some("2018".to_string()));
    /// ```
    pub fn with_edition(
        standard: impl Into<String>,
        edition: impl Into<String>,
        clause_id: impl Into<String>,
    ) -> Self {
        ClauseRef {
            standard: standard.into(),
            edition: Some(edition.into()),
            clause_id: clause_id.into(),
        }
    }
}

impl fmt::Display for ClauseRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(edition) = &self.edition {
            write!(f, "{} ({}) §{}", self.standard, edition, self.clause_id)
        } else {
            write!(f, "{} §{}", self.standard, self.clause_id)
        }
    }
}

/// A flexible value type for element properties.
///
/// # Examples
///
/// ```
/// use sysml_meta::Value;
/// use std::cmp::Ordering;
///
/// // Create values from various types
/// let b: Value = true.into();
/// let i: Value = 42i64.into();
/// let f: Value = 3.14.into();
/// let s: Value = "hello".into();
///
/// // Access values
/// assert_eq!(i.as_int(), Some(42));
/// assert_eq!(i.as_float(), Some(42.0));
/// assert_eq!(s.as_str(), Some("hello"));
///
/// // Compare numeric values
/// let a = Value::from(10);
/// let b = Value::from(20);
/// assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Less));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Value {
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Int(i64),
    /// Floating-point value.
    Float(f64),
    /// String value.
    String(String),
    /// Enumeration value (stored as string).
    Enum(String),
    /// Reference to another element.
    Ref(ElementId),
    /// List of values.
    List(Vec<Value>),
    /// Map of key-value pairs.
    Map(BTreeMap<String, Value>),
    /// Null/empty value.
    Null,
}

impl Value {
    /// Create a null value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::null();
    /// assert!(v.is_null());
    /// ```
    pub fn null() -> Self {
        Value::Null
    }

    /// Check if this is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// assert!(Value::Null.is_null());
    /// assert!(!Value::Bool(true).is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Try to get as bool.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::Bool(true);
    /// assert_eq!(v.as_bool(), Some(true));
    ///
    /// let v = Value::Int(1);
    /// assert_eq!(v.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::Int(42);
    /// assert_eq!(v.as_int(), Some(42));
    ///
    /// let v = Value::Float(42.0);
    /// assert_eq!(v.as_int(), None); // Float is not Int
    /// ```
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Try to get as float.
    ///
    /// Integers are automatically converted to floats.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::Float(3.14);
    /// assert_eq!(v.as_float(), Some(3.14));
    ///
    /// // Integers are converted to float
    /// let v = Value::Int(42);
    /// assert_eq!(v.as_float(), Some(42.0));
    /// ```
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Try to get as string.
    ///
    /// Works for both String and Enum values.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::String("hello".to_string());
    /// assert_eq!(v.as_str(), Some("hello"));
    ///
    /// let v = Value::Enum("variant".to_string());
    /// assert_eq!(v.as_str(), Some("variant"));
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::Enum(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as element reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    /// use sysml_id::ElementId;
    ///
    /// let id = ElementId::new_v4();
    /// let v = Value::Ref(id.clone());
    /// assert_eq!(v.as_ref(), Some(&id));
    /// ```
    pub fn as_ref(&self) -> Option<&ElementId> {
        match self {
            Value::Ref(id) => Some(id),
            _ => None,
        }
    }

    /// Try to get as list.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let v = Value::List(vec![Value::Int(1), Value::Int(2)]);
    /// assert!(v.as_list().is_some());
    /// assert_eq!(v.as_list().unwrap().len(), 2);
    /// ```
    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    /// Try to get as map.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    /// use std::collections::BTreeMap;
    ///
    /// let mut map = BTreeMap::new();
    /// map.insert("key".to_string(), Value::Int(42));
    /// let v = Value::Map(map);
    /// assert!(v.as_map().is_some());
    /// ```
    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Get the type name of this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// assert_eq!(Value::Bool(true).type_name(), "bool");
    /// assert_eq!(Value::Int(42).type_name(), "int");
    /// assert_eq!(Value::Float(3.14).type_name(), "float");
    /// assert_eq!(Value::Null.type_name(), "null");
    /// ```
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Enum(_) => "enum",
            Value::Ref(_) => "ref",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Null => "null",
        }
    }

    /// Compare two values numerically.
    ///
    /// Returns `Some(Ordering)` if both values are numeric (Int or Float),
    /// or `None` if comparison is not possible.
    ///
    /// Integer values are automatically promoted to float for comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    /// use std::cmp::Ordering;
    ///
    /// let a = Value::Int(10);
    /// let b = Value::Int(20);
    /// assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Less));
    ///
    /// // Mixed int/float comparison
    /// let a = Value::Int(10);
    /// let b = Value::Float(10.5);
    /// assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Less));
    ///
    /// // Equal values
    /// let a = Value::Float(3.14);
    /// let b = Value::Float(3.14);
    /// assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Equal));
    ///
    /// // Non-numeric values return None
    /// let a = Value::String("hello".to_string());
    /// let b = Value::Int(10);
    /// assert_eq!(a.partial_cmp_value(&b), None);
    /// ```
    pub fn partial_cmp_value(&self, other: &Value) -> Option<Ordering> {
        match (self.as_float(), other.as_float()) {
            (Some(a), Some(b)) => a.partial_cmp(&b),
            _ => None,
        }
    }

    /// Check if this value is numerically less than another.
    ///
    /// Returns `None` if either value is not numeric.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let a = Value::Int(10);
    /// let b = Value::Int(20);
    /// assert_eq!(a.is_less_than(&b), Some(true));
    /// assert_eq!(b.is_less_than(&a), Some(false));
    /// ```
    pub fn is_less_than(&self, other: &Value) -> Option<bool> {
        self.partial_cmp_value(other).map(|o| o == Ordering::Less)
    }

    /// Check if this value is numerically greater than another.
    ///
    /// Returns `None` if either value is not numeric.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysml_meta::Value;
    ///
    /// let a = Value::Int(20);
    /// let b = Value::Int(10);
    /// assert_eq!(a.is_greater_than(&b), Some(true));
    /// ```
    pub fn is_greater_than(&self, other: &Value) -> Option<bool> {
        self.partial_cmp_value(other).map(|o| o == Ordering::Greater)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Enum(e) => write!(f, "{}", e),
            Value::Ref(id) => write!(f, "@{}", id),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Null => write!(f, "null"),
        }
    }
}

// Convenience From implementations
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<ElementId> for Value {
    fn from(id: ElementId) -> Self {
        Value::Ref(id)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::List(v.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applicability_checks() {
        assert!(Applicability::Applicable.is_applicable());
        assert!(Applicability::NotApplicable.is_not_applicable());
        assert!(Applicability::TBD.is_tbd());
    }

    #[test]
    fn applicability_display() {
        assert_eq!(Applicability::Applicable.to_string(), "applicable");
        assert_eq!(Applicability::NotApplicable.to_string(), "not applicable");
        assert_eq!(Applicability::TBD.to_string(), "TBD");
    }

    #[test]
    fn clause_ref_basic() {
        let clause = ClauseRef::new("ISO 26262", "5.4.3");
        assert_eq!(clause.to_string(), "ISO 26262 §5.4.3");
    }

    #[test]
    fn clause_ref_with_edition() {
        let clause = ClauseRef::with_edition("ISO 26262", "2018", "5.4.3");
        assert_eq!(clause.to_string(), "ISO 26262 (2018) §5.4.3");
    }

    #[test]
    fn value_bool() {
        let v = Value::Bool(true);
        assert_eq!(v.as_bool(), Some(true));
        assert_eq!(v.type_name(), "bool");
    }

    #[test]
    fn value_int() {
        let v = Value::Int(42);
        assert_eq!(v.as_int(), Some(42));
        assert_eq!(v.as_float(), Some(42.0));
    }

    #[test]
    fn value_string() {
        let v = Value::String("hello".to_string());
        assert_eq!(v.as_str(), Some("hello"));
    }

    #[test]
    fn value_list() {
        let v = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert!(v.as_list().is_some());
        assert_eq!(v.as_list().unwrap().len(), 3);
    }

    #[test]
    fn value_map() {
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Value::Int(42));
        let v = Value::Map(map);
        assert!(v.as_map().is_some());
    }

    #[test]
    fn value_from_conversions() {
        let v: Value = true.into();
        assert!(matches!(v, Value::Bool(true)));

        let v: Value = 42i64.into();
        assert!(matches!(v, Value::Int(42)));

        let v: Value = "hello".into();
        assert!(matches!(v, Value::String(_)));
    }

    #[test]
    fn value_display() {
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::String("hello".into()).to_string(), "\"hello\"");
        assert_eq!(Value::Null.to_string(), "null");
    }

    #[test]
    fn clause_kind_display() {
        assert_eq!(ClauseKind::Operational.to_string(), "operational");
        assert_eq!(ClauseKind::Test.to_string(), "test");
        assert_eq!(ClauseKind::Informative.to_string(), "informative");
    }

    #[test]
    fn value_partial_cmp_int() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp_value(&a), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp_value(&a), Some(Ordering::Equal));
    }

    #[test]
    fn value_partial_cmp_float() {
        let a = Value::Float(3.14);
        let b = Value::Float(2.71);
        assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Greater));
    }

    #[test]
    fn value_partial_cmp_mixed() {
        let a = Value::Int(10);
        let b = Value::Float(10.5);
        assert_eq!(a.partial_cmp_value(&b), Some(Ordering::Less));
    }

    #[test]
    fn value_partial_cmp_non_numeric() {
        let a = Value::String("hello".to_string());
        let b = Value::Int(10);
        assert_eq!(a.partial_cmp_value(&b), None);
    }

    #[test]
    fn value_comparison_helpers() {
        let a = Value::Int(10);
        let b = Value::Int(20);

        assert_eq!(a.is_less_than(&b), Some(true));
        assert_eq!(a.is_greater_than(&b), Some(false));
        assert_eq!(b.is_greater_than(&a), Some(true));
    }
}
