/// Shared KKT solver for capacity algorithms.
///
/// Solves the constrained optimization max Q(β) subject to N^T β = 0, η^T β = 1,
/// where Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
///
/// Used by both the hk2017 (general) and billiard (Lagrangian product) algorithms.
/// Previously duplicated across both crates (294 LOC each); now unified here.
///
/// # Known edge cases
///
/// **Near-zero Q orbits:** Some (S,σ) pairs yield Q ≈ 0 (very high action). The error
/// bound E is valid but may exceed |Q| itself (relative error > 100%). This is harmless:
/// the capacity algorithm picks max Q, so near-zero Q orbits never win. The absolute
/// threshold `E < 1e-6` is chosen relative to Q_max ≈ O(1), not relative to each orbit's Q.
///
/// **Sign convention:** This module uses the SYMMETRIC KKT matrix convention (both
/// off-diagonal blocks have +n/+h). Solution components for multipliers are NEGATED
/// (μ = -λ, ξ = -ν). Experiments that copy the KKT construction (e.g. sys_optimization.rs)
/// may use the old ASYMMETRIC convention — check their comments.
use crate::geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

/// Minimum β_i value to consider a solution valid (filters numerical noise near zero).
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid (avoids division-by-near-zero in action).
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// Result of KKT solve with eigendecomposition diagnostics.
///
/// See `[lem:q-interval]` (thesis): |Q(β₀) - q_corrected| ≤ q_error_bound.
///
/// The corrected Q absorbs the first-order error from the KKT residual,
/// leaving only a second-order remainder bounded by q_error_bound.
#[derive(Clone, Debug)]
pub struct KktResult {
    /// Optimal β vector (numerical approximation, all components > 0).
    pub beta: Vec<f64>,
    /// Residual-corrected Q value: Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂).
    /// See `[eq:q-corrected]` (thesis).
    pub q_corrected: f64,
    /// Error bound E on Q̃: |Q(β₀) - Q̃| ≤ E.
    /// See `[eq:q-error-bound]` (thesis).
    /// Checked by debug_assert! in solver; available for experiment/accumulator use.
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

/// Absolute floor: if the largest eigenvalue magnitude is below this, the entire
/// matrix is treated as numerically zero (early return). The relative rank
/// detection is handled by EIGEN_CONDITION_TAU.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

/// Condition-number threshold for eigenvalue rank detection: eigenvalues with
/// |λ_i| < max_abs * EIGEN_CONDITION_TAU are treated as part of the null space.
///
/// An eigenvalue λ_j is "small" if |λ_j| < |λ|_max · τ. This detects both
/// isolated small eigenvalues (the classic gap case) and gradual decay
/// to small values (which a gap-ratio approach would miss).
///
/// For near-singular systems, the null space directions are used to search
/// for β > 0, and the Q error bound (see `[lem:q-interval]`) quantifies
/// the resulting objective uncertainty.
///
/// **Why 1e-3:** The degenerate (4,4) Lagrangian product at θ≈0° has
/// eigenvalue magnitudes around 8.6e-4 with |λ|_max ≈ 1–2, giving
/// |λ|/|λ|_max ≈ 4e-4. The threshold 1e-3 catches this with margin.
/// Well-conditioned random polytopes have smallest |λ| ≈ 0.01–0.1,
/// well above 1e-3 · |λ|_max.
///
/// Regression tests: `eigen_gap_ratio_44_degenerate`, `eigen_gap_ratio_44_theta43`.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm for the KKT solution (rejects numerically poor solutions).
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Compute Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
///
/// See `[lem:H-quadratic]` (thesis): Q(β) equals the symplectic action sum.
///
/// **Sign convention:** The thesis writes the sum with j < i in
/// ω₀(n_{σ(j)}, n_{σ(i)}); this code uses i > j (higher index first).
/// Since ω₀ is antisymmetric the two differ by sign: the code maximises
/// Q > 0, which finds the reverse-traversal representative of the orbit.
/// The capacity A = 1/(2Q) is unchanged. See `appendix-notation.tex`
/// ("Permutation orientation") for the full discussion.
///
/// Note: uses ω₀ directly, NOT from H_{ij}. H is symmetric by construction,
/// but Q needs the antisymmetric ω₀ values.
pub fn q_from_beta(
    normals: &[Vector4<f64>],
    perm: &[usize],
    beta: &[f64],
) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum()
}

/// Search null space for β with maximum margin (1D null space case).
///
/// Given particular solution β₀ and null space vector v, find α such that
/// β₀ + α·v has maximum min(β_j). Returns the midpoint of the feasible interval
/// for numerical stability.
///
/// Accepts results with all β > -EPS (uncertain candidates). The caller classifies
/// β > +EPS as certified vs β ∈ (-EPS, +EPS] as uncertain.
///
/// Q(β) is constant along the null space (the null space directions satisfy
/// the KKT stationarity conditions, so the objective doesn't change).
fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);

    for j in 0..m {
        if v[j].abs() < 1e-15 {
            // This component doesn't change with α — reject only if genuinely
            // infeasible (not a floating-point sign ambiguity).
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
        return None; // No feasible α
    }

    // Pick midpoint for maximum margin
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

/// Search null space for β with maximum margin (multi-dimensional null space).
///
/// Uses iterative coordinate ascent on the most-violated constraint.
/// Optimization targets β > +EPS (certified), but accepts β > -EPS (uncertain).
/// The caller classifies the result.
fn find_positive_beta_nd(beta0: &[f64], null_vecs: &[Vec<f64>]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![0.0; k];

    for _iter in 0..100 {
        // Current β
        let beta: Vec<f64> = (0..m)
            .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
            .collect();

        // Find most-violated component
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        if *worst_val > EPS_BETA_POSITIVE {
            return Some(beta); // All β > +EPS (certified)
        }

        // Gradient of β[worst_j] w.r.t. α: g_i = null_vecs[i][worst_j]
        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            // Can't improve this component. Accept if above -EPS (uncertain).
            return if *worst_val > -EPS_BETA_POSITIVE {
                Some(beta)
            } else {
                None
            };
        }

        // Step to push β[worst_j] to a small positive value
        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }

    // Final feasibility check: accept if all β > -EPS (uncertain candidate)
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

/// Build the symmetric KKT matrix and RHS vector for the constrained optimization.
///
/// The KKT system uses negated multipliers (μ = −λ, ξ = −ν) to obtain a
/// **symmetric** saddle-point matrix:
/// ```text
/// [ H   |  N   |  η ] [ β ]   [ 0 ]
/// [ N^T |  0   |  0 ] [ μ ] = [ 0 ]
/// [ η^T |  0   |  0 ] [ ξ ]   [ 1 ]
/// ```
///
/// The stationarity condition is Hβ + Nμ + ηξ = 0 (equivalently Hβ = Nλ + ην
/// with the original multipliers λ = −μ, ν = −ξ).
///
/// Symmetry enables eigendecomposition M = VΛV^T, giving eigenvalues with
/// signs (inertia) and orthogonal eigenvectors in one factorization.
///
/// Returns `(kkt, rhs)` where `kkt` is `(m+5) × (m+5)` symmetric.
///
/// Uses **period normalization** (γ on [0,T]); see `appendix-notation.tex`.
pub fn build_kkt_system(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left: H (m×m) — action matrix with ω₀ values
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }

    // Off-diagonal blocks: N (m×4) and N^T (4×m) — symmetric
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = n;
            kkt[(m + d, i)] = n;
        }
    }

    // Off-diagonal blocks: η (m×1) and η^T (1×m) — symmetric
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = h;
        kkt[(m + 4, i)] = h;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = 1.0;

    (kkt, rhs)
}

/// Eigendecomposition path with two-tier rank detection, inertia, and Q error bounding.
///
/// Since M is symmetric, M = VΛV^T with real eigenvalues and orthogonal
/// eigenvectors. The pseudoinverse solution is x̂ = Σ_i (v_i · b / λ_i) v_i
/// for retained eigenvalues (|λ_i| above threshold).
///
/// **Two-tier rank detection:** The eigendecomposition is computed once, then
/// the pseudoinverse is tried with two thresholds:
///
/// 1. **Permissive** (EPS_EIGEN_FLOOR = 1e-12): retains all but numerically zero
///    eigenvalues. This handles well-conditioned and mildly ill-conditioned systems
///    where the "small" eigenvalues are real, not noise.
///
/// 2. **Strict** (EIGEN_CONDITION_TAU = 1e-3): treats small eigenvalues as null space.
///    This handles genuinely rank-deficient systems (e.g. degenerate Lagrangian
///    products at θ≈0°) where including tiny eigenvalues amplifies noise.
///
/// The permissive tier runs first. If its residual exceeds EPS_KKT_RESIDUAL,
/// the system is near-singular and the strict tier takes over. This replaces
/// the old LU + SVD fallback pattern with a single factorization.
///
/// Returns a `KktResult` with the β vector, corrected Q̃, error bound E
/// satisfying |Q(β₀) - Q̃| ≤ E, and the inertia of M.
/// See `[lem:q-interval]` (thesis).
fn solve_kkt_eigen_path(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<KktResult> {
    let m = perm.len();
    let size = m + 5;

    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    let max_abs_ev = eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
    if max_abs_ev < EPS_EIGEN_FLOOR {
        return None;
    }

    // Inertia uses the strict threshold (for saddle-point structure analysis).
    // The KKT matrix M is (m+5)×(m+5). The constraint block contributes at most 5
    // negative eigenvalues, but H (the action matrix) can also have negative eigenvalues,
    // so n_negative can exceed 5. Empirically validated by q_error experiment (Tables 8-9).
    let strict_threshold = max_abs_ev * EIGEN_CONDITION_TAU;
    let n_positive = eigenvalues.iter().filter(|&&e| e > strict_threshold).count();
    let n_negative = eigenvalues.iter().filter(|&&e| e < -strict_threshold).count();
    let n_zero = size - n_positive - n_negative;

    // Tier 1: Permissive — retain all eigenvalues above machine-epsilon floor.
    // This handles well-conditioned systems and mildly ill-conditioned systems
    // (e.g. symplectomorphism-transformed polytopes with small but real eigenvalues).
    let permissive_threshold = EPS_EIGEN_FLOOR;
    let result = try_pseudoinverse_with_threshold(
        kkt, rhs, normals, heights, perm, eigenvalues, eigenvectors,
        permissive_threshold, n_positive, n_negative, n_zero,
    );
    if result.is_some() {
        return result;
    }

    // Tier 2: Strict — treat small eigenvalues as null space.
    // Triggered when tier 1's residual is too large (system is near-singular).
    // Examples: degenerate (4,4) product at θ≈0° with eigenvalues at ~8.6e-4.
    try_pseudoinverse_with_threshold(
        kkt, rhs, normals, heights, perm, eigenvalues, eigenvectors,
        strict_threshold, n_positive, n_negative, n_zero,
    )
}

/// Try to find an admissible β > 0 solution using a specific eigenvalue threshold.
///
/// Computes the pseudoinverse retaining eigenvalues with |λ_i| > threshold,
/// checks the residual, searches the null space if rank-deficient, and computes
/// the Q error bound.
///
/// Returns None if: residual too large, β ≤ 0 with full rank, or null space
/// search fails.
#[allow(clippy::too_many_arguments)]
fn try_pseudoinverse_with_threshold(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
    eigenvalues: &DVector<f64>,
    eigenvectors: &DMatrix<f64>,
    threshold: f64,
    n_positive: usize,
    n_negative: usize,
    n_zero: usize,
) -> Option<KktResult> {
    let m = perm.len();
    let size = m + 5;

    // Pseudoinverse solution: x̂ = Σ_i (v_i · b / λ_i) v_i for retained eigenvalues.
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

    // --- Q error bound computation (Algorithm [alg:q-interval]) ---
    // The solution vector is [β̂; μ̂; ξ̂] with negated multipliers (μ = -λ, ξ = -ν).
    // Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂)  [Lemma [lem:q-interval], Step 2-3].
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x0[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x0[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;

    // |λ_min| of RETAINED eigenvalues (those above the threshold).
    let abs_lambda_min = eigenvalues
        .iter()
        .filter(|&&e| e.abs() > threshold)
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();

    // If already feasible, compute error bound and return.
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_raw = q_from_beta(normals, perm, &beta0);
        let q_corrected = q_raw - q_correction;
        // Tight bound: E = (9/2) ‖r‖² / |λ_min|.
        // See [lem:q-interval] (thesis): uses KKT block structure identity
        // δβᵀHδβ = δxᵀMδx − 2r₂ᵀδμ − 2r₃δξ to avoid ‖H‖/|λ_min|² term.
        let r_sq = residual_norm * residual_norm;
        let q_error_bound = 4.5 * r_sq / abs_lambda_min;

        assert!(
            q_error_bound < 1e-6,
            "Q error bound unexpectedly large: E={:.2e}, |r|={:.2e}, |λ_min|={:.2e}",
            q_error_bound, residual_norm, abs_lambda_min
        );
        assert!(
            q_correction.abs() < 1e-6 || q_correction.abs() < 1e-6 * q_raw.abs(),
            "Q correction unexpectedly large: correction={:.2e}, Q_raw={:.2e}, ratio={:.2e}",
            q_correction, q_raw, q_correction.abs() / q_raw.abs().max(1e-30)
        );

        return Some(KktResult {
            beta: beta0,
            q_corrected,
            q_error_bound,
            n_positive,
            n_negative,
            n_zero,
        });
    }

    // Full rank at this threshold: unique solution. If any β_i ≤ -EPS,
    // the candidate is genuinely infeasible (not a sign error). If all
    // β_i > -EPS but some β_i ≤ +EPS, this is an UNKNOWN candidate:
    // the caller's uncertain track will handle it.
    if rank == size {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            let q_raw = q_from_beta(normals, perm, &beta0);
            let q_corrected = q_raw - q_correction;
            let r_sq = residual_norm * residual_norm;
            let q_error_bound = 4.5 * r_sq / abs_lambda_min;

            assert!(
                q_error_bound < 1e-6,
                "Q error bound unexpectedly large (full-rank uncertain): E={:.2e}, |r|={:.2e}, |λ_min|={:.2e}",
                q_error_bound, residual_norm, abs_lambda_min
            );

            return Some(KktResult {
                beta: beta0,
                q_corrected,
                q_error_bound,
                n_positive,
                n_negative,
                n_zero,
            });
        }
        return None;
    }

    // Rank-deficient: search null space for β > 0.
    // Null space = eigenvectors for eigenvalues below the threshold.
    // Q(β) is constant along the null space (constraint-preserving directions).
    let null_beta: Vec<Vec<f64>> = (0..size)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .map(|i| (0..m).map(|j| eigenvectors[(j, i)]).collect())
        .collect();

    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])
    } else {
        find_positive_beta_nd(&beta0, &null_beta)
    };

    // If null-space search found β > +EPS, use it. Otherwise fall back to β₀
    // if all β₀ > -EPS (UNKNOWN candidate for the caller's uncertain track).
    // Q is constant along the null space (rem:null-q-constant), so Q(β₀) is
    // correct regardless of which null-space point we use.
    let (beta_final, null_shifted) = match beta_opt {
        Some(beta) => (beta, true),
        None => {
            if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
                (beta0, false)
            } else {
                return None;
            }
        }
    };

    // Constraint verification: reject if β from null space search violates
    // N^T β ≈ 0 or η^T β ≈ 1 (catches noise amplification in degenerate cases).
    let constraint_residual: f64 = (0..4)
        .map(|d| {
            (0..m)
                .map(|i| beta_final[i] * normals[perm[i]][d])
                .sum::<f64>()
        })
        .map(|x: f64| x * x)
        .sum::<f64>()
        + ((0..m)
            .map(|i| beta_final[i] * heights[perm[i]])
            .sum::<f64>()
            - 1.0)
            .powi(2);
    if constraint_residual.sqrt() > EPS_KKT_RESIDUAL {
        return None;
    }

    // Q is constant along the null space, so Q(β_final) = Q(β₀).
    // See [lem:well-defined] (thesis): null-space directions preserve constraints,
    // so the KKT objective value is invariant.
    let q_raw = q_from_beta(normals, perm, &beta_final);
    // Only check constancy when null-space search shifted β (otherwise β_final IS β₀)
    // and when Q is significantly positive (near-zero Q is noise-dominated).
    if null_shifted && q_raw.abs() > 1e-6 {
        let q_raw_beta0 = q_from_beta(normals, perm, &(0..m).map(|i| x0[i]).collect::<Vec<_>>());
        debug_assert!(
            (q_raw - q_raw_beta0).abs() < 1e-3 * q_raw.abs(),
            "Null-space Q constancy violated: Q(β_final)={:.2e}, Q(β₀)={:.2e}, diff={:.2e}, rel={:.2e}",
            q_raw, q_raw_beta0, (q_raw - q_raw_beta0).abs(),
            (q_raw - q_raw_beta0).abs() / q_raw.abs()
        );
    }
    let q_corrected = q_raw - q_correction;
    // Tight bound: E = (9/2) ‖r‖² / |λ_min|.
    // See [lem:q-interval] (thesis): uses KKT block structure identity.
    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    // Threshold 1e-6: q_error experiment (Part 1) measures worst-case E = 2.9e-11
    // across 1.1M nodes (F ≤ 10). Typical E is 1e-16 to 1e-11. The 1e-6 threshold
    // is ~5 orders of magnitude above observed worst case — it catches genuine
    // solver failures without false positives on normal inputs.
    // If this fires on larger polytopes (F > 16), re-measure before widening.
    assert!(
        q_error_bound < 1e-6,
        "Q error bound unexpectedly large (null space path): E={:.2e}, |r|={:.2e}, |λ_min|={:.2e}",
        q_error_bound, residual_norm, abs_lambda_min
    );
    // No separate assertion on q_correction size: E already bounds |Q_corrected - Q_true|.
    // A large correction on small Q_raw (e.g. correction=1e-6, Q_raw=1e-7) is legitimate —
    // it means the uncorrected value was inaccurate but the correction fixed it.
    // The consumer (ehz_capacity) picks max Q, so near-zero Q orbits don't affect the answer.

    Some(KktResult {
        beta: beta_final,
        q_corrected,
        q_error_bound,
        n_positive,
        n_negative,
        n_zero,
    })
}

/// Solve the KKT system for max Q(β) subject to N^T β = 0, η^T β = 1.
///
/// Uses eigendecomposition of the symmetric KKT matrix M = VΛV^T.
/// This gives solution, rank detection, null space, and inertia in one
/// factorization.
///
/// Returns `Some(KktResult)` with β > 0, corrected Q̃, error bound E,
/// and inertia of M, or `None` if no admissible solution exists.
///
/// When the KKT system is rank-deficient (common for polytopes with
/// axis-aligned normals in symplectic subplanes), there is a family of
/// solutions parameterized by the null space. Q(β) is constant along
/// the null space, so we search for any member with β > 0.
///
/// See `[lem:kkt]` (thesis): the KKT conditions characterise the constrained
/// maximum of Q(β); the system is (m+5)×(m+5) with ν for η^Tβ = 1.
pub fn solve_kkt(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<KktResult> {
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    solve_kkt_eigen_path(&kkt, &rhs, normals, heights, perm)
}

