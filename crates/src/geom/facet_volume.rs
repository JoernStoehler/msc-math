//! Per-facet volume and centroid computation for 4D polytopes.
//!
//! Provides 3D volumes of individual facets (3-polytopes embedded in R^4)
//! by decomposing each facet into tetrahedra via ridge triangulation.
//! Used by derivative computation: ∂vol/∂h_k = facet_volume_3d(k).
//!
//! **Algorithm:** For each facet F_i, collect vertices on F_i, then for each
//! ridge F_i ∩ F_j, triangulate the ridge polygon and form tetrahedra with
//! the facet centroid as apex. Sum tetrahedron volumes via the 4D cross product.
//!
//! Mathematical correspondence: [def:volume] (per-facet specialization)

use crate::geom::cross_product_4d::cross_product_4d;
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

/// Tolerance for vertex-on-facet incidence test: |n·v − h| < EPS.
const EPS_FACET_INCIDENCE: f64 = 1e-8;

/// Tolerance for detecting degenerate (collinear) polygon vertices
/// during angular sorting.
const EPS_DEGENERATE: f64 = 1e-10;

/// Sort vertices of a convex polygon embedded in R^4 by angle around their centroid.
///
/// Projects vertices onto a 2D basis in the polygon plane and sorts by atan2 angle.
/// Returns the original vertices (unsorted) if < 4 vertices or if the polygon
/// is degenerate (collinear).
pub fn sort_polygon_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    if vertices.len() <= 3 {
        return vertices.to_vec();
    }

    let n = vertices.len() as f64;
    let centroid = vertices.iter().copied().sum::<Vector4<f64>>() / n;

    let d1 = (vertices[0] - centroid).normalize();

    let d2 = match vertices.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > EPS_DEGENERATE).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return vertices.to_vec(),
    };

    let mut indexed: Vec<(f64, Vector4<f64>)> = vertices
        .iter()
        .map(|v| {
            let rel = *v - centroid;
            let angle = rel.dot(&d2).atan2(rel.dot(&d1));
            (angle, *v)
        })
        .collect();

    indexed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    indexed.into_iter().map(|(_, v)| v).collect()
}

/// Compute the 3D volume of facet `facet_idx` of a polytope.
///
/// Decomposes the facet into tetrahedra by choosing the facet centroid as apex
/// and triangulating each ridge (2-face = intersection of two facets).
/// Returns 0.0 if the facet has fewer than 4 vertices.
///
/// Used for volume derivatives: ∂vol(K)/∂h_k = facet_volume_3d(K, k).
pub fn facet_volume_3d(polytope: &Polytope4D, facet_idx: usize) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();

    facet_volume_3d_raw(&normals, &heights, &vertices, facet_idx, f)
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
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();

    facet_volume_and_centroid_3d_raw(&normals, &heights, &vertices, facet_idx, f)
}

/// Raw version of `facet_volume_3d` operating on slices.
///
/// Provided for experiments that already have normals/heights/vertices extracted.
pub fn facet_volume_3d_raw(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
    fi: usize,
    f: usize,
) -> f64 {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (normals[fi].dot(v) - heights[fi]).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return 0.0;
    }

    let centroid = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;

    (0..f)
        .filter(|&fj| fj != fi)
        .flat_map(|fj| {
            let ridge_verts: Vec<Vector4<f64>> = facet_verts
                .iter()
                .filter(|v| (normals[fj].dot(v) - heights[fj]).abs() < EPS_FACET_INCIDENCE)
                .cloned()
                .collect();

            if ridge_verts.len() < 3 {
                return Vec::new();
            }

            let sorted = sort_polygon_vertices(&ridge_verts);
            (1..sorted.len() - 1)
                .map(|k| {
                    let a = sorted[0] - centroid;
                    let b = sorted[k] - centroid;
                    let c = sorted[k + 1] - centroid;
                    cross_product_4d(a, b, c).norm() / 6.0
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

/// Raw version of `facet_volume_and_centroid_3d` operating on slices.
///
/// Provided for experiments that already have normals/heights/vertices extracted.
pub fn facet_volume_and_centroid_3d_raw(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
    fi: usize,
    f: usize,
) -> (f64, Vector4<f64>) {
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (normals[fi].dot(v) - heights[fi]).abs() < EPS_FACET_INCIDENCE)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return (0.0, Vector4::zeros());
    }

    let apex = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;

    let mut total_vol = 0.0;
    let mut weighted_centroid = Vector4::zeros();

    for fj in 0..f {
        if fj == fi {
            continue;
        }
        let ridge_verts: Vec<Vector4<f64>> = facet_verts
            .iter()
            .filter(|v| (normals[fj].dot(v) - heights[fj]).abs() < EPS_FACET_INCIDENCE)
            .cloned()
            .collect();

        if ridge_verts.len() < 3 {
            continue;
        }

        let sorted = sort_polygon_vertices(&ridge_verts);
        for k in 1..sorted.len() - 1 {
            let a = sorted[0] - apex;
            let b = sorted[k] - apex;
            let c = sorted[k + 1] - apex;
            let tet_vol = cross_product_4d(a, b, c).norm() / 6.0;
            let tet_centroid = (apex + sorted[0] + sorted[k] + sorted[k + 1]) / 4.0;
            total_vol += tet_vol;
            weighted_centroid += tet_vol * tet_centroid;
        }
    }

    if total_vol > 1e-30 {
        (total_vol, weighted_centroid / total_vol)
    } else {
        (0.0, Vector4::zeros())
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
    // Strategy: fixture-based (hypercube with known facet volumes)

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

    /// Sum of facet volumes × heights / 4 = polytope volume (divergence theorem).
    /// For [-1,1]^4: sum = 8 * (8.0 * 1.0) / 4 = 16.0.
    #[test]
    fn facet_volume_sum_equals_polytope_volume() {
        let polytope = &known_polytopes::hypercube().polytope;
        let heights = polytope.heights_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| facet_volume_3d(polytope, fi) * heights[fi])
            .sum::<f64>()
            / 4.0;

        let vol_qhull = crate::geom::volume::volume(polytope)
            .expect("qhull volume");

        assert!(
            (vol_from_facets - vol_qhull).abs() / vol_qhull < 1e-6,
            "facet sum = {vol_from_facets}, qhull = {vol_qhull}"
        );
    }

    /// Facet volume and centroid: centroid should lie on the facet hyperplane.
    #[test]
    fn facet_centroid_on_hyperplane() {
        let polytope = &known_polytopes::hypercube().polytope;
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();
        let f = polytope.facet_count();

        for fi in 0..f {
            let (vol, centroid) = facet_volume_and_centroid_3d(polytope, fi);
            assert!(vol > 0.0, "facet {fi} should have positive volume");
            let dot = normals[fi].dot(&centroid);
            assert!(
                (dot - heights[fi]).abs() < 1e-6,
                "facet {fi}: centroid not on hyperplane, n·c = {dot}, h = {}",
                heights[fi]
            );
        }
    }

    /// Cross-validate facet volumes with a non-cubic polytope.
    #[test]
    fn crosspolytope_facet_volume_sum() {
        let polytope = &known_polytopes::crosspolytope().polytope;
        let heights = polytope.heights_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| facet_volume_3d(polytope, fi) * heights[fi])
            .sum::<f64>()
            / 4.0;

        let vol_qhull = crate::geom::volume::volume(polytope)
            .expect("qhull volume");

        assert!(
            (vol_from_facets - vol_qhull).abs() / vol_qhull < 1e-4,
            "facet sum = {vol_from_facets}, qhull = {vol_qhull}"
        );
    }
}
