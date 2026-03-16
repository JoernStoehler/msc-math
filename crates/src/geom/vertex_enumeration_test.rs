//! Tests for exact vertex enumeration pipeline and rational linear algebra.
//!
//! Each test is a mathematical proposition verified computationally.
//! Tests here cover: det4, solve4, dot4, rank_over_q, cross_product_4d_rational,
//! affine_rank_rational, construct_rational_pipeline, enumerate_vertices_exact,
//! check_bounded_rational, combinations4.

use super::*;
use crate::geom::polytope::{ConstructionError, Polytope4D};
use crate::geom::rational::{frac, rat};
use std::collections::BTreeSet;

// ── Test helpers ────────────────────────────────────────────────────────

/// Build a rational 4-simplex with exact rational coordinates.
///
/// Simplex with vertices at (-1/5)·1 + (9/5)·eᵢ for i=1..4, plus (-1/5)·1.
/// The origin is interior (all gaps = 1/5 > 0). Uses non-unit normals.
///
/// Facets:
///   0: -x₁ ≤ 1/5   (n = (-1,0,0,0), h = 1/5)
///   1: -x₂ ≤ 1/5   (n = (0,-1,0,0), h = 1/5)
///   2: -x₃ ≤ 1/5   (n = (0,0,-1,0), h = 1/5)
///   3: -x₄ ≤ 1/5   (n = (0,0,0,-1), h = 1/5)
///   4: x₁+x₂+x₃+x₄ ≤ 1   (n = (1,1,1,1), h = 1)
fn rational_simplex() -> Polytope4D {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![
        frac(1, 5),
        frac(1, 5),
        frac(1, 5),
        frac(1, 5),
        rat(1),
    ];
    Polytope4D::from_rationals(normals, heights).expect("simplex construction")
}

/// Build a rational hypercube [-1, 1]⁴ with exact integer coordinates.
///
/// 8 facets, 16 vertices.
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

/// Helper: extract vertex descriptors from incidence matrix.
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

// ── Exact arithmetic correctness ────────────────────────────────────────

/// Proposition: the 4-simplex has exactly 5 vertex descriptors,
/// each a 4-element subset of {0, 1, 2, 3, 4}.
#[test]
fn exact_simplex_vertices() {
    let s = rational_simplex();
    let vds = vertex_descriptors_from_incidence(&s);
    assert_eq!(vds.len(), 5);
    for vd in &vds {
        assert_eq!(vd.len(), 4, "simplex vertex should be on exactly 4 facets");
        assert!(
            vd.iter().all(|&i| i < 5),
            "facet indices should be in 0..5"
        );
    }
    let expected: Vec<BTreeSet<usize>> = (0..5)
        .map(|omit| (0..5).filter(|&i| i != omit).collect())
        .collect();
    let mut actual: Vec<BTreeSet<usize>> = vds;
    actual.sort();
    let mut expected_sorted = expected;
    expected_sorted.sort();
    assert_eq!(actual, expected_sorted);
}

/// Proposition: the 4-simplex vertices have exact rational coordinates.
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
    for coord in v {
        assert_eq!(
            coord, &expected,
            "vertex omitting sum-constraint should be (-1/5, -1/5, -1/5, -1/5)"
        );
    }
}

/// Proposition: the hypercube [-1,1]⁴ has exactly 16 vertex descriptors,
/// each a 4-element subset of {0,...,7}.
#[test]
fn exact_hypercube_vertices() {
    let h = rational_hypercube();
    let vds = vertex_descriptors_from_incidence(&h);
    assert_eq!(vds.len(), 16);
    for vd in &vds {
        assert_eq!(vd.len(), 4);
    }
    for vd in &vds {
        let pairs = [(0, 1), (2, 3), (4, 5), (6, 7)];
        for (a, b) in pairs {
            let has_a = vd.contains(&a);
            let has_b = vd.contains(&b);
            assert!(
                has_a ^ has_b,
                "vertex should pick exactly one from pair ({a}, {b}), got both={}, neither={}",
                has_a && has_b,
                !has_a && !has_b
            );
        }
    }
}

/// Proposition: the hypercube vertices are exactly the points (±1, ±1, ±1, ±1).
#[test]
fn exact_hypercube_vertex_coordinates() {
    let h = rational_hypercube();
    let one = rat(1);
    let neg_one = rat(-1);
    for v in h.vertices() {
        for coord in v {
            assert!(
                coord == &one || coord == &neg_one,
                "hypercube vertex coordinate should be ±1, got {coord}"
            );
        }
    }
    assert_eq!(h.vertices().len(), 16);
}

/// Proposition: rank_over_q computes exact matrix rank.
#[test]
fn rank_over_q_basic() {
    let id = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(rank_over_q(&id), 4);
    let dup = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)],
    ];
    assert_eq!(rank_over_q(&dup), 3);
    let zeros = vec![[rat(0), rat(0), rat(0), rat(0)]];
    assert_eq!(rank_over_q(&zeros), 0);
    let empty: Vec<[BigRational; 4]> = vec![];
    assert_eq!(rank_over_q(&empty), 0);
    let single = vec![[rat(3), rat(-1), rat(0), rat(7)]];
    assert_eq!(rank_over_q(&single), 1);
}

/// Proposition: simplex dual vertices pass boundedness check (positively span R^4).
#[test]
fn simplex_is_bounded() {
    let p = rational_simplex();
    assert!(check_bounded_rational(p.dual_vertices()));
}

/// Proposition: hypercube dual vertices pass boundedness check.
#[test]
fn hypercube_is_bounded() {
    let p = rational_hypercube();
    assert!(check_bounded_rational(p.dual_vertices()));
}

/// Proposition: cross product in 4D over Q is perpendicular to all three inputs.
#[test]
fn cross_product_4d_rational_perpendicular() {
    let a = [rat(1), rat(2), rat(3), rat(4)];
    let b = [rat(5), rat(-1), rat(2), rat(0)];
    let c = [rat(0), rat(3), rat(-2), rat(1)];
    let d = cross_product_4d_rational(&a, &b, &c);
    assert!(dot4(&d, &a).is_zero(), "d·a = {} ≠ 0", dot4(&d, &a));
    assert!(dot4(&d, &b).is_zero(), "d·b = {} ≠ 0", dot4(&d, &b));
    assert!(dot4(&d, &c).is_zero(), "d·c = {} ≠ 0", dot4(&d, &c));
    assert!(!d.iter().all(|x| x.is_zero()), "cross product is zero");
}

/// Proposition: affine rank of simplex vertices = 4 (they span R^4).
#[test]
fn simplex_vertices_affine_rank() {
    let p = rational_simplex();
    assert_eq!(affine_rank_rational(p.vertices()), 4);
}

/// Proposition: affine rank of coplanar points < 4.
#[test]
fn coplanar_points_affine_rank() {
    let points = vec![
        [rat(0), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(affine_rank_rational(&points), 3);
}

/// Proposition: the determinant formula is correct on known matrices.
#[test]
fn det4_known_values() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(det4(&id), rat(1));
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(5), rat(6), rat(7), rat(8)],
        [rat(9), rat(10), rat(11), rat(12)],
    ];
    assert_eq!(det4(&singular), rat(0));
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    assert_eq!(det4(&diag), rat(210));
}

/// Proposition: Cramer's rule solver gives exact solutions.
#[test]
fn solve4_exact() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
    let x = solve4(&id, &rhs).expect("non-singular");
    assert_eq!(x, rhs);
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    let rhs2 = [rat(4), rat(9), rat(10), rat(21)];
    let x2 = solve4(&diag, &rhs2).expect("non-singular");
    assert_eq!(x2, [rat(2), rat(3), rat(2), rat(3)]);
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert!(solve4(&singular, &[rat(1), rat(1), rat(1), rat(1)]).is_none());
}

// ── Validation error tests ──────────────────────────────────────────────

/// Too few facets should fail.
#[test]
fn reject_too_few_facets() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
    ];
    let heights = vec![rat(1); 4];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::TooFewFacets(4)),
        "expected TooFewFacets, got {err}"
    );
}

/// Zero normal should fail (produces zero dual vertex).
#[test]
fn reject_zero_normal() {
    let normals = vec![
        [rat(0), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let heights = vec![rat(1); 5];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::ZeroDualVertex(0)),
        "expected ZeroDualVertex(0), got {err}"
    );
}

/// Non-positive height should fail.
#[test]
fn reject_nonpositive_height() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
    ];
    let heights = vec![rat(1), rat(0), rat(1), rat(1), rat(1)];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::ZeroDualVertex(1)),
        "expected ZeroDualVertex(1), got {err}"
    );
}

/// Redundant facet should fail.
#[test]
fn reject_redundant_facet() {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
        [rat(1), rat(0), rat(0), rat(0)],
    ];
    let heights = vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1), rat(100)];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::RedundantFacet(5)),
        "expected RedundantFacet(5), got {err}"
    );
}

/// Parallel halfspaces are unbounded (normals have rank 1).
#[test]
fn reject_unbounded_parallel() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(3), rat(0), rat(0), rat(0)],
        [rat(4), rat(0), rat(0), rat(0)],
        [rat(5), rat(0), rat(0), rat(0)],
    ];
    let heights = vec![rat(1), rat(2), rat(3), rat(4), rat(5)];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::Unbounded),
        "expected Unbounded, got {err}"
    );
}

/// Normals span R^4 but only from one side — unbounded.
#[test]
fn reject_unbounded_one_sided() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![rat(1); 5];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::Unbounded),
        "expected Unbounded, got {err}"
    );
}

/// Non-simple polytope is supported: hypercube + diagonal cut.
#[test]
fn non_simple_polytope_accepted() {
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
        rat(1), rat(1), rat(1), rat(1),
        rat(1), rat(1), rat(1), rat(1),
        rat(2),
    ];
    let rp = Polytope4D::from_rationals(normals, heights)
        .expect("non-simple polytope should be accepted");
    let vds = vertex_descriptors_from_incidence(&rp);
    assert_eq!(vds.len(), 15);
    let non_simple_count = vds.iter()
        .filter(|vd| vd.len() > 4)
        .count();
    assert_eq!(non_simple_count, 4, "expected 4 non-simple vertices");
}
