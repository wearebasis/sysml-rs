use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

#[test]
fn test_drone_model() {
    let parser = PestParser::new();
    let source = std::fs::read_to_string(
        "/home/ricky/personal_repos/sysml-rs/sysmlv2-references/SysML-v2-Models/models/SE_Models/DroneModelLogical.sysml"
    ).unwrap();
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
