//! Tests for validation: boundedness accept/reject on known polytopes.
//!
//! Proposition: check_bounded correctly classifies bounded vs unbounded normal sets.
//! Reference: [lem:positive-span]
//!
//! Strategy: fixture-based on simplex, hypercube, and adversarial unbounded configurations.

use super::validation::check_bounded;
use nalgebra::Vector4;

/// Helper: simplex normals (5 facets, origin at centroid).
fn simplex_normals() -> Vec<Vector4<f64>> {
    vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ]
}

/// Helper: hypercube [-1,1]^4 normals (8 facets).
fn hypercube_normals() -> Vec<Vector4<f64>> {
    vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ]
}

// ---- Boundedness ----

#[test]
fn simplex_is_bounded() {
    assert!(check_bounded(&simplex_normals()));
}

#[test]
fn hypercube_is_bounded() {
    assert!(check_bounded(&hypercube_normals()));
}

#[test]
fn unbounded_normals_detected() {
    // All normals point roughly in the +x direction: fails positive spanning.
    let normals = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    assert!(!check_bounded(&normals));
}

#[test]
fn rank_deficient_normals_unbounded() {
    // Only 3 linearly independent directions: rank < 4.
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
    ];
    assert!(!check_bounded(&normals));
}
