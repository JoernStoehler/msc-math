//! Subset/permutation traversal for HK2017 sigma generation.

use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use nalgebra::DMatrix;

use super::combinatorics::combinations;
use super::permutations::for_each_cyclic_permutation;

/// Visit every HK2017 sigma without transition pruning.
pub fn for_each_sigma_unpruned(polytope: &Polytope4D, mut visit: impl FnMut(&[usize])) {
    for_each_sigma_unpruned_facet_count(polytope.facet_count(), &mut visit)
}

/// Visit every HK2017 sigma that survives transition pruning.
pub fn for_each_sigma_pruned(polytope: &Polytope4D, mut visit: impl FnMut(&[usize])) {
    let transition_is_allowed = build_transition_matrix(polytope);
    for_each_sigma_pruned_by_transition(&transition_is_allowed, &mut visit)
}

/// Visit every HK2017 sigma for a flat facet count, without transition pruning.
pub fn for_each_sigma_unpruned_facet_count(facet_count: usize, mut visit: impl FnMut(&[usize])) {
    for_each_sigma_impl(facet_count, None, &mut visit)
}

/// Visit every HK2017 sigma that survives a flat directed transition matrix.
pub fn for_each_sigma_pruned_by_transition(
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    assert_eq!(
        transition_is_allowed.nrows(),
        transition_is_allowed.ncols(),
        "transition_is_allowed must be square"
    );
    for_each_sigma_impl(
        transition_is_allowed.nrows(),
        Some(transition_is_allowed),
        &mut visit,
    )
}

fn for_each_sigma_impl(
    facet_count: usize,
    transition_is_allowed: Option<&DMatrix<bool>>,
    visit: &mut dyn FnMut(&[usize]),
) {
    for m in 2..=facet_count {
        for subset in combinations(facet_count, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(transition_is_allowed) = transition_is_allowed {
                    if !is_feasible_cycle(perm, transition_is_allowed) {
                        return;
                    }
                }
                visit(perm);
            });
        }
    }
}
