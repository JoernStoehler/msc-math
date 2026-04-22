//! Exhaustive orbit collection for visualization export.
//!
//! This keeps the HK2017 enumeration separate from trajectory recovery and
//! JSON export so `main.rs` stays focused on orchestration.

use symplectic::algorithms::hk2017::for_each_sigma_pruned;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

/// A valid orbit found by exhaustive enumeration.
pub(crate) struct CollectedOrbit {
    pub(crate) action: f64,
    pub(crate) permutation: Vec<usize>,
    pub(crate) beta: Vec<f64>,
}

/// Collect all certified Reeb orbits for the polytope, sorted by action.
pub(crate) fn collect_all_orbits(polytope: &Polytope4D) -> Vec<CollectedOrbit> {
    let mut orbits: Vec<CollectedOrbit> = Vec::new();

    for_each_sigma_pruned(polytope, |perm: &[usize]| {
        if let KktOutcome::Feasible(result) = solve_kkt_for(polytope, perm) {
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
