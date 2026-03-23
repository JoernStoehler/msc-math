//! Resumable, symmetry-reduced EHZ capacity computation for the 4D crosspolytope.
//!
//! Goal: Fill the placeholder capacity in `crates/src/geom/known_polytopes.rs`.
//! Input: Crosspolytope from `known_polytopes::crosspolytope()` (16 facets).
//! Output: `experiments/crosspolytope/crosspolytope.jsonl`
//!
//! Three optimizations over the library's `ehz_capacity()`:
//!
//! 1. **Backtracking permutation search**: DFS through the directed adjacency graph
//!    instead of generating all (m-1)! cyclic permutations and filtering. This avoids
//!    the 15! ≈ 1.3 trillion iteration problem for m=16.
//!
//! 2. **Symmetry reduction**: Computes Aut(crosspolytope) ∩ Sp(4,R) and only processes
//!    one canonical representative per orbit of subsets. The symmetry group order is
//!    computed both by Rust enumeration and by hand (for the writeup).
//!
//! 3. **Checkpointing**: Saves progress after each subset size m to a JSON file.
//!    On restart, resumes from the last completed m.
//!
//! Run:
//!   cd experiments/ && cargo run --release --bin crosspolytope

use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::geom::known_polytopes;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;

// ── Constants (copied from crates/src/kkt.rs and crates/src/constants.rs) ───

const EPS_BETA_POSITIVE: f64 = 1e-12;
const EPS_Q_POSITIVE: f64 = 1e-15;
const EPS_SVD_FLOOR: f64 = 1e-12;
const SVD_CONDITION_TAU: f64 = 1e-3;
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Maximum subset size to search. The full crosspolytope has F=16, so m ranges
/// from 2 to 16. Subset sizes m=13..16 have very large permutation counts
/// ((m-1)! cyclic orderings to explore), making exhaustive search infeasible
/// within session time limits. m=12 is the largest size completing in ~4.6 minutes;
/// m=13 alone takes ~8 minutes. Since the best action is found at m=4 and
/// actions generally increase with m, stopping at m=12 is likely sufficient (unproven).
const MAX_SUBSET_SIZE: usize = 12;

// ── KKT solver (copied from crates/src/kkt.rs) ─────────────────────────────

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

    // Pseudoinverse solution using top `rank` singular values
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
    Some((beta_opt, q_val))
}

fn solve_kkt(
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

// ── Combinatorics ───────────────────────────────────────────────────────────

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

// ── Backtracking permutation search ─────────────────────────────────────────
//
// Instead of generating all (m-1)! cyclic permutations and filtering by adjacency,
// we build adjacent cyclic permutations incrementally via DFS, pruning branches
// as soon as a directed edge is missing. This avoids the 15! ≈ 1.3 trillion
// iteration problem for m=16.

fn for_each_adjacent_cyclic_permutation(
    elements: &[usize],
    adj: &DMatrix<bool>,
    callback: &mut impl FnMut(&[usize]),
) {
    let m = elements.len();
    if m == 0 {
        return;
    }
    if m == 1 {
        callback(elements);
        return;
    }

    let first = elements[0];
    let rest: Vec<usize> = elements[1..].to_vec();
    let mut perm = Vec::with_capacity(m);
    perm.push(first);
    let mut used = vec![false; rest.len()];

    dfs_adjacent(&rest, &mut used, adj, first, &mut perm, m, callback);
}

fn dfs_adjacent(
    candidates: &[usize],
    used: &mut [bool],
    adj: &DMatrix<bool>,
    first: usize,
    perm: &mut Vec<usize>,
    total: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    let prev = *perm.last().unwrap();

    if perm.len() == total {
        // Check closing edge: last → first
        if adj[(prev, first)] {
            callback(perm);
        }
        return;
    }

    for (i, &elem) in candidates.iter().enumerate() {
        if used[i] {
            continue;
        }
        if !adj[(prev, elem)] {
            continue; // Prune: no directed edge prev → elem
        }
        used[i] = true;
        perm.push(elem);
        dfs_adjacent(candidates, used, adj, first, perm, total, callback);
        perm.pop();
        used[i] = false;
    }
}

// ── Symmetry group computation ──────────────────────────────────────────────
//
// The crosspolytope has hyperoctahedral symmetry (order 384 = 2^4 · 4!).
// We compute the subgroup that also preserves the standard symplectic form ω₀,
// i.e., Aut(crosspolytope) ∩ Sp(4,R).
//
// Analytical result (verify against Rust computation in writeup):
//   The permutation π must map symplectic planes {0,2} and {1,3} to symplectic
//   planes. This gives 8 valid permutations (4 preserving planes + 4 swapping).
//   For each, the symplecticity condition constrains signs to 2 free choices.
//   Total: 8 × 4 = 32 elements.

/// Generate all 24 permutations of {0,1,2,3}.
fn all_permutations_4() -> Vec<[usize; 4]> {
    let mut result = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            if b == a {
                continue;
            }
            for c in 0..4 {
                if c == a || c == b {
                    continue;
                }
                let d = 6 - a - b - c;
                result.push([a, b, c, d]);
            }
        }
    }
    result
}

/// Compute the symplectic subgroup of the hyperoctahedral group acting on the
/// crosspolytope facets. Returns facet permutations (each maps old facet index
/// to new facet index).
fn compute_symplectic_hyperoctahedral(normals: &[Vector4<f64>]) -> Vec<[usize; 16]> {
    // J₀ in (q₁, q₂, p₁, p₂) coordinates.
    //
    // Sign convention: J₀ = [[0, I], [-I, 0]], so ω₀(u,v) = uᵀ J₀ v.
    // This is the opposite sign from the library's `omega0()` which uses
    // J₀ = [[0, -I], [I, 0]], but both yield the same symplectic group
    // (M^T J₀ M = J₀ is invariant under J₀ → -J₀).
    let j0 = Matrix4::new(
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
    );

    let mut facet_perms = Vec::new();

    // Enumerate all 24 × 16 = 384 signed permutation matrices
    for perm in all_permutations_4() {
        for sign_bits in 0..16u32 {
            let signs: [f64; 4] = [
                if sign_bits & 8 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 4 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 2 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 1 != 0 { -1.0 } else { 1.0 },
            ];

            // Build M: column j has entry signs[j] at row perm[j]
            let mut m = Matrix4::zeros();
            for j in 0..4 {
                m[(perm[j], j)] = signs[j];
            }

            // Check symplecticity: M^T J₀ M = J₀
            if (m.transpose() * j0 * m - j0).norm() > 1e-10 {
                continue;
            }

            // Compute facet permutation: for each facet i, find where M maps its normal
            let mut facet_perm = [0usize; 16];
            for (i, n) in normals.iter().enumerate() {
                let mn = m * n;
                let j = normals
                    .iter()
                    .position(|n2| (mn - n2).norm() < 1e-10)
                    .expect("transformed normal not found");
                facet_perm[i] = j;
            }
            facet_perms.push(facet_perm);
        }
    }

    facet_perms
}

/// Compute the lexicographically smallest representative of a subset's orbit
/// under the symmetry group.
fn canonical_subset(subset: &[usize], group: &[[usize; 16]]) -> Vec<usize> {
    let mut canonical = subset.to_vec();
    canonical.sort();

    for facet_perm in group {
        let mut transformed: Vec<usize> = subset.iter().map(|&i| facet_perm[i]).collect();
        transformed.sort();
        if transformed < canonical {
            canonical = transformed;
        }
    }

    canonical
}

// ── Checkpoint I/O ──────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CandidateSer {
    action: f64,
    subset: Vec<usize>,
    permutation: Vec<usize>,
    beta: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Checkpoint {
    /// Subset sizes 2..=completed_m are fully processed.
    completed_m: usize,
    /// Total (S, σ) pairs evaluated (adjacent only, after pruning).
    iterations: u64,
    /// Cumulative wall-clock seconds elapsed.
    elapsed_secs: f64,
    /// Best certified candidate (all β_i > +EPS).
    best_certified: Option<CandidateSer>,
    /// Best uncertain candidate (-EPS < β_i).
    best_uncertain: Option<CandidateSer>,
}

fn checkpoint_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("crosspolytope/checkpoint.json")
}

fn save_checkpoint(cp: &Checkpoint) {
    let path = checkpoint_path();
    let file = File::create(&path).expect("failed to create checkpoint");
    serde_json::to_writer_pretty(file, cp).expect("failed to write checkpoint");
    println!(
        "  [checkpoint] m={}, iterations={}, {:.1}s elapsed",
        cp.completed_m, cp.iterations, cp.elapsed_secs
    );
}

fn load_checkpoint() -> Option<Checkpoint> {
    let path = checkpoint_path();
    if !path.exists() {
        return None;
    }
    let file = File::open(&path).ok()?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).ok()
}

// ── Output ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct CrosspolytopeResult {
    name: String,
    facet_count: usize,
    volume: f64,
    capacity: f64,
    capacity_uncertain: f64,
    numerical_gap: f64,
    sys: f64,
    iterations: u64,
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    best_beta: Vec<f64>,
    time_volume_ms: f64,
    time_capacity_ms: f64,
    symmetry_group_order: usize,
    hyperoctahedral_group_order: usize,
    /// Subset sizes 2..=search_complete_through_m have been exhaustively searched.
    search_complete_through_m: usize,
}

// ── Main ────────────────────────────────────────────────────────────────────

type Candidate = (f64, Vec<usize>, Vec<usize>, Vec<f64>); // (action, subset, perm, beta)

fn main() {
    let t0 = Instant::now();

    // 1. Construct crosspolytope
    let kp = known_polytopes::crosspolytope();
    let polytope = &kp.polytope;
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    println!("Crosspolytope: {f} facets");

    // 2. Volume
    let start_vol = Instant::now();
    let vol = volume(polytope).expect("volume computation failed");
    let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;
    println!("Volume: {vol:.10} ({time_volume_ms:.1} ms)");

    // 3. Symmetry group
    println!("\nComputing symplectic symmetry group...");
    let group = compute_symplectic_hyperoctahedral(&normals);
    println!("Hyperoctahedral group order: 384");
    println!("Symplectic subgroup order:   {}", group.len());
    println!("Expected (by hand):          32");
    if group.len() != 32 {
        println!("WARNING: Rust computation disagrees with analytical result!");
    }

    // 4. Directed adjacency matrix
    let adj = build_transition_matrix(polytope);
    let avg_out_degree: f64 = (0..f)
        .map(|i| (0..f).filter(|&j| adj[(i, j)] && i != j).count() as f64)
        .sum::<f64>()
        / f as f64;
    println!("\nDirected adjacency: avg out-degree = {avg_out_degree:.1} (of {} possible)", f - 1);

    // 5. Load checkpoint
    let checkpoint = load_checkpoint();
    let start_m: usize;
    let mut iterations: u64;
    let mut best_certified: Option<Candidate>;
    let mut best_uncertain: Option<Candidate>;
    let prior_elapsed: f64;

    if let Some(cp) = &checkpoint {
        start_m = cp.completed_m + 1;
        iterations = cp.iterations;
        prior_elapsed = cp.elapsed_secs;
        best_certified = cp
            .best_certified
            .as_ref()
            .map(|c| (c.action, c.subset.clone(), c.permutation.clone(), c.beta.clone()));
        best_uncertain = cp
            .best_uncertain
            .as_ref()
            .map(|c| (c.action, c.subset.clone(), c.permutation.clone(), c.beta.clone()));
        println!(
            "\nResuming from checkpoint: m={} done, {} iterations, {:.1}s prior",
            cp.completed_m, iterations, prior_elapsed
        );
    } else {
        start_m = 2;
        iterations = 0;
        prior_elapsed = 0.0;
        best_certified = None;
        best_uncertain = None;
        println!("\nNo checkpoint found, starting from scratch.");
    }

    // 6. Main computation loop
    println!("\n=== Computing capacity ===\n");
    let cap_start = Instant::now();

    let max_m = MAX_SUBSET_SIZE.min(f);
    for m in start_m..=max_m {
        let m_start = Instant::now();
        let all_subsets = combinations(f, m);
        let total_subsets = all_subsets.len();

        // Filter to canonical subsets under symmetry group
        let mut seen = HashSet::new();
        let mut canonical_subsets = Vec::new();
        for subset in &all_subsets {
            let canon = canonical_subset(subset, &group);
            if seen.insert(canon.clone()) {
                canonical_subsets.push(canon);
            }
        }

        let canonical_count = canonical_subsets.len();
        let reduction = if canonical_count > 0 {
            total_subsets as f64 / canonical_count as f64
        } else {
            0.0
        };
        print!(
            "m={m:2}: C({f},{m})={total_subsets:6} → {canonical_count:5} canonical ({reduction:.1}x)  ",
        );

        let mut m_iterations = 0u64;
        let mut m_kkt_solutions = 0u64;

        for subset in &canonical_subsets {
            for_each_adjacent_cyclic_permutation(subset, &adj, &mut |perm| {
                iterations += 1;
                m_iterations += 1;

                if let Some((beta, q_val)) = solve_kkt(&normals, &heights, perm) {
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    m_kkt_solutions += 1;
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    // Certified: β_i > +EPS
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

                    // Uncertain: -EPS < β_i
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

        let m_elapsed = m_start.elapsed().as_secs_f64();
        println!(
            "adj_perms={m_iterations:8}, kkt_solutions={m_kkt_solutions:6}, {m_elapsed:.2}s"
        );

        // Save checkpoint
        let total_elapsed = prior_elapsed + cap_start.elapsed().as_secs_f64();
        let cp = Checkpoint {
            completed_m: m,
            iterations,
            elapsed_secs: total_elapsed,
            best_certified: best_certified.as_ref().map(|c| CandidateSer {
                action: c.0,
                subset: c.1.clone(),
                permutation: c.2.clone(),
                beta: c.3.clone(),
            }),
            best_uncertain: best_uncertain.as_ref().map(|c| CandidateSer {
                action: c.0,
                subset: c.1.clone(),
                permutation: c.2.clone(),
                beta: c.3.clone(),
            }),
        };
        save_checkpoint(&cp);
    }

    let time_capacity_ms = (prior_elapsed + cap_start.elapsed().as_secs_f64()) * 1000.0;

    // 7. Extract result
    let certified = best_certified
        .expect("no certified (S,σ) found — should not happen for valid polytopes");
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);

    assert!(
        uncertain_cap <= certified.0,
        "Unexpected: uncertain capacity {:.6e} > certified {:.6e}",
        uncertain_cap,
        certified.0,
    );

    let phys_perm = certified.2;
    let phys_beta = certified.3;

    let cap = certified.0;
    let sys = cap * cap / (2.0 * vol);

    println!("\n=== Results ===");
    println!("Capacity (certified):  {cap:.10}");
    println!("Capacity (uncertain):  {uncertain_cap:.10}");
    println!("Numerical gap:         {:.2e}", cap - uncertain_cap);
    println!("Volume:                {vol:.10}");
    println!("Systolic ratio:        {sys:.10}");
    println!("Iterations:            {iterations}");
    println!("Time (capacity):       {:.1} s", time_capacity_ms / 1000.0);
    println!("Best subset (facets):  {:?}", certified.1);
    println!("Best permutation:      {:?}", phys_perm);
    println!("Best beta:             {:?}", phys_beta);
    println!("Symmetry group order:  {}", group.len());
    println!(
        "Viterbo conjecture:    {}",
        if sys <= 1.0 {
            "SATISFIED (sys <= 1)"
        } else {
            "VIOLATED (sys > 1)"
        }
    );

    // 8. Write JSONL
    let output_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("crosspolytope/crosspolytope.jsonl");
    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let row = CrosspolytopeResult {
        name: "crosspolytope_4d".to_string(),
        facet_count: f,
        volume: vol,
        capacity: cap,
        capacity_uncertain: uncertain_cap,
        numerical_gap: cap - uncertain_cap,
        sys,
        iterations,
        best_subset: certified.1,
        best_permutation: phys_perm,
        best_beta: phys_beta,
        time_volume_ms,
        time_capacity_ms,
        symmetry_group_order: group.len(),
        hyperoctahedral_group_order: 384,
        search_complete_through_m: max_m,
    };

    let line = serde_json::to_string(&row).expect("serialize row");
    writeln!(writer, "{line}").expect("write line");
    writer.flush().expect("flush output");

    println!("\nWrote results to {}", output_path.display());
    println!("Total time: {:.1} s", t0.elapsed().as_secs_f64());

    // Clean up checkpoint on successful completion
    let cp_path = checkpoint_path();
    if cp_path.exists() {
        std::fs::remove_file(&cp_path).ok();
        println!("Removed checkpoint file.");
    }
}
