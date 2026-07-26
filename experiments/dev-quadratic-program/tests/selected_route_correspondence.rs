use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays,
    selected_route::{
        general::solve_selected_general, product::solve_product_closure_capacity_hybrid,
    },
    try_exact_binary64_transition_matrix_assuming_origin_interior,
};
use symplectic::{
    algorithms::{
        capacity_4d::{
            check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
            check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, general_capacity,
            product_capacity, product_qp_minimizers,
        },
        hk2017::SimpleDirectedCyclesCanonical,
    },
    known_polytopes,
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
