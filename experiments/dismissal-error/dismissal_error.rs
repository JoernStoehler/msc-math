//! Dismissal error bound experiment.
//!
//! Empirically validates that the value loss from near-singular system dismissal
//! (Algorithm `[alg:near-singular-handling]`, thesis appendix) is negligible.
//!
//! For each polytope in the test dataset, enumerates all (S, σ) pairs (with A2
//! adjacency pruning). When the SVD path would dismiss a pair, instead computes
//! β₀ and evaluates the error bound from Remark `[lem:dismissal-error-bound]`:
//!
//!   error_bound = (α*)² · ‖δβ'‖ · (1 + ‖H‖/σ_C) · σ_j
//!
//! Convention: The library (crates/) is stable. This binary copies pub(crate)
//! internals (marked with source references). See CLAUDE.md "Library stability
//! boundary".
//!
//! Architecture:
//! 1. `cargo run --bin dismissal_error --release` generates dataset
//! 2. Writes to dismissal-error/dismissal-error.jsonl
//! 3. Python script reads JSONL, produces summary table and figure
//!
//! Dataset: capacity_dataset.json fixture (27 polytopes, 5–10 facets).

use nalgebra::{DMatrix, DVector, Vector4};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::{ehz_capacity, omega0, Polytope4D};

// ============================================================================
// Constants — copied from crates/src/kkt.rs
// ============================================================================

/// Minimum β_i value to consider a solution valid.
/// Copied from crates/src/kkt.rs:12
const EPS_BETA_POSITIVE: f64 = 1e-12;

/// Floor for SVD singular values.
/// Copied from crates/src/kkt.rs:20
const EPS_SVD_FLOOR: f64 = 1e-12;

/// Condition-number threshold for SVD rank detection.
/// Copied from crates/src/kkt.rs:41
const SVD_CONDITION_TAU: f64 = 1e-3;

/// Maximum acceptable residual norm.
/// Copied from crates/src/kkt.rs:44
const EPS_KKT_RESIDUAL: f64 = 1e-6;

/// Facet incidence tolerance.
/// Copied from crates/src/constants.rs
const EPS_FACET_INCIDENCE: f64 = 1e-8;

// ============================================================================
// Dataset schema (input)
// ============================================================================

#[derive(Debug, Deserialize)]
struct TestPolytope {
    name: String,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    #[allow(dead_code)]
    volume: Option<f64>,
    capacity: Option<f64>,
    #[allow(dead_code)]
    capacity_unpruned: Option<f64>,
    #[allow(dead_code)]
    capacity_billiard: Option<f64>,
    #[allow(dead_code)]
    base_index: Option<usize>,
    #[allow(dead_code)]
    transform: Option<String>,
}

// ============================================================================
// Output schema (JSONL)
// ============================================================================

#[derive(Debug, Serialize)]
struct DismissalRecord {
    polytope_name: String,
    facet_count: usize,
    pair_size: usize,
    sigma_j: f64,
    sigma_c: f64,
    nu_0: f64,
    alpha_star: f64,
    delta_beta_prime_norm: f64,
    h_norm: f64,
    error_bound: f64,
    capacity: f64,
    relative_error: f64,
}

#[derive(Debug, Serialize)]
struct SummaryRecord {
    #[serde(rename = "type")]
    record_type: String,
    polytope_name: String,
    facet_count: usize,
    capacity: f64,
    total_pairs: u64,
    dismissals_with_bound: u64,
    trivial_dismissals: u64,
    max_error_bound: f64,
    max_relative_error: f64,
    time_ms: f64,
}

// ============================================================================
// Dismissal diagnostics result
// ============================================================================

enum DismissalResult {
    /// System is not near-singular, or near-singular but not dismissed.
    NoDismissal,
    /// Dismissed, but β₀ ≤ 0: no admissible critical point, dismissal loses nothing.
    TrivialDismissal,
    /// Dismissed with β₀ > 0: error bound applies.
    Dismissed {
        sigma_j: f64,
        sigma_c: f64,
        nu_0: f64,
        alpha_star: f64,
        delta_beta_prime_norm: f64,
        h_norm: f64,
        error_bound: f64,
    },
}

// ============================================================================
// Copied from library — KKT system building (crates/src/kkt.rs:184-223)
// ============================================================================

/// Build the KKT matrix and RHS vector.
/// Layout: [H | -N | -η; N^T | 0 | 0; η^T | 0 | 0], RHS = [0,...,0,1].
fn build_kkt_system(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (DMatrix<f64>, DVector<f64>) {
    let m = perm.len();
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
    rhs[size - 1] = 1.0;

    (kkt, rhs)
}

// ============================================================================
// Copied from library — Q(β) computation (crates/src/kkt.rs:59-69)
// ============================================================================

/// Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
fn q_from_beta(normals: &[Vector4<f64>], perm: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| beta[i] * beta[j] * omega0(&normals[perm[i]], &normals[perm[j]]))
        .sum()
}

// ============================================================================
// Copied from library — null space search (crates/src/kkt.rs:79-169)
// ============================================================================

fn find_positive_beta_1d(beta0: &[f64], v: &[f64]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for j in 0..m {
        if v[j].abs() < 1e-15 {
            if beta0[j] <= EPS_BETA_POSITIVE {
                return None;
            }
        } else {
            let bound = -beta0[j] / v[j];
            if v[j] > 0.0 {
                lo = lo.max(bound);
            } else {
                hi = hi.min(bound);
            }
        }
    }
    if lo >= hi {
        return None;
    }
    let alpha = if lo.is_finite() && hi.is_finite() {
        (lo + hi) / 2.0
    } else if lo.is_finite() {
        lo + 1.0
    } else if hi.is_finite() {
        hi - 1.0
    } else {
        0.0
    };
    let beta: Vec<f64> = (0..m).map(|j| beta0[j] + alpha * v[j]).collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

fn find_positive_beta_nd(beta0: &[f64], null_vecs: &[Vec<f64>]) -> Option<Vec<f64>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![0.0; k];
    for _iter in 0..100 {
        let beta: Vec<f64> = (0..m)
            .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
            .collect();
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        if *worst_val > EPS_BETA_POSITIVE {
            return Some(beta);
        }
        let grad_sq: f64 = (0..k).map(|i| null_vecs[i][worst_j].powi(2)).sum();
        if grad_sq < 1e-30 {
            return None;
        }
        let target = EPS_BETA_POSITIVE * 100.0;
        let step = (target - worst_val) / grad_sq;
        for i in 0..k {
            alpha[i] += step * null_vecs[i][worst_j];
        }
    }
    let beta: Vec<f64> = (0..m)
        .map(|j| beta0[j] + (0..k).map(|i| alpha[i] * null_vecs[i][j]).sum::<f64>())
        .collect();
    if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) {
        Some(beta)
    } else {
        None
    }
}

// ============================================================================
// Copied from library — combinatorics (crates/src/algorithms/hk2017/mod.rs)
// ============================================================================

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
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

// ============================================================================
// Copied from library — permutations (crates/src/algorithms/hk2017/permutations.rs)
// ============================================================================

fn for_each_cyclic_permutation(
    elements: &[usize],
    callback: &mut impl FnMut(&[usize]),
) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf(&mut buf, 1, k, callback);
}

fn heap_perms_buf(
    buf: &mut [usize],
    offset: usize,
    k: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_perms_buf(buf, offset, k - 1, callback);
    for i in 0..k - 1 {
        if k % 2 == 0 {
            buf.swap(offset + i, offset + k - 1);
        } else {
            buf.swap(offset, offset + k - 1);
        }
        heap_perms_buf(buf, offset, k - 1, callback);
    }
}

// ============================================================================
// Copied from library — adjacency (crates/src/algorithms/hk2017/mod.rs:194-247)
// ============================================================================

fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();

    // Vertex adjacency
    let mut vertex_adj = vec![vec![false; f]; f];
    for v in polytope.vertices() {
        let incident: Vec<usize> = (0..f)
            .filter(|&i| (normals[i].dot(v) - heights[i]).abs() < EPS_FACET_INCIDENCE)
            .collect();
        for &i in &incident {
            for &j in &incident {
                vertex_adj[i][j] = true;
            }
        }
    }

    // Directed: vertex adj + ω₀ condition
    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            adj[i][j] = vertex_adj[i][j] && omega0(&normals[j], &normals[i]) >= 0.0;
        }
    }
    adj
}

fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

// ============================================================================
// Instrumented SVD path — computes dismissal diagnostics
// ============================================================================

/// Analyse a single (S, σ) pair for dismissal diagnostics.
///
/// Replicates the SVD path of `solve_kkt_svd_path` (crates/src/kkt.rs:233-359)
/// but instead of returning None on dismissal, computes the error bound from
/// Remark `[lem:dismissal-error-bound]` (thesis appendix eq. A.3).
fn solve_kkt_dismissal_diagnostics(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> DismissalResult {
    let m = perm.len();
    let size = m + 5;

    let (kkt, rhs) = build_kkt_system(normals, heights, perm);
    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
    if max_sv < EPS_SVD_FLOOR {
        return DismissalResult::NoDismissal;
    }

    let u = match svd.u.as_ref() {
        Some(u) => u,
        None => return DismissalResult::NoDismissal,
    };
    let v_t = match svd.v_t.as_ref() {
        Some(v_t) => v_t,
        None => return DismissalResult::NoDismissal,
    };

    // Determine numerical rank via condition-number threshold.
    let threshold = max_sv * SVD_CONDITION_TAU;
    let rank = sv.iter().filter(|&&s| s > threshold).count();

    // Only check for dismissal if near-singular and m >= 5.
    if rank >= size || m < 5 {
        return DismissalResult::NoDismissal;
    }

    // Build constraint matrix C = [N | η] ∈ ℝ^{m×5} and compute σ_C.
    let mut c_matrix = DMatrix::zeros(m, 5);
    for i in 0..m {
        let n = &normals[perm[i]];
        for j in 0..4 {
            c_matrix[(i, j)] = n[j];
        }
        c_matrix[(i, 4)] = heights[perm[i]];
    }
    let c_svd = c_matrix.clone().svd(false, false);
    let sigma_c = c_svd
        .singular_values
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);

    if sigma_c <= EPS_SVD_FLOOR {
        return DismissalResult::NoDismissal;
    }

    // Check each near-null direction for dismissal.
    for j in rank..size {
        let delta_beta_norm: f64 = (0..m).map(|k| v_t[(j, k)].powi(2)).sum::<f64>().sqrt();

        if delta_beta_norm <= sv[j] / sigma_c {
            // Near-null direction confined to multiplier block, not dismissed.
            continue;
        }

        // This direction would trigger dismissal. Compute β₀ via pseudoinverse
        // to evaluate the error bound.
        let mut x0 = DVector::zeros(size);
        for i in 0..rank {
            let coeff = u.column(i).dot(&rhs) / sv[i];
            for k in 0..size {
                x0[k] += coeff * v_t[(i, k)];
            }
        }

        let residual = (&kkt * &x0 - &rhs).norm();
        if residual > EPS_KKT_RESIDUAL {
            // Poor solution quality — cannot compute reliable bound.
            return DismissalResult::TrivialDismissal;
        }

        let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();

        // Try to find β > 0 (may need null space search).
        let beta_positive = if beta0.iter().all(|&b| b > EPS_BETA_POSITIVE) {
            Some(beta0.clone())
        } else if rank < size {
            let null_beta: Vec<Vec<f64>> = (rank..size)
                .map(|i| (0..m).map(|k| v_t[(i, k)]).collect())
                .collect();
            if null_beta.len() == 1 {
                find_positive_beta_1d(&beta0, &null_beta[0])
            } else {
                find_positive_beta_nd(&beta0, &null_beta)
            }
        } else {
            None
        };

        let beta_pos = match beta_positive {
            Some(b) => b,
            None => return DismissalResult::TrivialDismissal,
        };

        // β₀ > 0 exists. Compute the error bound.
        let nu_0 = q_from_beta(normals, perm, &beta_pos);
        if nu_0 <= 0.0 {
            return DismissalResult::TrivialDismissal;
        }

        // Extract δβ from the near-null direction (first m components of v_t row j).
        let delta_beta: Vec<f64> = (0..m).map(|k| v_t[(j, k)]).collect();

        // Compute constraint residual: r_β = N^T δβ ∈ R^4, r_ν = η^T δβ ∈ R.
        let mut r_constraint = DVector::zeros(5);
        for i in 0..m {
            for d in 0..4 {
                r_constraint[d] += normals[perm[i]][d] * delta_beta[i];
            }
            r_constraint[4] += heights[perm[i]] * delta_beta[i];
        }

        // Corrected direction: δβ' = δβ - C(C^T C)^{-1} [r_β; r_ν].
        // Compute C(C^T C)^{-1} r via solving (C^T C) z = r, then C z.
        let ctc = c_matrix.transpose() * &c_matrix;
        let z = match ctc.clone().lu().solve(&r_constraint) {
            Some(z) => z,
            None => return DismissalResult::TrivialDismissal,
        };
        let correction = &c_matrix * z;
        let delta_beta_prime: Vec<f64> = (0..m)
            .map(|i| delta_beta[i] - correction[i])
            .collect();
        let delta_beta_prime_norm: f64 =
            delta_beta_prime.iter().map(|x| x * x).sum::<f64>().sqrt();

        if delta_beta_prime_norm < 1e-15 {
            // Correction killed the direction — bound is zero.
            return DismissalResult::Dismissed {
                sigma_j: sv[j],
                sigma_c,
                nu_0,
                alpha_star: 0.0,
                delta_beta_prime_norm: 0.0,
                h_norm: 0.0,
                error_bound: 0.0,
            };
        }

        // α* = min{β₀,i / |δβ'_i| : δβ'_i < 0}.
        let alpha_star = delta_beta_prime
            .iter()
            .enumerate()
            .filter(|(_, &db)| db < 0.0)
            .map(|(i, &db)| beta_pos[i] / db.abs())
            .fold(f64::INFINITY, f64::min);

        if !alpha_star.is_finite() || alpha_star <= 0.0 {
            // δβ' has no negative component — mixed-sign check failed, skip.
            // This shouldn't happen if constraints are satisfied, but guard anyway.
            return DismissalResult::TrivialDismissal;
        }

        // ‖H‖ = spectral norm of H (top-left m×m block of KKT matrix).
        let h_matrix = kkt.view((0, 0), (m, m)).clone_owned();
        let h_svd = h_matrix.svd(false, false);
        let h_norm = h_svd
            .singular_values
            .iter()
            .cloned()
            .fold(0.0f64, f64::max);

        // Error bound: (α*)² · ‖δβ'‖ · (1 + ‖H‖/σ_C) · σ_j
        let sigma_j = sv[j];
        let error_bound =
            alpha_star.powi(2) * delta_beta_prime_norm * (1.0 + h_norm / sigma_c) * sigma_j;

        return DismissalResult::Dismissed {
            sigma_j,
            sigma_c,
            nu_0,
            alpha_star,
            delta_beta_prime_norm,
            h_norm,
            error_bound,
        };
    }

    // No near-null direction triggered dismissal.
    DismissalResult::NoDismissal
}

// ============================================================================
// Dataset loading
// ============================================================================

fn load_test_dataset(path: &std::path::Path) -> Vec<TestPolytope> {
    let file = File::open(path).unwrap_or_else(|e| {
        panic!(
            "Failed to open dataset: {}. Run from experiments/ directory.\nError: {}",
            path.display(),
            e
        )
    });
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse dataset JSON")
}

fn polytope_from_test(tp: &TestPolytope) -> Polytope4D {
    let normals: Vec<Vector4<f64>> = tp
        .normals
        .iter()
        .map(|n| Vector4::new(n[0], n[1], n[2], n[3]))
        .collect();
    Polytope4D::new(normals, tp.heights.clone())
        .unwrap_or_else(|e| panic!("Failed to construct polytope '{}': {}", tp.name, e))
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let dataset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../crates/tests/fixtures/capacity_dataset.json");
    let output_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dismissal-error/dismissal-error.jsonl");

    let dataset = load_test_dataset(&dataset_path);
    let file = File::create(&output_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    println!(
        "Dismissal error bound experiment: {} polytopes",
        dataset.len()
    );
    println!("Output: {}", output_path.display());
    println!();

    let mut total_dismissals_with_bound = 0u64;
    let mut total_trivial_dismissals = 0u64;
    let mut global_max_relative_error = 0.0f64;

    for tp in &dataset {
        let start = Instant::now();
        let polytope = polytope_from_test(tp);
        let f = polytope.facet_count();
        let normals = polytope.normals();
        let heights = polytope.heights();

        // Get production capacity.
        let capacity = if let Some(c) = tp.capacity {
            c
        } else {
            match ehz_capacity(&polytope) {
                Some(r) => r.capacity,
                None => {
                    println!("  {} (F={}): no capacity — skipping", tp.name, f);
                    continue;
                }
            }
        };

        // Precompute directed adjacency matrix (A2 pruning, matches production).
        let adj = build_directed_adjacency_matrix(&polytope);

        let mut total_pairs = 0u64;
        let mut dismissals_with_bound = 0u64;
        let mut trivial_dismissals = 0u64;
        let mut max_error_bound = 0.0f64;
        let mut max_relative_error = 0.0f64;

        for m in 2..=f {
            for subset in combinations(f, m) {
                for_each_cyclic_permutation(&subset, &mut |perm| {
                    if !is_adjacent_cycle(perm, &adj) {
                        return;
                    }
                    total_pairs += 1;

                    match solve_kkt_dismissal_diagnostics(normals, heights, perm) {
                        DismissalResult::NoDismissal => {}
                        DismissalResult::TrivialDismissal => {
                            trivial_dismissals += 1;
                        }
                        DismissalResult::Dismissed {
                            sigma_j,
                            sigma_c,
                            nu_0,
                            alpha_star,
                            delta_beta_prime_norm,
                            h_norm,
                            error_bound,
                        } => {
                            dismissals_with_bound += 1;
                            let relative_error = error_bound / capacity;
                            if error_bound > max_error_bound {
                                max_error_bound = error_bound;
                            }
                            if relative_error > max_relative_error {
                                max_relative_error = relative_error;
                            }

                            let record = DismissalRecord {
                                polytope_name: tp.name.clone(),
                                facet_count: f,
                                pair_size: perm.len(),
                                sigma_j,
                                sigma_c,
                                nu_0,
                                alpha_star,
                                delta_beta_prime_norm,
                                h_norm,
                                error_bound,
                                capacity,
                                relative_error,
                            };
                            let line = serde_json::to_string(&record).expect("serialize");
                            writeln!(writer, "{line}").expect("write");
                        }
                    }
                });
            }
        }

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        // Emit summary line.
        let summary = SummaryRecord {
            record_type: "summary".to_string(),
            polytope_name: tp.name.clone(),
            facet_count: f,
            capacity,
            total_pairs,
            dismissals_with_bound,
            trivial_dismissals,
            max_error_bound,
            max_relative_error,
            time_ms: elapsed,
        };
        let line = serde_json::to_string(&summary).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        total_dismissals_with_bound += dismissals_with_bound;
        total_trivial_dismissals += trivial_dismissals;
        if max_relative_error > global_max_relative_error {
            global_max_relative_error = max_relative_error;
        }

        println!(
            "  {} (F={}): {} pairs, {} dismissed (bound), {} trivial, max rel error = {:.2e}  [{:.0}ms]",
            tp.name, f, total_pairs, dismissals_with_bound, trivial_dismissals,
            max_relative_error, elapsed
        );
    }

    writer.flush().expect("flush");

    println!();
    println!("=== Summary ===");
    println!(
        "Total dismissals with bound: {}",
        total_dismissals_with_bound
    );
    println!("Total trivial dismissals: {}", total_trivial_dismissals);
    println!(
        "Global max relative error: {:.2e}",
        global_max_relative_error
    );
    println!("Output: {}", output_path.display());
}
