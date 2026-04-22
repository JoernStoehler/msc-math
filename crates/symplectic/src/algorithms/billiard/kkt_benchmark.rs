//! Profiling benchmarks for the KKT solver on billiard sigma sequences.
//!
//! Measures eigendecomposition-based KKT solver performance on all billiard
//! sigma sequences of the HKO pentagon. Reports total time, valid solution
//! count, per-sigma timing, and phase breakdown.
//!
//! Run with: `cargo test --release -- --ignored --nocapture bench_kkt`
//!
//! Mathematical correspondence: [alg:billiard], performance characterization

#![cfg(test)]

// Test-only profiling benchmarks (not library functionality).
//
// This module is only compiled during `cfg(test)` builds.
use crate::geom::known_polytopes;
use crate::kkt::qp_assembly::build_augmented_system;
use crate::kkt::saddle_point_solver::{
    solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};
use nalgebra::DVector;
use std::time::Instant;

use super::block_enumeration::{enumerate_blocks, enumerate_k_bounce_sigmas};
use super::facet_classification::classify_facets;

/// Collect all billiard sigma sequences for the HKO pentagon.
///
/// Returns the polytope and the sigma sequences as owned vectors.
fn pentagon_sigmas() -> (crate::geom::polytope::Polytope4D, Vec<Vec<usize>>) {
    let kp = known_polytopes::hko_pentagon();
    let polytope = kp.polytope.clone();

    let classification = classify_facets(&polytope).unwrap();
    let adj = polytope.vertex_adjacency();
    let q_blocks = enumerate_blocks(&classification.q_indices, adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, adj);

    let mut sigmas = Vec::new();
    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            sigmas.push(sigma.to_vec());
        });
    }
    (polytope, sigmas)
}

/// Benchmark eigendecomposition-based KKT solver on the HKO pentagon.
///
/// Times `solve_kkt_for` on all billiard sigma sequences. Reports total time,
/// valid solution count, and best capacity.
///
/// **Why release mode:** needs many KKT solves for stable timing.
/// **Why #[ignore]:** profiling test, not correctness. Run manually.
#[test]
#[ignore] // profiling test, run manually with --release --nocapture --ignored
fn bench_kkt_eigen() {
    let (polytope, sigmas) = pentagon_sigmas();

    eprintln!("Total sigmas to test: {}", sigmas.len());

    let start = Instant::now();
    let mut valid_count = 0u64;
    let mut best_capacity = f64::INFINITY;
    for sigma in &sigmas {
        if let KktOutcome::Feasible(result) = solve_kkt_for(&polytope, sigma) {
            if result.beta.iter().all(|&b| b > EPS_BETA_POSITIVE)
                && result.q_corrected > EPS_Q_POSITIVE
            {
                valid_count += 1;
                best_capacity = best_capacity.min(0.5 / result.q_corrected);
            }
        }
    }
    let elapsed = start.elapsed();

    eprintln!(
        "\n=== Eigendecomposition Benchmark (HKO Pentagon, {} sigmas) ===",
        sigmas.len()
    );
    eprintln!("Total time:       {:>8.2?}", elapsed);
    eprintln!("Valid solutions:   {valid_count}");
    eprintln!("Best capacity:     {best_capacity:.6}");
    eprintln!(
        "Per-sigma average: {:.1}us",
        elapsed.as_secs_f64() * 1e6 / sigmas.len() as f64
    );
}

/// Phase-by-phase profiling of eigendecomposition KKT solver.
///
/// Breaks down the solver into timed phases to identify bottlenecks:
///   1. Matrix construction (`build_augmented_system`)
///   2. Eigendecomposition (`symmetric_eigen`)
///   3. Rank detection + pseudoinverse
///   4. Residual check
///
/// **Why release mode:** needs many KKT solves for stable timing.
/// **Why #[ignore]:** profiling test, not correctness. Run manually.
#[test]
#[ignore] // profiling test, run manually with --release --nocapture --ignored
fn bench_kkt_eigen_profile() {
    let (polytope, sigmas) = pentagon_sigmas();
    let n_sigmas = sigmas.len();

    eprintln!("Profiling eigendecomposition phases on {n_sigmas} pentagon sigmas...\n");

    let mut t_build = 0.0f64;
    let mut t_eigen_decomp = 0.0f64;
    let mut t_rank_pinv = 0.0f64;
    let mut t_residual = 0.0f64;

    // Matches EIGEN_CONDITION_TAU in saddle_point_solver.rs (private constant).
    let eigen_condition_tau = 1e-3f64;
    // Matches EPS_EIGEN_FLOOR in saddle_point_solver.rs (private constant).
    let eps_eigen_floor = 1e-12f64;

    for sigma in &sigmas {
        let m = sigma.len();
        let size = m + 5;

        // Phase 1: Build KKT matrix.
        let t0 = Instant::now();
        let (kkt, rhs) = build_augmented_system(&polytope, sigma);
        t_build += t0.elapsed().as_secs_f64();

        // Phase 2: Eigendecomposition.
        let t0 = Instant::now();
        let eig = kkt.clone().symmetric_eigen();
        t_eigen_decomp += t0.elapsed().as_secs_f64();

        let eigenvalues = &eig.eigenvalues;
        let eigenvectors = &eig.eigenvectors;

        let max_abs_ev = eigenvalues.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
        if max_abs_ev < eps_eigen_floor {
            continue;
        }

        // Phase 3: Rank detection + pseudoinverse.
        let t0 = Instant::now();
        let threshold = max_abs_ev * eigen_condition_tau;

        let mut x0 = DVector::zeros(size);
        for i in 0..size {
            if eigenvalues[i].abs() > threshold {
                let coeff = eigenvectors.column(i).dot(&rhs) / eigenvalues[i];
                for j in 0..size {
                    x0[j] += coeff * eigenvectors[(j, i)];
                }
            }
        }
        t_rank_pinv += t0.elapsed().as_secs_f64();

        // Phase 4: Residual check.
        let t0 = Instant::now();
        let _residual = (&kkt * &x0 - &rhs).norm();
        t_residual += t0.elapsed().as_secs_f64();
    }

    let t_total = t_build + t_eigen_decomp + t_rank_pinv + t_residual;

    eprintln!("=== Eigendecomposition Phase Breakdown ({n_sigmas} sigmas) ===");
    eprintln!(
        "Matrix build:          {:>8.1}ms  ({:.1}%)",
        t_build * 1000.0,
        100.0 * t_build / t_total
    );
    eprintln!(
        "Eigendecomposition:    {:>8.1}ms  ({:.1}%)",
        t_eigen_decomp * 1000.0,
        100.0 * t_eigen_decomp / t_total
    );
    eprintln!(
        "Rank detect + pinv:    {:>8.1}ms  ({:.1}%)",
        t_rank_pinv * 1000.0,
        100.0 * t_rank_pinv / t_total
    );
    eprintln!(
        "Residual check:        {:>8.1}ms  ({:.1}%)",
        t_residual * 1000.0,
        100.0 * t_residual / t_total
    );
    eprintln!("--------------------------------------");
    eprintln!("Total:                 {:>8.1}ms", t_total * 1000.0);
    eprintln!(
        "Per-sigma average:     {:>8.1}us",
        t_total * 1e6 / n_sigmas as f64
    );
}
