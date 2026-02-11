use super::*;
use crate::polytope::Polytope4D;
use nalgebra::Vector4;

#[test]
fn simplex_4d_volume() {
    // Standard 4-simplex: conv{0, e1, e2, e3, e4}
    // Volume = 1/24
    let v0 = Vector4::zeros();
    let v1 = Vector4::x();
    let v2 = Vector4::y();
    let v3 = Vector4::z();
    let v4 = Vector4::w();

    let vol = simplex_volume_5(v0, v1, v2, v3, v4);
    assert!(
        (vol - 1.0 / 24.0).abs() < 1e-10,
        "simplex volume: got {vol}, expected {}",
        1.0 / 24.0
    );
}

#[test]
fn hypercube_volume() {
    // [-1, 1]^4 has volume 2^4 = 16
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![1.0; 8];
    let polytope = Polytope4D::new(normals, heights).expect("hypercube");
    let vol = volume(&polytope);
    assert!(
        (vol - 16.0).abs() < 1e-6,
        "hypercube volume: got {vol}, expected 16"
    );
}

#[test]
fn simplex_polytope_volume() {
    // Standard simplex conv{0, e1, e2, e3, e4}, volume = 1/24
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals_raw = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_raw = vec![0.0, 0.0, 0.0, 0.0, 0.5];
    let heights: Vec<f64> = normals_raw
        .iter()
        .zip(&heights_raw)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();
    let polytope = Polytope4D::new(normals_raw, heights).expect("simplex");
    let vol = volume(&polytope);
    assert!(
        (vol - 1.0 / 24.0).abs() < 1e-6,
        "simplex polytope volume: got {vol}, expected {}",
        1.0 / 24.0
    );
}

// ---- Property tests ----

/// Build a hypercube [-s, s]^4 (volume = (2s)^4 = 16s^4).
fn scaled_hypercube(s: f64) -> Polytope4D {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![s; 8];
    Polytope4D::new(normals, heights).expect("scaled hypercube")
}

#[test]
fn scaling_property() {
    // vol(λK) = λ^4 · vol(K) — for a hypercube, vol([-s,s]^4) = 16·s^4.
    let base_vol = volume(&scaled_hypercube(1.0));
    for &s in &[0.5, 2.0, 3.0, 0.1] {
        let scaled_vol = volume(&scaled_hypercube(s));
        let expected = base_vol * s.powi(4);
        assert!(
            (scaled_vol - expected).abs() < 1e-4,
            "scaling: vol({}·cube) = {scaled_vol}, expected {expected}",
            s
        );
    }
}

#[test]
fn volume_positive_for_known_polytopes() {
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let simplex_n = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let simplex_h_raw = vec![0.0, 0.0, 0.0, 0.0, 0.5];
    let simplex_h: Vec<f64> = simplex_n
        .iter()
        .zip(&simplex_h_raw)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();

    let polytopes = vec![
        Polytope4D::new(simplex_n, simplex_h).expect("simplex"),
        scaled_hypercube(1.0),
    ];

    for p in &polytopes {
        let vol = volume(p);
        assert!(vol > 0.0, "volume should be positive, got {vol}");
    }
}

#[test]
fn crosspolytope_volume() {
    // 4D crosspolytope: conv{±e1, ±e2, ±e3, ±e4}, volume = 8/3
    // H-representation: (±1,±1,±1,±1)/2 · x ≤ 1 (16 facets)
    let mut normals = Vec::with_capacity(16);
    for s0 in [-1.0_f64, 1.0] {
        for s1 in [-1.0_f64, 1.0] {
            for s2 in [-1.0_f64, 1.0] {
                for s3 in [-1.0_f64, 1.0] {
                    normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                }
            }
        }
    }
    let heights = vec![1.0; 16];
    let polytope = Polytope4D::new(normals, heights).expect("crosspolytope");
    let vol = volume(&polytope);
    // Vertices are ±2·e_i (normals (±1,±1,±1,±1)/2, heights 1.0).
    // Vol(conv{±a·e_i}) = a^n · 2^n/n! = 2^4 · 16/24 = 32/3 for a=2, n=4.
    let expected = 32.0 / 3.0;
    assert!(
        (vol - expected).abs() < 1e-6,
        "crosspolytope volume: got {vol}, expected {expected}"
    );
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property test: volume scaling property vol(λK) = λ⁴·vol(K)
        #[test]
        fn volume_scales_with_fourth_power(scale in 0.1f64..10.0) {
            // Create unit hypercube [-1,1]^4
            let normals = vec![
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(-1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, -1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, -1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                Vector4::new(0.0, 0.0, 0.0, -1.0),
            ];
            let heights_unit = vec![1.0; 8];
            let heights_scaled = vec![scale; 8];

            let unit_cube = Polytope4D::new(normals.clone(), heights_unit)
                .expect("unit hypercube construction");
            let scaled_cube = Polytope4D::new(normals, heights_scaled)
                .expect("scaled hypercube construction");

            let vol_unit = volume(&unit_cube);
            let vol_scaled = volume(&scaled_cube);

            // Volume should scale as λ⁴
            let expected_scaled = vol_unit * scale.powi(4);
            let relative_error = ((vol_scaled - expected_scaled) / expected_scaled).abs();

            prop_assert!(
                relative_error < 1e-4,
                "volume scaling failed: scale={}, vol_unit={}, vol_scaled={}, expected={}, relative_error={}",
                scale, vol_unit, vol_scaled, expected_scaled, relative_error
            );
        }
    }
}
