use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
};
use num_traits::{Signed, ToPrimitive};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::{known_polytopes, solve_orbit_sigma_saddle_point, OrbitAdmissibility};

/// Demonstrates that `beta > 0` is a genuine numerical decision.
///
/// The exact KKT point for this HKO sigma has all beta coordinates positive, but
/// the smallest exact beta is about 2e-17. The f64 solve rounds the minimum
/// beta margin to zero. A literal `beta_i > 0.0` filter would discard the
/// exactly positive orbit; the guarded f64 route keeps it as indeterminate for
/// exact fallback instead.
#[test]
fn literal_beta_positive_check_can_reject_exactly_positive_orbit() {
    let dual_vertices = &known_polytopes::hko_pentagon().dual_vertices_f64;
    let sigma = vec![0, 1, 6, 7, 3, 4, 5, 9];
    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let transition = exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    assert!(
        SimpleDirectedCyclesCanonical::new(&transition).any(|candidate| candidate == sigma),
        "the sigma is in the exact transition-pruned stream; this is not a pruning failure"
    );

    let f64_orbit = solve_orbit_sigma_saddle_point(dual_vertices, &sigma)
        .expect("f64 KKT solve should produce an unresolved orbit");
    assert_eq!(
        f64_orbit.admissibility,
        OrbitAdmissibility::IndeterminateF64
    );
    assert_eq!(f64_orbit.beta_margin, 0.0);
    assert!(
        !f64_orbit.beta.iter().all(|beta| *beta > 0.0),
        "a literal beta > 0 filter would reject this candidate: beta={:?}",
        f64_orbit.beta
    );

    let exact = solve_kkt_exact(&exact_vertices, &sigma).expect("exact KKT solve");
    assert!(
        exact.beta.iter().all(|beta| beta.is_positive()),
        "exact fallback resolves beta positivity positively: {:?}",
        exact.beta
    );
    let exact_min_beta = exact
        .beta
        .iter()
        .map(|beta| beta.to_f64().unwrap_or(f64::NAN))
        .fold(f64::INFINITY, f64::min);
    assert!(
        exact_min_beta > 0.0 && exact_min_beta < 1e-15,
        "the exact beta margin is positive but far below f64 decision scale: {exact_min_beta:.17e}"
    );
}
