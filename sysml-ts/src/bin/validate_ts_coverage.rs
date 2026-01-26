use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use sysml_codegen::{parse_ttl_vocab, parse_xtext_rules, XtextRule};

/// Paths to xtext specification files relative to the references directory.
const SYSML_XTEXT_PATH: &str =
    "SysML-v2-Pilot-Implementation/org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext";
const KERML_XTEXT_PATH: &str =
    "SysML-v2-Pilot-Implementation/org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext";
const KERML_EXPRESSIONS_PATH: &str = "SysML-v2-Pilot-Implementation/org.omg.kerml.expressions.xtext/src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext";

const GRAMMAR_PATH: &str = "tree-sitter/grammar.js";

fn main() -> Result<(), Box<dyn Error>> {
    let refs_dir = find_references_dir().ok_or("Could not find references/sysmlv2 directory")?;

    let sysml_xtext = fs::read_to_string(refs_dir.join(SYSML_XTEXT_PATH))?;
    let kerml_xtext = fs::read_to_string(refs_dir.join(KERML_XTEXT_PATH))?;
    let kerml_expr = fs::read_to_string(refs_dir.join(KERML_EXPRESSIONS_PATH))?;

    let mut xtext_rules_full = Vec::new();
    xtext_rules_full.extend(parse_xtext_rules(&sysml_xtext));
    xtext_rules_full.extend(parse_xtext_rules(&kerml_xtext));
    xtext_rules_full.extend(parse_xtext_rules(&kerml_expr));

    let xtext_element_rules = build_xtext_element_rules(&xtext_rules_full);

    let element_kind_names = load_element_kinds(&refs_dir)?;

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let grammar_path = manifest_dir.join(GRAMMAR_PATH);
    let grammar = fs::read_to_string(&grammar_path)?;

    let tree_rules = extract_tree_sitter_rule_names(&grammar);

    report_coverage(&tree_rules, &xtext_element_rules, &element_kind_names);

    Ok(())
}

fn report_coverage(
    tree_rules: &BTreeSet<String>,
    xtext_element_rules: &BTreeMap<String, String>,
    element_kind_names: &BTreeSet<String>,
) {
    let xtext_snake: BTreeMap<String, String> = xtext_element_rules
        .keys()
        .map(|name| (to_snake_case(name), name.to_string()))
        .collect();

    let element_snake: BTreeMap<String, String> = element_kind_names
        .iter()
        .map(|name| (to_snake_case(name), name.to_string()))
        .collect();

    let xtext_missing: Vec<String> = xtext_snake
        .iter()
        .filter(|(snake, _)| !tree_rules.contains(*snake))
        .map(|(_, original)| original.clone())
        .collect();

    let element_missing: Vec<String> = element_snake
        .iter()
        .filter(|(snake, _)| !tree_rules.contains(*snake))
        .map(|(_, original)| original.clone())
        .collect();

    let matched_xtext = xtext_snake.len().saturating_sub(xtext_missing.len());
    let matched_elements = element_snake.len().saturating_sub(element_missing.len());

    println!("Tree-sitter coverage report");
    println!("  grammar rules: {}", tree_rules.len());
    println!(
        "  xtext element rules: {} covered / {} total",
        matched_xtext,
        xtext_snake.len()
    );
    println!(
        "  element kinds: {} covered / {} total",
        matched_elements,
        element_snake.len()
    );

    let show_missing = env::var("SYSML_TS_SHOW_MISSING").is_ok();

    if show_missing {
        if !xtext_missing.is_empty() {
            println!("\nMissing tree-sitter rules for Xtext element rules:");
            for name in xtext_missing {
                println!("  - {}", name);
            }
        }
        if !element_missing.is_empty() {
            println!("\nMissing tree-sitter rules for ElementKinds:");
            for name in element_missing {
                println!("  - {}", name);
            }
        }
    } else {
        if !xtext_missing.is_empty() {
            println!(
                "  note: {} xtext element rules missing (set SYSML_TS_SHOW_MISSING=1 to list)",
                xtext_missing.len()
            );
        }
        if !element_missing.is_empty() {
            println!(
                "  note: {} element kinds missing (set SYSML_TS_SHOW_MISSING=1 to list)",
                element_missing.len()
            );
        }
    }
}

fn build_xtext_element_rules(xtext_rules: &[XtextRule]) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();

    for rule in xtext_rules {
        if rule.is_fragment || rule.is_terminal {
            continue;
        }

        if let Some(ref returns_type) = rule.returns_type {
            if returns_type.starts_with("SysML::") || returns_type.starts_with("KerML::") {
                mapping.insert(rule.name.clone(), returns_type.clone());
            }
        }
    }

    mapping
}

fn load_element_kinds(refs_dir: &Path) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let kerml_vocab_path = refs_dir.join("Kerml-Vocab.ttl");
    let sysml_vocab_path = refs_dir.join("SysML-vocab.ttl");

    let mut names = BTreeSet::new();

    if kerml_vocab_path.exists() {
        let content = fs::read_to_string(&kerml_vocab_path)?;
        for t in parse_ttl_vocab(&content).unwrap_or_default() {
            names.insert(t.name);
        }
    }

    if sysml_vocab_path.exists() {
        let content = fs::read_to_string(&sysml_vocab_path)?;
        for t in parse_ttl_vocab(&content).unwrap_or_default() {
            names.insert(t.name);
        }
    }

    Ok(names)
}

fn extract_tree_sitter_rule_names(grammar: &str) -> BTreeSet<String> {
    let mut rules = BTreeSet::new();

    for line in grammar.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("=>") {
            continue;
        }
        if let Some((name, _rest)) = trimmed.split_once(':') {
            let name = name.trim();
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                rules.insert(name.to_string());
            }
        }
    }

    rules
}

fn to_snake_case(name: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;

    for ch in name.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_lower = true;
        } else {
            out.push('_');
            prev_lower = false;
        }
    }

    while out.contains("__") {
        out = out.replace("__", "_");
    }

    out.trim_matches('_').to_string()
}

/// Find the sysmlv2 references directory by searching upward from the crate directory.
fn find_references_dir() -> Option<PathBuf> {
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

    let mut current = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);

    for _ in 0..5 {
        let refs_path = current.join("references").join("sysmlv2");
        if refs_path.exists() && refs_path.is_dir() {
            return Some(refs_path);
        }

        let refs_path = current.join("sysmlv2-references");
        if refs_path.exists() && refs_path.is_dir() {
            return Some(refs_path);
        }
        if !current.pop() {
            break;
        }
    }

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
