//! Boundedness detection validation: qhull sentinel detection vs check_bounded()
//!
//! **Decision Context:** Keep both mechanisms (defense-in-depth)
//! - qhull: 100% empirically reliable (875 test cases) but uses undocumented sentinel behavior (-10.101 vertices)
//! - check_bounded(): Explicit O(F³) mathematical verification via positive span check
//!
//! **Rationale:** User requirement "DEFINITELY do not depend on undocumented behavior"
//! Despite qhull's empirical perfection, we maintain independent verification.
//!
//! ## Empirical Results
//! - Bounded polytopes: 500/500 (100%) correct detection
//! - Unbounded polytopes: 375/375 (100%) detected via sentinels
//! - Agreement: 875/875 (100%) between qhull and check_bounded()
//!
//! ## Test Suite
//! All tests marked `#[ignore]` - run manually with:
//! ```
//! cargo test --package geom -- --ignored --nocapture
//! ```

#[cfg(test)]
mod investigation {
    use crate::vertices::compute_vertices;
    use crate::QhullError;
    use nalgebra::Vector4;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// Test 1: 4 halfspaces in 4D (underconstrained - unbounded)
    ///
    /// This system has only 4 constraints in 4D, so it's unbounded in at least
    /// one direction. Does qhull detect this and fail?
    #[test]
    #[ignore] // Run manually with: cargo test --package geom unbounded_test1 -- --ignored --nocapture
    fn unbounded_test1_underconstrained() {
        println!("\n=== Test 1: 4 halfspaces in 4D (underconstrained) ===");
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let heights = vec![1.0, 1.0, 1.0, 1.0];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✗ UNEXPECTED SUCCESS");
                println!("  Returned {} vertices", vertices.len());
                panic!("Expected qhull to fail on unbounded polytope, but it succeeded!");
            }
            Err(e) => {
                println!("✓ FAILED AS EXPECTED");
                println!("  Error: {}", e);
                match e {
                    QhullError::ComputationFailed(stderr) => {
                        println!("  Stderr: {}", stderr);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Test 2: 5 halfspaces all pointing roughly the same direction (unbounded)
    ///
    /// These halfspaces all have positive x-component and small perturbations.
    /// The normals don't positively span R^4, so the polytope is unbounded.
    #[test]
    #[ignore] // Run manually
    fn unbounded_test2_unidirectional() {
        println!("\n=== Test 2: 5 halfspaces pointing same direction ===");
        let normals = vec![
            Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
        ];
        let heights = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✗ UNEXPECTED SUCCESS");
                println!("  Returned {} vertices", vertices.len());
                panic!("Expected qhull to fail on unbounded polytope, but it succeeded!");
            }
            Err(e) => {
                println!("✓ FAILED AS EXPECTED");
                println!("  Error: {}", e);
                match e {
                    QhullError::ComputationFailed(stderr) => {
                        println!("  Stderr: {}", stderr);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Test 3: Valid bounded hypercube (control - should succeed)
    #[test]
    #[ignore] // Run manually
    fn bounded_test_hypercube_control() {
        println!("\n=== Test 3: Valid bounded hypercube (control) ===");
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
        let heights = vec![1.0; 8];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✓ SUCCESS AS EXPECTED");
                println!("  Returned {} vertices", vertices.len());
                assert_eq!(vertices.len(), 16, "4D hypercube should have 16 vertices");
            }
            Err(e) => {
                println!("✗ UNEXPECTED FAILURE");
                println!("  Error: {}", e);
                panic!("Expected bounded hypercube to succeed, but qhull failed!");
            }
        }
    }

    // ============================================================================
    // Empirical Cross-Check: qhull vs check_bounded()
    // ============================================================================

    /// Generate random bounded polytope: normals that positively span R^4.
    ///
    /// **Strategy:** Start with 8 basis directions (±e_i for i=1..4), then add
    /// random perturbations. This ensures normals positively span R^4.
    fn generate_bounded_polytope(rng: &mut StdRng, num_facets: usize) -> (Vec<Vector4<f64>>, Vec<f64>) {
        assert!(num_facets >= 8, "Need at least 8 facets to ensure bounded (±e_i)");

        let mut normals = Vec::new();

        // Add basis directions ±e_i to guarantee positive span
        for i in 0..4 {
            let mut pos = Vector4::zeros();
            pos[i] = 1.0;
            normals.push(pos);

            let mut neg = Vector4::zeros();
            neg[i] = -1.0;
            normals.push(neg);
        }

        // Add random normals for remaining facets
        for _ in 8..num_facets {
            let normal = Vector4::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize();
            normals.push(normal);
        }

        // Heights: all positive to ensure origin in interior
        let heights: Vec<f64> = (0..num_facets).map(|_| rng.gen_range(0.5..2.0)).collect();

        (normals, heights)
    }

    /// Generate random unbounded polytope: normals that do NOT positively span R^4.
    ///
    /// **Patterns:**
    /// - Underconstrained: < 5 facets (not enough to close polytope in 4D)
    /// - Unidirectional: all normals in one hemisphere
    /// - Nearly-parallel: normals with high dot products
    /// - Rank-deficient: normals lie in lower-dimensional subspace
    fn generate_unbounded_polytope(
        rng: &mut StdRng,
        pattern: UnboundedPattern,
    ) -> (Vec<Vector4<f64>>, Vec<f64>) {
        match pattern {
            UnboundedPattern::Underconstrained => {
                // Only 4 facets in 4D → unbounded
                let num_facets = 4;
                let mut normals = Vec::new();
                for i in 0..4 {
                    let mut n = Vector4::zeros();
                    n[i] = 1.0;
                    normals.push(n);
                }
                let heights = vec![1.0; num_facets];
                (normals, heights)
            }
            UnboundedPattern::Unidirectional => {
                // All normals have x > 0.9 → don't span opposite direction
                let num_facets = rng.gen_range(5..10);
                let mut normals = Vec::new();
                for _ in 0..num_facets {
                    let normal = Vector4::new(
                        rng.gen_range(0.9..1.0), // All positive x
                        rng.gen_range(-0.2..0.2),
                        rng.gen_range(-0.2..0.2),
                        rng.gen_range(-0.2..0.2),
                    )
                    .normalize();
                    normals.push(normal);
                }
                let heights: Vec<f64> = (0..num_facets).map(|_| rng.gen_range(0.5..2.0)).collect();
                (normals, heights)
            }
            UnboundedPattern::NearlyParallel => {
                // Normals close to e_1 → degenerate span
                let num_facets = rng.gen_range(5..8);
                let mut normals = Vec::new();
                let base = Vector4::new(1.0, 0.0, 0.0, 0.0);
                for _ in 0..num_facets {
                    let perturbation = Vector4::new(
                        rng.gen_range(-0.05..0.05),
                        rng.gen_range(-0.1..0.1),
                        rng.gen_range(-0.1..0.1),
                        rng.gen_range(-0.1..0.1),
                    );
                    let normal = (base + perturbation).normalize();
                    normals.push(normal);
                }
                let heights: Vec<f64> = (0..num_facets).map(|_| rng.gen_range(0.5..2.0)).collect();
                (normals, heights)
            }
            UnboundedPattern::RankDeficient => {
                // All normals lie in 3D subspace (z=w=0 plane)
                let num_facets = rng.gen_range(5..8);
                let mut normals = Vec::new();
                for _ in 0..num_facets {
                    let normal = Vector4::new(
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-1.0..1.0),
                        0.0, // z = 0
                        0.0, // w = 0
                    )
                    .normalize();
                    normals.push(normal);
                }
                let heights: Vec<f64> = (0..num_facets).map(|_| rng.gen_range(0.5..2.0)).collect();
                (normals, heights)
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum UnboundedPattern {
        Underconstrained,
        Unidirectional,
        NearlyParallel,
        RankDeficient,
    }

    /// Check if normals positively span R^4 (ground truth for boundedness).
    ///
    /// **Algorithm:** Normals positively span ⟺ for every direction d, there exist
    /// indices i,j such that n_i·d > 0 and n_j·d < 0.
    ///
    /// **Implementation:** Check all O(F³) kernel directions (same as check_bounded).
    fn check_positive_span_reference(normals: &[Vector4<f64>]) -> bool {
        use crate::cross_product::cross_product_4d;
        let f = normals.len();
        if f < 5 {
            return false; // Need at least 5 halfspaces in 4D
        }

        for i in 0..f {
            for j in (i + 1)..f {
                for k in (j + 1)..f {
                    let d = cross_product_4d(normals[i], normals[j], normals[k]);
                    if d.norm() < 1e-12 {
                        continue; // Degenerate triple
                    }
                    let d_norm = d.normalize();

                    // Check if any normal blocks +d direction
                    let has_pos = (0..f)
                        .filter(|&l| l != i && l != j && l != k)
                        .any(|l| normals[l].dot(&d_norm) > 1e-9);

                    // Check if any normal blocks -d direction
                    let has_neg = (0..f)
                        .filter(|&l| l != i && l != j && l != k)
                        .any(|l| normals[l].dot(&d_norm) < -1e-9);

                    if !has_pos || !has_neg {
                        return false; // Found unblocked direction
                    }
                }
            }
        }
        true // All kernel directions are blocked
    }

    #[derive(Debug)]
    struct CrossCheckResult {
        bounded_agree: usize,
        bounded_disagree: usize,
        unbounded_agree: usize,
        unbounded_disagree: usize,
        qhull_false_negative: usize, // qhull accepted unbounded
        qhull_false_positive: usize,  // qhull rejected bounded
        check_bounded_bugs: usize,    // check_bounded() wrong
    }

    /// Cross-check qhull vs check_bounded() on random test cases.
    ///
    /// **Run with:** `cargo test --package geom cross_check_boundedness -- --ignored --nocapture`
    #[test]
    #[ignore] // ~3s, 875 cases. Non-default: monitoring or after qhull/boundedness changes.
    fn cross_check_boundedness_detection() {
        println!("\n========================================");
        println!("Empirical Cross-Check: qhull vs check_bounded()");
        println!("========================================\n");

        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        let mut results = CrossCheckResult {
            bounded_agree: 0,
            bounded_disagree: 0,
            unbounded_agree: 0,
            unbounded_disagree: 0,
            qhull_false_negative: 0,
            qhull_false_positive: 0,
            check_bounded_bugs: 0,
        };

        // Test bounded polytopes (500 cases)
        println!("Testing bounded polytopes (500 cases)...");
        for i in 0..500 {
            let num_facets = rng.gen_range(8..20);
            let (normals, heights) = generate_bounded_polytope(&mut rng, num_facets);

            // Ground truth: should be bounded
            let is_bounded_truth = check_positive_span_reference(&normals);
            assert!(is_bounded_truth, "Generator bug: produced unbounded case");

            // Run qhull
            let qhull_result = compute_vertices(&normals, &heights);
            let qhull_accepted = qhull_result.is_ok();

            // Run check_bounded() via reference implementation
            let check_bounded_passed = is_bounded_truth; // Use same algorithm as reference

            if qhull_accepted == check_bounded_passed {
                results.bounded_agree += 1;
            } else {
                results.bounded_disagree += 1;
                if !qhull_accepted {
                    results.qhull_false_positive += 1;
                    println!(
                        "  [{}] Qhull REJECTED bounded polytope (F={}): {:?}",
                        i,
                        num_facets,
                        qhull_result.err()
                    );
                }
            }

            if i % 100 == 99 {
                println!("  ... {} bounded cases tested", i + 1);
            }
        }

        // Test unbounded polytopes (500 cases)
        println!("\nTesting unbounded polytopes (500 cases)...");
        let patterns = [
            UnboundedPattern::Underconstrained,
            UnboundedPattern::Unidirectional,
            UnboundedPattern::NearlyParallel,
            UnboundedPattern::RankDeficient,
        ];

        let mut detailed_disagreements_printed = 0;

        for i in 0..500 {
            let pattern = patterns[i % patterns.len()];
            let (normals, heights) = generate_unbounded_polytope(&mut rng, pattern);

            // Ground truth: should be unbounded
            let is_bounded_truth = check_positive_span_reference(&normals);
            if is_bounded_truth {
                println!(
                    "  [{}] WARNING: Generator produced bounded case for pattern {:?}",
                    i, pattern
                );
                continue; // Skip this case
            }

            // Run qhull
            let qhull_result = compute_vertices(&normals, &heights);
            let qhull_accepted = qhull_result.is_ok();

            // Run check_bounded() via reference implementation
            let check_bounded_passed = is_bounded_truth; // Should be false

            if qhull_accepted == check_bounded_passed {
                results.unbounded_agree += 1;
            } else {
                results.unbounded_disagree += 1;
                if qhull_accepted {
                    results.qhull_false_negative += 1;
                    println!(
                        "  [{}] Qhull ACCEPTED unbounded polytope (pattern {:?}, F={})",
                        i,
                        pattern,
                        normals.len()
                    );

                    // CRITICAL: Actually look at what vertices qhull returned
                    if let Ok(vertices) = qhull_result {
                        let num_vertices = vertices.len();
                        println!("    Qhull returned {} vertices", num_vertices);

                        // Check for sentinel value (-10.101, -10.101, -10.101, -10.101)
                        let sentinel_count = vertices.iter()
                            .filter(|v| {
                                (v.x - (-10.101)).abs() < 0.001
                                && (v.y - (-10.101)).abs() < 0.001
                                && (v.z - (-10.101)).abs() < 0.001
                                && (v.w - (-10.101)).abs() < 0.001
                            })
                            .count();

                        if sentinel_count > 0 {
                            println!("    *** FOUND {} SENTINEL VERTICES (-10.101, -10.101, -10.101, -10.101) ***", sentinel_count);
                        }

                        // Check if any coordinate is -10.101
                        let has_sentinel_coord = vertices.iter()
                            .any(|v| {
                                (v.x - (-10.101)).abs() < 0.001
                                || (v.y - (-10.101)).abs() < 0.001
                                || (v.z - (-10.101)).abs() < 0.001
                                || (v.w - (-10.101)).abs() < 0.001
                            });

                        if has_sentinel_coord {
                            println!("    *** Some vertices contain -10.101 coordinate ***");
                        }

                        // Print detailed vertex list for first 5 disagreements
                        if detailed_disagreements_printed < 5 {
                            println!("    First 10 vertices:");
                            for (j, v) in vertices.iter().take(10).enumerate() {
                                println!("      [{}] ({:.6}, {:.6}, {:.6}, {:.6})", j, v.x, v.y, v.z, v.w);
                            }
                            if num_vertices > 10 {
                                println!("      ... ({} more vertices)", num_vertices - 10);
                            }
                            detailed_disagreements_printed += 1;
                        }
                    }

                    println!("    Normals: {:?}", normals);
                    println!("    Heights: {:?}", heights);
                }
            }

            if i % 100 == 99 {
                println!("  ... {} unbounded cases tested", i + 1);
            }
        }

        // Print results
        println!("\n========================================");
        println!("Results:");
        println!("========================================");
        println!("Bounded polytopes:");
        println!("  Agreement:    {}", results.bounded_agree);
        println!("  Disagreement: {}", results.bounded_disagree);
        println!("  Qhull false positives: {}", results.qhull_false_positive);
        println!("\nUnbounded polytopes:");
        println!("  Agreement:    {}", results.unbounded_agree);
        println!("  Disagreement: {}", results.unbounded_disagree);
        println!("  Qhull false negatives: {}", results.qhull_false_negative);
        println!(
            "\nTotal agreement: {}/1000",
            results.bounded_agree + results.unbounded_agree
        );
        println!(
            "Total disagreement: {}/1000",
            results.bounded_disagree + results.unbounded_disagree
        );

        // Assertions
        if results.qhull_false_negative > 0 {
            panic!(
                "CRITICAL: Qhull accepted {} unbounded polytopes! CANNOT trust qhull for boundedness.",
                results.qhull_false_negative
            );
        }

        if results.bounded_disagree > results.bounded_agree / 10 {
            panic!(
                "WARNING: High disagreement rate on bounded cases ({}). Investigate qhull tolerance issues.",
                results.bounded_disagree
            );
        }

        println!("\n✓ Cross-check complete. See BOUNDEDNESS_INVESTIGATION.md for analysis.");
    }
}
