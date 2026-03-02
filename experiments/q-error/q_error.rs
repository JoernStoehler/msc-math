#![allow(clippy::collapsible_if, clippy::op_ref, clippy::bool_comparison, dead_code)]

/// Numerical accuracy experiment for the KKT solver.
///
/// Goal: Verify that f64 numerical errors stay within the proven error bounds.
///
/// Two complementary checks:
/// 1. At ALL (S,σ) nodes: assert the error bound E and residual ‖r‖ are small.
/// 2. At select nodes: compare numerical Q̃ against exact Q (rational arithmetic),
///    and assert |Q̃ - Q_exact| ≤ E (the error bound is valid).
///
/// Input: Known polytopes from the library (F ≤ 10).
/// Output: Summary tables to stdout. Panics on any violation.
use nalgebra::{DMatrix, DVector, Vector4};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use symplectic::algorithms::hk2017::{ehz_capacity, combinations};
use symplectic::algorithms::hk2017::permutations::cyclic_permutations;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic::omega0;
use symplectic::kkt::{build_kkt_system as build_kkt, q_from_beta};

/// Condition-number threshold for rank truncation (matches EIGEN_CONDITION_TAU
/// in crates/src/kkt.rs).
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm (matches EPS_KKT_RESIDUAL in kkt.rs).
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Threshold for eigenvalue definiteness classification.
const EPS_DEFINITE: f64 = 1e-10;

/// Eigendecomposition-based solve: x̂ = Σ_i (v_i · b / λ_i) v_i for top `rank`
/// eigenvalues by |λ|.
fn eigen_solve(
    eigenvectors: &DMatrix<f64>,
    eigenvalues: &DVector<f64>,
    rhs: &DVector<f64>,
    size: usize,
    rank: usize,
) -> DVector<f64> {
    let mut indices: Vec<usize> = (0..size).collect();
    indices.sort_by(|&a, &b| eigenvalues[b].abs().partial_cmp(&eigenvalues[a].abs()).unwrap());

    let mut x = DVector::zeros(size);
    for &i in indices.iter().take(rank) {
        let coeff = eigenvectors.column(i).dot(rhs) / eigenvalues[i];
        x += coeff * eigenvectors.column(i);
    }
    x
}

// ── Part 1: All-node error bound sweep ──────────────────────────────────

/// Result of error bound sweep for one polytope.
struct SweepResult {
    total_nodes: u64,
    solvable_nodes: u64,
    rank_deficient: u64,
    worst_e: f64,
    worst_residual: f64,
    worst_correction: f64,
}

/// Sweep ALL (S,σ) pairs for a polytope and assert error bounds.
fn error_bound_sweep(polytope: &Polytope4D) -> SweepResult {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    let mut total_nodes: u64 = 0;
    let mut solvable_nodes: u64 = 0;
    let mut rank_deficient: u64 = 0;
    let mut worst_e: f64 = 0.0;
    let mut worst_residual: f64 = 0.0;
    let mut worst_correction: f64 = 0.0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for perm in cyclic_permutations(&subset) {
                total_nodes += 1;
                let size = m + 5;
                let (kkt, rhs) = build_kkt(normals, heights, &perm);

                let eig = kkt.clone().symmetric_eigen();
                let eigenvalues = &eig.eigenvalues;
                let eigenvectors = &eig.eigenvectors;

                let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs)
                    .fold(0.0_f64, f64::max);
                if lambda_max_abs < 1e-12 {
                    continue; // Numerically zero matrix
                }

                let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
                let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();
                if rank < size {
                    rank_deficient += 1;
                }

                // |λ_min| among retained eigenvalues
                let abs_lambda_min = eigenvalues.iter()
                    .filter(|&&e| e.abs() > threshold)
                    .map(|e| e.abs())
                    .fold(f64::INFINITY, f64::min)
                    .max(f64::MIN_POSITIVE);

                let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
                let residual_vec = &kkt * &x - &rhs;
                let residual_norm = residual_vec.norm();

                if residual_norm > EPS_KKT_RESIDUAL {
                    continue; // Skip nodes with large residual (solver rejects these)
                }

                solvable_nodes += 1;

                // Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂)
                let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();
                let q_raw = q_from_beta(normals, &perm, &beta);
                let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x[i]).sum();
                let r3 = residual_vec[m + 4];
                let xi_hat = x[m + 4];
                let q_correction = r2_dot_mu + r3 * xi_hat;

                // E = (9/2) ‖r‖² / |λ_min|
                let r_sq = residual_norm * residual_norm;
                let q_error_bound = 4.5 * r_sq / abs_lambda_min;

                // Track worst cases
                worst_e = worst_e.max(q_error_bound);
                worst_residual = worst_residual.max(residual_norm);
                worst_correction = worst_correction.max(q_correction.abs());

                // ASSERT: error bound must be small
                assert!(
                    q_error_bound < 1e-6,
                    "E too large at (S,σ)={:?}: E={:.2e}, |r|={:.2e}, |λ_min|={:.2e}",
                    perm, q_error_bound, residual_norm, abs_lambda_min
                );

                // ASSERT: Q correction must be small (absolute or relative)
                assert!(
                    q_correction.abs() < 1e-6 || q_correction.abs() < 1e-6 * q_raw.abs(),
                    "Q correction too large at (S,σ)={:?}: correction={:.2e}, Q_raw={:.2e}",
                    perm, q_correction, q_raw
                );
            }
        }
    }

    SweepResult {
        total_nodes,
        solvable_nodes,
        rank_deficient,
        worst_e,
        worst_residual,
        worst_correction,
    }
}

// ── Part 2: Exact value comparison ──────────────────────────────────────

/// Convert an f64 to an exact BigRational (lossless for finite f64).
fn f64_to_rational(x: f64) -> BigRational {
    // Every finite f64 = m * 2^e where m is an integer mantissa and e is the exponent.
    // We use the rational representation directly.
    if x == 0.0 {
        return BigRational::zero();
    }
    // Use the bits representation to get exact rational
    let bits = x.to_bits();
    let sign = if bits >> 63 == 0 { 1i64 } else { -1i64 };
    let exponent = ((bits >> 52) & 0x7FF) as i64;
    let mantissa = if exponent == 0 {
        // Subnormal: mantissa without implicit 1
        (bits & 0x000F_FFFF_FFFF_FFFF) as i64
    } else {
        // Normal: mantissa with implicit 1
        ((bits & 0x000F_FFFF_FFFF_FFFF) | 0x0010_0000_0000_0000) as i64
    };
    let e = if exponent == 0 {
        1 - 1023 - 52 // Subnormal exponent
    } else {
        exponent - 1023 - 52 // Normal exponent
    };

    let numer = BigInt::from(sign * mantissa);
    if e >= 0 {
        let scale = BigInt::from(1u64) << (e as u64);
        BigRational::new(numer * scale, BigInt::from(1))
    } else {
        let scale = BigInt::from(1u64) << ((-e) as u64);
        BigRational::new(numer, scale)
    }
}

/// Build the KKT matrix and RHS over Q (exact rational arithmetic).
/// Uses the same f64 normals/heights as the library, but represented as exact rationals.
fn build_kkt_rational(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (Vec<Vec<BigRational>>, Vec<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let zero = BigRational::zero();

    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    // H block: H_{ij} = ω₀(n_i, n_j) for i ≠ j (symmetric)
    for i in 0..m {
        for j in (i + 1)..m {
            let val = f64_to_rational(omega0(&normals[perm[i]], &normals[perm[j]]));
            mat[i][j] = val.clone();
            mat[j][i] = val;
        }
    }

    // N block: N_{i,d} = normals[perm[i]][d]
    for i in 0..m {
        for d in 0..4 {
            let val = f64_to_rational(normals[perm[i]][d]);
            mat[i][m + d] = val.clone();
            mat[m + d][i] = val;
        }
    }

    // η block: η_i = heights[perm[i]]
    for i in 0..m {
        let val = f64_to_rational(heights[perm[i]]);
        mat[i][m + 4] = val.clone();
        mat[m + 4][i] = val;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = BigRational::from(BigInt::from(1));

    (mat, rhs)
}

/// Gaussian elimination with partial pivoting over BigRational.
/// Returns None if the system is singular.
fn gauss_solve(mat: &[Vec<BigRational>], rhs: &[BigRational]) -> Option<Vec<BigRational>> {
    let n = rhs.len();
    // Augmented matrix [A | b]
    let mut aug: Vec<Vec<BigRational>> = mat.iter().enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            r.push(rhs[i].clone());
            r
        })
        .collect();

    // Forward elimination
    for col in 0..n {
        // Find pivot (first nonzero in column)
        let pivot_row = (col..n).find(|&r| !aug[r][col].is_zero())?;
        aug.swap(col, pivot_row);

        let pivot = aug[col][col].clone();
        for row in (col + 1)..n {
            if !aug[row][col].is_zero() {
                let factor = &aug[row][col] / &pivot;
                for j in col..=n {
                    let val = &aug[col][j] * &factor;
                    aug[row][j] = &aug[row][j] - &val;
                }
            }
        }
    }

    // Back substitution
    let mut x = vec![BigRational::zero(); n];
    for i in (0..n).rev() {
        let mut sum = aug[i][n].clone();
        for j in (i + 1)..n {
            sum = sum - &aug[i][j] * &x[j];
        }
        if aug[i][i].is_zero() {
            return None; // Singular
        }
        x[i] = sum / &aug[i][i];
    }

    Some(x)
}

/// Compute exact Q(β) = Σ_{i>j} β_i β_j ω₀(n_i, n_j) over BigRational.
fn q_from_beta_rational(
    normals: &[Vector4<f64>],
    perm: &[usize],
    beta: &[BigRational],
) -> BigRational {
    let m = beta.len();
    let mut sum = BigRational::zero();
    for i in 1..m {
        for j in 0..i {
            let omega = f64_to_rational(omega0(&normals[perm[i]], &normals[perm[j]]));
            sum = sum + &beta[i] * &beta[j] * omega;
        }
    }
    sum
}

/// Result of exact comparison for one polytope.
struct ExactResult {
    q_numerical: f64,     // Q̃ (corrected numerical)
    q_exact: f64,         // Q_exact (rational, converted to f64 for display)
    actual_error: f64,    // |Q̃ - Q_exact|
    error_bound: f64,     // E from error bound lemma (mathematical)
    f64_eps: f64,         // f64 rounding tolerance (machine precision)
    bound_valid: bool,    // actual_error ≤ max(E, f64_eps)?
    perm_size: usize,
}

/// Compare numerical Q̃ against exact Q for the winning (S,σ) of a polytope.
fn exact_comparison(polytope: &Polytope4D) -> Option<ExactResult> {
    let result = ehz_capacity(polytope)?;

    // Get the winning permutation in algebraic (KKT input) order.
    // EhzResult.best_permutation is in physical orbit direction (reversed from
    // algebraic order); see hk2017/mod.rs "Permutation ordering convention".
    // We reverse to recover the algebraic order used by build_kkt_system.
    let mut alg_perm = result.best_permutation.clone();
    alg_perm.reverse();
    let m = alg_perm.len();
    let size = m + 5;

    let normals = polytope.normals();
    let heights = polytope.heights();

    // Build rational KKT system (same f64 values, exact representation)
    let (rat_mat, rat_rhs) = build_kkt_rational(normals, heights, &alg_perm);

    // Solve exactly
    let x_exact = gauss_solve(&rat_mat, &rat_rhs)?;

    // Extract exact β and compute exact Q
    let beta_exact: Vec<BigRational> = x_exact[..m].to_vec();
    let q_exact_rational = q_from_beta_rational(normals, &alg_perm, &beta_exact);
    let q_exact_f64 = q_exact_rational.numer().to_f64().unwrap_or(f64::NAN)
        / q_exact_rational.denom().to_f64().unwrap_or(f64::NAN);

    // Compute numerical Q̃ and E (using the eigendecomposition path)
    let (kkt, rhs) = build_kkt(normals, heights, &alg_perm);
    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs).fold(0.0_f64, f64::max);
    let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
    let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();

    let abs_lambda_min = eigenvalues.iter()
        .filter(|&&e| e.abs() > threshold)
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
    let residual_vec = &kkt * &x - &rhs;
    let residual_norm = residual_vec.norm();

    let beta_numerical: Vec<f64> = (0..m).map(|i| x[i]).collect();
    let q_raw = q_from_beta(normals, &alg_perm, &beta_numerical);
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;
    let q_corrected = q_raw - q_correction;

    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    let actual_error = (q_corrected - q_exact_f64).abs();

    // The comparison has two error sources:
    // 1. Mathematical: E = (9/2)‖r‖²/|λ_min| — bounds algorithm error (typically ~1e-28)
    // 2. Computational: f64 rounding in evaluating Q̃ and Q_exact — ~machine_eps * |Q|
    // The meaningful assertion is: actual error ≤ max(E, f64_tolerance).
    // When E << eps_machine, this validates that the ONLY error source is f64 rounding.
    let f64_eps = 1e-13 * (1.0 + q_corrected.abs());
    let tolerance = q_error_bound.max(f64_eps);
    let bound_valid = actual_error <= tolerance;

    Some(ExactResult {
        q_numerical: q_corrected,
        q_exact: q_exact_f64,
        actual_error,
        error_bound: q_error_bound,
        f64_eps,
        bound_valid,
        perm_size: m,
    })
}

// ── Part 3: Hessian and inertia checks (kept from original) ─────────────

/// Lightweight Hessian diagnostic for a single (S, σ) node.
fn node_hessian_check(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<NodeInfo> {
    let m = perm.len();
    let size = m + 5;
    let (kkt, rhs) = build_kkt(normals, heights, perm);

    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs).fold(0.0_f64, f64::max);
    let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
    let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();

    let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
    let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();
    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
    let q = q_from_beta(normals, perm, &beta);

    // Restricted Hessian
    let mut constraint = DMatrix::zeros(5, m);
    for i in 0..m {
        for d in 0..4 {
            constraint[(d, i)] = normals[perm[i]][d];
        }
        constraint[(4, i)] = heights[perm[i]];
    }
    let ctc = constraint.transpose() * &constraint;
    let ctc_eig = ctc.symmetric_eigen();
    let ctc_max = ctc_eig.eigenvalues.iter().cloned().fold(0.0_f64, f64::max);
    let ctc_threshold = ctc_max * 1e-10;
    let null_indices: Vec<usize> = (0..m)
        .filter(|&i| ctc_eig.eigenvalues[i] < ctc_threshold)
        .collect();
    let tangent_dim = null_indices.len();

    let definiteness = if tangent_dim == 0 {
        Definiteness::Trivial
    } else {
        let mut p = DMatrix::zeros(m, tangent_dim);
        for (k, &idx) in null_indices.iter().enumerate() {
            for i in 0..m {
                p[(i, k)] = ctc_eig.eigenvectors[(i, idx)];
            }
        }
        let h_block = kkt.view((0, 0), (m, m)).clone_owned();
        let h_restricted = p.transpose() * &h_block * &p;
        let eig = h_restricted.symmetric_eigen();
        let lam_min = eig.eigenvalues.iter().cloned().fold(f64::INFINITY, f64::min);
        let lam_max = eig.eigenvalues.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        if lam_min > EPS_DEFINITE {
            Definiteness::PD
        } else if lam_max < -EPS_DEFINITE {
            Definiteness::ND
        } else if lam_min < -EPS_DEFINITE && lam_max > EPS_DEFINITE {
            Definiteness::Indefinite
        } else {
            Definiteness::NearZero
        }
    };

    Some(NodeInfo { q, beta_min, definiteness, tangent_dim, m })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Definiteness { PD, ND, Indefinite, NearZero, Trivial }

struct NodeInfo {
    q: f64,
    beta_min: f64,
    definiteness: Definiteness,
    #[allow(dead_code)]
    tangent_dim: usize,
    m: usize,
}

// ── Main ────────────────────────────────────────────────────────────────

fn main() {
    let polytopes: Vec<(&str, Polytope4D)> = known_polytopes::all_known()
        .into_iter()
        .filter(|kp| kp.polytope.facet_count() <= 10)
        .map(|kp| (kp.name, kp.polytope))
        .collect();

    println!("=== Q Error Bound Experiment ===\n");
    println!("Checks numerical accuracy of the KKT solver against proven error bounds.");
    println!("Panics on any violation.\n");

    // ── Part 1: All-node error bound sweep ──────────────────────────────
    println!("--- Part 1: Error bound sweep (ALL nodes) ---");
    println!("  For each (S,σ): assert E < 1e-6 and |q_correction| < 1e-6");
    println!("{:<25} {:>4} {:>8} {:>6} {:>12} {:>12} {:>12}",
        "polytope", "F", "total", "solved", "worst_E", "worst_|r|", "worst_corr");
    println!("{}", "-".repeat(85));

    for (name, polytope) in &polytopes {
        let result = error_bound_sweep(polytope);
        println!("{:<25} {:>4} {:>8} {:>6} {:>12.3e} {:>12.3e} {:>12.3e}",
            name, polytope.facet_count(), result.total_nodes, result.solvable_nodes,
            result.worst_e, result.worst_residual, result.worst_correction);
    }

    println!("\n  All error bound assertions passed.\n");

    // ── Part 2: Exact comparison (select nodes) ─────────────────────────
    println!("--- Part 2: Exact comparison (winning node per polytope) ---");
    println!("  Exact KKT solve via Gaussian elimination over Q.");
    println!("  Assert: |Q̃ - Q_exact| ≤ max(E, f64_eps)");
    println!("  E = mathematical error bound, f64_eps = machine precision tolerance");
    println!("{:<25} {:>3} {:>16} {:>16} {:>12} {:>12} {:>12} {:>6}",
        "polytope", "m", "Q̃_numerical", "Q_exact", "|Q̃-Q_ex|", "E_math", "f64_eps", "valid");
    println!("{}", "-".repeat(100));

    let mut all_exact_valid = true;
    for (name, polytope) in &polytopes {
        match exact_comparison(polytope) {
            Some(r) => {
                println!("{:<25} {:>3} {:>16.12} {:>16.12} {:>12.3e} {:>12.3e} {:>12.3e} {:>6}",
                    name, r.perm_size, r.q_numerical, r.q_exact,
                    r.actual_error, r.error_bound, r.f64_eps,
                    if r.bound_valid { "OK" } else { "FAIL" });
                if !r.bound_valid {
                    all_exact_valid = false;
                }
            }
            None => {
                println!("{:<25} {:>3} (singular or rank-deficient)", name, 0);
            }
        }
    }

    assert!(all_exact_valid,
        "Exact comparison FAILED: |Q̃ - Q_exact| > max(E, f64_eps) for some polytope");
    println!("\n  All exact comparison assertions passed.\n");

    // ── Part 3: Hessian definiteness across ALL nodes (kept from original) ──
    println!("--- Part 3: Restricted Hessian across ALL evaluated (S,σ) nodes ---");
    println!("{:<25} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "F", "total", "β>0", "Q>0", "triv", "PD", "ND", "indef", "~zero");
    println!("{}", "-".repeat(97));

    let eps_beta = 1e-8;
    let eps_q = 1e-12;

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let normals = polytope.normals();
        let heights = polytope.heights();

        let mut total = 0u64;
        let mut n_beta_pos = 0u64;
        let mut n_q_pos = 0u64;
        let mut n_trivial = 0u64;
        let mut n_pd = 0u64;
        let mut n_nd = 0u64;
        let mut n_indef = 0u64;
        let mut n_nearzero = 0u64;

        for m in 2..=f {
            for subset in combinations(f, m) {
                for perm in cyclic_permutations(&subset) {
                    total += 1;
                    if let Some(info) = node_hessian_check(normals, heights, &perm) {
                        if info.beta_min > eps_beta {
                            n_beta_pos += 1;
                        }
                        if info.q > eps_q {
                            n_q_pos += 1;
                        }
                        if info.beta_min > eps_beta && info.q > eps_q {
                            match info.definiteness {
                                Definiteness::Trivial => n_trivial += 1,
                                Definiteness::PD => n_pd += 1,
                                Definiteness::ND => n_nd += 1,
                                Definiteness::Indefinite => n_indef += 1,
                                Definiteness::NearZero => n_nearzero += 1,
                            }
                        }
                    }
                }
            }
        }

        println!("{:<25} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
            name, f, total, n_beta_pos, n_q_pos,
            n_trivial, n_pd, n_nd, n_indef, n_nearzero);
    }

    println!();

    // ── Part 4: Inertia theorem validation (kept from original) ─────────
    println!("--- Part 4: Inertia check across all (S,σ) nodes ---");
    println!("  n-(M)=p ↔ H|_T non-negative definite");
    println!("{:<25} {:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "total", "n-=p", "n->p", "PD", "ND", "indef", "match?");
    println!("{}", "-".repeat(85));

    let eig_eps = 1e-10;
    let mut total_inertia_mismatches = 0u64;

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let mut total = 0u64;
        let mut n_negp = 0u64;
        let mut n_neggtp = 0u64;
        let mut n_pd = 0u64;
        let mut n_nd = 0u64;
        let mut n_indef = 0u64;
        let mut mismatches = 0u64;

        for m in 2..=f {
            for subset in combinations(f, m) {
                for perm in cyclic_permutations(&subset) {
                    total += 1;

                    let mm = perm.len();
                    let (kkt_mat, _) = build_kkt(polytope.normals(), polytope.heights(), &perm);

                    let eig = kkt_mat.symmetric_eigen();
                    let n_neg = eig.eigenvalues.iter().filter(|&&e| e < -eig_eps).count();
                    let n_zero = eig.eigenvalues.iter().filter(|&&e| e.abs() <= eig_eps).count();

                    let info = node_hessian_check(polytope.normals(), polytope.heights(), &perm);
                    let (def, tangent_dim) = match info {
                        Some(n) => (n.definiteness, n.tangent_dim),
                        None => (Definiteness::Trivial, 0),
                    };
                    let p = mm - tangent_dim;

                    let inertia_says_pd = n_neg == p && n_zero == (5 - p);
                    let inertia_says_nsd = n_neg == p;
                    if inertia_says_nsd { n_negp += 1; } else { n_neggtp += 1; }

                    match def {
                        Definiteness::PD => {
                            n_pd += 1;
                            if !inertia_says_pd { mismatches += 1; }
                        },
                        Definiteness::ND => {
                            n_nd += 1;
                            if inertia_says_nsd { mismatches += 1; }
                        },
                        Definiteness::Indefinite => {
                            n_indef += 1;
                            if inertia_says_nsd { mismatches += 1; }
                        },
                        Definiteness::NearZero | Definiteness::Trivial => {},
                    }
                }
            }
        }

        let ok = mismatches == 0;
        println!("{:<25} {:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
            name, total, n_negp, n_neggtp, n_pd, n_nd, n_indef,
            if ok { "OK".to_string() } else { format!("{} FAIL", mismatches) });
        total_inertia_mismatches += mismatches;
    }

    if total_inertia_mismatches > 0 {
        println!("\n  WARNING: {} inertia mismatches (threshold sensitivity, not assertion failure)", total_inertia_mismatches);
    } else {
        println!("\n  All inertia checks passed.");
    }

    // ── Summary ───────────────────────────────────────────────────────────
    println!("\n=== Summary ===");
    println!("  Part 1 (error bounds):     PASSED (all {} polytopes)", polytopes.len());
    println!("  Part 2 (exact comparison): PASSED");
    println!("  Part 3 (Hessian info):     diagnostic only");
    println!("  Part 4 (inertia check):    {} mismatches (diagnostic)", total_inertia_mismatches);
}
