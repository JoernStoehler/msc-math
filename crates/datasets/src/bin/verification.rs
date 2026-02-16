//! Verification tests for algorithm correctness.
//!
//! Tests:
//! 1. Direct comparison: 5 random + 5 Lagrangian, all algorithms agree
//! 2. Literature: 7 known polytopes match published values
//! 3. Conformality: c(αK) = α²c(K)
//! 4. Symplectic invariance: c(MK) = c(K) for M ∈ Sp(4)
//! 5. Continuity: small perturbations → small capacity changes
//! 6. Monotonicity: αK₁ ⊂ K₂ ⇒ α²c(K₁) ≤ c(K₂)
//!
//! Run: `cargo test --bin verification --release`

use billiard::billiard_capacity;
use datasets::random::generate_random_polytopes;
use geom::known_polytopes::{
    hko_pentagon, hypercube, lagrangian_triangle_product, lagrangian_triangle_square,
    simplex, symplectic_triangle_product, symplectic_triangle_square,
};
use geom::lagrangian_product::lagrangian_product;
use geom::polygon::random_polygon_2d;
use geom::polytope::Polytope4D;
use hk2017::{ehz_capacity, ehz_capacity_pruned};
use nalgebra::Matrix4;
use rand_distr::StandardNormal;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const TOLERANCE: f64 = 1e-8;

fn main() {
    println!("This binary only provides tests. Run: cargo test --bin verification --release");
}

/// Apply symplectomorphism (M, b) to polytope K: x ↦ Mx + b
/// Transforms K = {x : ⟨n_i, x⟩ ≤ h_i} to MK+b = {x : ⟨(M^{-T}n_i)/||·||, x⟩ ≤ (h_i + ⟨M^{-T}n_i, b⟩)/||·||}
fn apply_symplectomorphism(p: &Polytope4D, m: &Matrix4<f64>) -> Polytope4D {
    let m_inv_t = m.transpose().try_inverse().expect("M should be invertible");
    let mut normals = Vec::new();
    let mut heights = Vec::new();

    for (n, &h) in p.normals().iter().zip(p.heights().iter()) {
        let n_raw = m_inv_t * n;
        let norm = n_raw.norm();
        normals.push(n_raw / norm);
        heights.push(h / norm); // b=0 for our case
    }

    Polytope4D::new(normals, heights).expect("transformed polytope")
}

/// Generate random Sp(4) matrix using Cayley transform: M = (I - A)(I + A)^{-1}
/// where A ∈ sp(4) satisfies A^T J + J A = 0.
fn random_sp4_matrix(rng: &mut impl Rng) -> Matrix4<f64> {
    // P: arbitrary 2×2 (4 params)
    let p11: f64 = rng.sample(StandardNormal);
    let p12: f64 = rng.sample(StandardNormal);
    let p21: f64 = rng.sample(StandardNormal);
    let p22: f64 = rng.sample(StandardNormal);

    // Q: symmetric 2×2 (3 params)
    let q11: f64 = rng.sample(StandardNormal);
    let q12: f64 = rng.sample(StandardNormal);
    let q22: f64 = rng.sample(StandardNormal);

    // R: symmetric 2×2 (3 params)
    let r11: f64 = rng.sample(StandardNormal);
    let r12: f64 = rng.sample(StandardNormal);
    let r22: f64 = rng.sample(StandardNormal);

    // S = -P^T
    let s11 = -p11;
    let s12 = -p21;
    let s21 = -p12;
    let s22 = -p22;

    // Build A in block form
    #[rustfmt::skip]
    let a = Matrix4::new(
        p11, p12, q11, q12,
        p21, p22, q12, q22,
        r11, r12, s11, s12,
        r12, r22, s21, s22,
    );

    // Cayley transform: M = (I - A)(I + A)^{-1}
    let i = Matrix4::identity();
    let i_plus_a = &i + &a;
    let i_minus_a = &i - &a;

    i_minus_a * i_plus_a.try_inverse().expect("(I + A) should be invertible")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_direct_comparison() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // 5 random generic polytopes (F=5,6,7,8,9)
        let random_generic = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

        // 5 random Lagrangian products (small for speed, retry if unbounded)
        let mut random_lagrangian = Vec::new();
        while random_lagrangian.len() < 5 {
            let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                random_lagrangian.push(p);
            }
        }

        // Test random generic: pruned vs unpruned
        for (i, p) in random_generic.iter().enumerate() {
            let pruned = ehz_capacity_pruned(p).expect("HK2017 pruned failed").capacity;
            let unpruned = ehz_capacity(p).expect("HK2017 unpruned failed").capacity;
            let rel_err = (pruned - unpruned).abs() / pruned.max(1e-15);
            assert!(
                rel_err < TOLERANCE,
                "Random generic #{}: pruned={}, unpruned={}, rel_err={}",
                i, pruned, unpruned, rel_err
            );
        }

        // Test random Lagrangian: pruned vs unpruned vs billiard
        for (i, p) in random_lagrangian.iter().enumerate() {
            let pruned = ehz_capacity_pruned(p).expect("HK2017 pruned failed").capacity;
            let unpruned = ehz_capacity(p).expect("HK2017 unpruned failed").capacity;
            let billiard = billiard_capacity(p)
                .expect("Billiard call failed")
                .expect("Billiard should work on Lagrangian products")
                .capacity;

            let err_pu = (pruned - unpruned).abs() / pruned.max(1e-15);
            let err_pb = (pruned - billiard).abs() / pruned.max(1e-15);

            assert!(
                err_pu < TOLERANCE && err_pb < TOLERANCE,
                "Random Lagrangian #{}: pruned={}, unpruned={}, billiard={}, err_pu={}, err_pb={}",
                i, pruned, unpruned, billiard, err_pu, err_pb
            );
        }

        println!(
            "✓ Direct comparison: {} generic + {} Lagrangian products verified",
            random_generic.len(),
            random_lagrangian.len()
        );
    }

    #[test]
    fn test_2_literature() {
        // Use actual .capacity field from KnownPolytope structs
        let known = vec![
            simplex(),
            hypercube(),
            hko_pentagon(),
            lagrangian_triangle_product(),
            lagrangian_triangle_square(),
            symplectic_triangle_product(),
            symplectic_triangle_square(),
        ];

        for kp in &known {
            let published = kp.capacity;
            let pruned = ehz_capacity_pruned(&kp.polytope)
                .expect("HK2017 pruned failed")
                .capacity;
            let rel_err = (pruned - published).abs() / published.max(1e-15);

            assert!(
                rel_err < TOLERANCE,
                "{}: pruned={}, published={}, rel_err={}",
                kp.name, pruned, published, rel_err
            );

            // If Lagrangian product, also check billiard
            if let Ok(Some(billiard_result)) = billiard_capacity(&kp.polytope) {
                let bil_err = (billiard_result.capacity - published).abs() / published.max(1e-15);
                assert!(
                    bil_err < TOLERANCE,
                    "{}: billiard={}, published={}, rel_err={}",
                    kp.name, billiard_result.capacity, published, bil_err
                );
            }
        }

        println!("✓ Literature: {} polytopes verified", known.len());
    }

    #[test]
    fn test_3_conformality() {
        let mut rng = ChaCha8Rng::seed_from_u64(123);

        // 5 random generic
        let mut polytopes = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

        // 5 random Lagrangian products (retry if unbounded)
        let mut count = 0;
        while count < 5 {
            let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                polytopes.push(p);
                count += 1;
            }
        }

        for (i, p) in polytopes.iter().enumerate() {
            let c_p = ehz_capacity_pruned(p).expect("HK2017 failed").capacity;
            let alpha: f64 = rng.gen_range(0.5..2.0);

            let scaled = Polytope4D::new(
                p.normals().to_vec(),
                p.heights().iter().map(|&h| alpha * h).collect(),
            )
            .expect("Scaled polytope failed");

            let c_scaled = ehz_capacity_pruned(&scaled).expect("HK2017 on scaled failed").capacity;
            let expected = alpha * alpha * c_p;
            let rel_err = (c_scaled - expected).abs() / expected.max(1e-15);

            assert!(
                rel_err < TOLERANCE,
                "Polytope #{}: c({:.2}K) = {}, expected {:.2}²·{} = {}, rel_err={}",
                i, alpha, c_scaled, alpha, c_p, expected, rel_err
            );

            // If Lagrangian, check billiard too
            if let Ok(Some(bil)) = billiard_capacity(p) {
                if let Ok(Some(bil_scaled)) = billiard_capacity(&scaled) {
                    let expected_bil = alpha * alpha * bil.capacity;
                    let err_bil = (bil_scaled.capacity - expected_bil).abs() / expected_bil.max(1e-15);
                    assert!(err_bil < TOLERANCE);
                }
            }
        }

        println!("✓ Conformality: {} polytopes verified", polytopes.len());
    }

    #[test]
    fn test_4_symplectic_invariance() {
        let mut rng = ChaCha8Rng::seed_from_u64(456);

        // 5 random generic
        let mut polytopes = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

        // 5 random Lagrangian products (retry if unbounded)
        let mut count = 0;
        while count < 5 {
            let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                polytopes.push(p);
                count += 1;
            }
        }

        for (i, p) in polytopes.iter().enumerate() {
            let c_p = ehz_capacity_pruned(p).expect("HK2017 failed").capacity;
            let m = random_sp4_matrix(&mut rng);
            let transformed = apply_symplectomorphism(p, &m);
            let c_transformed = ehz_capacity_pruned(&transformed).expect("HK2017 on transformed failed").capacity;
            let rel_err = (c_transformed - c_p).abs() / c_p.max(1e-15);

            assert!(
                rel_err < TOLERANCE,
                "Polytope #{}: c(MK) = {}, c(K) = {}, rel_err={}",
                i, c_transformed, c_p, rel_err
            );
        }

        println!("✓ Symplectic invariance: {} polytopes verified", polytopes.len());
    }

    #[test]
    fn test_6_monotonicity() {
        let mut rng = ChaCha8Rng::seed_from_u64(999);

        // 5 random generic
        let mut polytopes = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

        // 5 random Lagrangian products (retry if unbounded)
        let mut count = 0;
        while count < 5 {
            let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                polytopes.push(p);
                count += 1;
            }
        }

        // Compute capacities for all polytopes
        let capacities: Vec<f64> = polytopes
            .iter()
            .map(|p| ehz_capacity_pruned(p).expect("HK2017 failed").capacity)
            .collect();

        // For each pair (i, j), find max α s.t. αK_i ⊂ K_j, then check monotonicity
        let mut tested_pairs = 0;
        for i in 0..polytopes.len() {
            for j in 0..polytopes.len() {
                if i == j {
                    continue;
                }

                let k1 = &polytopes[i];
                let k2 = &polytopes[j];

                // Find max α s.t. αK1 ⊂ K2
                // For each vertex v of K1, need α·v ∈ K2
                // K2 = {x : ⟨n_j, x⟩ ≤ h_j} so need α ≤ h_j / ⟨n_j, v⟩ for all j
                let mut alpha_max = f64::INFINITY;
                for vertex in k1.vertices() {
                    for (normal, &height) in k2.normals().iter().zip(k2.heights().iter()) {
                        let dot = normal.dot(vertex);
                        if dot > 1e-10 {
                            // vertex is on the positive side of this facet
                            alpha_max = alpha_max.min(height / dot);
                        }
                    }
                }

                // Only test pairs where α > 0.1 (non-trivial containment)
                if alpha_max > 0.1 && alpha_max < f64::INFINITY {
                    let scaled_capacity = alpha_max * alpha_max * capacities[i];
                    assert!(
                        scaled_capacity <= capacities[j] + TOLERANCE,
                        "Monotonicity failed: α={:.3}, α²·c(K{}) = {:.6} > c(K{}) = {:.6}",
                        alpha_max,
                        i,
                        scaled_capacity,
                        j,
                        capacities[j]
                    );
                    tested_pairs += 1;
                }
            }
        }

        println!(
            "✓ Monotonicity: {} pairs verified (from {} polytopes)",
            tested_pairs,
            polytopes.len()
        );
    }

    #[test]
    fn test_5_continuity() {
        let mut rng = ChaCha8Rng::seed_from_u64(789);

        // 5 random generic
        let mut polytopes = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

        // 5 random Lagrangian products (retry if unbounded)
        let mut count = 0;
        while count < 5 {
            let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                polytopes.push(p);
                count += 1;
            }
        }

        for (i, p) in polytopes.iter().enumerate() {
            let c_p = ehz_capacity_pruned(p).expect("HK2017 failed").capacity;
            let epsilon = 0.01;
            let perturbed_heights: Vec<_> = p
                .heights()
                .iter()
                .map(|&h| h * (1.0 + epsilon * (rng.gen::<f64>() - 0.5)))
                .collect();

            let perturbed = Polytope4D::new(p.normals().to_vec(), perturbed_heights)
                .expect("Perturbed polytope failed");

            let c_perturbed = ehz_capacity_pruned(&perturbed).expect("HK2017 on perturbed failed").capacity;
            let rel_change = (c_perturbed - c_p).abs() / c_p.max(1e-15);

            assert!(
                rel_change < 0.1,
                "Polytope #{}: {}% perturbation → {:.1}% capacity change",
                i, epsilon * 100.0, rel_change * 100.0
            );
        }

        println!("✓ Continuity: {} polytopes verified", polytopes.len());
    }
}
