//! Eigendecomposition-based KKT solver for the (m+5)x(m+5) saddle-point system.
//!
//! Solves the augmented KKT system M x = b where M is the symmetric saddle-point
//! matrix assembled by `qp_assembly::build_augmented_system`. The solution vector
//! x = [beta; mu; xi] yields dwell times beta, Lagrange multipliers mu (closure)
//! and xi (normalization).
//!
//! Key features:
//! - Two-tier eigenvalue rank detection (permissive then strict)
//! - Numerical null-space search for beta > 0 when rank-deficient
//! - Q error bound computation via [lem:q-error-bound]
//! - Inertia reporting for saddle-point structure analysis
//!
//! **Near-zero Q orbits:** Some (S,σ) pairs yield Q ≈ 0 (very high action). The error
//! bound E is valid but may exceed |Q| itself (relative error > 100%). This is harmless:
//! the capacity algorithm picks max Q, so near-zero Q orbits never win. The absolute
//! threshold `E < 1e-6` is chosen relative to Q_max ≈ O(1), not relative to each orbit's Q.
//!
//! **Sign convention:** Q > 0 when σ follows the positive Reeb direction (where
//! consecutive facets satisfy ω₀(n_{σ(k)}, n_{σ(k+1)}) ≥ 0). Callers pass
//! permutations in natural order directly — no reversal needed.
//!
//! Mathematical correspondence: [lem:kkt], [lem:q-error-bound]

use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::omega0;
use super::qp_assembly::build_augmented_system;
use nalgebra::{DMatrix, DVector, Vector4};

// ── Public constants ──

/// Minimum beta_i value to consider a solution certified positive.
///
/// Used by the accumulator and experiments to classify solution feasibility.
/// beta_i > EPS_BETA_POSITIVE means the component is unambiguously positive.
///
/// **Why 1e-12:** This filters f64 eigensolver noise. The KKT matrix entries are
/// O(1) (normals are unit vectors, heights normalized), so eigenvector components
/// are O(1) and beta values from the pseudoinverse are O(1). Machine epsilon is
/// ~1e-16; numerical roundoff in eigendecomposition accumulates to ~1e-12 for
/// (m+5) x (m+5) matrices with m up to 16. A value of 1e-12 is:
/// - Far above machine epsilon (can't be confused with exact zero)
/// - Far below typical beta values (O(0.1)--O(10)) for real orbits
/// - 10x tighter than EPS_MARGIN_TRUE (1e-9) so Indeterminate verdicts are
///   returned for any solution where beta is ambiguous.
/// Making it 10x larger (1e-11) would misclassify some real near-zero betas as
/// positive. Making it 10x smaller (1e-13) would pass some eigensolver noise
/// through as certified solutions.
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(beta) value to consider a solution meaningful.
///
/// Avoids division-by-near-zero when computing capacity = 1/(2Q).
///
/// **Why 1e-15:** Q = c_EHZ^{-2} / 2 is O(0.01)--O(10) for our polytopes
/// (typical c_EHZ ~ 0.3--3). Q < 1e-15 indicates either a degenerate orbit
/// with astronomically high action or pure f64 noise. In either case, this
/// orbit cannot be the capacity maximizer. 1e-15 is just above machine epsilon
/// (~1e-16) to avoid exact-zero false positives from cancellation.
pub const EPS_Q_POSITIVE: f64 = 1e-15;

// ── Internal constants ──

/// Absolute floor for eigenvalue magnitude. If the largest eigenvalue is below
/// this, the entire matrix is treated as numerically zero (early return).
///
/// **Why 1e-12:** The KKT matrix entries are O(1). A largest eigenvalue below
/// 1e-12 means the matrix is numerically zero (all eigenvalues in machine-noise
/// range). The relative rank detection is handled by EIGEN_CONDITION_TAU; this
/// absolute floor guards against the degenerate case before any ratio is computed.
/// Making it 10x larger (1e-11) risks discarding matrices that are genuinely
/// non-zero but small; 10x smaller (1e-13) risks attempting rank detection on
/// a pure-noise matrix.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

/// Condition-number threshold for eigenvalue rank detection.
///
/// An eigenvalue lambda_j is "small" if |lambda_j| < |lambda|_max * tau.
/// This catches both isolated small eigenvalues (gap case) and gradual decay.
///
/// **Why 1e-3:** The degenerate (4,4) Lagrangian product at theta ~ 0 deg has
/// eigenvalue magnitudes around 8.6e-4 with |lambda|_max ~ 1-2, giving
/// |lambda|/|lambda|_max ~ 4e-4. The threshold 1e-3 catches this with 2.5x
/// margin. Well-conditioned random polytopes have smallest |lambda| ~ 0.01-0.1,
/// well above 1e-3 * |lambda|_max, so no false rank-deficiency detections occur.
/// Making it 10x larger (1e-2) would treat some well-conditioned polytopes as
/// rank-deficient. Making it 10x smaller (1e-4) would miss the degenerate case.
///
/// Calibrated to detect the degenerate (4,4) Lagrangian product at theta ~ 0 deg,
/// which has eigenvalue magnitudes around 8.6e-4 with |lambda|_max ~ 1-2.
/// Well-conditioned polytopes have smallest |lambda| ~ 0.01-0.1.
///
/// Regression tests: eigen_gap_ratio_44_degenerate, eigen_gap_ratio_44_theta43.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm for the KKT solution.
///
/// Rejects numerically poor solutions where ||Mx - b|| is too large. Solutions
/// with residual below this have Q error bounds E = 4.5 * ||r||^2 / |lambda_min|
/// that are small relative to Q values of interest.
///
/// **Why 1e-6:** The q_error experiment (Part 1) measures worst-case E = 2.9e-11
/// across 1.1M nodes (F <= 10) with typical residuals ~1e-14 to ~1e-10.
/// Residuals above 1e-6 signal genuine numerical failure (e.g., extremely
/// ill-conditioned matrices). At residual = 1e-6 with |lambda_min| = 1e-3,
/// the error bound is E = 4.5e-9, which is ~5 orders of magnitude below
/// the observed worst case. Making it 10x larger (1e-5) risks accepting poor
/// solutions; 10x smaller (1e-7) would reject some valid solutions on
/// moderately ill-conditioned polytopes.
const EPS_KKT_RESIDUAL: f64 = 1e-6;

// ── Public types ──

/// Eigendecomposition info for the KKT matrix M.
///
/// Groups the eigenvalues, eigenvectors, and inertia from the symmetric
/// eigendecomposition M = V Lambda V^T. Used internally by the two-tier solver.
pub(crate) struct EigenInfo {
    /// Eigenvalues of M.
    pub eigenvalues: DVector<f64>,
    /// Orthogonal eigenvectors of M (columns).
    pub eigenvectors: DMatrix<f64>,
    /// Number of positive eigenvalues (using strict threshold).
    pub n_positive: usize,
    /// Number of negative eigenvalues (using strict threshold).
    pub n_negative: usize,
    /// Number of near-zero eigenvalues (using strict threshold).
    pub n_zero: usize,
}

/// Result of the saddle-point KKT solve with diagnostics.
///
/// Contains the solution beta, residual-corrected Q value with error bound,
/// and inertia of the KKT matrix M.
///
/// See [lem:q-error-bound] (thesis): |Q(beta_0) - q_corrected| <= q_error_bound.
#[derive(Clone, Debug)]
pub struct KktResult {
    /// Optimal beta vector (all components > -EPS_BETA_POSITIVE).
    pub beta: Vec<f64>,
    /// Residual-corrected Q value: Q_tilde = Q(beta_hat) + (r2^T mu_hat + r3 * xi_hat).
    /// See [eq:q-corrected] (thesis).
    pub q_corrected: f64,
    /// Error bound E on Q_tilde: |Q(beta_0) - Q_tilde| <= E.
    /// See [eq:q-error-bound] (thesis).
    #[allow(dead_code)]
    pub q_error_bound: f64,
    /// Inertia of M: number of positive eigenvalues.
    #[allow(dead_code)]
    pub n_positive: usize,
    /// Inertia of M: number of negative eigenvalues.
    #[allow(dead_code)]
    pub n_negative: usize,
    /// Inertia of M: number of near-zero eigenvalues.
    #[allow(dead_code)]
    pub n_zero: usize,
}

// ── Public API ──

/// Solve the augmented KKT system from a pre-assembled matrix and RHS.
///
/// Uses eigendecomposition with two-tier rank detection:
/// 1. Permissive (EPS_EIGEN_FLOOR = 1e-12): retains all but numerically zero eigenvalues.
/// 2. Strict (EIGEN_CONDITION_TAU = 1e-3): treats small eigenvalues as null space.
///
/// The permissive tier runs first. If its residual exceeds EPS_KKT_RESIDUAL,
/// the strict tier takes over. This replaces the old LU + SVD fallback with a
/// single factorization.
///
/// Returns `Some(KktResult)` with beta, corrected Q, error bound, and inertia,
/// or `None` if no admissible solution exists.
///
/// [lem:kkt]: KKT conditions characterize the EHZ capacity optimum as a saddle point.
pub fn solve_saddle_point(
    kkt_matrix: &DMatrix<f64>,
    rhs: &DVector<f64>,
) -> Option<KktResult> {
    let m = rhs.len() - 5;
    let size = rhs.len();

    let eig = kkt_matrix.clone().symmetric_eigen();
    let max_abs_ev = eig.eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
    if max_abs_ev < EPS_EIGEN_FLOOR {
        return None;
    }

    // Compute inertia using the strict threshold (for structure analysis).
    // The KKT matrix M is (m+5)×(m+5). The constraint block contributes at most 5
    // negative eigenvalues, but H (the action matrix) can also have negative eigenvalues,
    // so n_negative can exceed 5. Empirically validated by q_error experiment (Tables 8-9).
    let strict_threshold = max_abs_ev * EIGEN_CONDITION_TAU;
    let eigen_info = EigenInfo {
        n_positive: eig.eigenvalues.iter().filter(|&&e| e > strict_threshold).count(),
        n_negative: eig.eigenvalues.iter().filter(|&&e| e < -strict_threshold).count(),
        n_zero: size - eig.eigenvalues.iter().filter(|&&e| e > strict_threshold).count()
            - eig.eigenvalues.iter().filter(|&&e| e < -strict_threshold).count(),
        eigenvalues: eig.eigenvalues,
        eigenvectors: eig.eigenvectors,
    };

    // Tier 1: Permissive threshold — retain all eigenvalues above machine-epsilon floor.
    let result = try_pseudoinverse_with_threshold(
        kkt_matrix, rhs, m,
        &eigen_info, EPS_EIGEN_FLOOR,
    );
    if result.is_some() {
        return result;
    }

    // Tier 2: Strict threshold — treat small eigenvalues as null space.
    try_pseudoinverse_with_threshold(
        kkt_matrix, rhs, m,
        &eigen_info, strict_threshold,
    )
}

/// Convenience: solve KKT for a polytope and permutation in one call.
///
/// Assembles the augmented system from `qp_assembly::build_augmented_system`,
/// then calls `solve_saddle_point`.
///
/// [lem:kkt]: assembles and solves the augmented KKT system for a (polytope, permutation) pair.
pub fn solve_kkt_for(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult> {
    let (kkt, rhs) = build_augmented_system(polytope, perm);
    solve_saddle_point(&kkt, &rhs)
}

// ── Internal helpers ──

/// Compute Q(beta) = sum_{i>j} beta_i * beta_j * omega_0(n_{sigma(j)}, n_{sigma(i)}).
///
/// This is the action sum (1/2) beta^T H beta computed directly from normals
/// and the antisymmetric omega_0 form. Used for Q computation from the beta
/// solution vector. Uses omega_0 directly (not the symmetric H matrix).
///
/// [lem:H-quadratic]: Q(beta) = sum_{i>j} beta_i beta_j omega_0(n_{sigma(j)}, n_{sigma(i)}).
#[allow(dead_code)]
pub(crate) fn q_from_beta(
    normals: &[Vector4<f64>],
    perm: &[usize],
    beta: &[f64],
) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

/// Try to find an admissible beta > 0 solution using a specific eigenvalue threshold.
///
/// Computes the pseudoinverse retaining eigenvalues with |lambda_i| > threshold,
/// checks the residual, searches the null space if rank-deficient, and computes
/// the Q error bound.
///
/// Returns None if: residual too large, beta <= 0 with full rank, or null space
/// search fails.
fn try_pseudoinverse_with_threshold(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    m: usize,
    eigen_info: &EigenInfo,
    threshold: f64,
) -> Option<KktResult> {
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

    let residual_vec = kkt * &x0 - rhs;
    let residual_norm = residual_vec.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return None;
    }

    // Q error bound computation (Algorithm [alg:q-error-bound]).
    // Solution vector is [beta_hat; mu_hat; xi_hat].
    // Q_tilde = Q(beta_hat) + (r2^T mu_hat + r3 * xi_hat).
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x0[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x0[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;

    // |lambda_min| of RETAINED eigenvalues.
    let abs_lambda_min = eigenvalues
        .iter()
        .filter(|&&e| e.abs() > threshold)
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();

    // If already feasible (all beta > EPS), compute error bound and return.
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        return finalize_result(&beta0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info);
    }

    // Full rank at this threshold: unique solution. If some beta near zero,
    // still accept as uncertain candidate for the accumulator.
    if rank == size {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return finalize_result(&beta0, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info);
        }
        return None;
    }

    // Rank-deficient: search the *numerical* null space for beta > 0.
    // "Null space" here means eigenvectors whose eigenvalues are below the
    // threshold — not an exact kernel (which doesn't exist in f64). The
    // threshold is chosen so that these directions have negligible effect on
    // the KKT objective Q, bounded by [lem:q-error-bound].
    let null_beta: Vec<Vec<f64>> = (0..size)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .map(|i| (0..m).map(|j| eigenvectors[(j, i)]).collect())
        .collect();

    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])
    } else {
        find_positive_beta_nd(&beta0, &null_beta)
    };

    // Save beta0 for the Q constancy debug_assert after null-space shift.
    let beta0_ref = beta0.clone();

    // Use null-space result if found, else fall back to beta0 if above -EPS.
    let beta_final = match beta_opt {
        Some(beta) => beta,
        None => {
            if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
                beta0
            } else {
                return None;
            }
        }
    };

    // Q is constant along the null space ([lem:well-defined]): null-space
    // directions preserve constraints, so the KKT objective is invariant.
    // Verify this numerically — any disagreement indicates a bug in the
    // null-space extraction or the shift computation.
    #[cfg(debug_assertions)]
    {
        let mut q_final = 0.0_f64;
        let mut q_initial = 0.0_f64;
        for i in 0..m {
            for j in 0..m {
                q_final += beta_final[i] * kkt[(i, j)] * beta_final[j];
                q_initial += beta0_ref[i] * kkt[(i, j)] * beta0_ref[j];
            }
        }
        q_final *= 0.5;
        q_initial *= 0.5;
        // When both Q values are near-zero, the difference is pure f64 noise
        // and the capacity algorithm discards these solutions anyway (Q << 1
        // yields enormous action, never competitive). Only assert Q constancy
        // when Q is meaningfully nonzero.
        // Threshold 1e-6: meaningful Q values are O(0.01)--O(10). Near-zero
        // Q orbits (Q < 1e-6) have enormous action and never win the capacity
        // competition, so Q constancy noise there is harmless. The old solver
        // used the same threshold in its constancy check.
        let q_scale = q_initial.abs().max(q_final.abs());
        if q_scale > 1e-6 {
            assert!(
                (q_final - q_initial).abs() < 1e-8 * q_scale,
                "Q changed along null space: Q(beta0)={q_initial}, Q(beta_final)={q_final}"
            );
        }
    }

    // Constraint verification for null-space-shifted solutions.
    let full_x = {
        let mut v = DVector::zeros(size);
        for i in 0..m {
            v[i] = beta_final[i];
        }
        for i in m..size {
            v[i] = x0[i];
        }
        v
    };
    let _constraint_residual = (kkt * &full_x - rhs).norm();
    // Use a looser check: verify the beta constraints Cx=d hold, not the full KKT residual.
    // The Lagrange multiplier rows may not hold exactly after null-space shift.
    // Instead, check just the constraint rows (rows m..m+5) directly.
    let constraint_residual_norm = extract_constraint_residual(kkt, &beta_final, m);
    if constraint_residual_norm > EPS_KKT_RESIDUAL {
        return None;
    }

    finalize_result(&beta_final, kkt, m, q_correction, residual_norm, abs_lambda_min, eigen_info)
}

/// Compute the constraint residual for the beta vector using the KKT matrix structure.
///
/// The constraint rows are rows m..m+5 of the KKT matrix. They encode
/// N^T beta = 0 (rows m..m+4) and eta^T beta = 1 (row m+4).
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
    kkt: &DMatrix<f64>,
    m: usize,
    q_correction: f64,
    residual_norm: f64,
    abs_lambda_min: f64,
    eigen_info: &EigenInfo,
) -> Option<KktResult> {
    // Compute Q = (1/2) beta^T H beta using the top-left m x m block of the KKT matrix.
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;

    let q_corrected = q_raw + q_correction;

    // Tight bound: E = (9/2) ||r||^2 / |lambda_min|.
    // 4.5 = 9/2 comes from [lem:q-error-bound] (thesis): the KKT block structure
    // identity delta_beta^T H delta_beta = delta_x^T M delta_x - 2 r2^T delta_mu
    // - 2 r3 delta_xi removes the ||H||/|lambda_min|^2 term, leaving only the
    // quadratic term (9/2) ||r||^2 / |lambda_min|. The factor 9 comes from the
    // Cauchy-Schwarz bound on the two-variable quadratic form in the residual.
    // See [lem:q-error-bound] (thesis).
    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    // Calibration: q-error experiment (Part 1) measures worst-case E = 2.9e-11
    // across 1.1M nodes (F ≤ 10). The 1e-6 threshold is ~5 orders of magnitude
    // above observed values. If this fires on larger polytopes (F > 16),
    // re-measure before widening.
    assert!(
        q_error_bound < 1e-6,
        "Q error bound unexpectedly large: E={:.2e}, |r|={:.2e}, |lambda_min|={:.2e}",
        q_error_bound, residual_norm, abs_lambda_min
    );
    assert!(
        q_correction.abs() < 1e-6 || q_correction.abs() < 1e-6 * q_raw.abs(),
        "Q correction unexpectedly large: correction={:.2e}, Q_raw={:.2e}, ratio={:.2e}",
        q_correction, q_raw, q_correction.abs() / q_raw.abs().max(1e-30)
    );

    Some(KktResult {
        beta: beta.to_vec(),
        q_corrected,
        q_error_bound,
        n_positive: eigen_info.n_positive,
        n_negative: eigen_info.n_negative,
        n_zero: eigen_info.n_zero,
    })
}

/// Search null space for beta with maximum margin (1D null space case).
///
/// Given particular solution beta0 and null space vector v, find alpha such that
/// beta0 + alpha * v has maximum min(beta_j). Returns the midpoint of the
/// feasible interval for numerical stability.
///
/// Accepts results with all beta > -EPS (uncertain candidates). The caller
/// classifies beta > +EPS as certified vs beta in (-EPS, +EPS] as uncertain.
fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);

    for j in 0..m {
        if v[j].abs() < 1e-15 {
            // Component below numerical zero for a unit-scale eigenvector.
            // This component is fixed at beta0[j].
            if beta0[j] <= -EPS_BETA_POSITIVE {
                return None;
            }
        } else {
            let bound = -beta0[j] / v[j];
            if v[j] > 0.0 {
                lo = lo.max(bound);
            } else {
                hi = hi.min(bound);
            }
        }
    }

    if lo >= hi {
        return None;
    }

    // Midpoint maximizes minimum distance to interval endpoints.
    let alpha = if lo.is_finite() && hi.is_finite() {
        (lo + hi) / 2.0
    } else if lo.is_finite() {
        lo + 1.0
    } else if hi.is_finite() {
        hi - 1.0
    } else {
        0.0
    };

    let beta: Vec<f64> = (0..m).map(|j| beta0[j] + alpha * v[j]).collect();
    if beta.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

/// Search null space for beta with maximum margin (multi-dimensional case).
///
/// Uses iterative coordinate ascent on the most-violated constraint.
/// 100 iterations suffice: each step pushes the worst component to +EPS,
/// and with k null-space dimensions there are at most m constraints.
fn find_positive_beta_nd(beta0: &[f64], null_vecs: &[Vec<f64>]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![0.0; k];

    for _iter in 0..100 {
        let beta: Vec<f64> = (0..m)
            .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
            .collect();

        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        if *worst_val > EPS_BETA_POSITIVE {
            return Some(beta);
        }

        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            return if *worst_val > -EPS_BETA_POSITIVE {
                Some(beta)
            } else {
                None
            };
        }

        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }

    // Final feasibility check.
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}
