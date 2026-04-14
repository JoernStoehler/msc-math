//! Random systolic ratio sample over random 4D polytopes.
//!
//! Goal: Compute systolic ratios for random 4D polytopes across facet counts F=5..12,
//!   to probe whether random generic polytopes approach the Viterbo threshold.
//! Input: Polytope database at data/polytopes.jsonl (created if missing).
//! Output: crates/exp-sys-landscape/random-sample/random-sweep.jsonl
//!
//! Architecture:
//! 1. `cargo run -p exp-sys-landscape --release --bin sys-random-sample` generates dataset
//! 2. Polytopes are generated via `generate_polytope` (blake3 per-attempt seeding)
//!    and cached in the polytope database. Re-runs skip generation + capacity.
//! 3. Writes to random-sample/random-sweep.jsonl
//! 4. Python script plots sys vs F
//!
//! Dataset design:
//! - Random polytopes with facet counts F=5..12
//! - Height range h in [0.8, 1.2]
//! - HK2017 pruned only (production algorithm)

use database::{DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use std::collections::HashMap;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::volume::volume;
use symplectic::random::generate_polytope;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
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
    dual_vertices: Vec<[f64; 4]>,
    h_min: f64,
    h_max: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

/// Find a cached record by Source metadata. Linear scan, negligible for <1000 records.
fn find_by_source<'a>(
    db: &'a HashMap<DualVerticesKey, PolytopeRecord>,
    source: &Source,
) -> Option<(&'a DualVerticesKey, &'a PolytopeRecord)> {
    db.iter().find(|(_, r)| r.source.as_ref() == Some(source))
}

fn main() {
    let t0 = Instant::now();

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/polytopes.jsonl");
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("random-sample/random-sweep.jsonl");

    let mut db = database::load(&db_path).expect("failed to load database");
    println!("Loaded database: {} entries\n", db.len());

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut cache_hits = 0usize;
    let mut attempt: u64 = 0;

    for &(facet_count, n_samples) in RANDOM_PLAN {
        println!(
            "F={facet_count:2}: generating {n_samples:2} samples (h in [{H_MIN}, {H_MAX}])"
        );
        let mut accepted = 0usize;

        while accepted < n_samples {
            let source = Source::Random {
                master_seed: SEED,
                attempt,
                facet_count_target: facet_count,
                h_min: H_MIN,
                h_max: H_MAX,
            };

            // Try Source-based lookup first
            if let Some((_, record)) = find_by_source(&db, &source) {
                // Cache hit: reconstruct polytope from rational data (skip vertex enumeration)
                let p = record.to_polytope().expect("failed to reconstruct polytope from database");
                let vol = record.volume.expect("cached record missing volume");
                let cap = record.capacity.expect("cached record missing capacity");
                let sys = cap * cap / (2.0 * vol);

                let row = RandomSweepRow {
                    name: format!("random_F{facet_count}_{accepted}"),
                    facet_count,
                    dual_vertices: p.dual_vertices_f64().iter().map(|a| [a[0], a[1], a[2], a[3]]).collect(),
                    h_min: H_MIN,
                    h_max: H_MAX,
                    volume: vol,
                    capacity: cap,
                    sys,
                    iterations: 0, // not stored in database
                    time_volume_ms: 0.0,
                    time_capacity_ms: 0.0,
                };

                let line = serde_json::to_string(&row).expect("serialize row");
                writeln!(writer, "{line}").expect("write line");
                total += 1;
                accepted += 1;
                cache_hits += 1;
                attempt += 1;
                continue;
            }

            // Cache miss: generate polytope
            let p = match generate_polytope(facet_count, H_MIN, H_MAX, SEED, attempt) {
                Ok(p) => p,
                Err(_) => {
                    // Rejection: this (seed, attempt) doesn't produce a valid polytope
                    attempt += 1;
                    continue;
                }
            };

            let start_vol = Instant::now();
            let vol = volume(&p).expect("volume computation failed");
            let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

            let start_cap = Instant::now();
            let ehz = ehz_capacity(&p).expect("capacity computation failed");
            let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

            let cap = ehz.result.capacity;
            let sys = cap * cap / (2.0 * vol);

            // Insert into database
            let mut record = PolytopeRecord::from_polytope(&p);
            record.source = Some(source);
            record = record.with_computed_fields(vol, 0.0, cap, 0.0);
            record = record.with_sigmas(
                vec![SigmaAction {
                    perm: ehz.result.best_permutation.clone(),
                    action: cap,
                }],
                0.0, // gap_cutoff: only storing the best sigma
            );
            db.insert(record.key(), record);

            let row = RandomSweepRow {
                name: format!("random_F{facet_count}_{accepted}"),
                facet_count,
                dual_vertices: p.dual_vertices_f64().iter().map(|a| [a[0], a[1], a[2], a[3]]).collect(),
                h_min: H_MIN,
                h_max: H_MAX,
                volume: vol,
                capacity: cap,
                sys,
                iterations: ehz.result.iterations,
                time_volume_ms,
                time_capacity_ms,
            };

            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total += 1;
            accepted += 1;
            attempt += 1;
        }
    }

    writer.flush().expect("flush output");
    database::save(&db_path, &db).expect("failed to save database");

    println!("\nWrote {total} entries to {}", output_path.display());
    println!("Database: {} entries (saved to {})", db.len(), db_path.display());
    println!("Cache hits: {cache_hits}/{total}");
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
