//! Dense perturbation sweep around HKO2024 in Lagrangian product space.
//!
//! Goal: Estimate how the sys > 1 region around HKO2024 shrinks under dense
//! Lagrangian-product perturbations across epsilon levels.
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-lagrangian-product/lagrangian-search.jsonl
//!         experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-lagrangian-product/lagrangian-search-levels.jsonl
//!
//! Architecture:
//! 1. `cargo run -p exp-hko-local-maximum --release --bin hko-neighborhood-sampling -- m10-lagrangian-product`
//!    generates datasets
//! 2. Writes per-sample data to m10-lagrangian-product/lagrangian-search.jsonl
//! 3. Writes per-level summary to m10-lagrangian-product/lagrangian-search-levels.jsonl
//! 4. Python script analyzes and plots
//!
//! Dataset design:
//! - Base: HKO2024 (Lagrangian product of two regular pentagons at θ=18°)
//! - Perturbation: Uniform[-ε, ε] on the 2 nonzero Lagrangian components of each
//!   dual vertex (20 independent perturbation coordinates for 10 facets)
//! - Sweep ε over geometric range: 0.01 to 1.00
//! - 500 valid samples per ε level via rejection sampling
//! Explicit billiard algorithm because the output schema persists bounce counts;
//! the crate-level `ehz_capacity` entrypoint would hide that billiard-native data.

use crate::flat_polytope::HkoPolytopeCache;
use exp_hko_local_maximum::{capacity_billiard, exact_volume_reference_as_f64};
use nalgebra::Vector4;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::geom::known_polytopes;

const SEED: u64 = 42;

/// Number of valid samples to collect per epsilon level.
/// 500 × 13 levels = 6500 evals. Observed total runtime: ~7 min (2026-03-27 run).
const SAMPLES_PER_LEVEL: usize = 500;

/// Maximum attempts per epsilon level before moving on.
/// Worst observed acceptance rate in the current sweep was 21.3% at ε=1.0
/// (500/2350 in the first full run). 100K attempts leaves wide headroom for
/// the current `EPSILON_LEVELS`; re-validate if extending the sweep or changing
/// the perturbation distribution.
const MAX_ATTEMPTS_PER_LEVEL: usize = 100_000;
const SMOKE_MAX_ATTEMPTS_PER_LEVEL: usize = 128;

/// Epsilon levels: dense in the transition zone [0.02, 0.10], sparser outside.
/// 0.01 matches pentagon-perturb baseline. Dual vertex magnitudes are ~1.24,
/// so ε=1.0 is ~80% relative perturbation per component.
const EPSILON_LEVELS: &[f64] = &[
    0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.10, 0.15, 0.20, 0.50, 1.00,
];
const SMOKE_EPSILON_LEVELS: &[f64] = &[0.01];

#[derive(Debug, Serialize)]
struct SampleRow {
    epsilon: f64,
    sample_index: usize,
    is_base: bool,
    /// Perturbation in the 2D Lagrangian plane per facet.
    /// For q-facets: delta in (q₁, q₂). For p-facets: delta in (p₁, p₂).
    /// 10 entries × 2 components = 20 perturbation coordinates.
    delta_2d: Vec<[f64; 2]>,
    /// L2 norm of the full 20D perturbation vector.
    l2_norm: f64,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    capacity: f64,
    sys: f64,
    bounces: usize,
}

#[derive(Debug, Serialize)]
struct LevelRow {
    epsilon: f64,
    n_accepted: usize,
    n_attempts: usize,
    accept_rate: f64,
    n_above_1: usize,
    frac_above_1: f64,
    sys_min: f64,
    sys_mean: f64,
    sys_max: f64,
    sys_std: f64,
    time_s: f64,
}

/// Identify which 2D components are nonzero for each dual vertex.
/// Returns (i0, i1) index pairs: [0,1] for q-facets, [2,3] for p-facets.
///
/// For a Lagrangian product, each dual vertex lies entirely in q-space or p-space.
/// We classify by comparing the squared norms of the q and p components.
// TODO: add [def:lagrangian-facet-type] to formal math (trivial from the LP definition)
fn lagrangian_component_indices(duals: &[Vector4<f64>]) -> Vec<(usize, usize)> {
    duals
        .iter()
        .map(|a| {
            let q_sq = a[0] * a[0] + a[1] * a[1];
            let p_sq = a[2] * a[2] + a[3] * a[3];
            if q_sq > p_sq {
                (0, 1) // q-facet
            } else {
                (2, 3) // p-facet
            }
        })
        .collect()
}

/// Perturb dual vertices in their Lagrangian 2D plane.
/// Returns (perturbed_duals, delta_2d, l2_norm).
///
/// Only the nonzero 2D components are perturbed, so the result remains a
/// valid Lagrangian product (q-facets stay in q-space, p-facets in p-space).
// TODO: add [lem:lagrangian-perturbation-closure] to formal math
fn perturb_lagrangian(
    base: &[Vector4<f64>],
    indices: &[(usize, usize)],
    eps: f64,
    rng: &mut ChaCha8Rng,
) -> (Vec<Vector4<f64>>, Vec<[f64; 2]>, f64) {
    let mut perturbed = Vec::with_capacity(base.len());
    let mut deltas = Vec::with_capacity(base.len());
    let mut l2_sq = 0.0;

    for (a, &(i0, i1)) in base.iter().zip(indices.iter()) {
        let d0: f64 = rng.gen_range(-eps..=eps);
        let d1: f64 = rng.gen_range(-eps..=eps);
        l2_sq += d0 * d0 + d1 * d1;

        let mut v = *a;
        v[i0] += d0;
        v[i1] += d1;

        perturbed.push(v);
        deltas.push([d0, d1]);
    }

    (perturbed, deltas, l2_sq.sqrt())
}

fn v4_to_array(v: &Vector4<f64>) -> [f64; 4] {
    [v[0], v[1], v[2], v[3]]
}

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-neighborhood-sampling m10-lagrangian-product [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke              Run smoke mode with one sample and one epsilon level."#
    );
}

fn usage_error(message: String) -> ! {
    eprintln!("error: {message}\n");
    print_usage();
    std::process::exit(2);
}

fn parse_args(raw_args: &[String]) -> Args {
    let mut args = Args { smoke: false };

    for arg in raw_args {
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

pub fn run(raw_args: &[String]) {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let args = parse_args(raw_args);
    let smoke = args.smoke;

    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("empirical/neighborhood-sampling/m10-lagrangian-product");
    let samples_path = if smoke {
        base_dir.join("lagrangian-search-smoke.jsonl")
    } else {
        base_dir.join("lagrangian-search.jsonl")
    };
    let levels_path = if smoke {
        base_dir.join("lagrangian-search-levels-smoke.jsonl")
    } else {
        base_dir.join("lagrangian-search-levels.jsonl")
    };

    println!("Lagrangian search: dense perturbation sweep around HKO2024\n");

    std::fs::create_dir_all(&base_dir).unwrap_or_else(|err| {
        panic!(
            "create m10-lagrangian-product output dir {}: {err}",
            base_dir.display()
        )
    });

    let samples_file = File::create(&samples_path).unwrap_or_else(|err| {
        panic!(
            "failed to create samples file {}: {err}",
            samples_path.display()
        )
    });
    let mut samples_writer = BufWriter::new(samples_file);

    let levels_file = File::create(&levels_path).unwrap_or_else(|err| {
        panic!(
            "failed to create levels file {}: {err}",
            levels_path.display()
        )
    });
    let mut levels_writer = BufWriter::new(levels_file);

    // Base polytope
    let base = known_polytopes::hko_pentagon();
    let base_polytope =
        HkoPolytopeCache::from_rational_parts(base.dual_vertices.clone(), base.vertices.clone())
            .expect("HKO base cache");
    let base_duals: Vec<Vector4<f64>> = base_polytope.dual_vertices_f64.to_vec();
    let indices = lagrangian_component_indices(&base_duals);

    // Compute and write base row (epsilon = 0)
    let base_vol = exact_volume_reference_as_f64(
        &base_polytope.vertices,
        &base_polytope.vertex_facet_incidence,
    );
    let base_billiard = capacity_billiard(
        &base_polytope.dual_vertices,
        &base_polytope.dual_vertices_f64,
        &base_polytope.facet_intersection_is_nonempty,
        &base_polytope.omega_signs,
    )
    .expect("billiard classification failed");
    let base_classification = classify_facets_from_dual_vertices(&base_polytope.dual_vertices_f64)
        .expect("base polytope should classify as Lagrangian product");
    let base_cap = base_billiard.min_action;
    let base_sys = base_cap * base_cap / (2.0 * base_vol);
    let base_bounces = bounce_count_from_sigma_for_facets(
        &base_classification.q_indices,
        &base_classification.p_indices,
        base_billiard.best_sigma(),
    )
    .expect("bounce count returned None");

    println!(
        "Base: sys = {:.6}, cap = {:.6}, vol = {:.6}\n",
        base_sys, base_cap, base_vol
    );

    let base_row = SampleRow {
        epsilon: 0.0,
        sample_index: 0,
        is_base: true,
        delta_2d: vec![[0.0; 2]; base_duals.len()],
        l2_norm: 0.0,
        dual_vertices: base_duals.iter().map(v4_to_array).collect(),
        volume: base_vol,
        capacity: base_cap,
        sys: base_sys,
        bounces: base_bounces,
    };
    let line = serde_json::to_string(&base_row).expect("serialize");
    writeln!(samples_writer, "{line}").expect("write");

    let mut total_rows = 1usize;
    let epsilon_levels = if smoke {
        SMOKE_EPSILON_LEVELS
    } else {
        EPSILON_LEVELS
    };
    let target_samples = if smoke { 1 } else { SAMPLES_PER_LEVEL };
    let max_attempts_per_level = if smoke {
        SMOKE_MAX_ATTEMPTS_PER_LEVEL
    } else {
        MAX_ATTEMPTS_PER_LEVEL
    };

    // Header
    println!(
        "{:<8} {:>10} {:>10} {:>8} {:>10} {:>8} {:>8} {:>8} {:>6}",
        "epsilon", "accepted", "attempts", "acc%", "sys>1", "frac%", "sys_min", "sys_max", "time"
    );
    println!("{}", "-".repeat(86));

    // Sweep epsilon levels
    for &eps in epsilon_levels {
        let level_start = Instant::now();
        let mut accepted = 0usize;
        let mut attempts = 0usize;
        let mut n_above_1 = 0usize;
        let mut sys_values = Vec::with_capacity(SAMPLES_PER_LEVEL);

        while accepted < target_samples && attempts < max_attempts_per_level {
            attempts += 1;

            let (perturbed_duals, delta_2d, l2_norm) =
                perturb_lagrangian(&base_duals, &indices, eps, &mut rng);

            // Try to construct a valid polytope
            let polytope = match HkoPolytopeCache::from_f64(perturbed_duals.clone()) {
                Some(p) => p,
                None => continue,
            };
            let classification =
                match classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
                    Ok(classification) => classification,
                    Err(_) => continue,
                };

            // Keep the explicit billiard call here because `SampleRow` stores
            // `bounces`, which is only available from the billiard-native API.
            let billiard = capacity_billiard(
                &polytope.dual_vertices,
                &polytope.dual_vertices_f64,
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            )
            .expect("classification already succeeded");

            let vol =
                exact_volume_reference_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
            if vol <= 0.0 {
                continue;
            }

            let bounces = match bounce_count_from_sigma_for_facets(
                &classification.q_indices,
                &classification.p_indices,
                billiard.best_sigma(),
            ) {
                Some(k) => k,
                None => continue,
            };
            let cap = billiard.min_action;
            let sys = cap * cap / (2.0 * vol);

            if sys > 1.0 {
                n_above_1 += 1;
            }
            sys_values.push(sys);

            let row = SampleRow {
                epsilon: eps,
                sample_index: accepted,
                is_base: false,
                delta_2d,
                l2_norm,
                dual_vertices: perturbed_duals.iter().map(v4_to_array).collect(),
                volume: vol,
                capacity: cap,
                sys,
                bounces,
            };
            let line = serde_json::to_string(&row).expect("serialize");
            writeln!(samples_writer, "{line}").expect("write");

            accepted += 1;
        }

        total_rows += accepted;
        let level_time = level_start.elapsed().as_secs_f64();
        let accept_rate = if attempts > 0 {
            accepted as f64 / attempts as f64
        } else {
            0.0
        };
        let frac_above_1 = if accepted > 0 {
            n_above_1 as f64 / accepted as f64
        } else {
            0.0
        };

        // Compute statistics
        let (sys_min, sys_max, sys_mean, sys_std) = if !sys_values.is_empty() {
            let n = sys_values.len() as f64;
            let min = sys_values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = sys_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mean = sys_values.iter().sum::<f64>() / n;
            let var = sys_values
                .iter()
                .map(|&s| (s - mean) * (s - mean))
                .sum::<f64>()
                / n;
            (min, max, mean, var.sqrt())
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };

        // Write level summary
        let level_row = LevelRow {
            epsilon: eps,
            n_accepted: accepted,
            n_attempts: attempts,
            accept_rate,
            n_above_1,
            frac_above_1,
            sys_min,
            sys_mean,
            sys_max,
            sys_std,
            time_s: level_time,
        };
        let line = serde_json::to_string(&level_row).expect("serialize");
        writeln!(levels_writer, "{line}").expect("write");

        println!(
            "{:<8.2} {:>10} {:>10} {:>7.1}% {:>10} {:>7.1}% {:>8.4} {:>8.4} {:>5.1}s",
            eps,
            accepted,
            attempts,
            accept_rate * 100.0,
            n_above_1,
            frac_above_1 * 100.0,
            sys_min,
            sys_max,
            level_time
        );
    }

    samples_writer.flush().expect("flush samples");
    levels_writer.flush().expect("flush levels");
    println!(
        "\nWrote {total_rows} sample rows to {}",
        samples_path.display()
    );
    println!("Wrote level summary to {}", levels_path.display());
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
