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
//! Architecture:
//! - `mod.rs` — public API (`ehz_capacity_unpruned`, `ehz_capacity`, `EhzResult`) and
//!   high-level enumeration/pruning orchestration.
//! - `combinatorics.rs` — subset enumeration (`combinations`) for HK2017 loops.
//! - `permutations.rs` — cyclic permutation generation for each subset.
//! - `solver_bridge.rs` — conversion from KKT solver output to accumulator `Solution`.
//! - `orbit_recovery.rs` (test-only) — geometric orbit reconstruction for regression checks.
//! - `generate_capacity_fixtures.rs` (test-only) — fixture catalog/dataset generation/loaders.
//! - `tests_*.rs` — split test concerns (literature, pruning, invariance/conformality,
//!   derivative validation, and regression/edge-case coverage).
//!
//! # Complexity
//!
//! sum_{m=2}^{F} C(F,m) * (m-1)! — exponential in F.
//!
//! Mathematical correspondence: [alg:ehz]

mod combinatorics;
#[cfg(test)]
mod generate_capacity_fixtures;
#[cfg(test)]
mod orbit_recovery;
mod permutations;
mod solver_bridge;

use crate::algorithms::capacity_accumulator::{CapacityAccumulator, CapacityResult};
use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::EPS_Q_POSITIVE;
use crate::kkt::Verdict;
use combinatorics::combinations;
use permutations::for_each_cyclic_permutation;
use solver_bridge::solve_and_convert;

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
    let best_subset = best_subset_certified.map(|(_, s)| s).unwrap_or_else(|| {
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
    let adj = build_transition_matrix(polytope);
    let mut acc = CapacityAccumulator::new();

    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                // Adjacency pruning: skip non-adjacent cycles.
                if !is_feasible_cycle(perm, &adj) {
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
    let best_subset = best_subset_certified.map(|(_, s)| s).unwrap_or_else(|| {
        let mut s = result.best_permutation.clone();
        s.sort();
        s
    });

    Some(EhzResult {
        result,
        best_subset,
    })
}

#[cfg(test)]
mod tests_capacity_derivative;
#[cfg(test)]
mod tests_conformality;
#[cfg(test)]
mod tests_kkt_edge_cases;
#[cfg(test)]
mod tests_literature;
#[cfg(test)]
mod tests_pruning;
#[cfg(test)]
mod tests_regression;
#[cfg(test)]
mod tests_symplectic_invariance;
