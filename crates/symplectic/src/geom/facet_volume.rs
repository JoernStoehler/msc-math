//! Per-facet volume and centroid computation for 4D polytopes.
//!
//! Provides 3D volumes of individual facets (3-polytopes embedded in R^4)
//! by decomposing each facet into tetrahedra via ridge triangulation.
//! Used by derivative computation: ∂vol/∂a_k uses facet_volume_3d(k).
//!
//! **Algorithm:** For each facet F_i, collect vertices on F_i, then for each
//! ridge F_i ∩ F_j, triangulate the ridge polygon and form tetrahedra with
//! the facet centroid as apex. Sum tetrahedron volumes via the 4D cross product.
//!
//! Mathematical correspondence: [def:volume] (per-facet specialization)

use crate::geom::cross_product_4d::cross_product_4d;
use crate::geom::polygon_order::sort_polygon_order;
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

/// Tolerance for vertex-on-facet incidence test: |a·v − 1| < EPS.
const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// Floor for meaningful facet volume. Facets with total volume below this
/// are treated as degenerate (zero volume, zero centroid). Prevents
/// division by near-zero in centroid computation. Value is far below
/// any real facet volume (O(0.01)–O(100)) but above f64 underflow.
pub(crate) const EPS_VOLUME_FLOOR: f64 = 1e-30;

/// Sort vertices of a convex polygon embedded in R^4 by angle around their centroid.
///
/// Projects vertices onto a 2D basis in the polygon plane and sorts by atan2 angle.
/// Returns the original vertices (unsorted) if < 4 vertices or if the polygon
/// is degenerate (collinear).
fn sort_polygon_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    if vertices.len() <= 3 {
        return vertices.to_vec();
    }

    match sort_polygon_order(vertices) {
        Some(order) => order.into_iter().map(|idx| vertices[idx]).collect(),
        None => vertices.to_vec(),
    }
}

/// Compute the 3D volume of facet `facet_idx` of a polytope.
///
/// Decomposes the facet into tetrahedra by choosing the facet centroid as apex
/// and triangulating each ridge (2-face = intersection of two facets).
/// Returns 0.0 if the facet has fewer than 4 vertices.
///
/// Used for volume derivatives: ∂vol(K)/∂a_k uses facet_volume_3d(K, k).
pub fn facet_volume_3d(polytope: &Polytope4D, facet_idx: usize) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();

    facet_volume_3d_raw(duals, vertices, facet_idx)
}

/// Compute the 3D volume and area-weighted centroid of facet `facet_idx`.
///
/// Returns (volume, centroid). The centroid is the volume-weighted average
/// of the tetrahedra centroids. Returns (0.0, zero vector) if the facet
/// has fewer than 4 vertices.
pub fn facet_volume_and_centroid_3d(
    polytope: &Polytope4D,
    facet_idx: usize,
) -> (f64, Vector4<f64>) {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();

    facet_volume_and_centroid_3d_raw(duals, vertices, facet_idx)
}

/// Raw version of `facet_volume_3d` operating on slices.
///
/// Provided for experiments that already have dual vertices/vertices extracted.
pub fn facet_volume_3d_raw(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
    fi: usize,
) -> f64 {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (dual_vertices[fi].dot(v) - 1.0).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return 0.0;
    }

    let mut total = 0.0;

    for_each_facet_ridge_triangle(dual_vertices, &facet_verts, fi, |apex, a, b, c| {
        total += cross_product_4d(a - apex, b - apex, c - apex).norm() / 6.0;
    });

    total
}

/// Raw version of `facet_volume_and_centroid_3d` operating on slices.
///
/// Provided for experiments that already have dual vertices/vertices extracted.
pub fn facet_volume_and_centroid_3d_raw(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
    fi: usize,
) -> (f64, Vector4<f64>) {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (dual_vertices[fi].dot(v) - 1.0).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return (0.0, Vector4::zeros());
    }

    let mut total_vol = 0.0;
    let mut weighted_centroid = Vector4::zeros();

    for_each_facet_ridge_triangle(dual_vertices, &facet_verts, fi, |apex, a, b, c| {
        let tet_vol = cross_product_4d(a - apex, b - apex, c - apex).norm() / 6.0;
        let tet_centroid = (apex + a + b + c) / 4.0;
        total_vol += tet_vol;
        weighted_centroid += tet_vol * tet_centroid;
    });

    if total_vol > EPS_VOLUME_FLOOR {
        (total_vol, weighted_centroid / total_vol)
    } else {
        (0.0, Vector4::zeros())
    }
}

/// Visit each tetrahedron in the triangle fan induced by the facet's ridges.
fn for_each_facet_ridge_triangle(
    dual_vertices: &[Vector4<f64>],
    facet_verts: &[Vector4<f64>],
    fi: usize,
    mut visit: impl FnMut(Vector4<f64>, Vector4<f64>, Vector4<f64>, Vector4<f64>),
) {
    let apex = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;

    for (fj, a_j) in dual_vertices.iter().enumerate() {
        if fj == fi {
            continue;
        }
        let ridge_verts: Vec<Vector4<f64>> = facet_verts
            .iter()
            .filter(|v| (a_j.dot(v) - 1.0).abs() < EPS_FACET_INCIDENCE)
            .cloned()
            .collect();

        if ridge_verts.len() < 3 {
            continue;
        }

        let sorted = sort_polygon_vertices(&ridge_verts);
        for k in 1..sorted.len() - 1 {
            visit(apex, sorted[0], sorted[k], sorted[k + 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    // Tests for facet volume: per-facet 3D volumes of 4D polytope facets.
    //
    // Proposition: facet_volume_3d computes correct 3D volumes for facets of
    // known polytopes. For the hypercube [-1,1]^4, each facet is a cube [-1,1]^3
    // with volume 8.
    // Reference: [def:volume] (per-facet specialization)
    //
    // Strategy: fixture-based (hypercube: exact volumes known;
    //   crosspolytope: divergence-theorem cross-check with qhull)

    /// Each facet of [-1,1]^4 is a cube [-1,1]^3 with volume 8.
    #[test]
    fn hypercube_facet_volumes() {
        let polytope = &known_polytopes::hypercube().polytope;
        let f = polytope.facet_count();
        assert_eq!(f, 8, "hypercube should have 8 facets");

        for fi in 0..f {
            let vol = facet_volume_3d(polytope, fi);
            assert!(
                (vol - 8.0).abs() < 1e-6,
                "facet {fi}: volume = {vol}, expected 8.0"
            );
        }
    }

    /// Sum of facet volumes × h_i / 4 = polytope volume (divergence theorem).
    /// h_i = 1/|a_i|. For [-1,1]^4: h_i = 1, sum = 8 * (8.0 * 1.0) / 4 = 16.0.
    #[test]
    fn facet_volume_sum_equals_polytope_volume() {
        let polytope = &known_polytopes::hypercube().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| facet_volume_3d(polytope, fi) * (1.0 / duals[fi].norm()))
            .sum::<f64>()
            / 4.0;

        let vol_qhull = crate::geom::volume::volume_qhull(polytope).expect("qhull volume");

        assert!(
            (vol_from_facets - vol_qhull).abs() / vol_qhull < 1e-6,
            "facet sum = {vol_from_facets}, qhull = {vol_qhull}"
        );
    }

    /// Facet volume and centroid: centroid should lie on the facet hyperplane.
    #[test]
    fn facet_centroid_on_hyperplane() {
        let polytope = &known_polytopes::hypercube().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        for (fi, dual) in duals.iter().enumerate().take(f) {
            let (vol, centroid) = facet_volume_and_centroid_3d(polytope, fi);
            assert!(vol > 0.0, "facet {fi} should have positive volume");
            let dot = dual.dot(&centroid);
            assert!(
                (dot - 1.0).abs() < 1e-6,
                "facet {fi}: centroid not on hyperplane, a·c = {dot}, expected 1.0",
            );
        }
    }

    /// Cross-validate facet volumes with a non-cubic polytope.
    #[test]
    fn crosspolytope_facet_volume_sum() {
        let polytope = &known_polytopes::crosspolytope().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| facet_volume_3d(polytope, fi) * (1.0 / duals[fi].norm()))
            .sum::<f64>()
            / 4.0;

        let vol_qhull = crate::geom::volume::volume_qhull(polytope).expect("qhull volume");

        // Looser than hypercube (1e-6) because the crosspolytope has 16 facets
        // with non-axis-aligned normals, producing more triangulation error in
        // both our ridge-based decomposition and qhull.
        assert!(
            (vol_from_facets - vol_qhull).abs() / vol_qhull < 1e-4,
            "facet sum = {vol_from_facets}, qhull = {vol_qhull}"
        );
    }
}
