//! Tests for vertex enumeration: exact vertex computation from halfspaces.
//!
//! Proposition: The exact rational pipeline correctly enumerates all vertices
//! of a polytope K from its dual vertex (halfspace) representation, with
//! correct vertex-facet incidence and exact rational coordinates.
//! Reference: [lem:vertex-enumeration], [lem:positive-span]
//!
//! Strategy: fixture-based on simplex (5 facets) and hypercube (8 facets),
//! verifying vertex counts, descriptor structure, coordinate values,
//! affine rank, and boundedness.

use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::{frac, rat};
use crate::geom::vertex_enumeration::{affine_rank_rational, check_bounded_rational};
use std::collections::BTreeSet;

// ── Test fixtures ──────────────────────────────────────────────────────

/// Build a rational 4-simplex with exact rational coordinates.
///
/// Simplex with vertices at (-1/5)*1 + (9/5)*e_i for i=1..4, plus (-1/5)*1.
/// The origin is interior (all gaps = 1/5 > 0). Uses non-unit normals.
///
/// Facets:
///   0: -x_1 <= 1/5   (n = (-1,0,0,0), h = 1/5)
///   1: -x_2 <= 1/5   (n = (0,-1,0,0), h = 1/5)
///   2: -x_3 <= 1/5   (n = (0,0,-1,0), h = 1/5)
///   3: -x_4 <= 1/5   (n = (0,0,0,-1), h = 1/5)
///   4: x_1+x_2+x_3+x_4 <= 1   (n = (1,1,1,1), h = 1)
fn rational_simplex() -> Polytope4D {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];
    Polytope4D::from_rationals(normals, heights).expect("simplex construction")
}

/// Build a rational hypercube [-1, 1]^4 with exact integer coordinates.
///
/// 8 facets (+-e_i), 16 vertices (all sign combinations of (1,1,1,1)).
fn rational_hypercube() -> Polytope4D {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
    ];
    let heights = vec![rat(1); 8];
    Polytope4D::from_rationals(normals, heights).expect("hypercube construction")
}

/// Extract vertex descriptors (sets of incident facet indices) from incidence matrix.
fn vertex_descriptors_from_incidence(p: &Polytope4D) -> Vec<BTreeSet<usize>> {
    let inc = p.incidence();
    let v_count = p.vertices().len();
    let f_count = p.facet_count();
    (0..v_count)
        .map(|vi| {
            (0..f_count)
                .filter(|&fi| inc[(vi, fi)])
                .collect::<BTreeSet<usize>>()
        })
        .collect()
}

// ── Simplex vertex structure ────────────────────────────────────────────

/// Proposition: the 4-simplex has exactly 5 vertex descriptors,
/// each a 4-element subset of {0, 1, 2, 3, 4} (one facet omitted per vertex).
#[test]
fn exact_simplex_vertex_descriptors() {
    let s = rational_simplex();
    let vds = vertex_descriptors_from_incidence(&s);
    assert_eq!(vds.len(), 5, "simplex should have exactly 5 vertices");

    for vd in &vds {
        assert_eq!(vd.len(), 4, "simplex vertex should lie on exactly 4 facets");
        assert!(
            vd.iter().all(|&i| i < 5),
            "facet indices should be in 0..5"
        );
    }

    // Each vertex descriptor is {0..4} minus one element
    let expected: Vec<BTreeSet<usize>> = (0..5)
        .map(|omit| (0..5).filter(|&i| i != omit).collect())
        .collect();
    let mut actual = vds;
    actual.sort();
    let mut expected_sorted = expected;
    expected_sorted.sort();
    assert_eq!(actual, expected_sorted);
}

/// Proposition: the simplex vertex on facets {0,1,2,3} (omitting the sum-constraint)
/// has exact coordinates (-1/5, -1/5, -1/5, -1/5).
#[test]
fn exact_simplex_vertex_coordinates() {
    let s = rational_simplex();
    let vds = vertex_descriptors_from_incidence(&s);

    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = vds
        .iter()
        .position(|vd| *vd == target_vd)
        .expect("vertex {0,1,2,3} should exist");

    let v = &s.vertices()[idx];
    let expected = frac(-1, 5);
    for (c, coord) in v.iter().enumerate() {
        assert_eq!(
            coord, &expected,
            "coordinate {c} should be -1/5, got {coord}"
        );
    }
}

// ── Hypercube vertex structure ──────────────────────────────────────────

/// Proposition: the hypercube [-1,1]^4 has exactly 16 vertex descriptors,
/// each a 4-element subset of {0,...,7}, picking one from each opposing pair.
#[test]
fn exact_hypercube_vertex_descriptors() {
    let h = rational_hypercube();
    let vds = vertex_descriptors_from_incidence(&h);
    assert_eq!(vds.len(), 16, "hypercube should have 16 vertices");

    for vd in &vds {
        assert_eq!(
            vd.len(),
            4,
            "hypercube vertex should lie on exactly 4 facets"
        );
        // Each opposing pair (0,1), (2,3), (4,5), (6,7) should contribute exactly one
        let pairs = [(0, 1), (2, 3), (4, 5), (6, 7)];
        for (a, b) in pairs {
            let has_a = vd.contains(&a);
            let has_b = vd.contains(&b);
            assert!(
                has_a ^ has_b,
                "vertex should pick exactly one from pair ({a}, {b})"
            );
        }
    }
}

/// Proposition: the hypercube vertices are exactly the 16 points (+-1, +-1, +-1, +-1).
#[test]
fn exact_hypercube_vertex_coordinates() {
    let h = rational_hypercube();
    let one = rat(1);
    let neg_one = rat(-1);

    for v in h.vertices() {
        for coord in v {
            assert!(
                coord == &one || coord == &neg_one,
                "hypercube vertex coordinate should be +/-1, got {coord}"
            );
        }
    }
    assert_eq!(h.vertices().len(), 16);
}

// ── Affine rank ─────────────────────────────────────────────────────────

/// Proposition: affine rank of the 5 simplex vertices = 4 (they span R^4).
#[test]
fn simplex_vertices_affine_rank_is_4() {
    let p = rational_simplex();
    assert_eq!(affine_rank_rational(p.vertices()), 4);
}

/// Proposition: 4 points in the hyperplane x_3 = 0 have affine rank 3 (< 4).
#[test]
fn coplanar_points_affine_rank_below_4() {
    let points = vec![
        [rat(0), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(affine_rank_rational(&points), 3);
}

// ── Boundedness ─────────────────────────────────────────────────────────

/// Proposition: simplex dual vertices positively span R^4 (simplex is bounded).
#[test]
fn simplex_is_bounded() {
    let p = rational_simplex();
    assert!(check_bounded_rational(p.dual_vertices()));
}

/// Proposition: hypercube dual vertices positively span R^4 (hypercube is bounded).
#[test]
fn hypercube_is_bounded() {
    let p = rational_hypercube();
    assert!(check_bounded_rational(p.dual_vertices()));
}

// ── Non-simple polytope ─────────────────────────────────────────────────

/// Proposition: non-simple polytopes (vertices on > 4 facets) are correctly handled.
/// A hypercube with a diagonal cut at x_1+x_2+x_3+x_4 <= 2 produces 4 non-simple
/// vertices (on 5 facets) and 11 simple vertices, totalling 15.
#[test]
fn non_simple_polytope_vertex_enumeration() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![
        rat(1),
        rat(1),
        rat(1),
        rat(1),
        rat(1),
        rat(1),
        rat(1),
        rat(1),
        rat(2),
    ];
    let p =
        Polytope4D::from_rationals(normals, heights).expect("non-simple polytope should succeed");

    let vds = vertex_descriptors_from_incidence(&p);
    assert_eq!(vds.len(), 15, "cut hypercube should have 15 vertices");

    let non_simple_count = vds.iter().filter(|vd| vd.len() > 4).count();
    assert_eq!(
        non_simple_count, 4,
        "expected 4 non-simple vertices (on the diagonal cut)"
    );
}
