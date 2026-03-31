//! Stage 1: Generate input datasets for the verify-numerics experiment.
//!
//! Two modes:
//! - `artificial`: generate 15 synthetic matrix families → artificial.jsonl
//! - `natural --polytopes <path>`: enumerate polytope σ-nodes → collected.jsonl
//!
//! Both modes run the full f64 KKT solver and save input matrices + solver output.
//! Downstream stages (run.rs) load these datasets, filter, and add exact rational analysis.
//!
//! Usage:
//!   cargo run --release --bin collect_inputs -- artificial
//!   cargo run --release --bin collect_inputs -- natural --polytopes ../correctness/correctness.jsonl

use nalgebra::{DMatrix, DVector, Vector4};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

// ── Local solver module (self-contained, no library dependency) ──

#[path = "solvers.rs"]
mod solvers;

// ── Crate imports for natural mode ──

use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::omega0;

// ── Constants ──

/// Number of constraint rows. Matches the EHZ structure (4 closure + 1 normalization).
const P: usize = 5;

/// Output file path for artificial dataset.
const ARTIFICIAL_PATH: &str = "verify-numerics/artificial.jsonl";

/// Output file path for natural (collected) dataset.
const COLLECTED_PATH: &str = "verify-numerics/collected.jsonl";

// ── Output record ──

#[derive(Serialize, Deserialize)]
struct InputRow {
    // Metadata
    family: String,
    instance: usize,
    m: usize,
    dataset: String, // "artificial" or "natural"

    // KKT input matrices (row-major)
    h: Vec<Vec<f64>>, // m×m symmetric
    c: Vec<Vec<f64>>, // p×m (p=5 for EHZ)
    d: Vec<f64>,      // p

    // f64 solver output (for downstream filtering)
    verdict: String,    // "feasible", "infeasible", "singular", "panic"
    q: f64,             // q_corrected (NaN if not feasible)
    q_raw: f64,         // q_raw before correction (NaN if not feasible)
    margin: f64,        // min(β) (NaN if not feasible)
    residual_norm: f64, // KKT residual (NaN if not feasible)
    rank: usize,        // # retained eigenvalues (0 if not feasible)
    norm_h: f64,        // spectral norm of H
    sigma_min_c: f64,   // smallest singular value of C

    // Polytope metadata (only for natural dataset)
    #[serde(skip_serializing_if = "Option::is_none")]
    polytope_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_count: Option<usize>,
}

// ══════════════════════════════════════════════════════════════════════════════
// Matrix generation helpers
// ══════════════════════════════════════════════════════════════════════════════

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

/// Generate a random symmetric m×m integer matrix with zero diagonal.
/// Despite the name "antisymmetric", the result is symmetric: entries for i<j
/// are mirrored to j>i. Zero diagonal simulates ω₀-like structure.
fn random_antisymmetric_int(rng: &mut StdRng, m: usize, scale: i64) -> Vec<Vec<i64>> {
    let mut h = vec![vec![0i64; m]; m];
    for i in 0..m {
        for j in (i + 1)..m {
            let val = rng.gen_range(-scale..=scale);
            h[i][j] = val;
            h[j][i] = val;
        }
    }
    h
}

/// Generate a random p×m integer matrix with entries in [-scale, scale].
/// Last row is all 1s (normalization constraint).
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

/// Convert integer matrices to f64 DMatrix/DVector and build an InputRow skeleton.
fn int_to_f64_matrices(
    h_int: &[Vec<i64>],
    c_int: &[Vec<i64>],
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let m = h_int.len();
    let p = c_int.len();
    let h = DMatrix::from_fn(m, m, |i, j| h_int[i][j] as f64);
    let c = DMatrix::from_fn(p, m, |i, j| c_int[i][j] as f64);
    let mut d = DVector::zeros(p);
    d[p - 1] = 1.0;
    (h, c, d)
}

/// Construct a near-singular H via eigendecomposition: H = Q diag(λ) Q^T.
/// One eigenvalue is small (10^-(inst%10+2)), optionally a second small one.
fn make_near_singular_h(rng: &mut StdRng, m: usize, inst: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let small_val = 10.0_f64.powi(-((inst % 10) as i32 + 2));
    let mut eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 {
            small_val
        } else {
            rng.gen_range(0.5..2.0)
        }
    });
    if inst % 3 == 0 {
        eigenvalues[1] = small_val * 10.0;
    }

    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

/// Construct a singular H (with 1..3 exact zero eigenvalues).
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

/// Feasible-by-construction problem: pick random β>0 with sum=1, random H, random C
/// with last row=1, d = C*β.
fn make_feasible_problem(
    rng: &mut StdRng,
    m: usize,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();

    let h = DMatrix::from_fn(m, m, |i, j| {
        if i <= j {
            rng.gen_range(-5.0..5.0)
        } else {
            0.0
        }
    });
    let h = &h + h.transpose();

    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3i64..=3) as f64
        } else {
            1.0
        }
    });

    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    (h, c, d)
}

/// Feasible-by-construction with H having a tiny eigenvalue.
fn make_tiny_lambda_min_problem(
    rng: &mut StdRng,
    m: usize,
    inst: usize,
    small_val: f64,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);

    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });
    let d = &c * &beta_dv;

    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 {
            small_val
        } else if i == 1 && inst % 3 == 0 {
            small_val * 5.0
        } else {
            rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 }
        }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    (h, c, d)
}

/// Problem with near-dependent C rows (ill-conditioned constraints).
/// Row 3 ≈ row 0, controlled by small_val.
fn make_ill_conditioned_c_problem(
    rng: &mut StdRng,
    m: usize,
    _inst: usize,
    small_val: f64,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0);
    }

    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    let h = DMatrix::from_fn(m, m, |i, j| {
        if i == j {
            rng.gen_range(0.5..2.0)
        } else if i < j {
            rng.gen_range(-0.3..0.3)
        } else {
            0.0
        }
    });
    let h = &h + h.transpose();

    (h, c, d)
}

/// Large ||H|| combined with ill-conditioned C.
/// H has eigenvalues in [10, 100] magnitude, C has near-dependent rows.
fn make_large_h_ill_c_problem(
    rng: &mut StdRng,
    m: usize,
    _inst: usize,
    small_val: f64,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0);
    }

    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        let mag = rng.gen_range(10.0..100.0);
        if i % 2 == 0 {
            mag
        } else {
            -mag
        }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    (h, c, d)
}

/// Both H and C have near-zero singular values.
fn make_double_singular_problem(
    rng: &mut StdRng,
    m: usize,
    inst: usize,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let h_small = 10.0_f64.powi(-((inst % 8) as i32 + 1));
    let c_small = 10.0_f64.powi(-((inst / 8 % 8) as i32 + 1));

    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 {
            h_small
        } else {
            rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 }
        }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + c_small * rng.gen_range(-1.0..1.0);
    }

    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    (h, c, d)
}

/// H has clustered eigenvalues (near-degenerate eigenspaces).
fn make_clustered_eigenvalue_problem(
    rng: &mut StdRng,
    m: usize,
    inst: usize,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();

    let gap_exp = (inst % 10) as i32 + 3;
    let gap = 10.0_f64.powi(-gap_exp);

    let eigenvalues = DVector::from_fn(m, |i, _| {
        let cluster_center = if i < m / 2 { 2.0 } else { -1.5 };
        let perturbation = gap * rng.gen_range(-1.0..1.0);
        cluster_center + perturbation
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();

    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });

    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    (h, c, d)
}

/// Augmented KKT matrix M has clustered eigenvalues via diagonal H with close values.
fn make_clustered_m_eigenvalue_problem(
    rng: &mut StdRng,
    m: usize,
    inst: usize,
) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let gap_exp = (inst % 10) as i32 + 3;
    let gap = 10.0_f64.powi(-gap_exp);
    let base_val = rng.gen_range(0.5..2.0);

    let h = DMatrix::from_fn(m, m, |i, j| {
        if i == j {
            base_val + gap * rng.gen_range(-1.0..1.0)
        } else {
            0.0
        }
    });

    let kc_target = 10.0_f64.powi((inst / 10 % 6) as i32 + 1);
    let mut c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 {
            rng.gen_range(-3.0..3.0)
        } else {
            1.0
        }
    });
    let adjust = 1.0 / kc_target;
    for j in 0..m {
        c[(3, j)] = c[(0, j)] + adjust * rng.gen_range(-1.0..1.0);
    }

    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;

    (h, c, d)
}

// ══════════════════════════════════════════════════════════════════════════════
// Solver interaction
// ══════════════════════════════════════════════════════════════════════════════

/// Compute spectral norm of H (max absolute eigenvalue).
fn spectral_norm(h: &DMatrix<f64>) -> f64 {
    let eig = h.clone().symmetric_eigen();
    eig.eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0_f64, f64::max)
}

/// Compute smallest singular value of C.
fn sigma_min(c: &DMatrix<f64>) -> f64 {
    let svd = c.clone().svd(false, false);
    svd.singular_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}

/// Convert a DMatrix to row-major Vec<Vec<f64>>.
fn matrix_to_vecs(m: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}

/// Run the f64 saddle-point solver on (H, C, d) and produce an InputRow.
/// Uses catch_unwind to handle solver panics.
fn solve_and_record(
    family: &str,
    instance: usize,
    dataset: &str,
    h: &DMatrix<f64>,
    c: &DMatrix<f64>,
    d: &DVector<f64>,
    polytope_id: Option<usize>,
    perm_opt: Option<Vec<usize>>,
    facet_count: Option<usize>,
) -> InputRow {
    let m = h.nrows();
    let p = c.nrows();
    let size = m + p;

    let norm_h = spectral_norm(h);
    let smin_c = sigma_min(c);

    // Build augmented KKT matrix M = [[H, C^T], [C, 0]] and rhs = [0..0, d]
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for i in 0..m {
        for j in 0..m {
            kkt[(i, j)] = h[(i, j)];
        }
    }
    for i in 0..p {
        for j in 0..m {
            kkt[(j, m + i)] = c[(i, j)];
            kkt[(m + i, j)] = c[(i, j)];
        }
    }
    for i in 0..p {
        rhs[m + i] = d[i];
    }

    // Run solver with panic catching
    let h_clone = h.clone();
    let kkt_clone = kkt.clone();
    let rhs_clone = rhs.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        solvers::solve_saddle_point(&kkt_clone, &rhs_clone)
    }));

    // Eigendecompose for rank (matches run.rs logic)
    let eig = kkt.clone().symmetric_eigen();
    let max_abs = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    let tau = 1e-3; // matches EIGEN_CONDITION_TAU in solvers.rs
    let strict_threshold = max_abs * tau;
    let rank = eig
        .eigenvalues
        .iter()
        .filter(|&&e| e.abs() > strict_threshold)
        .count();

    match result {
        Ok(solvers::KktOutcome::Feasible(kkt_result)) => {
            // Compute residual
            let mut x_vec = kkt_result.beta.clone();
            x_vec.extend_from_slice(&kkt_result.mu);
            x_vec.push(kkt_result.xi);
            let x_dv = DVector::from_column_slice(&x_vec);
            let residual_vec = &kkt * &x_dv - &rhs;
            let residual_norm = residual_vec.norm();

            let margin = kkt_result
                .beta
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);

            InputRow {
                family: family.to_string(),
                instance,
                m,
                dataset: dataset.to_string(),
                h: matrix_to_vecs(&h_clone),
                c: matrix_to_vecs(c),
                d: d.iter().copied().collect(),
                verdict: "feasible".to_string(),
                q: kkt_result.q_corrected,
                q_raw: kkt_result.q_raw,
                margin,
                residual_norm,
                rank,
                norm_h,
                sigma_min_c: smin_c,
                polytope_id,
                perm: perm_opt,
                facet_count,
            }
        }
        Ok(solvers::KktOutcome::Infeasible) => InputRow {
            family: family.to_string(),
            instance,
            m,
            dataset: dataset.to_string(),
            h: matrix_to_vecs(&h_clone),
            c: matrix_to_vecs(c),
            d: d.iter().copied().collect(),
            verdict: "infeasible".to_string(),
            q: f64::NAN,
            q_raw: f64::NAN,
            margin: f64::NAN,
            residual_norm: f64::NAN,
            rank,
            norm_h,
            sigma_min_c: smin_c,
            polytope_id,
            perm: perm_opt,
            facet_count,
        },
        Ok(solvers::KktOutcome::SingularMatrix) => InputRow {
            family: family.to_string(),
            instance,
            m,
            dataset: dataset.to_string(),
            h: matrix_to_vecs(&h_clone),
            c: matrix_to_vecs(c),
            d: d.iter().copied().collect(),
            verdict: "singular".to_string(),
            q: f64::NAN,
            q_raw: f64::NAN,
            margin: f64::NAN,
            residual_norm: f64::NAN,
            rank,
            norm_h,
            sigma_min_c: smin_c,
            polytope_id,
            perm: perm_opt,
            facet_count,
        },
        Err(_) => InputRow {
            family: family.to_string(),
            instance,
            m,
            dataset: dataset.to_string(),
            h: matrix_to_vecs(&h_clone),
            c: matrix_to_vecs(c),
            d: d.iter().copied().collect(),
            verdict: "panic".to_string(),
            q: f64::NAN,
            q_raw: f64::NAN,
            margin: f64::NAN,
            residual_norm: f64::NAN,
            rank: 0,
            norm_h,
            sigma_min_c: smin_c,
            polytope_id,
            perm: perm_opt,
            facet_count,
        },
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Artificial mode
// ══════════════════════════════════════════════════════════════════════════════

fn generate_artificial() -> Vec<InputRow> {
    let mut rows = Vec::new();
    let mut rng = StdRng::seed_from_u64(42);

    // ── Family 1: Identity (sanity check) ──
    // H = I, C = [I_4 | 0; 1^T], d = [0,0,0,0,1]
    for m in [6, 8, 10] {
        let mut h_int = vec![vec![0i64; m]; m];
        for i in 0..m {
            h_int[i][i] = 1;
        }
        let mut c_int = vec![vec![0i64; m]; P];
        for i in 0..4.min(m) {
            c_int[i][i] = 1;
        }
        for j in 0..m {
            c_int[P - 1][j] = 1;
        }
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record(
            "identity",
            m,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 2: Random dense symmetric H ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h_int = random_symmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record(
            "random_dense",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 3: EHZ-like (antisymmetric pairs, simulating ω₀) ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h_int = random_antisymmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record(
            "ehz_like",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 4: Near-singular H (small eigenvalues via construction) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_near_singular_h(&mut rng, m, inst);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P);
        d[P - 1] = 1.0;
        rows.push(solve_and_record(
            "near_singular_h",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 5: Singular H (zero eigenvalues) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_singular_h(&mut rng, m);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P);
        d[P - 1] = 1.0;
        rows.push(solve_and_record(
            "singular_h",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 6: Indefinite H (mixed ± eigenvalues) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_indefinite_h(&mut rng, m);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P);
        d[P - 1] = 1.0;
        rows.push(solve_and_record(
            "indefinite_h",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 7: Small (m=6, minimum for p=5 constraints to have k≥1) ──
    for inst in 0..200 {
        let m = 6;
        let h_int = random_symmetric_int(&mut rng, m, 5);
        let c_int = random_constraint_int(&mut rng, P, m, 3);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record(
            "small_m6",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 8: Large (m=16) ──
    for inst in 0..200 {
        let m = 16;
        let h_int = random_symmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record(
            "large_m16",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 9: Feasible by construction ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let (h, c, d) = make_feasible_problem(&mut rng, m);
        rows.push(solve_and_record(
            "feasible_constructed",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 10: Tiny λ_min — feasible by construction ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let small_exp = (inst % 14) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_tiny_lambda_min_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record(
            "tiny_lam_min",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 11: Ill-conditioned C (near-dependent rows) ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_ill_conditioned_c_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record(
            "ill_cond_c",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 12: Large ||H|| + ill-conditioned C ──
    for inst in 0..500 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_large_h_ill_c_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record(
            "large_h_ill_c",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 13: Both H and C near-singular ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_double_singular_problem(&mut rng, m, inst);
        rows.push(solve_and_record(
            "double_singular",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 14: Near-degenerate eigenspaces of H ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_clustered_eigenvalue_problem(&mut rng, m, inst);
        rows.push(solve_and_record(
            "clustered_h_eig",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    // ── Family 15: Near-degenerate eigenspaces of M ──
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_clustered_m_eigenvalue_problem(&mut rng, m, inst);
        rows.push(solve_and_record(
            "clustered_m_eig",
            inst,
            "artificial",
            &h,
            &c,
            &d,
            None,
            None,
            None,
        ));
    }

    println!("Generated {} artificial problems", rows.len());
    rows
}

// ══════════════════════════════════════════════════════════════════════════════
// Natural mode
// ══════════════════════════════════════════════════════════════════════════════

/// Minimal polytope row — only the fields we need from any polytope JSONL.
/// Uses `serde(deny_unknown_fields = false)` implicitly (default) to ignore extra fields.
#[derive(Deserialize)]
struct PolytopeInput {
    dual_vertices: Vec<[f64; 4]>,
    #[serde(default)]
    facet_count: Option<usize>,
}

fn generate_natural(polytopes_path: &str, max_facets: usize) -> Vec<InputRow> {
    let file = std::fs::File::open(polytopes_path)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", polytopes_path, e));
    let reader = std::io::BufReader::new(file);

    let mut rows = Vec::new();
    let mut instance_counter = 0usize;

    for (poly_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| panic!("Read error at line {}: {}", poly_idx, e));
        if line.trim().is_empty() {
            continue;
        }
        let poly_row: PolytopeInput = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("JSON parse error at line {}: {}", poly_idx, e));

        let f = poly_row.dual_vertices.len();
        if f > max_facets {
            println!("  Skipping polytope {} (F={} > max_facets={})", poly_idx, f, max_facets);
            continue;
        }

        let dual_verts: Vec<Vector4<f64>> = poly_row
            .dual_vertices
            .iter()
            .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
            .collect();

        // Enumerate σ-nodes: all subsets of size m, all cyclic permutations
        for m in 2..=f {
            for subset in combinations(f, m) {
                for_each_cyclic_permutation(&subset, &mut |perm| {
                    // Build H (m×m): H[i][j] = omega0(a_{σ(i)}, a_{σ(j)})
                    let mut h = DMatrix::zeros(m, m);
                    for i in 0..m {
                        for j in (i + 1)..m {
                            let val = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
                            h[(i, j)] = val;
                            h[(j, i)] = val;
                        }
                    }

                    // Build C (5×m)
                    let mut c = DMatrix::zeros(P, m);
                    for (col, &facet_idx) in perm.iter().enumerate() {
                        for d in 0..4 {
                            c[(d, col)] = dual_verts[facet_idx][d];
                        }
                        c[(4, col)] = 1.0;
                    }

                    // Build d = [0, 0, 0, 0, 1]
                    let mut d = DVector::zeros(P);
                    d[P - 1] = 1.0;

                    let row = solve_and_record(
                        "polytope_sigma_node",
                        instance_counter,
                        "natural",
                        &h,
                        &c,
                        &d,
                        Some(poly_idx),
                        Some(perm.to_vec()),
                        Some(f),
                    );
                    rows.push(row);
                    instance_counter += 1;
                });
            }
        }

        if (poly_idx + 1) % 10 == 0 {
            println!(
                "  Processed {}/{} polytopes, {} σ-nodes so far",
                poly_idx + 1,
                "?",
                instance_counter
            );
        }
    }

    println!(
        "Generated {} natural problems from polytope σ-nodes",
        rows.len()
    );
    rows
}

// ══════════════════════════════════════════════════════════════════════════════
// Write output
// ══════════════════════════════════════════════════════════════════════════════

fn write_jsonl(rows: &[InputRow], path: &str) {
    let mut file = std::fs::File::create(path)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", path, e));

    for row in rows {
        let json = serde_json::to_string(row).expect("JSON serialization failed");
        writeln!(file, "{}", json).expect("Write failed");
    }

    println!("Wrote {} rows to {}", rows.len(), path);
}

fn print_summary(rows: &[InputRow], label: &str) {
    let total = rows.len();
    let feasible = rows.iter().filter(|r| r.verdict == "feasible").count();
    let infeasible = rows.iter().filter(|r| r.verdict == "infeasible").count();
    let singular = rows.iter().filter(|r| r.verdict == "singular").count();
    let panics = rows.iter().filter(|r| r.verdict == "panic").count();

    println!("\n=== {} Summary ===", label);
    println!("  Total:      {}", total);
    println!("  Feasible:   {} ({:.1}%)", feasible, 100.0 * feasible as f64 / total as f64);
    println!("  Infeasible: {} ({:.1}%)", infeasible, 100.0 * infeasible as f64 / total as f64);
    println!("  Singular:   {} ({:.1}%)", singular, 100.0 * singular as f64 / total as f64);
    println!("  Panics:     {} ({:.1}%)", panics, 100.0 * panics as f64 / total as f64);

    // Per-family breakdown
    let mut families: Vec<String> = rows.iter().map(|r| r.family.clone()).collect();
    families.sort();
    families.dedup();
    println!("\n  Per-family breakdown:");
    for fam in &families {
        let fam_rows: Vec<&InputRow> = rows.iter().filter(|r| r.family == *fam).collect();
        let n = fam_rows.len();
        let n_feas = fam_rows.iter().filter(|r| r.verdict == "feasible").count();
        let n_panic = fam_rows.iter().filter(|r| r.verdict == "panic").count();
        println!(
            "    {:<25} {:>5} rows, {:>5} feasible, {:>3} panics",
            fam, n, n_feas, n_panic
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Main
// ══════════════════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} artificial", args[0]);
        eprintln!("  {} natural --polytopes <path>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "artificial" => {
            let rows = generate_artificial();
            write_jsonl(&rows, ARTIFICIAL_PATH);
            print_summary(&rows, "Artificial");
        }
        "natural" => {
            let mut polytopes_path = None;
            let mut max_facets = usize::MAX;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--polytopes" => {
                        i += 1;
                        polytopes_path = Some(args[i].clone());
                    }
                    "--max-facets" => {
                        i += 1;
                        max_facets = args[i].parse().expect("--max-facets must be a number");
                    }
                    other => {
                        eprintln!("Unknown argument: {}", other);
                        std::process::exit(1);
                    }
                }
                i += 1;
            }
            let polytopes_path = polytopes_path.unwrap_or_else(|| {
                eprintln!("Usage: {} natural --polytopes <path> [--max-facets N]", args[0]);
                std::process::exit(1);
            });
            let rows = generate_natural(&polytopes_path, max_facets);
            write_jsonl(&rows, COLLECTED_PATH);
            print_summary(&rows, "Natural");
        }
        other => {
            eprintln!("Unknown mode '{}'. Use 'artificial' or 'natural'.", other);
            std::process::exit(1);
        }
    }
}
