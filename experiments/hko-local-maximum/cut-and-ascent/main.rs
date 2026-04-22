//! Cut-and-ascent on HKO2024: add a facet (F=10→F=11), then run gradient ascent.
//!
//! Tests whether HKO2024 is a local maximum in the F=11 polytope space.
//! The facet-splitting experiment showed 536/536 cuts decrease sys, but
//! did not run gradient ascent afterward. This experiment closes that gap.
//!
//! Algorithm: for each random direction n on S³, add a barely-non-redundant
//! facet a_{F+1} = n / (h_K(n) - ε), then run gradient ascent with overshoot
//! and wiggle escape. Same ascent algorithm as gradient-ascent-general.
//!
//! Usage: cargo run -p exp-hko-local-maximum --release --bin hko-cut-and-ascent
//! Flags: --fresh  (clear existing data and rerun)
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: cut-and-ascent/cut-and-ascent.jsonl

mod ascent;
mod sampling;

use ascent::{compute_sys, full_ascent};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use sampling::{
    add_facet, dvs_to_array, last_facet_active, load_completed_names, random_direction,
};
use serde::Serialize;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::geom::known_polytopes;

/// Reproducible RNG seed.
const SEED: u64 = 44;

/// Number of random facet placements to test.
/// 5 preliminary trials (2026-04-04) showed 0/5 improved. Increase to
/// 50-100 for statistical confidence.
const N_PLACEMENTS: usize = 20;
const SMOKE_N_PLACEMENTS: usize = 1;

/// Depth parameter for facet addition: a_{F+1} = n / (h_K(n) - ε).
/// 1e-3 used in facet-splitting experiment (SPLITTING_EPSILONS range
/// [1e-3, 1e-4]). If changed, verify Polytope4D construction doesn't
/// produce RedundantFacet errors at smaller ε.
const FACET_EPSILON: f64 = 1e-3;

/// Maximum gradient ascent iterations per phase.
pub(crate) const MAX_ITERATIONS: usize = 30;

/// Minimum improvement per iteration to continue.
pub(crate) const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Step fractions of t_max for within-bound line search.
pub(crate) const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Multipliers beyond t_max for crossing combinatorial boundaries.
pub(crate) const OVERSHOOT_MULTIPLIERS: &[f64] = &[1.5, 2.0, 3.0];

/// Prevents pathological steps when t_max is huge.
pub(crate) const MAX_STEP_SIZE: f64 = 100.0;

/// Number of random dual-vertex perturbations per escape round.
pub(crate) const N_WIGGLES: usize = 5;

/// Multiplicative perturbation scale for dual vertex components.
pub(crate) const WIGGLE_STRENGTH: f64 = 0.05;

/// Maximum rounds of escape attempts after convergence.
pub(crate) const MAX_ESCAPE_ROUNDS: usize = 3;

/// Per-trial time budget. 180s for F=11.
const TRIAL_TIME_BUDGET_SECS: f64 = 180.0;
const SMOKE_TRIAL_TIME_BUDGET_SECS: f64 = 8.0;

/// Numerical zero threshold for gradient norms, rates, and slack comparisons.
pub(crate) const EPS: f64 = 1e-15;

#[derive(Debug, Serialize)]
struct ResultRow {
    name: String,
    placement_direction: [f64; 4],
    epsilon: f64,
    hko_sys: f64,
    sys_after_cut: f64,
    final_sys: f64,
    delta_vs_hko: f64,
    n_iterations: usize,
    n_phases: usize,
    facet_remained_active: bool,
    total_time_ms: f64,
    final_dual_vertices: Vec<[f64; 4]>,
}

#[derive(Debug, Clone, Copy)]
struct Args {
    fresh: bool,
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-cut-and-ascent [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --fresh              Clear and rerun output file before sampling.
  --smoke              Run one-sample smoke mode."#
    );
}

fn usage_error(message: String) -> ! {
    eprintln!("error: {message}\n");
    print_usage();
    std::process::exit(2);
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        fresh: false,
        smoke: false,
    };

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--fresh" => {
                args.fresh = true;
                i += 1;
            }
            "--smoke" => {
                args.smoke = true;
                i += 1;
            }
            arg => usage_error(format!("unknown argument: {arg}")),
        }
    }

    args
}

fn main() {
    let t_global = Instant::now();
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("cut-and-ascent");
    let args = parse_args();
    let output_path = if args.smoke {
        base.join("cut-and-ascent-smoke.jsonl")
    } else {
        base.join("cut-and-ascent.jsonl")
    };

    println!("cut-and-ascent: facet addition + gradient ascent on HKO2024\n");

    std::fs::create_dir_all(&base).expect("create output dir");

    let completed = if args.smoke {
        HashSet::new()
    } else if args.fresh {
        let _ = std::fs::remove_file(&output_path);
        HashSet::new()
    } else {
        load_completed_names(&output_path)
    };

    if completed.is_empty() {
        println!("Starting fresh run.");
    } else {
        println!("Resuming: {} trials already completed.", completed.len());
    }

    let output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("open output JSONL");
    let mut writer = BufWriter::new(output_file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let n_placements = if args.smoke {
        SMOKE_N_PLACEMENTS
    } else {
        N_PLACEMENTS
    };
    let trial_budget = if args.smoke {
        SMOKE_TRIAL_TIME_BUDGET_SECS
    } else {
        TRIAL_TIME_BUDGET_SECS
    };

    let hko = known_polytopes::hko_pentagon();
    let hko_polytope = &hko.polytope;
    let hko_sys = compute_sys(hko_polytope).expect("HKO2024 sys");
    println!(
        "HKO2024: sys={hko_sys:.6}, F={}\n",
        hko_polytope.facet_count()
    );

    let mut n_improved = 0usize;
    let mut n_total = 0usize;

    for i in 0..n_placements {
        let trial_name = if args.smoke {
            "smoke".to_string()
        } else {
            format!("hko_p{i}")
        };
        if completed.contains(&trial_name) {
            continue;
        }

        let t0 = Instant::now();
        let dir = random_direction(&mut rng);

        let f11_polytope = match add_facet(hko_polytope, &dir, FACET_EPSILON) {
            Some(p) => p,
            None => {
                println!("[{trial_name}] facet addition failed");
                continue;
            }
        };

        let sys_after_cut = match compute_sys(&f11_polytope) {
            Some(s) => s,
            None => {
                println!("[{trial_name}] sys computation failed after cut");
                continue;
            }
        };

        match full_ascent(&f11_polytope, &mut rng, trial_budget) {
            Some(result) => {
                let delta = result.final_sys - hko_sys;
                n_total += 1;
                let improved = delta > CONVERGENCE_THRESHOLD;
                if improved {
                    n_improved += 1;
                }

                let active = last_facet_active(&result.final_polytope);

                let row = ResultRow {
                    name: trial_name.clone(),
                    placement_direction: [dir[0], dir[1], dir[2], dir[3]],
                    epsilon: FACET_EPSILON,
                    hko_sys,
                    sys_after_cut,
                    final_sys: result.final_sys,
                    delta_vs_hko: delta,
                    n_iterations: result.n_iters,
                    n_phases: result.n_phases,
                    facet_remained_active: active,
                    total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                    final_dual_vertices: dvs_to_array(&result.final_polytope),
                };

                serde_json::to_writer(&mut writer, &row).expect("write row");
                writeln!(writer).expect("newline");

                let marker = if improved { " *** IMPROVED ***" } else { "" };
                println!(
                    "[{trial_name}] cut={sys_after_cut:.6} → final={:.6} (Δ={delta:+.6}), \
                     active={active}, {:.1}s{marker}",
                    result.final_sys,
                    t0.elapsed().as_secs_f64(),
                );

                if improved {
                    eprintln!(
                        "*** HKO2024 IMPROVEMENT: {} sys={:.6} > {:.6} ***",
                        trial_name, result.final_sys, hko_sys
                    );
                }
            }
            None => {
                println!("[{trial_name}] gradient ascent failed");
            }
        }
    }

    writer.flush().expect("flush output");

    println!("\n========================================");
    println!("Improved: {n_improved}/{n_total}");
    println!("Total time: {:.1}s", t_global.elapsed().as_secs_f64());
    println!("Output: {}", output_path.display());
}
