//! Random Lagrangian product sweep over polygon pairs.
//!
//! Architecture:
//! 1. `cargo run --bin random_product_sweep --release` generates dataset
//! 2. Writes to random-product-sweep/random-product-sweep.jsonl
//! 3. Python script plots sys vs (k,m)
//!
//! Dataset design:
//! - Random 2D polygons with k, m in {3,4,5,6}
//! - Pairs with 3 <= k <= m <= 6 (10 buckets)
//! - 10 samples per bucket
//! - Height range h in [0.8, 1.2]
//! - Billiard algorithm only (Lagrangian products)

// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::algorithms::billiard::billiard_capacity;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::geom::volume::volume;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
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

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("random-product-sweep/random-product-sweep.jsonl");

    println!("Generating random Lagrangian product sweep...\n");

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    for &(k, m) in PAIRS {
        println!("Bucket ({k},{m}) with {SAMPLES_PER_BUCKET} samples");

        let mut accepted = 0usize;
        while accepted < SAMPLES_PER_BUCKET {
            let (qn, qh) = random_polygon_2d(k, H_MIN, H_MAX, &mut rng);
            let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);

            let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
                Ok(p) => p,
                Err(_) => continue,
            };

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
    println!("\nWrote {total} entries to {}", output_path.display());
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
