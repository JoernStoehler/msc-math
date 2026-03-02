#![allow(clippy::collapsible_if, clippy::op_ref, clippy::bool_comparison, dead_code)]

/// Measure empirical Q error bounds from the KKT eigendecomposition solve.
///
/// Goal: Check whether the analytical error bound E from the analytical error bound
///       from the Q~error lemma is tight or uselessly loose, and measure direction-aware errors.
/// Input: Known polytopes from the library (small, F ≤ 8).
/// Output: Tables to stdout with |λ_min|, ‖r‖, Q values, E, actual error.
use nalgebra::{DMatrix, DVector, Vector4};
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::symplectic::omega0;
use symplectic::geom::polytope::Polytope4D;

// WARNING: This is a copy of kkt::build_kkt_system (which is pub(crate)).
// If the KKT matrix construction changes in the library, update this copy.
// Last synced: 2026-03-01 (symmetric KKT with negated multipliers).
/// Build symmetric KKT matrix and RHS (copied from crate-internal build_kkt_system).
///
/// Uses negated multipliers (μ = −λ, ξ = −ν) for a symmetric saddle-point matrix:
/// ```text
/// [ H   |  N   |  η ] [ β ]   [ 0 ]   rows 0..m
/// [ N^T |  0   |  0 ] [ μ ] = [ 0 ]   rows m..m+4
/// [ η^T |  0   |  0 ] [ ξ ]   [ 1 ]   row  m+4
/// ```
fn build_kkt(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
    let size = m + 5;
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = h;
        kkt[(m + 4, i)] = h;
    }
    rhs[m + 4] = 1.0;
    (kkt, rhs)
}

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
///
/// Sign convention: q_from_beta = -(1/2) β^T H β, where H is the KKT matrix
/// upper-left block. Since ω₀ is antisymmetric and H_{ij} = ω₀(n_{min}, n_{max}),
/// the i>j sum uses the OPPOSITE argument order from H, giving the minus sign.
///
/// Consequence for second-order conditions:
/// - Hessian of q w.r.t. β is -H (constant)
/// - H|_T PD ↔ -H|_T ND ↔ q locally MAXIMIZED (this is the desired case)
/// - H|_T ND ↔ -H|_T PD ↔ q locally MINIMIZED (can improve by moving)
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum()
}

/// Compute -(1/2) β^T H β directly from the KKT matrix, for cross-checking q_from_beta.
fn q_from_hessian(kkt: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let m = beta.len();
    let beta_vec = DVector::from_iterator(m, beta.iter().cloned());
    let h_block = kkt.view((0, 0), (m, m));
    let hb = h_block * &beta_vec;
    -0.5 * beta_vec.dot(&hb)
}

/// Condition-number threshold for rank truncation (matches EIGEN_CONDITION_TAU
/// in crates/src/kkt.rs).
/// Used as: |λ_i| > lambda_max * EIGEN_CONDITION_TAU → retain eigenvalue i.
const EIGEN_CONDITION_TAU: f64 = 1e-3;

/// Threshold for eigenvalue definiteness classification.
const EPS_DEFINITE: f64 = 1e-10;

/// Generate all combinations of `k` elements from `{0, ..., n-1}`.
fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize, k: usize, start: usize, depth: usize,
    combo: &mut Vec<usize>, result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.clone());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

/// Generate all cyclic permutations of a slice (fix first element, permute rest).
fn cyclic_permutations(s: &[usize]) -> Vec<Vec<usize>> {
    if s.len() <= 1 {
        return vec![s.to_vec()];
    }
    let mut result = Vec::new();
    let first = s[0];
    let rest: Vec<usize> = s[1..].to_vec();
    for perm in permutations_of(&rest) {
        let mut p = vec![first];
        p.extend(perm);
        result.push(p);
    }
    result
}

fn permutations_of(s: &[usize]) -> Vec<Vec<usize>> {
    if s.len() <= 1 {
        return vec![s.to_vec()];
    }
    let mut result = Vec::new();
    for i in 0..s.len() {
        let elem = s[i];
        let rest: Vec<usize> = s.iter().enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &v)| v)
            .collect();
        for mut perm in permutations_of(&rest) {
            perm.insert(0, elem);
            result.push(perm);
        }
    }
    result
}

/// Lightweight Hessian diagnostic for a single (S, σ) node.
/// Returns (Q, beta_min, definiteness, tangent_dim) or None if KKT system unsolvable.
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

    // Truncated eigendecomposition solve
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

struct Diagnostics {
    lambda_max_abs: f64,
    /// Smallest |λ_i| of full matrix
    lambda_min_abs_full: f64,
    /// Smallest |λ_i| among retained eigenvalues (after truncation)
    lambda_min_abs_retained: f64,
    numerical_rank: usize,
    full_size: usize,
    residual_norm_full: f64,
    residual_norm_truncated: f64,
    q_raw: f64,
    q_first_order: f64,
    /// E_old = ‖r‖²(2/|λ_min| + |λ_max|/(2|λ_min|²)) — old vacuous bound
    e_old_full: f64,
    /// E_old using |λ_min| of retained eigenvalues
    e_old_retained: f64,
    /// E_tight = (9/2)‖r‖²/|λ_min| — tight bound from [lem:q-error-bound]
    e_tight_full: f64,
    /// E_tight using |λ_min| of retained eigenvalues
    e_tight_retained: f64,
    /// |2(r₁ᵀδλ + r₃δν)| computed from full eigendecomposition δx
    cross_abs_full: f64,
    /// |δβᵀHδβ| computed from full eigendecomposition δx
    quad_abs_full: f64,
    /// Same but using truncated eigendecomposition
    cross_abs_trunc: f64,
    quad_abs_trunc: f64,
    q_fully_corrected: f64,
    /// Eigenvalues with signs (for inertia checks)
    eigenvalues_signed: Vec<f64>,
    /// Absolute values of eigenvalues
    eigenvalues_abs: Vec<f64>,
    /// Eigenvalues of H restricted to T = ker([N^T; η^T])
    restricted_eigs: Vec<f64>,
    /// dim T = m - rank([N^T; η^T])
    tangent_dim: usize,
    /// Minimum β̂ component (for β > 0 check)
    beta_min: f64,
}

fn run_diagnostics(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Diagnostics {
    let m = perm.len();
    let size = m + 5;
    let (kkt, rhs) = build_kkt(normals, heights, perm);

    let eig = kkt.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs).fold(0.0_f64, f64::max);
    let lambda_min_abs_full = eigenvalues.iter().cloned().map(f64::abs)
        .fold(f64::INFINITY, f64::min);

    // Truncated rank (matching library): sort by |λ|, keep top `rank`
    let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
    let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();

    // |λ_min| among retained eigenvalues (top `rank` by |λ|)
    let mut sorted_abs: Vec<f64> = eigenvalues.iter().cloned().map(f64::abs).collect();
    sorted_abs.sort_by(|a, b| b.partial_cmp(a).unwrap()); // descending
    let lambda_min_abs_retained = sorted_abs.iter().take(rank).cloned()
        .fold(f64::INFINITY, f64::min);

    // Full eigendecomposition solve (all eigenvalues)
    let x_full = eigen_solve(eigenvectors, eigenvalues, &rhs, size, size);
    let r_full = &kkt * &x_full - &rhs;
    let r_full_norm = r_full.norm();

    // Truncated eigendecomposition solve (only top `rank` eigenvalues by |λ|)
    let x_trunc = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
    let r_trunc = &kkt * &x_trunc - &rhs;
    let r_trunc_norm = r_trunc.norm();

    let beta_full: Vec<f64> = (0..m).map(|i| x_full[i]).collect();
    let q_raw = q_from_beta(normals, perm, &beta_full);

    // Residual blocks from full eigendecomposition (code ordering: rows m..m+4 = constraint, m+4 = normalization)
    // Solution vector is [β̂; μ̂; ξ̂] with negated multipliers.
    let r1_full: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| r_full[i]));
    let r3_full = r_full[m + 4];
    let mu_hat: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| x_full[i]));
    let xi_hat = x_full[m + 4];

    // Q̃ = Q(β̂) - (r₂ᵀμ̂ + r₃ξ̂)  [Lemma [lem:q-interval], Step 2-3]
    let q_first_order = q_raw - (r1_full.dot(&mu_hat) + r3_full * xi_hat);

    // Analytical bounds (old vacuous bound and tight bound)
    let r_sq_full = r_full_norm * r_full_norm;
    let lmin_full = lambda_min_abs_full.max(f64::MIN_POSITIVE);
    let e_old_full = r_sq_full * (2.0 / lmin_full + lambda_max_abs / (2.0 * lmin_full * lmin_full));
    let e_tight_full = 4.5 * r_sq_full / lmin_full;
    let r_sq_trunc = r_trunc_norm * r_trunc_norm;
    let e_old_retained = r_sq_trunc * (2.0 / lambda_min_abs_retained
        + lambda_max_abs / (2.0 * lambda_min_abs_retained * lambda_min_abs_retained));
    let e_tight_retained = 4.5 * r_sq_trunc / lambda_min_abs_retained;

    // Direction-aware: δx = Q Λ⁻¹ Qᵀ r (using full eigendecomposition)
    let delta_x_full = eigen_solve(eigenvectors, eigenvalues, &r_full, size, size);
    let (cross_full, quad_full) = compute_remainder(normals, perm, m, &r_full, &delta_x_full);

    // Direction-aware: δx using truncated eigendecomposition
    let delta_x_trunc = eigen_solve(eigenvectors, eigenvalues, &r_trunc, size, rank);
    let (cross_trunc, quad_trunc) = compute_remainder(normals, perm, m, &r_trunc, &delta_x_trunc);

    // cross_new = r₁ᵀδμ + r₃δξ = -(r₁ᵀδλ + r₃δν), so sign flips vs old convention.
    let q_fully_corrected = q_first_order - 2.0 * cross_full - quad_full;

    // Restricted Hessian: H|_T where T = ker([N^T; η^T])
    // Build constraint matrix C (5 × m): rows = N^T rows + η^T row
    let mut constraint = DMatrix::zeros(5, m);
    for i in 0..m {
        for d in 0..4 {
            constraint[(d, i)] = normals[perm[i]][d];
        }
        constraint[(4, i)] = heights[perm[i]];
    }
    // Null space of C via eigendecomposition of C^T C (m × m, symmetric PSD).
    // Eigenvectors with eigenvalue ≈ 0 span ker(C) = T.
    let ctc = constraint.transpose() * &constraint; // m × m
    let ctc_eig = ctc.symmetric_eigen();
    // Threshold for "zero" eigenvalue
    let ctc_max = ctc_eig.eigenvalues.iter().cloned().fold(0.0_f64, f64::max);
    let ctc_threshold = ctc_max * 1e-10;
    // Collect null-space eigenvector indices (eigenvalue < threshold)
    let null_indices: Vec<usize> = (0..m)
        .filter(|&i| ctc_eig.eigenvalues[i] < ctc_threshold)
        .collect();
    let tangent_dim = null_indices.len();

    let restricted_eigs = if tangent_dim > 0 {
        // P = null space basis (m × tangent_dim)
        let mut p = DMatrix::zeros(m, tangent_dim);
        for (k, &idx) in null_indices.iter().enumerate() {
            for i in 0..m {
                p[(i, k)] = ctc_eig.eigenvectors[(i, idx)];
            }
        }
        // Extract H block (top-left m×m of KKT matrix)
        let h_block = kkt.view((0, 0), (m, m)).clone_owned();
        // H_restricted = P^T H P
        let h_restricted = p.transpose() * &h_block * &p;
        // Eigenvalues of symmetric matrix
        let eig = h_restricted.symmetric_eigen();
        let mut eigs: Vec<f64> = eig.eigenvalues.iter().cloned().collect();
        eigs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        eigs
    } else {
        vec![]
    };

    // Use truncated eigendecomposition beta for β > 0 check (full solve garbage on singular systems)
    let beta_trunc: Vec<f64> = (0..m).map(|i| x_trunc[i]).collect();
    let beta_min = beta_trunc.iter().cloned().fold(f64::INFINITY, f64::min);

    Diagnostics {
        lambda_max_abs,
        lambda_min_abs_full,
        lambda_min_abs_retained,
        numerical_rank: rank,
        full_size: size,
        residual_norm_full: r_full_norm,
        residual_norm_truncated: r_trunc_norm,
        q_raw,
        q_first_order,
        e_old_full,
        e_old_retained,
        e_tight_full,
        e_tight_retained,
        cross_abs_full: (2.0 * cross_full).abs(),
        quad_abs_full: quad_full.abs(),
        cross_abs_trunc: (2.0 * cross_trunc).abs(),
        quad_abs_trunc: quad_trunc.abs(),
        q_fully_corrected,
        eigenvalues_signed: eigenvalues.iter().cloned().collect(),
        eigenvalues_abs: eigenvalues.iter().cloned().map(f64::abs).collect(),
        restricted_eigs,
        tangent_dim,
        beta_min,
    }
}

fn eigen_solve(
    eigenvectors: &DMatrix<f64>,
    eigenvalues: &DVector<f64>,
    rhs: &DVector<f64>,
    size: usize,
    rank: usize,
) -> DVector<f64> {
    // Sort eigenvalues by descending |λ|, use top `rank`
    let mut indices: Vec<usize> = (0..size).collect();
    indices.sort_by(|&a, &b| eigenvalues[b].abs().partial_cmp(&eigenvalues[a].abs()).unwrap());

    let mut x = DVector::zeros(size);
    for &i in indices.iter().take(rank) {
        let coeff = eigenvectors.column(i).dot(rhs) / eigenvalues[i];
        x += coeff * eigenvectors.column(i);
    }
    x
}

/// Compute cross remainder r₁ᵀδλ + r₃δν and quadratic δβᵀHδβ.
fn compute_remainder(
    normals: &[Vector4<f64>],
    perm: &[usize],
    m: usize,
    residual: &DVector<f64>,
    delta_x: &DVector<f64>,
) -> (f64, f64) {
    let r1: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| residual[i]));
    let r3 = residual[m + 4];
    let delta_lambda: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| delta_x[i]));
    let delta_nu = delta_x[m + 4];
    let delta_beta: Vec<f64> = (0..m).map(|i| delta_x[i]).collect();

    let cross = r1.dot(&delta_lambda) + r3 * delta_nu;
    let quad = q_from_beta(normals, perm, &delta_beta);
    (cross, quad)
}

fn main() {
    // Small polytopes only (F ≤ 8 for fast ehz_capacity)
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", known_polytopes::simplex().polytope),
        ("hypercube", known_polytopes::hypercube().polytope),
        ("LP_tri_tri", known_polytopes::lagrangian_triangle_product().polytope),
        ("LP_tri_sq", known_polytopes::lagrangian_triangle_square().polytope),
        ("SP_tri_tri", known_polytopes::symplectic_triangle_product().polytope),
    ];

    println!("=== Q Error Bound Diagnostic ===\n");

    // Table 0: Verify sign convention q_from_beta = -(1/2) β^T H β
    println!("--- Table 0: Sign convention verification ---");
    println!("  q_from_beta computes Σ_{{i>j}} β_i β_j ω₀(n_i, n_j)");
    println!("  H_{{ij}} = ω₀(n_{{min}}, n_{{max}}) (symmetric)");
    println!("  Since ω₀ is antisymmetric: q_from_beta = -(1/2) β^T H β");
    println!();
    println!("{:<14} {:>16} {:>16} {:>12}", "polytope", "q_from_beta", "-(1/2)β^T·H·β", "rel_diff");
    println!("{}", "-".repeat(60));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };
        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();
        let m = alg_perm.len();
        let (kkt, rhs) = build_kkt(polytope.normals(), polytope.heights(), &alg_perm);
        let eig = kkt.clone().symmetric_eigen();
        let eigenvalues = &eig.eigenvalues;
        let eigenvectors = &eig.eigenvectors;
        let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs).fold(0.0_f64, f64::max);
        let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
        let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();
        let size = m + 5;
        let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
        let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();

        let q1 = q_from_beta(polytope.normals(), &alg_perm, &beta);
        let q2 = q_from_hessian(&kkt, &beta);
        let rel = if q1.abs() > 1e-15 { (q1 - q2).abs() / q1.abs() } else { (q1 - q2).abs() };
        println!("{:<14} {:>16.12} {:>16.12} {:>12.3e}", name, q1, q2, rel);
    }
    println!();

    // Table 1: Condition numbers and rank
    println!("--- Table 1: Eigendecomposition condition and rank ---");
    println!("{:<14} {:>3} {:>5} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "polytope", "m", "rank", "|λ_min|_full", "|λ_min|_ret", "κ_full", "‖r‖_full", "‖r‖_trunc");
    println!("{}", "-".repeat(88));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => {
                println!("{:<14} no capacity", name);
                continue;
            }
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();
        let m = alg_perm.len();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        println!("{:<14} {:>3} {:>2}/{:>2} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e}",
            name, m, d.numerical_rank, d.full_size,
            d.lambda_min_abs_full, d.lambda_min_abs_retained,
            d.lambda_max_abs / d.lambda_min_abs_full.max(f64::MIN_POSITIVE),
            d.residual_norm_full, d.residual_norm_truncated);
    }

    println!();

    // Table 2: Error bounds — old vs tight vs actual
    println!("--- Table 2: Old E vs tight E vs actual |R| (direction-aware, retained eigenvalues) ---");
    println!("{:<14} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "polytope", "E_old", "E_tight", "|R|_actual", "old/actual", "tight/actual");
    println!("{}", "-".repeat(74));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        let r_actual = d.cross_abs_trunc + d.quad_abs_trunc;
        let loose_old = if r_actual > 0.0 { d.e_old_retained / r_actual } else { f64::INFINITY };
        let loose_tight = if r_actual > 0.0 { d.e_tight_retained / r_actual } else { f64::INFINITY };

        println!("{:<14} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e}",
            name,
            d.e_old_retained, d.e_tight_retained, r_actual, loose_old, loose_tight);
    }

    println!();

    // Table 3: Q values
    println!("--- Table 3: Q corrections (how much does each correction level change Q?) ---");
    println!("{:<14} {:>16} {:>16} {:>16} {:>12}",
        "polytope", "Q_raw", "Q̃_1st_order", "Q̃_full_corr", "capacity");
    println!("{}", "-".repeat(74));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        println!("{:<14} {:>16.12} {:>16.12} {:>16.12} {:>12.8}",
            name,
            d.q_raw, d.q_first_order, d.q_fully_corrected,
            result.capacity);
    }

    println!();

    // Table 4: Eigenvalue spectra
    println!("--- Table 4: Eigenvalue spectra ---");
    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        // Show signed eigenvalues (sorted descending by absolute value) and absolute values
        let mut signed_sorted = d.eigenvalues_signed.clone();
        signed_sorted.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
        println!("{} (signed): [{}]", name,
            signed_sorted.iter()
                .map(|e| format!("{:+.3e}", e))
                .collect::<Vec<_>>()
                .join(", "));
        let mut abs_sorted = d.eigenvalues_abs.clone();
        abs_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        println!("{} (|λ|):    [{}]", name,
            abs_sorted.iter()
                .map(|e| format!("{:.3e}", e))
                .collect::<Vec<_>>()
                .join(", "));
    }

    println!();

    // Table 5: Restricted Hessian definiteness
    println!("--- Table 5: Restricted Hessian H|_T definiteness ---");
    println!("{:<14} {:>3} {:>6} {:>8} {:>12} {:>12} {:>12} {:>10}",
        "polytope", "m", "dim_T", "β_min", "λ_min(H|T)", "λ_max(H|T)", "definite?", "β>0?");
    println!("{}", "-".repeat(88));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        if d.restricted_eigs.is_empty() {
            println!("{:<14} {:>3} {:>6} {:>8.3e} {:>12} {:>12} {:>12} {:>10}",
                name, alg_perm.len(), d.tangent_dim, d.beta_min,
                "n/a", "n/a", "trivial", if d.beta_min > 0.0 { "yes" } else { "no" });
        } else {
            let lam_min = d.restricted_eigs.first().unwrap();
            let lam_max = d.restricted_eigs.last().unwrap();
            let eps = 1e-10;
            let definiteness = if *lam_min > eps {
                "PD"      // H|_T PD → Hess(q) = -H|_T is ND → q is MAXIMIZED (local max)
            } else if *lam_max < -eps {
                "ND"      // H|_T ND → Hess(q) = -H|_T is PD → q is MINIMIZED (not a max)
            } else if *lam_min < -eps && *lam_max > eps {
                "indef"   // saddle point of q — some directions increase, some decrease
            } else {
                "~zero"   // near-degenerate — numerically undecided
            };

            println!("{:<14} {:>3} {:>6} {:>8.3e} {:>12.3e} {:>12.3e} {:>12} {:>10}",
                name, alg_perm.len(), d.tangent_dim, d.beta_min,
                lam_min, lam_max, definiteness,
                if d.beta_min > 0.0 { "yes" } else { "no" });

            // Print full eigenvalue spectrum for insight
            println!("  eigenvalues: [{}]",
                d.restricted_eigs.iter()
                    .map(|e| format!("{:.3e}", e))
                    .collect::<Vec<_>>()
                    .join(", "));
        }
    }

    println!();

    // Table 6: Hessian definiteness across ALL (S, σ) nodes
    println!("--- Table 6: Restricted Hessian across ALL evaluated (S,σ) nodes ---");
    println!("{:<14} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "F", "total", "β>0", "Q>0", "triv", "PD", "ND", "indef", "~zero");
    println!("{}", "-".repeat(88));

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

        // Track PD+β>0 nodes: admissible local maxima of q (the "keeper" nodes)
        let mut pd_beta_pos: Vec<(usize, f64)> = Vec::new(); // (m, Q)

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
                        // Only classify definiteness for β>0, Q>0 nodes
                        if info.beta_min > eps_beta && info.q > eps_q {
                            match info.definiteness {
                                Definiteness::Trivial => n_trivial += 1,
                                Definiteness::PD => {
                                    n_pd += 1;
                                    pd_beta_pos.push((info.m, info.q));
                                }
                                Definiteness::ND => n_nd += 1,
                                Definiteness::Indefinite => n_indef += 1,
                                Definiteness::NearZero => n_nearzero += 1,
                            }
                        }
                    }
                }
            }
        }

        println!("{:<14} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
            name, f, total, n_beta_pos, n_q_pos,
            n_trivial, n_pd, n_nd, n_indef, n_nearzero);

        // Collect ALL node info for lattice checks (keyed by subset as BTreeSet)
        use std::collections::{BTreeSet, BTreeMap};
        struct SubsetInfo {
            has_neg_dir: bool,  // ND or indefinite (any negative eigenvalue)
            is_pd: bool,        // all-positive (PD)
            max_q: f64,         // max Q across all σ for this subset
        }
        let mut subset_info: BTreeMap<BTreeSet<usize>, SubsetInfo> = BTreeMap::new();

        for m_check in 2..=f {
            for subset in combinations(f, m_check) {
                let key: BTreeSet<usize> = subset.iter().cloned().collect();
                let mut has_neg = false;
                let mut all_pd = true;
                let mut any_nontrivial = false;
                let mut max_q = f64::NEG_INFINITY;

                for perm in cyclic_permutations(&subset) {
                    if let Some(info) = node_hessian_check(normals, heights, &perm) {
                        if info.beta_min > eps_beta && info.q > eps_q {
                            max_q = max_q.max(info.q);
                            match info.definiteness {
                                Definiteness::ND => { has_neg = true; all_pd = false; any_nontrivial = true; }
                                Definiteness::Indefinite => { has_neg = true; all_pd = false; any_nontrivial = true; }
                                Definiteness::PD => { any_nontrivial = true; }
                                Definiteness::NearZero => { all_pd = false; any_nontrivial = true; }
                                Definiteness::Trivial => { /* doesn't affect PD/ND classification */ }
                            }
                        }
                    }
                }

                if max_q > f64::NEG_INFINITY {
                    subset_info.insert(key, SubsetInfo {
                        has_neg_dir: has_neg,
                        is_pd: all_pd && any_nontrivial,
                        max_q,
                    });
                }
            }
        }

        // Check 1: Upward monotonicity — negative direction propagates to supersets
        // If S has negative direction, all S' ⊃ S with β>0,Q>0 should also have neg dir
        let mut up_violations = 0u32;
        let neg_subsets: Vec<&BTreeSet<usize>> = subset_info.iter()
            .filter(|(_, info)| info.has_neg_dir)
            .map(|(k, _)| k)
            .collect();
        for neg_set in &neg_subsets {
            for (sup_set, sup_info) in &subset_info {
                if sup_set.len() > neg_set.len() && neg_set.is_subset(sup_set) {
                    if sup_info.is_pd {
                        up_violations += 1;
                        println!("  UP-VIOLATION: neg-dir set {:?} has PD superset {:?}",
                            neg_set, sup_set);
                    }
                }
            }
        }
        if !neg_subsets.is_empty() {
            if up_violations == 0 {
                println!("  Upward (neg→supersets): CONFIRMED ({} neg-dir sets, no PD supersets)",
                    neg_subsets.len());
            }
        }

        // Check 2a: Downward (PD→subsets): PD node's Q ≥ all subsets' Q?
        // Rationale: PD means q is locally maximized. If also β>0 (admissible),
        // the critical point IS the constrained max for this face set S.
        // But the max for S includes boundary points (child nodes with some β_i=0).
        // So the PD interior max should dominate all child maxima.
        //
        // NOTE: this check compares across different permutations σ, which live
        // on different face sets with different H matrices. The monotonicity
        // argument only applies within a FIXED permutation ordering (same H).
        // Cross-σ violations may therefore not be real violations.
        let mut down_pd_violations = 0u32;
        let pd_sets: Vec<(&BTreeSet<usize>, &SubsetInfo)> = subset_info.iter()
            .filter(|(_, info)| info.is_pd)
            .collect();
        for (pd_set, pd_info) in &pd_sets {
            for (sub_set, sub_info) in &subset_info {
                if sub_set.len() < pd_set.len() && sub_set.is_subset(pd_set) {
                    if sub_info.max_q > pd_info.max_q + 1e-12 {
                        down_pd_violations += 1;
                        println!("  DOWN-PD-VIOLATION: PD {:?} Q={:.6e} < subset {:?} Q={:.6e}",
                            pd_set, pd_info.max_q, sub_set, sub_info.max_q);
                    }
                }
            }
        }
        if !pd_sets.is_empty() {
            if down_pd_violations == 0 {
                println!("  Downward (PD→subsets): CONFIRMED ({} PD sets, Q ≥ all child Q)", pd_sets.len());
            } else {
                println!("  Downward (PD→subsets): {} violations (may be cross-σ artifacts)",
                    down_pd_violations);
            }
        }

        // Check 2b: Downward (ND→subsets): ND node's Q ≥ all subsets' Q?
        let mut down_nd_violations = 0u32;
        let _nd_sets: Vec<(&BTreeSet<usize>, &SubsetInfo)> = subset_info.iter()
            .filter(|(_, info)| info.has_neg_dir && !info.is_pd) // ND or indefinite
            .collect();
        // More precisely, check nodes where ALL nontrivial perms are ND
        let pure_nd_sets: Vec<(&BTreeSet<usize>, &SubsetInfo)> = subset_info.iter()
            .filter(|(_, info)| info.has_neg_dir && info.is_pd == false)
            .collect();
        for (nd_set, nd_info) in &pure_nd_sets {
            for (sub_set, sub_info) in &subset_info {
                if sub_set.len() < nd_set.len() && sub_set.is_subset(nd_set) {
                    if sub_info.max_q > nd_info.max_q + 1e-12 {
                        down_nd_violations += 1;
                        if down_nd_violations <= 5 {
                            println!("  DOWN-ND-VIOLATION: neg-dir {:?} Q={:.6e} < subset {:?} Q={:.6e}",
                                nd_set, nd_info.max_q, sub_set, sub_info.max_q);
                        }
                    }
                }
            }
        }
        if !pure_nd_sets.is_empty() {
            if down_nd_violations == 0 {
                println!("  Downward (ND→subsets): CONFIRMED ({} neg-dir sets, all subsets Q ≤ parent Q)",
                    pure_nd_sets.len());
            } else {
                println!("  Downward (ND→subsets): FAILED ({} violations)", down_nd_violations);
            }
        }

        // Show PD+β>0 nodes detail
        if !pd_beta_pos.is_empty() {
            let q_max = pd_beta_pos.iter().map(|&(_, q)| q).fold(f64::NEG_INFINITY, f64::max);
            let q_min = pd_beta_pos.iter().map(|&(_, q)| q).fold(f64::INFINITY, f64::min);
            let by_m: std::collections::BTreeMap<usize, usize> = pd_beta_pos.iter()
                .fold(std::collections::BTreeMap::new(), |mut acc, &(m, _)| {
                    *acc.entry(m).or_insert(0) += 1;
                    acc
                });
            let m_dist: String = by_m.iter()
                .map(|(m, c)| format!("m={}:{}", m, c))
                .collect::<Vec<_>>()
                .join(" ");
            println!("  PD+β>0: {} nodes, Q∈[{:.6e}, {:.6e}], by size: {}",
                pd_beta_pos.len(), q_min, q_max, m_dist);
        }
    }

    println!();

    // Table 7: Q interval framework — per-node classification and majorization
    //
    // Note: This table prototypes interval majorization Q̃ ± E.
    // After investigation, point comparison on Q̃ was adopted instead
    // (E < 1e-13 across all test polytopes, making intervals unnecessary).
    // Table kept for documentation of the exploratory analysis.
    //
    // For each (S, σ):
    //   eigen → β̂, residual r, |λ_min|(retained), Q̃, E
    //   δβ_bound = ‖r‖ / |λ_min|  (perturbation bound on β)
    //   Admissibility:
    //     decided-admissible:   β̂_min > δβ_bound  (true β_i > 0 for all i)
    //     decided-inadmissible: β̂_min < -δβ_bound (true β_i < 0 for some i)
    //     uncertain:            |β̂_min| ≤ δβ_bound
    //   Q interval:
    //     decided-admissible:   [Q̃ - E, Q̃ + E]
    //     decided-inadmissible: skip (sub-faces handle this)
    //     uncertain:            [-∞, Q̃ + E]  (might not be admissible)
    //
    // Aggregation: Q*_low = max of decided-admissible (Q̃ - E) values.
    // Majorize: node with Q̃ + E < Q*_low is dominated (even if uncertain).
    // Remaining uncertain nodes need rational admissibility check.
    println!("--- Table 7: Q interval framework and majorization ---");
    println!("  Per (S,σ): eigen → Q̃ ± E, β̂_min vs δβ_bound = ‖r‖/|λ_min|");
    println!("  Admissible: [Q̃-E, Q̃+E].  Uncertain: [-∞, Q̃+E].  Inadmissible: skip.");
    println!("  Majorize: Q̃+E < best admissible (Q̃-E) → pruned.");
    println!();
    println!("{:<14} {:>6} {:>6} {:>6} {:>6} {:>6} {:>12} {:>12} {:>6}",
        "polytope", "total", "admis", "inadm", "uncrt", "Q>0", "Q*_low", "Q*_high", "surv");
    println!("{}", "-".repeat(88));

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let normals = polytope.normals();
        let heights = polytope.heights();

        let mut total = 0u64;
        let mut n_admissible = 0u64;
        let mut n_inadmissible = 0u64;
        let mut n_uncertain = 0u64;

        // Best known admissible Q lower bound
        let mut best_admissible_q_low: f64 = f64::NEG_INFINITY;
        let mut best_admissible_q_high: f64 = f64::NEG_INFINITY;

        // All nodes' Q_high values (for majorization check)
        struct NodeInterval {
            q_low: f64,    // -∞ for uncertain
            q_high: f64,
            admissibility: u8, // 0=admissible, 1=inadmissible, 2=uncertain
            q_positive: bool,
        }
        let mut all_nodes: Vec<NodeInterval> = Vec::new();

        for m in 2..=f {
            for subset in combinations(f, m) {
                for perm in cyclic_permutations(&subset) {
                    total += 1;
                    let size = m + 5;
                    let (kkt, rhs) = build_kkt(normals, heights, &perm);

                    let eig = kkt.clone().symmetric_eigen();
                    let eigenvalues = &eig.eigenvalues;
                    let eigenvectors = &eig.eigenvectors;

                    let lambda_max_abs = eigenvalues.iter().cloned().map(f64::abs)
                        .fold(0.0_f64, f64::max);
                    let threshold = lambda_max_abs * EIGEN_CONDITION_TAU;
                    let rank = eigenvalues.iter().filter(|&&e| e.abs() > threshold).count();

                    // |λ_min| among retained eigenvalues (top `rank` by |λ|)
                    let mut sorted_abs: Vec<f64> = eigenvalues.iter().cloned().map(f64::abs).collect();
                    sorted_abs.sort_by(|a, b| b.partial_cmp(a).unwrap());
                    let lambda_min_abs_ret = sorted_abs.iter().take(rank).cloned()
                        .fold(f64::INFINITY, f64::min).max(f64::MIN_POSITIVE);

                    let x = eigen_solve(eigenvectors, eigenvalues, &rhs, size, rank);
                    let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);

                    let r = &kkt * &x - &rhs;
                    let r_norm = r.norm();

                    let q_raw = q_from_beta(normals, &perm, &beta);

                    // First-order Q correction [lem:q-interval]
                    let r2: DVector<f64> = DVector::from_iterator(4, (m..m+4).map(|i| r[i]));
                    let r3 = r[m + 4];
                    let mu_hat: DVector<f64> = DVector::from_iterator(4, (m..m+4).map(|i| x[i]));
                    let xi_hat = x[m + 4];
                    let q_tilde = q_raw - (r2.dot(&mu_hat) + r3 * xi_hat);

                    // Error bound E (tight bound: [lem:q-error-bound])
                    let r_sq = r_norm * r_norm;
                    let e = 4.5 * r_sq / lambda_min_abs_ret;

                    // Admissibility: β̂_min vs δβ_bound = ‖r‖ / |λ_min|
                    let delta_beta_bound = r_norm / lambda_min_abs_ret;

                    let (admissibility, q_low, q_high) = if beta_min > delta_beta_bound {
                        // Decided admissible
                        n_admissible += 1;
                        (0u8, q_tilde - e, q_tilde + e)
                    } else if beta_min < -delta_beta_bound {
                        // Decided inadmissible
                        n_inadmissible += 1;
                        (1u8, f64::NEG_INFINITY, q_tilde + e)
                    } else {
                        // Uncertain
                        n_uncertain += 1;
                        (2u8, f64::NEG_INFINITY, q_tilde + e)
                    };

                    let q_positive = q_tilde > 1e-12;

                    if admissibility == 0 && q_positive {
                        if q_low > best_admissible_q_low {
                            best_admissible_q_low = q_low;
                        }
                        if q_high > best_admissible_q_high {
                            best_admissible_q_high = q_high;
                        }
                    }

                    all_nodes.push(NodeInterval { q_low, q_high, admissibility, q_positive });
                }
            }
        }

        // Count surviving uncertain nodes (Q_high ≥ best_admissible_q_low, Q > 0)
        let n_q_positive: u64 = all_nodes.iter().filter(|n| n.q_positive).count() as u64;
        let surviving_uncertain: u64 = all_nodes.iter()
            .filter(|n| n.admissibility == 2 && n.q_positive
                && n.q_high >= best_admissible_q_low)
            .count() as u64;

        println!("{:<14} {:>6} {:>6} {:>6} {:>6} {:>6} {:>12.6e} {:>12.6e} {:>6}",
            name, total, n_admissible, n_inadmissible, n_uncertain, n_q_positive,
            best_admissible_q_low, best_admissible_q_high, surviving_uncertain);

        // Show detail for uncertain survivors
        if surviving_uncertain > 0 && surviving_uncertain <= 10 {
            for (i, node) in all_nodes.iter().enumerate() {
                if node.admissibility == 2 && node.q_positive
                    && node.q_high >= best_admissible_q_low {
                    println!("  uncertain survivor #{}: Q_high={:.6e} vs Q*_low={:.6e}",
                        i, node.q_high, best_admissible_q_low);
                }
            }
        }
    }

    println!();

    // Table 8: Inertia theorem validation
    // Eigenvalues of M vs restricted Hessian H|_T
    // The saddle-point inertia theorem says:
    //   n_+(M) = n_+(H|_T), n_-(M) = n_-(H|_T) + 5, n_0(M) = n_0(H|_T)
    // We validate this on the BEST (S, σ) for each polytope.
    println!("--- Table 8: Inertia theorem validation ([lem:kkt-inertia]) ---");
    println!("  For the optimal (S, σ), compare eigenvalue counts of M vs H|_T.");
    println!("  Inertia theorem: n+(M) = n+(H|T)+p, n-(M) = n-(H|T)+p, n0(M) = n0(H|T)+(5-p)");
    println!("  where p = rank(A), A = [N^T; η^T]");
    println!("{:<14} {:>3} {:>3} {:>2} {:>5} {:>5} {:>5}  {:>5} {:>5} {:>5}  {:>5}",
        "polytope", "m", "dimT", "p", "n+(M)", "n0(M)", "n-(M)",
        "n+(HT)", "n0(HT)", "n-(HT)", "pass?");
    println!("{}", "-".repeat(84));

    let eig_eps = 1e-10;

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();
        let m = alg_perm.len();
        let size = m + 5;

        let (kkt, _rhs) = build_kkt(polytope.normals(), polytope.heights(), &alg_perm);

        // Eigenvalues of M (symmetric eigendecomposition)
        let eig_m = kkt.symmetric_eigen();
        let n_pos_m = eig_m.eigenvalues.iter().filter(|&&e| e > eig_eps).count();
        let n_neg_m = eig_m.eigenvalues.iter().filter(|&&e| e < -eig_eps).count();
        let n_zero_m = size - n_pos_m - n_neg_m;

        // Restricted Hessian H|_T (from the existing approach)
        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);
        let tangent_dim = d.tangent_dim;
        let p = m - tangent_dim; // rank(A) = m - dim(ker(A))

        let (n_pos_ht, n_neg_ht, n_zero_ht) = if d.restricted_eigs.is_empty() {
            (0, 0, 0)
        } else {
            let np = d.restricted_eigs.iter().filter(|&&e| e > eig_eps).count();
            let nn = d.restricted_eigs.iter().filter(|&&e| e < -eig_eps).count();
            let nz = tangent_dim - np - nn;
            (np, nn, nz)
        };

        // Corrected inertia theorem: n+(M) = n+(H|T) + p, n-(M) = n-(H|T) + p,
        // n0(M) = n0(H|T) + (5 - p)
        let pass = n_pos_m == n_pos_ht + p
            && n_neg_m == n_neg_ht + p
            && n_zero_m == n_zero_ht + (5 - p);

        println!("{:<14} {:>3} {:>3} {:>2} {:>5} {:>5} {:>5}  {:>5} {:>5} {:>5}  {:>5}",
            name, m, tangent_dim, p,
            n_pos_m, n_zero_m, n_neg_m,
            n_pos_ht, n_zero_ht, n_neg_ht,
            if pass { "✓" } else { "FAIL" });

        // Print eigenvalue spectra for comparison
        let mut eigs_m: Vec<f64> = eig_m.eigenvalues.iter().cloned().collect();
        eigs_m.sort_by(|a, b| b.partial_cmp(a).unwrap()); // descending
        println!("  M eigenvalues:  [{}]",
            eigs_m.iter().map(|e| format!("{:+.3e}", e)).collect::<Vec<_>>().join(", "));
        if !d.restricted_eigs.is_empty() {
            let mut eigs_ht = d.restricted_eigs.clone();
            eigs_ht.sort_by(|a, b| b.partial_cmp(a).unwrap());
            println!("  H|T eigenvalues: [{}]",
                eigs_ht.iter().map(|e| format!("{:+.3e}", e)).collect::<Vec<_>>().join(", "));
        }
    }

    // Table 9: Inertia across ALL (S, σ) nodes — check n-(M) = p ↔ H|_T non-negative definite
    println!();
    println!("--- Table 9: Inertia check across all (S,σ) nodes ---");
    println!("  For each node: compute eigenvalues of M, check n-(M)=p matches H|_T PD/NSD");
    println!("  p = rank(A) = m - dim_T. H|_T PD ↔ n-(M)=p and n0(M)=5-p.");
    println!("{:<14} {:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "total", "n-=p", "n->p", "PD", "ND", "indef", "match?");
    println!("{}", "-".repeat(74));

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let mut total = 0u64;
        let mut n_negp = 0u64;    // n-(M) = p (predicts H|_T NSD)
        let mut n_neggtp = 0u64;  // n-(M) > p (predicts H|_T has negative eigenvalue)
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

                    // Eigenvalues of M
                    let eig = kkt_mat.symmetric_eigen();
                    let n_neg = eig.eigenvalues.iter().filter(|&&e| e < -eig_eps).count();
                    let n_zero = eig.eigenvalues.iter().filter(|&&e| e.abs() <= eig_eps).count();

                    // Restricted Hessian approach
                    let info = node_hessian_check(polytope.normals(), polytope.heights(), &perm);
                    let (def, tangent_dim) = match info {
                        Some(n) => (n.definiteness, n.tangent_dim),
                        None => (Definiteness::Trivial, 0),
                    };
                    let p = mm - tangent_dim; // rank(A)

                    // H|_T PD ↔ n-(M)=p and n0(M)=5-p (no near-zero in H|_T)
                    let inertia_says_pd = n_neg == p && n_zero == (5 - p);
                    let inertia_says_nsd = n_neg == p; // n-(H|T) = 0
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
                        Definiteness::NearZero | Definiteness::Trivial => {
                            // Skip comparison for degenerate cases
                        },
                    }
                }
            }
        }

        let ok = mismatches == 0;
        println!("{:<14} {:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
            name, total, n_negp, n_neggtp, n_pd, n_nd, n_indef,
            if ok { "✓".to_string() } else { format!("{} FAIL", mismatches) });
    }
}
