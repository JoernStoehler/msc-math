//! Omega-obstacle experiment: do near-Lagrangian 2-faces help create high systolic ratios?
//!
//! Hypothesis: small |ω₀(n_i, n_j)| between adjacent facets → high sys.
//! Mechanism: Q(β) = Σ β_i β_j ω₀(...), capacity = 1/(2·max Q), sys = c²/(2V).
//! Small ω contributions → smaller Q → larger capacity → potentially larger sys.
//!
//! Phase A (observational): For each polytope, compute ω₀ for all ridge-adjacent pairs
//! and for orbit transitions. Plot min|ω| vs sys.
//!
//! Phase B (gradient): Compute ⟨∇_{n_k} sys, ∇_{n_k} ω(n_k, n_i)⟩ analytically.
//! Negative dot product → sys increases when ω decreases → hypothesis supported.
//!
//! Architecture:
//! 1. `cargo run --bin omega_obstacle --release` generates dataset
//! 2. Writes to omega-obstacle/omega-obstacle.jsonl
//! 3. Python script reads JSONL, produces figures
//!
//! Convention: Instrumented HK2017 and sensitivity infrastructure are copied from
//! sys-optimization (self-contained experiment binary, per CLAUDE.md).

use nalgebra::{DMatrix, DVector, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::time::Instant;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
// Using full module paths until then.
use symplectic::geom::known_polytopes;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

// ============================================================================
// Configuration
// ============================================================================

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples) pairs for random polytope generation.
const SAMPLING_PLAN: &[(usize, usize)] = &[
    (5, 200),
    (6, 200),
    (7, 200),
    (8, 200),
    (9, 100),
    (10, 50),
];

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct OmegaRow {
    source: String,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_ms: f64,

    // Orbit info
    orbit_length: usize,
    orbit_facets: Vec<usize>,
    orbit_betas: Vec<f64>,

    // Omega features — orbit transitions (physical direction, all ≥ 0)
    orbit_omegas: Vec<f64>,
    orbit_omega_min: f64,
    orbit_omega_mean: f64,

    // Omega features — all ridge-adjacent pairs
    ridge_omegas: Vec<[f64; 3]>, // [i, j, ω₀(n_i, n_j)] where i < j
    ridge_omega_abs_min: f64,
    n_ridges: usize,

    // Gradient dot products (Phase B)
    gradient_dots: Vec<GradientDot>,
}

#[derive(Debug, Serialize)]
struct GradientDot {
    facet_k: usize,
    neighbor_i: usize,
    k_on_orbit: bool,
    i_on_orbit: bool,
    omega: f64,
    dot: f64,
    grad_sys_norm: f64,
}

// ============================================================================
// Constants copied from library (crates/src/kkt.rs, constants.rs)
// ============================================================================

const EPS_BETA_POSITIVE: f64 = 1e-12;
const EPS_Q_POSITIVE: f64 = 1e-15;
const EPS_SVD_FLOOR: f64 = 1e-12;
const SVD_CONDITION_TAU: f64 = 1e-3;
const EPS_KKT_RESIDUAL: f64 = 1e-6;
const EPS_FACET_INCIDENCE: f64 = 1e-8;
const EPS_DEGENERATE: f64 = 1e-10;

// ============================================================================
// KKT solver — copied from sys-optimization/sys_optimization.rs
// (which copies from crates/src/kkt.rs with extensions for ν, λ)
// ============================================================================

fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for j in 0..m {
        if v[j].abs() < 1e-15 {
            if beta0[j] <= EPS_BETA_POSITIVE {
                return None;
            }
        } else {
            let bound = -beta0[j] / v[j];
            if v[j] > 0.0 {
                lo = lo.max(bound);
            } else {
                hi = hi.min(bound);
            }
        }
    }
    if lo >= hi {
        return None;
    }
    let alpha = if lo.is_finite() && hi.is_finite() {
        (lo + hi) / 2.0
    } else if lo.is_finite() {
        lo + 1.0
    } else if hi.is_finite() {
        hi - 1.0
    } else {
        0.0
    };
    let beta: Vec<f64> = (0..m).map(|j| beta0[j] + alpha * v[j]).collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

fn find_positive_beta_nd(beta0: &[f64], null_vecs: &[Vec<f64>]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![0.0; k];
    for _iter in 0..100 {
        let beta: Vec<f64> = (0..m)
            .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
            .collect();
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        if *worst_val > EPS_BETA_POSITIVE {
            return Some(beta);
        }
        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            return None;
        }
        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

/// Build KKT matrix with ASYMMETRIC sign convention (standard multipliers).
/// Source: sys-optimization/sys_optimization.rs:294-324
fn build_kkt_system(
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

/// SVD-based KKT solver returning (β, Q, ν, λ).
/// Source: sys-optimization/sys_optimization.rs:332-436
fn solve_kkt_svd_path(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64, f64, Vec<f64>)> {
    let m = perm.len();
    let size = m + 5;
    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
    if max_sv < EPS_SVD_FLOOR {
        return None;
    }
    let u = svd.u.as_ref()?;
    let v_t = svd.v_t.as_ref()?;

    let threshold = max_sv * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();

    // Early dismissal via δβ-component check
    if rank < size && m >= 5 {
        let mut c_matrix = DMatrix::zeros(m, 5);
        for i in 0..m {
            let n = &normals[perm[i]];
            for j in 0..4 {
                c_matrix[(i, j)] = n[j];
            }
            c_matrix[(i, 4)] = heights[perm[i]];
        }
        let c_svd = c_matrix.svd(false, false);
        let sigma_c = c_svd
            .singular_values
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);

        if sigma_c > EPS_SVD_FLOOR {
            for j in rank..size {
                let delta_beta_norm: f64 =
                    (0..m).map(|k| v_t[(j, k)].powi(2)).sum::<f64>().sqrt();
                if delta_beta_norm > sv[j] / sigma_c {
                    return None;
                }
            }
        }
    }

    let mut x0 = DVector::zeros(size);
    for i in 0..rank {
        let coeff = u.column(i).dot(rhs) / sv[i];
        for j in 0..size {
            x0[j] += coeff * v_t[(i, j)];
        }
    }
    let residual = (kkt * &x0 - rhs).norm();
    if residual > EPS_KKT_RESIDUAL {
        return None;
    }
    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    let lambda: Vec<f64> = (m..m + 4).map(|i| x0[i]).collect();
    let nu = x0[m + 4];
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_val = q_from_beta(normals, perm, &beta0);
        return Some((beta0, q_val, nu, lambda));
    }
    if rank == size {
        return None;
    }

    let null_beta: Vec<Vec<f64>> = (rank..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();
    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])?
    } else {
        find_positive_beta_nd(&beta0, &null_beta)?
    };

    let constraint_residual: f64 = (0..4)
        .map(|d| {
            (0..m)
                .map(|i| beta_opt[i] * normals[perm[i]][d])
                .sum::<f64>()
        })
        .map(|x: f64| x * x)
        .sum::<f64>()
        + ((0..m)
            .map(|i| beta_opt[i] * heights[perm[i]])
            .sum::<f64>()
            - 1.0)
            .powi(2);
    if constraint_residual.sqrt() > EPS_KKT_RESIDUAL {
        return None;
    }
    let q_val = q_from_beta(normals, perm, &beta_opt);
    Some((beta_opt, q_val, nu, lambda))
}

fn solve_kkt_full(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64, f64, Vec<f64>)> {
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}

// ============================================================================
// Combinatorial infrastructure — copied from sys-optimization
// ============================================================================

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.clone());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

fn for_each_cyclic_permutation(elements: &[usize], callback: &mut impl FnMut(&[usize])) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf(&mut buf, 1, k, callback);
}

fn heap_perms_buf(
    buf: &mut [usize],
    offset: usize,
    k: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_perms_buf(buf, offset, k - 1, callback);
    for i in 0..k - 1 {
        if k % 2 == 0 {
            buf.swap(offset + i, offset + k - 1);
        } else {
            buf.swap(offset, offset + k - 1);
        }
        heap_perms_buf(buf, offset, k - 1, callback);
    }
}

fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let mut adj = vec![vec![false; f]; f];
    for v in polytope.vertices_f64() {
        let incident: Vec<usize> = (0..f)
            .filter(|&i| (normals[i].dot(v) - heights[i]).abs() < EPS_FACET_INCIDENCE)
            .collect();
        for &i in &incident {
            for &j in &incident {
                adj[i][j] = true;
            }
        }
    }
    adj
}

fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let vertex_adj = build_adjacency_matrix(polytope);
    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            adj[i][j] = vertex_adj[i][j] && omega0(&normals[i], &normals[j]) >= 0.0;
        }
    }
    adj
}

fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// Instrumented HK2017 — returns ValidOrbit with β*, λ*, ν*, Q*
// Source: sys-optimization/sys_optimization.rs:570-663
// ============================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ValidOrbit {
    action: f64,
    subset: Vec<usize>,
    permutation: Vec<usize>, // positive Reeb direction
    beta: Vec<f64>,
    q_value: f64,
    nu: f64,
    lambda: Vec<f64>,
}

struct InstrumentedResult {
    capacity: f64,
    orbits: Vec<ValidOrbit>, // sorted by action ascending
    iterations: u64,
}

fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let adj = build_directed_adjacency_matrix(polytope);

    let mut orbits: Vec<ValidOrbit> = Vec::new();
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }
                iterations += 1;

                if let Some((beta, q_val, nu, lambda)) = solve_kkt_full(&normals, &heights, perm) {
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push(ValidOrbit {
                            action,
                            subset: subset.clone(),
                            permutation: perm.to_vec(),
                            beta: beta.clone(),
                            q_value: q_val,
                            nu,
                            lambda,
                        });
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.action.partial_cmp(&b.action).unwrap());
    let capacity = orbits[0].action;

    Some(InstrumentedResult {
        capacity,
        orbits,
        iterations,
    })
}

// ============================================================================
// Sensitivity computation — copied from sys-optimization
// ============================================================================

/// J₀(a,b,c,d) = (-c,-d,a,b) in (q₁,q₂,p₁,p₂) coordinates.
fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

/// 4D cross product. Source: crates/src/geom/cross_product.rs
fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let bc_01 = b[0] * c[1] - b[1] * c[0];
    let bc_02 = b[0] * c[2] - b[2] * c[0];
    let bc_03 = b[0] * c[3] - b[3] * c[0];
    let bc_12 = b[1] * c[2] - b[2] * c[1];
    let bc_13 = b[1] * c[3] - b[3] * c[1];
    let bc_23 = b[2] * c[3] - b[3] * c[2];
    Vector4::new(
        a[1] * bc_23 - a[2] * bc_13 + a[3] * bc_12,
        -(a[0] * bc_23 - a[2] * bc_03 + a[3] * bc_02),
        a[0] * bc_13 - a[1] * bc_03 + a[3] * bc_01,
        -(a[0] * bc_12 - a[1] * bc_02 + a[2] * bc_01),
    )
}

fn sort_polygon_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    if vertices.len() <= 3 {
        return vertices.to_vec();
    }
    let n = vertices.len() as f64;
    let centroid = vertices.iter().copied().sum::<Vector4<f64>>() / n;
    let d1 = (vertices[0] - centroid).normalize();
    let d2 = match vertices.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > EPS_DEGENERATE).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return vertices.to_vec(),
    };
    let mut indexed: Vec<(f64, Vector4<f64>)> = vertices
        .iter()
        .map(|v| {
            let rel = *v - centroid;
            let angle = rel.dot(&d2).atan2(rel.dot(&d1));
            (angle, *v)
        })
        .collect();
    indexed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    indexed.into_iter().map(|(_, v)| v).collect()
}

/// 3D volume and centroid of facet `fi`.
fn facet_volume_and_centroid_3d(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
    fi: usize,
    f: usize,
) -> (f64, Vector4<f64>) {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (normals[fi].dot(v) - heights[fi]).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return (0.0, Vector4::zeros());
    }

    let apex = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;
    let mut total_vol = 0.0;
    let mut weighted_centroid = Vector4::zeros();

    for fj in 0..f {
        if fj == fi {
            continue;
        }
        let ridge_verts: Vec<Vector4<f64>> = facet_verts
            .iter()
            .filter(|v| (normals[fj].dot(v) - heights[fj]).abs() < EPS_FACET_INCIDENCE)
            .cloned()
            .collect();
        if ridge_verts.len() < 3 {
            continue;
        }
        let sorted = sort_polygon_vertices(&ridge_verts);
        for k in 1..sorted.len() - 1 {
            let a = sorted[0] - apex;
            let b = sorted[k] - apex;
            let c = sorted[k + 1] - apex;
            let tet_vol = cross_product_4d(a, b, c).norm() / 6.0;
            let tet_centroid = (apex + sorted[0] + sorted[k] + sorted[k + 1]) / 4.0;
            total_vol += tet_vol;
            weighted_centroid += tet_vol * tet_centroid;
        }
    }

    if total_vol > 1e-30 {
        (total_vol, weighted_centroid / total_vol)
    } else {
        (0.0, Vector4::zeros())
    }
}

/// ∂vol/∂h_k = vol_3D(F_k).
#[allow(dead_code)]
fn compute_volume_derivatives_h(polytope: &Polytope4D) -> Vec<f64> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();
    (0..f)
        .map(|k| {
            let (vol, _) = facet_volume_and_centroid_3d(&normals, &heights, vertices, k, f);
            vol
        })
        .collect()
}

/// ∇_{n_k} vol projected to T_{n_k}S³.
/// Source: sys-optimization/sys_optimization.rs:959-976
fn compute_volume_derivatives_n(polytope: &Polytope4D) -> Vec<Vector4<f64>> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();
    (0..f)
        .map(|k| {
            let (s_k, centroid_k) = facet_volume_and_centroid_3d(&normals, &heights, vertices, k, f);
            if s_k < 1e-30 {
                return Vector4::zeros();
            }
            let tangent_centroid = centroid_k - heights[k] * normals[k];
            -s_k * tangent_centroid
        })
        .collect()
}

/// ∂c/∂n_k via envelope theorem, projected to T_{n_k}S³.
/// Source: sys-optimization/sys_optimization.rs:991-1032
fn compute_capacity_derivatives_n(
    best_orbit: &ValidOrbit,
    normals: &[Vector4<f64>],
    facet_count: usize,
) -> Vec<Vector4<f64>> {
    let q_sq = best_orbit.q_value * best_orbit.q_value;
    let perm = &best_orbit.permutation;
    let beta = &best_orbit.beta;
    let lambda = Vector4::new(
        best_orbit.lambda[0],
        best_orbit.lambda[1],
        best_orbit.lambda[2],
        best_orbit.lambda[3],
    );

    (0..facet_count)
        .map(|k| {
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(),
            };
            // P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)}
            let mut p = Vector4::zeros();
            for i in 0..i0 {
                p += beta[i] * normals[perm[i]];
            }
            let inner = 2.0 * p + beta[i0] * normals[k];
            let j0_inner = j0_apply(&inner);
            let dq_dn = beta[i0] * (j0_inner - lambda);
            let dq_dn_tangent = dq_dn - dq_dn.dot(&normals[k]) * normals[k];
            -dq_dn_tangent / (2.0 * q_sq)
        })
        .collect()
}

/// ∂c/∂h_k via envelope theorem.
/// Source: sys-optimization/sys_optimization.rs:929-948
#[allow(dead_code)]
fn compute_capacity_derivatives_h(
    best_orbit: &ValidOrbit,
    facet_count: usize,
) -> Vec<f64> {
    let q_sq = best_orbit.q_value * best_orbit.q_value;
    (0..facet_count)
        .map(|k| {
            match best_orbit.permutation.iter().position(|&f| f == k) {
                // Lemma lem:cap-derivative: ∂A/∂h_k = ν·β_{i₀}/(2Q²)
                Some(i0) => best_orbit.nu * best_orbit.beta[i0] / (2.0 * q_sq),
                None => 0.0,
            }
        })
        .collect()
}

/// Full ∇_{n_k} sys via chain rule: d(sys)/d(n_k) = (1/V)[c·dc/dn_k - sys·dV/dn_k].
fn compute_d_sys_n(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    instrumented: &InstrumentedResult,
) -> Vec<Vector4<f64>> {
    let normals = polytope.normals_f64();
    let f = normals.len();
    let best_orbit = &instrumented.orbits[0];

    let d_vol_n = compute_volume_derivatives_n(polytope);
    let d_cap_n = compute_capacity_derivatives_n(best_orbit, &normals, f);

    d_vol_n
        .iter()
        .zip(d_cap_n.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

// ============================================================================
// Phase A: Omega feature computation
// ============================================================================

/// Compute ω₀ for all ridge-adjacent pairs and orbit transitions.
fn compute_omega_features(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    orbit_facets: &[usize],  // physical direction (from EhzResult)
) -> (Vec<[f64; 3]>, Vec<f64>) {
    let normals = polytope.normals_f64();

    // Ridge omegas: for each ridge (2-face shared by facets i, j with i < j)
    let ridge_omegas: Vec<[f64; 3]> = skeleton
        .ridges
        .iter()
        .map(|r| {
            let i = r.facets[0];
            let j = r.facets[1];
            let w = omega0(&normals[i], &normals[j]);
            [i as f64, j as f64, w]
        })
        .collect();

    // Orbit omegas: ω₀(n_{σ(k)}, n_{σ(k+1)}) for physical transition σ(k) → σ(k+1).
    // For a physical transition A → B, feasibility requires ω₀(n_A, n_B) ≥ 0.
    let m = orbit_facets.len();
    let orbit_omegas: Vec<f64> = (0..m)
        .map(|k| {
            let from = orbit_facets[k];
            let to = orbit_facets[(k + 1) % m];
            omega0(&normals[from], &normals[to])
        })
        .collect();

    (ridge_omegas, orbit_omegas)
}

// ============================================================================
// Phase B: Gradient dot product computation
// ============================================================================

/// Compute ∇_{n_k} ω₀(n_k, n_i) projected to T_{n_k}S³.
///
/// Since ω₀(u, v) = ⟨J₀ u, v⟩ is bilinear:
///   ∂ω₀(n_k, n_i)/∂n_k = J₀^T n_i = -J₀ n_i  (because J₀^T = -J₀)
///
/// Wait — ω₀(n_k, n_i) = ⟨J₀ n_k, n_i⟩ (linear in n_k), so the gradient
/// w.r.t. n_k in R⁴ is J₀^T n_i = -J₀ n_i. But ω₀(u,v) = u^T J₀^T v
/// where we use the convention ω₀(u,v) = u[0]v[2] - u[2]v[0] + ...
/// Let's check: ω₀(u,v) = Σ (u_{q_j} v_{p_j} - u_{p_j} v_{q_j})
///            = u^T M v where M has the right entries.
/// ∂ω₀(n_k, n_i)/∂n_k = M n_i where M is the matrix of ω₀.
///
/// M = [[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]] = J₀^T = -J₀
/// (since J₀ is skew-symmetric: J₀^T = -J₀)
///
/// So ∂ω₀(n_k, n_i)/∂n_k = -J₀ n_i. Projected to T_{n_k}S³.
fn omega_gradient_on_tangent(n_k: &Vector4<f64>, n_i: &Vector4<f64>) -> Vector4<f64> {
    let neg_j0_ni = -j0_apply(n_i);
    // Project to T_{n_k}S³: remove component along n_k
    neg_j0_ni - neg_j0_ni.dot(n_k) * n_k
}

/// Compute gradient dot products for all (facet, ridge-neighbor) pairs.
fn compute_gradient_dots(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    d_sys_n: &[Vector4<f64>],
    orbit_facets: &[usize],
) -> Vec<GradientDot> {
    let normals = polytope.normals_f64();
    let orbit_set: HashSet<usize> = orbit_facets.iter().copied().collect();

    // Build ridge-neighbor lookup: for each facet k, list of neighbors
    let f = polytope.facet_count();
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); f];
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        neighbors[i].push(j);
        neighbors[j].push(i);
    }

    let mut dots = Vec::new();
    for k in 0..f {
        let grad_sys = &d_sys_n[k];
        let grad_sys_norm = grad_sys.norm();

        for &i in &neighbors[k] {
            let grad_omega = omega_gradient_on_tangent(&normals[k], &normals[i]);
            let dot = grad_sys.dot(&grad_omega);
            let w = omega0(&normals[k], &normals[i]);

            dots.push(GradientDot {
                facet_k: k,
                neighbor_i: i,
                k_on_orbit: orbit_set.contains(&k),
                i_on_orbit: orbit_set.contains(&i),
                omega: w,
                dot,
                grad_sys_norm,
            });
        }
    }

    dots
}

// ============================================================================
// Physical orbit extraction
// ============================================================================

/// Extract orbit permutation and beta from instrumented result.
///
/// With the natural convention, permutations and beta are already in positive Reeb order.
fn physical_orbit(orbit: &ValidOrbit) -> (Vec<usize>, Vec<f64>) {
    (orbit.permutation.clone(), orbit.beta.clone())
}

// ============================================================================
// Main
// ============================================================================

fn process_polytope(
    polytope: &Polytope4D,
    source: &str,
) -> Option<OmegaRow> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let t0 = Instant::now();

    // Volume
    let vol = volume(polytope).ok()?;

    // Instrumented capacity (Phase B needs λ, ν)
    let instrumented = ehz_capacity_instrumented(polytope)?;
    let cap = instrumented.capacity;
    let sys = cap * cap / (2.0 * vol);

    let time_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Physical orbit
    let best_orbit = &instrumented.orbits[0];
    let (orbit_facets, orbit_betas) = physical_orbit(best_orbit);

    // Phase A: omega features
    let skeleton = Skeleton::compute(polytope);
    let (ridge_omegas, orbit_omegas) = compute_omega_features(polytope, &skeleton, &orbit_facets);

    let orbit_omega_min = orbit_omegas
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let orbit_omega_mean = if orbit_omegas.is_empty() {
        0.0
    } else {
        orbit_omegas.iter().sum::<f64>() / orbit_omegas.len() as f64
    };

    let ridge_omega_abs_min = ridge_omegas
        .iter()
        .map(|r| r[2].abs())
        .fold(f64::INFINITY, f64::min);

    // Sanity: orbit omegas should all be ≥ 0 (feasibility)
    let n_negative = orbit_omegas.iter().filter(|&&w| w < -1e-10).count();
    if n_negative > 0 {
        let worst = orbit_omegas.iter().cloned().fold(f64::INFINITY, f64::min);
        eprintln!(
            "WARNING: {}: {}/{} orbit omegas < 0 (worst: {:.6e})",
            source, n_negative, orbit_omegas.len(), worst
        );
    }

    // Phase B: gradient dots
    let d_sys_n = compute_d_sys_n(polytope, vol, cap, sys, &instrumented);
    let gradient_dots = compute_gradient_dots(polytope, &skeleton, &d_sys_n, &orbit_facets);

    Some(OmegaRow {
        source: source.to_string(),
        facet_count: f,
        normals: normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
        heights: heights.to_vec(),
        volume: vol,
        capacity: cap,
        sys,
        iterations: instrumented.iterations,
        time_ms,
        orbit_length: orbit_facets.len(),
        orbit_facets,
        orbit_betas,
        orbit_omegas,
        orbit_omega_min,
        orbit_omega_mean,
        ridge_omegas,
        ridge_omega_abs_min,
        n_ridges: skeleton.ridges.len(),
        gradient_dots,
    })
}

fn main() {
    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("omega-obstacle");
    let out_path = out_dir.join("omega-obstacle.jsonl");
    let file = std::fs::File::create(&out_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut total = 0usize;
    let mut failed = 0usize;

    eprintln!("=== Omega-obstacle experiment ===");
    eprintln!("Output: {}", out_path.display());

    // Random polytopes
    for &(f, n) in SAMPLING_PLAN {
        let t0 = Instant::now();
        let polytopes = symplectic::random::generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);
        eprintln!(
            "F={}: generated {} polytopes in {:.1}s",
            f,
            polytopes.len(),
            t0.elapsed().as_secs_f64()
        );

        for (idx, polytope) in polytopes.iter().enumerate() {
            let source = format!("random_F{}_{}", f, idx);
            match process_polytope(polytope, &source) {
                Some(row) => {
                    serde_json::to_writer(&mut writer, &row).unwrap();
                    writeln!(writer).unwrap();
                    total += 1;
                }
                None => {
                    eprintln!("  SKIP: {} (capacity computation failed)", source);
                    failed += 1;
                }
            }
        }
        eprintln!(
            "  F={}: processed in {:.1}s (total so far: {})",
            f,
            t0.elapsed().as_secs_f64(),
            total
        );
    }

    // HKO counterexample
    {
        let hko = known_polytopes::hko_pentagon();
        let source = "hko_pentagon";
        match process_polytope(&hko.polytope, source) {
            Some(row) => {
                serde_json::to_writer(&mut writer, &row).unwrap();
                writeln!(writer).unwrap();
                total += 1;
                eprintln!("HKO pentagon: sys = {:.6}", row.sys);
            }
            None => {
                eprintln!("WARNING: HKO pentagon capacity failed");
                failed += 1;
            }
        }
    }

    // Other known polytopes for reference (skip F > 10 — instrumented HK2017 is exponential)
    for kp in &[
        known_polytopes::simplex(),
        known_polytopes::hypercube(),
    ] {
        if kp.polytope.facet_count() > 10 {
            eprintln!("SKIP: {} (F={} > 10, too expensive for instrumented HK2017)",
                      kp.name, kp.polytope.facet_count());
            continue;
        }
        match process_polytope(&kp.polytope, kp.name) {
            Some(row) => {
                serde_json::to_writer(&mut writer, &row).unwrap();
                writeln!(writer).unwrap();
                total += 1;
            }
            None => {
                failed += 1;
            }
        }
    }

    writer.flush().unwrap();
    eprintln!(
        "\nDone: {} polytopes written, {} failed. Output: {}",
        total,
        failed,
        out_path.display()
    );
}
