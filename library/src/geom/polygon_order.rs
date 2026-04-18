//! Internal ridge-polygon ordering helper for 4D geometry.
//!
//! Shared by facet-volume and skeleton code to keep the angle-sorting logic
//! in one place while leaving each caller free to preserve its own base-case
//! behavior.

use nalgebra::Vector4;

/// Threshold for degenerate first basis vector in polygon vertex sorting.
const EPS_BASIS_DEGENERATE: f64 = 1e-12;

/// Threshold for detecting collinear vertices in polygon vertex sorting.
const EPS_COLLINEAR: f64 = 1e-10;

/// Return the vertex order for a convex polygon embedded in `R^4`.
///
/// The returned indices refer to the input slice. Degenerate configurations
/// return `None` so callers can preserve their own fallback behavior.
pub(crate) fn sort_polygon_order(vertices: &[Vector4<f64>]) -> Option<Vec<usize>> {
    if vertices.len() < 3 {
        return Some((0..vertices.len()).collect());
    }

    let centroid = vertices.iter().copied().sum::<Vector4<f64>>() / vertices.len() as f64;
    let d1_raw = vertices[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < EPS_BASIS_DEGENERATE {
        return None;
    }
    let d1 = d1_raw / d1_norm;

    let d2 = vertices.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > EPS_COLLINEAR).then(|| proj.normalize())
    })?;

    let mut indexed: Vec<(f64, usize)> = vertices
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let rel = *v - centroid;
            let angle = rel.dot(&d2).atan2(rel.dot(&d1));
            (angle, idx)
        })
        .collect();

    indexed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Some(indexed.into_iter().map(|(_, idx)| idx).collect())
}
