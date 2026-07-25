use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays,
    selected_route::{
        general::solve_selected_general, product::solve_product_closure_capacity_hybrid,
    },
    try_exact_binary64_transition_matrix_assuming_origin_interior,
};
use symplectic::{
    algorithms::{capacity_4d::CapacityInput4d, hk2017::SimpleDirectedCyclesCanonical},
    known_polytopes,
};

#[test]
fn readable_and_production_general_bounds_match() {
    for fixture in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let exact = exact_binary64_dual_vertex_arrays(&fixture.dual_vertices_f64);
        let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact)
            .expect("known fixture transition graph");
        let words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
        let readable = solve_selected_general(&fixture.dual_vertices_f64, words)
            .expect("readable selected capacity");

        let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
            .expect("known fixture validation");
        let production = input
            .general_capacity()
            .expect("production general capacity");
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
        let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
            .expect("known fixture validation");
        let production = input
            .product_capacity()
            .expect("production product capacity");

        assert_eq!(readable.capacity_exact, *production.capacity_exact());
        let production = input
            .product_qp_minimizers()
            .expect("production product minimizers");
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
