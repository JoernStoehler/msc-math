//! Instrumented KKT solver and orbit enumeration infrastructure.
//!
//! Copied from sys_optimization.rs with minor cleanup. Provides `solve_kkt_full`
//! which returns (β, Q, ν, λ) — the full KKT data needed for analytical gradients
//! via the envelope theorem.
//!
//! Uses the ASYMMETRIC sign convention (upper-right = −n/−h, lower-left = +n/+h)
//! so that solution components give multipliers matching Hβ = Nλ + ην directly.
//!
//! Source: experiments/sys-optimization/sys_optimization.rs lines 164–663

use nalgebra::{DMatrix, DVector, Vector4};
// TODO: Polytope4D will be re-exported from top-level in wave 4 (subagent #16)
use symplectic::geom::polytope::Polytope4D;

// ============================================================================
// Constants (copied from crates/src/kkt.rs and crates/src/constants.rs)
// ============================================================================

/// Minimum β_i value to consider a solution valid.
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid.
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// Floor for SVD singular values.
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Condition-number threshold for SVD rank detection.
const SVD_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm.
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Facet incidence tolerance.
pub const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// Threshold for detecting degenerate (collinear) polygon vertices.
pub const EPS_DEGENERATE: f64 = 1e-10;

// ============================================================================
// Symplectic primitives
// ============================================================================

/// ω₀(u, v) = u_q1·v_p1 − u_p1·v_q1 + u_q2·v_p2 − u_p2·v_q2
/// Source: crates/src/geom/symplectic.rs:28
pub fn omega0_local(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

/// Apply J₀ to a vector: J₀(a,b,c,d) = (−c,−d,a,b).
/// J₀ = [[0, −I₂], [I₂, 0]] in (q₁, q₂, p₁, p₂) coordinates.
pub fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

// ============================================================================
// KKT solver
// ============================================================================

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) = (1/2) β^T H β.
/// Q > 0 for permutations in positive Reeb direction.
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0_local(&normals[perm[j]], &normals[perm[i]]))
        .sum()
}

/// Search 1D null space for β > 0 solution.
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

/// Build KKT matrix and RHS vector (ASYMMETRIC sign convention).
///
/// Layout: [H | −N | −η; N^T | 0 | 0; η^T | 0 | 0] x = [0; 0; 1]
/// Solution components: x = [β, λ, ν] where Hβ = Nλ + ην.
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
/// Returns (β, Q, ν, λ).
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
    // ν and λ from pseudoinverse solution (null-space search adjusts only β)
    Some((beta_opt, q_val, nu, lambda))
}

/// Full KKT solver returning (β, Q, ν, λ).
pub fn solve_kkt_full(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64, f64, Vec<f64>)> {
    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    solve_kkt_svd_path(&kkt, &rhs, normals, heights, perm)
}

// ============================================================================
// Orbit types
// ============================================================================

/// A valid orbit with full KKT data.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ValidOrbit {
    pub action: f64,
    pub subset: Vec<usize>,
    pub permutation: Vec<usize>, // positive Reeb direction
    pub beta: Vec<f64>,
    pub q_value: f64,
    /// Lagrange multiplier for the η^T β = 1 constraint.
    pub nu: f64,
    /// Lagrange multiplier vector (4 components) for the N^T β = 0 constraint.
    pub lambda: Vec<f64>,
}

/// Result of an instrumented capacity computation.
#[allow(dead_code)]
pub struct InstrumentedResult {
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub orbits: Vec<ValidOrbit>, // sorted by action ascending
    pub iterations: u64,
}

// ============================================================================
// Adjacency infrastructure
// ============================================================================

/// Build undirected facet adjacency matrix (vertex-sharing).
pub fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
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
pub fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
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
pub fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// Combinatorial enumeration (HK2017 style)
// ============================================================================

/// Generate all C(n,k) combinations in lexicographic order.
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
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
pub fn for_each_cyclic_permutation(elements: &[usize], callback: &mut impl FnMut(&[usize])) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf(&mut buf, 1, k, callback);
}

/// Heap's algorithm on buf[offset..offset+k].
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

// ============================================================================
// Block enumeration (billiard style)
// Copied from crates/src/algorithms/billiard/enumerate.rs
// ============================================================================

/// A block in a k-bounce orbit: either a single facet or an ordered adjacent pair.
#[derive(Debug, Clone, Copy)]
pub enum Block {
    Single(usize),
    Pair(usize, usize),
}

impl Block {
    #[inline]
    fn push_to(&self, buf: &mut Vec<usize>) {
        match *self {
            Block::Single(i) => buf.push(i),
            Block::Pair(i, j) => {
                buf.push(i);
                buf.push(j);
            }
        }
    }

    #[inline]
    fn contains(&self, idx: usize) -> bool {
        match *self {
            Block::Single(i) => i == idx,
            Block::Pair(i, j) => i == idx || j == idx,
        }
    }

    #[inline]
    fn overlaps(&self, other: &Block) -> bool {
        match *other {
            Block::Single(i) => self.contains(i),
            Block::Pair(i, j) => self.contains(i) || self.contains(j),
        }
    }
}

/// Enumerate all valid blocks for a set of facets.
pub fn enumerate_blocks(facet_indices: &[usize], adj: &[Vec<bool>]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for &i in facet_indices {
        blocks.push(Block::Single(i));
    }
    for (a, &i) in facet_indices.iter().enumerate() {
        for &j in &facet_indices[a + 1..] {
            if adj[i][j] {
                blocks.push(Block::Pair(i, j));
                blocks.push(Block::Pair(j, i));
            }
        }
    }
    blocks
}

/// Enumerate all σ sequences matching ([Q|QQ][P|PP])^k, lazily.
pub fn enumerate_k_bounce_sigmas(
    k: usize,
    q_blocks: &[Block],
    p_blocks: &[Block],
    mut callback: impl FnMut(&[usize]),
) {
    if k == 0 {
        return;
    }

    let mut q_sel = Vec::with_capacity(k);
    let mut p_sel = Vec::with_capacity(k);
    let mut sigma = Vec::with_capacity(4 * k);
    let mut q_perm_buf = vec![0usize; k.saturating_sub(1)];
    let mut p_perm_buf = vec![0usize; k];

    for_each_non_overlapping(q_blocks, k, &mut q_sel, &mut |q_selection| {
        for_each_non_overlapping(p_blocks, k, &mut p_sel, &mut |p_selection| {
            if k == 1 {
                sigma.clear();
                q_selection[0].push_to(&mut sigma);
                p_selection[0].push_to(&mut sigma);
                callback(&sigma);
            } else {
                for_each_permutation(k - 1, &mut q_perm_buf, &mut |q_rest_perm| {
                    for_each_permutation(k, &mut p_perm_buf, &mut |p_perm| {
                        sigma.clear();
                        q_selection[0].push_to(&mut sigma);
                        p_selection[p_perm[0]].push_to(&mut sigma);
                        for round in 1..k {
                            q_selection[1 + q_rest_perm[round - 1]].push_to(&mut sigma);
                            p_selection[p_perm[round]].push_to(&mut sigma);
                        }
                        callback(&sigma);
                    });
                });
            }
        });
    });
}

fn for_each_non_overlapping(
    blocks: &[Block],
    k: usize,
    selection: &mut Vec<Block>,
    callback: &mut impl FnMut(&[Block]),
) {
    selection.clear();
    non_overlapping_rec(blocks, k, 0, selection, callback);
}

fn non_overlapping_rec(
    blocks: &[Block],
    k: usize,
    start: usize,
    selection: &mut Vec<Block>,
    callback: &mut impl FnMut(&[Block]),
) {
    if selection.len() == k {
        callback(selection);
        return;
    }
    let remaining = k - selection.len();
    if start + remaining > blocks.len() {
        return;
    }
    for i in start..blocks.len() {
        if selection.iter().any(|s| s.overlaps(&blocks[i])) {
            continue;
        }
        selection.push(blocks[i]);
        non_overlapping_rec(blocks, k, i + 1, selection, callback);
        selection.pop();
    }
}

fn for_each_permutation(n: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
    for (i, slot) in buf.iter_mut().enumerate().take(n) {
        *slot = i;
    }
    heap_permute(n, buf, callback);
}

fn heap_permute(k: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_permute(k - 1, buf, callback);
    for i in 0..k - 1 {
        if k.is_multiple_of(2) {
            buf.swap(i, k - 1);
        } else {
            buf.swap(0, k - 1);
        }
        heap_permute(k - 1, buf, callback);
    }
}

// ============================================================================
// Facet classification (billiard style)
// Copied from crates/src/algorithms/billiard/lagrangian.rs
// ============================================================================

const EPS_LAGRANGIAN_NORMAL: f64 = 1e-10;

/// Classification of facets into q-type and p-type.
#[derive(Debug, Clone)]
pub struct FacetClassification {
    pub q_indices: Vec<usize>,
    pub p_indices: Vec<usize>,
}

/// Classify facets into q-type (normal in q-plane) and p-type (normal in p-plane).
/// Returns None if any facet is mixed (not a Lagrangian product).
pub fn classify_facets(polytope: &Polytope4D) -> Option<FacetClassification> {
    let normals = polytope.normals_f64();
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();

    for (i, n) in normals.iter().enumerate() {
        let q_norm_sq = n[0] * n[0] + n[1] * n[1];
        let p_norm_sq = n[2] * n[2] + n[3] * n[3];

        if p_norm_sq < EPS_LAGRANGIAN_NORMAL {
            q_indices.push(i);
        } else if q_norm_sq < EPS_LAGRANGIAN_NORMAL {
            p_indices.push(i);
        } else {
            return None;
        }
    }

    if q_indices.len() < 3 || p_indices.len() < 3 {
        return None;
    }

    Some(FacetClassification {
        q_indices,
        p_indices,
    })
}

// ============================================================================
// Instrumented capacity algorithms
// ============================================================================

/// Instrumented HK2017: enumerates all (S, σ) with adjacency pruning,
/// returns all certified orbits with full KKT data.
pub fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedResult> {
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

/// Instrumented billiard: same block-structured enumeration as the library
/// billiard algorithm, but returns full KKT data (ν, λ) for gradient computation.
pub fn billiard_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedResult> {
    let classification = classify_facets(polytope)?;
    let adj = build_adjacency_matrix(polytope); // undirected: for block building (same-type pairs)
    let directed_adj = build_directed_adjacency_matrix(polytope); // directed: for cycle pruning (ω₀ condition)
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    let mut orbits: Vec<ValidOrbit> = Vec::new();
    let mut best_uncertain_action: Option<f64> = None;
    let mut iterations: u64 = 0;

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            // Directed adjacency pruning: skip cycles where consecutive facets
            // violate the ω₀ transition feasibility condition.
            if !is_adjacent_cycle(sigma, &directed_adj) {
                return;
            }
            iterations += 1;

            if let Some((beta, q_val, nu, lambda)) = solve_kkt_full(&normals, &heights, sigma) {
                if q_val <= EPS_Q_POSITIVE {
                    return;
                }
                let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                let action = 0.5 / q_val;

                if beta_min > EPS_BETA_POSITIVE {
                    orbits.push(ValidOrbit {
                        action,
                        subset: sigma.to_vec(),
                        permutation: sigma.to_vec(),
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
// Facet volume helpers (for volume derivatives)
// Copied from sys_optimization.rs which copied from crates/src/geom/volume.rs
// ============================================================================

/// 4D cross product: vector perpendicular to three vectors in R⁴.
pub fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
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
pub fn sort_polygon_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
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
pub fn facet_volume_3d(
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
pub fn facet_volume_and_centroid_3d(
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
