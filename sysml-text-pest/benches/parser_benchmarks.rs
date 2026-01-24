//! End-to-end parser benchmarks with phase breakdown.
//!
//! This benchmark suite measures the complete parsing pipeline:
//! 1. Raw pest parsing (grammar matching)
//! 2. AST conversion (building elements from parse tree)
//! 3. Index rebuilding (building lookup tables)
//! 4. Resolution (resolving qualified names to ElementIds)
//!
//! Run with:
//! ```bash
//! cargo bench -p sysml-text-pest
//! ```
//!
//! For HTML reports:
//! ```bash
//! cargo bench -p sysml-text-pest -- --plotting-backend plotters
//! ```

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use pest::Parser as PestParserTrait;
use sysml_core::ModelGraph;
use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::{PestParser, Rule, SysmlGrammar};

// =============================================================================
// Synthetic Model Generators
// =============================================================================

/// Generate a simple flat model with N part definitions.
fn generate_flat_model(num_parts: usize) -> String {
    let mut s = String::from("package BenchmarkModel {\n");
    for i in 0..num_parts {
        s.push_str(&format!("    part def Part{};\n", i));
    }
    s.push_str("}\n");
    s
}

/// Generate a nested model with packages containing part definitions and usages.
fn generate_nested_model(num_packages: usize, parts_per_package: usize) -> String {
    let mut s = String::new();
    for p in 0..num_packages {
        s.push_str(&format!("package Package{} {{\n", p));

        // Add part definitions
        for i in 0..parts_per_package {
            s.push_str(&format!("    part def Part{}_{};\n", p, i));
        }

        // Add part usages that reference the definitions
        for i in 0..parts_per_package {
            s.push_str(&format!(
                "    part instance{}_{} : Part{}_{};\n",
                p, i, p, i
            ));
        }

        s.push_str("}\n\n");
    }
    s
}

/// Generate a model with deep nesting (packages within packages).
fn generate_deep_model(depth: usize, elements_per_level: usize) -> String {
    fn generate_level(depth: usize, elements: usize, indent: usize) -> String {
        if depth == 0 {
            return String::new();
        }

        let indent_str = "    ".repeat(indent);
        let mut s = String::new();

        for i in 0..elements {
            s.push_str(&format!("{}package Level{}_{} {{\n", indent_str, depth, i));
            s.push_str(&format!("{}    part def Part{};\n", indent_str, i));
            s.push_str(&generate_level(depth - 1, elements, indent + 1));
            s.push_str(&format!("{}}}\n", indent_str));
        }

        s
    }

    generate_level(depth, elements_per_level, 0)
}

/// Generate a model with complex relationships (specializations, typings).
fn generate_relationship_model(num_types: usize) -> String {
    let mut s = String::from("package RelationshipModel {\n");

    // Base types
    s.push_str("    part def Base;\n");
    for i in 0..num_types {
        s.push_str(&format!("    part def Type{} :> Base;\n", i));
    }

    // Usages with typings
    for i in 0..num_types {
        s.push_str(&format!("    part instance{} : Type{};\n", i, i));
    }

    // Attributes with multiplicities
    for i in 0..num_types {
        s.push_str(&format!(
            "    attribute count{}[0..*];\n",
            i
        ));
    }

    s.push_str("}\n");
    s
}

/// Generate a realistic model similar to library files.
fn generate_realistic_model(num_definitions: usize) -> String {
    let mut s = String::from("standard library package RealisticModel {\n");

    // Import statements
    s.push_str("    import ScalarValues::*;\n");
    s.push_str("    import Base::Anything;\n\n");

    // Documentation
    s.push_str("    doc /* This is a realistic benchmark model. */\n\n");

    // Abstract base definitions
    for i in 0..3 {
        s.push_str(&format!("    abstract part def AbstractBase{};\n", i));
    }
    s.push_str("\n");

    // Concrete definitions with specialization
    for i in 0..num_definitions {
        let base = i % 3;
        s.push_str(&format!(
            "    part def ConcretePart{} :> AbstractBase{} {{\n",
            i, base
        ));
        s.push_str(&format!("        attribute id : String;\n"));
        s.push_str(&format!("        attribute value : Real;\n"));
        s.push_str(&format!("        part subpart[0..10];\n"));
        s.push_str("    }\n\n");
    }

    // Action definitions
    for i in 0..num_definitions / 2 {
        s.push_str(&format!("    action def Action{} {{\n", i));
        s.push_str(&format!("        in item input{};\n", i));
        s.push_str(&format!("        out item output{};\n", i));
        s.push_str("    }\n\n");
    }

    s.push_str("}\n");
    s
}

// =============================================================================
// Phase 1: Raw Pest Parsing (Grammar Only)
// =============================================================================

fn bench_pest_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("phase1_pest_parsing");

    let sizes = [10, 50, 100, 500];

    for &size in &sizes {
        let source = generate_flat_model(size);
        let bytes = source.len();

        group.throughput(Throughput::Bytes(bytes as u64));
        group.bench_with_input(
            BenchmarkId::new("flat_model", size),
            &source,
            |b, source| {
                b.iter(|| {
                    let _ = SysmlGrammar::parse(Rule::File, black_box(source));
                });
            },
        );
    }

    // Nested model
    let nested = generate_nested_model(10, 10); // 10 packages, 10 parts each
    group.throughput(Throughput::Bytes(nested.len() as u64));
    group.bench_with_input(BenchmarkId::new("nested_10x10", 100), &nested, |b, source| {
        b.iter(|| {
            let _ = SysmlGrammar::parse(Rule::File, black_box(source));
        });
    });

    // Deep model
    let deep = generate_deep_model(5, 3); // 5 levels, 3 per level
    group.throughput(Throughput::Bytes(deep.len() as u64));
    group.bench_with_input(BenchmarkId::new("deep_5x3", 0), &deep, |b, source| {
        b.iter(|| {
            let _ = SysmlGrammar::parse(Rule::File, black_box(source));
        });
    });

    group.finish();
}

// =============================================================================
// Phase 2: AST Conversion (Building Elements)
// =============================================================================

fn bench_ast_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("phase2_ast_conversion");

    let sizes = [10, 50, 100, 500];

    for &size in &sizes {
        let source = generate_flat_model(size);

        // Pre-parse to get pairs
        let pairs = SysmlGrammar::parse(Rule::File, &source).unwrap();
        let pairs_clone = pairs.clone();

        group.bench_with_input(
            BenchmarkId::new("flat_model", size),
            &pairs_clone,
            |b, _| {
                b.iter(|| {
                    let pairs = SysmlGrammar::parse(Rule::File, &source).unwrap();
                    let mut graph = ModelGraph::new();
                    let converter =
                        sysml_text_pest::ast::Converter::new("bench.sysml", true);
                    let _ = converter.convert(pairs, &mut graph);
                    black_box(graph)
                });
            },
        );
    }

    // With vs without spans
    let source = generate_flat_model(100);
    group.bench_function("with_spans_100", |b| {
        b.iter(|| {
            let pairs = SysmlGrammar::parse(Rule::File, &source).unwrap();
            let mut graph = ModelGraph::new();
            let converter = sysml_text_pest::ast::Converter::new("bench.sysml", true);
            let _ = converter.convert(pairs, &mut graph);
            black_box(graph)
        });
    });

    group.bench_function("without_spans_100", |b| {
        b.iter(|| {
            let pairs = SysmlGrammar::parse(Rule::File, &source).unwrap();
            let mut graph = ModelGraph::new();
            let converter = sysml_text_pest::ast::Converter::new("bench.sysml", false);
            let _ = converter.convert(pairs, &mut graph);
            black_box(graph)
        });
    });

    group.finish();
}

// =============================================================================
// Phase 3: Index Rebuilding
// =============================================================================

fn bench_index_rebuilding(c: &mut Criterion) {
    let mut group = c.benchmark_group("phase3_index_rebuild");

    let sizes = [10, 50, 100, 500];

    for &size in &sizes {
        let source = generate_nested_model(size / 10, 10);
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &source)];
        let result = parser.parse(&files);

        // Clone graph for benchmarking rebuild
        group.bench_with_input(
            BenchmarkId::new("nested_model", size),
            &result.graph,
            |b, graph| {
                b.iter(|| {
                    let mut g = graph.clone();
                    g.rebuild_indexes();
                    black_box(g)
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// Phase 4: Resolution
// =============================================================================

fn bench_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("phase4_resolution");

    let sizes = [10, 50, 100];

    for &size in &sizes {
        let source = generate_relationship_model(size);
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &source)];

        group.bench_with_input(
            BenchmarkId::new("relationship_model", size),
            &files,
            |b, files| {
                b.iter(|| {
                    let mut result = parser.parse(files);
                    let res = result.resolve();
                    black_box(res)
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// End-to-End Benchmarks
// =============================================================================

fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");

    // Small model
    let small = generate_flat_model(10);
    group.throughput(Throughput::Bytes(small.len() as u64));
    group.bench_function("small_10_elements", |b| {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &small)];
        b.iter(|| {
            let result = parser.parse(black_box(&files));
            black_box(result)
        });
    });

    // Medium model
    let medium = generate_nested_model(10, 20);
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_function("medium_200_elements", |b| {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &medium)];
        b.iter(|| {
            let result = parser.parse(black_box(&files));
            black_box(result)
        });
    });

    // Large model
    let large = generate_nested_model(50, 20);
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_function("large_1000_elements", |b| {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &large)];
        b.iter(|| {
            let result = parser.parse(black_box(&files));
            black_box(result)
        });
    });

    // Realistic model
    let realistic = generate_realistic_model(50);
    group.throughput(Throughput::Bytes(realistic.len() as u64));
    group.bench_function("realistic_50_definitions", |b| {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &realistic)];
        b.iter(|| {
            let result = parser.parse(black_box(&files));
            black_box(result)
        });
    });

    // End-to-end with resolution
    let with_refs = generate_relationship_model(50);
    group.throughput(Throughput::Bytes(with_refs.len() as u64));
    group.bench_function("with_resolution_50", |b| {
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &with_refs)];
        b.iter(|| {
            let result = parser.parse(black_box(&files)).into_resolved();
            black_box(result)
        });
    });

    group.finish();
}

// =============================================================================
// Comparison: With vs Without Spans
// =============================================================================

fn bench_span_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("span_overhead");

    let sizes = [100, 500, 1000];

    for &size in &sizes {
        let source = generate_flat_model(size);
        let files = vec![SysmlFile::new("bench.sysml", &source)];

        group.bench_with_input(
            BenchmarkId::new("with_spans", size),
            &files,
            |b, files| {
                let parser = PestParser::new();
                b.iter(|| {
                    let result = parser.parse(black_box(files));
                    black_box(result)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("without_spans", size),
            &files,
            |b, files| {
                let parser = PestParser::without_spans();
                b.iter(|| {
                    let result = parser.parse(black_box(files));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// Multi-File Parsing
// =============================================================================

fn bench_multi_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_file");

    // Generate multiple files
    let files_1: Vec<SysmlFile> = (0..1)
        .map(|i| SysmlFile::new(format!("file{}.sysml", i), generate_flat_model(100)))
        .collect();
    let files_5: Vec<SysmlFile> = (0..5)
        .map(|i| SysmlFile::new(format!("file{}.sysml", i), generate_flat_model(100)))
        .collect();
    let files_10: Vec<SysmlFile> = (0..10)
        .map(|i| SysmlFile::new(format!("file{}.sysml", i), generate_flat_model(100)))
        .collect();

    let parser = PestParser::new();

    group.bench_function("1_file_100_parts", |b| {
        b.iter(|| {
            let result = parser.parse(black_box(&files_1));
            black_box(result)
        });
    });

    group.bench_function("5_files_100_parts_each", |b| {
        b.iter(|| {
            let result = parser.parse(black_box(&files_5));
            black_box(result)
        });
    });

    group.bench_function("10_files_100_parts_each", |b| {
        b.iter(|| {
            let result = parser.parse(black_box(&files_10));
            black_box(result)
        });
    });

    group.finish();
}

// =============================================================================
// Benchmark Groups
// =============================================================================

criterion_group!(
    phase_benchmarks,
    bench_pest_parsing,
    bench_ast_conversion,
    bench_index_rebuilding,
    bench_resolution,
);

criterion_group!(
    e2e_benchmarks,
    bench_end_to_end,
    bench_span_overhead,
    bench_multi_file,
);

criterion_main!(phase_benchmarks, e2e_benchmarks);
