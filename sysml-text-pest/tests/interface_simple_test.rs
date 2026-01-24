//! Simple interface test

use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

#[test]
fn parse_interface_simple() {
    let parser = PestParser::new();
    let source = r#"
package Test {
    interface myInterface;
}
"#;
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Simple interface should parse");
}

#[test]
fn parse_interface_typed() {
    let parser = PestParser::new();
    let source = r#"
package Test {
    interface myInterface : SomeType;
}
"#;
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Typed interface should parse");
}

#[test]
fn parse_interface_with_connect() {
    let parser = PestParser::new();
    let source = r#"
package Test {
    interface myInterface : SomeType connect a to b;
}
"#;
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Interface with connect should parse");
}

#[test]
fn parse_interface_full() {
    let parser = PestParser::new();
    let source = r#"
package Test {
    interface myInterface : SomeType connect endA ::> a to endB ::> b;
}
"#;
    let files = vec![SysmlFile::new("test.sysml", source)];
    let result = parser.parse(&files);

    if result.has_errors() {
        for d in &result.diagnostics {
            eprintln!("Error: {}", d);
        }
    }

    assert!(result.is_ok(), "Full interface should parse");
}
