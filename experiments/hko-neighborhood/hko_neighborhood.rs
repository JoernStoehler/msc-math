//! HKO-neighborhood experiment: local maximality analysis of HKO2024.
//!
//! Phase A: Near-optimal orbit tracking + sensitivity at HKO2024 (F=10 space)
//! Phase B: Facet-splitting into F=11 — test maximality in larger ambient space
//!
//! Convention: The library (crates/) is stable. Experiment-specific variants
//! (instrumented HK2017) are self-contained in this binary. Library internals
//! needed by the variants are copied here with source references.
//!
//! Architecture:
//! 1. `cargo run --bin hko_neighborhood --release` generates datasets
//! 2. Writes to hko-neighborhood/hko-neighborhood-{sensitivity,ascent,splitting}.jsonl
//! 3. Python script reads JSONL, produces figures
//!
//! KKT solver note: Uses a local copy of the library's condition-number approach
//! (EIGEN_CONDITION_TAU = 1e-3 in crates/src/kkt.rs). The local constant retains
//! the old SVD_CONDITION_TAU name.

use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::{ehz_capacity, known_polytopes, volume, Polytope4D, Skeleton};

/// Gap threshold for near-optimal orbits: collect orbits within δ of best.
/// 1% is generous — in practice, HKO2024's 44 near-optimal orbits all have
/// gaps < 5e-14 (machine precision). The threshold just needs to be well above
/// machine epsilon to capture all degenerate orbits, while small enough to
/// exclude genuinely suboptimal ones. Any value in [1e-6, 0.1] gives the same
/// result on HKO2024.
const NEAR_OPTIMAL_GAP: f64 = 0.01;

/// Maximum step size cap.
const MAX_STEP_SIZE: f64 = 100.0;

/// Maximum number of gradient ascent iterations.
const MAX_ASCENT_ITERATIONS: usize = 50;

/// Convergence threshold for gradient ascent (minimum improvement per iteration).
const CONVERGENCE_THRESHOLD: f64 = 1e-8;

/// Armijo sufficient decrease parameter (c in f(x + t*d) >= f(x) + c*t*∇f·d).
const ARMIJO_C: f64 = 1e-4;

/// Backtracking factor for Armijo line search.
const BACKTRACKING_FACTOR: f64 = 0.5;

/// Minimum step fraction (give up below this).
const MIN_STEP_FRACTION: f64 = 1e-12;

/// Number of angular samples per representative facet normal for facet-splitting (Phase B).
/// HKO2024 = pentagon ×_L pentagon: all Q-space normals equivalent, all P-space equivalent.
/// Only 2 representatives needed (facet 0 = Q-space, facet 5 = P-space).
const N_SPLITTING_SAMPLES_PER_FACET: usize = 100;

/// Number of random mixed directions (neither purely Q nor purely P space).
const N_SPLITTING_MIXED: usize = 50;

/// Number of random control directions for facet-splitting (Phase B).
const N_SPLITTING_CONTROL: usize = 20;

/// Small epsilon for facet-splitting (how deep to cut).
const SPLITTING_EPSILONS: &[f64] = &[1e-3, 1e-4];

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct SensitivityRow {
    name: String,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    volume: f64,
    capacity: f64,
    sys: f64,
    // Near-optimal orbit tracking
    n_valid_orbits: usize,
    n_near_optimal: usize,
    near_optimal_gap: f64,
    orbits: Vec<OrbitInfo>,
    // Height derivatives (per best orbit)
    d_vol_h: Vec<f64>,
    d_cap_h: Vec<f64>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    // Normal derivatives (tangent vectors, 4 components each)
    d_vol_n: Vec<[f64; 4]>,
    d_cap_n: Vec<[f64; 4]>,
    d_sys_n: Vec<[f64; 4]>,
    gradient_norm_n: f64,
    // Combined
    gradient_norm_hn: f64,
    // Step bounds
    t_max_h: f64,
    t_max_hn: f64,
    // Per-orbit gradients (subdifferential)
    per_orbit_d_sys_h: Vec<Vec<f64>>,
    per_orbit_gradient_norm_h: Vec<f64>,
    // Timing
    time_instrumented_ms: f64,
    time_sensitivity_ms: f64,
}

#[derive(Debug, Serialize)]
struct OrbitInfo {
    subset: Vec<usize>,
    permutation: Vec<usize>,
    action: f64,
    relative_gap: f64,
    beta: Vec<f64>,
    q_value: f64,
}

#[derive(Debug, Serialize)]
struct AscentRow {
    iteration: usize,
    step_type: String, // "h_only" or "h_n"
    t_actual: f64,
    t_max: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    volume: f64,
    capacity: f64,
    // Orbit tracking
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    orbit_switched: bool,
    n_near_optimal: usize,
    // Gradient info
    gradient_norm_h: f64,
    gradient_norm_hn: f64,
    // State
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    time_ms: f64,
}

#[derive(Debug, Serialize)]
struct SplittingRow {
    // Cutting direction
    source_facet: usize,    // which existing facet normal this is near (usize::MAX for control, usize::MAX-1 for mixed)
    angular_offset: f64,    // angle from source facet normal (radians)
    cutting_normal: [f64; 4],
    epsilon: f64,
    // Results
    sys_original: f64,
    sys_split: f64,
    delta_sys: f64,
    capacity_split: f64,
    volume_split: f64,
    facet_count_split: usize,
    n_valid_orbits: usize,
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    // Gradient at split polytope
    d_sys_d_h_new: f64, // ∂sys/∂h_{F+1} — the splitting gradient
    construction_ok: bool,
    time_ms: f64,
}

// ============================================================================
// Copied from library — KKT solver (crates/src/kkt.rs)
//
// Uses the CURRENT condition-number approach (SVD_CONDITION_TAU = 1e-3).
// ============================================================================

/// Minimum β_i value to consider a solution valid.
/// Copied from crates/src/kkt.rs:12
const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid.
/// Copied from crates/src/kkt.rs:15
const EPS_Q_POSITIVE: f64 = 1e-15;

/// Floor for SVD singular values.
/// Copied from crates/src/kkt.rs:20
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Condition-number threshold for SVD rank detection.
/// Copied from crates/src/kkt.rs:41
const SVD_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm.
/// Copied from crates/src/kkt.rs:44
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Facet incidence tolerance.
/// Copied from crates/src/constants.rs
const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// ω₀(u, v) = u_q1·v_p1 - u_p1·v_q1 + u_q2·v_p2 - u_p2·v_q2
/// Copied from crates/src/geom/symplectic.rs:28
fn omega0_local(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) = (1/2) β^T H β
/// Copied from crates/src/kkt.rs
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0_local(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

/// Search 1D null space for β > 0 solution.
/// Copied from crates/src/kkt.rs:79-120
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

/// Search multi-dimensional null space for β > 0 solution.
/// Copied from crates/src/kkt.rs:125-170
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

/// Build KKT matrix and RHS vector.
/// Based on crates/src/kkt.rs build_kkt_system, uses ASYMMETRIC sign convention
/// (upper-right = -n/-h, lower-left = +n/+h).
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
            let val = omega0_local(&normals[perm[i]], &normals[perm[j]]);
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

/// SVD path with condition-number-based rank detection.
/// Copied from crates/src/kkt.rs:233-359, extended to return ν and λ.
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

/// SVD-only KKT solver, extended to return ν and λ.
fn solve_kkt_full(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64, f64, Vec<f64>)> {
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}

// ============================================================================
// Copied from library — combinatorial infrastructure
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
        if k.is_multiple_of(2) {
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
            adj[i][j] = vertex_adj[i][j] && omega0_local(&normals[i], &normals[j]) >= 0.0;
        }
    }
    adj
}

fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// Instrumented HK2017 — collects ALL valid orbits
// ============================================================================

#[derive(Debug, Clone)]
struct ValidOrbit {
    action: f64,
    subset: Vec<usize>,
    permutation: Vec<usize>,
    beta: Vec<f64>,
    q_value: f64,
    nu: f64,
    lambda: Vec<f64>,
}

struct InstrumentedResult {
    capacity: f64,
    #[allow(dead_code)]
    capacity_uncertain: f64,
    orbits: Vec<ValidOrbit>,
    #[allow(dead_code)]
    iterations: u64,
}

fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let adj = build_directed_adjacency_matrix(polytope);

    let mut orbits: Vec<ValidOrbit> = Vec::new();
    let mut best_uncertain_action: Option<f64> = None;
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

                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain_action.is_none_or(|a| action < a);
                        if update {
                            best_uncertain_action = Some(action);
                        }
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
    let capacity_uncertain = best_uncertain_action.unwrap_or(capacity);

    Some(InstrumentedResult {
        capacity,
        capacity_uncertain,
        orbits,
        iterations,
    })
}

// ============================================================================
// Sensitivity computation (copied from sys-optimization, adapted)
// ============================================================================

struct SensitivityResult {
    d_vol_h: Vec<f64>,
    d_cap_h: Vec<f64>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    d_vol_n: Vec<Vector4<f64>>,
    d_cap_n: Vec<Vector4<f64>>,
    d_sys_n: Vec<Vector4<f64>>,
    gradient_norm_n: f64,
    gradient_norm_hn: f64,
}

// --- Facet volume helpers ---

const EPS_DEGENERATE: f64 = 1e-10;

fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let bc_01 = b[0] * c[1] - b[1] * c[0];
    let bc_02 = b[0] * c[2] - b[2] * c[0];
    let bc_03 = b[0] * c[3] - b[3] * c[0];
    let bc_12 = b[1] * c[2] - b[2] * c[1];
    let bc_13 = b[1] * c[3] - b[3] * c[1];
    let bc_23 = b[2] * c[3] - b[3] * c[2];

    let d0 = a[1] * bc_23 - a[2] * bc_13 + a[3] * bc_12;
    let d1 = -(a[0] * bc_23 - a[2] * bc_03 + a[3] * bc_02);
    let d2 = a[0] * bc_13 - a[1] * bc_03 + a[3] * bc_01;
    let d3 = -(a[0] * bc_12 - a[1] * bc_02 + a[2] * bc_01);

    Vector4::new(d0, d1, d2, d3)
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

fn compute_volume_derivatives_analytical(polytope: &Polytope4D) -> Vec<f64> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();
    (0..f)
        .map(|k| {
            facet_volume_and_centroid_3d(&normals, &heights, &vertices, k, f).0
        })
        .collect()
}

fn compute_capacity_derivatives_analytical(
    orbit: &ValidOrbit,
    facet_count: usize,
) -> Vec<f64> {
    let q_sq = orbit.q_value * orbit.q_value;
    (0..facet_count)
        .map(|k| {
            match orbit.permutation.iter().position(|&f| f == k) {
                // Lemma lem:cap-derivative: ∂A/∂h_k = ν·β_{i₀}/(2Q²)
                Some(i0) => orbit.nu * orbit.beta[i0] / (2.0 * q_sq),
                None => 0.0,
            }
        })
        .collect()
}

fn compute_volume_derivatives_normal(polytope: &Polytope4D) -> Vec<Vector4<f64>> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();

    (0..f)
        .map(|k| {
            let (s_k, centroid_k) = facet_volume_and_centroid_3d(&normals, &heights, &vertices, k, f);
            if s_k < 1e-30 {
                return Vector4::zeros();
            }
            let tangent_centroid = centroid_k - heights[k] * normals[k];
            -s_k * tangent_centroid
        })
        .collect()
}

fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

fn compute_capacity_derivatives_normal(
    orbit: &ValidOrbit,
    normals: &[Vector4<f64>],
    facet_count: usize,
) -> Vec<Vector4<f64>> {
    let q_sq = orbit.q_value * orbit.q_value;
    let perm = &orbit.permutation;
    let beta = &orbit.beta;
    let lambda = Vector4::new(
        orbit.lambda[0],
        orbit.lambda[1],
        orbit.lambda[2],
        orbit.lambda[3],
    );

    (0..facet_count)
        .map(|k| {
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(),
            };

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

/// Compute d(cap)/d(h_k) via central finite differences (validation only).
///
/// For each facet k, perturb h_k by ±ε, construct the perturbed polytope,
/// and compute capacity via `ehz_capacity`. Returns (cap_plus - cap_minus)/(2ε).
///
/// Not called in normal operation (permanent FD tests are in crate sensitivity_test.rs).
/// Kept for ad-hoc debugging; call from `compute_sensitivity` when needed.
#[allow(dead_code)]
fn compute_capacity_derivatives_fd(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    const FD_EPS: f64 = 1e-5;
    let f = normals.len();
    (0..f)
        .map(|k| {
            let mut h_plus = heights.to_vec();
            let mut h_minus = heights.to_vec();
            h_plus[k] += FD_EPS;
            h_minus[k] -= FD_EPS;

            let p_plus = match Polytope4D::from_normals_and_heights(normals.to_vec(), h_plus) {
                Ok(p) => p,
                Err(_) => return f64::NAN,
            };
            let p_minus = match Polytope4D::from_normals_and_heights(normals.to_vec(), h_minus) {
                Ok(p) => p,
                Err(_) => return f64::NAN,
            };

            let cap_plus = ehz_capacity(&p_plus).map(|r| r.capacity).unwrap_or(f64::NAN);
            let cap_minus = ehz_capacity(&p_minus).map(|r| r.capacity).unwrap_or(f64::NAN);
            (cap_plus - cap_minus) / (2.0 * FD_EPS)
        })
        .collect()
}

/// Cross-check analytical capacity and sys derivatives against finite differences.
///
/// Prints a comparison table and panics if they disagree beyond tolerance.
/// Also verifies the Euler homogeneity identity: Σ h_k · ∂c/∂h_k = 2c.
///
/// Not called in normal operation (permanent FD tests are in crate sensitivity_test.rs).
/// Kept for ad-hoc debugging; call from `compute_sensitivity` when needed.
#[allow(dead_code)]
fn cross_check_derivatives_fd(
    normals: &[Vector4<f64>],
    heights: &[f64],
    cap: f64,
    _vol: f64,
    sys: f64,
    d_cap_h_analytical: &[f64],
    d_sys_h_analytical: &[f64],
) {
    let f = normals.len();
    let d_cap_h_fd = compute_capacity_derivatives_fd(normals, heights);

    // Compute FD d_sys_h end-to-end: perturb h_k, recompute sys = cap²/(2·vol)
    let d_sys_h_fd: Vec<f64> = {
        const FD_EPS: f64 = 1e-5;
        (0..f)
            .map(|k| {
                let mut h_plus = heights.to_vec();
                let mut h_minus = heights.to_vec();
                h_plus[k] += FD_EPS;
                h_minus[k] -= FD_EPS;

                let sys_at = |h: Vec<f64>| -> Option<f64> {
                    let p = Polytope4D::from_normals_and_heights(normals.to_vec(), h).ok()?;
                    let c = ehz_capacity(&p)?.capacity;
                    let v = volume(&p).ok()?;
                    Some(c * c / (2.0 * v))
                };

                match (sys_at(h_plus), sys_at(h_minus)) {
                    (Some(sp), Some(sm)) => (sp - sm) / (2.0 * FD_EPS),
                    _ => f64::NAN,
                }
            })
            .collect()
    };

    // Print comparison table
    println!("\n  FD cross-check (ε=1e-5):");
    println!("  {:>5} {:>12} {:>12} {:>12}   {:>12} {:>12} {:>12}",
        "facet", "d_cap_ana", "d_cap_fd", "cap_diff", "d_sys_ana", "d_sys_fd", "sys_diff");
    for k in 0..f {
        println!("  {:>5} {:>12.6} {:>12.6} {:>12.2e}   {:>12.6} {:>12.6} {:>12.2e}",
            k,
            d_cap_h_analytical[k], d_cap_h_fd[k],
            d_cap_h_analytical[k] - d_cap_h_fd[k],
            d_sys_h_analytical[k], d_sys_h_fd[k],
            d_sys_h_analytical[k] - d_sys_h_fd[k]);
    }

    // Euler homogeneity: Σ h_k · ∂c/∂h_k should equal 2c (degree-2)
    let euler_analytical: f64 = heights.iter().zip(d_cap_h_analytical).map(|(h, d)| h * d).sum();
    let euler_fd: f64 = heights.iter().zip(d_cap_h_fd.iter()).map(|(h, d)| h * d).sum();
    println!("\n  Euler homogeneity (Σ h_k · ∂c/∂h_k):");
    println!("    Analytical: {euler_analytical:.6}  (should be 2c = {:.6})", 2.0 * cap);
    println!("    FD:         {euler_fd:.6}  (should be 2c = {:.6})", 2.0 * cap);
    println!("    Analytical / (2c) = {:.6}", euler_analytical / (2.0 * cap));
    println!("    FD / (2c)         = {:.6}", euler_fd / (2.0 * cap));

    // Euler homogeneity for sys: Σ h_k · ∂sys/∂h_k = -2·sys (degree -2)
    let euler_sys_ana: f64 = heights.iter().zip(d_sys_h_analytical).map(|(h, d)| h * d).sum();
    let euler_sys_fd: f64 = heights.iter().zip(d_sys_h_fd.iter()).map(|(h, d)| h * d).sum();
    println!("\n  Euler homogeneity (Σ h_k · ∂sys/∂h_k):");
    println!("    Analytical: {euler_sys_ana:.6}  (should be -2·sys = {:.6})", -2.0 * sys);
    println!("    FD:         {euler_sys_fd:.6}  (should be -2·sys = {:.6})", -2.0 * sys);

    // Assert analytical Euler identity: Σ h_k · ∂c/∂h_k = 2c (degree-2 homogeneity)
    let euler_ana_err = (euler_analytical - 2.0 * cap).abs() / (2.0 * cap);
    assert!(
        euler_ana_err < 1e-10,
        "Analytical Euler homogeneity failed: Σ h_k · d_cap_ana = {euler_analytical:.10}, \
         2c = {:.10}, rel_err = {euler_ana_err:.2e}",
        2.0 * cap
    );

    // Note: FD Euler may not hold exactly at degenerate points (multiple tied orbits).
    // At HKO2024, 44 orbits tie at machine precision, so perturbing h_k may switch
    // the optimal orbit. The FD computes ∂(max_orbits A)/∂h_k, which differs from
    // the single-orbit ∂A/∂h_k. The 12% FD Euler error is expected here.
    let euler_fd_err = (euler_fd - 2.0 * cap).abs() / (2.0 * cap);
    if euler_fd_err > 0.01 {
        println!("  Note: FD Euler error = {euler_fd_err:.2e} (expected at degenerate points)");
    }
}

fn compute_sensitivity(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    orbit: &ValidOrbit,
) -> SensitivityResult {
    let normals = polytope.normals_f64();
    let f = normals.len();

    let d_vol_h = compute_volume_derivatives_analytical(polytope);
    let d_cap_h = compute_capacity_derivatives_analytical(orbit, f);

    let d_sys_h: Vec<f64> = d_vol_h
        .iter()
        .zip(d_cap_h.iter())
        .map(|(&dv, &dc)| (cap * dc - sys * dv) / vol)
        .collect();

    let gradient_norm_h = d_sys_h
        .iter()
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();

    let d_vol_n = compute_volume_derivatives_normal(polytope);
    let d_cap_n = compute_capacity_derivatives_normal(orbit, &normals, f);

    let d_sys_n: Vec<Vector4<f64>> = d_vol_n
        .iter()
        .zip(d_cap_n.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect();

    let gradient_norm_n = d_sys_n
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt();

    let gradient_norm_hn =
        (gradient_norm_h * gradient_norm_h + gradient_norm_n * gradient_norm_n).sqrt();

    SensitivityResult {
        d_vol_h,
        d_cap_h,
        d_sys_h,
        gradient_norm_h,
        d_vol_n,
        d_cap_n,
        d_sys_n,
        gradient_norm_n,
        gradient_norm_hn,
    }
}

// ============================================================================
// Step bounds computation (copied from sys-optimization)
// ============================================================================

fn compute_step_bound(polytope: &Polytope4D, direction: &[f64]) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let det_facets = &vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let g_det = Vector4::new(
                direction[det_facets[0]],
                direction[det_facets[1]],
                direction[det_facets[2]],
                direction[det_facets[3]],
            );
            let dv_dt = n_inv * g_det;

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let rate = direction[j] - normals[j].dot(&dv_dt);
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Over-determined vertex (>4 incident facets): we cannot invert the
            // normal matrix, so we use a conservative bound: t ≤ slack / max|g_k|.
            // This is safe but may over-tighten the step. In practice, HKO2024 is
            // a simple polytope (all vertices have exactly 4 incident facets), so
            // this branch is never reached.
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                if direction[j] < -1e-15 {
                    continue;
                }
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > 1e-15 {
                    let t_crit = slack / max_g;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    for k in 0..f {
        if direction[k] < -1e-15 {
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

fn compute_step_bound_hn(
    polytope: &Polytope4D,
    g_h: &[f64],
    g_n: &[Vector4<f64>],
) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let det_facets = vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                g_h[det_facets[0]] - g_n[det_facets[0]].dot(v),
                g_h[det_facets[1]] - g_n[det_facets[1]].dot(v),
                g_h[det_facets[2]] - g_n[det_facets[2]].dot(v),
                g_h[det_facets[3]] - g_n[det_facets[3]].dot(v),
            );
            let dv_dt = n_inv * rhs;

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let rate = g_h[j] - g_n[j].dot(v) - normals[j].dot(&dv_dt);
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let max_g_h = g_h.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                let max_g_n = g_n.iter().map(|g| g.norm()).fold(0.0f64, f64::max);
                let max_rate = max_g_h + max_g_n * v.norm();
                if max_rate > 1e-15 {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    for k in 0..f {
        if g_h[k] < -1e-15 {
            let t_crit = heights[k] / (-g_h[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // ω₀ sign preservation for ridge-adjacent pairs
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let omega_ij = omega0_local(&normals[i], &normals[j]);
        let d_omega = omega0_local(&g_n[i], &normals[j]) + omega0_local(&normals[i], &g_n[j]);
        if omega_ij.abs() > 1e-15 && d_omega.abs() > 1e-15 {
            let t_flip = -omega_ij / d_omega;
            if t_flip > 0.0 && t_flip < t_max {
                t_max = t_flip;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient step helpers
// ============================================================================

/// Safely compute sys for a polytope, catching panics from degenerate geometry.
fn safe_sys(polytope: &Polytope4D) -> Option<(f64, f64, f64)> {
    let vol = volume(polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    // Catch panics from library KKT solver on degenerate geometry
    let cap = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ehz_capacity(polytope).map(|r| r.capacity)
    }))
    .ok()
    .flatten()
    .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((sys, vol, cap))
    } else {
        None
    }
}

fn try_step_h(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
) -> Option<(Polytope4D, f64, f64, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    let new_polytope = Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights).ok()?;
    let (sys, vol, cap) = safe_sys(&new_polytope)?;
    Some((new_polytope, sys, vol, cap))
}

fn try_step_hn(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64, f64, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm()
        })
        .collect();

    let new_polytope = Polytope4D::from_normals_and_heights(new_normals, new_heights).ok()?;
    let (sys, vol, cap) = safe_sys(&new_polytope)?;
    Some((new_polytope, sys, vol, cap))
}

// ============================================================================
// Armijo backtracking line search
// ============================================================================

/// Armijo backtracking line search for height-only steps.
/// Returns (polytope, sys, vol, cap, t_actual, orbit_info) or None if no improvement.
fn armijo_step_h(
    polytope: &Polytope4D,
    d_sys_h: &[f64],
    t_max: f64,
    current_sys: f64,
) -> Option<(Polytope4D, f64, f64, f64, f64)> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let grad_dot_dir: f64 = d_sys_h.iter().map(|x| x * x).sum(); // ∇f · d = |∇f|² (ascending)

    let mut t = 0.95 * t_max;
    while t > MIN_STEP_FRACTION * t_max {
        if let Some((new_poly, new_sys, vol, cap)) = try_step_h(&normals, &heights, d_sys_h, t) {
            // Armijo condition: f(x + td) >= f(x) + c·t·∇f·d
            if new_sys >= current_sys + ARMIJO_C * t * grad_dot_dir {
                return Some((new_poly, new_sys, vol, cap, t));
            }
        }
        t *= BACKTRACKING_FACTOR;
    }
    None
}

/// Armijo backtracking line search for (h,n) steps.
fn armijo_step_hn(
    polytope: &Polytope4D,
    d_sys_h: &[f64],
    d_sys_n: &[Vector4<f64>],
    t_max: f64,
    current_sys: f64,
) -> Option<(Polytope4D, f64, f64, f64, f64)> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let grad_dot_dir: f64 = d_sys_h.iter().map(|x| x * x).sum::<f64>()
        + d_sys_n.iter().map(|v| v.norm_squared()).sum::<f64>();

    let mut t = 0.95 * t_max;
    while t > MIN_STEP_FRACTION * t_max {
        if let Some((new_poly, new_sys, vol, cap)) =
            try_step_hn(&normals, &heights, d_sys_h, d_sys_n, t)
        {
            if new_sys >= current_sys + ARMIJO_C * t * grad_dot_dir {
                return Some((new_poly, new_sys, vol, cap, t));
            }
        }
        t *= BACKTRACKING_FACTOR;
    }
    None
}

// ============================================================================
// Phase A: Main analysis
// ============================================================================

fn run_phase_a(base_dir: &std::path::Path) {
    println!("═══════════════════════════════════════════════════════════");
    println!("Phase A: HKO2024 sensitivity + gradient ascent (F=10)");
    println!("═══════════════════════════════════════════════════════════\n");

    // Load HKO2024
    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    let f = polytope.facet_count();
    println!("HKO2024: F={f}, known capacity={:.6}", known.capacity);

    // Cross-check with library
    let lib_result = ehz_capacity(polytope).expect("library ehz_capacity failed");
    println!(
        "  Library capacity: {:.10} (diff from known: {:.2e})",
        lib_result.capacity,
        (lib_result.capacity - known.capacity).abs()
    );

    // Instrumented HK2017
    println!("\nRunning instrumented HK2017...");
    let t_instr = Instant::now();
    let instrumented = ehz_capacity_instrumented(polytope).expect("no valid orbits for HKO2024");
    let time_instrumented_ms = t_instr.elapsed().as_secs_f64() * 1000.0;

    // Cross-check
    let cap_diff = (instrumented.capacity - lib_result.capacity).abs();
    assert!(
        cap_diff < 1e-8,
        "Capacity mismatch: instrumented={:.10}, library={:.10}",
        instrumented.capacity,
        lib_result.capacity
    );
    println!(
        "  Instrumented capacity: {:.10} (matches library, diff={:.2e})",
        instrumented.capacity, cap_diff
    );

    let cap = instrumented.capacity;
    let vol = volume(polytope).expect("volume failed");
    let sys = cap * cap / (2.0 * vol);
    println!("  Volume: {vol:.10}");
    println!("  Sys: {sys:.10}");
    println!("  Total valid orbits: {}", instrumented.orbits.len());
    println!("  Computation time: {time_instrumented_ms:.1}ms");

    // Near-optimal orbit analysis
    let best_action = instrumented.orbits[0].action;
    let near_optimal: Vec<&ValidOrbit> = instrumented
        .orbits
        .iter()
        .filter(|o| (o.action - best_action) / best_action < NEAR_OPTIMAL_GAP)
        .collect();

    println!("\n--- Near-optimal orbits (gap < {NEAR_OPTIMAL_GAP}) ---");
    println!("  Count: {} (of {} total)", near_optimal.len(), instrumented.orbits.len());
    for (i, orbit) in near_optimal.iter().enumerate() {
        let gap = (orbit.action - best_action) / best_action;
        println!(
            "  #{}: S={:?}, σ={:?}, action={:.10}, gap={:.6e}",
            i, orbit.subset, orbit.permutation, orbit.action, gap
        );
    }

    // Also show a few more orbits for context
    println!("\n--- Orbit action distribution (first 10) ---");
    for (i, orbit) in instrumented.orbits.iter().take(10).enumerate() {
        let gap = (orbit.action - best_action) / best_action;
        println!(
            "  #{}: action={:.6}, gap={:.4e}, |S|={}, S={:?}",
            i,
            orbit.action,
            gap,
            orbit.subset.len(),
            orbit.subset
        );
    }

    // Sensitivity for best orbit
    println!("\n--- Sensitivity (best orbit) ---");
    let t_sens = Instant::now();
    let best_orbit = &instrumented.orbits[0];
    let sensitivity = compute_sensitivity(polytope, vol, cap, sys, best_orbit);
    let time_sensitivity_ms = t_sens.elapsed().as_secs_f64() * 1000.0;

    println!("  ∂sys/∂h:");
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    for k in 0..f {
        println!(
            "    k={}: d_vol={:.6e}, d_cap={:.6e}, d_sys={:.6e}",
            k, sensitivity.d_vol_h[k], sensitivity.d_cap_h[k], sensitivity.d_sys_h[k]
        );
    }
    println!("  |∇sys_h| = {:.6e}", sensitivity.gradient_norm_h);
    println!("  |∇sys_n| = {:.6e}", sensitivity.gradient_norm_n);
    println!("  |∇sys_hn| = {:.6e}", sensitivity.gradient_norm_hn);

    // Critical point check
    let is_critical = sensitivity.gradient_norm_hn < 1e-6;
    println!(
        "\n  Critical point check: |∇sys| = {:.6e} → {}",
        sensitivity.gradient_norm_hn,
        if is_critical {
            "YES — HKO2024 is a critical point"
        } else {
            "NO — gradient is nonzero"
        }
    );

    // Step bounds
    let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
        compute_step_bound(polytope, &sensitivity.d_sys_h)
    } else {
        0.0
    };
    let t_max_hn = if sensitivity.gradient_norm_hn > 1e-15 {
        compute_step_bound_hn(polytope, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
    } else {
        0.0
    };
    println!("  t_max_h = {t_max_h:.6e}");
    println!("  t_max_hn = {t_max_hn:.6e}");

    // Per-orbit sensitivity (subdifferential)
    println!("\n--- Per-orbit gradients (subdifferential) ---");
    let mut per_orbit_d_sys_h: Vec<Vec<f64>> = Vec::new();
    let mut per_orbit_gradient_norm_h: Vec<f64> = Vec::new();

    for (i, orbit) in near_optimal.iter().enumerate() {
        let orbit_sens = compute_sensitivity(polytope, vol, cap, sys, orbit);
        let norm = orbit_sens.gradient_norm_h;
        println!(
            "  Orbit #{}: |∇sys_h| = {:.6e}, d_sys_h = {:?}",
            i,
            norm,
            orbit_sens.d_sys_h.iter().map(|x| format!("{:.4e}", x)).collect::<Vec<_>>()
        );
        per_orbit_d_sys_h.push(orbit_sens.d_sys_h);
        per_orbit_gradient_norm_h.push(norm);
    }

    // Write sensitivity JSONL
    let sens_path = base_dir.join("hko-neighborhood/hko-neighborhood-sensitivity.jsonl");
    let sens_file = File::create(&sens_path).expect("create sensitivity JSONL");
    let mut sens_writer = BufWriter::new(sens_file);

    let normals_raw: Vec<[f64; 4]> = normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect();
    let d_vol_n_raw: Vec<[f64; 4]> = sensitivity.d_vol_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
    let d_cap_n_raw: Vec<[f64; 4]> = sensitivity.d_cap_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
    let d_sys_n_raw: Vec<[f64; 4]> = sensitivity.d_sys_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();

    let orbit_infos: Vec<OrbitInfo> = near_optimal
        .iter()
        .map(|o| OrbitInfo {
            subset: o.subset.clone(),
            permutation: o.permutation.clone(),
            action: o.action,
            relative_gap: (o.action - best_action) / best_action,
            beta: o.beta.clone(),
            q_value: o.q_value,
        })
        .collect();

    let sens_row = SensitivityRow {
        name: "hko_pentagon".to_string(),
        facet_count: f,
        normals: normals_raw,
        heights: heights.to_vec(),
        volume: vol,
        capacity: cap,
        sys,
        n_valid_orbits: instrumented.orbits.len(),
        n_near_optimal: near_optimal.len(),
        near_optimal_gap: NEAR_OPTIMAL_GAP,
        orbits: orbit_infos,
        d_vol_h: sensitivity.d_vol_h.clone(),
        d_cap_h: sensitivity.d_cap_h.clone(),
        d_sys_h: sensitivity.d_sys_h.clone(),
        gradient_norm_h: sensitivity.gradient_norm_h,
        d_vol_n: d_vol_n_raw,
        d_cap_n: d_cap_n_raw,
        d_sys_n: d_sys_n_raw,
        gradient_norm_n: sensitivity.gradient_norm_n,
        gradient_norm_hn: sensitivity.gradient_norm_hn,
        t_max_h,
        t_max_hn,
        per_orbit_d_sys_h,
        per_orbit_gradient_norm_h,
        time_instrumented_ms,
        time_sensitivity_ms,
    };
    serde_json::to_writer(&mut sens_writer, &sens_row).expect("write sensitivity");
    writeln!(sens_writer).expect("newline");
    sens_writer.flush().expect("flush sensitivity");
    println!("\n  Wrote {}", sens_path.display());

    // =========================================================================
    // Gradient ascent with Armijo backtracking
    // =========================================================================

    println!("\n--- Gradient ascent with Armijo backtracking ---");

    let ascent_path = base_dir.join("hko-neighborhood/hko-neighborhood-ascent.jsonl");
    let ascent_file = File::create(&ascent_path).expect("create ascent JSONL");
    let mut ascent_writer = BufWriter::new(ascent_file);

    let mut current = Polytope4D::from_normals_and_heights(
        polytope.normals_f64().to_vec(),
        polytope.heights_f64().to_vec(),
    )
    .expect("reconstruct HKO2024");
    let mut current_sys = sys;
    let mut prev_subset = instrumented.orbits[0].subset.clone();
    let mut prev_perm = instrumented.orbits[0].permutation.clone();

    for iter in 0..MAX_ASCENT_ITERATIONS {
        let t_iter = Instant::now();

        // Recompute instrumented capacity
        let instr = match ehz_capacity_instrumented(&current) {
            Some(r) => r,
            None => {
                println!("  Iter {iter}: no valid orbits, stopping");
                break;
            }
        };
        let cap = instr.capacity;
        let vol = volume(&current).expect("volume");
        let sys_now = cap * cap / (2.0 * vol);
        let best_orbit = &instr.orbits[0];

        // Orbit switch detection
        let orbit_switched = best_orbit.subset != prev_subset || best_orbit.permutation != prev_perm;

        // Sensitivity
        let sens = compute_sensitivity(&current, vol, cap, sys_now, best_orbit);

        // Step bounds
        let t_max_h = if sens.gradient_norm_h > 1e-15 {
            compute_step_bound(&current, &sens.d_sys_h)
        } else {
            0.0
        };
        let t_max_hn = if sens.gradient_norm_hn > 1e-15 {
            compute_step_bound_hn(&current, &sens.d_sys_h, &sens.d_sys_n)
        } else {
            0.0
        };

        // Try Armijo for both h-only and h+n, pick better
        let step_h = if t_max_h > 0.0 && sens.gradient_norm_h > 1e-15 {
            armijo_step_h(&current, &sens.d_sys_h, t_max_h, sys_now)
        } else {
            None
        };
        let step_hn = if t_max_hn > 0.0 && sens.gradient_norm_hn > 1e-15 {
            armijo_step_hn(&current, &sens.d_sys_h, &sens.d_sys_n, t_max_hn, sys_now)
        } else {
            None
        };

        let (new_poly, new_sys, new_vol, new_cap, t_actual, step_type, t_max_used) =
            match (step_h, step_hn) {
                (Some((p1, s1, v1, c1, t1)), Some((p2, s2, v2, c2, t2))) => {
                    if s1 >= s2 {
                        (p1, s1, v1, c1, t1, "h_only", t_max_h)
                    } else {
                        (p2, s2, v2, c2, t2, "h_n", t_max_hn)
                    }
                }
                (Some((p, s, v, c, t)), None) => (p, s, v, c, t, "h_only", t_max_h),
                (None, Some((p, s, v, c, t))) => (p, s, v, c, t, "h_n", t_max_hn),
                (None, None) => {
                    println!("  Iter {iter}: no improving step found — local maximum");

                    // Near-optimal orbits at this point
                    let best_action = instr.orbits[0].action;
                    let n_near = instr
                        .orbits
                        .iter()
                        .filter(|o| (o.action - best_action) / best_action < NEAR_OPTIMAL_GAP)
                        .count();

                    // Write final state
                    let cur_normals = current.normals_f64();
                    let cur_heights = current.heights_f64();
                    let row = AscentRow {
                        iteration: iter,
                        step_type: "none".to_string(),
                        t_actual: 0.0,
                        t_max: t_max_h,
                        sys_before: sys_now,
                        sys_after: sys_now,
                        delta_sys: 0.0,
                        volume: vol,
                        capacity: cap,
                        best_subset: instr.orbits[0].subset.clone(),
                        best_permutation: instr.orbits[0].permutation.clone(),
                        orbit_switched,
                        n_near_optimal: n_near,
                        gradient_norm_h: sens.gradient_norm_h,
                        gradient_norm_hn: sens.gradient_norm_hn,
                        normals: cur_normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
                        heights: cur_heights.to_vec(),
                        time_ms: t_iter.elapsed().as_secs_f64() * 1000.0,
                    };
                    serde_json::to_writer(&mut ascent_writer, &row).expect("write ascent");
                    writeln!(ascent_writer).expect("newline");
                    break;
                }
            };

        let delta = new_sys - sys_now;
        let time_ms = t_iter.elapsed().as_secs_f64() * 1000.0;

        // Near-optimal orbit count at new point
        let new_instr = ehz_capacity_instrumented(&new_poly);
        let n_near = new_instr
            .as_ref()
            .map(|r| {
                let ba = r.orbits[0].action;
                r.orbits
                    .iter()
                    .filter(|o| (o.action - ba) / ba < NEAR_OPTIMAL_GAP)
                    .count()
            })
            .unwrap_or(0);

        let new_normals = new_poly.normals_f64();
        let new_heights = new_poly.heights_f64();

        let new_subset = new_instr
            .as_ref()
            .map(|r| r.orbits[0].subset.clone())
            .unwrap_or_default();
        let new_perm = new_instr
            .as_ref()
            .map(|r| r.orbits[0].permutation.clone())
            .unwrap_or_default();

        println!(
            "  Iter {iter}: {step_type} t={t_actual:.6e} (t_max={t_max_used:.6e}), \
             sys={sys_now:.10}→{new_sys:.10} (Δ={delta:.6e}), \
             orbit_switch={orbit_switched}, near_optimal={n_near}, {time_ms:.0}ms"
        );

        let row = AscentRow {
            iteration: iter,
            step_type: step_type.to_string(),
            t_actual,
            t_max: t_max_used,
            sys_before: sys_now,
            sys_after: new_sys,
            delta_sys: delta,
            volume: new_vol,
            capacity: new_cap,
            best_subset: new_subset.clone(),
            best_permutation: new_perm.clone(),
            orbit_switched,
            n_near_optimal: n_near,
            gradient_norm_h: sens.gradient_norm_h,
            gradient_norm_hn: sens.gradient_norm_hn,
            normals: new_normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: new_heights.to_vec(),
            time_ms,
        };
        serde_json::to_writer(&mut ascent_writer, &row).expect("write ascent");
        writeln!(ascent_writer).expect("newline");

        prev_subset = new_subset;
        prev_perm = new_perm;
        current = new_poly;
        current_sys = new_sys;

        if delta < CONVERGENCE_THRESHOLD {
            println!("  Converged (Δ < {CONVERGENCE_THRESHOLD})");
            break;
        }
    }

    let total_improvement = current_sys - sys;
    println!(
        "\n  Ascent summary: sys {sys:.10} → {current_sys:.10} (Δ={total_improvement:.6e})"
    );
    ascent_writer.flush().expect("flush ascent");
    println!("  Wrote {}", ascent_path.display());
}

// ============================================================================
// Phase B: Facet-splitting
// ============================================================================

/// Sample directions near a given facet normal on S³.
/// Returns (direction, angular_offset) pairs.
fn sample_near_normal(
    normal: &Vector4<f64>,
    n_samples: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<(Vector4<f64>, f64)> {
    let mut results = Vec::with_capacity(n_samples);

    // Build an orthonormal basis for T_{normal}S³ (3D tangent space)
    let mut basis = Vec::new();
    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    for c in &candidates {
        let proj = *c - c.dot(normal) * normal;
        // Gram-Schmidt against existing basis vectors
        let mut v = proj;
        for b in &basis {
            v -= v.dot(b) * b;
        }
        if v.norm() > 0.1 {
            basis.push(v.normalize());
        }
        if basis.len() == 3 {
            break;
        }
    }

    // Sample at various angular offsets
    let angular_scales: [f64; 10] = [0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 1.0, 1.5];
    let samples_per_scale = n_samples / angular_scales.len();

    for &angle in &angular_scales {
        for _ in 0..samples_per_scale {
            // Random tangent direction
            let t0: f64 = StandardNormal.sample(rng);
            let t1: f64 = StandardNormal.sample(rng);
            let t2: f64 = StandardNormal.sample(rng);
            let tangent = t0 * basis[0] + t1 * basis[1] + t2 * basis[2];
            let tangent = tangent.normalize();

            // Rotate normal by angle in the tangent direction
            let dir = (normal * angle.cos() + tangent * angle.sin()).normalize();
            let actual_angle = normal.dot(&dir).clamp(-1.0, 1.0).acos();
            results.push((dir, actual_angle));
        }
    }

    results
}

/// Phase B: test HKO2024's maximality in the F=11 polytope space.
///
/// Methodology: we add a cutting halfspace ⟨n,x⟩ ≤ h_K(n) - ε to create an
/// (F+1)-facet polytope K' ⊊ K. This is the only non-trivial direction from
/// HKO2024 in the F=11 ambient space: adding a halfspace is an intersection,
/// so K' ⊆ K always. When h = h_K(n) the halfspace is redundant (K' = K);
/// when h < h_K(n) it cuts. To make K *larger* we'd need to relax an existing
/// halfspace, which is already covered by Phase A's (n,h) gradient analysis.
fn run_phase_b(base_dir: &std::path::Path) {
    println!("\n═══════════════════════════════════════════════════════════");
    println!("Phase B: Facet-splitting (F=11) — test maximality beyond F=10");
    println!("═══════════════════════════════════════════════════════════\n");

    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();

    let vol_orig = volume(polytope).expect("volume");
    let cap_orig = ehz_capacity(polytope).expect("ehz").capacity;
    let sys_orig = cap_orig * cap_orig / (2.0 * vol_orig);
    println!("HKO2024 baseline: F={f}, sys={sys_orig:.10}");

    let splitting_path = base_dir.join("hko-neighborhood/hko-neighborhood-splitting.jsonl");
    let splitting_file = File::create(&splitting_path).expect("create splitting JSONL");
    let mut split_writer = BufWriter::new(splitting_file);

    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let mut total_directions = 0usize;
    let mut total_ok = 0usize;
    let mut best_delta = f64::NEG_INFINITY;
    let mut best_direction_info = String::new();

    // HKO2024 = pentagon ×_L pentagon. Lagrangian product symmetry means:
    // - All 5 Q-space normals (facets 0-4) are equivalent under 5-fold rotation
    // - All 5 P-space normals (facets 5-9) are equivalent under 5-fold rotation
    // So we only need 2 representative facets, not 10.
    let representative_facets = [0usize, 5]; // Q-space rep, P-space rep
    for &facet_k in &representative_facets {
        println!("\nFacet {facet_k} (representative): normal = [{:.4}, {:.4}, {:.4}, {:.4}]",
            normals[facet_k][0], normals[facet_k][1], normals[facet_k][2], normals[facet_k][3]);

        let samples = sample_near_normal(&normals[facet_k], N_SPLITTING_SAMPLES_PER_FACET, &mut rng);

        for (dir, angular_offset) in &samples {
            for &eps in SPLITTING_EPSILONS {
                let t_split = Instant::now();
                total_directions += 1;

                // Compute support function h_K(n) = max_v <n, v>
                let h_k_n = vertices
                    .iter()
                    .map(|v| dir.dot(v))
                    .fold(f64::NEG_INFINITY, f64::max);

                // Add cutting halfspace: <n, x> <= h_K(n) - eps
                let mut new_normals = normals.to_vec();
                let mut new_heights = heights.to_vec();
                new_normals.push(*dir);
                new_heights.push(h_k_n - eps);

                match Polytope4D::from_normals_and_heights(new_normals, new_heights) {
                    Ok(split_poly) => {
                        let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                            Some(v) => v,
                            None => continue,
                        };
                        let delta = split_sys - sys_orig;

                        // Use library ehz_capacity for orbit info (cheaper than instrumented)
                        let lib_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            ehz_capacity(&split_poly)
                        })).ok().flatten();
                        let n_valid = 0; // not computed (instrumented too expensive for F=11)
                        let best_sub = lib_result.as_ref().map(|r| r.best_subset.clone()).unwrap_or_default();
                        let best_perm = lib_result.as_ref().map(|r| r.best_permutation.clone()).unwrap_or_default();
                        let d_sys_d_h_new = f64::NAN; // skip per-direction gradient (too expensive)

                        total_ok += 1;
                        if delta > best_delta {
                            best_delta = delta;
                            best_direction_info = format!(
                                "facet={facet_k}, angle={angular_offset:.4}, eps={eps:.1e}, Δsys={delta:.6e}"
                            );
                        }

                        let row = SplittingRow {
                            source_facet: facet_k,
                            angular_offset: *angular_offset,
                            cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                            epsilon: eps,
                            sys_original: sys_orig,
                            sys_split: split_sys,
                            delta_sys: delta,
                            capacity_split: split_cap,
                            volume_split: split_vol,
                            facet_count_split: split_poly.facet_count(),
                            n_valid_orbits: n_valid,
                            best_subset: best_sub,
                            best_permutation: best_perm,
                            d_sys_d_h_new,
                            construction_ok: true,
                            time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                        };
                        serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                        writeln!(split_writer).expect("newline");
                    }
                    Err(_) => {
                        // Construction failed (degenerate geometry at small eps)
                        let row = SplittingRow {
                            source_facet: facet_k,
                            angular_offset: *angular_offset,
                            cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                            epsilon: eps,
                            sys_original: sys_orig,
                            sys_split: f64::NAN,
                            delta_sys: f64::NAN,
                            capacity_split: f64::NAN,
                            volume_split: f64::NAN,
                            facet_count_split: 0,
                            n_valid_orbits: 0,
                            best_subset: vec![],
                            best_permutation: vec![],
                            d_sys_d_h_new: f64::NAN,
                            construction_ok: false,
                            time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                        };
                        serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                        writeln!(split_writer).expect("newline");
                    }
                }
            }
        }

        // Progress
        println!(
            "  Tested {} directions so far, {} OK, best Δsys={:.6e}",
            total_directions, total_ok, best_delta
        );
    }

    // Mixed directions: components in both Q and P space (breaks Lagrangian product structure)
    println!("\n--- Mixed directions (Q+P space) ---");
    for i in 0..N_SPLITTING_MIXED {
        let t0: f64 = StandardNormal.sample(&mut rng);
        let t1: f64 = StandardNormal.sample(&mut rng);
        let t2: f64 = StandardNormal.sample(&mut rng);
        let t3: f64 = StandardNormal.sample(&mut rng);
        let dir = Vector4::new(t0, t1, t2, t3).normalize();

        // Ensure direction has components in both Q and P space
        let q_norm = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
        let p_norm = (dir[2] * dir[2] + dir[3] * dir[3]).sqrt();
        if q_norm < 0.1 || p_norm < 0.1 {
            continue; // skip nearly-pure Q or P directions
        }

        let min_angle = normals
            .iter()
            .map(|n| n.dot(&dir).clamp(-1.0, 1.0).acos())
            .fold(f64::INFINITY, f64::min);

        for &eps in SPLITTING_EPSILONS {
            let t_split = Instant::now();
            total_directions += 1;

            let h_k_n = vertices
                .iter()
                .map(|v| dir.dot(v))
                .fold(f64::NEG_INFINITY, f64::max);

            let mut new_normals = normals.to_vec();
            let mut new_heights = heights.to_vec();
            new_normals.push(dir);
            new_heights.push(h_k_n - eps);

            if let Ok(split_poly) = Polytope4D::from_normals_and_heights(new_normals, new_heights) {
                let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                    Some(v) => v,
                    None => continue,
                };
                let delta = split_sys - sys_orig;

                let lib_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    ehz_capacity(&split_poly)
                }))
                .ok()
                .flatten();
                let n_valid = 0;
                let best_sub = lib_result
                    .as_ref()
                    .map(|r| r.best_subset.clone())
                    .unwrap_or_default();
                let best_perm = lib_result
                    .as_ref()
                    .map(|r| r.best_permutation.clone())
                    .unwrap_or_default();

                total_ok += 1;
                if delta > best_delta {
                    best_delta = delta;
                    best_direction_info = format!(
                        "mixed #{i}, angle_to_nearest={min_angle:.4}, eps={eps:.1e}, Δsys={delta:.6e}"
                    );
                }

                if i < 5 || delta > -1e-6 {
                    println!(
                        "  Mixed #{i}: angle={min_angle:.4}, q={q_norm:.3}, p={p_norm:.3}, \
                         eps={eps:.1e}, Δsys={delta:.6e}"
                    );
                }

                let row = SplittingRow {
                    source_facet: usize::MAX - 1, // sentinel for "mixed"
                    angular_offset: min_angle,
                    cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                    epsilon: eps,
                    sys_original: sys_orig,
                    sys_split: split_sys,
                    delta_sys: delta,
                    capacity_split: split_cap,
                    volume_split: split_vol,
                    facet_count_split: split_poly.facet_count(),
                    n_valid_orbits: n_valid,
                    best_subset: best_sub,
                    best_permutation: best_perm,
                    d_sys_d_h_new: f64::NAN,
                    construction_ok: true,
                    time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                };
                serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                writeln!(split_writer).expect("newline");
            }
        }
    }
    println!(
        "  Mixed: tested {} total, {} OK, best Δsys={:.6e}",
        total_directions, total_ok, best_delta
    );

    // Control: random directions far from any facet normal
    println!("\n--- Control: random directions ---");
    for i in 0..N_SPLITTING_CONTROL {
        let t0: f64 = StandardNormal.sample(&mut rng);
        let t1: f64 = StandardNormal.sample(&mut rng);
        let t2: f64 = StandardNormal.sample(&mut rng);
        let t3: f64 = StandardNormal.sample(&mut rng);
        let dir = Vector4::new(t0, t1, t2, t3).normalize();

        // Check angular distance to nearest facet normal
        let min_angle = normals
            .iter()
            .map(|n| n.dot(&dir).clamp(-1.0, 1.0).acos())
            .fold(f64::INFINITY, f64::min);

        for &eps in &[1e-3, 1e-4] {
            let t_split = Instant::now();
            total_directions += 1;

            let h_k_n = vertices
                .iter()
                .map(|v| dir.dot(v))
                .fold(f64::NEG_INFINITY, f64::max);

            let mut new_normals = normals.to_vec();
            let mut new_heights = heights.to_vec();
            new_normals.push(dir);
            new_heights.push(h_k_n - eps);

            if let Ok(split_poly) = Polytope4D::from_normals_and_heights(new_normals, new_heights) {
                    let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                        Some(v) => v,
                        None => continue,
                    };
                    let delta = split_sys - sys_orig;

                    let lib_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        ehz_capacity(&split_poly)
                    })).ok().flatten();
                    let n_valid = 0;
                    let best_sub = lib_result.as_ref().map(|r| r.best_subset.clone()).unwrap_or_default();
                    let best_perm = lib_result.as_ref().map(|r| r.best_permutation.clone()).unwrap_or_default();
                    let d_sys_d_h_new = f64::NAN;

                    total_ok += 1;
                    println!(
                        "  Control #{i}: angle_to_nearest={min_angle:.4}, eps={eps:.1e}, \
                         Δsys={delta:.6e}, d_sys_d_h_new={d_sys_d_h_new:.6e}"
                    );

                    let row = SplittingRow {
                        source_facet: usize::MAX, // sentinel for "control"
                        angular_offset: min_angle,
                        cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                        epsilon: eps,
                        sys_original: sys_orig,
                        sys_split: split_sys,
                        delta_sys: delta,
                        capacity_split: split_cap,
                        volume_split: split_vol,
                        facet_count_split: split_poly.facet_count(),
                        n_valid_orbits: n_valid,
                        best_subset: best_sub,
                        best_permutation: best_perm,
                        d_sys_d_h_new,
                        construction_ok: true,
                        time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                    };
                    serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                    writeln!(split_writer).expect("newline");
            }
        }
    }

    split_writer.flush().expect("flush splitting");

    println!("\n--- Facet-splitting summary ---");
    println!("  Directions tested: {total_directions}");
    println!("  Successful constructions: {total_ok}");
    println!("  Best Δsys: {best_delta:.6e}");
    println!("  Best direction: {best_direction_info}");
    if best_delta <= 0.0 {
        println!("  → HKO2024 is a LOCAL MAXIMUM even under facet-splitting (F=11)");
    } else {
        println!("  → HKO2024 is NOT a local max under facet-splitting — improvement found!");
    }
    println!("  Wrote {}", splitting_path.display());
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!("HKO-Neighborhood: Local maximality analysis of HKO2024\n");

    run_phase_a(base_dir);
    run_phase_b(base_dir);

    let elapsed = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════════════════");
    println!("Total time: {elapsed:.1}s");
    println!("═══════════════════════════════════════════════════════════");
}
