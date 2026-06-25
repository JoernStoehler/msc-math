//! Exact one-sigma KKT payload and solve.
//!
//! This is the exact-field analogue of the single-orbit building blocks behind
//! the floating-point `OrbitKktData` path.
//!
//! TODO: add [lem:kkt] to formal math for the exact KKT system assembly used
//! here.
//! TODO: add [lem:well-defined] to formal math for the rank-deficient exact
//! positivity search path used here.

use crate::exact::polytope::omega0;
use algebraic_numbers::{solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Vector4};

/// Exact one-sigma orbit payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactOrbitKktData<F: ExactScalar + 'static> {
    pub sigma: Vec<usize>,
    pub beta: Vec<F>,
    pub q: F,
    pub mu: Vector4<F>,
    pub xi: F,
}

impl<F: ExactScalar + 'static> ExactOrbitKktData<F> {
    pub fn action(&self) -> F {
        F::one() / ((F::one() + F::one()) * self.q.clone())
    }
}

/// Solve the selected KKT system exactly for one sigma.
///
/// Caller contract:
/// - `sigma` is an active traversal word, represented as a partial permutation
///   of facet indices into `dual_vertices`.
///
/// Mathematical non-success:
/// - returns `None` when the selected exact KKT system is inconsistent or has
///   no solution with strictly positive beta entries and strictly positive `q`.
pub fn solve_orbit_sigma_exact<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
    sigma: &[usize],
) -> Option<ExactOrbitKktData<F>> {
    assert!(is_partial_permutation(sigma, dual_vertices.len()));

    let m = sigma.len();
    let (matrix, rhs) = build_kkt_matrix(dual_vertices, sigma);

    let solution = match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => return None,
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => particular.iter().cloned().collect(),
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => choose_positive_solution(&particular, &kernel_basis, m)?,
    };

    let beta = solution[..m].to_vec();
    if !beta.iter().all(|entry| entry > &F::zero()) {
        return None;
    }

    let mu = Vector4::new(
        solution[m].clone(),
        solution[m + 1].clone(),
        solution[m + 2].clone(),
        solution[m + 3].clone(),
    );
    let xi = solution[m + 4].clone();
    let q = compute_q_exact(dual_vertices, sigma, &beta);
    if q <= F::zero() {
        return None;
    }

    Some(ExactOrbitKktData {
        sigma: sigma.to_vec(),
        beta,
        q,
        mu,
        xi,
    })
}

fn is_partial_permutation(indices: &[usize], upper_bound: usize) -> bool {
    let mut seen = vec![false; upper_bound];
    for &index in indices {
        if index >= upper_bound || seen[index] {
            return false;
        }
        seen[index] = true;
    }
    true
}

fn build_kkt_matrix<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
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

    for row in 0..m {
        matrix[(row, m + 4)] = F::one();
    }
    for col in 0..m {
        matrix[(m + 4, col)] = F::one();
    }
    rhs[m + 4] = F::one();

    (matrix, rhs)
}

fn compute_q_exact<F: ExactScalar>(dual_vertices: &[Vector4<F>], sigma: &[usize], beta: &[F]) -> F {
    let m = beta.len();
    let mut sum = F::zero();
    for i in 1..m {
        for j in 0..i {
            sum += beta[i].clone()
                * beta[j].clone()
                * omega0(&dual_vertices[sigma[j]], &dual_vertices[sigma[i]]);
        }
    }
    sum
}

fn choose_positive_solution<F: ExactScalar + 'static>(
    particular: &DVector<F>,
    null_space: &DMatrix<F>,
    beta_len: usize,
) -> Option<Vec<F>> {
    let beta0: Vec<F> = particular.iter().take(beta_len).cloned().collect();
    let null_columns: Vec<Vec<F>> = (0..null_space.ncols())
        .map(|col| {
            (0..null_space.nrows())
                .map(|row| null_space[(row, col)].clone())
                .collect()
        })
        .collect();
    let null_beta: Vec<Vec<F>> = null_columns
        .iter()
        .map(|column| column[..beta_len].to_vec())
        .collect();
    let alpha = find_positive_alpha(&beta0, &null_beta)?;

    let mut solution: Vec<F> = particular.iter().cloned().collect();
    for (col, basis) in null_columns.iter().enumerate() {
        for row in 0..solution.len() {
            solution[row] = solution[row].clone() + alpha[col].clone() * basis[row].clone();
        }
    }
    Some(solution)
}

fn find_positive_alpha<F: ExactScalar>(beta0: &[F], null_vecs: &[Vec<F>]) -> Option<Vec<F>> {
    let m = beta0.len();
    let k = null_vecs.len();

    type Constraint<F> = (Vec<F>, F);
    let mut constraints: Vec<Constraint<F>> = (0..m)
        .map(|row| {
            let coeffs = (0..k).map(|col| null_vecs[col][row].clone()).collect();
            (coeffs, -beta0[row].clone())
        })
        .collect();

    struct Bound<F: ExactScalar> {
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
            if coeff > &F::zero() {
                positive.push(constraint);
            } else if coeff < &F::zero() {
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

    let mut alpha = vec![F::zero(); k];
    let one = F::one();
    for (stage_idx, bounds) in stages.into_iter().enumerate() {
        let alpha_idx = k - 1 - stage_idx;
        let mut lower: Option<F> = None;
        let mut upper: Option<F> = None;

        for bound in bounds {
            let mut residual = bound.rhs.clone();
            for (coeff, assigned) in bound.remaining_coeffs.iter().zip(alpha[..alpha_idx].iter()) {
                residual -= coeff.clone() * assigned.clone();
            }
            let candidate = residual / bound.divisor.clone();
            if bound.divisor > F::zero() {
                lower = Some(match lower {
                    Some(cur) => cur.max(candidate),
                    None => candidate,
                });
            } else {
                upper = Some(match upper {
                    Some(cur) => cur.min(candidate),
                    None => candidate,
                });
            }
        }

        if let (Some(lo), Some(hi)) = (&lower, &upper) {
            if lo > hi {
                return None;
            }
        }

        alpha[alpha_idx] = match (lower, upper) {
            (Some(lo), Some(hi)) => {
                let two = one.clone() + one.clone();
                (lo + hi) / two
            }
            (Some(lo), None) => lo + one.clone(),
            (None, Some(hi)) => hi - one.clone(),
            (None, None) => F::zero(),
        };
    }

    let beta: Vec<F> = (0..m)
        .map(|row| {
            let mut value = beta0[row].clone();
            for col in 0..k {
                value += alpha[col].clone() * null_vecs[col][row].clone();
            }
            value
        })
        .collect();

    if beta.iter().all(|entry| entry > &F::zero()) {
        Some(alpha)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::solve_orbit_sigma_exact;
    use algebraic_numbers::{Algebraic, RealAlgebraicField};
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use num_traits::{One, Zero};

    enum TanPiFifth {}

    impl RealAlgebraicField for TanPiFifth {
        fn polynomial() -> Vec<BigRational> {
            vec![q(5), q(0), q(-10), q(0), q(1)]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (q(0), q(1))
        }
    }

    type TanPiFifthField = Algebraic<TanPiFifth>;

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    fn exact_simplex_dual_vertices() -> Vec<Vector4<BigRational>> {
        let z = BigRational::zero();
        vec![
            Vector4::new(q(-5), z.clone(), z.clone(), z.clone()),
            Vector4::new(z.clone(), q(-5), z.clone(), z.clone()),
            Vector4::new(z.clone(), z.clone(), q(-5), z.clone()),
            Vector4::new(z.clone(), z.clone(), z.clone(), q(-5)),
            Vector4::new(q(5), q(5), q(5), q(5)),
        ]
    }

    fn exact_hko_dual_vertices() -> Vec<Vector4<TanPiFifthField>> {
        let z = TanPiFifthField::zero();
        let one = TanPiFifthField::one();
        let t = TanPiFifthField::root();
        let t2 = t.clone() * t.clone();
        let t3 = t2.clone() * t.clone();
        let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from(4);
        let b = (TanPiFifthField::from(7) * t.clone() - t3.clone()) / TanPiFifthField::from(4);
        let sec36 = (TanPiFifthField::from(3) - t2.clone()) / TanPiFifthField::from(2);

        vec![
            Vector4::new(one.clone(), t.clone(), z.clone(), z.clone()),
            Vector4::new(-a.clone(), b.clone(), z.clone(), z.clone()),
            Vector4::new(-sec36.clone(), z.clone(), z.clone(), z.clone()),
            Vector4::new(-a.clone(), -b.clone(), z.clone(), z.clone()),
            Vector4::new(one.clone(), -t.clone(), z.clone(), z.clone()),
            Vector4::new(z.clone(), z.clone(), t.clone(), -one.clone()),
            Vector4::new(z.clone(), z.clone(), b.clone(), a.clone()),
            Vector4::new(z.clone(), z.clone(), z.clone(), sec36.clone()),
            Vector4::new(z.clone(), z.clone(), -b, a),
            Vector4::new(z.clone(), z.clone(), -t, -one),
        ]
    }

    #[test]
    fn simplex_sigma_solves_exactly() {
        let dual_vertices = exact_simplex_dual_vertices();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&dual_vertices, &sigma).expect("exact simplex sigma");
        assert!(orbit.beta.iter().all(|entry| entry > &BigRational::zero()));
        assert_eq!(orbit.action(), BigRational::new(1.into(), 4.into()));
    }

    #[test]
    fn hko_winning_sigma_solves_exactly() {
        let dual_vertices = exact_hko_dual_vertices();
        let sigma = [1usize, 8, 7, 3, 4, 5, 9];
        let orbit = solve_orbit_sigma_exact(&dual_vertices, &sigma).expect("exact HKO sigma");
        assert!(orbit
            .beta
            .iter()
            .all(|entry| entry > &TanPiFifthField::zero()));
        assert!(orbit.action() > TanPiFifthField::from(3));
        assert!(orbit.action() < TanPiFifthField::from(4));
    }
}
