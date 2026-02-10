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
#[ignore] // TODO: Enable when qhull implementation is complete
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

#[test]
#[ignore] // TODO: Enable when qhull implementation is complete
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
