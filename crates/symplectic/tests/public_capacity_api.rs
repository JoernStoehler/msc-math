use nalgebra::DMatrix;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, known_polytopes, solve_orbit_sigma_saddle_point,
    solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates, OrbitGuaranteeMode,
};

#[test]
fn unpruned_hk2017_candidates_aggregate_from_flat_dual_vertices() {
    let kp = known_polytopes::simplex();
    let dual_vertices = &kp.dual_vertices_f64;
    let dual_vertices_exact = &kp.dual_vertices;

    let (orbits, iterations) =
        solve_unpruned_hk2017_candidates(dual_vertices).expect("simplex candidates");
    let result = aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("simplex capacity");

    assert!(
        (result.min_action - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.min_action,
        kp.capacity
    );
    assert!(!result.best_sigma().is_empty());
}

#[test]
fn pruned_hk2017_candidates_use_explicit_transition_matrix() {
    let kp = known_polytopes::simplex();
    let dual_vertices = &kp.dual_vertices_f64;
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &kp.facet_intersection_is_nonempty,
            &kp.omega_signs,
        );

    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(dual_vertices, &transition_is_allowed)
            .expect("pruned simplex candidates");

    assert!(iterations > 0);
    assert!(!orbits.is_empty());
    assert!(orbits
        .iter()
        .all(|orbit| !orbit.sigma.is_empty() && orbit.sigma.len() == orbit.beta.len()));
}

#[test]
fn single_sigma_saddle_point_solver_is_concrete() {
    let kp = known_polytopes::simplex();
    let dual_vertices = &kp.dual_vertices_f64;
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &kp.facet_intersection_is_nonempty,
            &kp.omega_signs,
        );
    let (orbits, _) = solve_pruned_hk2017_candidates(dual_vertices, &transition_is_allowed)
        .expect("pruned simplex candidates");
    let sigma = orbits[0].sigma.clone();

    let orbit = solve_orbit_sigma_saddle_point(dual_vertices, &sigma)
        .expect("known candidate should solve");

    assert_eq!(orbit.sigma, sigma);
    assert_eq!(orbit.sigma.len(), orbit.beta.len());
    assert!(orbit.q > 0.0);
    assert!(orbit.q_error_bound >= 0.0);
}

#[test]
#[should_panic(expected = "transition_is_allowed")]
fn pruned_hk2017_candidates_reject_mismatched_transition_matrix() {
    let kp = known_polytopes::simplex();
    let bad_transition = DMatrix::from_element(1, 1, true);

    let _ = solve_pruned_hk2017_candidates(&kp.dual_vertices_f64, &bad_transition);
}
