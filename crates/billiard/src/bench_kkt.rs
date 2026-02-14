/// Quick profiling of KKT solver variants.
/// Run with: cargo test -p billiard --release bench_kkt -- --nocapture --ignored
use geom::known_polytopes;
use geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector};
use std::time::Instant;

use crate::enumerate::{enumerate_blocks, enumerate_k_bounce_sigmas};
use crate::lagrangian::classify_facets;

const EPS_SVD_TOLERANCE: f64 = 1e-10;
const EPS_KKT_RESIDUAL: f64 = 1e-6;
const EPS_BETA_POSITIVE: f64 = 1e-12;
const EPS_Q_POSITIVE: f64 = 1e-15;

/// Solve KKT using LU with SVD fallback (current approach).
fn solve_kkt_lu_svd(
    normals: &[nalgebra::Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
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

    let lu = kkt.clone().full_piv_lu();
    let solution = if lu.is_invertible() {
        lu.solve(&rhs)?
    } else {
        let svd = kkt.clone().svd(true, true);
        svd.solve(&rhs, EPS_SVD_TOLERANCE).ok()?
    };

    let residual = (&kkt * &solution - &rhs).norm();
    if residual > EPS_KKT_RESIDUAL {
        return None;
    }

    let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();
    let q_val: f64 = (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum();

    Some((beta, q_val))
}

/// Solve KKT using direct SVD (no LU attempt).
fn solve_kkt_svd_only(
    normals: &[nalgebra::Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
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

    let svd = kkt.clone().svd(true, true);
    let solution = svd.solve(&rhs, EPS_SVD_TOLERANCE).ok()?;

    let residual = (&kkt * &solution - &rhs).norm();
    if residual > EPS_KKT_RESIDUAL {
        return None;
    }

    let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();
    let q_val: f64 = (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum();

    Some((beta, q_val))
}

/// Build facet adjacency matrix.
fn build_adjacency_matrix(polytope: &geom::polytope::Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();
    let mut adj = vec![vec![false; f]; f];
    for v in polytope.vertices() {
        let incident: Vec<usize> = (0..f)
            .filter(|&i| (normals[i].dot(v) - heights[i]).abs() < 1e-8)
            .collect();
        for &i in &incident {
            for &j in &incident {
                adj[i][j] = true;
            }
        }
    }
    adj
}

#[test]
#[ignore] // profiling test, run manually
fn bench_kkt_lu_vs_svd() {
    let kp = known_polytopes::hko_pentagon();
    let polytope = &kp.polytope;
    let normals = polytope.normals();
    let heights = polytope.heights();

    let classification = classify_facets(polytope).unwrap();
    let adj = build_adjacency_matrix(polytope);
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    // Collect all sigma sequences
    let mut sigmas = Vec::new();
    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            sigmas.push(sigma.to_vec());
        });
    }

    eprintln!("Total sigmas to test: {}", sigmas.len());

    // Time LU + SVD fallback
    let start = Instant::now();
    let mut lu_svd_count = 0u64;
    let mut lu_svd_best = f64::INFINITY;
    for sigma in &sigmas {
        if let Some((beta, q_val)) = solve_kkt_lu_svd(normals, heights, sigma) {
            if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                lu_svd_count += 1;
                lu_svd_best = lu_svd_best.min(0.5 / q_val);
            }
        }
    }
    let lu_svd_time = start.elapsed();

    // Time SVD only
    let start = Instant::now();
    let mut svd_count = 0u64;
    let mut svd_best = f64::INFINITY;
    for sigma in &sigmas {
        if let Some((beta, q_val)) = solve_kkt_svd_only(normals, heights, sigma) {
            if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                svd_count += 1;
                svd_best = svd_best.min(0.5 / q_val);
            }
        }
    }
    let svd_time = start.elapsed();

    // Time just enumeration (no KKT solve)
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
}
