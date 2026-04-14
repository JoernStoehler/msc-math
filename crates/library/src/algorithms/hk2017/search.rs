//! Exhaustive HK2017 search entry points (pruned and unpruned).

use crate::algorithms::capacity_accumulator::CapacityAccumulator;
use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::EPS_Q_POSITIVE;
use crate::kkt::Verdict;

use super::invariants::solve_and_convert;
use super::permutations::for_each_cyclic_permutation;
use super::selection::combinations;
use super::EhzResult;

pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult> {
    run_search(polytope, |_, _| true)
}

pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let adj = build_transition_matrix(polytope);
    run_search(polytope, |perm, _subset| is_feasible_cycle(perm, &adj))
}

fn run_search<F>(polytope: &Polytope4D, mut keep_perm: F) -> Option<EhzResult>
where
    F: FnMut(&[usize], &[usize]) -> bool,
{
    let f = polytope.facet_count();
    let mut acc = CapacityAccumulator::new();
    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !keep_perm(perm, &subset) {
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
