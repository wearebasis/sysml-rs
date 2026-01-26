//! Build script for sysml-text-pest.
//!
//! This script generates the complete sysml.pest grammar by:
//! 1. Parsing xtext specification files to extract keywords, operators, and enums
//! 2. Generating pest rules for the extracted data
//! 3. Concatenating manual fragment files with generated rules
//! 4. Writing the final grammar to OUT_DIR

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use sysml_codegen::{
    extract_all_keyword_strings, generate_pest_enums, generate_pest_keywords_from_strings,
    generate_pest_operators, parse_ttl_vocab, parse_xtext_enums, parse_xtext_operators,
    parse_xtext_rule_names, parse_xtext_rules, validate_expression_coverage,
    validate_grammar_element_linkage, validate_keyword_coverage, validate_xtext_rule_coverage,
};

/// Paths to xtext specification files relative to the references directory.
const SYSML_XTEXT_PATH: &str =
    "SysML-v2-Pilot-Implementation/org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext";
const KERML_XTEXT_PATH: &str =
    "SysML-v2-Pilot-Implementation/org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext";
const KERML_EXPRESSIONS_PATH: &str = "SysML-v2-Pilot-Implementation/org.omg.kerml.expressions.xtext/src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext";

/// Extra keywords that are used in the grammar but not found in xtext files.
/// These may be from future spec versions, extensions, or grammar variations.
const EXTRA_KEYWORDS: &[&str] = &[
    "readonly",  // Feature property
    "bool",      // Boolean type alias
];

fn main() {
    // Find the references directory
    let refs_dir = find_references_dir().expect("Could not find references/sysmlv2 directory");

    // Print rerun triggers for spec files
    println!(
        "cargo:rerun-if-changed={}",
        refs_dir.join(SYSML_XTEXT_PATH).display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        refs_dir.join(KERML_XTEXT_PATH).display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        refs_dir.join(KERML_EXPRESSIONS_PATH).display()
    );

    // Print rerun triggers for fragment files
    println!("cargo:rerun-if-changed=src/grammar/fragments/");

    // Print rerun triggers for TTL vocab files (used for element linkage validation)
    println!(
        "cargo:rerun-if-changed={}",
        refs_dir.join("Kerml-Vocab.ttl").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        refs_dir.join("SysML-vocab.ttl").display()
    );

    // Load specification files
    let sysml_xtext = fs::read_to_string(refs_dir.join(SYSML_XTEXT_PATH))
        .expect("Failed to read SysML.xtext");
    let kerml_xtext =
        fs::read_to_string(refs_dir.join(KERML_XTEXT_PATH)).expect("Failed to read KerML.xtext");
    let kerml_expr = fs::read_to_string(refs_dir.join(KERML_EXPRESSIONS_PATH))
        .expect("Failed to read KerMLExpressions.xtext");

    // Extract all keywords from specifications (comprehensive extraction)
    let mut all_keywords = std::collections::HashSet::new();
    all_keywords.extend(extract_all_keyword_strings(&sysml_xtext));
    all_keywords.extend(extract_all_keyword_strings(&kerml_xtext));
    all_keywords.extend(extract_all_keyword_strings(&kerml_expr));

    // Add extra keywords used in the grammar but not in current xtext
    // These may be from future spec versions or grammar extensions
    for extra in EXTRA_KEYWORDS.iter() {
        all_keywords.insert(extra.to_string());
    }

    let keywords: Vec<String> = all_keywords.into_iter().collect();

    let operators = parse_xtext_operators(&kerml_expr);
    let enums = parse_xtext_enums(&sysml_xtext);

    // Generate pest sections
    let keywords_pest = generate_pest_keywords_from_strings(&keywords);
    let operators_pest = generate_pest_operators(&operators);
    let enums_pest = generate_pest_enums(&enums);

    // Load manual fragment files
    let fragments_dir = PathBuf::from("src/grammar/fragments");
    let header = fs::read_to_string(fragments_dir.join("header.pest"))
        .expect("Failed to read header.pest");
    let terminals = fs::read_to_string(fragments_dir.join("terminals.pest"))
        .expect("Failed to read terminals.pest");
    let names =
        fs::read_to_string(fragments_dir.join("names.pest")).expect("Failed to read names.pest");
    let tokens = fs::read_to_string(fragments_dir.join("tokens.pest"))
        .expect("Failed to read tokens.pest");
    let structure = fs::read_to_string(fragments_dir.join("structure.pest"))
        .expect("Failed to read structure.pest");
    let definitions = fs::read_to_string(fragments_dir.join("definitions.pest"))
        .expect("Failed to read definitions.pest");
    let usages = fs::read_to_string(fragments_dir.join("usages.pest"))
        .expect("Failed to read usages.pest");
    let features = fs::read_to_string(fragments_dir.join("features.pest"))
        .expect("Failed to read features.pest");
    let annotations = fs::read_to_string(fragments_dir.join("annotations.pest"))
        .expect("Failed to read annotations.pest");
    let relationships = fs::read_to_string(fragments_dir.join("relationships.pest"))
        .expect("Failed to read relationships.pest");
    let actions = fs::read_to_string(fragments_dir.join("actions.pest"))
        .expect("Failed to read actions.pest");
    let expressions = fs::read_to_string(fragments_dir.join("expressions.pest"))
        .expect("Failed to read expressions.pest");

    // Concatenate all parts in the correct order
    let grammar = format!(
        "{header}\n\n\
         {terminals}\n\n\
         {names}\n\n\
         {keywords_pest}\n\n\
         {operators_pest}\n\n\
         {enums_pest}\n\n\
         {tokens}\n\n\
         {structure}\n\n\
         {definitions}\n\n\
         {usages}\n\n\
         {features}\n\n\
         {annotations}\n\n\
         {relationships}\n\n\
         {actions}\n\n\
         {expressions}",
        header = header.trim(),
        terminals = terminals.trim(),
        names = names.trim(),
        keywords_pest = keywords_pest.trim(),
        operators_pest = operators_pest.trim(),
        enums_pest = enums_pest.trim(),
        tokens = tokens.trim(),
        structure = structure.trim(),
        definitions = definitions.trim(),
        usages = usages.trim(),
        features = features.trim(),
        annotations = annotations.trim(),
        relationships = relationships.trim(),
        actions = actions.trim(),
        expressions = expressions.trim(),
    );

    // Write to source tree (pest_derive requires this)
    // The grammar file path is resolved relative to the source file by pest_derive
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let dest_path = Path::new(&manifest_dir)
        .join("src")
        .join("grammar")
        .join("sysml.pest");

    // Only write if content changed (to avoid triggering unnecessary rebuilds)
    let should_write = match fs::read_to_string(&dest_path) {
        Ok(existing) => existing != grammar,
        Err(_) => true,
    };

    if should_write {
        fs::write(&dest_path, &grammar).expect("Failed to write generated grammar");
        println!("cargo:warning=Generated grammar at: {}", dest_path.display());
    }

    // Validate keyword coverage against grammar rules
    let validation = validate_keyword_coverage(&keywords, &grammar);

    if !validation.missing_usage_rules.is_empty() {
        for rule in &validation.missing_usage_rules {
            println!("cargo:warning=Missing usage rule: {}", rule);
        }
    }

    if !validation.missing_definition_rules.is_empty() {
        for rule in &validation.missing_definition_rules {
            println!("cargo:warning=Missing definition rule: {}", rule);
        }
    }

    // Optionally fail the build if SYSML_STRICT_VALIDATION is set
    if env::var("SYSML_STRICT_VALIDATION").is_ok() && !validation.is_valid() {
        panic!(
            "Keyword-to-grammar validation failed!\n{}",
            validation.format_report()
        );
    }

    // Validate expression rule coverage
    let expr_validation = validate_expression_coverage(&grammar);

    if !expr_validation.missing_rules.is_empty() {
        for rule in &expr_validation.missing_rules {
            println!("cargo:warning=Missing expression rule: {}", rule);
        }
    }

    // Also fail for strict validation if expression rules are missing
    if env::var("SYSML_STRICT_VALIDATION").is_ok() && !expr_validation.is_valid() {
        panic!(
            "Expression coverage validation failed!\n{}",
            expr_validation.format_report()
        );
    }

    // Validate xtext rule coverage (compares xtext rule names to pest grammar rules)
    // This helps catch parser gaps where xtext rules don't have pest equivalents
    let mut xtext_rules = Vec::new();
    xtext_rules.extend(parse_xtext_rule_names(&sysml_xtext));
    xtext_rules.extend(parse_xtext_rule_names(&kerml_xtext));
    xtext_rules.extend(parse_xtext_rule_names(&kerml_expr));

    let xtext_coverage = validate_xtext_rule_coverage(&xtext_rules, &grammar);

    // Report missing rules as warnings (informational, not blocking)
    // Use SYSML_SHOW_MISSING_RULES=1 to see the full list
    if env::var("SYSML_SHOW_MISSING_RULES").is_ok() && !xtext_coverage.missing_rules.is_empty() {
        println!(
            "cargo:warning=Xtext rule coverage: {}/{} rules",
            xtext_coverage.covered_rules.len(),
            xtext_coverage.covered_rules.len() + xtext_coverage.missing_rules.len()
        );
        for rule in &xtext_coverage.missing_rules {
            println!("cargo:warning=Missing xtext rule: {}", rule);
        }
    }

    // =====================================================================
    // GRAMMAR-ELEMENT LINKAGE VALIDATION (Phase 1)
    // =====================================================================
    // Validate that every pest grammar rule producing an element has a
    // matching ElementKind variant, and vice versa.
    // =====================================================================

    // Parse xtext rules with full return type information
    let mut xtext_rules_full = Vec::new();
    xtext_rules_full.extend(parse_xtext_rules(&sysml_xtext));
    xtext_rules_full.extend(parse_xtext_rules(&kerml_xtext));
    xtext_rules_full.extend(parse_xtext_rules(&kerml_expr));

    // Load TTL vocabulary files to get ElementKind names
    let kerml_vocab_path = refs_dir.join("Kerml-Vocab.ttl");
    let sysml_vocab_path = refs_dir.join("SysML-vocab.ttl");

    let element_kind_names = if kerml_vocab_path.exists() && sysml_vocab_path.exists() {
        let kerml_vocab = fs::read_to_string(&kerml_vocab_path)
            .expect("Failed to read Kerml-Vocab.ttl");
        let sysml_vocab = fs::read_to_string(&sysml_vocab_path)
            .expect("Failed to read SysML-vocab.ttl");

        let kerml_types = parse_ttl_vocab(&kerml_vocab).unwrap_or_default();
        let sysml_types = parse_ttl_vocab(&sysml_vocab).unwrap_or_default();

        let mut names: Vec<String> = kerml_types
            .iter()
            .chain(sysml_types.iter())
            .map(|t| t.name.clone())
            .collect();
        names.sort();
        names.dedup();
        names
    } else {
        println!("cargo:warning=TTL vocab files not found, skipping grammar-element linkage validation");
        Vec::new()
    };

    if !element_kind_names.is_empty() {
        let linkage = validate_grammar_element_linkage(
            &xtext_rules_full,
            &grammar,
            &element_kind_names,
        );

        let (covered, total, percent) = linkage.coverage_stats();
        println!(
            "cargo:warning=Grammar-Element Linkage: {}/{} ElementKinds have grammar rules ({:.1}%)",
            covered, total, percent
        );

        if !linkage.missing_element_kinds.is_empty() {
            for rule in &linkage.missing_element_kinds {
                println!("cargo:warning=Pest rule without ElementKind: {}", rule);
            }
        }

        // Optionally fail build on strict validation
        if env::var("SYSML_STRICT_VALIDATION").is_ok() && !linkage.is_valid() {
            panic!(
                "Grammar-Element Linkage validation failed!\n{}",
                linkage.format_report()
            );
        }
    }

    // Also write to OUT_DIR for debugging/inspection
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let debug_path = Path::new(&out_dir).join("sysml.pest");
    fs::write(&debug_path, grammar).expect("Failed to write debug grammar copy");
}

/// Find the sysmlv2 references directory by searching upward from the crate directory.
fn find_references_dir() -> Option<PathBuf> {
    // First, check if SYSML_REFS_DIR or SYSMLV2_REFS_DIR environment variable is set
    if let Ok(refs_dir) = env::var("SYSML_REFS_DIR") {
        let path = PathBuf::from(refs_dir);
        if path.exists() {
            return Some(path);
        }
    }
    if let Ok(refs_dir) = env::var("SYSMLV2_REFS_DIR") {
        let path = PathBuf::from(refs_dir);
        if path.exists() {
            return Some(path);
        }
    }

    // Search upward from the crate directory
    let mut current = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);

    for _ in 0..5 {
        // Check for in-repo references folder
        let refs_path = current.join("references").join("sysmlv2");
        if refs_path.exists() && refs_path.is_dir() {
            return Some(refs_path);
        }

        // Check for sysmlv2-references at this level
        let refs_path = current.join("sysmlv2-references");
        if refs_path.exists() && refs_path.is_dir() {
            return Some(refs_path);
        }

        // Go up one level
        if !current.pop() {
            break;
        }
    }

    // Try sibling directory (common development layout)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);
    if let Some(parent) = manifest_dir.parent() {
        if let Some(grandparent) = parent.parent() {
            let candidate_paths = [
                grandparent.join("references").join("sysmlv2"),
                grandparent.join("sysmlv2-references"),
            ];
            for refs_path in candidate_paths {
                if refs_path.exists() && refs_path.is_dir() {
                    return Some(refs_path);
                }
            }
        }
    }

    None
}
