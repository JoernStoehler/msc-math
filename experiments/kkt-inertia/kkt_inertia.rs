#![allow(clippy::collapsible_if, clippy::op_ref, clippy::bool_comparison, dead_code)]

/// KKT matrix inertia experiment.
///
/// Validates Lemma lem:kkt-inertia (Inertia of the KKT matrix) from
/// thesis/appendix-numerical.tex. The lemma states:
///   n_+(M) = n_+(H|_T) + p,  n_0(M) = n_0(H|_T) + (5 - p),  n_-(M) = n_-(H|_T) + p
/// where M is the KKT matrix, H|_T is the restricted Hessian on the tangent
/// space T = ker(A), and p = rank(A).
///
/// Two parts:
/// 1. Census: classify H|_T definiteness for all (S,σ) nodes with β>0, Q>0.
/// 2. Inertia check: verify the inertia decomposition formula.
///    On mismatch, print eigenvalue diagnostics to identify threshold artifacts
///    vs genuine violations.
///
/// Input: Known polytopes from the library (F ≤ 10).
/// Output: Summary tables to stdout. No hard assertions (diagnostic experiment).
use nalgebra::{DMatrix, DVector, Vector4};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::cyclic_permutations;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::augmented::{build_kkt_system as build_kkt, q_from_beta};

/// Condition-number threshold for rank truncation (matches EIGEN_CONDITION_TAU
/// in crates/src/kkt.rs).
const EIGEN_CONDITION_TAU: f64 = 1e-3;

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Definiteness { PD, ND, Indefinite, NearZero, Trivial }

struct NodeInfo {
    q: f64,
    beta_min: f64,
    definiteness: Definiteness,
    tangent_dim: usize,
    #[allow(dead_code)]
    m: usize,
}

/// Restricted Hessian eigenvalues for a single (S,σ) node.
/// Returns None only if the KKT matrix is numerically zero.
struct HessianEigenvalues {
    /// Eigenvalues of H|_T (restricted Hessian on tangent space).
    /// Empty if tangent_dim == 0.
    h_t_eigenvalues: Vec<f64>,
    /// Eigenvalues of the full KKT matrix M.
    m_eigenvalues: Vec<f64>,
    /// p = rank(A) = m - tangent_dim.
    p: usize,
    /// tangent_dim = dim(T) = dim(ker(A)).
    tangent_dim: usize,
}

/// Compute node info (β, Q, definiteness) for a single (S,σ).
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

/// Compute eigenvalues of both M and H|_T for diagnostic output on mismatches.
fn eigenvalue_diagnostics(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<HessianEigenvalues> {
    let m = perm.len();
    let (kkt, _) = build_kkt(normals, heights, perm);

    let eig = kkt.clone().symmetric_eigen();
    let mut m_eigenvalues: Vec<f64> = eig.eigenvalues.iter().cloned().collect();
    m_eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Tangent space via constraint null space
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
    let p = m - tangent_dim;

    let h_t_eigenvalues = if tangent_dim == 0 {
        vec![]
    } else {
        let mut proj = DMatrix::zeros(m, tangent_dim);
        for (k, &idx) in null_indices.iter().enumerate() {
            for i in 0..m {
                proj[(i, k)] = ctc_eig.eigenvectors[(i, idx)];
            }
        }
        let h_block = kkt.view((0, 0), (m, m)).clone_owned();
        let h_restricted = proj.transpose() * &h_block * &proj;
        let eig = h_restricted.symmetric_eigen();
        let mut vals: Vec<f64> = eig.eigenvalues.iter().cloned().collect();
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vals
    };

    Some(HessianEigenvalues { h_t_eigenvalues, m_eigenvalues, p, tangent_dim })
}

fn main() {
    let polytopes: Vec<(&str, Polytope4D)> = known_polytopes::all_known()
        .into_iter()
        .filter(|kp| kp.polytope.facet_count() <= 10)
        .map(|kp| (kp.name, kp.polytope))
        .collect();

    println!("=== KKT Inertia Experiment ===\n");
    println!("Validates Lemma lem:kkt-inertia: eigenvalue inertia decomposition");
    println!("of the KKT matrix M into restricted Hessian H|_T and constraint rank p.\n");

    // ── Part 1: Hessian definiteness census ─────────────────────────────
    println!("--- Part 1: Restricted Hessian H|_T across all (S,σ) nodes ---");
    println!("  Filtered to β>0, Q>0 nodes (valid KKT solutions).");
    println!("{:<25} {:>4} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "F", "total", "β>0", "Q>0", "triv", "PD", "ND", "indef", "~zero");
    println!("{}", "-".repeat(97));

    let eps_beta = 1e-8;
    let eps_q = 1e-12;

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();

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
                    if let Some(info) = node_hessian_check(&normals, &heights, &perm) {
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

    // ── Part 2: Inertia theorem validation ──────────────────────────────
    println!("--- Part 2: Inertia decomposition check ---");
    println!("  Lemma: n_-(M) = n_-(H|_T) + p, where p = rank(A).");
    println!("  Check: n_-(M) = p ↔ H|_T has no negative eigenvalues.");
    println!("{:<25} {:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "polytope", "total", "n-=p", "n->p", "PD", "ND", "indef", "match?");
    println!("{}", "-".repeat(85));

    let eig_eps = 1e-10;
    let mut total_inertia_mismatches = 0u64;
    // Collect mismatch details for diagnostic output
    let mut mismatch_details: Vec<(String, Vec<usize>, HessianEigenvalues, Definiteness)> = Vec::new();

    for (name, polytope) in &polytopes {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();
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
                    let (kkt_mat, _) = build_kkt(&normals, &heights, &perm);

                    let eig = kkt_mat.symmetric_eigen();
                    let n_neg = eig.eigenvalues.iter().filter(|&&e| e < -eig_eps).count();
                    let n_zero = eig.eigenvalues.iter().filter(|&&e| e.abs() <= eig_eps).count();

                    let info = node_hessian_check(&normals, &heights, &perm);
                    let (def, tangent_dim) = match info {
                        Some(n) => (n.definiteness, n.tangent_dim),
                        None => (Definiteness::Trivial, 0),
                    };
                    let p = mm - tangent_dim;

                    let inertia_says_pd = n_neg == p && n_zero == (5 - p);
                    let inertia_says_nsd = n_neg == p;
                    if inertia_says_nsd { n_negp += 1; } else { n_neggtp += 1; }

                    let is_mismatch = match def {
                        Definiteness::PD => {
                            n_pd += 1;
                            !inertia_says_pd
                        },
                        Definiteness::ND => {
                            n_nd += 1;
                            inertia_says_nsd
                        },
                        Definiteness::Indefinite => {
                            n_indef += 1;
                            inertia_says_nsd
                        },
                        Definiteness::NearZero | Definiteness::Trivial => false,
                    };

                    if is_mismatch {
                        mismatches += 1;
                        // Collect eigenvalue diagnostics for this mismatch
                        if let Some(diag) = eigenvalue_diagnostics(&normals, &heights, &perm) {
                            mismatch_details.push((
                                name.to_string(),
                                perm.to_vec(),
                                diag,
                                def,
                            ));
                        }
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
        println!("\n  {} inertia mismatches found. Eigenvalue diagnostics:\n", total_inertia_mismatches);
        for (i, (name, perm, diag, def)) in mismatch_details.iter().enumerate() {
            println!("  Mismatch {}: polytope={}, perm={:?}", i + 1, name, perm);
            println!("    H|_T classification: {:?}  (tangent_dim={}, p={})", def, diag.tangent_dim, diag.p);
            println!("    H|_T eigenvalues: {:?}", diag.h_t_eigenvalues.iter()
                .map(|v| format!("{:.3e}", v)).collect::<Vec<_>>());
            println!("    M eigenvalues:    {:?}", diag.m_eigenvalues.iter()
                .map(|v| format!("{:.3e}", v)).collect::<Vec<_>>());
            let n_neg_m = diag.m_eigenvalues.iter().filter(|&&e| e < -eig_eps).count();
            let n_zero_m = diag.m_eigenvalues.iter().filter(|&&e| e.abs() <= eig_eps).count();
            let n_pos_m = diag.m_eigenvalues.iter().filter(|&&e| e > eig_eps).count();
            println!("    M inertia (n+,n0,n-) = ({},{},{}), expected n- = p = {}",
                n_pos_m, n_zero_m, n_neg_m, diag.p);
            // Check if any H|_T eigenvalue is near the classification threshold
            let near_threshold: Vec<f64> = diag.h_t_eigenvalues.iter()
                .filter(|&&v| v.abs() < 10.0 * EPS_DEFINITE && v.abs() > 0.1 * EPS_DEFINITE)
                .cloned().collect();
            if !near_threshold.is_empty() {
                println!("    → H|_T eigenvalue(s) near threshold ({:.0e}): {:?}",
                    EPS_DEFINITE,
                    near_threshold.iter().map(|v| format!("{:.3e}", v)).collect::<Vec<_>>());
            }
            // Check if any M eigenvalue is near the threshold
            let m_near: Vec<f64> = diag.m_eigenvalues.iter()
                .filter(|&&v| v.abs() < 10.0 * eig_eps && v.abs() > 0.1 * eig_eps)
                .cloned().collect();
            if !m_near.is_empty() {
                println!("    → M eigenvalue(s) near threshold ({:.0e}): {:?}",
                    eig_eps,
                    m_near.iter().map(|v| format!("{:.3e}", v)).collect::<Vec<_>>());
            }
            println!();
        }
    } else {
        println!("\n  All inertia checks passed.");
    }

    // ── Summary ───────────────────────────────────────────────────────────
    println!("=== Summary ===");
    println!("  Part 1 (H|_T census):      diagnostic (all {} polytopes)", polytopes.len());
    println!("  Part 2 (inertia check):    {} mismatches", total_inertia_mismatches);
}
