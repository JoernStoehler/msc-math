//! Compute a bounded face-level Euclidean feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with scalar summaries
//! of edge lengths and facet 3-volumes derived from exact polytope geometry.
//! Input Artifacts:
//!   - experiments/sys-landscape/normalized-dataset outputs under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use num_bigint::BigInt;
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

#[derive(Debug, Deserialize)]
struct PolytopeInputRow {
    poly_id: String,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
}

#[derive(Debug, Serialize)]
struct FaceGeometryFeatureRow {
    poly_id: String,
    facet_count: usize,
    vertex_count: usize,
    edge_count: usize,
    edge_length_mean: f64,
    edge_length_std: f64,
    edge_length_min: f64,
    edge_length_max: f64,
    edge_length_max_share: f64,
    facet_volume_mean: f64,
    facet_volume_std: f64,
    facet_volume_min: f64,
    facet_volume_max: f64,
    facet_volume_sum: f64,
    facet_volume_max_share: f64,
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
        std::env::temp_dir().join(format!("sys-feature-face-geometry-{stamp}.jsonl"))
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

fn parse_vec4(data: &[[String; 4]]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| parse_rational(&row[i])))
        .collect()
}

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

fn max_share(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    if total <= 0.0 {
        return 0.0;
    }
    values.iter().copied().fold(f64::NEG_INFINITY, f64::max) / total
}

fn build_row(poly: &PolytopeInputRow) -> FaceGeometryFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        parse_vec4(&poly.dual_vertices_rational),
        parse_vec4(&poly.vertices_rational),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let skeleton = Skeleton::compute(&polytope);
    let vertices = polytope.vertices_f64();

    let edge_lengths = skeleton
        .edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm())
        .collect::<Vec<_>>();
    let facet_volumes = (0..poly.facet_count)
        .map(|facet| facet_volume_3d(&polytope, facet))
        .collect::<Vec<_>>();

    let (
        edge_length_mean,
        edge_length_std,
        edge_length_min,
        edge_length_max,
    ) = stats_or_zero(&edge_lengths);
    let (
        facet_volume_mean,
        facet_volume_std,
        facet_volume_min,
        facet_volume_max,
    ) = stats_or_zero(&facet_volumes);

    FaceGeometryFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: poly.facet_count,
        vertex_count: vertices.len(),
        edge_count: skeleton.edges.len(),
        edge_length_mean,
        edge_length_std,
        edge_length_min,
        edge_length_max,
        edge_length_max_share: max_share(&edge_lengths),
        facet_volume_mean,
        facet_volume_std,
        facet_volume_min,
        facet_volume_max,
        facet_volume_sum: facet_volumes.iter().sum::<f64>(),
        facet_volume_max_share: max_share(&facet_volumes),
    }
}

fn main() {
    let (normalized_dir, out) = parse_args();
    let polytopes = read_jsonl::<PolytopeInputRow>(&normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&out, &rows);
    println!("Wrote {} face-geometry rows", rows.len());
    println!("Output path: {}", out.display());
}
