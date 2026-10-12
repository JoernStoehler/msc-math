use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays, generated_f64_cases_with_source_filter,
    selected_route::{
        general::solve_selected_general, product::solve_product_closure_capacity_hybrid,
    },
    solve_exact_capacity_for_transition_pruned_sigmas,
    try_exact_binary64_transition_matrix_assuming_origin_interior,
};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact,
    algorithms::{
        capacity_4d::{
            check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
            check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, general_capacity,
            general_qp_action_window, product_capacity, product_qp_minimizers,
        },
        hk2017::SimpleDirectedCyclesCanonical,
    },
    derivatives::{capacity_subgradients_a, capacity_subgradients_a_from_exact_orbits},
    known_polytopes, solve_pruned_hk2017_candidates, OrbitAdmissibility, OrbitGuaranteeMode,
};

fn checked_geometry(
    dual_vertices: &[nalgebra::Vector4<f64>],
) -> symplectic::algorithms::capacity_4d::PolytopeGeometry4d {
    check_facet_count(dual_vertices.len()).expect("capacity facet-count bound");
    check_finite_dual_vertices(dual_vertices).expect("finite dual vertices");
    check_dual_vertex_norm_bounds(dual_vertices).expect("capacity dual-vertex norm bounds");
    let geometry =
        exact_binary64_polytope_geometry(dual_vertices).expect("exact polytope geometry");
    check_primal_vertex_norm_bounds(&geometry).expect("capacity primal-vertex norm bounds");
    geometry
}

#[test]
fn readable_and_production_general_bounds_match() {
    for fixture in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let exact = exact_binary64_dual_vertex_arrays(&fixture.dual_vertices_f64);
        let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact)
            .expect("known fixture transition graph");
        let words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
        let readable = solve_selected_general(&fixture.dual_vertices_f64, words)
            .expect("readable selected capacity");

        let geometry = checked_geometry(&fixture.dual_vertices_f64);
        let production = general_capacity(&geometry).expect("production general capacity");
        assert_eq!(readable.0, production.bounds().lower());
        assert_eq!(readable.1, production.bounds().upper());
    }
}

#[test]
fn production_general_action_window_matches_complete_exact_control() {
    let case = generated_f64_cases_with_source_filter(
        1,
        99540836,
        &["seed99540836:F5:sample0:attempt5000000008".to_string()],
    )
    .pop()
    .expect("retained generated F5 case");
    let geometry = checked_geometry(&case.dual_vertices);
    let multiple = BigRational::new(101.into(), 100.into());
    let production =
        general_qp_action_window(&geometry, multiple).expect("production exact action window");

    let exact_duals = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
    let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
        .expect("exact transition graph");
    let gap = production.capacity_exact().clone() * BigRational::new(1.into(), 100.into());
    let reference =
        solve_exact_capacity_for_transition_pruned_sigmas(&exact_duals, &transition, gap)
            .expect("complete exact action window");

    assert_eq!(production.capacity_exact(), &reference.capacity_exact);
    let production_rows = production
        .witnesses()
        .iter()
        .map(|witness| (witness.sigma.clone(), witness.action()))
        .collect::<Vec<_>>();
    let reference_rows = reference
        .orbits
        .iter()
        .map(|candidate| (candidate.sigma.clone(), candidate.action_exact.clone()))
        .collect::<Vec<_>>();
    assert_eq!(production_rows, reference_rows);
}

#[test]
fn exact_window_derivatives_match_the_retained_f64_route_on_f10() {
    let case = generated_f64_cases_with_source_filter(
        1,
        99540836,
        &["seed99540836:F10:sample0:attempt10000000000".to_string()],
    )
    .into_iter()
    .next()
    .expect("retained generated F10 case");
    let geometry = checked_geometry(&case.dual_vertices);
    let production =
        general_qp_action_window(&geometry, BigRational::new(1001.into(), 1000.into()))
            .expect("production exact action window");
    let production_gradients =
        capacity_subgradients_a_from_exact_orbits(&case.dual_vertices, production.witnesses())
            .expect("exact KKT data fits binary64");

    let transition = symplectic::capacity_4d::capacity_transition_graph(&geometry);
    let (orbits, iterations) = solve_pruned_hk2017_candidates(&case.dual_vertices, &transition)
        .expect("retained f64 candidates");
    let capacity = production
        .capacity_exact()
        .to_f64()
        .expect("capacity fits binary64");
    let retained = aggregate_orbits_with_dual_vertices_exact(
        &exact_binary64_dual_vertex_arrays(&case.dual_vertices),
        orbits,
        iterations,
        capacity * 0.01,
        OrbitGuaranteeMode::AllSafe,
    )
    .expect("retained exact fallback");
    let near_active = retained
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            ) && orbit.action <= retained.min_action * 1.001
        })
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        production
            .witnesses()
            .iter()
            .map(|witness| &witness.sigma)
            .collect::<Vec<_>>(),
        near_active
            .iter()
            .map(|orbit| &orbit.sigma)
            .collect::<Vec<_>>()
    );
    let retained_gradients = capacity_subgradients_a(&case.dual_vertices, &near_active)
        .expect("retained KKT payloads have multipliers");
    for (production, retained) in production_gradients.iter().zip(retained_gradients) {
        for (production, retained) in production.iter().zip(retained) {
            assert!((production - retained).norm() < 1e-10);
        }
    }
}

#[test]
fn readable_and_production_product_certificates_match() {
    for fixture in [
        known_polytopes::lagrangian_triangle_product(),
        known_polytopes::lagrangian_triangle_square(),
        known_polytopes::hko_pentagon(),
    ] {
        let readable = solve_product_closure_capacity_hybrid(&fixture.dual_vertices_f64)
            .expect("readable selected product capacity");
        let geometry = checked_geometry(&fixture.dual_vertices_f64);
        let production = product_capacity(&geometry).expect("production product capacity");

        assert_eq!(readable.capacity_exact, *production.capacity_exact());
        let production = product_qp_minimizers(&geometry).expect("production product minimizers");
        let readable_winners = readable
            .winners
            .iter()
            .map(|winner| winner.sigma.clone())
            .collect::<Vec<_>>();
        let production_winners = production
            .candidates()
            .iter()
            .map(|winner| winner.sigma().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(readable_winners, production_winners);
    }
}
