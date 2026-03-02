//! Correctness verification: dataset generator and tests.
//!
//! Goal: Verify correctness of capacity algorithms via 6 mathematical propositions
//!   (direct comparison, literature agreement, conformality, symplectic invariance,
//!   continuity, monotonicity).
//! Input: None (generates polytopes from hardcoded seeds and literature definitions).
//! Output: experiments/correctness/correctness.jsonl (47 polytopes, 71 capacity values).
//!
//! Architecture:
//! 1. `cargo run --bin correctness --release` generates 47 polytopes, computes 71 capacities
//! 2. Writes to correctness/correctness.jsonl
//! 3. `cargo test --bin correctness --release` reads dataset and verifies properties
//!
//! Polytope breakdown:
//! - 10 base polytopes (5 random generic + 5 Lagrangian products)
//! - 7 literature polytopes
//! - 10 scaled (from base, with random α ∈ [0.5, 2.0])
//! - 10 transformed (from base, with random M ∈ Sp(4))
//! - 10 perturbed (from base, with 1% height perturbation)
//!
//! Total: 47 polytopes
//!
//! Capacity breakdown:
//! - Base: 10 pruned + 10 unpruned + 5 billiard (Lagrangian only)
//! - Literature: 7 pruned + 4 billiard (Lagrangian only)
//! - Scaled: 10 pruned + 5 billiard (Lagrangian only)
//! - Transformed: 10 pruned
//! - Perturbed: 10 pruned
//!
//! Total: 47 pruned + 10 unpruned + 14 billiard = 71 capacity values

use symplectic::billiard_capacity;
use symplectic::random::generate_random_polytopes;
use symplectic::known_polytopes::{
    hko_pentagon, hypercube, lagrangian_triangle_product, lagrangian_triangle_square,
    simplex, symplectic_triangle_product, symplectic_triangle_square,
};
use symplectic::lagrangian_product;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::Polytope4D;
use symplectic::{ehz_capacity_unpruned, ehz_capacity};
use nalgebra::Matrix4;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Serialize, Deserialize)]
struct VerificationEntry {
    name: String,
    test_group: String, // "base", "literature", "scaled", "transformed", "perturbed"
    base_index: Option<usize>, // For scaled/transformed/perturbed: index into base group
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    capacity_pruned: f64,
    capacity_unpruned: Option<f64>,
    capacity_billiard: Option<f64>,
    alpha: Option<f64>, // For scaled polytopes
    expected_capacity: Option<f64>, // For literature
}

fn main() {
    println!("Generating correctness dataset (47 polytopes, 71 capacity values)...\n");
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut entries = Vec::new();
    
    // Construct output path relative to repo root (works from any cwd)
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("correctness/correctness.jsonl");

    // Test 1: Generate 10 base polytopes (5 random generic + 5 Lagrangian products)
    println!("Test 1: Generating 10 base polytopes (5 generic + 5 Lagrangian)...");
    let base_generic = generate_random_polytopes(5, 5, 0.5, 2.0, &mut rng);

    let mut base_lagrangian = Vec::new();
    while base_lagrangian.len() < 5 {
        let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
        let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
        if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
            base_lagrangian.push(p);
        }
    }

    let mut base_polytopes = Vec::new();
    base_polytopes.extend(base_generic);
    base_polytopes.extend(base_lagrangian);

    // Compute capacities for base polytopes (10 pruned + 10 unpruned + 5 billiard)
    for (i, p) in base_polytopes.iter().enumerate() {
        let pruned = ehz_capacity(p).expect("pruned").capacity;
        let unpruned = ehz_capacity_unpruned(p).expect("unpruned").capacity;
        let billiard = if i >= 5 {
            billiard_capacity(p).ok().flatten().map(|r| r.capacity)
        } else {
            None
        };

        entries.push(VerificationEntry {
            name: format!("base_{}", i),
            test_group: "base".to_string(),
            base_index: None,
            facet_count: p.facet_count(),
            normals: p.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: p.heights().to_vec(),
            capacity_pruned: pruned,
            capacity_unpruned: Some(unpruned),
            capacity_billiard: billiard,
            alpha: None,
            expected_capacity: None,
        });
    }
    println!("  → 10 pruned + 10 unpruned + 5 billiard = 25 capacity values\n");

    // Test 2: Literature polytopes (7 pruned + 4 billiard)
    println!("Test 2: Processing 7 literature polytopes...");
    let literature = vec![
        simplex(),
        hypercube(),
        hko_pentagon(),
        lagrangian_triangle_product(),
        symplectic_triangle_product(),
        lagrangian_triangle_square(),
        symplectic_triangle_square(),
    ];

    for kp in literature {
        let pruned = ehz_capacity(&kp.polytope).expect("pruned").capacity;
        let billiard = billiard_capacity(&kp.polytope).ok().flatten().map(|r| r.capacity);

        entries.push(VerificationEntry {
            name: kp.name.to_string(),
            test_group: "literature".to_string(),
            base_index: None,
            facet_count: kp.polytope.facet_count(),
            normals: kp.polytope.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: kp.polytope.heights().to_vec(),
            capacity_pruned: pruned,
            capacity_unpruned: None,
            capacity_billiard: billiard,
            alpha: None,
            expected_capacity: Some(kp.capacity),
        });
    }
    println!("  → 7 pruned + 4 billiard = 11 capacity values\n");

    // Test 3: Scaled polytopes - REUSE base polytopes (10 pruned + 5 billiard)
    println!("Test 3: Generating 10 scaled polytopes (reusing base)...");
    for (i, p) in base_polytopes.iter().enumerate() {
        let alpha: f64 = rng.gen_range(0.5..2.0);
        let scaled = Polytope4D::new(
            p.normals().to_vec(),
            p.heights().iter().map(|&h| alpha * h).collect(),
        ).expect("scaled");

        let pruned = ehz_capacity(&scaled).expect("pruned").capacity;
        let billiard = if i >= 5 {
            billiard_capacity(&scaled).ok().flatten().map(|r| r.capacity)
        } else {
            None
        };

        entries.push(VerificationEntry {
            name: format!("scaled_{}", i),
            test_group: "scaled".to_string(),
            base_index: Some(i),
            facet_count: scaled.facet_count(),
            normals: scaled.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: scaled.heights().to_vec(),
            capacity_pruned: pruned,
            capacity_unpruned: None,
            capacity_billiard: billiard,
            alpha: Some(alpha),
            expected_capacity: None,
        });
    }
    println!("  → 10 pruned + 5 billiard = 15 capacity values\n");

    // Test 4: Transformed polytopes - REUSE base polytopes (10 pruned)
    println!("Test 4: Generating 10 transformed polytopes (reusing base)...");
    for (i, p) in base_polytopes.iter().enumerate() {
        let m = random_sp4_matrix(&mut rng);
        let transformed = apply_symplectomorphism(p, &m);
        let pruned = ehz_capacity(&transformed).expect("pruned").capacity;

        entries.push(VerificationEntry {
            name: format!("transformed_{}", i),
            test_group: "transformed".to_string(),
            base_index: Some(i),
            facet_count: transformed.facet_count(),
            normals: transformed.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: transformed.heights().to_vec(),
            capacity_pruned: pruned,
            capacity_unpruned: None,
            capacity_billiard: None,
            alpha: None,
            expected_capacity: None,
        });
    }
    println!("  → 10 pruned = 10 capacity values\n");

    // Test 5: Perturbed polytopes - REUSE base polytopes (10 pruned)
    println!("Test 5: Generating 10 perturbed polytopes (reusing base)...");
    for (i, p) in base_polytopes.iter().enumerate() {
        let epsilon = 0.01;
        let perturbed_heights: Vec<_> = p.heights()
            .iter()
            .map(|&h| h * (1.0 + epsilon * (rng.gen::<f64>() - 0.5)))
            .collect();

        let perturbed = Polytope4D::new(p.normals().to_vec(), perturbed_heights)
            .expect("perturbed");
        let pruned = ehz_capacity(&perturbed).expect("pruned").capacity;

        entries.push(VerificationEntry {
            name: format!("perturbed_{}", i),
            test_group: "perturbed".to_string(),
            base_index: Some(i),
            facet_count: perturbed.facet_count(),
            normals: perturbed.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: perturbed.heights().to_vec(),
            capacity_pruned: pruned,
            capacity_unpruned: None,
            capacity_billiard: None,
            alpha: None,
            expected_capacity: None,
        });
    }
    println!("  → 10 pruned = 10 capacity values\n");

    // Write to JSONL
    println!("Writing {} entries to {}...", entries.len(), output_path.display());
    let file = File::create(output_path).expect("create file");
    let mut writer = BufWriter::new(file);
    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).expect("write entry");
        writeln!(&mut writer).expect("write newline");
    }

    let total_pruned = entries.len();
    let total_unpruned = entries.iter().filter(|e| e.capacity_unpruned.is_some()).count();
    let total_billiard = entries.iter().filter(|e| e.capacity_billiard.is_some()).count();

    println!("\n✓ Dataset complete:");
    println!("  Polytopes: {}", entries.len());
    println!("  Capacity values: {} pruned + {} unpruned + {} billiard = {} total",
             total_pruned, total_unpruned, total_billiard,
             total_pruned + total_unpruned + total_billiard);
}

fn apply_symplectomorphism(p: &Polytope4D, m: &Matrix4<f64>) -> Polytope4D {
    let m_inv_t = m.transpose().try_inverse().expect("invertible");
    let normals: Vec<_> = p.normals().iter().map(|n| {
        let n_raw = m_inv_t * n;
        n_raw / n_raw.norm()
    }).collect();
    let heights: Vec<_> = p.normals().iter().zip(p.heights().iter()).map(|(n, &h)| {
        let n_raw = m_inv_t * n;
        h / n_raw.norm()
    }).collect();
    Polytope4D::new(normals, heights).expect("transformed")
}

fn random_sp4_matrix(rng: &mut impl Rng) -> Matrix4<f64> {
    let p11: f64 = rng.sample(StandardNormal);
    let p12: f64 = rng.sample(StandardNormal);
    let p21: f64 = rng.sample(StandardNormal);
    let p22: f64 = rng.sample(StandardNormal);
    let q11: f64 = rng.sample(StandardNormal);
    let q12: f64 = rng.sample(StandardNormal);
    let q22: f64 = rng.sample(StandardNormal);
    let r11: f64 = rng.sample(StandardNormal);
    let r12: f64 = rng.sample(StandardNormal);
    let r22: f64 = rng.sample(StandardNormal);

    #[rustfmt::skip]
    let a = Matrix4::new(
        p11, p12, q11, q12,
        p21, p22, q12, q22,
        r11, r12, -p11, -p21,
        r12, r22, -p12, -p22,
    );

    let i = Matrix4::identity();
    let i_minus_a = i - a;
    let i_plus_a = i + a;
    i_minus_a * i_plus_a.try_inverse().expect("invertible")
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector4;
    use std::io::{BufRead, BufReader};
    const TOL: f64 = 1e-6;

    fn load_dataset() -> Vec<VerificationEntry> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("correctness/correctness.jsonl");
        let file = File::open(&path)
            .expect("Run `cargo run --bin correctness --release` first");
        let reader = BufReader::new(file);
        reader.lines()
            .map(|line| {
                let line_str = line.expect("read line");
                serde_json::from_str::<VerificationEntry>(&line_str).expect("parse")
            })
            .collect()
    }

    #[test]
    fn test_1_direct_comparison() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        assert_eq!(base.len(), 10, "Expected 10 base polytopes");

        let mut billiard_count = 0;
        for entry in base {
            let pruned = entry.capacity_pruned;
            let unpruned = entry.capacity_unpruned.expect("unpruned missing");
            assert!((pruned - unpruned).abs() / pruned < TOL,
                "{}: pruned≠unpruned", entry.name);

            if let Some(bil) = entry.capacity_billiard {
                assert!((pruned - bil).abs() / pruned < TOL,
                    "{}: pruned≠billiard", entry.name);
                billiard_count += 1;
            }
        }
        assert_eq!(billiard_count, 5, "Expected exactly 5 Lagrangian products with billiard values");
    }

    #[test]
    fn test_2_literature() {
        let dataset = load_dataset();
        let lit: Vec<_> = dataset.iter().filter(|e| e.test_group == "literature").collect();
        assert_eq!(lit.len(), 7, "Expected 7 literature polytopes");

        let mut billiard_count = 0;
        for entry in lit {
            let expected = entry.expected_capacity.expect("expected missing");
            assert!((entry.capacity_pruned - expected).abs() / expected < TOL,
                "{}: computed≠published", entry.name);

            if let Some(bil) = entry.capacity_billiard {
                assert!((bil - expected).abs() / expected < TOL,
                    "{}: billiard≠published", entry.name);
                billiard_count += 1;
            }
        }
        assert_eq!(billiard_count, 4, "Expected exactly 4 Lagrangian products with billiard values");
    }

    #[test]
    fn test_3_conformality() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let scaled: Vec<_> = dataset.iter().filter(|e| e.test_group == "scaled").collect();
        assert_eq!(scaled.len(), 10, "Expected 10 scaled polytopes");

        let mut billiard_count = 0;
        for entry in scaled {
            let base_entry = &base[entry.base_index.unwrap()];
            let alpha = entry.alpha.unwrap();
            let expected = alpha * alpha * base_entry.capacity_pruned;
            assert!((entry.capacity_pruned - expected).abs() / expected < TOL,
                "{}: c(αK)≠α²c(K)", entry.name);

            // If base has billiard, scaled should too (and satisfy conformality)
            if let Some(base_bil) = base_entry.capacity_billiard {
                let bil = entry.capacity_billiard.expect("scaled Lagrangian missing billiard");
                let expected_bil = alpha * alpha * base_bil;
                assert!((bil - expected_bil).abs() / expected_bil < TOL,
                    "{}: billiard c(αK)≠α²c(K)", entry.name);
                billiard_count += 1;
            }
        }
        assert_eq!(billiard_count, 5, "Expected exactly 5 scaled Lagrangian products with billiard");
    }

    #[test]
    fn test_4_symplectic_invariance() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let transformed: Vec<_> = dataset.iter().filter(|e| e.test_group == "transformed").collect();
        assert_eq!(transformed.len(), 10);

        for entry in transformed {
            let base_entry = &base[entry.base_index.unwrap()];
            assert!((entry.capacity_pruned - base_entry.capacity_pruned).abs() / base_entry.capacity_pruned < TOL,
                "{}: c(MK)≠c(K)", entry.name);
        }
    }

    #[test]
    fn test_5_continuity() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let perturbed: Vec<_> = dataset.iter().filter(|e| e.test_group == "perturbed").collect();
        assert_eq!(perturbed.len(), 10);

        for entry in perturbed {
            let base_entry = &base[entry.base_index.unwrap()];
            let rel_change = (entry.capacity_pruned - base_entry.capacity_pruned).abs() / base_entry.capacity_pruned;
            assert!(rel_change < 0.1, "{}: 1% perturbation → {:.1}% change",
                entry.name, rel_change * 100.0);
        }
    }

    #[test]
    fn test_6_monotonicity() {
        let dataset = load_dataset();
        let mut tested = 0;

        for i in 0..dataset.len() {
            for j in 0..dataset.len() {
                if i == j { continue; }
                let k1 = &dataset[i];
                let k2 = &dataset[j];

                let p1 = Polytope4D::new(
                    k1.normals.iter().map(|n| Vector4::from_row_slice(n)).collect(),
                    k1.heights.clone()
                ).expect("p1");
                let p2 = Polytope4D::new(
                    k2.normals.iter().map(|n| Vector4::from_row_slice(n)).collect(),
                    k2.heights.clone()
                ).expect("p2");

                let mut alpha_max = f64::INFINITY;
                for v in p1.vertices() {
                    for (n, &h) in p2.normals().iter().zip(p2.heights().iter()) {
                        let dot = n.dot(v);
                        if dot > 1e-10 {
                            alpha_max = alpha_max.min(h / dot);
                        }
                    }
                }

                if alpha_max > 0.1 && alpha_max < f64::INFINITY {
                    let scaled_cap = alpha_max * alpha_max * k1.capacity_pruned;
                    assert!(scaled_cap <= k2.capacity_pruned + TOL,
                        "{}⊂{}: α²c(K1)>c(K2)", k1.name, k2.name);
                    tested += 1;
                }
            }
        }
        assert!(tested >= 20, "Only {} monotonicity pairs tested", tested);
    }
}
