//! Projection-based solver for the constrained QP.
//!
//! Solves: max (1/2) beta^T H beta  subject to  C beta = d, beta > 0.
//!
//! # Algorithm
//!
//! **Step 1 -- Solve constraints.** C beta = d -> particular solution beta0, null-space
//! basis V. If inconsistent: return False.
//!
//! **Step 2 -- Project objective.** Form the reduced Hessian H' = V^T H V and reduced
//! gradient b' = V^T H beta0. Solve H' alpha = b' via eigendecomposition, partitioning
//! eigenvalues into retained (|lambda| > threshold) and null (|lambda| <= threshold).
//!
//! **Step 3 -- Compose search space.** The full solution is beta = beta0 + V(alpha0 + W gamma),
//! where W are the null-space eigenvectors of H'. These directions don't change Q
//! but can change beta -- so they're the search space for finding beta > 0.
//!
//! **Step 4 -- Max-margin search.** Find gamma maximizing min_k beta_k via
//! `beta_feasibility::find_max_margin`. Classify the verdict from the margin.
//!
//! **Step 5 -- Compute Q.** Q = (1/2) beta^T H beta, constant over the solution set.
//!
//! Mathematical correspondence: [lem:kkt], Part C.2 of algorithm design

use super::constraint_solver;
use super::beta_feasibility;
use super::{classify_margin, QP, Solution, Verdict};
use nalgebra::{DMatrix, DVector};

/// Eigenvalue threshold for the reduced Hessian H'.
///
/// Near-null eigenvalues mean Q varies little along those directions but beta varies
/// a lot. These directions are included in the margin search space rather than
/// used for optimization.
///
/// Same role as EIGEN_CONDITION_TAU in saddle_point_solver.rs (1e-3).
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

/// Absolute floor: if max|lambda| of H' is below this, the entire reduced Hessian
/// is numerically zero. Q = 0 along all null-space directions.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

/// Solve the QP via constraint projection.
///
/// See module doc for the 5-step algorithm. Returns a Solution with verdict,
/// Q value, beta vector, and margin.
///
/// [lem:kkt]
pub fn solve_projected(qp: &QP) -> Solution {
    let m = qp.c.ncols();

    // Step 1: Solve constraints.
    let constraint_sol = match constraint_solver::solve_constraints(&qp.c, &qp.d) {
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

    // Particular solution for H' alpha = b' using retained eigenvalues (pseudoinverse).
    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = pi.dot(&b_prime) / eigenvalues[i];
            alpha0 += coeff * pi;
        }
    }

    // Null-space directions of H' (columns of W in alpha-space).
    let null_indices: Vec<usize> = (0..k)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .collect();

    // Step 3: Compose search space.
    // beta_base = beta0 + V * alpha0 (the "optimized" particular solution).
    let beta_base = beta0 + v * &alpha0;

    // V_search = V * W_alpha (m x |null_indices|).
    // These are the directions in beta-space that don't change Q.
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
    let margin_result = beta_feasibility::find_max_margin(&beta_base, &v_search);

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
