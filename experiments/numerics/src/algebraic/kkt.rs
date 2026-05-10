//! Exact KKT solve over experiment-owned reporting scalars.
//!
//! This is the experiment analogue of `symplectic::kkt::rational_solver` and
//! `symplectic::exact::orbit`. It solves a selected sigma exactly, including
//! the rank-deficient/null-space positivity path, but does not attempt
//! exhaustive exact search across all sigmas.
//!
//! TODO: add [lem:...] to formal math for the exact KKT system assembly used
//! here.
//! TODO: add [lem:...] to formal math for the rank-deficient exact positivity
//! search path used here.

use super::field::{is_strictly_negative, is_strictly_positive, ExperimentScalar};
use super::geom::omega0;
use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use nalgebra::{DMatrix, DVector};

/// Result of an exact KKT solve over an experiment-owned exact scalar.
#[derive(Clone, Debug, PartialEq)]
pub struct ExactKktResult<F: ExperimentScalar + 'static> {
    /// Exact beta vector, aligned with the supplied sigma.
    pub beta: Vec<F>,
    /// Exact `Q(beta)`.
    pub q_exact: F,
    /// Best-effort `f64` approximation for comparison with existing numerics.
    pub q_exact_f64: f64,
}

/// Solve the selected KKT system exactly for one sigma.
pub fn solve_kkt_exact<F: ExperimentScalar + 'static>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
) -> Option<ExactKktResult<F>> {
    let m = sigma.len();
    let (matrix, rhs) = build_kkt_matrix(dual_vertices, sigma);

    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => None,
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => {
            let solution: Vec<F> = particular.iter().cloned().collect();
            let beta = solution[..m].to_vec();
            if !beta.iter().all(is_strictly_positive) {
                return None;
            }
            let q_exact = compute_q_exact(dual_vertices, sigma, &beta);
            Some(ExactKktResult {
                q_exact_f64: q_exact.to_f64(),
                q_exact,
                beta,
            })
        }
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            let beta0: Vec<F> = particular.iter().take(m).cloned().collect();
            let null_beta: Vec<Vec<F>> = (0..kernel_basis.ncols())
                .map(|col| (0..m).map(|row| kernel_basis[(row, col)].clone()).collect())
                .collect();
            let beta = find_positive_beta(&beta0, &null_beta)?;
            let q_exact = compute_q_exact(dual_vertices, sigma, &beta);
            Some(ExactKktResult {
                q_exact_f64: q_exact.to_f64(),
                q_exact,
                beta,
            })
        }
    }
}

fn build_kkt_matrix<F: ExperimentScalar + 'static>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
) -> (DMatrix<F>, DVector<F>) {
    let m = sigma.len();
    let size = m + 5;
    let mut matrix = DMatrix::from_element(size, size, F::zero());
    let mut rhs = DVector::from_element(size, F::zero());

    for i in 0..m {
        for j in (i + 1)..m {
            let omega = omega0(&dual_vertices[sigma[i]], &dual_vertices[sigma[j]]);
            matrix[(i, j)] = omega.clone();
            matrix[(j, i)] = omega;
        }
    }

    for i in 0..m {
        for dim in 0..4 {
            let value = dual_vertices[sigma[i]][dim].clone();
            matrix[(i, m + dim)] = value.clone();
            matrix[(m + dim, i)] = value;
        }
    }

    for i in 0..m {
        matrix[(i, m + 4)] = F::one();
        matrix[(m + 4, i)] = F::one();
    }
    rhs[m + 4] = F::one();

    (matrix, rhs)
}

fn compute_q_exact<F: ExperimentScalar>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
    beta: &[F],
) -> F {
    let m = beta.len();
    let mut sum = F::zero();
    for i in 1..m {
        for j in 0..i {
            sum = sum
                + beta[i].clone()
                    * beta[j].clone()
                    * omega0(&dual_vertices[sigma[j]], &dual_vertices[sigma[i]]);
        }
    }
    sum
}

fn find_positive_beta<F: ExperimentScalar>(beta0: &[F], null_vecs: &[Vec<F>]) -> Option<Vec<F>> {
    let m = beta0.len();
    let k = null_vecs.len();

    type Constraint<F> = (Vec<F>, F);

    let mut constraints: Vec<Constraint<F>> = (0..m)
        .map(|row| {
            let coeffs = (0..k).map(|col| null_vecs[col][row].clone()).collect();
            (coeffs, -beta0[row].clone())
        })
        .collect();

    struct Bound<F: ExperimentScalar> {
        remaining_coeffs: Vec<F>,
        rhs: F,
        divisor: F,
    }

    let mut stages: Vec<Vec<Bound<F>>> = Vec::with_capacity(k);

    for elim_idx in (0..k).rev() {
        let mut bounds = Vec::new();
        let mut positive = Vec::new();
        let mut negative = Vec::new();
        let mut new_constraints = Vec::new();

        for constraint in &constraints {
            let coeff = &constraint.0[elim_idx];
            if is_strictly_positive(coeff) {
                positive.push(constraint);
            } else if is_strictly_negative(coeff) {
                negative.push(constraint);
            } else {
                let mut new_coeffs = constraint.0.clone();
                new_coeffs.remove(elim_idx);
                new_constraints.push((new_coeffs, constraint.1.clone()));
            }
        }

        for constraint in positive.iter().chain(negative.iter()) {
            let mut remaining = constraint.0.clone();
            let divisor = remaining.remove(elim_idx);
            bounds.push(Bound {
                remaining_coeffs: remaining,
                rhs: constraint.1.clone(),
                divisor,
            });
        }
        stages.push(bounds);

        for (c_l, r_l) in &positive {
            for (c_u, r_u) in &negative {
                let a_l = &c_l[elim_idx];
                let a_u = &c_u[elim_idx];
                let mut coeffs = Vec::with_capacity(c_l.len() - 1);
                for idx in 0..c_l.len() {
                    if idx == elim_idx {
                        continue;
                    }
                    coeffs.push(a_l.clone() * c_u[idx].clone() - a_u.clone() * c_l[idx].clone());
                }
                let rhs = a_l.clone() * r_u.clone() - a_u.clone() * r_l.clone();
                new_constraints.push((coeffs, rhs));
            }
        }

        constraints = new_constraints;
    }

    for (coeffs, rhs) in &constraints {
        assert!(
            coeffs.is_empty(),
            "final Fourier-Motzkin stage should eliminate all variables"
        );
        if !is_strictly_negative(rhs) {
            return None;
        }
    }

    let two = F::one() + F::one();
    let mut alpha = vec![F::zero(); k];

    for assign_var in 0..k {
        let stage_idx = k - 1 - assign_var;
        let mut lower: Option<F> = None;
        let mut upper: Option<F> = None;

        for bound in &stages[stage_idx] {
            let mut numerator = bound.rhs.clone();
            for (idx, coeff) in bound.remaining_coeffs.iter().enumerate() {
                numerator = numerator - coeff.clone() * alpha[idx].clone();
            }
            let value = numerator / bound.divisor.clone();
            if is_strictly_positive(&bound.divisor) {
                lower = Some(match lower.take() {
                    Some(old) => old.max(value),
                    None => value,
                });
            } else {
                upper = Some(match upper.take() {
                    Some(old) => old.min(value),
                    None => value,
                });
            }
        }

        alpha[assign_var] = match (lower, upper) {
            (Some(lo), Some(hi)) => {
                assert!(lo < hi, "strict bounds should leave a non-empty interval");
                (lo + hi) / two.clone()
            }
            (Some(lo), None) => lo + F::one(),
            (None, Some(hi)) => hi - F::one(),
            (None, None) => F::zero(),
        };
    }

    let beta: Vec<F> = (0..m)
        .map(|row| {
            let mut value = beta0[row].clone();
            for col in 0..k {
                value = value + alpha[col].clone() * null_vecs[col][row].clone();
            }
            value
        })
        .collect();

    assert!(beta.iter().all(is_strictly_positive));
    Some(beta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic::fixtures::{
        exact_hko_pentagon, exact_hypercube, exact_simplex, hko_capacity_formula_f64,
        HKO_RANK_DEFICIENT_SIGMA, HKO_WINNING_SIGMA,
    };
    use symplectic::ehz_capacity_pruned;
    use symplectic::geom::known_polytopes;
    use symplectic::kkt::rational_solver as library_rational_solver;

    fn assert_close(lhs: f64, rhs: f64, tolerance: f64, label: &str) {
        let diff = (lhs - rhs).abs();
        assert!(
            diff <= tolerance,
            "{label}: |{lhs} - {rhs}| = {diff} > {tolerance}"
        );
    }

    #[test]
    fn simplex_best_sigma_matches_library_rational_solver() {
        let exact = exact_simplex().expect("exact simplex");
        let library = known_polytopes::simplex();
        let best = ehz_capacity_pruned(&library.polytope).expect("library simplex capacity");

        let exact_result =
            solve_kkt_exact(exact.dual_vertices(), best.best_sigma()).expect("exact simplex sigma");
        let library_result = library_rational_solver::solve_kkt_exact(
            library.polytope.dual_vertices(),
            best.best_sigma(),
        )
        .expect("library rational simplex sigma");

        assert_eq!(exact_result.q_exact, library_result.q_exact);
        assert_eq!(exact_result.beta, library_result.beta);
    }

    #[test]
    fn hypercube_best_sigma_matches_library_rational_solver() {
        let exact = exact_hypercube().expect("exact hypercube");
        let library = known_polytopes::hypercube();
        let best = ehz_capacity_pruned(&library.polytope).expect("library hypercube capacity");

        let exact_result = solve_kkt_exact(exact.dual_vertices(), best.best_sigma())
            .expect("exact hypercube sigma");
        let library_result = library_rational_solver::solve_kkt_exact(
            library.polytope.dual_vertices(),
            best.best_sigma(),
        )
        .expect("library rational hypercube sigma");

        assert_eq!(exact_result.q_exact, library_result.q_exact);
        assert_eq!(exact_result.beta, library_result.beta);
    }

    #[test]
    fn hko_selected_winning_sigma_matches_closed_formula() {
        let exact = exact_hko_pentagon().expect("exact hko");
        let result = solve_kkt_exact(exact.dual_vertices(), HKO_WINNING_SIGMA)
            .expect("exact hko winning sigma");

        assert!(result.q_exact_f64 > 0.14);
        assert!(result.beta.iter().all(is_strictly_positive));
        let action_f64 = 1.0 / (2.0 * result.q_exact_f64);
        assert_close(
            action_f64,
            hko_capacity_formula_f64(),
            1.0e-12,
            "winning action",
        );
    }

    #[test]
    fn hko_rank_deficient_sigma_matches_closed_formula() {
        let exact = exact_hko_pentagon().expect("exact hko");
        let result = solve_kkt_exact(exact.dual_vertices(), HKO_RANK_DEFICIENT_SIGMA)
            .expect("exact hko rank-deficient sigma");

        assert!(result.q_exact_f64 > 0.0);
        assert!(result.beta.iter().all(is_strictly_positive));
        let action_f64 = 1.0 / (2.0 * result.q_exact_f64);
        assert_close(
            action_f64,
            hko_capacity_formula_f64(),
            1.0e-12,
            "rank-deficient action",
        );
    }
}
