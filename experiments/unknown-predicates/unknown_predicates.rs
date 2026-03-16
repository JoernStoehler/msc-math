//! Phase 1: Check whether UNKNOWN admissibility predicates appear in practice.
//!
//! Regenerates the random-sweep and lagrangian-products datasets using the same
//! seeds/parameters, then records the certified vs uncertain capacity for each
//! polytope. If `numerical_gap > 0` for any polytope, an UNKNOWN predicate
//! affected the capacity — meaning Phase 2 (high-precision re-solve) is needed.
//!
//! Architecture:
//! 1. `cargo run --bin unknown_predicates --release` generates dataset
//! 2. Writes to unknown-predicates/unknown-predicates.jsonl
//! 3. Python script reads JSONL, summarizes findings

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
use symplectic::random::generate_random_polytopes;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::algorithms::billiard::billiard_capacity;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::volume::volume;

// ---------------------------------------------------------------------------
// Random-sweep parameters (must match random_sweep.rs exactly)
// ---------------------------------------------------------------------------
const RANDOM_SEED: u64 = 42;
const RANDOM_H_MIN: f64 = 0.8;
const RANDOM_H_MAX: f64 = 1.2;
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

// ---------------------------------------------------------------------------
// Lagrangian-products parameters (must match lagrangian_sweep.rs exactly)
// ---------------------------------------------------------------------------
const PENTAGON_START_DEG: f64 = 0.0;
const PENTAGON_END_DEG: f64 = 36.0;
const PENTAGON_STEP_DEG: f64 = 1.0;

const PAIR_STEP_DEG: f64 = 6.0;
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

// ---------------------------------------------------------------------------
// Output schema
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct Row {
    dataset: String,
    name: String,
    algorithm: String,
    facet_count: usize,
    capacity: f64,
    capacity_uncertain: f64,
    numerical_gap: f64,
    has_unknown: bool,
    beta_min: f64,
    volume: f64,
    sys: f64,
    time_ms: f64,
}

fn main() {
    let t0 = Instant::now();

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("unknown-predicates/unknown-predicates.jsonl");
    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total_rows = 0usize;
    let mut total_unknowns = 0usize;

    // -----------------------------------------------------------------------
    // Part 1: Random-sweep polytopes (ehz_capacity)
    // -----------------------------------------------------------------------
    println!("=== Part 1: Random-sweep polytopes ===\n");

    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED);

    for &(facet_count, n_samples) in RANDOM_PLAN {
        let polytopes =
            generate_random_polytopes(n_samples, facet_count, RANDOM_H_MIN, RANDOM_H_MAX, &mut rng);

        for (i, p) in polytopes.iter().enumerate() {
            let vol = volume(p).expect("volume computation failed");

            let start = Instant::now();
            let result = ehz_capacity(p).expect("ehz_capacity returned None");
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let gap = result.numerical_gap();
            let beta_min = result
                .best_beta
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let has_unknown = gap > 0.0;
            let sys = result.capacity * result.capacity / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: random_F{facet_count}_{i}: gap={gap:.2e}, \
                     cap={:.8}, cap_unc={:.8}, beta_min={beta_min:.2e}",
                    result.capacity, result.capacity_uncertain
                );
            }

            let row = Row {
                dataset: "random-sweep".to_string(),
                name: format!("random_F{facet_count}_{i}"),
                algorithm: "ehz_pruned".to_string(),
                facet_count,
                capacity: result.capacity,
                capacity_uncertain: result.capacity_uncertain,
                numerical_gap: gap,
                has_unknown,
                beta_min,
                volume: vol,
                sys,
                time_ms,
            };
            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total_rows += 1;
        }

        println!(
            "  F={facet_count:2}: {n_samples} polytopes processed"
        );
    }

    // -----------------------------------------------------------------------
    // Part 2: Lagrangian-products — pentagon 5×5 sweep (billiard_capacity)
    // -----------------------------------------------------------------------
    println!("\n=== Part 2: Lagrangian-products ===\n");

    // Pentagon 5×5 sweep
    {
        let steps =
            ((PENTAGON_END_DEG - PENTAGON_START_DEG) / PENTAGON_STEP_DEG).round() as usize;
        let (qn, qh) = regular_polygon_2d(5, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);

        for i in 0..=steps {
            let angle_deg = PENTAGON_START_DEG + PENTAGON_STEP_DEG * (i as f64);
            let theta = angle_deg.to_radians();

            let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                .expect("pentagon product construction failed");

            let vol = volume(&polytope).expect("volume computation failed");

            let start = Instant::now();
            let result = billiard_capacity(&polytope)
                .expect("billiard error")
                .expect("billiard returned None");
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let gap = result.capacity - result.capacity_uncertain;
            let beta_min = result
                .best_beta
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let has_unknown = gap > 0.0;
            let sys = result.capacity * result.capacity / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: pentagon_5x5_{angle_deg:.0}deg: gap={gap:.2e}, \
                     cap={:.8}, cap_unc={:.8}, beta_min={beta_min:.2e}",
                    result.capacity, result.capacity_uncertain
                );
            }

            let row = Row {
                dataset: "lagrangian-products".to_string(),
                name: format!("pentagon_5x5_{angle_deg:.0}deg"),
                algorithm: "billiard".to_string(),
                facet_count: 10,
                capacity: result.capacity,
                capacity_uncertain: result.capacity_uncertain,
                numerical_gap: gap,
                has_unknown,
                beta_min,
                volume: vol,
                sys,
                time_ms,
            };
            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total_rows += 1;
        }

        println!("  pentagon 5×5: {} angles processed", steps + 1);
    }

    // Polygon pair sweeps
    for &(n1, n2) in PAIRS {
        let end_deg = 180.0 / lcm(n1, n2) as f64;
        let angles = sweep_angles(0.0, end_deg, PAIR_STEP_DEG);

        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);

        for angle_deg in &angles {
            let theta = angle_deg.to_radians();
            let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                .expect("polygon product construction failed");

            let vol = volume(&polytope).expect("volume computation failed");

            let start = Instant::now();
            let result = billiard_capacity(&polytope)
                .expect("billiard error")
                .expect("billiard returned None");
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let gap = result.capacity - result.capacity_uncertain;
            let beta_min = result
                .best_beta
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let has_unknown = gap > 0.0;
            let sys = result.capacity * result.capacity / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: pair_{n1}x{n2}_{angle_deg:.0}deg: gap={gap:.2e}, \
                     cap={:.8}, cap_unc={:.8}, beta_min={beta_min:.2e}",
                    result.capacity, result.capacity_uncertain
                );
            }

            let row = Row {
                dataset: "lagrangian-products".to_string(),
                name: format!("pair_{n1}x{n2}_{angle_deg:.0}deg"),
                algorithm: "billiard".to_string(),
                facet_count: n1 + n2,
                capacity: result.capacity,
                capacity_uncertain: result.capacity_uncertain,
                numerical_gap: gap,
                has_unknown,
                beta_min,
                volume: vol,
                sys,
                time_ms,
            };
            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total_rows += 1;
        }

        println!("  pair ({n1},{n2}): {} angles processed", angles.len());
    }

    // -----------------------------------------------------------------------
    // Summary
    // -----------------------------------------------------------------------
    writer.flush().expect("flush output");
    let total_time = t0.elapsed().as_secs_f64();

    println!("\n=== Summary ===\n");
    println!("Total polytopes:  {total_rows}");
    println!("UNKNOWNs found:   {total_unknowns}");
    println!("Total time:       {total_time:.1}s");
    println!("Output:           {}", output_path.display());

    if total_unknowns == 0 {
        println!("\nResult: Algorithm is empirically exact at f64 precision.");
        println!("No UNKNOWN predicates appeared across the full dataset.");
        println!("Phase 2 (high-precision re-solve) is NOT needed.");
    } else {
        println!("\nResult: {total_unknowns} UNKNOWN predicate(s) found.");
        println!("Phase 2 (high-precision re-solve) is needed for affected polytopes.");
        println!("See JSONL for details (has_unknown=true entries).");
    }
}

fn sweep_angles(start_deg: f64, end_deg: f64, step_deg: f64) -> Vec<f64> {
    let mut angles = Vec::new();
    let mut angle = start_deg;
    while angle <= end_deg + 1e-9 {
        angles.push(angle);
        angle += step_deg;
    }
    if (angles.last().unwrap_or(&start_deg) - end_deg).abs() > 1e-9 {
        angles.push(end_deg);
    }
    angles
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
