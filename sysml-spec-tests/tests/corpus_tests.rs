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
    use std::time::Instant;

    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let files = discover_corpus_files(&config);
    let allow_list = load_allow_list(EXPECTED_FAILURES);

    let mut total_resolved = 0usize;
    let mut total_unresolved = 0usize;
    let mut files_parsed = 0usize;
    let mut files_with_unresolved: Vec<(String, usize)> = Vec::new();

    // Timing accumulators
    let mut parse_time = std::time::Duration::ZERO;
    let mut resolve_time = std::time::Duration::ZERO;
    let overall_start = Instant::now();

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

        let t0 = Instant::now();
        let mut result = parser.parse(&[sysml_file]);
        parse_time += t0.elapsed();

        if result.has_errors() {
            continue; // Skip files that don't parse
        }

        files_parsed += 1;

        // Run resolution and collect statistics
        let t1 = Instant::now();
        let res = result.resolve();
        resolve_time += t1.elapsed();
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

    // Timing report
    let overall_elapsed = overall_start.elapsed();
    println!("\n=== Timing Breakdown ===");
    println!("Parse time:   {:?} ({:.1}%)", parse_time, 100.0 * parse_time.as_secs_f64() / overall_elapsed.as_secs_f64());
    println!("Resolve time: {:?} ({:.1}%)", resolve_time, 100.0 * resolve_time.as_secs_f64() / overall_elapsed.as_secs_f64());
    println!("Other:        {:?}", overall_elapsed - parse_time - resolve_time);
    println!("Total:        {:?}", overall_elapsed);

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

/// Quick resolution spot-check using a small representative subset of files.
///
/// This test is FAST (~5-10 seconds) and useful for iterative development.
/// Use this for regular checks; use full corpus tests for milestone validation.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests corpus_resolution_quick -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn corpus_resolution_quick() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    // Load the standard library ONCE
    let lib_config = LibraryConfig::from_corpus_path(&config.corpus_path);
    let library = match load_standard_library(&parser, &lib_config) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load standard library: {}", e);
            return;
        }
    };

    let lib_stats = LibraryStats::from_graph(&library);
    println!("Library: {} elements, {} packages", lib_stats.element_count, lib_stats.package_count);

    let files = discover_corpus_files(&config);
    let allow_list = load_allow_list(EXPECTED_FAILURES);

    // Filter to model files only (skip library, skip allow-listed)
    let model_files: Vec<_> = files
        .iter()
        .filter(|f| {
            !f.relative_path.contains("library.kernel")
                && !f.relative_path.contains("library.systems")
                && !f.relative_path.contains("library.domain")
                && !allow_list.contains(&f.relative_path)
                && !allow_list.iter().any(|p| {
                    if p.starts_with("**/") {
                        f.relative_path.ends_with(&p[3..])
                    } else {
                        f.relative_path.contains(p)
                    }
                })
        })
        .collect();

    // Select just 1 representative file for quick check
    // The full library resolution is slow (~30s), so minimize files
    let subset: Vec<_> = model_files.iter().take(1).collect();

    println!("Quick check: {} file (library has {} elements)", subset.len(), lib_stats.element_count);

    let mut total_resolved = 0usize;
    let mut total_unresolved = 0usize;
    let mut unresolved_names: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for file in subset {
        let sysml_file = SysmlFile::new(&file.relative_path, &file.content);
        let mut result = parser.parse(&[sysml_file]);

        if result.has_errors() {
            continue;
        }

        let res = result.resolve_with_library(library.clone());
        total_resolved += res.resolved_count;
        total_unresolved += res.unresolved_count;

        // Track unresolved names
        for diag in res.diagnostics.iter().filter(|d| d.is_error()) {
            let msg = diag.to_string();
            if let Some(start) = msg.find('\'') {
                if let Some(end) = msg[start+1..].find('\'') {
                    let name = &msg[start+1..start+1+end];
                    *unresolved_names.entry(name.to_string()).or_default() += 1;
                }
            }
        }
    }

    let total = total_resolved + total_unresolved;
    let rate = if total > 0 { 100.0 * total_resolved as f64 / total as f64 } else { 100.0 };

    println!("\n=== Quick Resolution Check ===");
    println!("Resolved: {} / {} ({:.1}%)", total_resolved, total, rate);
    println!("Unresolved: {}", total_unresolved);

    if !unresolved_names.is_empty() {
        println!("\nTop unresolved names:");
        let mut sorted: Vec<_> = unresolved_names.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (name, count) in sorted.iter().take(10) {
            println!("  {} ({} times)", name, count);
        }
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

/// Debug test to inspect library index contents.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests library_index_debug -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn library_index_debug() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    // Load the standard library
    let lib_config = LibraryConfig::from_corpus_path(&config.corpus_path);
    let mut library = match load_standard_library(&parser, &lib_config) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load standard library: {}", e);
            panic!("Standard library loading failed");
        }
    };

    // Ensure library index is built
    library.ensure_library_index();

    // Report library statistics
    let lib_stats = LibraryStats::from_graph(&library);
    println!("\n=== Standard Library Loaded ===");
    println!("Elements: {}", lib_stats.element_count);
    println!("Library packages: {}", lib_stats.package_count);
    println!("Package names: {:?}", lib_stats.package_names);

    // Check for specific library types that should be indexed
    let test_names = [
        // Types that fail to resolve in corpus_resolution_multi_file
        "CartesianVectorValue",
        "NumericalVectorValue",
        "VectorValue",
        "Expression",
        "Occurrence",
        "Collection",
        "Percentage",
        "Array",
        "Point",
        "Temperature",
        // Common library types that should definitely be indexed
        "Integer",
        "Real",
        "String",
        "Boolean",
        "Anything",
        "DataValue",
        // Package names (should be indexed)
        "VectorValues",
        "Base",
        "ScalarValues",
        "Collections",
    ];

    println!("\n=== Library Index Lookup ===");
    let mut found = 0;
    let mut missing = 0;
    for name in &test_names {
        match library.resolve_in_library(name) {
            Some(id) => {
                println!("  {} -> {} (FOUND)", name, id);
                found += 1;
            }
            None => {
                println!("  {} -> NOT FOUND", name);
                missing += 1;
            }
        }
    }
    println!("\nFound: {}, Missing: {}", found, missing);

    // Count elements by kind to understand what's in the library
    use std::collections::HashMap;
    let mut kinds: HashMap<String, usize> = HashMap::new();
    for elem in library.elements.values() {
        *kinds.entry(format!("{:?}", elem.kind)).or_default() += 1;
    }
    let mut sorted: Vec<_> = kinds.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("\n=== Library Elements by Kind (top 15) ===");
    for (kind, count) in sorted.iter().take(15) {
        println!("  {}: {}", kind, count);
    }

    // Count elements with names
    let named = library.elements.values().filter(|e| e.name.is_some()).count();
    println!("\nNamed elements: {}/{}", named, library.element_count());

    // List some named elements to see what's there
    println!("\n=== Sample Named Elements ===");
    let mut named_elements: Vec<_> = library
        .elements
        .values()
        .filter_map(|e| e.name.as_ref().map(|n| (n.clone(), format!("{:?}", e.kind))))
        .collect();
    named_elements.sort();
    for (name, kind) in named_elements.iter().take(50) {
        println!("  {}: {}", name, kind);
    }
}

/// Debug test to check specific library file parsing.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests vector_values_debug -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn vector_values_debug() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    // Parse VectorValues.kerml directly
    let vector_values_path = config.corpus_path
        .join("SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.kernel/VectorValues.kerml");

    let content = std::fs::read_to_string(&vector_values_path)
        .expect("Failed to read VectorValues.kerml");

    println!("=== Parsing VectorValues.kerml ===");
    println!("Content length: {} bytes", content.len());

    let file = SysmlFile::new("VectorValues.kerml", &content);
    let result = parser.parse(&[file]);

    println!("\nParse errors: {}", result.error_count());
    for diag in result.diagnostics.iter().take(10) {
        println!("  {}", diag);
    }

    println!("\nElements: {}", result.graph.element_count());

    // List all elements with names
    let mut named: Vec<_> = result.graph.elements.values()
        .filter_map(|e| {
            e.name.as_ref().map(|n| {
                let owner = e.owner.as_ref()
                    .and_then(|o| result.graph.elements.get(o))
                    .and_then(|o| o.name.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("(no owner)");
                (n.clone(), format!("{:?}", e.kind), owner.to_string())
            })
        })
        .collect();
    named.sort();

    println!("\n=== Named Elements ===");
    for (name, kind, owner) in &named {
        println!("  {} ({}) - owner: {}", name, kind, owner);
    }

    // Check for memberships
    let memberships = result.graph.elements.values()
        .filter(|e| e.kind == sysml_core::ElementKind::OwningMembership)
        .count();
    println!("\nOwningMemberships: {}", memberships);

    // Check root packages
    let roots: Vec<_> = result.graph.elements.values()
        .filter(|e| e.owner.is_none())
        .filter_map(|e| e.name.as_ref().map(|n| (n.clone(), format!("{:?}", e.kind))))
        .collect();
    println!("\nRoot elements: {:?}", roots);
}

/// Debug test to check Occurrences.kerml parsing.
///
/// Run with:
/// ```bash
/// SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
///     cargo test -p sysml-spec-tests occurrences_debug -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Requires SYSML_CORPUS_PATH env var - run with --ignored"]
fn occurrences_debug() {
    let config = CoverageConfig::from_env()
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let parser = PestParser::new();

    let file_path = config.corpus_path
        .join("SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests/library.kernel/Occurrences.kerml");

    let content = std::fs::read_to_string(&file_path)
        .expect("Failed to read Occurrences.kerml");

    println!("=== Parsing Occurrences.kerml ===");
    println!("Content length: {} bytes", content.len());

    let file = SysmlFile::new("Occurrences.kerml", &content);
    let result = parser.parse(&[file]);

    println!("\nParse errors: {}", result.error_count());
    for diag in result.diagnostics.iter().take(15) {
        println!("  {}", diag);
    }

    println!("\nElements: {}", result.graph.element_count());

    // Show first error context
    if result.error_count() > 0 {
        println!("\n=== Error Context ===");
        if let Some(diag) = result.diagnostics.iter().find(|d| d.is_error()) {
            // Try to show the line from the content
            if let Some(span) = &diag.span {
                if let Some(line_num) = span.line {
                    let lines: Vec<&str> = content.lines().collect();
                    let ln = line_num as usize;
                    if ln > 0 && ln <= lines.len() {
                        if ln > 1 {
                            println!("Line {}: {}", ln - 1, lines[ln - 2]);
                        }
                        println!("Line {}: {}", ln, lines[ln - 1]);
                        if ln < lines.len() {
                            println!("Line {}: {}", ln + 1, lines[ln]);
                        }
                    }
                }
            }
        }
    }
}
