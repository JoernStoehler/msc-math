/// EHZ capacity computation via the Haim-Kislev 2017 algorithm.
///
/// Computes c_EHZ(K) for a convex polytope K ⊂ R^4 by exhaustive search
/// over all subsets S ⊆ {1,...,F} and cyclic permutations σ of S.
///
/// # Algorithm (general-case-algorithm.tex, `[alg:ehz]`)
///
/// For each (S, σ):
/// 1. Build normal matrix N, action matrix H, height vector η
/// 2. Solve KKT system for the constrained maximum of Q(β) on M(K)
/// 3. Filter: discard if any β_i ≤ 0
/// 4. Evaluate: A(S,σ) = (1/2) Q(β)^{-1}
///
/// Return c_EHZ(K) = min A(S,σ).
///
/// # Permutation ordering convention
///
/// The `best_permutation` in [`EhzResult`] is in **physical orbit direction**:
/// σ = [a, b, c, ...] means the Reeb orbit visits F_a → F_b → F_c → ... → F_a.
/// For consecutive facets, ω₀(n_{σ(k)}, n_{σ(k+1)}) ≥ 0.
///
/// Internally, the Q-function Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}) uses
/// the reversed ("algebraic") ordering. The reversal happens at the output boundary:
/// the best permutation is reversed before being stored in [`EhzResult`].
///
/// # Complexity
///
/// Σ_{m=2}^{F} C(F,m) · (m-1)! — exponential in F.
mod permutations;

use crate::constants::EPS_FACET_INCIDENCE;
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic::omega0;
use crate::kkt::{solve_kkt, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use permutations::{cyclic_permutations, for_each_cyclic_permutation};

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
    /// The EHZ capacity c_EHZ(K) from certified check (β_i > +EPS).
    pub capacity: f64,
    /// Capacity from uncertain check (-EPS < β_i). Always ≤ capacity.
    ///
    /// Includes orbits where some β_i is near zero (within ±EPS of zero),
    /// which might be valid solutions with floating-point sign error.
    /// If `capacity_uncertain < capacity`, there exist borderline orbits
    /// with lower action than any certified orbit.
    pub capacity_uncertain: f64,
    /// The subset S (facet indices) achieving the minimum action.
    pub best_subset: Vec<usize>,
    /// The cyclic permutation σ of S in **physical orbit direction**.
    /// σ[k] → σ[k+1] is the direction of the Reeb orbit.
    pub best_permutation: Vec<usize>,
    /// The β vector at the optimum.
    pub best_beta: Vec<f64>,
    /// Total number of (S, σ) pairs evaluated.
    pub iterations: u64,
}

impl EhzResult {
    /// Gap between certified and uncertain capacity. Zero means high confidence.
    /// Positive means there exist borderline orbits with lower action.
    pub fn numerical_gap(&self) -> f64 {
        self.capacity - self.capacity_uncertain
    }
}

/// Compute c_EHZ(K) for a convex polytope K ⊂ R^4.
///
/// Reference (unpruned) implementation of `[alg:ehz]` (thesis): exhaustive
/// search over all (S, σ) pairs. For production use, prefer
/// `ehz_capacity` which applies `[cor:adjacency-pruning]` and is
/// used in all experiments.
///
/// Returns `None` if no valid (S, σ) pair yields β > 0 (should not happen
/// for valid polytopes, but guards against degenerate input).
pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    // Enumerate all subsets S ⊆ {0,...,F-1} with |S| ≥ 2.
    for m in 2..=f {
        for subset in combinations(f, m) {
            // For each cyclic permutation of S
            for perm in cyclic_permutations(&subset) {
                iterations += 1;

                if let Some((beta, q_val)) = solve_kkt(normals, heights, &perm) {
                    if q_val <= EPS_Q_POSITIVE {
                        continue;
                    }
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    // Certified: β_i > +EPS (all predicates TRUE)
                    if beta_min > EPS_BETA_POSITIVE {
                        let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_certified = Some((
                                action,
                                subset.clone(),
                                perm.clone(),
                                beta.clone(),
                            ));
                        }
                    }

                    // Uncertain: -EPS < β_i (some β_i near zero, predicate UNKNOWN)
                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_uncertain = Some((
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

    // Return based on certified candidate (primary). Uncertain is supplementary.
    let certified = best_certified?;
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);

    // Safety net: if an UNKNOWN orbit achieves lower action than the best certified
    // orbit, the reported capacity might be wrong and we cannot resolve it at f64
    // precision. Fail loudly rather than publish a potentially false result.
    assert!(
        uncertain_cap >= certified.0,
        "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} (gap = {:.6e}). \
         An UNKNOWN orbit achieves lower action than the best certified orbit. \
         Cannot resolve at f64 precision.",
        certified.0, uncertain_cap, certified.0 - uncertain_cap,
    );

    // Reverse σ from internal algebraic order to physical orbit direction
    let mut phys_perm = certified.2;
    phys_perm.reverse();
    let mut phys_beta = certified.3;
    phys_beta.reverse();
    Some(EhzResult {
        capacity: certified.0,
        capacity_uncertain: uncertain_cap,
        best_subset: certified.1,
        best_permutation: phys_perm,
        best_beta: phys_beta,
        iterations,
    })
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

/// Build directed facet adjacency matrix: adj[i][j] = true iff the algorithm's
/// internal ordering can have i followed by j.
///
/// This combines vertex adjacency (F_i ∩ F_j ≠ ∅) with the directed ω₀ condition.
/// In the algorithm's internal (algebraic) ordering, consecutive (i, j) corresponds
/// to a physical Reeb transition F_j → F_i, so we require ω₀(n_j, n_i) ≥ 0.
///
/// [lem:transition-feasibility] Condition (1): ω₀(n_i, n_j) ≥ 0 is necessary for
/// physical transition F_i → F_j. Here we check ω₀(n_j, n_i) ≥ 0 for the reversed
/// direction matching the algebraic convention.
pub(crate) fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let vertex_adj = build_adjacency_matrix(polytope);
    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            // Algebraic convention: adj[i][j] = true iff consecutive (i,j) in σ is valid.
            // Physical meaning: transition F_j → F_i, requiring ω₀(n_j, n_i) ≥ 0.
            adj[i][j] = vertex_adj[i][j] && omega0(&normals[j], &normals[i]) >= 0.0;
        }
    }
    adj
}

/// Check if a cyclic permutation forms an adjacent cycle.
fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

/// Compute c_EHZ(K) with directed adjacency pruning; see `[cor:adjacency-pruning]` (thesis).
///
/// **Production variant used in all experiments.**
/// Skips (S, σ) pairs where consecutive facets violate vertex adjacency
/// or the directed ω₀ condition `[lem:transition-feasibility]` condition (1).
/// This is the A2 pruning level from the ablation study.
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    // Precompute directed adjacency matrix (A2: vertex adj + ω₀ condition)
    let adj = build_directed_adjacency_matrix(polytope);

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
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
                            best_uncertain = Some((
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

    let certified = best_certified?;
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);

    // Safety net: if an UNKNOWN orbit achieves lower action than the best certified
    // orbit, the reported capacity might be wrong and we cannot resolve it at f64
    // precision. Fail loudly rather than publish a potentially false result.
    assert!(
        uncertain_cap >= certified.0,
        "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} (gap = {:.6e}). \
         An UNKNOWN orbit achieves lower action than the best certified orbit. \
         Cannot resolve at f64 precision.",
        certified.0, uncertain_cap, certified.0 - uncertain_cap,
    );

    // Reverse σ from internal algebraic order to physical orbit direction
    let mut phys_perm = certified.2;
    phys_perm.reverse();
    let mut phys_beta = certified.3;
    phys_beta.reverse();
    Some(EhzResult {
        capacity: certified.0,
        capacity_uncertain: uncertain_cap,
        best_subset: certified.1,
        best_permutation: phys_perm,
        best_beta: phys_beta,
        iterations,
    })
}

/// Test dataset infrastructure for property tests.
/// Only used in tests, but declared as a public module to allow cross-crate test imports.
pub mod test_dataset;

#[cfg(test)]
mod capacity_properties_test;

#[cfg(test)]
#[path = "hk2017_test.rs"]
mod hk2017_test;

#[cfg(test)]
#[path = "square_product_diagnostic.rs"]
mod square_product_diagnostic;
