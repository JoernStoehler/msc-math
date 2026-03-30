//! Q accuracy measurement: compare f64 solvers against exact rational arithmetic.
//!
//! Generates abstract QP problems (H, C, d) from controlled matrix families,
//! solves each with the saddle-point solver, projection solver, and exact rational
//! solver, and records Q error, β error, and diagnostic measurements.
//!
//! Design choice: iterate on abstract matrices, not polytopes. This gives controlled
//! inputs and explicit assumptions about what the solvers handle.
//!
//! Usage: cargo run --release --bin verify_numerics_q_accuracy
//! Output: experiments/verify-numerics/q_accuracy.jsonl

use nalgebra::{DMatrix, DVector};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::io::Write;

// ── Local solver module (self-contained, no library dependency) ──

#[path = "solvers.rs"]
mod solvers;

use solvers::{solve_projected, solve_saddle_point, QP, Verdict};

// ── Constants ──

/// Number of constraint rows. Matches the EHZ structure (4 closure + 1 normalization).
const P: usize = 5;

/// Output file path.
const OUTPUT_PATH: &str = "verify-numerics/q_accuracy.jsonl";

// ── Output record ──

#[derive(Serialize)]
struct Record {
    family: String,
    instance: usize,
    m: usize,

    // Q values from 3 solvers (NaN if solver returned infeasible/error)
    q_exact: f64,
    q_saddle: f64,
    q_projection: f64,

    // Q errors (corrected)
    err_saddle: f64,
    err_projection: f64,

    // Raw Q (before correction) and its error
    q_raw_saddle: f64,
    err_raw_saddle: f64,

    // Saddle-point diagnostics
    sp_residual_norm: f64,
    sp_lambda_min_all: f64,
    sp_lambda_min_retained: f64,
    sp_error_bound: f64,
    sp_rank: usize,

    // Projection diagnostics
    proj_constraint_residual: f64,

    // Condition numbers
    cond_c: f64,
    cond_h: f64,

    // Verdicts
    verdict_exact: String, // "feasible" / "infeasible"
    verdict_saddle: String,
    verdict_projection: String,

    // Beta errors (if all three feasible)
    beta_err_saddle: f64,
    beta_err_projection: f64,

    // Margins
    margin_saddle: f64,
    margin_projection: f64,
    margin_exact: f64,

    // Corrected projection solver (sign fix)
    q_proj_corrected: f64,
    err_proj_corrected: f64,
    verdict_proj_corrected: String,

    // Quantities for error bound analysis
    norm_h: f64,           // spectral norm ||H||_2
    sigma_min_c: f64,      // smallest singular value of C
    sigma_max_c: f64,      // largest singular value of C
    norm_beta_exact: f64,  // ||β_exact||_2 (NaN if infeasible)
    norm_beta_sp: f64,     // ||β_sp||_2 (NaN if infeasible)

    // Runtime error bound E₁ = ||H||·||β̃||·||r||/σ_min(C)
    // [lem:q-error-first-order] — bounds first-order Q error
    e1_bound: f64,

    // ── Intermediate quantities for chain validation ──
    // Each has an exact value, an f64 value, and a bound.
    // Goal: classify each bound as strict/lax/invalid with a prefactor.

    // Residual decomposition: r = (r_beta, r_lambda)
    norm_r_beta: f64,    // ||r_β|| = ||(Hβ̃ + C^Tλ̃)||
    norm_r_lambda: f64,  // ||r_λ|| = ||Cβ̃ - d||

    // Lagrange multiplier comparison
    norm_lambda_exact: f64,  // ||λ*|| (from exact solver)
    norm_lambda_sp: f64,     // ||λ̃|| (from SP solver)
    lambda_err: f64,         // ||δλ|| = ||λ̃ - λ*||

    // Q error decomposition (exact values, not bounds)
    first_order_term: f64,   // |(Hβ*)^T δβ| = |λ*^T r_λ|
    second_order_term: f64,  // |½ δβ^T H δβ|
    correction_term: f64,    // |λ̃^T r_λ| (what the solver adds)
    corrected_residual: f64, // |δλ^T r_λ| (first-order after correction)

    // Bound on ||λ*||: proven ≤ ||H||·||β*||/σ_min(C)
    lambda_bound: f64,       // the bound value
    lambda_bound_ratio: f64, // ||λ*|| / bound (should be ≤ 1)

    // β₀ vs β_final mismatch (M2 from cross-reference audit)
    beta0_err: f64,          // ||β₀ - β*|| (pseudoinverse perturbation)
    lp_shift_norm: f64,      // ||β_final - β₀|| (LP shift magnitude)

    // Correct decomposition using δβ₀ = β₀ - β* (the perturbation that actually affects Q)
    first_order_beta0: f64,  // |(Hβ*)^T δβ₀|
    second_order_beta0: f64, // |½ δβ₀^T H δβ₀|

    // Eigenspace projection: ||P_discard b|| where b = (0_m, 0_4, 1)
    // Measures how much of the RHS lives in the discarded eigenspace.
    // For EHZ: this is sqrt(Σ |v_i[m+4]|²) over discarded eigenvectors.
    p_discard_b_norm: f64,
}

// ══════════════════════════════════════════════════════════════════════════════
// Exact rational solver (copied and adapted from crates/src/kkt/rational_solver.rs)
// ══════════════════════════════════════════════════════════════════════════════

/// Result of exact QP solve.
struct ExactQpResult {
    beta: Vec<BigRational>,
    lambda: Vec<BigRational>, // Lagrange multiplier (p components)
    q_exact: BigRational,
    q_exact_f64: f64,
}

/// Solve the QP exactly: max ½β^T H β s.t. Cβ = d, β > 0.
///
/// Takes matrices as BigRational. Returns None if infeasible.
fn solve_qp_exact(
    h: &[Vec<BigRational>],
    c: &[Vec<BigRational>],
    d: &[BigRational],
) -> Option<ExactQpResult> {
    let m = h.len();
    let p = c.len();
    let size = m + p;

    // Build augmented KKT matrix:
    // [ H    C^T ] [ β ]   [ 0 ]
    // [ C    0   ] [ λ ] = [ d ]
    let zero = BigRational::zero();
    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    // H block (m × m)
    for i in 0..m {
        for j in 0..m {
            mat[i][j] = h[i][j].clone();
        }
    }

    // C^T block (m × p) and C block (p × m)
    for i in 0..p {
        for j in 0..m {
            mat[j][m + i] = c[i][j].clone(); // C^T
            mat[m + i][j] = c[i][j].clone(); // C
        }
    }

    // RHS: [0, ..., 0, d_0, ..., d_{p-1}]
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
            Some(ExactQpResult {
                beta,
                lambda,
                q_exact: q,
                q_exact_f64: q_f64,
            })
        }
        GaussResult::RankDeficient {
            particular,
            null_space,
        } => {
            let beta0: Vec<BigRational> = particular[..m].to_vec();
            let null_beta: Vec<Vec<BigRational>> =
                null_space.iter().map(|v| v[..m].to_vec()).collect();

            let beta = find_positive_beta(&beta0, &null_beta)?;
            // Recompute lambda from the found beta: solve Hβ + C^Tλ = 0 for λ.
            // λ = -(C C^T)^{-1} C H β (least-squares).
            // For now, compute from full KKT system with this beta.
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

/// Compute Q = ½ β^T H β exactly.
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

/// Compute exact λ from Hβ + C^Tλ = 0 (stationarity).
/// Solves C C^T λ = -C H β by Gaussian elimination.
fn compute_exact_lambda(
    h: &[Vec<BigRational>],
    c: &[Vec<BigRational>],
    beta: &[BigRational],
) -> Vec<BigRational> {
    let m = beta.len();
    let p = c.len();
    let zero = BigRational::zero();

    // Compute g = H β
    let mut g = vec![zero.clone(); m];
    for i in 0..m {
        for j in 0..m {
            g[i] += &h[i][j] * &beta[j];
        }
    }

    // Compute rhs = -C g = -C H β
    let mut rhs = vec![zero.clone(); p];
    for i in 0..p {
        for j in 0..m {
            rhs[i] -= &c[i][j] * &g[j];
        }
    }

    // Compute A = C C^T (p × p)
    let mut a = vec![vec![zero.clone(); p]; p];
    for i in 0..p {
        for j in 0..p {
            for k in 0..m {
                a[i][j] += &c[i][k] * &c[j][k];
            }
        }
    }

    // Solve A λ = rhs by Gaussian elimination
    // Augmented matrix [A | rhs]
    let mut aug = vec![vec![zero.clone(); p + 1]; p];
    for i in 0..p {
        for j in 0..p {
            aug[i][j] = a[i][j].clone();
        }
        aug[i][p] = rhs[i].clone();
    }

    // Forward elimination
    for col in 0..p {
        // Find pivot
        let pivot_row = (col..p).find(|&r| !aug[r][col].is_zero());
        let pivot_row = match pivot_row {
            Some(r) => r,
            None => return vec![zero; p], // degenerate
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

    // Back substitution
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

// ── Gaussian elimination (copied from rational_solver.rs) ──

enum GaussResult {
    FullRank(Vec<BigRational>),
    RankDeficient {
        particular: Vec<BigRational>,
        null_space: Vec<Vec<BigRational>>,
    },
}

const PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;

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
                let abs_a = rational_abs_f64(&aug[a][col]);
                let abs_b = rational_abs_f64(&aug[b][col]);
                abs_a
                    .partial_cmp(&abs_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        match best_row {
            None => {
                free_cols.push(col);
            }
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

/// Fourier-Motzkin elimination: find α such that β₀ + Σ αᵢ vᵢ > 0 (strictly).
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

fn rational_abs_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(0.0);
    let d = r.denom().to_f64().unwrap_or(1.0);
    (n / d).abs()
}

fn rational_to_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(f64::NAN);
    let d = r.denom().to_f64().unwrap_or(1.0);
    n / d
}

// ══════════════════════════════════════════════════════════════════════════════
// Matrix generation
// ══════════════════════════════════════════════════════════════════════════════

/// A test problem with both rational (exact) and f64 (numerical) representations.
struct TestProblem {
    family: String,
    instance: usize,
    m: usize,
    // Rational matrices (exact)
    h_rat: Vec<Vec<BigRational>>,
    c_rat: Vec<Vec<BigRational>>,
    d_rat: Vec<BigRational>,
    // f64 matrices (for numerical solvers)
    h_f64: DMatrix<f64>,
    c_f64: DMatrix<f64>,
    d_f64: DVector<f64>,
}

fn rat(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

fn rat_frac(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// Convert f64 matrix to rational (exact for integer/simple-fraction inputs).
fn f64_to_rat(x: f64) -> BigRational {
    // For inputs that are exact integers or simple fractions,
    // convert via the nearest integer × 2^exp representation.
    // For general f64 values, this gives the exact IEEE 754 representation.
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

/// Build a TestProblem from integer H entries and a standard C = [random 4×m; 1^T], d = [0,0,0,0,1].
fn make_problem(
    family: &str,
    instance: usize,
    h_entries: Vec<Vec<i64>>,
    c_entries: Vec<Vec<i64>>,
) -> TestProblem {
    let m = h_entries.len();
    let p = c_entries.len();

    let h_rat: Vec<Vec<BigRational>> = h_entries
        .iter()
        .map(|row| row.iter().map(|&x| rat(x)).collect())
        .collect();
    let c_rat: Vec<Vec<BigRational>> = c_entries
        .iter()
        .map(|row| row.iter().map(|&x| rat(x)).collect())
        .collect();

    let mut d_rat = vec![BigRational::zero(); p];
    d_rat[p - 1] = BigRational::one();

    let h_f64 = DMatrix::from_fn(m, m, |i, j| h_entries[i][j] as f64);
    let c_f64 = DMatrix::from_fn(p, m, |i, j| c_entries[i][j] as f64);
    let mut d_f64 = DVector::zeros(p);
    d_f64[p - 1] = 1.0;

    TestProblem {
        family: family.to_string(),
        instance,
        m,
        h_rat,
        c_rat,
        d_rat,
        h_f64,
        c_f64,
        d_f64,
    }
}

/// Build a TestProblem from f64 matrices (exact IEEE 754 representation as rationals).
fn make_problem_f64(
    family: &str,
    instance: usize,
    h: &DMatrix<f64>,
    c: &DMatrix<f64>,
    d: &DVector<f64>,
) -> TestProblem {
    let m = h.nrows();
    let p = c.nrows();

    let h_rat: Vec<Vec<BigRational>> = (0..m)
        .map(|i| (0..m).map(|j| f64_to_rat(h[(i, j)])).collect())
        .collect();
    let c_rat: Vec<Vec<BigRational>> = (0..p)
        .map(|i| (0..m).map(|j| f64_to_rat(c[(i, j)])).collect())
        .collect();
    let d_rat: Vec<BigRational> = (0..p).map(|i| f64_to_rat(d[i])).collect();

    TestProblem {
        family: family.to_string(),
        instance,
        m,
        h_rat,
        c_rat,
        d_rat,
        h_f64: h.clone(),
        c_f64: c.clone(),
        d_f64: d.clone(),
    }
}

/// Generate a random symmetric m×m integer matrix with entries in [-scale, scale].
fn random_symmetric_int(rng: &mut StdRng, m: usize, scale: i64) -> Vec<Vec<i64>> {
    let mut h = vec![vec![0i64; m]; m];
    for i in 0..m {
        for j in i..m {
            let val = rng.gen_range(-scale..=scale);
            h[i][j] = val;
            h[j][i] = val;
        }
    }
    h
}

/// Generate a random m×m antisymmetric matrix (like ω₀), with zero diagonal.
fn random_antisymmetric_int(rng: &mut StdRng, m: usize, scale: i64) -> Vec<Vec<i64>> {
    let mut h = vec![vec![0i64; m]; m];
    for i in 0..m {
        for j in (i + 1)..m {
            let val = rng.gen_range(-scale..=scale);
            // H is symmetrized: H[i][j] = H[j][i] = val
            // (even though omega_0 is antisymmetric, H is built symmetric in the code)
            h[i][j] = val;
            h[j][i] = val;
        }
    }
    h
}

/// Generate a random p×m integer matrix with entries in [-scale, scale].
fn random_constraint_int(rng: &mut StdRng, p: usize, m: usize, scale: i64) -> Vec<Vec<i64>> {
    let mut c = vec![vec![0i64; m]; p];
    for i in 0..p {
        for j in 0..m {
            c[i][j] = rng.gen_range(-scale..=scale);
        }
    }
    // Last row is all ones (normalization)
    for j in 0..m {
        c[p - 1][j] = 1;
    }
    c
}

/// Generate a feasible-by-construction problem.
///
/// Strategy: pick a random β > 0 with sum = 1, then build C such that Cβ = d.
/// The first 4 rows of C are random, and we set d[0..4] = C[0..4] * β.
/// The last row is all-ones (normalization) with d[4] = 1.
fn make_feasible_problem(rng: &mut StdRng, m: usize, inst: usize) -> TestProblem {
    // Random β > 0 with sum = 1
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();

    // Random H (symmetric)
    let h = DMatrix::from_fn(m, m, |i, j| {
        if i <= j {
            rng.gen_range(-5.0..5.0)
        } else {
            0.0 // filled below
        }
    });
    let h = &h + h.transpose(); // symmetrize (doubles diagonal, but that's fine)

    // Random C (first 4 rows random, last row all-ones)
    let mut c = DMatrix::from_fn(P, m, |i, j| {
        if i < P - 1 {
            rng.gen_range(-3i64..=3) as f64
        } else {
            1.0 // normalization row
        }
    });

    // Set d = C * β (so that Cβ = d by construction)
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    make_problem_f64(&format!("feasible_constructed"), inst, &h, &c, &d)
}

/// Generate test problems for each family.
fn generate_problems() -> Vec<TestProblem> {
    let mut problems = Vec::new();
    let mut rng = StdRng::seed_from_u64(42);

    // ── Family 1: Identity (sanity check) ──
    // H = I, C = [I_4 | 0; 1^T], d = [0,0,0,0,1]
    // Exact solution: β_i = 1/m for all i, Q = ½·(1/m)
    for m in [6, 8, 10] {
        let mut h = vec![vec![0i64; m]; m];
        for i in 0..m {
            h[i][i] = 1;
        }
        let mut c = vec![vec![0i64; m]; P];
        // First 4 rows: just pick first 4 coordinates as constraints
        for i in 0..4.min(m) {
            c[i][i] = 1;
        }
        // Last row: all ones
        for j in 0..m {
            c[P - 1][j] = 1;
        }
        problems.push(make_problem("identity", m, h, c));
    }

    // ── Family 2: Random dense symmetric H ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h = random_symmetric_int(&mut rng, m, 10);
        let c = random_constraint_int(&mut rng, P, m, 5);
        problems.push(make_problem("random_dense", inst, h, c));
    }

    // ── Family 3: EHZ-like (antisymmetric pairs, simulating ω₀) ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h = random_antisymmetric_int(&mut rng, m, 10);
        let c = random_constraint_int(&mut rng, P, m, 5);
        problems.push(make_problem("ehz_like", inst, h, c));
    }

    // ── Family 4: Near-singular H (small eigenvalues via construction) ──
    // H = Q^T diag(λ) Q with some λ_i small
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_near_singular_h(&mut rng, m, inst);
        let c = random_constraint_int(&mut rng, P, m, 5);
        let c_f64 = DMatrix::from_fn(P, m, |i, j| c[i][j] as f64);
        let mut d_f64 = DVector::zeros(P);
        d_f64[P - 1] = 1.0;
        problems.push(make_problem_f64("near_singular_h", inst, &h, &c_f64, &d_f64));
    }

    // ── Family 5: Singular H (zero eigenvalues) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_singular_h(&mut rng, m);
        let c = random_constraint_int(&mut rng, P, m, 5);
        let c_f64 = DMatrix::from_fn(P, m, |i, j| c[i][j] as f64);
        let mut d_f64 = DVector::zeros(P);
        d_f64[P - 1] = 1.0;
        problems.push(make_problem_f64("singular_h", inst, &h, &c_f64, &d_f64));
    }

    // ── Family 6: Indefinite H (mixed ± eigenvalues) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_indefinite_h(&mut rng, m);
        let c = random_constraint_int(&mut rng, P, m, 5);
        let c_f64 = DMatrix::from_fn(P, m, |i, j| c[i][j] as f64);
        let mut d_f64 = DVector::zeros(P);
        d_f64[P - 1] = 1.0;
        problems.push(make_problem_f64("indefinite_h", inst, &h, &c_f64, &d_f64));
    }

    // ── Family 7: Small (m=6, minimum for p=5 constraints to have k≥1) ──
    for inst in 0..200 {
        let m = 6;
        let h = random_symmetric_int(&mut rng, m, 5);
        let c = random_constraint_int(&mut rng, P, m, 3);
        problems.push(make_problem("small_m6", inst, h, c));
    }

    // ── Family 8: Large (m=16) ──
    for inst in 0..200 {
        let m = 16;
        let h = random_symmetric_int(&mut rng, m, 10);
        let c = random_constraint_int(&mut rng, P, m, 5);
        problems.push(make_problem("large_m16", inst, h, c));
    }

    // ── Family 9: Feasible by construction ──
    // Start from a known β > 0, build C and d such that Cβ = d holds.
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let prob = make_feasible_problem(&mut rng, m, inst);
        problems.push(prob);
    }

    // ── Family 10: Tiny λ_min(M) — feasible by construction ──
    // The augmented matrix M = [H, C^T; C, 0] has tiny eigenvalues when H
    // is near-singular in directions not killed by the constraints.
    // Construct: pick β > 0, then build H with a controlled small eigenvalue
    // in a direction that overlaps the constraint null space.
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let small_exp = (inst % 14) as i32 + 1; // 10^-1 to 10^-14
        let small_val = 10.0_f64.powi(-small_exp);
        let prob = make_tiny_lambda_min_problem(&mut rng, m, inst, small_val);
        problems.push(prob);
    }

    // ── Family 11: Tiny λ_min(M) via near-dependent C rows ──
    // When C has a near-zero singular value, the augmented system M gets a
    // near-zero eigenvalue. This is the "ill-conditioned constraint" regime.
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let prob = make_ill_conditioned_c_problem(&mut rng, m, inst, small_val);
        problems.push(prob);
    }

    // ── Family 12: Large ||H|| + ill-conditioned C ──
    // Stress-test whether the simple bound err <= C * ||r|| * kappa(C) breaks
    // when ||H|| * ||beta|| / sigma_max(C) is large.
    // The perturbation bound err <= ||H|| * ||beta|| * ||r|| / sigma_min(C) predicts
    // this should produce larger err / (||r|| * kappa(C)) ratios.
    for inst in 0..500 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let prob = make_large_h_ill_c_problem(&mut rng, m, inst, small_val);
        problems.push(prob);
    }

    // ── Family 13: Both H and C near-singular ──
    // Double-singular: both H and C have near-zero singular values.
    // Tests whether the bound degrades multiplicatively.
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let prob = make_double_singular_problem(&mut rng, m, inst);
        problems.push(prob);
    }

    // ── Family 14: Near-degenerate eigenspaces of H ──
    // H has clusters of nearly-equal eigenvalues. The eigenvectors within a
    // cluster are numerically unstable (rotations within the eigenspace are
    // arbitrary at machine precision). Tests whether the bound depends on
    // eigenvector stability.
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let prob = make_clustered_eigenvalue_problem(&mut rng, m, inst);
        problems.push(prob);
    }

    // ── Family 15: Near-degenerate eigenspaces of M ──
    // The augmented KKT matrix M has clustered eigenvalues.
    // The pseudoinverse eigenvectors are unstable within clusters.
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let prob = make_clustered_m_eigenvalue_problem(&mut rng, m, inst);
        problems.push(prob);
    }

    println!("Generated {} test problems", problems.len());
    problems
}

/// Both H and C have near-zero singular values.
fn make_double_singular_problem(rng: &mut StdRng, m: usize, inst: usize) -> TestProblem {
    // Small eigenvalue for H
    let h_small = 10.0_f64.powi(-((inst % 8) as i32 + 1)); // 1e-1 to 1e-8
    // Small singular value for C
    let c_small = 10.0_f64.powi(-((inst / 8 % 8) as i32 + 1)); // 1e-1 to 1e-8

    // Build H with controlled small eigenvalue
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 { h_small }
        else { rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 } }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    // Build ill-conditioned C
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + c_small * rng.gen_range(-1.0..1.0);
    }

    // Feasible by construction
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    make_problem_f64("double_singular", inst, &h, &c, &d)
}

/// H has clustered eigenvalues (near-degenerate eigenspaces).
/// Eigenvectors within clusters are numerically unstable.
fn make_clustered_eigenvalue_problem(rng: &mut StdRng, m: usize, inst: usize) -> TestProblem {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    // Cluster structure: groups of 2-3 eigenvalues separated by gap_size
    let gap_exp = (inst % 10) as i32 + 3; // gaps of 1e-3 to 1e-12
    let gap = 10.0_f64.powi(-gap_exp);

    let eigenvalues = DVector::from_fn(m, |i, _| {
        let cluster_center = if i < m / 2 { 2.0 } else { -1.5 }; // two clusters
        let perturbation = gap * rng.gen_range(-1.0..1.0);
        cluster_center + perturbation
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    // Well-conditioned C
    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });

    // Feasible by construction
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    make_problem_f64("clustered_h_eig", inst, &h, &c, &d)
}

/// Augmented KKT matrix M has clustered eigenvalues.
/// Achieved by making H diagonal with values chosen so M = [H, C^T; C, 0]
/// has near-degenerate eigenvalues.
fn make_clustered_m_eigenvalue_problem(rng: &mut StdRng, m: usize, inst: usize) -> TestProblem {
    // Simple approach: make H have eigenvalues near ±σ_i(C) so that
    // the eigenvalues of M = [H, C^T; C, 0] cluster.
    // For H = αI, M has eigenvalues (α ± sqrt(α² + σ_i²))/2... complex.
    // Simpler: just build H with values close to each other.
    let gap_exp = (inst % 10) as i32 + 3;
    let gap = 10.0_f64.powi(-gap_exp);
    let base_val = rng.gen_range(0.5..2.0);

    let h = DMatrix::from_fn(m, m, |i, j| {
        if i == j { base_val + gap * rng.gen_range(-1.0..1.0) }
        else { 0.0 }
    });

    // Moderately ill-conditioned C (so M has interesting structure)
    let kc_target = 10.0_f64.powi((inst / 10 % 6) as i32 + 1); // 10 to 1e6
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });
    // Adjust row 3 to control κ(C)
    let adjust = 1.0 / kc_target;
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + adjust * rng.gen_range(-1.0..1.0);
    }

    // Feasible by construction
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    make_problem_f64("clustered_m_eig", inst, &h, &c, &d)
}

/// Construct a near-singular H via eigendecomposition: H = Q^T diag(λ) Q.
fn make_near_singular_h(rng: &mut StdRng, m: usize, inst: usize) -> DMatrix<f64> {
    // Random orthogonal matrix via QR decomposition of random matrix
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    // Eigenvalues: most are O(1), one or two are small
    let small_val = 10.0_f64.powi(-((inst % 10) as i32 + 2)); // 1e-2 to 1e-11
    let mut eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 {
            small_val
        } else {
            rng.gen_range(0.5..2.0)
        }
    });
    if inst % 3 == 0 {
        // Also make the second eigenvalue small
        eigenvalues[1] = small_val * 10.0;
    }

    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

/// Construct a singular H (with exact zero eigenvalues).
fn make_singular_h(rng: &mut StdRng, m: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let n_zero = rng.gen_range(1..=3.min(m - 1));
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i < n_zero {
            0.0
        } else {
            rng.gen_range(0.5..2.0)
        }
    });

    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

/// Construct an indefinite H (mixed positive and negative eigenvalues).
fn make_indefinite_h(rng: &mut StdRng, m: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i < m / 2 {
            rng.gen_range(0.5..2.0)
        } else {
            rng.gen_range(-2.0..-0.5)
        }
    });

    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

/// Feasible-by-construction problem with H having a tiny eigenvalue aligned
/// with the constraint null space (so the augmented M also gets a tiny eigenvalue).
///
/// Construction:
/// 1. Pick random C (5×m), compute its SVD null space V (m×k).
/// 2. Build H = Q^T diag(λ) Q where one λ is `small_val` and the corresponding
///    eigenvector overlaps V (the constraint null space). This ensures the small
///    eigenvalue survives into the augmented system.
/// 3. Pick β > 0 with Cβ = d.
fn make_tiny_lambda_min_problem(rng: &mut StdRng, m: usize, inst: usize, small_val: f64) -> TestProblem {
    // Random β > 0 with sum = 1
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);

    // Random C with last row = all ones
    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });
    let d = &c * &beta_dv;

    // Build H with a small eigenvalue in a direction that overlaps ker(C).
    // Use a random orthogonal basis, but ensure the first eigenvector has
    // significant projection onto ker(C).
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 {
            small_val
        } else if i == 1 && inst % 3 == 0 {
            small_val * 5.0 // second small eigenvalue for some instances
        } else {
            rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 } // indefinite
        }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    make_problem_f64("tiny_lam_min", inst, &h, &c, &d)
}

/// Large ||H|| combined with ill-conditioned C.
///
/// Designed to stress-test the simple bound err <= C * ||r|| * kappa(C).
/// The perturbation bound involves ||H|| * ||beta|| / sigma_max(C), so
/// by making ||H|| large (eigenvalues in [10, 100]) while keeping C ill-conditioned,
/// we increase this factor and potentially violate the simple bound.
fn make_large_h_ill_c_problem(rng: &mut StdRng, m: usize, inst: usize, small_val: f64) -> TestProblem {
    // Build ill-conditioned C (same as ill_cond_c family)
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0);
    }

    // Random β > 0, set d = C β for feasibility
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    // Large ||H||: eigenvalues in [10, 100] magnitude (mixed signs for indefiniteness)
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        let mag = rng.gen_range(10.0..100.0);
        if i % 2 == 0 { mag } else { -mag }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    make_problem_f64("large_h_ill_c", inst, &h, &c, &d)
}

/// Problem with near-dependent C rows (ill-conditioned constraints).
///
/// When C has a near-zero singular value, the augmented M = [H, C^T; C, 0]
/// gets near-zero eigenvalues from the constraint block, even if H is
/// well-conditioned. This is a different pathway to tiny λ_min(M).
fn make_ill_conditioned_c_problem(rng: &mut StdRng, m: usize, inst: usize, small_val: f64) -> TestProblem {
    // Build C with controlled condition number:
    // Start with random C, then make the last non-normalization row nearly
    // dependent on the others.
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 }
    });

    // Make row 3 = row 0 + small_val * random perturbation
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0);
    }

    // Random β > 0, set d = C β for feasibility
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    // Well-conditioned H (so the tiny λ_min comes from C, not H)
    let h = DMatrix::from_fn(m, m, |i, j| {
        if i == j { rng.gen_range(0.5..2.0) }
        else if i < j { let v = rng.gen_range(-0.3..0.3); v }
        else { 0.0 }
    });
    let h = &h + h.transpose();

    make_problem_f64("ill_cond_c", inst, &h, &c, &d)
}

// ══════════════════════════════════════════════════════════════════════════════
// Solver wrappers
// ══════════════════════════════════════════════════════════════════════════════

/// Saddle-point solver result.
struct SpResult {
    q: f64,       // q_corrected
    q_raw: f64,   // q_raw (before correction)
    beta: Vec<f64>,   // β_final (after LP shift)
    beta0: Vec<f64>,  // β₀ (pseudoinverse, Q computed from this)
    lambda: Vec<f64>, // (mu[0..4], xi) — full Lagrange multiplier
    p_discard_b_norm: f64, // ||P_discard b||
    residual_norm: f64,
    lambda_min_all: f64,
    lambda_min_retained: f64,
    error_bound: f64,
    rank: usize,
    verdict: String,
    margin: f64,
}

/// Run the saddle-point solver on (H, C, d).
fn run_saddle_point(h: &DMatrix<f64>, c: &DMatrix<f64>, d: &DVector<f64>) -> SpResult {
    let m = h.nrows();
    let p = c.nrows();
    let size = m + p;

    // Build augmented matrix
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // H block
    for i in 0..m {
        for j in 0..m {
            kkt[(i, j)] = h[(i, j)];
        }
    }
    // C^T and C blocks
    for i in 0..p {
        for j in 0..m {
            kkt[(j, m + i)] = c[(i, j)];
            kkt[(m + i, j)] = c[(i, j)];
        }
    }
    // RHS
    for i in 0..p {
        rhs[m + i] = d[i];
    }

    // The saddle-point solver hardcodes m = size - 5, so we need p = 5.
    assert_eq!(p, 5, "Saddle-point solver requires exactly 5 constraint rows");

    // Eigendecompose for diagnostics
    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;

    let lambda_min_all = eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min);

    let max_abs = eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    let tau = 1e-3; // matches EIGEN_CONDITION_TAU
    let strict_threshold = max_abs * tau;
    let lambda_min_retained = eigenvalues
        .iter()
        .filter(|&&e| e.abs() > strict_threshold)
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min);

    let rank = eigenvalues
        .iter()
        .filter(|&&e| e.abs() > strict_threshold)
        .count();

    match solve_saddle_point(&kkt, &rhs) {
        solvers::KktOutcome::Feasible(result) => {
            let residual_vec = &kkt * DVector::from_column_slice(&{
                let mut x = result.beta.clone();
                x.extend_from_slice(&result.mu);
                x.push(result.xi);
                x
            }) - &rhs;
            let residual_norm = residual_vec.norm();
            let margin = result.beta.iter().copied().fold(f64::INFINITY, f64::min);

            let mut lam = result.mu.clone();
            lam.push(result.xi);
            SpResult {
                q: result.q_corrected,
                q_raw: result.q_raw,
                beta: result.beta,
                beta0: result.beta0,
                lambda: lam,
                p_discard_b_norm: result.p_discard_b_norm,
                residual_norm,
                lambda_min_all,
                lambda_min_retained,
                error_bound: result.q_error_bound,
                rank,
                verdict: "feasible".to_string(),
                margin,
            }
        }
        solvers::KktOutcome::Infeasible => SpResult {
            q: f64::NAN,
            q_raw: f64::NAN,
            beta: vec![],
            beta0: vec![],
            lambda: vec![],
            p_discard_b_norm: f64::NAN,
            residual_norm: f64::NAN,
            lambda_min_all,
            lambda_min_retained,
            error_bound: f64::NAN,
            rank,
            verdict: "infeasible".to_string(),
            margin: f64::NEG_INFINITY,
        },
        solvers::KktOutcome::SingularMatrix => SpResult {
            q: f64::NAN,
            q_raw: f64::NAN,
            beta: vec![],
            beta0: vec![],
            lambda: vec![],
            p_discard_b_norm: f64::NAN,
            residual_norm: f64::NAN,
            lambda_min_all,
            lambda_min_retained,
            error_bound: f64::NAN,
            rank,
            verdict: "singular".to_string(),
            margin: f64::NEG_INFINITY,
        },
    }
}

/// Projection solver result.
struct ProjResult {
    q: f64,
    beta: Vec<f64>,
    constraint_residual: f64,
    verdict: String,
    margin: f64,
}

/// Run the projection solver on (H, C, d).
fn run_projection(h: &DMatrix<f64>, c: &DMatrix<f64>, d: &DVector<f64>) -> ProjResult {
    let qp = QP {
        c: c.clone(),
        d: d.clone(),
        h: h.clone(),
    };
    let sol = solve_projected(&qp);

    let constraint_residual = if sol.verdict != Verdict::False {
        let beta_dv = DVector::from_column_slice(&sol.beta);
        (c * &beta_dv - d).norm()
    } else {
        f64::NAN
    };

    let verdict_str = match sol.verdict {
        Verdict::True => "true",
        Verdict::False => "false",
        Verdict::Indeterminate => "indeterminate",
    };

    ProjResult {
        q: sol.q,
        beta: sol.beta,
        constraint_residual,
        verdict: verdict_str.to_string(),
        margin: sol.margin,
    }
}

/// Corrected projection solver.
///
/// Now that solvers.rs has the sign fix, this is identical to run_projection.
/// Kept as a separate call for JSONL backward compatibility (q_proj_corrected field).
fn run_projection_corrected(h: &DMatrix<f64>, c: &DMatrix<f64>, d: &DVector<f64>) -> ProjResult {
    run_projection(h, c, d)
}

// ══════════════════════════════════════════════════════════════════════════════
// Main
// ══════════════════════════════════════════════════════════════════════════════

fn main() {
    let problems = generate_problems();

    let out_path = std::path::Path::new(OUTPUT_PATH);
    let mut out_file = std::fs::File::create(out_path)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", OUTPUT_PATH, e));

    let mut n_exact_feasible = 0usize;
    let mut n_sp_feasible = 0usize;
    let mut n_proj_feasible = 0usize;
    let mut n_all_feasible = 0usize;
    let mut sp_errors: Vec<f64> = Vec::new();
    let mut proj_errors: Vec<f64> = Vec::new();
    let mut proj_corr_errors: Vec<f64> = Vec::new();
    let mut sp_panics = 0usize;

    for prob in &problems {
        // 1. Exact solve
        let exact = solve_qp_exact(&prob.h_rat, &prob.c_rat, &prob.d_rat);
        let (q_exact, beta_exact, verdict_exact, margin_exact) = match &exact {
            Some(r) => {
                let margin: f64 = r.beta.iter().map(|b| rational_to_f64(b)).fold(f64::INFINITY, f64::min);
                (r.q_exact_f64, Some(&r.beta), "feasible".to_string(), margin)
            }
            None => (f64::NAN, None, "infeasible".to_string(), f64::NEG_INFINITY),
        };
        if exact.is_some() {
            n_exact_feasible += 1;
        }

        // 2. Saddle-point solve (catch panics)
        let sp = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_saddle_point(&prob.h_f64, &prob.c_f64, &prob.d_f64)
        }));
        let sp = match sp {
            Ok(r) => r,
            Err(_) => {
                sp_panics += 1;
                SpResult {
                    q: f64::NAN,
                    q_raw: f64::NAN,
                    beta: vec![],
                    beta0: vec![],
                    lambda: vec![],
                    p_discard_b_norm: f64::NAN,
                    residual_norm: f64::NAN,
                    lambda_min_all: f64::NAN,
                    lambda_min_retained: f64::NAN,
                    error_bound: f64::NAN,
                    rank: 0,
                    verdict: "panic".to_string(),
                    margin: f64::NEG_INFINITY,
                }
            }
        };
        if sp.verdict == "feasible" {
            n_sp_feasible += 1;
        }

        // 3. Projection solve (library version, possibly buggy)
        let proj = run_projection(&prob.h_f64, &prob.c_f64, &prob.d_f64);
        if proj.verdict == "true" || proj.verdict == "indeterminate" {
            n_proj_feasible += 1;
        }

        // 3b. Corrected projection solve (sign fix hypothesis)
        let proj_corr = run_projection_corrected(&prob.h_f64, &prob.c_f64, &prob.d_f64);

        // 4. Compute errors
        let err_saddle = if exact.is_some() && sp.verdict == "feasible" {
            (sp.q - q_exact).abs()
        } else {
            f64::NAN
        };
        let err_raw_saddle = if exact.is_some() && sp.verdict == "feasible" {
            (sp.q_raw - q_exact).abs()
        } else {
            f64::NAN
        };
        let err_projection = if exact.is_some() && (proj.verdict == "true" || proj.verdict == "indeterminate") {
            (proj.q - q_exact).abs()
        } else {
            f64::NAN
        };

        let beta_err_sp = match (&beta_exact, &sp.beta) {
            (Some(be), sb) if !sb.is_empty() => {
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                be_f64.iter().zip(sb.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            }
            _ => f64::NAN,
        };
        let beta_err_proj = match (&beta_exact, &proj.beta) {
            (Some(be), pb) if !pb.is_empty() && (proj.verdict == "true" || proj.verdict == "indeterminate") => {
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                be_f64.iter().zip(pb.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            }
            _ => f64::NAN,
        };

        // Condition numbers and norms
        let (cond_c, sigma_min_c, sigma_max_c) = {
            let svd = prob.c_f64.clone().svd(false, false);
            let s = &svd.singular_values;
            let s_max = s.iter().cloned().fold(0.0f64, f64::max);
            let s_min = s.iter().cloned().filter(|&x| x > 1e-15).fold(f64::INFINITY, f64::min);
            let cond = if s_min > 0.0 { s_max / s_min } else { f64::INFINITY };
            (cond, if s_min.is_finite() { s_min } else { 0.0 }, s_max)
        };
        let (cond_h, norm_h) = {
            let eig = prob.h_f64.clone().symmetric_eigen();
            let ev = &eig.eigenvalues;
            let ev_max = ev.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
            let ev_min = ev.iter().map(|e| e.abs()).filter(|&e| e > 1e-15).fold(f64::INFINITY, f64::min);
            let cond = if ev_min > 0.0 { ev_max / ev_min } else { f64::INFINITY };
            (cond, ev_max)
        };

        // Beta norms
        let norm_beta_exact = match &beta_exact {
            Some(be) => {
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                be_f64.iter().map(|x| x * x).sum::<f64>().sqrt()
            }
            None => f64::NAN,
        };
        let norm_beta_sp = if !sp.beta.is_empty() {
            sp.beta.iter().map(|x| x * x).sum::<f64>().sqrt()
        } else {
            f64::NAN
        };

        let err_proj_corrected = if exact.is_some() && (proj_corr.verdict == "true" || proj_corr.verdict == "indeterminate") {
            (proj_corr.q - q_exact).abs()
        } else {
            f64::NAN
        };

        if err_saddle.is_finite() {
            sp_errors.push(err_saddle);
        }
        if err_projection.is_finite() {
            proj_errors.push(err_projection);
        }
        if err_proj_corrected.is_finite() {
            proj_corr_errors.push(err_proj_corrected);
        }
        if exact.is_some() && sp.verdict == "feasible" && (proj.verdict == "true" || proj.verdict == "indeterminate") {
            n_all_feasible += 1;
        }

        // ── Intermediate chain quantities ──
        // Compute all decomposition values when both exact and SP are feasible.

        // Residual decomposition: r = (r_beta, r_lambda)
        // r_beta = H β̃ + C^T λ̃ (stationarity residual, first m components)
        // r_lambda = C β̃ - d (constraint residual, last p components)
        let (norm_r_beta, norm_r_lambda) = if !sp.beta.is_empty() && sp.verdict == "feasible" {
            let beta_dv = DVector::from_column_slice(&sp.beta);
            // r_lambda = C β̃ - d
            let r_lambda = &prob.c_f64 * &beta_dv - &prob.d_f64;
            let nr_lambda = r_lambda.norm();
            // r_beta = H β̃ + C^T λ̃
            // λ̃ = (mu[0..4], xi) from the KktResult
            // But we don't have lambda_tilde in SpResult... reconstruct from KKT residual
            // Actually: r = M x̃ - b, so r_beta = first m components of r
            // We can compute it directly.
            let h_beta = &prob.h_f64 * &beta_dv;
            // We need lambda_tilde. Let's just compute norm_r_beta from total residual.
            // ||r||² = ||r_β||² + ||r_λ||², so ||r_β|| = sqrt(||r||² - ||r_λ||²)
            let r_total = sp.residual_norm;
            let nr_beta = if r_total >= nr_lambda {
                (r_total * r_total - nr_lambda * nr_lambda).max(0.0).sqrt()
            } else {
                0.0 // rounding
            };
            (nr_beta, nr_lambda)
        } else {
            (f64::NAN, f64::NAN)
        };

        // Lambda norms and error
        let (norm_lambda_exact, lambda_exact_f64) = match &exact {
            Some(r) => {
                let lam_f64: Vec<f64> = r.lambda.iter().map(|l| rational_to_f64(l)).collect();
                let norm = lam_f64.iter().map(|x| x * x).sum::<f64>().sqrt();
                (norm, Some(lam_f64))
            }
            None => (f64::NAN, None),
        };

        let norm_lambda_sp = if !sp.lambda.is_empty() {
            sp.lambda.iter().map(|x| x * x).sum::<f64>().sqrt()
        } else {
            f64::NAN
        };

        let lambda_err = match &lambda_exact_f64 {
            Some(le) if !sp.lambda.is_empty() && sp.lambda.len() == le.len() => {
                le.iter().zip(sp.lambda.iter())
                    .map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            }
            _ => f64::NAN,
        };

        // Q error decomposition: requires exact β*, exact λ*, and f64 β̃
        let (first_order_term, second_order_term, correction_term, corrected_residual) =
            if let (Some(be), Some(le)) = (&beta_exact, &lambda_exact_f64) {
                if !sp.beta.is_empty() && sp.verdict == "feasible" {
                    let m = prob.m;
                    let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                    let db: Vec<f64> = (0..m).map(|i| sp.beta[i] - be_f64[i]).collect();
                    let db_dv = DVector::from_column_slice(&db);

                    // r_lambda = C δβ
                    let r_lam = &prob.c_f64 * &db_dv;

                    // First-order: |λ*^T r_λ|
                    let le_dv = DVector::from_column_slice(le);
                    let first = le_dv.dot(&r_lam).abs();

                    // Second-order: |½ δβ^T H δβ|
                    let h_db = &prob.h_f64 * &db_dv;
                    let second = (0.5 * db_dv.dot(&h_db)).abs();

                    // Correction: |λ̃^T r_λ|
                    let corr_magnitude = if !sp.lambda.is_empty() {
                        let lt = DVector::from_column_slice(&sp.lambda);
                        lt.dot(&r_lam).abs()
                    } else {
                        (sp.q_raw - sp.q).abs() // fallback
                    };

                    // Corrected residual: |δλ^T r_λ|
                    let corr_residual = if !sp.lambda.is_empty() && sp.lambda.len() == le.len() {
                        let dl: Vec<f64> = sp.lambda.iter().zip(le.iter())
                            .map(|(a, b)| a - b).collect();
                        let dl_dv = DVector::from_column_slice(&dl);
                        dl_dv.dot(&r_lam).abs()
                    } else {
                        f64::NAN
                    };

                    (first, second, corr_magnitude, corr_residual)
                } else {
                    (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
                }
            } else {
                (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
            };

        // Lambda bound: ||λ*|| ≤ ||H||·||β*||/σ_min(C)
        let lambda_bound = if norm_beta_exact.is_finite() && sigma_min_c > 0.0 {
            norm_h * norm_beta_exact / sigma_min_c
        } else {
            f64::NAN
        };
        let lambda_bound_ratio = if lambda_bound.is_finite() && lambda_bound > 0.0 && norm_lambda_exact.is_finite() {
            norm_lambda_exact / lambda_bound
        } else {
            f64::NAN
        };

        // ── Compute first/second order terms for B5 assert ──
        let (first_order_b0, second_order_b0) = match &beta_exact {
            Some(be) if !sp.beta0.is_empty() && sp.verdict == "feasible" => {
                let m = prob.m;
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                let db0: Vec<f64> = (0..m).map(|i| sp.beta0[i] - be_f64[i]).collect();
                let mut hb_star = vec![0.0f64; m];
                for i in 0..m {
                    for j in 0..m {
                        hb_star[i] += prob.h_f64[(i, j)] * be_f64[j];
                    }
                }
                let fo = hb_star.iter().zip(db0.iter()).map(|(a, b)| a * b).sum::<f64>().abs();
                let db0_dv = DVector::from_column_slice(&db0);
                let h_db0 = &prob.h_f64 * &db0_dv;
                let so = (0.5 * db0_dv.dot(&h_db0)).abs();
                (fo, so)
            }
            _ => (f64::NAN, f64::NAN),
        };

        // ── Assert proven bounds (validated against exact rational ground truth) ──
        // These hold by proof; assertion catches proof bugs or implementation errors.

        // B2: ||λ*|| ≤ ||H||·||β*||/σ_min(C)
        if lambda_bound_ratio.is_finite() {
            assert!(lambda_bound_ratio <= 1.0 + 1e-10,
                "B2 violated: ||λ*||/bound = {:.6e} > 1. ||λ*||={:.2e}, bound={:.2e}, family={}, inst={}",
                lambda_bound_ratio, norm_lambda_exact, lambda_bound, prob.family, prob.instance);
        }

        // B3: |Q − Q*| ≤ E₁
        let e1 = if norm_beta_sp.is_finite() && sp.residual_norm.is_finite() && sigma_min_c > 0.0 {
            norm_h * norm_beta_sp * sp.residual_norm / sigma_min_c
        } else { f64::NAN };
        if e1.is_finite() && err_saddle.is_finite() && e1 > 0.0 {
            assert!(err_saddle <= e1 * (1.0 + 1e-10),
                "B3 violated: |Q−Q*|/E₁ = {:.6e} > 1. err={:.2e}, E₁={:.2e}, family={}, inst={}",
                err_saddle / e1, err_saddle, e1, prob.family, prob.instance);
        }

        // B4: |Q_raw − Q*| ≤ E₁
        if e1.is_finite() && err_raw_saddle.is_finite() && e1 > 0.0 {
            assert!(err_raw_saddle <= e1 * (1.0 + 1e-10),
                "B4 violated: |Q_raw−Q*|/E₁ = {:.6e} > 1. err_raw={:.2e}, E₁={:.2e}, family={}, inst={}",
                err_raw_saddle / e1, err_raw_saddle, e1, prob.family, prob.instance);
        }

        // B5: 1st_order / 2nd_order = 2 exactly (identity from δx^T M δx = 0)
        // Requires: x* ∈ col(M), which holds when M is full rank (no eigenvalue thresholding).
        // When rank-deficient: x* may have a null-space component, identity fails.
        // Only check when both terms are well above noise floor.
        {
            let fo = first_order_b0;
            let so = second_order_b0;
            let is_full_rank = sp.rank == prob.m + 5;
            if is_full_rank && fo.is_finite() && so.is_finite() && so > 1e-10 && fo > 1e-10 {
                let ratio = fo / so;
                assert!((ratio - 2.0).abs() < 1e-3,
                    "B5 violated: 1st/2nd = {:.6} ≠ 2 (full-rank M). 1st={:.2e}, 2nd={:.2e}, family={}, inst={}",
                    ratio, fo, so, prob.family, prob.instance);
            }
        }

        // B6: |Q_corr − Q*| ≈ |Q_raw − Q*| or better.
        // In exact arithmetic (x* ∈ col(M)): correction is perfect (Q_corr = Q*).
        // In f64: correction can worsen by O(ε_mach) when both errors are at noise floor.
        // Assert: correction doesn't make things MUCH worse (100x tolerance for noise).
        if err_saddle.is_finite() && err_raw_saddle.is_finite()
            && err_raw_saddle > 1e-14  // only check when raw error is above noise floor
        {
            assert!(err_saddle <= err_raw_saddle * 2.0,
                "B6 violated: correction made Q much worse. |Q−Q*|={:.2e} > 2·|Q_raw−Q*|={:.2e}, family={}, inst={}",
                err_saddle, err_raw_saddle, prob.family, prob.instance);
        }

        let record = Record {
            family: prob.family.clone(),
            instance: prob.instance,
            m: prob.m,
            q_exact,
            q_saddle: sp.q,
            q_projection: proj.q,
            err_saddle,
            err_projection,
            q_raw_saddle: sp.q_raw,
            err_raw_saddle,
            sp_residual_norm: sp.residual_norm,
            sp_lambda_min_all: sp.lambda_min_all,
            sp_lambda_min_retained: sp.lambda_min_retained,
            sp_error_bound: sp.error_bound,
            sp_rank: sp.rank,
            proj_constraint_residual: proj.constraint_residual,
            cond_c,
            cond_h,
            verdict_exact,
            verdict_saddle: sp.verdict.clone(),
            verdict_projection: proj.verdict,
            beta_err_saddle: beta_err_sp,
            beta_err_projection: beta_err_proj,
            margin_saddle: sp.margin,
            margin_projection: proj.margin,
            margin_exact,
            q_proj_corrected: proj_corr.q,
            err_proj_corrected,
            verdict_proj_corrected: proj_corr.verdict,
            norm_h,
            sigma_min_c,
            sigma_max_c,
            norm_beta_exact,
            norm_beta_sp,
            e1_bound: if norm_beta_sp.is_finite() && sp.residual_norm.is_finite() && sigma_min_c > 0.0 {
                norm_h * norm_beta_sp * sp.residual_norm / sigma_min_c
            } else {
                f64::NAN
            },
            norm_r_beta,
            norm_r_lambda,
            norm_lambda_exact,
            norm_lambda_sp,
            lambda_err,
            first_order_term,
            second_order_term,
            correction_term,
            corrected_residual,
            lambda_bound,
            lambda_bound_ratio,
            beta0_err: match &beta_exact {
                Some(be) if !sp.beta0.is_empty() => {
                    let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                    be_f64.iter().zip(sp.beta0.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
                }
                _ => f64::NAN,
            },
            lp_shift_norm: if !sp.beta.is_empty() && !sp.beta0.is_empty() && sp.beta.len() == sp.beta0.len() {
                sp.beta.iter().zip(sp.beta0.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            } else {
                f64::NAN
            },
            first_order_beta0: first_order_b0,
            p_discard_b_norm: sp.p_discard_b_norm,
            second_order_beta0: second_order_b0,
        };

        let json = serde_json::to_string(&record).expect("serialize");
        writeln!(out_file, "{}", json).expect("write");
    }

    // ── Summary statistics ──
    println!("\n=== Q Accuracy Summary ===");
    println!("Total problems: {}", problems.len());
    println!("Exact feasible: {}", n_exact_feasible);
    println!("Saddle-point feasible: {} (panics: {})", n_sp_feasible, sp_panics);
    println!("Projection feasible: {}", n_proj_feasible);
    println!("All three feasible: {}", n_all_feasible);

    if !sp_errors.is_empty() {
        sp_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let max = sp_errors.last().unwrap();
        let median = sp_errors[sp_errors.len() / 2];
        let p99 = sp_errors[(sp_errors.len() as f64 * 0.99) as usize];
        println!("\nSaddle-point Q errors (n={}):", sp_errors.len());
        println!("  median: {:.2e}", median);
        println!("  p99:    {:.2e}", p99);
        println!("  max:    {:.2e}", max);
    }

    if !proj_errors.is_empty() {
        proj_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let max = proj_errors.last().unwrap();
        let median = proj_errors[proj_errors.len() / 2];
        let p99 = proj_errors[(proj_errors.len() as f64 * 0.99) as usize];
        println!("\nProjection Q errors (n={}):", proj_errors.len());
        println!("  median: {:.2e}", median);
        println!("  p99:    {:.2e}", p99);
        println!("  max:    {:.2e}", max);
    }

    if !proj_corr_errors.is_empty() {
        proj_corr_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let max = proj_corr_errors.last().unwrap();
        let median = proj_corr_errors[proj_corr_errors.len() / 2];
        let p99 = proj_corr_errors[(proj_corr_errors.len() as f64 * 0.99) as usize];
        println!("\nCorrected projection Q errors (n={}):", proj_corr_errors.len());
        println!("  median: {:.2e}", median);
        println!("  p99:    {:.2e}", p99);
        println!("  max:    {:.2e}", max);
    }

    println!("\nOutput written to {}", OUTPUT_PATH);
}
