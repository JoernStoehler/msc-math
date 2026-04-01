//! Shared types and helpers for collect_poly.rs and collect_synth.rs.
//!
//! Included via `#[path = "collect_common.rs"] mod common;` in each binary.

use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[path = "solvers.rs"]
pub mod solvers;

/// Number of constraint rows. Matches the EHZ structure (4 closure + 1 normalization).
pub const P: usize = 5;

/// Row in collected_*.jsonl — input matrices + f64 solver output.
#[derive(Serialize, Deserialize)]
pub struct InputRow {
    // Metadata
    pub family: String,
    pub instance: usize,
    pub m: usize,
    pub dataset: String, // "synthetic" or "poly"

    // KKT input matrices (row-major)
    pub h: Vec<Vec<f64>>, // m×m symmetric
    pub c: Vec<Vec<f64>>, // p×m (p=5 for EHZ)
    pub d: Vec<f64>,      // p

    // f64 solver output (raw vectors for downstream filtering/analysis)
    pub verdict: String, // "feasible", "beta_non_positive", "residual_too_large", "panic"
    pub q: f64,
    pub q_raw: f64,
    pub margin: f64,
    pub residual_norm: f64,
    pub rank: usize,
    pub norm_h: f64,
    pub sigma_min_c: f64,

    // Raw solver vectors (empty if not feasible)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beta: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lambda: Vec<f64>,

    // Polytope metadata (only for poly dataset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytope_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_count: Option<usize>,
}

/// Spectral norm = max singular value.
pub fn spectral_norm(m: &DMatrix<f64>) -> f64 {
    let svd = m.clone().svd(false, false);
    svd.singular_values
        .iter()
        .cloned()
        .fold(0.0f64, f64::max)
}

/// Smallest singular value.
pub fn sigma_min(m: &DMatrix<f64>) -> f64 {
    let svd = m.clone().svd(false, false);
    svd.singular_values
        .iter()
        .cloned()
        .filter(|&s| s > 1e-15)
        .fold(f64::INFINITY, f64::min)
}

/// Convert DMatrix to Vec<Vec<f64>> for JSON serialization.
pub fn matrix_to_vecs(m: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}

/// Run the saddle-point solver on (H, C, d) and record all output.
pub fn solve_and_record(
    family: &str,
    instance: usize,
    dataset: &str,
    h: &DMatrix<f64>,
    c: &DMatrix<f64>,
    d: &DVector<f64>,
    polytope_id: Option<usize>,
    perm_opt: Option<Vec<usize>>,
    facet_count: Option<usize>,
) -> InputRow {
    let m = h.nrows();
    let p = c.nrows();
    let size = m + p;

    let norm_h = spectral_norm(h);
    let smin_c = sigma_min(c);

    // Build augmented KKT matrix M = [[H, C^T], [C, 0]] and rhs = [0..0, d]
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for i in 0..m {
        for j in 0..m {
            kkt[(i, j)] = h[(i, j)];
        }
    }
    for i in 0..p {
        for j in 0..m {
            kkt[(j, m + i)] = c[(i, j)];
            kkt[(m + i, j)] = c[(i, j)];
        }
    }
    for i in 0..p {
        rhs[m + i] = d[i];
    }

    // Run solver with panic catching
    let h_clone = h.clone();
    let kkt_clone = kkt.clone();
    let rhs_clone = rhs.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        solvers::solve_saddle_point(&kkt_clone, &rhs_clone)
    }));

    // Eigendecompose for rank
    let eig = kkt.clone().symmetric_eigen();
    let max_abs = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    let tau = 1e-3;
    let strict_threshold = max_abs * tau;
    let rank = eig
        .eigenvalues
        .iter()
        .filter(|&&e| e.abs() > strict_threshold)
        .count();

    let mut row = InputRow {
        family: family.to_string(),
        instance,
        m,
        dataset: dataset.to_string(),
        h: matrix_to_vecs(&h_clone),
        c: matrix_to_vecs(c),
        d: d.iter().copied().collect(),
        verdict: String::new(),
        q: f64::NAN,
        q_raw: f64::NAN,
        margin: f64::NAN,
        residual_norm: f64::NAN,
        rank,
        norm_h,
        sigma_min_c: smin_c,
        beta: Vec::new(),
        lambda: Vec::new(),
        polytope_id,
        perm: perm_opt,
        facet_count,
    };

    match result {
        Ok(ref outcome) => {
            row.verdict = outcome.verdict_str().to_string();
            if let solvers::KktOutcome::Feasible(kkt_result) = outcome {
                let mut x_vec = kkt_result.beta.clone();
                x_vec.extend_from_slice(&kkt_result.mu);
                x_vec.push(kkt_result.xi);
                let x_dv = DVector::from_column_slice(&x_vec);
                let residual_vec = &kkt * &x_dv - &rhs;
                row.residual_norm = residual_vec.norm();
                row.margin = kkt_result.beta.iter().copied().fold(f64::INFINITY, f64::min);
                row.q = kkt_result.q_corrected;
                row.q_raw = kkt_result.q_raw;
                row.beta = kkt_result.beta.clone();
                let mut lam = kkt_result.mu.clone();
                lam.push(kkt_result.xi);
                row.lambda = lam;
            }
        }
        Err(_) => {
            row.verdict = "panic".to_string();
            row.rank = 0;
        }
    }

    row
}

/// Write rows to JSONL.
pub fn write_jsonl(rows: &[InputRow], path: &str) {
    let mut file = std::fs::File::create(path)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", path, e));

    for row in rows {
        let json = serde_json::to_string(row).expect("JSON serialization failed");
        writeln!(file, "{}", json).expect("Write failed");
    }

    println!("Wrote {} rows to {}", rows.len(), path);
}

/// Print summary stats.
pub fn print_summary(rows: &[InputRow], label: &str) {
    let total = rows.len();
    let mut verdict_counts: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *verdict_counts.entry(&row.verdict).or_insert(0) += 1;
    }

    println!("\n=== {} Summary ===", label);
    println!("  Total: {}", total);
    for (verdict, count) in &verdict_counts {
        println!(
            "  {:20} {:>6} ({:.1}%)",
            verdict,
            count,
            100.0 * *count as f64 / total as f64
        );
    }

    let mut families: Vec<String> = rows.iter().map(|r| r.family.clone()).collect();
    families.sort();
    families.dedup();
    println!("\n  Per-family breakdown:");
    for fam in &families {
        let fam_rows: Vec<&InputRow> = rows.iter().filter(|r| r.family == *fam).collect();
        let n = fam_rows.len();
        let n_feas = fam_rows.iter().filter(|r| r.verdict == "feasible").count();
        let n_panic = fam_rows.iter().filter(|r| r.verdict == "panic").count();
        println!(
            "    {:<25} {:>5} rows, {:>5} feasible, {:>3} panics",
            fam, n, n_feas, n_panic
        );
    }
}
