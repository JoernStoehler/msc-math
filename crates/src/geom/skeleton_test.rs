use super::Skeleton;
use crate::geom::known_polytopes;

#[test]
fn simplex_skeleton() {
    // 4-simplex: 5 vertices, 10 edges, 10 triangular ridges
    let kp = known_polytopes::simplex();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices().len(), 5, "simplex vertex count");
    assert_eq!(skel.edges.len(), 10, "simplex edge count");
    assert_eq!(skel.ridges.len(), 10, "simplex ridge count");

    // Every ridge of a simplex is a triangle (3 vertices)
    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 3, "simplex ridge is triangle");
    }

    // Every vertex is on exactly 4 facets (in a 4-simplex)
    for vf in &skel.vertex_facets {
        assert_eq!(vf.len(), 4, "simplex vertex incident to 4 facets");
    }
}

#[test]
fn hypercube_skeleton() {
    // [-1,1]^4: 16 vertices, 32 edges, 24 square ridges
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices().len(), 16, "hypercube vertex count");
    assert_eq!(skel.edges.len(), 32, "hypercube edge count");
    assert_eq!(skel.ridges.len(), 24, "hypercube ridge count");

    // Every ridge of a hypercube is a square (4 vertices)
    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 4, "hypercube ridge is square");
    }

    // Every vertex of [-1,1]^4 is on exactly 4 facets
    for vf in &skel.vertex_facets {
        assert_eq!(vf.len(), 4, "hypercube vertex incident to 4 facets");
    }
}

#[test]
fn crosspolytope_skeleton() {
    // 4D cross-polytope: 8 vertices, 24 edges, 32 triangular ridges
    let kp = known_polytopes::crosspolytope();
    let skel = Skeleton::compute(&kp.polytope);

    assert_eq!(kp.polytope.vertices().len(), 8, "crosspolytope vertex count");
    assert_eq!(skel.edges.len(), 24, "crosspolytope edge count");
    assert_eq!(skel.ridges.len(), 32, "crosspolytope ridge count");

    // Every ridge of a cross-polytope is a triangle
    for ridge in &skel.ridges {
        assert_eq!(ridge.vertices.len(), 3, "crosspolytope ridge is triangle");
    }
}

#[test]
fn lagrangian_triangle_product_skeleton() {
    // Triangle ×_L Triangle: 6 facets, 9 vertices, 18 edges, 9 ridges
    // (product of two triangles in Lagrangian subspaces)
    let kp = known_polytopes::lagrangian_triangle_product();
    let skel = Skeleton::compute(&kp.polytope);

    // 3 vertices × 3 vertices = 9 vertices
    assert_eq!(kp.polytope.vertices().len(), 9, "lag tri prod vertex count");

    // Each ridge is a pair of facets sharing vertices.
    // For a product P1 ×_L P2: ridges are either (fi, fj) within P1 or P2,
    // or (fi from P1, fj from P2). We just check totals are reasonable.
    assert!(!skel.edges.is_empty(), "has edges");
    assert!(!skel.ridges.is_empty(), "has ridges");

    // Every ridge has ≥3 vertices (polygon)
    for ridge in &skel.ridges {
        assert!(
            ridge.vertices.len() >= 3,
            "ridge has {} vertices, need ≥3",
            ridge.vertices.len()
        );
    }
}

#[test]
fn edges_are_sorted() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    for &[i, j] in &skel.edges {
        assert!(i < j, "edge indices not sorted: [{i}, {j}]");
    }
}

#[test]
fn ridges_have_sorted_facets() {
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

#[test]
fn ridge_vertices_lie_on_both_facets() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let normals = kp.polytope.normals();
    let heights = kp.polytope.heights();
    let vertices = kp.polytope.vertices();

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
