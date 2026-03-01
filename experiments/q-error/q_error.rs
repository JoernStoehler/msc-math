/// Measure empirical Q error bounds from the KKT SVD solve.
///
/// Goal: Check whether the analytical error bound E from Lemma B.5
///       is tight or uselessly loose, and measure direction-aware errors.
/// Input: Known polytopes from the library (small, F ≤ 8).
/// Output: Tables to stdout with σ_min, ‖r‖, Q values, E, actual error.
use nalgebra::{DMatrix, DVector, Vector4};
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::symplectic::omega0;
use symplectic::geom::polytope::Polytope4D;

/// Build KKT matrix and RHS (copied from crate-internal build_kkt_system).
///
/// Block structure (code ordering):
/// ```text
/// [ H    | -N   | -η ] [ β ]   [ 0 ]   rows 0..m
/// [ N^T  |  0   |  0 ] [ λ ] = [ 0 ]   rows m..m+4
/// [ η^T  |  0   |  0 ] [ ν ]   [ 1 ]   row  m+4
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
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
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
    let hb = &h_block * &beta_vec;
    -0.5 * beta_vec.dot(&hb)
}

/// SVD_CONDITION_TAU from the library (threshold for rank truncation).
const SVD_CONDITION_TAU: f64 = 1e-3;

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

    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let u = svd.u.as_ref().unwrap();
    let v_t = svd.v_t.as_ref().unwrap();

    let sigma_max = sv[0];
    let threshold = sigma_max * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();

    // Truncated SVD solve
    let x = svd_solve(u, sv, v_t, &rhs, size, rank);
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
    sigma_max: f64,
    /// Smallest SV of full matrix
    sigma_min_full: f64,
    /// Smallest RETAINED SV (after truncation)
    sigma_min_retained: f64,
    numerical_rank: usize,
    full_size: usize,
    residual_norm_full: f64,
    residual_norm_truncated: f64,
    q_raw: f64,
    q_first_order: f64,
    /// E using σ_min of full matrix (current code)
    e_full: f64,
    /// E using σ_min of retained SVs
    e_retained: f64,
    /// |2(r₁ᵀδλ + r₃δν)| computed from full SVD δx
    cross_abs_full: f64,
    /// |δβᵀHδβ| computed from full SVD δx
    quad_abs_full: f64,
    /// Same but using truncated SVD
    cross_abs_trunc: f64,
    quad_abs_trunc: f64,
    q_fully_corrected: f64,
    singular_values: Vec<f64>,
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

    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let u = svd.u.as_ref().unwrap();
    let v_t = svd.v_t.as_ref().unwrap();

    let sigma_max = sv[0];
    let sigma_min_full = sv.iter().cloned().fold(f64::INFINITY, f64::min);

    // Truncated rank (matching library)
    let threshold = sigma_max * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();
    let sigma_min_retained = sv.iter().take(rank).cloned().fold(f64::INFINITY, f64::min);

    // Full SVD solve (all SVs)
    let x_full = svd_solve(u, sv, v_t, &rhs, size, size);
    let r_full = &kkt * &x_full - &rhs;
    let r_full_norm = r_full.norm();

    // Truncated SVD solve (only top `rank` SVs)
    let x_trunc = svd_solve(u, sv, v_t, &rhs, size, rank);
    let r_trunc = &kkt * &x_trunc - &rhs;
    let r_trunc_norm = r_trunc.norm();

    let beta_full: Vec<f64> = (0..m).map(|i| x_full[i]).collect();
    let q_raw = q_from_beta(normals, perm, &beta_full);

    // Residual blocks from full SVD (code ordering: rows m..m+4 = constraint, m+4 = normalization)
    let r1_full: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| r_full[i]));
    let r3_full = r_full[m + 4];
    let lambda_hat: DVector<f64> = DVector::from_iterator(4, (m..m + 4).map(|i| x_full[i]));
    let nu_hat = x_full[m + 4];

    let q_first_order = q_raw - 2.0 * (r1_full.dot(&lambda_hat) + r3_full * nu_hat);

    // Analytical bounds
    let r_sq_full = r_full_norm * r_full_norm;
    let e_full = r_sq_full * (4.0 / sigma_min_full.max(f64::MIN_POSITIVE)
        + sigma_max / (sigma_min_full.max(f64::MIN_POSITIVE).powi(2)));
    let r_sq_trunc = r_trunc_norm * r_trunc_norm;
    let e_retained = r_sq_trunc * (4.0 / sigma_min_retained
        + sigma_max / (sigma_min_retained * sigma_min_retained));

    // Direction-aware: δx = V Σ⁻¹ Uᵀ r (using full SVD)
    let delta_x_full = svd_solve(u, sv, v_t, &r_full, size, size);
    let (cross_full, quad_full) = compute_remainder(normals, perm, m, &r_full, &delta_x_full);

    // Direction-aware: δx using truncated SVD
    let delta_x_trunc = svd_solve(u, sv, v_t, &r_trunc, size, rank);
    let (cross_trunc, quad_trunc) = compute_remainder(normals, perm, m, &r_trunc, &delta_x_trunc);

    let q_fully_corrected = q_first_order + 2.0 * cross_full - quad_full;

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

    // Use truncated SVD beta for β > 0 check (full SVD garbage on singular systems)
    let beta_trunc: Vec<f64> = (0..m).map(|i| x_trunc[i]).collect();
    let beta_min = beta_trunc.iter().cloned().fold(f64::INFINITY, f64::min);

    Diagnostics {
        sigma_max,
        sigma_min_full,
        sigma_min_retained,
        numerical_rank: rank,
        full_size: size,
        residual_norm_full: r_full_norm,
        residual_norm_truncated: r_trunc_norm,
        q_raw,
        q_first_order,
        e_full,
        e_retained,
        cross_abs_full: (2.0 * cross_full).abs(),
        quad_abs_full: quad_full.abs(),
        cross_abs_trunc: (2.0 * cross_trunc).abs(),
        quad_abs_trunc: quad_trunc.abs(),
        q_fully_corrected,
        singular_values: sv.iter().cloned().collect(),
        restricted_eigs,
        tangent_dim,
        beta_min,
    }
}

fn svd_solve(
    u: &DMatrix<f64>,
    sv: &DVector<f64>,
    v_t: &DMatrix<f64>,
    rhs: &DVector<f64>,
    size: usize,
    rank: usize,
) -> DVector<f64> {
    let mut x = DVector::zeros(size);
    for i in 0..rank {
        let coeff = u.column(i).dot(rhs) / sv[i];
        for j in 0..size {
            x[j] += coeff * v_t[(i, j)];
        }
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
        let svd = kkt.clone().svd(true, true);
        let sv = &svd.singular_values;
        let u = svd.u.as_ref().unwrap();
        let v_t = svd.v_t.as_ref().unwrap();
        let sigma_max = sv[0];
        let threshold = sigma_max * SVD_CONDITION_TAU;
        let rank = sv.iter().filter(|&&s| s > threshold).count();
        let size = m + 5;
        let x = svd_solve(u, sv, v_t, &rhs, size, rank);
        let beta: Vec<f64> = (0..m).map(|i| x[i]).collect();

        let q1 = q_from_beta(polytope.normals(), &alg_perm, &beta);
        let q2 = q_from_hessian(&kkt, &beta);
        let rel = if q1.abs() > 1e-15 { (q1 - q2).abs() / q1.abs() } else { (q1 - q2).abs() };
        println!("{:<14} {:>16.12} {:>16.12} {:>12.3e}", name, q1, q2, rel);
    }
    println!();

    // Table 1: Condition numbers and rank
    println!("--- Table 1: SVD condition and rank ---");
    println!("{:<14} {:>3} {:>5} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "polytope", "m", "rank", "σ_min_full", "σ_min_ret", "κ_full", "‖r‖_full", "‖r‖_trunc");
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
            d.sigma_min_full, d.sigma_min_retained,
            d.sigma_max / d.sigma_min_full.max(f64::MIN_POSITIVE),
            d.residual_norm_full, d.residual_norm_truncated);
    }

    println!();

    // Table 2: Error bounds — analytical vs direction-aware
    println!("--- Table 2: Analytical E vs actual |R| (direction-aware) ---");
    println!("{:<14} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "polytope", "E_full", "|R|_full", "loose_full", "E_retained", "|R|_trunc", "loose_ret");
    println!("{}", "-".repeat(88));

    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        let r_actual_full = d.cross_abs_full + d.quad_abs_full;
        let r_actual_trunc = d.cross_abs_trunc + d.quad_abs_trunc;
        let loose_full = if r_actual_full > 0.0 { d.e_full / r_actual_full } else { f64::INFINITY };
        let loose_ret = if r_actual_trunc > 0.0 { d.e_retained / r_actual_trunc } else { f64::INFINITY };

        println!("{:<14} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e}",
            name,
            d.e_full, r_actual_full, loose_full,
            d.e_retained, r_actual_trunc, loose_ret);
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

    // Table 4: SV spectra
    println!("--- Table 4: Singular value spectra ---");
    for (name, polytope) in &polytopes {
        let result = match ehz_capacity(polytope) {
            Some(r) => r,
            None => continue,
        };

        let mut alg_perm = result.best_permutation.clone();
        alg_perm.reverse();

        let d = run_diagnostics(polytope.normals(), polytope.heights(), &alg_perm);

        println!("{}: [{}]", name,
            d.singular_values.iter()
                .map(|s| format!("{:.3e}", s))
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
}
