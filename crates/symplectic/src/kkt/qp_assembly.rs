//! Flat dual-vertex data + permutation -> QP matrices or augmented KKT system.
//!
//! Bridges geometry data to the solver's abstract matrix inputs. The assembly
//! boundary only needs dual vertices and a permutation. Two assembly modes:
//!
//! - `build_qp_from_dual_vertices`: assembles the QP struct {C, d, H} using
//!   dual vertices directly. Used by the projection solver path.
//! - `build_augmented_system_from_dual_vertices`: assembles the (m+5)x(m+5)
//!   saddle-point system using dual vertices directly. Used by the
//!   eigendecomposition solver path.
//!
//! Mathematical correspondence: [lem:kkt]

use super::QP;
use crate::geom::symplectic_form::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

/// Assemble the QP {C, d, H} from dual vertices and a cyclic permutation.
///
/// Given dual vertices a_i of K = {x : a_i^T x <= 1} and a permutation sigma
/// of m facet indices, assembles:
///
/// - **C** (5 x m): closure constraints (sum a_{sigma(i)} beta_i = 0, four rows)
///   plus normalization (sum beta_i = 1, one row). Note: when using dual vertices
///   directly, the closure constraint is sum a_{sigma(i)} beta_i = 0 (not normals).
/// - **d** (5 x 1): [0, 0, 0, 0, 1]^T
/// - **H** (m x m): action matrix, symmetrized. For i < j:
///   H_{ij} = H_{ji} = omega_0(a_{sigma(i)}, a_{sigma(j)}).
///   H_{ii} = 0 (since omega_0(a, a) = 0 by antisymmetry of omega_0).
///   Note: H is symmetric by construction (both entries set to the same value),
///   not because omega_0 is symmetric (it is antisymmetric: omega_0(a_j, a_i) = -omega_0(a_i, a_j)).
///   The quadratic form (1/2) beta^T H beta equals the symplectic action sum.
///
/// Uses dual vertices directly (not normalized normals), which simplifies the
/// constraint structure: the closure + normalization constraints become a single
/// linear system without the height scaling that appears in the normals/heights
/// parameterization.
///
/// # Panics
/// - If any index in `perm` is out of bounds for `dual_vertices`.
///
/// [lem:kkt]: KKT optimality conditions characterize the EHZ capacity optimum.
pub fn build_qp_from_dual_vertices(dual_vertices: &[Vector4<f64>], perm: &[usize]) -> QP {
    assert_permutation_indices_in_bounds(dual_vertices, perm);

    let m = perm.len();

    // Constraint matrix C (5 x m):
    // Rows 0..3: closure constraint sum_i a_{sigma(i)} beta_i = 0 (per coordinate)
    // Row 4: normalization sum_i beta_i = 1
    //
    // This uses the dual-vertex (beta') parameterization: beta'_i = h_{sigma(i)} * beta_i,
    // where beta_i are the dwell-time coefficients in the normals/heights parameterization.
    // The correspondence is: sum n_{sigma(i)} beta_i = 0 + sum h_{sigma(i)} beta_i = 1
    // becomes sum a_{sigma(i)} beta'_i = 0 + sum beta'_i = 1 after multiplying through
    // by the heights. The QP therefore operates in beta' coordinates.
    //
    // [lem:dual-vertex-qp]: the dual-vertex QP formulation (closure A^T beta = 0,
    // normalization 1^T beta = 1, action H_{ij} = omega_0(a_i, a_j)) computes the
    // EHZ symplectic action directly from dual vertices, without factoring a_i = h_i n_i.
    let mut c = DMatrix::zeros(5, m);
    for (col, &facet_idx) in perm.iter().enumerate() {
        let a = &dual_vertices[facet_idx];
        for d in 0..4 {
            c[(d, col)] = a[d];
        }
        c[(4, col)] = 1.0;
    }

    let mut d = DVector::zeros(5);
    d[4] = 1.0;

    // Action matrix H (m x m): H_{ij} = omega_0(a_{sigma(i)}, a_{sigma(j)}) for i != j.
    // H is symmetric with zero diagonal: omega_0(a, a) = 0 by antisymmetry.
    // The quadratic form Q(beta) = (1/2) beta^T H beta = sum_{i>j} beta_i beta_j omega_0(a_{sigma(j)}, a_{sigma(i)}).
    let mut h = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            h[(i, j)] = val;
            h[(j, i)] = val;
        }
    }

    QP { c, d, h }
}

/// Assemble the augmented (m+5)x(m+5) KKT system from dual vertices and a permutation.
///
/// Builds the symmetric saddle-point matrix M and right-hand side b:
///
/// ```text
/// [ H   |  A   |  1 ] [ beta ]   [ 0 ]
/// [ A^T |  0   |  0 ] [  mu  ] = [ 0 ]
/// [ 1^T |  0   |  0 ] [  xi  ]   [ 1 ]
/// ```
///
/// where:
/// - H (m x m): action matrix, H_{ij} = omega_0(a_{sigma(i)}, a_{sigma(j)})
/// - A (m x 4): dual vertices, A_{i,d} = a_{sigma(i),d}
/// - 1 (m x 1): all ones
///
/// Uses dual vertices a_i directly (K = {x : a_i^T x <= 1}).
/// Stationarity: H beta + A mu + 1 xi = 0, with Lagrange multipliers mu in R^4, xi in R.
/// Symmetry enables eigendecomposition M = V Lambda V^T.
///
/// # Panics
/// - If any index in `perm` is out of bounds for `dual_vertices`.
///
/// [lem:kkt]: the augmented saddle-point system encodes stationarity + closure + normalization.
pub fn build_augmented_system_from_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    assert_permutation_indices_in_bounds(dual_vertices, perm);

    let m = perm.len();
    let size = m + 5;

    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left block: H (m x m) — action matrix with omega_0 values between dual vertices.
    // H_{ij} = omega_0(a_{sigma(i)}, a_{sigma(j)}) for i != j, H_{ii} = 0.
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }

    // Off-diagonal blocks: A (m x 4) and A^T (4 x m) — placed symmetrically.
    for i in 0..m {
        for d in 0..4 {
            let a = dual_vertices[perm[i]][d];
            kkt[(i, m + d)] = a;
            kkt[(m + d, i)] = a;
        }
    }

    // Off-diagonal blocks: 1 (m x 1) and 1^T (1 x m) — placed symmetrically.
    for i in 0..m {
        kkt[(i, m + 4)] = 1.0;
        kkt[(m + 4, i)] = 1.0;
    }

    // RHS: [0, ..., 0, 1] — normalization constraint.
    rhs[m + 4] = 1.0;

    (kkt, rhs)
}

fn assert_permutation_indices_in_bounds(dual_vertices: &[Vector4<f64>], perm: &[usize]) {
    for &facet_idx in perm {
        assert!(
            facet_idx < dual_vertices.len(),
            "permutation index {facet_idx} out of bounds for {} dual vertices",
            dual_vertices.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::polytope::Polytope4D;
    use crate::geom::symplectic_form::omega0;

    // Tests for qp_assembly: QP and augmented system construction from dual vertices.
    //
    // Proposition: build_qp_from_dual_vertices and build_augmented_system_from_dual_vertices
    // correctly encode the KKT constraints (closure, normalization) and objective
    // (symplectic action) for given dual vertices and facet permutation.
    // Reference: [lem:kkt]
    //
    // Strategy: fixture-based on known polytopes (hypercube), verifying
    // matrix dimensions, symmetry of H, constraint structure, and known values.

    // ── build_qp_from_dual_vertices tests ──

    #[test]
    #[should_panic(expected = "permutation index 8 out of bounds for 8 dual vertices")]
    fn flat_qp_rejects_out_of_bounds_permutation_index() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let _ = build_qp_from_dual_vertices(dual_vertices, &[0, 8]);
    }

    /// Verify QP dimensions match the permutation length.
    #[test]
    fn qp_dimensions_match_permutation() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        // Use first 3 facets as a short permutation
        let perm: Vec<usize> = (0..3.min(n)).collect();
        let m = perm.len();

        let qp = build_qp_from_dual_vertices(dual_vertices, &perm);

        assert_eq!(
            qp.c.nrows(),
            5,
            "C should have 5 rows (4 closure + 1 normalization)"
        );
        assert_eq!(qp.c.ncols(), m, "C should have m={} columns", m);
        assert_eq!(qp.d.nrows(), 5, "d should have 5 rows");
        assert_eq!(qp.h.nrows(), m, "H should be m x m");
        assert_eq!(qp.h.ncols(), m, "H should be m x m");
    }

    /// H matrix must be symmetric (omega_0 placed symmetrically: H_{ij} = H_{ji}).
    #[test]
    fn qp_h_is_symmetric() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();
        let m = perm.len();

        let qp = build_qp_from_dual_vertices(dual_vertices, &perm);

        for i in 0..m {
            for j in 0..m {
                assert!(
                    (qp.h[(i, j)] - qp.h[(j, i)]).abs() < 1e-15,
                    "H[{},{}]={} != H[{},{}]={}",
                    i,
                    j,
                    qp.h[(i, j)],
                    j,
                    i,
                    qp.h[(j, i)]
                );
            }
        }
    }

    /// H diagonal must be zero (omega_0(a, a) = 0 by antisymmetry of the symplectic form).
    #[test]
    fn qp_h_diagonal_is_zero() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();

        let qp = build_qp_from_dual_vertices(dual_vertices, &perm);

        for i in 0..perm.len() {
            assert!(
                qp.h[(i, i)].abs() < 1e-15,
                "H[{0},{0}] = {1} should be 0",
                i,
                qp.h[(i, i)]
            );
        }
    }

    /// H entries match omega_0 applied to the permuted dual vertices.
    #[test]
    fn qp_h_entries_match_omega0() {
        let polytope = make_test_polytope();
        let dual_verts = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();

        let qp = build_qp_from_dual_vertices(dual_verts, &perm);

        for i in 0..perm.len() {
            for j in (i + 1)..perm.len() {
                let expected = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
                assert!(
                    (qp.h[(i, j)] - expected).abs() < 1e-15,
                    "H[{},{}]={} != omega0(a_{}, a_{})={}",
                    i,
                    j,
                    qp.h[(i, j)],
                    perm[i],
                    perm[j],
                    expected
                );
            }
        }
    }

    /// C matrix encodes dual vertex coordinates in rows 0..3 and ones in row 4.
    #[test]
    fn qp_constraint_matrix_structure() {
        let polytope = make_test_polytope();
        let dual_verts = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();

        let qp = build_qp_from_dual_vertices(dual_verts, &perm);

        for (col, &facet_idx) in perm.iter().enumerate() {
            let a = &dual_verts[facet_idx];
            for d in 0..4 {
                assert!(
                    (qp.c[(d, col)] - a[d]).abs() < 1e-15,
                    "C[{},{}]={} != a_{}[{}]={}",
                    d,
                    col,
                    qp.c[(d, col)],
                    facet_idx,
                    d,
                    a[d]
                );
            }
            assert!(
                (qp.c[(4, col)] - 1.0).abs() < 1e-15,
                "C[4,{}]={} should be 1.0",
                col,
                qp.c[(4, col)]
            );
        }
    }

    /// d vector is [0, 0, 0, 0, 1].
    #[test]
    fn qp_rhs_vector() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let perm: Vec<usize> = (0..3.min(polytope.facet_count())).collect();

        let qp = build_qp_from_dual_vertices(dual_vertices, &perm);

        for d in 0..4 {
            assert!(qp.d[d].abs() < 1e-15, "d[{}]={} should be 0", d, qp.d[d]);
        }
        assert!(
            (qp.d[4] - 1.0).abs() < 1e-15,
            "d[4]={} should be 1.0",
            qp.d[4]
        );
    }

    // ── build_augmented_system_from_dual_vertices tests ──

    /// Augmented system has size (m+5) x (m+5).
    #[test]
    fn augmented_system_dimensions() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..3.min(n)).collect();
        let m = perm.len();
        let size = m + 5;

        let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);

        assert_eq!(kkt.nrows(), size);
        assert_eq!(kkt.ncols(), size);
        assert_eq!(rhs.nrows(), size);
    }

    /// The augmented KKT matrix must be symmetric (saddle-point structure).
    #[test]
    fn augmented_system_is_symmetric() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();

        let (kkt, _) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);
        let size = kkt.nrows();

        for i in 0..size {
            for j in 0..size {
                assert!(
                    (kkt[(i, j)] - kkt[(j, i)]).abs() < 1e-15,
                    "KKT[{},{}]={} != KKT[{},{}]={}",
                    i,
                    j,
                    kkt[(i, j)],
                    j,
                    i,
                    kkt[(j, i)]
                );
            }
        }
    }

    /// The augmented system block structure: H uses dual vertices, off-diagonal blocks
    /// contain dual vertex coordinates and ones, bottom-right 5x5 is zero, RHS = [0..0, 1].
    #[test]
    fn augmented_system_block_structure() {
        let polytope = make_test_polytope();
        let dual_verts = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();
        let m = perm.len();

        let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_verts, &perm);

        // Check H block (top-left m x m): omega_0 between permuted dual vertices.
        for i in 0..m {
            for j in (i + 1)..m {
                let expected = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
                assert!(
                    (kkt[(i, j)] - expected).abs() < 1e-15,
                    "H[{},{}]={} != omega0(a_{}, a_{})={}",
                    i,
                    j,
                    kkt[(i, j)],
                    perm[i],
                    perm[j],
                    expected
                );
            }
            // Diagonal is zero
            assert!(
                kkt[(i, i)].abs() < 1e-15,
                "H[{0},{0}]={1} should be 0",
                i,
                kkt[(i, i)]
            );
        }

        // Check A block (rows 0..m, cols m..m+4) and A^T (rows m..m+4, cols 0..m).
        for i in 0..m {
            for d in 0..4 {
                let expected = dual_verts[perm[i]][d];
                assert!(
                    (kkt[(i, m + d)] - expected).abs() < 1e-15,
                    "A[{},{}] mismatch",
                    i,
                    d
                );
                assert!(
                    (kkt[(m + d, i)] - expected).abs() < 1e-15,
                    "A^T[{},{}] mismatch",
                    d,
                    i
                );
            }
        }

        // Check ones block (rows 0..m, col m+4) and ones^T (row m+4, cols 0..m).
        for i in 0..m {
            assert!(
                (kkt[(i, m + 4)] - 1.0).abs() < 1e-15,
                "ones[{}] mismatch",
                i
            );
            assert!(
                (kkt[(m + 4, i)] - 1.0).abs() < 1e-15,
                "ones^T[{}] mismatch",
                i
            );
        }

        // Check zero block (bottom-right 5x5).
        for i in m..m + 5 {
            for j in m..m + 5 {
                assert!(
                    kkt[(i, j)].abs() < 1e-15,
                    "Zero block [{},{}]={} should be 0",
                    i,
                    j,
                    kkt[(i, j)]
                );
            }
        }

        // Check RHS: [0, ..., 0, 1].
        for i in 0..m + 4 {
            assert!(rhs[i].abs() < 1e-15, "rhs[{}]={} should be 0", i, rhs[i]);
        }
        assert!(
            (rhs[m + 4] - 1.0).abs() < 1e-15,
            "rhs[{}]={} should be 1.0",
            m + 4,
            rhs[m + 4]
        );
    }

    /// Permutation reordering: flat QP assembly with permuted indices should
    /// produce different H entries when the permutation changes.
    #[test]
    fn permutation_reorders_matrices() {
        let polytope = make_test_polytope();
        let n = polytope.facet_count();
        if n < 3 {
            return; // Need at least 3 facets for a non-trivial permutation
        }

        let perm_identity: Vec<usize> = (0..3).collect();
        let perm_reversed: Vec<usize> = vec![2, 1, 0];

        let dual_verts = polytope.dual_vertices_f64();
        let qp_id = build_qp_from_dual_vertices(dual_verts, &perm_identity);
        let qp_rev = build_qp_from_dual_vertices(dual_verts, &perm_reversed);

        // H[0,1] with identity uses facets (0,1); with reversed uses facets (2,1).
        let expected_id_01 = omega0(&dual_verts[0], &dual_verts[1]);
        let expected_rev_01 = omega0(&dual_verts[2], &dual_verts[1]);

        assert!(
            (qp_id.h[(0, 1)] - expected_id_01).abs() < 1e-15,
            "Identity permutation H[0,1] mismatch"
        );
        assert!(
            (qp_rev.h[(0, 1)] - expected_rev_01).abs() < 1e-15,
            "Reversed permutation H[0,1] mismatch"
        );
    }

    /// For a hypercube with axis-aligned normals, many omega_0 values are zero
    /// (omega_0(e_i, e_j) = 0 unless (i,j) is a symplectic pair like (q1,p1) or (q2,p2)).
    /// The augmented system's H block should reflect this sparsity.
    #[test]
    fn hypercube_augmented_h_sparsity() {
        let polytope = make_test_polytope();
        let dual_vertices = polytope.dual_vertices_f64();
        let n = polytope.facet_count();
        let perm: Vec<usize> = (0..n).collect();

        let (kkt, _) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);

        // Count non-zero entries in the H block (m x m).
        // For axis-aligned normals, omega_0(e_i, e_j) != 0 only for
        // symplectic pairs: (q1,p1), (q2,p2), i.e. coordinate pairs (0,2) and (1,3).
        let m = perm.len();
        let mut nonzero_count = 0;
        for i in 0..m {
            for j in (i + 1)..m {
                if kkt[(i, j)].abs() > 1e-15 {
                    nonzero_count += 1;
                }
            }
        }
        // With 8 axis-aligned facets, most pairs have omega_0 = 0.
        // The non-zero pairs are those with normals forming symplectic pairs.
        // This is a structural sanity check, not an exact count (depends on facet ordering).
        assert!(
            nonzero_count < m * (m - 1) / 2,
            "Hypercube H block should be sparse, but has {} / {} non-zero upper-triangle entries",
            nonzero_count,
            m * (m - 1) / 2
        );
    }

    // ── Helpers ──

    /// Construct a test polytope (4D hypercube [-1,1]^4 with 8 facets).
    ///
    /// Uses axis-aligned normals, making omega_0 values easy to verify by hand.
    fn make_test_polytope() -> Polytope4D {
        use nalgebra::Vector4;
        let normals = [
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ];
        let heights = [1.0; 8];
        Polytope4D::from_f64(
            normals
                .iter()
                .zip(heights.iter())
                .map(|(n, &h)| n / h)
                .collect(),
        )
        .expect("Hypercube construction should succeed")
    }
}
