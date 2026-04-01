//! Self-contained f64 KKT solver module for the verify-numerics experiment.
//!
//! Copied from `crates/src/kkt/` to remove the dependency on `symplectic::kkt::*`.
//! Contains: saddle-point solver, projection solver (with sign fix), constraint
//! solver, beta feasibility (max-margin LP), and shared types/constants.
//!
//! Sign fix in projection solver: the library computes alpha0 = (H')^+ g, but
//! stationarity H' alpha + g = 0 requires alpha0 = -(H')^+ g. This module
//! implements the corrected version.
//!
//! Dependency: `good_lp` with feature `clarabel` was added to experiments/Cargo.toml
//! to support the LP solver in find_max_margin.

use good_lp::{constraint, default_solver, variable, variables, Expression, SolverModel,
    Solution as LpSolution};
use nalgebra::{DMatrix, DVector};

// ══════════════════════════════════════════════════════════════════════════════
// Types (from mod.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Constrained quadratic program: max (1/2) beta^T H beta  s.t. C beta = d, beta > 0.
pub struct QP {
    /// Constraint matrix (p x m).
    pub c: DMatrix<f64>,
    /// Constraint right-hand side (p x 1).
    pub d: DVector<f64>,
    /// Objective matrix (m x m, symmetric). Q(beta) = (1/2) beta^T H beta.
    pub h: DMatrix<f64>,
}

/// Trinary verdict for feasibility of beta > 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Certified feasible: all beta_k > eps.
    True,
    /// Certified infeasible: no beta > 0 exists in the solution set.
    False,
    /// Ambiguous: beta has near-zero components or near-null eigenvalues.
    Indeterminate,
}

/// Result of solving a QP (projection solver).
#[derive(Clone, Debug)]
pub struct Solution {
    /// Trinary verdict classifying the solution feasibility.
    pub verdict: Verdict,
    /// Optimal objective value: Q = (1/2) beta^T H beta.
    pub q: f64,
    /// Solution vector.
    pub beta: Vec<f64>,
    /// min_k beta_k.
    pub margin: f64,
}

// ══════════════════════════════════════════════════════════════════════════════
// Saddle-point solver types (from saddle_point_solver.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Outcome of the saddle-point KKT solve.
#[derive(Clone, Debug)]
pub enum KktOutcome {
    /// Found a stationary point with β > 0 (all components > -EPS_BETA_POSITIVE).
    /// Q may be positive, zero, or negative — the caller decides what to do.
    Feasible(KktResult),
    /// β has a non-positive component at the stationary point (and LP search
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
    /// Pseudoinverse beta (β₀, before LP shift). Q is computed from this.
    pub beta0: Vec<f64>,
    /// Uncorrected Q value: ½ β₀^T H β₀.
    pub q_raw: f64,
    /// Residual-corrected Q value: q_raw + λ̃^T r_λ.
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
    /// With b = (0_m, 0_4, 1), this equals sqrt(sum |v_i[m+4]|^2) over discarded eigenvectors.
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

// ══════════════════════════════════════════════════════════════════════════════
// Constraint solver types (from constraint_solver.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Solution of the linear constraint system Cx = d.
#[derive(Clone, Debug)]
pub struct ConstraintSolution {
    /// Particular solution x0 (minimum-norm via SVD pseudoinverse).
    pub x0: DVector<f64>,
    /// Orthonormal numerical null-space basis V in R^{m x k}.
    pub null_basis: DMatrix<f64>,
    /// Numerical rank of C.
    pub rank: usize,
}

// ══════════════════════════════════════════════════════════════════════════════
// Beta feasibility types (from beta_feasibility.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Result of the max-margin feasibility search.
#[derive(Clone, Debug)]
pub struct MarginResult {
    /// The maximum margin: max_alpha min_j (beta0 + V * alpha)_j.
    pub margin: f64,
    /// The optimal alpha achieving the margin (in null-space coordinates).
    pub alpha: DVector<f64>,
    /// The solution point beta = beta0 + V * alpha.
    pub beta: DVector<f64>,
}

// ══════════════════════════════════════════════════════════════════════════════
// Constants
// ══════════════════════════════════════════════════════════════════════════════

// -- Verdict thresholds (mod.rs) --

/// beta_k > EPS_MARGIN_TRUE -> certified positive.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// beta_k < -EPS_MARGIN_FALSE -> certified negative (infeasible).
const EPS_MARGIN_FALSE: f64 = 1e-9;

/// Absolute floor for eigenvalue magnitude. Matrix treated as numerically zero
/// if largest eigenvalue is below this.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

// -- Saddle-point solver constants --

/// Minimum beta_i value to consider a solution certified positive.
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(beta) value to consider a solution meaningful.
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// Condition-number threshold for eigenvalue rank detection.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm for the KKT solution.
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// E₁ error bound tolerance. If E₁ exceeds this, the Q value is unreliable
/// and should be classified as INDETERMINATE (fall back to rational arithmetic).
/// [lem:q-error-first-order]: E₁ = ||H|| · ||β̃|| · ||r|| / σ_min(C).
pub const EPS_E1_BOUND: f64 = 1e-6;

/// Conjectured upper bound on ||H|| / σ_min(C) for EHZ polytope orbits.
/// Observed max: 21 across 186 polytope orbits (verify-numerics experiment, 2026-03-30).
/// Threshold set at 5x the observed max for safety margin.
/// On panic → escalate to Jörn: the conjecture about ||H||/σ_min(C) has been disproven,
/// which means E₁ may be too large for certification at the default tolerance.
/// TODO: prove for all polytopes, or wait for the first panic to provide a counterexample.
const CONJECTURED_H_OVER_SIGMA_MIN_C: f64 = 100.0;

/// Threshold for filtering Type A eigenvectors in null-space search.
const EPS_TYPE_A_FILTER: f64 = 1e-10;

// -- Constraint solver constants --

/// Relative threshold for SVD rank detection.
const EPS_RANK_THRESHOLD: f64 = 1e-10;

/// Maximum residual ||Cx0 - d|| to accept as consistent.
const EPS_CONSISTENCY: f64 = 1e-8;

// -- Projection solver constants --

/// Eigenvalue threshold for the reduced Hessian H'.
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

// ══════════════════════════════════════════════════════════════════════════════
// Utility functions (from mod.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Compute Q = (1/2) beta^T H beta from pre-assembled H and beta.
pub fn q_value(h: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let b = DVector::from_column_slice(beta);
    0.5 * b.dot(&(h * &b))
}

/// Compute the E₁ error bound and check assumptions/conjectures.
///
/// Returns E₁ = ||H|| · ||β̃|| · ||r|| / σ_min(C).
/// Panics if any assumption or conjecture is violated.
/// All checks run before panicking, so the panic message reports ALL violations.
///
/// [lem:q-error-first-order]: |Q̃ - Q*| ≤ E₁ (proven, max empirical ratio 0.196).
pub fn compute_e1_bound(
    norm_h: f64,
    norm_beta: f64,
    residual_norm: f64,
    norm_r_beta: f64,
    norm_r_lambda: f64,
    sigma_min_c: f64,
) -> f64 {
    let _ = norm_r_lambda; // used for documentation; the proof uses ||r_λ|| ≤ ||r|| trivially

    let mut violations = Vec::new();

    // σ_min(C) numerically nonzero (C full row rank).
    // f64 SVD resolution: ~ε_mach · σ_max(C). Use 1e-10 · σ_max(C) as threshold
    // (matches EPS_RANK_THRESHOLD in constraint_solver.rs).
    // We don't have σ_max(C) here, but σ_min(C) < 1e-12 is suspicious for any C from EHZ
    // (observed min: 0.11 across 186 orbits).
    if !(sigma_min_c > 1e-12) {
        violations.push(format!(
            "σ_min(C) = {:.2e} < 1e-12: numerically rank-deficient. EHZ observed min: 0.11.",
            sigma_min_c));
    }

    // ||H||/σ_min(C) ≤ 100 (conjectured for EHZ polytopes, observed max 21 across 186 orbits).
    if sigma_min_c > 0.0 && norm_h / sigma_min_c > CONJECTURED_H_OVER_SIGMA_MIN_C {
        violations.push(format!(
            "||H||/σ_min(C) = {:.2e} > {:.0e}. Observed max: 21. Conjecture disproven → escalate to Jörn.",
            norm_h / sigma_min_c, CONJECTURED_H_OVER_SIGMA_MIN_C));
    }

    // ||r_β|| < 1e-3 (stationarity residual small in absolute terms).
    // Observed max: 1.1e-9 across 533 cases. Ratio ||r_β||/||r_λ|| can exceed 1
    // when both are at machine epsilon (irrelevant for Q error).
    if !(norm_r_beta < 1e-3) {
        violations.push(format!(
            "||r_β|| = {:.2e} > 1e-3. Observed max: 1.1e-9. Stationarity badly violated.",
            norm_r_beta));
    }

    // ||β̃|| ≤ 2 (for EHZ: 1^T β = 1, β > 0 ⇒ ||β||₂ ≤ ||β||₁ = 1; threshold 2x for f64 slack).
    if !(norm_beta <= 2.0) {
        violations.push(format!(
            "||β̃|| = {:.4} > 2. For EHZ: ||β||₂ ≤ 1. Solver returned un-normalized β.",
            norm_beta));
    }

    if !violations.is_empty() {
        panic!("compute_e1_bound: {} violation(s):\n  {}", violations.len(), violations.join("\n  "));
    }

    norm_h * norm_beta * residual_norm / sigma_min_c
}

/// Classify a margin value into a trinary verdict.
fn classify_margin(margin: f64) -> Verdict {
    if margin > EPS_MARGIN_TRUE {
        Verdict::True
    } else if margin < -EPS_MARGIN_FALSE {
        Verdict::False
    } else {
        Verdict::Indeterminate
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Constraint solver (from constraint_solver.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Solve Cx = d via SVD with threshold rank detection.
///
/// Returns `None` if the system is inconsistent (d not in the column space of C).
pub fn solve_constraints(
    c: &DMatrix<f64>,
    d: &DVector<f64>,
) -> Option<ConstraintSolution> {
    let p = c.nrows();
    let m = c.ncols();
    assert_eq!(p, d.nrows(), "C has {} rows but d has {} rows", p, d.nrows());

    // Edge case: zero-row constraint matrix (no constraints).
    if p == 0 {
        return Some(ConstraintSolution {
            x0: DVector::zeros(m),
            null_basis: DMatrix::identity(m, m),
            rank: 0,
        });
    }

    // Edge case: zero-column system. Consistent only if d = 0.
    if m == 0 {
        if d.norm() < EPS_CONSISTENCY {
            return Some(ConstraintSolution {
                x0: DVector::zeros(0),
                null_basis: DMatrix::zeros(0, 0),
                rank: 0,
            });
        } else {
            return None;
        }
    }

    // Step 1: Compute SVD of C.
    // Pad to square if underdetermined so V^T has full m rows.
    let c_for_svd = if p < m {
        let mut padded = DMatrix::zeros(m, m);
        for i in 0..p {
            for j in 0..m {
                padded[(i, j)] = c[(i, j)];
            }
        }
        padded
    } else {
        c.clone()
    };

    let svd = c_for_svd.svd(true, true);
    let sigma = &svd.singular_values;
    let u = svd.u.as_ref().expect("SVD computed with u=true");
    let vt = svd.v_t.as_ref().expect("SVD computed with v_t=true");

    // Step 2: Rank detection.
    let sigma_max = sigma.iter().cloned().fold(0.0_f64, f64::max);
    let threshold = sigma_max * EPS_RANK_THRESHOLD;
    let rank = sigma.iter().filter(|&&s| s > threshold).count();

    // Step 3: Consistency check.
    let mut d_proj = DVector::zeros(p);
    for i in 0..rank {
        let ui_full = u.column(i);
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        d_proj += coeff * DVector::from_fn(p, |k, _| ui_full[k]);
    }
    let residual = (d - &d_proj).norm();
    if residual > EPS_CONSISTENCY {
        return None;
    }

    // Step 4: Particular solution (minimum-norm via pseudoinverse).
    let mut x0 = DVector::zeros(m);
    for i in 0..rank {
        let ui_full = u.column(i);
        let vi = vt.row(i).transpose();
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        x0 += (coeff / sigma[i]) * &vi;
    }

    // Step 5: Null basis -- columns of V for zero singular values.
    let null_dim = m - rank;
    let null_basis = if null_dim > 0 {
        let mut basis = DMatrix::zeros(m, null_dim);
        for j in 0..null_dim {
            let row = vt.row(rank + j);
            for i in 0..m {
                basis[(i, j)] = row[i];
            }
        }
        basis
    } else {
        DMatrix::zeros(m, 0)
    };

    Some(ConstraintSolution {
        x0,
        null_basis,
        rank,
    })
}

// ══════════════════════════════════════════════════════════════════════════════
// Beta feasibility — max-margin LP (from beta_feasibility.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Find the point in {beta0 + V * alpha} with maximum minimum component.
///
/// Chebyshev center LP via clarabel interior-point solver (through `good_lp`).
pub fn find_max_margin(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let m = beta0.len();
    let k = null_basis.ncols();

    let mut vars = variables!();

    // Variables alpha_1..alpha_k: unbounded.
    let alpha_vars: Vec<_> = (0..k)
        .map(|_| vars.add(variable().min(f64::NEG_INFINITY)))
        .collect();

    // Variable t: unbounded (objective: maximize t).
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let objective: Expression = t_var.into();

    let mut model = vars.maximise(objective).using(default_solver);

    // Constraints: for each j, sum_i V[j,i] * alpha_i - t >= -beta0[j]
    for j in 0..m {
        let mut lhs: Expression = Expression::from(0.0);
        for i in 0..k {
            let coeff = null_basis[(j, i)];
            if coeff != 0.0 {
                lhs += coeff * alpha_vars[i];
            }
        }
        lhs -= t_var;
        model = model.with(constraint!(lhs >= -beta0[j]));
    }

    match model.solve() {
        Ok(solution) => {
            let alpha = DVector::from_fn(k, |i, _| {
                let val: f64 = solution.value(alpha_vars[i]);
                val
            });
            let beta = beta0 + null_basis * &alpha;
            let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
            MarginResult {
                margin,
                alpha,
                beta,
            }
        }
        Err(_) => {
            // Solver error. Shouldn't happen for our formulation, but handle gracefully.
            let beta = beta0.clone();
            let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
            MarginResult {
                margin,
                alpha: DVector::zeros(k),
                beta,
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Saddle-point solver (from saddle_point_solver.rs)
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
        "KKT matrix has all eigenvalues < {:.0e} (max |λ| = {:.2e}). \
         This means both H ≈ 0 and C ≈ 0 — garbage input, not a valid QP.",
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

    // Pseudoinverse solution: x_hat = sum_i (v_i . b / lambda_i) v_i for retained eigenvalues.
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

    // ||P_discard b||: b = (0_m, 0_4, 1), so b-component of discarded eigenvector v_i
    // is v_i[m+4]. ||P_discard b||^2 = sum |v_i[m+4]|^2 over discarded eigenvectors.
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
        return None; // Tier fallback: try next threshold.
    }

    // Q error bound computation ([lem:q-error-bound]).
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

    // Helper: set p_discard_b_norm on feasible results.
    let set_p_discard = |outcome: KktOutcome| -> KktOutcome {
        match outcome {
            KktOutcome::Feasible(mut r) => { r.p_discard_b_norm = p_discard_b_norm; KktOutcome::Feasible(r) }
            other => other,
        }
    };

    // If already feasible (all beta > EPS), compute error bound and return.
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, &eigen_info)));
    }

    // Full rank at this threshold: unique solution.
    if rank == size {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, &eigen_info)));
        }
        return Some(KktOutcome::BetaNonPositive);
    }

    // Rank-deficient: search the numerical null space for beta > 0.
    let mut null_columns: Vec<DVector<f64>> = Vec::new();
    for i in 0..size {
        if eigenvalues[i].abs() <= threshold {
            let v_beta = DVector::from_fn(m, |j, _| eigenvectors[(j, i)]);
            if v_beta.norm() >= EPS_TYPE_A_FILTER {
                // Type C check: constraint violation must be O(|lambda|), not O(1).
                let mut constraint_violation_sq = 0.0;
                for row in m..size {
                    let dot: f64 = (0..m).map(|j| kkt[(row, j)] * eigenvectors[(j, i)]).sum();
                    constraint_violation_sq += dot * dot;
                }
                let constraint_violation = constraint_violation_sq.sqrt();
                assert!(
                    constraint_violation < 0.1,
                    "Type C eigenvector detected: ||constraint * v_beta|| = {:.2e}, \
                     |lambda| = {:.2e}, ||v_beta|| = {:.2e}, m = {}. \
                     This was expected to be O(|lambda|) but is O(1).",
                    constraint_violation, eigenvalues[i].abs(), v_beta.norm(), m
                );
                null_columns.push(v_beta);
            }
        }
    }

    let k_eff = null_columns.len();

    // Fast path: if all null-space directions were Type A (k_eff=0),
    // use beta0 directly.
    if k_eff == 0 {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(set_p_discard(finalize_result(&beta0, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, &eigen_info)));
        } else {
            return Some(KktOutcome::BetaNonPositive);
        }
    }

    let mut null_basis = DMatrix::zeros(m, k_eff);
    for (col, v) in null_columns.iter().enumerate() {
        null_basis.set_column(col, v);
    }

    let beta0_dv = DVector::from_column_slice(&beta0);
    let margin_result = find_max_margin(&beta0_dv, &null_basis);

    // Accept the LP result, projecting back onto {Cβ = d} if needed.
    //
    // The LP shifts β₀ along approximate null-space directions (discarded eigenvectors
    // of M). These directions have small but nonzero eigenvalues, so the shift introduces
    // a constraint violation proportional to |shift| × |λ_discarded|. When the shift is
    // large (e.g., ~2) and the eigenvalue is small (e.g., ~1e-4), the violation can be
    // too large for the old tolerance check.
    //
    // Fix attempt: project the LP result back onto {Cβ = d} using C's SVD. This preserves
    // β > 0 (approximately) while restoring constraint feasibility. The projection subtracts
    // C^+(Cβ_lp - d) from β_lp, where C^+ is the pseudoinverse of C.
    //
    // Result: fixes 2/26 false negatives (stress-test). Doesn't fix the 9 natural polytope
    // false negatives because the approximate null-space direction is NOT in null(C) — projecting
    // back to Cβ = d pulls β back toward the boundary. A more fundamental fix would be to use
    // the projection solver approach (solve C first, then optimize H in null(C)).
    let beta_final = if margin_result.margin > -EPS_BETA_POSITIVE {
        let lp_beta_dv = margin_result.beta.clone();
        let lp_constraint_residual = extract_constraint_residual(kkt, lp_beta_dv.as_slice(), m);

        if lp_constraint_residual <= EPS_KKT_RESIDUAL {
            // Constraints already satisfied — use LP result directly.
            lp_beta_dv.as_slice().to_vec()
        } else {
            // Project LP result back onto {Cβ = d}.
            // Extract C from the KKT matrix (rows m..m+5, cols 0..m).
            let c_mat = DMatrix::from_fn(5, m, |i, j| kkt[(m + i, j)]);
            let d_vec = DVector::from_fn(5, |i, _| if i == 4 { 1.0 } else { 0.0 });
            let c_beta = &c_mat * &lp_beta_dv;
            let residual = &c_beta - &d_vec;

            // Compute C^+ * residual via SVD.
            let svd = c_mat.svd(true, true);
            let correction = svd.solve(&residual, 1e-12).unwrap_or_else(|_| DVector::zeros(m));
            let projected = &lp_beta_dv - &correction;

            let proj_residual = extract_constraint_residual(kkt, projected.as_slice(), m);
            let proj_margin = projected.iter().copied().fold(f64::INFINITY, f64::min);

            if proj_residual <= EPS_KKT_RESIDUAL && proj_margin > -EPS_BETA_POSITIVE {
                // Projection restored feasibility while keeping β > 0.
                projected.as_slice().to_vec()
            } else {
                // Projection didn't help — fall back to β₀.
                beta0.clone()
            }
        }
    } else if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        beta0.clone()
    } else {
        return Some(KktOutcome::BetaNonPositive);
    };

    let beta0_ref = beta0;

    // Verify constraints on the final beta.
    let constraint_residual_norm = extract_constraint_residual(kkt, &beta_final, m);
    // NOTE: In the experiment, we do NOT panic on large constraint residuals.
    // We record the result and compare against exact ground truth.
    if constraint_residual_norm > EPS_KKT_RESIDUAL {
        eprintln!(
            "KKT constraint residual too large after LP: ||r|| = {:.2e} > {:.2e}",
            constraint_residual_norm, EPS_KKT_RESIDUAL
        );
    }

    // Compute Q from pseudoinverse beta0, not LP-shifted beta_final.
    Some(match set_p_discard(finalize_result(&beta0_ref, mu0, xi0, kkt, m, q_correction, residual_norm, abs_lambda_min, &eigen_info)) {
        KktOutcome::Feasible(mut result) => {
            result.beta = beta_final;
            KktOutcome::Feasible(result)
        }
        other => other,
    })
}

/// Compute the constraint residual for the beta vector using the KKT matrix structure.
fn extract_constraint_residual(kkt: &DMatrix<f64>, beta: &[f64], m: usize) -> f64 {
    let mut sq_sum = 0.0;
    for row in m..(m + 5) {
        let rhs_val = if row == m + 4 { 1.0 } else { 0.0 };
        let dot: f64 = (0..m).map(|j| kkt[(row, j)] * beta[j]).sum();
        sq_sum += (dot - rhs_val).powi(2);
    }
    sq_sum.sqrt()
}

/// Build the final KktResult with Q computation and error bound assertion.
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
    // Compute Q = (1/2) beta^T H beta using the top-left m x m block of the KKT matrix.
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;

    let q_corrected = q_raw + q_correction;

    // NOTE: Previously returned Infeasible when q_corrected <= 0. Removed:
    // Q sign is the caller's decision. The solver reports the stationary point
    // regardless of Q sign. This lets the experiment study Q ≤ 0 cases.

    // Tight bound: E = (9/2) ||r||^2 / |lambda_min|.
    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    // NOTE: In the experiment, we do NOT panic on large error bounds.
    // We record the result and compare against exact ground truth.
    // The library code panics here; we want to measure actual errors instead.

    KktOutcome::Feasible(KktResult {
        beta: beta.to_vec(),
        beta0: beta.to_vec(), // Will be overwritten if LP shift happens
        mu,
        xi,
        q_raw,
        q_corrected,
        q_error_bound,
        n_positive: eigen_info.n_positive,
        n_negative: eigen_info.n_negative,
        n_zero: eigen_info.n_zero,
        p_discard_b_norm: 0.0, // Set by caller (try_pseudoinverse_with_threshold)
    })
}

// ══════════════════════════════════════════════════════════════════════════════
// Projection solver (from projection_solver.rs) — WITH SIGN FIX
// ══════════════════════════════════════════════════════════════════════════════

/// Solve the QP via constraint projection.
///
/// **Sign fix applied:** The library computes alpha0 = (H')^+ g, but the correct
/// stationarity condition H' alpha + g = 0 requires alpha0 = -(H')^+ g.
/// This function implements the corrected version.
pub fn solve_projected(qp: &QP) -> Solution {
    let m = qp.c.ncols();

    // Step 1: Solve constraints.
    let constraint_sol = match solve_constraints(&qp.c, &qp.d) {
        Some(sol) => sol,
        None => {
            return Solution {
                verdict: Verdict::False,
                q: 0.0,
                beta: vec![0.0; m],
                margin: f64::NEG_INFINITY,
            };
        }
    };

    let beta0 = &constraint_sol.x0;
    let v = &constraint_sol.null_basis;
    let k = v.ncols();

    // Special case: k = 0 (unique beta from constraints).
    if k == 0 {
        let q = q_value_from_dvec(&qp.h, beta0);
        let margin = beta0.iter().copied().fold(f64::INFINITY, f64::min);
        let verdict = classify_margin(margin);
        return Solution {
            verdict,
            q,
            beta: beta0.as_slice().to_vec(),
            margin,
        };
    }

    // Step 2: Project and optimize.
    // Reduced Hessian: H' = V^T H V (k x k symmetric).
    let hv = &qp.h * v;
    let h_prime = v.transpose() * &hv;

    // Reduced gradient: b' = V^T H beta0 (k x 1).
    let h_beta0 = &qp.h * beta0;
    let b_prime = v.transpose() * &h_beta0;

    // Eigendecompose H' = P Lambda P^T.
    let eig = h_prime.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    // Partition eigenvalues into retained and null.
    let lambda_max = eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max);
    let threshold = if lambda_max < EPS_EIGEN_FLOOR {
        f64::INFINITY // nothing retained
    } else {
        lambda_max * EPS_EIGEN_THRESHOLD
    };

    // SIGN FIX: alpha0 = -(H')^+ g (negated vs library which computes (H')^+ g).
    // Stationarity condition: H' alpha + g = 0  =>  alpha = -(H')^+ g.
    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = -pi.dot(&b_prime) / eigenvalues[i]; // NEGATED
            alpha0 += coeff * &pi;
        }
    }

    // Null-space directions of H' (columns of W in alpha-space).
    let null_indices: Vec<usize> = (0..k)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .collect();

    // Step 3: Compose search space.
    let beta_base = beta0 + v * &alpha0;

    let n_null = null_indices.len();
    let v_search = if n_null > 0 {
        let mut w_alpha = DMatrix::zeros(k, n_null);
        for (j, &idx) in null_indices.iter().enumerate() {
            let col = eigenvectors.column(idx);
            for i in 0..k {
                w_alpha[(i, j)] = col[i];
            }
        }
        v * w_alpha
    } else {
        DMatrix::zeros(m, 0)
    };

    // Step 4: Max-margin search.
    let margin_result = find_max_margin(&beta_base, &v_search);

    // Step 5: Compute Q.
    let q = q_value_from_dvec(&qp.h, &margin_result.beta);
    let margin = margin_result.margin;
    let verdict = classify_margin(margin);

    Solution {
        verdict,
        q,
        beta: margin_result.beta.as_slice().to_vec(),
        margin,
    }
}

/// Compute Q = (1/2) beta^T H beta from DVector (internal helper).
fn q_value_from_dvec(h: &DMatrix<f64>, beta: &DVector<f64>) -> f64 {
    0.5 * beta.dot(&(h * beta))
}
