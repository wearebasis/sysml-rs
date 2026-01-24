//! Single-pass extraction data structures.
//!
//! These structs hold all data extracted from pest pairs in a single traversal,
//! avoiding the overhead of multiple `.clone().into_inner()` calls.

use pest::iterators::Pair;

use crate::Rule;

/// All data extracted from a Usage element in one pass.
///
/// This struct captures all fields that would otherwise require multiple
/// tree traversals to extract.
#[derive(Debug, Default)]
pub struct UsageExtraction<'a> {
    // === Flags from prefix ===
    pub is_abstract: bool,
    pub is_variation: bool,
    pub is_readonly: bool,
    pub is_derived: bool,
    pub is_end: bool,
    pub is_reference: bool,
    // KerML feature modifiers (for standard library parsing)
    pub is_composite: bool,
    pub is_portion: bool,
    pub is_variable: bool,
    pub is_constant: bool,

    // === Direction ===
    pub direction: Option<&'static str>,

    // === Identification from declaration ===
    pub name: Option<String>,

    // === Multiplicity ===
    /// (lower, upper) where upper=None means unbounded (*)
    pub multiplicity: Option<(i64, Option<i64>)>,

    // === Feature value ===
    pub value_expression: Option<String>,
    pub value_is_default: bool,
    pub value_is_initial: bool,

    // === Feature specializations ===
    /// FeatureTyping targets (from `:` or `typed by` syntax)
    pub typings: Vec<String>,
    /// Subsetting targets (from `:>` or `subsets` syntax)
    pub subsettings: Vec<String>,
    /// Redefinition targets (from `:>>` or `redefines` syntax)
    pub redefinitions: Vec<String>,
    /// ReferenceSubsetting targets (from `::>` or `references` syntax)
    pub references: Vec<String>,

    // === Body members (stored for later processing) ===
    pub body_pairs: Vec<Pair<'a, Rule>>,
}

/// All data extracted from a Definition element in one pass.
#[derive(Debug, Default)]
pub struct DefinitionExtraction<'a> {
    // === Flags from prefix ===
    pub is_abstract: bool,
    pub is_variation: bool,

    // === Identification from declaration ===
    pub name: Option<String>,

    // === Subclassification targets ===
    pub subclassifications: Vec<String>,

    // === Body members (stored for later processing) ===
    pub body_pairs: Vec<Pair<'a, Rule>>,
}

/// All data extracted from a Package element in one pass.
#[derive(Debug, Default)]
pub struct PackageExtraction<'a> {
    pub name: Option<String>,
    pub is_standard: bool,
    pub body_pairs: Vec<Pair<'a, Rule>>,
}

/// Single-pass extraction implementation.
impl<'a> UsageExtraction<'a> {
    /// Create a new empty usage extraction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract all data from a usage pair in a single pass.
    pub fn from_pair(pair: Pair<'a, Rule>) -> Self {
        let mut extraction = Self::new();
        extraction.extract_from(pair);
        extraction
    }

    /// Extract data from a pair, populating self.
    fn extract_from(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                // Prefix rules containing flags
                Rule::RefPrefix
                | Rule::BasicUsagePrefix
                | Rule::UsagePrefix
                | Rule::OccurrenceUsagePrefix
                | Rule::BasicOccurrenceUsagePrefix
                | Rule::UnextendedUsagePrefix
                | Rule::EndUsagePrefix => {
                    self.extract_prefix(inner);
                }

                // Declaration rules containing name and specializations
                Rule::UsageDeclaration
                | Rule::FeatureDeclaration
                | Rule::ActionNodeUsageDeclaration => {
                    self.extract_declaration(inner);
                }

                // Body rules containing children
                Rule::UsageBody
                | Rule::DefinitionBody
                | Rule::ActionBody
                | Rule::StateBody
                | Rule::CalculationBody
                | Rule::FunctionBody
                | Rule::RequirementBody
                | Rule::CaseBody
                | Rule::ViewBody
                | Rule::InterfaceBody
                | Rule::EnumerationBody
                | Rule::MetadataBody
                | Rule::ActionNodeBody => {
                    self.extract_body(inner);
                }

                // UsageCompletion contains ValuePart (for attribute usages, etc.)
                Rule::UsageCompletion => {
                    self.extract_usage_completion(inner);
                }

                // ValuePart can appear directly in some contexts
                Rule::ValuePart => {
                    self.extract_value_part(inner);
                }

                // Direct flag rules (can appear at top level in some contexts)
                Rule::Abstract => self.is_abstract = true,
                Rule::Variation => self.is_variation = true,
                Rule::Readonly => self.is_readonly = true,
                Rule::Derived => self.is_derived = true,
                Rule::End => self.is_end = true,
                Rule::Reference => self.is_reference = true,
                Rule::Composite => self.is_composite = true,
                Rule::FeaturePortion => self.is_portion = true,
                Rule::Variable => self.is_variable = true,
                Rule::Constant => self.is_constant = true,

                // Direct direction keywords
                Rule::KW_IN => self.direction = Some("in"),
                Rule::KW_OUT => self.direction = Some("out"),
                Rule::KW_INOUT => self.direction = Some("inout"),

                // Skip other rules
                _ => {}
            }
        }
    }

    /// Extract flags and direction from prefix rules.
    fn extract_prefix(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                // Nested prefix rules
                Rule::RefPrefix
                | Rule::BasicUsagePrefix
                | Rule::UsagePrefix
                | Rule::OccurrenceUsagePrefix
                | Rule::BasicOccurrenceUsagePrefix
                | Rule::UnextendedUsagePrefix
                | Rule::EndUsagePrefix => {
                    self.extract_prefix(inner);
                }

                // Flag rules
                Rule::Abstract => self.is_abstract = true,
                Rule::Variation => self.is_variation = true,
                Rule::Readonly => self.is_readonly = true,
                Rule::Derived => self.is_derived = true,
                Rule::End => self.is_end = true,
                Rule::Reference => self.is_reference = true,
                Rule::Composite => self.is_composite = true,
                Rule::FeaturePortion => self.is_portion = true,
                Rule::Variable => self.is_variable = true,
                Rule::Constant => self.is_constant = true,

                // Direction
                Rule::FeatureDirectionKind => {
                    let text = inner.as_str().trim();
                    self.direction = match text {
                        "in" => Some("in"),
                        "out" => Some("out"),
                        "inout" => Some("inout"),
                        _ => None,
                    };
                }
                Rule::KW_IN => self.direction = Some("in"),
                Rule::KW_OUT => self.direction = Some("out"),
                Rule::KW_INOUT => self.direction = Some("inout"),

                _ => {}
            }
        }
    }

    /// Extract name, multiplicity, value, and specializations from declaration.
    fn extract_declaration(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                // Nested declaration
                Rule::UsageDeclaration
                | Rule::FeatureDeclaration
                | Rule::ActionNodeUsageDeclaration => {
                    self.extract_declaration(inner);
                }

                // Identification
                Rule::Identification => {
                    self.extract_identification(inner);
                }
                Rule::RegularName => {
                    if self.name.is_none() {
                        self.name = extract_name_from_regular_name(&inner);
                    }
                }

                // Feature specialization part
                Rule::FeatureSpecializationPart => {
                    self.extract_feature_specialization_part(inner);
                }
                Rule::FeatureSpecialization => {
                    self.extract_feature_specialization(inner);
                }

                // Multiplicity
                Rule::MultiplicityPart | Rule::OwnedMultiplicity => {
                    self.extract_multiplicity_part(inner);
                }

                // Value part
                Rule::ValuePart => {
                    self.extract_value_part(inner);
                }

                // Usage completion (contains value part)
                Rule::UsageCompletion => {
                    self.extract_usage_completion(inner);
                }

                _ => {}
            }
        }
    }

    /// Extract name from Identification.
    fn extract_identification(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::RegularName = inner.as_rule() {
                self.name = extract_name_from_regular_name(&inner);
                return;
            }
        }
    }

    /// Extract from FeatureSpecializationPart.
    fn extract_feature_specialization_part(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::FeatureSpecialization => {
                    self.extract_feature_specialization(inner);
                }
                Rule::MultiplicityPart | Rule::OwnedMultiplicity => {
                    self.extract_multiplicity_part(inner);
                }
                _ => {}
            }
        }
    }

    /// Extract from FeatureSpecialization.
    fn extract_feature_specialization(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::Typings => {
                    extract_qualified_names_into(&inner, &mut self.typings);
                }
                Rule::Subsettings => {
                    extract_qualified_names_into(&inner, &mut self.subsettings);
                }
                Rule::Redefinitions => {
                    extract_qualified_names_into(&inner, &mut self.redefinitions);
                }
                Rule::References_ => {
                    extract_qualified_names_into(&inner, &mut self.references);
                }
                _ => {}
            }
        }
    }

    /// Extract multiplicity bounds.
    fn extract_multiplicity_part(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::MultiplicityRange => {
                    self.multiplicity = parse_multiplicity_text(inner.as_str());
                    return;
                }
                Rule::OwnedMultiplicity | Rule::MultiplicityPart => {
                    self.extract_multiplicity_part(inner);
                    if self.multiplicity.is_some() {
                        return;
                    }
                }
                _ => {}
            }
        }
    }

    /// Extract from UsageCompletion.
    fn extract_usage_completion(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::ValuePart = inner.as_rule() {
                self.extract_value_part(inner);
            }
        }
    }

    /// Extract feature value.
    fn extract_value_part(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::FeatureValue = inner.as_rule() {
                self.extract_feature_value(inner);
                return;
            }
        }
    }

    /// Extract from FeatureValue.
    fn extract_feature_value(&mut self, pair: Pair<'a, Rule>) {
        let text = pair.as_str();

        // Check for "default" keyword
        if text.starts_with("default") {
            self.value_is_default = true;
        }

        // Check for ":=" operator
        if text.contains(":=") {
            self.value_is_initial = true;
        }

        // Find OwnedExpression child
        for inner in pair.clone().into_inner() {
            if inner.as_rule() == Rule::OwnedExpression {
                self.value_expression = Some(inner.as_str().trim().to_string());
                return;
            }
        }

        // Fallback: extract from text
        let mut s = text.trim();
        if s.starts_with("default") {
            s = s.strip_prefix("default").unwrap_or(s).trim();
        }
        if s.starts_with(":=") {
            s = s.strip_prefix(":=").unwrap_or(s).trim();
        } else if s.starts_with('=') {
            s = s.strip_prefix('=').unwrap_or(s).trim();
        }
        if !s.is_empty() {
            self.value_expression = Some(s.to_string());
        }
    }

    /// Extract body children pairs for later processing.
    fn extract_body(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            // Store body items for later processing by the work stack
            self.body_pairs.push(inner);
        }
    }
}

impl<'a> DefinitionExtraction<'a> {
    /// Create a new empty definition extraction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract all data from a definition pair in a single pass.
    pub fn from_pair(pair: Pair<'a, Rule>) -> Self {
        let mut extraction = Self::new();
        extraction.extract_from(pair);
        extraction
    }

    /// Extract data from a pair, populating self.
    fn extract_from(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                // Prefix rules containing flags
                Rule::BasicDefinitionPrefix
                | Rule::DefinitionPrefix
                | Rule::OccurrenceDefinitionPrefix => {
                    self.extract_prefix(inner);
                }

                // Declaration rules
                Rule::DefinitionDeclaration => {
                    self.extract_declaration(inner);
                }

                // Body rules
                Rule::DefinitionBody
                | Rule::ActionBody
                | Rule::StateBody
                | Rule::CalculationBody
                | Rule::FunctionBody
                | Rule::RequirementBody
                | Rule::CaseBody
                | Rule::ViewBody
                | Rule::InterfaceBody
                | Rule::EnumerationBody
                | Rule::MetadataBody => {
                    self.extract_body(inner);
                }

                // Direct flag rules
                Rule::Abstract => self.is_abstract = true,
                Rule::Variation => self.is_variation = true,

                _ => {}
            }
        }
    }

    /// Extract flags from prefix rules.
    fn extract_prefix(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::BasicDefinitionPrefix
                | Rule::DefinitionPrefix
                | Rule::OccurrenceDefinitionPrefix => {
                    self.extract_prefix(inner);
                }
                Rule::Abstract => self.is_abstract = true,
                Rule::Variation => self.is_variation = true,
                _ => {}
            }
        }
    }

    /// Extract name and subclassifications from declaration.
    fn extract_declaration(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::Identification => {
                    self.extract_identification(inner);
                }
                Rule::RegularName => {
                    if self.name.is_none() {
                        self.name = extract_name_from_regular_name(&inner);
                    }
                }
                Rule::SubclassificationPart => {
                    self.extract_subclassification_part(inner);
                }
                _ => {}
            }
        }
    }

    /// Extract name from Identification.
    fn extract_identification(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::RegularName = inner.as_rule() {
                self.name = extract_name_from_regular_name(&inner);
                return;
            }
        }
    }

    /// Extract subclassification targets.
    fn extract_subclassification_part(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::OwnedSubclassification = inner.as_rule() {
                let qname = inner.as_str().trim().to_string();
                if !qname.is_empty() {
                    self.subclassifications.push(qname);
                }
            }
        }
    }

    /// Extract body children pairs for later processing.
    fn extract_body(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            self.body_pairs.push(inner);
        }
    }
}

impl<'a> PackageExtraction<'a> {
    /// Create a new empty package extraction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract all data from a package pair in a single pass.
    pub fn from_pair(pair: Pair<'a, Rule>, is_library: bool) -> Self {
        let mut extraction = Self::new();
        extraction.extract_from(pair, is_library);
        extraction
    }

    /// Extract data from a pair, populating self.
    fn extract_from(&mut self, pair: Pair<'a, Rule>, is_library: bool) {
        // Check for 'standard' flag in library packages
        if is_library && pair.as_str().starts_with("standard") {
            self.is_standard = true;
        }

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::Identification => {
                    self.extract_identification(inner);
                }
                Rule::RegularName => {
                    if self.name.is_none() {
                        self.name = extract_name_from_regular_name(&inner);
                    }
                }
                Rule::PackageBody => {
                    self.extract_body(inner);
                }
                _ => {}
            }
        }
    }

    /// Extract name from Identification.
    fn extract_identification(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            if let Rule::RegularName = inner.as_rule() {
                self.name = extract_name_from_regular_name(&inner);
                return;
            }
        }
    }

    /// Extract body children pairs for later processing.
    fn extract_body(&mut self, pair: Pair<'a, Rule>) {
        for inner in pair.into_inner() {
            self.body_pairs.push(inner);
        }
    }
}

// =============================================================================
// Helper functions
// =============================================================================

/// Extract name from RegularName pair.
fn extract_name_from_regular_name(pair: &Pair<'_, Rule>) -> Option<String> {
    for inner in pair.clone().into_inner() {
        if let Rule::Name = inner.as_rule() {
            let inner_str = inner.as_str().to_string();
            for name_inner in inner.into_inner() {
                match name_inner.as_rule() {
                    Rule::ID => return Some(name_inner.as_str().to_string()),
                    Rule::UNRESTRICTED_NAME => {
                        let s = name_inner.as_str();
                        if s.len() >= 2 {
                            return Some(s[1..s.len() - 1].to_string());
                        }
                    }
                    _ => {}
                }
            }
            return Some(inner_str);
        }
    }
    Some(pair.as_str().to_string())
}

/// Extract qualified names from a pair into a vec.
fn extract_qualified_names_into(pair: &Pair<'_, Rule>, targets: &mut Vec<String>) {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::QualifiedName => {
                let name = inner.as_str().trim().to_string();
                if !name.is_empty() {
                    targets.push(name);
                }
            }
            Rule::TypedBy
            | Rule::FeatureTyping
            | Rule::OwnedFeatureTyping
            | Rule::FeatureType
            | Rule::Subsets
            | Rule::OwnedSubsetting
            | Rule::Redefines
            | Rule::OwnedRedefinition
            | Rule::OwnedReferenceSubsetting => {
                extract_qualified_names_into(&inner, targets);
            }
            _ => {
                // Recurse into other children
                extract_qualified_names_into(&inner, targets);
            }
        }
    }
}

/// Parse multiplicity text like "[4]", "[0..10]", "[*]", "[1..*]".
fn parse_multiplicity_text(text: &str) -> Option<(i64, Option<i64>)> {
    let content = text.trim_start_matches('[').trim_end_matches(']').trim();

    if content == "*" {
        return Some((0, None));
    }

    if let Some(dot_pos) = content.find("..") {
        let lower_str = content[..dot_pos].trim();
        let upper_str = content[dot_pos + 2..].trim();

        let lower: i64 = lower_str.parse().ok()?;
        let upper = if upper_str == "*" {
            None
        } else {
            Some(upper_str.parse().ok()?)
        };

        Some((lower, upper))
    } else {
        let value: i64 = content.parse().ok()?;
        Some((value, Some(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiplicity_exact() {
        assert_eq!(parse_multiplicity_text("[4]"), Some((4, Some(4))));
    }

    #[test]
    fn test_parse_multiplicity_range() {
        assert_eq!(parse_multiplicity_text("[0..10]"), Some((0, Some(10))));
    }

    #[test]
    fn test_parse_multiplicity_unbounded() {
        assert_eq!(parse_multiplicity_text("[*]"), Some((0, None)));
        assert_eq!(parse_multiplicity_text("[1..*]"), Some((1, None)));
    }
}
