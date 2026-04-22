//! Assemble the combined datascience-facing feature datasets.
//!
//! Goal: load core tables once, compute all cheap derived feature datasets, and
//! write them into one features output directory.

use exp_sys_landscape::datascience::io::write_jsonl;
use exp_sys_landscape::polytope_features::{enrich_row as enrich_polytope_row, load_inputs as load_polytope_inputs};
use exp_sys_landscape::trajectory_features::{enrich_row as enrich_trajectory_row, load_inputs as load_trajectory_inputs};
use std::path::PathBuf;

struct Args {
    core_tables_dir: PathBuf,
    out_dir: PathBuf,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut core_tables_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--core-tables-dir" => {
                let value = args.get(i + 1).expect("--core-tables-dir requires a value");
                core_tables_dir = Some(PathBuf::from(value));
                i += 2;
            }
            "--out-dir" => {
                let value = args.get(i + 1).expect("--out-dir requires a value");
                out_dir = Some(PathBuf::from(value));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        core_tables_dir: core_tables_dir.expect("--core-tables-dir is required"),
        out_dir: out_dir.expect("--out-dir is required"),
    }
}

fn main() {
    let args = parse_args();
    std::fs::create_dir_all(&args.out_dir).expect("create features output dir");

    let polytope_inputs = load_polytope_inputs(&args.core_tables_dir);
    let polytope_rows = polytope_inputs
        .iter()
        .map(enrich_polytope_row)
        .collect::<Vec<_>>();
    write_jsonl(&args.out_dir.join("polytope-features.jsonl"), &polytope_rows);

    let trajectory_inputs = load_trajectory_inputs(&args.core_tables_dir);
    let trajectory_rows = trajectory_inputs
        .iter()
        .map(enrich_trajectory_row)
        .collect::<Vec<_>>();
    write_jsonl(&args.out_dir.join("trajectory-features.jsonl"), &trajectory_rows);

    println!(
        "Wrote {} polytope-feature rows to {}",
        polytope_rows.len(),
        args.out_dir.join("polytope-features.jsonl").display()
    );
    println!(
        "Wrote {} trajectory-feature rows to {}",
        trajectory_rows.len(),
        args.out_dir.join("trajectory-features.jsonl").display()
    );
}
