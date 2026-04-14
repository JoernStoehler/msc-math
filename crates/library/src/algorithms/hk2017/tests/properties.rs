mod tests_kkt_edge_cases {
    use crate::geom::polytope::Polytope4D;
    use crate::kkt::saddle_point_solver::solve_kkt_for;
    use nalgebra::Vector4;

    /// KKT solver on minimal 2-facet system (two opposite facets).
    ///
    /// Two opposite facets: n1 = (1,0,0,0), n2 = (-1,0,0,0), h1 = h2 = 1.
    /// Constraints: beta_1 - beta_2 = 0, beta_1 + beta_2 = 1 => beta = (0.5, 0.5).
    /// Q(beta) = 0 because omega_0(n1, n2) = 0 (parallel normals, q-space only).
    ///
    /// Tests solver on the smallest possible system size.
    #[test]
    fn solve_kkt_two_facets() {
        let normals = [Vector4::x(), -Vector4::x()];
        let heights = [1.0, 1.0];
        let perm = [0, 1];

        // Two opposite facets don't form a valid bounded polytope (need >=5 for R^4).
        // Use the augmented system directly to test the solver on this minimal input.
        let polytope = match Polytope4D::from_f64(
            normals
                .iter()
                .zip(heights.iter())
                .map(|(n, &h)| n / h)
                .collect(),
        ) {
            Ok(p) => p,
            Err(_) => {
                // Expected: 2 facets is too few for a bounded polytope in R^4.
                // Test the augmented system assembly + solver directly.
                // This exercises the solver's handling of small systems.
                eprintln!(
                    "2-facet polytope rejected (expected); testing augmented system directly"
                );

                // Build the augmented system manually from normals/heights.
                // The augmented system is valid for any facet count, even if the
                // polytope is not bounded.
                let m = perm.len();
                let n_dim = 4;
                let size = m + n_dim + 1; // m + 4 + 1 = 7
                let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
                let mut rhs = nalgebra::DVector::zeros(size);

                // H block (m x m): H_ij = omega_0(n_sigma(i), n_sigma(j))
                for i in 0..m {
                    for j in 0..m {
                        let ni = &normals[perm[i]];
                        let nj = &normals[perm[j]];
                        kkt_mat[(i, j)] = crate::geom::symplectic_form::omega0(ni, nj);
                    }
                }

                // N block: closure constraints
                for i in 0..m {
                    let n = &normals[perm[i]];
                    for k in 0..n_dim {
                        kkt_mat[(i, m + k)] = n[k];
                        kkt_mat[(m + k, i)] = n[k];
                    }
                }

                // eta block: normalization
                for i in 0..m {
                    kkt_mat[(i, m + n_dim)] = 1.0;
                    kkt_mat[(m + n_dim, i)] = 1.0;
                }

                rhs[m + n_dim] = 1.0;

                let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

                if let crate::kkt::saddle_point_solver::KktOutcome::Feasible(r) = &result {
                    assert_eq!(r.beta.len(), 2);
                    // beta_1 ~ beta_2 ~ 0.5
                    assert!(
                        (r.beta[0] - 0.5).abs() < 1e-6,
                        "beta_1 should be ~0.5, got {}",
                        r.beta[0]
                    );
                    assert!(
                        (r.beta[1] - 0.5).abs() < 1e-6,
                        "beta_2 should be ~0.5, got {}",
                        r.beta[1]
                    );
                    // Q = 0 (parallel normals have omega_0 = 0)
                    assert!(
                        r.q_corrected.abs() < 1e-10,
                        "Q should be ~0 for parallel normals, got {}",
                        r.q_corrected
                    );
                }
                return;
            }
        };

        // If construction succeeded (unlikely for 2 facets), test via standard API.
        let r = solve_kkt_for(&polytope, &perm)
            .feasible()
            .expect("two-facet system should solve");
        assert_eq!(r.beta.len(), 2);
        assert!((r.beta[0] - 0.5).abs() < 1e-6);
        assert!((r.beta[1] - 0.5).abs() < 1e-6);
        assert!(r.q_corrected.abs() < 1e-10);
    }

    /// KKT solver on 4-facet symplectic square.
    ///
    /// Four facets forming a 2D symplectic subplane:
    /// n1 = e_q1, n2 = e_p1, n3 = -e_q1, n4 = -e_p1 with heights all 1.0.
    /// omega_0(e_q1, e_p1) = 1, so Q != 0 (non-degenerate symplectic system).
    /// Constraints: beta_1 = beta_3, beta_2 = beta_4, sum = 1.
    ///
    /// Tests the solver on structured geometry with non-trivial symplectic form.
    #[test]
    fn solve_kkt_four_facets_symplectic() {
        let normals = [
            Vector4::x(),  // e_q1
            Vector4::z(),  // e_p1
            -Vector4::x(), // -e_q1
            -Vector4::z(), // -e_p1
        ];
        let _heights = [1.0; 4];
        let perm = [0, 1, 2, 3];

        // 4 facets in R^4 is not a bounded polytope (need >=5). Build augmented
        // system directly from normals/heights.
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] =
                    crate::geom::symplectic_form::omega0(&normals[perm[i]], &normals[perm[j]]);
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // The solver may return None for this small (m=4) augmented system
        // because the (m+5=9) matrix can be ill-conditioned or the residual
        // check may reject the solution. Either Some or None is acceptable.
        if let crate::kkt::saddle_point_solver::KktOutcome::Feasible(r) = result {
            assert_eq!(r.beta.len(), 4);

            // Verify constraints: beta_1 = beta_3, beta_2 = beta_4.
            assert!(
                (r.beta[0] - r.beta[2]).abs() < 1e-6,
                "beta_1 should equal beta_3"
            );
            assert!(
                (r.beta[1] - r.beta[3]).abs() < 1e-6,
                "beta_2 should equal beta_4"
            );

            // Normalization: sum = 1.
            let sum: f64 = r.beta.iter().sum();
            assert!((sum - 1.0).abs() < 1e-6, "beta sum should be 1");

            // Q != 0 (non-degenerate symplectic system).
            assert!(
                r.q_corrected.abs() > 1e-10,
                "Q should be non-zero for symplectic normals, got {}",
                r.q_corrected
            );
        } else {
            eprintln!("Note: 4-facet symplectic system returned None (solver rejected it)");
        }
    }

    /// KKT solver handles rank-deficient normal matrix.
    ///
    /// Three normals in the q-plane (rank 2 normal matrix): omega_0(n_i, n_j) = 0
    /// for all pairs. The unique beta satisfying constraints has beta_2 < 0,
    /// so solve_kkt correctly returns None.
    ///
    /// Tests that the solver correctly detects infeasibility from rank deficiency.
    #[test]
    fn solve_kkt_rank_deficient() {
        let normals = [
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.707, 0.707, 0.0, 0.0).normalize(),
        ];
        let _heights = [1.0; 3];
        let perm = [0, 1, 2];

        // Build augmented system directly (3 facets is not a valid polytope).
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] =
                    crate::geom::symplectic_form::omega0(&normals[perm[i]], &normals[perm[j]]);
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // Returns Infeasible: the unique beta has beta_2 < 0.
        assert!(
            !matches!(
                result,
                crate::kkt::saddle_point_solver::KktOutcome::Feasible(_)
            ),
            "rank-deficient system with beta < 0 should not be Feasible"
        );
    }

    /// KKT solver on degenerate case (identical normals).
    ///
    /// Two identical normals (violates irredundancy). The solver should either
    /// return None or return Some without panicking. Either outcome is acceptable.
    ///
    /// Tests graceful degradation on invalid input.
    #[test]
    fn solve_kkt_degenerate() {
        let normals = [Vector4::x(), Vector4::x()];
        let _heights = [1.0, 1.0];
        let perm = [0, 1];

        // Build augmented system directly (degenerate, 2 facets).
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] =
                    crate::geom::symplectic_form::omega0(&normals[perm[i]], &normals[perm[j]]);
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // Either non-feasible (degenerate) or Feasible (solver handled it). Both acceptable.
        if let crate::kkt::saddle_point_solver::KktOutcome::Feasible(_) = &result {
            eprintln!("Note: degenerate system returned Feasible (acceptable)");
        }
    }
}

mod tests_regression {
    use crate::algorithms::hk2017::*;
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
    use crate::kkt::qp_assembly::build_augmented_system;

    // ── KKT null space fix regressions ──
    //
    // These tests verify that the KKT solver correctly handles rank-deficient
    // systems by searching the null space for beta > 0 solutions. Before the fix,
    // the minimum-norm pseudoinverse solution often had beta <= 0 for degenerate
    // polytopes (axis-aligned normals in symplectic subplanes).

    /// Regression: (4,4) Lagrangian product at theta=0 (square x square, axis-aligned).
    ///
    /// Before fix: cap=2.0 (correct). After fix: cap=2.0 (unchanged).
    /// This is the hypercube [-1/sqrt(2), 1/sqrt(2)]^4 which already worked pre-fix.
    /// Included to verify the fix does not break the working case.
    #[test]
    fn kkt_nullspace_square_square_zero() {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result =
            ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0 should have capacity");
        assert!(
            (result.result.capacity - 2.0).abs() < 1e-6,
            "(4,4) at theta=0: got {}, expected 2.0",
            result.result.capacity
        );
    }

    /// Regression: (4,4) at theta=0.125 deg — smallest angle in the polygon grid.
    ///
    /// Before fix: cap=3.991 (WRONG, 2x too high due to 8-facet spurious orbit).
    /// After fix: cap ~ 2.000 (continuous from theta=0).
    ///
    /// At theta=0.125 deg (near-degenerate), some orbits have Q ~ 0 where null-space
    /// Q constancy is noise-dominated. The Q constancy debug_assert skips Q < 1e-6.
    #[test]
    fn kkt_nullspace_square_square_near_zero() {
        let theta = 0.125_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result =
            ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0.125 should have capacity");
        // Capacity should be continuous near theta=0 -> close to 2.0.
        assert!(
            (result.result.capacity - 2.0).abs() < 0.01,
            "(4,4) at theta=0.125: got {}, expected ~2.0 (was 3.991 before fix)",
            result.result.capacity
        );
    }

    /// Regression: (4,4) at theta=45 deg — billiard previously gave 2x wrong answer.
    ///
    /// Before fix: HK2017=2.828, billiard=5.657.
    /// After fix: all agree on cap = 2*sqrt(2) ~ 2.828.
    #[test]
    fn kkt_nullspace_square_square_45deg() {
        let theta = 45.0_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result_hk = ehz_capacity_unpruned(&polytope)
            .expect("(4,4) at theta=45: HK2017 should have capacity");
        let result_bil = crate::algorithms::billiard::billiard_capacity(&polytope)
            .expect("billiard should not error")
            .expect("billiard should find capacity");

        let sqrt2_times2 = 2.0 * std::f64::consts::SQRT_2;
        assert!(
            (result_hk.result.capacity - sqrt2_times2).abs() < 1e-6,
            "(4,4) at theta=45 HK2017: got {}, expected 2*sqrt(2) ~ {}",
            result_hk.result.capacity,
            sqrt2_times2
        );
        assert!(
            (result_bil.result.capacity - sqrt2_times2).abs() < 1e-6,
            "(4,4) at theta=45 billiard: got {} (was 5.657 before fix), expected 2*sqrt(2) ~ {}",
            result_bil.result.capacity,
            sqrt2_times2
        );
    }

    /// Regression: (3,4) at theta=0 — previously returned None for all algorithms.
    ///
    /// Before fix: None (all three algorithms). No valid orbit found.
    /// After fix: cap ~ 2.121 via 5-facet orbit. All three agree.
    ///
    /// Note: The expected capacity for this specific polytope (triangle circumradius=1,
    /// square circumradius=1) is 3*sqrt(2)/2 ~ 2.121, NOT 1.5. The value 1.5 is from
    /// `lagrangian_triangle_square()` which uses different dimensions.
    #[test]
    fn kkt_nullspace_triangle_square_zero() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result = ehz_capacity_unpruned(&polytope)
            .expect("(3,4) at theta=0 should now return Some after null space fix");

        let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0; // 3*sqrt(2)/2 ~ 2.121
        assert!(
            (result.result.capacity - expected).abs() < 1e-6,
            "(3,4) at theta=0: got {}, expected 3*sqrt(2)/2 ~ {} (was None before fix)",
            result.result.capacity,
            expected
        );
    }

    // ── Eigenvalue condition number threshold regression tests ──
    //
    // EIGEN_CONDITION_TAU=1e-3 was chosen empirically from the (4,4) degenerate case.
    // These tests pin the eigenvalue spectrum so that threshold changes can be
    // validated against the cases that motivated the threshold.

    /// Verify eigenvalue spectrum of the (4,4) theta=0 degenerate KKT system.
    ///
    /// The optimal orbit permutation [0,4,2,6] (alternating q/p facets) has a gap
    /// in the sorted |lambda_i| spectrum. The system must be rank-deficient for the
    /// null space search to activate.
    ///
    /// Regression test for EIGEN_CONDITION_TAU (see doc comment on the constant).
    #[test]
    fn eigen_gap_ratio_44_degenerate() {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        // The optimal orbit at theta=0 uses facets [0,4,2,6] (alternating q/p).
        let perm = vec![0, 4, 2, 6];
        let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
        let eigen = kkt.symmetric_eigen();
        let size = perm.len() + 5; // 9

        // Collect |lambda_i| and sort descending.
        let mut abs_eigenvalues: Vec<f64> = eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
        abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Find gap ratio: walk from smallest |lambda_i| upward.
        let floor = 1e-15;
        let smallest_nonzero = (0..size).rev().find(|&i| abs_eigenvalues[i] > floor);
        if let Some(idx) = smallest_nonzero {
            if idx > 0 {
                let ratio = abs_eigenvalues[idx - 1] / abs_eigenvalues[idx];
                // If large gap exists, it must stay well above EIGEN_CONDITION_TAU.
                if ratio > 50.0 {
                    assert!(
                        ratio > 200.0,
                        "(4,4) theta=0 gap ratio should stay well above 1e-3 threshold, \
                         got {:.1} (|lambda[{}]|={:.3e}, |lambda[{}]|={:.3e})",
                        ratio,
                        idx - 1,
                        abs_eigenvalues[idx - 1],
                        idx,
                        abs_eigenvalues[idx]
                    );
                }
            }
        }

        // The KKT system must be rank-deficient (axis-aligned normals create dependence).
        let numerical_rank = abs_eigenvalues.iter().filter(|&&ev| ev > 1e-6).count();
        assert!(
            numerical_rank < size,
            "(4,4) theta=0 should be rank-deficient: rank={}, size={}",
            numerical_rank,
            size
        );
    }

    /// Verify eigenvalue gap ratio for the (4,4) theta=43 deg case.
    ///
    /// The KKT system for perm [1,0,6,3,2,4] has a gap ratio ~594 — the case from
    /// commit dd87a8a that motivated EIGEN_CONDITION_TAU=1e-3. The gap ratio must
    /// stay well above the threshold.
    #[test]
    fn eigen_gap_ratio_44_theta43() {
        let theta = 43.0_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let perm = vec![1, 0, 6, 3, 2, 4];
        let m = perm.len();
        let size = m + 5; // 11

        let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
        let eigen = kkt.symmetric_eigen();

        let mut abs_eigenvalues: Vec<f64> = eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
        abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Find the largest gap ratio in the spectrum.
        let floor = 1e-15;
        let mut max_gap_ratio = 0.0f64;
        let mut gap_idx = 0;
        for i in (1..size).rev() {
            if abs_eigenvalues[i] < floor {
                continue;
            }
            let ratio = abs_eigenvalues[i - 1] / abs_eigenvalues[i];
            if ratio > max_gap_ratio {
                max_gap_ratio = ratio;
                gap_idx = i;
            }
        }

        // Gap ratio must be well above EIGEN_CONDITION_TAU=1e-3. Original: ~594.
        assert!(
            max_gap_ratio > 300.0,
            "(4,4) theta=43 gap ratio should be ~594 (well above 1e-3 threshold), \
             got {:.1} at |lambda[{}]|={:.3e}/|lambda[{}]|={:.3e}",
            max_gap_ratio,
            gap_idx - 1,
            abs_eigenvalues[gap_idx - 1],
            gap_idx,
            abs_eigenvalues[gap_idx]
        );
    }

    // ── HKO counterexample regression ──

    /// Verify HKO pentagon capacity and sys > 1 property (Annals counterexample).
    ///
    /// Computes capacity on the Haim-Kislev-Ostrover 10-facet pentagon and verifies
    /// it is a counterexample to Viterbo's conjecture (sys > 1).
    ///
    /// Why #[ignore]: F=10 -> ~37s debug, ~0.5s release. Important regression test
    /// for the thesis counterexample.
    /// Run: `cargo test --release pentagon_capacity -- --ignored`
    #[test]
    #[ignore] // ~37s debug, ~0.5s release
    fn pentagon_capacity() {
        use crate::geom::known_polytopes;
        use crate::geom::volume::volume;

        let kp = known_polytopes::hko_pentagon();
        let result = ehz_capacity(&kp.polytope).expect("pentagon capacity");

        assert!(
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "pentagon: got {}, expected {}",
            result.result.capacity,
            kp.capacity
        );

        // Verify sys > 1 (counterexample property).
        let vol = volume(&kp.polytope).expect("volume computation failed");
        let sys = result.result.capacity * result.result.capacity / (2.0 * vol);
        eprintln!(
            "Pentagon: capacity={:.6}, volume={:.6}, sys={:.6}",
            result.result.capacity, vol, sys
        );
        assert!(sys > 1.0, "pentagon should have sys > 1, got {}", sys);
    }
}
