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
// Note: this error wraps rational pipeline failures (e.g. NoVertices from
// inconsistent halfspaces). Deterministic tests exist in rational_test.rs.

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

// ---- Vertex ordering invariant ----

/// Verify that vertices[i] is incident to exactly the facets in vertex_descriptors[i].
///
/// This is the core invariant of vertex-descriptor alignment: after construction,
/// the f64 vertex at index i must lie on the facets listed in the exact
/// combinatorial data at the same index.
///
/// **Why debug mode:** Fast (F ≤ 10), exercises vertex pipeline with bounds checks.
/// **Why these inputs:** All known polytopes cover simplex (F=5), hypercube (F=8),
/// crosspolytope (F=16), pentagon (F=10), and Lagrangian/symplectic products (F=7-8).
#[test]
fn vertex_ordering_matches_exact_descriptors() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        let p = &kp.polytope;
        let exact = p.exact_data();

        assert_eq!(
            p.vertices().len(),
            exact.vertex_descriptors.len(),
            "{}: vertex count mismatch",
            kp.name
        );

        for (vi, (vertex, descriptor)) in p
            .vertices()
            .iter()
            .zip(exact.vertex_descriptors.iter())
            .enumerate()
        {
            // Vertex must lie ON each facet in its descriptor
            for &fi in descriptor {
                let residual = (p.normals()[fi].dot(vertex) - p.heights()[fi]).abs();
                assert!(
                    residual < EPS_FACET_INCIDENCE,
                    "{}: vertex {} should be on facet {} but residual = {:.2e}",
                    kp.name, vi, fi, residual
                );
            }

            // Vertex must be strictly inside each facet NOT in its descriptor
            for fi in 0..p.facet_count() {
                if !descriptor.contains(&fi) {
                    let slack = p.heights()[fi] - p.normals()[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{}: vertex {} should be interior to facet {} but slack = {:.2e}",
                        kp.name, vi, fi, slack
                    );
                }
            }
        }
    }
}

/// Verify vertex ordering invariant for the `from_rational()` construction path.
///
/// `from_rational()` uses `new_with_exact_data()` with pre-computed rational
/// vertices converted to f64. This test constructs a polytope via the rational
/// path and checks the same vertex-descriptor alignment invariant.
///
/// **Why debug mode:** Small polytopes (F=5, F=8), fast.
/// **Why these inputs:** Simplex and hypercube exercise both simple (4 facets per
/// vertex) and axis-aligned (4 facets per vertex) cases.
#[test]
fn vertex_ordering_via_from_rational() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::rational::RationalPolytope4D;
    use crate::geom::known_polytopes;

    // Test with simplex (F=5) and hypercube (F=8)
    for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let orig = &kp.polytope;

        // Build rational representation from f64, then convert back via from_rational()
        let rp = RationalPolytope4D::from_f64(orig.normals(), orig.heights())
            .expect("rational construction should succeed");
        let p = Polytope4D::from_rational(&rp)
            .expect("from_rational should succeed");
        let exact = p.exact_data();

        assert_eq!(
            p.vertices().len(),
            exact.vertex_descriptors.len(),
            "{} (from_rational): vertex count mismatch",
            kp.name
        );

        for (vi, (vertex, descriptor)) in p
            .vertices()
            .iter()
            .zip(exact.vertex_descriptors.iter())
            .enumerate()
        {
            for &fi in descriptor {
                let residual = (p.normals()[fi].dot(vertex) - p.heights()[fi]).abs();
                assert!(
                    residual < EPS_FACET_INCIDENCE,
                    "{} (from_rational): vertex {} should be on facet {} but residual = {:.2e}",
                    kp.name, vi, fi, residual
                );
            }

            for fi in 0..p.facet_count() {
                if !descriptor.contains(&fi) {
                    let slack = p.heights()[fi] - p.normals()[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{} (from_rational): vertex {} should be interior to facet {} but slack = {:.2e}",
                        kp.name, vi, fi, slack
                    );
                }
            }
        }
    }
}
