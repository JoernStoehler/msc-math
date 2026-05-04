use super::*;
use nalgebra::Vector4;

// Tests for polytope: construction, accessors, and invariants.
//
// Proposition: Polytope4D construction validates inputs (nonzero, non-duplicate,
// bounded, irredundant) and produces consistent incidence/adjacency/omega data.
// Reference: [def:polytope-dual], [def:polar-body]
//
// Strategy: fixture-based (simplex, hypercube, known polytopes)

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
    let p = Polytope4D::from_f64(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 5);
    assert_eq!(p.dual_vertices_f64().len(), 5);
    assert!(
        !p.vertices_f64().is_empty(),
        "vertices should be precomputed"
    );
}

#[test]
fn vertices_satisfy_halfspace_inequalities() {
    let halfspaces = simplex_halfspaces_5();
    let p = Polytope4D::from_f64(halfspaces).unwrap();

    const EPS: f64 = 1e-8;
    for v in p.vertices_f64() {
        for (i, a) in p.dual_vertices_f64().iter().enumerate() {
            let lhs = a.dot(v);
            assert!(
                lhs <= 1.0 + EPS,
                "vertex {} violates halfspace {}: {} > 1",
                v,
                i,
                lhs
            );
        }
    }
}

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

        let duals = p.dual_vertices_f64();
        for vi in 0..v_count {
            let vertex = &p.vertices_f64()[vi];

            for fi in 0..p.facet_count() {
                if incidence[(vi, fi)] {
                    let residual = (duals[fi].dot(vertex) - 1.0).abs();
                    assert!(
                        residual < EPS_FACET_INCIDENCE,
                        "{}: vertex {} should be on facet {} but residual = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        residual
                    );
                } else {
                    let slack = 1.0 - duals[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{}: vertex {} should be interior to facet {} but slack = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        slack
                    );
                }
            }
        }
    }
}

#[test]
fn vertex_ordering_via_rational_reconstruction() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::known_polytopes;
    use crate::geom::rational_arithmetic;

    for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let orig = &kp.polytope;

        let rational_duals: Vec<[num_rational::BigRational; 4]> = orig
            .dual_vertices_f64()
            .iter()
            .map(|a| std::array::from_fn(|i| rational_arithmetic::f64_to_rational(a[i])))
            .collect();
        let p = Polytope4D::new(rational_duals)
            .expect("rational dual vertex construction should succeed");
        let incidence = p.incidence();

        assert_eq!(
            p.vertices_f64().len(),
            incidence.nrows(),
            "{} (rational reconstruction): vertex count mismatch",
            kp.name
        );

        let duals = p.dual_vertices_f64();
        for vi in 0..p.vertices_f64().len() {
            let vertex = &p.vertices_f64()[vi];
            for fi in 0..p.facet_count() {
                if incidence[(vi, fi)] {
                    let residual = (duals[fi].dot(vertex) - 1.0).abs();
                    assert!(
                        residual < EPS_FACET_INCIDENCE,
                        "{} (rational reconstruction): vertex {} on facet {} residual = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        residual
                    );
                } else {
                    let slack = 1.0 - duals[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{} (rational reconstruction): vertex {} interior to facet {} slack = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        slack
                    );
                }
            }
        }
    }
}

#[test]
fn adjacency_matrix_symmetric_no_self_loops() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        let p = &kp.polytope;
        let adj = p.vertex_adjacency();
        let f = p.facet_count();

        for i in 0..f {
            assert!(!adj[(i, i)], "{}: facet {} is self-adjacent", kp.name, i);
            for j in (i + 1)..f {
                assert_eq!(
                    adj[(i, j)],
                    adj[(j, i)],
                    "{}: adjacency not symmetric at ({}, {})",
                    kp.name,
                    i,
                    j
                );
            }
        }
    }
}

#[test]
fn omega_signs_antisymmetric() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        let p = &kp.polytope;
        let omega = p.omega_signs();
        let f = p.facet_count();

        for i in 0..f {
            assert_eq!(
                omega[(i, i)],
                0,
                "{}: diagonal omega[{},{}] should be 0",
                kp.name,
                i,
                i
            );
            for j in (i + 1)..f {
                assert_eq!(
                    omega[(i, j)],
                    -omega[(j, i)],
                    "{}: omega not antisymmetric at ({}, {})",
                    kp.name,
                    i,
                    j
                );
            }
        }
    }
}

#[test]
fn dual_vertices_count_and_nonzero() {
    let halfspaces = simplex_halfspaces_5();
    let p = Polytope4D::from_f64(halfspaces).unwrap();

    assert_eq!(p.dual_vertices_f64().len(), 5);
    for (i, dv) in p.dual_vertices_f64().iter().enumerate() {
        assert!(
            dv.norm() > 1e-10,
            "dual vertex[{i}] should be nonzero: {:?}",
            dv
        );
    }
}

#[test]
fn dual_vertices_nonzero() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        for (i, a) in kp.polytope.dual_vertices_f64().iter().enumerate() {
            assert!(
                a.norm() > 0.0,
                "{}: dual_vertex[{i}] should be nonzero, got {:?}",
                kp.name,
                a
            );
        }
    }
}

#[test]
fn dual_vertices_normalize_to_unit() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        for (i, a) in kp.polytope.dual_vertices_f64().iter().enumerate() {
            let norm = a.norm();
            assert!(
                norm > 1e-10,
                "{}: dual_vertex[{i}] has near-zero norm = {}",
                kp.name,
                norm
            );
            let unit = a / norm;
            assert!(
                (unit.norm() - 1.0).abs() < 1e-10,
                "{}: dual_vertex[{i}] normalized norm = {}",
                kp.name,
                unit.norm()
            );
        }
    }
}

fn simplex_halfspaces() -> Vec<Vector4<f64>> {
    vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ]
}

#[test]
fn reject_too_few_facets_4() {
    let halfspaces = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(4));
}

#[test]
fn reject_too_few_facets_0() {
    let halfspaces: Vec<Vector4<f64>> = vec![];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(0));
}

#[test]
fn reject_too_few_facets_1() {
    let halfspaces = vec![Vector4::x()];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(1));
}

#[test]
fn reject_zero_halfspace() {
    let mut halfspaces = simplex_halfspaces();
    halfspaces[2] = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::ZeroDualVertex(2));
}

#[test]
fn reject_near_zero_halfspace() {
    let mut halfspaces = simplex_halfspaces();
    halfspaces[0] = Vector4::new(1e-16, 0.0, 0.0, 0.0);
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::ZeroDualVertex(0));
}

#[test]
fn reject_duplicate_halfspaces() {
    let halfspaces = vec![
        Vector4::x(),
        Vector4::y(),
        Vector4::z(),
        Vector4::w(),
        Vector4::x(),
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::DuplicateHalfspaces { i: 0, j: 4 });
}

#[test]
fn reject_unbounded_all_positive_x() {
    let halfspaces = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

#[test]
fn reject_unbounded_missing_one_direction() {
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

#[test]
fn reject_redundant_diagonal_facet() {
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
        n_diag / 10.0,
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the added diagonal facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

#[test]
fn reject_redundant_nearly_parallel_facet() {
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
        n_tilted / 100.0,
    ];
    let err = Polytope4D::from_f64(halfspaces).unwrap_err();
    match err {
        ConstructionError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "the nearly-parallel far facet should be redundant");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

#[test]
fn simplex_accepted() {
    let halfspaces = simplex_halfspaces();
    let p = Polytope4D::from_f64(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 5);
}

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
    let p = Polytope4D::from_f64(halfspaces).unwrap();
    assert_eq!(p.facet_count(), 8);
}

#[test]
fn from_f64_division_accepted() {
    let normals = [
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = [1.0; 8];
    let p = Polytope4D::from_f64(
        normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .unwrap();
    assert_eq!(p.facet_count(), 8);
}

#[test]
fn non_simple_polytope_accepted() {
    let p = &crate::geom::known_polytopes::crosspolytope().polytope;
    assert_eq!(p.facet_count(), 16);
    assert!(!p.vertices_f64().is_empty());
}

#[test]
fn from_rational_parts_matches_from_f64() {
    let halfspaces = simplex_halfspaces_5();
    let original = Polytope4D::from_f64(halfspaces).unwrap();

    let reconstructed = Polytope4D::from_rational_parts(
        original.dual_vertices().to_vec(),
        original.vertices().to_vec(),
    )
    .unwrap();

    assert_eq!(original.facet_count(), reconstructed.facet_count());
    assert_eq!(original.vertices().len(), reconstructed.vertices().len());
    assert_eq!(original.incidence(), reconstructed.incidence());
    assert_eq!(original.omega_signs(), reconstructed.omega_signs());
    assert_eq!(
        original.vertex_adjacency(),
        reconstructed.vertex_adjacency()
    );
}

#[test]
fn from_rational_parts_crosspolytope() {
    let original = &crate::geom::known_polytopes::crosspolytope().polytope;

    let reconstructed = Polytope4D::from_rational_parts(
        original.dual_vertices().to_vec(),
        original.vertices().to_vec(),
    )
    .unwrap();

    assert_eq!(original.facet_count(), reconstructed.facet_count());
    assert_eq!(original.vertices().len(), reconstructed.vertices().len());
    assert_eq!(original.incidence(), reconstructed.incidence());
    assert_eq!(original.omega_signs(), reconstructed.omega_signs());
    assert_eq!(
        original.vertex_adjacency(),
        reconstructed.vertex_adjacency()
    );
}
