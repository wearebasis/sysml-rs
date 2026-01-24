use sysml_text_pest::PestParser;
use sysml_text::{SysmlFile, Parser};

#[test]
fn test_parse_family() {
    let content = std::fs::read_to_string("/home/ricky/personal_repos/sysml-rs/sysmlv2-references/SysML-v2-Models/models/example_family/family.sysml")
        .expect("Failed to read file");
    
    let parser = PestParser::new();
    let sysml_file = SysmlFile::new("family.sysml", &content);
    let result = parser.parse(&[sysml_file]);
    
    if result.is_ok() {
        println!("Parse succeeded!");
    } else {
        println!("Parse failed!");
    }
    
    for diag in result.diagnostics.iter().filter(|d| d.is_error()) {
        println!("{}", diag);
    }
}
