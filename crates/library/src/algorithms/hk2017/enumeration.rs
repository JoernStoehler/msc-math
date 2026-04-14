//! Subset/permutation traversal for HK2017 capacity enumeration.

use crate::algorithms::capacity_accumulator::{CapacityAccumulator, CapacityResult};
use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::EPS_Q_POSITIVE;
use crate::kkt::Verdict;

use super::combinatorics::combinations;
use super::permutations::for_each_cyclic_permutation;
use super::solver_bridge::solve_and_convert;

pub(super) struct EnumerationOutcome {
    pub(super) result: CapacityResult,
    pub(super) best_subset: Vec<usize>,
}

pub(super) fn enumerate_unpruned(polytope: &Polytope4D) -> Option<EnumerationOutcome> {
    enumerate_impl(polytope, false)
}

pub(super) fn enumerate_pruned(polytope: &Polytope4D) -> Option<EnumerationOutcome> {
    enumerate_impl(polytope, true)
}

fn enumerate_impl(polytope: &Polytope4D, use_pruning: bool) -> Option<EnumerationOutcome> {
    let f = polytope.facet_count();
    let adj = use_pruning.then(|| build_transition_matrix(polytope));
    let mut acc = CapacityAccumulator::new();
    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(adj) = adj.as_ref() {
                    if !is_feasible_cycle(perm, adj) {
                        return;
                    }
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

    Some(EnumerationOutcome {
        result,
        best_subset,
    })
}
