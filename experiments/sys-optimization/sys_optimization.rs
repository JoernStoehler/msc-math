//! Sys-optimization Phase 1–2: sensitivity analysis and finite gradient steps.
//!
//! Computes d(sys)/d(h_k) for polytopes from random-sweep and random-product-sweep,
//! then takes finite gradient steps bounded by combinatorial type preservation.
//!
//! Convention: The library (crates/) is stable. Experiment-specific variants
//! (instrumented HK2017) are self-contained in this binary. Library internals
//! needed by the variants are copied here with source references.
//!
//! KKT solver note: Uses the CURRENT library condition-number approach
//! (SVD_CONDITION_TAU = 1e-3), not the old gap-ratio approach from ablation.rs.
//!
//! Architecture:
//! 1. `cargo run --bin sys_optimization --release` generates datasets
//! 2. Writes to sys-optimization/sys-optimization-sensitivity.jsonl
//!    and sys-optimization/sys-optimization-steps.jsonl
//! 3. Python script reads JSONL, produces figures and stats
//!
//! Input: random-sweep/random-sweep.jsonl, random-product-sweep/random-product-sweep.jsonl
//! Filter: F ≤ 10 (HK2017 is exponential in F)

use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::{ehz_capacity, volume, Polytope4D, Skeleton};

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Heights are O(1), so steps beyond 100 are unreasonable.
const MAX_STEP_SIZE: f64 = 100.0;

/// Perturbation size for central finite differences (used for volume derivatives).
const FD_EPS: f64 = 1e-7;

/// Step fractions of t_max to evaluate.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

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
    d_vol: Vec<f64>,
    d_cap: Vec<f64>,
    d_sys: Vec<f64>,
    gradient_norm: f64,
    n_favorable: usize,
    t_max: f64,
    time_instrumented_ms: f64,
    time_sensitivity_ms: f64,
}

#[derive(Debug, Serialize)]
struct StepRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
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

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)})
/// Copied from crates/src/kkt.rs:59-69
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0_local(&normals[perm[i]], &normals[perm[j]]))
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
/// Copied from crates/src/kkt.rs:184-223
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
) -> Option<(Vec<f64>, f64, f64)> {
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
    let nu = x0[m + 4];
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_val = q_from_beta(normals, perm, &beta0);
        return Some((beta0, q_val, nu));
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
    // ν comes from x0 (pseudoinverse solution). The null-space search only
    // adjusts β, not the multipliers, so ν = x0[m+4] is correct.
    Some((beta_opt, q_val, nu))
}

/// SVD-only KKT solver, extended to return ν.
/// Copied from crates/src/kkt.rs:solve_kkt_svd_only, which is the production
/// path (profiling showed LU+SVD is slower than SVD-only; see kkt.rs docs).
///
/// Returns (β, Q, ν) where ν is the Lagrange multiplier for η^T β = 1.
fn solve_kkt_full(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64, f64)> {
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
    let normals = polytope.normals();
    let heights = polytope.heights();
    let mut adj = vec![vec![false; f]; f];
    for v in polytope.vertices() {
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

/// Build directed adjacency for the algebraic (Q-maximizing) ordering.
/// Matches the library's build_directed_adjacency_matrix convention:
/// adj[i][j] = vertex_adj[i][j] AND ω₀(n_j, n_i) >= 0
/// (algebraic consecutive pair (i,j) corresponds to physical F_j → F_i)
/// Copied from crates/src/algorithms/hk2017/mod.rs:216-229
fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let vertex_adj = build_adjacency_matrix(polytope);
    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            adj[i][j] = vertex_adj[i][j] && omega0_local(&normals[j], &normals[i]) >= 0.0;
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
    permutation: Vec<usize>, // algebraic (internal) ordering
    beta: Vec<f64>,
    q_value: f64,
    /// Lagrange multiplier for the η^T β = 1 constraint.
    /// Used for analytical capacity derivative via envelope theorem:
    /// dA/dh_k = ν · β_{i₀} / (2Q²) where perm[i₀] = k.
    nu: f64,
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
    let normals = polytope.normals();
    let heights = polytope.heights();
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

                if let Some((beta, q_val, nu)) = solve_kkt_full(normals, heights, perm) {
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
    d_vol: Vec<f64>,
    d_cap: Vec<f64>,
    d_sys: Vec<f64>,
    gradient_norm: f64,
    runner_up_gap: f64,
}

/// Compute d(vol)/d(h_k) via central finite differences.
fn compute_volume_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = normals.len();
    (0..f)
        .map(|k| {
            let mut h_plus = heights.to_vec();
            let mut h_minus = heights.to_vec();
            h_plus[k] += FD_EPS;
            h_minus[k] -= FD_EPS;

            let p_plus = match Polytope4D::new(normals.to_vec(), h_plus) {
                Ok(p) => p,
                Err(_) => return f64::NAN,
            };
            let p_minus = match Polytope4D::new(normals.to_vec(), h_minus) {
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
/// In code: `-nu * beta[i0] / (2 * q_sq)` because the code's ν is negative,
/// so negating it gives the positive result that matches the standard formula.
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
                    // See doc comment: dA/dh_k = ν·β_{i₀}/(2Q²) in standard convention.
                    // Code's ν < 0, so we negate: -ν·β/(2Q²) > 0.
                    -best_orbit.nu * best_orbit.beta[i0] / (2.0 * q_sq)
                }
                None => 0.0, // Facet not in orbit → height doesn't affect this orbit's action
            }
        })
        .collect()
}

/// Compute full sensitivity: d(sys)/d(h_k) via chain rule.
///
/// d(cap)/d(h_k) is computed analytically via the envelope theorem on the
/// winning orbit's KKT system. d(vol)/d(h_k) via central finite differences.
fn compute_sensitivity(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vol: f64,
    cap: f64,
    sys: f64,
    instrumented: &InstrumentedResult,
) -> SensitivityResult {
    let f = normals.len();
    let d_vol = compute_volume_derivatives(normals, heights);
    let d_cap = compute_capacity_derivatives_analytical(&instrumented.orbits[0], f);

    // Chain rule: d(sys)/d(h_k) = (1/vol) * [c * dc/dh_k - sys * dvol/dh_k]
    let d_sys: Vec<f64> = d_vol
        .iter()
        .zip(d_cap.iter())
        .map(|(&dv, &dc)| {
            if dv.is_nan() || dc.is_nan() {
                f64::NAN
            } else {
                (cap * dc - sys * dv) / vol
            }
        })
        .collect();

    let gradient_norm = d_sys
        .iter()
        .filter(|x| !x.is_nan())
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();

    let runner_up_gap = if instrumented.orbits.len() >= 2 {
        instrumented.orbits[1].action - instrumented.orbits[0].action
    } else {
        f64::INFINITY
    };

    SensitivityResult {
        d_vol,
        d_cap,
        d_sys,
        gradient_norm,
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
    let normals = polytope.normals();
    let heights = polytope.heights();
    let vertices = polytope.vertices();
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

// ============================================================================
// Gradient step evaluation
// ============================================================================

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

    match Polytope4D::new(normals.to_vec(), new_heights) {
        Ok(new_polytope) => {
            let new_vol = volume(&new_polytope).unwrap_or(f64::NAN);
            // Use library ehz_capacity for the step evaluation (not instrumented — faster)
            let new_cap = ehz_capacity(&new_polytope)
                .map(|r| r.capacity)
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
                t_fraction: 0.0, // filled in by caller
                t_actual: t,
                old_sys,
                new_sys,
                delta_sys: new_sys - old_sys,
                new_volume: new_vol,
                new_capacity: new_cap,
                vertex_count_old: old_vertex_count,
                vertex_count_new: new_polytope.vertices().len(),
                construction_ok: true,
            }
        }
        Err(e) => {
            eprintln!("    Step t={t:.6} failed: {e}");
            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
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

        match Polytope4D::new(normals, row.heights) {
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

    println!("Sys-optimization Phase 1–2: sensitivity analysis + finite gradient steps\n");

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
        let normals = polytope.normals();
        let heights = polytope.heights();

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
        let sensitivity = compute_sensitivity(normals, heights, vol, cap, sys, &instrumented);
        let time_sensitivity_ms = t_sens.elapsed().as_secs_f64() * 1000.0;

        // Count favorable facets: d_sys > 0 means increasing h_k improves sys,
        // d_sys < 0 means decreasing h_k improves sys. Either is "favorable".
        let n_favorable = sensitivity
            .d_sys
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
        // Direction: steepest ascent = d_sys itself
        let t_max = if sensitivity.gradient_norm > 1e-15 {
            compute_step_bound(polytope, &sensitivity.d_sys)
        } else {
            0.0
        };

        println!(
            "orbits={}, sys={:.6}, |∇sys|={:.4e}, t_max={:.4e}, {:.0}ms",
            instrumented.orbits.len(),
            sys,
            sensitivity.gradient_norm,
            t_max,
            time_instrumented_ms + time_sensitivity_ms
        );

        // Write sensitivity row
        let normals_raw: Vec<[f64; 4]> = normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect();
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
            d_vol: sensitivity.d_vol,
            d_cap: sensitivity.d_cap,
            d_sys: sensitivity.d_sys.clone(),
            gradient_norm: sensitivity.gradient_norm,
            n_favorable,
            t_max,
            time_instrumented_ms,
            time_sensitivity_ms,
        };
        serde_json::to_writer(&mut sens_writer, &sens_row).expect("write sensitivity");
        writeln!(sens_writer).expect("newline");

        // =========================================================================
        // Phase 2: Gradient steps
        // =========================================================================

        if t_max <= 0.0 || sensitivity.gradient_norm < 1e-15 {
            continue;
        }

        let vertex_count_old = polytope.vertices().len();

        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            let mut step_row = evaluate_gradient_step(
                normals,
                heights,
                &sensitivity.d_sys,
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

    sens_writer.flush().expect("flush sensitivity");
    steps_writer.flush().expect("flush steps");

    // =========================================================================
    // Summary
    // =========================================================================

    let total_time = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════");
    println!("Summary");
    println!("═══════════════════════════════════════════════");
    println!("Polytopes processed: {n_polytopes}");
    println!(
        "Favorable facets:    {total_favorable}/{total_facets} ({:.1}%)",
        100.0 * total_favorable as f64 / total_facets.max(1) as f64
    );
    println!("Steps that improved sys: {n_improved}");
    if best_sys_after > f64::NEG_INFINITY {
        println!(
            "Best sys achieved:   {:.6} (from {:.6}, Δ={:.6})",
            best_sys_after,
            best_sys_before,
            best_sys_after - best_sys_before
        );
    }
    println!("Total time:          {total_time:.1}s");
    println!();
    println!("Output:");
    println!("  {}", sensitivity_path.display());
    println!("  {}", steps_path.display());
}
