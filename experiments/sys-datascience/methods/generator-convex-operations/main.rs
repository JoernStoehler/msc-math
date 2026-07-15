//! Target-free smoke for natural convex operations on planar current-law factors.
//!
//! The producer deliberately stops at validated two-dimensional geometry.  It
//! never evaluates a capacity or selects a row by a downstream target.  Every
//! operation is area-normalized and recentered by the named area centroid.

use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

const SCHEMA: &str = "generator-convex-operations-row-v2";
const REPORT_SCHEMA: &str = "generator-convex-operations-report-v2";
const LAW_VERSION: &str = "generator-convex-operations-v2";
const PRODUCER_PATH: &str =
    "experiments/sys-datascience/methods/generator-convex-operations/main.rs";
const CARGO_MANIFEST_PATH: &str = "experiments/sys-landscape/Cargo.toml";
const CARGO_LOCK_PATH: &str = "Cargo.lock";
const SUPPORT_GRID_SIZE: usize = 64;
const SEEDS: &[u64] = &[20260715, 20260716, 20260717];
const SIDE_COUNTS: &[usize] = &[3, 4, 6];
const ROWS_PER_BUCKET: usize = 2;

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    attempts: usize,
    rows_per_bucket: usize,
}

#[derive(Clone, Debug)]
struct Factor {
    vertices: Vec<Vector2<f64>>,
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}

#[derive(Clone, Debug)]
struct Generated {
    output: Factor,
    sources: Vec<Factor>,
    active_input_inequalities: usize,
    active_source_vertices: usize,
    lineage: String,
    source_coordinate_overlap_a: Option<f64>,
    source_coordinate_overlap_b: Option<f64>,
    source_ids: Vec<String>,
    source_rotation_angles_rad: Vec<f64>,
    reflection_theta_rad: Option<f64>,
    reflection_axis: Option<[f64; 2]>,
    reflection_symmetry_residual: Option<f64>,
}

#[derive(Serialize)]
struct Row {
    schema: &'static str,
    law_version: &'static str,
    sample_id: String,
    output_id: String,
    source_ids: Vec<String>,
    operation: String,
    law_kind: String,
    parameter: String,
    operation_parameters: BTreeMap<String, String>,
    seed: u64,
    side_count_requested: usize,
    source_side_counts: Vec<usize>,
    output_side_count: Option<usize>,
    active_input_inequalities: Option<usize>,
    active_source_vertices: Option<usize>,
    row_index: usize,
    attempt: usize,
    attempts: usize,
    accepted: bool,
    status: String,
    rejection_reason: Option<String>,
    lineage: String,
    center: &'static str,
    area_normalized: bool,
    area: Option<f64>,
    vertices_ccw: Option<Vec<[f64; 2]>>,
    support_signature_64: Option<Vec<f64>>,
    source_vertices_ccw: Vec<Vec<[f64; 2]>>,
    source_support_signatures_64: Vec<Vec<f64>>,
    source_rotation_angles_rad: Vec<f64>,
    reflection_theta_rad: Option<f64>,
    reflection_axis: Option<[f64; 2]>,
    reflection_symmetry_residual: Option<f64>,
    perimeter: Option<f64>,
    covariance_anisotropy: Option<f64>,
    source_coordinate_overlap_a: Option<f64>,
    source_coordinate_overlap_b: Option<f64>,
}

#[derive(Serialize)]
struct Disposition {
    operation: &'static str,
    law_kind: &'static str,
    status: &'static str,
    formula: &'static str,
    note: &'static str,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    law_version: &'static str,
    seed_panel: Vec<u64>,
    side_counts: Vec<usize>,
    rows_per_bucket: usize,
    max_attempts_per_row: usize,
    requested_rows: usize,
    rows: usize,
    status_counts: BTreeMap<String, usize>,
    operation_summary: BTreeMap<String, OperationSummary>,
    side_count_histogram: BTreeMap<String, usize>,
    dispositions: Vec<Disposition>,
    command: &'static str,
    producer_revision: String,
    producer_tree: String,
    producer_paths: Vec<&'static str>,
    cargo_lock_blake3: String,
    input_hashes: BTreeMap<String, String>,
    output_rows_blake3: String,
    output_rows_count: usize,
    source_dirty: bool,
    interpretation_boundary: &'static str,
    abandoned: Vec<Abandonment>,
}

#[derive(Default, Serialize)]
struct OperationSummary {
    requested: usize,
    accepted: usize,
    exhausted: usize,
    total_attempts: usize,
    output_side_counts: BTreeMap<String, usize>,
}

#[derive(Serialize)]
struct Abandonment {
    operation: &'static str,
    status: &'static str,
    reason: &'static str,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/generator-convex-operations/artifacts",
        ),
        attempts: 32,
        rows_per_bucket: ROWS_PER_BUCKET,
    };
    let mut i = 1;
    while i < argv.len() {
        let value = |flag: &str| {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} needs a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(value("--out-dir"));
                i += 2;
            }
            "--attempts" => {
                args.attempts = value("--attempts").parse().expect("attempts must be usize");
                i += 2;
            }
            "--rows-per-bucket" => {
                args.rows_per_bucket = value("--rows-per-bucket")
                    .parse()
                    .expect("rows must be usize");
                i += 2;
            }
            "--help" | "-h" => {
                println!("--out-dir DIR --attempts N --rows-per-bucket N");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.rows_per_bucket > 0);
    args
}

fn seed_bytes(
    seed: u64,
    operation: &str,
    n: usize,
    row: usize,
    attempt: usize,
    role: &str,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&seed.to_le_bytes());
    for value in [operation, role] {
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(0);
    }
    bytes.extend_from_slice(&(n as u64).to_le_bytes());
    bytes.extend_from_slice(&(row as u64).to_le_bytes());
    bytes.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&bytes).as_bytes()
}

fn shoelace(vertices: &[Vector2<f64>]) -> f64 {
    vertices
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let b = vertices[(i + 1) % vertices.len()];
            a[0] * b[1] - b[0] * a[1]
        })
        .sum::<f64>()
        / 2.0
}

fn cross(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>) -> f64 {
    let ab = b - a;
    let ac = c - a;
    ab[0] * ac[1] - ab[1] * ac[0]
}

fn convex_hull(mut points: Vec<Vector2<f64>>) -> Vec<Vector2<f64>> {
    points.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    points.dedup_by(|a, b| (*a - *b).norm() < 1e-11);
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    for point in points.iter().copied() {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 1e-11
        {
            lower.pop();
        }
        lower.push(point);
    }
    let mut upper = Vec::new();
    for point in points.iter().rev().copied() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 1e-11
        {
            upper.pop();
        }
        upper.push(point);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn polygon_centroid(vertices: &[Vector2<f64>]) -> Option<Vector2<f64>> {
    let mut twice_area = 0.0;
    let mut sum = Vector2::zeros();
    for i in 0..vertices.len() {
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        let c = a[0] * b[1] - b[0] * a[1];
        twice_area += c;
        sum += (a + b) * c;
    }
    (twice_area.abs() > 1e-12).then_some(sum / (3.0 * twice_area))
}

fn factor_from_vertices(mut vertices: Vec<Vector2<f64>>) -> Option<Factor> {
    vertices = convex_hull(vertices);
    if vertices.len() < 3 {
        return None;
    }
    if shoelace(&vertices) < 0.0 {
        vertices.reverse();
    }
    let area = shoelace(&vertices);
    if !area.is_finite() || area <= 1e-11 {
        return None;
    }
    let mut normals = Vec::with_capacity(vertices.len());
    let mut heights = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        let edge = b - a;
        let len = edge.norm();
        if !len.is_finite() || len <= 1e-11 {
            return None;
        }
        let normal = Vector2::new(edge[1] / len, -edge[0] / len);
        let height = normal.dot(&a);
        if !height.is_finite() {
            return None;
        }
        normals.push(normal);
        heights.push(height);
    }
    let factor = Factor {
        vertices,
        normals,
        heights,
    };
    validate_factor(&factor).then_some(factor)
}

fn centered_normalized(vertices: Vec<Vector2<f64>>) -> Option<Factor> {
    let factor = factor_from_vertices(vertices)?;
    let center = polygon_centroid(&factor.vertices)?;
    let translated: Vec<_> = factor.vertices.iter().map(|p| *p - center).collect();
    let area = shoelace(&translated).abs();
    if !area.is_finite() || area <= 1e-11 {
        return None;
    }
    let scale = area.sqrt().recip();
    factor_from_vertices(translated.into_iter().map(|p| p * scale).collect())
}

fn validate_factor(factor: &Factor) -> bool {
    if factor.vertices.len() < 3
        || factor.normals.len() != factor.vertices.len()
        || factor.heights.len() != factor.vertices.len()
    {
        return false;
    }
    let area = shoelace(&factor.vertices);
    if !area.is_finite() || area <= 1e-10 {
        return false;
    }
    for i in 0..factor.vertices.len() {
        let a = factor.vertices[i];
        let b = factor.vertices[(i + 1) % factor.vertices.len()];
        let c = factor.vertices[(i + 2) % factor.vertices.len()];
        if cross(a, b, c) <= 1e-9 {
            return false;
        }
        let n = factor.normals[i];
        if (n.norm() - 1.0).abs() > 1e-8 {
            return false;
        }
        if (n.dot(&a) - factor.heights[i]).abs() > 1e-7 {
            return false;
        }
        if factor
            .vertices
            .iter()
            .any(|p| n.dot(p) > factor.heights[i] + 1e-8)
        {
            return false;
        }
        if !factor.heights[i].is_finite() {
            return false;
        }
    }
    true
}

fn current_factor(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let mut angles: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * TAU).collect();
    angles.sort_by(f64::total_cmp);
    let heights: Vec<f64> = (0..n).map(|_| 0.8 + 0.4 * rng.gen::<f64>()).collect();
    let mut vertices = Vec::with_capacity(n);
    for i in 0..n {
        let j = (i + 1) % n;
        let a = Vector2::new(angles[i].cos(), angles[i].sin());
        let b = Vector2::new(angles[j].cos(), angles[j].sin());
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() <= 1e-10 {
            return None;
        }
        let x = (heights[i] * b[1] - heights[j] * a[1]) / det;
        let y = (a[0] * heights[j] - b[0] * heights[i]) / det;
        vertices.push(Vector2::new(x, y));
    }
    let factor = factor_from_vertices(vertices)?;
    // Current-law conditioning: every requested H inequality is active.
    (factor.vertices.len() == n).then(|| centered_normalized(factor.vertices).unwrap())
}

fn rotate(factor: &Factor, theta: f64) -> Option<Factor> {
    let (s, c) = theta.sin_cos();
    centered_normalized(
        factor
            .vertices
            .iter()
            .map(|p| Vector2::new(c * p[0] - s * p[1], s * p[0] + c * p[1]))
            .collect(),
    )
}

fn minkowski_raw(a: &Factor, b: &Factor) -> Vec<Vector2<f64>> {
    a.vertices
        .iter()
        .flat_map(|x| b.vertices.iter().map(move |y| *x + *y))
        .collect()
}

fn minkowski_sum(a: &Factor, b: &Factor) -> Option<Factor> {
    centered_normalized(minkowski_raw(a, b))
}

fn reflected(factor: &Factor, theta: f64) -> Option<Factor> {
    let (s, c) = (2.0 * theta).sin_cos();
    let reflect = |p: Vector2<f64>| {
        // Reflection across the line through the origin with direction
        // `(cos(theta), sin(theta))`: [[cos(2θ), sin(2θ)], [sin(2θ), -cos(2θ)]].
        Vector2::new(c * p[0] + s * p[1], s * p[0] - c * p[1])
    };
    centered_normalized(factor.vertices.iter().map(|p| reflect(*p)).collect())
}

fn line_intersection(a: Vector2<f64>, ha: f64, b: Vector2<f64>, hb: f64) -> Option<Vector2<f64>> {
    let det = a[0] * b[1] - a[1] * b[0];
    (det.abs() > 1e-10).then_some(Vector2::new(
        (ha * b[1] - hb * a[1]) / det,
        (a[0] * hb - b[0] * ha) / det,
    ))
}

fn intersection_raw(a: &Factor, b: &Factor) -> Option<Vec<Vector2<f64>>> {
    let mut points = Vec::new();
    for (na, &ha) in a.normals.iter().zip(&a.heights) {
        for (nb, &hb) in b.normals.iter().zip(&b.heights) {
            if let Some(p) = line_intersection(*na, ha, *nb, hb) {
                if a.normals
                    .iter()
                    .zip(&a.heights)
                    .all(|(n, h)| n.dot(&p) <= *h + 1e-8)
                    && b.normals
                        .iter()
                        .zip(&b.heights)
                        .all(|(n, h)| n.dot(&p) <= *h + 1e-8)
                {
                    points.push(p);
                }
            }
        }
    }
    for p in a.vertices.iter().chain(&b.vertices) {
        if a.normals
            .iter()
            .zip(&a.heights)
            .all(|(n, h)| n.dot(p) <= *h + 1e-8)
            && b.normals
                .iter()
                .zip(&b.heights)
                .all(|(n, h)| n.dot(p) <= *h + 1e-8)
        {
            points.push(*p);
        }
    }
    let hull = convex_hull(points);
    (hull.len() >= 3).then_some(hull)
}

fn active_inequalities(output: &Factor, source: &Factor) -> usize {
    source
        .normals
        .iter()
        .zip(&source.heights)
        .filter(|(n, h)| {
            output
                .vertices
                .iter()
                .any(|p| (n.dot(p) - **h).abs() < 5e-7)
        })
        .count()
}

fn point_on_boundary(p: Vector2<f64>, factor: &Factor) -> bool {
    factor
        .normals
        .iter()
        .zip(&factor.heights)
        .any(|(n, h)| (n.dot(&p) - *h).abs() < 5e-7)
}

fn hull_union(a: &Factor, b: &Factor) -> Option<(Factor, usize)> {
    let raw = convex_hull(a.vertices.iter().chain(&b.vertices).copied().collect());
    let raw_factor = factor_from_vertices(raw)?;
    let active = a
        .vertices
        .iter()
        .chain(&b.vertices)
        .filter(|p| point_on_boundary(**p, &raw_factor))
        .count();
    let hull = centered_normalized(raw_factor.vertices)?;
    Some((hull, active))
}

fn area_intersection(a: &Factor, b: &Factor) -> Option<f64> {
    intersection_raw(a, b).map(|v| shoelace(&v).abs())
}

fn overlap_ratio(output: &Factor, source: &Factor) -> Option<f64> {
    area_intersection(output, source).map(|area| area / shoelace(&source.vertices).abs())
}

fn source_id(
    operation: &str,
    seed: u64,
    n: usize,
    row: usize,
    attempt: usize,
    index: usize,
) -> String {
    format!(
        "{LAW_VERSION}/{operation}/seed={seed}/n={n}/row={row}/attempt={attempt}/source={index}"
    )
}

fn operation(
    operation: &str,
    n: usize,
    seed: u64,
    row: usize,
    attempt: usize,
) -> Option<Generated> {
    let mut ra = ChaCha8Rng::from_seed(seed_bytes(seed, operation, n, row, attempt, "a"));
    let a = current_factor(n, &mut ra)?;
    let angle_a = ra.gen::<f64>() * TAU;
    let a = rotate(&a, angle_a)?;
    let mut source_rotation_angles_rad = vec![angle_a];
    let b = if matches!(
        operation,
        "minkowski-sum" | "intersection" | "convex-hull-union"
    ) {
        let mut rb = ChaCha8Rng::from_seed(seed_bytes(seed, operation, n, row, attempt, "b"));
        let angle_b = rb.gen::<f64>() * TAU;
        source_rotation_angles_rad.push(angle_b);
        let b = rotate(&current_factor(n, &mut rb)?, angle_b)?;
        Some(b)
    } else {
        None
    };
    let reflection_theta_rad = if operation == "minkowski-symmetrization" {
        Some(ra.gen::<f64>() * PI)
    } else {
        None
    };
    let (
        output,
        active,
        active_vertices,
        lineage,
        sources,
        source_coordinate_overlap_a,
        source_coordinate_overlap_b,
    ) = match operation {
        "baseline" => (
            a.clone(),
            0,
            n,
            "current-law fresh baseline".to_string(),
            vec![a.clone()],
            Some(1.0),
            None,
        ),
        "minkowski-sum" => {
            let b = b.clone()?;
            let raw = factor_from_vertices(minkowski_raw(&a, &b))?;
            let output = centered_normalized(raw.vertices.clone())?;
            (
                output,
                0,
                0,
                "independent current-law A + independent current-law B".to_string(),
                vec![a.clone(), b.clone()],
                overlap_ratio(&raw, &a),
                overlap_ratio(&raw, &b),
            )
        }
        "intersection" => {
            let b = b?;
            let raw = intersection_raw(&a, &b)?;
            let raw_factor = factor_from_vertices(raw)?;
            let active =
                active_inequalities(&raw_factor, &a) + active_inequalities(&raw_factor, &b);
            let overlap_a = overlap_ratio(&raw_factor, &a);
            let overlap_b = overlap_ratio(&raw_factor, &b);
            let out = centered_normalized(raw_factor.vertices.clone())?;
            (
                out,
                active,
                0,
                "independent rotated current-law A ∩ B; retain active inequalities".to_string(),
                vec![a.clone(), b],
                overlap_a,
                overlap_b,
            )
        }
        "difference-body" => {
            let neg: Vec<_> = a.vertices.iter().map(|p| -*p).collect();
            let b = centered_normalized(neg)?;
            let raw = factor_from_vertices(minkowski_raw(&a, &b))?;
            let output = centered_normalized(raw.vertices.clone())?;
            (
                output,
                0,
                0,
                "deterministic pushforward K + (-K)".to_string(),
                vec![a.clone()],
                overlap_ratio(&raw, &a),
                None,
            )
        }
        "convex-hull-union" => {
            let b = b?;
            let raw =
                factor_from_vertices(a.vertices.iter().chain(&b.vertices).copied().collect())?;
            let active = a
                .vertices
                .iter()
                .chain(&b.vertices)
                .filter(|p| point_on_boundary(**p, &raw))
                .count();
            let overlap_a = overlap_ratio(&raw, &a);
            let overlap_b = overlap_ratio(&raw, &b);
            let out = centered_normalized(raw.vertices.clone())?;
            (
                out,
                0,
                active,
                "independent rotated current-law hull(A ∪ B)".to_string(),
                vec![a.clone(), b],
                overlap_a,
                overlap_b,
            )
        }
        "minkowski-symmetrization" => {
            let theta = reflection_theta_rad?;
            let reflected = reflected(&a, theta)?;
            let raw = factor_from_vertices(minkowski_raw(&a, &reflected))?;
            let out = centered_normalized(raw.vertices.clone())?;
            (
                out,
                0,
                0,
                "deterministic pushforward (K + reflection_u K)/2; classical Minkowski symmetrization".to_string(),
                vec![a.clone()],
                overlap_ratio(&raw, &a),
                None,
            )
        }
        _ => return None,
    };
    let source_ids = (0..sources.len())
        .map(|index| source_id(operation, seed, n, row, attempt, index))
        .collect();
    let reflection_axis = reflection_theta_rad.map(|theta| [theta.cos(), theta.sin()]);
    let reflection_symmetry_residual =
        reflection_theta_rad.map(|theta| reflection_symmetry_residual(&output, theta));
    Some(Generated {
        output,
        sources,
        active_input_inequalities: active,
        active_source_vertices: active_vertices,
        lineage,
        source_coordinate_overlap_a,
        source_coordinate_overlap_b,
        source_ids,
        source_rotation_angles_rad,
        reflection_axis,
        reflection_symmetry_residual,
        reflection_theta_rad,
    })
}

fn perimeter(factor: &Factor) -> f64 {
    factor
        .vertices
        .iter()
        .enumerate()
        .map(|(i, p)| (*p - factor.vertices[(i + 1) % factor.vertices.len()]).norm())
        .sum()
}

fn covariance_anisotropy(factor: &Factor) -> Option<f64> {
    let center = polygon_centroid(&factor.vertices)?;
    let mut xx = 0.0;
    let mut yy = 0.0;
    let mut xy = 0.0;
    for p in &factor.vertices {
        let q = *p - center;
        xx += q[0] * q[0];
        yy += q[1] * q[1];
        xy += q[0] * q[1];
    }
    let trace = xx + yy;
    if trace <= 1e-12 {
        return None;
    }
    let disc = ((xx - yy).powi(2) + 4.0 * xy * xy).sqrt();
    Some(disc / trace)
}

fn vertices_arrays(factor: &Factor) -> Vec<[f64; 2]> {
    factor.vertices.iter().map(|p| [p[0], p[1]]).collect()
}

fn support_signature_64(factor: &Factor) -> Vec<f64> {
    (0..SUPPORT_GRID_SIZE)
        .map(|k| {
            let theta = TAU * k as f64 / SUPPORT_GRID_SIZE as f64;
            let direction = Vector2::new(theta.cos(), theta.sin());
            factor
                .vertices
                .iter()
                .map(|p| direction.dot(p))
                .fold(f64::NEG_INFINITY, f64::max)
        })
        .collect()
}

fn reflect_point(point: Vector2<f64>, theta: f64) -> Vector2<f64> {
    let (s, c) = (2.0 * theta).sin_cos();
    Vector2::new(c * point[0] + s * point[1], s * point[0] - c * point[1])
}

fn reflection_symmetry_residual(factor: &Factor, theta: f64) -> f64 {
    factor
        .vertices
        .iter()
        .map(|point| {
            let reflected = reflect_point(*point, theta);
            factor
                .vertices
                .iter()
                .map(|candidate| (reflected - *candidate).norm())
                .fold(f64::INFINITY, f64::min)
        })
        .fold(0.0, f64::max)
}

fn central_symmetry_residual(factor: &Factor) -> f64 {
    factor
        .vertices
        .iter()
        .map(|point| {
            factor
                .vertices
                .iter()
                .map(|candidate| (-*point - *candidate).norm())
                .fold(f64::INFINITY, f64::min)
        })
        .fold(0.0, f64::max)
}

fn row_for(
    operation_name: &str,
    n: usize,
    seed: u64,
    row: usize,
    attempt: usize,
    generated: Generated,
) -> Row {
    let output = &generated.output;
    let area = shoelace(&output.vertices).abs();
    let sources = &generated.sources;
    let sample_id =
        format!("{LAW_VERSION}/{operation_name}/seed={seed}/n={n}/row={row}/attempt={attempt}");
    let mut operation_parameters = BTreeMap::new();
    operation_parameters.insert("requested_side_count".to_string(), n.to_string());
    for (index, angle) in generated.source_rotation_angles_rad.iter().enumerate() {
        operation_parameters.insert(
            format!("source_rotation_{index}_rad"),
            format!("{angle:.17e}"),
        );
    }
    if let Some(theta) = generated.reflection_theta_rad {
        operation_parameters.insert("reflection_theta_rad".to_string(), format!("{theta:.17e}"));
        operation_parameters.insert(
            "reflection_axis".to_string(),
            format!("[{:.17e},{:.17e}]", theta.cos(), theta.sin()),
        );
    }
    let law_kind = if matches!(
        operation_name,
        "minkowski-sum" | "intersection" | "convex-hull-union"
    ) {
        "binary_random"
    } else if operation_name == "baseline" {
        "fresh_current_law"
    } else {
        "deterministic_pushforward"
    };
    Row {
        schema: SCHEMA,
        law_version: LAW_VERSION,
        sample_id: sample_id.clone(),
        output_id: format!("{sample_id}/output"),
        source_ids: generated.source_ids.clone(),
        operation: operation_name.to_string(),
        law_kind: law_kind.to_string(),
        parameter: format!("side_count={n}"),
        operation_parameters,
        seed,
        side_count_requested: n,
        source_side_counts: sources.iter().map(|s| s.vertices.len()).collect(),
        output_side_count: Some(output.vertices.len()),
        active_input_inequalities: Some(generated.active_input_inequalities),
        active_source_vertices: Some(generated.active_source_vertices),
        row_index: row,
        attempt,
        attempts: attempt + 1,
        accepted: true,
        status: "survived".to_string(),
        rejection_reason: None,
        lineage: generated.lineage,
        center: "area_centroid",
        area_normalized: (area - 1.0).abs() < 1e-8,
        area: Some(area),
        vertices_ccw: Some(vertices_arrays(output)),
        support_signature_64: Some(support_signature_64(output)),
        source_vertices_ccw: sources.iter().map(vertices_arrays).collect(),
        source_support_signatures_64: sources.iter().map(support_signature_64).collect(),
        source_rotation_angles_rad: generated.source_rotation_angles_rad,
        reflection_theta_rad: generated.reflection_theta_rad,
        reflection_axis: generated.reflection_axis,
        reflection_symmetry_residual: generated.reflection_symmetry_residual,
        perimeter: Some(perimeter(output)),
        covariance_anisotropy: covariance_anisotropy(output),
        source_coordinate_overlap_a: generated.source_coordinate_overlap_a,
        source_coordinate_overlap_b: generated.source_coordinate_overlap_b,
    }
}

fn exhausted_row(operation: &str, n: usize, seed: u64, row: usize, attempts: usize) -> Row {
    let kind = if matches!(
        operation,
        "minkowski-sum" | "intersection" | "convex-hull-union"
    ) {
        "binary_random"
    } else if operation == "baseline" {
        "fresh_current_law"
    } else {
        "deterministic_pushforward"
    };
    Row {
        schema: SCHEMA,
        law_version: LAW_VERSION,
        sample_id: format!(
            "{LAW_VERSION}/{operation}/seed={seed}/n={n}/row={row}/attempt={}",
            attempts.saturating_sub(1)
        ),
        output_id: String::new(),
        source_ids: Vec::new(),
        operation: operation.to_string(),
        law_kind: kind.to_string(),
        parameter: format!("side_count={n}"),
        operation_parameters: BTreeMap::from([("requested_side_count".to_string(), n.to_string())]),
        seed,
        side_count_requested: n,
        source_side_counts: Vec::new(),
        output_side_count: None,
        active_input_inequalities: None,
        active_source_vertices: None,
        row_index: row,
        attempt: attempts.saturating_sub(1),
        attempts,
        accepted: false,
        status: "exhausted".to_string(),
        rejection_reason: Some("no strict-valid operation result in bounded attempts".to_string()),
        lineage: "bounded attempt exhaustion; no row deleted by side-count filtering".to_string(),
        center: "area_centroid",
        area_normalized: false,
        area: None,
        vertices_ccw: None,
        support_signature_64: None,
        source_vertices_ccw: Vec::new(),
        source_support_signatures_64: Vec::new(),
        source_rotation_angles_rad: Vec::new(),
        reflection_theta_rad: None,
        reflection_axis: None,
        reflection_symmetry_residual: None,
        perimeter: None,
        covariance_anisotropy: None,
        source_coordinate_overlap_a: None,
        source_coordinate_overlap_b: None,
    }
}

fn dispositions() -> Vec<Disposition> {
    vec![
        Disposition {
            operation: "baseline",
            law_kind: "fresh_current_law",
            status: "survivor",
            formula: "current-law random H polygon, conditioned all prescribed inequalities active",
            note: "fresh baseline control; not an independent target sample",
        },
        Disposition {
            operation: "minkowski-sum",
            law_kind: "binary_random",
            status: "survivor",
            formula: "(A+B) normalized by area",
            note: "support addition h_{A+B}(u)=h_A(u)+h_B(u)",
        },
        Disposition {
            operation: "intersection",
            law_kind: "binary_random",
            status: "survivor",
            formula: "A ∩ B from both H-representations",
            note: "active inequalities and output side count retained",
        },
        Disposition {
            operation: "difference-body",
            law_kind: "deterministic_pushforward",
            status: "survivor",
            formula: "A+(-A), normalized",
            note: "deterministic pushforward; not independent breadth",
        },
        Disposition {
            operation: "convex-hull-union",
            law_kind: "binary_random",
            status: "survivor",
            formula: "conv(A ∪ B), normalized",
            note: "active source subset and side count retained",
        },
        Disposition {
            operation: "minkowski-symmetrization",
            law_kind: "deterministic_pushforward",
            status: "survivor",
            formula: "(A + R_u A)/2, normalized",
            note: "classical polygonal Minkowski symmetrization in uniform direction u",
        },
    ]
}

fn git_value(args: &[&str]) -> String {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn producer_revision() -> String {
    git_value(&[
        "log",
        "-1",
        "--format=%H",
        "--",
        PRODUCER_PATH,
        CARGO_MANIFEST_PATH,
    ])
}

fn producer_tree(revision: &str) -> String {
    git_value(&["rev-parse", &format!("{revision}^{{tree}}")])
}

fn blake3_file(path: &str) -> String {
    let bytes = std::fs::read(path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    blake3::hash(&bytes).to_hex().to_string()
}

fn tracked_source_dirty() -> bool {
    !git_value(&["status", "--porcelain=v1", "--untracked-files=no"]).is_empty()
}

const PRODUCER_COMMAND: &str =
    "cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-convex-operations -- --attempts 32 --rows-per-bucket 2";

fn main() {
    let args = parse_args();
    let source_dirty = tracked_source_dirty();
    if source_dirty {
        eprintln!("tracked source is dirty; refusing to produce a retained packet");
        std::process::exit(2);
    }
    let producer_revision = producer_revision();
    let producer_tree = producer_tree(&producer_revision);
    let cargo_lock_blake3 = blake3_file(CARGO_LOCK_PATH);
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("rows.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut writer = BufWriter::new(File::create(rows_path).expect("create rows"));
    let operations = [
        "baseline",
        "minkowski-sum",
        "intersection",
        "difference-body",
        "convex-hull-union",
        "minkowski-symmetrization",
    ];
    let requested_rows = operations.len() * SEEDS.len() * SIDE_COUNTS.len() * args.rows_per_bucket;
    let mut rows = 0;
    let mut status_counts = BTreeMap::new();
    let mut summaries: BTreeMap<String, OperationSummary> = BTreeMap::new();
    let mut side_hist = BTreeMap::new();
    let mut generation_total = 0.0;
    let mut validation_total = 0.0;
    for operation_name in operations {
        let summary = summaries.entry(operation_name.to_string()).or_default();
        summary.requested = SEEDS.len() * SIDE_COUNTS.len() * args.rows_per_bucket;
        for &seed in SEEDS {
            for &n in SIDE_COUNTS {
                for row in 0..args.rows_per_bucket {
                    let mut result = None;
                    let mut attempts_used = 0;
                    for attempt in 0..args.attempts {
                        attempts_used = attempt + 1;
                        let t = std::time::Instant::now();
                        let generated = operation(operation_name, n, seed, row, attempt);
                        generation_total += t.elapsed().as_secs_f64() * 1000.0;
                        let Some(generated) = generated else { continue };
                        let t = std::time::Instant::now();
                        let valid = validate_factor(&generated.output);
                        validation_total += t.elapsed().as_secs_f64() * 1000.0;
                        if valid {
                            result =
                                Some(row_for(operation_name, n, seed, row, attempt, generated));
                            break;
                        }
                    }
                    let row_value = result.unwrap_or_else(|| {
                        exhausted_row(operation_name, n, seed, row, args.attempts)
                    });
                    if row_value.accepted {
                        summary.accepted += 1;
                        if let Some(s) = row_value.output_side_count {
                            *summary.output_side_counts.entry(s.to_string()).or_default() += 1;
                            *side_hist.entry(s.to_string()).or_default() += 1;
                        }
                    } else {
                        summary.exhausted += 1;
                    }
                    summary.total_attempts += attempts_used;
                    *status_counts.entry(row_value.status.clone()).or_default() += 1;
                    serde_json::to_writer(&mut writer, &row_value).expect("write row");
                    writer.write_all(b"\n").expect("newline");
                    rows += 1;
                }
            }
        }
    }
    writer.flush().expect("flush rows");
    println!(
        "volatile timing (not retained): generation_ms={generation_total:.6}, validation_ms={validation_total:.6}"
    );
    let output_rows = args.out_dir.join("rows.jsonl");
    let output_rows_blake3 = blake3_file(output_rows.to_str().expect("rows path UTF-8"));
    let report = Report {
        schema: REPORT_SCHEMA,
        law_version: LAW_VERSION,
        seed_panel: SEEDS.to_vec(),
        side_counts: SIDE_COUNTS.to_vec(),
        rows_per_bucket: args.rows_per_bucket,
        max_attempts_per_row: args.attempts,
        requested_rows,
        rows,
        status_counts,
        operation_summary: summaries,
        side_count_histogram: side_hist,
        dispositions: dispositions(),
        command: PRODUCER_COMMAND,
        producer_revision,
        producer_tree,
        producer_paths: vec![PRODUCER_PATH, CARGO_MANIFEST_PATH],
        cargo_lock_blake3,
        input_hashes: BTreeMap::new(),
        output_rows_blake3,
        output_rows_count: rows,
        source_dirty,
        interpretation_boundary: "Target-free geometry smoke only. Rows expose operation lineage, side counts, active subsets, common shape views, and source-coordinate directed overlaps; no population ranking, sys/capacity, transfer, quotient-distance, or independence claim is supported.",
        abandoned: vec![Abandonment {
            operation: "crofton-poisson-cell",
            status: "abandoned",
            reason: "faithful stationary line process plus finite-window/side-count conditioning was not available within this bounded planar geometry owner",
        }],
    };
    serde_json::to_writer_pretty(File::create(report_path).expect("create report"), &report)
        .expect("write report");
    let terminal_rows = rows == requested_rows
        && report.output_rows_count == report.requested_rows
        && report
            .status_counts
            .keys()
            .all(|status| status == "survived" || status == "exhausted");
    if !terminal_rows {
        eprintln!("requested row contract failed; refusing successful exit");
        std::process::exit(3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rectangle(x: f64, y: f64) -> Factor {
        centered_normalized(vec![
            Vector2::new(-x, -y),
            Vector2::new(x, -y),
            Vector2::new(x, y),
            Vector2::new(-x, y),
        ])
        .unwrap()
    }

    fn asymmetric_triangle() -> Factor {
        centered_normalized(vec![
            Vector2::new(-0.8, -0.2),
            Vector2::new(1.3, -0.1),
            Vector2::new(-0.15, 1.1),
        ])
        .unwrap()
    }

    fn support(f: &Factor, u: Vector2<f64>) -> f64 {
        f.vertices
            .iter()
            .map(|p| u.dot(p))
            .fold(f64::NEG_INFINITY, f64::max)
    }

    #[test]
    fn minkowski_support_addition_fixture() {
        let a = rectangle(1.0, 0.5);
        let b = rectangle(0.25, 0.75);
        let raw = convex_hull(minkowski_raw(&a, &b));
        let f = factor_from_vertices(raw).unwrap();
        for u in [
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(1.0, 1.0).normalize(),
        ] {
            assert!((support(&f, u) - support(&a, u) - support(&b, u)).abs() < 1e-8);
        }
    }

    #[test]
    fn intersection_halfspace_fixture() {
        let a = rectangle(1.0, 1.0);
        let b = rectangle(0.5, 2.0);
        let raw = intersection_raw(&a, &b).unwrap();
        let f = factor_from_vertices(raw).unwrap();
        assert!((shoelace(&f.vertices).abs() - 0.5).abs() < 1e-8);
        assert!(f
            .vertices
            .iter()
            .all(|p| p[0].abs() <= 0.5 + 1e-8 && p[1].abs() <= 1.0 + 1e-8));
    }

    #[test]
    fn difference_body_is_centrally_symmetric() {
        let a = asymmetric_triangle();
        let neg = centered_normalized(a.vertices.iter().map(|p| -*p).collect()).unwrap();
        let d = minkowski_sum(&a, &neg).unwrap();
        for p in &d.vertices {
            assert!(d.vertices.iter().any(|q| (*q + *p).norm() < 1e-8));
        }
    }

    #[test]
    fn minkowski_symmetrization_is_reflection_symmetric_not_generically_central() {
        let a = asymmetric_triangle();
        let theta = 0.37;
        let reflected = reflected(&a, theta).unwrap();
        let raw = factor_from_vertices(minkowski_raw(&a, &reflected)).unwrap();
        let s = centered_normalized(raw.vertices).unwrap();
        assert!(reflection_symmetry_residual(&s, theta) < 1e-8);
        assert!(central_symmetry_residual(&s) > 1e-4);
    }

    #[test]
    fn canonical_shape_linkage_has_stable_ids_and_support_grid() {
        let generated = operation("minkowski-sum", 3, 20260715, 0, 0).unwrap();
        assert_eq!(generated.source_ids.len(), generated.sources.len());
        assert_eq!(
            generated.source_rotation_angles_rad.len(),
            generated.sources.len()
        );
        assert_eq!(
            support_signature_64(&generated.output).len(),
            SUPPORT_GRID_SIZE
        );
        assert!(generated
            .source_ids
            .iter()
            .all(|id| id.contains("/source=")));
        assert!(generated
            .output
            .vertices
            .iter()
            .all(|p| p[0].is_finite() && p[1].is_finite()));
    }

    #[test]
    fn hull_contains_both_inputs() {
        let a = rectangle(1.0, 0.5);
        let b = rectangle(0.25, 0.75);
        let (h, active) = hull_union(&a, &b).unwrap();
        assert!(active >= 4);
        assert!((shoelace(&h.vertices).abs() - 1.0).abs() < 1e-8);
        let raw_hull =
            factor_from_vertices(a.vertices.iter().chain(&b.vertices).copied().collect()).unwrap();
        for p in a.vertices.iter().chain(&b.vertices) {
            assert!(raw_hull
                .normals
                .iter()
                .zip(&raw_hull.heights)
                .all(|(n, x)| n.dot(p) <= *x + 1e-8));
        }
    }

    #[test]
    fn current_law_replay_is_deterministic() {
        let seed = seed_bytes(7, "baseline", 4, 0, 0, "a");
        let mut a = ChaCha8Rng::from_seed(seed);
        let mut b = ChaCha8Rng::from_seed(seed);
        let x = current_factor(4, &mut a);
        let y = current_factor(4, &mut b);
        assert_eq!(x.map(|f| f.vertices), y.map(|f| f.vertices));
    }

    #[test]
    fn strict_validation_rejects_collinear_cycle() {
        let bad = Factor {
            vertices: vec![
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(2.0, 0.0),
            ],
            normals: vec![],
            heights: vec![],
        };
        assert!(!validate_factor(&bad));
    }
}
