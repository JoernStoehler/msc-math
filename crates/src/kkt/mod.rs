//! KKT solver module for constrained quadratic optimization on polytopes.
//!
//! Solves: max (1/2) beta^T H beta  subject to  C beta = d, beta > 0.
//!
//! The KKT module is context-independent: it operates on abstract matrices (C, d, H)
//! without knowing they come from symplectic geometry. Assembly of (C, d, H) from
//! polytope geometry lives in `qp_assembly`.
//!
//! Mathematical correspondence: [lem:kkt], [lem:q-error-bound]
//!
//! ## Submodules
//!
//! - `qp_assembly` — Polytope4D + permutation -> QP matrices or augmented system
//! - `saddle_point_solver` — (m+5)x(m+5) eigendecomposition solver
//! - `constraint_solver` — Solve Cx=d for particular solution + null space basis via SVD
//! - `beta_feasibility` — Max-margin LP search for beta>0 in affine solution set
//! - `projection_solver` — Project to constraint null space, optimize reduced objective
//! - `rational_solver` — Exact rational KKT solver

pub mod qp_assembly;
pub mod saddle_point_solver;
pub mod constraint_solver;
pub mod beta_feasibility;
pub mod projection_solver;
pub mod rational_solver;

#[cfg(test)]
#[path = "qp_assembly_test.rs"]
mod qp_assembly_test;

#[cfg(test)]
#[path = "saddle_point_solver_test.rs"]
mod saddle_point_solver_test;

#[cfg(test)]
#[path = "constraint_solver_test.rs"]
mod constraint_solver_test;

#[cfg(test)]
#[path = "beta_feasibility_test.rs"]
mod beta_feasibility_test;

#[cfg(test)]
#[path = "projection_solver_test.rs"]
mod projection_solver_test;

#[cfg(test)]
#[path = "rational_solver_test.rs"]
mod rational_solver_test;

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

/// Trinary verdict for feasibility of beta > 0.
///
/// **Critical invariant:** False is never returned unless certified infeasible.
/// When in doubt, return Indeterminate. The accumulator handles resolution
/// (e.g. via rational fallback).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Certified feasible: all beta_k > eps. Safe to use Q and beta.
    True,
    /// Certified infeasible: no beta > 0 exists in the solution set.
    False,
    /// Ambiguous: beta has near-zero components, or near-null eigenvalues
    /// prevent definitive classification. Q is still valid.
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

/// beta_k > EPS_MARGIN_TRUE -> component is certified positive.
///
/// **Why 1e-9:** This is the classification boundary between True and Indeterminate.
/// It sits between EPS_BETA_POSITIVE (1e-12, eigensolver noise floor) and typical
/// real beta values (O(0.01)--O(10)). A beta in (1e-12, 1e-9) is above noise but
/// ambiguous -- classified as Indeterminate and routed to the rational fallback.
/// Beta above 1e-9 is unambiguously positive (gap of 3 orders of magnitude from noise).
/// The symmetric value for EPS_MARGIN_FALSE ensures symmetric Indeterminate bands.
/// Making it 10x larger (1e-8) would route more valid solutions to the rational
/// fallback unnecessarily; 10x smaller (1e-10) would certify some noisy solutions.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// beta_k < -EPS_MARGIN_FALSE -> component is certified negative (infeasible).
///
/// **Why 1e-9:** Same as EPS_MARGIN_TRUE. The symmetric design means the
/// Indeterminate band is (-1e-9, +1e-9) -- beta in this range is ambiguous.
/// Beta below -1e-9 is unambiguously negative (infeasible), returned as False.
/// Using a symmetric threshold simplifies reasoning: both boundaries are
/// separated from the noise floor (1e-12) by the same factor.
const EPS_MARGIN_FALSE: f64 = 1e-9;

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
/// - margin > +EPS_MARGIN_TRUE -> True (certified positive)
/// - margin < -EPS_MARGIN_FALSE -> False (certified negative)
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
