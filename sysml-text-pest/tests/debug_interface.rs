//! Debug interface test

use pest::Parser;
use sysml_text::{Parser as SysmlParser, SysmlFile};
use sysml_text_pest::{PestParser, SysmlGrammar, Rule};

#[test]
fn debug_part_works() {
    let parser = PestParser::new();

    // Test part - this should work
    let source = "package P { part myPart; }";
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Part Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Simple part should parse");
}

#[test]
fn debug_interface_works() {
    let parser = PestParser::new();

    // Test interface - this should work after keyword boundary fix
    let source = "package P { interface myInterface; }";
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Interface Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Simple interface should parse");
}

#[test]
fn debug_interface_def_works() {
    let parser = PestParser::new();

    // Test interface def - this should work
    let source = "package P { interface def MyInterface; }";
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Interface def Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Interface def should parse");
}

#[test]
fn debug_interface_usage_rule_directly() {
    // Test the InterfaceUsage rule directly
    let input = "interface myInterface;";

    match SysmlGrammar::parse(Rule::InterfaceUsage, input) {
        Ok(pairs) => {
            eprintln!("InterfaceUsage parsed successfully!");
            for pair in pairs {
                eprintln!("  {:?}: {:?}", pair.as_rule(), pair.as_str());
            }
        }
        Err(e) => {
            eprintln!("InterfaceUsage parse error: {}", e);
        }
    }

    // Also test UsageDeclaration directly
    let input2 = "myInterface";
    match SysmlGrammar::parse(Rule::UsageDeclaration, input2) {
        Ok(pairs) => {
            eprintln!("UsageDeclaration parsed successfully!");
            for pair in pairs {
                eprintln!("  {:?}: {:?}", pair.as_rule(), pair.as_str());
            }
        }
        Err(e) => {
            eprintln!("UsageDeclaration parse error: {}", e);
        }
    }

    // And FeatureDeclaration
    match SysmlGrammar::parse(Rule::FeatureDeclaration, input2) {
        Ok(pairs) => {
            eprintln!("FeatureDeclaration parsed successfully!");
            for pair in pairs {
                eprintln!("  {:?}: {:?}", pair.as_rule(), pair.as_str());
            }
        }
        Err(e) => {
            eprintln!("FeatureDeclaration parse error: {}", e);
        }
    }

    // And Identification
    match SysmlGrammar::parse(Rule::Identification, input2) {
        Ok(pairs) => {
            eprintln!("Identification parsed successfully!");
            for pair in pairs {
                eprintln!("  {:?}: {:?}", pair.as_rule(), pair.as_str());
            }
        }
        Err(e) => {
            eprintln!("Identification parse error: {}", e);
        }
    }
}
