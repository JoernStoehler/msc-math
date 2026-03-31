//! KKT solver accuracy measurement: compare f64 solvers against exact rational arithmetic.
//!
//! Loads QP problems (H, C, d) from two datasets:
//! - artificial.jsonl: synthetic matrix families with controlled properties (stress-tests)
//! - collected.jsonl: actual (H, C, d) from polytope σ-nodes (real input distribution)
//!
//! For each problem: runs saddle-point solver, projection solver, and exact rational
//! solver. Records Q error, β error, margin, and ~50 diagnostic fields.
//!
//! Usage: cargo run --release --bin verify_numerics
//! Output: experiments/verify-numerics/results.jsonl

use nalgebra::{DMatrix, DVector};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

// ── Local solver module (self-contained, no library dependency) ──

#[path = "solvers.rs"]
mod solvers;

use solvers::{solve_projected, solve_saddle_point, QP, Verdict};

// ── Constants ──

/// Number of constraint rows. Matches the EHZ structure (4 closure + 1 normalization).
const P: usize = 5;

/// Output file path.
const OUTPUT_PATH: &str = "verify-numerics/results.jsonl";

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

/// Row from collected.jsonl or artificial.jsonl (written by collect_inputs.rs).
/// Row from collected.jsonl or artificial.jsonl (written by collect_inputs.rs).
/// Only the fields we need for analysis. Extra JSON fields are silently ignored.
#[derive(Deserialize)]
struct InputRow {
    family: String,
    instance: usize,
    m: usize,
    #[allow(dead_code)]
    dataset: String,
    h: Vec<Vec<f64>>,
    c: Vec<Vec<f64>>,
    d: Vec<f64>,
    verdict: String,
    // q, margin, etc. may be null (NaN) for infeasible rows — use Option<f64>
    #[serde(default)]
    q: Option<f64>,
    #[serde(default)]
    margin: Option<f64>,
    #[serde(default)]
    sigma_min_c: Option<f64>,
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
// Test problem representation
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

// ══════════════════════════════════════════════════════════════════════════════
// Dataset loading
// ══════════════════════════════════════════════════════════════════════════════

const ARTIFICIAL_PATH: &str = "verify-numerics/artificial.jsonl";
const COLLECTED_PATH: &str = "verify-numerics/collected.jsonl";

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

/// Load problems from JSONL datasets (artificial + natural).
/// Natural dataset is filtered in-memory: only feasible rows are kept
/// (the exact solver is expensive, so we skip rows the f64 solver already rejects).
fn load_datasets() -> Vec<TestProblem> {
    let mut problems = Vec::new();

    // Load artificial dataset (all rows — it's small)
    if let Ok(file) = std::fs::File::open(ARTIFICIAL_PATH) {
        let reader = std::io::BufReader::new(file);
        for (idx, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|e| panic!("Read error in artificial.jsonl line {}: {}", idx, e));
            if line.trim().is_empty() { continue; }
            let row: InputRow = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("Parse error in artificial.jsonl line {}: {}", idx, e));
            problems.push(input_row_to_test_problem(&row));
        }
        println!("Loaded {} problems from {}", problems.len(), ARTIFICIAL_PATH);
    } else {
        println!("Warning: {} not found, skipping artificial dataset", ARTIFICIAL_PATH);
    }

    let n_artificial = problems.len();

    // Load natural dataset with filtering.
    //
    // Include:
    // - All feasible rows with Q > 0 (the standard case)
    // - Sample of feasible rows with Q ≤ 0 (saddle points, indefinite H on constraint set)
    // - Sample of beta_non_positive rows with σ_min(C) > 1e-6 (check for false negatives)
    // - All rows with σ_min(C) < 1e-3 and σ_min(C) > 0 (near-singular constraint regime)
    //
    // Exclude:
    // - residual_too_large (solver couldn't solve the system)
    // - σ_min(C) = 0 exactly (C is rank-deficient, exact solver can't find unique solution)
    if let Ok(file) = std::fs::File::open(COLLECTED_PATH) {
        let reader = std::io::BufReader::new(file);
        let mut n_total = 0;
        let mut n_loaded = 0;
        let mut n_q_pos = 0;
        let mut n_q_leq0 = 0;
        let mut n_beta_np = 0;
        let mut n_skipped_smin0 = 0;

        // Cap Q≤0 and beta_non_positive to avoid blowing up exact solver time.
        let max_q_leq0 = 500;
        let max_beta_np = 500;

        for (idx, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|e| panic!("Read error in collected.jsonl line {}: {}", idx, e));
            if line.trim().is_empty() { continue; }
            n_total += 1;
            let row: InputRow = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("Parse error in collected.jsonl line {}: {}", idx, e));

            // Skip residual_too_large (solver failed)
            if row.verdict == "residual_too_large" || row.verdict == "singular" || row.verdict == "panic" {
                continue;
            }

            // Skip σ_min(C) = 0 (rank-deficient C, exact solver won't work)
            if row.sigma_min_c.unwrap_or(0.0) < 1e-15 {
                n_skipped_smin0 += 1;
                continue;
            }

            let include = if row.verdict == "feasible" {
                let q = row.q.unwrap_or(f64::NAN);
                if q > 1e-15 {
                    // Q > 0: always include (the standard case)
                    n_q_pos += 1;
                    true
                } else if n_q_leq0 < max_q_leq0 {
                    // Q ≤ 0: sample (saddle points / indefinite H)
                    n_q_leq0 += 1;
                    true
                } else {
                    false
                }
            } else if row.verdict == "beta_non_positive" {
                // β has negative component: check for false negatives
                if row.sigma_min_c.unwrap_or(0.0) > 1e-6 && n_beta_np < max_beta_np {
                    n_beta_np += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if include {
                problems.push(input_row_to_test_problem(&row));
                n_loaded += 1;
            }
        }
        println!("Loaded {}/{} problems from {} (Q>0: {}, Q≤0: {}, β<0: {}, skipped σ_min=0: {})",
            n_loaded, n_total, COLLECTED_PATH, n_q_pos, n_q_leq0, n_beta_np, n_skipped_smin0);
    } else {
        println!("Warning: {} not found, skipping natural dataset", COLLECTED_PATH);
    }

    println!("Total: {} problems ({} artificial, {} natural)", problems.len(), n_artificial, problems.len() - n_artificial);
    problems
}

/// Convert an InputRow (from JSONL) to a TestProblem (for analysis).
/// Rationalizes f64 matrices to BigRational via exact IEEE 754 representation.
fn input_row_to_test_problem(row: &InputRow) -> TestProblem {
    let m = row.m;
    let p = row.c.len(); // number of constraint rows (typically 5)

    let h_f64 = DMatrix::from_fn(m, m, |i, j| row.h[i][j]);
    let c_f64 = DMatrix::from_fn(p, m, |i, j| row.c[i][j]);
    let d_f64 = DVector::from_column_slice(&row.d);

    let h_rat: Vec<Vec<BigRational>> = (0..m)
        .map(|i| (0..m).map(|j| f64_to_rat(row.h[i][j])).collect())
        .collect();
    let c_rat: Vec<Vec<BigRational>> = (0..p)
        .map(|i| (0..m).map(|j| f64_to_rat(row.c[i][j])).collect())
        .collect();
    let d_rat: Vec<BigRational> = row.d.iter().map(|&v| f64_to_rat(v)).collect();

    TestProblem {
        family: row.family.clone(),
        instance: row.instance,
        m,
        h_rat,
        c_rat,
        d_rat,
        h_f64,
        c_f64,
        d_f64,
    }
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
        ref other @ (solvers::KktOutcome::BetaNonPositive | solvers::KktOutcome::ResidualTooLarge) => SpResult {
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
            verdict: other.verdict_str().to_string(),
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
    let problems = load_datasets();

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

        // All proposition/bound checks are done in post-processing (analyze.py),
        // not here. The binary just records all quantities.

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
