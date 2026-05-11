//! Instrumented orbit search for HKO local-maximum experiments.

use crate::HkoPolytopeCache;
use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::algorithms::{OrbitAdmissibility, OrbitKktData};
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for_dual_vertices, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

#[derive(Debug, Clone)]
pub struct InstrumentedOrbitSearch {
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub orbits: Vec<OrbitKktData>,
    pub iterations: u64,
}

fn action_bounds_from_q(q: f64, q_error_bound: f64) -> (f64, f64) {
    let q_upper = q + q_error_bound;
    let action_lower = 0.5 / q_upper;
    let q_lower = q - q_error_bound;
    let action_upper = if q_lower > EPS_Q_POSITIVE {
        0.5 / q_lower
    } else {
        f64::INFINITY
    };
    (action_lower, action_upper)
}

/// Enumerate all "valid" HK2017 orbits for the HKO local-maximum experiments.
///
/// These binaries intentionally keep the stricter `beta > EPS_BETA_POSITIVE`
/// validity policy rather than adopting the richer library collector semantics.
pub fn ehz_capacity_instrumented(polytope: &HkoPolytopeCache) -> Option<InstrumentedOrbitSearch> {
    let f = polytope.facet_count();
    let dual_vertices = &polytope.dual_vertices_f64;
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );

    let mut orbits: Vec<OrbitKktData> = Vec::new();
    let mut best_uncertain_action: Option<f64> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_feasible_cycle(perm, &transition_is_allowed) {
                    return;
                }
                iterations += 1;

                if let KktOutcome::Feasible(kkt_result) =
                    solve_kkt_for_dual_vertices(dual_vertices, perm)
                {
                    let q_val = kkt_result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta = &kkt_result.beta;
                    let beta_min = beta.iter().copied().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;
                    let (action_lower, action_upper) =
                        action_bounds_from_q(q_val, kkt_result.q_error_bound);

                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push(OrbitKktData {
                            sigma: perm.to_vec(),
                            beta: beta.clone(),
                            beta_margin: beta_min,
                            action,
                            action_lower,
                            action_upper,
                            q: q_val,
                            q_error_bound: kkt_result.q_error_bound,
                            mu: Some(
                                kkt_result
                                    .mu
                                    .as_slice()
                                    .try_into()
                                    .expect("closure multiplier must stay 4D"),
                            ),
                            xi: Some(kkt_result.xi),
                            admissibility: OrbitAdmissibility::AdmissibleF64,
                        });
                    }

                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain_action.is_none_or(|a| action < a);
                        if update {
                            best_uncertain_action = Some(action);
                        }
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.action.total_cmp(&b.action));
    let capacity = orbits[0].action;
    let capacity_uncertain = best_uncertain_action.unwrap_or(capacity);

    Some(InstrumentedOrbitSearch {
        capacity,
        capacity_uncertain,
        orbits,
        iterations,
    })
}
