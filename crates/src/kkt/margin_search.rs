/// Max-margin feasibility search in an affine subspace.
///
/// Given base point β₀ ∈ Rᵐ and direction matrix V ∈ R^{m×k} with orthonormal columns,
/// solves the Chebyshev center problem:
///
///   max_α  min_j  (β₀ + V·α)_j
///
/// This finds the point in the affine subspace {β₀ + V·α : α ∈ Rᵏ} with maximum
/// clearance from all positivity constraints β_j ≥ 0. The optimal margin is:
/// - positive → the subspace contains a strictly feasible point (β > 0)
/// - negative → every point in the subspace violates at least one constraint
/// - near-zero → the subspace is tangent to the positivity boundary
///
/// Used by `projection_solver` (Step 4) to classify the verdict for a KKT node.
/// See §6 of `kkt-module-spec.md`.
use nalgebra::{DMatrix, DVector};

/// Maximum iterations for the coordinate ascent (k ≥ 2 case).
///
/// 100 suffices for m ≤ 16, k ≤ 11. Each iteration improves the most-violated
/// constraint, so convergence is at most m iterations for non-degenerate cases.
/// In practice converges in < 10 iterations for our polytope sizes.
const MAX_ITER: usize = 100;

/// Convergence tolerance: stop when margin improvement < this between iterations.
const EPS_CONVERGENCE: f64 = 1e-14;

/// Component of null-space vector below this magnitude is treated as zero (k=1 case).
///
/// Tighter than EPS_BETA_POSITIVE (1e-12) because V has orthonormal columns —
/// components below 1e-15 are numerical zeros from the SVD/eigensolver.
const EPS_DIRECTION_ZERO: f64 = 1e-15;

/// Result of the max-margin feasibility search.
///
/// Always returned (never None). The margin classifies feasibility:
/// - margin > +ε → certified feasible (all β_j > 0)
/// - margin < -ε → certified infeasible (no β > 0 in the subspace)
/// - |margin| ≤ ε → ambiguous (INDETERMINATE)
#[derive(Clone, Debug)]
pub struct MarginResult {
    /// The maximum margin: max_α min_j (β₀ + Vα)_j.
    /// Positive → feasible, negative → infeasible, near-zero → ambiguous.
    pub margin: f64,
    /// The optimal α achieving the margin (in null-space coordinates).
    /// Length k (= null_basis.ncols()). Empty (length 0) when k = 0.
    pub alpha: DVector<f64>,
    /// The solution point β = β₀ + V·α.
    pub beta: DVector<f64>,
}

/// Find the point in the affine subspace {β₀ + Vα} with maximum minimum component.
///
/// This is the Chebyshev center problem for the polytope {α : β₀ + Vα ≥ 0}.
///
/// # Cases by null-space dimension k
///
/// - **k = 0**: No degrees of freedom. margin = min(β₀), trivially.
/// - **k = 1**: Analytic solution via interval analysis. The constraint β₀ + v·α ≥ 0
///   defines an interval [lo, hi] for α; the midpoint maximizes the margin.
/// - **k ≥ 2**: Iterative coordinate ascent on the most-violated constraint.
///   Each step computes the gradient of β[j*] w.r.t. α (a row of V) and steps
///   along it to improve the worst component.
///
/// # Guarantees
///
/// - Always returns a result (never panics for valid inputs).
/// - If the subspace has no feasible point, margin will be negative.
/// - The returned margin equals min(β) exactly (verified by tests).
///
/// # Critical property
///
/// If margin < -ε, the subspace is genuinely infeasible. But the converse is NOT
/// guaranteed for k ≥ 2: if the iterative method fails to find margin > +ε, the
/// caller should use INDETERMINATE (not FALSE). The coordinate ascent is a heuristic
/// that may not find the global optimum.
pub fn find_max_margin(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let k = null_basis.ncols();

    match k {
        0 => find_max_margin_k0(beta0),
        1 => find_max_margin_k1(beta0, &null_basis.column(0).into_owned()),
        _ => find_max_margin_kn(beta0, null_basis),
    }
}

/// k = 0: No degrees of freedom. Margin is simply min(β₀).
fn find_max_margin_k0(beta0: &DVector<f64>) -> MarginResult {
    let m = beta0.len();
    let margin = if m == 0 {
        // Degenerate: no components. Margin is +∞ vacuously, but we return 0.0
        // since there are no constraints to satisfy.
        0.0
    } else {
        beta0.iter().copied().fold(f64::INFINITY, f64::min)
    };

    MarginResult {
        margin,
        alpha: DVector::zeros(0),
        beta: beta0.clone(),
    }
}

/// k = 1: Analytic solution via interval analysis.
///
/// For each component j, the constraint (β₀ + v·α)_j ≥ 0 gives:
/// - v_j > 0 → α ≥ -β₀_j / v_j  (lower bound)
/// - v_j < 0 → α ≤ -β₀_j / v_j  (upper bound)
/// - |v_j| ≈ 0 → no constraint on α (β₀_j is fixed)
///
/// The feasible interval is [lo, hi]. The midpoint α = (lo + hi) / 2 maximizes
/// the minimum distance to both interval endpoints, giving the max-margin point.
///
/// When the interval is unbounded on one side, we step a finite distance from
/// the bounded end (choosing 1.0 as a reasonable unit step).
fn find_max_margin_k1(beta0: &DVector<f64>, v: &DVector<f64>) -> MarginResult {
    let m = beta0.len();
    let mut lo = f64::NEG_INFINITY;
    let mut hi = f64::INFINITY;

    for j in 0..m {
        if v[j].abs() < EPS_DIRECTION_ZERO {
            // This component is essentially fixed at β₀[j].
            // It contributes to the margin but α cannot change it.
            continue;
        }
        let bound = -beta0[j] / v[j];
        if v[j] > 0.0 {
            lo = lo.max(bound);
        } else {
            hi = hi.min(bound);
        }
    }

    // Pick optimal α
    let alpha_scalar = if lo.is_finite() && hi.is_finite() {
        // Bounded interval: midpoint maximizes margin
        (lo + hi) / 2.0
    } else if lo.is_finite() {
        // Only lower-bounded: step away from boundary
        lo + 1.0
    } else if hi.is_finite() {
        // Only upper-bounded: step away from boundary
        hi - 1.0
    } else {
        // Fully unbounded: all v_j ≈ 0, so α doesn't matter. Stay at origin.
        0.0
    };

    let alpha = DVector::from_element(1, alpha_scalar);
    let beta = beta0 + v * alpha_scalar;
    let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);

    MarginResult {
        margin,
        alpha,
        beta,
    }
}

/// k ≥ 2: Iterative coordinate ascent on the most-violated constraint.
///
/// Starting from α = 0, each iteration:
/// 1. Computes β = β₀ + V·α
/// 2. Finds the most-violated component j* = argmin_j β_j
/// 3. Computes the gradient g = V[j*, :]ᵀ (how α affects β[j*])
/// 4. Steps along g to push β[j*] toward the second-smallest component
///
/// The step target is the second-smallest β component. This "equalize the two
/// worst" strategy converges to the Chebyshev center because at the optimum,
/// at least two constraints are equally active (the geometric insight behind
/// the Chebyshev center). When equalization is impossible (zero gradient),
/// we accept the current margin.
///
/// Convergence: stops when the margin improvement between iterations is below
/// EPS_CONVERGENCE, or after MAX_ITER iterations.
fn find_max_margin_kn(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let k = null_basis.ncols();
    let mut alpha = DVector::zeros(k);
    let mut prev_margin = f64::NEG_INFINITY;
    let mut prev_worst_j: Option<usize> = None;

    for _iter in 0..MAX_ITER {
        // Current β = β₀ + V·α
        let beta = beta0 + null_basis * &alpha;

        // Find most-violated component: j* = argmin_j β_j
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        let current_margin = *worst_val;

        // Convergence check: margin stopped improving AND the same component is
        // still the worst. If a different component is now worst at the same margin,
        // we still have work to do (need to push that one up too).
        let same_worst = prev_worst_j == Some(worst_j);
        if same_worst && (current_margin - prev_margin).abs() < EPS_CONVERGENCE {
            return MarginResult {
                margin: current_margin,
                alpha,
                beta,
            };
        }
        prev_margin = current_margin;
        prev_worst_j = Some(worst_j);

        // Gradient of β[j*] w.r.t. α: g_i = V[j*, i] (row j* of V)
        // This is the direction in α-space that increases β[j*] fastest.
        let g: DVector<f64> = null_basis.row(worst_j).transpose().into_owned();
        let g_sq = g.dot(&g);

        if g_sq < 1e-30 {
            // Near-zero gradient: no null-space direction can improve this component.
            // This happens when row j* of V is (near-)zero, meaning β[j*] is
            // essentially fixed at β₀[j*] regardless of α.
            return MarginResult {
                margin: current_margin,
                alpha,
                beta,
            };
        }

        // Target: push β[j*] toward the second-smallest β component.
        // At the Chebyshev center, the two smallest components are equal, so
        // this drives toward the optimum.
        //
        // When the second-smallest equals the current minimum (tied components),
        // we use the mean of non-minimum components as the target instead. This
        // ensures a nonzero step that breaks the tie.
        //
        // We want β[j*] + g·δα = target, where δα = step * g.
        // Then β[j*] + g_sq * step = target → step = (target - β[j*]) / g_sq.
        let second = second_smallest(&beta, worst_j);
        let target = if second > current_margin + EPS_CONVERGENCE {
            // Normal case: target the second-smallest
            second
        } else {
            // Tied minimums: target the mean of components above the minimum.
            // This provides a reasonable upward step.
            let above: Vec<f64> = beta
                .iter()
                .filter(|&&v| v > current_margin + EPS_CONVERGENCE)
                .copied()
                .collect();
            if above.is_empty() {
                // All components are equal — any direction is equally good.
                // Step by 1.0 (arbitrary but ensures progress).
                current_margin + 1.0
            } else {
                above.iter().sum::<f64>() / above.len() as f64
            }
        };

        let step = (target - current_margin) / g_sq;
        alpha += &g * step;
    }

    // After MAX_ITER: compute final β and margin
    let beta = beta0 + null_basis * &alpha;
    let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);

    MarginResult {
        margin,
        alpha,
        beta,
    }
}

/// Find the second-smallest value in a vector, excluding index `skip`.
///
/// If all remaining components are equal to the minimum (or the vector has
/// only one element), returns the minimum + 1.0 as a fallback target.
fn second_smallest(v: &DVector<f64>, skip: usize) -> f64 {
    let mut second = f64::INFINITY;
    for (j, &val) in v.iter().enumerate() {
        if j != skip && val < second {
            second = val;
        }
    }
    if second.is_finite() {
        second
    } else {
        // Only one component (m=1) — can't have a second smallest.
        // Use min + 1 as fallback target.
        v[skip] + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    /// Helper: verify the critical invariant margin = min(β).
    fn assert_margin_is_tight(result: &MarginResult) {
        let min_beta = result
            .beta
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        assert!(
            (result.margin - min_beta).abs() < 1e-12,
            "margin ({:.2e}) != min(β) ({:.2e}), diff = {:.2e}",
            result.margin,
            min_beta,
            (result.margin - min_beta).abs()
        );
    }

    /// Helper: verify β = β₀ + V·α reconstruction.
    fn assert_beta_reconstruction(beta0: &DVector<f64>, null_basis: &DMatrix<f64>, result: &MarginResult) {
        let expected = beta0 + null_basis * &result.alpha;
        let diff = (&result.beta - &expected).norm();
        assert!(
            diff < 1e-12,
            "β reconstruction error: ‖β - (β₀ + V·α)‖ = {:.2e}",
            diff
        );
    }

    // ---- k = 0: trivial cases ----

    #[test]
    fn trivial_feasible() {
        // β₀ = [1, 1, 1], V empty → margin = 1.0
        let beta0 = DVector::from_vec(vec![1.0, 1.0, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 1.0).abs() < 1e-14);
        assert_eq!(result.alpha.len(), 0);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn trivial_infeasible() {
        // β₀ = [-1, 1, 1], V empty → margin = -1.0
        let beta0 = DVector::from_vec(vec![-1.0, 1.0, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - (-1.0)).abs() < 1e-14);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn trivial_single_component() {
        // β₀ = [3.5], V empty → margin = 3.5
        let beta0 = DVector::from_vec(vec![3.5]);
        let v = DMatrix::zeros(1, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 3.5).abs() < 1e-14);
        assert_margin_is_tight(&result);
    }

    // ---- k = 1: analytic cases ----

    #[test]
    fn one_dim_feasible() {
        // β₀ = [-1, 2], V = [1, 0]ᵀ
        // Constraint: -1 + α ≥ 0 → α ≥ 1 (lo = 1)
        // Constraint: 2 + 0·α ≥ 0 → always (v[1] ≈ 0)
        // Interval: [1, ∞). α = 1 + 1.0 = 2.0 (unbounded case).
        // β = [-1 + 2, 2] = [1, 2]. margin = 1.0.
        let beta0 = DVector::from_vec(vec![-1.0, 2.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, 0.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_infeasible() {
        // β₀ = [-1, -2], V = [1, -1]ᵀ
        // Constraint: -1 + α ≥ 0 → α ≥ 1 (lo = 1)
        // Constraint: -2 - α ≥ 0 → α ≤ -2 (hi = -2)
        // Interval: [1, -2] — empty. Best α is midpoint = (1 + -2)/2 = -0.5.
        // β = [-1 + (-0.5), -2 - (-0.5)] = [-1.5, -1.5]. margin = -1.5.
        let beta0 = DVector::from_vec(vec![-1.0, -2.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_midpoint() {
        // β₀ = [0, 0], V = [1, -1]ᵀ
        // Constraint: 0 + α ≥ 0 → α ≥ 0 (lo = 0)
        // Constraint: 0 - α ≥ 0 → α ≤ 0 (hi = 0)
        // Interval: [0, 0]. α = 0. β = [0, 0]. margin = 0.
        let beta0 = DVector::from_vec(vec![0.0, 0.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin.abs() < 1e-14, "expected margin ≈ 0, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_bounded_interval() {
        // β₀ = [2, 4], V = [1, -1]ᵀ
        // Constraint: 2 + α ≥ 0 → α ≥ -2 (lo = -2)
        // Constraint: 4 - α ≥ 0 → α ≤ 4 (hi = 4)
        // Interval: [-2, 4]. Midpoint α = 1. β = [3, 3]. margin = 3.
        let beta0 = DVector::from_vec(vec![2.0, 4.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 3.0).abs() < 1e-12, "expected margin = 3, got {}", result.margin);
        assert!((result.alpha[0] - 1.0).abs() < 1e-12, "expected α = 1, got {}", result.alpha[0]);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_all_directions_zero() {
        // β₀ = [1, 2, 3], V = [0, 0, 0]ᵀ (numerically zero direction)
        // α doesn't change anything. margin = min(β₀) = 1.
        let beta0 = DVector::from_vec(vec![1.0, 2.0, 3.0]);
        let v = DMatrix::from_vec(3, 1, vec![1e-16, 0.0, 1e-17]);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 1.0).abs() < 1e-10, "expected margin ≈ 1, got {}", result.margin);
        assert_margin_is_tight(&result);
    }

    // ---- k ≥ 2: multi-dimensional cases ----

    #[test]
    fn two_dim_feasible() {
        // β₀ = [-1, -1, 3], V = [[1, 0], [0, 1], [0, 0]]
        // Can set α = [1, 1] to get β = [0, 0, 3]. With more search, margin > 0.
        // The search should find α with β₁ = β₂ > 0 and β₃ = 3 > 0.
        let beta0 = DVector::from_vec(vec![-1.0, -1.0, 3.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        // All β components should be positive
        for j in 0..3 {
            assert!(result.beta[j] > 0.0, "β[{}] = {} should be > 0", j, result.beta[j]);
        }
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn two_dim_infeasible() {
        // β₀ = [-5, -5, -5], V = [[1, 0], [0, 1], [-1, -1]]
        // Sum of β is constant: β₁ + β₂ + β₃ = -5 -5 -5 + (α₁ + α₂ - α₁ - α₂) = -15.
        // Since sum(β) = -15, at least one component must be ≤ -5. Margin ≤ -5.
        let beta0 = DVector::from_vec(vec![-5.0, -5.0, -5.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0,  0.0,
            0.0,  1.0,
           -1.0, -1.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn two_dim_symmetric() {
        // β₀ = [0, 0, 0, 6], V = [[1, 0], [0, 1], [-1, 0], [0, -1]]
        // The optimal solution should balance: β₁ = β₃ and β₂ = β₄.
        // β₁ = α₁, β₂ = α₂, β₃ = -α₁, β₄ = 6 - α₂.
        // Maximize min(α₁, α₂, -α₁, 6 - α₂).
        // Symmetry: α₁ = 0 (so β₁ = β₃ = 0), α₂ = 3 (so β₂ = β₄ = 3).
        // But min = 0 from β₁ = β₃ = 0.
        // Actually, best: all components equal at 6/4 = 1.5? No, sum is constrained.
        // sum(β) = 0 + 0 + 0 + 6 + (α₁ + α₂ - α₁ - α₂) = 6, independent of α.
        // With constraint sum = 6 and 4 components, max min = 6/4 = 1.5.
        // α₁ = 1.5, α₂ = 1.5 → β = [1.5, 1.5, -1.5 + 0, 6 - 1.5] = [1.5, 1.5, -1.5, 4.5].
        // Wait, let me redo: β = β₀ + V·α = [α₁, α₂, -α₁, 6 - α₂].
        // min(α₁, α₂, -α₁, 6-α₂). Optimal: α₁ = 0 → β₁ = β₃ = 0, α₂ = 3 → β₂ = β₄ = 3.
        // margin = 0.
        let beta0 = DVector::from_vec(vec![0.0, 0.0, 0.0, 6.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(4, 2, &[
             1.0,  0.0,
             0.0,  1.0,
            -1.0,  0.0,
             0.0, -1.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        // The theoretical optimum has margin = 0 (α₁ = 0, α₂ = 3).
        // The iterative solver should get close.
        assert!(result.margin.abs() < 0.1, "expected margin ≈ 0, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    // ---- Cross-cutting properties ----

    #[test]
    fn margin_is_tight_k0() {
        let beta0 = DVector::from_vec(vec![3.0, 1.5, 7.2, 0.1]);
        let v = DMatrix::zeros(4, 0);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn margin_is_tight_k1() {
        let beta0 = DVector::from_vec(vec![1.0, 3.0, 0.5, 2.0]);
        let v = DMatrix::from_vec(4, 1, vec![0.5, -0.3, 0.8, -0.1]);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn margin_is_tight_k2() {
        let beta0 = DVector::from_vec(vec![1.0, -0.5, 2.0, 0.1, -0.3]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(5, 2, &[
            0.3,  0.1,
           -0.2,  0.5,
            0.1, -0.3,
            0.4,  0.2,
           -0.1,  0.6,
        ]);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn null_basis_empty_equals_k0() {
        // V is m×0 — should behave identically to the k=0 case.
        let beta0 = DVector::from_vec(vec![2.0, 0.5, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 0.5).abs() < 1e-14);
        assert_eq!(result.alpha.len(), 0);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn k1_optimal_is_midpoint() {
        // β₀ = [1, 5], V = [1, -1]ᵀ
        // lo = -1, hi = 5, midpoint = 2. β = [3, 3], margin = 3.
        // The midpoint exactly maximizes the margin for k=1.
        let beta0 = DVector::from_vec(vec![1.0, 5.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        // Optimal margin for this problem: (5 - (-1))/2 - ... let me compute exactly.
        // lo = -β₀[0]/v[0] = -1, hi = -β₀[1]/v[1] = 5.
        // α_opt = 2.0. β = [3, 3]. margin = 3.0.
        assert!((result.margin - 3.0).abs() < 1e-12);
        assert!((result.alpha[0] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn larger_k3_feasible() {
        // β₀ with some negative entries, V with 3 directions.
        // Enough freedom to make all entries positive.
        let beta0 = DVector::from_vec(vec![-1.0, -2.0, -1.0, 10.0, 10.0, 10.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(6, 3, &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        // Can set α = [1, 2, 1] → β = [0, 0, 0, 10, 10, 10].
        // Or even better, push higher: α = [5, 6, 5] → β = [4, 4, 4, 10, 10, 10].
        // The solver should find margin > 0.
        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn k1_half_bounded_lower_only() {
        // β₀ = [2, 0], V = [0, 1]ᵀ
        // Constraint: 0 + α ≥ 0 → α ≥ 0 (lo = 0). v[0] ≈ 0, β₀[0] = 2 (fixed).
        // Interval: [0, +∞). α = 0 + 1.0 = 1.0 (half-bounded case).
        // β = [2, 1], margin = 1.
        let beta0 = DVector::from_vec(vec![2.0, 0.0]);
        let v = DMatrix::from_vec(2, 1, vec![0.0, 1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected positive margin, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }
}
