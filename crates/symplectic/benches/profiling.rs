//! Criterion benchmarks for the systolic ratio pipeline.
//!
//! Benchmarks each phase of the pipeline at multiple facet counts
//! to produce a phase breakdown and enable regression detection.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::Zero;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
use symplectic::random::generate_random_polytopes;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    solve_pruned_hk2017_candidates, CertifiedOrbitSearchResult, CertifiedOrbitSetMode,
    OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

// Same seed and height range as
// experiments/verification/algorithm-comparison/benchmark/main.rs for consistency.
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

/// F=5 is the minimum for a bounded 4D polytope; F=11 is the highest where
/// capacity completes in <200ms (F=12 takes ~2s, too slow for criterion warmup).
const FACET_COUNTS: &[usize] = &[5, 6, 7, 8, 9, 10, 11];

/// Pre-generate raw normals/heights for each facet count (for construction benchmarks).
fn raw_inputs(f: usize) -> (Vec<Vector4<f64>>, Vec<f64>) {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let polytopes = generate_random_polytopes(1, f, H_MIN, H_MAX, &mut rng);
    let duals = polytopes[0].dual_vertices_f64();
    let normals = duals.iter().map(|a| a / a.norm()).collect();
    let heights = duals.iter().map(|a| 1.0 / a.norm()).collect();
    (normals, heights)
}

/// Pre-construct a polytope for capacity/volume benchmarks.
fn prebuilt_polytope(f: usize) -> Polytope4D {
    let (normals, heights) = raw_inputs(f);
    Polytope4D::from_f64(
        normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .expect("construction failed")
}

fn capacity_pruned_hk2017(polytope: &Polytope4D) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(polytope.dual_vertices_f64(), &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        polytope.dual_vertices(),
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn capacity_pruned_hk2017_certified(
    polytope: &Polytope4D,
    action_gap_exact: BigRational,
    mode: CertifiedOrbitSetMode,
) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(polytope.dual_vertices_f64(), &transition_is_allowed)?;
    aggregate_certified_orbits_with_dual_vertices_exact(
        polytope.dual_vertices(),
        orbits,
        iterations,
        action_gap_exact,
        mode,
    )
}

/// Find a valid permutation for single-KKT benchmarks.
fn find_valid_permutation(polytope: &Polytope4D) -> Vec<usize> {
    let f = polytope.facet_count();
    let dual_vertices = polytope.dual_vertices_f64();
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    );
    let mut found: Option<Vec<usize>> = None;
    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if found.is_some() {
                    return;
                }
                if is_feasible_cycle(perm, &transition_is_allowed)
                    && matches!(
                        solve_kkt_for_dual_vertices(dual_vertices, perm),
                        KktOutcome::Feasible(_)
                    )
                {
                    found = Some(perm.to_vec());
                }
            });
            if let Some(found) = found {
                return found;
            }
        }
    }
    panic!("no valid permutation found for F={f}");
}

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction");
    for &f in FACET_COUNTS {
        let (normals, heights) = raw_inputs(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| {
                Polytope4D::from_f64(
                    normals
                        .iter()
                        .zip(heights.iter())
                        .map(|(n, &h)| n / h)
                        .collect(),
                )
                .expect("construction failed")
            });
        });
    }
    group.finish();
}

fn bench_transition_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_matrix");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        let facet_intersection_is_nonempty = polytope.facet_intersection_is_nonempty();
        let omega_signs = polytope.omega_signs();
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| {
                build_transition_matrix_from_facet_intersections_and_omega(
                    facet_intersection_is_nonempty,
                    omega_signs,
                )
            });
        });
    }
    group.finish();
}

fn bench_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity");
    // F=11 takes ~170ms per call; limit measurement time.
    group.sample_size(10);
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| capacity_pruned_hk2017(&polytope));
        });
    }
    group.finish();
}

fn bench_capacity_certified_minimizers(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity_certified_minimizers");
    group.sample_size(10);
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| {
                capacity_pruned_hk2017_certified(
                    &polytope,
                    BigRational::zero(),
                    CertifiedOrbitSetMode::MinimizersOnly,
                )
            });
        });
    }
    group.finish();
}

fn bench_kkt_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("kkt_single");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        let dual_vertices = polytope.dual_vertices_f64();
        let perm = find_valid_permutation(&polytope);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| solve_kkt_for_dual_vertices(dual_vertices, &perm));
        });
    }
    group.finish();
}

fn bench_pruning_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("pruning_check");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            polytope.facet_intersection_is_nonempty(),
            polytope.omega_signs(),
        );
        // Use a size-3 permutation for the pruning check.
        let perm: Vec<usize> = (0..std::cmp::min(3, f)).collect();
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| is_feasible_cycle(&perm, &transition_is_allowed));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_construction,
    bench_transition_matrix,
    bench_capacity,
    bench_capacity_certified_minimizers,
    bench_kkt_single,
    bench_pruning_check,
);
criterion_main!(benches);
