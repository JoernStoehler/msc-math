//! Finite cross-check of Theorem `thm:pentagon-symmetric-partner` (thesis
//! Section 11) with the production QP capacity route. Not a proof: binary64
//! inputs are exact dyadic approximations of the irrational bodies.
//! Run: cargo test --release -p symplectic --test symmetric_partner_theorem_anchor -- --nocapture

use nalgebra::{Vector2, Vector4};
use std::f64::consts::PI;
use symplectic::capacity_4d::capacity_from_dual_vertices;

/// Facet rows n/h of a polygon given by outward normal angles and heights.
fn rows(angles: &[f64], heights: &[f64]) -> Vec<Vector2<f64>> {
    angles
        .iter()
        .zip(heights)
        .map(|(a, h)| Vector2::new(a.cos() / h, a.sin() / h))
        .collect()
}

fn area(vertices: &[Vector2<f64>]) -> f64 {
    let n = vertices.len();
    0.5 * (0..n)
        .map(|i| {
            let (a, b) = (vertices[i], vertices[(i + 1) % n]);
            a.x * b.y - a.y * b.x
        })
        .sum::<f64>()
}

/// Regular polygon with `n` vertices at radius `r`, first vertex at angle `phase`.
fn regular(n: usize, r: f64, phase: f64) -> (Vec<Vector2<f64>>, Vec<f64>, Vec<f64>) {
    let verts: Vec<_> = (0..n)
        .map(|k| {
            let t = phase + 2.0 * PI * k as f64 / n as f64;
            Vector2::new(r * t.cos(), r * t.sin())
        })
        .collect();
    let angles: Vec<_> = (0..n)
        .map(|k| phase + PI * (2 * k + 1) as f64 / n as f64)
        .collect();
    let heights = vec![r * (PI / n as f64).cos(); n];
    (verts, angles, heights)
}

#[test]
fn symmetric_partner_capacity_matches_decagon_scale() {
    let theta = PI / 5.0;
    let h = theta.cos();
    // Q: normals n_i at angles 2 pi i / 5, heights h (circumradius one).
    let q_angles: Vec<_> = (0..5).map(|i| 2.0 * PI * i as f64 / 5.0).collect();
    let q_rows = rows(&q_angles, &vec![h; 5]);
    let q_area = 2.5 * (2.0 * PI / 5.0).sin();
    let r_b = 1.0 / (2.0 * (1.0 + h));
    // B: vertices +-R n_i, i.e. a regular decagon with a vertex at angle 0.
    let (b_verts, _, _) = regular(10, r_b, 0.0);
    let bound = (10.0 + 6.0 * 5f64.sqrt()) / 25.0;

    // Largest r with rB inside K = {x : m_j . x <= g_j}, K centred at 0.
    let lambda_b = |angles: &[f64], heights: &[f64]| {
        b_verts
            .iter()
            .flat_map(|v| {
                angles.iter().zip(heights).filter_map(move |(a, g)| {
                    let s = a.cos() * v.x + a.sin() * v.y;
                    (s > 0.0).then_some(g / s)
                })
            })
            .fold(f64::INFINITY, f64::min)
    };

    let mut cases: Vec<(String, Vec<Vector2<f64>>, Vec<f64>, Vec<f64>)> = Vec::new();
    let (v, a, g) = regular(10, r_b, 0.0);
    cases.push(("B (equality)".into(), v, a, g));
    let (v, a, g) = regular(10, 3.0 * r_b, 0.0);
    cases.push(("3B (equality, dilate)".into(), v, a, g));
    let (v, a, g) = regular(10, r_b, 0.05);
    cases.push(("B rotated 0.05".into(), v, a, g));
    for n in [4usize, 6, 8] {
        let (v, a, g) = regular(n, 1.0, 0.0);
        cases.push((format!("regular {n}-gon"), v, a, g));
    }
    let (v, a, g) = regular(8, 1.0, 0.2);
    cases.push(("regular 8-gon rotated 0.2".into(), v, a, g));

    for (label, verts, angles, heights) in cases {
        let k_rows = rows(&angles, &heights);
        let duals: Vec<Vector4<f64>> = k_rows
            .iter()
            .map(|r| Vector4::new(r.x, r.y, 0.0, 0.0))
            .chain(q_rows.iter().map(|r| Vector4::new(0.0, 0.0, r.x, r.y)))
            .collect();
        let cap = capacity_from_dual_vertices(&duals).expect("capacity");
        let b = cap.bounds();
        let c = 0.5 * (b.lower() + b.upper());
        let lam = lambda_b(&angles, &heights);
        let sys = c * c / (2.0 * area(&verts) * q_area);
        println!(
            "{label:28} c=[{:.15}, {:.15}] lambda_B={lam:.15} rel_diff={:.2e} sys={sys:.15} bound={bound:.15} route={}",
            b.lower(),
            b.upper(),
            (c - lam).abs() / lam,
            cap.route_name()
        );
        assert!(
            (c - lam).abs() <= 1e-9 * lam,
            "{label}: capacity != lambda_B"
        );
        assert!(sys <= bound + 1e-9, "{label}: ratio exceeds bound");
        if label.contains("equality") {
            assert!((sys - bound).abs() <= 1e-9, "{label}: equality case");
        } else {
            assert!(sys < bound - 1e-6, "{label}: expected strict inequality");
        }
    }
}
