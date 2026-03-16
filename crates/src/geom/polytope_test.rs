use super::*;
use nalgebra::Vector4;

/// 5 halfspaces forming a simplex-like polytope. aᵢ = nᵢ/hᵢ with hᵢ = 1.
fn simplex_halfspaces_5() -> Vec<Vector4<f64>> {
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
    let halfspaces = simplex_halfspaces_5();
    let p = Polytope4D::new(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 5);
    assert_eq!(p.normals_f64().len(), 5);
    assert_eq!(p.heights_f64().len(), 5);
    assert!(!p.vertices_f64().is_empty(), "vertices should be precomputed");
}

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

#[test]
fn reject_too_few_facets() {
    let halfspaces = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(4));
}

#[test]
fn reject_zero_halfspace() {
    let mut halfspaces = simplex_halfspaces_5();
    halfspaces[2] = Vector4::new(0.0, 0.0, 0.0, 0.0); // zero vector
    let err = Polytope4D::new(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::ZeroDualVertex(2));
}

// ---- Boundedness (regression: constructor must reject unbounded inputs) ----

#[test]
fn reject_unbounded() {
    // All halfspaces point roughly in the +x direction — unbounded in -x.
    // (Unit normals with height 1.0 → halfspaces = normals.)
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

#[test]
fn reject_unbounded_missing_one_direction() {
    // Bounded in x, y, z but not in w (no -w halfspace).
    // (Unit normals with height 1.0 → halfspaces = normals.)
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

// ---- Irredundancy (regression: constructor must reject redundant facets) ----

#[test]
fn reject_redundant_facet() {
    // Hypercube [-1,1]^4 + one redundant diagonal facet far from the polytope.
    // Cube halfspaces: ±eᵢ/1 = ±eᵢ. Redundant: n/h = normalize(1,1,0,0)/10.
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
        n_diag / 10.0, // x+y ≤ √2·10 — never active
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
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
    // Use a slightly tilted normal to avoid the duplicate check.
    // n/h with h=100 → very small halfspace vector (far-out facet).
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
        n_tilted / 100.0, // nearly +x, far out → redundant
    ];
    let err = Polytope4D::new(halfspaces).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the nearly-parallel far facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

// ---- Rational pipeline errors (ZeroDualVertex, NoVertices, F64Conversion) ----
// Tested directly in rational_test.rs via from_rationals() constructor.

// ---- Positive tests ----

#[test]
fn vertices_satisfy_halfspace_inequalities() {
    let halfspaces = simplex_halfspaces_5();
    let p = Polytope4D::new(halfspaces).unwrap();

    const EPS: f64 = 1e-8;
    for v in p.vertices_f64() {
        for (i, (n, &h)) in p.normals_f64().iter().zip(p.heights_f64().iter()).enumerate() {
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

/// Verify that the incidence matrix is consistent with f64 vertex positions.
///
/// For each vertex v and facet f: if incidence[v,f] is true, the f64 vertex
/// must lie on that facet (within tolerance). If false, it must be strictly
/// interior.
#[test]
fn vertex_ordering_matches_incidence() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        let p = &kp.polytope;
        let incidence = p.incidence();
        let v_count = p.vertices_f64().len();

        assert_eq!(
            incidence.nrows(),
            v_count,
            "{}: incidence row count mismatch",
            kp.name
        );

        for vi in 0..v_count {
            let vertex = &p.vertices_f64()[vi];

            // Vertex must lie ON each facet where incidence is true
            for fi in 0..p.facet_count() {
                if incidence[(vi, fi)] {
                    let residual =
                        (p.normals_f64()[fi].dot(vertex) - p.heights_f64()[fi]).abs();
                    assert!(
                        residual < EPS_FACET_INCIDENCE,
                        "{}: vertex {} should be on facet {} but residual = {:.2e}",
                        kp.name, vi, fi, residual
                    );
                } else {
                    let slack =
                        p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
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

/// Verify vertex ordering invariant for the `from_rationals()` construction path.
///
/// Constructs a polytope via the rational path (from_rationals) and checks
/// the same vertex-incidence alignment invariant.
#[test]
fn vertex_ordering_via_from_rationals() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::known_polytopes;

    // Test with simplex (F=5) and hypercube (F=8)
    for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let orig = &kp.polytope;

        // Build via from_rationals using the dual vertices' rational data
        // Re-derive rational normals and heights from the original f64 data
        let rational_normals: Vec<[num_rational::BigRational; 4]> = orig
            .normals_f64()
            .iter()
            .map(|n| {
                std::array::from_fn(|i| {
                    crate::geom::rational::f64_to_rational(n[i])
                })
            })
            .collect();
        let rational_heights: Vec<num_rational::BigRational> = orig
            .heights_f64()
            .iter()
            .map(|&h| crate::geom::rational::f64_to_rational(h))
            .collect();

        let p = Polytope4D::from_rationals(rational_normals, rational_heights)
            .expect("from_rationals should succeed");
        let incidence = p.incidence();

        assert_eq!(
            p.vertices_f64().len(),
            incidence.nrows(),
            "{} (from_rationals): vertex count mismatch",
            kp.name
        );

        for vi in 0..p.vertices_f64().len() {
            let vertex = &p.vertices_f64()[vi];
            for fi in 0..p.facet_count() {
                if incidence[(vi, fi)] {
                    let residual =
                        (p.normals_f64()[fi].dot(vertex) - p.heights_f64()[fi]).abs();
                    assert!(
                        residual < EPS_FACET_INCIDENCE,
                        "{} (from_rationals): vertex {} should be on facet {} but residual = {:.2e}",
                        kp.name, vi, fi, residual
                    );
                } else {
                    let slack =
                        p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{} (from_rationals): vertex {} should be interior to facet {} but slack = {:.2e}",
                        kp.name, vi, fi, slack
                    );
                }
            }
        }
    }
}
