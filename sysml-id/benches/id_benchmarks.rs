//! Benchmarks for sysml-id crate.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sysml_id::{ElementId, QualifiedName};

/// Benchmark ElementId::new_v4() generation.
fn bench_element_id_generation(c: &mut Criterion) {
    c.bench_function("ElementId::new_v4", |b| {
        b.iter(|| ElementId::new_v4())
    });
}

/// Benchmark ElementId::from_string() with various inputs.
fn bench_element_id_from_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("ElementId::from_string");

    // Short string
    group.bench_function("short", |b| {
        b.iter(|| ElementId::from_string(black_box("test")))
    });

    // Longer string
    group.bench_function("medium", |b| {
        b.iter(|| ElementId::from_string(black_box("my-unique-element-identifier")))
    });

    // Valid UUID string (fast path)
    group.bench_function("valid_uuid", |b| {
        b.iter(|| ElementId::from_string(black_box("550e8400-e29b-41d4-a716-446655440000")))
    });

    group.finish();
}

/// Benchmark QualifiedName parsing with various depths.
fn bench_qualified_name_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("QualifiedName::parse");

    // Various qualified name lengths
    let names = [
        ("depth_1", "Element"),
        ("depth_3", "Package::Part::Attribute"),
        ("depth_5", "Root::Level1::Level2::Level3::Leaf"),
        ("depth_10", "A::B::C::D::E::F::G::H::I::J"),
    ];

    for (name, input) in names {
        group.bench_with_input(BenchmarkId::new("simple", name), &input, |b, input| {
            b.iter(|| input.parse::<QualifiedName>())
        });
    }

    group.finish();
}

/// Benchmark QualifiedName parsing with Unicode characters.
fn bench_qualified_name_unicode(c: &mut Criterion) {
    let mut group = c.benchmark_group("QualifiedName::parse_unicode");

    let names = [
        ("ascii", "Package::Part::Attribute"),
        ("japanese", "パッケージ::部品::属性"),
        ("mixed", "Package::部品::Attribute"),
    ];

    for (name, input) in names {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| input.parse::<QualifiedName>())
        });
    }

    group.finish();
}

/// Benchmark QualifiedName escaped parsing.
fn bench_qualified_name_escaped(c: &mut Criterion) {
    let mut group = c.benchmark_group("QualifiedName::parse_escaped");

    let names = [
        ("no_escapes", "A::B::C"),
        ("with_colon", "A\\:B::C"),
        ("with_backslash", "A\\\\B::C"),
        ("complex", "A\\:B::C\\\\D::E"),
    ];

    for (name, input) in names {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| QualifiedName::parse_escaped(black_box(input)))
        });
    }

    group.finish();
}

/// Benchmark QualifiedName building via child() method.
fn bench_qualified_name_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("QualifiedName::build");

    group.bench_function("from_single", |b| {
        b.iter(|| QualifiedName::from_single(black_box("Root")))
    });

    group.bench_function("child_chain_3", |b| {
        b.iter(|| {
            QualifiedName::from_single("Root")
                .child("Level1")
                .child("Level2")
        })
    });

    group.bench_function("child_chain_5", |b| {
        b.iter(|| {
            QualifiedName::from_single("Root")
                .child("A")
                .child("B")
                .child("C")
                .child("D")
        })
    });

    group.finish();
}

/// Benchmark QualifiedName operations.
fn bench_qualified_name_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("QualifiedName::ops");

    let qn: QualifiedName = "A::B::C::D::E".parse().unwrap();

    group.bench_function("simple_name", |b| {
        b.iter(|| black_box(&qn).simple_name())
    });

    group.bench_function("parent", |b| {
        b.iter(|| black_box(&qn).parent())
    });

    let prefix: QualifiedName = "A::B".parse().unwrap();
    group.bench_function("starts_with", |b| {
        b.iter(|| black_box(&qn).starts_with(black_box(&prefix)))
    });

    group.bench_function("to_string", |b| {
        b.iter(|| black_box(&qn).to_string())
    });

    group.bench_function("to_escaped_string", |b| {
        b.iter(|| black_box(&qn).to_escaped_string())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_element_id_generation,
    bench_element_id_from_string,
    bench_qualified_name_parsing,
    bench_qualified_name_unicode,
    bench_qualified_name_escaped,
    bench_qualified_name_building,
    bench_qualified_name_operations,
);

criterion_main!(benches);
