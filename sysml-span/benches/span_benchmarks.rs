//! Benchmarks for sysml-span crate
//!
//! Tests optimizations:
//! - #5: String-based file storage (Span creation/cloning)
//! - #10: Synthetic span allocation
//! - #11: Multiple diagnostic iterations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sysml_span::{Diagnostic, Diagnostics, Span};

/// Benchmark: Span creation with different filename lengths
/// Tests optimization #5: String-based file storage
fn bench_span_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("span/creation");

    for filename_len in [10, 50, 200] {
        let filename: String = "a".repeat(filename_len) + ".sysml";

        group.bench_with_input(
            BenchmarkId::new("filename_len", filename_len),
            &filename,
            |b, filename| {
                b.iter(|| Span::new(black_box(filename.as_str()), 0, 100));
            },
        );
    }

    group.finish();
}

/// Benchmark: Span cloning (high frequency operation)
/// Tests optimization #5: String cloning overhead
fn bench_span_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("span/clone");

    let span_short = Span::new("short.sysml", 0, 100);
    let span_long = Span::new(&"a".repeat(200), 0, 100);

    group.bench_function("short_filename", |b| {
        b.iter(|| black_box(span_short.clone()));
    });

    group.bench_function("long_filename", |b| {
        b.iter(|| black_box(span_long.clone()));
    });

    group.finish();
}

/// Benchmark: Span merge operation
/// Tests optimization: Excessive cloning in merge
fn bench_span_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("span/merge");

    let span1 = Span::new("test.sysml", 0, 50);
    let span2 = Span::new("test.sysml", 40, 100);

    group.bench_function("same_file", |b| {
        b.iter(|| black_box(span1.merge(&span2)));
    });

    group.finish();
}

/// Benchmark: Synthetic span creation
/// Tests optimization #10: Repeated string allocation
fn bench_synthetic_span(c: &mut Criterion) {
    c.bench_function("span/synthetic", |b| {
        b.iter(|| black_box(Span::synthetic()));
    });
}

/// Benchmark: Span with location creation
fn bench_span_with_location(c: &mut Criterion) {
    c.bench_function("span/with_location", |b| {
        b.iter(|| {
            black_box(Span::with_location(
                "test/path/to/file.sysml",
                100,
                200,
                10,
                5,
            ))
        });
    });
}

/// Benchmark: Diagnostic creation and building
/// Tests optimization: Builder pattern copies
fn bench_diagnostic_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnostic/building");

    let span = Span::new("test.sysml", 0, 100);

    group.bench_function("simple_error", |b| {
        b.iter(|| Diagnostic::error("Test error message").with_span(span.clone()));
    });

    group.bench_function("full_diagnostic", |b| {
        b.iter(|| {
            Diagnostic::error("Test error message")
                .with_span(span.clone())
                .with_code("E001")
                .with_note("This is a note")
                .with_note("This is another note")
                .with_note("Try doing something else")
        });
    });

    group.bench_function("with_related", |b| {
        let related_span = Span::new("other.sysml", 50, 100);
        b.iter(|| {
            Diagnostic::error("Test error message")
                .with_span(span.clone())
                .with_related(related_span.clone(), "defined here")
        });
    });

    group.finish();
}

/// Benchmark: Diagnostics collection operations
/// Tests optimization #11: Multiple iterations
fn bench_diagnostics_collection(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnostics/collection");

    for (total, errors) in [(100, 10), (1000, 100), (10000, 1000)] {
        let mut diagnostics = Diagnostics::new();
        for i in 0..total {
            if i < errors {
                diagnostics.push(Diagnostic::error(format!("Error {}", i)));
            } else {
                diagnostics.push(Diagnostic::warning(format!("Warning {}", i)));
            }
        }

        group.bench_with_input(
            BenchmarkId::new("has_errors", total),
            &diagnostics,
            |b, diags| {
                b.iter(|| black_box(diags.has_errors()));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("error_count", total),
            &diagnostics,
            |b, diags| {
                b.iter(|| black_box(diags.error_count()));
            },
        );

        // Combined operation (what users typically do)
        group.bench_with_input(
            BenchmarkId::new("has_then_count", total),
            &diagnostics,
            |b, diags| {
                b.iter(|| {
                    if diags.has_errors() {
                        black_box(diags.error_count())
                    } else {
                        0
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Diagnostics push operations
fn bench_diagnostics_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnostics/push");

    group.bench_function("push_error", |b| {
        b.iter_batched(
            Diagnostics::new,
            |mut diags| {
                for i in 0..100 {
                    diags.push(Diagnostic::error(format!("Error {}", i)));
                }
                black_box(diags)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("push_with_span", |b| {
        let span = Span::new("test.sysml", 0, 100);
        b.iter_batched(
            Diagnostics::new,
            |mut diags| {
                for i in 0..100 {
                    diags.push(
                        Diagnostic::error(format!("Error {}", i)).with_span(span.clone()),
                    );
                }
                black_box(diags)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark: Display/formatting performance
fn bench_diagnostic_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnostic/display");

    let span = Span::with_location("test.sysml", 100, 150, 5, 10);
    let simple = Diagnostic::error("Simple error").with_span(span.clone());
    let complex = Diagnostic::error("Complex error")
        .with_span(span.clone())
        .with_code("E001")
        .with_note("Note 1")
        .with_note("Note 2")
        .with_note("Help text");

    group.bench_function("format_simple", |b| {
        b.iter(|| format!("{}", black_box(&simple)));
    });

    group.bench_function("format_complex", |b| {
        b.iter(|| format!("{}", black_box(&complex)));
    });

    group.finish();
}

/// Benchmark: Span comparison operations
fn bench_span_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("span/comparison");

    let span1 = Span::new("file.sysml", 0, 100);
    let span2 = Span::new("file.sysml", 0, 100);
    let span3 = Span::new("other.sysml", 0, 100);

    group.bench_function("eq_same", |b| {
        b.iter(|| black_box(&span1) == black_box(&span2));
    });

    group.bench_function("eq_different", |b| {
        b.iter(|| black_box(&span1) == black_box(&span3));
    });

    group.bench_function("contains", |b| {
        b.iter(|| black_box(span1.contains(50)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_span_creation,
    bench_span_clone,
    bench_span_merge,
    bench_synthetic_span,
    bench_span_with_location,
    bench_diagnostic_building,
    bench_diagnostics_collection,
    bench_diagnostics_push,
    bench_diagnostic_display,
    bench_span_comparison,
);

criterion_main!(benches);
