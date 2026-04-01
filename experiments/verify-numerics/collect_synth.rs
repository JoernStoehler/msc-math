//! Stage 1 (synthetic): Generate 15 matrix families → collected_synth.jsonl.
//!
//! Deterministic (seeded RNG). Families stress-test specific failure modes:
//! ill-conditioned C, near-singular H, clustered eigenvalues, etc.
//!
//! Usage:
//!   cargo run --release --bin collect_synth

use nalgebra::{DMatrix, DVector};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[path = "collect_common.rs"]
mod common;

use common::{solve_and_record, write_jsonl, print_summary, P};

const OUTPUT_PATH: &str = "verify-numerics/collected_synth.jsonl";

// ── Matrix generation helpers ──

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

fn random_constraint_int(rng: &mut StdRng, p: usize, m: usize, scale: i64) -> Vec<Vec<i64>> {
    let mut c = vec![vec![0i64; m]; p];
    for i in 0..p {
        for j in 0..m {
            c[i][j] = rng.gen_range(-scale..=scale);
        }
    }
    for j in 0..m {
        c[p - 1][j] = 1;
    }
    c
}

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

fn make_near_singular_h(rng: &mut StdRng, m: usize, inst: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let small_val = 10.0_f64.powi(-((inst % 10) as i32 + 2));
    let mut eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 { small_val } else { rng.gen_range(0.5..2.0) }
    });
    if inst % 3 == 0 { eigenvalues[1] = small_val * 10.0; }
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

fn make_singular_h(rng: &mut StdRng, m: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let n_zero = rng.gen_range(1..=3.min(m - 1));
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i < n_zero { 0.0 } else { rng.gen_range(0.5..2.0) }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

fn make_indefinite_h(rng: &mut StdRng, m: usize) -> DMatrix<f64> {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i < m / 2 { rng.gen_range(0.5..2.0) } else { rng.gen_range(-2.0..-0.5) }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    &q * &lambda * q.transpose()
}

fn make_feasible_problem(rng: &mut StdRng, m: usize) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let h = DMatrix::from_fn(m, m, |i, j| {
        if i <= j { rng.gen_range(-5.0..5.0) } else { 0.0 }
    });
    let h = &h + h.transpose();
    let c = DMatrix::from_fn(P, m, |i, _| {
        if i < P - 1 { rng.gen_range(-3i64..=3) as f64 } else { 1.0 }
    });
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    (h, c, d)
}

fn make_tiny_lambda_min_problem(rng: &mut StdRng, m: usize, inst: usize, small_val: f64) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    let d = &c * &beta_dv;
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 { small_val }
        else if i == 1 && inst % 3 == 0 { small_val * 5.0 }
        else { rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 } }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();
    (h, c, d)
}

fn make_ill_conditioned_c_problem(rng: &mut StdRng, m: usize, _inst: usize, small_val: f64) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let mut c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    for j in 0..m { c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0); }
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    let h = DMatrix::from_fn(m, m, |i, j| { if i == j { rng.gen_range(0.5..2.0) } else if i < j { rng.gen_range(-0.3..0.3) } else { 0.0 } });
    let h = &h + h.transpose();
    (h, c, d)
}

fn make_large_h_ill_c_problem(rng: &mut StdRng, m: usize, _inst: usize, small_val: f64) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let mut c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    for j in 0..m { c[(3, j)] = c[(0, j)] + small_val * rng.gen_range(-1.0..1.0); }
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| { let mag = rng.gen_range(10.0..100.0); if i % 2 == 0 { mag } else { -mag } });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();
    (h, c, d)
}

fn make_double_singular_problem(rng: &mut StdRng, m: usize, inst: usize) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let h_small = 10.0_f64.powi(-((inst % 8) as i32 + 1));
    let c_small = 10.0_f64.powi(-((inst / 8 % 8) as i32 + 1));
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let eigenvalues = DVector::from_fn(m, |i, _| {
        if i == 0 { h_small } else { rng.gen_range(0.5..2.0) * if i % 2 == 0 { 1.0 } else { -1.0 } }
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();
    let mut c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    for j in 0..m { c[(3, j)] = c[(0, j)] + c_small * rng.gen_range(-1.0..1.0); }
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    (h, c, d)
}

fn make_clustered_eigenvalue_problem(rng: &mut StdRng, m: usize, inst: usize) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let random_mat = DMatrix::from_fn(m, m, |_, _| rng.gen_range(-1.0..1.0));
    let qr = random_mat.qr();
    let q = qr.q();
    let gap_exp = (inst % 10) as i32 + 3;
    let gap = 10.0_f64.powi(-gap_exp);
    let eigenvalues = DVector::from_fn(m, |i, _| {
        let center = if i < m / 2 { 2.0 } else { -1.5 };
        center + gap * rng.gen_range(-1.0..1.0)
    });
    let lambda = DMatrix::from_diagonal(&eigenvalues);
    let h = &q * &lambda * q.transpose();
    let c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    (h, c, d)
}

fn make_clustered_m_eigenvalue_problem(rng: &mut StdRng, m: usize, inst: usize) -> (DMatrix<f64>, DMatrix<f64>, DVector<f64>) {
    let gap_exp = (inst % 10) as i32 + 3;
    let gap = 10.0_f64.powi(-gap_exp);
    let base_val = rng.gen_range(0.5..2.0);
    let h = DMatrix::from_fn(m, m, |i, j| { if i == j { base_val + gap * rng.gen_range(-1.0..1.0) } else { 0.0 } });
    let kc_target = 10.0_f64.powi((inst / 10 % 6) as i32 + 1);
    let mut c = DMatrix::from_fn(P, m, |i, _| { if i < P - 1 { rng.gen_range(-3.0..3.0) } else { 1.0 } });
    let adjust = 1.0 / kc_target;
    for j in 0..m { c[(3, j)] = c[(0, j)] + adjust * rng.gen_range(-1.0..1.0); }
    let raw: Vec<f64> = (0..m).map(|_| rng.gen_range(0.1..1.0)).collect();
    let sum: f64 = raw.iter().sum();
    let beta: Vec<f64> = raw.iter().map(|x| x / sum).collect();
    let beta_dv = DVector::from_column_slice(&beta);
    let d = &c * &beta_dv;
    (h, c, d)
}

// ── Main ──

fn main() {
    let mut rows = Vec::new();
    let mut rng = StdRng::seed_from_u64(42);

    // Family 1: Identity
    for m in [6, 8, 10] {
        let mut h_int = vec![vec![0i64; m]; m];
        for i in 0..m { h_int[i][i] = 1; }
        let mut c_int = vec![vec![0i64; m]; P];
        for i in 0..4.min(m) { c_int[i][i] = 1; }
        for j in 0..m { c_int[P - 1][j] = 1; }
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record("identity", m, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 2: Random dense
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h_int = random_symmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record("random_dense", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 3: EHZ-like
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let h_int = random_antisymmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record("ehz_like", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 4: Near-singular H
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_near_singular_h(&mut rng, m, inst);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P); d[P - 1] = 1.0;
        rows.push(solve_and_record("near_singular_h", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 5: Singular H
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_singular_h(&mut rng, m);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P); d[P - 1] = 1.0;
        rows.push(solve_and_record("singular_h", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 6: Indefinite H
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let h = make_indefinite_h(&mut rng, m);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let c = DMatrix::from_fn(P, m, |i, j| c_int[i][j] as f64);
        let mut d = DVector::zeros(P); d[P - 1] = 1.0;
        rows.push(solve_and_record("indefinite_h", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 7: Small m=6
    for inst in 0..200 {
        let m = 6;
        let h_int = random_symmetric_int(&mut rng, m, 5);
        let c_int = random_constraint_int(&mut rng, P, m, 3);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record("small_m6", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 8: Large m=16
    for inst in 0..200 {
        let m = 16;
        let h_int = random_symmetric_int(&mut rng, m, 10);
        let c_int = random_constraint_int(&mut rng, P, m, 5);
        let (h, c, d) = int_to_f64_matrices(&h_int, &c_int);
        rows.push(solve_and_record("large_m16", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 9: Feasible by construction
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let (h, c, d) = make_feasible_problem(&mut rng, m);
        rows.push(solve_and_record("feasible_constructed", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 10: Tiny λ_min
    for inst in 0..500 {
        let m = rng.gen_range(6..=12);
        let small_exp = (inst % 14) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_tiny_lambda_min_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record("tiny_lam_min", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 11: Ill-conditioned C
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_ill_conditioned_c_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record("ill_cond_c", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 12: Large H + ill-conditioned C
    for inst in 0..500 {
        let m = rng.gen_range(6..=10);
        let small_exp = (inst % 12) as i32 + 1;
        let small_val = 10.0_f64.powi(-small_exp);
        let (h, c, d) = make_large_h_ill_c_problem(&mut rng, m, inst, small_val);
        rows.push(solve_and_record("large_h_ill_c", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 13: Both near-singular
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_double_singular_problem(&mut rng, m, inst);
        rows.push(solve_and_record("double_singular", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 14: Clustered H eigenvalues
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_clustered_eigenvalue_problem(&mut rng, m, inst);
        rows.push(solve_and_record("clustered_h_eig", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    // Family 15: Clustered M eigenvalues
    for inst in 0..200 {
        let m = rng.gen_range(6..=10);
        let (h, c, d) = make_clustered_m_eigenvalue_problem(&mut rng, m, inst);
        rows.push(solve_and_record("clustered_m_eig", inst, "synthetic", &h, &c, &d, None, None, None));
    }

    println!("Generated {} synthetic problems", rows.len());
    write_jsonl(&rows, OUTPUT_PATH);
    print_summary(&rows, "Synthetic");
}
