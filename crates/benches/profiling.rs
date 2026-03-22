//! Criterion benchmarks for the systolic ratio pipeline.
//!
//! Benchmarks each phase of the pipeline at multiple facet counts
//! to produce a phase breakdown and enable regression detection.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::{combinations, ehz_capacity};
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;
use symplectic::random::generate_random_polytopes;

// Same seed and height range as experiments/benchmark/run.rs for consistency.
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
    let normals = polytopes[0].normals_f64().to_vec();
    let heights = polytopes[0].heights_f64().to_vec();
    (normals, heights)
}

/// Pre-construct a polytope for capacity/volume benchmarks.
fn prebuilt_polytope(f: usize) -> Polytope4D {
    let (normals, heights) = raw_inputs(f);
    Polytope4D::from_normals_and_heights(normals, heights).expect("construction failed")
}

/// Find a valid permutation for single-KKT benchmarks.
fn find_valid_permutation(polytope: &Polytope4D) -> Vec<usize> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);
    let mut found: Option<Vec<usize>> = None;
    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if found.is_some() {
                    return;
                }
                if is_feasible_cycle(perm, &adj) {
                    if solve_kkt_for(polytope, perm).is_some() {
                        found = Some(perm.to_vec());
                    }
                }
            });
            if found.is_some() {
                return found.unwrap();
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
                Polytope4D::from_normals_and_heights(normals.clone(), heights.clone())
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
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| build_transition_matrix(&polytope));
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
            b.iter(|| ehz_capacity(&polytope));
        });
    }
    group.finish();
}

fn bench_kkt_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("kkt_single");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        let perm = find_valid_permutation(&polytope);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| solve_kkt_for(&polytope, &perm));
        });
    }
    group.finish();
}

fn bench_pruning_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("pruning_check");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        let adj = build_transition_matrix(&polytope);
        // Use a size-3 permutation for the pruning check.
        let perm: Vec<usize> = (0..std::cmp::min(3, f)).collect();
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| is_feasible_cycle(&perm, &adj));
        });
    }
    group.finish();
}

fn bench_volume(c: &mut Criterion) {
    let mut group = c.benchmark_group("volume");
    for &f in FACET_COUNTS {
        let polytope = prebuilt_polytope(f);
        group.bench_with_input(BenchmarkId::from_parameter(f), &f, |b, _| {
            b.iter(|| volume(&polytope).expect("volume failed"));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_construction,
    bench_transition_matrix,
    bench_capacity,
    bench_kkt_single,
    bench_pruning_check,
    bench_volume,
);
criterion_main!(benches);
