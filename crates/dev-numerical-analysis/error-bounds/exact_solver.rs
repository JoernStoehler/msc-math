//! Exact rational QP solver for the verify-numerics experiment.
//!
//! Solves: max (1/2) beta^T H beta s.t. C beta = d, beta > 0
//! using BigRational arithmetic (no floating-point rounding).
//!
//! Algorithm: Gaussian elimination on the augmented KKT system, then
//! Fourier-Motzkin elimination to find beta > 0 in the null space.
//!
//! Included via `#[path = "exact_solver.rs"] mod exact_solver;` in binaries.

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
    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    for i in 0..m {
        for j in 0..m {
            mat[i][j] = h[i][j].clone();
        }
    }
    for i in 0..p {
        for j in 0..m {
            mat[j][m + i] = c[i][j].clone();
            mat[m + i][j] = c[i][j].clone();
        }
    }
    for i in 0..p {
        rhs[m + i] = d[i].clone();
    }

    match gauss_solve_with_null_space(&mat, &rhs)? {
        GaussResult::FullRank(x) => {
            let beta: Vec<BigRational> = x[..m].to_vec();
            let lambda: Vec<BigRational> = x[m..].to_vec();
            if !beta.iter().all(|b| b.is_positive()) {
                return None;
            }
            let q = compute_q_exact(h, &beta);
            let q_f64 = rational_to_f64(&q);
            Some(ExactQpResult { beta, lambda, q_exact: q, q_exact_f64: q_f64 })
        }
        GaussResult::RankDeficient { particular, null_space } => {
            let beta0: Vec<BigRational> = particular[..m].to_vec();
            let null_beta: Vec<Vec<BigRational>> =
                null_space.iter().map(|v| v[..m].to_vec()).collect();
            let beta = find_positive_beta(&beta0, &null_beta)?;
            let lambda = compute_exact_lambda(h, c, &beta);
            let q = compute_q_exact(h, &beta);
            let q_f64 = rational_to_f64(&q);
            Some(ExactQpResult { beta, lambda, q_exact: q, q_exact_f64: q_f64 })
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
        r * BigRational::new(
            BigInt::from(1i64) << (exponent as usize),
            BigInt::from(1),
        )
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

    let mut a = vec![vec![zero.clone(); p]; p];
    for i in 0..p {
        for j in 0..p {
            for k in 0..m {
                a[i][j] += &c[i][k] * &c[j][k];
            }
        }
    }

    let mut aug = vec![vec![zero.clone(); p + 1]; p];
    for i in 0..p {
        for j in 0..p {
            aug[i][j] = a[i][j].clone();
        }
        aug[i][p] = rhs[i].clone();
    }

    for col in 0..p {
        let pivot_row = (col..p).find(|&r| !aug[r][col].is_zero());
        let pivot_row = match pivot_row {
            Some(r) => r,
            None => return vec![zero; p],
        };
        aug.swap(col, pivot_row);
        let pivot = aug[col][col].clone();
        for row in (col + 1)..p {
            let factor = &aug[row][col] / &pivot;
            for j in col..=p {
                let val = &aug[col][j] * &factor;
                aug[row][j] -= val;
            }
        }
    }

    let mut lambda = vec![zero.clone(); p];
    for col in (0..p).rev() {
        let mut val = aug[col][p].clone();
        for j in (col + 1)..p {
            val -= &aug[col][j] * &lambda[j];
        }
        if !aug[col][col].is_zero() {
            lambda[col] = val / &aug[col][col];
        }
    }
    lambda
}

// ── Gaussian elimination ──

enum GaussResult {
    FullRank(Vec<BigRational>),
    RankDeficient {
        particular: Vec<BigRational>,
        null_space: Vec<Vec<BigRational>>,
    },
}

const PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;

fn rational_abs_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(0.0);
    let d = r.denom().to_f64().unwrap_or(1.0);
    (n / d).abs()
}

fn gauss_solve_with_null_space(
    mat: &[Vec<BigRational>],
    rhs: &[BigRational],
) -> Option<GaussResult> {
    let n = rhs.len();

    let max_entry_abs: f64 = mat
        .iter()
        .flat_map(|row| row.iter())
        .map(rational_abs_f64)
        .fold(0.0_f64, f64::max);
    let threshold = max_entry_abs * PIVOT_RELATIVE_THRESHOLD;

    let mut aug: Vec<Vec<BigRational>> = mat
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            r.push(rhs[i].clone());
            r
        })
        .collect();

    let mut pivot_positions: Vec<(usize, usize)> = Vec::new();
    let mut free_cols: Vec<usize> = Vec::new();
    let mut current_row = 0;

    for col in 0..n {
        let best_row = (current_row..n)
            .filter(|&r| !aug[r][col].is_zero())
            .max_by(|&a, &b| {
                rational_abs_f64(&aug[a][col])
                    .partial_cmp(&rational_abs_f64(&aug[b][col]))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        match best_row {
            None => { free_cols.push(col); }
            Some(best) if rational_abs_f64(&aug[best][col]) <= threshold => {
                free_cols.push(col);
            }
            Some(best) => {
                aug.swap(current_row, best);
                for row in (current_row + 1)..n {
                    if !aug[row][col].is_zero() {
                        let factor = &aug[row][col] / &aug[current_row][col];
                        #[allow(clippy::needless_range_loop)]
                        for j in col..=n {
                            let val = &aug[current_row][j] * &factor;
                            aug[row][j] -= &val;
                        }
                    }
                }
                pivot_positions.push((current_row, col));
                current_row += 1;
            }
        }
    }

    let rank = pivot_positions.len();

    for aug_row in aug.iter().take(n).skip(rank) {
        let rhs_abs = rational_abs_f64(&aug_row[n]);
        if rhs_abs > threshold.max(1e-10) {
            return None;
        }
    }

    if free_cols.is_empty() {
        let x = back_substitute(&aug, &pivot_positions, n)?;
        return Some(GaussResult::FullRank(x));
    }

    let x_particular = back_substitute(&aug, &pivot_positions, n)?;

    let null_space: Vec<Vec<BigRational>> = free_cols
        .iter()
        .map(|&free_col| {
            let mut x = vec![BigRational::zero(); n];
            x[free_col] = BigRational::one();
            for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
                let mut sum = BigRational::zero();
                for j in (pivot_col + 1)..n {
                    sum += &aug[pivot_row][j] * &x[j];
                }
                x[pivot_col] = -sum / &aug[pivot_row][pivot_col];
            }
            x
        })
        .collect();

    Some(GaussResult::RankDeficient {
        particular: x_particular,
        null_space,
    })
}

fn back_substitute(
    aug: &[Vec<BigRational>],
    pivot_positions: &[(usize, usize)],
    n: usize,
) -> Option<Vec<BigRational>> {
    let mut x = vec![BigRational::zero(); n];
    for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
        let mut sum = aug[pivot_row][n].clone();
        for j in (pivot_col + 1)..n {
            sum -= &aug[pivot_row][j] * &x[j];
        }
        if aug[pivot_row][pivot_col].is_zero() {
            return None;
        }
        x[pivot_col] = sum / &aug[pivot_row][pivot_col];
    }
    Some(x)
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
                    if i == elim_idx { continue; }
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
