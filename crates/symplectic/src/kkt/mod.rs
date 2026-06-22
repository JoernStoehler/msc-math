//! KKT solver module for constrained quadratic optimization on polytopes.
//!
//! Solves: max (1/2) beta^T H beta  subject to  C beta = d, beta > 0.
//!
//! The KKT module is context-independent: it operates on abstract matrices (C, d, H)
//! without knowing they come from symplectic geometry. Assembly of (C, d, H) from
//! dual vertices lives in `qp_assembly`.
//!
//! Mathematical correspondence: [lem:kkt], [lem:q-error-bound]
//!
//! ## Two f64 solver strategies
//!
//! Two f64 solvers attack the same QP from different directions:
//!
//! - `saddle_point_solver` — solves the (m+5)x(m+5) augmented KKT system
//!   via eigendecomposition. Default path used by `hk2017` and `billiard`
//!   (see `solve_kkt_for_dual_vertices`).
//! - `projection_solver` — projects onto `ker(C)`, reduces to the k-dim
//!   Hessian `H' = V^T H V`, then runs a max-margin search. Used when the
//!   saddle-point path needs a cross-check or when beta>0 feasibility is
//!   the bottleneck.
//!
//! `rational_solver` is a separate exact-arithmetic track (see below); it
//! is NOT a third strategy on the f64 hot path.
//!
//! ## Submodules
//!
//! - `qp_assembly` — dual vertices + permutation -> QP matrices or augmented system
//! - `saddle_point_solver` — (m+5)x(m+5) eigendecomposition solver (main path)
//! - `constraint_solver` — Solve Cx=d for particular solution + null space basis via SVD
//! - `beta_feasibility` — Max-margin LP search for beta>0 in affine solution set
//! - `projection_solver` — Project to constraint null space, optimize reduced objective
//! - `rational_solver` — Exact rational KKT solver (validation/ground-truth only; NOT used in main capacity enumeration)

pub mod beta_feasibility;
pub mod constraint_solver;
pub mod projection_solver;
pub mod qp_assembly;
pub mod rational_solver;
pub mod saddle_point_solver;
#[cfg(test)]
mod test_saddle_point_solver;

use nalgebra::{DMatrix, DVector};

// ── Public types ──

/// Constrained quadratic program: max (1/2) beta^T H beta  s.t. C beta = d, beta > 0.
///
/// For EHZ capacity: C encodes closure + normalization, H encodes symplectic action,
/// beta are dwell-time coefficients. But this struct is context-free.
///
/// # Dimensions
/// - C: p x m (p constraints, m variables)
/// - d: p x 1
/// - H: m x m, symmetric
///
/// Mathematical correspondence: [lem:kkt]
pub struct QP {
    /// Constraint matrix (p x m).
    pub c: DMatrix<f64>,
    /// Constraint right-hand side (p x 1).
    pub d: DVector<f64>,
    /// Objective matrix (m x m, symmetric). Q(beta) = (1/2) beta^T H beta.
    pub h: DMatrix<f64>,
}

/// Trinary numerical verdict for feasibility of beta > 0.
///
/// The current f64 classifiers use static numerical margins. Treat these
/// verdicts as route-local f64 labels, not theorem-backed predicates.
/// Development notes in `experiments/dev-quadratic-program/` track the
/// proof-backed replacement work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Static-margin f64 feasible label: all beta_k > eps.
    True,
    /// Static-margin f64 infeasible label: some beta_k < -eps.
    False,
    /// Ambiguous: beta has near-zero components, or near-null eigenvalues
    /// prevent definitive classification.
    Indeterminate,
}

/// Result of solving a QP.
///
/// Q is always valid when verdict != False. beta is the best point found.
/// margin = min_k beta_k quantifies clearance from the positivity boundary.
#[derive(Clone, Debug)]
pub struct Solution {
    /// Trinary verdict classifying the solution feasibility.
    pub verdict: Verdict,
    /// Optimal objective value: Q = (1/2) beta^T H beta.
    /// Constant over the solution set (null space of projected Hessian).
    /// Valid for True and Indeterminate. Zero for False.
    pub q: f64,
    /// Solution vector. For True: all components > 0. For Indeterminate:
    /// best-effort max-margin point. For False: empty or not meaningful.
    pub beta: Vec<f64>,
    /// min_k beta_k. Positive -> True, negative -> False, near-zero -> Indeterminate.
    pub margin: f64,
}

// ── Verdict threshold constants ──

/// beta_k > EPS_MARGIN_TRUE -> component receives the f64 positive label.
///
/// **Why 1e-9:** This is the classification boundary between True and Indeterminate.
/// It sits between EPS_BETA_POSITIVE (1e-12, eigensolver noise floor) and typical
/// real beta values (O(0.01)--O(10)). A beta in (1e-12, 1e-9) is above noise but
/// ambiguous -- classified as Indeterminate and routed to the rational fallback.
/// Beta above 1e-9 is treated as positive by this static f64 classifier.
/// The symmetric value for EPS_MARGIN_FALSE ensures symmetric Indeterminate bands.
/// Making it 10x larger (1e-8) would route more valid solutions to the rational
/// fallback unnecessarily; 10x smaller (1e-10) would accept more noisy solutions.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// beta_k < -EPS_MARGIN_FALSE -> component receives the f64 negative label.
///
/// **Why 1e-9:** Same as EPS_MARGIN_TRUE. The symmetric design means the
/// Indeterminate band is (-1e-9, +1e-9) -- beta in this range is ambiguous.
/// Beta below -1e-9 is treated as negative by this static f64 classifier.
/// Using a symmetric threshold simplifies reasoning: both boundaries are
/// separated from the noise floor (1e-12) by the same factor.
const EPS_MARGIN_FALSE: f64 = 1e-9;

// ── Shared numerical constants ──

/// Absolute floor for eigenvalue magnitude. If the largest eigenvalue of
/// a matrix is below this, the matrix is treated as numerically zero.
///
/// **Why 1e-12:** KKT and reduced-Hessian matrix entries are O(1). Eigenvalues
/// below 1e-12 are in the machine-noise range (machine epsilon ~1e-16, plus
/// accumulation from O(m) operations). Used by both `saddle_point_solver` (for
/// the augmented (m+5)x(m+5) matrix) and `projection_solver` (for the kxk
/// reduced Hessian H'). Despite different matrix structures, the absolute floor
/// is the same: it guards against attempting rank detection on a pure-noise
/// matrix, and the O(1) entry-scale argument applies to both.
/// Making it 10x larger (1e-11) risks discarding matrices that are genuinely
/// non-zero but small; 10x smaller (1e-13) risks attempting rank detection on
/// a pure-noise matrix.
pub(crate) const EPS_EIGEN_FLOOR: f64 = 1e-12;

// ── Utility functions ──

/// Compute Q = (1/2) beta^T H beta from pre-assembled H and beta.
///
/// Mathematical correspondence: [lem:H-quadratic]: Q(beta) = (1/2) beta^T H beta where H_{ij} = omega_0(a_i, a_j).
pub fn q_value(h: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let b = DVector::from_column_slice(beta);
    0.5 * b.dot(&(h * &b))
}

/// Classify a margin value into a trinary verdict.
///
/// Uses symmetric thresholds:
/// - margin > +EPS_MARGIN_TRUE -> True (static f64 positive label)
/// - margin < -EPS_MARGIN_FALSE -> False (static f64 negative label)
/// - otherwise -> Indeterminate (ambiguous)
pub(crate) fn classify_margin(margin: f64) -> Verdict {
    if margin > EPS_MARGIN_TRUE {
        Verdict::True
    } else if margin < -EPS_MARGIN_FALSE {
        Verdict::False
    } else {
        Verdict::Indeterminate
    }
}
