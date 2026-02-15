/// EHZ capacity computation via the Haim-Kislev 2017 algorithm.
///
/// Computes c_EHZ(K) for a convex polytope K ⊂ R^4 by exhaustive search
/// over all subsets S ⊆ {1,...,F} and cyclic permutations σ of S.
///
/// # Algorithm (chapter-algorithm.tex, `alg:ehz`)
///
/// For each (S, σ):
/// 1. Build normal matrix N, action matrix H, height vector η
/// 2. Solve KKT system for the constrained maximum of Q(β) on M(K)
/// 3. Filter: discard if any β_i ≤ 0
/// 4. Evaluate: A(S,σ) = (1/2) Q(β)^{-1}
///
/// Return c_EHZ(K) = min A(S,σ).
///
/// # Complexity
///
/// Σ_{m=2}^{F} C(F,m) · (m-1)! — exponential in F.
mod permutations;

use geom::polytope::Polytope4D;
use geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector};
use permutations::{cyclic_permutations, for_each_cyclic_permutation};
/// Minimum β_i value to consider a solution valid (filters numerical noise near zero).
const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid (avoids division-by-near-zero in action).
const EPS_Q_POSITIVE: f64 = 1e-15;

/// Floor for SVD: singular values below max_sv * EPS_SVD_FLOOR are treated as exactly zero.
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Gap ratio threshold for rank detection: if sv[i-1]/sv[i] > this, sv[i..] are
/// treated as part of the null space. Catches near-degenerate KKT systems where a
/// singular value ~1e-4 is separated from genuine nonzero values ~0.5 by a gap of
/// ~600x. Without gap detection, the pseudoinverse amplifies this sv by ~1/sv ≈ 1000,
/// producing garbage β₀ and breaking cyclic invariance.
const SVD_GAP_THRESHOLD: f64 = 100.0;

/// Maximum acceptable residual norm for the KKT solution (rejects numerically poor solutions).
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Tolerance for vertex-facet incidence in adjacency matrix (matches qhull precision).
const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// Intermediate best candidate: (action, subset, permutation, beta).
type Candidate = (
    f64,        // action = 0.5 / Q(β)
    Vec<usize>, // subset S ⊆ {0,...,F-1}
    Vec<usize>, // cyclic permutation σ of S
    Vec<f64>,   // optimal β vector
);

/// Result of the EHZ capacity computation.
#[derive(Clone, Debug)]
pub struct EhzResult {
    /// The EHZ capacity c_EHZ(K).
    pub capacity: f64,
    /// The subset S (facet indices) achieving the minimum action.
    pub best_subset: Vec<usize>,
    /// The cyclic permutation σ of S achieving the minimum action.
    pub best_permutation: Vec<usize>,
    /// The β vector at the optimum.
    pub best_beta: Vec<f64>,
    /// Total number of (S, σ) pairs evaluated.
    pub iterations: u64,
}

/// Compute c_EHZ(K) for a convex polytope K ⊂ R^4.
///
/// Returns `None` if no valid (S, σ) pair yields β > 0 (should not happen
/// for valid polytopes, but guards against degenerate input).
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    let mut best: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    // Enumerate all subsets S ⊆ {0,...,F-1} with |S| ≥ 2.
    for m in 2..=f {
        for subset in combinations(f, m) {
            // For each cyclic permutation of S
            for perm in cyclic_permutations(&subset) {
                iterations += 1;

                if let Some((beta, q_val)) = solve_kkt(normals, heights, &perm) {
                    // All β_i > 0 filter
                    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                        let action = 0.5 / q_val;
                        let update = match &best {
                            None => true,
                            Some((best_a, _, _, _)) => action < *best_a,
                        };
                        if update {
                            best = Some((
                                action,
                                subset.clone(),
                                perm.clone(),
                                beta,
                            ));
                        }
                    }
                }
            }
        }
    }

    best.map(|(capacity, best_subset, best_permutation, best_beta)| EhzResult {
        capacity,
        best_subset,
        best_permutation,
        best_beta,
        iterations,
    })
}

/// Compute Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
///
/// Note: uses ω₀(n_{σ(i)}, n_{σ(j)}) with i > j directly, NOT from H_{ij}.
/// H is symmetric by construction, but Q needs the antisymmetric ω₀ values.
fn q_from_beta(
    normals: &[nalgebra::Vector4<f64>],
    perm: &[usize],
    beta: &[f64],
) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum()
}

/// Search null space for a β > 0 solution (1D null space case).
///
/// Given particular solution β₀ and null space vector v, find α such that
/// β₀ + α·v > 0 componentwise. Returns the midpoint of the feasible interval
/// for numerical stability.
///
/// Q(β) is constant along the null space (the null space directions satisfy
/// the KKT stationarity conditions, so the objective doesn't change).
fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);

    for j in 0..m {
        if v[j].abs() < 1e-15 {
            // This component doesn't change with α
            if beta0[j] <= EPS_BETA_POSITIVE {
                return None; // Can't make this component positive
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
        return None; // No feasible α
    }

    // Pick midpoint for maximum margin
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

/// Search null space for a β > 0 solution (multi-dimensional null space).
///
/// Uses iterative coordinate ascent on the most-violated constraint.
fn find_positive_beta_nd(beta0: &[f64], null_vecs: &[Vec<f64>]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![0.0; k];

    for _iter in 0..100 {
        // Current β
        let beta: Vec<f64> = (0..m)
            .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
            .collect();

        // Find most-violated component
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        if *worst_val > EPS_BETA_POSITIVE {
            return Some(beta);
        }

        // Gradient of β[worst_j] w.r.t. α: g_i = null_vecs[i][worst_j]
        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            return None; // Can't improve this component
        }

        // Step to push β[worst_j] to a small positive value
        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }

    // Final feasibility check
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

/// Solve the KKT system for max Q(β) subject to N^T β = 0, η^T β = 1.
///
/// The KKT conditions are:
///   H β = N λ + ν η      (m equations)
///   N^T β = 0             (4 equations)
///   η^T β = 1             (1 equation)
///
/// This is an (m+5) × (m+5) linear system in (β, λ, ν).
///
/// Returns Some((β, Q(β))) if a solution with β > 0 exists, None otherwise.
///
/// When the KKT system is rank-deficient (common for polytopes with
/// axis-aligned normals in symplectic subplanes), there is a family of
/// solutions parameterized by the null space. Q(β) is constant along
/// the null space, so we search for any member with β > 0.
///
/// Note: chapter-algorithm.tex `eq:linear-system` omits the ν multiplier,
/// making the system overdetermined. We use the correct KKT system here.
pub(crate) fn solve_kkt(
    normals: &[nalgebra::Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
    let m = perm.len();

    // Build KKT system directly (no separate H matrix):
    // [ H    | -N   | -η ] [ β ]   [ 0 ]
    // [ N^T  |  0   |  0 ] [ λ ] = [ 0 ]
    // [ η^T  |  0   |  0 ] [ ν ]   [ 1 ]
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left: H (m×m) — action matrix with ω₀ values
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }

    // Top block columns m..m+4: -N (m×4) and bottom block: N^T (4×m)
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }

    // Top block column m+4: -η and last row: η^T
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
        kkt[(m + 4, i)] = h;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = 1.0;

    // --- SVD with gap-based rank detection ---
    //
    // We use SVD (not LU) because SVD reveals the rank structure. For near-
    // degenerate KKT systems (e.g. Lagrangian products near special angles),
    // a singular value ~1e-4 can be separated from genuine values ~0.5 by a
    // gap of ~600x. A fixed tolerance either misses this (too tight → garbage
    // pseudoinverse, broken cyclic invariance) or over-truncates for well-
    // conditioned systems (too loose → conformality violations).
    //
    // Gap-based detection: walk from the smallest sv upward, treating sv[i]
    // as zero if sv[i-1]/sv[i] > SVD_GAP_THRESHOLD. This adapts to each
    // system's spectrum.
    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
    if max_sv < EPS_SVD_FLOOR {
        return None;
    }

    let u = svd.u.as_ref()?;
    let v_t = svd.v_t.as_ref()?;

    // Determine numerical rank via gap detection.
    // sv is sorted descending. First count values above the noise floor,
    // then walk upward looking for a large gap.
    let floor = max_sv * EPS_SVD_FLOOR;
    let nonzero = sv.iter().filter(|&&s| s > floor).count();
    let mut rank = nonzero;
    for i in (1..nonzero).rev() {
        if sv[i - 1] / sv[i] > SVD_GAP_THRESHOLD {
            rank = i;
            break;
        }
    }

    // Compute pseudoinverse solution manually using only the top `rank` SVs.
    // This avoids relying on nalgebra's solve() tolerance interpretation.
    let mut x0 = DVector::zeros(size);
    for i in 0..rank {
        let coeff = u.column(i).dot(&rhs) / sv[i];
        for j in 0..size {
            x0[j] += coeff * v_t[(i, j)];
        }
    }

    let residual = (&kkt * &x0 - &rhs).norm();
    if residual > EPS_KKT_RESIDUAL {
        return None;
    }

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();

    // If already feasible, return
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        let q_val = q_from_beta(normals, perm, &beta0);
        return Some((beta0, q_val));
    }

    // Full rank: unique solution with β ≤ 0, orbit is genuinely infeasible
    if rank == size {
        return None;
    }

    // Rank-deficient: search null space for β > 0.
    // Null space = right singular vectors for sv's below rank threshold.
    let null_beta: Vec<Vec<f64>> = (rank..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();

    let beta_opt = if null_beta.len() == 1 {
        find_positive_beta_1d(&beta0, &null_beta[0])?
    } else {
        find_positive_beta_nd(&beta0, &null_beta)?
    };

    // Constraint verification: reject if β from null space search violates
    // N^T β ≈ 0 or η^T β ≈ 1 (catches noise amplification in degenerate cases).
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

/// Generate all combinations of `k` elements from `{0, ..., n-1}` in lexicographic order.
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

/// Build facet adjacency matrix: adj[i][j] = true iff F_i ∩ F_j ≠ ∅.
/// Two facets are adjacent if they share at least one vertex.
pub(crate) fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();
    let mut adj = vec![vec![false; f]; f];

    // Facet i is adjacent to facet j if some vertex lies on both
    for v in polytope.vertices() {
        let incident: Vec<usize> = (0..f)
            .filter(|&i| (normals[i].dot(v) - heights[i]).abs() < EPS_FACET_INCIDENCE)
            .collect();
        // All pairs in incident are adjacent
        for &i in &incident {
            for &j in &incident {
                adj[i][j] = true;
            }
        }
    }

    adj
}

/// Check if a cyclic permutation forms an adjacent cycle.
fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

/// Compute c_EHZ(K) with adjacency pruning (chapter-algorithm.tex, `cor:adjacency-pruning`).
///
/// Skips (S, σ) pairs where consecutive facets are not adjacent,
/// significantly reducing search space for polytopes with sparse adjacency.
pub fn ehz_capacity_pruned(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    // Precompute adjacency matrix once
    let adj = build_adjacency_matrix(polytope);

    let mut best: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                // **ADJACENCY PRUNING**: skip non-adjacent cycles
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }

                iterations += 1;

                if let Some((beta, q_val)) = solve_kkt(normals, heights, perm) {
                    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                        let action = 0.5 / q_val;
                        let update = match &best {
                            None => true,
                            Some((best_a, _, _, _)) => action < *best_a,
                        };
                        if update {
                            best = Some((
                                action,
                                subset.clone(),
                                perm.to_vec(),
                                beta,
                            ));
                        }
                    }
                }
            });
        }
    }

    best.map(|(capacity, best_subset, best_permutation, best_beta)| EhzResult {
        capacity,
        best_subset,
        best_permutation,
        best_beta,
        iterations,
    })
}

/// Test dataset infrastructure for property tests.
/// Only used in tests, but declared as a public module to allow cross-crate test imports.
pub mod test_dataset;

#[cfg(test)]
mod capacity_properties_test;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
#[path = "square_product_diagnostic.rs"]
mod square_product_diagnostic;
