//! Sys-optimization Phase 1–4: sensitivity, gradient steps, iteration, validity testing.
//!
//! Computes d(sys)/d(h_k) and d(sys)/d(n_k) for polytopes from random-sweep and
//! random-product-sweep, then takes finite gradient steps bounded by combinatorial type
//! preservation (Phase 2) and iterates to convergence (Phase 3).
//!
//! Convention: The library (crates/) is stable. Experiment-specific variants
//! (instrumented HK2017) are self-contained in this binary. Library internals
//! needed by the variants are copied here with source references.
//!
//! KKT solver note: Uses a local copy of the library's condition-number approach
//! (EIGEN_CONDITION_TAU = 1e-3 in crates/src/kkt.rs). The local constant retains
//! the old SVD_CONDITION_TAU name.
//!
//! Architecture:
//! 1. `cargo run --bin sys_optimization --release` generates datasets
//! 2. Writes to sys-optimization/sys-optimization-{sensitivity,steps,iterations,validity}.jsonl
//! 3. Python script reads JSONL, produces figures and stats
//!
//! Input: random-sweep/random-sweep.jsonl, random-product-sweep/random-product-sweep.jsonl
//! Filter: F ≤ 10 (HK2017 is exponential in F)

use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::volume::volume;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Heights are O(1), so steps beyond 100 are unreasonable.
const MAX_STEP_SIZE: f64 = 100.0;

/// Step fractions of t_max to evaluate.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Maximum number of gradient ascent iterations in Phase 3.
const MAX_ITERATIONS: usize = 20;

/// Minimum improvement per iteration to continue (convergence threshold).
const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Number of random directions to test per polytope in Phase 4.
const N_RANDOM_DIRECTIONS: usize = 10;

/// Step fractions of t_max to test in Phase 4 (includes beyond-t_max values).
const VALIDITY_STEP_FRACTIONS: &[f64] = &[0.01, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0];

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct SensitivityRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    volume: f64,
    capacity: f64,
    sys: f64,
    n_valid_orbits: usize,
    best_action: f64,
    runner_up_action: f64,
    runner_up_gap: f64,
    // Height derivatives
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
    n_favorable: usize,
    t_max_h: f64,
    t_max_hn: f64,
    time_instrumented_ms: f64,
    time_sensitivity_ms: f64,
}

#[derive(Debug, Serialize)]
struct StepRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    step_type: String, // "h_only" or "h_n"
    t_fraction: f64,
    t_actual: f64,
    old_sys: f64,
    new_sys: f64,
    delta_sys: f64,
    new_volume: f64,
    new_capacity: f64,
    vertex_count_old: usize,
    vertex_count_new: usize,
    construction_ok: bool,
}

#[derive(Debug, Serialize)]
struct IterationRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    iteration: usize,
    step_type: String,       // "h_only" or "h_n"
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    starting_sys: f64,
    cumulative_delta: f64,
    gradient_norm_h: f64,
    gradient_norm_n: f64,
    gradient_norm_hn: f64,
    vertex_count: usize,
    time_ms: f64,
}

#[derive(Debug, Serialize)]
struct ValidityRow {
    name: String,
    facet_count: usize,
    starting_sys: f64,
    direction_type: String,  // "gradient_h", "gradient_hn", "random"
    direction_index: usize,  // 0 for gradient, 0..N-1 for random
    t_fraction: f64,         // fraction of t_max
    t_actual: f64,
    t_max: f64,
    predicted_delta_sys: f64,
    actual_delta_sys: f64,
    prediction_error: f64,
    relative_error: f64,
    directional_derivative: f64,
    construction_ok: bool,
    vertex_count_changed: bool,
    beyond_t_max: bool,
}

// ============================================================================
// Input deserialization (reads from random-sweep / random-product-sweep JSONL)
// ============================================================================

#[derive(Debug, Deserialize)]
struct InputRow {
    name: String,
    #[serde(alias = "facet_count")]
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
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
/// Copied from crates/src/kkt.rs — Q > 0 for permutations in positive Reeb direction.
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
/// Based on crates/src/kkt.rs build_kkt_system, but uses the ASYMMETRIC sign
/// convention (upper-right = -n/-h, lower-left = +n/+h). The library
/// uses the SYMMETRIC convention (+n/+h in both blocks).
/// This file retains the asymmetric convention because the gradient formulas
/// (compute_capacity_derivatives_analytical/normal) extract multipliers
/// directly from the solution vector with the sign matching Hβ = Nλ + ην.
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
/// Copied from crates/src/kkt.rs:233-359, extended to return ν.
/// Uses SVD_CONDITION_TAU (current library approach), NOT ablation's gap-ratio.
///
/// Returns (β, Q, ν) where ν is the Lagrange multiplier for the η^T β = 1 constraint.
/// ν is needed for the analytical capacity derivative (envelope theorem).
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

    // Condition-number threshold rank detection
    let threshold = max_sv * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();

    // Early dismissal via δβ-component check (from current library)
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

    // Compute pseudoinverse solution using top `rank` singular values
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

    // Rank-deficient: search null space for β > 0
    let null_beta: Vec<Vec<f64>> = (rank..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();
    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])?
    } else {
        find_positive_beta_nd(&beta0, &null_beta)?
    };

    // Constraint verification
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
    // ν and λ come from x0 (pseudoinverse solution). The null-space search only
    // adjusts β, not the multipliers, so ν = x0[m+4] and λ = x0[m..m+4] are correct.
    Some((beta_opt, q_val, nu, lambda))
}

/// SVD-only KKT solver, extended to return ν and λ.
/// Local SVD-based KKT solver. The library has since migrated to eigendecomposition
/// (solve_kkt_eigen_path in kkt.rs), but this experiment retains its own SVD copy
/// for self-containedness.
///
/// Returns (β, Q, ν, λ) where ν is the Lagrange multiplier for η^T β = 1
/// and λ ∈ R⁴ is the Lagrange multiplier vector for N^T β = 0.
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
// (crates/src/algorithms/hk2017/mod.rs, permutations.rs)
// ============================================================================

/// Generate all C(n,k) combinations in lexicographic order.
/// Copied from crates/src/algorithms/hk2017/mod.rs:157-180
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

/// Call callback once for each cyclic permutation of elements.
/// Copied from crates/src/algorithms/hk2017/permutations.rs:22-35
fn for_each_cyclic_permutation(elements: &[usize], callback: &mut impl FnMut(&[usize])) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf(&mut buf, 1, k, callback);
}

/// Heap's algorithm on buf[offset..offset+k].
/// Copied from crates/src/algorithms/hk2017/permutations.rs:38-57
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

/// Build undirected facet adjacency matrix (vertex-sharing).
/// Copied from crates/src/algorithms/hk2017/mod.rs:184-204
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

/// Build directed adjacency for positive Reeb direction.
/// adj[i][j] = vertex_adj[i][j] AND ω₀(n_i, n_j) >= 0
/// (transition F_i → F_j in positive Reeb direction)
/// Copied from crates/src/algorithms/hk2017/mod.rs
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

/// Check if a cyclic permutation forms an adjacent cycle.
/// Copied from crates/src/algorithms/hk2017/mod.rs:232-235
fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// Instrumented HK2017 — collects ALL valid orbits
// ============================================================================

/// A valid orbit from the instrumented HK2017 computation.
#[derive(Debug, Clone)]
struct ValidOrbit {
    action: f64,
    subset: Vec<usize>,
    permutation: Vec<usize>, // positive Reeb direction
    beta: Vec<f64>,
    q_value: f64,
    /// Lagrange multiplier for the η^T β = 1 constraint.
    /// Used for analytical capacity derivative via envelope theorem:
    /// dA/dh_k = ν · β_{i₀} / (2Q²) where perm[i₀] = k.
    nu: f64,
    /// Lagrange multiplier vector (4 components) for the N^T β = 0 constraint.
    /// Used for analytical capacity derivative w.r.t. normals via envelope theorem.
    lambda: Vec<f64>,
}

/// Result of the instrumented HK2017 computation.
struct InstrumentedResult {
    capacity: f64,
    capacity_uncertain: f64,
    orbits: Vec<ValidOrbit>, // sorted by action ascending
    iterations: u64,
}

/// Instrumented version of ehz_capacity that collects ALL valid orbits.
/// Same algorithm as production ehz_capacity, but collects ALL valid orbits.
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

                    // Certified: β_i > +EPS
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

                    // Track uncertain best
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

    // Sort by action ascending
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
// Sensitivity computation
// ============================================================================

struct SensitivityResult {
    // Height derivatives
    d_vol_h: Vec<f64>,
    d_cap_h: Vec<f64>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    // Normal derivatives (tangent vectors in T_{n_k}S³)
    d_vol_n: Vec<Vector4<f64>>,
    d_cap_n: Vec<Vector4<f64>>,
    d_sys_n: Vec<Vector4<f64>>,
    gradient_norm_n: f64,
    // Combined gradient norm
    gradient_norm_hn: f64,
    // Gap info
    runner_up_gap: f64,
}

// ----------------------------------------------------------------------------
// Facet volume helpers (copied from crates/src/geom/volume.rs deprecated module
// and crates/src/geom/cross_product.rs). Needed for analytical volume derivatives.
// ----------------------------------------------------------------------------

/// Threshold for detecting degenerate (collinear) polygon vertices.
const EPS_DEGENERATE: f64 = 1e-10;

/// 4D cross product: vector perpendicular to three vectors in R⁴.
///
/// Source: crates/src/geom/cross_product.rs
fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let bc_01 = b[0] * c[1] - b[1] * c[0];
    let bc_02 = b[0] * c[2] - b[2] * c[0];
    let bc_03 = b[0] * c[3] - b[3] * c[0];
    let bc_12 = b[1] * c[2] - b[2] * c[1];
    let bc_13 = b[1] * c[3] - b[3] * c[1];
    let bc_23 = b[2] * c[3] - b[3] * c[2];

    let d0 =   a[1] * bc_23 - a[2] * bc_13 + a[3] * bc_12;
    let d1 = -(a[0] * bc_23 - a[2] * bc_03 + a[3] * bc_02);
    let d2 =   a[0] * bc_13 - a[1] * bc_03 + a[3] * bc_01;
    let d3 = -(a[0] * bc_12 - a[1] * bc_02 + a[2] * bc_01);

    Vector4::new(d0, d1, d2, d3)
}

/// Sort vertices of a convex polygon in R^4 by angle around their centroid.
///
/// Source: crates/src/geom/volume.rs (deprecated module)
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

/// Compute the 3D volume of facet `fi` by decomposing into tetrahedra.
///
/// Source: crates/src/geom/volume.rs (deprecated module)
fn facet_volume_3d(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
    fi: usize,
    f: usize,
) -> f64 {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (normals[fi].dot(v) - heights[fi]).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return 0.0;
    }

    let centroid = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;

    (0..f)
        .filter(|&fj| fj != fi)
        .flat_map(|fj| {
            let ridge_verts: Vec<Vector4<f64>> = facet_verts
                .iter()
                .filter(|v| (normals[fj].dot(v) - heights[fj]).abs() < EPS_FACET_INCIDENCE)
                .cloned()
                .collect();

            if ridge_verts.len() < 3 {
                return Vec::new();
            }

            let sorted = sort_polygon_vertices(&ridge_verts);
            (1..sorted.len() - 1)
                .map(|k| {
                    let a = sorted[0] - centroid;
                    let b = sorted[k] - centroid;
                    let c = sorted[k + 1] - centroid;
                    cross_product_4d(a, b, c).norm() / 6.0
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

/// Compute the 3D volume and area-weighted centroid of facet `fi`.
///
/// Returns (S_k, x̄_k) where S_k is the 3D volume of the facet and
/// x̄_k = (1/S_k) ∫_{F_k} x dσ_k is the area-weighted centroid.
///
/// Same tetrahedralization as `facet_volume_3d`, but also accumulates
/// volume-weighted simplex centroids: x̄ = Σ τ_j c_j / Σ τ_j
/// where c_j = (apex + v0 + vk + vk+1) / 4 for each fan triangle.
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
            // Centroid of the tetrahedron (apex, sorted[0], sorted[k], sorted[k+1])
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

/// Compute d(vol)/d(h_k) analytically: ∂vol/∂h_k = S_k (3D volume of facet k).
///
/// Standard result for convex bodies in H-representation.
/// Uses the divergence theorem: vol(K) = (1/4) Σ h_i · S_i,
/// so ∂vol/∂h_k = S_k / 4... NO: the full derivative is S_k because
/// moving facet k outward by dh adds a slab of thickness dh and cross-section S_k.
///
/// More precisely: for K = {x : n_i · x ≤ h_i}, ∂vol(K)/∂h_k = vol_3D(F_k)
/// where F_k is the k-th facet (a 3D polytope).
fn compute_volume_derivatives_analytical(polytope: &Polytope4D) -> Vec<f64> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();
    (0..f)
        .map(|k| facet_volume_3d(&normals, &heights, &vertices, k, f))
        .collect()
}

/// Compute d(vol)/d(h_k) via central finite differences (validation only).
///
/// Kept as a cross-check for the analytical version. Used in debug_assert.
/// Uses eps=1e-3 (not 1e-7) because qhull volume precision is ~1e-8 relative,
/// so eps=1e-7 puts the volume change at the numerical noise floor.
fn compute_volume_derivatives_fd(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    const FD_EPS: f64 = 1e-3;
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

            let vol_plus = volume(&p_plus).unwrap_or(f64::NAN);
            let vol_minus = volume(&p_minus).unwrap_or(f64::NAN);
            (vol_plus - vol_minus) / (2.0 * FD_EPS)
        })
        .collect()
}

/// Compute d(c_EHZ)/d(h_k) analytically via the envelope theorem.
///
/// For orbit (S,σ) with KKT solution (β, Q, ν), the action is A = 1/(2Q).
/// The KKT system solves: Hβ − Nλ − νη = 0, N^Tβ = 0, η^Tβ = 1.
/// Here ν is the multiplier for η^Tβ = 1. For the winning orbit, ν < 0.
///
/// By the envelope theorem, dQ*/dh_k = ∂L/∂h_k = −ν · β_{i₀},
/// where i₀ is the orbit position with perm[i₀] = k.
/// Since A = 1/(2Q): dA/dh_k = −dQ*/(2Q²·dh_k) = ν · β_{i₀} / (2Q²).
///
/// Lemma lem:cap-derivative (thesis): ∂A/∂h_k = ν · β_{i₀} / (2Q²).
/// ν is the Lagrange multiplier for the normalization constraint η^T β = 1
/// and is positive for the capacity-achieving orbit.
///
/// If facet k is not in the orbit (k ∉ S), then dA/dh_k = 0.
///
/// For the capacity c = min_orbits A(orbit), we use the derivative of the
/// minimum-action orbit. This is exact in the non-degenerate case. At
/// orbit-switching boundaries, the capacity is non-smooth and the derivative
/// is one-sided.
fn compute_capacity_derivatives_analytical(
    best_orbit: &ValidOrbit,
    facet_count: usize,
) -> Vec<f64> {
    let q_sq = best_orbit.q_value * best_orbit.q_value;

    (0..facet_count)
        .map(|k| {
            // Find position of facet k in the orbit's permutation
            match best_orbit.permutation.iter().position(|&f| f == k) {
                Some(i0) => {
                    // Lemma lem:cap-derivative: ∂A/∂h_k = ν·β_{i₀}/(2Q²)
                    best_orbit.nu * best_orbit.beta[i0] / (2.0 * q_sq)
                }
                None => 0.0, // Facet not in orbit → height doesn't affect this orbit's action
            }
        })
        .collect()
}

/// Compute d(vol)/d(n_k) analytically, projected onto T_{n_k}S³.
///
/// For δ ⊥ n_k: ∂vol/∂n_k · δ = −∫_{F_k} (δ · x) dσ_k = −S_k (x̄_k · δ)
/// where S_k = 3D volume of facet k, x̄_k = area-weighted centroid of facet k.
///
/// Since n_k · x̄_k = h_k (centroid lies on facet plane), the tangent gradient is:
///   (∇_{n_k} vol)_tangent = −S_k (x̄_k − h_k n_k)
///
/// Returns one tangent vector per facet (already projected to T_{n_k}S³).
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
            // Tangent part of centroid: x̄_k − (x̄_k · n_k) n_k = x̄_k − h_k n_k
            let tangent_centroid = centroid_k - heights[k] * normals[k];
            -s_k * tangent_centroid
        })
        .collect()
}

/// Compute d(c_EHZ)/d(n_k) analytically via the envelope theorem, projected onto T_{n_k}S³.
///
/// For orbit (S,σ) with KKT solution (β*, Q*, ν*, λ*), the action A = 1/(2Q*).
/// The KKT system: H_{ij} = ω₀(n_{σ(i)}, n_{σ(j)}), N = [n_{σ(1)}|⋯|n_{σ(m)}], η_i = h_{σ(i)}.
///
/// H depends on normals only; η depends on heights only. By the envelope theorem:
///   ∂Q*/∂n_k = ½ β*^T (∂H/∂n_k) β* − λ*^T (∂N/∂n_k)^T β*
///            = β*_{i₀} [J₀(2P_{i₀} + β*_{i₀} n_k) − λ*]
/// where P_{i₀} = Σ_{i<i₀} β*_i n_{σ(i)} and σ(i₀) = k.
///
/// Then ∂A/∂n_k = −∂Q*/∂n_k / (2Q*²), projected onto T_{n_k}S³.
///
/// If facet k is not in the orbit, the derivative is zero.
fn compute_capacity_derivatives_normal(
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
            // Find position of facet k in the orbit's permutation
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(), // facet not in orbit
            };

            // P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)}
            let mut p = Vector4::zeros();
            for i in 0..i0 {
                p += beta[i] * normals[perm[i]];
            }

            // ∂Q*/∂n_k = β_{i₀} · [J₀(2P + β_{i₀} n_k) − λ]
            let inner = 2.0 * p + beta[i0] * normals[k];
            let j0_inner = j0_apply(&inner);
            let dq_dn = beta[i0] * (j0_inner - lambda);

            // Project onto T_{n_k}S³: remove normal component
            let dq_dn_tangent = dq_dn - dq_dn.dot(&normals[k]) * normals[k];

            // ∂A/∂n_k = −∂Q*/∂n_k / (2Q²)
            -dq_dn_tangent / (2.0 * q_sq)
        })
        .collect()
}

/// Apply J₀ to a vector: J₀(a,b,c,d) = (-c,-d,a,b).
///
/// J₀ = [[0, -I₂], [I₂, 0]] in (q₁, q₂, p₁, p₂) coordinates.
fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

/// Compute full sensitivity: d(sys)/d(h_k) and d(sys)/d(n_k) via chain rule.
///
/// Height derivatives: analytical (envelope theorem + facet volumes).
/// Normal derivatives: analytical (envelope theorem with ∂H/∂n_k, ∂N/∂n_k + centroid formula).
fn compute_sensitivity(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    instrumented: &InstrumentedResult,
) -> SensitivityResult {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let f = normals.len();
    let best_orbit = &instrumented.orbits[0];

    // --- Height derivatives (existing) ---
    let d_vol_h = compute_volume_derivatives_analytical(polytope);

    // Cross-check: analytical volume derivatives (h) vs finite differences
    debug_assert!({
        let d_vol_fd = compute_volume_derivatives_fd(&normals, &heights);
        let ok = d_vol_h.iter().zip(d_vol_fd.iter()).all(|(a, fd)| {
            if fd.is_nan() { return true; }
            let tol = (0.05 * a.abs()).max(0.1);
            (a - fd).abs() < tol
        });
        if !ok {
            eprintln!("volume h-derivative mismatch: analytical={:?} fd={:?}", d_vol_h, d_vol_fd);
        }
        ok
    }, "volume h-derivative: analytical vs FD mismatch");

    let d_cap_h = compute_capacity_derivatives_analytical(best_orbit, f);

    let d_sys_h: Vec<f64> = d_vol_h
        .iter()
        .zip(d_cap_h.iter())
        .map(|(&dv, &dc)| {
            if dv.is_nan() || dc.is_nan() {
                f64::NAN
            } else {
                (cap * dc - sys * dv) / vol
            }
        })
        .collect();

    let gradient_norm_h = d_sys_h
        .iter()
        .filter(|x| !x.is_nan())
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();

    // --- Normal derivatives (new) ---
    let d_vol_n = compute_volume_derivatives_normal(polytope);
    let d_cap_n = compute_capacity_derivatives_normal(best_orbit, &normals, f);

    // Chain rule: d(sys)/d(n_k) = (1/vol) * [c * dc/dn_k - sys * dvol/dn_k]
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

    let gradient_norm_hn = (gradient_norm_h * gradient_norm_h + gradient_norm_n * gradient_norm_n).sqrt();

    let runner_up_gap = if instrumented.orbits.len() >= 2 {
        instrumented.orbits[1].action - instrumented.orbits[0].action
    } else {
        f64::INFINITY
    };

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
        runner_up_gap,
    }
}

// ============================================================================
// Step bounds computation
// ============================================================================

/// Compute maximum step t > 0 along direction g before combinatorial type changes.
///
/// For step direction g = (g_0, ..., g_{F-1}), new heights h'_k = h_k + t * g_k.
/// The combinatorial type changes when a vertex hits a new facet or becomes infeasible.
fn compute_step_bound(
    polytope: &Polytope4D,
    direction: &[f64],
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
            // Simple vertex: exactly 4 determining facets
            // Build 4×4 normal matrix and compute vertex movement direction
            let det_facets = &vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue, // Degenerate vertex, skip
            };

            // RHS: height changes for the determining facets
            let g_det = Vector4::new(
                direction[det_facets[0]],
                direction[det_facets[1]],
                direction[det_facets[2]],
                direction[det_facets[3]],
            );

            // Vertex movement: dv/dt = N^{-1} * g_det
            let dv_dt = n_inv * g_det;

            // Check each non-determining facet
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                // Current slack: h_j - n_j · v > 0
                let slack = heights[j] - normals[j].dot(v);
                // Rate of slack change: d(slack)/dt = g_j - n_j · dv/dt
                let rate = direction[j] - normals[j].dot(&dv_dt);
                // Slack hits zero at t = slack / (-rate) when rate < 0
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex (>4 incident facets, e.g. in Lagrangian products).
            // Conservative bound: check slack for all non-incident facets assuming
            // vertex moves at worst rate. Use minimum slack / max possible rate.
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                // Conservative: assume vertex doesn't move but facet j moves
                // This underestimates t_max but is always safe
                if direction[j] < -1e-15 {
                    // Facet j moves inward: doesn't affect this vertex
                    // (constraint h_j - n_j·v gets tighter only if n_j·v increases)
                    continue;
                }
                // For non-incident facets, the vertex might approach due to height changes
                // of the determining facets. Without inverting a >4 system, use a crude bound:
                // assume the slack can decrease at most at rate proportional to max |g_k|
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > 1e-15 {
                    // Very conservative: t_max ≤ slack / max_g
                    let t_crit = slack / max_g;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    // Also check that all heights stay positive
    for k in 0..f {
        if direction[k] < -1e-15 {
            // h_k + t * g_k > 0 → t < -h_k / g_k = h_k / |g_k|
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // Cap at practical maximum
    t_max.min(MAX_STEP_SIZE)
}

/// Compute maximum step t > 0 along combined (g_h, g_n) direction.
///
/// Extends `compute_step_bound` to handle both height and normal perturbations.
/// At step t: h'_k = h_k + t·g_{h,k}, n'_k = normalize(n_k + t·g_{n,k}).
///
/// Additional constraint: ω₀(n_i, n_j) must not change sign for ridge-adjacent pairs.
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

    // --- Vertex-crossing checks ---
    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            // Simple vertex: v = N_v⁻¹ h_v
            // When both normals and heights change:
            // N_v(t) · v(t) = h_v(t) differentiate:
            // N_v · dv/dt + dN_v/dt · v = dh_v/dt
            // dv/dt = N_v⁻¹ (dh_v/dt - dN_v/dt · v)
            // where dN_v/dt has rows g_{n,det[r]} and dh_v/dt has entries g_{h,det[r]}
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

            // RHS: dh_v/dt - dN_v/dt · v
            let rhs = Vector4::new(
                g_h[det_facets[0]] - g_n[det_facets[0]].dot(v),
                g_h[det_facets[1]] - g_n[det_facets[1]].dot(v),
                g_h[det_facets[2]] - g_n[det_facets[2]].dot(v),
                g_h[det_facets[3]] - g_n[det_facets[3]].dot(v),
            );
            let dv_dt = n_inv * rhs;

            // Check each non-determining facet
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                // Slack: s_j(t) = h_j(t) - n_j(t) · v(t)
                // ds_j/dt = g_{h,j} - g_{n,j} · v - n_j · dv/dt
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
            // Non-simple vertex: conservative bound
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                // Conservative: max rate from all sources
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

    // --- Height positivity ---
    for k in 0..f {
        if g_h[k] < -1e-15 {
            let t_crit = heights[k] / (-g_h[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // --- ω₀ sign preservation for ridge-adjacent pairs ---
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let omega_ij = omega0_local(&normals[i], &normals[j]);
        // d(ω₀(n_i(t), n_j(t)))/dt = ω₀(g_{n,i}, n_j) + ω₀(n_i, g_{n,j})
        let d_omega = omega0_local(&g_n[i], &normals[j]) + omega0_local(&normals[i], &g_n[j]);
        // Sign flips when omega_ij + t * d_omega = 0 → t = -omega_ij / d_omega
        // Only relevant if the sign would flip (omega_ij and d_omega have opposite signs)
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
// Gradient step evaluation
// ============================================================================

/// Take a gradient step in (h,n) space and evaluate the result.
fn evaluate_gradient_step_hn(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
    old_sys: f64,
    old_vertex_count: usize,
) -> StepRow {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm() // renormalize to unit length
        })
        .collect();

    match Polytope4D::from_normals_and_heights(new_normals, new_heights) {
        Ok(new_polytope) => {
            let new_vol = volume(&new_polytope).unwrap_or(f64::NAN);
            let new_cap = ehz_capacity(&new_polytope)
                .map(|r| r.result.capacity)
                .unwrap_or(f64::NAN);
            let new_sys = if new_vol > 0.0 && new_cap.is_finite() {
                new_cap * new_cap / (2.0 * new_vol)
            } else {
                f64::NAN
            };

            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_n".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys,
                delta_sys: new_sys - old_sys,
                new_volume: new_vol,
                new_capacity: new_cap,
                vertex_count_old: old_vertex_count,
                vertex_count_new: new_polytope.vertices_f64().len(),
                construction_ok: true,
            }
        }
        Err(e) => {
            eprintln!("    (h,n) step t={t:.6} failed: {e}");
            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_n".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys: f64::NAN,
                delta_sys: f64::NAN,
                new_volume: f64::NAN,
                new_capacity: f64::NAN,
                vertex_count_old: old_vertex_count,
                vertex_count_new: 0,
                construction_ok: false,
            }
        }
    }
}

/// Take a gradient step and evaluate the result.
fn evaluate_gradient_step(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
    old_sys: f64,
    old_vertex_count: usize,
) -> StepRow {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    match Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights) {
        Ok(new_polytope) => {
            let new_vol = volume(&new_polytope).unwrap_or(f64::NAN);
            // Use library ehz_capacity for the step evaluation (not instrumented — faster)
            let new_cap = ehz_capacity(&new_polytope)
                .map(|r| r.result.capacity)
                .unwrap_or(f64::NAN);
            let new_sys = if new_vol > 0.0 && new_cap.is_finite() {
                new_cap * new_cap / (2.0 * new_vol)
            } else {
                f64::NAN
            };

            StepRow {
                name: String::new(), // filled in by caller
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_only".to_string(),
                t_fraction: 0.0, // filled in by caller
                t_actual: t,
                old_sys,
                new_sys,
                delta_sys: new_sys - old_sys,
                new_volume: new_vol,
                new_capacity: new_cap,
                vertex_count_old: old_vertex_count,
                vertex_count_new: new_polytope.vertices_f64().len(),
                construction_ok: true,
            }
        }
        Err(e) => {
            eprintln!("    Step t={t:.6} failed: {e}");
            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_only".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys: f64::NAN,
                delta_sys: f64::NAN,
                new_volume: f64::NAN,
                new_capacity: f64::NAN,
                vertex_count_old: old_vertex_count,
                vertex_count_new: 0,
                construction_ok: false,
            }
        }
    }
}

// ============================================================================
// Phase 3 helpers: gradient steps that return the new polytope
// ============================================================================

/// Try a height-only gradient step, returning the new polytope and its sys value.
fn try_step_h_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    let new_polytope = match Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights) {
        Ok(p) => p,
        Err(_) => return None,
    };
    let vol = volume(&new_polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(&new_polytope)
        .map(|r| r.result.capacity)
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((new_polytope, sys))
    } else {
        None
    }
}

/// Try a (h,n) gradient step, returning the new polytope and its sys value.
fn try_step_hn_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm()
        })
        .collect();

    let new_polytope = match Polytope4D::from_normals_and_heights(new_normals, new_heights) {
        Ok(p) => p,
        Err(_) => return None,
    };
    let vol = volume(&new_polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(&new_polytope)
        .map(|r| r.result.capacity)
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((new_polytope, sys))
    } else {
        None
    }
}

// ============================================================================
// Data loading
// ============================================================================

fn load_polytopes_from_jsonl(path: &std::path::Path, source: &str) -> Vec<(String, String, Polytope4D)> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("WARNING: Could not open {}: {e}", path.display());
            eprintln!("  Run the corresponding experiment binary first.");
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let mut polytopes = Vec::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("  Line {}: read error: {e}", line_no + 1);
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let row: InputRow = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  Line {}: parse error: {e}", line_no + 1);
                continue;
            }
        };

        if row.facet_count > MAX_FACET_COUNT {
            continue;
        }

        let normals: Vec<Vector4<f64>> = row
            .normals
            .iter()
            .map(|n| Vector4::new(n[0], n[1], n[2], n[3]))
            .collect();

        match Polytope4D::from_normals_and_heights(normals, row.heights) {
            Ok(p) => polytopes.push((row.name, source.to_string(), p)),
            Err(e) => {
                eprintln!("  {}: construction failed: {e}", row.name);
            }
        }
    }

    polytopes
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!("Sys-optimization Phase 1–3: sensitivity + steps + iterative ascent\n");

    // =========================================================================
    // Load starting polytopes
    // =========================================================================

    println!("Loading starting polytopes (F ≤ {MAX_FACET_COUNT})...");

    let random_sweep_path = base_dir.join("random-sweep/random-sweep.jsonl");
    let random_product_path = base_dir.join("random-product-sweep/random-product-sweep.jsonl");

    let mut polytopes: Vec<(String, String, Polytope4D)> = Vec::new();

    let rs = load_polytopes_from_jsonl(&random_sweep_path, "random-sweep");
    println!("  random-sweep: {} polytopes loaded", rs.len());
    polytopes.extend(rs);

    let rp = load_polytopes_from_jsonl(&random_product_path, "random-product-sweep");
    println!("  random-product-sweep: {} polytopes loaded", rp.len());
    polytopes.extend(rp);

    let n_polytopes = polytopes.len();
    println!("  Total: {n_polytopes} polytopes\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes loaded. Run random_sweep and random_product_sweep first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Phase 1: Sensitivity analysis
    // =========================================================================

    println!("Phase 1: Computing sensitivities...\n");

    let sensitivity_path = base_dir.join("sys-optimization/sys-optimization-sensitivity.jsonl");
    let steps_path = base_dir.join("sys-optimization/sys-optimization-steps.jsonl");

    let sens_file = File::create(&sensitivity_path).expect("create sensitivity JSONL");
    let mut sens_writer = BufWriter::new(sens_file);

    let steps_file = File::create(&steps_path).expect("create steps JSONL");
    let mut steps_writer = BufWriter::new(steps_file);

    let mut total_favorable = 0usize;
    let mut total_facets = 0usize;
    let mut best_sys_after = f64::NEG_INFINITY;
    let mut best_sys_before = 0.0f64;
    let mut n_improved = 0usize;

    for (idx, (name, source, polytope)) in polytopes.iter().enumerate() {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();

        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        // --- Instrumented HK2017 ---
        let t_instr = Instant::now();
        let instrumented = match ehz_capacity_instrumented(polytope) {
            Some(r) => r,
            None => {
                println!("SKIP (no valid orbits)");
                continue;
            }
        };
        let time_instrumented_ms = t_instr.elapsed().as_secs_f64() * 1000.0;

        // Cross-check: instrumented capacity must match library ehz_capacity
        let lib_result = ehz_capacity(polytope).expect("library ehz_capacity failed");
        let cap_diff = (instrumented.capacity - lib_result.capacity).abs();
        assert!(
            cap_diff < 1e-8,
            "Capacity mismatch for {}: instrumented={:.10}, library={:.10}, diff={:.2e}",
            name,
            instrumented.capacity,
            lib_result.capacity,
            cap_diff
        );

        let cap = instrumented.capacity;
        let vol = volume(polytope).expect("volume failed");
        let sys = cap * cap / (2.0 * vol);

        // --- Sensitivity ---
        let t_sens = Instant::now();
        let sensitivity = compute_sensitivity(polytope, vol, cap, sys, &instrumented);
        let time_sensitivity_ms = t_sens.elapsed().as_secs_f64() * 1000.0;

        // Count favorable facets: d_sys_h > 0 means increasing h_k improves sys,
        // d_sys_h < 0 means decreasing h_k improves sys. Either is "favorable".
        let n_favorable = sensitivity
            .d_sys_h
            .iter()
            .filter(|&&ds| !ds.is_nan() && ds.abs() > 1e-10)
            .count();
        total_favorable += n_favorable;
        total_facets += f;

        let runner_up_action = if instrumented.orbits.len() >= 2 {
            instrumented.orbits[1].action
        } else {
            f64::INFINITY
        };

        // --- Step bounds ---
        // h-only direction: steepest ascent = d_sys_h
        let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
            compute_step_bound(polytope, &sensitivity.d_sys_h)
        } else {
            0.0
        };

        // (h,n) direction: steepest ascent = (d_sys_h, d_sys_n)
        let t_max_hn = if sensitivity.gradient_norm_hn > 1e-15 {
            compute_step_bound_hn(polytope, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
        } else {
            0.0
        };

        println!(
            "orbits={}, sys={:.6}, |∇h|={:.4e}, |∇n|={:.4e}, |∇hn|={:.4e}, t_h={:.4e}, t_hn={:.4e}, {:.0}ms",
            instrumented.orbits.len(),
            sys,
            sensitivity.gradient_norm_h,
            sensitivity.gradient_norm_n,
            sensitivity.gradient_norm_hn,
            t_max_h,
            t_max_hn,
            time_instrumented_ms + time_sensitivity_ms
        );

        // Write sensitivity row
        let normals_raw: Vec<[f64; 4]> = normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect();
        let d_vol_n_raw: Vec<[f64; 4]> = sensitivity.d_vol_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let d_cap_n_raw: Vec<[f64; 4]> = sensitivity.d_cap_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let d_sys_n_raw: Vec<[f64; 4]> = sensitivity.d_sys_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let sens_row = SensitivityRow {
            name: name.clone(),
            source_dataset: source.clone(),
            facet_count: f,
            normals: normals_raw,
            heights: heights.to_vec(),
            volume: vol,
            capacity: cap,
            sys,
            n_valid_orbits: instrumented.orbits.len(),
            best_action: instrumented.orbits[0].action,
            runner_up_action,
            runner_up_gap: sensitivity.runner_up_gap,
            d_vol_h: sensitivity.d_vol_h,
            d_cap_h: sensitivity.d_cap_h,
            d_sys_h: sensitivity.d_sys_h.clone(),
            gradient_norm_h: sensitivity.gradient_norm_h,
            d_vol_n: d_vol_n_raw,
            d_cap_n: d_cap_n_raw,
            d_sys_n: d_sys_n_raw,
            gradient_norm_n: sensitivity.gradient_norm_n,
            gradient_norm_hn: sensitivity.gradient_norm_hn,
            n_favorable,
            t_max_h,
            t_max_hn,
            time_instrumented_ms,
            time_sensitivity_ms,
        };
        serde_json::to_writer(&mut sens_writer, &sens_row).expect("write sensitivity");
        writeln!(sens_writer).expect("newline");

        // =========================================================================
        // Phase 2: Gradient steps (h-only)
        // =========================================================================

        let vertex_count_old = polytope.vertices_f64().len();

        if t_max_h > 0.0 && sensitivity.gradient_norm_h > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_h;
                let mut step_row = evaluate_gradient_step(
                    &normals,
                    &heights,
                    &sensitivity.d_sys_h,
                    t,
                    sys,
                    vertex_count_old,
                );
                step_row.name = name.clone();
                step_row.source_dataset = source.clone();
                step_row.t_fraction = frac;

                if step_row.construction_ok && step_row.new_sys > best_sys_after {
                    best_sys_after = step_row.new_sys;
                    best_sys_before = sys;
                }
                if step_row.construction_ok && step_row.delta_sys > 1e-10 {
                    n_improved += 1;
                }

                serde_json::to_writer(&mut steps_writer, &step_row).expect("write step");
                writeln!(steps_writer).expect("newline");
            }
        }

        // =========================================================================
        // Phase 2b: Gradient steps (h+n combined)
        // =========================================================================

        if t_max_hn > 0.0 && sensitivity.gradient_norm_hn > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_hn;
                let mut step_row = evaluate_gradient_step_hn(
                    &normals,
                    &heights,
                    &sensitivity.d_sys_h,
                    &sensitivity.d_sys_n,
                    t,
                    sys,
                    vertex_count_old,
                );
                step_row.name = name.clone();
                step_row.source_dataset = source.clone();
                step_row.t_fraction = frac;

                serde_json::to_writer(&mut steps_writer, &step_row).expect("write step");
                writeln!(steps_writer).expect("newline");
            }
        }
    }

    sens_writer.flush().expect("flush sensitivity");
    steps_writer.flush().expect("flush steps");

    // =========================================================================
    // Phase 3: Iterative gradient ascent
    // =========================================================================

    println!("\nPhase 3: Iterative gradient ascent (max {MAX_ITERATIONS} iterations)...\n");

    let iterations_path = base_dir.join("sys-optimization/sys-optimization-iterations.jsonl");
    let iter_file = File::create(&iterations_path).expect("create iterations JSONL");
    let mut iter_writer = BufWriter::new(iter_file);

    let mut total_iterations = 0usize;
    let mut max_sys_achieved = f64::NEG_INFINITY;
    let mut max_sys_name = String::new();
    let mut n_converged = 0usize;

    for (idx, (name, source, start_polytope)) in polytopes.iter().enumerate() {
        let f = start_polytope.facet_count();
        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        let t_poly = Instant::now();

        // Reconstruct to get an owned polytope we can replace each iteration
        let mut current = match Polytope4D::from_normals_and_heights(
            start_polytope.normals_f64().to_vec(),
            start_polytope.heights_f64().to_vec(),
        ) {
            Ok(p) => p,
            Err(e) => {
                println!("SKIP (reconstruct failed: {e})");
                continue;
            }
        };

        let mut starting_sys = 0.0f64;
        let mut current_sys = 0.0f64;
        let mut n_iter = 0usize;
        let mut converged = false;
        let mut n_h_only = 0usize;
        let mut n_h_n = 0usize;

        for iter in 0..MAX_ITERATIONS {
            let t_iter = Instant::now();

            // 1. Instrumented HK2017
            let instrumented = match ehz_capacity_instrumented(&current) {
                Some(r) => r,
                None => break,
            };
            let cap = instrumented.capacity;
            let vol = volume(&current).expect("volume");
            let sys = cap * cap / (2.0 * vol);

            if iter == 0 {
                starting_sys = sys;
                current_sys = sys;
            }

            // 2. Sensitivity
            let sensitivity = compute_sensitivity(&current, vol, cap, sys, &instrumented);

            // 3. Step bounds
            let normals = current.normals_f64();
            let heights = current.heights_f64();

            let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
                compute_step_bound(&current, &sensitivity.d_sys_h)
            } else {
                0.0
            };
            let t_max_hn = if sensitivity.gradient_norm_hn > 1e-15 {
                compute_step_bound_hn(&current, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
            } else {
                0.0
            };

            // 4. Try all step fractions for both types, pick best
            let mut best: Option<(Polytope4D, f64, String, f64, f64)> = None;

            if t_max_h > 0.0 && sensitivity.gradient_norm_h > 1e-15 {
                for &frac in STEP_FRACTIONS {
                    let t = frac * t_max_h;
                    if let Some((p, new_sys)) = try_step_h_polytope(
                        &normals, &heights, &sensitivity.d_sys_h, t,
                    ) {
                        if new_sys > sys && best.as_ref().map_or(true, |b| new_sys > b.1) {
                            best = Some((p, new_sys, "h_only".to_string(), frac, t));
                        }
                    }
                }
            }

            if t_max_hn > 0.0 && sensitivity.gradient_norm_hn > 1e-15 {
                for &frac in STEP_FRACTIONS {
                    let t = frac * t_max_hn;
                    if let Some((p, new_sys)) = try_step_hn_polytope(
                        &normals, &heights, &sensitivity.d_sys_h, &sensitivity.d_sys_n, t,
                    ) {
                        if new_sys > sys && best.as_ref().map_or(true, |b| new_sys > b.1) {
                            best = Some((p, new_sys, "h_n".to_string(), frac, t));
                        }
                    }
                }
            }

            let time_ms = t_iter.elapsed().as_secs_f64() * 1000.0;

            // 5. Take best step or stop
            match best {
                Some((new_polytope, new_sys, step_type, frac, t)) => {
                    let delta = new_sys - sys;
                    let cumulative = new_sys - starting_sys;

                    let row = IterationRow {
                        name: name.clone(),
                        source_dataset: source.clone(),
                        facet_count: f,
                        iteration: iter,
                        step_type: step_type.clone(),
                        t_fraction: frac,
                        t_actual: t,
                        sys_before: sys,
                        sys_after: new_sys,
                        delta_sys: delta,
                        starting_sys,
                        cumulative_delta: cumulative,
                        gradient_norm_h: sensitivity.gradient_norm_h,
                        gradient_norm_n: sensitivity.gradient_norm_n,
                        gradient_norm_hn: sensitivity.gradient_norm_hn,
                        vertex_count: new_polytope.vertices_f64().len(),
                        time_ms,
                    };
                    serde_json::to_writer(&mut iter_writer, &row).expect("write iteration");
                    writeln!(iter_writer).expect("newline");

                    if step_type == "h_only" {
                        n_h_only += 1;
                    } else {
                        n_h_n += 1;
                    }

                    current = new_polytope;
                    current_sys = new_sys;
                    n_iter = iter + 1;

                    if delta < CONVERGENCE_THRESHOLD {
                        converged = true;
                        break;
                    }
                }
                None => break,
            }
        }

        let total_delta = current_sys - starting_sys;
        let poly_time = t_poly.elapsed().as_secs_f64();
        total_iterations += n_iter;

        if current_sys > max_sys_achieved {
            max_sys_achieved = current_sys;
            max_sys_name = name.clone();
        }
        if converged {
            n_converged += 1;
        }

        println!(
            "iter={n_iter} (h:{n_h_only} n:{n_h_n}), sys: {:.6}→{:.6} (Δ={:.6}), {}{:.1}s",
            starting_sys,
            current_sys,
            total_delta,
            if converged { "converged, " } else { "" },
            poly_time,
        );
    }

    iter_writer.flush().expect("flush iterations");

    // =========================================================================
    // Phase 4: Gradient validity testing
    // =========================================================================

    println!("\nPhase 4: Gradient validity testing...\n");

    let validity_path = base_dir.join("sys-optimization/sys-optimization-validity.jsonl");
    let validity_file = File::create(&validity_path).expect("create validity JSONL");
    let mut validity_writer = BufWriter::new(validity_file);

    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut total_validity_evals = 0usize;
    let mut total_ok_within = 0usize;
    let mut total_ok_beyond = 0usize;
    let mut total_within = 0usize;
    let mut total_beyond = 0usize;

    for (idx, (name, _source, polytope)) in polytopes.iter().enumerate() {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();
        let vertex_count_orig = polytope.vertices_f64().len();

        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        // Recompute sensitivity (same as Phase 1)
        let instrumented = match ehz_capacity_instrumented(polytope) {
            Some(r) => r,
            None => {
                println!("SKIP");
                continue;
            }
        };
        let cap = instrumented.capacity;
        let vol = volume(polytope).expect("volume failed");
        let sys = cap * cap / (2.0 * vol);

        let sens = compute_sensitivity(polytope, vol, cap, sys, &instrumented);

        // --- Build directions to test ---
        struct Direction {
            g_h: Vec<f64>,
            g_n: Vec<Vector4<f64>>,
            dir_type: String,
            dir_index: usize,
            directional_deriv: f64,
        }

        let mut directions: Vec<Direction> = Vec::new();

        // Direction 1: gradient h-only (normalize d_sys_h, zero normal component)
        if sens.gradient_norm_h > 1e-15 {
            let scale = 1.0 / sens.gradient_norm_h;
            let g_h: Vec<f64> = sens.d_sys_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = vec![Vector4::zeros(); f];
            let dd = sens.gradient_norm_h; // ∇sys · (∇_h sys / |∇_h sys|) = |∇_h sys|
            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "gradient_h".to_string(),
                dir_index: 0,
                directional_deriv: dd,
            });
        }

        // Direction 2: gradient (h,n) (normalize combined)
        if sens.gradient_norm_hn > 1e-15 {
            let scale = 1.0 / sens.gradient_norm_hn;
            let g_h: Vec<f64> = sens.d_sys_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = sens.d_sys_n.iter().map(|v| v * scale).collect();
            let dd = sens.gradient_norm_hn; // ∇sys · (∇sys / |∇sys|) = |∇sys|
            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "gradient_hn".to_string(),
                dir_index: 0,
                directional_deriv: dd,
            });
        }

        // Directions 3..12: random (h,n) directions
        for dir_idx in 0..N_RANDOM_DIRECTIONS {
            // Random h-component: Gaussian in R^F
            let raw_h: Vec<f64> = (0..f)
                .map(|_| StandardNormal.sample(&mut rng))
                .collect();

            // Random n-component: Gaussian in R^4, projected to T_{n_k}S³
            let raw_n: Vec<Vector4<f64>> = (0..f)
                .map(|k| {
                    let v = Vector4::new(
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                    );
                    // Project to tangent space: v - (v·n_k) n_k
                    v - normals[k] * v.dot(&normals[k])
                })
                .collect();

            // Normalize the combined (h, n) direction to unit norm
            let norm_sq: f64 = raw_h.iter().map(|x| x * x).sum::<f64>()
                + raw_n.iter().map(|v| v.norm_squared()).sum::<f64>();
            let norm = norm_sq.sqrt();
            if norm < 1e-15 {
                continue;
            }
            let scale = 1.0 / norm;
            let g_h: Vec<f64> = raw_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = raw_n.iter().map(|v| v * scale).collect();

            // Directional derivative: ∇sys · δ
            let dd: f64 = sens
                .d_sys_h
                .iter()
                .zip(g_h.iter())
                .map(|(ds, d)| ds * d)
                .sum::<f64>()
                + sens
                    .d_sys_n
                    .iter()
                    .zip(g_n.iter())
                    .map(|(ds, d)| ds.dot(d))
                    .sum::<f64>();

            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "random".to_string(),
                dir_index: dir_idx,
                directional_deriv: dd,
            });
        }

        let mut n_evals = 0usize;

        // For each direction, compute t_max and test at multiple fractions
        for dir in &directions {
            // Compute step bound for this direction
            let t_max = if dir.g_n.iter().all(|v| v.norm() < 1e-15) {
                // Pure height direction
                compute_step_bound(polytope, &dir.g_h)
            } else {
                compute_step_bound_hn(polytope, &dir.g_h, &dir.g_n)
            };

            if t_max < 1e-15 {
                continue; // Degenerate direction
            }

            for &frac in VALIDITY_STEP_FRACTIONS {
                let t = frac * t_max;
                let beyond = frac > 1.0;

                // Predicted delta_sys from linear approximation
                let predicted_delta = t * dir.directional_deriv;

                // Actual: construct perturbed polytope, compute sys
                let (actual_delta, construction_ok, vertex_count_changed) =
                    if dir.g_n.iter().all(|v| v.norm() < 1e-15) {
                        // h-only step
                        match try_step_h_polytope(&normals, &heights, &dir.g_h, t) {
                            Some((new_poly, new_sys)) => {
                                let vc = new_poly.vertices_f64().len();
                                (new_sys - sys, true, vc != vertex_count_orig)
                            }
                            None => (f64::NAN, false, false),
                        }
                    } else {
                        // (h,n) step
                        match try_step_hn_polytope(&normals, &heights, &dir.g_h, &dir.g_n, t) {
                            Some((new_poly, new_sys)) => {
                                let vc = new_poly.vertices_f64().len();
                                (new_sys - sys, true, vc != vertex_count_orig)
                            }
                            None => (f64::NAN, false, false),
                        }
                    };

                let prediction_error = if actual_delta.is_finite() {
                    (actual_delta - predicted_delta).abs()
                } else {
                    f64::NAN
                };

                let relative_error = if actual_delta.is_finite() {
                    let denom = actual_delta.abs().max(1e-10);
                    (actual_delta - predicted_delta).abs() / denom
                } else {
                    f64::NAN
                };

                if beyond {
                    total_beyond += 1;
                    if construction_ok {
                        total_ok_beyond += 1;
                    }
                } else {
                    total_within += 1;
                    if construction_ok {
                        total_ok_within += 1;
                    }
                }

                let row = ValidityRow {
                    name: name.clone(),
                    facet_count: f,
                    starting_sys: sys,
                    direction_type: dir.dir_type.clone(),
                    direction_index: dir.dir_index,
                    t_fraction: frac,
                    t_actual: t,
                    t_max,
                    predicted_delta_sys: predicted_delta,
                    actual_delta_sys: actual_delta,
                    prediction_error,
                    relative_error,
                    directional_derivative: dir.directional_deriv,
                    construction_ok,
                    vertex_count_changed,
                    beyond_t_max: beyond,
                };

                serde_json::to_writer(&mut validity_writer, &row)
                    .expect("write validity row");
                validity_writer.write_all(b"\n").expect("write newline");
                n_evals += 1;
                total_validity_evals += 1;
            }
        }

        println!("{} directions × {} steps = {} evaluations", directions.len(), VALIDITY_STEP_FRACTIONS.len(), n_evals);
    }

    validity_writer.flush().expect("flush validity");

    // =========================================================================
    // Summary
    // =========================================================================

    let total_time = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 1–2)");
    println!("═══════════════════════════════════════════════");
    println!("Polytopes processed: {n_polytopes}");
    println!(
        "Favorable facets:    {total_favorable}/{total_facets} ({:.1}%)",
        100.0 * total_favorable as f64 / total_facets.max(1) as f64
    );
    println!("Steps that improved sys: {n_improved}");
    if best_sys_after > f64::NEG_INFINITY {
        println!(
            "Best sys (single step): {:.6} (from {:.6}, Δ={:.6})",
            best_sys_after,
            best_sys_before,
            best_sys_after - best_sys_before
        );
    }

    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 3 — iterative)");
    println!("═══════════════════════════════════════════════");
    println!("Total iterations:    {total_iterations}");
    println!(
        "Mean iterations:     {:.1}",
        total_iterations as f64 / n_polytopes.max(1) as f64
    );
    println!("Converged:           {n_converged}/{n_polytopes}");
    if max_sys_achieved > f64::NEG_INFINITY {
        println!(
            "Best sys (iterative): {:.6} ({})",
            max_sys_achieved, max_sys_name
        );
    }

    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 4 — validity testing)");
    println!("═══════════════════════════════════════════════");
    println!("Total evaluations:   {total_validity_evals}");
    println!(
        "Within t_max:        {total_ok_within}/{total_within} OK ({:.0}%)",
        100.0 * total_ok_within as f64 / total_within.max(1) as f64
    );
    println!(
        "Beyond t_max:        {total_ok_beyond}/{total_beyond} OK ({:.0}%)",
        100.0 * total_ok_beyond as f64 / total_beyond.max(1) as f64
    );

    println!("\nTotal time:          {total_time:.1}s");
    println!();
    println!("Output:");
    println!("  {}", sensitivity_path.display());
    println!("  {}", steps_path.display());
    println!("  {}", iterations_path.display());
    println!("  {}", validity_path.display());
}
