//! Tests for qp_assembly: QP and augmented system construction from polytope geometry.
//!
//! Proposition: build_qp and build_augmented_system correctly encode the KKT
//! constraints (closure, normalization) and objective (symplectic action) for
//! a given polytope and facet permutation.
//! Reference: [lem:kkt]
//!
//! Strategy: fixture-based on known polytopes (hypercube), verifying
//! matrix dimensions, symmetry of H, constraint structure, and known values.

use super::qp_assembly::{build_augmented_system, build_qp};
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::omega0;

// ── build_qp tests ──

/// Verify QP dimensions match the permutation length.
#[test]
fn qp_dimensions_match_permutation() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    // Use first 3 facets as a short permutation
    let perm: Vec<usize> = (0..3.min(n)).collect();
    let m = perm.len();

    let qp = build_qp(&polytope, &perm);

    assert_eq!(qp.c.nrows(), 5, "C should have 5 rows (4 closure + 1 normalization)");
    assert_eq!(qp.c.ncols(), m, "C should have m={} columns", m);
    assert_eq!(qp.d.nrows(), 5, "d should have 5 rows");
    assert_eq!(qp.h.nrows(), m, "H should be m x m");
    assert_eq!(qp.h.ncols(), m, "H should be m x m");
}

/// H matrix must be symmetric (omega_0 placed symmetrically: H_{ij} = H_{ji}).
#[test]
fn qp_h_is_symmetric() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..n).collect();
    let m = perm.len();

    let qp = build_qp(&polytope, &perm);

    for i in 0..m {
        for j in 0..m {
            assert!(
                (qp.h[(i, j)] - qp.h[(j, i)]).abs() < 1e-15,
                "H[{},{}]={} != H[{},{}]={}",
                i, j, qp.h[(i, j)], j, i, qp.h[(j, i)]
            );
        }
    }
}

/// H diagonal must be zero (omega_0(a, a) = 0 by antisymmetry of the symplectic form).
#[test]
fn qp_h_diagonal_is_zero() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..n).collect();

    let qp = build_qp(&polytope, &perm);

    for i in 0..perm.len() {
        assert!(
            qp.h[(i, i)].abs() < 1e-15,
            "H[{0},{0}] = {1} should be 0",
            i, qp.h[(i, i)]
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

    let qp = build_qp(&polytope, &perm);

    for i in 0..perm.len() {
        for j in (i + 1)..perm.len() {
            let expected = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
            assert!(
                (qp.h[(i, j)] - expected).abs() < 1e-15,
                "H[{},{}]={} != omega0(a_{}, a_{})={}",
                i, j, qp.h[(i, j)], perm[i], perm[j], expected
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

    let qp = build_qp(&polytope, &perm);

    for (col, &facet_idx) in perm.iter().enumerate() {
        let a = &dual_verts[facet_idx];
        for d in 0..4 {
            assert!(
                (qp.c[(d, col)] - a[d]).abs() < 1e-15,
                "C[{},{}]={} != a_{}[{}]={}",
                d, col, qp.c[(d, col)], facet_idx, d, a[d]
            );
        }
        assert!(
            (qp.c[(4, col)] - 1.0).abs() < 1e-15,
            "C[4,{}]={} should be 1.0",
            col, qp.c[(4, col)]
        );
    }
}

/// d vector is [0, 0, 0, 0, 1].
#[test]
fn qp_rhs_vector() {
    let polytope = make_test_polytope();
    let perm: Vec<usize> = (0..3.min(polytope.facet_count())).collect();

    let qp = build_qp(&polytope, &perm);

    for d in 0..4 {
        assert!(
            qp.d[d].abs() < 1e-15,
            "d[{}]={} should be 0",
            d, qp.d[d]
        );
    }
    assert!(
        (qp.d[4] - 1.0).abs() < 1e-15,
        "d[4]={} should be 1.0",
        qp.d[4]
    );
}

// ── build_augmented_system tests ──

/// Augmented system has size (m+5) x (m+5).
#[test]
fn augmented_system_dimensions() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..3.min(n)).collect();
    let m = perm.len();
    let size = m + 5;

    let (kkt, rhs) = build_augmented_system(&polytope, &perm);

    assert_eq!(kkt.nrows(), size);
    assert_eq!(kkt.ncols(), size);
    assert_eq!(rhs.nrows(), size);
}

/// The augmented KKT matrix must be symmetric (saddle-point structure).
#[test]
fn augmented_system_is_symmetric() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..n).collect();

    let (kkt, _) = build_augmented_system(&polytope, &perm);
    let size = kkt.nrows();

    for i in 0..size {
        for j in 0..size {
            assert!(
                (kkt[(i, j)] - kkt[(j, i)]).abs() < 1e-15,
                "KKT[{},{}]={} != KKT[{},{}]={}",
                i, j, kkt[(i, j)], j, i, kkt[(j, i)]
            );
        }
    }
}

/// The augmented system block structure: H uses normals, off-diagonal blocks
/// contain normal coordinates and heights, bottom-right 5x5 is zero, RHS = [0..0, 1].
#[test]
fn augmented_system_block_structure() {
    let polytope = make_test_polytope();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..n).collect();
    let m = perm.len();

    let (kkt, rhs) = build_augmented_system(&polytope, &perm);

    // Check H block (top-left m x m): omega_0 between permuted normals.
    for i in 0..m {
        for j in (i + 1)..m {
            let expected = omega0(&normals[perm[i]], &normals[perm[j]]);
            assert!(
                (kkt[(i, j)] - expected).abs() < 1e-15,
                "H[{},{}]={} != omega0(n_{}, n_{})={}",
                i, j, kkt[(i, j)], perm[i], perm[j], expected
            );
        }
        // Diagonal is zero
        assert!(
            kkt[(i, i)].abs() < 1e-15,
            "H[{0},{0}]={1} should be 0",
            i, kkt[(i, i)]
        );
    }

    // Check N block (rows 0..m, cols m..m+4) and N^T (rows m..m+4, cols 0..m).
    for i in 0..m {
        for d in 0..4 {
            let expected = normals[perm[i]][d];
            assert!(
                (kkt[(i, m + d)] - expected).abs() < 1e-15,
                "N[{},{}] mismatch",
                i, d
            );
            assert!(
                (kkt[(m + d, i)] - expected).abs() < 1e-15,
                "N^T[{},{}] mismatch",
                d, i
            );
        }
    }

    // Check eta block (rows 0..m, col m+4) and eta^T (row m+4, cols 0..m).
    for i in 0..m {
        let expected = heights[perm[i]];
        assert!(
            (kkt[(i, m + 4)] - expected).abs() < 1e-15,
            "eta[{}] mismatch",
            i
        );
        assert!(
            (kkt[(m + 4, i)] - expected).abs() < 1e-15,
            "eta^T[{}] mismatch",
            i
        );
    }

    // Check zero block (bottom-right 5x5).
    for i in m..m + 5 {
        for j in m..m + 5 {
            assert!(
                kkt[(i, j)].abs() < 1e-15,
                "Zero block [{},{}]={} should be 0",
                i, j, kkt[(i, j)]
            );
        }
    }

    // Check RHS: [0, ..., 0, 1].
    for i in 0..m + 4 {
        assert!(
            rhs[i].abs() < 1e-15,
            "rhs[{}]={} should be 0",
            i, rhs[i]
        );
    }
    assert!(
        (rhs[m + 4] - 1.0).abs() < 1e-15,
        "rhs[{}]={} should be 1.0",
        m + 4, rhs[m + 4]
    );
}

/// Permutation reordering: build_qp with permuted indices should produce
/// different H entries when the permutation changes.
#[test]
fn permutation_reorders_matrices() {
    let polytope = make_test_polytope();
    let n = polytope.facet_count();
    if n < 3 {
        return; // Need at least 3 facets for a non-trivial permutation
    }

    let perm_identity: Vec<usize> = (0..3).collect();
    let perm_reversed: Vec<usize> = vec![2, 1, 0];

    let qp_id = build_qp(&polytope, &perm_identity);
    let qp_rev = build_qp(&polytope, &perm_reversed);

    // H[0,1] with identity uses facets (0,1); with reversed uses facets (2,1).
    let dual_verts = polytope.dual_vertices_f64();
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
    let n = polytope.facet_count();
    let perm: Vec<usize> = (0..n).collect();

    let (kkt, _) = build_augmented_system(&polytope, &perm);

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
    Polytope4D::from_normals_and_heights(normals, heights)
        .expect("Hypercube construction should succeed")
}
