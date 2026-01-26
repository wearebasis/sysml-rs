use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;
use std::path::PathBuf;

fn references_root() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SYSML_CORPUS_PATH") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    if let Ok(path) = std::env::var("SYSML_REFS_DIR") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    if let Ok(path) = std::env::var("SYSMLV2_REFS_DIR") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent()?;
    let in_repo = repo_root.join("references").join("sysmlv2");
    if in_repo.exists() {
        return Some(in_repo);
    }

    let legacy = repo_root.parent()?.join("sysmlv2-references");
    if legacy.exists() {
        return Some(legacy);
    }

    None
}

#[test]
fn test_drone_model() {
    let parser = PestParser::new();
    let Some(root) = references_root() else {
        eprintln!("Skipping test: references directory not found");
        return;
    };
    let path = root.join("SysML-v2-Models/models/SE_Models/DroneModelLogical.sysml");
    if !path.exists() {
        eprintln!("Skipping test: DroneModelLogical.sysml not found at {:?}", path);
        return;
    }
    let source = std::fs::read_to_string(&path).expect("Failed to read DroneModelLogical.sysml");
    let files = vec![SysmlFile::new("DroneModelLogical.sysml", &source)];
    let result = parser.parse(&files);
    
    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
        panic!("Parse failed");
    }
    println!("SUCCESS: Parsed {} lines", source.lines().count());
}
