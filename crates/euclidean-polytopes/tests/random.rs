use euclidean_polytopes::sample_random_dual_vertices_f64;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn random_seeded_rng_produces_deterministic_dual_vertices() {
    let mut rng1 = ChaCha8Rng::seed_from_u64(42);
    let mut rng2 = ChaCha8Rng::seed_from_u64(42);

    let sample1 = sample_random_dual_vertices_f64(8, 0.5, 2.0, &mut rng1);
    let sample2 = sample_random_dual_vertices_f64(8, 0.5, 2.0, &mut rng2);

    assert_eq!(sample1, sample2);
}

#[test]
fn random_sample_has_requested_length_and_finite_nonzero_vectors() {
    let mut rng = ChaCha8Rng::seed_from_u64(7);
    let dual_vertices = sample_random_dual_vertices_f64(11, 0.5, 2.0, &mut rng);

    assert_eq!(dual_vertices.len(), 11);
    for dual_vertex in dual_vertices {
        assert!(dual_vertex.iter().all(|coordinate| coordinate.is_finite()));
        assert!(dual_vertex.norm() > 0.0);
    }
}

#[test]
fn random_implied_heights_stay_in_requested_half_open_range() {
    let mut rng = ChaCha8Rng::seed_from_u64(99);
    let h_min = 0.25;
    let h_max = 3.5;

    for dual_vertex in sample_random_dual_vertices_f64(64, h_min, h_max, &mut rng) {
        let implied_height = 1.0 / dual_vertex.norm();
        assert!(
            h_min <= implied_height && implied_height < h_max,
            "implied height {implied_height} outside [{h_min}, {h_max})"
        );
    }
}

#[test]
#[should_panic(expected = "facet_count must be at least 5")]
fn random_invalid_facet_count_panics() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let _ = sample_random_dual_vertices_f64(4, 0.5, 2.0, &mut rng);
}

#[test]
fn random_invalid_height_ranges_panic() {
    for (h_min, h_max) in [
        (0.0, 1.0),
        (-1.0, 1.0),
        (1.0, 1.0),
        (2.0, 1.0),
        (f64::NAN, 1.0),
        (1.0, f64::INFINITY),
    ] {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = sample_random_dual_vertices_f64(5, h_min, h_max, &mut rng);
        }));
        assert!(
            result.is_err(),
            "expected panic for height range h_min={h_min}, h_max={h_max}"
        );
    }
}

#[test]
fn random_cheap_seed_and_range_sweep_preserves_sampler_contract() {
    for facet_count in 5..=9 {
        for seed in 0..8 {
            let h_min = 0.1 + 0.05 * seed as f64;
            let h_max = h_min + 1.0 + 0.1 * facet_count as f64;
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let dual_vertices =
                sample_random_dual_vertices_f64(facet_count, h_min, h_max, &mut rng);

            assert_eq!(dual_vertices.len(), facet_count);
            for dual_vertex in dual_vertices {
                assert!(dual_vertex.iter().all(|coordinate| coordinate.is_finite()));
                let implied_height = 1.0 / dual_vertex.norm();
                assert!(h_min <= implied_height && implied_height < h_max);
            }
        }
    }
}
