//! Exhaustive orbit collection for visualization export.
//!
//! This keeps the HK2017 enumeration separate from trajectory recovery and
//! JSON export so `main.rs` stays focused on orchestration.

use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for_dual_vertices, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

/// A valid orbit found by exhaustive enumeration.
pub(crate) struct CollectedOrbit {
    pub(crate) action: f64,
    pub(crate) permutation: Vec<usize>,
    pub(crate) beta: Vec<f64>,
}

/// Collect all certified Reeb orbits for the polytope, sorted by action.
pub(crate) fn collect_all_orbits(polytope: &Polytope4D) -> Vec<CollectedOrbit> {
    let dual_vertices = polytope.dual_vertices_f64();
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    );
    let mut orbits: Vec<CollectedOrbit> = Vec::new();

    for_each_sigma_pruned_by_transition(&transition_is_allowed, |perm: &[usize]| {
        if let KktOutcome::Feasible(result) = solve_kkt_for_dual_vertices(dual_vertices, perm) {
            let q_val = result.q_corrected;
            if q_val <= EPS_Q_POSITIVE {
                return;
            }
            let beta_min = result.beta.iter().cloned().fold(f64::INFINITY, f64::min);
            if beta_min <= EPS_BETA_POSITIVE {
                return;
            }

            orbits.push(CollectedOrbit {
                action: 0.5 / q_val,
                permutation: perm.to_vec(),
                beta: result.beta,
            });
        }
    });

    orbits.sort_by(|a, b| a.action.partial_cmp(&b.action).unwrap());
    orbits
}
