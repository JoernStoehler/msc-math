//! Criterion benchmarks for the systolic ratio pipeline.
//!
//! Benchmarks each phase of the pipeline at multiple facet counts
//! to produce a phase breakdown and enable regression detection.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence, polar_vertices_exact_rational,
    PolarVerticesExact,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::Zero;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
use symplectic::random::generate_random_dual_vertices;
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

struct BenchGeometry {
    dual_vertices: Vec<Vector4<f64>>,
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: DMatrix<bool>,
    omega_signs: DMatrix<i8>,
}

fn rational_array_to_vector(values: &[BigRational; 4]) -> Vector4<BigRational> {
    Vector4::new(
        values[0].clone(),
        values[1].clone(),
        values[2].clone(),
        values[3].clone(),
    )
}

fn construct_flat_geometry(dual_vertices: Vec<Vector4<f64>>) -> BenchGeometry {
    let dual_vertices_exact: Vec<[BigRational; 4]> = dual_vertices
        .iter()
        .map(|vertex| std::array::from_fn(|coordinate| f64_to_rational(vertex[coordinate])))
        .collect();
    let dual_vertices_exact_vectors: Vec<Vector4<BigRational>> = dual_vertices_exact
        .iter()
        .map(rational_array_to_vector)
        .collect();

    let PolarVerticesExact {
        vertex_facet_incidence,
        ..
    } = polar_vertices_exact_rational(&dual_vertices_exact_vectors);
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&dual_vertices_exact_vectors);

    BenchGeometry {
        dual_vertices,
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
    }
}

/// Pre-generate raw normals/heights for each facet count (for construction benchmarks).
fn raw_inputs(f: usize) -> (Vec<Vector4<f64>>, Vec<f64>) {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let dual_vertices_sets = generate_random_dual_vertices(1, f, H_MIN, H_MAX, &mut rng);
    let dual_vertices = &dual_vertices_sets[0];
    let normals = dual_vertices.iter().map(|a| a / a.norm()).collect();
    let heights = dual_vertices.iter().map(|a| 1.0 / a.norm()).collect();
    (normals, heights)
}

/// Pre-construct flat geometry for capacity/volume benchmarks.
fn prebuilt_geometry(f: usize) -> BenchGeometry {
    let (normals, heights) = raw_inputs(f);
    construct_flat_geometry(
        normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
}

fn capacity_pruned_hk2017(geometry: &BenchGeometry) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&geometry.dual_vertices, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn capacity_pruned_hk2017_certified(
    geometry: &BenchGeometry,
    action_gap_exact: BigRational,
    mode: CertifiedOrbitSetMode,
) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&geometry.dual_vertices, &transition_is_allowed)?;
    aggregate_certified_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        action_gap_exact,
        mode,
    )
}

/// Find a valid permutation for single-KKT benchmarks.
fn find_valid_permutation(geometry: &BenchGeometry) -> Vec<usize> {
    let f = geometry.dual_vertices.len();
    let dual_vertices = &geometry.dual_vertices;
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
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
                construct_flat_geometry(
                    normals
                        .iter()
                        .zip(heights.iter())
                        .map(|(n, &h)| n / h)
                        .collect(),
                )
            });
        });
    }
    group.finish();
}

fn bench_transition_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_matrix");
    for &f in FACET_COUNTS {
        let geometry = prebuilt_geometry(f);
        let facet_intersection_is_nonempty = &geometry.facet_intersection_is_nonempty;
        let omega_signs = &geometry.omega_signs;
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
        let geometry = prebuilt_geometry(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| capacity_pruned_hk2017(&geometry));
        });
    }
    group.finish();
}

fn bench_capacity_certified_minimizers(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity_certified_minimizers");
    group.sample_size(10);
    for &f in FACET_COUNTS {
        let geometry = prebuilt_geometry(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| {
                capacity_pruned_hk2017_certified(
                    &geometry,
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
        let geometry = prebuilt_geometry(f);
        let dual_vertices = &geometry.dual_vertices;
        let perm = find_valid_permutation(&geometry);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| solve_kkt_for_dual_vertices(dual_vertices, &perm));
        });
    }
    group.finish();
}

fn bench_pruning_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("pruning_check");
    for &f in FACET_COUNTS {
        let geometry = prebuilt_geometry(f);
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &geometry.facet_intersection_is_nonempty,
            &geometry.omega_signs,
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
