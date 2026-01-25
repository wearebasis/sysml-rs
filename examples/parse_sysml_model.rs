//! Example: Parse + resolve the SysmlRs model (text -> model graph -> diagnostics)
//!
//! Run with:
//!   cargo run --example parse_sysml_model

use std::fs;

use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn main() {
    let path = "model/sysml-rs.sysml";
    let content =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    let file = SysmlFile::new(path, &content);
    let parser = PestParser::new();

    let mut result = parser.parse(&[file]);

    println!("=== Parse Diagnostics ===");
    println!("Errors: {}", result.error_count());
    for diag in result.diagnostics.iter().take(20) {
        println!("  - {}", diag);
    }

    let resolution = result.resolve();

    println!("\n=== Resolution Summary ===");
    println!("Resolved: {}", resolution.resolved_count);
    println!("Unresolved: {}", resolution.unresolved_count);

    if resolution.diagnostics.error_count() > 0 {
        println!("\n=== Resolution Diagnostics (errors) ===");
        for diag in resolution
            .diagnostics
            .iter()
            .filter(|d| d.is_error())
            .take(20)
        {
            println!("  - {}", diag);
        }
    }
}
