//! Compute a datascience-facing dual-vertex feature table keyed by `poly_id`.
//!
//! Goal: expose cached exact dual vertices as floating-point feature columns for
//! downstream analysis, while keeping exact coordinates in the raw cache layer.
//! Input Artifacts:
//!   - experiments/sys-landscape/datasets/normalized/ under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct PolytopeInputRow {
    poly_id: String,
    dual_vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
}

#[derive(Debug, Serialize)]
struct DualVerticesFeatureRow {
    poly_id: String,
    facet_count: usize,
    dual_vertex_count: usize,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_flat_f64: Vec<f64>,
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
        std::env::temp_dir().join(format!("sys-feature-dual-vertices-{stamp}.jsonl"))
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

fn parse_rational(token: &str) -> BigRational {
    if let Some((numer, denom)) = token.split_once('/') {
        let numer =
            BigInt::from_str(numer).unwrap_or_else(|e| panic!("bad numerator {token}: {e}"));
        let denom =
            BigInt::from_str(denom).unwrap_or_else(|e| panic!("bad denominator {token}: {e}"));
        BigRational::new(numer, denom)
    } else {
        BigRational::from_integer(
            BigInt::from_str(token).unwrap_or_else(|e| panic!("bad integer {token}: {e}")),
        )
    }
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
}

fn main() {
    let (normalized_dir, out) = parse_args();
    let mut rows = read_jsonl::<PolytopeInputRow>(&normalized_dir.join("polytopes.jsonl"))
        .into_iter()
        .map(|row| {
            let dual_vertices_f64 = row
                .dual_vertices_rational
                .iter()
                .map(|vertex| std::array::from_fn(|i| rational_to_f64(&parse_rational(&vertex[i]))))
                .collect::<Vec<_>>();
            let dual_vertices_flat_f64 = dual_vertices_f64
                .iter()
                .flat_map(|vertex| vertex.iter().copied())
                .collect::<Vec<_>>();
            DualVerticesFeatureRow {
                poly_id: row.poly_id,
                facet_count: row.facet_count,
                dual_vertex_count: dual_vertices_f64.len(),
                dual_vertices_f64,
                dual_vertices_flat_f64,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&out, &rows);
    println!("Wrote {} dual-vertex rows to {}", rows.len(), out.display());
}
