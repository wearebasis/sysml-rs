use sysml_text_pest::PestParser;
use sysml_text::{SysmlFile, Parser};
use sysml_spec_tests::try_find_references_dir;

#[test]
fn test_parse_family_debug() {
    let Some(root) = try_find_references_dir() else {
        eprintln!("Skipping test: references directory not found");
        return;
    };
    let path = root.join("SysML-v2-Models/models/example_family/family.sysml");
    if !path.exists() {
        eprintln!("Skipping test: family.sysml not found at {:?}", path);
        return;
    }
    let content = std::fs::read_to_string(&path).expect("Failed to read file");
    
    // Extract lines around line 53-56
    let lines: Vec<_> = content.lines().collect();
    println!("\n=== Lines 50-75 (family.sysml) ===");
    for (i, line) in lines.iter().enumerate() {
        if i >= 49 && i < 75 {
            println!("{:4}: {}", i+1, line);
        }
    }
    
    let parser = PestParser::new();
    let sysml_file = SysmlFile::new("family.sysml", &content);
    let result = parser.parse(&[sysml_file]);
    
    println!("\n=== Parse Result ===");
    if result.is_ok() {
        println!("Parse succeeded!");
    } else {
        println!("Parse failed!");
    }
    
    println!("\n=== Errors ===");
    for diag in result.diagnostics.iter().filter(|d| d.is_error()) {
        println!("{}", diag);
    }
}
