//! Compute a scalar capacity feature table keyed by `poly_id`.
//!
//! Goal: expose cached capacity values from the normalized dataset as a simple
//! datascience-facing feature block.
//! Input Artifacts:
//!   - experiments/sys-landscape/datasets/normalized/ under `--normalized-dir`
//!     (`capacity_results.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

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

fn parse_args() -> (PathBuf, PathBuf) {
    let args: Vec<String> = std::env::args().collect();
    let mut normalized_dir: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
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
            other => panic!("unknown argument: {other}"),
        }
    }
    let normalized_dir = normalized_dir.expect("--normalized-dir is required");
    let out = out.unwrap_or_else(|| {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before UNIX_EPOCH")
            .as_millis();
        std::env::temp_dir().join(format!("sys-feature-capacity-{stamp}.jsonl"))
    });
    (normalized_dir, out)
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<T>(&line)
                .unwrap_or_else(|e| panic!("parse {}: {e}\nline={line}", path.display()))
        })
        .collect()
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(writer).expect("write newline");
    }
    writer.flush().expect("flush output");
}

fn main() {
    let (normalized_dir, out) = parse_args();
    let mut rows = read_jsonl::<CapacityInputRow>(&normalized_dir.join("capacity_results.jsonl"))
        .into_iter()
        .map(|row| CapacityFeatureRow {
            poly_id: row.poly_id,
            capacity: row.capacity,
            capacity_iterations: row.iterations,
            capacity_source: row.search_result_source,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&out, &rows);
    println!("Wrote {} capacity rows to {}", rows.len(), out.display());
}
