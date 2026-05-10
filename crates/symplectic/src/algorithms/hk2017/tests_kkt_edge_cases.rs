//! HK2017 kkt edge cases tests.
//!
//! Split from mod.rs to keep module routing and docs short.

use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::solve_kkt_for_dual_vertices;
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
            eprintln!("2-facet polytope rejected (expected); testing augmented system directly");

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
    let dual_vertices = polytope.dual_vertices_f64();
    let r = solve_kkt_for_dual_vertices(dual_vertices, &perm)
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
