//! Benchmark dataset generator: curated polytopes for timing analysis.
//!
//! Goal: Generate a mixed random/Lagrangian timing benchmark for pruned,
//! unpruned, and billiard capacity algorithms.
//! Input Artifacts: None (generates all benchmark fixtures internally).
//! Output Artifacts: experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl
//!
//! Architecture:
//! 1. `cargo run -p dev-algorithm-comparison --release --bin cmp-benchmark -- --smoke`
//!    runs a small smoke subset and writes `benchmark/benchmark-smoke.jsonl`.
//! 2. `cargo run -p dev-algorithm-comparison --release --bin cmp-benchmark` generates
//!    the full benchmark dataset and writes `benchmark/benchmark.jsonl`.
//! 3. Python script reads JSONL and fits timing model
//!
//! Dataset design:
//! - Random polytopes for HK2017 timing model (F=5..12)
//! - Lagrangian products for billiard vs HK2017 comparison
//! - Smaller counts at high F for practical runtime
//!
//! Total: ~85 polytopes, ~100 capacity computations
//!
//! Capacity routing is intentionally explicit in this file because the dataset
//! compares pruned HK2017, unpruned HK2017, and billiard timings on the same
//! fixtures. The crate-level `ehz_capacity` entrypoint hides per-algorithm
//! comparison paths.

use dev_capacity_validation::{
    capacity_billiard as cache_capacity_billiard,
    capacity_pruned_hk2017 as cache_capacity_pruned_hk2017,
    capacity_unpruned_hk2017 as cache_capacity_unpruned_hk2017, VerificationPolytopeCache,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::random::generate_random_dual_vertices;
use symplectic::{BilliardError, OrbitSearchError, OrbitSearchResult};

const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkEntry {
    name: String,
    group: String, // "random" or "lagrangian"
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,

    // Pruned timing (all polytopes)
    time_pruned_ms: f64,
    capacity_pruned: f64,
    iterations_pruned: u64,

    // Unpruned timing (F <= 7 only, for algorithm agreement)
    time_unpruned_ms: Option<f64>,
    capacity_unpruned: Option<f64>,
    iterations_unpruned: Option<u64>,

    // Billiard timing (Lagrangian products only)
    time_billiard_ms: Option<f64>,
    capacity_billiard: Option<f64>,
    iterations_billiard: Option<u64>,
}

/// Sample plan: (facet_count, n_samples, include_unpruned)
/// Smaller counts at high F for practical runtime
const RANDOM_PLAN: &[(usize, usize, bool)] = &[
    (5, 10, true),   // F=5: 10 samples, test pruned vs unpruned
    (6, 10, true),   // F=6: 10 samples, test pruned vs unpruned
    (7, 10, true),   // F=7: 10 samples, test pruned vs unpruned
    (8, 15, false),  // F=8: 15 samples, pruned only (timing model)
    (9, 15, false),  // F=9: 15 samples, pruned only
    (10, 15, false), // F=10: 15 samples, pruned only
    (11, 5, false),  // F=11: 5 samples, pruned only (expensive)
    (12, 3, false),  // F=12: 3 samples, pruned only (very expensive)
];

/// Lagrangian products for billiard timing comparison
const LAGRANGIAN_PLAN: &[(usize, usize)] = &[
    (3, 3), // Triangle × Triangle (F=6)
    (3, 4), // Triangle × Square (F=7)
    (4, 4), // Square × Square (F=8)
    (3, 5), // Triangle × Pentagon (F=8)
    (4, 5), // Square × Pentagon (F=9)
    (5, 5), // Pentagon × Pentagon (F=10)
];
const SMOKE_RANDOM_PLAN: &[(usize, usize, bool)] = &[(5, 1, true)];
const SMOKE_LAGRANGIAN_PLAN: &[(usize, usize)] = &[(3, 3)];

fn cache_from_dual_vertices(
    dual_vertices: Vec<nalgebra::Vector4<f64>>,
) -> VerificationPolytopeCache {
    VerificationPolytopeCache::from_f64_dual_vertices(dual_vertices)
        .expect("accepted random dual vertices should reconstruct")
}

fn capacity_pruned_hk2017(
    polytope: &VerificationPolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    cache_capacity_pruned_hk2017(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
}

fn capacity_unpruned_hk2017(
    polytope: &VerificationPolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    cache_capacity_unpruned_hk2017(&polytope.dual_vertices_f64, &polytope.dual_vertices)
}

fn capacity_billiard(
    polytope: &VerificationPolytopeCache,
) -> Result<OrbitSearchResult, BilliardError> {
    cache_capacity_billiard(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
}

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: cmp-benchmark [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke              Run a small smoke subset and write smoke output."#
    );
}

fn usage_error(message: String) -> ! {
    eprintln!("error: {message}\n");
    print_usage();
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut args = Args { smoke: false };
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--smoke" => args.smoke = true,
            other => usage_error(format!("unknown argument: {other}")),
        }
    }
    args
}

fn output_path(manifest_dir: &Path, smoke: bool) -> PathBuf {
    if smoke {
        manifest_dir.join("benchmark/benchmark-smoke.jsonl")
    } else {
        manifest_dir.join("benchmark/benchmark.jsonl")
    }
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut entries = Vec::new();

    // Construct output path relative to repo root (works from any cwd)
    let output_path = output_path(std::path::Path::new(env!("CARGO_MANIFEST_DIR")), args.smoke);
    let random_plan = if args.smoke {
        SMOKE_RANDOM_PLAN
    } else {
        RANDOM_PLAN
    };
    let lagrangian_plan = if args.smoke {
        SMOKE_LAGRANGIAN_PLAN
    } else {
        LAGRANGIAN_PLAN
    };
    let lagrangian_samples = if args.smoke { 1 } else { 2 };

    println!("Generating benchmark dataset...\n");

    // Part 1: Random polytopes for HK2017 timing model
    println!("Part 1: Random polytopes for HK2017 timing model");
    for &(f, n, include_unpruned) in random_plan {
        print!("  F={f:2}: generating {n:2} polytopes... ");
        let polytopes = generate_random_dual_vertices(n, f, H_MIN, H_MAX, &mut rng)
            .into_iter()
            .map(cache_from_dual_vertices)
            .collect::<Vec<_>>();

        for (i, p) in polytopes.iter().enumerate() {
            // Pruned (always)
            let t_start = Instant::now();
            let result_pruned = capacity_pruned_hk2017(p).expect("pruned failed");
            let time_pruned_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            // Unpruned (only if F <= 7)
            let (time_unpruned_ms, capacity_unpruned, iterations_unpruned) = if include_unpruned {
                let t_start = Instant::now();
                let result = capacity_unpruned_hk2017(p).expect("unpruned failed");
                let time_ms = t_start.elapsed().as_secs_f64() * 1000.0;
                (
                    Some(time_ms),
                    Some(result.capacity()),
                    Some(result.iterations),
                )
            } else {
                (None, None, None)
            };

            entries.push(BenchmarkEntry {
                name: format!("random_F{f}_{i}"),
                group: "random".to_string(),
                facet_count: p.facet_count(),
                dual_vertices: p
                    .dual_vertices_f64
                    .iter()
                    .map(|a| [a[0], a[1], a[2], a[3]])
                    .collect(),
                time_pruned_ms,
                capacity_pruned: result_pruned.capacity(),
                iterations_pruned: result_pruned.iterations,
                time_unpruned_ms,
                capacity_unpruned,
                iterations_unpruned,
                time_billiard_ms: None,
                capacity_billiard: None,
                iterations_billiard: None,
            });
        }
        println!("done");
    }

    // Part 2: Lagrangian products for billiard timing
    println!("\nPart 2: Lagrangian products for billiard vs HK2017 comparison");
    for &(n, m) in lagrangian_plan {
        print!("  ({n},{m}): generating {lagrangian_samples} samples... ");

        for i in 0..lagrangian_samples {
            // Generate random Lagrangian product (retry until valid)
            let p = loop {
                let (qn, qh) = random_polygon_2d(n, H_MIN, H_MAX, &mut rng);
                let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
                if let Some(polytope) =
                    VerificationPolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
                {
                    break polytope;
                }
            };

            // Pruned
            let t_start = Instant::now();
            let result_pruned = capacity_pruned_hk2017(&p).expect("pruned failed");
            let time_pruned_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            // Billiard
            let t_start = Instant::now();
            let result_billiard = capacity_billiard(&p).expect("billiard failed");
            let time_billiard_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            entries.push(BenchmarkEntry {
                name: format!("lagrangian_{n}x{m}_{i}"),
                group: "lagrangian".to_string(),
                facet_count: p.facet_count(),
                dual_vertices: p
                    .dual_vertices_f64
                    .iter()
                    .map(|a| [a[0], a[1], a[2], a[3]])
                    .collect(),
                time_pruned_ms,
                capacity_pruned: result_pruned.capacity(),
                iterations_pruned: result_pruned.iterations,
                time_unpruned_ms: None,
                capacity_unpruned: None,
                iterations_unpruned: None,
                time_billiard_ms: Some(time_billiard_ms),
                capacity_billiard: Some(result_billiard.capacity()),
                iterations_billiard: Some(result_billiard.iterations),
            });
        }
        println!("done");
    }

    // Write to JSONL
    let file = File::create(&output_path).unwrap_or_else(|err| {
        panic!(
            "failed to create output file {}: {err}",
            output_path.display()
        )
    });
    let mut writer = BufWriter::new(file);

    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).expect("failed to serialize entry");
        writeln!(writer).expect("failed to write newline");
    }

    println!(
        "\nWrote {} entries to {}",
        entries.len(),
        output_path.display()
    );
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
    println!("\nDataset breakdown:");
    println!(
        "  Random polytopes: {} (F=5..12)",
        entries.iter().filter(|e| e.group == "random").count()
    );
    println!(
        "  Lagrangian products: {} (billiard comparison)",
        entries.iter().filter(|e| e.group == "lagrangian").count()
    );
}
