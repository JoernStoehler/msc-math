//! Subset/permutation traversal for HK2017 sigma generation.

use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;

use super::combinatorics::combinations;
use super::permutations::for_each_cyclic_permutation;

/// Visit every HK2017 sigma without transition pruning.
pub fn for_each_sigma_unpruned(polytope: &Polytope4D, mut visit: impl FnMut(&[usize])) {
    for_each_sigma_impl(polytope, false, &mut visit)
}

/// Visit every HK2017 sigma that survives transition pruning.
pub fn for_each_sigma_pruned(polytope: &Polytope4D, mut visit: impl FnMut(&[usize])) {
    for_each_sigma_impl(polytope, true, &mut visit)
}

fn for_each_sigma_impl(polytope: &Polytope4D, use_pruning: bool, visit: &mut dyn FnMut(&[usize])) {
    let f = polytope.facet_count();
    let transition_is_allowed = use_pruning.then(|| build_transition_matrix(polytope));

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(transition_is_allowed) = transition_is_allowed.as_ref() {
                    if !is_feasible_cycle(perm, transition_is_allowed) {
                        return;
                    }
                }
                visit(perm);
            });
        }
    }
}
