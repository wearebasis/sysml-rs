//! Corpus coverage tests.
//!
//! These tests parse all .sysml files in the reference corpus and verify
//! that the parser handles them correctly.
//!
//! Tests are `#[ignore]` by default and enabled via:
//! ```bash
//! SYSML_CORPUS_PATH=/path/to/sysmlv2-references cargo test -p sysml-spec-tests -- --ignored
//! ```

use std::collections::HashSet;

use sysml_spec_tests::{
    corpus::{discover_corpus_files, parse_all_corpus_files},
    element_coverage::{constructible_kinds, ElementCoverageTracker},
    load_allow_list,
    report::{format_failures, generate_report},
    CoverageConfig,
};

/// The allow-list of files expected to fail.
/// This list MUST shrink over time as parser bugs are fixed.
const EXPECTED_FAILURES: &str = include_str!("../data/expected_failures.txt");

#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_coverage() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let allow_list = load_allow_list(EXPECTED_FAILURES);
    let (results, summary) = parse_all_corpus_files(&config, &allow_list);

    // Collect unexpected failures
    let unexpected_failures: Vec<_> = results
        .iter()
        .filter(|r| !r.success)
        .filter(|r| {
            !allow_list.contains(&r.path)
                && !allow_list.iter().any(|pattern| {
                    if pattern.starts_with("**/") {
                        r.path.ends_with(&pattern[3..])
                    } else {
                        r.path.contains(pattern)
                    }
                })
        })
        .map(|r| (r.path.clone(), r.errors.clone()))
        .collect();

    // Generate report for display
    let element_kinds_produced: HashSet<_> = results
        .iter()
        .filter(|r| r.success)
        .flat_map(|_| {
            // We'd need to re-parse to get actual kinds, but for now just track success
            Vec::<String>::new()
        })
        .collect();

    let report = generate_report(
        &results,
        &summary,
        &element_kinds_produced,
        &constructible_kinds(),
        None,
        None,
    );

    println!("{}", report);

    // Fail on ANY unexpected failure
    assert!(
        unexpected_failures.is_empty(),
        "Unexpected parse failures (not in allow-list):\n{}",
        format_failures(&unexpected_failures)
    );
}

#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_file_discovery() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let files = discover_corpus_files(&config);

    println!("Discovered {} .sysml files:", files.len());
    for file in &files {
        println!("  - {}", file.relative_path);
    }

    // Expect at least some files
    assert!(!files.is_empty(), "No .sysml files found in corpus");
}

#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn element_kind_coverage() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let files = discover_corpus_files(&config);
    let mut tracker = ElementCoverageTracker::new();

    for file in &files {
        tracker.track_parse(&file.content);
    }

    let expected = constructible_kinds();
    let produced = tracker.produced_kinds();
    let missing = tracker.missing_kinds(&expected);

    println!("Element Kind Coverage:");
    println!("  Produced: {}/{}", produced.len(), expected.len());

    if !missing.is_empty() {
        let mut missing_sorted: Vec<_> = missing.iter().collect();
        missing_sorted.sort();
        println!("\n  Uncovered kinds:");
        for kind in missing_sorted {
            println!("    - {}", kind);
        }
    }

    // Report but don't fail on missing kinds (parser may not support all yet)
    let coverage = (produced.len() as f64 / expected.len() as f64) * 100.0;
    println!("\n  Coverage: {:.1}%", coverage);
}

#[test]
fn parse_simple_package() {
    // Quick sanity test that doesn't require corpus
    let mut tracker = ElementCoverageTracker::new();
    tracker.track_parse("package Test { }");
    assert!(tracker.was_produced("Package"));
}

#[test]
fn parse_part_definition() {
    let mut tracker = ElementCoverageTracker::new();
    tracker.track_parse("package Test { part def Vehicle; }");
    assert!(tracker.was_produced("Package"));
    assert!(tracker.was_produced("PartDefinition"));
}

#[test]
fn parse_part_usage() {
    let mut tracker = ElementCoverageTracker::new();
    tracker.track_parse("package Test { part def Vehicle; part car : Vehicle; }");
    assert!(tracker.was_produced("Package"));
    assert!(tracker.was_produced("PartDefinition"));
    assert!(tracker.was_produced("PartUsage"));
}

#[test]
fn allow_list_format() {
    // Verify the allow list can be loaded
    let list = load_allow_list(EXPECTED_FAILURES);
    println!("Allow list contains {} entries", list.len());

    // Print entries for debugging
    for entry in &list {
        println!("  - {}", entry);
    }
}
