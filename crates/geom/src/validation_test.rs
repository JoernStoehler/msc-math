use super::*;
use crate::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

/// Helper: build the centered simplex (5 facets, origin at centroid).
fn simplex_normals_heights() -> (Vec<Vector4<f64>>, Vec<f64>) {
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_orig = vec![0.0, 0.0, 0.0, 0.0, 1.0];
    let heights: Vec<f64> = normals
        .iter()
        .zip(&heights_orig)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();
    (normals, heights)
}

/// Helper: build the hypercube [-1,1]^4 (8 facets).
fn hypercube_normals_heights() -> (Vec<Vector4<f64>>, Vec<f64>) {
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
    (normals, heights)
}

// ---- Boundedness ----

#[test]
fn simplex_is_bounded() {
    let (normals, _) = simplex_normals_heights();
    assert!(check_bounded(&normals));
}

#[test]
fn hypercube_is_bounded() {
    let (normals, _) = hypercube_normals_heights();
    assert!(check_bounded(&normals));
}

#[test]
fn unbounded_normals_detected() {
    let normals = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    assert!(!check_bounded(&normals));
}

// ---- Irredundancy (via Polytope4D::new rejecting redundant facets) ----

#[test]
fn simplex_construction_succeeds() {
    let (normals, heights) = simplex_normals_heights();
    assert!(Polytope4D::new(normals, heights).is_ok());
}

#[test]
fn hypercube_construction_succeeds() {
    let (normals, heights) = hypercube_normals_heights();
    assert!(Polytope4D::new(normals, heights).is_ok());
}

#[test]
fn constructor_rejects_redundant_facet() {
    // Hypercube + one redundant facet with a distinct normal direction
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 0.0, 0.0).normalize(), // redundant: x+y ≤ √2·10
    ];
    let heights = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0];

    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "redundant facet should be index 8 (the added one)");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

#[test]
fn constructor_rejects_unbounded() {
    let normals = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    let heights = vec![1.0; 5];

    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

// ---- Affine rank ----

#[test]
fn affine_rank_single_point() {
    let points = vec![Vector4::new(1.0, 2.0, 3.0, 4.0)];
    assert_eq!(affine_rank(&points), 0);
}

#[test]
fn affine_rank_collinear() {
    let points = vec![
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(2.0, 0.0, 0.0, 0.0),
    ];
    assert_eq!(affine_rank(&points), 1);
}

#[test]
fn affine_rank_3d() {
    let points = vec![
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
    ];
    assert_eq!(affine_rank(&points), 3);
}
