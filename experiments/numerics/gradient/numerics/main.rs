//! First-order prediction test for analytical gradients (Q1 generic + Q2 non-generic).
//!
//! Goal: Validate first-order gradient predictions on generic and symmetric
//! non-generic polytopes.
//! Input Artifacts: None (generates all test polytopes internally).
//! Output Artifacts: experiments/numerics/gradient/numerics/gradient-correctness-q1-generic.jsonl
//!         experiments/numerics/gradient/numerics/gradient-correctness-q2-nongeneric.jsonl
//!
//! Tests the defining property of a gradient: f(a+td) - f(a) - t*g*d = o(t).
//! The residual r(t) = |f(a+td) - f(a) - t*g*d| should decrease as t -> 0.
//! The log-log slope of r(t) vs t reveals smoothness: slope ~ 2 for C^2.
//!
//! Q1: Generic random polytopes -- convergence rates, dimension scaling
//! Q2: Non-generic geometry -- Lagrangian products with symmetry-degenerate orbits
//!
//! Split from gradient-validation/main.rs (Q1-Q4 shared the first_order_test framework;
//! Q1+Q2 are the basic validation cases with no special polytope construction).
//!
//! Methodology:
//! - For each polytope, compute base values and analytical gradients
//! - Sample random directions d in R^{4F} (unit vectors via Muller's method)
//! - Sweep perturbation size t geometrically from 1e-1 to 1e-7
//! - For capacity: flat dual-vertex KKT solve with the base orbit on the perturbed polytope
//! - For volume: euclidean_volume_f64() of the perturbed polytope
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
//! 1. `cargo run --release --bin gradient-basic-validation` -> JSONL files
//! 2. Python analyze.py -> convergence plots and slope analysis
//!
//! Self-contained: generates all polytopes internally.

#[path = "../src/flat_polytope.rs"]
mod flat_polytope;

use crate::flat_polytope::GradientPolytopeCache;
use dev_gradient::{analyze_polytope, first_order_test, write_rows, PolytopeInfo};
use euclidean_polytopes::sample_random_dual_vertices_f64;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::{lagrangian_product, regular_polygon_2d, rotate_polygon_2d};

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

/// Q1: polytopes per facet count. 20 gives 600 traces total (6 F-values x 20 x
/// 5 dirs), enough for stable slope medians. Runtime scales linearly.
const Q1_POLYTOPES_PER_F: usize = 20;

/// Skip Q2 polytopes with F > this to avoid slow ehz_capacity calls.
/// LP(5,5) has F=10 (~3 min per ehz_capacity call in v1). F<=8 is tractable.
const MAX_FACET_Q2: usize = 8;

/// Smoke-test settings for Phase 2.
const SMOKE_Q1_FACET_COUNTS: &[usize] = &[5];
const SMOKE_Q1_POLYTOPES_PER_F: usize = 1;
const SMOKE_Q2_REGULAR_PAIRS: &[(usize, usize)] = &[(3, 3)];
const SMOKE_Q2_ROTATION_ANGLES: &[f64] = &[PI / 7.0];
const SMOKE_Q2_RANDOM_PAIRS: &[(usize, usize)] = &[(3, 3)];
const SMOKE_Q2_RANDOM_PER_PAIR: usize = 1;
const SMOKE_N_DIRS: usize = 1;

struct BasicValidationConfig {
    q1_facet_counts: &'static [usize],
    q1_polytopes_per_f: usize,
    q2_regular_pairs: &'static [(usize, usize)],
    q2_rotation_angles: &'static [f64],
    q2_random_pairs: &'static [(usize, usize)],
    q2_random_per_pair: usize,
    n_dirs: usize,
}

fn analyze_cached_polytope(cache: &GradientPolytopeCache) -> Option<PolytopeInfo> {
    analyze_polytope(
        &cache.dual_vertices,
        &cache.vertices,
        &cache.dual_vertices_f64,
        &cache.vertices_f64,
        &cache.vertex_facet_incidence,
        &cache.facet_intersection_is_nonempty,
        &cache.omega_signs,
    )
}

fn basic_validation_config(smoke: bool) -> BasicValidationConfig {
    if smoke {
        BasicValidationConfig {
            q1_facet_counts: SMOKE_Q1_FACET_COUNTS,
            q1_polytopes_per_f: SMOKE_Q1_POLYTOPES_PER_F,
            q2_regular_pairs: SMOKE_Q2_REGULAR_PAIRS,
            q2_rotation_angles: SMOKE_Q2_ROTATION_ANGLES,
            q2_random_pairs: SMOKE_Q2_RANDOM_PAIRS,
            q2_random_per_pair: SMOKE_Q2_RANDOM_PER_PAIR,
            n_dirs: SMOKE_N_DIRS,
        }
    } else {
        BasicValidationConfig {
            q1_facet_counts: &[5, 6, 7, 8, 9, 10],
            q1_polytopes_per_f: Q1_POLYTOPES_PER_F,
            q2_regular_pairs: &[(3, 3), (3, 4), (4, 4), (3, 5), (4, 5), (5, 5)],
            q2_rotation_angles: &[PI / 7.0, PI / 5.0, PI / 3.0],
            q2_random_pairs: &[(3, 3), (3, 4), (4, 4), (5, 5)],
            q2_random_per_pair: 5,
            n_dirs: N_DIRS,
        }
    }
}

fn sample_random_cache_batch(
    count: usize,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut ChaCha8Rng,
) -> Vec<GradientPolytopeCache> {
    let mut accepted = Vec::with_capacity(count);
    while accepted.len() < count {
        let dual_vertices_f64 = sample_random_dual_vertices_f64(facet_count, h_min, h_max, rng);
        if let Some(cache) = GradientPolytopeCache::from_f64(dual_vertices_f64) {
            accepted.push(cache);
        }
    }
    accepted
}

// ============================================================================
// Phases
// ============================================================================

fn run_q1(base_dir: &str, cfg: &BasicValidationConfig) {
    let path = format!("{}/gradient-correctness-q1-generic.jsonl", base_dir);
    let file = File::create(&path).expect("create Q1 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    for &f_count in cfg.q1_facet_counts {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + f_count as u64);
        let polytopes =
            sample_random_cache_batch(cfg.q1_polytopes_per_f, f_count, 0.5, 2.0, &mut rng);

        for (i, cache) in polytopes.iter().enumerate() {
            let info = match analyze_cached_polytope(cache) {
                Some(info) => info,
                None => {
                    eprintln!("  Q1: F={} polytope {} — failed, skipping", f_count, i);
                    continue;
                }
            };

            let id = format!("generic_F{}_{:03}", f_count, i);
            let rows = first_order_test(
                &info, "q1", &id, "random", cfg.n_dirs, &mut rng, None, None, None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();

            if (i + 1) % 5 == 0 {
                println!(
                    "  Q1: F={} — {}/{} polytopes done",
                    f_count,
                    i + 1,
                    Q1_POLYTOPES_PER_F
                );
            }
        }
    }

    writer.flush().expect("flush Q1");
    println!("Q1 done: {} rows written to {}", total_rows, path);
}

fn run_q2(base_dir: &str, cfg: &BasicValidationConfig) {
    let path = format!("{}/gradient-correctness-q2-nongeneric.jsonl", base_dir);
    let file = File::create(&path).expect("create Q2 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 200);

    // Regular Lagrangian products
    for &(n1, n2) in cfg.q2_regular_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            println!(
                "  Q2: skipping LP({},{}) — F={} > {}",
                n1,
                n2,
                n1 + n2,
                MAX_FACET_Q2
            );
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn, ph) = regular_polygon_2d(n2, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).expect("regular LP");
        let cache = GradientPolytopeCache::from_f64(polytope).expect("regular LP cache");
        let id = format!("lp_regular_{}_{}", n1, n2);

        if let Some(info) = analyze_cached_polytope(&cache) {
            let rows = first_order_test(
                &info,
                "q2",
                &id,
                "lagrangian_regular",
                cfg.n_dirs,
                &mut rng,
                None,
                None,
                None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        } else {
            eprintln!("  Q2: regular LP({},{}) — failed", n1, n2);
        }
    }

    // Rotated Lagrangian products
    for &(n1, n2) in cfg.q2_regular_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        for (ai, &theta) in cfg.q2_rotation_angles.iter().enumerate() {
            let (pn, ph) = regular_polygon_2d(n2, 1.0);
            let (pn_rot, ph_rot) = rotate_polygon_2d(&pn, &ph, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn_rot, &ph_rot).expect("rotated LP");
            let cache = GradientPolytopeCache::from_f64(polytope).expect("rotated LP cache");
            let id = format!("lp_rotated_{}_{}_{}", n1, n2, ai);

            if let Some(info) = analyze_cached_polytope(&cache) {
                let rows = first_order_test(
                    &info,
                    "q2",
                    &id,
                    "lagrangian_rotated",
                    cfg.n_dirs,
                    &mut rng,
                    None,
                    None,
                    None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: rotated LP({},{},θ={:.3}) — failed", n1, n2, theta);
            }
        }
    }

    // Random Lagrangian products
    for &(n1, n2) in cfg.q2_random_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        for j in 0..cfg.q2_random_per_pair {
            let (qn, qh) = random_polygon_2d(n1, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(n2, 0.5, 2.0, &mut rng);
            let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(
                        "  Q2: random LP({},{},{}) — construction: {:?}",
                        n1, n2, j, e
                    );
                    continue;
                }
            };
            let cache = GradientPolytopeCache::from_f64(polytope).expect("random LP cache");
            let id = format!("lp_random_{}_{}_{:02}", n1, n2, j);

            if let Some(info) = analyze_cached_polytope(&cache) {
                let rows = first_order_test(
                    &info,
                    "q2",
                    &id,
                    "lagrangian_random",
                    cfg.n_dirs,
                    &mut rng,
                    None,
                    None,
                    None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: random LP({},{},{}) — failed", n1, n2, j);
            }
        }
    }

    writer.flush().expect("flush Q2");
    println!("Q2 done: {} rows written to {}", total_rows, path);
}

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
    eprintln!(
        "Usage: cargo run -p dev-gradient --release --bin gradient-basic-validation [--smoke]"
    );
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

// Main
// ============================================================================

fn main() {
    let smoke = smoke_mode();
    let cfg = basic_validation_config(smoke);
    let base_dir = if smoke {
        let smoke_dir = smoke_output_dir("dev-numerics-smoke");
        println!("Smoke output: {smoke_dir}");
        smoke_dir
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("numerics")
            .to_string_lossy()
            .into_owned()
    };

    println!(
        "=== Gradient Correctness: Basic Validation (Q1 + Q2){} ===\n",
        if smoke { " [smoke]" } else { "" }
    );
    let t0 = Instant::now();

    println!("--- Q1: Generic random polytopes ---");
    let tp = Instant::now();
    run_q1(&base_dir, &cfg);
    println!("  Q1 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("--- Q2: Non-generic geometry (Lagrangian products) ---");
    let tp = Instant::now();
    run_q2(&base_dir, &cfg);
    println!("  Q2 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}
