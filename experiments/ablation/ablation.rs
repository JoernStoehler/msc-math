//! Ablation study: compare HK2017 algorithm variants on a fixed dataset.
//!
//! Phase A (this file): baseline variants imported from the symplectic crate.
//! Phase B (next session): self-contained improvement variants.
//!
//! Goal: Verify V0 (unpruned) and V1 (pruned) agree on all test polytopes,
//!       and measure the pruning speedup.
//!
//! Architecture:
//! 1. `cargo run --bin ablation --release` generates the ablation dataset
//! 2. Writes to ablation/ablation.jsonl
//! 3. Python script reads JSONL, checks agreement, plots timing comparison
//!
//! Dataset:
//! - Random generic polytopes: 5 per F ∈ {5, 6, 7, 8} (seed 42)
//! - Random Lagrangian products: 5 per pair (3×3), (3×4), (4×4) (same seed)
//! - Regression cases: (3,4) θ=0°, (4,4) θ=0°, hypercube (always included)
//!
//! Output format: one JSONL entry per (polytope, variant).
//! Each entry: {polytope_name, variant, group, facet_count, normals, heights,
//!              capacity, capacity_lenient, iterations, time_ms}

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::geom::polygon::{random_polygon_2d, regular_polygon_2d};
use symplectic::random::generate_random_polytopes;
use symplectic::{
    ehz_capacity, ehz_capacity_pruned, known_polytopes, lagrangian_product, EhzResult, Polytope4D,
};

const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;
const N_PER_GROUP: usize = 5;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct AblationEntry {
    polytope_name: String,
    variant: String,        // "v0_unpruned" | "v1_pruned"
    group: String,          // "random_generic" | "random_lagrangian" | "regression"
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    capacity: f64,
    capacity_lenient: f64,
    iterations: u64,
    time_ms: f64,
}

// ============================================================================
// Variant definitions
// ============================================================================

struct Variant {
    name: &'static str,
    run: fn(&Polytope4D) -> Option<EhzResult>,
}

const VARIANTS: &[Variant] = &[
    Variant {
        name: "v0_unpruned",
        run: ehz_capacity,
    },
    Variant {
        name: "v1_pruned",
        run: ehz_capacity_pruned,
    },
];

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ablation/ablation.jsonl");

    println!("Ablation study — Phase A: baselines\n");
    println!("Variants: V0 (unpruned), V1 (pruned)");
    println!("Seed: {SEED}, h ∈ [{H_MIN}, {H_MAX}]\n");

    // =========================================================================
    // Build test polytope set
    // =========================================================================

    // (name, group, polytope, expected_capacity)
    let mut polytopes: Vec<(String, String, Polytope4D, Option<f64>)> = Vec::new();

    // --- Part 1: Random generic polytopes, F=5..8 ---
    println!("Part 1: Random generic polytopes (F=5..8, {N_PER_GROUP} each)...");
    for f in [5usize, 6, 7, 8] {
        let ps = generate_random_polytopes(N_PER_GROUP, f, H_MIN, H_MAX, &mut rng);
        for (i, p) in ps.into_iter().enumerate() {
            polytopes.push((
                format!("random_F{f}_{i}"),
                "random_generic".to_string(),
                p,
                None,
            ));
        }
        println!("  F={f}: {N_PER_GROUP} polytopes");
    }

    // --- Part 2: Random Lagrangian products, (3×3)/(3×4)/(4×4) ---
    println!("\nPart 2: Random Lagrangian products ({N_PER_GROUP} per pair)...");
    for (n, m) in [(3usize, 3usize), (3, 4), (4, 4)] {
        for i in 0..N_PER_GROUP {
            let p = loop {
                let (qn, qh) = random_polygon_2d(n, H_MIN, H_MAX, &mut rng);
                let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
                if let Ok(poly) = lagrangian_product(&qn, &qh, &pn, &ph) {
                    break poly;
                }
            };
            polytopes.push((
                format!("random_lagrangian_{n}x{m}_{i}"),
                "random_lagrangian".to_string(),
                p,
                None,
            ));
        }
        println!("  ({n}×{m}): {N_PER_GROUP} polytopes (F={})", n + m);
    }

    // --- Part 3: Regression cases ---
    println!("\nPart 3: Regression cases...");

    // (3,4) θ=0° — null-space fix case (before fix: returned None)
    // Expected: 3√2/2 ≈ 2.121 (triangle circumradius=1, square circumradius=1)
    {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = lagrangian_product(&qn, &qh, &pn, &ph).expect("(3,4) construction");
        let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0;
        polytopes.push((
            "regression_34_theta0".to_string(),
            "regression".to_string(),
            p,
            Some(expected),
        ));
        println!("  (3,4) θ=0°: F=7, expected {expected:.6}");
    }

    // (4,4) θ=0° — SVD gap-threshold case (LU falls through to SVD for degenerate KKT)
    // Expected: 2.0 (square circumradius=1, this is the hypercube scaled by 1/√2)
    {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let p = lagrangian_product(&qn, &qh, &pn, &ph).expect("(4,4) construction");
        polytopes.push((
            "regression_44_theta0".to_string(),
            "regression".to_string(),
            p,
            Some(2.0),
        ));
        println!("  (4,4) θ=0°: F=8, expected 2.0");
    }

    // Hypercube [-1,1]^4 — LU residual check case (no SVD needed)
    // Expected: 4.0 (from HK2019 Ex 4.6)
    {
        let kp = known_polytopes::hypercube();
        println!(
            "  hypercube:  F={}, expected {}",
            kp.polytope.facet_count(),
            kp.capacity
        );
        polytopes.push((
            "regression_hypercube".to_string(),
            "regression".to_string(),
            kp.polytope,
            Some(kp.capacity),
        ));
    }

    let n_polytopes = polytopes.len();
    let n_entries = n_polytopes * VARIANTS.len();
    println!("\nTotal: {n_polytopes} polytopes × {} variants = {n_entries} entries\n", VARIANTS.len());

    // =========================================================================
    // Run ablation variants
    // =========================================================================

    let mut entries: Vec<AblationEntry> = Vec::with_capacity(n_entries);
    let mut n_disagreements = 0usize;
    let mut n_failures = 0usize;

    for (polytope_name, group, polytope, expected) in &polytopes {
        let normals_raw: Vec<[f64; 4]> = polytope
            .normals()
            .iter()
            .map(|n| [n[0], n[1], n[2], n[3]])
            .collect();
        let heights_raw = polytope.heights().to_vec();
        let f = polytope.facet_count();

        // Collect results for this polytope to check agreement
        let mut capacities: Vec<(String, f64)> = Vec::new();

        for variant in VARIANTS {
            let t_start = Instant::now();
            let result = (variant.run)(polytope);
            let time_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            match result {
                None => {
                    eprintln!("  FAILURE: {} / {} returned None", polytope_name, variant.name);
                    n_failures += 1;
                }
                Some(r) => {
                    capacities.push((variant.name.to_string(), r.capacity));

                    // Check against known expected capacity
                    if let Some(exp) = expected {
                        if (r.capacity - exp).abs() > 1e-5 {
                            eprintln!(
                                "  WRONG: {} / {}: got {:.8}, expected {:.8} (diff={:.2e})",
                                polytope_name,
                                variant.name,
                                r.capacity,
                                exp,
                                (r.capacity - exp).abs()
                            );
                            n_disagreements += 1;
                        }
                    }

                    entries.push(AblationEntry {
                        polytope_name: polytope_name.clone(),
                        variant: variant.name.to_string(),
                        group: group.clone(),
                        facet_count: f,
                        normals: normals_raw.clone(),
                        heights: heights_raw.clone(),
                        capacity: r.capacity,
                        capacity_lenient: r.capacity_lenient,
                        iterations: r.iterations,
                        time_ms,
                    });
                }
            }
        }

        // Check V0 vs V1 agreement
        if capacities.len() == 2 {
            let (_, c0) = &capacities[0];
            let (_, c1) = &capacities[1];
            if (c0 - c1).abs() > 1e-5 {
                eprintln!(
                    "  DISAGREE: {} V0={:.8} V1={:.8} (diff={:.2e})",
                    polytope_name,
                    c0,
                    c1,
                    (c0 - c1).abs()
                );
                n_disagreements += 1;
            }
        }
    }

    // =========================================================================
    // Write JSONL output
    // =========================================================================

    let file = File::create(&output_path).expect("failed to create ablation.jsonl");
    let mut writer = BufWriter::new(file);

    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).expect("failed to serialize entry");
        writeln!(writer).expect("failed to write newline");
    }

    writer.flush().expect("failed to flush output");

    // =========================================================================
    // Summary
    // =========================================================================

    let total_time = t0.elapsed().as_secs_f64();
    println!("Results:");
    println!("  Entries written:  {}", entries.len());
    println!("  Disagreements:    {n_disagreements}");
    println!("  Failures (None):  {n_failures}");
    println!("  Total time:       {total_time:.1}s");
    println!();
    println!("Output: {}", output_path.display());

    if n_disagreements > 0 || n_failures > 0 {
        eprintln!("\nABLATION ISSUES FOUND: {n_disagreements} disagreements, {n_failures} failures");
        std::process::exit(1);
    } else {
        println!("\nAll variants agree. Ready for Python analysis.");
    }
}
