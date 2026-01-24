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
    generate_pest_operators, parse_xtext_enums, parse_xtext_operators, validate_keyword_coverage,
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
    let refs_dir = find_references_dir().expect("Could not find sysmlv2-references directory");

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

    // Also write to OUT_DIR for debugging/inspection
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let debug_path = Path::new(&out_dir).join("sysml.pest");
    fs::write(&debug_path, grammar).expect("Failed to write debug grammar copy");
}

/// Find the sysmlv2-references directory by searching upward from the crate directory.
fn find_references_dir() -> Option<PathBuf> {
    // First, check if SYSMLV2_REFS_DIR environment variable is set
    if let Ok(refs_dir) = env::var("SYSMLV2_REFS_DIR") {
        let path = PathBuf::from(refs_dir);
        if path.exists() {
            return Some(path);
        }
    }

    // Search upward from the crate directory
    let mut current = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);

    for _ in 0..5 {
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
            let refs_path = grandparent.join("sysmlv2-references");
            if refs_path.exists() && refs_path.is_dir() {
                return Some(refs_path);
            }
        }
    }

    None
}
