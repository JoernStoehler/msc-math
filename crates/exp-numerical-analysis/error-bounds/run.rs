//! Stage 3: Compare f64 projection solver against exact rational arithmetic.
//!
//! Takes a single filtered JSONL file (from stage 2), runs the f64 projection
//! solver and exact rational solver on each row. Records Q error, beta error,
//! perturbation chain diagnostics, and certification bound validation.
//!
//! Usage:
//!   cargo run --release --bin verify_numerics -- <input.jsonl> <output.jsonl>

use nalgebra::{DMatrix, DVector};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

// ── Local solver modules (self-contained, no library dependency) ──

#[path = "projection_solver.rs"]
mod solvers;

#[path = "exact_solver.rs"]
mod exact_solver;

use solvers::{solve_projected_with_diagnostics, QP, Verdict};
use exact_solver::{solve_qp_exact, f64_to_rat, rational_to_f64};

// ── Output record ──

#[derive(Serialize)]
struct Record {
    family: String,
    instance: usize,
    m: usize,

    q_exact: f64,
    q_projection: f64,
    err_projection: f64,
    proj_constraint_residual: f64,

    cond_c: f64,
    cond_h: f64,

    verdict_exact: String,
    verdict_projection: String,

    beta_err_projection: f64,

    margin_projection: f64,
    margin_exact: f64,

    norm_h: f64,
    sigma_min_c: f64,
    sigma_max_c: f64,
    norm_beta_exact: f64,

    // ── Perturbation chain diagnostics [lem:link-beta] eq:eta-computable ──

    proj_eps_gamma: f64,
    proj_eta_max: f64,
    proj_eta_ratio: f64,
    proj_beta_err_inf: f64,
    proj_certified_margin: f64,
    proj_n_reliable_eigs: usize,
    proj_n_uncertain_eigs: usize,
    proj_null_dim: usize,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    proj_delta_alpha: Vec<f64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    proj_eigenvalues: Vec<f64>,
}

/// Row from collected_poly.jsonl. Only the fields we need.
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
    #[allow(dead_code)]
    verdict: String,
    #[serde(default)]
    #[allow(dead_code)]
    q: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    margin: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    sigma_min_c: Option<f64>,
}

// ══════════════════════════════════════════════════════════════════════════════
// Test problem representation
// ══════════════════════════════════════════════════════════════════════════════

struct TestProblem {
    family: String,
    instance: usize,
    m: usize,
    h_rat: Vec<Vec<BigRational>>,
    c_rat: Vec<Vec<BigRational>>,
    d_rat: Vec<BigRational>,
    h_f64: DMatrix<f64>,
    c_f64: DMatrix<f64>,
    d_f64: DVector<f64>,
}

// ══════════════════════════════════════════════════════════════════════════════
// Dataset loading
// ══════════════════════════════════════════════════════════════════════════════

fn load_input(path: &str) -> Vec<TestProblem> {
    let file = std::fs::File::open(path)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", path, e));
    let reader = std::io::BufReader::new(file);

    let mut problems = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| panic!("Read error in {} line {}: {}", path, idx, e));
        if line.trim().is_empty() { continue; }
        let row: InputRow = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("Parse error in {} line {}: {}", path, idx, e));
        problems.push(input_row_to_test_problem(&row));
    }
    println!("Loaded {} problems from {}", problems.len(), path);
    problems
}

fn input_row_to_test_problem(row: &InputRow) -> TestProblem {
    let m = row.m;
    let p = row.c.len();

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
// Main
// ══════════════════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.jsonl> <output.jsonl>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];
    let output_path = &args[2];

    let problems = load_input(input_path);

    let out_path = std::path::Path::new(output_path);
    let mut out_file = std::fs::File::create(out_path)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", output_path, e));

    let mut n_exact_feasible = 0usize;
    let mut n_proj_feasible = 0usize;
    let mut n_both_feasible = 0usize;
    let mut proj_errors: Vec<f64> = Vec::new();

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

        // 2. Projection solve with diagnostics
        let (proj_sol, proj_diag) = {
            let qp = QP {
                c: prob.c_f64.clone(),
                d: prob.d_f64.clone(),
                h: prob.h_f64.clone(),
            };
            solve_projected_with_diagnostics(&qp)
        };

        let proj_verdict_str = match proj_sol.verdict {
            Verdict::True => "true",
            Verdict::False => "false",
            Verdict::Indeterminate => "indeterminate",
        };
        let proj_feasible = proj_sol.verdict != Verdict::False;

        let proj_constraint_residual = if proj_feasible {
            let beta_dv = DVector::from_column_slice(&proj_sol.beta);
            (&prob.c_f64 * &beta_dv - &prob.d_f64).norm()
        } else {
            f64::NAN
        };

        if proj_feasible {
            n_proj_feasible += 1;
        }

        // 3. Compute errors
        let err_projection = if exact.is_some() && proj_feasible {
            (proj_sol.q - q_exact).abs()
        } else {
            f64::NAN
        };

        let beta_err_proj = match (&beta_exact, &proj_sol.beta) {
            (Some(be), pb) if !pb.is_empty() && proj_feasible => {
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                be_f64.iter().zip(pb.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            }
            _ => f64::NAN,
        };

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

        let norm_beta_exact = match &beta_exact {
            Some(be) => {
                let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                be_f64.iter().map(|x| x * x).sum::<f64>().sqrt()
            }
            None => f64::NAN,
        };

        if err_projection.is_finite() {
            proj_errors.push(err_projection);
        }
        if exact.is_some() && proj_feasible {
            n_both_feasible += 1;
        }

        let record = Record {
            family: prob.family.clone(),
            instance: prob.instance,
            m: prob.m,
            q_exact,
            q_projection: proj_sol.q,
            err_projection,
            proj_constraint_residual,
            cond_c,
            cond_h,
            verdict_exact,
            verdict_projection: proj_verdict_str.to_string(),
            beta_err_projection: beta_err_proj,
            margin_projection: proj_sol.margin,
            margin_exact,
            norm_h,
            sigma_min_c,
            sigma_max_c,
            norm_beta_exact,
            proj_eps_gamma: proj_diag.as_ref().map_or(f64::NAN, |d| d.eps_gamma),
            proj_eta_max: proj_diag.as_ref().map_or(f64::NAN, |d| {
                d.eta.iter().cloned().fold(0.0f64, f64::max)
            }),
            proj_eta_ratio: {
                match (&beta_exact, &proj_diag) {
                    (Some(be), Some(diag)) if proj_feasible => {
                        let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                        let mut max_ratio = 0.0f64;
                        for k_idx in 0..prob.m.min(be_f64.len()).min(diag.eta.len()).min(proj_sol.beta.len()) {
                            let actual_err = (proj_sol.beta[k_idx] - be_f64[k_idx]).abs();
                            if diag.eta[k_idx] > 0.0 && diag.eta[k_idx].is_finite() {
                                max_ratio = max_ratio.max(actual_err / diag.eta[k_idx]);
                            }
                        }
                        max_ratio
                    }
                    _ => f64::NAN,
                }
            },
            proj_beta_err_inf: match (&beta_exact, &proj_sol.beta) {
                (Some(be), pb) if !pb.is_empty() && proj_feasible => {
                    let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                    be_f64.iter().zip(pb.iter())
                        .map(|(a, b)| (a - b).abs())
                        .fold(0.0f64, f64::max)
                }
                _ => f64::NAN,
            },
            proj_certified_margin: match &proj_diag {
                Some(diag) if proj_feasible => {
                    proj_sol.beta.iter().zip(diag.eta.iter())
                        .map(|(&b, &e)| b - e)
                        .fold(f64::INFINITY, f64::min)
                }
                _ => f64::NAN,
            },
            proj_n_reliable_eigs: proj_diag.as_ref().map_or(0, |d| {
                d.eigenvalues.iter().filter(|&&g| g.abs() > d.eps_gamma).count()
            }),
            proj_n_uncertain_eigs: proj_diag.as_ref().map_or(0, |d| {
                d.eigenvalues.iter().filter(|&&g| g.abs() <= d.eps_gamma).count()
            }),
            proj_null_dim: proj_diag.as_ref().map_or(0, |d| d.null_dim),
            proj_delta_alpha: {
                // [rem:eigendirection-error]: |delta_alpha_j| ~ eps_mach / |gamma_j|
                match (&beta_exact, &proj_diag) {
                    (Some(be), Some(diag)) if !proj_sol.beta.is_empty()
                        && proj_feasible
                        && diag.null_dim > 0 =>
                    {
                        let be_f64: Vec<f64> = be.iter().map(|b| rational_to_f64(b)).collect();
                        let k = diag.null_dim;
                        let m = prob.m;
                        let db = DVector::from_fn(m, |i, _| proj_sol.beta[i] - be_f64[i]);
                        let vt_db = diag.null_basis.transpose() * &db;
                        let delta_alpha = diag.eigenvectors.transpose() * &vt_db;
                        (0..k).map(|j| delta_alpha[j].abs()).collect()
                    }
                    _ => Vec::new(),
                }
            },
            proj_eigenvalues: proj_diag.as_ref().map_or(Vec::new(), |d| {
                d.eigenvalues.iter().copied().collect()
            }),
        };

        let json = serde_json::to_string(&record).expect("serialize");
        writeln!(out_file, "{}", json).expect("write");
    }

    println!("\n=== Q Accuracy Summary ===");
    println!("Total problems: {}", problems.len());
    println!("Exact feasible: {}", n_exact_feasible);
    println!("Projection feasible: {}", n_proj_feasible);
    println!("Both feasible: {}", n_both_feasible);

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

    println!("\nOutput written to {}", output_path);
}
