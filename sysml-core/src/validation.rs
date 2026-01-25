//! Validation types for SysML element property constraints.
//!
//! This module provides types for representing validation errors
//! when element properties don't match their shape constraints.

use std::fmt;

/// A validation error for an element property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// The property that failed validation.
    pub property: String,
    /// The kind of validation error.
    pub kind: ValidationErrorKind,
}

impl ValidationError {
    /// Create an error for a missing required property.
    pub fn missing_required(property: impl Into<String>) -> Self {
        ValidationError {
            property: property.into(),
            kind: ValidationErrorKind::MissingRequired,
        }
    }

    /// Create an error for a property with the wrong type.
    pub fn wrong_type(
        property: impl Into<String>,
        expected: &'static str,
        got: impl Into<String>,
    ) -> Self {
        ValidationError {
            property: property.into(),
            kind: ValidationErrorKind::WrongType {
                expected,
                got: got.into(),
            },
        }
    }

    /// Create an error for a property that should have at least one value.
    pub fn min_cardinality(property: impl Into<String>) -> Self {
        ValidationError {
            property: property.into(),
            kind: ValidationErrorKind::MinCardinality,
        }
    }

    /// Create an error for a property that should have at most one value.
    pub fn max_cardinality(property: impl Into<String>) -> Self {
        ValidationError {
            property: property.into(),
            kind: ValidationErrorKind::MaxCardinality,
        }
    }

    /// Create an error for a read-only property being modified.
    pub fn read_only(property: impl Into<String>) -> Self {
        ValidationError {
            property: property.into(),
            kind: ValidationErrorKind::ReadOnly,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValidationErrorKind::MissingRequired => {
                write!(f, "missing required property '{}'", self.property)
            }
            ValidationErrorKind::WrongType { expected, got } => {
                write!(
                    f,
                    "property '{}' has wrong type: expected {}, got {}",
                    self.property, expected, got
                )
            }
            ValidationErrorKind::MinCardinality => {
                write!(
                    f,
                    "property '{}' requires at least one value",
                    self.property
                )
            }
            ValidationErrorKind::MaxCardinality => {
                write!(f, "property '{}' allows at most one value", self.property)
            }
            ValidationErrorKind::ReadOnly => {
                write!(f, "property '{}' is read-only", self.property)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Convert ValidationError to Diagnostic for unified error reporting.
///
/// Error codes:
/// - V001: MissingRequired
/// - V002: WrongType
/// - V003: MinCardinality
/// - V004: MaxCardinality
/// - V005: ReadOnly
impl From<ValidationError> for sysml_span::Diagnostic {
    fn from(error: ValidationError) -> Self {
        let code = match &error.kind {
            ValidationErrorKind::MissingRequired => "V001",
            ValidationErrorKind::WrongType { .. } => "V002",
            ValidationErrorKind::MinCardinality => "V003",
            ValidationErrorKind::MaxCardinality => "V004",
            ValidationErrorKind::ReadOnly => "V005",
        };

        sysml_span::Diagnostic::error(format!("{}: {}", error.property, error.kind))
            .with_code(code.to_string())
    }
}

/// The kind of validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationErrorKind {
    /// A required property (Exactly-one) is missing.
    MissingRequired,
    /// The property value has the wrong type.
    WrongType {
        /// The expected type name.
        expected: &'static str,
        /// The actual type name.
        got: String,
    },
    /// A One-or-many property has no values.
    MinCardinality,
    /// A Zero-or-one or Exactly-one property has multiple values.
    MaxCardinality,
    /// A read-only property was modified.
    ReadOnly,
}


impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationErrorKind::MissingRequired => {
                write!(f, "missing required property")
            }
            ValidationErrorKind::WrongType { expected, got } => {
                write!(f, "wrong type: expected {}, got {}", expected, got)
            }
            ValidationErrorKind::MinCardinality => {
                write!(f, "requires at least one value")
            }
            ValidationErrorKind::MaxCardinality => {
                write!(f, "allows at most one value")
            }
            ValidationErrorKind::ReadOnly => {
                write!(f, "read-only property")
            }
        }
    }
}

/// Result of validating an element.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// The errors found during validation.
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    /// Create a new empty validation result.
    pub fn new() -> Self {
        ValidationResult { errors: Vec::new() }
    }

    /// Check if validation passed (no errors).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Add an error to the result.
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Merge another validation result into this one.
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid() {
            write!(f, "validation passed")
        } else {
            writeln!(f, "validation failed with {} error(s):", self.errors.len())?;
            for error in &self.errors {
                writeln!(f, "  - {}", error)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_required() {
        let error = ValidationError::missing_required("elementId");
        assert_eq!(error.property, "elementId");
        assert!(matches!(error.kind, ValidationErrorKind::MissingRequired));
        assert!(error.to_string().contains("missing required"));
    }

    #[test]
    fn test_wrong_type() {
        let error = ValidationError::wrong_type("owningType", "ElementId", "string");
        assert_eq!(error.property, "owningType");
        assert!(matches!(error.kind, ValidationErrorKind::WrongType { .. }));
        assert!(error.to_string().contains("wrong type"));
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid());

        result.add_error(ValidationError::missing_required("prop1"));
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);

        result.add_error(ValidationError::wrong_type("prop2", "bool", "string"));
        assert_eq!(result.error_count(), 2);
    }

    #[test]
    fn test_merge_results() {
        let mut result1 = ValidationResult::new();
        result1.add_error(ValidationError::missing_required("a"));

        let mut result2 = ValidationResult::new();
        result2.add_error(ValidationError::missing_required("b"));

        result1.merge(result2);
        assert_eq!(result1.error_count(), 2);
    }

    // === Diagnostic Conversion Tests (Phase 5) ===

    #[test]
    fn validation_error_to_diagnostic() {
        use sysml_span::Diagnostic;

        let error = ValidationError::missing_required("elementId");
        let diag: Diagnostic = error.into();

        assert!(diag.is_error());
        assert_eq!(diag.code, Some("V001".to_string()));
        assert!(diag.message.contains("elementId"));
    }

    #[test]
    fn all_validation_errors_have_codes() {
        use sysml_span::Diagnostic;

        let errors = vec![
            ValidationError::missing_required("prop1"),
            ValidationError::wrong_type("prop2", "ElementId", "string"),
            ValidationError::min_cardinality("prop3"),
            ValidationError::max_cardinality("prop4"),
            ValidationError::read_only("prop5"),
        ];

        let expected_codes = ["V001", "V002", "V003", "V004", "V005"];

        for (error, expected_code) in errors.into_iter().zip(expected_codes.iter()) {
            let diag: Diagnostic = error.into();
            assert_eq!(diag.code, Some(expected_code.to_string()), "Wrong code for error");
            assert!(diag.is_error());
        }
    }
}
