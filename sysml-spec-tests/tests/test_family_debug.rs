use sysml_text_pest::PestParser;
use sysml_text::{SysmlFile, Parser};

#[test]
fn test_parse_family_debug() {
    let content = std::fs::read_to_string("/home/ricky/personal_repos/sysml-rs/sysmlv2-references/SysML-v2-Models/models/example_family/family.sysml")
        .expect("Failed to read file");
    
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
