//! Correctness verification: dataset generator and tests.
//!
//! Goal: Verify correctness of capacity algorithms via 6 mathematical propositions
//!   (direct comparison, literature agreement, conformality, symplectic invariance,
//!   continuity, monotonicity).
//! Input Artifacts: None (generates polytopes from hardcoded seeds and literature definitions).
//! Output Artifacts: experiments/verification/correctness/correctness.jsonl (47 polytopes, 71 capacity values).
//!
//! Architecture:
//! 1. `cargo run -p dev-capacity-validation --release --bin axioms-correctness` generates
//!    47 polytopes, computes 71 capacities
//! 2. Writes to correctness/correctness.jsonl
//! 3. `cargo test -p dev-capacity-validation --bin axioms-correctness --release` reads
//!    dataset and verifies properties
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
//!
//! Capacity routing is intentionally explicit in this file because the dataset
//! compares pruned, unpruned, and billiard outputs on the same verification
//! fixtures. The crate-level `ehz_capacity` entrypoint would hide those
//! per-algorithm checks.

use dev_capacity_validation::{
    capacity_billiard, capacity_pruned_hk2017, capacity_unpruned_hk2017, VerificationPolytopeCache,
};
use nalgebra::Matrix4;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use symplectic::classify_facets_from_dual_vertices;
use symplectic::geom::known_polytopes::{
    hko_pentagon, hypercube, lagrangian_triangle_product, lagrangian_triangle_square, simplex,
    symplectic_triangle_product, symplectic_triangle_square,
};
use symplectic::geom::polygon::random_polygon_2d;

fn capacity_pruned(polytope: &VerificationPolytopeCache) -> f64 {
    capacity_pruned_hk2017(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .expect("pruned")
    .capacity()
}

fn capacity_unpruned(polytope: &VerificationPolytopeCache) -> f64 {
    capacity_unpruned_hk2017(&polytope.dual_vertices_f64, &polytope.dual_vertices)
        .expect("unpruned")
        .capacity()
}

fn maybe_billiard_capacity(polytope: &VerificationPolytopeCache) -> Option<f64> {
    if classify_facets_from_dual_vertices(&polytope.dual_vertices_f64).is_err() {
        return None;
    }
    Some(
        capacity_billiard(
            &polytope.dual_vertices_f64,
            &polytope.dual_vertices,
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        )
        .expect("classification already succeeded")
        .capacity(),
    )
}

fn dual_vertices_row(polytope: &VerificationPolytopeCache) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationEntry {
    name: String,
    test_group: String, // "base", "literature", "scaled", "transformed", "perturbed"
    base_index: Option<usize>, // For scaled/transformed/perturbed: index into base group
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    capacity_pruned: f64,
    capacity_unpruned: Option<f64>,
    capacity_billiard: Option<f64>,
    alpha: Option<f64>,             // For scaled polytopes
    expected_capacity: Option<f64>, // For literature
}

fn print_usage() {
    eprintln!("Usage: cargo run -p dev-capacity-validation --release --bin axioms-correctness");
    eprintln!("  Refreshes correctness/correctness.jsonl.");
    eprintln!("  Use cargo test -p dev-capacity-validation --bin axioms-correctness --release to check it.");
}

fn parse_args() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return;
    }
    if args == ["--help"] || args == ["-h"] {
        print_usage();
        std::process::exit(0);
    }
    eprintln!("unknown argument(s): {}\n", args.join(" "));
    print_usage();
    std::process::exit(2);
}

fn main() {
    parse_args();

    println!("Generating correctness dataset (47 polytopes, 71 capacity values)...\n");
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut entries = Vec::new();

    // Construct output path relative to repo root (works from any cwd)
    let output_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("correctness/correctness.jsonl");

    // Test 1: Generate 10 base polytopes (5 random generic + 5 Lagrangian products)
    println!("Test 1: Generating 10 base polytopes (5 generic + 5 Lagrangian)...");
    let mut base_generic = Vec::new();
    while base_generic.len() < 5 {
        if let Some(p) = VerificationPolytopeCache::sample_random(5, 0.5, 2.0, &mut rng) {
            base_generic.push(p);
        }
    }

    let mut base_lagrangian = Vec::new();
    while base_lagrangian.len() < 5 {
        let (qn, qh) = random_polygon_2d(3, 0.5, 2.0, &mut rng);
        let (pn, ph) = random_polygon_2d(4, 0.5, 2.0, &mut rng);
        if let Some(p) = VerificationPolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph) {
            base_lagrangian.push(p);
        }
    }

    let mut base_polytopes = Vec::new();
    base_polytopes.extend(base_generic);
    base_polytopes.extend(base_lagrangian);

    // Compute capacities for base polytopes (10 pruned + 10 unpruned + 5 billiard)
    for (i, p) in base_polytopes.iter().enumerate() {
        let pruned = capacity_pruned(p);
        let unpruned = capacity_unpruned(p);
        let billiard = if i >= 5 {
            maybe_billiard_capacity(p)
        } else {
            None
        };

        entries.push(VerificationEntry {
            name: format!("base_{}", i),
            test_group: "base".to_string(),
            base_index: None,
            facet_count: p.facet_count(),
            dual_vertices: dual_vertices_row(p),
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
        let polytope = VerificationPolytopeCache::from_f64_dual_vertices(
            kp.polytope.dual_vertices_f64().to_vec(),
        )
        .expect("known literature polytope should reconstruct");
        let pruned = capacity_pruned(&polytope);
        let billiard = maybe_billiard_capacity(&polytope);

        entries.push(VerificationEntry {
            name: kp.name.to_string(),
            test_group: "literature".to_string(),
            base_index: None,
            facet_count: polytope.facet_count(),
            dual_vertices: dual_vertices_row(&polytope),
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
        let scaled = VerificationPolytopeCache::from_f64_dual_vertices(
            p.dual_vertices_f64.iter().map(|a| a / alpha).collect(),
        )
        .expect("scaled");

        let pruned = capacity_pruned(&scaled);
        let billiard = if i >= 5 {
            maybe_billiard_capacity(&scaled)
        } else {
            None
        };

        entries.push(VerificationEntry {
            name: format!("scaled_{}", i),
            test_group: "scaled".to_string(),
            base_index: Some(i),
            facet_count: scaled.facet_count(),
            dual_vertices: dual_vertices_row(&scaled),
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
        let pruned = capacity_pruned(&transformed);

        entries.push(VerificationEntry {
            name: format!("transformed_{}", i),
            test_group: "transformed".to_string(),
            base_index: Some(i),
            facet_count: transformed.facet_count(),
            dual_vertices: dual_vertices_row(&transformed),
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
        let perturbed = VerificationPolytopeCache::from_f64_dual_vertices(
            p.dual_vertices_f64
                .iter()
                .map(|a| {
                    // Perturb height h_i → h_i * (1 + ε·δ), so a_i → a_i / (1 + ε·δ)
                    let delta = rng.gen::<f64>() - 0.5;
                    a / (1.0 + epsilon * delta)
                })
                .collect(),
        )
        .expect("perturbed");
        let pruned = capacity_pruned(&perturbed);

        entries.push(VerificationEntry {
            name: format!("perturbed_{}", i),
            test_group: "perturbed".to_string(),
            base_index: Some(i),
            facet_count: perturbed.facet_count(),
            dual_vertices: dual_vertices_row(&perturbed),
            capacity_pruned: pruned,
            capacity_unpruned: None,
            capacity_billiard: None,
            alpha: None,
            expected_capacity: None,
        });
    }
    println!("  → 10 pruned = 10 capacity values\n");

    // Write to JSONL
    println!(
        "Writing {} entries to {}...",
        entries.len(),
        output_path.display()
    );
    let file = File::create(&output_path).unwrap_or_else(|err| {
        panic!(
            "failed to create correctness output {}: {err}",
            output_path.display()
        )
    });
    let mut writer = BufWriter::new(file);
    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).unwrap_or_else(|err| {
            panic!(
                "failed to serialize correctness row for {}: {err}",
                output_path.display()
            )
        });
        writeln!(&mut writer).unwrap_or_else(|err| {
            panic!(
                "failed to write newline to correctness output {}: {err}",
                output_path.display()
            )
        });
    }
    writer.flush().unwrap_or_else(|err| {
        panic!(
            "failed to flush correctness output {}: {err}",
            output_path.display()
        )
    });

    let total_pruned = entries.len();
    let total_unpruned = entries
        .iter()
        .filter(|e| e.capacity_unpruned.is_some())
        .count();
    let total_billiard = entries
        .iter()
        .filter(|e| e.capacity_billiard.is_some())
        .count();

    println!("\n✓ Dataset complete:");
    println!("  Polytopes: {}", entries.len());
    println!(
        "  Capacity values: {} pruned + {} unpruned + {} billiard = {} total",
        total_pruned,
        total_unpruned,
        total_billiard,
        total_pruned + total_unpruned + total_billiard
    );
}

fn apply_symplectomorphism(
    p: &VerificationPolytopeCache,
    m: &Matrix4<f64>,
) -> VerificationPolytopeCache {
    // For symplectomorphism M: a' = M^{-T} a
    let m_inv_t = m.transpose().try_inverse().expect("invertible");
    let new_duals: Vec<_> = p.dual_vertices_f64.iter().map(|a| m_inv_t * a).collect();
    VerificationPolytopeCache::from_f64_dual_vertices(new_duals).expect("transformed")
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
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("correctness/correctness.jsonl");
        let file = File::open(&path).expect(
            "Run `cargo run -p dev-capacity-validation --release --bin axioms-correctness` first",
        );
        let reader = BufReader::new(file);
        reader
            .lines()
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
            assert!(
                (pruned - unpruned).abs() / pruned < TOL,
                "{}: pruned≠unpruned",
                entry.name
            );

            if let Some(bil) = entry.capacity_billiard {
                assert!(
                    (pruned - bil).abs() / pruned < TOL,
                    "{}: pruned≠billiard",
                    entry.name
                );
                billiard_count += 1;
            }
        }
        assert_eq!(
            billiard_count, 5,
            "Expected exactly 5 Lagrangian products with billiard values"
        );
    }

    #[test]
    fn test_2_literature() {
        let dataset = load_dataset();
        let lit: Vec<_> = dataset
            .iter()
            .filter(|e| e.test_group == "literature")
            .collect();
        assert_eq!(lit.len(), 7, "Expected 7 literature polytopes");

        let mut billiard_count = 0;
        for entry in lit {
            let expected = entry.expected_capacity.expect("expected missing");
            assert!(
                (entry.capacity_pruned - expected).abs() / expected < TOL,
                "{}: computed≠published",
                entry.name
            );

            if let Some(bil) = entry.capacity_billiard {
                assert!(
                    (bil - expected).abs() / expected < TOL,
                    "{}: billiard≠published",
                    entry.name
                );
                billiard_count += 1;
            }
        }
        assert_eq!(
            billiard_count, 4,
            "Expected exactly 4 Lagrangian products with billiard values"
        );
    }

    #[test]
    fn test_3_conformality() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let scaled: Vec<_> = dataset
            .iter()
            .filter(|e| e.test_group == "scaled")
            .collect();
        assert_eq!(scaled.len(), 10, "Expected 10 scaled polytopes");

        let mut billiard_count = 0;
        for entry in scaled {
            let base_entry = &base[entry.base_index.unwrap()];
            let alpha = entry.alpha.unwrap();
            let expected = alpha * alpha * base_entry.capacity_pruned;
            assert!(
                (entry.capacity_pruned - expected).abs() / expected < TOL,
                "{}: c(αK)≠α²c(K)",
                entry.name
            );

            // If base has billiard, scaled should too (and satisfy conformality)
            if let Some(base_bil) = base_entry.capacity_billiard {
                let bil = entry
                    .capacity_billiard
                    .expect("scaled Lagrangian missing billiard");
                let expected_bil = alpha * alpha * base_bil;
                assert!(
                    (bil - expected_bil).abs() / expected_bil < TOL,
                    "{}: billiard c(αK)≠α²c(K)",
                    entry.name
                );
                billiard_count += 1;
            }
        }
        assert_eq!(
            billiard_count, 5,
            "Expected exactly 5 scaled Lagrangian products with billiard"
        );
    }

    #[test]
    fn test_4_symplectic_invariance() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let transformed: Vec<_> = dataset
            .iter()
            .filter(|e| e.test_group == "transformed")
            .collect();
        assert_eq!(transformed.len(), 10);

        for entry in transformed {
            let base_entry = &base[entry.base_index.unwrap()];
            assert!(
                (entry.capacity_pruned - base_entry.capacity_pruned).abs()
                    / base_entry.capacity_pruned
                    < TOL,
                "{}: c(MK)≠c(K)",
                entry.name
            );
        }
    }

    #[test]
    fn test_5_continuity() {
        let dataset = load_dataset();
        let base: Vec<_> = dataset.iter().filter(|e| e.test_group == "base").collect();
        let perturbed: Vec<_> = dataset
            .iter()
            .filter(|e| e.test_group == "perturbed")
            .collect();
        assert_eq!(perturbed.len(), 10);

        for entry in perturbed {
            let base_entry = &base[entry.base_index.unwrap()];
            let rel_change = (entry.capacity_pruned - base_entry.capacity_pruned).abs()
                / base_entry.capacity_pruned;
            assert!(
                rel_change < 0.1,
                "{}: 1% perturbation → {:.1}% change",
                entry.name,
                rel_change * 100.0
            );
        }
    }

    #[test]
    fn test_6_monotonicity() {
        let dataset = load_dataset();
        let mut tested = 0;

        for i in 0..dataset.len() {
            for j in 0..dataset.len() {
                if i == j {
                    continue;
                }
                let k1 = &dataset[i];
                let k2 = &dataset[j];

                let p1 = VerificationPolytopeCache::from_f64_dual_vertices(
                    k1.dual_vertices
                        .iter()
                        .map(|a| Vector4::from_row_slice(a))
                        .collect(),
                )
                .expect("p1");
                let p2 = VerificationPolytopeCache::from_f64_dual_vertices(
                    k2.dual_vertices
                        .iter()
                        .map(|a| Vector4::from_row_slice(a))
                        .collect(),
                )
                .expect("p2");

                // Containment check: a · v ≤ 1 for all dual vertices a of K2
                // α_max = min over v,a of 1 / (a · v) when a · v > 0
                let mut alpha_max = f64::INFINITY;
                for v in &p1.vertices_f64 {
                    for a in &p2.dual_vertices_f64 {
                        let dot = a.dot(v);
                        if dot > 1e-10 {
                            alpha_max = alpha_max.min(1.0 / dot);
                        }
                    }
                }

                if alpha_max > 0.1 && alpha_max < f64::INFINITY {
                    let scaled_cap = alpha_max * alpha_max * k1.capacity_pruned;
                    assert!(
                        scaled_cap <= k2.capacity_pruned + TOL,
                        "{}⊂{}: α²c(K1)>c(K2)",
                        k1.name,
                        k2.name
                    );
                    tested += 1;
                }
            }
        }
        assert!(tested >= 20, "Only {} monotonicity pairs tested", tested);
    }
}
