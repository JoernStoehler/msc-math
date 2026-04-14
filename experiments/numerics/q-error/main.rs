#![allow(clippy::collapsible_if, clippy::op_ref, clippy::bool_comparison, dead_code)]

//! Numerical accuracy experiment for the KKT solver.
//!
//! Goal: Verify that f64 numerical errors stay within the proven error bounds.
//!
//! Two complementary checks:
//! 1. At ALL (S,σ) nodes: assert the error bound E and residual ‖r‖ are small.
//! 2. At select nodes: compare numerical Q̃ against exact Q (rational arithmetic),
//!    and assert |Q̃ - Q_exact| ≤ E (the error bound is valid).
//!
//! Input: Known polytopes from the library (F ≤ 10).
//! Output: Summary tables to stdout. Panics on any violation.
use nalgebra::{DMatrix, DVector, Vector4};
// TODO: `ehz_capacity` and `combinations` move to `algorithms::hk2017` (wave 3, subagent #6)
// TODO: `cyclic_permutations` stays at `algorithms::hk2017::permutations::cyclic_permutations` (wave 3)
// TODO: `build_kkt_system` renamed to `kkt::qp_assembly::build_augmented_system` with signature
//   change: now takes (polytope, perm) instead of (normals, heights, perm).
//   `q_from_beta` removed from public API.
// TODO: `kkt_rational` renamed to `kkt::rational_solver` (wave 2, subagent #3)
// TODO: ehz_capacity will be re-exported from algorithms::hk2017 once wave 3 (subagent #6) writes hk2017/mod.rs
use symplectic::algorithms::hk2017::ehz_capacity;
// TODO: cyclic_permutations will be available from hk2017::permutations once wave 3 (subagent #6) writes it
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::rational_solver as kkt_rational;

// ── Local copies of library functions (modules not yet written in migration) ──

/// Generate all cyclic permutations (fix first element, permute rest).
/// Previously imported from `symplectic::algorithms::hk2017::permutations::cyclic_permutations`.
fn cyclic_permutations(elements: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for_each_cyclic_permutation_local(elements, &mut |p| result.push(p.to_vec()));
    result
}

fn for_each_cyclic_permutation_local(
    elements: &[usize],
    callback: &mut impl FnMut(&[usize]),
) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf_local(&mut buf, 1, k, callback);
}

fn heap_perms_buf_local(
    buf: &mut [usize],
    offset: usize,
    k: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_perms_buf_local(buf, offset, k - 1, callback);
    for i in 0..k - 1 {
        if k % 2 == 0 {
            buf.swap(offset + i, offset + k - 1);
        } else {
            buf.swap(offset, offset + k - 1);
        }
        heap_perms_buf_local(buf, offset, k - 1, callback);
    }
}

/// Generate all C(n,k) combinations in lexicographic order.
/// Previously imported from `symplectic::algorithms::hk2017::combinations`.
fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.clone());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

/// Build the (m+5)x(m+5) augmented KKT system from normals, heights, and permutation.
/// Previously imported as `symplectic::kkt::augmented::build_kkt_system`.
fn build_kkt(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = h;
        kkt[(m + 4, i)] = h;
    }
    rhs[m + 4] = 1.0;
    (kkt, rhs)
}

/// Q(beta) = sum_{i>j} beta_i beta_j omega_0(n_{sigma(j)}, n_{sigma(i)}).
/// Previously imported as `symplectic::kkt::augmented::q_from_beta`.
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

/// Condition-number threshold for rank truncation (matches EIGEN_CONDITION_TAU
/// in library/src/kkt.rs).
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm (matches EPS_KKT_RESIDUAL in kkt.rs).
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Eigendecomposition-based solve: x̂ = Σ_i (v_i · b / λ_i) v_i for top `rank`
/// eigenvalues by |λ|.
fn eigen_solve(
    eigenvectors: &DMatrix<f64>,
    eigenvalues: &DVector<f64>,
    rhs: &DVector<f64>,
    size: usize,
    rank: usize,
) -> DVector<f64> {
    let mut indices: Vec<usize> = (0..size).collect();
    indices.sort_by(|&a, &b| eigenvalues[b].abs().partial_cmp(&eigenvalues[a].abs()).unwrap());

    let mut x = DVector::zeros(size);
    for &i in indices.iter().take(rank) {
        let coeff = eigenvectors.column(i).dot(rhs) / eigenvalues[i];
        x += coeff * eigenvectors.column(i);
    }
    x
}

// ── Part 1: All-node error bound sweep ──────────────────────────────────

/// Result of error bound sweep for one polytope.
struct SweepResult {
    total_nodes: u64,
    solvable_nodes: u64,
    rank_deficient: u64,
    worst_e: f64,
    worst_residual: f64,
    worst_correction: f64,
}

/// Sweep ALL (S,σ) pairs for a polytope and assert error bounds.
fn error_bound_sweep(polytope: &Polytope4D) -> SweepResult {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();

    let mut total_nodes: u64 = 0;
    let mut solvable_nodes: u64 = 0;
    let mut rank_deficient: u64 = 0;
    let mut worst_e: f64 = 0.0;
    let mut worst_residual: f64 = 0.0;
    let mut worst_correction: f64 = 0.0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for perm in cyclic_permutations(&subset) {
                total_nodes += 1;
                let size = m + 5;
                let (kkt, rhs) = build_kkt(&normals, &heights, &perm);

                let eig = kkt.clone().symmetric_eigen();
                let eigenvalues = &eig.eigenvalues;
                let eigenvectors = &eig.eigenvectors;

                let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs)
                    .fold(0.0_f64, f64::max);
                if lambda_max_abs < 1e-12 {
                    continue; // Numerically zero matrix
                }

                let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
                let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();
                if rank < size {
                    rank_deficient += 1;
                }

                // |λ_min| among retained eigenvalues
                let abs_lambda_min = eigenvalues.iter()
                    .filter(|&&e| e.abs() > threshold)
                    .map(|e| e.abs())
                    .fold(f64::INFINITY, f64::min)
                    .max(f64::MIN_POSITIVE);

                let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
                let residual_vec = &kkt * &x - &rhs;
                let residual_norm = residual_vec.norm();

                if residual_norm > EPS_KKT_RESIDUAL {
                    continue; // Skip nodes with large residual (solver rejects these)
                }

                solvable_nodes += 1;

                // Q̃ = Q(β̂) + (r₂ᵀμ̂ + r₃ξ̂)
                let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();
                let q_raw = q_from_beta(&normals, &perm, &beta);
                let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x[i]).sum();
                let r3 = residual_vec[m + 4];
                let xi_hat = x[m + 4];
                let q_correction = r2_dot_mu + r3 * xi_hat;

                // E = (9/2) ‖r‖² / |λ_min|
                let r_sq = residual_norm * residual_norm;
                let q_error_bound = 4.5 * r_sq / abs_lambda_min;

                // Track worst cases
                worst_e = worst_e.max(q_error_bound);
                worst_residual = worst_residual.max(residual_norm);
                worst_correction = worst_correction.max(q_correction.abs());

                // ASSERT: error bound must be small
                assert!(
                    q_error_bound < 1e-6,
                    "E too large at (S,σ)={:?}: E={:.2e}, |r|={:.2e}, |λ_min|={:.2e}",
                    perm, q_error_bound, residual_norm, abs_lambda_min
                );

                // ASSERT: Q correction must be small (absolute or relative)
                assert!(
                    q_correction.abs() < 1e-6 || q_correction.abs() < 1e-6 * q_raw.abs(),
                    "Q correction too large at (S,σ)={:?}: correction={:.2e}, Q_raw={:.2e}",
                    perm, q_correction, q_raw
                );
            }
        }
    }

    SweepResult {
        total_nodes,
        solvable_nodes,
        rank_deficient,
        worst_e,
        worst_residual,
        worst_correction,
    }
}

// ── Part 2: Exact value comparison ──────────────────────────────────────
//
// Uses symplectic::kkt_rational::solve_kkt_exact for the rational solve.
// The local duplicate code (f64_to_rational, build_kkt_rational, gauss_solve,
// q_from_beta_rational) was removed in favor of the library implementation.

/// Result of exact comparison for one polytope.
struct ExactResult {
    q_numerical: f64,     // Q̃ (corrected numerical)
    q_exact: f64,         // Q_exact (rational, converted to f64 for display)
    actual_error: f64,    // |Q̃ - Q_exact|
    error_bound: f64,     // E from error bound lemma (mathematical)
    f64_eps: f64,         // f64 rounding tolerance (machine precision)
    bound_valid: bool,    // actual_error ≤ max(E, f64_eps)?
    perm_size: usize,
}

/// Compare numerical Q̃ against exact Q for the winning (S,σ) of a polytope.
fn exact_comparison(polytope: &Polytope4D) -> Option<ExactResult> {
    let result = ehz_capacity(polytope)?;

    // best_permutation follows positive Reeb direction — same order passed to solve_kkt/build_kkt.
    let perm = &result.result.best_permutation;
    let m = perm.len();
    let size = m + 5;

    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();

    // Solve the KKT system exactly via the library's rational solver.
    let exact_result = kkt_rational::solve_kkt_exact(polytope.dual_vertices(), perm)?;
    let q_exact_f64 = exact_result.q_exact_f64;

    // Compute numerical Q̃ and E (using the eigendecomposition path)
    let (kkt, rhs) = build_kkt(&normals, &heights, perm);
    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs).fold(0.0_f64, f64::max);
    let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
    let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();

    let abs_lambda_min = eigenvalues.iter()
        .filter(|&&e| e.abs() > threshold)
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
    let residual_vec = &kkt * &x - &rhs;
    let residual_norm = residual_vec.norm();

    let beta_numerical: Vec<f64> = (0..m).map(|i| x[i]).collect();
    let q_raw = q_from_beta(&normals, perm, &beta_numerical);
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;
    let q_corrected = q_raw + q_correction;

    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    let actual_error = (q_corrected - q_exact_f64).abs();

    // The comparison has two error sources:
    // 1. Mathematical: E = (9/2)‖r‖²/|λ_min| — bounds algorithm error (typically ~1e-28)
    // 2. Computational: f64 rounding in evaluating Q̃ and Q_exact — ~machine_eps * |Q|
    // The meaningful assertion is: actual error ≤ max(E, f64_tolerance).
    // When E << eps_machine, this validates that the ONLY error source is f64 rounding.
    let f64_eps = 1e-13 * (1.0 + q_corrected.abs());
    let tolerance = q_error_bound.max(f64_eps);
    let bound_valid = actual_error <= tolerance;

    Some(ExactResult {
        q_numerical: q_corrected,
        q_exact: q_exact_f64,
        actual_error,
        error_bound: q_error_bound,
        f64_eps,
        bound_valid,
        perm_size: m,
    })
}

// ── Main ────────────────────────────────────────────────────────────────

fn main() {
    let polytopes: Vec<(&str, Polytope4D)> = known_polytopes::all_known()
        .into_iter()
        .filter(|kp| kp.polytope.facet_count() <= 10)
        .map(|kp| (kp.name, kp.polytope.clone()))
        .collect();

    println!("=== Q Error Bound Experiment ===\n");
    println!("Checks numerical accuracy of the KKT solver against proven error bounds.");
    println!("Panics on any violation.\n");

    // ── Part 1: All-node error bound sweep ──────────────────────────────
    println!("--- Part 1: Error bound sweep (ALL nodes) ---");
    println!("  For each (S,σ): assert E < 1e-6 and |q_correction| < 1e-6");
    println!("{:<25} {:>4} {:>8} {:>6} {:>12} {:>12} {:>12}",
        "polytope", "F", "total", "solved", "worst_E", "worst_|r|", "worst_corr");
    println!("{}", "-".repeat(85));

    for (name, polytope) in &polytopes {
        let result = error_bound_sweep(polytope);
        println!("{:<25} {:>4} {:>8} {:>6} {:>12.3e} {:>12.3e} {:>12.3e}",
            name, polytope.facet_count(), result.total_nodes, result.solvable_nodes,
            result.worst_e, result.worst_residual, result.worst_correction);
    }

    println!("\n  All error bound assertions passed.\n");

    // ── Part 2: Exact comparison (select nodes) ─────────────────────────
    println!("--- Part 2: Exact comparison (winning node per polytope) ---");
    println!("  Exact KKT solve via Gaussian elimination over Q.");
    println!("  Assert: |Q̃ - Q_exact| ≤ max(E, f64_eps)");
    println!("  E = mathematical error bound, f64_eps = machine precision tolerance");
    println!("{:<25} {:>3} {:>16} {:>16} {:>12} {:>12} {:>12} {:>6}",
        "polytope", "m", "Q̃_numerical", "Q_exact", "|Q̃-Q_ex|", "E_math", "f64_eps", "valid");
    println!("{}", "-".repeat(100));

    let mut all_exact_valid = true;
    for (name, polytope) in &polytopes {
        match exact_comparison(polytope) {
            Some(r) => {
                println!("{:<25} {:>3} {:>16.12} {:>16.12} {:>12.3e} {:>12.3e} {:>12.3e} {:>6}",
                    name, r.perm_size, r.q_numerical, r.q_exact,
                    r.actual_error, r.error_bound, r.f64_eps,
                    if r.bound_valid { "OK" } else { "FAIL" });
                if !r.bound_valid {
                    all_exact_valid = false;
                }
            }
            None => {
                println!("{:<25} {:>3} (singular or rank-deficient)", name, 0);
            }
        }
    }

    assert!(all_exact_valid,
        "Exact comparison FAILED: |Q̃ - Q_exact| > max(E, f64_eps) for some polytope");
    println!("\n  All exact comparison assertions passed.\n");

    // ── Summary ───────────────────────────────────────────────────────────
    println!("=== Summary ===");
    println!("  Part 1 (error bounds):     PASSED (all {} polytopes)", polytopes.len());
    println!("  Part 2 (exact comparison): PASSED");
}
