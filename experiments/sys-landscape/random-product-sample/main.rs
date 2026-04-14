//! Random Lagrangian product sample over polygon pairs.
//!
//! Architecture:
//! 1. `cargo run -p exp-sys-landscape --release --bin sys-random-product-sample` generates dataset
//! 2. Polytopes are cached in data/polytopes.jsonl. Re-runs skip capacity.
//! 3. Writes to random-product-sample/random-product-sweep.jsonl
//! 4. Python script plots sys vs (k,m)
//!
//! Dataset design:
//! - Random 2D polygons with k, m in {3,4,5,6}
//! - Pairs with 3 <= k <= m <= 6 (10 buckets)
//! - 10 samples per bucket
//! - Height range h in [0.8, 1.2]
//! - Billiard algorithm only (Lagrangian products)
//!
//! Note: Uses shared RNG (no blake3 per-attempt seeding) because there is no
//! generate_polytope equivalent for Lagrangian products. Database lookup is
//! key-based (BigRational dual vertices), not Source-based.

use std::collections::HashMap;
use symplectic::algorithms::billiard::billiard_capacity;
use symplectic::database::{load, save, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::geom::volume::volume;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const SAMPLES_PER_BUCKET: usize = 10;

const PAIRS: &[(usize, usize)] = &[
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (4, 4),
    (4, 5),
    (4, 6),
    (5, 5),
    (5, 6),
    (6, 6),
];

#[derive(Debug, Serialize)]
struct RandomProductRow {
    name: String,
    k: usize,
    m: usize,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    h_min: f64,
    h_max: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    bounces: usize,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/polytopes.jsonl");
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("random-product-sample/random-product-sweep.jsonl");

    let mut db: HashMap<DualVerticesKey, PolytopeRecord> =
        load(&db_path).expect("failed to load database");
    println!("Loaded database: {} entries\n", db.len());

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut cache_hits = 0usize;

    for &(k, m) in PAIRS {
        println!("Bucket ({k},{m}) with {SAMPLES_PER_BUCKET} samples");

        let mut accepted = 0usize;
        while accepted < SAMPLES_PER_BUCKET {
            // Generate polygon pair using shared RNG (advances RNG regardless of acceptance)
            let (qn, qh) = random_polygon_2d(k, H_MIN, H_MAX, &mut rng);
            let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);

            let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let key: DualVerticesKey = polytope.dual_vertices().to_vec();

            // Key-based lookup: check if this exact polytope is already cached
            if let Some(record) = db.get_mut(&key) {
                // Backfill source for records that predate source tracking
                if record.source.is_none() {
                    record.source = Some(Source::LagrangianProduct {
                        n1: k,
                        n2: m,
                        circumradius_q: 0.0,
                        circumradius_p: 0.0,
                        rotation_p_rad: 0.0,
                    });
                }
                if let (Some(vol), Some(cap)) = (record.volume, record.capacity) {
                    let sys = cap * cap / (2.0 * vol);

                    let row = RandomProductRow {
                        name: format!("random_{k}x{m}_{accepted}"),
                        k,
                        m,
                        facet_count: k + m,
                        dual_vertices: polytope
                            .dual_vertices_f64()
                            .iter()
                            .map(|a| [a[0], a[1], a[2], a[3]])
                            .collect(),
                        h_min: H_MIN,
                        h_max: H_MAX,
                        volume: vol,
                        capacity: cap,
                        sys,
                        iterations: 0,
                        bounces: 0,
                        time_volume_ms: 0.0,
                        time_capacity_ms: 0.0,
                    };

                    let line = serde_json::to_string(&row).expect("serialize row");
                    writeln!(writer, "{line}").expect("write line");
                    accepted += 1;
                    total += 1;
                    cache_hits += 1;
                    continue;
                }
            }

            // Cache miss: compute capacity
            let start_vol = Instant::now();
            let vol = volume(&polytope).expect("volume computation failed");
            let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

            let start_cap = Instant::now();
            let result = billiard_capacity(&polytope)
                .expect("billiard failed")
                .expect("billiard returned None");
            let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

            let cap = result.result.capacity;
            let sys = cap * cap / (2.0 * vol);

            // Insert into database
            let mut record = PolytopeRecord::from_polytope(&polytope);
            record.source = Some(Source::LagrangianProduct {
                n1: k,
                n2: m,
                // Random polygon pair — no fixed circumradius or rotation.
                circumradius_q: 0.0,
                circumradius_p: 0.0,
                rotation_p_rad: 0.0,
            });
            record = record.with_computed_fields(vol, 0.0, cap, 0.0);
            record = record.with_sigmas(
                vec![SigmaAction {
                    perm: result.result.best_permutation.clone(),
                    action: cap,
                }],
                0.0,
            );
            db.insert(key, record);

            let row = RandomProductRow {
                name: format!("random_{k}x{m}_{accepted}"),
                k,
                m,
                facet_count: k + m,
                dual_vertices: polytope
                    .dual_vertices_f64()
                    .iter()
                    .map(|a| [a[0], a[1], a[2], a[3]])
                    .collect(),
                h_min: H_MIN,
                h_max: H_MAX,
                volume: vol,
                capacity: cap,
                sys,
                iterations: result.result.iterations,
                bounces: result.bounce_count,
                time_volume_ms,
                time_capacity_ms,
            };

            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            accepted += 1;
            total += 1;
        }
    }

    writer.flush().expect("flush output");
    save(&db_path, &db).expect("failed to save database");

    println!("\nWrote {total} entries to {}", output_path.display());
    println!("Database: {} entries (saved to {})", db.len(), db_path.display());
    println!("Cache hits: {cache_hits}/{total}");
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
