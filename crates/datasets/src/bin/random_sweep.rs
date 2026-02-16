//! Random systolic ratio sweep over random 4D polytopes.
//!
//! Architecture:
//! 1. `cargo run --bin random_sweep --release` generates dataset
//! 2. Writes to experiments/data/random-sweep.jsonl
//! 3. Python script (experiments/scripts/random_sweep.py) plots sys vs F
//!
//! Dataset design:
//! - Random polytopes with facet counts F=5..12
//! - Height range h in [0.8, 1.2]
//! - HK2017 pruned only (production algorithm)

use datasets::random::generate_random_polytopes;
use geom::volume::volume;
use hk2017::ehz_capacity_pruned;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples)
const RANDOM_PLAN: &[(usize, usize)] = &[
    (5, 10),
    (6, 10),
    (7, 10),
    (8, 10),
    (9, 10),
    (10, 10),
    (11, 5),
    (12, 5),
];

#[derive(Debug, Serialize)]
struct RandomSweepRow {
    name: String,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    h_min: f64,
    h_max: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("experiments/data/random-sweep.jsonl"))
        .expect("failed to construct output path");

    println!("Generating random sweep dataset...\n");

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    for &(facet_count, n_samples) in RANDOM_PLAN {
        println!(
            "F={facet_count:2}: generating {n_samples:2} samples (h in [{H_MIN}, {H_MAX}])"
        );
        let polytopes = generate_random_polytopes(n_samples, facet_count, H_MIN, H_MAX, &mut rng);

        for (i, p) in polytopes.iter().enumerate() {
            let start_vol = Instant::now();
            let vol = volume(p).expect("volume computation failed");
            let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

            let start_cap = Instant::now();
            let result = ehz_capacity_pruned(p).expect("capacity computation failed");
            let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

            let cap = result.capacity;
            let sys = cap * cap / (2.0 * vol);

            let row = RandomSweepRow {
                name: format!("random_F{facet_count}_{i}"),
                facet_count,
                normals: p.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
                heights: p.heights().to_vec(),
                h_min: H_MIN,
                h_max: H_MAX,
                volume: vol,
                capacity: cap,
                sys,
                iterations: result.iterations,
                time_volume_ms,
                time_capacity_ms,
            };

            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total += 1;
        }
    }

    writer.flush().expect("flush output");
    println!("\nWrote {total} entries to {}", output_path.display());
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
