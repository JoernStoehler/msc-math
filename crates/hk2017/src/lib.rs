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
use permutations::cyclic_permutations;
use std::time::{Duration, Instant};

/// Minimum β_i value to consider a solution valid (filters numerical noise near zero).
const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid (avoids division-by-near-zero in action).
const EPS_Q_POSITIVE: f64 = 1e-15;

/// SVD tolerance for solving the KKT system (singular values below this are treated as zero).
const EPS_SVD_TOLERANCE: f64 = 1e-10;

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

/// Progress report emitted during `ehz_capacity_pruned_with_progress`.
#[derive(Clone, Debug)]
pub struct ProgressReport {
    /// Current subset size being searched.
    pub m: usize,
    /// Maximum subset size (= facet count F).
    pub m_max: usize,
    /// KKT solves performed in this m-level so far.
    pub m_evaluated: u64,
    /// Permutations skipped by adjacency pruning in this m-level so far.
    pub m_pruned: u64,
    /// Total (S,σ) pairs for this m = C(F,m) · (m-1)!.
    pub m_theoretical: u64,
    /// Cumulative KKT solves across all completed + current m-levels.
    pub total_evaluated: u64,
    /// Cumulative pruned across all completed + current m-levels.
    pub total_pruned: u64,
    /// Σ_{k=2}^{F} C(F,k)·(k-1)! — total search space without pruning.
    pub grand_total: u64,
    /// Current best action found (None if no valid candidate yet).
    pub best_action: Option<f64>,
    /// Elapsed wall time since search started.
    pub elapsed: Duration,
    /// Whether this report is for a completed m-level (true) or a periodic update (false).
    pub m_completed: bool,
}

/// Compute C(n,k) as u64.
fn binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k); // use smaller k for efficiency
    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}

/// Compute n! as u64.
fn factorial(n: usize) -> u64 {
    (1..=n as u64).product()
}

/// Total (S,σ) pairs for subset size m out of F facets: C(F,m) · (m-1)!.
fn pairs_for_m(f: usize, m: usize) -> u64 {
    binomial(f, m) * factorial(m - 1)
}

/// Total search space: Σ_{m=2}^{F} C(F,m) · (m-1)!.
pub fn total_search_space(f: usize) -> u64 {
    (2..=f).map(|m| pairs_for_m(f, m)).sum()
}

/// Compute c_EHZ(K) with adjacency pruning and progress reporting.
///
/// The callback is invoked:
/// - At the end of each m-level (m_completed = true)
/// - Every ~10 seconds during long m-levels (m_completed = false)
pub fn ehz_capacity_pruned_with_progress(
    polytope: &Polytope4D,
    mut on_progress: impl FnMut(&ProgressReport),
) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();
    let adj = build_adjacency_matrix(polytope);

    let grand_total = total_search_space(f);
    let start = Instant::now();

    let mut best: Option<Candidate> = None;
    let mut total_evaluated: u64 = 0;
    let mut total_pruned: u64 = 0;
    let mut last_report = Instant::now();

    for m in 2..=f {
        let m_theoretical = pairs_for_m(f, m);
        let mut m_evaluated: u64 = 0;
        let mut m_pruned: u64 = 0;

        for subset in combinations(f, m) {
            for perm in cyclic_permutations(&subset) {
                if !is_adjacent_cycle(&perm, &adj) {
                    m_pruned += 1;
                    // Periodic progress during long m-levels (every ~10s)
                    if last_report.elapsed() > Duration::from_secs(10) {
                        last_report = Instant::now();
                        on_progress(&ProgressReport {
                            m,
                            m_max: f,
                            m_evaluated,
                            m_pruned,
                            m_theoretical,
                            total_evaluated: total_evaluated + m_evaluated,
                            total_pruned: total_pruned + m_pruned,
                            grand_total,
                            best_action: best.as_ref().map(|(a, _, _, _)| *a),
                            elapsed: start.elapsed(),
                            m_completed: false,
                        });
                    }
                    continue;
                }

                m_evaluated += 1;

                if let Some((beta, q_val)) = solve_kkt(normals, heights, &perm) {
                    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                        let action = 0.5 / q_val;
                        let update = match &best {
                            None => true,
                            Some((best_a, _, _, _)) => action < *best_a,
                        };
                        if update {
                            best = Some((action, subset.clone(), perm.clone(), beta));
                        }
                    }
                }

                // Periodic progress during long m-levels (every ~10s)
                if last_report.elapsed() > Duration::from_secs(10) {
                    last_report = Instant::now();
                    on_progress(&ProgressReport {
                        m,
                        m_max: f,
                        m_evaluated,
                        m_pruned,
                        m_theoretical,
                        total_evaluated: total_evaluated + m_evaluated,
                        total_pruned: total_pruned + m_pruned,
                        grand_total,
                        best_action: best.as_ref().map(|(a, _, _, _)| *a),
                        elapsed: start.elapsed(),
                        m_completed: false,
                    });
                }
            }
        }

        total_evaluated += m_evaluated;
        total_pruned += m_pruned;

        // Report m-level completion
        on_progress(&ProgressReport {
            m,
            m_max: f,
            m_evaluated,
            m_pruned,
            m_theoretical,
            total_evaluated,
            total_pruned,
            grand_total,
            best_action: best.as_ref().map(|(a, _, _, _)| *a),
            elapsed: start.elapsed(),
            m_completed: true,
        });
    }

    best.map(|(capacity, best_subset, best_permutation, best_beta)| EhzResult {
        capacity,
        best_subset,
        best_permutation,
        best_beta,
        iterations: total_evaluated,
    })
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

/// Solve the KKT system for max Q(β) subject to N^T β = 0, η^T β = 1.
///
/// The KKT conditions are:
///   H β = N λ + ν η      (m equations)
///   N^T β = 0             (4 equations)
///   η^T β = 1             (1 equation)
///
/// This is an (m+5) × (m+5) linear system in (β, λ, ν).
///
/// Returns Some((β, Q(β))) if the system has a unique solution, None otherwise.
///
/// Note: chapter-algorithm.tex `eq:linear-system` omits the ν multiplier,
/// making the system overdetermined. We use the correct KKT system here.
fn solve_kkt(
    normals: &[nalgebra::Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
    let m = perm.len();

    // Build H ∈ R^{m×m}: action matrix
    // H_{ij} = ω₀(n_{σ(i)}, n_{σ(j)}) for i < j
    // H_{ij} = ω₀(n_{σ(j)}, n_{σ(i)}) for i > j  (= H_{ji} by construction)
    // H_{ii} = 0
    let mut h_mat = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            h_mat[(i, j)] = val;
            h_mat[(j, i)] = val;
        }
    }

    // Build KKT system: (m+5) × (m+5) matrix
    // [ H    | -N   | -η ] [ β ]   [ 0 ]
    // [ N^T  |  0   |  0 ] [ λ ] = [ 0 ]
    // [ η^T  |  0   |  0 ] [ ν ]   [ 1 ]
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left: H (m×m)
    for i in 0..m {
        for j in 0..m {
            kkt[(i, j)] = h_mat[(i, j)];
        }
    }

    // Top block columns m..m+4: -N (m×4)
    for i in 0..m {
        for d in 0..4 {
            kkt[(i, m + d)] = -normals[perm[i]][d];
        }
    }

    // Top block column m+4: -η (m×1)
    for i in 0..m {
        kkt[(i, m + 4)] = -heights[perm[i]];
    }

    // Bottom block: N^T (4×m) in rows m..m+4
    for i in 0..m {
        for d in 0..4 {
            kkt[(m + d, i)] = normals[perm[i]][d];
        }
    }

    // Last row: η^T (1×m) in row m+4
    for i in 0..m {
        kkt[(m + 4, i)] = heights[perm[i]];
    }

    // RHS: [0, ..., 0, 0, ..., 0, 1]
    rhs[m + 4] = 1.0;

    // Solve via SVD (handles rank-deficient systems where normals
    // don't span R^4, e.g. hypercube's optimal 4-facet orbit uses
    // normals in a 2D symplectic subplane, giving rank(N^T) = 2).
    let svd = kkt.clone().svd(true, true);
    let solution = svd.solve(&rhs, EPS_SVD_TOLERANCE).ok()?;

    // Verify the solution satisfies the constraints
    let residual = &kkt * &solution - &rhs;
    if residual.norm() > EPS_KKT_RESIDUAL {
        return None;
    }

    // Extract β (first m components)
    let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();

    // Compute Q(β) = Σ_{j<i} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)})
    //
    // Note: we compute directly from ω₀, NOT from H_{ij}.
    // H is symmetric by construction: H_{ij} = ω₀(n_{σ(min)}, n_{σ(max)}).
    // But Q uses ω₀(n_{σ(i)}, n_{σ(j)}) with i > j, which equals -H_{ij}.
    // Using H would give -Q.
    let q_val: f64 = (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum();

    Some((beta, q_val))
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
fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
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
            for perm in cyclic_permutations(&subset) {
                // **ADJACENCY PRUNING**: skip non-adjacent cycles
                if !is_adjacent_cycle(&perm, &adj) {
                    continue;
                }

                iterations += 1;

                // Rest is identical to ehz_capacity
                if let Some((beta, q_val)) = solve_kkt(normals, heights, &perm) {
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

/// Test dataset infrastructure for property tests.
/// Only used in tests, but declared as a public module to allow cross-crate test imports.
pub mod test_dataset;

#[cfg(test)]
mod capacity_properties_test;

#[cfg(test)]
mod lib_test;
