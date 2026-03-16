//! Polytope4D + permutation -> QP matrices or augmented KKT system.
//!
//! Bridges geometry (dual vertices, normals, heights) to the solver's abstract
//! matrix inputs. Two assembly modes:
//!
//! - `build_qp`: assembles the QP struct {C, d, H} using dual vertices directly.
//!   Used by the projection solver path.
//! - `build_augmented_system`: assembles the (m+5)x(m+5) saddle-point system
//!   using normals and heights. Used by the eigendecomposition solver path.
//!
//! Mathematical correspondence: [lem:kkt]

use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::omega0;
use super::QP;
use nalgebra::{DMatrix, DVector};

/// Assemble the QP {C, d, H} from a polytope and cyclic permutation.
///
/// Given a polytope with dual vertices a_i (= h_i * n_i, where n_i is the outward
/// unit normal and h_i the support distance) and a permutation sigma of m facet
/// indices, assembles:
///
/// - **C** (5 x m): closure constraints (sum a_{sigma(i)} beta_i = 0, four rows)
///   plus normalization (sum beta_i = 1, one row). Note: when using dual vertices
///   directly, the closure constraint is sum a_{sigma(i)} beta_i = 0 (not normals).
/// - **d** (5 x 1): [0, 0, 0, 0, 1]^T
/// - **H** (m x m): action matrix, H_{ij} = omega_0(a_{sigma(i)}, a_{sigma(j)}).
///   Symmetric because omega_0 is antisymmetric: H_{ij} = omega_0(a_i, a_j) and
///   H_{ji} = omega_0(a_j, a_i) = -omega_0(a_i, a_j), but H_{ij} is defined as
///   the SYMMETRIC matrix whose quadratic form gives the action. Specifically,
///   H_{ij} = H_{ji} = omega_0(a_{sigma(i)}, a_{sigma(j)}) for i < j, and H_{ii} = 0.
///
/// Uses dual vertices directly (not normalized normals), which simplifies the
/// constraint structure: the closure + normalization constraints become a single
/// linear system without the height scaling that appears in the normals/heights
/// parameterization.
///
/// # Panics
/// - If any index in `perm` is out of bounds for the polytope's facets.
///
/// [lem:kkt]: KKT optimality conditions characterize the EHZ capacity optimum.
pub fn build_qp(polytope: &Polytope4D, perm: &[usize]) -> QP {
    let m = perm.len();
    let dual_verts = polytope.dual_vertices_f64();

    // Constraint matrix C (5 x m):
    // Rows 0..3: closure constraint sum_i a_{sigma(i)} beta_i = 0 (per coordinate)
    // Row 4: normalization sum_i beta_i = 1
    //
    // Note: When using dual vertices a_i = h_i * n_i, the closure constraint
    // N^T beta = 0 (with normals) becomes A^T beta = 0 (with dual vertices),
    // because N = A * diag(1/||a_i||) and the height normalization absorbs the
    // scaling. The normalization constraint changes from eta^T beta = 1 (with
    // heights eta_i = 1/||a_i||) to simply sum(beta_i) = 1 when we use dual
    // vertices and fold the ||a_i|| factors into the definition of beta.
    //
    // TODO: Verify this dual-vertex formulation against the normals/heights
    // formulation. The closure constraint in the normals parameterization is
    // sum n_{sigma(i)} beta_i = 0 with normalization sum h_{sigma(i)} beta_i = 1.
    // With dual vertices a_i = h_i * n_i, we get sum a_{sigma(i)} beta'_i = 0
    // and sum beta'_i = 1, where beta'_i = h_{sigma(i)} * beta_i. The QP
    // operates in the beta' (dual-vertex) parameterization.
    let mut c = DMatrix::zeros(5, m);
    for (col, &facet_idx) in perm.iter().enumerate() {
        let a = &dual_verts[facet_idx];
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
            let val = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
            h[(i, j)] = val;
            h[(j, i)] = val;
        }
    }

    QP { c, d, h }
}

/// Assemble the augmented (m+5)x(m+5) KKT system from a polytope and permutation.
///
/// Builds the symmetric saddle-point matrix M and right-hand side b:
///
/// ```text
/// [ H   |  N   |  eta ] [ beta ]   [ 0 ]
/// [ N^T |  0   |  0   ] [  mu  ] = [ 0 ]
/// [eta^T|  0   |  0   ] [  xi  ]   [ 1 ]
/// ```
///
/// where:
/// - H (m x m): action matrix, H_{ij} = omega_0(n_{sigma(i)}, n_{sigma(j)})
/// - N (m x 4): facet normals, N_{i,d} = n_{sigma(i),d}
/// - eta (m x 1): facet heights, eta_i = h_{sigma(i)}
///
/// Stationarity: H beta + N mu + eta xi = 0, with Lagrange multipliers mu in R^4, xi in R.
/// Symmetry enables eigendecomposition M = V Lambda V^T.
///
/// Uses **period normalization** (trajectory on [0,T]); see appendix-notation.tex.
///
/// # Panics
/// - If any index in `perm` is out of bounds for the polytope's facets.
///
/// [lem:kkt]: the augmented saddle-point system encodes stationarity + closure + normalization.
pub fn build_augmented_system(
    polytope: &Polytope4D,
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
    let size = m + 5;
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left block: H (m x m) — action matrix with omega_0 values between normals.
    // H_{ij} = omega_0(n_{sigma(i)}, n_{sigma(j)}) for i != j, H_{ii} = 0.
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }

    // Off-diagonal blocks: N (m x 4) and N^T (4 x m) — placed symmetrically.
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = n;
            kkt[(m + d, i)] = n;
        }
    }

    // Off-diagonal blocks: eta (m x 1) and eta^T (1 x m) — placed symmetrically.
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = h;
        kkt[(m + 4, i)] = h;
    }

    // RHS: [0, ..., 0, 1] — normalization constraint.
    rhs[m + 4] = 1.0;

    (kkt, rhs)
}
