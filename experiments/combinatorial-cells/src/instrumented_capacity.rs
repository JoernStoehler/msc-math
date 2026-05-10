//! Instrumented capacity helper for combinatorial-cell experiments.

use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

/// Shared "all valid orbit" summary used by several combinatorial-cells binaries.
///
/// This stays experiment-local because these binaries care about the total valid-orbit
/// count and the best/second-best action gap, which are not yet part of the
/// library's near-minimum collector surface.
#[derive(Debug, Clone)]
pub struct InstrumentedCapacitySummary {
    pub capacity: f64,
    pub best_permutation: Vec<usize>,
    pub n_valid_orbits: usize,
    /// `action_second_best - action_best`. `f64::INFINITY` if there is only one orbit.
    pub orbit_gap: f64,
}

/// Enumerate all valid HK2017 orbits, then return the best action/permutation plus
/// the total valid-orbit count and the best/second-best action gap.
pub fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedCapacitySummary> {
    let f = polytope.facet_count();
    let transition_is_allowed = build_transition_matrix(polytope);
    let mut orbits: Vec<(f64, Vec<usize>)> = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_feasible_cycle(perm, &transition_is_allowed) {
                    return;
                }

                if let KktOutcome::Feasible(kkt_result) = solve_kkt_for(polytope, perm) {
                    let q_val = kkt_result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = kkt_result
                        .beta
                        .iter()
                        .copied()
                        .fold(f64::INFINITY, f64::min);
                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push((0.5 / q_val, perm.to_vec()));
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.0.total_cmp(&b.0));

    let best_action = orbits[0].0;
    let best_permutation = orbits[0].1.clone();
    let n_valid_orbits = orbits.len();
    let orbit_gap = if orbits.len() >= 2 {
        orbits[1].0 - orbits[0].0
    } else {
        f64::INFINITY
    };

    Some(InstrumentedCapacitySummary {
        capacity: best_action,
        best_permutation,
        n_valid_orbits,
        orbit_gap,
    })
}
