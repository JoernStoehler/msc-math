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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;
    use std::f64::consts::PI;

    // Tests for polygon: area, vertex count, regularity, rotation invariance.
    //
    // Proposition: regular_polygon_2d produces an n-gon with unit normals and
    // inradius = circumradius * cos(pi/n). Area formulas agree with the known
    // (n/2) * R^2 * sin(2*pi/n). Rotation preserves area and heights.
    //
    // Strategy: fixture-based (triangle, square, pentagon, hexagon) + edge cases

    /// Verify regular_polygon_2d(3) produces exactly 3 normals and heights.
    #[test]
    fn regular_triangle_has_3_normals() {
        let (normals, heights) = regular_polygon_2d(3, 1.0);
        assert_eq!(normals.len(), 3);
        assert_eq!(heights.len(), 3);
    }

    /// Verify regular_polygon_2d(5) produces exactly 5 normals and heights.
    #[test]
    fn regular_pentagon_has_5_normals() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        assert_eq!(normals.len(), 5);
        assert_eq!(heights.len(), 5);
    }

    /// Verify all normals of regular n-gons (n=3..8) have unit length.
    #[test]
    fn regular_polygon_normals_are_unit() {
        for n in 3..=8 {
            let (normals, _) = regular_polygon_2d(n, 1.0);
            for (i, normal) in normals.iter().enumerate() {
                let norm = normal.norm();
                assert!(
                    (norm - 1.0).abs() < 1e-12,
                    "n={n}, normal[{i}] has norm {norm}"
                );
            }
        }
    }

    /// Verify heights equal the inradius R*cos(pi/n) for regular n-gons.
    #[test]
    fn regular_polygon_heights_are_inradius() {
        for n in 3..=8 {
            let circumradius = 1.0;
            let (_, heights) = regular_polygon_2d(n, circumradius);
            let expected_inradius = circumradius * (PI / n as f64).cos();
            for (i, &h) in heights.iter().enumerate() {
                assert!(
                    (h - expected_inradius).abs() < 1e-12,
                    "n={n}, height[{i}] = {h}, expected {expected_inradius}"
                );
            }
        }
    }

    /// Verify the pentagon's first normal starts at angle pi/2 per our convention.
    #[test]
    fn regular_pentagon_matches_hko_convention() {
        // Our convention: normals start at pi/2.
        // k=0: pi/2 => (cos 90, sin 90) = (0, 1)
        let (normals, heights) = regular_polygon_2d(5, 1.0);

        // First normal should be at pi/2 (pointing up)
        assert!(normals[0][0].abs() < 1e-12, "first normal x should be ~0");
        assert!(
            (normals[0][1] - 1.0).abs() < 1e-12,
            "first normal y should be ~1"
        );

        // Heights should be cos(pi/5)
        let expected = (PI / 5.0).cos();
        assert!(
            (heights[0] - expected).abs() < 1e-10,
            "pentagon inradius: got {}, expected {}",
            heights[0],
            expected
        );
    }

    /// Verify that rotation does not change the height values.
    #[test]
    fn rotation_preserves_heights() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let (_, rotated_heights) = rotate_polygon_2d(&normals, &heights, PI / 3.0);
        assert_eq!(heights, rotated_heights);
    }

    /// Verify that rotated normals remain unit vectors.
    #[test]
    fn rotation_preserves_unit_normals() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let (rotated, _) = rotate_polygon_2d(&normals, &heights, 1.234);
        for (i, n) in rotated.iter().enumerate() {
            let norm = n.norm();
            assert!(
                (norm - 1.0).abs() < 1e-12,
                "rotated normal[{i}] has norm {norm}"
            );
        }
    }

    /// Verify that rotation by zero leaves normals unchanged.
    #[test]
    fn rotation_by_zero_is_identity() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let (rotated, _) = rotate_polygon_2d(&normals, &heights, 0.0);
        for (orig, rot) in normals.iter().zip(rotated.iter()) {
            assert!((orig - rot).norm() < 1e-14);
        }
    }

    /// Verify that rotation by 2*pi returns to the original normals.
    #[test]
    fn rotation_by_2pi_is_identity() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let (rotated, _) = rotate_polygon_2d(&normals, &heights, 2.0 * PI);
        for (orig, rot) in normals.iter().zip(rotated.iter()) {
            assert!((orig - rot).norm() < 1e-12);
        }
    }

    // ---- Area tests ----

    /// Verify the unit-circumradius square has area 2.
    #[test]
    fn square_area_is_correct() {
        // Square with circumradius 1: inradius = cos(pi/4) = sqrt(2)/2
        // Side length = 2 * sin(pi/4) = sqrt(2). Area = side^2 = 2.
        let (normals, heights) = regular_polygon_2d(4, 1.0);
        let area = polygon_area(&normals, &heights).unwrap();
        assert!(
            (area - 2.0).abs() < 1e-10,
            "square area: got {area}, expected 2.0"
        );
    }

    /// Verify the unit-circumradius equilateral triangle has area 3*sqrt(3)/4.
    #[test]
    fn equilateral_triangle_area() {
        // Circumradius 1: area = (3/2) * sin(2*pi/3) = 3*sqrt(3)/4
        let (normals, heights) = regular_polygon_2d(3, 1.0);
        let area = polygon_area(&normals, &heights).unwrap();
        let expected = 3.0 * 3.0_f64.sqrt() / 4.0;
        assert!(
            (area - expected).abs() < 1e-10,
            "triangle area: got {area}, expected {expected}"
        );
    }

    /// Verify the unit-circumradius regular pentagon has area (5/2)*sin(2*pi/5).
    #[test]
    fn regular_pentagon_area() {
        // Circumradius 1: area = (5/2) * sin(2*pi/5)
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let area = polygon_area(&normals, &heights).unwrap();
        let expected = 2.5 * (2.0 * PI / 5.0).sin();
        assert!(
            (area - expected).abs() < 1e-10,
            "pentagon area: got {area}, expected {expected}"
        );
    }

    /// Verify the unit-circumradius regular hexagon has area 3*sqrt(3)/2.
    #[test]
    fn regular_hexagon_area() {
        // Circumradius 1: area = (6/2) * sin(2*pi/6) = 3 * sin(60) = 3*sqrt(3)/2
        let (normals, heights) = regular_polygon_2d(6, 1.0);
        let area = polygon_area(&normals, &heights).unwrap();
        let expected = 3.0 * (2.0 * PI / 6.0).sin();
        assert!(
            (area - expected).abs() < 1e-10,
            "hexagon area: got {area}, expected {expected}"
        );
    }

    /// Verify that rotating a polygon preserves its area.
    #[test]
    fn rotation_preserves_area() {
        let (normals, heights) = regular_polygon_2d(5, 1.0);
        let orig_area = polygon_area(&normals, &heights).unwrap();
        let (rot_normals, rot_heights) = rotate_polygon_2d(&normals, &heights, PI / 7.0);
        let rot_area = polygon_area(&rot_normals, &rot_heights).unwrap();
        assert!(
            (orig_area - rot_area).abs() < 1e-10,
            "area changed under rotation: {orig_area} vs {rot_area}"
        );
    }

    // ---- random_polygon_2d tests ----

    /// Verify random_polygon_2d produces the requested number of normals and heights.
    #[test]
    fn random_polygon_has_correct_count() {
        let mut rng = rand::thread_rng();
        let (normals, heights) = random_polygon_2d(5, 0.5, 2.0, &mut rng);
        assert_eq!(normals.len(), 5);
        assert_eq!(heights.len(), 5);
    }

    /// Verify random polygon normals are unit vectors.
    #[test]
    fn random_polygon_normals_are_unit() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let (normals, _) = random_polygon_2d(5, 0.5, 2.0, &mut rng);
            for n in &normals {
                assert!((n.norm() - 1.0).abs() < 1e-12);
            }
        }
    }

    /// Verify random polygon heights fall within the requested [h_min, h_max] range.
    #[test]
    fn random_polygon_heights_in_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let (_, heights) = random_polygon_2d(5, 0.5, 2.0, &mut rng);
            for &h in &heights {
                assert!(
                    (0.5..=2.0).contains(&h),
                    "height {h} out of range [0.5, 2.0]"
                );
            }
        }
    }

    /// Verify random polygon normal angles are in sorted (counterclockwise) order.
    #[test]
    fn random_polygon_angles_are_sorted() {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let (normals, _) = random_polygon_2d(5, 0.5, 2.0, &mut rng);
            let angles: Vec<f64> = normals
                .iter()
                .map(|n| {
                    let a = n[1].atan2(n[0]);
                    if a < 0.0 {
                        a + 2.0 * PI
                    } else {
                        a
                    }
                })
                .collect();
            for i in 1..angles.len() {
                assert!(
                    angles[i] >= angles[i - 1],
                    "angles not sorted: {:?}",
                    angles
                );
            }
        }
    }

    // ---- Panic tests ----

    /// Verify regular_polygon_2d panics for n < 3 sides.
    #[test]
    #[should_panic(expected = "at least 3")]
    fn regular_polygon_2_sides_panics() {
        regular_polygon_2d(2, 1.0);
    }

    /// Verify regular_polygon_2d panics for zero circumradius.
    #[test]
    #[should_panic(expected = "positive")]
    fn regular_polygon_zero_radius_panics() {
        regular_polygon_2d(3, 0.0);
    }

    /// Verify regular_polygon_2d panics for negative circumradius.
    #[test]
    #[should_panic(expected = "positive")]
    fn regular_polygon_negative_radius_panics() {
        regular_polygon_2d(4, -1.0);
    }

    /// Verify random_polygon_2d panics for n < 3 sides.
    #[test]
    #[should_panic(expected = "at least 3")]
    fn random_polygon_too_few_sides_panics() {
        let mut rng = rand::thread_rng();
        random_polygon_2d(2, 0.5, 2.0, &mut rng);
    }

    /// Verify random_polygon_2d panics for h_min = 0.
    #[test]
    #[should_panic(expected = "positive")]
    fn random_polygon_zero_hmin_panics() {
        let mut rng = rand::thread_rng();
        random_polygon_2d(3, 0.0, 2.0, &mut rng);
    }

    /// Verify random_polygon_2d panics for negative h_min.
    #[test]
    #[should_panic(expected = "positive")]
    fn random_polygon_negative_hmin_panics() {
        let mut rng = rand::thread_rng();
        random_polygon_2d(3, -1.0, 2.0, &mut rng);
    }

    /// Verify random_polygon_2d panics when h_max equals h_min.
    #[test]
    #[should_panic(expected = "h_max must exceed")]
    fn random_polygon_hmax_equals_hmin_panics() {
        let mut rng = rand::thread_rng();
        random_polygon_2d(3, 1.0, 1.0, &mut rng);
    }

    /// Verify random_polygon_2d panics when h_max < h_min.
    #[test]
    #[should_panic(expected = "h_max must exceed")]
    fn random_polygon_hmax_less_than_hmin_panics() {
        let mut rng = rand::thread_rng();
        random_polygon_2d(3, 2.0, 1.0, &mut rng);
    }

    // ---- polygon_area edge cases ----

    /// Verify polygon_area returns None for fewer than 3 normals.
    #[test]
    fn polygon_area_too_few_normals_returns_none() {
        let normals = vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 1.0)];
        let heights = vec![1.0, 1.0];
        assert!(polygon_area(&normals, &heights).is_none());
    }

    /// Verify polygon_area returns None for degenerate polygon with parallel normals.
    #[test]
    fn polygon_area_parallel_normals_returns_none() {
        // Three normals, two of them parallel -- degenerate polygon.
        let normals = vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 0.0), // parallel to first
            Vector2::new(0.0, 1.0),
        ];
        let heights = vec![1.0, 2.0, 1.0];
        assert!(polygon_area(&normals, &heights).is_none());
    }
}
