//! Shared types and helpers for collect_poly.rs.
//!
//! Included via `#[path = "collect_common.rs"] mod common;` in each binary.

use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[path = "projection_solver.rs"]
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
    pub h: Vec<Vec<f64>>, // m x m symmetric
    pub c: Vec<Vec<f64>>, // p x m (p=5 for EHZ)
    pub d: Vec<f64>,      // p

    // f64 projection solver output
    pub verdict: String, // "true", "false", "indeterminate"
    pub q: f64,
    pub margin: f64,
    pub norm_h: f64,
    pub sigma_min_c: f64,

    // Raw solver vectors (empty if not feasible)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beta: Vec<f64>,

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

/// Run the projection solver on (H, C, d) and record all output.
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
    let norm_h = spectral_norm(h);
    let smin_c = sigma_min(c);

    let qp = solvers::QP {
        h: h.clone(),
        c: c.clone(),
        d: d.clone(),
    };

    // Run solver with panic catching
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        solvers::solve_projected(&qp)
    }));

    let (verdict, q, margin, beta) = match result {
        Ok(sol) => (
            match sol.verdict {
                solvers::Verdict::True => "true".to_string(),
                solvers::Verdict::False => "false".to_string(),
                solvers::Verdict::Indeterminate => "indeterminate".to_string(),
            },
            sol.q,
            sol.margin,
            sol.beta,
        ),
        Err(_) => ("panic".to_string(), f64::NAN, f64::NAN, Vec::new()),
    };

    InputRow {
        family: family.to_string(),
        instance,
        m,
        dataset: dataset.to_string(),
        h: matrix_to_vecs(h),
        c: matrix_to_vecs(c),
        d: d.iter().copied().collect(),
        verdict,
        q,
        margin,
        norm_h,
        sigma_min_c: smin_c,
        beta,
        polytope_id,
        perm: perm_opt,
        facet_count,
    }
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
        let n_true = fam_rows.iter().filter(|r| r.verdict == "true").count();
        let n_panic = fam_rows.iter().filter(|r| r.verdict == "panic").count();
        println!(
            "    {:<25} {:>5} rows, {:>5} true, {:>3} panics",
            fam, n, n_true, n_panic
        );
    }
}
