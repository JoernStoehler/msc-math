//! Assemble the combined polytope-level datascience feature table.
//!
//! Goal: load normalized polytope/capacity tables, enrich each polytope with
//! deterministic geometry/orbit/scalar features, and write one combined JSONL
//! keyed by `poly_id`.

use exp_sys_landscape::features::{default_feature_output_path, write_jsonl};
use exp_sys_landscape::polytope_features::{build_cache_index, enrich_row, load_inputs};
use exp_sys_landscape::{package_root, raw_dataset_cache_path};
use std::path::PathBuf;

struct Args {
    normalized_dir: PathBuf,
    out: PathBuf,
    continuation_cache: PathBuf,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut normalized_dir: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut continuation_cache: Option<PathBuf> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--normalized-dir" => {
                let value = args.get(i + 1).expect("--normalized-dir requires a value");
                normalized_dir = Some(PathBuf::from(value));
                i += 2;
            }
            "--out" => {
                let value = args.get(i + 1).expect("--out requires a value");
                out = Some(PathBuf::from(value));
                i += 2;
            }
            "--continuation-cache" => {
                let value = args
                    .get(i + 1)
                    .expect("--continuation-cache requires a value");
                continuation_cache = Some(PathBuf::from(value));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        normalized_dir: normalized_dir.expect("--normalized-dir is required"),
        out: out.unwrap_or_else(|| default_feature_output_path("polytope-features")),
        continuation_cache: continuation_cache.unwrap_or_else(|| raw_dataset_cache_path("continuation")),
    }
}

fn main() {
    let args = parse_args();
    let inputs = load_inputs(&args.normalized_dir);
    let cache = build_cache_index(&package_root(), &args.continuation_cache);
    let rows = inputs
        .iter()
        .map(|row| enrich_row(row, &cache))
        .collect::<Vec<_>>();
    write_jsonl(&args.out, &rows);
    println!(
        "Wrote {} polytope-feature rows to {}",
        rows.len(),
        args.out.display()
    );
}
