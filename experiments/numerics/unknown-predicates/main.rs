//! Phase 1: Check whether UNKNOWN admissibility predicates appear in practice.
//!
//! Goal: Measure whether practical datasets encounter UNKNOWN admissibility
//! predicates or near-boundary beta/action intervals.
//! Input Artifacts: None (regenerates the comparison datasets internally from hardcoded plans).
//! Output Artifacts: experiments/numerics/unknown-predicates/unknown-predicates.jsonl
//!
//! Regenerates the random-sample and lagrangian-products datasets using the same
//! seeds/parameters, then records the explicit-search action interval and
//! admissibility margin for each polytope. If the HK2017 interval is nontrivial
//! or the returned orbits have a tiny beta margin, the explicit path is near an
//! UNKNOWN admissibility boundary and Phase 2 (high-precision re-solve) is needed.
//!
//! Architecture:
//! 1. `cargo run -p dev-numerical-analysis --release --bin num-unknown-predicates -- --smoke`
//!    writes `unknown-predicates/unknown-predicates-smoke.jsonl`.
//! 2. `cargo run -p dev-numerical-analysis --release --bin num-unknown-predicates`
//!    generates the full dataset and writes `unknown-predicates/unknown-predicates.jsonl`.
//! 3. Python script reads JSONL, summarizes findings

use dev_numerical_analysis::euclidean_volume_f64;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
use symplectic::random::generate_random_polytopes;
use symplectic::{ehz_capacity_billiard, ehz_capacity_pruned};

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
const SMOKE_RANDOM_PLAN: &[(usize, usize)] = &[(5, 1)];

// ---------------------------------------------------------------------------
// Lagrangian-products parameters (must match main.rs in lagrangian-products/ exactly)
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
const SMOKE_PAIRS: &[(usize, usize)] = &[(3, 3)];

// ---------------------------------------------------------------------------
// Output schema
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct Row {
    dataset: String,
    name: String,
    algorithm: String,
    facet_count: usize,
    min_action: f64,
    min_action_lower: f64,
    min_action_upper: f64,
    beta_min: f64,
    has_unknown: bool,
    volume: f64,
    sys: f64,
    time_ms: f64,
}

const BETA_MARGIN_TAU: f64 = 1e-12;
const ACTION_INTERVAL_TAU: f64 = 1e-12;

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: num-unknown-predicates [options]

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
        manifest_dir.join("unknown-predicates/unknown-predicates-smoke.jsonl")
    } else {
        manifest_dir.join("unknown-predicates/unknown-predicates.jsonl")
    }
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let random_plan = if args.smoke {
        SMOKE_RANDOM_PLAN
    } else {
        RANDOM_PLAN
    };
    let pair_plan = if args.smoke { SMOKE_PAIRS } else { PAIRS };

    let output_path = output_path(std::path::Path::new(env!("CARGO_MANIFEST_DIR")), args.smoke);
    let file = File::create(&output_path).unwrap_or_else(|err| {
        panic!(
            "failed to create output file {}: {err}",
            output_path.display()
        )
    });
    let mut writer = BufWriter::new(file);

    let mut total_rows = 0usize;
    let mut total_unknowns = 0usize;

    // -----------------------------------------------------------------------
    // Part 1: Random-sweep polytopes (explicit pruned HK2017 path)
    // -----------------------------------------------------------------------
    println!("=== Part 1: Random-sweep polytopes ===\n");

    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED);

    for &(facet_count, n_samples) in random_plan {
        let polytopes =
            generate_random_polytopes(n_samples, facet_count, RANDOM_H_MIN, RANDOM_H_MAX, &mut rng);

        for (i, p) in polytopes.iter().enumerate() {
            let vol = euclidean_volume_f64(p.vertices(), p.incidence());

            let start = Instant::now();
            let result = ehz_capacity_pruned(p).expect("ehz_capacity_pruned failed");
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let beta_min = result
                .orbits
                .iter()
                .map(|orbit| orbit.beta_margin)
                .fold(f64::INFINITY, f64::min);
            let has_unknown = (result.min_action_upper - result.min_action_lower)
                > ACTION_INTERVAL_TAU
                || beta_min <= BETA_MARGIN_TAU;
            let sys = result.min_action * result.min_action / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: random_F{facet_count}_{i}: action=[{:.8}, {:.8}], \
                     beta_min={beta_min:.2e}",
                    result.min_action_lower, result.min_action_upper
                );
            }

            let row = Row {
                dataset: "random-sweep".to_string(),
                name: format!("random_F{facet_count}_{i}"),
                algorithm: "ehz_pruned".to_string(),
                facet_count,
                min_action: result.min_action,
                min_action_lower: result.min_action_lower,
                min_action_upper: result.min_action_upper,
                beta_min,
                has_unknown,
                volume: vol,
                sys,
                time_ms,
            };
            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total_rows += 1;
        }

        println!("  F={facet_count:2}: {n_samples} polytopes processed");
    }

    // -----------------------------------------------------------------------
    // Part 2: Lagrangian-products — pentagon 5×5 sweep (ehz_capacity_billiard)
    // -----------------------------------------------------------------------
    println!("\n=== Part 2: Lagrangian-products ===\n");

    {
        let steps = if args.smoke {
            0
        } else {
            ((PENTAGON_END_DEG - PENTAGON_START_DEG) / PENTAGON_STEP_DEG).round() as usize
        };
        let (qn, qh) = regular_polygon_2d(5, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);

        for i in 0..=steps {
            let angle_deg = PENTAGON_START_DEG + PENTAGON_STEP_DEG * (i as f64);
            let theta = angle_deg.to_radians();

            let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                .expect("pentagon product construction failed");

            let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());

            let start = Instant::now();
            let result = ehz_capacity_billiard(&polytope).unwrap_or_else(|err| {
                panic!("billiard capacity failed for pentagon_5x5_{angle_deg:.0}deg: {err}")
            });
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let min_action = result.capacity();
            let beta_min = result
                .best_beta()
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let has_unknown = beta_min <= BETA_MARGIN_TAU;
            let sys = min_action * min_action / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: pentagon_5x5_{angle_deg:.0}deg: action={min_action:.8}, \
                     beta_min={beta_min:.2e}"
                );
            }

            let row = Row {
                dataset: "lagrangian-products".to_string(),
                name: format!("pentagon_5x5_{angle_deg:.0}deg"),
                algorithm: "billiard".to_string(),
                facet_count: 10,
                min_action,
                min_action_lower: min_action,
                min_action_upper: min_action,
                beta_min,
                has_unknown,
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

    for &(n1, n2) in pair_plan {
        let end_deg = 180.0 / lcm(n1, n2) as f64;
        let mut angles = sweep_angles(0.0, end_deg, PAIR_STEP_DEG);
        if args.smoke {
            angles.truncate(1);
        }

        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);

        for angle_deg in &angles {
            let theta = angle_deg.to_radians();
            let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                .expect("polygon product construction failed");

            let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());

            let start = Instant::now();
            let result = ehz_capacity_billiard(&polytope).unwrap_or_else(|err| {
                panic!("billiard capacity failed for pair_{n1}x{n2}_{angle_deg:.0}deg: {err}")
            });
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let min_action = result.capacity();
            let beta_min = result
                .best_beta()
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min);
            let has_unknown = beta_min <= BETA_MARGIN_TAU;
            let sys = min_action * min_action / (2.0 * vol);

            if has_unknown {
                total_unknowns += 1;
                eprintln!(
                    "  UNKNOWN: pair_{n1}x{n2}_{angle_deg:.0}deg: action={min_action:.8}, \
                     beta_min={beta_min:.2e}"
                );
            }

            let row = Row {
                dataset: "lagrangian-products".to_string(),
                name: format!("pair_{n1}x{n2}_{angle_deg:.0}deg"),
                algorithm: "billiard".to_string(),
                facet_count: n1 + n2,
                min_action,
                min_action_lower: min_action,
                min_action_upper: min_action,
                beta_min,
                has_unknown,
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

    writer.flush().expect("flush output");
    let total_time = t0.elapsed().as_secs_f64();

    println!("\n=== Summary ===\n");
    println!("Total polytopes:  {total_rows}");
    println!("UNKNOWNs found:   {total_unknowns}");
    println!("Total time:       {total_time:.1}s");
    println!("Output:           {}", output_path.display());

    if total_unknowns == 0 {
        println!("\nResult: Algorithm is empirically exact at f64 precision.");
        println!("No borderline admissibility cases appeared across the full dataset.");
        println!("Phase 2 (high-precision re-solve) is NOT needed.");
    } else {
        println!("\nResult: {total_unknowns} borderline admissibility case(s) found.");
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
