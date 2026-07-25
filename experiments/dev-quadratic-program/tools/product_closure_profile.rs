use std::hint::black_box;
use std::time::{Duration, Instant};
use symplectic::algorithms::capacity_4d::CapacityInput4d;
use symplectic::geom::known_polytopes;

const ROUNDS: usize = 21;

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
        let input = CapacityInput4d::try_from_dual_vertices(&fixture.dual_vertices_f64)
            .expect("known product validates");
        black_box(input.product_capacity().expect("product capacity"));
        black_box(input.product_qp_minimizers().expect("product minimizers"));

        let capacity_samples = (0..ROUNDS)
            .map(|_| {
                let started = Instant::now();
                black_box(input.product_capacity().expect("product capacity"));
                started.elapsed()
            })
            .collect();
        let minimizer_samples = (0..ROUNDS)
            .map(|_| {
                let started = Instant::now();
                black_box(input.product_qp_minimizers().expect("product minimizers"));
                started.elapsed()
            })
            .collect();
        let minimizers = input.product_qp_minimizers().expect("product minimizers");

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
