//! Profiling harness: runs the full systolic ratio pipeline for one polytope.
//!
//! Usage: cargo run --release --bin benchmark_profile [F] [iterations]
//!   F: facet count (default 9)
//!   iterations: repeat count for profiler sampling (default 10)
//!
//! Designed for use with `cargo flamegraph` or `perf record`.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::volume::volume;
use symplectic::random::generate_random_polytopes;

// Same seed and height range as crates/exp-algorithm-comparison/benchmark/run.rs for consistency.
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(9);
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    eprintln!("Profiling: F={f}, {iterations} iterations");

    // Generate dual vertices once (not profiled).
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let polytopes = generate_random_polytopes(1, f, H_MIN, H_MAX, &mut rng);
    let dual_vertices: Vec<Vector4<f64>> = polytopes[0].dual_vertices_f64().to_vec();

    for i in 0..iterations {
        // Phase 1: Construction (rational vertex enum, incidence, adjacency, omega signs)
        let p = symplectic::geom::polytope::Polytope4D::from_f64(
            dual_vertices.clone(),
        )
        .expect("construction failed");

        // Phase 2: Capacity (enumeration, pruning, KKT solve, accumulation)
        let cap_result = ehz_capacity(&p).expect("capacity failed");

        // Phase 3: Volume (qhull subprocess)
        let vol = volume(&p).expect("volume failed");

        // Phase 4: Systolic ratio
        let sys = cap_result.result.capacity.powi(2) / (2.0 * vol);

        if i == 0 {
            eprintln!(
                "  F={f}: capacity={:.6}, volume={:.6}, sys={:.6}",
                cap_result.result.capacity, vol, sys
            );
        }
    }
    eprintln!("Done.");
}
