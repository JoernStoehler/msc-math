//! Compute a scalar volume feature table keyed by `poly_id`.
//!
//! Goal: expose cached volume values from the normalized dataset as a simple
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
    volume: f64,
}

#[derive(Debug, Serialize)]
struct VolumeFeatureRow {
    poly_id: String,
    volume: f64,
}

fn enrich_row(row: CapacityInputRow) -> VolumeFeatureRow {
    VolumeFeatureRow {
        poly_id: row.poly_id,
        volume: row.volume,
    }
}

fn main() {
    let args = parse_standard_feature_args("volume");
    let mut rows = read_jsonl::<CapacityInputRow>(&args.normalized_dir.join("capacity_results.jsonl"))
        .into_iter()
        .map(enrich_row)
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} volume rows to {}", rows.len(), args.out.display());
}
