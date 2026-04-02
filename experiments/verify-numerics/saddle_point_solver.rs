//! Saddle-point KKT solver — DEAD CODE, kept for reference.
//!
//! This solver operates on the augmented KKT matrix M = [[H, C^T], [C, 0]].
//! It was the original solver used in the verify-numerics experiment but has been
//! superseded by the projection solver (projection_solver.rs) which is more
//! numerically stable for the EHZ problem structure.
//!
//! Not imported by any binary. To resurrect: add `#[path = "saddle_point_solver.rs"]`
//! and import the types/functions from projection_solver.rs that this file depends on
//! (QP, solve_constraints, find_max_margin, MarginResult).

use nalgebra::{DMatrix, DVector};

// ══════════════════════════════════════════════════════════════════════════════
// Types
// ══════════════════════════════════════════════════════════════════════════════

/// Outcome of the saddle-point KKT solve.
#[derive(Clone, Debug)]
pub enum KktOutcome {
    /// Found a stationary point with beta > 0 (all components > -EPS_BETA_POSITIVE).
    /// Q may be positive, zero, or negative — the caller decides what to do.
    Feasible(KktResult),
    /// Beta has a non-positive component at the stationary point (and LP search
    /// in the null space couldn't fix it). Q was not checked.
    BetaNonPositive,
    /// Residual too large: the pseudoinverse solution doesn't satisfy Mx = b
    /// within tolerance at any threshold tier.
    ResidualTooLarge,
}

impl KktOutcome {
    /// Extract the feasible result, or None if not feasible.
    pub fn feasible(self) -> Option<KktResult> {
        match self {
            KktOutcome::Feasible(r) => Some(r),
            _ => None,
        }
    }

    /// Short string name for JSONL serialization.
    pub fn verdict_str(&self) -> &'static str {
        match self {
            KktOutcome::Feasible(_) => "feasible",
            KktOutcome::BetaNonPositive => "beta_non_positive",
            KktOutcome::ResidualTooLarge => "residual_too_large",
        }
    }
}

/// Feasible KKT solution with diagnostics.
#[derive(Clone, Debug)]
pub struct KktResult {
    /// Optimal beta vector (all components > -EPS_BETA_POSITIVE).
    pub beta: Vec<f64>,
    /// Lagrange multiplier for closure constraints (4 components).
    pub mu: Vec<f64>,
    /// Lagrange multiplier for normalization constraint (scalar).
    pub xi: f64,
    /// Pseudoinverse beta (beta0, before LP shift). Q is computed from this.
    pub beta0: Vec<f64>,
    /// Uncorrected Q value: (1/2) beta0^T H beta0.
    pub q_raw: f64,
    /// Residual-corrected Q value: q_raw + lambda^T r_lambda.
    pub q_corrected: f64,
    /// Error bound E on Q_tilde.
    pub q_error_bound: f64,
    /// Inertia of M: number of positive eigenvalues.
    pub n_positive: usize,
    /// Inertia of M: number of negative eigenvalues.
    pub n_negative: usize,
    /// Inertia of M: number of near-zero eigenvalues.
    pub n_zero: usize,
    /// ||P_discard b||: norm of b projected onto the discarded eigenspace.
    pub p_discard_b_norm: f64,
}

/// Eigendecomposition info for the KKT matrix M.
struct EigenInfo {
    eigenvalues: DVector<f64>,
    eigenvectors: DMatrix<f64>,
    n_positive: usize,
    n_negative: usize,
    n_zero: usize,
}

/// Result of the max-margin feasibility search (duplicated from projection_solver).
#[derive(Clone, Debug)]
struct MarginResult {
    margin: f64,
    alpha: DVector<f64>,
    beta: DVector<f64>,
}

// ══════════════════════════════════════════════════════════════════════════════
// Constants
// ══════════════════════════════════════════════════════════════════════════════

/// Minimum beta_i value to consider a solution certified positive.
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(beta) value to consider a solution meaningful.
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// Condition-number threshold for eigenvalue rank detection.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm for the KKT solution.
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// E1 error bound tolerance.
pub const EPS_E1_BOUND: f64 = 1e-6;

/// Conjectured upper bound on ||H|| / sigma_min(C) for EHZ polytope orbits.
/// Observed max: 21 across 186 polytope orbits (verify-numerics experiment, 2026-03-30).
const CONJECTURED_H_OVER_SIGMA_MIN_C: f64 = 100.0;

/// Threshold for filtering Type A eigenvectors in null-space search.
const EPS_TYPE_A_FILTER: f64 = 1e-10;

/// Absolute floor for eigenvalue magnitude.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

// ══════════════════════════════════════════════════════════════════════════════
// Utility
// ══════════════════════════════════════════════════════════════════════════════

/// Compute the E1 error bound and check assumptions/conjectures.
///
/// Returns E1 = ||H|| * ||beta|| * ||r|| / sigma_min(C).
/// Panics if any assumption or conjecture is violated.
pub fn compute_e1_bound(
    norm_h: f64,
    norm_beta: f64,
    residual_norm: f64,
    norm_r_beta: f64,
    norm_r_lambda: f64,
    sigma_min_c: f64,
) -> f64 {
    let _ = norm_r_lambda;

    let mut violations = Vec::new();

    if !(sigma_min_c > 1e-12) {
        violations.push(format!(
            "sigma_min(C) = {:.2e} < 1e-12: numerically rank-deficient.",
            sigma_min_c));
    }

    if sigma_min_c > 0.0 && norm_h / sigma_min_c > CONJECTURED_H_OVER_SIGMA_MIN_C {
        violations.push(format!(
            "||H||/sigma_min(C) = {:.2e} > {:.0e}.",
            norm_h / sigma_min_c, CONJECTURED_H_OVER_SIGMA_MIN_C));
    }

    if !(norm_r_beta < 1e-3) {
        violations.push(format!(
            "||r_beta|| = {:.2e} > 1e-3.", norm_r_beta));
    }

    if !(norm_beta <= 2.0) {
        violations.push(format!(
            "||beta|| = {:.4} > 2.", norm_beta));
    }

    if !violations.is_empty() {
        panic!("compute_e1_bound: {} violation(s):\n  {}", violations.len(), violations.join("\n  "));
    }

    norm_h * norm_beta * residual_norm / sigma_min_c
}

// ══════════════════════════════════════════════════════════════════════════════
// LP max-margin search (duplicated from projection_solver for self-containment)
// ══════════════════════════════════════════════════════════════════════════════

// NOTE: To compile this file, add `use good_lp::{...}` and the find_max_margin
// function from projection_solver.rs. Omitted here since this is dead code.

// ══════════════════════════════════════════════════════════════════════════════
// Saddle-point solver
// ══════════════════════════════════════════════════════════════════════════════

/// Solve the augmented KKT system from a pre-assembled matrix and RHS.
///
/// Uses eigendecomposition with two-tier rank detection:
/// 1. Permissive (EPS_EIGEN_FLOOR = 1e-12): retains all but numerically zero eigenvalues.
/// 2. Strict (EIGEN_CONDITION_TAU = 1e-3): treats small eigenvalues as null space.
pub fn solve_saddle_point(
    kkt_matrix: &DMatrix<f64>,
    rhs: &DVector<f64>,
) -> KktOutcome {
    let m = rhs.len() - 5;
    let size = rhs.len();

    let eig = kkt_matrix.clone().symmetric_eigen();
    let max_abs_ev = eig.eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
    assert!(
        max_abs_ev >= EPS_EIGEN_FLOOR,
        "KKT matrix has all eigenvalues < {:.0e} (max |lambda| = {:.2e}). \
         This means both H ~ 0 and C ~ 0 — garbage input, not a valid QP.",
        EPS_EIGEN_FLOOR, max_abs_ev
    );

    // Compute inertia using the strict threshold.
    let strict_threshold = max_abs_ev * EIGEN_CONDITION_TAU;
    let eigen_info = EigenInfo {
        n_positive: eig.eigenvalues.iter().filter(|&&e| e > strict_threshold).count(),
        n_negative: eig.eigenvalues.iter().filter(|&&e| e < -strict_threshold).count(),
        n_zero: size - eig.eigenvalues.iter().filter(|&&e| e > strict_threshold).count()
            - eig.eigenvalues.iter().filter(|&&e| e < -strict_threshold).count(),
        eigenvalues: eig.eigenvalues,
        eigenvectors: eig.eigenvectors,
    };

    // Tier 1: Permissive threshold.
    if let Some(outcome) = try_pseudoinverse_with_threshold(
        kkt_matrix, rhs, m,
        &eigen_info, EPS_EIGEN_FLOOR,
    ) {
        if let KktOutcome::Feasible(_) = &outcome {
            return outcome;
        }
    }

    // Tier 2: Strict threshold.
    try_pseudoinverse_with_threshold(
        kkt_matrix, rhs, m,
        &eigen_info, strict_threshold,
    ).unwrap_or(KktOutcome::ResidualTooLarge)
}

/// Try to find an admissible beta > 0 solution using a specific eigenvalue threshold.
fn try_pseudoinverse_with_threshold(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    m: usize,
    eigen_info: &EigenInfo,
    threshold: f64,
) -> Option<KktOutcome> {
    let size = m + 5;
    let eigenvalues = &eigen_info.eigenvalues;
    let eigenvectors = &eigen_info.eigenvectors;

    // Pseudoinverse solution.
    let mut x0 = DVector::zeros(size);
    let mut rank = 0usize;
    for i in 0..size {
        if eigenvalues[i].abs() > threshold {
            rank += 1;
            let coeff = eigenvectors.column(i).dot(rhs) / eigenvalues[i];
            for j in 0..size {
                x0[j] += coeff * eigenvectors[(j, i)];
            }
        }
    }

    // ||P_discard b||.
    let mut p_discard_b_sq = 0.0;
    for i in 0..size {
        if eigenvalues[i].abs() <= threshold {
            let comp = eigenvectors[(m + 4, i)];
            p_discard_b_sq += comp * comp;
        }
    }
    let p_discard_b_norm = p_discard_b_sq.sqrt();

    let residual_vec = kkt * &x0 - rhs;
    let residual_norm = residual_vec.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return None;
    }

    // Q error bound computation.
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x0[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x0[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;

    let abs_lambda_min = eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    let mu0: Vec<f64> = (m..m + 4).map(|i| x0[i]).collect();
    let xi0 = x0[m + 4];

    let set_p_discard = |outcome: KktOutcome| -> KktOutcome {
        match outcome {
            KktOutcome::Feasible(mut r) => { r.p_discard_b_norm = p_discard_b_norm; KktOutcome::Feasible(r) }
            other => other,
        }
    };

    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info)));
    }

    if rank == size {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info)));
        }
        return Some(KktOutcome::BetaNonPositive);
    }

    // Rank-deficient: search the numerical null space for beta > 0.
    let mut null_columns: Vec<DVector<f64>> = Vec::new();
    for i in 0..size {
        if eigenvalues[i].abs() <= threshold {
            let v_beta = DVector::from_fn(m, |j, _| eigenvectors[(j, i)]);
            if v_beta.norm() >= EPS_TYPE_A_FILTER {
                let mut constraint_violation_sq = 0.0;
                for row in m..size {
                    let dot: f64 = (0..m).map(|j| kkt[(row, j)] * eigenvectors[(j, i)]).sum();
                    constraint_violation_sq += dot * dot;
                }
                let constraint_violation = constraint_violation_sq.sqrt();
                assert!(
                    constraint_violation < 0.1,
                    "Type C eigenvector detected: ||constraint * v_beta|| = {:.2e}, \
                     |lambda| = {:.2e}, ||v_beta|| = {:.2e}, m = {}.",
                    constraint_violation, eigenvalues[i].abs(), v_beta.norm(), m
                );
                null_columns.push(v_beta);
            }
        }
    }

    let k_eff = null_columns.len();

    if k_eff == 0 {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info)));
        } else {
            return Some(KktOutcome::BetaNonPositive);
        }
    }

    let mut null_basis = DMatrix::zeros(m, k_eff);
    for (col, v) in null_columns.iter().enumerate() {
        null_basis.set_column(col, v);
    }

    // NOTE: find_max_margin is not available in this dead-code file.
    // To resurrect, import from projection_solver.rs or duplicate here.
    // The following is the original logic using find_max_margin:
    //
    // let beta0_dv = DVector::from_column_slice(&beta0);
    // let margin_result = find_max_margin(&beta0_dv, &null_basis);
    // ... (LP shift + projection logic)
    //
    // For now, fall back to beta0 directly:
    if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info)));
    }
    Some(KktOutcome::BetaNonPositive)
}

/// Compute the constraint residual for the beta vector.
fn extract_constraint_residual(kkt: &DMatrix<f64>, beta: &[f64], m: usize) -> f64 {
    let mut sq_sum = 0.0;
    for row in m..(m + 5) {
        let rhs_val = if row == m + 4 { 1.0 } else { 0.0 };
        let dot: f64 = (0..m).map(|j| kkt[(row, j)] * beta[j]).sum();
        sq_sum += (dot - rhs_val).powi(2);
    }
    sq_sum.sqrt()
}

/// Build the final KktResult with Q computation.
#[allow(clippy::too_many_arguments)]
fn finalize_result(
    beta: &[f64],
    mu: Vec<f64>,
    xi: f64,
    kkt: &DMatrix<f64>,
    m: usize,
    q_correction: f64,
    residual_norm: f64,
    abs_lambda_min: f64,
    eigen_info: &EigenInfo,
) -> KktOutcome {
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;

    let q_corrected = q_raw + q_correction;
    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    KktOutcome::Feasible(KktResult {
        beta: beta.to_vec(),
        beta0: beta.to_vec(),
        mu,
        xi,
        q_raw,
        q_corrected,
        q_error_bound,
        n_positive: eigen_info.n_positive,
        n_negative: eigen_info.n_negative,
        n_zero: eigen_info.n_zero,
        p_discard_b_norm: 0.0,
    })
}
