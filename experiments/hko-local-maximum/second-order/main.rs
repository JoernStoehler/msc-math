//! Second-order analysis of flat directions at HKO2024.
//!
//! Goal: Test whether the first-order-flat directions at HKO2024 remain
//! descending at second order, both for basis directions and random flat probes.
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: experiments/hko-local-maximum/second-order/second-order-base.jsonl
//!         experiments/hko-local-maximum/second-order/second-order-curves.jsonl
//!         experiments/hko-local-maximum/second-order/second-order-random.jsonl
//!
//! Phase 1: Compute per-orbit ∇_{a_i} sys in R^40 for all near-optimal orbits,
//!          build gradient matrix G, SVD → flat directions (null space of G).
//! Phase 2: For each flat direction d, evaluate sys(HKO + ε·d) on the 28 nonzero
//!          ±ε points from `EPSILON_GRID`; the base point lives in
//!          `second-order-base.jsonl`.
//!
//! Replaces the broken Phase C LP (subdifferential-lp/phase_c_lp_test.py) with
//! clean a_i-space computation. The old script reads normals/heights fields that
//! no longer exist in the JSONL after the a_i migration.
//!
//! Mathematical basis: Danskin's theorem gives D_d⁺ sys = min_i (∇sys_i · d).
//! Flat directions d satisfy ∇sys_i · d = 0 for all active orbits i.
//! Second-order analysis: if sys(K + εd) < sys(K) for all ε ≠ 0 and all flat d,
//! then K is a strict local maximum. See formal/hko-local-maximality-conditions.tex for formal statement.

mod curvature;
mod phase1;

use curvature::{curvature_at_epsilon, run_phase2, run_phase3};
use phase1::run_phase1;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::geom::known_polytopes;
use symplectic::geom::volume::volume;

/// Gap threshold for near-optimal orbits.
pub(crate) const NEAR_OPTIMAL_GAP: f64 = 1e-10;

/// SVD rank threshold.
pub(crate) const SVD_RANK_THRESHOLD: f64 = 1e-8;

/// Epsilon grid for finite-difference curves.
pub(crate) const EPSILON_GRID: &[f64] = &[
    5e-5, 1e-4, 2e-4, 5e-4, 1e-3, 2e-3, 5e-3, 1e-2, 1.5e-2, 2e-2, 2.5e-2, 3e-2, 3.5e-2, 4e-2,
];

/// Number of random directions in ker(G) to sample for negative-definiteness check.
pub(crate) const N_RANDOM_DIRECTIONS: usize = 100;

/// Epsilon values for the random-direction curvature check.
pub(crate) const EPSILON_RANDOM: &[f64] = &[1e-4, 5e-4, 1e-3, 5e-3];

/// RNG seed for reproducibility.
pub(crate) const RANDOM_SEED: u64 = 42;

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-second-order [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke              Run smoke mode and exit after phase 1 probe."#
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
            "--smoke" => {
                args.smoke = true;
            }
            other => usage_error(format!("unknown argument: {other}")),
        }
    }

    args
}

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = base_dir.join("second-order");
    let args = parse_args();

    println!("═══════════════════════════════════════════════════════════");
    println!("Second-order analysis of flat directions at HKO2024");
    println!("═══════════════════════════════════════════════════════════\n");

    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    println!("HKO2024: F={}, known sys≈{:.6}", polytope.facet_count(), {
        let v = volume(polytope);
        known.capacity * known.capacity / (2.0 * v)
    });

    println!("\n--- Phase 1: Gradient matrix and flat directions ---");
    let t_phase1 = Instant::now();
    let (mut base_row, flat_directions) = run_phase1(polytope);
    base_row.time_phase1_ms = t_phase1.elapsed().as_secs_f64() * 1000.0;
    println!("  Phase 1 time: {:.0}ms", base_row.time_phase1_ms);

    let sys_diff =
        (base_row.sys_base - known.capacity * known.capacity / (2.0 * base_row.volume_base)).abs();
    assert!(
        sys_diff < 1e-8,
        "sys_base mismatch: computed={:.10}, expected={:.10}",
        base_row.sys_base,
        known.capacity * known.capacity / (2.0 * base_row.volume_base)
    );

    std::fs::create_dir_all(&out_dir).expect("create output dir");

    if args.smoke {
        if let Some(direction) = flat_directions.first() {
            let eps = EPSILON_GRID[0];
            let curv = curvature_at_epsilon(polytope, direction, eps, base_row.sys_base)
                .expect("smoke curvature probe failed");
            println!("  Smoke curvature at ε={eps:.1e}: {curv:.6e}");
        } else {
            println!("\n  Smoke mode: no flat directions, exiting after phase 1.");
        }
        println!("\n═══════════════════════════════════════════════════════════");
        println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
        println!("═══════════════════════════════════════════════════════════");
        return;
    }

    let base_path = out_dir.join("second-order-base.jsonl");
    let base_file = File::create(&base_path).expect("create base JSONL");
    let mut base_writer = BufWriter::new(base_file);
    serde_json::to_writer(&mut base_writer, &base_row).expect("write base row");
    writeln!(base_writer).expect("newline");
    base_writer.flush().expect("flush base");
    println!("  Wrote {}", base_path.display());

    if flat_directions.is_empty() {
        println!("\n  No flat directions — 0 ∈ interior of conv(gradients).");
        println!("  HKO2024 is a strict first-order local max. No second-order analysis needed.");
    } else {
        println!(
            "\n--- Phase 2: Finite-difference curves along {} flat directions ---",
            flat_directions.len()
        );
        let t_phase2 = Instant::now();

        let curves_path = out_dir.join("second-order-curves.jsonl");
        let curves_file = File::create(&curves_path).expect("create curves JSONL");
        let mut curves_writer = BufWriter::new(curves_file);

        run_phase2(
            polytope,
            base_row.sys_base,
            &flat_directions,
            &mut curves_writer,
        );

        curves_writer.flush().expect("flush curves");
        println!("  Phase 2 time: {:.1}s", t_phase2.elapsed().as_secs_f64());
        println!("  Wrote {}", curves_path.display());

        println!(
            "\n--- Phase 3: Random directions in flat subspace ({} samples) ---",
            N_RANDOM_DIRECTIONS
        );
        let t_phase3 = Instant::now();

        let random_path = out_dir.join("second-order-random.jsonl");
        let random_file = File::create(&random_path).expect("create random JSONL");
        let mut random_writer = BufWriter::new(random_file);

        run_phase3(
            polytope,
            base_row.sys_base,
            &flat_directions,
            &mut random_writer,
        );

        random_writer.flush().expect("flush random");
        println!("  Phase 3 time: {:.1}s", t_phase3.elapsed().as_secs_f64());
        println!("  Wrote {}", random_path.display());
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
    println!("═══════════════════════════════════════════════════════════");
}
