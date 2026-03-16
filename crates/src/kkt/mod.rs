/// KKT solver module for constrained quadratic optimization.
///
/// Solves: max (1/2) βᵀHβ  subject to  Cβ = d, β > 0.
///
/// Two solver variants:
/// - **Augmented** (`augmented`): builds the (m+5)×(m+5) symmetric saddle-point system
///   and solves via eigendecomposition. The original approach, kept for ablation.
/// - **Projection** (`projection_solver`): projects to the constraint null space, then
///   optimizes in the reduced (m-p)-dimensional space. The new approach.
///
/// The KKT module is context-independent: it operates on abstract matrices (C, d, H)
/// without knowing they come from symplectic geometry. Assembly of (C, d, H) from
/// dual vertices and permutations lives in the caller (hk2017, billiard).
///
/// Sub-components (import directly from each sub-module, not via re-exports here):
/// - `constraint_solver`: solve Cβ = d for particular solution + null space basis
/// - `margin_search`: max-margin feasibility search for β > 0
/// - `projection_solver`: the projection-based solver combining both sub-components
/// - `augmented`: the legacy augmented (m+5) solver
pub mod augmented;
pub mod constraint_solver;
pub mod margin_search;
mod projection_solver;

use nalgebra::{DMatrix, DVector};

// ── Public types ──

/// Constrained quadratic program: max (1/2) βᵀHβ  s.t. Cβ = d, β > 0.
///
/// For EHZ capacity: C encodes closure + normalization, H encodes symplectic action,
/// β are dwell-time coefficients. But this struct is context-free.
///
/// # Dimensions
/// - C: p × m (p constraints, m variables)
/// - d: p × 1
/// - H: m × m, symmetric
pub struct QP {
    pub c: DMatrix<f64>,
    pub d: DVector<f64>,
    pub h: DMatrix<f64>,
}

/// Trinary verdict for feasibility of β > 0.
///
/// **Critical invariant:** False is never returned unless certified infeasible.
/// When in doubt, return Indeterminate. The accumulator handles resolution
/// (e.g. via rational fallback).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Certified feasible: all β_k > ε. Safe to use Q and β.
    True,
    /// Certified infeasible: no β > 0 exists in the solution set.
    False,
    /// Ambiguous: β has near-zero components, or near-null eigenvalues
    /// prevent definitive classification. Q is still valid.
    Indeterminate,
}

/// Result of solving a QP.
///
/// Q is always valid when verdict ≠ False. β is the best point found.
/// margin = min_k β_k quantifies clearance from the positivity boundary.
#[derive(Clone, Debug)]
pub struct Solution {
    pub verdict: Verdict,
    /// Optimal objective value: Q = (1/2) βᵀHβ.
    /// Constant over the solution set (null space of projected Hessian).
    /// Valid for True and Indeterminate. Zero for False.
    pub q: f64,
    /// Solution vector. For True: all components > 0. For Indeterminate:
    /// best-effort max-margin point. For False: empty or not meaningful.
    pub beta: Vec<f64>,
    /// min_k β_k. Positive → True, negative → False, near-zero → Indeterminate.
    pub margin: f64,
}

// ── Verdict threshold constants ──

/// β_k > EPS_MARGIN_TRUE → component is certified positive.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// β_k < -EPS_MARGIN_FALSE → component is certified negative (infeasible).
const EPS_MARGIN_FALSE: f64 = 1e-9;

// ── Entry point ──

/// Solve the constrained QP: max (1/2) βᵀHβ  s.t. Cβ = d, β > 0.
///
/// Uses the projection method: project to the constraint null space,
/// optimize the reduced objective, search for β > 0 via max-margin search.
///
/// **Note:** The production capacity pipeline (hk2017, billiard) currently
/// calls `augmented::solve_kkt` directly — this entry point is not yet
/// wired into the pipeline.
///
/// # Panics
/// - If dimensions are inconsistent (C.ncols ≠ H.nrows, etc.)
pub fn solve(qp: &QP) -> Solution {
    let m = qp.h.nrows();
    assert_eq!(qp.h.ncols(), m, "H must be square");
    assert_eq!(qp.c.ncols(), m, "C.ncols must equal H.nrows (m)");
    assert_eq!(
        qp.c.nrows(),
        qp.d.nrows(),
        "C.nrows must equal d.nrows (p)"
    );

    projection_solver::solve_projected(qp)
}

/// Compute Q = (1/2) βᵀHβ from pre-assembled H and β.
pub fn q_value(h: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let b = DVector::from_column_slice(beta);
    0.5 * b.dot(&(h * &b))
}

/// Classify a margin value into a trinary verdict.
pub(crate) fn classify_margin(margin: f64) -> Verdict {
    if margin > EPS_MARGIN_TRUE {
        Verdict::True
    } else if margin < -EPS_MARGIN_FALSE {
        Verdict::False
    } else {
        Verdict::Indeterminate
    }
}
