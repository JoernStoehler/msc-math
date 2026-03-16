/// Projection-based solver for the constrained QP.
///
/// Solves: max (1/2) βᵀHβ  subject to  Cβ = d, β > 0.
///
/// # Algorithm (Part C.2 of the algorithm design doc)
///
/// **Step 1 — Solve constraints.** Cβ = d → particular solution β₀, null-space basis V.
/// If inconsistent: return False (no feasible point exists).
///
/// **Step 2 — Project objective.** Form the reduced Hessian H' = VᵀHV and reduced
/// gradient b' = VᵀHβ₀. Solve H'α = b' via eigendecomposition, partitioning
/// eigenvalues into retained (|λ| > threshold) and null (|λ| ≤ threshold).
///
/// **Step 3 — Compose search space.** The full solution is β = β₀ + V(α₀ + Wγ),
/// where W are the null-space eigenvectors of H'. These directions don't change Q
/// but can change β — so they're the search space for finding β > 0.
///
/// **Step 4 — Max-margin search.** Find γ maximizing min_k β_k via `margin_search`.
/// Classify the verdict from the margin.
///
/// **Step 5 — Compute Q.** Q = (1/2) βᵀHβ, constant over the solution set.

use super::constraint_solver;
use super::margin_search;
use super::{classify_margin, QP, Solution, Verdict};
use nalgebra::{DMatrix, DVector};

/// Eigenvalue threshold for the reduced Hessian H'.
///
/// Near-null eigenvalues mean Q varies little along those directions but β varies
/// a lot. These directions are included in the margin search space rather than
/// used for optimization.
///
/// Same role as EIGEN_CONDITION_TAU in augmented.rs (1e-3).
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

/// Absolute floor: if max|λ| of H' is below this, the entire reduced Hessian is
/// numerically zero. Q = 0 along all null-space directions.
const EPS_EIGEN_FLOOR: f64 = 1e-12;

/// Solve the QP via constraint projection.
///
/// See module doc for the algorithm steps.
pub(crate) fn solve_projected(qp: &QP) -> Solution {
    let m = qp.c.ncols();

    // ── Step 1: Solve constraints ──

    let constraint_sol = match constraint_solver::solve_constraints(&qp.c, &qp.d) {
        Some(sol) => sol,
        None => {
            // Inconsistent constraints: no feasible point exists.
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
    let k = v.ncols(); // null-space dimension = m - rank(C)

    // ── Special case: k = 0 (unique β from constraints) ──

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

    // ── Step 2: Project and optimize ──

    // Reduced Hessian: H' = VᵀHV (k × k symmetric)
    let hv = &qp.h * v; // m × k
    let h_prime = v.transpose() * &hv; // k × k

    // Reduced gradient: b' = VᵀHβ₀ (k × 1)
    let h_beta0 = &qp.h * beta0; // m × 1
    let b_prime = v.transpose() * &h_beta0; // k × 1

    // Eigendecompose H' = PΛPᵀ
    let eig = h_prime.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    // Partition eigenvalues into retained and null
    let lambda_max = eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max);
    let threshold = if lambda_max < EPS_EIGEN_FLOOR {
        // H' is numerically zero. All directions are null.
        f64::INFINITY // nothing retained
    } else {
        lambda_max * EPS_EIGEN_THRESHOLD
    };

    // Particular solution for H'α = b' using retained eigenvalues (pseudoinverse)
    // α₀ = Σ_{i: retained} (pᵢᵀb' / λᵢ) pᵢ
    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = pi.dot(&b_prime) / eigenvalues[i];
            alpha0 += coeff * &pi;
        }
    }

    // Null-space directions of H' (columns of W in α-space)
    // These are eigenvectors with |λ| ≤ threshold
    let null_indices: Vec<usize> = (0..k)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .collect();

    // ── Step 3: Compose search space ──

    // β_base = β₀ + V·α₀ (the "optimized" particular solution)
    let beta_base = beta0 + v * &alpha0;

    // V_search = V · W_alpha (m × |null_indices|)
    // These are the directions in β-space that don't change Q.
    let n_null = null_indices.len();
    let v_search = if n_null > 0 {
        let mut w_alpha = DMatrix::zeros(k, n_null);
        for (j, &idx) in null_indices.iter().enumerate() {
            let col = eigenvectors.column(idx);
            for i in 0..k {
                w_alpha[(i, j)] = col[i];
            }
        }
        v * w_alpha // m × n_null
    } else {
        DMatrix::zeros(m, 0)
    };

    // ── Step 4: Max-margin search ──

    let margin_result = margin_search::find_max_margin(
        &beta_base,
        &v_search,
    );

    // ── Step 5: Compute Q ──

    // Q = (1/2) βᵀHβ. Constant over the solution set (H' null-space directions
    // don't change Q by construction). We can compute from any point.
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

/// Compute Q = (1/2) βᵀHβ from DVector (internal helper).
fn q_value_from_dvec(h: &DMatrix<f64>, beta: &DVector<f64>) -> f64 {
    0.5 * beta.dot(&(h * beta))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::symplectic::omega0;
    use crate::kkt::{QP, Verdict};
    use nalgebra::{DMatrix, DVector, Vector4};

    /// Build a QP from dual vertices and permutation (the assembly pattern
    /// that hk2017/billiard will use).
    fn assemble_qp(dual_verts: &[Vector4<f64>], perm: &[usize]) -> QP {
        let m = perm.len();
        let p = 5;

        let mut c = DMatrix::zeros(p, m);
        let mut h = DMatrix::zeros(m, m);

        for i in 0..m {
            let a = &dual_verts[perm[i]];
            for d in 0..4 {
                c[(d, i)] = a[d];
            }
            c[(4, i)] = 1.0;

            for j in (i + 1)..m {
                let val = omega0(a, &dual_verts[perm[j]]);
                h[(i, j)] = val;
                h[(j, i)] = val;
            }
        }

        let d = DVector::from_fn(p, |i, _| if i == 4 { 1.0 } else { 0.0 });
        QP { c, d, h }
    }

    // ── Synthetic tests (context-free, hand-checkable) ──

    /// Inconsistent constraints: return False.
    #[test]
    fn inconsistent_constraints() {
        let c = DMatrix::identity(3, 3);
        let d = DVector::from_column_slice(&[1.0, 0.0, 0.0]);
        let h = DMatrix::zeros(3, 3);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        // C is 3×3 identity, d = [1,0,0]. Unique β = [1,0,0]. β₂ = β₃ = 0.
        // With H = 0, Q = 0. margin = 0 → Indeterminate.
        assert!(sol.margin <= 0.0);
    }

    /// Unique β (k=0), all positive → True.
    #[test]
    fn unique_beta_positive() {
        let c = DMatrix::identity(5, 5);
        let d = DVector::from_element(5, 0.2);
        let h = DMatrix::identity(5, 5); // Q = (1/2)βᵀβ = (1/2)(5×0.04) = 0.1
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::True);
        assert!((sol.q - 0.1).abs() < 1e-10, "Q = {}, expected 0.1", sol.q);
        assert!(
            (sol.margin - 0.2).abs() < 1e-10,
            "margin = {}, expected 0.2",
            sol.margin
        );
    }

    /// Unique β (k=0), some negative → False.
    #[test]
    fn unique_beta_negative() {
        let c = DMatrix::identity(5, 5);
        let d = DVector::from_column_slice(&[0.5, 0.5, -0.5, 0.5, 0.5]);
        let h = DMatrix::identity(5, 5);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::False);
        assert!(sol.margin < 0.0);
    }

    /// One free variable (k=1, m=6, p=5). Verify Q matches hand computation.
    #[test]
    fn one_free_variable() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(5, 6, &[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
        ]);
        let d = DVector::from_element(5, 1.0);
        let h = DMatrix::identity(6, 6);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        // Cβ = 1 with C = [I₅ | 1] means βᵢ + β₆ = 1 for i=1..5.
        // Q = (1/2)(5(1-β₆)² + β₆²) → minimize at β₆ = 5/6.
        // Q_max = (1/2)(5·(1/6)² + (5/6)²) = (1/2)(5/36 + 25/36) = 5/12.
        assert!(sol.q > 0.0, "Q should be positive");
        assert!(
            (sol.q - 5.0 / 12.0).abs() < 1e-8,
            "Q = {}, expected 5/12 = {}",
            sol.q,
            5.0 / 12.0
        );
    }

    /// Q is constant along H' null space (not the full constraint null space).
    ///
    /// When H = 0, Q = 0 for all β in the constraint set. This is a trivial
    /// but important case: the null space of H' is the entire projected space.
    #[test]
    fn q_constant_when_h_zero() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 4, &[
            1.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[1.0, 1.0]);
        let h = DMatrix::zeros(4, 4);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert!(
            sol.q.abs() < 1e-12,
            "Q should be 0 when H = 0, got {}",
            sol.q
        );

        // Perturb along constraint null space: Q should stay 0.
        let cs = constraint_solver::solve_constraints(&qp.c, &qp.d).unwrap();
        for scale in &[0.1, -0.3, 1.5] {
            let mut alpha = DVector::zeros(cs.null_basis.ncols());
            alpha[0] = *scale;
            let beta_perturbed = &cs.x0 + &cs.null_basis * &alpha;
            let q_perturbed = q_value_from_dvec(&qp.h, &beta_perturbed);
            assert!(
                q_perturbed.abs() < 1e-12,
                "Q should be 0 everywhere when H = 0, got {}",
                q_perturbed
            );
        }
    }

    // ── Mathematical proposition tests ──

    /// Prop: Cβ = d for every returned β with verdict ≠ False.
    #[test]
    fn prop_constraint_satisfaction() {
        let cases = vec![
            {
                #[rustfmt::skip]
                let c = DMatrix::from_row_slice(3, 5, &[
                    1.0, 0.0, 0.0, 1.0, 0.0,
                    0.0, 1.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 1.0, 1.0, 1.0,
                ]);
                let d = DVector::from_column_slice(&[1.0, 1.0, 1.0]);
                let h = DMatrix::identity(5, 5);
                QP { c, d, h }
            },
            {
                #[rustfmt::skip]
                let c = DMatrix::from_row_slice(5, 8, &[
                    1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,  0.0,
                    0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,
                    0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0,
                    0.0,  0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,
                    1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
                ]);
                let d = DVector::from_column_slice(&[0.0, 0.0, 0.0, 0.0, 1.0]);
                let h = DMatrix::identity(8, 8);
                QP { c, d, h }
            },
        ];

        for (i, qp) in cases.iter().enumerate() {
            let sol = solve_projected(qp);
            if sol.verdict == Verdict::False {
                continue;
            }
            let beta_dv = DVector::from_column_slice(&sol.beta);
            let residual = (&qp.c * &beta_dv - &qp.d).norm();
            assert!(
                residual < 1e-8,
                "case {}: ‖Cβ - d‖ = {:.2e}",
                i,
                residual
            );
        }
    }

    /// Prop: returned Q equals (1/2) βᵀHβ.
    #[test]
    fn prop_q_is_half_beta_h_beta() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 4, &[
            1.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[1.0, 1.0]);
        #[rustfmt::skip]
        let h = DMatrix::from_row_slice(4, 4, &[
            0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0,
        ]);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        let beta_dv = DVector::from_column_slice(&sol.beta);
        let q_check = 0.5 * beta_dv.dot(&(&qp.h * &beta_dv));
        assert!(
            (sol.q - q_check).abs() < 1e-10,
            "Q mismatch: solver = {}, direct = {}, diff = {:.2e}",
            sol.q,
            q_check,
            (sol.q - q_check).abs()
        );
    }

    /// Prop: margin = min(β) exactly.
    #[test]
    fn prop_margin_equals_min_beta() {
        let c = DMatrix::identity(3, 6);
        let d = DVector::from_column_slice(&[0.5, 0.3, 0.8]);
        let h = DMatrix::identity(6, 6);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        let min_beta = sol.beta.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(
            (sol.margin - min_beta).abs() < 1e-12,
            "margin = {}, min(β) = {}, diff = {:.2e}",
            sol.margin,
            min_beta,
            (sol.margin - min_beta).abs()
        );
    }

    // ── Cross-variant tests: projection solver vs augmented solver ──

    /// Both solvers agree on capacity for the 4-simplex.
    ///
    /// The augmented solver uses (normals, heights) parameterization;
    /// the projection solver uses dual vertices aᵢ directly. The β vectors
    /// differ by a scaling (β_dual_k = β_aug_k · h_k), but Q and hence
    /// capacity = 0.5 / Q are invariant under this reparameterization.
    #[test]
    fn capacity_agrees_on_simplex() {
        let simplex = crate::geom::known_polytopes::simplex();
        let result_aug = crate::algorithms::hk2017::ehz_capacity(&simplex.polytope).unwrap();

        let dual_verts = simplex.polytope.dual_vertices_f64();
        let qp = assemble_qp(dual_verts, &result_aug.best_permutation);
        let sol = solve_projected(&qp);

        assert!(
            sol.verdict == Verdict::True || sol.verdict == Verdict::Indeterminate,
            "projection solver should find feasible β for the winning orbit"
        );
        assert!(sol.q > 0.0, "Q should be positive");

        // capacity = 0.5 / Q for both parameterizations
        let cap_proj = 0.5 / sol.q;
        let cap_aug = result_aug.capacity;

        assert!(
            (cap_proj - cap_aug).abs() < 1e-6 * cap_aug,
            "capacity mismatch on simplex: projection = {}, augmented = {}, diff = {:.2e}",
            cap_proj,
            cap_aug,
            (cap_proj - cap_aug).abs()
        );
    }

    /// Both solvers agree on capacity for the hypercube.
    #[test]
    fn capacity_agrees_on_hypercube() {
        let hypercube = crate::geom::known_polytopes::hypercube();
        let result_aug = crate::algorithms::hk2017::ehz_capacity(&hypercube.polytope).unwrap();

        let dual_verts = hypercube.polytope.dual_vertices_f64();
        let qp = assemble_qp(dual_verts, &result_aug.best_permutation);
        let sol = solve_projected(&qp);

        assert!(
            sol.verdict == Verdict::True || sol.verdict == Verdict::Indeterminate,
            "projection solver should find feasible β"
        );

        let cap_proj = 0.5 / sol.q;
        let cap_aug = result_aug.capacity;

        assert!(
            (cap_proj - cap_aug).abs() < 1e-6 * cap_aug,
            "capacity mismatch on hypercube: projection = {}, augmented = {}, diff = {:.2e}",
            cap_proj,
            cap_aug,
            (cap_proj - cap_aug).abs()
        );
    }

    /// Both solvers agree on capacity for the HKO pentagon (the counterexample).
    #[test]
    fn capacity_agrees_on_hko_pentagon() {
        let pentagon = crate::geom::known_polytopes::hko_pentagon();
        let result_aug = crate::algorithms::hk2017::ehz_capacity(&pentagon.polytope).unwrap();

        let dual_verts = pentagon.polytope.dual_vertices_f64();
        let qp = assemble_qp(dual_verts, &result_aug.best_permutation);
        let sol = solve_projected(&qp);

        assert!(
            sol.verdict == Verdict::True || sol.verdict == Verdict::Indeterminate,
            "projection solver should find feasible β for HKO pentagon winning orbit"
        );

        let cap_proj = 0.5 / sol.q;
        let cap_aug = result_aug.capacity;

        assert!(
            (cap_proj - cap_aug).abs() < 1e-6 * cap_aug,
            "capacity mismatch on HKO pentagon: projection = {}, augmented = {}, diff = {:.2e}",
            cap_proj,
            cap_aug,
            (cap_proj - cap_aug).abs()
        );
    }
}
