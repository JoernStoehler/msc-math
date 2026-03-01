/// Shared KKT solver for capacity algorithms.
///
/// Solves the constrained optimization max Q(β) subject to N^T β = 0, η^T β = 1,
/// where Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
///
/// Used by both the hk2017 (general) and billiard (Lagrangian product) algorithms.
/// Previously duplicated across both crates (294 LOC each); now unified here.
use crate::geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

/// Minimum β_i value to consider a solution valid (filters numerical noise near zero).
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid (avoids division-by-near-zero in action).
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// Result of KKT solve with Q error bound.
///
/// See `[lem:v2-q-interval]` (thesis): |Q(β₀) - q_corrected| ≤ q_error_bound.
///
/// The corrected Q absorbs the first-order error from the KKT residual,
/// leaving only a second-order remainder bounded by q_error_bound.
#[derive(Clone, Debug)]
pub struct KktResult {
    /// Optimal β vector (numerical approximation, all components > 0).
    pub beta: Vec<f64>,
    /// Residual-corrected Q value: Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂).
    /// See `[eq:v2-q-corrected]` (thesis).
    pub q_corrected: f64,
    /// Error bound E on Q̃: |Q(β₀) - Q̃| ≤ E.
    /// See `[eq:v2-q-error-bound]` (thesis).
    /// Used by the accumulator for majorization.
    #[allow(dead_code)]
    pub q_error_bound: f64,
}

impl KktResult {
    /// Lower bound on true Q (conservative for capacity upper bound).
    #[allow(dead_code)]
    pub fn q_min(&self) -> f64 {
        self.q_corrected - self.q_error_bound
    }
    /// Upper bound on true Q (optimistic for capacity lower bound).
    #[allow(dead_code)]
    pub fn q_max(&self) -> f64 {
        self.q_corrected + self.q_error_bound
    }
}

/// Absolute floor: if the largest singular value is below this, the entire matrix
/// is treated as numerically zero (early return). The relative rank detection is
/// handled by SVD_CONDITION_TAU.
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Condition-number threshold for SVD rank detection: singular values below
/// max_sv * SVD_CONDITION_TAU are treated as part of the null space.
///
/// A singular value σ_j is "small" if σ_j < σ_1 · τ. This detects both
/// isolated small singular values (the classic gap case) and gradual decay
/// to small values (which a gap-ratio approach would miss).
///
/// For near-singular systems, the null space directions are used to search
/// for β > 0, and the Q error bound (see `[lem:v2-q-interval]`) quantifies
/// the resulting objective uncertainty.
///
/// **Why 1e-3:** The degenerate (4,4) Lagrangian product at θ≈0° has
/// sv[8]≈0.51, sv[9]≈8.6e-4 with σ_1 ≈ 1–2, giving sv[9]/σ_1 ≈ 4e-4.
/// The threshold 1e-3 catches this with margin. Well-conditioned random
/// polytopes have smallest sv ≈ 0.01–0.1, well above 1e-3 · σ_1.
///
/// Regression tests: `svd_gap_ratio_44_degenerate`, `svd_gap_ratio_44_theta43`.
const SVD_CONDITION_TAU: f64 = 1e-3;

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
pub(crate) fn q_from_beta(
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

/// Search null space for a β > 0 solution (1D null space case).
///
/// Given particular solution β₀ and null space vector v, find α such that
/// β₀ + α·v > 0 componentwise. Returns the midpoint of the feasible interval
/// for numerical stability.
///
/// Q(β) is constant along the null space (the null space directions satisfy
/// the KKT stationarity conditions, so the objective doesn't change).
fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);

    for j in 0..m {
        if v[j].abs() < 1e-15 {
            // This component doesn't change with α
            if beta0[j] <= EPS_BETA_POSITIVE {
                return None; // Can't make this component positive
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
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

/// Search null space for a β > 0 solution (multi-dimensional null space).
///
/// Uses iterative coordinate ascent on the most-violated constraint.
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
            return Some(beta);
        }

        // Gradient of β[worst_j] w.r.t. α: g_i = null_vecs[i][worst_j]
        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            return None; // Can't improve this component
        }

        // Step to push β[worst_j] to a small positive value
        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }

    // Final feasibility check
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
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
/// Symmetry enables eigendecomposition-based solvers and gives signed
/// eigenvalues (vs unsigned singular values from SVD).
///
/// Returns `(kkt, rhs)` where `kkt` is `(m+5) × (m+5)` symmetric.
///
/// Uses **period normalization** (γ on [0,T]); see `appendix-notation.tex`.
pub(crate) fn build_kkt_system(
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

/// SVD path with condition-number-based rank detection and Q error bounding.
///
/// Operates on a pre-built KKT matrix. Used by both `solve_kkt` (as fallback
/// after LU fails) and `solve_kkt_svd_only` (directly).
///
/// Returns a `KktResult` with the β vector, corrected Q̃, and error bound E
/// satisfying |Q(β₀) - Q̃| ≤ E. See `[lem:v2-q-interval]` (thesis).
///
/// Condition-number detection: singular values below max_sv * SVD_CONDITION_TAU
/// are treated as part of the null space (for the β > 0 search).
fn solve_kkt_svd_path(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<KktResult> {
    let m = perm.len();
    let size = m + 5;

    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
    if max_sv < EPS_SVD_FLOOR {
        return None;
    }

    let u = svd.u.as_ref()?;
    let v_t = svd.v_t.as_ref()?;

    // Determine numerical rank via condition-number threshold.
    // sv is sorted descending. Singular values below max_sv * τ
    // are treated as part of the near-null space.
    let threshold = max_sv * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();

    // Compute pseudoinverse solution manually using only the top `rank` SVs.
    // This avoids relying on nalgebra's solve() tolerance interpretation.
    let mut x0 = DVector::zeros(size);
    for i in 0..rank {
        let coeff = u.column(i).dot(rhs) / sv[i];
        for j in 0..size {
            x0[j] += coeff * v_t[(i, j)];
        }
    }

    let residual_vec = kkt * &x0 - rhs;
    let residual_norm = residual_vec.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return None;
    }

    // --- Q error bound computation (Algorithm [alg:v2-q-interval]) ---
    // Extract residual blocks: r₂ = N^T β̂ (rows m..m+4), r₃ = η^T β̂ - 1 (row m+4).
    // The solution vector is [β̂; μ̂; ξ̂] with negated multipliers (μ = -λ, ξ = -ν).
    // Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂)  [Lemma [lem:v2-q-interval], Step 2-3].
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x0[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x0[m + 4];

    // σ_min of RETAINED singular values (those above the rank threshold).
    // Using full-matrix σ_min is catastrophically wrong for rank-deficient systems:
    // near-zero SVs give E = O(1/σ_min²) → 10^66 or NaN.
    // The retained σ_min bounds errors in the directions the SVD actually resolves.
    let sigma_min = sv
        .iter()
        .take(rank)
        .cloned()
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    // ‖H‖ ≤ σ₁ = max_sv (H is a submatrix of M, so ‖H‖ ≤ ‖M‖).
    let h_norm_bound = max_sv;

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();

    // If already feasible, compute error bound and return.
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_raw = q_from_beta(normals, perm, &beta0);
        let q_corrected = q_raw - (r2_dot_mu + r3 * xi_hat);
        let r_sq = residual_norm * residual_norm;
        let q_error_bound =
            r_sq * (2.0 / sigma_min + h_norm_bound / (2.0 * sigma_min * sigma_min));
        return Some(KktResult {
            beta: beta0,
            q_corrected,
            q_error_bound,
        });
    }

    // Full rank: unique solution with β ≤ 0, candidate pair is genuinely infeasible.
    if rank == size {
        return None;
    }

    // Rank-deficient: search null space for β > 0.
    // Null space = right singular vectors for sv's below rank threshold.
    // Q(β) is constant along the null space (constraint-preserving directions).
    let null_beta: Vec<Vec<f64>> = (rank..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();

    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])?
    } else {
        find_positive_beta_nd(&beta0, &null_beta)?
    };

    // Constraint verification: reject if β from null space search violates
    // N^T β ≈ 0 or η^T β ≈ 1 (catches noise amplification in degenerate cases).
    let constraint_residual: f64 = (0..4)
        .map(|d| {
            (0..m)
                .map(|i| beta_opt[i] * normals[perm[i]][d])
                .sum::<f64>()
        })
        .map(|x: f64| x * x)
        .sum::<f64>()
        + ((0..m)
            .map(|i| beta_opt[i] * heights[perm[i]])
            .sum::<f64>()
            - 1.0)
            .powi(2);
    if constraint_residual.sqrt() > EPS_KKT_RESIDUAL {
        return None;
    }

    // Q is constant along the null space, so Q(β_opt) = Q(β₀).
    // |Q(β₀) - Q̃| ≤ E bounds the true Q for the whole family.
    let q_raw = q_from_beta(normals, perm, &beta_opt);
    let q_corrected = q_raw - (r2_dot_mu + r3 * xi_hat);
    let r_sq = residual_norm * residual_norm;
    let q_error_bound =
        r_sq * (2.0 / sigma_min + h_norm_bound / (2.0 * sigma_min * sigma_min));

    Some(KktResult {
        beta: beta_opt,
        q_corrected,
        q_error_bound,
    })
}

/// Solve the KKT system for max Q(β) subject to N^T β = 0, η^T β = 1.
///
/// Production variant: tries LU decomposition first, falls back to SVD with
/// gap-based rank detection for rank-deficient systems.
///
/// Note: profiling showed the LU fast path adds 6-12% overhead in practice
/// because most permutations yield β ≤ 0 even when LU reports invertible.
/// The SVD-only variant (`solve_kkt_svd_only`) is faster. LU is retained
/// for evaluation but not used in the production capacity algorithms.
///
/// Returns `Some(KktResult)` with β > 0 and a Q error bound, or `None` if no
/// admissible solution exists.
///
/// When the KKT system is rank-deficient (common for polytopes with
/// axis-aligned normals in symplectic subplanes), there is a family of
/// solutions parameterized by the null space. Q(β) is constant along
/// the null space, so we search for any member with β > 0.
///
/// See `[lem:kkt]` (thesis): the KKT conditions characterise the constrained
/// maximum of Q(β); the system is (m+5)×(m+5) with ν for η^Tβ = 1.
pub(crate) fn solve_kkt(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<KktResult> {
    let m = perm.len();
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);

    // --- LU fast path ---
    // Full-pivoting LU handles the common case (full-rank, well-conditioned).
    // Falls through to SVD when: not invertible, bad residual, or β ≤ 0.
    let lu = kkt.clone().full_piv_lu();
    if lu.is_invertible() {
        if let Some(solution) = lu.solve(&rhs) {
            let residual = (&kkt * &solution - &rhs).norm();
            if residual <= EPS_KKT_RESIDUAL {
                let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();
                if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
                    let q_val = q_from_beta(normals, perm, &beta);
                    // LU fast path: well-conditioned solution with tiny residual.
                    // Q̃ = Q(β̂) with E = 0 (correction negligible for LU).
                    return Some(KktResult {
                        beta,
                        q_corrected: q_val,
                        q_error_bound: 0.0,
                    });
                }
            }
        }
    }

    // --- SVD fallback with gap-based rank detection ---
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}

/// Solve the KKT system using SVD only (no LU fast path).
///
/// Ablation variant for benchmarking and testing. Uses the same gap-based
/// rank detection as the SVD fallback in `solve_kkt`, but skips the LU
/// fast path. Comparing `solve_kkt` vs `solve_kkt_svd_only` isolates
/// the LU fast path's contribution to performance and correctness.
#[cfg(test)]
pub(crate) fn solve_kkt_svd_only(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<KktResult> {
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}
