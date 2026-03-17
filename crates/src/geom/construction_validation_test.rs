//! Tests for polytope construction: edge cases and error paths.
//!
//! Proposition: Polytope4D::new rejects invalid inputs with the correct
//! ConstructionError variant: too few facets, zero dual vertex, duplicates,
//! unbounded, and redundant facets.
//! Reference: [def:polytope-dual]
//!
//! Strategy: exhaustive for each error variant

use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

/// Minimal valid halfspaces for a simplex-like polytope (5 facets).
fn simplex_halfspaces() -> Vec<Vector4<f64>> {
    vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ]
}

// ---- TooFewFacets ----

/// Verify Polytope4D::new rejects 4 facets (minimum is 5 in R^4).
#[test]
fn reject_too_few_facets_4() {
    let halfspaces = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(4));
}

/// Verify Polytope4D::new rejects an empty halfspace list.
#[test]
fn reject_too_few_facets_0() {
    let halfspaces: Vec<Vector4<f64>> = vec![];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(0));
}

/// Verify Polytope4D::new rejects a single halfspace.
#[test]
fn reject_too_few_facets_1() {
    let halfspaces = vec![Vector4::x()];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(1));
}

// ---- ZeroDualVertex ----

/// Verify Polytope4D::new rejects a zero-vector halfspace.
#[test]
fn reject_zero_halfspace() {
    let mut halfspaces = simplex_halfspaces();
    halfspaces[2] = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::ZeroDualVertex(2));
}

/// Verify Polytope4D::new rejects a near-zero (sub-epsilon) halfspace.
#[test]
fn reject_near_zero_halfspace() {
    let mut halfspaces = simplex_halfspaces();
    halfspaces[0] = Vector4::new(1e-16, 0.0, 0.0, 0.0);
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::ZeroDualVertex(0));
}

// ---- DuplicateHalfspaces ----

/// Verify Polytope4D::new rejects duplicate halfspaces.
#[test]
fn reject_duplicate_halfspaces() {
    let halfspaces = vec![
        Vector4::x(),
        Vector4::y(),
        Vector4::z(),
        Vector4::w(),
        Vector4::x(), // duplicate of [0]
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::DuplicateHalfspaces { i: 0, j: 4 });
}

// ---- Unbounded ----

/// Verify Polytope4D::new rejects halfspaces all pointing in roughly +x direction.
#[test]
fn reject_unbounded_all_positive_x() {
    // All halfspaces point roughly in +x direction -- unbounded in -x.
    let halfspaces = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

/// Verify Polytope4D::new rejects halfspaces missing the -w direction.
#[test]
fn reject_unbounded_missing_one_direction() {
    // Bounded in x, y, z but not in w (no -w halfspace).
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        // missing -Vector4::w()
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

// ---- RedundantFacet ----

/// Verify Polytope4D::new rejects a redundant diagonal facet on the hypercube.
#[test]
fn reject_redundant_diagonal_facet() {
    // Hypercube [-1,1]^4 + one redundant diagonal facet far from the polytope.
    let n_diag = Vector4::new(1.0, 1.0, 0.0, 0.0).normalize();
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        n_diag / 10.0, // x+y <= sqrt(2)*10 -- never active on [-1,1]^4
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the added diagonal facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

/// Verify Polytope4D::new rejects a nearly-parallel far-out redundant facet.
#[test]
fn reject_redundant_nearly_parallel_facet() {
    // Hypercube + a nearly parallel facet far from the polytope.
    let n_tilted = Vector4::new(1.0, 0.001, 0.0, 0.0).normalize();
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        n_tilted / 100.0, // nearly +x, far out -- redundant
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the nearly-parallel far facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

// ---- Positive tests: valid inputs are accepted ----

/// Verify a valid 5-facet simplex is accepted.
#[test]
fn simplex_accepted() {
    let halfspaces = simplex_halfspaces();
    let p = Polytope4D::new(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 5);
}

/// Verify a valid 8-facet hypercube is accepted.
#[test]
fn hypercube_accepted() {
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let p = Polytope4D::new(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 8);
}

/// Verify from_normals_and_heights accepts a valid hypercube.
#[test]
fn from_normals_and_heights_accepted() {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![1.0; 8];
    let p = Polytope4D::from_normals_and_heights(normals, heights).unwrap();
    assert_eq!(p.facet_count(), 8);
}

/// Non-simple polytopes (where more than 4 facets meet at a vertex)
/// should be accepted. The crosspolytope is a canonical example.
#[test]
fn non_simple_polytope_accepted() {
    let p = crate::geom::known_polytopes::crosspolytope().polytope;
    assert_eq!(p.facet_count(), 16);
    assert!(!p.vertices_f64().is_empty());
}
