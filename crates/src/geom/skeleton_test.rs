//! Tests for skeleton: face lattice construction and facet_centroid.
//!
//! Proposition: The skeleton correctly computes edges, ridges, and vertex-facet
//! incidence from the exact rational incidence matrix.
//! Reference: [def:face-lattice]
//!
//! Strategy: fixture-based on known polytopes with analytically known f-vectors.

use crate::geom::known_polytopes;
use crate::geom::skeleton::Skeleton;

/// 4-simplex: 5 vertices, 10 edges, 10 triangular ridges, 5 facets.
/// Every vertex lies on exactly 4 facets.
#[test]
fn simplex_f_vector() {
    let kp = known_polytopes::simplex();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices_f64().len(), 5, "vertices");
    assert_eq!(skel.edges.len(), 10, "edges");
    assert_eq!(skel.ridges.len(), 10, "ridges");

    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 3, "simplex ridge is a triangle");
    }
    for vf in &skel.vertex_facets {
        assert_eq!(vf.len(), 4, "simplex vertex on 4 facets");
    }
}

/// Hypercube [-1,1]^4: 16 vertices, 32 edges, 24 square ridges, 8 facets.
/// Every vertex lies on exactly 4 facets.
#[test]
fn hypercube_f_vector() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices_f64().len(), 16, "vertices");
    assert_eq!(skel.edges.len(), 32, "edges");
    assert_eq!(skel.ridges.len(), 24, "ridges");

    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 4, "hypercube ridge is a square");
    }
    for vf in &skel.vertex_facets {
        assert_eq!(vf.len(), 4, "hypercube vertex on 4 facets");
    }
}

/// 4D cross-polytope: 8 vertices, 24 edges, 32 triangular ridges, 16 facets.
#[test]
fn crosspolytope_f_vector() {
    let kp = known_polytopes::crosspolytope();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices_f64().len(), 8, "vertices");
    assert_eq!(skel.edges.len(), 24, "edges");
    assert_eq!(skel.ridges.len(), 32, "ridges");

    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 3, "crosspolytope ridge is a triangle");
    }
}

/// Lagrangian triangle product: 9 vertices from 3x3 product structure.
#[test]
fn lagrangian_triangle_product_basic() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices_f64().len(), 9, "vertices");
    assert!(!skel.edges.is_empty(), "has edges");
    assert!(!skel.ridges.is_empty(), "has ridges");

    for ridge in &skel.ridges {
        assert!(
            ridge.vertices.len() >= 3,
            "ridge has {} vertices, need >= 3",
            ridge.vertices.len()
        );
    }
}

/// Edge indices are always sorted: i < j.
#[test]
fn edges_are_sorted() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    for &[i, j] in &skel.edges {
        assert!(i < j, "edge not sorted: [{i}, {j}]");
    }
}

/// Ridge facet pairs are always sorted: facets[0] < facets[1].
#[test]
fn ridge_facets_are_sorted() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    for ridge in &skel.ridges {
        assert!(
            ridge.facets[0] < ridge.facets[1],
            "ridge facets not sorted: {:?}",
            ridge.facets
        );
    }
}

/// Ridge vertices actually lie on both facets of the ridge.
#[test]
fn ridge_vertices_on_both_facets() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let normals = kp.polytope.normals_f64();
    let heights = kp.polytope.heights_f64();
    let vertices = kp.polytope.vertices_f64();

    for ridge in &skel.ridges {
        for &vi in &ridge.vertices {
            for &fi in &ridge.facets {
                let residual = (normals[fi].dot(&vertices[vi]) - heights[fi]).abs();
                assert!(
                    residual < 1e-7,
                    "vertex {vi} not on facet {fi}: residual {residual}"
                );
            }
        }
    }
}

/// facet_centroid returns a point that lies on the facet hyperplane.
#[test]
fn facet_centroid_on_facet() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let normals = kp.polytope.normals_f64();
    let heights = kp.polytope.heights_f64();

    for fi in 0..normals.len() {
        let centroid = skel.facet_centroid(&kp.polytope, fi);
        let residual = (normals[fi].dot(&centroid) - heights[fi]).abs();
        assert!(
            residual < 1e-7,
            "centroid of facet {fi} not on facet: residual {residual}"
        );
    }
}

/// facet_centroid returns a point inside the polytope (satisfies all halfspaces).
#[test]
fn facet_centroid_inside_polytope() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
        ("crosspolytope", known_polytopes::crosspolytope()),
    ];

    for (name, kp) in &polytopes {
        let skel = Skeleton::compute(&kp.polytope);
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();

        for fi in 0..normals.len() {
            let centroid = skel.facet_centroid(&kp.polytope, fi);
            for (fk, (nk, hk)) in normals.iter().zip(heights.iter()).enumerate() {
                let violation = nk.dot(&centroid) - hk;
                assert!(
                    violation < 1e-6,
                    "{name} facet {fi} centroid violates facet {fk} by {violation:.2e}"
                );
            }
        }
    }
}
