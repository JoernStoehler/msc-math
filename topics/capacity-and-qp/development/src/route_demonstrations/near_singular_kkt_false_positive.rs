use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::{known_polytopes, solve_orbit_sigma_saddle_point, OrbitAdmissibility};

/// Demonstrates a non-pruning f64 failure on a near-singular KKT system.
///
/// Conservative transition pruning is not enough: this sigma is present in the
/// exact binary64 transition-pruned stream, and the f64 KKT solve reports a
/// healthy-looking positive beta margin. Exact rational KKT over the same
/// binary64-rounded HKO input rejects the sigma. The large action interval is
/// the route-level signal that this candidate needs exact resolution, not a
/// trusted f64 admissibility decision.
#[test]
fn f64_kkt_can_accept_sigma_rejected_by_exact_binary64_kkt() {
    let dual_vertices = &known_polytopes::hko_pentagon().dual_vertices_f64;
    let sigma = vec![1, 8, 7, 3, 4, 5, 9];

    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let transition = exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    assert!(
        SimpleDirectedCyclesCanonical::new(&transition).any(|candidate| candidate == sigma),
        "the sigma is in the exact transition-pruned stream; this is not a pruning failure"
    );

    let f64_orbit = solve_orbit_sigma_saddle_point(dual_vertices, &sigma)
        .expect("f64 KKT solve should return a candidate");
    assert_eq!(f64_orbit.admissibility, OrbitAdmissibility::AdmissibleF64);
    assert!(
        f64_orbit.beta_margin > 0.05,
        "the f64 beta margin should look safely positive, not barely roundoff-scale: {f64_orbit:?}"
    );
    assert!(
        f64_orbit.q_error_bound > 1e-3,
        "near-singular KKT should expose a large q/action uncertainty: {f64_orbit:?}"
    );
    assert!(
        f64_orbit.action_upper - f64_orbit.action_lower > 0.1,
        "the action interval should be wide enough that the route cannot treat the scalar as a certificate: {f64_orbit:?}"
    );

    assert!(
        solve_kkt_exact(&exact_vertices, &sigma).is_none(),
        "exact binary64 KKT rejects this f64-admissible sigma"
    );
}
