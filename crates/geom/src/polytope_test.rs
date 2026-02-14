use super::*;
use nalgebra::Vector4;

fn unit_normals_5() -> Vec<Vector4<f64>> {
    vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ]
}

#[test]
fn valid_construction() {
    let normals = unit_normals_5();
    let heights = vec![1.0; 5];
    let p = Polytope4D::new(normals, heights).unwrap();
    assert_eq!(p.facet_count(), 5);
    assert_eq!(p.normals().len(), 5);
    assert_eq!(p.heights().len(), 5);
    assert!(!p.vertices().is_empty(), "vertices should be precomputed");
}

#[test]
fn reject_duplicate_normals() {
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

#[test]
fn reject_length_mismatch() {
    let normals = unit_normals_5();
    let heights = vec![1.0; 4]; // one too few
    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(
        err,
        ConstructionError::LengthMismatch {
            normals: 5,
            heights: 4
        }
    );
}

#[test]
fn reject_too_few_facets() {
    let normals = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    let heights = vec![1.0; 4];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(4));
}

#[test]
fn reject_non_unit_normal() {
    let mut normals = unit_normals_5();
    normals[2] = Vector4::new(0.0, 0.0, 2.0, 0.0); // not unit
    let heights = vec![1.0; 5];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::NonUnitNormal { index: 2, .. } => {}
        other => panic!("expected NonUnitNormal at index 2, got {other:?}"),
    }
}

#[test]
fn reject_negative_height() {
    let normals = unit_normals_5();
    let heights = vec![1.0, 1.0, -0.5, 1.0, 1.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::NonPositiveHeight { index: 2, .. } => {}
        other => panic!("expected NonPositiveHeight at index 2, got {other:?}"),
    }
}

#[test]
fn reject_zero_height() {
    let normals = unit_normals_5();
    let heights = vec![1.0, 0.0, 1.0, 1.0, 1.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::NonPositiveHeight { index: 1, .. } => {}
        other => panic!("expected NonPositiveHeight at index 1, got {other:?}"),
    }
}

// ---- Boundedness (regression: constructor must reject unbounded inputs) ----

#[test]
fn reject_unbounded() {
    // All normals point roughly in the +x direction — unbounded in -x.
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

#[test]
fn reject_unbounded_missing_one_direction() {
    // Bounded in x, y, z but not in w (no -w normal).
    let normals = vec![
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
    let heights = vec![1.0; 8];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

// ---- Irredundancy (regression: constructor must reject redundant facets) ----

#[test]
fn reject_redundant_facet() {
    // Hypercube [-1,1]^4 + one redundant diagonal facet far from the polytope.
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 0.0, 0.0).normalize(), // x+y ≤ √2·10 — never active
    ];
    let heights = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the added diagonal facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

#[test]
fn reject_redundant_parallel_facet() {
    // Hypercube + a parallel facet in the same direction as facet 0 but further out.
    // Normals are near-identical so this should hit DuplicateHalfspaces first.
    // Use a slightly tilted normal to avoid the duplicate check.
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        Vector4::new(1.0, 0.001, 0.0, 0.0).normalize(), // nearly +x, far out
    ];
    let heights = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the nearly-parallel far facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

// ---- VertexEnumerationFailed ----
// Note: this error path requires qhull to fail on a bounded set of halfspaces,
// which is hard to trigger deterministically without mocking. The path is tested
// indirectly by the qhull module's own tests for malformed inputs.

// ---- NaN/infinity rejection ----

#[test]
fn reject_nan_height() {
    let normals = unit_normals_5();
    let heights = vec![1.0, f64::NAN, 1.0, 1.0, 1.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::NonPositiveHeight { index: 1, .. } => {}
        other => panic!("expected NonPositiveHeight for NaN, got {other:?}"),
    }
}

#[test]
fn reject_infinity_height() {
    let normals = unit_normals_5();
    let heights = vec![1.0, 1.0, f64::INFINITY, 1.0, 1.0];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    match err {
        ConstructionError::NonPositiveHeight { index: 2, .. } => {}
        other => panic!("expected NonPositiveHeight for infinity, got {other:?}"),
    }
}

// ---- Positive tests ----

#[test]
fn vertices_satisfy_halfspace_inequalities() {
    let normals = unit_normals_5();
    let heights = vec![1.0; 5];
    let p = Polytope4D::new(normals, heights).unwrap();

    const EPS: f64 = 1e-8;
    for v in p.vertices() {
        for (i, (n, &h)) in p.normals().iter().zip(p.heights().iter()).enumerate() {
            let lhs = n.dot(v);
            assert!(
                lhs <= h + EPS,
                "vertex {} violates halfspace {}: {} > {}",
                v, i, lhs, h
            );
        }
    }
}
