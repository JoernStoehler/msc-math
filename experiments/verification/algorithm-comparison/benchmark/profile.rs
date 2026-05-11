//! Profiling harness: runs the full systolic ratio pipeline for one polytope.
//!
//! Usage: cargo run -p dev-algorithm-comparison --release --bin cmp-benchmark-profile [F] [iterations]
//!   F: facet count (default 9)
//!   iterations: repeat count for profiler sampling (default 10)
//!
//! Designed for use with `cargo flamegraph` or `perf record`.
//!
//! The explicit routing is intentional: this profiling harness targets the
//! pruned HK2017 path directly rather than the crate-level `ehz_capacity`
//! entrypoint.
//!
//! Input Artifacts: None (generates its profiling fixture internally).
//! Output Artifacts: None (profiling output is handled by the external profiler).

use euclidean_polytopes::sample_random_dual_vertices_f64;
use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSearchResult,
};

#[path = "../flat_polytope.rs"]
mod flat_polytope;

use flat_polytope::FlatPolytopeCache;

// Same seed and height range as experiments/verification/algorithm-comparison/benchmark/main.rs for consistency.
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

fn capacity_pruned_hk2017(
    polytope: &FlatPolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(9);
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    eprintln!("Profiling: F={f}, {iterations} iterations");

    // Generate dual vertices once (not profiled).
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let dual_vertices: Vec<Vector4<f64>> =
        sample_random_dual_vertices_f64(f, H_MIN, H_MAX, &mut rng);

    for i in 0..iterations {
        // Phase 1: Construction (rational vertex enum, incidence, adjacency, omega signs)
        let p = FlatPolytopeCache::from_f64_dual_vertices(dual_vertices.clone())
            .expect("construction failed");

        // Phase 2: Capacity (enumeration, pruning, KKT solve, accumulation)
        let cap_result = capacity_pruned_hk2017(&p).expect("capacity failed");

        // Phase 3: Volume (pure-Rust origin-star triangulation)
        let vol = euclidean_volume_f64(&p.vertices, &p.vertex_facet_incidence);

        // Phase 4: Systolic ratio
        let sys = cap_result.capacity().powi(2) / (2.0 * vol);

        if i == 0 {
            eprintln!(
                "  F={f}: capacity={:.6}, volume={:.6}, sys={:.6}",
                cap_result.capacity(),
                vol,
                sys
            );
        }
    }
    eprintln!("Done.");
}
