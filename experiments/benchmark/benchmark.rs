//! Benchmark dataset generator: curated polytopes for timing analysis.
//!
//! Architecture:
//! 1. `cargo run --bin benchmark --release` generates benchmark dataset
//! 2. Writes to benchmark/benchmark.jsonl
//! 3. Python script reads JSONL and fits timing model
//!
//! Dataset design:
//! - Random polytopes for HK2017 timing model (F=5..12)
//! - Lagrangian products for billiard vs HK2017 comparison
//! - Smaller counts at high F for practical runtime
//!
//! Total: ~85 polytopes, ~100 capacity computations

use symplectic::billiard_capacity;
use symplectic::random::generate_random_polytopes;
use symplectic::lagrangian_product;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::{ehz_capacity_unpruned, ehz_capacity};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkEntry {
    name: String,
    group: String, // "random" or "lagrangian"
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,

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

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut entries = Vec::new();
    
    // Construct output path relative to repo root (works from any cwd)
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("benchmark/benchmark.jsonl");

    println!("Generating benchmark dataset...\n");

    // Part 1: Random polytopes for HK2017 timing model
    println!("Part 1: Random polytopes for HK2017 timing model");
    for &(f, n, include_unpruned) in RANDOM_PLAN {
        print!("  F={f:2}: generating {n:2} polytopes... ");
        let polytopes = generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);

        for (i, p) in polytopes.iter().enumerate() {
            // Pruned (always)
            let t_start = Instant::now();
            let result_pruned = ehz_capacity(p).expect("pruned failed");
            let time_pruned_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            // Unpruned (only if F <= 7)
            let (time_unpruned_ms, capacity_unpruned, iterations_unpruned) = if include_unpruned {
                let t_start = Instant::now();
                let result = ehz_capacity_unpruned(p).expect("unpruned failed");
                let time_ms = t_start.elapsed().as_secs_f64() * 1000.0;
                (Some(time_ms), Some(result.capacity), Some(result.iterations))
            } else {
                (None, None, None)
            };

            entries.push(BenchmarkEntry {
                name: format!("random_F{f}_{i}"),
                group: "random".to_string(),
                facet_count: p.facet_count(),
                normals: p.normals_f64().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
                heights: p.heights_f64().to_vec(),
                time_pruned_ms,
                capacity_pruned: result_pruned.capacity,
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
    for &(n, m) in LAGRANGIAN_PLAN {
        print!("  ({n},{m}): generating 2 samples... ");

        for i in 0..2 {
            // Generate random Lagrangian product (retry until valid)
            let p = loop {
                let (qn, qh) = random_polygon_2d(n, H_MIN, H_MAX, &mut rng);
                let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
                if let Ok(polytope) = lagrangian_product(&qn, &qh, &pn, &ph) {
                    break polytope;
                }
            };

            // Pruned
            let t_start = Instant::now();
            let result_pruned = ehz_capacity(&p).expect("pruned failed");
            let time_pruned_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            // Billiard
            let t_start = Instant::now();
            let result_billiard = billiard_capacity(&p)
                .expect("billiard failed")
                .expect("billiard returned None");
            let time_billiard_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            entries.push(BenchmarkEntry {
                name: format!("lagrangian_{n}x{m}_{i}"),
                group: "lagrangian".to_string(),
                facet_count: p.facet_count(),
                normals: p.normals_f64().iter().map(|v| [v[0], v[1], v[2], v[3]]).collect(),
                heights: p.heights_f64().to_vec(),
                time_pruned_ms,
                capacity_pruned: result_pruned.capacity,
                iterations_pruned: result_pruned.iterations,
                time_unpruned_ms: None,
                capacity_unpruned: None,
                iterations_unpruned: None,
                time_billiard_ms: Some(time_billiard_ms),
                capacity_billiard: Some(result_billiard.capacity),
                iterations_billiard: Some(result_billiard.iterations),
            });
        }
        println!("done");
    }

    // Write to JSONL
    let file = File::create(&output_path).expect("failed to create output file");
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
        entries
            .iter()
            .filter(|e| e.group == "lagrangian")
            .count()
    );
}
