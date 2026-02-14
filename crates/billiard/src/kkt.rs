/// KKT solver for the billiard algorithm.
///
/// Duplicated from hk2017 to keep crates independent.
/// The billiard copy may later diverge (e.g. exploiting ω₀ sparsity
/// for Lagrangian products).
use geom::symplectic::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

/// Minimum β_i value to consider a solution valid (filters numerical noise near zero).
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(β) value to consider a solution valid (avoids division-by-near-zero in action).
pub const EPS_Q_POSITIVE: f64 = 1e-15;

/// SVD tolerance for solving the KKT system (singular values below this are treated as zero).
const EPS_SVD_TOLERANCE: f64 = 1e-10;

/// Maximum acceptable residual norm for the KKT solution (rejects numerically poor solutions).
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Solve the KKT system for max Q(β) subject to N^T β = 0, η^T β = 1.
///
/// The KKT conditions are:
///   H β = N λ + ν η      (m equations)
///   N^T β = 0             (4 equations)
///   η^T β = 1             (1 equation)
///
/// This is an (m+5) × (m+5) linear system in (β, λ, ν).
///
/// Returns Some((β, Q(β))) if the system has a unique solution, None otherwise.
pub fn solve_kkt(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<(Vec<f64>, f64)> {
    let m = perm.len();

    // Build KKT system directly:
    // [ H    | -N   | -η ] [ β ]   [ 0 ]
    // [ N^T  |  0   |  0 ] [ λ ] = [ 0 ]
    // [ η^T  |  0   |  0 ] [ ν ]   [ 1 ]
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    // Top-left: H (m×m) — action matrix with ω₀ values
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }

    // Top block columns m..m+4: -N (m×4) and bottom block: N^T (4×m)
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }

    // Top block column m+4: -η and last row: η^T
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
        kkt[(m + 4, i)] = h;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = 1.0;

    // Solve KKT system. Try LU first (fast), fall back to SVD for rank-deficient systems.
    let lu = kkt.clone().full_piv_lu();
    let solution = if lu.is_invertible() {
        lu.solve(&rhs)?
    } else {
        let svd = kkt.clone().svd(true, true);
        svd.solve(&rhs, EPS_SVD_TOLERANCE).ok()?
    };

    // Verify the solution satisfies the constraints
    let residual = (&kkt * &solution - &rhs).norm();
    if residual > EPS_KKT_RESIDUAL {
        return None;
    }

    // Extract β (first m components)
    let beta: Vec<f64> = (0..m).map(|i| solution[i]).collect();

    // Compute Q(β) = Σ_{j<i} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)})
    let q_val: f64 = (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum();

    Some((beta, q_val))
}
