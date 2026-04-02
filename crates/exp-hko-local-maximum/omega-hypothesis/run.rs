//! Omega-obstacle experiment: do near-Lagrangian 2-faces help create high systolic ratios?
//!
//! Hypothesis: small |ω₀(n_i, n_j)| between adjacent facets → high sys.
//! Mechanism: Q(β) = Σ β_i β_j ω₀(...), capacity = 1/(2·max Q), sys = c²/(2V).
//! Small ω contributions → smaller Q → larger capacity → potentially larger sys.
//!
//! Phase A (observational): For each polytope, compute ω₀ for all ridge-adjacent pairs
//! and for orbit transitions. Plot min|ω| vs sys.
//!
//! Phase B (gradient): Compute ⟨∇_{n_k} sys, ∇_{n_k} ω(n_k, n_i)⟩ analytically.
//! Negative dot product → sys increases when ω decreases → hypothesis supported.
//!
//! Architecture:
//! 1. `cargo run --bin omega_obstacle --release` generates dataset
//! 2. Writes to omega-obstacle/omega-obstacle.jsonl
//! 3. Python script reads JSONL, produces figures

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::derivatives::{
    capacity_derivatives_a, volume_derivatives_a,
};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;

// ============================================================================
// Configuration
// ============================================================================

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples) pairs for random polytope generation.
const SAMPLING_PLAN: &[(usize, usize)] = &[
    (5, 200),
    (6, 200),
    (7, 200),
    (8, 200),
    (9, 100),
    (10, 50),
];

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct OmegaRow {
    source: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_ms: f64,

    // Orbit info
    orbit_length: usize,
    orbit_facets: Vec<usize>,
    orbit_betas: Vec<f64>,

    // Omega features — orbit transitions (physical direction, all ≥ 0)
    orbit_omegas: Vec<f64>,
    orbit_omega_min: f64,
    orbit_omega_mean: f64,

    // Omega features — all ridge-adjacent pairs
    ridge_omegas: Vec<[f64; 3]>, // [i, j, ω₀(n_i, n_j)] where i < j
    ridge_omega_abs_min: f64,
    n_ridges: usize,

    // Gradient dot products (Phase B)
    gradient_dots: Vec<GradientDot>,
}

#[derive(Debug, Serialize)]
struct GradientDot {
    facet_k: usize,
    neighbor_i: usize,
    k_on_orbit: bool,
    i_on_orbit: bool,
    omega: f64,
    dot: f64,
    grad_sys_norm: f64,
}

// ============================================================================
// Sensitivity computation
// ============================================================================

/// J₀(a,b,c,d) = (-c,-d,a,b) in (q₁,q₂,p₁,p₂) coordinates.
/// Equivalent to `symplectic::geom::symplectic_form::j4() * v` but avoids matrix allocation.
fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

/// Full ∇_{n_k} sys via chain rule: d(sys)/d(n_k) = (1/V)[c·dc/dn_k - sys·dV/dn_k].
#[allow(clippy::too_many_arguments)]
fn compute_d_sys_a(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    best_perm: &[usize],
    best_beta: &[f64],
    best_q: f64,
    best_mu: &[f64],
) -> Vec<Vector4<f64>> {
    let duals = polytope.dual_vertices_f64();

    let d_vol_a = volume_derivatives_a(polytope);
    let d_cap_a = capacity_derivatives_a(best_beta, best_q, best_mu, best_perm, duals);

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

// ============================================================================
// Phase A: Omega feature computation
// ============================================================================

/// Compute ω₀ for all ridge-adjacent pairs and orbit transitions.
fn compute_omega_features(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    orbit_facets: &[usize],  // physical direction (from EhzResult)
) -> (Vec<[f64; 3]>, Vec<f64>) {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();

    // Ridge omegas: for each ridge (2-face shared by facets i, j with i < j)
    let ridge_omegas: Vec<[f64; 3]> = skeleton
        .ridges
        .iter()
        .map(|r| {
            let i = r.facets[0];
            let j = r.facets[1];
            let w = omega0(&normals[i], &normals[j]);
            [i as f64, j as f64, w]
        })
        .collect();

    // Orbit omegas: ω₀(n_{σ(k)}, n_{σ(k+1)}) for physical transition σ(k) → σ(k+1).
    // For a physical transition A → B, feasibility requires ω₀(n_A, n_B) ≥ 0.
    let m = orbit_facets.len();
    let orbit_omegas: Vec<f64> = (0..m)
        .map(|k| {
            let from = orbit_facets[k];
            let to = orbit_facets[(k + 1) % m];
            omega0(&normals[from], &normals[to])
        })
        .collect();

    (ridge_omegas, orbit_omegas)
}

// ============================================================================
// Phase B: Gradient dot product computation
// ============================================================================

/// Compute ∇_{n_k} ω₀(n_k, n_i) projected to T_{n_k}S³.
///
/// Since ω₀(u, v) = ⟨J₀ u, v⟩ is bilinear:
///   ∂ω₀(n_k, n_i)/∂n_k = J₀^T n_i = -J₀ n_i  (because J₀^T = -J₀)
///
/// Wait — ω₀(n_k, n_i) = ⟨J₀ n_k, n_i⟩ (linear in n_k), so the gradient
/// w.r.t. n_k in R⁴ is J₀^T n_i = -J₀ n_i. But ω₀(u,v) = u^T J₀^T v
/// where we use the convention ω₀(u,v) = u[0]v[2] - u[2]v[0] + ...
/// Let's check: ω₀(u,v) = Σ (u_{q_j} v_{p_j} - u_{p_j} v_{q_j})
///            = u^T M v where M has the right entries.
/// ∂ω₀(n_k, n_i)/∂n_k = M n_i where M is the matrix of ω₀.
///
/// M = [[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]] = J₀^T = -J₀
/// (since J₀ is skew-symmetric: J₀^T = -J₀)
///
/// So ∂ω₀(n_k, n_i)/∂n_k = -J₀ n_i. Projected to T_{n_k}S³.
fn omega_gradient_on_tangent(n_k: &Vector4<f64>, n_i: &Vector4<f64>) -> Vector4<f64> {
    let neg_j0_ni = -j0_apply(n_i);
    // Project to T_{n_k}S³: remove component along n_k
    neg_j0_ni - neg_j0_ni.dot(n_k) * n_k
}

/// Compute gradient dot products for all (facet, ridge-neighbor) pairs.
fn compute_gradient_dots(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    d_sys_a: &[Vector4<f64>],
    orbit_facets: &[usize],
) -> Vec<GradientDot> {
    let normals: Vec<Vector4<f64>> = polytope.dual_vertices_f64().iter().map(|a| a / a.norm()).collect();
    let orbit_set: HashSet<usize> = orbit_facets.iter().copied().collect();

    // Build ridge-neighbor lookup: for each facet k, list of neighbors
    let f = polytope.facet_count();
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); f];
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        neighbors[i].push(j);
        neighbors[j].push(i);
    }

    let mut dots = Vec::new();
    for k in 0..f {
        let grad_sys = &d_sys_a[k];
        let grad_sys_norm = grad_sys.norm();

        for &i in &neighbors[k] {
            let grad_omega = omega_gradient_on_tangent(&normals[k], &normals[i]);
            let dot = grad_sys.dot(&grad_omega);
            let w = omega0(&normals[k], &normals[i]);

            dots.push(GradientDot {
                facet_k: k,
                neighbor_i: i,
                k_on_orbit: orbit_set.contains(&k),
                i_on_orbit: orbit_set.contains(&i),
                omega: w,
                dot,
                grad_sys_norm,
            });
        }
    }

    dots
}

// ============================================================================
// Main
// ============================================================================

fn process_polytope(
    polytope: &Polytope4D,
    source: &str,
) -> Option<OmegaRow> {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();

    let t0 = Instant::now();

    // Volume
    let vol = volume(polytope).ok()?;

    // Capacity via library ehz_capacity
    let ehz_result = symplectic::ehz_capacity(polytope)?;
    let cap = ehz_result.result.capacity;
    let sys = cap * cap / (2.0 * vol);

    let time_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Retrieve full KKT solution for the best orbit (beta, mu, xi)
    let best_perm = &ehz_result.result.best_permutation;
    let kkt_result = solve_kkt_for(polytope, best_perm).feasible()?;
    let best_beta = &kkt_result.beta;

    // Phase A: omega features
    let skeleton = Skeleton::compute(polytope);
    let (ridge_omegas, orbit_omegas) = compute_omega_features(polytope, &skeleton, best_perm);

    let orbit_omega_min = orbit_omegas
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let orbit_omega_mean = if orbit_omegas.is_empty() {
        0.0
    } else {
        orbit_omegas.iter().sum::<f64>() / orbit_omegas.len() as f64
    };

    let ridge_omega_abs_min = ridge_omegas
        .iter()
        .map(|r| r[2].abs())
        .fold(f64::INFINITY, f64::min);

    // Sanity: orbit omegas should all be ≥ 0 (feasibility)
    let n_negative = orbit_omegas.iter().filter(|&&w| w < -1e-10).count();
    if n_negative > 0 {
        let worst = orbit_omegas.iter().cloned().fold(f64::INFINITY, f64::min);
        eprintln!(
            "WARNING: {}: {}/{} orbit omegas < 0 (worst: {:.6e})",
            source, n_negative, orbit_omegas.len(), worst
        );
    }

    // Phase B: gradient dots (using library derivative functions with dual vertex parameterization)
    let d_sys_a = compute_d_sys_a(
        polytope, vol, cap, sys,
        best_perm, best_beta, kkt_result.q_corrected, &kkt_result.mu,
    );
    let gradient_dots = compute_gradient_dots(polytope, &skeleton, &d_sys_a, best_perm);

    Some(OmegaRow {
        source: source.to_string(),
        facet_count: f,
        dual_vertices: duals.iter().map(|a| [a[0], a[1], a[2], a[3]]).collect(),
        volume: vol,
        capacity: cap,
        sys,
        iterations: ehz_result.result.iterations,
        time_ms,
        orbit_length: best_perm.len(),
        orbit_facets: best_perm.clone(),
        orbit_betas: best_beta.clone(),
        orbit_omegas,
        orbit_omega_min,
        orbit_omega_mean,
        ridge_omegas,
        ridge_omega_abs_min,
        n_ridges: skeleton.ridges.len(),
        gradient_dots,
    })
}

fn main() {
    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("omega-obstacle");
    let out_path = out_dir.join("omega-obstacle.jsonl");
    let file = std::fs::File::create(&out_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut total = 0usize;
    let mut failed = 0usize;

    eprintln!("=== Omega-obstacle experiment ===");
    eprintln!("Output: {}", out_path.display());

    // Random polytopes
    for &(f, n) in SAMPLING_PLAN {
        let t0 = Instant::now();
        let polytopes = symplectic::random::generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);
        eprintln!(
            "F={}: generated {} polytopes in {:.1}s",
            f,
            polytopes.len(),
            t0.elapsed().as_secs_f64()
        );

        for (idx, polytope) in polytopes.iter().enumerate() {
            let source = format!("random_F{}_{}", f, idx);
            match process_polytope(polytope, &source) {
                Some(row) => {
                    serde_json::to_writer(&mut writer, &row).unwrap();
                    writeln!(writer).unwrap();
                    total += 1;
                }
                None => {
                    eprintln!("  SKIP: {} (capacity computation failed)", source);
                    failed += 1;
                }
            }
        }
        eprintln!(
            "  F={}: processed in {:.1}s (total so far: {})",
            f,
            t0.elapsed().as_secs_f64(),
            total
        );
    }

    // HKO counterexample
    {
        let hko = known_polytopes::hko_pentagon();
        let source = "hko_pentagon";
        match process_polytope(&hko.polytope, source) {
            Some(row) => {
                serde_json::to_writer(&mut writer, &row).unwrap();
                writeln!(writer).unwrap();
                total += 1;
                eprintln!("HKO pentagon: sys = {:.6}", row.sys);
            }
            None => {
                eprintln!("WARNING: HKO pentagon capacity failed");
                failed += 1;
            }
        }
    }

    // Other known polytopes for reference (skip F > 10 — instrumented HK2017 is exponential)
    for kp in &[
        known_polytopes::simplex(),
        known_polytopes::hypercube(),
    ] {
        if kp.polytope.facet_count() > 10 {
            eprintln!("SKIP: {} (F={} > 10, too expensive for instrumented HK2017)",
                      kp.name, kp.polytope.facet_count());
            continue;
        }
        match process_polytope(&kp.polytope, kp.name) {
            Some(row) => {
                serde_json::to_writer(&mut writer, &row).unwrap();
                writeln!(writer).unwrap();
                total += 1;
            }
            None => {
                failed += 1;
            }
        }
    }

    writer.flush().unwrap();
    eprintln!(
        "\nDone: {} polytopes written, {} failed. Output: {}",
        total,
        failed,
        out_path.display()
    );
}
