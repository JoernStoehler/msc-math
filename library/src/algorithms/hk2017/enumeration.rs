//! Subset/permutation traversal for HK2017 capacity enumeration.

use crate::algorithms::capacity_accumulator::CapacityResult;
use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::algorithms::orbit_search::{
    collect_legacy_capacity,
    collect_orbits,
    OrbitGuaranteeMode,
    OrbitSearchResult,
    OrbitSolveBackend,
};
use crate::geom::polytope::Polytope4D;

use super::combinatorics::combinations;
use super::permutations::for_each_cyclic_permutation;

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

pub(super) fn collect_unpruned(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, crate::algorithms::OrbitSearchError> {
    collect_impl(polytope, gap, mode, backend, false)
}

pub(super) fn collect_pruned(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, crate::algorithms::OrbitSearchError> {
    collect_impl(polytope, gap, mode, backend, true)
}

fn enumerate_impl(polytope: &Polytope4D, use_pruning: bool) -> Option<EnumerationOutcome> {
    let f = polytope.facet_count();
    let adj = use_pruning.then(|| build_transition_matrix(polytope));
    let (result, best_subset) = collect_legacy_capacity(
        polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| {
            for m in 2..=f {
                for subset in combinations(f, m) {
                    for_each_cyclic_permutation(&subset, &mut |perm| {
                        if let Some(adj) = adj.as_ref() {
                            if !is_feasible_cycle(perm, adj) {
                                return;
                            }
                        }
                        visit(perm, subset.clone());
                    });
                }
            }
        },
        |result| {
            let mut subset = result.best_permutation.clone();
            subset.sort();
            subset
        },
    )?;

    Some(EnumerationOutcome {
        result,
        best_subset,
    })
}

fn collect_impl(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
    use_pruning: bool,
) -> Result<OrbitSearchResult, crate::algorithms::OrbitSearchError> {
    let f = polytope.facet_count();
    let adj = use_pruning.then(|| build_transition_matrix(polytope));

    collect_orbits(polytope, gap, mode, backend, |visit| {
        for m in 2..=f {
            for subset in combinations(f, m) {
                for_each_cyclic_permutation(&subset, &mut |perm| {
                    if let Some(adj) = adj.as_ref() {
                        if !is_feasible_cycle(perm, adj) {
                            return;
                        }
                    }
                    visit(perm);
                });
            }
        }
    })
}
