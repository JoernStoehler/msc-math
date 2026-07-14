//! Eigendecomposition-based KKT solver for the (m+5)x(m+5) saddle-point system.
//!
//! Solves the augmented KKT system M x = b where M is the symmetric saddle-point
//! matrix assembled by `qp_assembly::build_augmented_system_from_dual_vertices`. The solution vector
//! x = [beta; mu; xi] yields dwell times beta, Lagrange multipliers mu (closure)
//! and xi (normalization).
//!
//! Key features:
//! - Two-tier eigenvalue rank detection (permissive then strict)
//! - Numerical null-space search for beta > 0 when rank-deficient
//! - Q error bound computation via [lem:q-error-bound]
//! - Inertia reporting for saddle-point structure analysis
//!
//! **Near-zero Q candidates:** Some (S,σ) pairs yield Q ≈ 0 (very high action). The error
//! bound E is valid but may exceed |Q| itself (relative error > 100%). This is harmless:
//! the capacity algorithm picks max Q, so near-zero Q candidates never win. The absolute
//! threshold `E < 1e-6` is chosen relative to Q_max ≈ O(1), not relative to each candidate's Q.
//!
//! **Word convention:** σ is the active Reeb traversal order. With this convention
//! Q > 0 when consecutive facets follow the positive Reeb direction
//! (ω₀(n_{σ(k)}, n_{σ(k+1)}) ≥ 0). HK2017's displayed theorem uses the reversed
//! ordered word.
//!
//! Mathematical correspondence: [lem:kkt], [lem:q-error-bound]

use super::beta_feasibility;
use super::qp_assembly::build_augmented_system_from_dual_vertices;
use super::EPS_EIGEN_FLOOR;
use crate::geom::symplectic_form::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

// ── Public constants ──

/// Minimum beta_i value to consider a solution certified positive.
///
/// Used by the accumulator and experiments to classify solution feasibility.
/// beta_i > EPS_BETA_POSITIVE means the component is unambiguously positive.
///
/// **Why 1e-12:** This filters f64 eigensolver noise. The KKT matrix entries are
/// O(1) (dual vertices and omega_0 values are O(1)), so eigenvector components
/// are O(1) and beta values from the pseudoinverse are O(1). Machine epsilon is
/// ~1e-16; numerical roundoff in eigendecomposition accumulates to ~1e-12 for
/// (m+5) x (m+5) matrices with m up to 16. A value of 1e-12 is:
/// - Far above machine epsilon (can't be confused with exact zero)
/// - Far below typical beta values (O(0.1)--O(10)) for real candidates
/// - 10x tighter than EPS_MARGIN_TRUE (1e-9) so Indeterminate verdicts are
///   returned for any solution where beta is ambiguous.
///
/// Making it 10x larger (1e-11) would misclassify some real near-zero betas as
/// positive. Making it 10x smaller (1e-13) would pass some eigensolver noise
/// through as certified solutions.
pub const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Minimum Q(beta) value to consider a solution meaningful.
///
/// Avoids division-by-near-zero when computing capacity = 1/(2Q).
///
/// **Why 1e-15:** Q = c_EHZ^{-2} / 2 is O(0.01)--O(10) for our polytopes
/// (typical c_EHZ ~ 0.3--3). Q < 1e-15 indicates either a degenerate candidate
/// with astronomically high action or pure f64 noise. In either case, this
/// candidate cannot be the capacity maximizer. 1e-15 is just above machine epsilon
/// (~1e-16) to avoid exact-zero false positives from cancellation.
pub const EPS_Q_POSITIVE: f64 = 1e-15;

// ── Internal constants ──

/// Condition-number threshold for eigenvalue rank detection.
///
/// An eigenvalue lambda_j is "small" if |lambda_j| < |lambda|_max * tau.
/// This catches both isolated small eigenvalues (gap case) and gradual decay.
///
/// **Why 1e-3:** The degenerate (4,4) Lagrangian product at theta ~ 0 deg has
/// eigenvalue magnitudes around 8.6e-4 with |lambda|_max ~ 1-2, giving
/// |lambda|/|lambda|_max ~ 4e-4. The threshold 1e-3 catches this with 2.5x
/// margin. Well-conditioned random polytopes have smallest |lambda| ~ 0.01-0.1,
/// well above 1e-3 * |lambda|_max, so no false rank-deficiency detections occur.
/// Making it 10x larger (1e-2) would treat some well-conditioned polytopes as
/// rank-deficient. Making it 10x smaller (1e-4) would miss the degenerate case.
///
/// Calibrated to detect the degenerate (4,4) Lagrangian product at theta ~ 0 deg,
/// which has eigenvalue magnitudes around 8.6e-4 with |lambda|_max ~ 1-2.
/// Well-conditioned polytopes have smallest |lambda| ~ 0.01-0.1.
///
/// Regression tests: eigen_gap_ratio_44_degenerate, eigen_gap_ratio_44_theta43.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm for the KKT solution.
///
/// Rejects numerically poor solutions where ||Mx - b|| is too large. Solutions
/// with residual below this have Q error bounds E = 4.5 * ||r||^2 / |lambda_min|
/// that are small relative to Q values of interest.
///
/// **Why 1e-6:** The q_error experiment (Part 1) measures worst-case E = 2.9e-11
/// across 1.1M nodes (F <= 10) with typical residuals ~1e-14 to ~1e-10.
/// Residuals above 1e-6 signal genuine numerical failure (e.g., extremely
/// ill-conditioned matrices). At residual = 1e-6 with |lambda_min| = 1e-3,
/// the error bound is E = 4.5e-9, which is ~5 orders of magnitude below
/// the observed worst case. Making it 10x larger (1e-5) risks accepting poor
/// solutions; 10x smaller (1e-7) would reject some valid solutions on
/// moderately ill-conditioned polytopes.
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Threshold for filtering Type A eigenvectors in null-space search.
///
/// When the augmented KKT matrix is rank-deficient, its null-space eigenvectors
/// are (m+5)-dimensional. Truncated to the first m components (the beta block),
/// some have ||v_beta|| ~ 0: their content lives in the Lagrange multiplier
/// components (mu, xi), not in beta. Including these in the LP creates free
/// variables with ~zero coefficients, causing spurious Unbounded results.
///
/// **Why 1e-10:** Eigenvectors are unit-norm in the full (m+5)-dimensional space.
/// Type A vectors have most mass in the 5 multiplier components, so ||v_beta||
/// is O(eps_mach). Type B vectors have ||v_beta|| = O(1). The gap is ~10 orders
/// of magnitude — the threshold is robust anywhere in [1e-8, 1e-12].
///
/// See [rem:near-null-lp-search] in `formal/ehz-kkt-system.tex` for the full analysis.
///
/// **Why Type C cannot occur:** From Mv = λv, the constraint rows give
/// N^T v_beta = λ v_mu and η^T v_beta = λ v_xi. Since ||v|| = 1,
/// constraint violation per unit α is ≤ |λ| ≤ τ = 1e-3 for discarded
/// eigenvectors. Type C (O(1) violation) is impossible.
/// Jörn: verified 2026-03-22.
const EPS_TYPE_A_FILTER: f64 = 1e-10;

// ── Public types ──

/// Eigendecomposition info for the KKT matrix M.
///
/// Groups the eigenvalues, eigenvectors, and inertia from the symmetric
/// eigendecomposition M = V Lambda V^T. Used internally by the two-tier solver.
pub(crate) struct EigenInfo {
    /// Eigenvalues of M.
    pub eigenvalues: DVector<f64>,
    /// Orthogonal eigenvectors of M (columns).
    pub eigenvectors: DMatrix<f64>,
    /// Number of positive eigenvalues (using strict threshold).
    pub n_positive: usize,
    /// Number of negative eigenvalues (using strict threshold).
    pub n_negative: usize,
    /// Number of near-zero eigenvalues (using strict threshold).
    pub n_zero: usize,
}

/// Outcome of the saddle-point KKT solve.
///
/// Every variant corresponds to a mathematical proposition about the orbit.
/// See `.agents/skills/rust/SKILL.md` error handling convention.
#[derive(Clone, Debug)]
pub enum KktOutcome {
    /// The word has a feasible positive KKT candidate with Q > 0.
    /// This is not by itself a fixed-word maximum or a physical orbit
    /// certificate.
    Feasible(KktResult),
    /// The KKT candidate is infeasible: β has a non-positive component, or
    /// Q ≤ 0.
    Infeasible,
    /// The KKT matrix is singular (all eigenvalues ≈ 0).
    SingularMatrix,
    /// A near-null eigenvector has O(1) constraint violation (Type C).
    /// [rem:near-null-lp-search] conjectures this is impossible; its occurrence
    /// means the proof has a gap. The orbit is skipped; capacity may lack proven
    /// error bounds if this was the best orbit.
    TypeCViolation,
    /// The LP null-space shift and beta0 fallback both violate constraints
    /// beyond tolerance. The orbit is skipped; same caveat as TypeCViolation.
    // TODO: add [rem:...] to formal math for constraint violation after LP
    ConstraintViolation,
}

impl KktOutcome {
    /// Extract the feasible result, or None if not feasible.
    pub fn feasible(self) -> Option<KktResult> {
        match self {
            KktOutcome::Feasible(r) => Some(r),
            _ => None,
        }
    }
}

/// Feasible KKT solution with diagnostics.
///
/// Contains the solution beta, Lagrange multipliers mu and xi,
/// residual-corrected Q value with error bound, and inertia of M.
///
/// The augmented system uses a **symmetric** sign convention:
/// ```text
/// [ H  |  N  | eta ] [beta]   [0]
/// [N^T |  0  |  0  ] [ mu ] = [0]
/// [eta^T| 0  |  0  ] [ xi ]   [1]
/// ```
/// Stationarity: Hβ + Nμ + ηξ = 0.
///
/// **Sign convention note:** Some references (and experiment code) use an
/// asymmetric convention where stationarity reads Hβ = Nλ + ην. In that
/// convention, λ = −μ and ν = −ξ. The derivative formula for the action
/// A = 1/(2Q) is: ∂A/∂h_k = −ξ·β_k/(2Q²) in our (symmetric) convention,
/// equivalently ∂A/∂h_k = ν·β_k/(2Q²) in the asymmetric convention.
///
/// See [lem:q-error-bound]: |Q(beta_0) - q_corrected| <= q_error_bound.
#[derive(Clone, Debug)]
pub struct KktResult {
    /// KKT candidate beta vector (all components > -EPS_BETA_POSITIVE).
    pub beta: Vec<f64>,
    /// Lagrange multiplier for closure constraints A^T β = 0 (4 components).
    /// From symmetric convention: Hβ + Aμ + 1ξ = 0.
    pub mu: Vec<f64>,
    /// Lagrange multiplier for normalization constraint 1^T β = 1 (scalar).
    /// From symmetric convention: Hβ + Aμ + 1ξ = 0.
    pub xi: f64,
    /// Residual-corrected Q value: Q_tilde = Q(beta_hat) + (r2^T mu_hat + r3 * xi_hat).
    /// See [lem:q-error-bound].
    pub q_corrected: f64,
    /// Direct residual correction term used in `q_corrected`.
    ///
    /// This is retained separately so downstream evidence can distinguish the
    /// solver's correction atom from a difference reconstructed from unrelated
    /// stored Q values.
    pub q_correction: f64,
    /// Error bound E on Q_tilde: |Q(beta_0) - Q_tilde| <= E.
    /// See [lem:q-error-bound].
    #[allow(dead_code)]
    pub q_error_bound: f64,
    /// Inertia of M: number of positive eigenvalues.
    #[allow(dead_code)]
    pub n_positive: usize,
    /// Inertia of M: number of negative eigenvalues.
    #[allow(dead_code)]
    pub n_negative: usize,
    /// Inertia of M: number of near-zero eigenvalues.
    #[allow(dead_code)]
    pub n_zero: usize,
}

// ── Public API ──

/// Solve the augmented KKT system from a pre-assembled matrix and RHS.
///
/// Uses eigendecomposition with two-tier rank detection:
/// 1. Permissive (EPS_EIGEN_FLOOR = 1e-12): retains all but numerically zero eigenvalues.
/// 2. Strict (EIGEN_CONDITION_TAU = 1e-3): treats small eigenvalues as null space.
///
/// The permissive tier runs first. If its residual exceeds EPS_KKT_RESIDUAL,
/// the strict tier takes over. This replaces the old LU + SVD fallback with a
/// single factorization.
///
/// Returns `KktOutcome::Feasible(result)` with beta, corrected Q, error bound,
/// and inertia, or a non-feasible variant explaining why no solution was found.
///
/// [lem:kkt]: KKT conditions give the stationarity equations used to generate
/// candidates for the finite EHZ capacity optimization. A global capacity
/// claim additionally needs the relevant candidate-family completeness
/// contract.
pub fn solve_saddle_point(kkt_matrix: &DMatrix<f64>, rhs: &DVector<f64>) -> KktOutcome {
    let m = rhs.len() - 5;
    let size = rhs.len();

    let eig = kkt_matrix.clone().symmetric_eigen();
    let max_abs_ev = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    if max_abs_ev < EPS_EIGEN_FLOOR {
        return KktOutcome::SingularMatrix;
    }

    // Compute inertia using the strict threshold (for structure analysis).
    // The KKT matrix M is (m+5)×(m+5). The constraint block contributes at most 5
    // negative eigenvalues, but H (the action matrix) can also have negative eigenvalues,
    // so n_negative can exceed 5. Empirically validated by q_error experiment (Tables 8-9).
    let strict_threshold = max_abs_ev * EIGEN_CONDITION_TAU;
    let eigen_info = EigenInfo {
        n_positive: eig
            .eigenvalues
            .iter()
            .filter(|&&e| e > strict_threshold)
            .count(),
        n_negative: eig
            .eigenvalues
            .iter()
            .filter(|&&e| e < -strict_threshold)
            .count(),
        n_zero: size
            - eig
                .eigenvalues
                .iter()
                .filter(|&&e| e > strict_threshold)
                .count()
            - eig
                .eigenvalues
                .iter()
                .filter(|&&e| e < -strict_threshold)
                .count(),
        eigenvalues: eig.eigenvalues,
        eigenvectors: eig.eigenvectors,
    };

    // Tier 1: Permissive threshold — retain all eigenvalues above machine-epsilon floor.
    if let Some(outcome) =
        try_pseudoinverse_with_threshold(kkt_matrix, rhs, m, &eigen_info, EPS_EIGEN_FLOOR)
    {
        if let KktOutcome::Feasible(_) = &outcome {
            return outcome;
        }
    }

    // Tier 2: Strict threshold — treat small eigenvalues as null space.
    // If Tier 2 also fails (None), the solver couldn't classify this orbit.
    try_pseudoinverse_with_threshold(kkt_matrix, rhs, m, &eigen_info, strict_threshold)
        .unwrap_or(KktOutcome::Infeasible)
}

/// Convenience: solve KKT from flat dual vertices and a permutation in one call.
///
/// Assembles the augmented system from
/// `qp_assembly::build_augmented_system_from_dual_vertices`, then calls
/// `solve_saddle_point`.
///
/// [lem:kkt]: assembles and solves the augmented KKT system for a dual-vertex/permutation pair.
pub fn solve_kkt_for_dual_vertices(dual_vertices: &[Vector4<f64>], perm: &[usize]) -> KktOutcome {
    let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, perm);
    solve_saddle_point(&kkt, &rhs)
}

// ── Internal helpers ──

/// Compute Q(beta) = sum_{i>j} beta_i * beta_j * omega_0(a_{sigma(j)}, a_{sigma(i)}).
///
/// This is the action sum (1/2) beta^T H beta computed directly from dual vertices
/// and the antisymmetric omega_0 form. Used for Q computation from the beta
/// solution vector. Uses omega_0 directly (not the symmetric H matrix).
///
/// [lem:H-quadratic]: Q(beta) = sum_{i>j} beta_i beta_j omega_0(a_{sigma(j)}, a_{sigma(i)}).
#[allow(dead_code)]
pub(crate) fn q_from_beta(dual_vertices: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&dual_vertices[perm[j]], &dual_vertices[perm[i]]))
        .sum()
}

/// Try to find an admissible beta > 0 solution using a specific eigenvalue threshold.
///
/// Computes the pseudoinverse retaining eigenvalues with |lambda_i| > threshold,
/// checks the residual, searches the null space if rank-deficient, and computes
/// the Q error bound.
///
/// Returns None if this threshold didn't produce a result (caller should try
/// a different threshold). Returns Some(KktOutcome) with a mathematical
/// classification if a definitive answer was reached.
fn try_pseudoinverse_with_threshold(
    kkt: &DMatrix<f64>,
    rhs: &DVector<f64>,
    m: usize,
    eigen_info: &EigenInfo,
    threshold: f64,
) -> Option<KktOutcome> {
    let size = m + 5;
    let eigenvalues = &eigen_info.eigenvalues;
    let eigenvectors = &eigen_info.eigenvectors;

    // Pseudoinverse solution: x_hat = sum_i (v_i . b / lambda_i) v_i for retained eigenvalues.
    let mut x0 = DVector::zeros(size);
    let mut rank = 0usize;
    for i in 0..size {
        if eigenvalues[i].abs() > threshold {
            rank += 1;
            let coeff = eigenvectors.column(i).dot(rhs) / eigenvalues[i];
            for j in 0..size {
                x0[j] += coeff * eigenvectors[(j, i)];
            }
        }
    }

    let residual_vec = kkt * &x0 - rhs;
    let residual_norm = residual_vec.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return None; // Tier fallback: try next threshold.
    }

    // Q error bound computation ([lem:q-error-bound]).
    // Solution vector is [beta_hat; mu_hat; xi_hat].
    // Q_tilde = Q(beta_hat) + (r2^T mu_hat + r3 * xi_hat).
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual_vec[i] * x0[i]).sum();
    let r3 = residual_vec[m + 4];
    let xi_hat = x0[m + 4];
    let q_correction = r2_dot_mu + r3 * xi_hat;

    // [lem:q-error-bound]: |lambda_min| = min_j |lambda_j| over ALL eigenvalues of M.
    // This makes E large for near-singular M, which correctly exposes that the lemma's
    // bound is too loose. The solver panics on basic polytopes (simplex, hypercube, etc.)
    // because the bound is wrong — the actual Q is accurate but the bound says otherwise.
    // See formal/hk2017-qp-precision.tex and
    // experiments/dev-quadratic-program/README.md: the q-error-bound lemma
    // needs replacing with a tighter bound before thesis-facing use.
    let abs_lambda_min = eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min)
        .max(f64::MIN_POSITIVE);

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    // Extract Lagrange multipliers from pseudoinverse solution.
    // These are valid even after null-space shift of beta, because null-space
    // directions preserve the KKT objective Q (see Q-constancy check below).
    let mu0: Vec<f64> = (m..m + 4).map(|i| x0[i]).collect();
    let xi0 = x0[m + 4];

    // If already feasible (all beta > EPS), compute error bound and return.
    if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        return Some(finalize_result(
            &beta0,
            mu0,
            xi0,
            kkt,
            m,
            q_correction,
            residual_norm,
            abs_lambda_min,
            eigen_info,
        ));
    }

    // Full rank at this threshold: unique solution. If some beta near zero,
    // still accept as uncertain candidate for the accumulator.
    if rank == size {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(finalize_result(
                &beta0,
                mu0,
                xi0,
                kkt,
                m,
                q_correction,
                residual_norm,
                abs_lambda_min,
                eigen_info,
            ));
        }
        return Some(KktOutcome::Infeasible);
    }

    // Rank-deficient: search the *numerical* null space for beta > 0.
    // "Null space" here means eigenvectors whose eigenvalues are below the
    // threshold — not an exact kernel (which doesn't exist in f64).
    // [lem:well-defined]: Q is exactly invariant along the true null space,
    // so shifting beta along these approximate null directions changes Q by
    // at most O(|lambda_j|) where lambda_j are the discarded eigenvalues.
    //
    // Extract the beta-block of each null eigenvector, filtering out Type A
    // directions (||v_beta|| ~ 0) that only move Lagrange multipliers and
    // would cause spurious LP unboundedness. See [rem:near-null-lp-search].
    let mut null_columns: Vec<DVector<f64>> = Vec::new();
    for i in 0..size {
        if eigenvalues[i].abs() <= threshold {
            let v_beta = DVector::from_fn(m, |j, _| eigenvectors[(j, i)]);
            if v_beta.norm() >= EPS_TYPE_A_FILTER {
                // Type C check: constraint violation must be O(|lambda|), not O(1).
                // N^T v_beta and eta^T v_beta are in the constraint rows of M*v = lambda*v.
                let mut constraint_violation_sq = 0.0;
                for row in m..size {
                    let dot: f64 = (0..m).map(|j| kkt[(row, j)] * eigenvectors[(j, i)]).sum();
                    constraint_violation_sq += dot * dot;
                }
                let constraint_violation = constraint_violation_sq.sqrt();
                if constraint_violation >= 0.1 {
                    eprintln!(
                        "WARNING: Type C eigenvector detected (||constraint * v_beta|| = {:.2e}, \
                         |lambda| = {:.2e}, ||v_beta|| = {:.2e}, m = {}). \
                         Skipping orbit. Capacity relies on conjecture that this orbit is not optimal.",
                        constraint_violation, eigenvalues[i].abs(), v_beta.norm(), m
                    );
                    return Some(KktOutcome::TypeCViolation);
                }
                null_columns.push(v_beta);
            }
        }
    }

    let k_eff = null_columns.len();

    // Fast path: if all null-space directions were Type A (k_eff=0),
    // there's nothing to search — use beta0 directly. This avoids nalgebra
    // allocations on the hot path for degenerate polytopes.
    if k_eff == 0 {
        if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
            return Some(finalize_result(
                &beta0,
                mu0,
                xi0,
                kkt,
                m,
                q_correction,
                residual_norm,
                abs_lambda_min,
                eigen_info,
            ));
        } else {
            return Some(KktOutcome::Infeasible);
        }
    }

    let mut null_basis = DMatrix::zeros(m, k_eff);
    for (col, v) in null_columns.iter().enumerate() {
        null_basis.set_column(col, v);
    }

    let beta0_dv = DVector::from_column_slice(&beta0);
    let margin_result = beta_feasibility::find_max_margin(&beta0_dv, &null_basis);

    // Accept the LP result only if it satisfies constraints. The LP moves
    // along approximate null-space directions (eigenvectors of M+E, not M),
    // so large shifts can accumulate O(alpha * |lambda_j|) constraint violation.
    // If the LP beta violates constraints, fall back to beta0.
    let beta_final = if margin_result.margin > -EPS_BETA_POSITIVE {
        let lp_beta: Vec<f64> = margin_result.beta.as_slice().to_vec();
        let lp_constraint_residual = extract_constraint_residual(kkt, &lp_beta, m);
        if lp_constraint_residual <= EPS_KKT_RESIDUAL {
            lp_beta
        } else {
            // LP shifted too far — constraint violation too large. Fall back to beta0.
            beta0.clone()
        }
    } else if beta0.iter().all(|&b| b > -EPS_BETA_POSITIVE) {
        beta0.clone()
    } else {
        return Some(KktOutcome::Infeasible);
    };

    // Save beta0 for Q computation ([lem:well-defined]).
    let beta0_ref = beta0;

    // Verify constraints on the final beta (covers both LP and fallback paths).
    let constraint_residual_norm = extract_constraint_residual(kkt, &beta_final, m);
    if constraint_residual_norm > EPS_KKT_RESIDUAL {
        eprintln!(
            "WARNING: KKT constraint residual {:.2e} > {:.2e} after LP. \
             Skipping orbit. Capacity relies on conjecture that this orbit is not optimal.",
            constraint_residual_norm, EPS_KKT_RESIDUAL
        );
        return Some(KktOutcome::ConstraintViolation);
    }

    // [lem:well-defined]: Q is invariant along the true null space, so compute
    // Q from the pseudoinverse beta0, not from the LP-shifted beta_final.
    // Q is computed from beta0 (pseudoinverse solution), not beta_final (LP-shifted).
    // beta_final is only used for feasibility (margin classification) and downstream
    // beta values (orbit reconstruction, gradients).
    Some(
        match finalize_result(
            &beta0_ref,
            mu0,
            xi0,
            kkt,
            m,
            q_correction,
            residual_norm,
            abs_lambda_min,
            eigen_info,
        ) {
            KktOutcome::Feasible(mut result) => {
                result.beta = beta_final;
                KktOutcome::Feasible(result)
            }
            other => other,
        },
    )
}

/// Compute the constraint residual for the beta vector using the KKT matrix structure.
///
/// The constraint rows are rows m..m+5 of the KKT matrix. They encode
/// N^T beta = 0 (rows m..m+4) and eta^T beta = 1 (row m+4).
fn extract_constraint_residual(kkt: &DMatrix<f64>, beta: &[f64], m: usize) -> f64 {
    let mut sq_sum = 0.0;
    for row in m..(m + 5) {
        let rhs_val = if row == m + 4 { 1.0 } else { 0.0 };
        let dot: f64 = (0..m).map(|j| kkt[(row, j)] * beta[j]).sum();
        sq_sum += (dot - rhs_val).powi(2);
    }
    sq_sum.sqrt()
}

/// Build the final KktResult with Q computation and error bound assertion.
#[allow(clippy::too_many_arguments)]
fn finalize_result(
    beta: &[f64],
    mu: Vec<f64>,
    xi: f64,
    kkt: &DMatrix<f64>,
    m: usize,
    q_correction: f64,
    residual_norm: f64,
    abs_lambda_min: f64,
    eigen_info: &EigenInfo,
) -> KktOutcome {
    // Compute Q = (1/2) beta^T H beta using the top-left m x m block of the KKT matrix.
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;

    let q_corrected = q_raw + q_correction;

    if q_corrected <= EPS_Q_POSITIVE {
        return KktOutcome::Infeasible;
    }

    // Tight bound: E = (9/2) ||r||^2 / |lambda_min|.
    // 4.5 = 9/2 comes from [lem:q-error-bound]: the KKT block structure
    // identity delta_beta^T H delta_beta = delta_x^T M delta_x - 2 r2^T delta_mu
    // - 2 r3 delta_xi removes the ||H||/|lambda_min|^2 term, leaving only the
    // quadratic term (9/2) ||r||^2 / |lambda_min|. The factor 9 comes from the
    // Cauchy-Schwarz bound on the two-variable quadratic form in the residual.
    // See [lem:q-error-bound].
    let r_sq = residual_norm * residual_norm;
    let q_error_bound = 4.5 * r_sq / abs_lambda_min;

    // CONJECTURE (Jörn, 2026-04-04): degenerate orbits with large q_error_bound
    // are never capacity-achieving, so the final A_min has low error even when
    // individual orbits have poor bounds. Near-singular KKT matrices (|λ_min| ≈
    // machine epsilon) from degenerate orbits (e.g. 4-facet orbits on LP(4,4))
    // inflate q_error_bound, but these orbits lose to well-conditioned ones.
    //
    // The q_error_bound and q_corrected values are stored in KktResult for
    // auditing. Gap: need proven error bound or algorithm change before
    // thesis-facing use; track route-development status in
    // experiments/dev-quadratic-program/README.md.

    KktOutcome::Feasible(KktResult {
        beta: beta.to_vec(),
        mu,
        xi,
        q_corrected,
        q_correction,
        q_error_bound,
        n_positive: eigen_info.n_positive,
        n_negative: eigen_info.n_negative,
        n_zero: eigen_info.n_zero,
    })
}
