//! Subdifferential prediction at orbit-switching boundaries (Q5 + Q5b).
//!
//! Goal: Validate Clarke-subdifferential capacity predictions near orbit
//! switching boundaries and exact symmetry-forced ties.
//! Input Artifacts: None (generates all test polytopes internally).
//! Output Artifacts: experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5-subdiff.jsonl
//!         experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5b-symmetric.jsonl
//!
//! Tests whether the Clarke subdifferential (set of per-orbit gradients)
//! correctly predicts the capacity to first order near switching boundaries.
//!
//! Q5: Orbit-switching -- subdifferential prediction at near-tied orbits
//! Q5b: Exact switching boundaries -- subdifferential at symmetric/degenerate polytopes
//!
//! Split from gradient-validation/main.rs. Q5/Q5b use full ehz_capacity on
//! perturbed polytopes (unlike Q1-Q4 which use fixed-orbit solve_kkt_for).
//!
//! Methodology (Q5):
//! - Enumerate all certified orbits within generous action gap of the best
//! - Compute per-orbit gradient g_i via capacity_derivatives_a for each
//! - Subdifferential prediction: D_d c = min_i(g_i . d)
//! - Compare against actual capacity change via full ehz_capacity on perturbed polytope
//! - Records orbit switching (which orbit wins in the perturbed polytope)
//! - [prop:capacity-smoothness-classification](b): at switching boundaries, D_d c = min_i(nabla A_i . d).
//!   In formal/capacity-smoothness-classification.tex.
//!
//! Methodology (Q5b):
//! - Use LP(n,n) (regular Lagrangian products) where symmetry forces exact orbit ties
//! - Enumerate all orbits, identify those tied at the minimum action (gap = 0)
//! - Compute per-orbit gradients, predict D_d c = min_i(g_i . d)
//! - Compare against full ehz_capacity on perturbed polytope
//! - [prop:capacity-smoothness-classification](b): at exact boundaries, the subdiff
//!   formula gives the correct directional derivative
//!
//! Mathematical correspondence:
//! - [lem:cap-derivative] (unverified): envelope theorem formula for dc/da_k.
//!   In formal/capacity-derivatives.tex.
//! - [prop:capacity-smoothness-classification] (unverified): refined decomposition into
//!   per-orbit feasibility/smoothness and capacity-level min structure.
//!   In formal/capacity-smoothness-classification.tex.
//! - [thm:subdiff-with-appearance] (unverified): direction-filtered subdifferential
//!   extending the formula to orbits at the feasibility boundary.
//!   In formal/capacity-boundary-subdifferential.tex.
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient-subdifferential` -> JSONL files
//! 2. Python analyze.py -> convergence plots and orbit switching analysis
//!
//! Self-contained: generates all polytopes internally.

use dev_gradient::{ehz_capacity_safe, enumerate_all_orbits, random_direction, solve_kkt_safe};
use nalgebra::{DVector, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{
    capacity_derivatives_a_from_kkt_result, clarke_directional_derivative_a,
    directional_derivative_a,
};
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices;
use symplectic::kkt::saddle_point_solver::{KktResult, EPS_Q_POSITIVE};
use symplectic::random::generate_random_polytopes;
use symplectic::Polytope4D;
use symplectic::{lagrangian_product, regular_polygon_2d};

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

/// Perturbation sizes for the first-order prediction test.
/// Geometric sweep from 1e-1 to 1e-7 with half-decade spacing (13 values).
/// Large t: tests robustness far from base point.
/// Small t: tests convergence to zero (the defining gradient property).
/// Below ~1e-7, floating-point cancellation in f(a+td)-f(a) dominates.
const T_VALUES: &[f64] = &[
    1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7,
];

/// Minimum beta for certified orbit in Q5/Q5b enumeration.
/// Matches the library's EPS_MARGIN_TRUE (1e-9) from kkt/mod.rs -- orbits with
/// beta below this are Indeterminate in the production accumulator.
const EPS_BETA_CERTIFIED: f64 = 1e-9;

/// Q5: generous action gap threshold for orbit enumeration.
/// All orbits with action <= best_action + this threshold are kept.
/// Analysis filters to tighter thresholds in post-processing (no recomputation).
/// Value 0.1 chosen to include orbits up to ~10% of typical capacities (O(1)-O(10)),
/// matching the upper boundary of the "medium" gap bin (1e-3 to 1e-1). Orbits further
/// than 0.1 from the best have very different gradients and are not subdifferential
/// candidates at any realistic step size.
const Q5_GAP_THRESHOLD: f64 = 0.1;

/// Q5: polytopes per gap bin per facet count.
/// 15 per bin x 4 bins x 2 F-values = 120 polytopes (target), x 5 dirs x 13 t ~ 7800
/// ehz_capacity calls on perturbed polytopes. 131s total (run 2026-03-27, F=6-7).
const Q5_PER_BIN: usize = 15;

/// Q5: max candidates to generate when filling gap bins.
/// 3000 fills all non-tiny bins at F=6-7 (run 2026-03-27: 47/60 at F=6, 53/60 at F=7).
/// Tiny bins (gap < 1e-5) are structurally rare and underfill regardless of budget.
const Q5_MAX_CANDIDATES: usize = 3000;

/// Q5: gap bins for polytope selection (by gap between best and second-best orbit).
const Q5_GAP_BINS: [(f64, f64, &str); 4] = [
    (1e-1, f64::INFINITY, "large"),
    (1e-3, 1e-1, "medium"),
    (1e-5, 1e-3, "small"),
    (0.0, 1e-5, "tiny"),
];

/// Q5b: relative tolerance for identifying tied orbits at symmetric polytopes.
/// Two orbits are "tied" when |action_1 - action_2| / action_1 < this threshold.
/// 1e-8 is well above machine epsilon (~1e-16) but tight enough to filter out
/// genuinely distinct actions. Symmetric LP(n,n) should produce exact ties
/// (relative gap ~1e-14), so this is conservative.
const Q5B_TIE_RTOL: f64 = 1e-8;

/// Q5b: number of random directions per polytope. More than Q1-Q5 (5 dirs)
/// because the subdiff formula min_i(g_i . d) depends on direction -- some
/// directions agree with the single-orbit gradient, others don't. At LP(3,3)
/// with 2 tied orbits, 10 directions yielded 6 agreeing and 4 disagreeing
/// (run 2026-03-27), providing both sub-cases in one polytope. Reducing to 5
/// risks capturing only one sub-case. LP(5,5) uses 5 dirs due to ehz_capacity
/// cost (~70s per orbit enumeration).
const Q5B_N_DIRS: usize = 10;

/// Smoke-test settings for Phase 2.
const SMOKE_Q5_FACET_COUNTS: &[usize] = &[6];
const SMOKE_Q5_PER_BIN: usize = 1;
const SMOKE_Q5_MAX_CANDIDATES: usize = 30;
const SMOKE_Q5_GAP_THRESHOLD: f64 = 0.05;
const SMOKE_Q5_N_DIRS: usize = 1;
const SMOKE_Q5_T_VALUES: &[f64] = &[1e-2, 1e-4];
const SMOKE_Q5B_REGULAR_NS: &[usize] = &[3];
const SMOKE_Q5B_N_DIRS: usize = 3;
const SMOKE_Q5B_INCLUDE_SIMPLEX: bool = true;
const SMOKE_Q5B_INCLUDE_HYPERCUBE: bool = false;
const SMOKE_Q5B_INCLUDE_HKO: bool = false;
const SMOKE_Q5B_INCLUDE_GORBITS: bool = false;
const SMOKE_Q5B_GORBIT_ORDERS: &[usize] = &[];
const SMOKE_Q5B_GORBIT_ATTEMPTS: usize = 0;

// ============================================================================
// Output schema
// ============================================================================

/// Q5: per-orbit info embedded in each JSONL row for post-hoc gap-threshold filtering.
#[derive(Debug, Serialize)]
struct OrbitGradInfo {
    action: f64,
    grad_dot_d: f64,
}

/// Q5 output row: subdifferential prediction test for orbit-switching behavior.
#[derive(Debug, Serialize)]
struct SubdiffRow {
    phase: String,
    polytope_id: String,
    facet_count: usize,

    n_orbits: usize,
    action_gap: f64,

    dir_idx: usize,
    t: f64,
    log_t: f64,

    c_base: f64,
    c_perturbed: f64,
    actual_change: f64,

    subdiff_dot_d: f64,
    subdiff_predicted: f64,
    subdiff_residual: f64,
    subdiff_log_residual: f64,

    single_dot_d: f64,
    single_predicted: f64,
    single_residual: f64,
    single_log_residual: f64,

    /// Smallest beta component of the best orbit's KKT solution.
    /// Probes IFT boundary: smoothness of A_sigma requires all beta > 0
    /// ([lem:per-orbit-smooth]). Small min_beta means the orbit is near
    /// the boundary of its feasibility region.
    min_beta: f64,

    base_best_perm: String,
    perturbed_best_perm: String,
    orbit_switched: bool,

    /// Op 2 augmented subdiff: when orbit_switched is true, we compute the
    /// appearing orbit's gradient at the perturbed point a + td and include it
    /// in the subdiff min retroactively. Tests whether the orbit's gradient at
    /// a nearby feasible point would fix the prediction.
    /// NaN when not applicable (orbit_switched = false or gradient unavailable).
    augmented_dot_d: f64,
    augmented_predicted: f64,
    augmented_residual: f64,
    augmented_log_residual: f64,

    /// JSON array of {action, grad_dot_d} per orbit -- for post-hoc gap-threshold analysis.
    orbit_grads: String,

    /// [thm:subdiff-with-appearance] Direction-filtered subdifferential.
    /// Uses inclusive orbit enumeration (beta >= 0) and filters boundary orbits
    /// by nabla_a beta_k . d > 0 for all k with beta_k = 0.
    /// NaN when no directionally feasible orbits exist for this direction.
    filtered_dot_d: f64,
    filtered_predicted: f64,
    filtered_residual: f64,
    filtered_log_residual: f64,

    /// Number of tied orbits with beta >= 0 (inclusive enumeration).
    n_inclusive_tied: usize,
    /// Number of tied orbits with all beta > 0 (interior orbits).
    n_interior_tied: usize,
    /// Number of orbits in R(d) for this direction (interior + directionally feasible boundary).
    n_dir_feasible: usize,

    time_ms: f64,
}

// ============================================================================
// Helper functions
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
        "Usage: cargo run -p dev-gradient --release --bin gradient-subdifferential [--smoke]"
    );
    eprintln!("  --smoke: run a reduced run into a temporary directory");
    eprintln!("  -h, --help: show usage");
    std::process::exit(code);
}

fn smoke_output_dir(label: &str) -> String {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{label}-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create smoke output dir");
    dir.to_string_lossy().into_owned()
}

/// Compute nabla_a beta_k . d for each orbit position k, given a perturbation direction d.
///
/// Returns a Vec<f64> of length m where entry k = (d_beta_k/d_a) . d, the rate of change
/// of the k-th dwell-time weight when dual vertices move in direction d.
///
/// Uses [lem:kkt-sensitivity]: dx/da = -M^{-1}(dM x_0), where dM = dM/dt|_{t=0}
/// for the perturbation a(t) = a_0 + t.d. The product dM.x_0 simplifies to:
///   (dM.x_0)[i] = omega_0(a_{sigma(i)}, D) + d_{sigma(i)}.mu   for i < m,
///   (dM.x_0)[m+d] = D_d                                         for d < 4,
///   (dM.x_0)[m+4] = 0,
/// where D = Sigma_l beta_l . d_{sigma(l)} (using closure constraint A^T beta = 0).
fn beta_directional_sensitivity(
    polytope: &Polytope4D,
    perm: &[usize],
    kkt: &KktResult,
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
) -> Vec<f64> {
    let m = perm.len();
    let size = m + 5;

    // D = Sigma_i beta_i . d_{sigma(i)} -- weighted direction sum over orbit positions
    let mut big_d = Vector4::zeros();
    for i in 0..m {
        big_d += kkt.beta[i] * direction[perm[i]];
    }

    // Build RHS = dM . x_0
    let mu_vec = Vector4::new(kkt.mu[0], kkt.mu[1], kkt.mu[2], kkt.mu[3]);
    let mut rhs = DVector::zeros(size);
    for i in 0..m {
        rhs[i] = omega0(&duals[perm[i]], &big_d) + direction[perm[i]].dot(&mu_vec);
    }
    for d in 0..4 {
        rhs[m + d] = big_d[d];
    }
    // rhs[m + 4] = 0 already

    // Build M and solve M.w = -rhs via eigendecomposition
    let dual_vertices = polytope.dual_vertices_f64();
    let (kkt_matrix, _) = build_augmented_system_from_dual_vertices(dual_vertices, perm);
    let eig = kkt_matrix.symmetric_eigen();

    // Pseudoinverse threshold: same as saddle_point_solver's EIGEN_CONDITION_TAU (1e-3)
    // relative to max eigenvalue magnitude.
    let max_abs_eig = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    let threshold = max_abs_eig * 1e-3;

    // w = -M^{-1} . rhs = -Sigma_i (v_i . rhs / lambda_i) . v_i
    let mut w = DVector::zeros(size);
    for i in 0..size {
        let lambda = eig.eigenvalues[i];
        if lambda.abs() > threshold.max(1e-12) {
            let coeff = eig.eigenvectors.column(i).dot(&rhs) / lambda;
            for j in 0..size {
                w[j] -= coeff * eig.eigenvectors[(j, i)];
            }
        }
    }

    // Return beta-components: w[0..m]
    (0..m).map(|k| w[k]).collect()
}

/// Like enumerate_all_orbits but includes boundary orbits (beta >= 0 up to
/// numerical tolerance). Orbits with beta_k ~ 0 are feasible on the boundary
/// of the orbit's feasibility region -- they represent orbits that are about
/// to appear/disappear and contribute to the Clarke subdifferential.
fn enumerate_all_orbits_inclusive(polytope: &Polytope4D) -> Vec<(f64, Vec<usize>, KktResult)> {
    enumerate_orbits_inner(polytope, -EPS_BETA_CERTIFIED)
}

fn enumerate_orbits_inner(
    polytope: &Polytope4D,
    beta_threshold: f64,
) -> Vec<(f64, Vec<usize>, KktResult)> {
    let f = polytope.facet_count();
    let mut orbits = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(kkt) = solve_kkt_safe(polytope, perm) {
                    let min_beta = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
                    if min_beta > beta_threshold && kkt.q_corrected > EPS_Q_POSITIVE {
                        let action = 0.5 / kkt.q_corrected;
                        orbits.push((action, perm.to_vec(), kkt));
                    }
                }
            });
        }
    }

    orbits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    orbits
}

// ============================================================================
// Q5: Orbit-switching and subdifferential prediction
// ============================================================================

fn run_q5(base_dir: &str, smoke: bool) {
    let path = format!("{}/gradient-correctness-q5-subdiff.jsonl", base_dir);
    let file = File::create(&path).expect("create Q5 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let facet_counts: &[usize] = if smoke {
        SMOKE_Q5_FACET_COUNTS
    } else {
        &[6, 7]
    };
    let q5_per_bin = if smoke { SMOKE_Q5_PER_BIN } else { Q5_PER_BIN };
    let q5_max_candidates = if smoke {
        SMOKE_Q5_MAX_CANDIDATES
    } else {
        Q5_MAX_CANDIDATES
    };
    let q5_gap_threshold = if smoke {
        SMOKE_Q5_GAP_THRESHOLD
    } else {
        Q5_GAP_THRESHOLD
    };
    let q5_n_dirs = if smoke { SMOKE_Q5_N_DIRS } else { N_DIRS };
    let t_values: &[f64] = if smoke { SMOKE_Q5_T_VALUES } else { T_VALUES };

    for &f_count in facet_counts {
        // Benchmark ehz_capacity at this F
        let mut bench_rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 550 + f_count as u64);
        let bench_polys =
            generate_random_polytopes(if smoke { 1 } else { 5 }, f_count, 0.5, 2.0, &mut bench_rng);
        let t0 = Instant::now();
        for p in &bench_polys {
            ehz_capacity_safe(p);
        }
        let bench_ms = t0.elapsed().as_secs_f64() * 1000.0 / bench_polys.len() as f64;
        println!(
            "  Q5: F={} ehz_capacity benchmark: {:.2}ms/call",
            f_count, bench_ms
        );

        // Fill gap bins: find polytopes with different action gap levels
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 500 + f_count as u64);
        let mut bin_counts = [0usize; 4];
        let mut generated = 0;

        struct PolytopeWithOrbits {
            polytope: Polytope4D,
            orbits: Vec<(f64, Vec<usize>, KktResult)>,
            gap: f64,
        }
        let mut polytope_data: Vec<PolytopeWithOrbits> = Vec::new();

        println!(
            "  Q5: Finding polytopes with near-tied orbits at F={}...",
            f_count
        );

        while generated < q5_max_candidates && bin_counts.iter().any(|&c| c < q5_per_bin) {
            let polytopes =
                generate_random_polytopes(if smoke { 2 } else { 10 }, f_count, 0.5, 2.0, &mut rng);

            for polytope in &polytopes {
                generated += 1;
                if bin_counts.iter().all(|&c| c >= q5_per_bin) {
                    break;
                }

                // Use enumerate_all_orbits for binning (need second-best action).
                // Then filter to gap threshold for storage.
                let all_orbits = enumerate_all_orbits(polytope);
                if all_orbits.len() < 2 {
                    continue;
                }

                let best_action = all_orbits[0].0;
                let second_action = all_orbits[1].0;
                let gap = second_action - best_action;

                let bin_idx = Q5_GAP_BINS
                    .iter()
                    .position(|&(lo, hi, _)| gap >= lo && gap < hi);
                let bin_idx = match bin_idx {
                    Some(idx) if bin_counts[idx] < q5_per_bin => idx,
                    _ => continue,
                };

                // Keep only orbits within generous gap threshold
                let filtered: Vec<_> = all_orbits
                    .into_iter()
                    .filter(|(action, _, _)| *action <= best_action + q5_gap_threshold)
                    .collect();

                polytope_data.push(PolytopeWithOrbits {
                    polytope: polytope.clone(),
                    orbits: filtered,
                    gap,
                });
                bin_counts[bin_idx] += 1;

                if generated % 200 == 0 {
                    println!(
                        "    {} candidates, bins: large={}, medium={}, small={}, tiny={}",
                        generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
                    );
                }
            }
        }

        println!(
            "  Q5: F={} — {} polytopes (bins: {}/{}/{}/{}), from {} candidates",
            f_count,
            polytope_data.len(),
            bin_counts[0],
            bin_counts[1],
            bin_counts[2],
            bin_counts[3],
            generated,
        );

        // Process each polytope
        let mut dir_rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 600 + f_count as u64);

        for (pi, pd) in polytope_data.iter().enumerate() {
            let duals = pd.polytope.dual_vertices_f64();
            let best_perm = &pd.orbits[0].1;
            let best_kkt = &pd.orbits[0].2;
            let c_base = pd.orbits[0].0; // capacity = action of best orbit
            let min_beta = best_kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);

            // Compute per-orbit gradients
            let orbit_grads: Vec<Vec<Vector4<f64>>> = pd
                .orbits
                .iter()
                .map(|(_action, perm, kkt)| {
                    capacity_derivatives_a_from_kkt_result(&pd.polytope, perm, kkt)
                })
                .collect();

            let id = format!("q5_F{}_{:03}", f_count, pi);
            let base_perm_str = serde_json::to_string(best_perm).unwrap();

            for dir_idx in 0..q5_n_dirs {
                let direction = random_direction(duals.len(), &mut dir_rng);

                // g_i . d for each orbit
                let orbit_gd: Vec<f64> = orbit_grads
                    .iter()
                    .map(|g| directional_derivative_a(g, &direction))
                    .collect();

                // [prop:capacity-smoothness-classification](b): at switching boundaries,
                // the directional derivative D_d c = min_i(nabla_a A_i . d).
                let subdiff_gd = clarke_directional_derivative_a(&orbit_grads, &direction)
                    .expect("nonempty orbit list should define Clarke directional derivative");
                // [lem:cap-derivative]: single best-orbit gradient prediction.
                let single_gd = orbit_gd[0];

                // Per-orbit details for post-hoc gap-threshold analysis
                let orbit_info: Vec<OrbitGradInfo> = pd
                    .orbits
                    .iter()
                    .zip(orbit_gd.iter())
                    .map(|((action, _, _), &gd)| OrbitGradInfo {
                        action: *action,
                        grad_dot_d: gd,
                    })
                    .collect();
                let orbit_grads_json = serde_json::to_string(&orbit_info).unwrap();

                for &t in t_values {
                    let t0 = Instant::now();

                    // Perturb dual vertices
                    let perturbed_duals: Vec<Vector4<f64>> = duals
                        .iter()
                        .zip(direction.iter())
                        .map(|(a, d)| a + t * d)
                        .collect();

                    let perturbed_polytope = match Polytope4D::from_f64(perturbed_duals) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };

                    // Full ehz_capacity on perturbed polytope -- the key difference from Q1-Q4
                    let perturbed_ehz = match ehz_capacity_safe(&perturbed_polytope) {
                        Some(r) => r,
                        None => continue,
                    };

                    let c_perturbed = perturbed_ehz.capacity();
                    let perturbed_perm = perturbed_ehz.best_sigma();
                    let perturbed_perm_str = serde_json::to_string(perturbed_perm).unwrap();
                    let orbit_switched = perturbed_perm != best_perm;

                    let actual = c_perturbed - c_base;

                    let subdiff_pred = t * subdiff_gd;
                    let subdiff_res = (actual - subdiff_pred).abs();

                    let single_pred = t * single_gd;
                    let single_res = (actual - single_pred).abs();

                    let elapsed = t0.elapsed().as_secs_f64() * 1000.0;

                    let row = SubdiffRow {
                        phase: "q5".to_string(),
                        polytope_id: id.clone(),
                        facet_count: f_count,
                        n_orbits: pd.orbits.len(),
                        action_gap: pd.gap,
                        dir_idx,
                        t,
                        log_t: t.abs().log10(),
                        c_base,
                        c_perturbed,
                        actual_change: actual,
                        subdiff_dot_d: subdiff_gd,
                        subdiff_predicted: subdiff_pred,
                        subdiff_residual: subdiff_res,
                        subdiff_log_residual: subdiff_res.max(1e-300).log10(),
                        single_dot_d: single_gd,
                        single_predicted: single_pred,
                        single_residual: single_res,
                        single_log_residual: single_res.max(1e-300).log10(),
                        augmented_dot_d: f64::NAN,
                        augmented_predicted: f64::NAN,
                        augmented_residual: f64::NAN,
                        augmented_log_residual: f64::NAN,
                        min_beta,
                        base_best_perm: base_perm_str.clone(),
                        perturbed_best_perm: perturbed_perm_str,
                        orbit_switched,
                        orbit_grads: orbit_grads_json.clone(),
                        filtered_dot_d: f64::NAN,
                        filtered_predicted: f64::NAN,
                        filtered_residual: f64::NAN,
                        filtered_log_residual: f64::NAN,
                        n_inclusive_tied: 0,
                        n_interior_tied: 0,
                        n_dir_feasible: 0,
                        time_ms: elapsed,
                    };

                    let json = serde_json::to_string(&row).expect("serialize Q5 row");
                    writeln!(writer, "{}", json).expect("write Q5 row");
                    total_rows += 1;
                }
            }

            if (pi + 1) % 10 == 0 {
                println!(
                    "  Q5: F={} — {}/{} polytopes done",
                    f_count,
                    pi + 1,
                    polytope_data.len()
                );
            }
        }
    }

    writer.flush().expect("flush Q5");
    println!("Q5 done: {} rows written to {}", total_rows, path);
}

// ============================================================================
// Q5b: Subdifferential prediction at exact switching boundaries (symmetric polytopes)
// ============================================================================

/// Q5b tests the subdifferential formula D_d c = min_i(g_i . d) at points where
/// multiple orbits are exactly tied (gap = 0), using polytopes where symmetry
/// forces exact orbit degeneracy.
///
/// Polytope sources:
/// - Regular LP(n,n) for n = 3, 4, 5 (Lagrangian products of regular polygons)
/// - hko2024 (LP(5,5) with rotation -- Viterbo counterexample)
/// - Simplex (5 facets, S_5 symmetry, non-product)
/// - Hypercube (8 facets, hyperoctahedral symmetry, non-product)
/// - G-orbit polytopes: dual vertices = G.a_1 for finite G in Sp(4,R)
///
/// [prop:capacity-smoothness-classification](b): at switching boundaries with r >= 2
/// tied orbits, D_d c = min_i(g_i . d). When gradients are distinct, c is Lipschitz
/// but not differentiable. When all gradients match, min_i reduces to the common
/// gradient and c is C^1 at that point (degenerate tie).
///
/// Expected outcomes for DISTINCT gradients:
/// - Subdiff residual slope ~ 2: formula gives correct directional derivative,
///   with O(t^2) remainder from C^2 per-orbit actions.
/// - Single-orbit residual slope ~ 1 (in directions where orbits disagree on g.d)
///   or slope ~ 2 (in directions where they happen to agree).
///
/// Process one polytope for Q5b: enumerate tied orbits, compute gradients,
/// test subdiff prediction. Returns number of rows written.
fn q5b_process_polytope(
    polytope: &Polytope4D,
    id: &str,
    n_dirs: usize,
    rng: &mut ChaCha8Rng,
    writer: &mut BufWriter<File>,
) -> usize {
    let f_count = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();

    // Inclusive enumeration: beta >= 0 (picks up boundary orbits with beta_k = 0).
    // [thm:subdiff-with-appearance] needs these to compute the direction-filtered subdiff.
    println!(
        "  Q5b: {} — F={}, enumerating orbits (inclusive beta >= 0)...",
        id, f_count
    );
    let t_enum = Instant::now();
    let all_orbits = enumerate_all_orbits_inclusive(polytope);
    let enum_secs = t_enum.elapsed().as_secs_f64();
    println!(
        "  Q5b: {} — {} certified orbits in {:.1}s",
        id,
        all_orbits.len(),
        enum_secs,
    );

    if all_orbits.is_empty() {
        eprintln!("  Q5b: {} — no certified orbits, skipping", id);
        return 0;
    }

    let best_action = all_orbits[0].0;

    let tied_orbits: Vec<_> = all_orbits
        .iter()
        .filter(|(action, _, _)| {
            (action - best_action).abs() / best_action.max(1e-30) < Q5B_TIE_RTOL
        })
        .collect();

    let n_inclusive_tied = tied_orbits.len();

    // Classify orbits: interior (all beta > 0) vs boundary (some beta_k ~ 0).
    // Boundary orbits need the direction filter from [thm:subdiff-with-appearance].
    let is_interior: Vec<bool> = tied_orbits
        .iter()
        .map(|(_, _, kkt)| kkt.beta.iter().all(|&b| b > EPS_BETA_CERTIFIED))
        .collect();
    let n_interior_tied = is_interior.iter().filter(|&&b| b).count();
    let n_boundary = n_inclusive_tied - n_interior_tied;

    println!(
        "  Q5b: {} — {} tied orbits (action ~ {:.8}): {} interior, {} boundary",
        id, n_inclusive_tied, best_action, n_interior_tied, n_boundary,
    );

    if n_inclusive_tied < 2 {
        println!("  Q5b: {} — only 1 tied orbit, no boundary to test", id);
        return 0;
    }

    // Compute gradients for ALL tied orbits (interior + boundary).
    let orbit_grads: Vec<Vec<Vector4<f64>>> = tied_orbits
        .iter()
        .map(|(_action, perm, kkt)| capacity_derivatives_a_from_kkt_result(polytope, perm, kkt))
        .collect();
    let interior_orbit_grads: Vec<Vec<Vector4<f64>>> = orbit_grads
        .iter()
        .zip(is_interior.iter())
        .filter(|(_, interior)| **interior)
        .map(|(g, _)| g.clone())
        .collect();

    let min_beta = tied_orbits[0]
        .2
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);

    let mut rows_written = 0;

    for dir_idx in 0..n_dirs {
        let direction = random_direction(duals.len(), rng);

        // Compute g_i . d for all tied orbits
        let orbit_gd: Vec<f64> = orbit_grads
            .iter()
            .map(|g| directional_derivative_a(g, &direction))
            .collect();

        // subdiff_gd: min over INTERIOR orbits only (the standard formula).
        // [prop:capacity-smoothness-classification](b): D_d c = min_i(g_i . d)
        // This works when all tied orbits have beta > 0 but fails at orbit appearance.
        let subdiff_gd = clarke_directional_derivative_a(&interior_orbit_grads, &direction)
            .expect("interior tied-orbit set should be nonempty");
        let single_gd = orbit_gd
            .iter()
            .zip(is_interior.iter())
            .find(|(_, &interior)| interior)
            .map(|(&gd, _)| gd)
            .unwrap_or(orbit_gd[0]);

        // Direction filter: [thm:subdiff-with-appearance]
        // For boundary orbits, check nabla_a beta_k . d > 0 for all k with beta_k = 0.
        let mut dir_feasible_gd: Vec<f64> = Vec::new();
        let mut n_dir_feasible = 0usize;

        for (idx, ((_action, perm, kkt), &gd)) in
            tied_orbits.iter().zip(orbit_gd.iter()).enumerate()
        {
            if is_interior[idx] {
                // Interior orbits are always directionally feasible
                dir_feasible_gd.push(gd);
                n_dir_feasible += 1;
            } else {
                // Boundary orbit: compute nabla_a beta_k . d and check sign
                let beta_sens =
                    beta_directional_sensitivity(polytope, perm, kkt, &duals, &direction);

                // Check: for every k with beta_k ~ 0, need nabla_a beta_k . d > 0
                let mut feasible = true;
                for (k, &bk) in kkt.beta.iter().enumerate() {
                    if bk <= EPS_BETA_CERTIFIED {
                        if beta_sens[k] <= 0.0 {
                            feasible = false;
                            break;
                        }
                    }
                }
                if feasible {
                    dir_feasible_gd.push(gd);
                    n_dir_feasible += 1;
                }
            }
        }

        let (filtered_gd, filtered_valid) = if dir_feasible_gd.is_empty() {
            (f64::NAN, false)
        } else {
            (
                dir_feasible_gd
                    .iter()
                    .copied()
                    .fold(f64::INFINITY, f64::min),
                true,
            )
        };

        let orbit_info: Vec<OrbitGradInfo> = tied_orbits
            .iter()
            .zip(orbit_gd.iter())
            .map(|((action, _, _), &gd)| OrbitGradInfo {
                action: *action,
                grad_dot_d: gd,
            })
            .collect();
        let orbit_grads_json = serde_json::to_string(&orbit_info).unwrap();
        let base_perm_str = serde_json::to_string(&tied_orbits[0].1).unwrap();

        for &t in T_VALUES {
            let t0 = Instant::now();

            let perturbed_duals: Vec<Vector4<f64>> = duals
                .iter()
                .zip(direction.iter())
                .map(|(a, d)| a + t * d)
                .collect();

            let perturbed_polytope = match Polytope4D::from_f64(perturbed_duals) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let perturbed_ehz = match ehz_capacity_safe(&perturbed_polytope) {
                Some(r) => r,
                None => continue,
            };

            let c_perturbed = perturbed_ehz.capacity();
            let perturbed_perm = perturbed_ehz.best_sigma();
            let perturbed_perm_str = serde_json::to_string(perturbed_perm).unwrap();
            let orbit_switched = !tied_orbits
                .iter()
                .any(|(_, perm, _)| perm == perturbed_perm);

            let actual = c_perturbed - best_action;

            let subdiff_pred = t * subdiff_gd;
            let subdiff_res = (actual - subdiff_pred).abs();

            let single_pred = t * single_gd;
            let single_res = (actual - single_pred).abs();

            // Op 2: augmented subdiff -- include appearing orbit's gradient
            // computed at the perturbed point where it IS feasible.
            let (aug_gd, aug_pred, aug_res, aug_log_res) = if orbit_switched {
                let perturbed_duals_vec = perturbed_polytope.dual_vertices_f64();
                let appearing_grad =
                    solve_kkt_safe(&perturbed_polytope, perturbed_perm).map(|kkt| {
                        let _ = perturbed_duals_vec;
                        capacity_derivatives_a_from_kkt_result(
                            &perturbed_polytope,
                            perturbed_perm,
                            &kkt,
                        )
                    });
                if let Some(grad) = appearing_grad {
                    let appearing_gd = directional_derivative_a(&grad, &direction);
                    let aug = subdiff_gd.min(appearing_gd);
                    let pred = t * aug;
                    let res = (actual - pred).abs();
                    (aug, pred, res, res.max(1e-300).log10())
                } else {
                    (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
                }
            } else {
                (
                    subdiff_gd,
                    subdiff_pred,
                    subdiff_res,
                    subdiff_res.max(1e-300).log10(),
                )
            };

            // Direction-filtered subdiff prediction
            let (filt_pred, filt_res, filt_log_res) = if filtered_valid {
                let pred = t * filtered_gd;
                let res = (actual - pred).abs();
                (pred, res, res.max(1e-300).log10())
            } else {
                (f64::NAN, f64::NAN, f64::NAN)
            };

            let elapsed = t0.elapsed().as_secs_f64() * 1000.0;

            let row = SubdiffRow {
                phase: "q5b".to_string(),
                polytope_id: id.to_string(),
                facet_count: f_count,
                n_orbits: n_inclusive_tied,
                action_gap: 0.0,
                dir_idx,
                t,
                log_t: t.abs().log10(),
                c_base: best_action,
                c_perturbed,
                actual_change: actual,
                subdiff_dot_d: subdiff_gd,
                subdiff_predicted: subdiff_pred,
                subdiff_residual: subdiff_res,
                subdiff_log_residual: subdiff_res.max(1e-300).log10(),
                single_dot_d: single_gd,
                single_predicted: single_pred,
                single_residual: single_res,
                single_log_residual: single_res.max(1e-300).log10(),
                augmented_dot_d: aug_gd,
                augmented_predicted: aug_pred,
                augmented_residual: aug_res,
                augmented_log_residual: aug_log_res,
                min_beta,
                base_best_perm: base_perm_str.clone(),
                perturbed_best_perm: perturbed_perm_str,
                orbit_switched,
                orbit_grads: orbit_grads_json.clone(),
                filtered_dot_d: filtered_gd,
                filtered_predicted: filt_pred,
                filtered_residual: filt_res,
                filtered_log_residual: filt_log_res,
                n_inclusive_tied,
                n_interior_tied,
                n_dir_feasible,
                time_ms: elapsed,
            };

            let json = serde_json::to_string(&row).expect("serialize Q5b row");
            writeln!(writer, "{}", json).expect("write Q5b row");
            rows_written += 1;
        }
    }

    println!("  Q5b: {} — {} rows written", id, rows_written);
    rows_written
}

fn run_q5b(base_dir: &str, smoke: bool) {
    let path = format!("{}/gradient-correctness-q5b-symmetric.jsonl", base_dir);
    let file = File::create(&path).expect("create Q5b JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 700);

    // -- Part 1: Regular Lagrangian products LP(n,n) --
    let regular_ns: &[usize] = if smoke {
        SMOKE_Q5B_REGULAR_NS
    } else {
        &[3, 4, 5]
    };
    let q5b_n_dirs = if smoke { SMOKE_Q5B_N_DIRS } else { Q5B_N_DIRS };
    for &n in regular_ns {
        let (qn, qh) = regular_polygon_2d(n, 1.0);
        let (pn, ph) = regular_polygon_2d(n, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).expect("regular LP");
        let n_dirs = if smoke {
            q5b_n_dirs
        } else if polytope.facet_count() <= 8 {
            Q5B_N_DIRS
        } else {
            5
        };
        let id = format!("q5b_lp{}_{}", n, n);
        total_rows += q5b_process_polytope(&polytope, &id, n_dirs, &mut rng, &mut writer);
    }

    // -- Part 2: hko2024 (Viterbo counterexample, rotated LP(5,5)) --
    if !smoke || SMOKE_Q5B_INCLUDE_HKO {
        let kp = symplectic::known_polytopes::hko_pentagon();
        // F=10, expensive -- use 5 directions like LP(5,5)
        total_rows += q5b_process_polytope(&kp.polytope, "q5b_hko2024", 5, &mut rng, &mut writer);
    }

    // -- Part 3: Non-product polytopes (simplex, hypercube) --
    if !smoke || SMOKE_Q5B_INCLUDE_SIMPLEX {
        let kp = symplectic::known_polytopes::simplex();
        total_rows += q5b_process_polytope(
            &kp.polytope,
            "q5b_simplex",
            q5b_n_dirs,
            &mut rng,
            &mut writer,
        );
    }
    if !smoke || SMOKE_Q5B_INCLUDE_HYPERCUBE {
        let kp = symplectic::known_polytopes::hypercube();
        total_rows += q5b_process_polytope(
            &kp.polytope,
            "q5b_hypercube",
            q5b_n_dirs,
            &mut rng,
            &mut writer,
        );
    }

    // -- Part 4: G-orbit polytopes --
    // Construct polytopes as orbits of a seed dual vertex under a finite
    // symplectic group. The generator is a rotation by 2pi/n in both symplectic
    // planes (q-plane and p-plane) simultaneously:
    //   M = diag(R(2pi/n), R(2pi/n))  where R(theta) is 2D rotation.
    // This is symplectic because it's a unitary matrix in the U(2) subset Sp(4) embedding.
    // The orbit of a generic a_1 has n distinct dual vertices -> n-facet polytope.
    // Orbits in the capacity problem related by this G will have equal action.
    //
    // With a single seed, the orbit lives in a 2D subspace if the seed has
    // matching q/p phase -- so we use two seeds with different phases to ensure
    // the dual vertices positively span R^4. Failing that, we reject.
    if !smoke || SMOKE_Q5B_INCLUDE_GORBITS {
        println!("  Q5b: Generating G-orbit polytopes...");
        // Rotation orders: 2 seeds per order -> F = 2*order facets.
        // F <= 10 is tractable for enumerate_all_orbits; F = 14+ takes hours.
        let gorbit_orders: &[usize] = if smoke {
            SMOKE_Q5B_GORBIT_ORDERS
        } else {
            &[3, 4, 5]
        };
        let gorbit_attempts = if smoke { SMOKE_Q5B_GORBIT_ATTEMPTS } else { 50 };

        for &order in gorbit_orders {
            let theta = 2.0 * PI / order as f64;
            let (c, s) = (theta.cos(), theta.sin());
            // M = diag(R(theta), R(theta)) -- simultaneous rotation in q and p planes
            let gen = nalgebra::Matrix4::new(
                c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, c, -s, 0.0, 0.0, s, c,
            );

            let mut found = 0;
            for attempt in 0..gorbit_attempts {
                // Two random seeds, concatenate orbits
                let seed1 = Vector4::new(
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                );
                let seed2 = Vector4::new(
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                    StandardNormal.sample(&mut rng),
                );

                // Build orbit from both seeds
                let mut duals = Vec::new();
                for seed in &[seed1, seed2] {
                    let mut current = *seed;
                    for _ in 0..order {
                        let is_dup = duals
                            .iter()
                            .any(|v: &Vector4<f64>| (v - current).norm() < 1e-10);
                        if is_dup {
                            break;
                        }
                        duals.push(current);
                        current = &gen * current;
                    }
                }

                let polytope = match Polytope4D::from_f64(duals) {
                    Ok(p) => p,
                    Err(_) => continue, // Unbounded, degenerate, etc.
                };

                let id = format!("q5b_gorbit_n{}_{:02}", order, attempt);
                let n_dirs = if polytope.facet_count() <= 8 {
                    q5b_n_dirs
                } else {
                    5
                };
                let rows = q5b_process_polytope(&polytope, &id, n_dirs, &mut rng, &mut writer);
                if rows > 0 {
                    found += 1;
                }
                // Keep up to 3 successful polytopes per order
                if found >= 3 {
                    break;
                }
            }
            println!(
                "  Q5b: G-orbit order {} — {} polytopes with tied orbits (from {} attempts)",
                order, found, gorbit_attempts,
            );
        }
    }

    writer.flush().expect("flush Q5b");
    println!("Q5b done: {} rows written to {}", total_rows, path);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let smoke = smoke_mode();
    let base_dir = if smoke {
        let smoke_dir = smoke_output_dir("dev-numerics-subdifferential-smoke");
        println!("Smoke output: {smoke_dir}");
        smoke_dir
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("numerics-subdifferential")
            .to_string_lossy()
            .into_owned()
    };

    println!(
        "=== Gradient Correctness: Subdifferential (Q5 + Q5b){} ===\n",
        if smoke { " [smoke]" } else { "" }
    );
    let t0 = Instant::now();

    println!("--- Q5: Orbit-switching (subdifferential prediction) ---");
    let tp = Instant::now();
    run_q5(&base_dir, smoke);
    println!("  Q5 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("--- Q5b: Subdifferential at exact boundaries (symmetric polytopes) ---");
    let tp = Instant::now();
    run_q5b(&base_dir, smoke);
    println!("  Q5b time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}
