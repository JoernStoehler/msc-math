//! Tests for polytope: construction, accessors, and invariants.
//!
//! Proposition: Polytope4D construction validates inputs (nonzero, non-duplicate,
//! bounded, irredundant) and produces consistent incidence/adjacency/omega data.
//! Reference: [def:polytope-dual], [def:polar-body]
//!
//! Strategy: fixture-based (simplex, hypercube, known polytopes)

use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

/// 5 halfspaces forming a simplex-like polytope. a_i = n_i/h_i with h_i = 1.
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
                v,
                i,
                lhs,
                h
            );
        }
    }
}

/// Verify that the incidence matrix is consistent with f64 vertex positions.
///
/// For each vertex v and facet f: if incidence[v,f] is true, the f64 vertex
/// must lie on that facet (within tolerance). If false, strictly interior.
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

            for fi in 0..p.facet_count() {
                if incidence[(vi, fi)] {
                    let residual =
                        (p.normals_f64()[fi].dot(vertex) - p.heights_f64()[fi]).abs();
                    assert!(
                        residual < EPS_FACET_INCIDENCE,
                        "{}: vertex {} should be on facet {} but residual = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        residual
                    );
                } else {
                    let slack = p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
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

/// Verify vertex ordering invariant via the from_rationals() construction path.
#[test]
fn vertex_ordering_via_from_rationals() {
    use crate::constants::EPS_FACET_INCIDENCE;
    use crate::geom::known_polytopes;
    use crate::geom::rational_arithmetic;

    for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let orig = &kp.polytope;

        let rational_normals: Vec<[num_rational::BigRational; 4]> = orig
            .normals_f64()
            .iter()
            .map(|n| std::array::from_fn(|i| rational_arithmetic::f64_to_rational(n[i])))
            .collect();
        let rational_heights: Vec<num_rational::BigRational> = orig
            .heights_f64()
            .iter()
            .map(|&h| rational_arithmetic::f64_to_rational(h))
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
                        "{} (from_rationals): vertex {} on facet {} residual = {:.2e}",
                        kp.name,
                        vi,
                        fi,
                        residual
                    );
                } else {
                    let slack = p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
                    assert!(
                        slack > EPS_FACET_INCIDENCE,
                        "{} (from_rationals): vertex {} interior to facet {} slack = {:.2e}",
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

/// Adjacency matrix should be symmetric and have no self-adjacency.
#[test]
fn adjacency_matrix_symmetric_no_self_loops() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        let p = &kp.polytope;
        let adj = p.adjacency();
        let f = p.facet_count();

        for i in 0..f {
            assert!(
                !adj[(i, i)],
                "{}: facet {} is self-adjacent",
                kp.name,
                i
            );
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

/// Omega signs matrix should be antisymmetric with zero diagonal.
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

/// The dual vertices accessor returns the right count and nonzero vectors.
#[test]
fn dual_vertices_count_and_nonzero() {
    let halfspaces = simplex_halfspaces_5();
    let p = Polytope4D::new(halfspaces).unwrap();

    assert_eq!(p.dual_vertices_f64().len(), 5);
    for (i, dv) in p.dual_vertices_f64().iter().enumerate() {
        assert!(
            dv.norm() > 1e-10,
            "dual vertex[{i}] should be nonzero: {:?}",
            dv
        );
    }
}

/// Heights are positive for bounded polytopes.
#[test]
fn heights_positive() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        for (i, &h) in kp.polytope.heights_f64().iter().enumerate() {
            assert!(
                h > 0.0,
                "{}: height[{i}] should be positive, got {h}",
                kp.name
            );
        }
    }
}

/// Normals are unit vectors.
#[test]
fn normals_are_unit() {
    use crate::geom::known_polytopes;

    for kp in known_polytopes::all_known() {
        for (i, n) in kp.polytope.normals_f64().iter().enumerate() {
            assert!(
                (n.norm() - 1.0).abs() < 1e-10,
                "{}: normal[{i}] should be unit, norm = {}",
                kp.name,
                n.norm()
            );
        }
    }
}
