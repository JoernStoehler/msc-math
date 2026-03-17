//! 2D convex polygon constructors for Lagrangian product experiments.
//!
//! A 2D convex polygon is represented as outward unit normals (n_k in S^1) and
//! positive heights (h_k > 0): P = { x in R^2 | n_k . x <= h_k for all k }.
//!
//! Angle convention: regular n-gon normals at angles pi/2 + 2*pi*k/n,
//! matching `known_polytopes::hko_pentagon` and `lagrangian_triangle_product`.
//!
//! Mathematical correspondence: [def:polygon-h-rep]

use nalgebra::Vector2;
use std::f64::consts::PI;

/// Threshold for treating two consecutive normals as parallel (degenerate polygon).
///
/// det(n_i, n_j) < EPS_PARALLEL means the angle between n_i and n_j is < ~1e-12 rad.
/// For unit normals this equals sin(angle), so EPS_PARALLEL ~ 1e-12 rad ~ 2.3e-13 deg.
///
/// **Why 1e-12:** Our normals are computed from f64 trig (cos/sin), which has
/// roundoff ~1e-16. A determinant below 1e-12 between two ostensibly distinct
/// normals would cause catastrophic cancellation in vertex computation (dividing
/// by near-zero det), producing wildly wrong vertices. Well-separated normals
/// (angular gap > 0.01 rad) have det ~ 0.01, safely above 1e-12. Degenerate
/// or near-degenerate polygons where two normals are within ~1e-12 rad are
/// rejected as numerically unreliable.
const EPS_PARALLEL: f64 = 1e-12;

/// Regular n-gon with circumradius R, centered at origin.
///
/// Outward unit normals at angles pi/2 + 2*pi*k/n for k = 0, ..., n-1.
/// Heights = R * cos(pi/n) (the inradius).
///
/// # Panics
///
/// Panics if n < 3 or R <= 0.
///
/// Mathematical correspondence: [def:polygon-h-rep]
pub fn regular_polygon_2d(n: usize, circumradius: f64) -> (Vec<Vector2<f64>>, Vec<f64>) {
    assert!(n >= 3, "polygon needs at least 3 sides, got {n}");
    assert!(
        circumradius > 0.0,
        "circumradius must be positive, got {circumradius}"
    );

    let inradius = circumradius * (PI / n as f64).cos();
    let normals: Vec<Vector2<f64>> = (0..n)
        .map(|k| {
            let angle = PI / 2.0 + 2.0 * PI * (k as f64) / (n as f64);
            Vector2::new(angle.cos(), angle.sin())
        })
        .collect();
    let heights = vec![inradius; n];

    (normals, heights)
}

/// Rotate all normals of a 2D polygon by angle theta (radians, counterclockwise).
///
/// Heights are unchanged (rotation preserves distance from origin).
///
/// Mathematical correspondence: rotation is an area-preserving linear map
pub fn rotate_polygon_2d(
    normals: &[Vector2<f64>],
    heights: &[f64],
    theta: f64,
) -> (Vec<Vector2<f64>>, Vec<f64>) {
    let (sin_t, cos_t) = theta.sin_cos();
    let rotated: Vec<Vector2<f64>> = normals
        .iter()
        .map(|n| Vector2::new(cos_t * n[0] - sin_t * n[1], sin_t * n[0] + cos_t * n[1]))
        .collect();
    (rotated, heights.to_vec())
}

/// Random convex polygon with n sides.
///
/// Generates n uniformly random normal directions on S^1 (sorted by angle),
/// with heights sampled uniformly in [h_min, h_max]. The result is a bounded
/// convex polygon containing the origin.
///
/// # Panics
///
/// Panics if n < 3, h_min <= 0, or h_min >= h_max.
pub fn random_polygon_2d<R: rand::Rng>(
    n: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut R,
) -> (Vec<Vector2<f64>>, Vec<f64>) {
    assert!(n >= 3, "polygon needs at least 3 sides, got {n}");
    assert!(h_min > 0.0, "h_min must be positive, got {h_min}");
    assert!(h_max > h_min, "h_max must exceed h_min");

    // Sample n angles uniformly in [0, 2*pi), sort them
    let mut angles: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * 2.0 * PI).collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let normals: Vec<Vector2<f64>> = angles
        .iter()
        .map(|&a| Vector2::new(a.cos(), a.sin()))
        .collect();

    let heights: Vec<f64> = (0..n)
        .map(|_| h_min + rng.gen::<f64>() * (h_max - h_min))
        .collect();

    (normals, heights)
}

/// Area of a 2D convex polygon given by H-representation.
///
/// Uses the vertex enumeration approach: compute vertices as pairwise
/// intersections of adjacent halfplane boundaries, then apply the shoelace formula.
///
/// Returns None if the polygon is degenerate (fewer than 3 normals or parallel normals).
///
/// Mathematical correspondence: [def:polygon-area]
pub fn polygon_area(normals: &[Vector2<f64>], heights: &[f64]) -> Option<f64> {
    let n = normals.len();
    if n < 3 {
        return None;
    }

    // Compute vertices: intersection of consecutive halfplane boundaries
    // n_k . x = h_k and n_{k+1} . x = h_{k+1}
    let mut vertices = Vec::with_capacity(n);
    for i in 0..n {
        let j = (i + 1) % n;
        let ni = &normals[i];
        let nj = &normals[j];
        let det = ni[0] * nj[1] - ni[1] * nj[0];
        if det.abs() < EPS_PARALLEL {
            // Parallel normals -- degenerate
            return None;
        }
        let x = (heights[i] * nj[1] - heights[j] * ni[1]) / det;
        let y = (ni[0] * heights[j] - nj[0] * heights[i]) / det;
        vertices.push((x, y));
    }

    // Shoelace formula
    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].0 * vertices[j].1 - vertices[j].0 * vertices[i].1;
    }
    Some(area.abs() / 2.0)
}
