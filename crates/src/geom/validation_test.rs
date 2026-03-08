use super::*;
use crate::constants::EPS_FACET_INCIDENCE;
use crate::geom::polytope::{ConstructionError, Polytope4D};
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

// ---- Duplicate normals (from Polytope4D::new) ----

#[test]
fn reject_exact_duplicate_normals() {
    let normals = vec![
        Vector4::x(),
        Vector4::y(),
        Vector4::z(),
        Vector4::w(),
        Vector4::x(), // duplicate of [0]
    ];
    let heights = vec![1.0; 5];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(err, ConstructionError::DuplicateHalfspaces { i: 0, j: 4 });
}

// ---- Vertex enumeration ----

#[test]
fn simplex_has_5_vertices() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = Polytope4D::new(normals, heights).expect("valid polytope");
    assert_eq!(polytope.vertices().len(), 5);
}

#[test]
fn hypercube_has_16_vertices() {
    let (normals, heights) = hypercube_normals_heights();
    let polytope = Polytope4D::new(normals, heights).expect("valid polytope");
    assert_eq!(polytope.vertices().len(), 16);
}

#[test]
fn simplex_vertices_satisfy_constraints() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = Polytope4D::new(normals.clone(), heights.clone()).expect("valid polytope");
    for v in polytope.vertices() {
        for (n, &h) in normals.iter().zip(&heights) {
            assert!(
                n.dot(v) <= h + EPS_FACET_INCIDENCE,
                "vertex {v:?} violates constraint n·v = {} > h = {}",
                n.dot(v),
                h
            );
        }
    }
}

// ---- Full construction pipeline ----

#[test]
fn simplex_construction_has_5_facets() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = Polytope4D::new(normals, heights).expect("valid polytope");
    assert_eq!(polytope.facet_count(), 5);
}

#[test]
fn hypercube_construction_has_8_facets() {
    let (normals, heights) = hypercube_normals_heights();
    let polytope = Polytope4D::new(normals, heights).expect("valid polytope");
    assert_eq!(polytope.facet_count(), 8);
}

