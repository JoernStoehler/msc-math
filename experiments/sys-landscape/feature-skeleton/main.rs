//! Compute a bounded skeleton-derived feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with cheap
//! combinatorial features from the exact 4D face lattice.
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
struct SkeletonFeatureRow {
    poly_id: String,
    vertex_count: usize,
    edge_count: usize,
    ridge_count: usize,
    facet_count: usize,
    is_simple: bool,
    simple_vertex_fraction: f64,
    edge_density: f64,
    vertex_incident_facets_mean: f64,
    vertex_incident_facets_std: f64,
    vertex_incident_facets_min: f64,
    vertex_incident_facets_max: f64,
    vertex_degree_mean: f64,
    vertex_degree_std: f64,
    vertex_degree_min: f64,
    vertex_degree_max: f64,
    ridge_size_mean: f64,
    ridge_size_std: f64,
    ridge_size_min: f64,
    ridge_size_max: f64,
    facet_vertex_count_mean: f64,
    facet_vertex_count_std: f64,
    facet_vertex_count_min: f64,
    facet_vertex_count_max: f64,
    facet_neighbor_count_mean: f64,
    facet_neighbor_count_std: f64,
    facet_neighbor_count_min: f64,
    facet_neighbor_count_max: f64,
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
        std::env::temp_dir().join(format!("sys-feature-skeleton-{stamp}.jsonl"))
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
        let numer = BigInt::from_str(numer).unwrap_or_else(|e| panic!("bad numerator {token}: {e}"));
        let denom = BigInt::from_str(denom).unwrap_or_else(|e| panic!("bad denominator {token}: {e}"));
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

fn stats(values: &[f64]) -> (f64, f64, f64, f64) {
    assert!(!values.is_empty(), "stats requires non-empty slice");
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

fn build_row(poly: &PolytopeInputRow) -> SkeletonFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        parse_vec4(&poly.dual_vertices_rational),
        parse_vec4(&poly.vertices_rational),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let skeleton = Skeleton::compute(&polytope);

    let vertex_count = polytope.vertices().len();
    let edge_count = skeleton.edges.len();
    let ridge_count = skeleton.ridges.len();
    let facet_count = poly.facet_count;

    let vertex_incident_facets = skeleton
        .vertex_facets
        .iter()
        .map(|facets| facets.len() as f64)
        .collect::<Vec<_>>();
    let simple_vertices = vertex_incident_facets
        .iter()
        .filter(|&&count| (count - 4.0).abs() < f64::EPSILON)
        .count();
    let simple_vertex_fraction = simple_vertices as f64 / vertex_count as f64;
    let is_simple = simple_vertices == vertex_count;

    let mut vertex_degrees = vec![0usize; vertex_count];
    for edge in &skeleton.edges {
        vertex_degrees[edge[0]] += 1;
        vertex_degrees[edge[1]] += 1;
    }
    let vertex_degrees = vertex_degrees
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();

    let ridge_sizes = skeleton
        .ridges
        .iter()
        .map(|ridge| ridge.vertices.len() as f64)
        .collect::<Vec<_>>();

    let mut facet_vertex_counts = vec![0usize; facet_count];
    for facets in &skeleton.vertex_facets {
        for &facet in facets {
            facet_vertex_counts[facet] += 1;
        }
    }
    let facet_vertex_counts = facet_vertex_counts
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();

    let facet_neighbor_counts = (0..facet_count)
        .map(|facet| {
            (0..facet_count)
                .filter(|&other| polytope.vertex_adjacency()[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();

    let edge_density = if vertex_count >= 2 {
        (2.0 * edge_count as f64) / ((vertex_count * (vertex_count - 1)) as f64)
    } else {
        0.0
    };

    let (
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
    ) = stats(&vertex_incident_facets);
    let (vertex_degree_mean, vertex_degree_std, vertex_degree_min, vertex_degree_max) =
        stats(&vertex_degrees);
    let (ridge_size_mean, ridge_size_std, ridge_size_min, ridge_size_max) =
        stats(&ridge_sizes);
    let (
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
    ) = stats(&facet_vertex_counts);
    let (
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    ) = stats(&facet_neighbor_counts);

    SkeletonFeatureRow {
        poly_id: poly.poly_id.clone(),
        vertex_count,
        edge_count,
        ridge_count,
        facet_count,
        is_simple,
        simple_vertex_fraction,
        edge_density,
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
        vertex_degree_mean,
        vertex_degree_std,
        vertex_degree_min,
        vertex_degree_max,
        ridge_size_mean,
        ridge_size_std,
        ridge_size_min,
        ridge_size_max,
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    }
}

fn main() {
    let (normalized_dir, out) = parse_args();
    let polytopes = read_jsonl::<PolytopeInputRow>(&normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&out, &rows);
    println!("Wrote {} skeleton rows", rows.len());
    println!("Output path: {}", out.display());
}
