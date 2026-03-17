//! HK2017 algorithm for EHZ capacity of general 4D polytopes.
//!
//! Implements the Haim-Kislev 2017 exhaustive enumeration algorithm: for each
//! subset S of facets and each cyclic permutation sigma of S, solve the KKT
//! system for the constrained maximum of Q(beta) and track the minimum action
//! across all certified solutions.
//!
//! Two entry points:
//! - `ehz_capacity`: production variant with directed adjacency pruning ([cor:adjacency-pruning])
//! - `ehz_capacity_unpruned`: reference implementation without pruning
//!
//! Both use `CapacityAccumulator` for the enumerate -> solve -> track pattern.
//!
//! Submodules:
//! - `permutations` — cyclic permutation generation (allocating + callback)
//! - `orbit_recovery` — recover a Reeb orbit from a KKT solution
//! - `generate_capacity_fixtures` — fixture generation for 33 test polytopes
//!
//! # Complexity
//!
//! sum_{m=2}^{F} C(F,m) * (m-1)! — exponential in F.
//!
//! Mathematical correspondence: [alg:ehz]

pub mod permutations;
pub mod orbit_recovery;
pub mod generate_capacity_fixtures;

#[cfg(test)]
#[path = "permutations_test.rs"]
mod permutations_test;

#[cfg(test)]
#[path = "orbit_recovery_test.rs"]
mod orbit_recovery_test;

#[cfg(test)]
#[path = "literature_test.rs"]
mod literature_test;

#[cfg(test)]
#[path = "kkt_edge_cases_test.rs"]
mod kkt_edge_cases_test;

#[cfg(test)]
#[path = "pruning_test.rs"]
mod pruning_test;

#[cfg(test)]
#[path = "regression_test.rs"]
mod regression_test;

#[cfg(test)]
#[path = "conformality_test.rs"]
mod conformality_test;

#[cfg(test)]
#[path = "symplectic_invariance_test.rs"]
mod symplectic_invariance_test;

#[cfg(test)]
#[path = "capacity_derivative_test.rs"]
mod capacity_derivative_test;

use crate::algorithms::capacity_accumulator::{CapacityAccumulator, CapacityResult};
use crate::algorithms::facet_adjacency::{build_directed_adjacency_matrix, is_adjacent_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use crate::kkt::{classify_margin, Solution, Verdict};
use permutations::for_each_cyclic_permutation;

/// Result of the EHZ capacity computation.
///
/// Wraps [`CapacityResult`] (shared accumulator output) plus the algorithm-specific
/// `best_subset` field identifying which facet indices participate in the optimal orbit.
///
/// Access capacity fields via `.result.capacity` (no Deref — explicit field access).
///
/// [alg:ehz]: result of exhaustive (S, sigma) enumeration.
#[derive(Clone, Debug)]
pub struct EhzResult {
    /// Core capacity result from the accumulator: capacity, uncertainty, best
    /// permutation, beta vector, and iteration count.
    pub result: CapacityResult,
    /// Facet indices S participating in the optimal orbit (unordered).
    pub best_subset: Vec<usize>,
}

/// Compute c_EHZ(K) for a convex polytope K in R^4.
///
/// Reference (unpruned) implementation of [alg:ehz]: exhaustive search over all
/// (S, sigma) pairs with |S| >= 2. For production use, prefer [`ehz_capacity`]
/// which applies directed adjacency pruning ([cor:adjacency-pruning]).
///
/// Returns `None` if no valid (S, sigma) pair yields a certified beta > 0
/// (should not happen for valid polytopes, but guards against degenerate input).
///
/// # Permutation ordering convention
///
/// `best_permutation` follows the **positive Reeb direction**: sigma = [a, b, c, ...]
/// means the Reeb trajectory visits F_a -> F_b -> F_c -> ... -> F_a.
/// For consecutive facets, omega_0(n_{sigma(k)}, n_{sigma(k+1)}) >= 0.
///
/// [alg:ehz]: exhaustive capacity computation.
pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let mut acc = CapacityAccumulator::new();

    // Track which subset corresponds to the best certified permutation.
    // The accumulator tracks permutations but not subsets — we need the subset
    // for the EhzResult.
    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(solution) = solve_and_convert(polytope, perm) {
                    // Track subset for the best certified candidate.
                    if solution.verdict == Verdict::True && solution.q > EPS_Q_POSITIVE {
                        let action = 0.5 / solution.q;
                        let update = best_subset_certified
                            .as_ref()
                            .is_none_or(|(best, _)| action < *best);
                        if update {
                            best_subset_certified = Some((action, subset.clone()));
                        }
                    }
                    acc.submit(perm, &solution);
                }
            });
        }
    }

    let result = acc.finalize()?;
    // Use tracked subset if available; fallback to deriving from best_permutation.
    let best_subset = best_subset_certified
        .map(|(_, s)| s)
        .unwrap_or_else(|| {
            let mut s = result.best_permutation.clone();
            s.sort();
            s
        });

    Some(EhzResult {
        result,
        best_subset,
    })
}

/// Compute c_EHZ(K) with directed adjacency pruning.
///
/// **Production variant used in all experiments.** Skips (S, sigma) pairs where
/// consecutive facets violate vertex adjacency or the directed omega_0 condition
/// from [lem:numerical-transition-feasibility]. This is the A2 pruning level
/// from the ablation study.
///
/// Returns `None` if no valid orbit is found.
///
/// [alg:ehz] with [cor:adjacency-pruning].
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let adj = build_directed_adjacency_matrix(polytope);
    let mut acc = CapacityAccumulator::new();

    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                // Adjacency pruning: skip non-adjacent cycles.
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }

                if let Some(solution) = solve_and_convert(polytope, perm) {
                    if solution.verdict == Verdict::True && solution.q > EPS_Q_POSITIVE {
                        let action = 0.5 / solution.q;
                        let update = best_subset_certified
                            .as_ref()
                            .is_none_or(|(best, _)| action < *best);
                        if update {
                            best_subset_certified = Some((action, subset.clone()));
                        }
                    }
                    acc.submit(perm, &solution);
                }
            });
        }
    }

    let result = acc.finalize()?;
    let best_subset = best_subset_certified
        .map(|(_, s)| s)
        .unwrap_or_else(|| {
            let mut s = result.best_permutation.clone();
            s.sort();
            s
        });

    Some(EhzResult {
        result,
        best_subset,
    })
}

/// Generate all combinations of `k` elements from `{0, ..., n-1}` in lexicographic order.
///
/// Returns an empty vec if `k == 0` or `k > n`.
///
/// [alg:ehz]: "for each S subseteq {1, ..., F} with |S| >= 2".
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 || k > n {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

/// Recursive helper for lexicographic combination generation.
fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut [usize],
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.to_vec());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

// ── Internal helpers ──

/// Solve the KKT system for a (polytope, permutation) pair and convert the
/// result into a `Solution` for the accumulator.
///
/// The saddle-point solver returns `KktResult` with `q_corrected` and `beta`.
/// We compute `margin = min(beta)` and classify via `classify_margin` to produce
/// a `Solution` with a trinary `Verdict`.
fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let kkt = solve_kkt_for(polytope, perm)?;
    Some(kkt_result_to_solution(kkt))
}

/// Convert a `KktResult` (saddle-point solver output) to a `Solution` (accumulator input).
///
/// Maps: q_corrected -> q, beta -> beta, min(beta) -> margin, classify_margin -> verdict.
fn kkt_result_to_solution(result: KktResult) -> Solution {
    let margin = result
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    Solution {
        verdict: classify_margin(margin),
        q: result.q_corrected,
        beta: result.beta,
        margin,
    }
}
