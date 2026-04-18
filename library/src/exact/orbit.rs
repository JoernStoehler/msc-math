//! Exact one-sigma KKT payload and solve.
//!
//! This is the exact-field analogue of the single-orbit building blocks behind
//! the floating-point `OrbitKktData` path.
//!
//! TODO: add [lem:kkt] to formal math for the exact KKT system assembly used
//! here.
//! TODO: add [lem:well-defined] to formal math for the rank-deficient exact
//! positivity search path used here.

use crate::exact::polytope::{omega0, ExactPolytope4D};
use real_algebraic::{cmp_field, max_field, min_field, OrderedField};

/// Exact one-sigma orbit payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactOrbitKktData<F: OrderedField> {
    pub sigma: Vec<usize>,
    pub beta: Vec<F>,
    pub q: F,
    pub mu: [F; 4],
    pub xi: F,
}

impl<F: OrderedField> ExactOrbitKktData<F> {
    pub fn action(&self) -> F {
        F::one() / (F::from_i64(2) * self.q.clone())
    }
}

/// Solve the selected KKT system exactly for one sigma.
pub fn solve_orbit_sigma_exact<F: OrderedField>(
    polytope: &ExactPolytope4D<F>,
    sigma: &[usize],
) -> Option<ExactOrbitKktData<F>> {
    let m = sigma.len();
    let (matrix, rhs) = build_kkt_matrix(polytope.dual_vertices(), sigma);

    let solution = match gauss_solve_with_null_space(&matrix, &rhs)? {
        GaussResult::FullRank(solution) => solution,
        GaussResult::RankDeficient {
            particular,
            null_space,
        } => choose_positive_solution(&particular, &null_space, m)?,
    };

    let beta = solution[..m].to_vec();
    if !beta.iter().all(OrderedField::is_positive) {
        return None;
    }

    let mu = std::array::from_fn(|idx| solution[m + idx].clone());
    let xi = solution[m + 4].clone();
    let q = compute_q_exact(polytope.dual_vertices(), sigma, &beta);

    Some(ExactOrbitKktData {
        sigma: sigma.to_vec(),
        beta,
        q,
        mu,
        xi,
    })
}

fn build_kkt_matrix<F: OrderedField>(
    dual_vertices: &[[F; 4]],
    sigma: &[usize],
) -> (Vec<Vec<F>>, Vec<F>) {
    let m = sigma.len();
    let size = m + 5;
    let mut matrix = vec![vec![F::zero(); size]; size];
    let mut rhs = vec![F::zero(); size];

    for i in 0..m {
        for j in (i + 1)..m {
            let omega = omega0(&dual_vertices[sigma[i]], &dual_vertices[sigma[j]]);
            matrix[i][j] = omega.clone();
            matrix[j][i] = omega;
        }
    }

    for i in 0..m {
        for dim in 0..4 {
            let value = dual_vertices[sigma[i]][dim].clone();
            matrix[i][m + dim] = value.clone();
            matrix[m + dim][i] = value;
        }
    }

    for i in 0..m {
        matrix[i][m + 4] = F::one();
        matrix[m + 4][i] = F::one();
    }
    rhs[m + 4] = F::one();

    (matrix, rhs)
}

fn compute_q_exact<F: OrderedField>(
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

enum GaussResult<F: OrderedField> {
    FullRank(Vec<F>),
    RankDeficient {
        particular: Vec<F>,
        null_space: Vec<Vec<F>>,
    },
}

fn gauss_solve_with_null_space<F: OrderedField>(
    matrix: &[Vec<F>],
    rhs: &[F],
) -> Option<GaussResult<F>> {
    let n = rhs.len();
    let mut aug: Vec<Vec<F>> = matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let mut line = row.clone();
            line.push(rhs[row_idx].clone());
            line
        })
        .collect();

    let mut pivot_positions = Vec::new();
    let mut free_cols = Vec::new();
    let mut current_row = 0usize;

    for col in 0..n {
        match (current_row..n).find(|&row| !aug[row][col].is_zero()) {
            Some(pivot_row) => {
                aug.swap(current_row, pivot_row);
                let pivot = aug[current_row][col].clone();
                for row in (current_row + 1)..n {
                    if aug[row][col].is_zero() {
                        continue;
                    }
                    let factor = aug[row][col].clone() / pivot.clone();
                    for j in col..=n {
                        let correction = aug[current_row][j].clone() * factor.clone();
                        aug[row][j] = aug[row][j].clone() - correction;
                    }
                }
                pivot_positions.push((current_row, col));
                current_row += 1;
            }
            None => free_cols.push(col),
        }
    }

    let rank = pivot_positions.len();
    for row in rank..n {
        if !aug[row][n].is_zero() {
            return None;
        }
    }

    if free_cols.is_empty() {
        return Some(GaussResult::FullRank(back_substitute(&aug, &pivot_positions, n)?));
    }

    let particular = back_substitute(&aug, &pivot_positions, n)?;
    let null_space: Vec<Vec<F>> = free_cols
        .iter()
        .map(|&free_col| {
            let mut vector = vec![F::zero(); n];
            vector[free_col] = F::one();
            for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
                let mut sum = F::zero();
                for j in (pivot_col + 1)..n {
                    sum = sum + aug[pivot_row][j].clone() * vector[j].clone();
                }
                vector[pivot_col] = -sum / aug[pivot_row][pivot_col].clone();
            }
            vector
        })
        .collect();

    Some(GaussResult::RankDeficient {
        particular,
        null_space,
    })
}

fn back_substitute<F: OrderedField>(
    aug: &[Vec<F>],
    pivot_positions: &[(usize, usize)],
    n: usize,
) -> Option<Vec<F>> {
    let mut solution = vec![F::zero(); n];
    for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
        if aug[pivot_row][pivot_col].is_zero() {
            return None;
        }
        let mut rhs = aug[pivot_row][n].clone();
        for j in (pivot_col + 1)..n {
            rhs = rhs - aug[pivot_row][j].clone() * solution[j].clone();
        }
        solution[pivot_col] = rhs / aug[pivot_row][pivot_col].clone();
    }
    Some(solution)
}

fn choose_positive_solution<F: OrderedField>(
    particular: &[F],
    null_space: &[Vec<F>],
    beta_len: usize,
) -> Option<Vec<F>> {
    let beta0 = particular[..beta_len].to_vec();
    let null_beta: Vec<Vec<F>> = null_space.iter().map(|vec| vec[..beta_len].to_vec()).collect();
    let alpha = find_positive_alpha(&beta0, &null_beta)?;

    let mut solution = particular.to_vec();
    for (col, basis) in null_space.iter().enumerate() {
        for row in 0..solution.len() {
            solution[row] = solution[row].clone() + alpha[col].clone() * basis[row].clone();
        }
    }
    Some(solution)
}

fn find_positive_alpha<F: OrderedField>(beta0: &[F], null_vecs: &[Vec<F>]) -> Option<Vec<F>> {
    let m = beta0.len();
    let k = null_vecs.len();

    type Constraint<F> = (Vec<F>, F);
    let mut constraints: Vec<Constraint<F>> = (0..m)
        .map(|row| {
            let coeffs = (0..k).map(|col| null_vecs[col][row].clone()).collect();
            (coeffs, -beta0[row].clone())
        })
        .collect();

    struct Bound<F: OrderedField> {
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
            if coeff.is_positive() {
                positive.push(constraint);
            } else if coeff.is_negative() {
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
                residual = residual - coeff.clone() * assigned.clone();
            }
            let candidate = residual / bound.divisor.clone();
            if bound.divisor.is_positive() {
                lower = Some(match lower {
                    Some(cur) => max_field(cur, candidate),
                    None => candidate,
                });
            } else {
                upper = Some(match upper {
                    Some(cur) => min_field(cur, candidate),
                    None => candidate,
                });
            }
        }

        if let (Some(lo), Some(hi)) = (&lower, &upper) {
            if cmp_field(lo, hi).is_gt() {
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
                value = value + alpha[col].clone() * null_vecs[col][row].clone();
            }
            value
        })
        .collect();

    if beta.iter().all(OrderedField::is_positive) {
        Some(alpha)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::solve_orbit_sigma_exact;
    use crate::exact::polytope::ExactPolytope4D;
    use real_algebraic::{Algebraic, OrderedField, Rational, TanPiFifth};

    type TanPiFifthField = Algebraic<TanPiFifth>;

    fn exact_simplex() -> ExactPolytope4D<Rational> {
        let z = Rational::from_i64(0);
        ExactPolytope4D::new(vec![
            [Rational::from_i64(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), Rational::from_i64(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), Rational::from_i64(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), Rational::from_i64(-5)],
            [
                Rational::from_i64(5),
                Rational::from_i64(5),
                Rational::from_i64(5),
                Rational::from_i64(5),
            ],
        ])
        .expect("exact simplex")
    }

    fn exact_hko() -> ExactPolytope4D<TanPiFifthField> {
        let z = TanPiFifthField::zero();
        let one = TanPiFifthField::one();
        let t = TanPiFifthField::generator();
        let t2 = t.clone() * t.clone();
        let t3 = t2.clone() * t.clone();
        let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from_i64(4);
        let b = (TanPiFifthField::from_i64(7) * t.clone() - t3.clone()) / TanPiFifthField::from_i64(4);
        let sec36 = (TanPiFifthField::from_i64(3) - t2.clone()) / TanPiFifthField::from_i64(2);

        ExactPolytope4D::new(vec![
            [one.clone(), t.clone(), z.clone(), z.clone()],
            [-a.clone(), b.clone(), z.clone(), z.clone()],
            [-sec36.clone(), z.clone(), z.clone(), z.clone()],
            [-a.clone(), -b.clone(), z.clone(), z.clone()],
            [one.clone(), -t.clone(), z.clone(), z.clone()],
            [z.clone(), z.clone(), t.clone(), -one.clone()],
            [z.clone(), z.clone(), b.clone(), a.clone()],
            [z.clone(), z.clone(), z.clone(), sec36.clone()],
            [z.clone(), z.clone(), -b, a],
            [z.clone(), z.clone(), -t, -one],
        ])
        .expect("exact HKO")
    }

    #[test]
    fn simplex_sigma_solves_exactly() {
        let polytope = exact_simplex();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&polytope, &sigma).expect("exact simplex sigma");
        assert!(orbit.beta.iter().all(OrderedField::is_positive));
        assert_eq!(orbit.action(), Rational::new(1.into(), 4.into()));
    }

    #[test]
    fn hko_winning_sigma_solves_exactly() {
        let polytope = exact_hko();
        let sigma = [1usize, 8, 7, 3, 4, 5, 9];
        let orbit = solve_orbit_sigma_exact(&polytope, &sigma).expect("exact HKO sigma");
        assert!(orbit.beta.iter().all(OrderedField::is_positive));
        let action = orbit.action().to_f64();
        assert!((action - 3.440954801177934).abs() < 1.0e-12);
    }
}
