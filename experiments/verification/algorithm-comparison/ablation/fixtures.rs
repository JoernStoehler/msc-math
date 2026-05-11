//! Fixture builders for the ablation dataset families.
//!
//! Extracted from the original monolithic `ablation/main.rs`. The regression
//! cut-simplex fixture is the experiment copy of the non-simple example in
//! `formal/search-pruning-correctness.tex` [ex:a3-prunes].

//! Dataset constructors for the algorithm-comparison ablation experiment.
//!
//! The regression and non-simple fixtures stay local to this binary because the
//! study compares variant behavior on a fixed hand-picked dataset rather than a
//! durable library surface.

use crate::flat_polytope::FlatPolytopeCache;
use crate::models::{AblationFixture, H_MAX, H_MIN, N_PER_GROUP, SEED};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use symplectic::geom::known_polytopes;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::{random_polygon_2d, regular_polygon_2d};

fn make_bipyramid(
    normals_3d: &[[f64; 3]],
    heights_3d: &[f64],
    apex_height: f64,
) -> FlatPolytopeCache {
    let k = normals_3d.len();
    let mut normals = Vec::with_capacity(2 * k);
    let mut heights = Vec::with_capacity(2 * k);

    for i in 0..k {
        let [nx, ny, nz] = normals_3d[i];
        let h = heights_3d[i];
        let c = h / apex_height;
        let norm4 = (nx * nx + ny * ny + nz * nz + c * c).sqrt();

        normals.push(Vector4::new(nx / norm4, ny / norm4, nz / norm4, c / norm4));
        heights.push(h / norm4);

        normals.push(Vector4::new(nx / norm4, ny / norm4, nz / norm4, -c / norm4));
        heights.push(h / norm4);
    }

    FlatPolytopeCache::from_f64_dual_vertices(
        normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .expect("bipyramid construction")
}

fn make_cut_simplex(cut_slope: f64) -> FlatPolytopeCache {
    // This is the cut-simplex family discussed in
    // formal/search-pruning-correctness.tex:\ref{ex:a3-prunes}.
    let s19 = 19.0_f64.sqrt();
    let norm = (1.0 + cut_slope * cut_slope).sqrt();
    let normals = vec![
        Vector4::new(-4.0, 1.0, 1.0, 1.0) / s19,
        Vector4::new(1.0, -4.0, 1.0, 1.0) / s19,
        Vector4::new(1.0, 1.0, -4.0, 1.0) / s19,
        Vector4::new(1.0, 1.0, 1.0, -4.0) / s19,
        Vector4::new(1.0, 1.0, 1.0, 1.0) / 2.0,
        Vector4::new(1.0, cut_slope, 0.0, 0.0) / norm,
    ];
    let heights = vec![2.0 / s19, 2.0 / s19, 2.0 / s19, 2.0 / s19, 1.0, 2.0 / norm];
    FlatPolytopeCache::from_f64_dual_vertices(
        normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .expect("cut simplex construction")
}

pub fn build_ablation_polytopes() -> Vec<AblationFixture> {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut polytopes = Vec::new();

    println!("Part 1: Random generic polytopes (F=5..10, {N_PER_GROUP} each)...");
    for f in [5usize, 6, 7, 8, 9, 10] {
        for i in 0..N_PER_GROUP {
            let p = FlatPolytopeCache::sample_random(f, H_MIN, H_MAX, &mut rng)
                .expect("random polytope construction");
            polytopes.push(AblationFixture {
                name: format!("random_F{f}_{i}"),
                group: "random_generic".to_string(),
                geometry: p,
                expected_capacity: None,
            });
        }
        println!("  F={f}: {N_PER_GROUP} polytopes");
    }

    println!("\nPart 2: Random Lagrangian products ({N_PER_GROUP} per pair)...");
    for (n, m) in [(3usize, 3usize), (3, 4), (4, 4)] {
        for i in 0..N_PER_GROUP {
            let p = loop {
                let (qn, qh) = random_polygon_2d(n, H_MIN, H_MAX, &mut rng);
                let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
                if let Ok(dual_vertices) = lagrangian_product(&qn, &qh, &pn, &ph) {
                    let poly = FlatPolytopeCache::from_f64_dual_vertices(dual_vertices)
                        .expect("validated lagrangian product should reconstruct");
                    break poly;
                }
            };
            polytopes.push(AblationFixture {
                name: format!("random_lagrangian_{n}x{m}_{i}"),
                group: "random_lagrangian".to_string(),
                geometry: p,
                expected_capacity: None,
            });
        }
        println!("  ({n}×{m}): {N_PER_GROUP} polytopes (F={})", n + m);
    }

    println!("\nPart 3: Regression cases...");

    {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = FlatPolytopeCache::from_f64_dual_vertices(
            lagrangian_product(&qn, &qh, &pn, &ph).expect("(3,4) construction"),
        )
        .expect("validated (3,4) should reconstruct");
        let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0;
        polytopes.push(AblationFixture {
            name: "regression_34_theta0".to_string(),
            group: "regression".to_string(),
            geometry: p,
            expected_capacity: Some(expected),
        });
        println!("  (3,4) θ=0°: F=7, expected {expected:.6}");
    }

    {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = FlatPolytopeCache::from_f64_dual_vertices(
            lagrangian_product(&qn, &qh, &pn, &ph).expect("(4,4) construction"),
        )
        .expect("validated (4,4) should reconstruct");
        polytopes.push(AblationFixture {
            name: "regression_44_theta0".to_string(),
            group: "regression".to_string(),
            geometry: p,
            expected_capacity: Some(2.0),
        });
        println!("  (4,4) θ=0°: F=8, expected 2.0");
    }

    {
        let kp = known_polytopes::hypercube();
        println!(
            "  hypercube:  F={}, expected {}",
            kp.facet_count(),
            kp.capacity
        );
        polytopes.push(AblationFixture {
            name: "regression_hypercube".to_string(),
            group: "regression".to_string(),
            geometry: FlatPolytopeCache::from_known(kp),
            expected_capacity: Some(kp.capacity),
        });
    }

    {
        let p = make_cut_simplex(2.0);
        println!(
            "  cut simplex: F={}, non-simple (v₀ on 5 facets)",
            p.facet_count()
        );
        polytopes.push(AblationFixture {
            name: "regression_cut_simplex".to_string(),
            group: "regression".to_string(),
            geometry: p,
            expected_capacity: None,
        });
    }

    println!("\nPart 4: Non-simple polytopes (bipyramids + cut simplices)...");

    {
        let s3_2 = 3.0_f64.sqrt() / 2.0;
        let normals_3d: &[[f64; 3]] = &[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, 1.0],
            [0.5, s3_2, 0.0],
            [-1.0, 0.0, 0.0],
            [0.5, -s3_2, 0.0],
        ];
        let heights_3d: &[f64] = &[1.0, 1.0, 0.5, 0.5, 0.5];
        let p = make_bipyramid(normals_3d, heights_3d, 1.5);
        println!(
            "  bipyramid (triangular prism): F={}, non-simple (apices on 5 facets)",
            p.facet_count()
        );
        polytopes.push(AblationFixture {
            name: "nonsimple_bipyramid_triprism".to_string(),
            group: "non_simple".to_string(),
            geometry: p,
            expected_capacity: None,
        });
    }

    {
        let s5 = 5.0_f64.sqrt();
        let normals_3d: &[[f64; 3]] = &[
            [0.0, 0.0, -1.0],
            [2.0 / s5, 0.0, 1.0 / s5],
            [0.0, -2.0 / s5, 1.0 / s5],
            [-2.0 / s5, 0.0, 1.0 / s5],
            [0.0, 2.0 / s5, 1.0 / s5],
        ];
        let heights_3d: &[f64] = &[0.4, 1.6 / s5, 1.6 / s5, 1.6 / s5, 1.6 / s5];
        let p = make_bipyramid(normals_3d, heights_3d, 1.5);
        println!(
            "  bipyramid (square pyramid): F={}, non-simple (apices on 5 facets)",
            p.facet_count()
        );
        polytopes.push(AblationFixture {
            name: "nonsimple_bipyramid_sqpyr".to_string(),
            group: "non_simple".to_string(),
            geometry: p,
            expected_capacity: None,
        });
    }

    for (label, slope) in [("shallow", 1.5), ("medium", 2.5), ("deep", 4.0)] {
        let p = make_cut_simplex(slope);
        println!(
            "  cut simplex ({label}, c={slope}): F={}, non-simple",
            p.facet_count()
        );
        polytopes.push(AblationFixture {
            name: format!("nonsimple_cut_simplex_{label}"),
            group: "non_simple".to_string(),
            geometry: p,
            expected_capacity: None,
        });
    }

    polytopes
}
