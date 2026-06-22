use super::{
    build_tube_for_word_f64, cached_words_contain, capacity_f64, closed_tube_for_sigma_f64,
    counts_by_plus_depth, enumerate_transition_pruned_words, half_cache_depth, intersect_tubes_f64,
    is_simple_closable_word, primitive_tube_f64, split_closed_word_into_half_words,
    word_has_allowed_transitions, CapacityF64Error, F64TubeError, FlatTubeInput,
    DEFAULT_OMEGA_STABILITY_EPS,
};
use crate::algorithms::flow_graph::exact_tube::ExactFlatTubeInput;
use crate::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use crate::algorithms::test_helpers::pruned_capacity_for_fixture;
use crate::geom::known_polytopes;
use nalgebra::DMatrix;
use num_rational::BigRational;

fn complete_transition_matrix(facet_count: usize) -> DMatrix<bool> {
    DMatrix::from_fn(facet_count, facet_count, |i, j| i != j)
}

fn closed_raw_word(sigma: &[usize]) -> Vec<usize> {
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    word
}

#[test]
fn simple_closable_word_accepts_exactly_the_prefix_shapes_we_use() {
    assert!(is_simple_closable_word(&[0, 1, 2]));
    assert!(is_simple_closable_word(&[0, 1, 2, 3]));
    assert!(is_simple_closable_word(&[0, 1, 2, 0]));
    assert!(is_simple_closable_word(&[0, 1, 2, 0, 1]));

    assert!(!is_simple_closable_word(&[0]));
    assert!(!is_simple_closable_word(&[0, 1]));
    assert!(!is_simple_closable_word(&[0, 0, 1]));
    assert!(!is_simple_closable_word(&[0, 1, 2, 0, 3]));
    assert!(!is_simple_closable_word(&[0, 1, 2, 0, 3, 4]));
    assert!(!is_simple_closable_word(&[0, 1, 2, 1]));
}

#[test]
fn complete_graph_counts_include_closure_special_prefixes() {
    let transition = complete_transition_matrix(5);
    let words = enumerate_transition_pruned_words(&transition, 2);
    let counts = counts_by_plus_depth(&words, 2);

    assert_eq!(counts[1], 5 * 4 * 4);
    assert_eq!(counts[2], 5 * 4 * (3 * 3 + 1));
}

#[test]
fn half_cache_splits_every_transition_pruned_closed_word_on_complete_graphs() {
    for facet_count in 5..=8 {
        let transition = complete_transition_matrix(facet_count);
        let half_depth = half_cache_depth(facet_count);
        let cache = enumerate_transition_pruned_words(&transition, half_depth);

        let mut missing = Vec::new();
        for_each_sigma_pruned_by_transition(&transition, |sigma| {
            let closed = closed_raw_word(sigma);
            let Some((left, right)) = split_closed_word_into_half_words(&closed, half_depth) else {
                missing.push(closed);
                return;
            };
            if !cached_words_contain(&cache, &left) || !cached_words_contain(&cache, &right) {
                missing.push(closed);
            }
        });

        assert!(
            missing.is_empty(),
            "missing half-cache split for F={facet_count}: {missing:?}"
        );
    }
}

#[test]
fn half_cache_splits_every_transition_pruned_closed_word_on_sparse_graph() {
    let transition = DMatrix::from_row_slice(
        6,
        6,
        &[
            false, true, false, false, true, false, //
            false, false, true, false, false, true, //
            true, false, false, true, false, false, //
            false, true, false, false, true, false, //
            false, false, true, false, false, true, //
            true, false, false, true, false, false, //
        ],
    );
    let half_depth = half_cache_depth(6);
    let cache = enumerate_transition_pruned_words(&transition, half_depth);

    let mut checked = 0usize;
    for_each_sigma_pruned_by_transition(&transition, |sigma| {
        checked += 1;
        let closed = closed_raw_word(sigma);
        assert!(word_has_allowed_transitions(&closed, &transition));
        let (left, right) = split_closed_word_into_half_words(&closed, half_depth)
            .expect("transition-pruned closed word should split");
        assert!(cached_words_contain(&cache, &left), "left={left:?}");
        assert!(cached_words_contain(&cache, &right), "right={right:?}");
    });
    assert!(checked > 0);
}

#[test]
fn transition_pruned_words_never_use_forbidden_edges() {
    let transition = DMatrix::from_row_slice(
        4,
        4,
        &[
            false, true, false, false, //
            false, false, true, false, //
            true, false, false, true, //
            false, false, false, false, //
        ],
    );
    let words = enumerate_transition_pruned_words(&transition, half_cache_depth(4));
    assert!(!words.is_empty());
    for word in words {
        assert!(word_has_allowed_transitions(&word.facets, &transition));
    }
}

#[test]
fn input_precheck_rejects_geometric_zero_omega_transition() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );
    assert_eq!(
        input.validate_no_geometric_zero_omega_transitions(),
        Err(F64TubeError::UnsupportedZeroOmegaTransition)
    );
}

#[test]
fn input_precheck_accepts_zero_omega_when_two_face_is_empty() {
    let dual_vertices = vec![nalgebra::Vector4::x(), nalgebra::Vector4::y()];
    let facet_intersection_is_nonempty = DMatrix::from_row_slice(
        2,
        2,
        &[
            false, false, //
            false, false,
        ],
    );
    let omega_signs = DMatrix::from_row_slice(
        2,
        2,
        &[
            0, 0, //
            0, 0,
        ],
    );
    let input = FlatTubeInput::new(
        &dual_vertices,
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
    assert_eq!(input.validate_no_geometric_zero_omega_transitions(), Ok(()));
}

#[test]
fn input_precheck_rejects_geometric_small_f64_omega_transition() {
    let dual_vertices = vec![
        nalgebra::Vector4::new(1.0, 0.0, 0.0, 0.0),
        nalgebra::Vector4::new(0.0, 1.0, 1e-14, 0.0),
    ];
    let facet_intersection_is_nonempty = DMatrix::from_row_slice(
        2,
        2,
        &[
            false, true, //
            true, false,
        ],
    );
    let omega_signs = DMatrix::from_row_slice(
        2,
        2,
        &[
            0, 1, //
            -1, 0,
        ],
    );
    let input = FlatTubeInput::new(
        &dual_vertices,
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
    assert_eq!(
        input.validate_f64_omega_stability(DEFAULT_OMEGA_STABILITY_EPS),
        Err(F64TubeError::NumericallyUnstableOmegaTransition)
    );
}

#[test]
fn capacity_f64_rejects_when_no_positive_orbit_is_found() {
    let dual_vertices_f64 = vec![nalgebra::Vector4::x(), nalgebra::Vector4::y()];
    let dual_vertices_exact = vec![
        [
            BigRational::from_integer(1.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
        ],
        [
            BigRational::from_integer(0.into()),
            BigRational::from_integer(1.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
        ],
    ];
    let facet_intersection_is_nonempty = DMatrix::from_element(2, 2, false);
    let omega_signs = DMatrix::from_element(2, 2, 1);
    let input = FlatTubeInput::new(
        &dual_vertices_f64,
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
    let exact_input = ExactFlatTubeInput {
        dual_vertices: &dual_vertices_exact,
        facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
        omega_signs: &omega_signs,
    };

    assert_eq!(
        capacity_f64(&input, &exact_input, 0.0),
        Err(CapacityF64Error::NoPositiveOrbit)
    );
}

#[test]
fn primitive_tube_f64_has_consistent_redundant_maps_on_known_fixture() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );

    let mut checked = 0usize;
    for previous in 0..fixture.facet_count() {
        for current in 0..fixture.facet_count() {
            for next in 0..fixture.facet_count() {
                if previous == current || current == next {
                    continue;
                }
                let Ok(tube) = primitive_tube_f64(&input, [previous, current, next], f64::INFINITY)
                else {
                    continue;
                };
                if tube.is_empty() {
                    continue;
                }
                let Some(start_vertex) = tube.start_polygon.vertices().first().copied() else {
                    continue;
                };
                let end_point = tube.start_to_end.apply(start_vertex);
                let roundtrip = tube.end_to_start.apply(end_point);
                let roundtrip_error = (roundtrip - start_vertex).norm();
                assert!(
                    roundtrip_error < 1e-7,
                    "facets=({previous},{current},{next}) roundtrip_error={roundtrip_error:e} det={:e}",
                    tube.start_to_end.matrix.determinant()
                );
                assert!(tube.end_polygon.contains(&end_point));
                assert!(
                    (tube.action_at_start(start_vertex) - tube.action_at_end(end_point)).abs()
                        < 1e-7
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0);
}

#[test]
fn primitive_tube_f64_marks_forbidden_transition_empty() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );

    let mut checked = 0usize;
    for previous in 0..fixture.facet_count() {
        for current in 0..fixture.facet_count() {
            for next in 0..fixture.facet_count() {
                if previous == current || current == next {
                    continue;
                }
                if fixture.facet_intersection_is_nonempty[(previous, current)]
                    && fixture.facet_intersection_is_nonempty[(current, next)]
                    && fixture.omega_signs[(previous, current)] >= 0
                    && fixture.omega_signs[(current, next)] >= 0
                {
                    continue;
                }
                match primitive_tube_f64(&input, [previous, current, next], f64::INFINITY) {
                    Ok(tube) => {
                        assert!(tube.is_empty());
                        checked += 1;
                    }
                    Err(
                        F64TubeError::SingularFaceFrame
                        | F64TubeError::NumericallyUnstableOmegaTransition,
                    ) => {}
                    Err(err) => panic!("unexpected primitive error: {err:?}"),
                }
            }
        }
    }
    assert!(checked > 0);
}

#[test]
fn intersect_tubes_f64_composes_maps_and_actions() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );

    let transition = complete_transition_matrix(fixture.facet_count());
    let mut composed = None;
    'outer: for word in enumerate_transition_pruned_words(&transition, 2) {
        let [a, b, c, d] = word.facets.as_slice() else {
            continue;
        };
        let Ok(first) = primitive_tube_f64(&input, [*a, *b, *c], f64::INFINITY) else {
            continue;
        };
        let Ok(second) = primitive_tube_f64(&input, [*b, *c, *d], f64::INFINITY) else {
            continue;
        };
        if first.is_empty() || second.is_empty() {
            continue;
        }
        if let Ok(tube) = intersect_tubes_f64(&first, &second) {
            if !tube.is_empty() {
                composed = Some((first, second, tube));
                break 'outer;
            }
        }
    }

    let (first, second, tube) = composed.expect("fixture should have one composable live tube");
    let start = tube
        .start_polygon
        .vertices()
        .first()
        .copied()
        .expect("live tube has a vertex");
    let middle = first.start_to_end.apply(start);
    let end = second.start_to_end.apply(middle);
    assert!((tube.start_to_end.apply(start) - end).norm() < 1e-7);
    let action_sum = first.action_at_start(start) + second.action_at_start(middle);
    assert!((tube.action_at_start(start) - action_sum).abs() < 1e-7);
}

#[test]
fn build_tube_for_word_f64_matches_manual_primitive_intersection() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );

    let mut checked = 0usize;
    for word in
        enumerate_transition_pruned_words(&complete_transition_matrix(fixture.facet_count()), 2)
    {
        let [a, b, c, d] = word.facets.as_slice() else {
            continue;
        };
        let Ok(Some(first)) = build_tube_for_word_f64(&input, &[*a, *b, *c], f64::INFINITY) else {
            continue;
        };
        let Ok(Some(second)) = build_tube_for_word_f64(&input, &[*b, *c, *d], f64::INFINITY) else {
            continue;
        };
        let Ok(manual) = intersect_tubes_f64(&first, &second) else {
            continue;
        };
        if manual.is_empty() {
            continue;
        }
        let recursive = build_tube_for_word_f64(&input, &word.facets, f64::INFINITY)
            .expect("recursive build should not error")
            .expect("manual live tube should be recursive-live");
        assert_eq!(recursive.sequence(), manual.sequence());
        assert_eq!(
            recursive.start_polygon().inequality_count(),
            manual.start_polygon().inequality_count()
        );
        assert_eq!(
            recursive.end_polygon().inequality_count(),
            manual.end_polygon().inequality_count()
        );
        checked += 1;
        if checked >= 10 {
            break;
        }
    }
    assert!(checked > 0);
}

#[test]
fn build_tube_for_word_f64_classifies_half_cache_words_on_hko_fixture() {
    let fixture = known_polytopes::hko_pentagon();
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );
    let transition = crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );
    let words =
        enumerate_transition_pruned_words(&transition, half_cache_depth(fixture.facet_count()));

    let mut live = 0usize;
    let mut empty = 0usize;
    let mut construction_errors = 0usize;
    for word in words.iter().take(500) {
        match build_tube_for_word_f64(&input, &word.facets, f64::INFINITY) {
            Ok(Some(tube)) => {
                assert!(!tube.is_empty());
                live += 1;
            }
            Ok(None) => empty += 1,
            Err(
                F64TubeError::SingularTubeMap
                | F64TubeError::UnsupportedDegenerateTransition
                | F64TubeError::NumericallyUnstableOmegaTransition,
            ) => {
                construction_errors += 1;
            }
            Err(err) => panic!("unexpected build error for {:?}: {err:?}", word.facets),
        }
    }
    assert!(live + empty + construction_errors > 0);
}

#[test]
fn closed_tube_f64_reports_near_zero_omega_for_hko_qp_best_sigma() {
    let fixture = known_polytopes::hko_pentagon();
    let qp = pruned_capacity_for_fixture(fixture).expect("QP capacity for HKO");
    let input = FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    );

    assert_eq!(
        input.validate_no_geometric_zero_omega_transitions(),
        Err(F64TubeError::UnsupportedZeroOmegaTransition)
    );
    assert_eq!(qp.best_sigma(), &[1, 8, 7, 3, 4, 5, 9]);
    assert_eq!(
        closed_tube_for_sigma_f64(&input, qp.best_sigma(), f64::INFINITY),
        Err(F64TubeError::NumericallyUnstableOmegaTransition)
    );

    let zero_omega_primitive_count = (0..qp.best_sigma().len())
        .filter(|&i| {
            let sigma = qp.best_sigma();
            fixture.omega_signs[(sigma[i], sigma[(i + 1) % sigma.len()])] == 0
                || fixture.omega_signs[(sigma[(i + 1) % sigma.len()], sigma[(i + 2) % sigma.len()])]
                    == 0
        })
        .count();
    assert_eq!(zero_omega_primitive_count, 6);
}
