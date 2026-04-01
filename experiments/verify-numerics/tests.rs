//! Conjecture tests for the verify-numerics experiment.
//!
//! Each test loads (H, C, d) from testdata/*.jsonl, runs the f64 projection solver
//! and exact rational solver, and checks the conjecture property.
//!
//! Run: `cargo test --test verify_numerics_tests`

use nalgebra::{DMatrix, DVector};
use num_rational::BigRational;
use serde::Deserialize;
use std::io::BufRead;

#[path = "projection_solver.rs"]
mod solvers;

#[path = "exact_solver.rs"]
mod exact_solver;

use solvers::{solve_projected_with_diagnostics, QP, Verdict};
use exact_solver::{solve_qp_exact, f64_to_rat, rational_to_f64};

// ══════════════════════════════════════════════════════════════════════════════
// Test data loading
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct TestInput {
    h: Vec<Vec<f64>>,
    c: Vec<Vec<f64>>,
    d: Vec<f64>,
}

struct TestCase {
    h_f64: DMatrix<f64>,
    c_f64: DMatrix<f64>,
    d_f64: DVector<f64>,
    h_rat: Vec<Vec<BigRational>>,
    c_rat: Vec<Vec<BigRational>>,
    d_rat: Vec<BigRational>,
    m: usize,
}

fn load_testdata(filename: &str) -> Vec<TestCase> {
    let path = format!(
        "{}/verify-numerics/testdata/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename,
    );
    let file = std::fs::File::open(&path)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", path, e));
    let reader = std::io::BufReader::new(file);

    let mut cases = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| panic!("Read error line {}: {}", idx, e));
        if line.trim().is_empty() { continue; }
        let input: TestInput = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("Parse error line {}: {}", idx, e));

        let m = input.h.len();
        let p = input.c.len();

        let h_f64 = DMatrix::from_fn(m, m, |i, j| input.h[i][j]);
        let c_f64 = DMatrix::from_fn(p, m, |i, j| input.c[i][j]);
        let d_f64 = DVector::from_column_slice(&input.d);

        let h_rat: Vec<Vec<BigRational>> = (0..m)
            .map(|i| (0..m).map(|j| f64_to_rat(input.h[i][j])).collect())
            .collect();
        let c_rat: Vec<Vec<BigRational>> = (0..p)
            .map(|i| (0..m).map(|j| f64_to_rat(input.c[i][j])).collect())
            .collect();
        let d_rat: Vec<BigRational> = input.d.iter().map(|&v| f64_to_rat(v)).collect();

        cases.push(TestCase { h_f64, c_f64, d_f64, h_rat, c_rat, d_rat, m });
    }
    cases
}

// ══════════════════════════════════════════════════════════════════════════════
// Test: eigendirection error scaling [rem:eigendirection-error]
//
// Conjecture: |delta_alpha_j| ~ eps_mach / |gamma_j| for retained eigenvalues.
// We check that the ratio |delta_alpha_j| * |gamma_j| / eps_mach is O(1),
// i.e., bounded by a constant (here: m^2, the same safety factor as eta).
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn eigendirection_error_scaling() {
    let cases = load_testdata("eigendirection_scaling.jsonl");
    assert!(!cases.is_empty(), "No test cases loaded");

    let eps_mach = f64::EPSILON;
    let mut n_tested = 0;
    let mut n_violated = 0;
    let mut max_ratio = 0.0f64;
    let mut violations = Vec::new();

    for (case_idx, case) in cases.iter().enumerate() {
        let exact = match solve_qp_exact(&case.h_rat, &case.c_rat, &case.d_rat) {
            Some(r) => r,
            None => continue, // skip infeasible
        };

        let qp = QP {
            h: case.h_f64.clone(),
            c: case.c_f64.clone(),
            d: case.d_f64.clone(),
        };
        let (proj_sol, proj_diag) = solve_projected_with_diagnostics(&qp);
        let diag = match proj_diag {
            Some(d) if d.null_dim > 0 => d,
            _ => continue, // skip k=0
        };

        if proj_sol.verdict == Verdict::False { continue; }

        let be_f64: Vec<f64> = exact.beta.iter().map(|b| rational_to_f64(b)).collect();
        let k = diag.null_dim;
        let m = case.m;
        let c_safety = (m * m) as f64;

        // Compute delta_alpha = W^T V^T (beta_tilde - beta*)
        let db = DVector::from_fn(m, |i, _| proj_sol.beta[i] - be_f64[i]);
        let vt_db = diag.null_basis.transpose() * &db;
        let delta_alpha = diag.eigenvectors.transpose() * &vt_db;

        // Check each retained eigendirection
        for j in 0..k {
            let gamma_j = diag.eigenvalues[j].abs();
            if gamma_j <= diag.eps_gamma { continue; } // skip null eigenvalues

            let da_j = delta_alpha[j].abs();
            // The scaling predicts: |delta_alpha_j| ~ eps_mach / |gamma_j|
            // So |delta_alpha_j| * |gamma_j| / eps_mach should be O(1).
            let ratio = da_j * gamma_j / eps_mach;

            max_ratio = max_ratio.max(ratio);
            n_tested += 1;

            if ratio > c_safety {
                n_violated += 1;
                violations.push(format!(
                    "case {} m={} j={}: |da|={:.2e}, |gamma|={:.2e}, ratio={:.1}",
                    case_idx, m, j, da_j, gamma_j, ratio
                ));
            }
        }
    }

    println!(
        "eigendirection_error_scaling: {} eigendirections tested, {} violations, max ratio = {:.1}",
        n_tested, n_violated, max_ratio
    );
    if !violations.is_empty() {
        for v in &violations {
            println!("  VIOLATION: {}", v);
        }
    }
    assert_eq!(n_violated, 0, "{} violations (max ratio {:.1})", n_violated, max_ratio);
    assert!(n_tested >= 10, "Too few eigendirections tested: {}", n_tested);
}

// ══════════════════════════════════════════════════════════════════════════════
// Test: eta bound validity [lem:link-beta] eq:eta-computable
//
// For well-conditioned problems where the bound is finite (all retained
// eigenvalues satisfy |gamma_j| > e_delta_h_prime) AND there are no null
// eigenvalues (no LP shift): |beta_k - beta*_k| <= eta_k for all k.
//
// Cases with null eigenvalues (LP null-space search) are skipped because
// the bound covers the algebraic critical point only, not the LP shift.
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn eta_bound_validity() {
    let cases = load_testdata("eta_bound_validity.jsonl");
    assert!(!cases.is_empty(), "No test cases loaded");

    let mut n_tested = 0;
    let mut n_violated = 0;
    let mut n_infinite_eta = 0;
    let mut max_ratio = 0.0f64;
    let mut violations = Vec::new();

    for (case_idx, case) in cases.iter().enumerate() {
        let exact = match solve_qp_exact(&case.h_rat, &case.c_rat, &case.d_rat) {
            Some(r) => r,
            None => continue,
        };

        let qp = QP {
            h: case.h_f64.clone(),
            c: case.c_f64.clone(),
            d: case.d_f64.clone(),
        };
        let (proj_sol, proj_diag) = solve_projected_with_diagnostics(&qp);
        let diag = match proj_diag {
            Some(d) => d,
            None => continue,
        };

        if proj_sol.verdict == Verdict::False { continue; }

        let be_f64: Vec<f64> = exact.beta.iter().map(|b| rational_to_f64(b)).collect();
        let m = case.m;

        // Skip cases with null eigenvalues (LP null-space search shifts beta
        // by O(1), which the bound doesn't cover — see math.tex discussion).
        let has_null_eigenvalues = diag.eigenvalues.iter().any(|&g| {
            let lambda_max = diag.eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
            let threshold = if lambda_max < 1e-12 { f64::INFINITY } else { lambda_max * 1e-3 };
            g.abs() <= threshold
        });
        if has_null_eigenvalues {
            n_infinite_eta += 1; // count as skipped
            continue;
        }

        for k in 0..m {
            if k >= proj_sol.beta.len() || k >= be_f64.len() || k >= diag.eta.len() {
                break;
            }

            let actual_err = (proj_sol.beta[k] - be_f64[k]).abs();
            let eta_k = diag.eta[k];

            if eta_k.is_infinite() {
                n_infinite_eta += 1;
                continue; // trivially satisfied
            }

            n_tested += 1;
            let ratio = if eta_k > 0.0 { actual_err / eta_k } else { 0.0 };
            max_ratio = max_ratio.max(ratio);

            if actual_err > eta_k {
                n_violated += 1;
                violations.push(format!(
                    "case {} m={} k={}: |err|={:.2e} > eta={:.2e}, ratio={:.2}",
                    case_idx, m, k, actual_err, eta_k, ratio
                ));
            }
        }
    }

    println!(
        "eta_bound_validity: {} components tested, {} violations, {} infinite eta, max ratio = {:.3}",
        n_tested, n_violated, n_infinite_eta, max_ratio
    );
    if !violations.is_empty() {
        for v in &violations {
            println!("  VIOLATION: {}", v);
        }
    }
    assert_eq!(n_violated, 0, "{} violations (max ratio {:.3})", n_violated, max_ratio);
    assert!(n_tested >= 10, "Too few components tested: {}", n_tested);
}
