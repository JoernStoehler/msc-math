/// Quick profiling of KKT solver variants (LU+SVD vs SVD-only).
///
/// Run with: cargo test -p billiard --release bench_kkt -- --nocapture --ignored
use crate::geom::known_polytopes;
use crate::geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector, Vector4};
use std::time::Instant;

use super::enumerate::{enumerate_blocks, enumerate_k_bounce_sigmas};
use crate::kkt::{solve_kkt, solve_kkt_svd_only, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use super::lagrangian::classify_facets;

/// Collect all billiard sigma sequences for the HKO pentagon.
fn pentagon_sigmas() -> (Vec<Vector4<f64>>, Vec<f64>, Vec<Vec<usize>>) {
    let kp = known_polytopes::hko_pentagon();
    let polytope = &kp.polytope;
    let normals = polytope.normals().to_vec();
    let heights = polytope.heights().to_vec();

    let classification = classify_facets(polytope).unwrap();
    let adj = crate::algorithms::hk2017::build_adjacency_matrix(polytope);
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    let mut sigmas = Vec::new();
    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            sigmas.push(sigma.to_vec());
        });
    }
    (normals, heights, sigmas)
}

/// Build KKT matrix (duplicated here for profiling individual phases).
fn build_kkt_matrix(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
        kkt[(m + 4, i)] = h;
    }
    rhs[m + 4] = 1.0;
    (kkt, rhs)
}

/// Profile LU+SVD vs SVD-only on the HKO pentagon.
///
/// **What:** Measures wall-clock time for both KKT solver variants on all
/// sigma sequences of the HKO pentagon.
///
/// **Why release mode:** Needs many KKT solves to get stable timing.
/// **Why #[ignore]:** Profiling test, not correctness. Run manually.
/// **Run with:** `cargo test -p billiard --release bench_kkt_lu_vs_svd -- --nocapture --ignored`
#[test]
#[ignore] // profiling test, run manually
fn bench_kkt_lu_vs_svd() {
    let (normals, heights, sigmas) = pentagon_sigmas();

    eprintln!("Total sigmas to test: {}", sigmas.len());

    // Time LU + SVD fallback (production variant)
    let start = Instant::now();
    let mut lu_svd_count = 0u64;
    let mut lu_svd_best = f64::INFINITY;
    for sigma in &sigmas {
        if let Some(result) = solve_kkt(&normals, &heights, sigma) {
            if result.beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && result.q_corrected > EPS_Q_POSITIVE {
                lu_svd_count += 1;
                lu_svd_best = lu_svd_best.min(0.5 / result.q_corrected);
            }
        }
    }
    let lu_svd_time = start.elapsed();

    // Time SVD only (ablation variant)
    let start = Instant::now();
    let mut svd_count = 0u64;
    let mut svd_best = f64::INFINITY;
    for sigma in &sigmas {
        if let Some(result) = solve_kkt_svd_only(&normals, &heights, sigma) {
            if result.beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && result.q_corrected > EPS_Q_POSITIVE {
                svd_count += 1;
                svd_best = svd_best.min(0.5 / result.q_corrected);
            }
        }
    }
    let svd_time = start.elapsed();

    // Time just enumeration (no KKT solve)
    let kp = known_polytopes::hko_pentagon();
    let polytope = &kp.polytope;
    let classification = classify_facets(polytope).unwrap();
    let adj = crate::algorithms::hk2017::build_adjacency_matrix(polytope);
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);
    let start = Instant::now();
    let mut enum_count = 0u64;
    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |_sigma| {
            enum_count += 1;
        });
    }
    let enum_time = start.elapsed();

    eprintln!("\n=== Profiling Results (HKO Pentagon, {} sigmas) ===", sigmas.len());
    eprintln!("Enumeration only:   {:>8.2?}  ({} sigmas)", enum_time, enum_count);
    eprintln!("LU + SVD fallback:  {:>8.2?}  ({} valid, best={:.6})", lu_svd_time, lu_svd_count, lu_svd_best);
    eprintln!("SVD only:           {:>8.2?}  ({} valid, best={:.6})", svd_time, svd_count, svd_best);
    eprintln!(
        "Ratio (SVD/LU):     {:.2}x",
        svd_time.as_secs_f64() / lu_svd_time.as_secs_f64()
    );
    eprintln!(
        "Enum fraction:      {:.1}% of LU time",
        100.0 * enum_time.as_secs_f64() / lu_svd_time.as_secs_f64()
    );

    // Sanity: both variants should find the same capacity
    assert_eq!(lu_svd_count, svd_count, "different valid orbit counts");
    assert!(
        (lu_svd_best - svd_best).abs() < 1e-10,
        "different best capacities: LU+SVD={lu_svd_best}, SVD={svd_best}"
    );
}

/// Profile SVD path phase-by-phase to find bottleneck.
///
/// Breaks down the gap-based SVD solver into timed phases:
/// 1. Matrix construction
/// 2. SVD decomposition
/// 3. Gap-based rank detection + manual pseudoinverse
/// 4. Residual check
/// 5. For comparison: nalgebra's built-in svd.solve() (the pre-fix approach)
///
/// **Run with:** `cargo test -p billiard --release bench_kkt_svd_profile -- --nocapture --ignored`
#[test]
#[ignore]
fn bench_kkt_svd_profile() {
    let (normals, heights, sigmas) = pentagon_sigmas();
    let n_sigmas = sigmas.len();

    eprintln!("Profiling SVD phases on {n_sigmas} pentagon sigmas...\n");

    // Phase timers (cumulative across all sigmas)
    let mut t_build = 0.0f64;
    let mut t_svd_decomp = 0.0f64;
    let mut t_gap_rank = 0.0f64;
    let mut t_manual_pinv = 0.0f64;
    let mut t_residual = 0.0f64;
    let mut t_nalgebra_solve = 0.0f64;
    let mut t_lu_decomp = 0.0f64;
    let mut t_lu_solve_check = 0.0f64;
    let mut lu_invertible_count = 0u64;
    let mut lu_success_count = 0u64;

    let eps_svd_floor = 1e-12f64;
    let svd_gap_threshold = 100.0f64;
    let eps_kkt_residual = 1e-6f64;

    for sigma in &sigmas {
        let m = sigma.len();
        let size = m + 5;

        // Phase 1: Build KKT matrix
        let t0 = Instant::now();
        let (kkt, rhs) = build_kkt_matrix(&normals, &heights, sigma);
        t_build += t0.elapsed().as_secs_f64();

        // Phase 1b: LU decomposition
        let t0 = Instant::now();
        let lu = kkt.clone().full_piv_lu();
        let is_inv = lu.is_invertible();
        t_lu_decomp += t0.elapsed().as_secs_f64();

        if is_inv {
            lu_invertible_count += 1;
            let t0 = Instant::now();
            if let Some(solution) = lu.solve(&rhs) {
                let residual = (&kkt * &solution - &rhs).norm();
                if residual <= eps_kkt_residual {
                    let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();
                    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
                        lu_success_count += 1;
                    }
                }
            }
            t_lu_solve_check += t0.elapsed().as_secs_f64();
        }

        // Phase 2: SVD decomposition
        let t0 = Instant::now();
        let svd = kkt.clone().svd(true, true);
        t_svd_decomp += t0.elapsed().as_secs_f64();

        let sv = &svd.singular_values;
        let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
        if max_sv < eps_svd_floor {
            continue;
        }
        let u = svd.u.as_ref().unwrap();
        let v_t = svd.v_t.as_ref().unwrap();

        // Phase 3a: Gap-based rank detection
        let t0 = Instant::now();
        let floor = max_sv * eps_svd_floor;
        let nonzero = sv.iter().filter(|&&s| s > floor).count();
        let mut rank = nonzero;
        for i in (1..nonzero).rev() {
            if sv[i - 1] / sv[i] > svd_gap_threshold {
                rank = i;
                break;
            }
        }
        t_gap_rank += t0.elapsed().as_secs_f64();

        // Phase 3b: Manual pseudoinverse
        let t0 = Instant::now();
        let mut x0 = DVector::zeros(size);
        for i in 0..rank {
            let coeff = u.column(i).dot(&rhs) / sv[i];
            for j in 0..size {
                x0[j] += coeff * v_t[(i, j)];
            }
        }
        t_manual_pinv += t0.elapsed().as_secs_f64();

        // Phase 4: Residual check
        let t0 = Instant::now();
        let _residual = (&kkt * &x0 - &rhs).norm();
        t_residual += t0.elapsed().as_secs_f64();

        // Phase 5: nalgebra's built-in solve (pre-fix approach for comparison)
        let t0 = Instant::now();
        let _x_old = svd.solve(&rhs, eps_svd_floor);
        t_nalgebra_solve += t0.elapsed().as_secs_f64();
    }

    let t_total_new = t_build + t_svd_decomp + t_gap_rank + t_manual_pinv + t_residual;
    let t_total_old = t_build + t_svd_decomp + t_nalgebra_solve;
    let t_total_lu_svd = t_build + t_lu_decomp + t_lu_solve_check + t_svd_decomp + t_gap_rank + t_manual_pinv + t_residual;

    eprintln!("=== SVD Phase Breakdown ({n_sigmas} sigmas) ===");
    eprintln!("Matrix build:         {:>8.1}ms  ({:.1}%)", t_build * 1000.0, 100.0 * t_build / t_total_new);
    eprintln!("SVD decomposition:    {:>8.1}ms  ({:.1}%)", t_svd_decomp * 1000.0, 100.0 * t_svd_decomp / t_total_new);
    eprintln!("Gap-based rank:       {:>8.1}ms  ({:.1}%)", t_gap_rank * 1000.0, 100.0 * t_gap_rank / t_total_new);
    eprintln!("Manual pseudoinverse: {:>8.1}ms  ({:.1}%)", t_manual_pinv * 1000.0, 100.0 * t_manual_pinv / t_total_new);
    eprintln!("Residual check:       {:>8.1}ms  ({:.1}%)", t_residual * 1000.0, 100.0 * t_residual / t_total_new);
    eprintln!("──────────────────────────────────────");
    eprintln!("Total (gap-based):    {:>8.1}ms", t_total_new * 1000.0);
    eprintln!();
    eprintln!("nalgebra svd.solve(): {:>8.1}ms  (pre-fix approach)", t_nalgebra_solve * 1000.0);
    eprintln!("Total (svd.solve):    {:>8.1}ms", t_total_old * 1000.0);
    eprintln!();
    eprintln!("Gap-based / svd.solve() ratio: {:.2}x", t_total_new / t_total_old);
    eprintln!();
    eprintln!("=== LU Fast Path Stats ===");
    eprintln!("LU decomposition:     {:>8.1}ms", t_lu_decomp * 1000.0);
    eprintln!("LU solve+check:       {:>8.1}ms", t_lu_solve_check * 1000.0);
    eprintln!("LU invertible:        {lu_invertible_count}/{n_sigmas} ({:.1}%)", 100.0 * lu_invertible_count as f64 / n_sigmas as f64);
    eprintln!("LU full success:      {lu_success_count}/{n_sigmas} ({:.1}%)", 100.0 * lu_success_count as f64 / n_sigmas as f64);
    eprintln!("Total (LU+SVD):       {:>8.1}ms", t_total_lu_svd * 1000.0);
    eprintln!("LU overhead vs SVD:   {:>8.1}ms ({:.1}%)", (t_lu_decomp + t_lu_solve_check) * 1000.0, 100.0 * (t_lu_decomp + t_lu_solve_check) / t_total_new);
}
