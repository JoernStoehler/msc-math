use nalgebra::{DMatrix, Vector4};
use num_traits::{Signed, ToPrimitive};
use symplectic::algorithms::capacity_4d::{
    capacity, capacity_from_dual_vertices, check_dual_vertex_norm_bounds, check_facet_count,
    check_finite_dual_vertices, check_primal_vertex_norm_bounds, classify_lagrangian_product,
    exact_binary64_polytope_geometry, general_capacity, general_qp_minimizers, product_capacity,
    qp_minimizers, solve_sigma_exact, Capacity4d, CapacityError4d, CapacityFromDualVerticesError4d,
    CapacityInputBoundsError4d, ExactSigmaInputError4d, PolytopeGeometry4d,
    PolytopeGeometryError4d, QpCandidateFamily4d,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, known_polytopes, solve_orbit_sigma_saddle_point,
    solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates, OrbitGuaranteeMode,
};

fn checked_geometry(dual_vertices: &[Vector4<f64>]) -> PolytopeGeometry4d {
    check_facet_count(dual_vertices.len()).expect("capacity facet-count bound");
    check_finite_dual_vertices(dual_vertices).expect("finite dual vertices");
    check_dual_vertex_norm_bounds(dual_vertices).expect("capacity dual-vertex norm bounds");
    let geometry =
        exact_binary64_polytope_geometry(dual_vertices).expect("exact polytope geometry");
    check_primal_vertex_norm_bounds(&geometry).expect("capacity primal-vertex norm bounds");
    geometry
}

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
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    assert!(classify_lagrangian_product(&geometry).is_some());

    let Capacity4d::Product(result) = capacity(&geometry).expect("product capacity") else {
        panic!("structural product must use the product route");
    };
    let minimizers = qp_minimizers(&geometry).expect("product minimizers");
    assert_eq!(
        minimizers.family(),
        QpCandidateFamily4d::ProductClosureVertex
    );
    assert!(!minimizers.candidates().is_empty());
    assert!(minimizers
        .candidates()
        .iter()
        .all(|candidate| candidate.sigma().len() <= 6));
    for candidate in minimizers.candidates() {
        let action_f64 = candidate
            .action_exact()
            .to_f64()
            .expect("fixture action fits binary64");
        assert!(minimizers.bounds().lower() <= action_f64);
        assert!(action_f64 <= minimizers.bounds().upper());
        let orbit = solve_sigma_exact(&geometry, candidate.sigma())
            .expect("returned product sigma is valid")
            .expect("returned product candidate has a positive exact KKT witness");
        assert_eq!(&orbit.action(), candidate.action_exact());
    }
    let exact_as_f64 = result
        .capacity_exact()
        .to_f64()
        .expect("fixture capacity fits binary64");
    assert!(result.bounds().lower() <= exact_as_f64);
    assert!(exact_as_f64 <= result.bounds().upper());
}

#[test]
fn degenerate_product_minimizers_have_consistent_exact_payloads() {
    let fixture = known_polytopes::hypercube();
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    let minimizers = qp_minimizers(&geometry).expect("hypercube minimizers");
    assert_eq!(
        minimizers.family(),
        QpCandidateFamily4d::ProductClosureVertex
    );
    assert!(minimizers.candidates().len() >= 2);
    for candidate in minimizers.candidates() {
        let orbit = solve_sigma_exact(&geometry, candidate.sigma())
            .expect("returned hypercube sigma is valid")
            .expect("returned hypercube candidate has a positive exact witness");
        assert_eq!(&orbit.action(), candidate.action_exact());
    }

    let general = general_qp_minimizers(&geometry).expect("forced general hypercube minimizers");
    assert_eq!(general.family(), QpCandidateFamily4d::GeneralHk);
    let mut product_sigmas = minimizers
        .candidates()
        .iter()
        .map(|candidate| candidate.sigma().to_vec())
        .collect::<Vec<_>>();
    let mut general_sigmas = general
        .candidates()
        .iter()
        .map(|candidate| candidate.sigma().to_vec())
        .collect::<Vec<_>>();
    product_sigmas.sort();
    general_sigmas.sort();
    assert_eq!(general_sigmas, product_sigmas);
}

#[test]
fn explicit_product_route_rejects_a_valid_non_product() {
    let fixture = known_polytopes::simplex();
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    assert!(classify_lagrangian_product(&geometry).is_none());
    assert_eq!(
        product_capacity(&geometry),
        Err(CapacityError4d::ProductRouteRequiresStructuralProduct)
    );
}

#[test]
fn automatic_product_dispatch_bypasses_the_forced_general_candidate_cap() {
    let octagon = [
        (1.0, 0.0),
        (0.75, 0.75),
        (0.0, 1.0),
        (-0.75, 0.75),
        (-1.0, 0.0),
        (-0.75, -0.75),
        (0.0, -1.0),
        (0.75, -0.75),
    ];
    let mut duals = octagon
        .iter()
        .map(|&(q1, q2)| Vector4::new(q1, q2, 0.0, 0.0))
        .collect::<Vec<_>>();
    duals.extend(
        octagon
            .iter()
            .map(|&(p1, p2)| Vector4::new(0.0, 0.0, p1, p2)),
    );

    let geometry = checked_geometry(&duals);
    assert!(matches!(capacity(&geometry), Ok(Capacity4d::Product(_))));
    assert_eq!(
        general_capacity(&geometry),
        Err(CapacityError4d::GeneralCandidateLimitExceeded {
            limit: symplectic::algorithms::capacity_4d::MAX_GENERAL_CANDIDATES,
        })
    );
}

#[test]
fn automatic_dispatch_returns_bounds_for_a_general_polytope() {
    let fixture = known_polytopes::simplex();
    let result =
        capacity_from_dual_vertices(&fixture.dual_vertices_f64).expect("one-shot simplex capacity");
    let Capacity4d::General(result) = result else {
        panic!("simplex must use the general route");
    };
    assert!(result.bounds().lower().is_finite());
    assert!(result.bounds().lower() > 0.0);
    assert!(result.bounds().lower() <= result.bounds().upper());
}

#[test]
fn general_minimizers_are_exact_and_support_on_demand_kkt_payloads() {
    let fixture = known_polytopes::simplex();
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    let minimizers = general_qp_minimizers(&geometry).expect("general simplex minimizers");
    assert_eq!(minimizers.family(), QpCandidateFamily4d::GeneralHk);
    assert!(!minimizers.candidates().is_empty());

    for candidate in minimizers.candidates() {
        let action_f64 = candidate
            .action_exact()
            .to_f64()
            .expect("fixture action fits binary64");
        assert!(minimizers.bounds().lower() <= action_f64);
        assert!(action_f64 <= minimizers.bounds().upper());
        let orbit = solve_sigma_exact(&geometry, candidate.sigma())
            .expect("returned sigma is valid")
            .expect("returned candidate has a positive exact KKT witness");
        assert_eq!(&orbit.action(), candidate.action_exact());
    }
}

#[test]
fn general_minimizers_match_complete_exact_simplex_enumeration() {
    let fixture = known_polytopes::simplex();
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    let observed = general_qp_minimizers(&geometry).expect("general simplex minimizers");

    let exact = fixture
        .dual_vertices_f64
        .iter()
        .map(|vertex| {
            [
                symplectic::geom::rational_arithmetic::f64_to_rational(vertex[0]),
                symplectic::geom::rational_arithmetic::f64_to_rational(vertex[1]),
                symplectic::geom::rational_arithmetic::f64_to_rational(vertex[2]),
                symplectic::geom::rational_arithmetic::f64_to_rational(vertex[3]),
            ]
        })
        .collect::<Vec<_>>();
    let transition =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        );
    let resolved = symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical::new(&transition)
        .filter_map(|sigma| {
            symplectic::kkt::rational_solver::solve_kkt_exact(&exact, &sigma)
                .filter(|result| result.q_exact.is_positive())
                .map(|result| (sigma, result.q_exact))
        })
        .collect::<Vec<_>>();
    let maximum_q = resolved
        .iter()
        .map(|(_, q)| q)
        .max()
        .expect("simplex has an exact positive candidate");
    let mut expected_sigmas = resolved
        .iter()
        .filter(|(_, q)| q == maximum_q)
        .map(|(sigma, _)| sigma.clone())
        .collect::<Vec<_>>();
    let mut observed_sigmas = observed
        .candidates()
        .iter()
        .map(|candidate| candidate.sigma().to_vec())
        .collect::<Vec<_>>();
    expected_sigmas.sort();
    observed_sigmas.sort();
    assert_eq!(observed_sigmas, expected_sigmas);
}

#[test]
fn exact_sigma_validation_is_soft() {
    let fixture = known_polytopes::simplex();
    let geometry = checked_geometry(&fixture.dual_vertices_f64);
    assert_eq!(
        solve_sigma_exact(&geometry, &[]),
        Err(ExactSigmaInputError4d::Empty)
    );
    assert_eq!(
        solve_sigma_exact(&geometry, &[0, 0]),
        Err(ExactSigmaInputError4d::RepeatedFacet { facet: 0 })
    );
    assert_eq!(
        solve_sigma_exact(&geometry, &[fixture.dual_vertices_f64.len()]),
        Err(ExactSigmaInputError4d::FacetOutOfRange {
            position: 0,
            facet: fixture.dual_vertices_f64.len(),
            facet_count: fixture.dual_vertices_f64.len(),
        })
    );
}

#[test]
fn validation_soft_errors_outside_the_coordinate_contract() {
    let mut nonfinite = known_polytopes::simplex().dual_vertices_f64.clone();
    nonfinite[0][2] = f64::NAN;
    assert_eq!(
        check_finite_dual_vertices(&nonfinite),
        Err(PolytopeGeometryError4d::NonFiniteCoordinate {
            facet: 0,
            coordinate: 2,
        })
    );
    assert!(matches!(
        capacity_from_dual_vertices(&nonfinite),
        Err(CapacityFromDualVerticesError4d::Geometry(
            PolytopeGeometryError4d::NonFiniteCoordinate {
                facet: 0,
                coordinate: 2,
            }
        ))
    ));

    let mut duals = known_polytopes::simplex().dual_vertices_f64.clone();
    duals[0] = Vector4::new(1e4, 0.0, 0.0, 0.0);
    assert!(matches!(
        check_dual_vertex_norm_bounds(&duals),
        Err(CapacityInputBoundsError4d::DualNormOutOfRange { facet: 0 })
    ));

    let crosspolytope = known_polytopes::crosspolytope();
    let geometry = checked_geometry(&crosspolytope.dual_vertices_f64);
    assert!(matches!(
        general_capacity(&geometry),
        Err(CapacityError4d::GeneralCandidateLimitExceeded { .. })
    ));
}
