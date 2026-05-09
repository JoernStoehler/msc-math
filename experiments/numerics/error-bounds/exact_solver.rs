//! Exact rational QP solver for the verify-numerics experiment.
//!
//! Solves: max (1/2) beta^T H beta s.t. C beta = d, beta > 0
//! using BigRational arithmetic (no floating-point rounding).
//!
//! Algorithm: assemble the augmented KKT system, solve it with the shared exact
//! linear solver, then use Fourier-Motzkin elimination to find beta > 0 in the
//! null space.
//!
//! Included via `#[path = "exact_solver.rs"] mod exact_solver;` in binaries.

use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use nalgebra::{DMatrix, DVector};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

// ══════════════════════════════════════════════════════════════════════════════
// Types
// ══════════════════════════════════════════════════════════════════════════════

/// Result of exact QP solve.
pub struct ExactQpResult {
    pub beta: Vec<BigRational>,
    pub lambda: Vec<BigRational>,
    pub q_exact: BigRational,
    pub q_exact_f64: f64,
}

// ══════════════════════════════════════════════════════════════════════════════
// Public API
// ══════════════════════════════════════════════════════════════════════════════

/// Solve the QP exactly: max (1/2) beta^T H beta s.t. C beta = d, beta > 0.
///
/// Takes matrices as BigRational. Returns None if infeasible.
pub fn solve_qp_exact(
    h: &[Vec<BigRational>],
    c: &[Vec<BigRational>],
    d: &[BigRational],
) -> Option<ExactQpResult> {
    let m = h.len();
    let p = c.len();
    let size = m + p;

    let zero = BigRational::zero();
    let mut mat = DMatrix::from_element(size, size, zero.clone());
    let mut rhs = DVector::from_element(size, zero);

    for i in 0..m {
        for j in 0..m {
            mat[(i, j)] = h[i][j].clone();
        }
    }
    for i in 0..p {
        for j in 0..m {
            mat[(j, m + i)] = c[i][j].clone();
            mat[(m + i, j)] = c[i][j].clone();
        }
    }
    for i in 0..p {
        rhs[m + i] = d[i].clone();
    }

    match solve_linear_system(&mat, &rhs) {
        LinearSystemSolution::Inconsistent => None,
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => {
            let x: Vec<BigRational> = particular.iter().cloned().collect();
            let beta: Vec<BigRational> = x[..m].to_vec();
            let lambda: Vec<BigRational> = x[m..].to_vec();
            if !beta.iter().all(|b| b.is_positive()) {
                return None;
            }
            let q = compute_q_exact(h, &beta);
            let q_f64 = rational_to_f64(&q);
            Some(ExactQpResult {
                beta,
                lambda,
                q_exact: q,
                q_exact_f64: q_f64,
            })
        }
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            let beta0: Vec<BigRational> = particular.iter().take(m).cloned().collect();
            let null_beta: Vec<Vec<BigRational>> = (0..kernel_basis.ncols())
                .map(|col| (0..m).map(|row| kernel_basis[(row, col)].clone()).collect())
                .collect();
            let beta = find_positive_beta(&beta0, &null_beta)?;
            let lambda = compute_exact_lambda(h, c, &beta);
            let q = compute_q_exact(h, &beta);
            let q_f64 = rational_to_f64(&q);
            Some(ExactQpResult {
                beta,
                lambda,
                q_exact: q,
                q_exact_f64: q_f64,
            })
        }
    }
}

/// Convert f64 to exact BigRational via IEEE 754 representation.
pub fn f64_to_rat(x: f64) -> BigRational {
    if x == 0.0 {
        return BigRational::zero();
    }
    let bits = x.to_bits();
    let sign = if bits >> 63 == 0 { 1i64 } else { -1i64 };
    let exponent = ((bits >> 52) & 0x7FF) as i64 - 1023 - 52;
    let mantissa = if (bits >> 52) & 0x7FF == 0 {
        (bits & 0xFFFFFFFFFFFFF) as i64
    } else {
        ((bits & 0xFFFFFFFFFFFFF) | 0x10000000000000) as i64
    };
    let r = BigRational::new(BigInt::from(sign * mantissa), BigInt::from(1));
    if exponent >= 0 {
        r * BigRational::new(BigInt::from(1i64) << (exponent as usize), BigInt::from(1))
    } else {
        r / BigRational::new(
            BigInt::from(1i64) << ((-exponent) as usize),
            BigInt::from(1),
        )
    }
}

pub fn rational_to_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(f64::NAN);
    let d = r.denom().to_f64().unwrap_or(1.0);
    n / d
}

// ══════════════════════════════════════════════════════════════════════════════
// Internal helpers
// ══════════════════════════════════════════════════════════════════════════════

fn compute_q_exact(h: &[Vec<BigRational>], beta: &[BigRational]) -> BigRational {
    let m = beta.len();
    let two = BigRational::from(BigInt::from(2));
    let mut sum = BigRational::zero();
    for i in 0..m {
        for j in 0..m {
            sum += &beta[i] * &beta[j] * &h[i][j];
        }
    }
    sum / two
}

fn compute_exact_lambda(
    h: &[Vec<BigRational>],
    c: &[Vec<BigRational>],
    beta: &[BigRational],
) -> Vec<BigRational> {
    let m = beta.len();
    let p = c.len();
    let zero = BigRational::zero();

    let mut g = vec![zero.clone(); m];
    for i in 0..m {
        for j in 0..m {
            g[i] += &h[i][j] * &beta[j];
        }
    }

    let mut rhs = vec![zero.clone(); p];
    for i in 0..p {
        for j in 0..m {
            rhs[i] -= &c[i][j] * &g[j];
        }
    }

    let mut a = DMatrix::from_element(p, p, zero.clone());
    for i in 0..p {
        for j in 0..p {
            for k in 0..m {
                a[(i, j)] += &c[i][k] * &c[j][k];
            }
        }
    }

    let rhs = DVector::from_vec(rhs);
    if let LinearSystemSolution::Consistent {
        particular,
        kernel_basis,
    } = solve_linear_system(&a, &rhs)
    {
        if kernel_basis.ncols() == 0 {
            return particular.iter().cloned().collect();
        }
    }

    vec![zero; p]
}

fn find_positive_beta(
    beta0: &[BigRational],
    null_vecs: &[Vec<BigRational>],
) -> Option<Vec<BigRational>> {
    let m = beta0.len();
    let k = null_vecs.len();

    type Constraint = (Vec<BigRational>, BigRational);

    let mut constraints: Vec<Constraint> = (0..m)
        .map(|j| {
            let coeffs: Vec<BigRational> = (0..k).map(|i| null_vecs[i][j].clone()).collect();
            (coeffs, -&beta0[j])
        })
        .collect();

    struct Bound {
        remaining_coeffs: Vec<BigRational>,
        rhs: BigRational,
        divisor: BigRational,
    }

    let mut stages: Vec<Vec<Bound>> = Vec::with_capacity(k);

    for elim_idx in (0..k).rev() {
        let mut bounds = Vec::new();
        let mut positive: Vec<&Constraint> = Vec::new();
        let mut negative: Vec<&Constraint> = Vec::new();
        let mut new_constraints: Vec<Constraint> = Vec::new();

        for c in &constraints {
            let coeff = &c.0[elim_idx];
            if coeff.is_positive() {
                positive.push(c);
            } else if coeff.is_negative() {
                negative.push(c);
            } else {
                let mut new_coeffs = c.0.clone();
                new_coeffs.remove(elim_idx);
                new_constraints.push((new_coeffs, c.1.clone()));
            }
        }

        for c in positive.iter().chain(negative.iter()) {
            let mut remaining = c.0.clone();
            let divisor = remaining.remove(elim_idx);
            bounds.push(Bound {
                remaining_coeffs: remaining,
                rhs: c.1.clone(),
                divisor,
            });
        }
        stages.push(bounds);

        for (c_l, r_l) in &positive {
            for (c_u, r_u) in &negative {
                let a_l = &c_l[elim_idx];
                let a_u = &c_u[elim_idx];
                let mut new_coeffs = Vec::with_capacity(c_l.len() - 1);
                for i in 0..c_l.len() {
                    if i == elim_idx {
                        continue;
                    }
                    new_coeffs.push(a_l * &c_u[i] - a_u * &c_l[i]);
                }
                let new_rhs = a_l * r_u - a_u * r_l;
                new_constraints.push((new_coeffs, new_rhs));
            }
        }

        constraints = new_constraints;
    }

    for (coeffs, rhs) in &constraints {
        assert!(coeffs.is_empty());
        if !rhs.is_negative() {
            return None;
        }
    }

    let two = BigRational::from(BigInt::from(2));
    let mut alpha = vec![BigRational::zero(); k];

    for assign_var in 0..k {
        let stage_idx = k - 1 - assign_var;
        let mut lo: Option<BigRational> = None;
        let mut hi: Option<BigRational> = None;

        for bound in &stages[stage_idx] {
            let mut numerator = bound.rhs.clone();
            for (i, c) in bound.remaining_coeffs.iter().enumerate() {
                numerator -= c * &alpha[i];
            }
            let value = &numerator / &bound.divisor;

            if bound.divisor.is_positive() {
                lo = Some(match lo {
                    Some(l) => l.max(value),
                    None => value,
                });
            } else {
                hi = Some(match hi {
                    Some(h) => h.min(value),
                    None => value,
                });
            }
        }

        alpha[assign_var] = match (&lo, &hi) {
            (Some(l), Some(h)) => {
                assert!(l < h);
                (l + h) / &two
            }
            (Some(l), None) => l + BigRational::one(),
            (None, Some(h)) => h - BigRational::one(),
            (None, None) => BigRational::zero(),
        };
    }

    let beta: Vec<BigRational> = (0..m)
        .map(|j| {
            let mut val = beta0[j].clone();
            for i in 0..k {
                val += &alpha[i] * &null_vecs[i][j];
            }
            val
        })
        .collect();

    assert!(beta.iter().all(|b| b.is_positive()));
    Some(beta)
}
