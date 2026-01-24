//! Quick ad-hoc benchmark for parsing speed.
//!
//! Run with:
//! ```bash
//! cargo run -p sysml-text-pest --example quick_bench --release
//! ```
//!
//! Or with the corpus:
//! ```bash
//! SYSML_CORPUS_PATH=/path/to/sysmlv2-references \
//!   cargo run -p sysml-text-pest --example quick_bench --release -- --corpus
//! ```

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use sysml_text::{Parser, SysmlFile};
use sysml_text_pest::PestParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let use_corpus = args.iter().any(|a| a == "--corpus");

    if use_corpus {
        bench_corpus();
    } else {
        bench_synthetic();
    }
}

fn bench_synthetic() {
    println!("=== Synthetic Model Benchmarks ===\n");

    // Generate test models of various sizes
    let sizes = [10, 50, 100, 500, 1000];

    for &size in &sizes {
        let source = generate_model(size);
        let bytes = source.len();
        let parser = PestParser::new();
        let files = vec![SysmlFile::new("bench.sysml", &source)];

        // Warmup
        for _ in 0..3 {
            let _ = parser.parse(&files);
        }

        // Timed runs
        let iterations = 10;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = parser.parse(&files);
        }
        let elapsed = start.elapsed();

        let avg = elapsed / iterations;
        let throughput = (bytes as f64 * iterations as f64) / elapsed.as_secs_f64();

        println!(
            "  {:>5} parts: {:>8.2?} avg, {:>10.0} bytes/sec ({} bytes)",
            size,
            avg,
            throughput,
            bytes
        );
    }

    println!();
    bench_phases();
}

fn bench_phases() {
    println!("=== Phase Breakdown (500 parts) ===\n");

    let source = generate_model(500);
    let parser_with_spans = PestParser::new();
    let parser_no_spans = PestParser::without_spans();
    let files = vec![SysmlFile::new("bench.sysml", &source)];

    // Full parse with spans
    let (time_with, result_with) = time_it(|| parser_with_spans.parse(&files));
    println!("  Full parse (with spans):    {:>10.2?}", time_with);
    println!("    - {} elements created", result_with.graph.element_count());

    // Full parse without spans
    let (time_without, result_without) = time_it(|| parser_no_spans.parse(&files));
    println!("  Full parse (without spans): {:>10.2?}", time_without);
    println!("    - {} elements created", result_without.graph.element_count());

    // With resolution
    let (time_resolved, result_resolved) = time_it(|| {
        let mut r = parser_with_spans.parse(&files);
        r.resolve();
        r
    });
    println!("  Parse + resolution:         {:>10.2?}", time_resolved);
    println!(
        "    - {} elements, {} relationships",
        result_resolved.graph.element_count(),
        result_resolved.graph.relationship_count()
    );

    println!();
    println!("  Span overhead: {:>+.1?} ({:.1}%)",
        time_with.saturating_sub(time_without),
        if time_without.as_nanos() > 0 {
            (time_with.as_nanos() as f64 / time_without.as_nanos() as f64 - 1.0) * 100.0
        } else {
            0.0
        }
    );
}

fn bench_corpus() {
    println!("=== Corpus Benchmarks ===\n");

    let corpus_path = env::var("SYSML_CORPUS_PATH")
        .expect("SYSML_CORPUS_PATH environment variable must be set");

    let corpus_dir = PathBuf::from(&corpus_path);

    // Find all .sysml files
    let mut files: Vec<(String, String)> = Vec::new();
    find_sysml_files(&corpus_dir, &mut files);

    println!("Found {} .sysml files\n", files.len());

    let parser = PestParser::new();

    // Parse each file and collect timing
    let mut total_bytes = 0usize;
    let mut total_time = Duration::ZERO;
    let mut total_elements = 0usize;
    let mut success_count = 0usize;
    let mut fail_count = 0usize;

    for (path, content) in &files {
        let sysml_files = vec![SysmlFile::new(path, content)];

        let start = Instant::now();
        let result = parser.parse(&sysml_files);
        let elapsed = start.elapsed();

        total_bytes += content.len();
        total_time += elapsed;
        total_elements += result.graph.element_count();

        if result.is_ok() {
            success_count += 1;
        } else {
            fail_count += 1;
        }
    }

    println!("Results:");
    println!("  Files:      {} success, {} failed", success_count, fail_count);
    println!("  Total time: {:?}", total_time);
    println!("  Total size: {} bytes ({:.1} KB)", total_bytes, total_bytes as f64 / 1024.0);
    println!("  Elements:   {}", total_elements);
    println!();
    println!("Throughput:   {:.0} bytes/sec", total_bytes as f64 / total_time.as_secs_f64());
    println!("Avg per file: {:?}", total_time / files.len() as u32);
}

fn find_sysml_files(dir: &PathBuf, files: &mut Vec<(String, String)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_sysml_files(&path, files);
            } else if path.extension().map_or(false, |e| e == "sysml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    files.push((path.display().to_string(), content));
                }
            }
        }
    }
}

fn generate_model(num_parts: usize) -> String {
    let mut s = String::with_capacity(num_parts * 50);
    s.push_str("package BenchmarkModel {\n");

    // Add some definitions
    for i in 0..num_parts / 10 {
        s.push_str(&format!("    part def Type{};\n", i));
    }

    // Add usages that reference definitions
    for i in 0..num_parts {
        let type_idx = i % (num_parts / 10).max(1);
        s.push_str(&format!("    part instance{} : Type{};\n", i, type_idx));
    }

    s.push_str("}\n");
    s
}

fn time_it<T, F: FnOnce() -> T>(f: F) -> (Duration, T) {
    let start = Instant::now();
    let result = f();
    (start.elapsed(), result)
}
