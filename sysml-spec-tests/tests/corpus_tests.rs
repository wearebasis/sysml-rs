//! Corpus coverage and resolution tests.
//!
//! These tests parse .sysml files in the reference corpus and verify
//! coverage and resolution behavior. Tests are `#[ignore]` by default
//! and enabled via:
//! ```bash
//! SYSML_CORPUS_PATH=/path/to/sysmlv2-references cargo test -p sysml-spec-tests -- --ignored
//! ```

use std::collections::HashSet;

use sysml_text::library::{load_standard_library, LibraryConfig, LibraryStats};
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

use sysml_spec_tests::{
    corpus::{collect_element_kinds, discover_corpus_files, parse_all_corpus_files},
    element_coverage::constructible_kinds,
    load_allow_list,
    report::{format_failures, generate_report},
    CoverageConfig,
};

/// Allow-list of files expected to fail parsing.
const EXPECTED_FAILURES: &str = include_str!("../data/expected_failures.txt");

const SMOKE_TEST_TARGET_FILES: usize = 5;
const SMOKE_TEST_MIN_FILES: usize = 3;

fn is_allow_listed(path: &str, allow_list: &HashSet<String>) -> bool {
    allow_list.contains(path)
        || allow_list.iter().any(|pattern| {
            if pattern.starts_with("**/") {
                path.ends_with(&pattern[3..])
            } else {
                path.contains(pattern)
            }
        })
}

fn is_library_file(path: &str) -> bool {
    path.contains("library.kernel")
        || path.contains("library.systems")
        || path.contains("library.domain")
}

#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_coverage() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let allow_list = load_allow_list(EXPECTED_FAILURES);
    let (results, summary) = parse_all_corpus_files(&config, &allow_list);

    assert!(summary.total_files > 0, "No .sysml files found in corpus");

    let unexpected_failures: Vec<_> = results
        .iter()
        .filter(|r| !r.success)
        .filter(|r| !is_allow_listed(&r.path, &allow_list))
        .map(|r| (r.path.clone(), r.errors.clone()))
        .collect();

    let element_kinds_produced = collect_element_kinds(&config);
    let report = generate_report(
        &results,
        &summary,
        &element_kinds_produced,
        &constructible_kinds(),
        None,
        None,
    );

    println!("{}", report);

    assert!(
        unexpected_failures.is_empty(),
        "Unexpected parse failures (not in allow-list):\n{}",
        format_failures(&unexpected_failures)
    );
}

/// Quick health check using a small subset of model files.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests corpus_smoke_test -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_smoke_test() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    let lib_config = LibraryConfig::from_corpus_path(&config.corpus_path);
    let library = match load_standard_library(&parser, &lib_config) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load standard library: {}", e);
            panic!("Standard library loading failed");
        }
    };

    let lib_stats = LibraryStats::from_graph(&library);
    println!(
        "Library loaded: {} elements, {} packages",
        lib_stats.element_count, lib_stats.package_count
    );

    let allow_list = load_allow_list(EXPECTED_FAILURES);
    let files = discover_corpus_files(&config);

    let model_files: Vec<_> = files
        .iter()
        .filter(|f| !is_library_file(&f.relative_path))
        .filter(|f| !is_allow_listed(&f.relative_path, &allow_list))
        .take(SMOKE_TEST_TARGET_FILES)
        .map(|f| SysmlFile::new(&f.relative_path, &f.content))
        .collect();

    assert!(
        model_files.len() >= SMOKE_TEST_MIN_FILES,
        "Expected at least {} eligible corpus files, found {}",
        SMOKE_TEST_MIN_FILES,
        model_files.len()
    );

    println!("Smoke test: {} files", model_files.len());

    let mut result = parser.parse(&model_files);
    let parse_errors = result.error_count();
    assert_eq!(
        parse_errors, 0,
        "Smoke test parse errors: {}",
        parse_errors
    );

    let res = result.resolve_with_library(library);
    let total_refs = res.resolved_count + res.unresolved_count;
    if total_refs > 0 {
        let rate = 100.0 * res.resolved_count as f64 / total_refs as f64;
        println!("Resolved: {} / {} ({:.1}%)", res.resolved_count, total_refs, rate);
    } else {
        println!("Resolved: 0 / 0 (no references)");
    }

    let error_count = res.diagnostics.error_count();
    println!("Resolution errors: {}", error_count);

    if error_count > 0 {
        println!("Sample unresolved references:");
        for diag in res.diagnostics.iter().filter(|d| d.is_error()).take(10) {
            println!("  - {}", diag);
        }
    }
}

/// Full resolution with all model files parsed together + standard library.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests corpus_resolution_multi_file -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_resolution_multi_file() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    let lib_config = LibraryConfig::from_corpus_path(&config.corpus_path);
    let library = match load_standard_library(&parser, &lib_config) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load standard library: {}", e);
            panic!("Standard library loading failed");
        }
    };

    let lib_stats = LibraryStats::from_graph(&library);
    println!("\n=== Standard Library Loaded ===");
    println!("Elements: {}", lib_stats.element_count);
    println!("Library packages: {}", lib_stats.package_count);

    let allow_list = load_allow_list(EXPECTED_FAILURES);
    let files = discover_corpus_files(&config);

    let mut model_files: Vec<SysmlFile> = Vec::new();
    for file in &files {
        if is_allow_listed(&file.relative_path, &allow_list) {
            continue;
        }
        if is_library_file(&file.relative_path) {
            continue;
        }
        model_files.push(SysmlFile::new(&file.relative_path, &file.content));
    }

    println!("\n=== Parsing {} model files together ===", model_files.len());

    let mut result = parser.parse(&model_files);

    let parse_errors = result.error_count();
    let element_count = result.graph.element_count();
    println!("Parse errors: {}", parse_errors);
    println!("Elements parsed: {}", element_count);

    let res = result.resolve_with_library(library);

    println!("\n=== Multi-File Resolution Results ===");
    println!("Total resolved references: {}", res.resolved_count);
    println!("Total unresolved references: {}", res.unresolved_count);

    let total_refs = res.resolved_count + res.unresolved_count;
    if total_refs > 0 {
        let rate = 100.0 * res.resolved_count as f64 / total_refs as f64;
        println!("Resolution rate: {:.1}%", rate);
    }

    let error_count = res.diagnostics.error_count();
    println!("Resolution errors: {}", error_count);

    let unresolved_samples: Vec<_> = res
        .diagnostics
        .iter()
        .filter(|d| d.is_error())
        .take(20)
        .collect();

    if !unresolved_samples.is_empty() {
        println!("\nSample unresolved references ({} total):", error_count);
        for diag in unresolved_samples {
            println!("  - {}", diag);
        }
    }

    use std::collections::HashMap;
    let mut unresolved_names: HashMap<String, usize> = HashMap::new();
    for diag in res.diagnostics.iter().filter(|d| d.is_error()) {
        let msg = diag.to_string();
        if let Some(start) = msg.find('\'') {
            if let Some(end) = msg[start + 1..].find('\'') {
                let name = &msg[start + 1..start + 1 + end];
                *unresolved_names.entry(name.to_string()).or_default() += 1;
            }
        }
    }

    println!("\nMost common unresolved names:");
    let mut sorted: Vec<_> = unresolved_names.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (name, count) in sorted.iter().take(15) {
        println!("  {} ({} times)", name, count);
    }
}
