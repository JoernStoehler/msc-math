//! Tests for polygon: area, vertex count, regularity, rotation invariance.
//!
//! Proposition: regular_polygon_2d produces an n-gon with unit normals and
//! inradius = circumradius * cos(pi/n). Area formulas agree with the known
//! (n/2) * R^2 * sin(2*pi/n). Rotation preserves area and heights.
//!
//! Strategy: fixture-based (triangle, square, pentagon, hexagon) + edge cases

use crate::geom::polygon::{polygon_area, random_polygon_2d, regular_polygon_2d, rotate_polygon_2d};
use nalgebra::Vector2;
use std::f64::consts::PI;

#[test]
fn regular_triangle_has_3_normals() {
    let (normals, heights) = regular_polygon_2d(3, 1.0);
    assert_eq!(normals.len(), 3);
    assert_eq!(heights.len(), 3);
}

#[test]
fn regular_pentagon_has_5_normals() {
    let (normals, heights) = regular_polygon_2d(5, 1.0);
    assert_eq!(normals.len(), 5);
    assert_eq!(heights.len(), 5);
}

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

#[test]
fn rotation_preserves_heights() {
    let (normals, heights) = regular_polygon_2d(5, 1.0);
    let (_, rotated_heights) = rotate_polygon_2d(&normals, &heights, PI / 3.0);
    assert_eq!(heights, rotated_heights);
}

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

#[test]
fn rotation_by_zero_is_identity() {
    let (normals, heights) = regular_polygon_2d(5, 1.0);
    let (rotated, _) = rotate_polygon_2d(&normals, &heights, 0.0);
    for (orig, rot) in normals.iter().zip(rotated.iter()) {
        assert!((orig - rot).norm() < 1e-14);
    }
}

#[test]
fn rotation_by_2pi_is_identity() {
    let (normals, heights) = regular_polygon_2d(5, 1.0);
    let (rotated, _) = rotate_polygon_2d(&normals, &heights, 2.0 * PI);
    for (orig, rot) in normals.iter().zip(rotated.iter()) {
        assert!((orig - rot).norm() < 1e-12);
    }
}

// ---- Area tests ----

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

#[test]
fn random_polygon_has_correct_count() {
    let mut rng = rand::thread_rng();
    let (normals, heights) = random_polygon_2d(5, 0.5, 2.0, &mut rng);
    assert_eq!(normals.len(), 5);
    assert_eq!(heights.len(), 5);
}

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

#[test]
#[should_panic(expected = "at least 3")]
fn regular_polygon_2_sides_panics() {
    regular_polygon_2d(2, 1.0);
}

#[test]
#[should_panic(expected = "positive")]
fn regular_polygon_zero_radius_panics() {
    regular_polygon_2d(3, 0.0);
}

#[test]
#[should_panic(expected = "positive")]
fn regular_polygon_negative_radius_panics() {
    regular_polygon_2d(4, -1.0);
}

#[test]
#[should_panic(expected = "at least 3")]
fn random_polygon_too_few_sides_panics() {
    let mut rng = rand::thread_rng();
    random_polygon_2d(2, 0.5, 2.0, &mut rng);
}

#[test]
#[should_panic(expected = "positive")]
fn random_polygon_zero_hmin_panics() {
    let mut rng = rand::thread_rng();
    random_polygon_2d(3, 0.0, 2.0, &mut rng);
}

#[test]
#[should_panic(expected = "positive")]
fn random_polygon_negative_hmin_panics() {
    let mut rng = rand::thread_rng();
    random_polygon_2d(3, -1.0, 2.0, &mut rng);
}

#[test]
#[should_panic(expected = "h_max must exceed")]
fn random_polygon_hmax_equals_hmin_panics() {
    let mut rng = rand::thread_rng();
    random_polygon_2d(3, 1.0, 1.0, &mut rng);
}

#[test]
#[should_panic(expected = "h_max must exceed")]
fn random_polygon_hmax_less_than_hmin_panics() {
    let mut rng = rand::thread_rng();
    random_polygon_2d(3, 2.0, 1.0, &mut rng);
}

// ---- polygon_area edge cases ----

#[test]
fn polygon_area_too_few_normals_returns_none() {
    let normals = vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 1.0)];
    let heights = vec![1.0, 1.0];
    assert!(polygon_area(&normals, &heights).is_none());
}

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
