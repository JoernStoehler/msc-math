//! First-order prediction test for analytical gradients (Q3 near-degeneracy + Q4 barely-cutting).
//!
//! Goal: Validate first-order gradient predictions near degeneracy and near
//! redundant-facet boundaries.
//! Input Artifacts: None (generates all test polytopes internally).
//! Output Artifacts: experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q3-degeneracy.jsonl
//!         experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q4-redundant.jsonl
//!
//! Tests the defining property of a gradient: f(a+td) - f(a) - t*g*d = o(t).
//! The residual r(t) = |f(a+td) - f(a) - t*g*d| should decrease as t -> 0.
//! The log-log slope of r(t) vs t reveals smoothness: slope ~ 2 for C^2.
//!
//! Q3: Near-degeneracy -- small action gap between best and second-best orbit
//! Q4: Barely-cutting facets -- near-redundant halfspaces
//!
//! Split from gradient-validation/main.rs (Q1-Q4 shared the first_order_test framework;
//! Q3+Q4 test edge cases with special polytope construction).
//!
//! Methodology:
//! - For each polytope, compute base values and analytical gradients
//! - Sample random directions d in R^{4F} (unit vectors via Muller's method)
//! - Sweep perturbation size t geometrically from 1e-1 to 1e-7
//! - For capacity: solve_kkt_for with the base orbit on the perturbed polytope
//! - For volume: volume_f64() of the perturbed polytope
//! - For sys = c^2/(2*vol): derived from perturbed cap and vol
//!
//! Mathematical correspondence:
//! - [lem:cap-derivative] (unverified): envelope theorem formula for dc/da_k.
//!   In formal/capacity-derivatives.tex.
//! - [lem:vol-derivative] (unverified): chain rule formula for dvol/da_k.
//!   In formal/capacity-derivatives.tex.
//! - [prop:capacity-piecewise-smooth] (unverified): piecewise C^inf, generic differentiability.
//!   In formal/capacity-derivatives.tex.
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient-edge-cases` -> JSONL files
//! 2. Python analyze.py -> convergence plots and slope analysis
//!
//! Self-contained: generates all polytopes internally.

use dev_gradient::{analyze_polytope, enumerate_all_orbits, first_order_test, write_rows};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::geom::facet_volume::facet_volume_3d_f64;
use symplectic::random::generate_random_polytopes;
use symplectic::Polytope4D;

// ============================================================================
// CLI helpers
// ============================================================================

fn smoke_mode() -> bool {
    let mut smoke = false;
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--smoke" => smoke = true,
            "-h" | "--help" => print_usage_and_exit(0),
            _ => {
                eprintln!("unknown argument: {arg}");
                print_usage_and_exit(2);
            }
        }
    }
    smoke
}

fn print_usage_and_exit(code: i32) -> ! {
    eprintln!("Usage: cargo run -p dev-gradient --release --bin gradient-edge-cases [--smoke]");
    eprintln!("  --smoke: run a reduced run into a temporary directory");
    eprintln!("  -h, --help: show usage");
    std::process::exit(code);
}

fn smoke_output_dir(label: &str) -> String {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{label}-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create smoke output dir");
    dir.to_string_lossy().into_owned()
}

// ============================================================================
// Constants
// ============================================================================

/// Base seed for deterministic RNG across all phases.
const SEED_BASE: u64 = 7777;

/// Number of random perturbation directions per polytope.
/// 5 directions in R^{4F} provides reasonable coverage for detecting
/// direction-dependent issues with isotropic sampling. Increasing to 10+
/// would tighten the slope distribution but 5 already gives IQR width < 0.1
/// for capacity. Decreasing below 3 risks missing direction-dependent bugs.
const N_DIRS: usize = 5;

/// Q3: max candidates to generate when filling gap bins.
/// 2000 is enough to fill all bins at F=6 (verified in v1).
const Q3_MAX_CANDIDATES: usize = 2000;

/// Q3: max polytopes per gap bin. 20 gives ~100 traces per bin (x5 dirs),
/// enough for meaningful per-bin slope statistics.
const Q3_PER_BIN: usize = 20;

/// Q4: base polytopes to augment with barely-cutting facets.
/// 10 x 5 deltas x 5 dirs = 250 traces. Runtime is fast (F=7, ~12s total).
const Q4_BASE_COUNT: usize = 10;

/// Q4: barely-cutting delta values. Range 1e-1 to 1e-5 spans from "substantial cut"
/// to "facet volume near zero". Below 1e-5, Polytope4D::from_f64 may reject as
/// degenerate.
const Q4_DELTAS: &[f64] = &[1e-1, 1e-2, 1e-3, 1e-4, 1e-5];

/// Q3: gap bins (lower_bound, upper_bound, label).
const Q3_GAP_BINS: [(f64, f64, &str); 4] = [
    (1e-1, f64::INFINITY, "large"),
    (1e-2, 1e-1, "medium"),
    (1e-4, 1e-2, "small"),
    (0.0, 1e-4, "tiny"),
];

/// Smoke-test settings for Phase 2.
const SMOKE_Q3_F_COUNT: usize = 6;
const SMOKE_Q3_MAX_CANDIDATES: usize = 30;
const SMOKE_Q3_PER_BIN: usize = 1;
const SMOKE_Q3_BATCH_SIZE: usize = 2;
const SMOKE_Q3_N_DIRS: usize = 1;
const SMOKE_Q4_F_COUNT: usize = 6;
const SMOKE_Q4_BASE_COUNT: usize = 1;
const SMOKE_Q4_DELTAS: &[f64] = &[1e-1, 1e-3];
const SMOKE_Q4_N_DIRS: usize = 1;

struct EdgeCasesConfig {
    q3_f_count: usize,
    q3_max_candidates: usize,
    q3_per_bin: usize,
    q3_batch_size: usize,
    q3_n_dirs: usize,
    q4_f_count: usize,
    q4_base_count: usize,
    q4_deltas: &'static [f64],
    q4_n_dirs: usize,
}

fn edge_cases_config(smoke: bool) -> EdgeCasesConfig {
    if smoke {
        EdgeCasesConfig {
            q3_f_count: SMOKE_Q3_F_COUNT,
            q3_max_candidates: SMOKE_Q3_MAX_CANDIDATES,
            q3_per_bin: SMOKE_Q3_PER_BIN,
            q3_batch_size: SMOKE_Q3_BATCH_SIZE,
            q3_n_dirs: SMOKE_Q3_N_DIRS,
            q4_f_count: SMOKE_Q4_F_COUNT,
            q4_base_count: SMOKE_Q4_BASE_COUNT,
            q4_deltas: SMOKE_Q4_DELTAS,
            q4_n_dirs: SMOKE_Q4_N_DIRS,
        }
    } else {
        EdgeCasesConfig {
            q3_f_count: 6,
            q3_max_candidates: Q3_MAX_CANDIDATES,
            q3_per_bin: Q3_PER_BIN,
            q3_batch_size: 10,
            q3_n_dirs: N_DIRS,
            q4_f_count: 6,
            q4_base_count: Q4_BASE_COUNT,
            q4_deltas: Q4_DELTAS,
            q4_n_dirs: N_DIRS,
        }
    }
}

// ============================================================================
// Phase-specific helpers
// ============================================================================

/// Sample a random unit vector on S^3 (for Q4 facet normal generation).
fn random_unit_s3(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    loop {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let z: f64 = StandardNormal.sample(rng);
        let w: f64 = StandardNormal.sample(rng);
        let v = Vector4::new(x, y, z, w);
        let norm = v.norm();
        if norm > 1e-10 {
            return v / norm;
        }
    }
}

/// Add a barely-cutting facet near a random vertex of the polytope.
/// The new halfspace passes delta inside the vertex. Returns None if
/// construction fails after 50 attempts.
fn add_barely_cutting_facet(
    polytope: &Polytope4D,
    delta: f64,
    rng: &mut ChaCha8Rng,
) -> Option<Polytope4D> {
    let vertices = polytope.vertices_f64();
    let duals = polytope.dual_vertices_f64();

    // 50 attempts: success rate ~80% at delta>=1e-3, ~50% at delta=1e-5 (v1 data).
    for _ in 0..50 {
        let idx = Uniform::from(0..vertices.len()).sample(rng);
        let v = &vertices[idx];
        let n = random_unit_s3(rng);
        // h = n*v - delta: hyperplane passes delta inside vertex v
        let h = n.dot(v) - delta;
        if h <= 0.0 {
            continue;
        }
        let a_new = n / h;
        let mut new_duals = duals.to_vec();
        new_duals.push(a_new);
        if let Ok(p) = Polytope4D::from_f64(new_duals) {
            return Some(p);
        }
    }
    None
}

// ============================================================================
// Phases
// ============================================================================

fn run_q3(base_dir: &str, cfg: &EdgeCasesConfig) {
    let path = format!("{}/gradient-correctness-q3-degeneracy.jsonl", base_dir);
    let file = File::create(&path).expect("create Q3 JSONL");
    let mut writer = BufWriter::new(file);
    let mut bin_counts = [0usize; 4];
    let mut total_rows = 0;
    let mut generated = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 300);
    let f_count = cfg.q3_f_count; // Small F for tractable orbit enumeration

    println!("  Q3: Generating candidates (F={})...", f_count);

    while generated < cfg.q3_max_candidates && bin_counts.iter().any(|&c| c < cfg.q3_per_bin) {
        let polytopes = generate_random_polytopes(cfg.q3_batch_size, f_count, 0.5, 2.0, &mut rng);

        for polytope in &polytopes {
            generated += 1;
            if bin_counts.iter().all(|&c| c >= cfg.q3_per_bin) {
                break;
            }

            let orbits = enumerate_all_orbits(polytope);
            if orbits.len() < 2 {
                continue;
            }

            let best_action = orbits[0].0;
            let second_action = orbits[1].0;
            let gap = second_action - best_action;

            let bin_idx = Q3_GAP_BINS
                .iter()
                .position(|&(lo, hi, _)| gap >= lo && gap < hi);
            let bin_idx = match bin_idx {
                Some(idx) if bin_counts[idx] < cfg.q3_per_bin => idx,
                _ => continue,
            };

            let info = match analyze_polytope(polytope) {
                Some(info) => info,
                None => continue,
            };

            let id = format!(
                "degeneracy_{}_{:03}",
                Q3_GAP_BINS[bin_idx].2, bin_counts[bin_idx]
            );
            let rows = first_order_test(
                &info,
                "q3",
                &id,
                "near_degenerate",
                cfg.q3_n_dirs,
                &mut rng,
                Some(gap),
                None,
                None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
            bin_counts[bin_idx] += 1;

            if generated % 100 == 0 {
                println!(
                    "  Q3: {} candidates, bins: large={}, medium={}, small={}, tiny={}",
                    generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
                );
            }
        }
    }

    writer.flush().expect("flush Q3");
    println!(
        "Q3 done: {} rows, {} candidates, bins: large={}, medium={}, small={}, tiny={}",
        total_rows, generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
    );
}

fn run_q4(base_dir: &str, cfg: &EdgeCasesConfig) {
    let path = format!("{}/gradient-correctness-q4-redundant.jsonl", base_dir);
    let file = File::create(&path).expect("create Q4 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 400);
    let f_count = cfg.q4_f_count;
    let base_polytopes = generate_random_polytopes(cfg.q4_base_count, f_count, 0.5, 2.0, &mut rng);

    for (i, base) in base_polytopes.iter().enumerate() {
        for &delta in cfg.q4_deltas {
            let augmented = match add_barely_cutting_facet(base, delta, &mut rng) {
                Some(p) => p,
                None => {
                    eprintln!("  Q4: base {} delta={:.0e} — construction failed", i, delta);
                    continue;
                }
            };

            let info = match analyze_polytope(&augmented) {
                Some(info) => info,
                None => {
                    eprintln!("  Q4: base {} delta={:.0e} — capacity failed", i, delta);
                    continue;
                }
            };

            let min_fv = (0..augmented.facet_count())
                .map(|k| facet_volume_3d_f64(&augmented, k))
                .filter(|&fv| fv > 0.0)
                .fold(f64::INFINITY, f64::min);

            let id = format!("barely_cutting_{:02}_d{:.0e}", i, delta);
            let rows = first_order_test(
                &info,
                "q4",
                &id,
                "barely_cutting",
                cfg.q4_n_dirs,
                &mut rng,
                None,
                Some(delta),
                Some(min_fv),
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        }
        println!("  Q4: base polytope {}/{} done", i + 1, cfg.q4_base_count);
    }

    writer.flush().expect("flush Q4");
    println!("Q4 done: {} rows written", total_rows);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let smoke = smoke_mode();
    let cfg = edge_cases_config(smoke);
    let base_dir = if smoke {
        let smoke_dir = smoke_output_dir("dev-numerics-edge-cases-smoke");
        println!("Smoke output: {smoke_dir}");
        smoke_dir
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("numerics-edge-cases")
            .to_string_lossy()
            .into_owned()
    };

    println!(
        "=== Gradient Correctness: Edge Cases (Q3 + Q4){} ===\n",
        if smoke { " [smoke]" } else { "" }
    );
    let t0 = Instant::now();

    println!("--- Q3: Near-degeneracy ---");
    let tp = Instant::now();
    run_q3(&base_dir, &cfg);
    println!("  Q3 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("--- Q4: Barely-cutting facets ---");
    let tp = Instant::now();
    run_q4(&base_dir, &cfg);
    println!("  Q4 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}
