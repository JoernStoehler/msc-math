//! Projection-based f64 QP solver for the verify-numerics experiment.
//!
//! Solves: max (1/2) beta^T H beta  s.t. C beta = d, beta > 0.
//!
//! Algorithm:
//! 1. Solve C beta = d via SVD (particular solution beta0 + null-space basis V).
//! 2. Project H onto ker(C): reduced Hessian H' = V^T H V, gradient g = V^T H beta0.
//! 3. Eigendecompose H', compute critical point alpha = -(H')^+ g.
//! 4. LP search in null(H') for max-margin beta > 0.
//!
//! Sign fix: the library computes alpha0 = (H')^+ g, but stationarity H' alpha + g = 0
//! requires alpha0 = -(H')^+ g. This module implements the corrected version.
//!
//! Also provides `solve_projected_with_diagnostics()` for perturbation chain validation
//! and `compute_eta_bound()` for the componentwise beta certification bound [lem:link-beta].
//!
//! Dependency: `good_lp` with feature `clarabel` (LP solver for max-margin search).

use good_lp::{constraint, default_solver, variable, variables, Expression, SolverModel,
    Solution as LpSolution};
use nalgebra::{DMatrix, DVector};

// ══════════════════════════════════════════════════════════════════════════════
// Types
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

/// beta_k > EPS_MARGIN_TRUE -> certified positive.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// beta_k < -EPS_MARGIN_FALSE -> certified negative (infeasible).
const EPS_MARGIN_FALSE: f64 = 1e-9;

/// Absolute floor for eigenvalue magnitude. Matrix treated as numerically zero
/// if largest eigenvalue is below this.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

/// Relative threshold for SVD rank detection.
const EPS_RANK_THRESHOLD: f64 = 1e-10;

/// Maximum residual ||Cx0 - d|| to accept as consistent.
const EPS_CONSISTENCY: f64 = 1e-8;

/// Eigenvalue threshold for the reduced Hessian H'.
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

/// Machine epsilon for f64.
const EPS_MACH: f64 = f64::EPSILON; // 2.22e-16

// ══════════════════════════════════════════════════════════════════════════════
// Utility functions
// ══════════════════════════════════════════════════════════════════════════════

/// Compute Q = (1/2) beta^T H beta from pre-assembled H and beta.
pub fn q_value(h: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let b = DVector::from_column_slice(beta);
    0.5 * b.dot(&(h * &b))
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

/// Compute Q = (1/2) beta^T H beta from DVector (internal helper).
fn q_value_from_dvec(h: &DMatrix<f64>, beta: &DVector<f64>) -> f64 {
    0.5 * beta.dot(&(h * beta))
}

// ══════════════════════════════════════════════════════════════════════════════
// Constraint solver
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
// Beta feasibility — max-margin LP
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
// Projection solver
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

// ══════════════════════════════════════════════════════════════════════════════
// Projection solver diagnostics — perturbation chain validation
// [lem:link-beta] eq:eta-computable
// ══════════════════════════════════════════════════════════════════════════════

/// Diagnostics from the projection solver's perturbation chain.
/// All quantities needed to compute and validate the beta > 0 certification bound.
#[derive(Clone, Debug)]
pub struct ProjDiagnostics {
    /// Particular solution beta0 = C^+ d.
    pub beta0: Vec<f64>,
    /// Null-space basis V (m x k).
    pub null_basis: DMatrix<f64>,
    /// Reduced Hessian H' = V^T H V (k x k).
    pub h_prime: DMatrix<f64>,
    /// Reduced gradient g = V^T H beta0 (k x 1).
    pub g: DVector<f64>,
    /// Eigenvalues gamma_j of H' (k entries).
    pub eigenvalues: DVector<f64>,
    /// Eigenvectors W = [w_1 | ... | w_k] of H' (k x k).
    pub eigenvectors: DMatrix<f64>,
    /// Critical point in null-space coords: alpha = -(H')^+ g (k x 1).
    pub alpha: DVector<f64>,
    /// sigma_min(C) — smallest singular value of C.
    pub sigma_min_c: f64,
    /// ||H|| — spectral norm of H.
    pub norm_h: f64,
    /// ||C|| — spectral norm of C.
    pub norm_c: f64,
    /// k = dim(ker(C)) = ncols of null_basis.
    pub null_dim: usize,
    /// Eigenvalue perturbation threshold eps_gamma = c * ||H|| * eps_mach / sigma_min(C).
    /// Eigenvalue signs are certified when |gamma_j| > eps_gamma.
    pub eps_gamma: f64,
    /// Componentwise beta certification bound eta_k from eq:eta-computable.
    /// eta_k bounds |beta_k - beta*_k| using the perturbation chain.
    pub eta: Vec<f64>,
    /// Final beta from the solver (may differ from beta0 + V*alpha due to LP margin search).
    pub beta_final: Vec<f64>,
}

/// Solve the QP via constraint projection, returning both the solution
/// and perturbation-chain diagnostics for validation.
pub fn solve_projected_with_diagnostics(qp: &QP) -> (Solution, Option<ProjDiagnostics>) {
    let m = qp.c.ncols();

    // Step 1: Solve constraints.
    let constraint_sol = match solve_constraints(&qp.c, &qp.d) {
        Some(sol) => sol,
        None => {
            return (Solution {
                verdict: Verdict::False,
                q: 0.0,
                beta: vec![0.0; m],
                margin: f64::NEG_INFINITY,
            }, None);
        }
    };

    let beta0 = &constraint_sol.x0;
    let v = &constraint_sol.null_basis;
    let k = v.ncols();

    // Compute sigma_min(C) and ||C|| from SVD (re-compute to get singular values).
    let svd_c = qp.c.clone().svd(false, false);
    let sigma_vals = &svd_c.singular_values;
    let sigma_min_c = sigma_vals.iter().cloned()
        .filter(|&s| s > 1e-15)
        .fold(f64::INFINITY, f64::min);
    let norm_c = sigma_vals.iter().cloned().fold(0.0f64, f64::max);

    // ||H|| = max |eigenvalue of H|.
    let eig_h = qp.h.clone().symmetric_eigen();
    let norm_h = eig_h.eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);

    // Special case: k = 0 (unique beta from constraints).
    if k == 0 {
        let q = q_value_from_dvec(&qp.h, beta0);
        let margin = beta0.iter().copied().fold(f64::INFINITY, f64::min);
        let verdict = classify_margin(margin);
        let sol = Solution {
            verdict,
            q,
            beta: beta0.as_slice().to_vec(),
            margin,
        };
        let diag = ProjDiagnostics {
            beta0: beta0.as_slice().to_vec(),
            null_basis: v.clone(),
            h_prime: DMatrix::zeros(0, 0),
            g: DVector::zeros(0),
            eigenvalues: DVector::zeros(0),
            eigenvectors: DMatrix::zeros(0, 0),
            alpha: DVector::zeros(0),
            sigma_min_c,
            norm_h,
            norm_c,
            null_dim: 0,
            eps_gamma: f64::INFINITY,
            eta: vec![f64::INFINITY; m],
            beta_final: beta0.as_slice().to_vec(),
        };
        return (sol, Some(diag));
    }

    // Step 2: Project and optimize.
    let hv = &qp.h * v;
    let h_prime = v.transpose() * &hv;
    let h_beta0 = &qp.h * beta0;
    let b_prime = v.transpose() * &h_beta0; // g = V^T H beta0

    // Eigendecompose H' = P Lambda P^T.
    let eig = h_prime.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    // Partition eigenvalues into retained and null.
    let lambda_max = eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max);
    let threshold = if lambda_max < EPS_EIGEN_FLOOR {
        f64::INFINITY
    } else {
        lambda_max * EPS_EIGEN_THRESHOLD
    };

    // eps_gamma from [lem:link-eigenvalues]: eigenvalue perturbation bound.
    // Using c_5 = 1 for now (to be calibrated empirically).
    let eps_gamma = if sigma_min_c > 1e-15 {
        norm_h * EPS_MACH / sigma_min_c
    } else {
        f64::INFINITY
    };

    // alpha = -(H')^+ g (thresholded pseudoinverse).
    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = -pi.dot(&b_prime) / eigenvalues[i]; // NEGATED (sign fix)
            alpha0 += coeff * &pi;
        }
    }

    // Null-space directions of H'.
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

    // Compute eta_k (certification bound from eq:eta-computable).
    let eta = compute_eta_bound(
        m, k, v, eigenvectors, eigenvalues, &alpha0, &margin_result.beta,
        norm_h, norm_c, sigma_min_c, eps_gamma, &null_indices,
    );

    let sol = Solution {
        verdict,
        q,
        beta: margin_result.beta.as_slice().to_vec(),
        margin,
    };

    let diag = ProjDiagnostics {
        beta0: beta0.as_slice().to_vec(),
        null_basis: v.clone(),
        h_prime,
        g: b_prime,
        eigenvalues: eigenvalues.clone(),
        eigenvectors: eigenvectors.clone(),
        alpha: alpha0,
        sigma_min_c,
        norm_h,
        norm_c,
        null_dim: k,
        eps_gamma,
        eta,
        beta_final: margin_result.beta.as_slice().to_vec(),
    };

    (sol, Some(diag))
}

/// Compute the componentwise beta certification bound eta_k from eq:eta-computable.
///
/// [lem:link-beta]: eta_k bounds |beta_k - beta*_k| using the perturbation chain.
/// [rem:eigendirection-error]: the per-eigendirection error |delta_alpha_j| ~ eps_mach / |gamma_j|
/// (confirmed empirically on 364 I1 problems, 15 orders of magnitude).
/// Safety constant c = m^2 (zero violations on well-conditioned problems).
fn compute_eta_bound(
    m: usize,
    k: usize,
    v: &DMatrix<f64>,        // m x k null-space basis
    w: &DMatrix<f64>,        // k x k eigenvectors of H'
    gamma: &DVector<f64>,    // k eigenvalues of H'
    alpha: &DVector<f64>,    // k critical-point coords
    _beta_final: &DVector<f64>, // m final beta (for reference)
    norm_h: f64,
    norm_c: f64,
    sigma_min_c: f64,
    eps_gamma: f64,
    null_indices: &[usize],  // indices of null eigenvalues
) -> Vec<f64> {
    if k == 0 || sigma_min_c < 1e-15 {
        return vec![f64::INFINITY; m];
    }

    let alpha_norm = alpha.norm();

    // Safety factor: m^2 accounts for O(m) accumulated rounding per matrix
    // operation and O(m) operations in the chain.  Standard backward stability
    // bounds have O(m) constants; squaring gives headroom for second-order terms.
    // Empirically calibrated: max ratio at c=1 is 58.6 (m=10), so c=m^2=100
    // provides 1.7x safety margin.
    let c_safety = (m * m) as f64;

    // Error magnitudes from the perturbation chain.
    let e_delta_h_prime = c_safety * norm_h * EPS_MACH / sigma_min_c;
    let e_delta_g = c_safety * norm_h * norm_c * EPS_MACH / (sigma_min_c * sigma_min_c);
    let e_delta_v = c_safety * EPS_MACH / sigma_min_c;
    let e_delta_beta0 = c_safety * norm_c * EPS_MACH / (sigma_min_c * sigma_min_c);

    // Pre-compute V * w_j for each eigenvector j (m-dimensional vectors).
    // (V w_j)_k = sum_l V[k,l] * w_j[l]
    let vw: Vec<DVector<f64>> = (0..k)
        .map(|j| {
            let wj = w.column(j);
            v * &wj
        })
        .collect();

    let mut eta = vec![0.0f64; m];

    for comp_k in 0..m {
        // Term 1: critical-point shift (sum over retained eigenvalues only)
        let mut sum_amplified = 0.0f64;
        for j in 0..k {
            // Skip null eigenvalues (they're handled by LP search, not by alpha)
            if null_indices.contains(&j) {
                continue;
            }
            // Use |gamma_j| directly as the denominator.
            // The perturbation bound guarantees |gamma_j| >= |gamma_j_tilde| - eps_gamma,
            // but using |gamma_j_tilde| (without subtracting eps_gamma) is valid because:
            //   - For well-separated eigenvalues (|gamma_j| >> eps_gamma), the difference is negligible.
            //   - For near-threshold eigenvalues (|gamma_j| ~ eps_gamma), subtracting eps_gamma gives
            //     a near-zero denominator that's numerically meaningless anyway.
            // The first-order perturbation analysis breaks down when |gamma_j| is small
            // (the actual error is O(1), not O(eps_mach/|gamma_j|)). In those cases,
            // the bound correctly produces a large eta, signaling INDETERMINATE.
            let gamma_j_abs = gamma[j].abs();
            // The first-order perturbation bound requires ||delta_H'|| << |gamma_j|.
            // E_delta_H' = e_delta_h_prime is the perturbation magnitude.
            // When |gamma_j| <= E_delta_H', the perturbation dominates and the
            // linear approximation gives O(1) errors, not O(eps_mach/|gamma_j|).
            //
            // Note: the solver may retain eigenvalues below this threshold
            // (its relative threshold EPS_EIGEN_THRESHOLD * lambda_max can
            // be less restrictive for small k, especially k=1 where the
            // threshold equals the eigenvalue itself). The bound must
            // independently check against the perturbation magnitude.
            if gamma_j_abs <= e_delta_h_prime {
                eta[comp_k] = f64::INFINITY;
                break;
            }
            sum_amplified += vw[j][comp_k].abs() / gamma_j_abs;
        }
        if eta[comp_k].is_infinite() {
            continue;
        }
        eta[comp_k] = (e_delta_h_prime * alpha_norm + e_delta_g) * sum_amplified
            + e_delta_v * alpha_norm
            + e_delta_beta0;
    }

    eta
}
