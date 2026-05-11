use super::*;
use crate::geom::known_polytopes;

// Tests for facet_adjacency: directed transition feasibility and cycle checks.
//
// Proposition: directed transition feasibility correctly combines facet
// intersection nonemptiness and omega_0-sign conditions.
// Reference: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]
//
// Strategy: fixture-based on simplex, hypercube, and Lagrangian products.

#[test]
fn flat_transition_matrix_combines_facet_intersection_and_nonnegative_omega() {
    #[rustfmt::skip]
    let facet_intersection_is_nonempty = DMatrix::from_row_slice(3, 3, &[
        false, true,  true,
        true,  false, true,
        true,  true,  false,
    ]);
    #[rustfmt::skip]
    let omega_signs = DMatrix::from_row_slice(3, 3, &[
        0,  1, -1,
       -1,  0,  0,
        1, -1,  0,
    ]);

    let directed = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersection_is_nonempty,
        &omega_signs,
    );

    #[rustfmt::skip]
    let expected = DMatrix::from_row_slice(3, 3, &[
        false, true,  false,
        false, false, true,
        true,  false, false,
    ]);
    assert_eq!(directed, expected);
}

#[test]
#[should_panic(
    expected = "facet_intersection_is_nonempty and omega_signs must have the same shape"
)]
fn flat_transition_matrix_rejects_shape_mismatch() {
    let facet_intersection_is_nonempty = DMatrix::from_element(2, 2, false);
    let omega_signs = DMatrix::from_element(3, 3, 0);

    let _ = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
}

/// Simplex (5 facets): every pair of facets shares a vertex (complete graph).
/// Facet-intersection nonemptiness should be all-true except diagonal.
#[test]
#[allow(clippy::needless_range_loop)]
fn simplex_facet_intersection_is_complete() {
    let kp = known_polytopes::simplex();
    let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
    let f = kp.facet_count();
    assert_eq!(f, 5);

    for i in 0..f {
        for j in 0..f {
            if i == j {
                assert!(
                    !facet_intersection_is_nonempty[(i, j)],
                    "diagonal should be false"
                );
            } else {
                assert!(
                    facet_intersection_is_nonempty[(i, j)],
                    "simplex facets {i} and {j} should intersect"
                );
            }
        }
    }
}

/// Hypercube (8 facets in 4D): opposite facets do not intersect.
/// Each facet should intersect exactly 6 others (all except itself and its opposite).
#[test]
#[allow(clippy::needless_range_loop)]
fn hypercube_facet_intersection_excludes_opposite_facets() {
    let kp = known_polytopes::hypercube();
    let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
    let f = kp.facet_count();
    assert_eq!(f, 8);

    for i in 0..f {
        let intersecting_facet_count: usize = (0..f)
            .filter(|&j| facet_intersection_is_nonempty[(i, j)])
            .count();
        // In a 4D hypercube, each facet (a 3D cube) shares vertices with 6 of 7 other facets.
        assert_eq!(
            intersecting_facet_count, 6,
            "facet {i} should intersect 6 other facets, got {intersecting_facet_count}"
        );
    }
}

/// Facet-intersection nonemptiness is symmetric.
#[test]
#[allow(clippy::needless_range_loop)]
fn facet_intersection_is_symmetric() {
    for kp in known_polytopes::all_known() {
        let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
        let f = kp.facet_count();
        for i in 0..f {
            for j in 0..f {
                assert_eq!(
                    facet_intersection_is_nonempty[(i, j)],
                    facet_intersection_is_nonempty[(j, i)],
                    "asymmetric facet intersection at ({i},{j}) for {}",
                    kp.name
                );
            }
        }
    }
}

/// Directed transition feasibility is a subset of facet-intersection nonemptiness.
#[test]
fn directed_transition_is_subset_of_facet_intersection() {
    for kp in known_polytopes::all_known() {
        let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
        let omega_signs = &kp.omega_signs;
        let directed = build_transition_matrix_from_facet_intersections_and_omega(
            &facet_intersection_is_nonempty,
            &omega_signs,
        );
        let f = kp.facet_count();
        for i in 0..f {
            for j in 0..f {
                if directed[(i, j)] {
                    assert!(
                        facet_intersection_is_nonempty[(i, j)],
                        "directed transition ({i},{j}) true but facets do not intersect for {}",
                        kp.name
                    );
                }
            }
        }
    }
}

/// Directed transition feasibility is NOT symmetric in general (omega_0 is antisymmetric).
/// If omega_0(n_i, n_j) > 0, then omega_0(n_j, n_i) < 0, so directed[(i,j)] and directed[(j,i)]
/// cannot both be true unless omega_0(n_i, n_j) = 0.
#[test]
fn directed_transition_antisymmetry_property() {
    for kp in known_polytopes::all_known() {
        let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
        let omega_signs = &kp.omega_signs;
        let directed = build_transition_matrix_from_facet_intersections_and_omega(
            &facet_intersection_is_nonempty,
            &omega_signs,
        );
        let omega_signs = &kp.omega_signs;
        let f = kp.facet_count();
        for i in 0..f {
            for j in (i + 1)..f {
                if directed[(i, j)] && directed[(j, i)] {
                    // Both directions allowed only when omega_0(n_i, n_j) = 0
                    assert_eq!(
                        omega_signs[(i, j)],
                        0,
                        "both directed[{i}][{j}] and directed[{j}][{i}] true \
                         but omega_signs != 0 for {}",
                        kp.name
                    );
                }
            }
        }
    }
}

/// Directed transition feasibility strictly prunes compared to facet intersection on generic polytopes.
/// For polytopes where omega_0 signs are nonzero (generic case), directed should have
/// strictly fewer true entries than facet-intersection nonemptiness.
#[test]
fn directed_transition_prunes_vs_facet_intersection() {
    // The simplex is generic enough that directed should prune some edges
    let kp = known_polytopes::simplex();
    let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
    let omega_signs = &kp.omega_signs;
    let directed = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
    let count_facet_intersections: usize = facet_intersection_is_nonempty
        .iter()
        .filter(|&&v| v)
        .count();
    let count_directed: usize = directed.iter().filter(|&&v| v).count();

    assert!(
        count_directed < count_facet_intersections,
        "directed ({count_directed}) should have fewer entries than facet intersections ({count_facet_intersections})"
    );
}

/// is_feasible_cycle: a valid cycle on a complete transition graph always returns true.
#[test]
fn is_feasible_cycle_complete_graph() {
    // 4-facet complete graph (all true except diagonal)
    let transition_is_allowed = DMatrix::from_fn(4, 4, |i, j| i != j);
    assert!(is_feasible_cycle(&[0, 1, 2, 3], &transition_is_allowed));
    assert!(is_feasible_cycle(&[3, 2, 1, 0], &transition_is_allowed));
    assert!(is_feasible_cycle(&[0, 2, 1, 3], &transition_is_allowed));
}

/// is_feasible_cycle: returns false when a transition is missing.
#[test]
fn is_feasible_cycle_missing_edge() {
    // 4 facets, but 0->2 is not allowed.
    #[rustfmt::skip]
    let data = vec![
        false, true, false, true,
        true, false, true, true,
        false, true, false, true,
        true, true, true, false,
    ];
    let transition_is_allowed = DMatrix::from_row_slice(4, 4, &data);
    // 0->2 missing, so [0,2,1,3] should fail at the 0->2 step
    assert!(!is_feasible_cycle(&[0, 2, 1, 3], &transition_is_allowed));
    // But [0,1,2,3] should work: 0->1 ok, 1->2 ok, 2->3 ok, 3->0 ok
    assert!(is_feasible_cycle(&[0, 1, 2, 3], &transition_is_allowed));
}

/// is_feasible_cycle: empty permutation is accepted.
#[test]
fn is_feasible_cycle_empty() {
    let transition_is_allowed = DMatrix::from_element(0, 0, false);
    assert!(is_feasible_cycle(&[], &transition_is_allowed));
}

/// is_feasible_cycle: single-element permutation checks the diagonal entry.
#[test]
fn is_feasible_cycle_single_element() {
    let transition_is_allowed = DMatrix::from_element(1, 1, false);
    // Single element: checks transition_is_allowed[(0,0)] which is false.
    assert!(!is_feasible_cycle(&[0], &transition_is_allowed));
}

/// Lagrangian product: directed transition feasibility respects the Q/P facet structure.
/// Q-type facets have normals in the (q1,q2,0,0) subspace and P-type in (0,0,p1,p2).
/// omega_0 between two Q-type normals is 0 (both in Lagrangian subspace), similarly for P-type.
/// omega_0 between a Q-type and P-type normal is generically nonzero.
#[test]
fn lagrangian_product_q_q_transitions_bidirectional() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let facet_intersection_is_nonempty = &kp.facet_intersection_is_nonempty;
    let omega_signs = &kp.omega_signs;
    let directed = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersection_is_nonempty,
        &omega_signs,
    );
    let f = kp.facet_count();

    // For pairs where omega_signs == 0 and facets intersect,
    // both directions should be allowed in the directed matrix.
    for i in 0..f {
        for j in 0..f {
            if facet_intersection_is_nonempty[(i, j)] && omega_signs[(i, j)] == 0 {
                assert!(
                    directed[(i, j)] && directed[(j, i)],
                    "omega_0=0 pair ({i},{j}) should be bidirectional in directed transition matrix"
                );
            }
        }
    }
}
