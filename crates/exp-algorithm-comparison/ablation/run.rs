//! Ablation study: compare HK2017 algorithm variants on a fixed dataset.
//!
//! A-axis variants: A0 (unpruned), A1 (vertex adjacency),
//! A2 (directed ω₀), A3 (Reeb-flow feasibility).
//!
//! Convention: The library (crates/) is stable. New variants are implemented as
//! self-contained code in this binary. Library internals needed by the new variants
//! are copied here (marked with source references). If a variant is later promoted
//! to production, it enters the library then.
//!
//! KKT solver note: The copied `solve_kkt_svd_path` uses the old gap-ratio approach
//! (`SVD_GAP_THRESHOLD = 100.0`), not the library's current condition-number approach
//! (`SVD_CONDITION_TAU = 1e-3`). This is intentional: all variants use the same solver
//! for apples-to-apples comparison. Correctness is validated by agreement with A0.
//!
//! Architecture:
//! 1. `cargo run --bin ablation --release` generates the ablation dataset
//! 2. Writes to ablation/ablation.jsonl
//! 3. Python script reads JSONL, checks agreement, plots timing comparison
//!
//! Dataset:
//! - Random generic polytopes: 5 per F ∈ {5, 6, 7, 8, 9, 10} (seed 42)
//! - Random Lagrangian products: 5 per pair (3×3), (3×4), (4×4) (same seed)
//! - Regression cases: (3,4) θ=0°, (4,4) θ=0°, hypercube (always included)
//! - Non-simple polytopes: bipyramids over 3-polytopes, cut simplices at 3 depths
//!
//! Output format: one JSONL entry per (polytope, variant).
//! Each entry: {polytope_name, variant, group, facet_count, normals, heights,
//!              capacity, capacity_uncertain, iterations, time_ms}

use nalgebra::{DMatrix, DVector, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::geom::polygon::{random_polygon_2d, regular_polygon_2d};
use symplectic::random::generate_random_polytopes;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::algorithms::hk2017::{ehz_capacity_unpruned, EhzResult};
use symplectic::geom::known_polytopes;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polytope::Polytope4D;

const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;
const N_PER_GROUP: usize = 5;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct AblationEntry {
    polytope_name: String,
    variant: String, // "a0_unpruned" | "a1_vertex_adj" | "a2_omega_directed" | "a3_reeb_feasible"
    group: String,   // "random_generic" | "random_lagrangian" | "regression"
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    capacity: f64,
    capacity_uncertain: f64,
    iterations: u64,
    time_ms: f64,
}

// ============================================================================
// Copied from library — KKT solver (crates/library/src/kkt.rs)
//
// These are exact copies of pub(crate) functions that can't be imported from
// the experiment binary. Source commit: see crates/library/src/kkt.rs header.
// ============================================================================

/// Minimum β_i value to consider a solution valid.
/// Copied from crates/library/src/kkt.rs:12
const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid.
/// Copied from crates/library/src/kkt.rs:15
const EPS_Q_POSITIVE: f64 = 1e-15;

/// Floor for SVD singular values.
/// Copied from crates/library/src/kkt.rs:18
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Gap ratio threshold for rank detection.
/// Copied from crates/library/src/kkt.rs:46
const SVD_GAP_THRESHOLD: f64 = 100.0;

/// Maximum acceptable residual norm.
/// Copied from crates/library/src/kkt.rs:49
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Facet incidence tolerance.
/// Copied from crates/library/src/constants.rs
const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// Tolerance for directed adjacency ω₀ check. Conservative: allow transitions
/// where ω₀(n_i, n_j) ≥ -EPS_DIRECTED to avoid discarding valid orbits.
const EPS_DIRECTED: f64 = 1e-8;

/// ω₀(u, v) = u_q1·v_p1 - u_p1·v_q1 + u_q2·v_p2 - u_p2·v_q2
/// Copied from crates/library/src/geom/symplectic.rs:28
fn omega0(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) = (1/2) β^T H β
/// Copied from crates/library/src/kkt.rs — Q > 0 for permutations in positive Reeb direction.
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

/// Search 1D null space for β > 0 solution.
/// Copied from crates/library/src/kkt.rs:75-116
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
/// Copied from crates/library/src/kkt.rs:121-166
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
/// Copied from crates/library/src/kkt.rs:178-217
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

/// SVD path with gap-based rank detection.
/// Copied from crates/library/src/kkt.rs:231-325
fn solve_kkt_svd_path(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
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
    let floor = max_sv * EPS_SVD_FLOOR;
    let nonzero = sv.iter().filter(|&&s| s > floor).count();
    let mut rank = nonzero;
    for i in (1..nonzero).rev() {
        if sv[i - 1] / sv[i] > SVD_GAP_THRESHOLD {
            rank = i;
            break;
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
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_val = q_from_beta(normals, perm, &beta0);
        return Some((beta0, q_val));
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
    Some((beta_opt, q_val))
}

/// LU fast path + SVD fallback. Used by A2 and A3.
/// Copied from crates/library/src/kkt.rs:346-373
fn solve_kkt_full(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
    let m = perm.len();
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    let lu = kkt.clone().full_piv_lu();
    if lu.is_invertible() {
        if let Some(solution) = lu.solve(&rhs) {
            let residual = (&kkt * &solution - &rhs).norm();
            if residual <= EPS_KKT_RESIDUAL {
                let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();
                if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
                    let q_val = q_from_beta(normals, perm, &beta);
                    return Some((beta, q_val));
                }
            }
        }
    }
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}

// ============================================================================
// Copied from library — combinatorial infrastructure
// (crates/library/src/algorithms/hk2017/mod.rs, permutations.rs)
// ============================================================================

/// Generate all C(n,k) combinations in lexicographic order.
/// Copied from crates/library/src/algorithms/hk2017/mod.rs:135-158
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
/// Copied from crates/library/src/algorithms/hk2017/permutations.rs:22-35
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
/// Copied from crates/library/src/algorithms/hk2017/permutations.rs:38-57
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

/// Convert a DMatrix<bool> to Vec<Vec<bool>> for use with ablation-specific
/// adjacency infrastructure (is_adjacent_cycle, build_a3_adjacency, etc.).
fn dmatrix_to_vec(adj: &DMatrix<bool>) -> Vec<Vec<bool>> {
    let f = adj.nrows();
    (0..f)
        .map(|i| (0..f).map(|j| adj[(i, j)]).collect())
        .collect()
}

/// Check if a cyclic permutation forms an adjacent cycle in the given graph.
/// Copied from crates/library/src/algorithms/hk2017/mod.rs:185-188
fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// A2: directed ω₀ adjacency (positive Reeb direction)
//
// The physical Reeb orbit traverses facets in a specific cyclic order. For a
// transition from F_i to F_j, the Reeb flow on F_i must carry the orbit toward
// the ridge F_i ∩ F_j, requiring ω₀(n_i, n_j) ≥ 0.
//
// Directed edge i→j: ω₀(n_i, n_j) ≥ -EPS (conservative, avoids discarding
// valid transitions near zero).
// Combined with vertex adjacency: dir_adj[i][j] = vertex_adj[i][j] AND ω₀ ≥ -EPS.
//
// The ablation empirically checks that A2 agrees with A0 on all test polytopes.
// ============================================================================

/// Build directed adjacency for positive Reeb direction.
/// Edge i→j allowed iff vertex-adjacent AND ω₀(n_i, n_j) ≥ -EPS.
fn build_directed_adjacency(
    vertex_adj: &DMatrix<bool>,
    normals: &[Vector4<f64>],
) -> Vec<Vec<bool>> {
    let f = normals.len();
    let mut dir_adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            if i == j {
                continue;
            }
            if vertex_adj[(i, j)] {
                dir_adj[i][j] = omega0(&normals[i], &normals[j]) >= -EPS_DIRECTED;
            }
        }
    }
    dir_adj
}

// ============================================================================
// Generic capacity loop for local variants
// ============================================================================

/// Intermediate best candidate: (action, subset, permutation, beta).
type Candidate = (f64, Vec<usize>, Vec<usize>, Vec<f64>);

/// Capacity computation with configurable adjacency and solver.
///
/// Same algorithm as `ehz_capacity`, but parameterized:
/// - `adj`: adjacency matrix (undirected for A1, directed for A2/A3)
/// - `solver`: KKT solver function
fn ehz_capacity_unpruned_with(
    polytope: &Polytope4D,
    adj: &[Vec<bool>],
    solver: fn(&[Vector4<f64>], &[f64], &[usize]) -> Option<(Vec<f64>, f64)>,
) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_adjacent_cycle(perm, adj) {
                    return;
                }
                iterations += 1;

                if let Some((beta, q_val)) = solver(&normals, &heights, perm) {
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    if beta_min > EPS_BETA_POSITIVE {
                        let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_certified = Some((
                                action,
                                subset.clone(),
                                perm.to_vec(),
                                beta.clone(),
                            ));
                        }
                    }

                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_uncertain =
                                Some((action, subset.clone(), perm.to_vec(), beta));
                        }
                    }
                }
            });
        }
    }

    let certified = best_certified?;
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);
    Some(EhzResult {
        result: symplectic::algorithms::capacity_accumulator::CapacityResult {
            capacity: certified.0,
            capacity_uncertain: uncertain_cap,
            best_permutation: certified.2,
            best_beta: certified.3,
            iterations,
        },
        best_subset: certified.1,
    })
}

/// A1: undirected vertex adjacency + standard LU/SVD solver.
/// This is the ablation's own A1 — independent of the library's `ehz_capacity`,
/// which was promoted to A2-level pruning. Uses `solve_kkt_full` for apples-to-apples
/// comparison with A2 and A3.
fn ehz_capacity_unpruned_a1(polytope: &Polytope4D) -> Option<EhzResult> {
    let vertex_adj = dmatrix_to_vec(polytope.vertex_adjacency());
    ehz_capacity_unpruned_with(polytope, &vertex_adj, solve_kkt_full)
}

/// A2: directed ω₀ adjacency + standard LU/SVD solver.
fn ehz_capacity_unpruned_a2(polytope: &Polytope4D) -> Option<EhzResult> {
    let vertex_adj = polytope.vertex_adjacency();
    let normals: Vec<Vector4<f64>> = polytope.dual_vertices_f64().iter().map(|a| a / a.norm()).collect();
    let dir_adj = build_directed_adjacency(vertex_adj, &normals);
    ehz_capacity_unpruned_with(polytope, &dir_adj, solve_kkt_full)
}

// ============================================================================
// A3: full Reeb-flow feasibility
//
// A3 strengthens A2 by checking that the physical orbit transition at a ridge
// is not blocked by other facets' halfspace constraints.
//
// For physical transition F_src → F_dst at x ∈ F_src ∩ F_dst:
//   (1) x − εR_src ∈ F_src  (backward on F_src: orbit was on F_src approaching x)
//   (2) x + εR_dst ∈ F_dst  (forward on F_dst: orbit departs on F_dst)
//
// These reduce to: ∃x on the ridge with n_k·x < h_k for each "blocking" facet k,
// where B = { k ∉ {src,dst} : ω₀(n_src, n_k) < 0 OR ω₀(n_dst, n_k) > 0 }.
//
// Implementation: reduce to 2D LP feasibility via SVD + Fourier-Motzkin elimination.
// Precomputed once per polytope: O(F⁴) total, negligible vs the exponential search.
//
// Convention: adj[i][j] = phys_feasible(i → j) (positive Reeb direction).
// ============================================================================

/// Check 2D feasibility of half-plane constraints via Fourier-Motzkin elimination.
///
/// Each constraint is (a, b, c) meaning a·s + b·t ≤ c.
/// Returns true iff ∃(s,t) satisfying all constraints.
fn fourier_motzkin_2d_feasible(constraints: &[(f64, f64, f64)]) -> bool {
    let eps = 1e-15;

    // Partition by sign of b (coefficient of t)
    let mut upper_t: Vec<(f64, f64)> = Vec::new(); // t ≤ slope·s + intercept
    let mut lower_t: Vec<(f64, f64)> = Vec::new(); // t ≥ slope·s + intercept
    let mut s_bounds: Vec<(f64, f64)> = Vec::new(); // coeff·s ≤ rhs

    for &(a, b, c) in constraints {
        if b.abs() < eps {
            s_bounds.push((a, c));
        } else if b > 0.0 {
            upper_t.push((-a / b, c / b));
        } else {
            lower_t.push((-a / b, c / b));
        }
    }

    // Eliminate t: for each (lower, upper) pair, derive s constraint
    // lower: t ≥ sl·s + il,  upper: t ≤ su·s + iu
    // → (sl - su)·s ≤ iu - il
    for &(sl, il) in &lower_t {
        for &(su, iu) in &upper_t {
            s_bounds.push((sl - su, iu - il));
        }
    }

    // Check 1D feasibility of s
    let mut s_lo = f64::NEG_INFINITY;
    let mut s_hi = f64::INFINITY;

    for &(coeff, rhs) in &s_bounds {
        if coeff.abs() < eps {
            if rhs < -eps {
                return false; // 0·s ≤ negative: infeasible
            }
        } else if coeff > 0.0 {
            s_hi = s_hi.min(rhs / coeff);
        } else {
            s_lo = s_lo.max(rhs / coeff);
        }
    }

    s_lo <= s_hi + eps
}

/// Check if a physical transition F_src → F_dst is feasible.
///
/// Parameterizes the ridge F_src ∩ F_dst as a 2D affine subspace (via SVD),
/// projects all halfspace constraints to 2D, and checks feasibility.
/// Blocking facets (where the Reeb flow would exit K) get strict inequality.
fn is_physical_transition_feasible(
    normals: &[Vector4<f64>],
    heights: &[f64],
    src: usize,
    dst: usize,
) -> bool {
    let f = normals.len();

    // Blocking set: k where backward flow on F_src or forward flow on F_dst exits K through F_k
    let blocking: Vec<bool> = (0..f)
        .map(|k| {
            if k == src || k == dst {
                return false;
            }
            let omega_src_k = omega0(&normals[src], &normals[k]);
            let omega_dst_k = omega0(&normals[dst], &normals[k]);
            omega_src_k < 0.0 || omega_dst_k > 0.0
        })
        .collect();

    // If no blocking facets, any point on the ridge works (A1+A2 already ensure the ridge exists)
    if !blocking.iter().any(|&b| b) {
        return true;
    }

    // Parameterize the ridge: {x : n_src·x = h_src, n_dst·x = h_dst}
    let n_src = &normals[src];
    let n_dst = &normals[dst];

    let a_mat = DMatrix::from_row_slice(
        2,
        4,
        &[
            n_src[0], n_src[1], n_src[2], n_src[3], n_dst[0], n_dst[1], n_dst[2], n_dst[3],
        ],
    );
    let b_vec = DVector::from_row_slice(&[heights[src], heights[dst]]);

    // Particular solution via least-norm: x₀ = Aᵀ(AAᵀ)⁻¹b
    // AAᵀ is 2×2, always invertible for non-parallel normals
    let aat = &a_mat * a_mat.transpose(); // 2×2
    let aat_inv = match aat.try_inverse() {
        Some(inv) => inv,
        None => return false, // Degenerate: normals nearly parallel
    };
    let lambda = &aat_inv * &b_vec; // 2×1
    let x0_dv = a_mat.transpose() * lambda; // 4×1
    let x0 = Vector4::new(x0_dv[0], x0_dv[1], x0_dv[2], x0_dv[3]);

    // Null space of A (2×4, rank 2): use eigendecomposition of AᵀA (4×4 symmetric)
    // Eigenvectors with eigenvalue ≈ 0 span the null space
    let ata = a_mat.transpose() * &a_mat; // 4×4
    let eigen = ata.symmetric_eigen();
    let mut null_vecs: Vec<Vector4<f64>> = Vec::new();
    for col in 0..4 {
        if eigen.eigenvalues[col].abs() < 1e-10 {
            null_vecs.push(Vector4::new(
                eigen.eigenvectors[(0, col)],
                eigen.eigenvectors[(1, col)],
                eigen.eigenvectors[(2, col)],
                eigen.eigenvectors[(3, col)],
            ));
        }
    }
    if null_vecs.len() < 2 {
        return false; // Unexpected: 2×4 rank-2 matrix should have 2D null space
    }
    let u1 = null_vecs[0];
    let u2 = null_vecs[1];

    // Project constraints to 2D: n_k · (x₀ + s·u₁ + t·u₂) ≤ rhs
    let delta = EPS_FACET_INCIDENCE; // margin for strict inequality

    let mut constraints: Vec<(f64, f64, f64)> = Vec::new();
    for k in 0..f {
        if k == src || k == dst {
            continue;
        }
        let a_k = normals[k].dot(&u1);
        let b_k = normals[k].dot(&u2);
        let slack = heights[k] - normals[k].dot(&x0);
        let c_k = if blocking[k] { slack - delta } else { slack };
        constraints.push((a_k, b_k, c_k));
    }

    fourier_motzkin_2d_feasible(&constraints)
}

/// Build A3 adjacency matrix: A2 + Reeb-flow feasibility check.
///
/// adj[i][j] = true iff physical transition F_i → F_j is feasible.
fn build_a3_adjacency(
    a2_adj: &[Vec<bool>],
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Vec<Vec<bool>> {
    let f = normals.len();
    let mut a3_adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            if i == j || !a2_adj[i][j] {
                continue;
            }
            a3_adj[i][j] = is_physical_transition_feasible(normals, heights, i, j);
        }
    }
    a3_adj
}

/// A3: full Reeb-flow feasibility + standard LU/SVD solver.
fn ehz_capacity_unpruned_a3(polytope: &Polytope4D) -> Option<EhzResult> {
    let vertex_adj = polytope.vertex_adjacency();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let a2_adj = build_directed_adjacency(vertex_adj, &normals);
    let a3_adj = build_a3_adjacency(&a2_adj, &normals, &heights);
    ehz_capacity_unpruned_with(polytope, &a3_adj, solve_kkt_full)
}

// ============================================================================
// Variant definitions
// ============================================================================

struct Variant {
    name: &'static str,
    run: fn(&Polytope4D) -> Option<EhzResult>,
}

const VARIANTS: &[Variant] = &[
    Variant {
        name: "a0_unpruned",
        run: ehz_capacity_unpruned,
    },
    Variant {
        name: "a1_vertex_adj",
        run: ehz_capacity_unpruned_a1,
    },
    Variant {
        name: "a2_omega_directed",
        run: ehz_capacity_unpruned_a2,
    },
    Variant {
        name: "a3_reeb_feasible",
        run: ehz_capacity_unpruned_a3,
    },
];

// ============================================================================
// Non-simple polytope constructors
// ============================================================================

/// Construct a bipyramid over a 3-polytope P ⊂ R³ with apices at (0,0,0,±a).
///
/// P is given by its H-representation: {x ∈ R³ : nᵢ·x ≤ hᵢ} with unit normals.
/// The bipyramid B ⊂ R⁴ has 2k facets (k = number of 3D faces). Each apex lies
/// on k facets, so B is non-simple whenever k ≥ 5.
///
/// Derivation: B = {x ∈ R⁴ : nᵢ·x₁₂₃ + (hᵢ/a)x₄ ≤ hᵢ and nᵢ·x₁₂₃ − (hᵢ/a)x₄ ≤ hᵢ ∀i}.
fn make_bipyramid(
    normals_3d: &[[f64; 3]],
    heights_3d: &[f64],
    apex_height: f64,
) -> Polytope4D {
    let k = normals_3d.len();
    let mut normals = Vec::with_capacity(2 * k);
    let mut heights = Vec::with_capacity(2 * k);

    for i in 0..k {
        let [nx, ny, nz] = normals_3d[i];
        let h = heights_3d[i];
        let c = h / apex_height;
        let norm4 = (nx * nx + ny * ny + nz * nz + c * c).sqrt();

        // Upper facet: outward normal (nᵢ, +c), tight at upper apex (0,0,0,a)
        normals.push(Vector4::new(nx / norm4, ny / norm4, nz / norm4, c / norm4));
        heights.push(h / norm4);

        // Lower facet: outward normal (nᵢ, −c), tight at lower apex (0,0,0,−a)
        normals.push(Vector4::new(nx / norm4, ny / norm4, nz / norm4, -c / norm4));
        heights.push(h / norm4);
    }

    Polytope4D::from_f64(
        normals.iter().zip(heights.iter()).map(|(n, &h)| n / h).collect(),
    ).expect("bipyramid construction")
}

/// Construct a cut 4-simplex: standard simplex intersected with x₁ + c·x₂ ≤ 2.
///
/// The cutting plane always passes through v₀=(2,0,0,0), making v₀ lie on 5 facets
/// (non-simple). Parameter `cut_slope` controls the cut depth: larger values remove
/// more material near v₁=(0,2,0,0).
fn make_cut_simplex(cut_slope: f64) -> Polytope4D {
    let s19 = 19.0_f64.sqrt();
    let norm = (1.0 + cut_slope * cut_slope).sqrt();
    let normals = vec![
        Vector4::new(-4.0, 1.0, 1.0, 1.0) / s19, // F₀: opposite v₀
        Vector4::new(1.0, -4.0, 1.0, 1.0) / s19,  // F₁: opposite v₁
        Vector4::new(1.0, 1.0, -4.0, 1.0) / s19,  // F₂: opposite v₂
        Vector4::new(1.0, 1.0, 1.0, -4.0) / s19,  // F₃: opposite v₃
        Vector4::new(1.0, 1.0, 1.0, 1.0) / 2.0,   // F₄: opposite v₄
        Vector4::new(1.0, cut_slope, 0.0, 0.0) / norm, // F₅: cutting plane
    ];
    let heights = vec![
        2.0 / s19,
        2.0 / s19,
        2.0 / s19,
        2.0 / s19,
        1.0,
        2.0 / norm,
    ];
    Polytope4D::from_f64(
        normals.iter().zip(heights.iter()).map(|(n, &h)| n / h).collect(),
    ).expect("cut simplex construction")
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ablation/ablation.jsonl");

    println!("Ablation study — A-axis (adjacency pruning)\n");
    println!("Variants: A0 (unpruned), A1 (vertex adj), A2 (directed ω₀), A3 (Reeb feasibility)");
    println!("Seed: {SEED}, h ∈ [{H_MIN}, {H_MAX}]\n");

    // =========================================================================
    // Build test polytope set
    // =========================================================================

    // (name, group, polytope, expected_capacity)
    let mut polytopes: Vec<(String, String, Polytope4D, Option<f64>)> = Vec::new();

    // --- Part 1: Random generic polytopes, F=5..10 ---
    println!("Part 1: Random generic polytopes (F=5..10, {N_PER_GROUP} each)...");
    for f in [5usize, 6, 7, 8, 9, 10] {
        let ps = generate_random_polytopes(N_PER_GROUP, f, H_MIN, H_MAX, &mut rng);
        for (i, p) in ps.into_iter().enumerate() {
            polytopes.push((
                format!("random_F{f}_{i}"),
                "random_generic".to_string(),
                p,
                None,
            ));
        }
        println!("  F={f}: {N_PER_GROUP} polytopes");
    }

    // --- Part 2: Random Lagrangian products, (3×3)/(3×4)/(4×4) ---
    println!("\nPart 2: Random Lagrangian products ({N_PER_GROUP} per pair)...");
    for (n, m) in [(3usize, 3usize), (3, 4), (4, 4)] {
        for i in 0..N_PER_GROUP {
            let p = loop {
                let (qn, qh) = random_polygon_2d(n, H_MIN, H_MAX, &mut rng);
                let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
                if let Ok(poly) = lagrangian_product(&qn, &qh, &pn, &ph) {
                    break poly;
                }
            };
            polytopes.push((
                format!("random_lagrangian_{n}x{m}_{i}"),
                "random_lagrangian".to_string(),
                p,
                None,
            ));
        }
        println!("  ({n}×{m}): {N_PER_GROUP} polytopes (F={})", n + m);
    }

    // --- Part 3: Regression cases ---
    println!("\nPart 3: Regression cases...");

    // (3,4) θ=0° — null-space fix case (before fix: returned None)
    // Expected: 3√2/2 ≈ 2.121 (triangle circumradius=1, square circumradius=1)
    {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = lagrangian_product(&qn, &qh, &pn, &ph).expect("(3,4) construction");
        let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0;
        polytopes.push((
            "regression_34_theta0".to_string(),
            "regression".to_string(),
            p,
            Some(expected),
        ));
        println!("  (3,4) θ=0°: F=7, expected {expected:.6}");
    }

    // (4,4) θ=0° — SVD gap-threshold case (LU falls through to SVD for degenerate KKT)
    // Expected: 2.0 (square circumradius=1, this is the hypercube scaled by 1/√2)
    {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = lagrangian_product(&qn, &qh, &pn, &ph).expect("(4,4) construction");
        polytopes.push((
            "regression_44_theta0".to_string(),
            "regression".to_string(),
            p,
            Some(2.0),
        ));
        println!("  (4,4) θ=0°: F=8, expected 2.0");
    }

    // Hypercube [-1,1]^4 — LU residual check case (no SVD needed)
    // Expected: 4.0 (from HK2019 Ex 4.6)
    {
        let kp = known_polytopes::hypercube();
        println!(
            "  hypercube:  F={}, expected {}",
            kp.polytope.facet_count(),
            kp.capacity
        );
        polytopes.push((
            "regression_hypercube".to_string(),
            "regression".to_string(),
            kp.polytope.clone(),
            Some(kp.capacity),
        ));
    }

    // Cut simplex — non-simple polytope where A3 may prune beyond A2.
    // See Example [ex:a3-prunes] in ablation.tex.
    //
    // Construction: 4-simplex with vertices v₀=(2,0,0,0), v₁=(0,2,0,0),
    // v₂=(0,0,2,0), v₃=(0,0,0,2), v₄=(-2,-2,-2,-2) (centroid at origin),
    // intersected with the halfspace x₁ + 2x₂ ≤ 2 (passes through v₀,
    // cuts off v₁). Result: 6 facets, 7 vertices, v₀ on 5 facets (non-simple).
    {
        let p = make_cut_simplex(2.0);
        println!(
            "  cut simplex: F={}, non-simple (v₀ on 5 facets)",
            p.facet_count()
        );
        polytopes.push((
            "regression_cut_simplex".to_string(),
            "regression".to_string(),
            p,
            None, // capacity not known analytically
        ));
    }

    // --- Part 4: Non-simple polytopes (A2≠A3 regime) ---
    println!("\nPart 4: Non-simple polytopes (bipyramids + cut simplices)...");

    // Bipyramid over triangular prism (5 faces in R³ → 10 facets in R⁴).
    // Each apex lies on 5 facets → non-simple.
    //
    // Triangular prism centered at origin, height ±1 along z-axis,
    // equilateral triangle cross-section with circumradius 1.
    {
        let s3_2 = 3.0_f64.sqrt() / 2.0;
        let normals_3d: &[[f64; 3]] = &[
            [0.0, 0.0, -1.0],   // bottom
            [0.0, 0.0, 1.0],    // top
            [0.5, s3_2, 0.0],   // side 1 (unit: √(0.25 + 0.75) = 1)
            [-1.0, 0.0, 0.0],   // side 2
            [0.5, -s3_2, 0.0],  // side 3
        ];
        let heights_3d: &[f64] = &[1.0, 1.0, 0.5, 0.5, 0.5];
        let p = make_bipyramid(normals_3d, heights_3d, 1.5);
        println!(
            "  bipyramid (triangular prism): F={}, non-simple (apices on 5 facets)",
            p.facet_count()
        );
        polytopes.push((
            "nonsimple_bipyramid_triprism".to_string(),
            "non_simple".to_string(),
            p,
            None,
        ));
    }

    // Bipyramid over square pyramid (5 faces in R³ → 10 facets in R⁴).
    //
    // Square pyramid: base at z = -0.4 with vertices (±1, ±1, -0.4),
    // apex at (0, 0, 1.6). Centroid at origin.
    {
        let s5 = 5.0_f64.sqrt();
        let normals_3d: &[[f64; 3]] = &[
            [0.0, 0.0, -1.0],       // base
            [2.0 / s5, 0.0, 1.0 / s5],   // side 1
            [0.0, -2.0 / s5, 1.0 / s5],  // side 2
            [-2.0 / s5, 0.0, 1.0 / s5],  // side 3
            [0.0, 2.0 / s5, 1.0 / s5],   // side 4
        ];
        let heights_3d: &[f64] = &[0.4, 1.6 / s5, 1.6 / s5, 1.6 / s5, 1.6 / s5];
        let p = make_bipyramid(normals_3d, heights_3d, 1.5);
        println!(
            "  bipyramid (square pyramid): F={}, non-simple (apices on 5 facets)",
            p.facet_count()
        );
        polytopes.push((
            "nonsimple_bipyramid_sqpyr".to_string(),
            "non_simple".to_string(),
            p,
            None,
        ));
    }

    // Cut simplices at 3 depths: x₁ + c·x₂ ≤ 2 through v₀=(2,0,0,0).
    // Larger c removes more material near v₁=(0,2,0,0).
    // Each produces F=6, v₀ on 5 facets (non-simple).
    // Different c values change the ω₀ pattern of the cutting facet,
    // giving different A2/A3 blocking sets.
    for (label, slope) in [("shallow", 1.5), ("medium", 2.5), ("deep", 4.0)] {
        let p = make_cut_simplex(slope);
        println!(
            "  cut simplex ({label}, c={slope}): F={}, non-simple",
            p.facet_count()
        );
        polytopes.push((
            format!("nonsimple_cut_simplex_{label}"),
            "non_simple".to_string(),
            p,
            None,
        ));
    }

    let n_polytopes = polytopes.len();
    let n_entries = n_polytopes * VARIANTS.len();
    println!(
        "\nTotal: {n_polytopes} polytopes × {} variants = {n_entries} entries\n",
        VARIANTS.len()
    );

    // =========================================================================
    // Run ablation variants
    // =========================================================================

    let mut entries: Vec<AblationEntry> = Vec::with_capacity(n_entries);
    let mut n_disagreements = 0usize;
    let mut n_failures = 0usize;

    for (polytope_name, group, polytope, expected) in &polytopes {
        let duals_raw: Vec<[f64; 4]> = polytope
            .dual_vertices_f64()
            .iter()
            .map(|a| [a[0], a[1], a[2], a[3]])
            .collect();
        let f = polytope.facet_count();

        // Collect results for this polytope to check agreement
        let mut capacities: Vec<(String, f64)> = Vec::new();

        for variant in VARIANTS {
            let t_start = Instant::now();
            let result = (variant.run)(polytope);
            let time_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            match result {
                None => {
                    eprintln!(
                        "  FAILURE: {} / {} returned None",
                        polytope_name, variant.name
                    );
                    n_failures += 1;
                }
                Some(r) => {
                    capacities.push((variant.name.to_string(), r.result.capacity));

                    // Check against known expected capacity
                    if let Some(exp) = expected {
                        if (r.result.capacity - exp).abs() > 1e-5 {
                            eprintln!(
                                "  WRONG: {} / {}: got {:.8}, expected {:.8} (diff={:.2e})",
                                polytope_name,
                                variant.name,
                                r.result.capacity,
                                exp,
                                (r.result.capacity - exp).abs()
                            );
                            n_disagreements += 1;
                        }
                    }

                    entries.push(AblationEntry {
                        polytope_name: polytope_name.clone(),
                        variant: variant.name.to_string(),
                        group: group.clone(),
                        facet_count: f,
                        dual_vertices: duals_raw.clone(),
                        capacity: r.result.capacity,
                        capacity_uncertain: r.result.capacity_uncertain,
                        iterations: r.result.iterations,
                        time_ms,
                    });
                }
            }
        }

        // Check all-variant agreement (all pairs)
        for i in 0..capacities.len() {
            for j in (i + 1)..capacities.len() {
                let (ref name_i, c_i) = capacities[i];
                let (ref name_j, c_j) = capacities[j];
                if (c_i - c_j).abs() > 1e-5 {
                    eprintln!(
                        "  DISAGREE: {} {}={:.8} {}={:.8} (diff={:.2e})",
                        polytope_name,
                        name_i,
                        c_i,
                        name_j,
                        c_j,
                        (c_i - c_j).abs()
                    );
                    n_disagreements += 1;
                }
            }
        }
    }

    // =========================================================================
    // Write JSONL output
    // =========================================================================

    let file = File::create(&output_path).expect("failed to create ablation.jsonl");
    let mut writer = BufWriter::new(file);

    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).expect("failed to serialize entry");
        writeln!(writer).expect("failed to write newline");
    }

    writer.flush().expect("failed to flush output");

    // =========================================================================
    // Summary
    // =========================================================================

    let total_time = t0.elapsed().as_secs_f64();
    println!("Results:");
    println!("  Entries written:  {}", entries.len());
    println!("  Disagreements:    {n_disagreements}");
    println!("  Failures (None):  {n_failures}");
    println!("  Total time:       {total_time:.1}s");
    println!();
    println!("Output: {}", output_path.display());

    if n_disagreements > 0 || n_failures > 0 {
        eprintln!(
            "\nABLATION ISSUES FOUND: {n_disagreements} disagreements, {n_failures} failures"
        );
        std::process::exit(1);
    } else {
        println!("\nAll variants agree. Ready for Python analysis.");
    }
}
