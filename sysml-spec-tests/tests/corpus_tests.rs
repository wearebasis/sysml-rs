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

use sysml_text::library::{load_standard_library, LibraryConfig, LibraryStats};
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

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

/// Minimum coverage threshold for element kinds.
/// The test fails if coverage drops below this percentage.
const ELEMENT_KIND_COVERAGE_THRESHOLD: f64 = 10.0;

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
    println!("  Expected: {} kinds (from constructible_kinds.txt)", expected.len());
    println!("  Produced: {}/{}", produced.len(), expected.len());

    if !produced.is_empty() {
        let mut produced_sorted: Vec<_> = produced.iter().collect();
        produced_sorted.sort();
        println!("\n  Covered kinds:");
        for kind in produced_sorted {
            println!("    + {}", kind);
        }
    }

    if !missing.is_empty() {
        let mut missing_sorted: Vec<_> = missing.iter().collect();
        missing_sorted.sort();
        println!("\n  Uncovered kinds:");
        for kind in missing_sorted {
            println!("    - {}", kind);
        }
    }

    let coverage = (produced.len() as f64 / expected.len() as f64) * 100.0;
    println!("\n  Coverage: {:.1}%", coverage);
    println!("  Threshold: {:.1}%", ELEMENT_KIND_COVERAGE_THRESHOLD);

    // Fail if coverage drops below threshold
    assert!(
        coverage >= ELEMENT_KIND_COVERAGE_THRESHOLD,
        "ElementKind coverage {:.1}% is below threshold {:.1}%.\n\
         Produced {} of {} expected kinds.",
        coverage,
        ELEMENT_KIND_COVERAGE_THRESHOLD,
        produced.len(),
        expected.len()
    );
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

/// Test that measures resolution quality across real corpus files.
///
/// This test parses and resolves all corpus files, then reports statistics
/// on how many cross-references were resolved vs unresolved.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests corpus_resolution -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_resolution() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let files = discover_corpus_files(&config);
    let allow_list = load_allow_list(EXPECTED_FAILURES);

    let mut total_resolved = 0usize;
    let mut total_unresolved = 0usize;
    let mut files_parsed = 0usize;
    let mut files_with_unresolved: Vec<(String, usize)> = Vec::new();

    let parser = PestParser::new();

    for file in &files {
        // Skip files in allow list (known parse failures)
        let in_allow_list = allow_list.contains(&file.relative_path)
            || allow_list.iter().any(|pattern| {
                if pattern.starts_with("**/") {
                    file.relative_path.ends_with(&pattern[3..])
                } else {
                    file.relative_path.contains(pattern)
                }
            });

        if in_allow_list {
            continue;
        }

        let sysml_file = SysmlFile::new(&file.relative_path, &file.content);
        let mut result = parser.parse(&[sysml_file]);

        if result.has_errors() {
            continue; // Skip files that don't parse
        }

        files_parsed += 1;

        // Run resolution and collect statistics
        let res = result.resolve();
        total_resolved += res.resolved_count;
        total_unresolved += res.unresolved_count;

        if res.unresolved_count > 0 {
            files_with_unresolved.push((
                file.relative_path.clone(),
                res.unresolved_count,
            ));
        }
    }

    // Generate report
    println!("\n=== Corpus Resolution Report ===");
    println!("Files parsed successfully: {}", files_parsed);
    println!("Total resolved references: {}", total_resolved);
    println!("Total unresolved references: {}", total_unresolved);

    let total_refs = total_resolved + total_unresolved;
    if total_refs > 0 {
        let rate = 100.0 * total_resolved as f64 / total_refs as f64;
        println!("Resolution rate: {:.1}%", rate);
    }

    if !files_with_unresolved.is_empty() {
        println!("\nFiles with unresolved references ({} files):", files_with_unresolved.len());
        // Sort by unresolved count descending
        let mut sorted = files_with_unresolved.clone();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (path, count) in sorted.iter().take(20) {
            println!("  - {} ({} unresolved)", path, count);
        }
        if sorted.len() > 20 {
            println!("  ... and {} more files", sorted.len() - 20);
        }
    } else {
        println!("\nAll references resolved!");
    }

    // For now, this test is informational - it doesn't fail
    // As resolution improves, we can add assertions like:
    // assert!(total_unresolved == 0, "Expected all references to resolve");
}

/// Test resolution with all model files parsed together + standard library.
///
/// This test parses ALL model files into a single graph, then resolves once.
/// This enables cross-file references to resolve correctly.
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

    // Load the standard library
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

    let files = discover_corpus_files(&config);
    let allow_list = load_allow_list(EXPECTED_FAILURES);

    // Collect ALL model files (not library files)
    let mut model_files: Vec<SysmlFile> = Vec::new();

    for file in &files {
        // Skip files in allow list
        let in_allow_list = allow_list.contains(&file.relative_path)
            || allow_list.iter().any(|pattern| {
                if pattern.starts_with("**/") {
                    file.relative_path.ends_with(&pattern[3..])
                } else {
                    file.relative_path.contains(pattern)
                }
            });

        if in_allow_list {
            continue;
        }

        // Skip library files
        if file.relative_path.contains("library.kernel")
            || file.relative_path.contains("library.systems")
        {
            continue;
        }

        model_files.push(SysmlFile::new(&file.relative_path, &file.content));
    }

    println!("\n=== Parsing {} model files together ===", model_files.len());

    // Parse ALL files together into one graph
    let mut result = parser.parse(&model_files);

    let parse_errors = result.error_count();
    let element_count = result.graph.element_count();
    println!("Parse errors: {}", parse_errors);
    println!("Elements parsed: {}", element_count);

    // Resolve WITH library
    let res = result.resolve_with_library(library);

    println!("\n=== Multi-File Resolution Results ===");
    println!("Total resolved references: {}", res.resolved_count);
    println!("Total unresolved references: {}", res.unresolved_count);

    let total_refs = res.resolved_count + res.unresolved_count;
    if total_refs > 0 {
        let rate = 100.0 * res.resolved_count as f64 / total_refs as f64;
        println!("Resolution rate: {:.1}%", rate);
    }

    // Report unresolved by type
    let error_count = res.diagnostics.error_count();
    println!("Resolution errors: {}", error_count);

    // Sample some unresolved references for debugging
    let unresolved_samples: Vec<_> = res.diagnostics.iter()
        .filter(|d| d.is_error())
        .take(20)
        .collect();

    if !unresolved_samples.is_empty() {
        println!("\nSample unresolved references ({} total):", error_count);
        for diag in unresolved_samples {
            println!("  - {}", diag);
        }
    }

    // Count by unresolved name
    use std::collections::HashMap;
    let mut unresolved_names: HashMap<String, usize> = HashMap::new();
    for diag in res.diagnostics.iter().filter(|d| d.is_error()) {
        let msg = diag.to_string();
        // Extract the unresolved name from "Unresolved reference 'X'"
        if let Some(start) = msg.find('\'') {
            if let Some(end) = msg[start+1..].find('\'') {
                let name = &msg[start+1..start+1+end];
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

/// Test resolution with standard library pre-loaded.
///
/// This test should show significantly higher resolution rates than
/// `corpus_resolution` because standard library types will resolve.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests corpus_resolution_with_library -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_resolution_with_library() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    // Load the standard library
    let lib_config = LibraryConfig::from_corpus_path(&config.corpus_path);
    let library = match load_standard_library(&parser, &lib_config) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load standard library: {}", e);
            eprintln!("Library path: {:?}", lib_config.library_path);
            panic!("Standard library loading failed");
        }
    };

    // Report library statistics
    let lib_stats = LibraryStats::from_graph(&library);
    println!("\n=== Standard Library Loaded ===");
    println!("Elements: {}", lib_stats.element_count);
    println!("Library packages: {}", lib_stats.package_count);
    println!("Package names: {:?}", lib_stats.package_names);

    let files = discover_corpus_files(&config);
    let allow_list = load_allow_list(EXPECTED_FAILURES);

    let mut total_resolved = 0usize;
    let mut total_unresolved = 0usize;
    let mut files_parsed = 0usize;
    let mut files_with_unresolved: Vec<(String, usize)> = Vec::new();

    for file in &files {
        // Skip files in allow list (known parse failures)
        let in_allow_list = allow_list.contains(&file.relative_path)
            || allow_list.iter().any(|pattern| {
                if pattern.starts_with("**/") {
                    file.relative_path.ends_with(&pattern[3..])
                } else {
                    file.relative_path.contains(pattern)
                }
            });

        if in_allow_list {
            continue;
        }

        // Skip library files themselves (they're already in the library graph)
        if file.relative_path.contains("library.kernel")
            || file.relative_path.contains("library.systems")
        {
            continue;
        }

        let sysml_file = SysmlFile::new(&file.relative_path, &file.content);
        let mut result = parser.parse(&[sysml_file]);

        if result.has_errors() {
            continue; // Skip files that don't parse
        }

        files_parsed += 1;

        // Resolve WITH library - clone library for each file
        let res = result.resolve_with_library(library.clone());
        total_resolved += res.resolved_count;
        total_unresolved += res.unresolved_count;

        if res.unresolved_count > 0 {
            files_with_unresolved.push((
                file.relative_path.clone(),
                res.unresolved_count,
            ));
        }
    }

    // Generate report
    println!("\n=== Corpus Resolution WITH Library ===");
    println!("Files parsed successfully: {}", files_parsed);
    println!("Total resolved references: {}", total_resolved);
    println!("Total unresolved references: {}", total_unresolved);

    let total_refs = total_resolved + total_unresolved;
    if total_refs > 0 {
        let rate = 100.0 * total_resolved as f64 / total_refs as f64;
        println!("Resolution rate: {:.1}%", rate);
    }

    if !files_with_unresolved.is_empty() {
        println!("\nFiles with unresolved references ({} files):", files_with_unresolved.len());
        // Sort by unresolved count descending
        let mut sorted = files_with_unresolved.clone();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (path, count) in sorted.iter().take(20) {
            println!("  - {} ({} unresolved)", path, count);
        }
        if sorted.len() > 20 {
            println!("  ... and {} more files", sorted.len() - 20);
        }
    } else {
        println!("\nAll references resolved!");
    }
}
