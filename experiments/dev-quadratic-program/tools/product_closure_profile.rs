use nalgebra::Vector4;
use std::hint::black_box;
use std::time::{Duration, Instant};
use symplectic::algorithms::capacity_4d::{
    check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
    check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, product_capacity,
    product_qp_minimizers, PolytopeGeometry4d,
};
use symplectic::geom::known_polytopes;

const ROUNDS: usize = 21;

fn checked_geometry(dual_vertices: &[Vector4<f64>]) -> PolytopeGeometry4d {
    check_facet_count(dual_vertices.len()).expect("capacity facet-count bound");
    check_finite_dual_vertices(dual_vertices).expect("finite dual vertices");
    check_dual_vertex_norm_bounds(dual_vertices).expect("capacity dual-vertex norm bounds");
    let geometry =
        exact_binary64_polytope_geometry(dual_vertices).expect("exact polytope geometry");
    check_primal_vertex_norm_bounds(&geometry).expect("capacity primal-vertex norm bounds");
    geometry
}

fn median(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn main() {
    println!("algorithm.id=product_closure_vertex");
    println!("algorithm.arithmetic=outward_f64_with_exact_contender_resolution");
    println!("algorithm.kkt=false");
    println!("algorithm.production=true");
    for fixture in [
        known_polytopes::lagrangian_triangle_product(),
        known_polytopes::lagrangian_triangle_square(),
        known_polytopes::hypercube(),
    ] {
        let geometry = checked_geometry(&fixture.dual_vertices_f64);
        black_box(product_capacity(&geometry).expect("product capacity"));
        black_box(product_qp_minimizers(&geometry).expect("product minimizers"));

        let capacity_samples = (0..ROUNDS)
            .map(|_| {
                let started = Instant::now();
                black_box(product_capacity(&geometry).expect("product capacity"));
                started.elapsed()
            })
            .collect();
        let minimizer_samples = (0..ROUNDS)
            .map(|_| {
                let started = Instant::now();
                black_box(product_qp_minimizers(&geometry).expect("product minimizers"));
                started.elapsed()
            })
            .collect();
        let minimizers = product_qp_minimizers(&geometry).expect("product minimizers");

        println!("profile.source_id={}", fixture.name);
        println!("profile.facets={}", fixture.dual_vertices_f64.len());
        println!("profile.rounds={ROUNDS}");
        println!(
            "profile.capacity_median_ms={:.6}",
            median(capacity_samples).as_secs_f64() * 1e3
        );
        println!(
            "profile.minimizers_median_ms={:.6}",
            median(minimizer_samples).as_secs_f64() * 1e3
        );
        println!("profile.minimizer_count={}", minimizers.candidates().len());
    }
}
