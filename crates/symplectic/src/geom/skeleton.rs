//! Face lattice (combinatorial skeleton) of a 4D convex polytope.
//!
//! Computes the full face structure from exact vertex-facet incidence:
//! - 0-faces (vertices): stored on [`Polytope4D`]
//! - 1-faces (edges): vertex pairs sharing >= 3 common facets
//! - 2-faces (ridges): facet pairs sharing >= 3 vertices
//! - 3-faces (facets): stored on [`Polytope4D`] as dual vertices
//!
//! The skeleton is NOT stored on `Polytope4D` to keep that struct lightweight.
//! Compute on demand via [`Skeleton::compute`].
//!
//! Mathematical correspondence: [def:face-lattice]

use crate::geom::polygon_order::sort_polygon_order;
use crate::geom::polytope::Polytope4D;
use euclidean_polytopes::{
    edges_from_incidence, two_faces_from_incidence, vertex_facets_from_incidence,
};
use nalgebra::Vector4;

/// Full combinatorial skeleton of a 4D polytope.
///
/// Contains vertex-facet incidence, edges, and ridges. Computed from the
/// exact rational incidence matrix on [`Polytope4D`].
#[derive(Clone, Debug)]
pub struct Skeleton {
    /// `vertex_facets[v]` = sorted facet indices incident to vertex `v`.
    pub vertex_facets: Vec<Vec<usize>>,

    /// Edges as pairs `[i, j]` of vertex indices with `i < j`.
    pub edges: Vec<[usize; 2]>,

    /// 2-faces (ridges): each is the intersection of two facets.
    pub ridges: Vec<Ridge>,
}

/// A 2-dimensional face (ridge) of a 4D polytope.
///
/// The intersection of two facets, forming a convex polygon in R^4.
/// Vertices are sorted into convex polygon order within their 2D affine hull.
#[derive(Clone, Debug)]
pub struct Ridge {
    /// Two facet indices with `facets[0] < facets[1]`.
    pub facets: [usize; 2],

    /// Vertex indices forming the polygon boundary, in convex order.
    pub vertices: Vec<usize>,
}

impl Skeleton {
    /// Compute the skeleton from exact vertex-facet incidence.
    ///
    /// Uses the rational incidence matrix from [`Polytope4D::incidence`] rather
    /// than floating-point tolerance checks, so the combinatorial structure is
    /// exact.
    ///
    /// Complexity: O(V^2 F) for edges, O(F^2 V) for ridges.
    /// Fine for our polytopes (V <= 200, F <= 16).
    pub fn compute(polytope: &Polytope4D) -> Self {
        let vertices = polytope.vertices_f64();
        let incidence = polytope.incidence();

        let vertex_facets = vertex_facets_from_incidence(incidence);
        let edges = edges_from_incidence(incidence);
        let ridges = two_faces_from_incidence(incidence)
            .into_iter()
            .map(|two_face| Ridge {
                facets: two_face.facets,
                vertices: sort_polygon_vertices(vertices, &two_face.vertices),
            })
            .collect();

        Self {
            vertex_facets,
            edges,
            ridges,
        }
    }

    /// Compute the centroid of a facet's vertices.
    ///
    /// Returns the arithmetic mean of all vertices incident to the given facet.
    /// Useful as a starting point for Reeb trajectory simulation.
    ///
    /// # Panics
    ///
    /// Returns the zero vector if the facet has no vertices (should not happen
    /// for a valid polytope).
    pub fn facet_centroid(&self, polytope: &Polytope4D, facet: usize) -> Vector4<f64> {
        let vertices = polytope.vertices_f64();
        let facet_verts: Vec<usize> = self
            .vertex_facets
            .iter()
            .enumerate()
            .filter_map(|(vi, facets)| facets.contains(&facet).then_some(vi))
            .collect();

        if facet_verts.is_empty() {
            return Vector4::zeros();
        }

        facet_verts
            .iter()
            .map(|&vi| vertices[vi])
            .sum::<Vector4<f64>>()
            / facet_verts.len() as f64
    }
}

/// Sort vertex indices into convex polygon order within their 2D affine hull.
///
/// Algorithm: compute the centroid, build a 2D orthonormal basis via
/// Gram-Schmidt on the offset vectors, then sort by polar angle.
fn sort_polygon_vertices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() <= 2 {
        return indices.to_vec();
    }

    let coords: Vec<Vector4<f64>> = indices.iter().map(|&i| all_vertices[i]).collect();
    match sort_polygon_order(&coords) {
        Some(order) => order.into_iter().map(|pos| indices[pos]).collect(),
        None => indices.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    // Tests for skeleton: face lattice construction and facet_centroid.
    //
    // Proposition: The skeleton correctly computes edges, ridges, and vertex-facet
    // incidence from the exact rational incidence matrix.
    // Reference: [def:face-lattice]
    //
    // Strategy: fixture-based on known polytopes with analytically known f-vectors.

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
        let duals = kp.polytope.dual_vertices_f64();
        let vertices = kp.polytope.vertices_f64();

        for ridge in &skel.ridges {
            for &vi in &ridge.vertices {
                for &fi in &ridge.facets {
                    let residual = (duals[fi].dot(&vertices[vi]) - 1.0).abs();
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
        let duals = kp.polytope.dual_vertices_f64();

        for (fi, dual) in duals.iter().enumerate() {
            let centroid = skel.facet_centroid(&kp.polytope, fi);
            let residual = (dual.dot(&centroid) - 1.0).abs();
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
            let duals = kp.polytope.dual_vertices_f64();

            for fi in 0..duals.len() {
                let centroid = skel.facet_centroid(&kp.polytope, fi);
                for (fk, ak) in duals.iter().enumerate() {
                    let violation = ak.dot(&centroid) - 1.0;
                    assert!(
                        violation < 1e-6,
                        "{name} facet {fi} centroid violates facet {fk} by {violation:.2e}"
                    );
                }
            }
        }
    }
}
