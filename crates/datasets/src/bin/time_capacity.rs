use datasets::known_polytopes::*;
use hk2017::ehz_capacity_pruned;
use std::time::Instant;

fn main() {
    let polytopes = vec![
        ("simplex", simplex().polytope, 5),
        ("triangle_product", triangle_product().polytope, 6),
        ("symplectic_tri_sq", symplectic_triangle_square().polytope, 7),
        ("hypercube", hypercube().polytope, 8),
        ("hko_pentagon", hko_pentagon().polytope, 10),
        ("crosspolytope", crosspolytope().polytope, 16),
    ];

    println!("polytope,facets,iterations,total_ms,ms_per_run,capacity");

    for (name, p, facets) in &polytopes {
        let iterations = if *facets <= 8 { 100 } else if *facets <= 10 { 10 } else { 3 };

        let start = Instant::now();
        let mut capacity = 0.0;
        for _ in 0..iterations {
            if let Some(result) = ehz_capacity_pruned(p) {
                capacity = result.capacity;
            }
        }
        let elapsed = start.elapsed();

        let total_ms = elapsed.as_secs_f64() * 1000.0;
        let ms_per_run = total_ms / (iterations as f64);

        println!("{},{},{},{:.3},{:.3},{:.6}", name, facets, iterations, total_ms, ms_per_run, capacity);
    }
}
