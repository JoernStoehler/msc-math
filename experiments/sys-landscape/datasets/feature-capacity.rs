//! Compute a scalar capacity feature table keyed by `poly_id`.
//!
//! Goal: expose cached capacity values from the normalized dataset as a simple
//! datascience-facing feature block.
//! Input Artifacts:
//!   - experiments/sys-landscape/datasets/normalized/ under `--normalized-dir`
//!     (`capacity_results.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::features::{parse_standard_feature_args, read_jsonl, write_jsonl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct CapacityInputRow {
    poly_id: String,
    capacity: f64,
    #[serde(default)]
    iterations: Option<u64>,
    search_result_source: String,
}

#[derive(Debug, Serialize)]
struct CapacityFeatureRow {
    poly_id: String,
    capacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_iterations: Option<u64>,
    capacity_source: String,
}

fn enrich_row(row: CapacityInputRow) -> CapacityFeatureRow {
    CapacityFeatureRow {
        poly_id: row.poly_id,
        capacity: row.capacity,
        capacity_iterations: row.iterations,
        capacity_source: row.search_result_source,
    }
}

fn main() {
    let args = parse_standard_feature_args("capacity");
    let mut rows = read_jsonl::<CapacityInputRow>(&args.normalized_dir.join("capacity_results.jsonl"))
        .into_iter()
        .map(enrich_row)
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} capacity rows to {}", rows.len(), args.out.display());
}
