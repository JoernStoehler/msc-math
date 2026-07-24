use nalgebra::{DMatrix, Vector4};
use num_traits::ToPrimitive;
use symplectic::algorithms::capacity_4d::{
    Capacity4d, CapacityError4d, CapacityInput4d, CapacityInputError,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, known_polytopes, solve_orbit_sigma_saddle_point,
    solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates, OrbitGuaranteeMode,
};

#[test]
fn unpruned_hk2017_candidates_aggregate_from_flat_dual_vertices() {
    let kp = known_polytopes::simplex();
    let (orbits, iterations) =
        solve_unpruned_hk2017_candidates(&kp.dual_vertices_f64).expect("simplex candidates");
    let result = aggregate_orbits_with_dual_vertices_exact(
        &kp.dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("simplex capacity");

    assert!((result.min_action - kp.capacity).abs() < 1e-6);
    assert!(!result.best_sigma().is_empty());
}

#[test]
fn pruned_hk2017_candidates_use_explicit_transition_matrix() {
    let kp = known_polytopes::simplex();
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &kp.facet_intersection_is_nonempty,
            &kp.omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&kp.dual_vertices_f64, &transition_is_allowed)
            .expect("pruned simplex candidates");

    assert!(iterations > 0);
    assert!(orbits
        .iter()
        .all(|orbit| !orbit.sigma.is_empty() && orbit.sigma.len() == orbit.beta.len()));
}

#[test]
fn single_sigma_saddle_point_solver_is_concrete() {
    let kp = known_polytopes::simplex();
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &kp.facet_intersection_is_nonempty,
            &kp.omega_signs,
        );
    let (orbits, _) = solve_pruned_hk2017_candidates(&kp.dual_vertices_f64, &transition_is_allowed)
        .expect("pruned simplex candidates");
    let sigma = orbits[0].sigma.clone();
    let orbit = solve_orbit_sigma_saddle_point(&kp.dual_vertices_f64, &sigma)
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

#[test]
fn automatic_dispatch_returns_exact_sparse_product_certificate() {
    let fixture = known_polytopes::lagrangian_triangle_product();
    let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
        .expect("known product must validate");
    assert!(input.is_structural_product());

    let Capacity4d::Product(result) = input.capacity().expect("product capacity") else {
        panic!("structural product must use the product route");
    };
    assert!(!result.winners().is_empty());
    assert!(result
        .winners()
        .iter()
        .all(|winner| winner.sigma().len() <= 6));
    let exact_as_f64 = result
        .capacity_exact()
        .to_f64()
        .expect("fixture capacity fits binary64");
    assert!(result.bounds().lower() <= exact_as_f64);
    assert!(exact_as_f64 <= result.bounds().upper());
}

#[test]
fn explicit_product_route_rejects_a_valid_non_product() {
    let fixture = known_polytopes::simplex();
    let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
        .expect("simplex must validate");
    assert!(!input.is_structural_product());
    assert_eq!(
        input.product_capacity(),
        Err(CapacityError4d::ProductRouteRequiresStructuralProduct)
    );
}

#[test]
fn automatic_dispatch_returns_bounds_for_a_general_polytope() {
    let fixture = known_polytopes::simplex();
    let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
        .expect("simplex must validate");
    let Capacity4d::General(result) = input.capacity().expect("general capacity") else {
        panic!("simplex must use the general route");
    };
    assert!(result.bounds().lower().is_finite());
    assert!(result.bounds().lower() > 0.0);
    assert!(result.bounds().lower() <= result.bounds().upper());
}

#[test]
fn validation_soft_errors_outside_the_coordinate_contract() {
    let mut duals = known_polytopes::simplex().dual_vertices_f64.clone();
    duals[0] = Vector4::new(1e4, 0.0, 0.0, 0.0);
    assert!(matches!(
        CapacityInput4d::try_from_dual_vertices(&duals),
        Err(CapacityInputError::DualNormOutOfRange { facet: 0 })
    ));
}
