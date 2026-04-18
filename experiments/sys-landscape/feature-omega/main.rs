//! Compute a bounded symplectic/transition feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with cheap symplectic
//! summaries from exact facet adjacency, omega signs, and ridge-local `omega_0`
//! magnitudes, without orbit recomputation.
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
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;

#[derive(Debug, Deserialize)]
struct PolytopeInputRow {
    poly_id: String,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
}

#[derive(Debug, Serialize)]
struct OmegaFeatureRow {
    poly_id: String,
    facet_count: usize,
    allpair_abs_omega_mean: f64,
    allpair_abs_omega_std: f64,
    allpair_abs_omega_min: f64,
    allpair_abs_omega_max: f64,
    allpair_zero_fraction: f64,
    ridge_abs_omega_mean: f64,
    ridge_abs_omega_std: f64,
    ridge_abs_omega_min: f64,
    ridge_abs_omega_max: f64,
    ridge_zero_fraction: f64,
    ridge_abs_omega_le_1e3_fraction: f64,
    ridge_abs_omega_le_1e2_fraction: f64,
    ridge_abs_omega_le_1e1_fraction: f64,
    transition_density: f64,
    transition_bidirectional_fraction: f64,
    transition_out_degree_mean: f64,
    transition_out_degree_std: f64,
    transition_out_degree_min: f64,
    transition_out_degree_max: f64,
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
        std::env::temp_dir().join(format!("sys-feature-omega-{stamp}.jsonl"))
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

fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
}

fn build_row(poly: &PolytopeInputRow) -> OmegaFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        parse_vec4(&poly.dual_vertices_rational),
        parse_vec4(&poly.vertices_rational),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let skeleton = Skeleton::compute(&polytope);
    let duals = polytope.dual_vertices_f64();
    let f = poly.facet_count;

    let mut allpair_abs_omegas = Vec::new();
    let mut allpair_zero_count = 0usize;
    for i in 0..f {
        for j in (i + 1)..f {
            let value = omega0(&duals[i], &duals[j]);
            if polytope.omega_signs()[(i, j)] == 0 {
                allpair_zero_count += 1;
            }
            allpair_abs_omegas.push(value.abs());
        }
    }

    let ridge_abs_omegas = skeleton
        .ridges
        .iter()
        .map(|ridge| omega0(&duals[ridge.facets[0]], &duals[ridge.facets[1]]).abs())
        .collect::<Vec<_>>();
    let ridge_zero_count = skeleton
        .ridges
        .iter()
        .filter(|ridge| polytope.omega_signs()[(ridge.facets[0], ridge.facets[1])] == 0)
        .count();

    let transition = build_transition_matrix(&polytope);
    let mut transition_true_count = 0usize;
    let mut adjacent_pair_count = 0usize;
    let mut bidirectional_pair_count = 0usize;
    let mut out_degrees = Vec::new();
    for i in 0..f {
        let mut out = 0usize;
        for j in 0..f {
            if transition[(i, j)] {
                transition_true_count += 1;
                out += 1;
            }
        }
        out_degrees.push(out as f64);
    }
    for i in 0..f {
        for j in (i + 1)..f {
            if polytope.vertex_adjacency()[(i, j)] {
                adjacent_pair_count += 1;
                if transition[(i, j)] && transition[(j, i)] {
                    bidirectional_pair_count += 1;
                }
            }
        }
    }

    let (allpair_abs_omega_mean, allpair_abs_omega_std, allpair_abs_omega_min, allpair_abs_omega_max) =
        stats(&allpair_abs_omegas);
    let (ridge_abs_omega_mean, ridge_abs_omega_std, ridge_abs_omega_min, ridge_abs_omega_max) =
        stats(&ridge_abs_omegas);
    let (transition_out_degree_mean, transition_out_degree_std, transition_out_degree_min, transition_out_degree_max) =
        stats(&out_degrees);

    let total_pairs = (f * (f - 1) / 2) as f64;
    let transition_density = transition_true_count as f64 / (f * (f - 1)) as f64;
    let transition_bidirectional_fraction = if adjacent_pair_count > 0 {
        bidirectional_pair_count as f64 / adjacent_pair_count as f64
    } else {
        0.0
    };

    OmegaFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: f,
        allpair_abs_omega_mean,
        allpair_abs_omega_std,
        allpair_abs_omega_min,
        allpair_abs_omega_max,
        allpair_zero_fraction: allpair_zero_count as f64 / total_pairs,
        ridge_abs_omega_mean,
        ridge_abs_omega_std,
        ridge_abs_omega_min,
        ridge_abs_omega_max,
        ridge_zero_fraction: ridge_zero_count as f64 / skeleton.ridges.len() as f64,
        ridge_abs_omega_le_1e3_fraction: fraction_at_most(&ridge_abs_omegas, 1e-3),
        ridge_abs_omega_le_1e2_fraction: fraction_at_most(&ridge_abs_omegas, 1e-2),
        ridge_abs_omega_le_1e1_fraction: fraction_at_most(&ridge_abs_omegas, 1e-1),
        transition_density,
        transition_bidirectional_fraction,
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    }
}

fn main() {
    let (normalized_dir, out) = parse_args();
    let polytopes = read_jsonl::<PolytopeInputRow>(&normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&out, &rows);
    println!("Wrote {} omega rows", rows.len());
    println!("Output path: {}", out.display());
}
