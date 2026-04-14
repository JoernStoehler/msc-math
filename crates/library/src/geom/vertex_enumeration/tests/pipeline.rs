//! End-to-end and regression tests for the refactored vertex enumeration pipeline.

use super::super::boundedness::{check_bounded_rational, integer_scale_dual_vertices};
use super::super::linear_algebra::{
    affine_rank_rational, cross_product_4d_rational, det4, dot4, rank_over_q, solve4,
};
use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::{frac, rat};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use std::collections::BTreeSet;

// Tests for vertex enumeration: exact vertex computation from halfspaces.
//
// Proposition: The exact rational pipeline correctly enumerates all vertices
// of a polytope K from its dual vertex (halfspace) representation, with
// correct vertex-facet incidence and exact rational coordinates.
// Reference: [lem:vertex-enumeration], [lem:positive-span]
//
// Strategy: fixture-based on simplex (5 facets) and hypercube (8 facets),
// verifying vertex counts, descriptor structure, coordinate values,
// affine rank, and boundedness.

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
    let dual_vertices: Vec<[BigRational; 4]> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
        .collect();
    Polytope4D::new(dual_vertices).expect("simplex construction")
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
    let dual_vertices: Vec<[BigRational; 4]> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
        .collect();
    Polytope4D::new(dual_vertices).expect("hypercube construction")
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
        assert!(vd.iter().all(|&i| i < 5), "facet indices should be in 0..5");
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
    let dual_vertices: Vec<[BigRational; 4]> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
        .collect();
    let p = Polytope4D::new(dual_vertices).expect("non-simple polytope should succeed");

    let vds = vertex_descriptors_from_incidence(&p);
    assert_eq!(vds.len(), 15, "cut hypercube should have 15 vertices");

    let non_simple_count = vds.iter().filter(|vd| vd.len() > 4).count();
    assert_eq!(
        non_simple_count, 4,
        "expected 4 non-simple vertices (on the diagonal cut)"
    );
}

// ---- Linear algebra tests ----
//
// Tests for exact rational linear algebra helpers used by vertex enumeration.
//
// Proposition: The low-level linear algebra routines (det4, solve4, rank_over_q,
// cross_product_4d_rational, dot4) compute exact results over Q with no
// floating-point approximation.
// Reference: [lem:vertex-enumeration]
//
// Strategy: fixture-based on known matrices (identity, diagonal, singular)
// and vectors, verifying exact algebraic identities.

// ── Determinant ─────────────────────────────────────────────────────────

/// Proposition: det(I_4) = 1.
#[test]
fn det4_identity() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(det4(&id), rat(1));
}

/// Proposition: a matrix with two identical rows has determinant 0.
#[test]
fn det4_singular() {
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(5), rat(6), rat(7), rat(8)],
        [rat(9), rat(10), rat(11), rat(12)],
    ];
    assert_eq!(det4(&singular), rat(0));
}

/// Proposition: det(diag(2,3,5,7)) = 210.
#[test]
fn det4_diagonal() {
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    assert_eq!(det4(&diag), rat(210));
}

// ── Linear system solver (Cramer's rule) ────────────────────────────────

/// Proposition: solving I*x = b yields x = b.
#[test]
fn solve4_identity_system() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
    let x = solve4(&id, &rhs).expect("non-singular");
    assert_eq!(x, rhs);
}

/// Proposition: solving diag(2,3,5,7)*x = (4,9,10,21) yields x = (2,3,2,3).
#[test]
fn solve4_diagonal_system() {
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    let rhs = [rat(4), rat(9), rat(10), rat(21)];
    let x = solve4(&diag, &rhs).expect("non-singular");
    assert_eq!(x, [rat(2), rat(3), rat(2), rat(3)]);
}

/// Proposition: solve4 returns None for a singular system (two identical rows).
#[test]
fn solve4_singular_returns_none() {
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert!(solve4(&singular, &[rat(1), rat(1), rat(1), rat(1)]).is_none());
}

// ── Matrix rank ─────────────────────────────────────────────────────────

/// Proposition: rank(I_4) = 4.
#[test]
fn rank_over_q_identity() {
    let id = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(rank_over_q(&id), 4);
}

/// Proposition: replacing one row with a scalar multiple of another drops rank to 3.
#[test]
fn rank_over_q_dependent_row() {
    let rows = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)], // 2 * row 0
    ];
    assert_eq!(rank_over_q(&rows), 3);
}

/// Proposition: the zero vector has rank 0.
#[test]
fn rank_over_q_zero_vector() {
    let zeros = vec![[rat(0), rat(0), rat(0), rat(0)]];
    assert_eq!(rank_over_q(&zeros), 0);
}

/// Proposition: the empty set has rank 0.
#[test]
fn rank_over_q_empty() {
    let empty: Vec<[BigRational; 4]> = vec![];
    assert_eq!(rank_over_q(&empty), 0);
}

/// Proposition: a single nonzero vector has rank 1.
#[test]
fn rank_over_q_single_nonzero() {
    let single = vec![[rat(3), rat(-1), rat(0), rat(7)]];
    assert_eq!(rank_over_q(&single), 1);
}

// ── 4D cross product ────────────────────────────────────────────────────

/// Proposition: cross_product_4d_rational(a, b, c) is perpendicular to all three inputs
/// and is nonzero when a, b, c are linearly independent.
#[test]
fn cross_product_4d_rational_perpendicular() {
    let a = [rat(1), rat(2), rat(3), rat(4)];
    let b = [rat(5), rat(-1), rat(2), rat(0)];
    let c = [rat(0), rat(3), rat(-2), rat(1)];
    let d = cross_product_4d_rational(&a, &b, &c);

    assert!(
        dot4(&d, &a).is_zero(),
        "d . a = {} should be 0",
        dot4(&d, &a)
    );
    assert!(
        dot4(&d, &b).is_zero(),
        "d . b = {} should be 0",
        dot4(&d, &b)
    );
    assert!(
        dot4(&d, &c).is_zero(),
        "d . c = {} should be 0",
        dot4(&d, &c)
    );
    assert!(
        !d.iter().all(|x| x.is_zero()),
        "cross product should be nonzero for independent inputs"
    );
}

/// Proposition: cross product of three dependent vectors is the zero vector.
#[test]
fn cross_product_4d_rational_dependent_is_zero() {
    let a = [rat(1), rat(0), rat(0), rat(0)];
    let b = [rat(0), rat(1), rat(0), rat(0)];
    // c = a + b, linearly dependent
    let c = [rat(1), rat(1), rat(0), rat(0)];
    let d = cross_product_4d_rational(&a, &b, &c);
    assert!(
        d.iter().all(|x| x.is_zero()),
        "cross product of dependent vectors should be zero"
    );
}

// ── Edge cases for integer-scaled vertex enumeration pipeline ────────

/// Proposition: a vertex on 6 facets is correctly detected as non-simple.
///
/// Takes hypercube [-1,1]^4 (8 facets) and adds two diagonal cuts:
///   facet 8: x_1+x_2+x_3+x_4 <= 2
///   facet 9: x_1+x_2-x_3-x_4 <= 2
/// Both are non-redundant (they cut off cube vertices like (1,1,1,1)).
///
/// Vertex (1,1,1,-1) is tight on 6 facets: x_1<=1, x_2<=1, x_3<=1,
/// -x_4<=1, sum=1+1+1-1=2, diff=1+1-1+1=2. This exceeds the 5-facet
/// case in `non_simple_polytope_vertex_enumeration`.
#[test]
fn highly_non_simple_vertex_on_6_facets() {
    // Hypercube [-1,1]^4 plus two diagonal cuts.
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],   // 0: x_1 <= 1
        [rat(-1), rat(0), rat(0), rat(0)],  // 1: -x_1 <= 1
        [rat(0), rat(1), rat(0), rat(0)],   // 2: x_2 <= 1
        [rat(0), rat(-1), rat(0), rat(0)],  // 3: -x_2 <= 1
        [rat(0), rat(0), rat(1), rat(0)],   // 4: x_3 <= 1
        [rat(0), rat(0), rat(-1), rat(0)],  // 5: -x_3 <= 1
        [rat(0), rat(0), rat(0), rat(1)],   // 6: x_4 <= 1
        [rat(0), rat(0), rat(0), rat(-1)],  // 7: -x_4 <= 1
        [rat(1), rat(1), rat(1), rat(1)],   // 8: x_1+x_2+x_3+x_4 <= 2
        [rat(1), rat(1), rat(-1), rat(-1)], // 9: x_1+x_2-x_3-x_4 <= 2
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
        rat(2),
    ];
    let dual_vertices: Vec<[BigRational; 4]> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
        .collect();
    let p = Polytope4D::new(dual_vertices).expect("doubly-cut hypercube should succeed");

    let vds = vertex_descriptors_from_incidence(&p);

    // Find vertex (1,1,1,-1): on facets 0,2,4,7,8,9 (6 facets).
    let target = [rat(1), rat(1), rat(1), rat(-1)];
    let idx = p
        .vertices()
        .iter()
        .position(|v| (0..4).all(|c| v[c] == target[c]))
        .expect("vertex (1,1,1,-1) should exist");

    let vd = &vds[idx];
    assert_eq!(
        vd.len(),
        6,
        "vertex (1,1,1,-1) should lie on exactly 6 facets, got {}: {:?}",
        vd.len(),
        vd
    );
    // Verify specific facet incidence
    for &fi in &[0, 2, 4, 7, 8, 9] {
        assert!(
            vd.contains(&fi),
            "vertex (1,1,1,-1) should be incident to facet {fi}"
        );
    }
}

/// Proposition: vertex enumeration is correct for large-coordinate dual vertices (~1e6).
///
/// Scales the standard simplex dual vertices by 1e6. The integer scaling
/// pipeline must handle numerators of magnitude ~1e6 correctly, and the f64
/// prefilter must not give wrong signs for large coordinates.
#[test]
fn large_coordinate_dual_vertices() {
    let scale = rat(1_000_000);
    // Simplex with normals scaled by 1e6: same polytope, just different
    // representation (n_i -> 1e6 * n_i, h_i -> 1e6 * h_i, so y_i unchanged).
    // Instead, scale the dual vertices themselves to get large coordinates.
    // y_i = scale * original_y_i means h-rep: (scale * n_i / h_i) . x <= 1,
    // so K = {x : scale * y_i . x <= 1} = (1/scale) * K_original.
    let base_normals: Vec<[BigRational; 4]> = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let base_heights: Vec<BigRational> =
        vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];

    // Dual vertices = n_i / h_i, then scale by 1e6
    let dual_vertices: Vec<[BigRational; 4]> = base_normals
        .iter()
        .zip(base_heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &(&n[c] / h) * &scale))
        .collect();

    // Verify the coordinates are indeed large
    let max_coord: f64 = dual_vertices
        .iter()
        .flat_map(|y| y.iter())
        .filter(|c| !c.is_zero())
        .map(|c| {
            let f = c.numer().to_string().parse::<f64>().unwrap()
                / c.denom().to_string().parse::<f64>().unwrap();
            f.abs()
        })
        .fold(0.0f64, f64::max);
    assert!(
        max_coord >= 1e5,
        "dual vertex coordinates should be large, max = {max_coord}"
    );

    let p = Polytope4D::new(dual_vertices).expect("large-coordinate simplex should succeed");
    assert_eq!(
        p.vertices().len(),
        5,
        "scaled simplex should still have 5 vertices"
    );

    // The vertex on facets {0,1,2,3} should be (-1/5, -1/5, -1/5, -1/5) / scale
    let vds = vertex_descriptors_from_incidence(&p);
    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = vds
        .iter()
        .position(|vd| *vd == target_vd)
        .expect("vertex {0,1,2,3} should exist");
    let expected = frac(-1, 5_000_000);
    for (c, coord) in p.vertices()[idx].iter().enumerate() {
        assert_eq!(
            coord, &expected,
            "coordinate {c} of scaled simplex vertex should be -1/5000000"
        );
    }
}

/// Proposition: vertex enumeration is correct for small-coordinate dual vertices (~1e-6).
///
/// Scales dual vertices by 1e-6. Near-zero f64 values in the prefilter must
/// not cause incorrect sign decisions; the exact integer fallback must handle
/// the resulting small numerators and large common denominator correctly.
#[test]
fn small_coordinate_dual_vertices() {
    // Scale = 1/1000000
    let scale = frac(1, 1_000_000);
    let base_normals: Vec<[BigRational; 4]> = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let base_heights: Vec<BigRational> =
        vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];

    let dual_vertices: Vec<[BigRational; 4]> = base_normals
        .iter()
        .zip(base_heights.iter())
        .map(|(n, h)| std::array::from_fn(|c| &(&n[c] / h) * &scale))
        .collect();

    let p = Polytope4D::new(dual_vertices).expect("small-coordinate simplex should succeed");
    assert_eq!(
        p.vertices().len(),
        5,
        "scaled simplex should still have 5 vertices"
    );

    // Vertex on facets {0,1,2,3}: coordinates = -1/5 / scale = -1/5 * 1e6 = -200000
    let vds = vertex_descriptors_from_incidence(&p);
    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = vds
        .iter()
        .position(|vd| *vd == target_vd)
        .expect("vertex {0,1,2,3} should exist");
    let expected = rat(-200_000);
    for (c, coord) in p.vertices()[idx].iter().enumerate() {
        assert_eq!(
            coord, &expected,
            "coordinate {c} of small-scale simplex vertex should be -200000"
        );
    }
}

/// Proposition: exact rational construction handles non-power-of-2 denominators.
///
/// Constructs a simplex from exact rationals with denominators 3, 5, 7 — values
/// that are not exactly representable in f64. The common denominator computation
/// (lcm) must correctly combine these primes, and the integer Cramer pipeline
/// must produce exact vertex coordinates.
#[test]
fn exact_rational_non_power_of_two_denominators() {
    // A simplex with non-power-of-2 heights: h_0..h_3 = 1/3, h_4 = 2/7.
    // Dual vertices y_i = n_i / h_i.
    //
    // Facets:
    //   0: -x_1 <= 1/3   => y = (-3, 0, 0, 0)
    //   1: -x_2 <= 1/3   => y = (0, -3, 0, 0)
    //   2: -x_3 <= 1/3   => y = (0, 0, -3, 0)
    //   3: -x_4 <= 1/3   => y = (0, 0, 0, -3)
    //   4: x_1+x_2+x_3+x_4 <= 2/7  => y = (7/2, 7/2, 7/2, 7/2)
    let dual_vertices: Vec<[BigRational; 4]> = vec![
        [rat(-3), rat(0), rat(0), rat(0)],
        [rat(0), rat(-3), rat(0), rat(0)],
        [rat(0), rat(0), rat(-3), rat(0)],
        [rat(0), rat(0), rat(0), rat(-3)],
        [frac(7, 2), frac(7, 2), frac(7, 2), frac(7, 2)],
    ];

    let p = Polytope4D::new(dual_vertices.clone())
        .expect("non-power-of-2 denominator simplex should succeed");
    assert_eq!(p.vertices().len(), 5, "simplex should have 5 vertices");

    // Verify common denominator handles the lcm(1,1,1,1,2) = 2 correctly
    // by checking that the integer scaling produces correct results.
    let (int_verts, common_denom) = integer_scale_dual_vertices(&dual_vertices);
    // lcm of all denominators: denominators are 1,1,1,1,2 so lcm = 2
    assert_eq!(
        common_denom,
        BigInt::from(2),
        "common denominator should be lcm(1,1,1,1,2) = 2"
    );
    // int_verts[0] = (-3, 0, 0, 0) * 2 = (-6, 0, 0, 0)
    assert_eq!(int_verts[0][0], BigInt::from(-6));
    // int_verts[4] = (7/2, 7/2, 7/2, 7/2) * 2 = (7, 7, 7, 7)
    assert_eq!(int_verts[4][0], BigInt::from(7));

    // Vertex on facets {0,1,2,3} (omitting the sum constraint):
    // Solving -x_i = 1/3 for i=1..4 gives x_i = -1/3.
    let vds = vertex_descriptors_from_incidence(&p);
    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = vds
        .iter()
        .position(|vd| *vd == target_vd)
        .expect("vertex {0,1,2,3} should exist");
    let expected = frac(-1, 3);
    for (c, coord) in p.vertices()[idx].iter().enumerate() {
        assert_eq!(
            coord, &expected,
            "coordinate {c} should be -1/3, got {coord}"
        );
    }

    // Vertex on facets {1,2,3,4} (omitting facet 0, the -x_1 constraint):
    // x_2 = x_3 = x_4 = -1/3, and x_1+x_2+x_3+x_4 = 2/7.
    // So x_1 = 2/7 - (-1/3)*3 = 2/7 + 1 = 9/7.
    let target_vd2: BTreeSet<usize> = [1, 2, 3, 4].into_iter().collect();
    let idx2 = vds
        .iter()
        .position(|vd| *vd == target_vd2)
        .expect("vertex {1,2,3,4} should exist");
    let v = &p.vertices()[idx2];
    assert_eq!(v[0], frac(9, 7), "x_1 should be 9/7, got {}", v[0]);
    assert_eq!(v[1], frac(-1, 3), "x_2 should be -1/3, got {}", v[1]);
    assert_eq!(v[2], frac(-1, 3), "x_3 should be -1/3, got {}", v[2]);
    assert_eq!(v[3], frac(-1, 3), "x_4 should be -1/3, got {}", v[3]);
}

/// Proposition: integer Cramer's rule produces exact vertex coordinates for the
/// hypercube, matching the known analytical values.
///
/// For the hypercube [-1,1]^4, each vertex (s_1, s_2, s_3, s_4) with s_i in {-1,+1}
/// is the unique solution of the 4x4 system formed by its 4 defining facets.
/// This test verifies that the integer pipeline produces these exact coordinates
/// by checking every vertex against the expected Cramer solution.
#[test]
fn integer_cramer_exact_coordinates_hypercube() {
    let h = rational_hypercube();
    let vds = vertex_descriptors_from_incidence(&h);

    // For each vertex, verify coordinates match the expected sign pattern.
    // Facet 2k: +e_{k+1} . x <= 1, so vertex on facet 2k has x_{k+1} = +1.
    // Facet 2k+1: -e_{k+1} . x <= 1, so vertex on facet 2k+1 has x_{k+1} = -1.
    for (vi, vd) in vds.iter().enumerate() {
        let v = &h.vertices()[vi];
        for dim in 0..4 {
            let expected = if vd.contains(&(2 * dim)) {
                rat(1)
            } else {
                assert!(
                    vd.contains(&(2 * dim + 1)),
                    "vertex must be on one of the pair"
                );
                rat(-1)
            };
            assert_eq!(
                v[dim], expected,
                "vertex {vi}, coordinate {dim}: expected {expected}, got {}",
                v[dim]
            );
        }
    }
}

/// Proposition: integer Cramer's rule produces exact vertex coordinates for the
/// simplex, verifiable via the defining equations y_i . v = 1.
///
/// For each vertex v and each defining facet i (in its descriptor), the inner
/// product y_i . v must be exactly 1. For non-defining facets, y_i . v < 1.
/// This end-to-end check verifies the full Cramer pipeline (det4_int, numerator
/// dets, coordinate assembly) without relying on known analytical formulas.
#[test]
fn integer_cramer_exact_coordinates_simplex() {
    let s = rational_simplex();
    let vds = vertex_descriptors_from_incidence(&s);
    let dual_verts = s.dual_vertices();

    for (vi, vd) in vds.iter().enumerate() {
        let v = &s.vertices()[vi];
        for fi in 0..dual_verts.len() {
            let prod = dot4(
                &std::array::from_fn(|c| dual_verts[fi][c].clone()),
                &std::array::from_fn(|c| v[c].clone()),
            );
            if vd.contains(&fi) {
                assert_eq!(
                    prod,
                    rat(1),
                    "vertex {vi} on facet {fi}: y.v should be exactly 1, got {prod}"
                );
            } else {
                assert!(
                    prod < rat(1),
                    "vertex {vi} not on facet {fi}: y.v should be < 1, got {prod}"
                );
            }
        }
    }
}
