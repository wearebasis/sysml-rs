use sysml_span::{DiagnosticRenderer, HashMapSourceProvider};
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn main() {
    let demo_source = r#"
package Demo {
  package NotAType;
  part def Vehicle;

  // Relationship target mismatch: specialization expects a Type, got Package.
  part def Car :> NotAType;

  // Relationship target mismatch: FeatureTyping expects a Type, got Package.
  part car : NotAType;

  // Unresolved reference (resolution error).
  part ghost : MissingType;

  // Unresolved attribute type.
  attribute mass : Mass;
}
"#;

    let syntax_error_source = r#"
package SyntaxErrors {
  part def Broken {
    part x : ;
  }
}
"#;

    let files = vec![
        SysmlFile::new("demo.sysml", demo_source),
        SysmlFile::new("syntax_error.sysml", syntax_error_source),
    ];
    let parser = PestParser::new();

    // Full pipeline: parse -> resolve -> validate.
    let result = parser.parse(&files).into_resolved().into_validated();

    if result.diagnostics.is_empty() {
        println!("No diagnostics.");
        return;
    }

    let mut provider = HashMapSourceProvider::new();
    for file in &files {
        provider.insert(file.path.clone(), file.text.clone());
    }

    let renderer = DiagnosticRenderer::new();
    let output = renderer.render_all(result.diagnostics.iter(), &provider);
    println!("{output}");
}
