//! Directional boundary probing of the sys > 1 region around HKO2024.
//!
//! Architecture:
//! 1. `cargo run --bin lagrangian_probe --release` generates dataset
//! 2. Writes to lagrangian-boundary/lagrangian-probe.jsonl
//! 3. Python script (analyze.py) reads and plots
//!
//! For each random direction u on S^19 (unit sphere in 20D Lagrangian
//! perturbation space), binary-search for the radius r(u) where sys
//! crosses 1 along the ray δ = t·u from HKO2024.
//!
//! This directly measures the shape of the sys > 1 boundary without
//! model assumptions, unlike the L∞-box sweep in main.rs which measures
//! only the average size.
//!
//! This binary only needs scalar capacity/sys values, so it uses the root
//! `symplectic::ehz_capacity` wrapper instead of the billiard-native API.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;

const SEED: u64 = 43;

/// Number of random directions to probe.
/// 500 directions × ~15 bisection steps × ~30ms/eval ≈ 225s.
const N_DIRECTIONS: usize = 500;
const SMOKE_N_DIRECTIONS: usize = 1;

/// Bisection tolerance: stop when the interval [lo, hi] satisfies hi - lo < TOL.
/// At ~0.09 mean radius, 1e-4 gives ~0.1% relative precision.
const BISECT_TOL: f64 = 1e-4;

/// Maximum bisection iterations per direction.
const MAX_BISECT_ITER: usize = 50;

/// Upper bound for binary search. Must be large enough that sys < 1
/// for all directions at this radius. From the sweep: at ε=0.15 (L2 ~ 0.39),
/// no sample had sys > 1 in 500 trials. Use 0.5 as a safe upper bound.
/// Re-validate if the experiment finds directions where sys > 1 at r > 0.5.
const R_MAX: f64 = 0.5;

#[derive(Debug, Serialize)]
struct ProbeRow {
    direction_index: usize,
    /// The 20D unit direction vector (2 Lagrangian components per facet × 10 facets).
    direction: Vec<[f64; 2]>,
    /// Boundary radius: the L2 distance from HKO where sys crosses 1.
    /// NaN if the boundary wasn't found (sys > 1 at R_MAX, or polytope invalid).
    radius: f64,
    /// sys value at the boundary (should be ≈ 1.0 within bisection tolerance).
    sys_at_boundary: f64,
    /// Number of bisection iterations used.
    bisect_iters: usize,
    /// Whether the probe succeeded (valid polytope at all tested radii).
    success: bool,
    /// Reason for failure, if any.
    failure_reason: String,
}

/// Identify which 2D components are nonzero for each dual vertex.
/// Returns (i0, i1) index pairs: [0,1] for q-facets, [2,3] for p-facets.
// TODO: add [def:lagrangian-facet-type] to formal math (trivial from the LP definition)
fn lagrangian_component_indices(duals: &[Vector4<f64>]) -> Vec<(usize, usize)> {
    duals
        .iter()
        .map(|a| {
            let q_sq = a[0] * a[0] + a[1] * a[1];
            let p_sq = a[2] * a[2] + a[3] * a[3];
            if q_sq > p_sq {
                (0, 1)
            } else {
                (2, 3)
            }
        })
        .collect()
}

/// Sample a random unit vector on S^{d-1} via Gaussian projection.
fn random_direction(d: usize, rng: &mut ChaCha8Rng) -> Vec<f64> {
    let v: Vec<f64> = (0..d).map(|_| StandardNormal.sample(rng)).collect();
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    v.iter().map(|x| x / norm).collect()
}

/// Evaluate sys at HKO + t * direction (in 20D Lagrangian perturbation space).
/// Returns None if the polytope is invalid or capacity is unavailable.
fn eval_sys_at_ray(
    base_duals: &[Vector4<f64>],
    indices: &[(usize, usize)],
    direction_flat: &[f64],
    t: f64,
) -> Option<f64> {
    let mut perturbed = Vec::with_capacity(base_duals.len());
    for (k, (a, &(i0, i1))) in base_duals.iter().zip(indices.iter()).enumerate() {
        let mut v = *a;
        v[i0] += t * direction_flat[2 * k];
        v[i1] += t * direction_flat[2 * k + 1];
        perturbed.push(v);
    }

    let polytope = Polytope4D::from_f64(perturbed).ok()?;
    let ehz = ehz_capacity(&polytope).ok()?;
    let vol = volume(&polytope).ok()?;
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz.capacity();
    Some(cap * cap / (2.0 * vol))
}

/// Binary search for the radius where sys crosses 1 along a ray.
/// Returns (radius, sys_at_boundary, iterations, success, failure_reason).
fn bisect_boundary(
    base_duals: &[Vector4<f64>],
    indices: &[(usize, usize)],
    direction_flat: &[f64],
) -> (f64, f64, usize, bool, String) {
    // First check: sys at origin should be > 1 (it's HKO)
    // Then check: sys at R_MAX should be < 1

    // Check R_MAX
    match eval_sys_at_ray(base_duals, indices, direction_flat, R_MAX) {
        Some(sys) if sys >= 1.0 => {
            return (R_MAX, sys, 0, false, "sys >= 1 at R_MAX".to_string());
        }
        None => {
            // Polytope invalid at R_MAX. Find a valid upper bound.
            // Binary search for the largest valid t
            let mut hi = R_MAX;
            let mut lo = 0.0;
            for _ in 0..20 {
                let mid = (lo + hi) / 2.0;
                if eval_sys_at_ray(base_duals, indices, direction_flat, mid).is_some() {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            // lo is the largest valid radius. Check if sys < 1 there.
            match eval_sys_at_ray(base_duals, indices, direction_flat, lo) {
                Some(sys) if sys < 1.0 => {
                    // Good — use lo as upper bound and proceed to bisection below
                    return bisect_range(base_duals, indices, direction_flat, 0.0, lo);
                }
                Some(sys) => {
                    return (
                        lo,
                        sys,
                        0,
                        false,
                        format!("sys={sys:.6} >= 1 at largest valid radius {lo:.6}"),
                    );
                }
                None => {
                    return (0.0, 0.0, 0, false, "no valid radius found".to_string());
                }
            }
        }
        Some(_) => {} // sys < 1 at R_MAX, proceed
    }

    bisect_range(base_duals, indices, direction_flat, 0.0, R_MAX)
}

fn bisect_range(
    base_duals: &[Vector4<f64>],
    indices: &[(usize, usize)],
    direction_flat: &[f64],
    mut lo: f64,
    mut hi: f64,
) -> (f64, f64, usize, bool, String) {
    let mut iters = 0;
    let mut last_sys = 0.0;

    while hi - lo > BISECT_TOL && iters < MAX_BISECT_ITER {
        let mid = (lo + hi) / 2.0;
        match eval_sys_at_ray(base_duals, indices, direction_flat, mid) {
            Some(sys) => {
                last_sys = sys;
                if sys > 1.0 {
                    lo = mid; // still inside S
                } else {
                    hi = mid; // outside S
                }
            }
            None => {
                // Polytope invalid at mid — treat as outside (shrink hi)
                hi = mid;
            }
        }
        iters += 1;
    }

    let radius = (lo + hi) / 2.0;
    // Evaluate sys at the final radius for reporting
    let sys_final =
        eval_sys_at_ray(base_duals, indices, direction_flat, radius).unwrap_or(last_sys);

    (radius, sys_final, iters, true, String::new())
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let smoke = std::env::args().any(|a| a == "--smoke");
    let n_directions = if smoke {
        SMOKE_N_DIRECTIONS
    } else {
        N_DIRECTIONS
    };

    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("lagrangian-boundary");
    let output_path = if smoke {
        base_dir.join("lagrangian-probe-smoke.jsonl")
    } else {
        base_dir.join("lagrangian-probe.jsonl")
    };

    println!("Directional boundary probing of sys > 1 region around HKO2024\n");

    std::fs::create_dir_all(&base_dir).expect("create lagrangian-boundary output dir");

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    // Base polytope
    let base = known_polytopes::hko_pentagon();
    let base_polytope = &base.polytope;
    let base_duals: Vec<Vector4<f64>> = base_polytope.dual_vertices_f64().to_vec();
    let indices = lagrangian_component_indices(&base_duals);
    let d = base_duals.len() * 2; // 20D perturbation space

    // Verify base sys
    let base_vol = volume(base_polytope).expect("volume failed");
    let base_ehz = ehz_capacity(base_polytope).expect("capacity unavailable");
    let base_sys = base_ehz.capacity().powi(2) / (2.0 * base_vol);
    println!("Base sys = {base_sys:.6} (should be ~1.047)");
    println!("Probing {n_directions} random directions...\n");

    let mut radii = Vec::with_capacity(n_directions);
    let mut n_success = 0usize;
    let mut n_fail = 0usize;

    for i in 0..n_directions {
        let dir = random_direction(d, &mut rng);
        let (radius, sys_at_boundary, bisect_iters, success, failure_reason) =
            bisect_boundary(&base_duals, &indices, &dir);

        if success {
            radii.push(radius);
            n_success += 1;
        } else {
            n_fail += 1;
        }

        // Convert flat direction to per-facet format for output
        let direction_2d: Vec<[f64; 2]> = (0..base_duals.len())
            .map(|k| [dir[2 * k], dir[2 * k + 1]])
            .collect();

        let row = ProbeRow {
            direction_index: i,
            direction: direction_2d,
            radius,
            sys_at_boundary,
            bisect_iters,
            success,
            failure_reason,
        };
        let line = serde_json::to_string(&row).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        if (i + 1) % 100 == 0 {
            let elapsed = t0.elapsed().as_secs_f64();
            println!(
                "  {}/{}: {n_success} success, {n_fail} fail, {:.1}s",
                i + 1,
                N_DIRECTIONS,
                elapsed
            );
        }
    }

    writer.flush().expect("flush");

    // Summary statistics
    if !radii.is_empty() {
        let n = radii.len() as f64;
        let mean = radii.iter().sum::<f64>() / n;
        let var = radii.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n;
        let std = var.sqrt();
        let min = radii.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = radii.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        println!("\n=== BOUNDARY RADII (n={n_success}) ===");
        println!("  mean = {mean:.4}, std = {std:.4}, CV = {:.3}", std / mean);
        println!("  min = {min:.4}, max = {max:.4}, ratio = {:.2}", max / min);
        println!(
            "  per-component: mean = {:.4} (= mean/sqrt(20/3))",
            mean / (20.0_f64 / 3.0).sqrt()
        );
    }

    println!("\nWrote {} rows to {}", n_directions, output_path.display());
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
