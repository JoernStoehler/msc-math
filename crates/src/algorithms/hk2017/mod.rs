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
/// The `best_permutation` in [`EhzResult`] follows the **positive Reeb direction**:
/// σ = [a, b, c, ...] means the Reeb trajectory visits F_a → F_b → F_c → ... → F_a.
/// For consecutive facets, ω₀(n_{σ(k)}, n_{σ(k+1)}) ≥ 0 (positive Reeb direction,
/// R_k = (2/h_k) J₀ n_k).
///
/// Q(β) = (1/2) β^T H β > 0 for permutations in positive Reeb direction.
/// Permutations are passed directly to `solve_kkt` — no reversal needed.
///
/// # Complexity
///
/// Σ_{m=2}^{F} C(F,m) · (m-1)! — exponential in F.
pub mod permutations;

use crate::geom::polytope::Polytope4D;
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
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    // Enumerate all subsets S ⊆ {0,...,F-1} with |S| ≥ 2.
    for m in 2..=f {
        for subset in combinations(f, m) {
            // For each cyclic permutation of S
            for perm in cyclic_permutations(&subset) {
                iterations += 1;

                if let Some(result) = solve_kkt(normals, heights, &perm) {
                    let q_val = result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        continue;
                    }
                    let beta_min = result.beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    // Certified: β_i > +EPS (all predicates TRUE)
                    if beta_min > EPS_BETA_POSITIVE {
                        let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_certified = Some((
                                action,
                                subset.clone(),
                                perm.clone(),
                                result.beta.clone(),
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
                                result.beta,
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

    // Safety net: if an UNKNOWN orbit achieves significantly lower action than the
    // best certified orbit, the reported capacity might be wrong and we cannot
    // resolve it at f64 precision. Fail loudly rather than publish a potentially
    // false result.
    // Tolerance 1e-10: capacity values are O(1)–O(10), so 1e-10 is ~10 orders
    // below typical values. Consistent with billiard_capacity tolerance.
    // Near-degenerate Lagrangian products (e.g. (4,4) at θ≈0°) produce
    // gaps up to ~2.4e-11 from f64 rounding noise.
    let gap = certified.0 - uncertain_cap;
    assert!(
        gap <= 1e-10,
        "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} (gap = {:.6e}). \
         An UNKNOWN orbit achieves lower action than the best certified orbit. \
         Cannot resolve at f64 precision.",
        certified.0, uncertain_cap, gap,
    );

    // Sanity: winning orbit has positive capacity (Q > 0 ⟹ action = 0.5/Q > 0).
    assert!(certified.0 > 0.0, "capacity must be positive, got {:.2e}", certified.0);
    assert!(certified.0.is_finite(), "capacity must be finite, got {:.2e}", certified.0);

    // Candidate already stores perm and β in natural (positive Reeb) order.
    Some(EhzResult {
        capacity: certified.0,
        capacity_uncertain: uncertain_cap,
        best_subset: certified.1,
        best_permutation: certified.2,
        best_beta: certified.3,
        iterations,
    })
}


/// Generate all combinations of `k` elements from `{0, ..., n-1}` in lexicographic order.
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

/// Build facet adjacency matrix: adj[i][j] = true iff F_i ∩ F_j ≠ ∅.
/// Two facets are adjacent if they share at least one vertex.
/// Diagonal is false (a facet is not adjacent to itself); safe because
/// `is_adjacent_cycle` iterates distinct-element permutations.
///
/// Uses the exact adjacency matrix from `Polytope4D`.
pub fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let poly_adj = polytope.adjacency();

    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            adj[i][j] = poly_adj[(i, j)];
        }
    }
    adj
}

/// Build directed facet adjacency matrix in the **physical Reeb direction**:
/// `adj[i][j] = true` iff the transition F_i → F_j is feasible.
///
/// This combines vertex adjacency (F_i ∩ F_j ≠ ∅) with the directed ω₀ condition
/// from `[lem:numerical-transition-feasibility]`: a transition F_i → F_j exists
/// only if ω₀(n_i, n_j) ≥ 0, where n_i, n_j are the outward facet normals.
///
/// Uses the exact omega_signs matrix from the rational pipeline (always available),
/// so there is no f64 tolerance ambiguity near ω₀ = 0.
pub fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let vertex_adj = build_adjacency_matrix(polytope);
    let omega_signs = polytope.omega_signs();
    let mut adj = vec![vec![false; f]; f];

    for i in 0..f {
        for j in 0..f {
            if !vertex_adj[i][j] {
                continue;
            }
            // Physical convention: adj[i][j] needs ω₀(n_i, n_j) ≥ 0.
            // omega_signs[(i,j)] directly stores sign(ω₀(y_i, y_j)),
            // which has the same sign as ω₀(n_i, n_j) since h_i, h_j > 0.
            // Values: +1, -1, or 0. Transition allowed when >= 0.
            adj[i][j] = omega_signs[(i, j)] >= 0;
        }
    }

    adj
}

/// Check if a cyclic permutation forms an adjacent cycle.
pub fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

/// Compute c_EHZ(K) with directed adjacency pruning; see `[cor:adjacency-pruning]` (thesis).
///
/// **Production variant used in all experiments.**
/// Skips (S, σ) pairs where consecutive facets violate vertex adjacency
/// or the directed ω₀ condition `[lem:numerical-transition-feasibility]` condition (1).
/// This is the A2 pruning level from the ablation study.
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    // Precompute directed adjacency matrix (A2: vertex adj + ω₀ condition)
    let adj = build_directed_adjacency_matrix(polytope);

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                // **ADJACENCY PRUNING**: skip non-adjacent cycles.
                // adj uses physical convention: adj[i][j] = transition F_i → F_j feasible.
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }

                iterations += 1;

                if let Some(result) = solve_kkt(normals, heights, perm) {
                    let q_val = result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = result.beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    if beta_min > EPS_BETA_POSITIVE {
                        let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_certified = Some((
                                action,
                                subset.clone(),
                                perm.to_vec(),
                                result.beta.clone(),
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
                                result.beta,
                            ));
                        }
                    }
                }
            });
        }
    }

    let certified = best_certified?;
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);

    // Safety net: see ehz_capacity_unpruned for full comment.
    // Tolerance 1e-10: consistent with billiard_capacity and ehz_capacity_unpruned.
    let gap = certified.0 - uncertain_cap;
    assert!(
        gap <= 1e-10,
        "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} (gap = {:.6e}). \
         An UNKNOWN orbit achieves lower action than the best certified orbit. \
         Cannot resolve at f64 precision.",
        certified.0, uncertain_cap, gap,
    );

    // Sanity: winning orbit has positive capacity (Q > 0 ⟹ action = 0.5/Q > 0).
    assert!(certified.0 > 0.0, "capacity must be positive, got {:.2e}", certified.0);
    assert!(certified.0.is_finite(), "capacity must be finite, got {:.2e}", certified.0);

    // Candidate already stores perm and β in natural (positive Reeb) order.
    Some(EhzResult {
        capacity: certified.0,
        capacity_uncertain: uncertain_cap,
        best_subset: certified.1,
        best_permutation: certified.2,
        best_beta: certified.3,
        iterations,
    })
}

pub mod recover;

/// Test dataset infrastructure for property tests.
/// Only used in tests, but declared as a public module to allow cross-crate test imports.
pub mod test_dataset;

#[cfg(test)]
mod capacity_properties_test;

#[cfg(test)]
#[path = "hk2017_test.rs"]
mod hk2017_test;

#[cfg(test)]
#[path = "recover_test.rs"]
mod recover_test;

#[cfg(test)]
#[path = "square_product_diagnostic.rs"]
mod square_product_diagnostic;
