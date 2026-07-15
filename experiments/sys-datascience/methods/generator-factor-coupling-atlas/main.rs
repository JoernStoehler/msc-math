//! Target-free fixed-marginal Gaussian-copula factor coupling smoke.
//!
//! The two factors use the current IID planar fan law, but their primitive
//! uniforms are coupled by a Gaussian copula.  Angle and support streams are
//! coupled separately so the one-factor construction marginal is unchanged
//! before the all-active/exact-product conditioning boundary.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

const VERSION: &str = "generator-factor-coupling-atlas-v1";
const DEFAULT_SEED: u64 = 20260715;
const RHO_VALUES: &[f64] = &[0.0, 0.5, 0.9, 1.0];
const SIDES: &[usize] = &[4, 6];
const ROTATIONS: &[&str] = &["uniform", "zero", "pi/4"];

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
    attempts: usize,
    rows_per_arm: usize,
}

#[derive(Clone, Debug)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
    vertices: Vec<Vector2<f64>>,
}

#[derive(Clone, Debug)]
struct GeneratedPair {
    q: Factor,
    p: Factor,
    q_angle_u: Vec<f64>,
    p_angle_u: Vec<f64>,
    q_height_u: Vec<f64>,
    p_height_u: Vec<f64>,
}

#[derive(Serialize)]
struct Row {
    schema: &'static str,
    law_version: &'static str,
    sample_id: String,
    pairing_id: String,
    seed: u64,
    row_index: usize,
    attempt: usize,
    attempts: usize,
    side_count: usize,
    rho: f64,
    rotation_population: String,
    accepted: bool,
    status: String,
    rejection_reason: Option<String>,
    q_area: Option<f64>,
    p_area: Option<f64>,
    q_perimeter: Option<f64>,
    p_perimeter: Option<f64>,
    q_width: Option<f64>,
    p_width: Option<f64>,
    q_support_cv: Option<f64>,
    p_support_cv: Option<f64>,
    q_gap_cv: Option<f64>,
    p_gap_cv: Option<f64>,
    width_balance: Option<f64>,
    quotient_distance: Option<f64>,
    product_volume: Option<f64>,
    angle_primitive_corr: Option<f64>,
    height_primitive_corr: Option<f64>,
    primitive_q_mean: Option<f64>,
    primitive_p_mean: Option<f64>,
    primitive_q_range: Option<f64>,
    primitive_p_range: Option<f64>,
    generation_ms: f64,
    validation_ms: f64,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    law_version: &'static str,
    source_revision: String,
    source_tree: String,
    source_dirty: bool,
    source_dirty_scope: &'static str,
    command: String,
    seed: u64,
    seeds: Vec<u64>,
    rho_values: Vec<f64>,
    side_counts: Vec<usize>,
    rotation_populations: Vec<&'static str>,
    rows: usize,
    requested_rows: usize,
    status_counts: BTreeMap<String, usize>,
    attempts_total: usize,
    accepted_rows: usize,
    exhausted_rows: usize,
    mean_generation_ms: f64,
    mean_validation_ms: f64,
    marginal_control: MarginalControl,
    dependence_control: Vec<DependenceControl>,
    endpoint_control: EndpointControl,
    copula_formula: &'static str,
    primitive_marginal_contract: &'static str,
    conditioning_contract: &'static str,
    interpretation_boundary: &'static str,
}

#[derive(Serialize, Default)]
struct MarginalControl {
    retained_rows: usize,
    primitive_values: usize,
    q_mean: Option<f64>,
    p_mean: Option<f64>,
    q_min: Option<f64>,
    q_max: Option<f64>,
    p_min: Option<f64>,
    p_max: Option<f64>,
    note: &'static str,
}

#[derive(Serialize)]
struct DependenceControl {
    rho: f64,
    rotation_population: String,
    pairs: usize,
    angle_corr_mean: Option<f64>,
    height_corr_mean: Option<f64>,
    quotient_distance_mean: Option<f64>,
}

#[derive(Serialize)]
struct EndpointControl {
    rho_one_shared_primitives: bool,
    rho_one_max_quotient_distance: Option<f64>,
    corrupted_coupling_rejected: bool,
    exact_product_incidence_rows: usize,
    note: &'static str,
}

#[derive(Default)]
struct Accumulator {
    rows: usize,
    accepted: usize,
    exhausted: usize,
    attempts_total: usize,
    generation_ms: f64,
    validation_ms: f64,
    q_uniform: Vec<f64>,
    p_uniform: Vec<f64>,
    controls: BTreeMap<(String, u64), (Vec<f64>, Vec<f64>, Vec<f64>)>,
    endpoint_distances: Vec<f64>,
}

fn parse_args() -> Args {
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/generator-factor-coupling-atlas/artifacts",
        ),
        seed: DEFAULT_SEED,
        attempts: 64,
        rows_per_arm: 1,
    };
    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        let val = |flag: &str| {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} needs a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(val("--out-dir"));
                i += 2;
            }
            "--seed" => {
                args.seed = val("--seed").parse().expect("seed must be u64");
                i += 2;
            }
            "--attempts" => {
                args.attempts = val("--attempts").parse().expect("attempts must be usize");
                i += 2;
            }
            "--rows-per-arm" => {
                args.rows_per_arm = val("--rows-per-arm").parse().expect("rows must be usize");
                i += 2;
            }
            "--help" | "-h" => {
                println!("--out-dir DIR --seed N --attempts N --rows-per-arm N");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.rows_per_arm > 0);
    args
}

fn hash_seed(
    seed: u64,
    rho: f64,
    sides: usize,
    rotation: &str,
    row: usize,
    attempt: usize,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&seed.to_le_bytes());
    bytes.extend_from_slice(&rho.to_le_bytes());
    bytes.extend_from_slice(&(sides as u64).to_le_bytes());
    bytes.extend_from_slice(rotation.as_bytes());
    bytes.extend_from_slice(&(row as u64).to_le_bytes());
    bytes.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&bytes).as_bytes()
}

fn sample_id(
    seed: u64,
    rho: f64,
    sides: usize,
    rotation: &str,
    row: usize,
    attempt: usize,
) -> String {
    format!("{VERSION}/rho={rho:.3}/n={sides}/rotation={rotation}/seed={seed}/row={row}/attempt={attempt}")
}

fn pairing_id(seed: u64, rho: f64, sides: usize, rotation: &str, row: usize) -> String {
    format!("{VERSION}/rho={rho:.3}/n={sides}/rotation={rotation}/seed={seed}/row={row}")
}

/// Abramowitz--Stegun normal CDF approximation; only the CDF transform is
/// needed, and endpoint tests use exact stream sharing rather than this error.
fn normal_cdf(x: f64) -> f64 {
    let ax = x.abs();
    let t = 1.0 / (1.0 + 0.2316419 * ax);
    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    let tail = (-(ax * ax) / 2.0).exp() * poly / (2.0 * PI).sqrt();
    (if x >= 0.0 { 1.0 - tail } else { tail }).clamp(f64::MIN_POSITIVE, 1.0 - f64::EPSILON)
}

fn shoelace(v: &[Vector2<f64>]) -> f64 {
    v.iter()
        .enumerate()
        .map(|(i, a)| {
            let b = v[(i + 1) % v.len()];
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
    points.dedup_by(|a, b| (*a - *b).norm() < 1e-12);
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    for p in points.iter().copied() {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 1e-12
        {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper = Vec::new();
    for p in points.iter().rev().copied() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 1e-12
        {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn from_vertices(mut vertices: Vec<Vector2<f64>>) -> Option<Factor> {
    if vertices.len() < 3 {
        return None;
    }
    if shoelace(&vertices) < 0.0 {
        vertices.reverse();
    }
    let area = shoelace(&vertices);
    if !area.is_finite() || area <= 1e-12 {
        return None;
    }
    let mut normals = Vec::with_capacity(vertices.len());
    let mut heights = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        let edge = vertices[(i + 1) % vertices.len()] - vertices[i];
        let len = edge.norm();
        if !len.is_finite() || len <= 1e-12 {
            return None;
        }
        let normal = Vector2::new(edge[1] / len, -edge[0] / len);
        let height = normal.dot(&vertices[i]);
        if !height.is_finite() || height <= 1e-10 {
            return None;
        }
        normals.push(normal);
        heights.push(height);
    }
    if vertices.iter().any(|p| {
        normals
            .iter()
            .zip(&heights)
            .any(|(n, h)| n.dot(p) > *h + 1e-8)
    }) {
        return None;
    }
    Some(Factor {
        normals,
        heights,
        vertices,
    })
}

fn angle_factor(angles: &[f64], heights: &[f64]) -> Option<Factor> {
    if angles.len() < 3 || angles.len() != heights.len() {
        return None;
    }
    let mut vertices = Vec::with_capacity(angles.len());
    for i in 0..angles.len() {
        let j = (i + 1) % angles.len();
        let ni = Vector2::new(angles[i].cos(), angles[i].sin());
        let nj = Vector2::new(angles[j].cos(), angles[j].sin());
        let det = ni[0] * nj[1] - ni[1] * nj[0];
        if det.abs() <= 1e-12 {
            return None;
        }
        vertices.push(Vector2::new(
            (heights[i] * nj[1] - heights[j] * ni[1]) / det,
            (ni[0] * heights[j] - nj[0] * heights[i]) / det,
        ));
    }
    from_vertices(vertices)
}

fn normalize(mut f: Factor) -> Option<Factor> {
    let area = shoelace(&f.vertices).abs();
    if !area.is_finite() || area <= 1e-12 {
        return None;
    }
    let scale = area.sqrt().recip();
    for p in &mut f.vertices {
        *p *= scale;
    }
    for h in &mut f.heights {
        *h *= scale;
    }
    ((shoelace(&f.vertices).abs() - 1.0).abs() <= 1e-8).then_some(f)
}

fn sort_stream(mut angles: Vec<f64>, mut heights: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut ix: Vec<usize> = (0..angles.len()).collect();
    ix.sort_by(|&i, &j| angles[i].total_cmp(&angles[j]));
    let sorted_angles = ix.iter().map(|&i| angles[i]).collect();
    let sorted_heights = ix.iter().map(|&i| heights[i]).collect();
    angles.clear();
    heights.clear();
    (sorted_angles, sorted_heights)
}

fn pearson(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.len() < 2 {
        return None;
    }
    let ma = a.iter().sum::<f64>() / a.len() as f64;
    let mb = b.iter().sum::<f64>() / b.len() as f64;
    let (mut num, mut da, mut db) = (0.0, 0.0, 0.0);
    for (&x, &y) in a.iter().zip(b) {
        let dx = x - ma;
        let dy = y - mb;
        num += dx * dy;
        da += dx * dx;
        db += dy * dy;
    }
    (da > 0.0 && db > 0.0).then_some(num / (da.sqrt() * db.sqrt()))
}

fn mean(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}
fn range(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| {
        xs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - xs.iter().copied().fold(f64::INFINITY, f64::min)
    })
}
fn cv(xs: &[f64]) -> Option<f64> {
    let m = mean(xs)?;
    (m > 0.0)
        .then(|| (xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / xs.len() as f64).sqrt() / m)
}

fn perimeter(f: &Factor) -> f64 {
    (0..f.vertices.len())
        .map(|i| (f.vertices[(i + 1) % f.vertices.len()] - f.vertices[i]).norm())
        .sum()
}

fn diameter(f: &Factor) -> f64 {
    let mut best: f64 = 0.0;
    for (i, a) in f.vertices.iter().enumerate() {
        for b in f.vertices.iter().skip(i + 1) {
            best = best.max((*a - *b).norm());
        }
    }
    best
}

fn factor_features(f: &Factor) -> Option<(f64, f64, f64, f64, f64)> {
    let area = shoelace(&f.vertices).abs();
    let mut angles: Vec<f64> = f
        .normals
        .iter()
        .map(|n| n[1].atan2(n[0]).rem_euclid(TAU))
        .collect();
    angles.sort_by(f64::total_cmp);
    let gaps: Vec<f64> = (0..angles.len())
        .map(|i| {
            angles[(i + 1) % angles.len()] - angles[i]
                + if i + 1 == angles.len() { TAU } else { 0.0 }
        })
        .collect();
    Some((area, perimeter(f), diameter(f), cv(&f.heights)?, cv(&gaps)?))
}

/// Minimize a cyclic shift and a common angle offset.  This removes the
/// independent global rotations while retaining the factor's ordered shape.
fn quotient_distance(q: &Factor, p: &Factor) -> Option<f64> {
    if q.normals.len() != p.normals.len() {
        return None;
    }
    let n = q.normals.len();
    let qa: Vec<f64> = q.normals.iter().map(|u| u[1].atan2(u[0])).collect();
    let pa: Vec<f64> = p.normals.iter().map(|u| u[1].atan2(u[0])).collect();
    let mut best = f64::INFINITY;
    for shift in 0..n {
        let deltas: Vec<f64> = (0..n)
            .map(|i| {
                let d = pa[(i + shift) % n] - qa[i];
                d.sin().atan2(d.cos())
            })
            .collect();
        let s = deltas.iter().map(|d| d.sin()).sum::<f64>();
        let c = deltas.iter().map(|d| d.cos()).sum::<f64>();
        let offset = s.atan2(c);
        let mut err = 0.0;
        for i in 0..n {
            let d = (pa[(i + shift) % n] - qa[i] - offset).sin();
            err += d * d + (p.heights[(i + shift) % n] - q.heights[i]).powi(2);
        }
        best = best.min((err / n as f64).sqrt());
    }
    best.is_finite().then_some(best)
}

fn generate(n: usize, rho: f64, rotation: &str, rng: &mut ChaCha8Rng) -> Option<GeneratedPair> {
    let normal = Normal::new(0.0, 1.0).ok()?;
    let root = (1.0 - rho * rho).max(0.0).sqrt();
    let mut qa = Vec::with_capacity(n);
    let mut pa = Vec::with_capacity(n);
    let mut qh = Vec::with_capacity(n);
    let mut ph = Vec::with_capacity(n);
    for _ in 0..n {
        let z = normal.sample(rng);
        let w = if rho == 1.0 {
            z
        } else {
            rho * z + root * normal.sample(rng)
        };
        qa.push(normal_cdf(z));
        pa.push(normal_cdf(w));
        let z = normal.sample(rng);
        let w = if rho == 1.0 {
            z
        } else {
            rho * z + root * normal.sample(rng)
        };
        qh.push(normal_cdf(z));
        ph.push(normal_cdf(w));
    }
    let base = rng.gen::<f64>() * TAU;
    let rel = match rotation {
        "uniform" => rng.gen::<f64>() * TAU,
        "zero" => 0.0,
        "pi/4" => PI / 4.0,
        _ => return None,
    };
    let qrot = base;
    let prot = (base + rel).rem_euclid(TAU);
    let qangles: Vec<f64> = qa
        .iter()
        .map(|u| (TAU * u + qrot).rem_euclid(TAU))
        .collect();
    let pangles: Vec<f64> = pa
        .iter()
        .map(|u| (TAU * u + prot).rem_euclid(TAU))
        .collect();
    let qheights: Vec<f64> = qh.iter().map(|u| 0.8 + 0.4 * u).collect();
    let pheights: Vec<f64> = ph.iter().map(|u| 0.8 + 0.4 * u).collect();
    let (qangles, qheights) = sort_stream(qangles, qheights);
    let (pangles, pheights) = sort_stream(pangles, pheights);
    let q = normalize(angle_factor(&qangles, &qheights)?)?;
    let p = normalize(angle_factor(&pangles, &pheights)?)?;
    Some(GeneratedPair {
        q,
        p,
        q_angle_u: qa,
        p_angle_u: pa,
        q_height_u: qh,
        p_height_u: ph,
    })
}

fn seed_for(seed: u64, rho: f64, n: usize, rotation: &str, row: usize, attempt: usize) -> [u8; 32] {
    hash_seed(seed, rho, n, rotation, row, attempt)
}

fn evaluate(
    pair: GeneratedPair,
    args: &Args,
    seed: u64,
    rho: f64,
    n: usize,
    rotation: &str,
    row: usize,
    attempt: usize,
    generation_ms: f64,
) -> Row {
    let sample_id = sample_id(seed, rho, n, rotation, row, attempt);
    let pairing_id = pairing_id(seed, rho, n, rotation, row);
    let (qa, pa, qcv, pcv) = (
        factor_features(&pair.q),
        factor_features(&pair.p),
        cv(&pair.q.heights),
        cv(&pair.p.heights),
    );
    let start = Instant::now();
    let poly = SysLandscapePolytopeCache::from_lagrangian_product(
        &pair.q.normals,
        &pair.q.heights,
        &pair.p.normals,
        &pair.p.heights,
    );
    let mut validation_ms = start.elapsed().as_secs_f64() * 1000.0;
    let (volume, valid) = if let Some(poly) = poly {
        let vs = Instant::now();
        let v = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
        validation_ms += vs.elapsed().as_secs_f64() * 1000.0;
        (v, v.is_finite() && v > 0.0)
    } else {
        (f64::NAN, false)
    };
    let angle_corr = pearson(&pair.q_angle_u, &pair.p_angle_u);
    let height_corr = pearson(&pair.q_height_u, &pair.p_height_u);
    let primitive_q = [pair.q_angle_u.as_slice(), pair.q_height_u.as_slice()].concat();
    let primitive_p = [pair.p_angle_u.as_slice(), pair.p_height_u.as_slice()].concat();
    let distance = quotient_distance(&pair.q, &pair.p);
    let width_balance = qa.zip(pa).map(|(a, b)| (a.2 - b.2).abs() / (a.2 + b.2));
    Row {
        schema: "generator-factor-coupling-atlas-row-v1",
        law_version: VERSION,
        sample_id,
        pairing_id,
        seed,
        row_index: row,
        attempt,
        attempts: attempt + 1,
        side_count: n,
        rho,
        rotation_population: rotation.to_owned(),
        accepted: valid,
        status: if valid { "survived" } else { "invalid" }.to_owned(),
        rejection_reason: (!valid)
            .then(|| "exact product reconstruction or volume rejected".to_owned()),
        q_area: qa.map(|x| x.0),
        p_area: pa.map(|x| x.0),
        q_perimeter: qa.map(|x| x.1),
        p_perimeter: pa.map(|x| x.1),
        q_width: qa.map(|x| x.2),
        p_width: pa.map(|x| x.2),
        q_support_cv: qcv,
        p_support_cv: pcv,
        q_gap_cv: qa.map(|x| x.3),
        p_gap_cv: pa.map(|x| x.3),
        width_balance,
        quotient_distance: distance,
        product_volume: valid.then_some(volume),
        angle_primitive_corr: angle_corr,
        height_primitive_corr: height_corr,
        primitive_q_mean: mean(&primitive_q),
        primitive_p_mean: mean(&primitive_p),
        primitive_q_range: range(&primitive_q),
        primitive_p_range: range(&primitive_p),
        generation_ms,
        validation_ms,
    }
}

fn exhausted(seed: u64, rho: f64, n: usize, rotation: &str, row: usize, args: &Args) -> Row {
    Row {
        schema: "generator-factor-coupling-atlas-row-v1",
        law_version: VERSION,
        sample_id: sample_id(seed, rho, n, rotation, row, args.attempts.saturating_sub(1)),
        pairing_id: pairing_id(seed, rho, n, rotation, row),
        seed,
        row_index: row,
        attempt: args.attempts.saturating_sub(1),
        attempts: args.attempts,
        side_count: n,
        rho,
        rotation_population: rotation.to_owned(),
        accepted: false,
        status: "exhausted".to_owned(),
        rejection_reason: Some("bounded attempts exhausted".to_owned()),
        q_area: None,
        p_area: None,
        q_perimeter: None,
        p_perimeter: None,
        q_width: None,
        p_width: None,
        q_support_cv: None,
        p_support_cv: None,
        q_gap_cv: None,
        p_gap_cv: None,
        width_balance: None,
        quotient_distance: None,
        product_volume: None,
        angle_primitive_corr: None,
        height_primitive_corr: None,
        primitive_q_mean: None,
        primitive_p_mean: None,
        primitive_q_range: None,
        primitive_p_range: None,
        generation_ms: 0.0,
        validation_ms: 0.0,
    }
}

fn git_value(args: &[&str]) -> String {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "unknown".into())
}
fn source_dirty() -> bool {
    Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=no"])
        .output()
        .map(|o| !o.status.success() || !o.stdout.is_empty())
        .unwrap_or(true)
}

fn main() {
    let args = parse_args();
    let source_revision = git_value(&["rev-parse", "HEAD"]);
    let source_tree = git_value(&["rev-parse", "HEAD^{tree}"]);
    let dirty = source_dirty();
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("coupling-rows.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut out = BufWriter::new(File::create(rows_path).expect("create rows"));
    let seeds = vec![args.seed, args.seed + 1, args.seed + 2];
    let requested =
        seeds.len() * RHO_VALUES.len() * SIDES.len() * ROTATIONS.len() * args.rows_per_arm;
    let mut acc = Accumulator::default();
    let mut statuses = BTreeMap::new();
    let mut endpoint_shared = true;
    let mut exact_rows = 0usize;
    for &seed in &seeds {
        for &rho in RHO_VALUES {
            for &n in SIDES {
                for &rotation in ROTATIONS {
                    for row in 0..args.rows_per_arm {
                        let mut accepted = None;
                        for attempt in 0..args.attempts {
                            let mut rng = ChaCha8Rng::from_seed(seed_for(
                                seed, rho, n, rotation, row, attempt,
                            ));
                            let st = Instant::now();
                            let pair = generate(n, rho, rotation, &mut rng);
                            let gen_ms = st.elapsed().as_secs_f64() * 1000.0;
                            acc.attempts_total += 1;
                            let Some(pair) = pair else { continue };
                            if rho == 1.0 {
                                endpoint_shared &= pair
                                    .q_angle_u
                                    .iter()
                                    .zip(&pair.p_angle_u)
                                    .all(|(a, b)| (a - b).abs() < 1e-14)
                                    && pair
                                        .q_height_u
                                        .iter()
                                        .zip(&pair.p_height_u)
                                        .all(|(a, b)| (a - b).abs() < 1e-14);
                            }
                            let outrow =
                                evaluate(pair, &args, seed, rho, n, rotation, row, attempt, gen_ms);
                            acc.generation_ms += outrow.generation_ms;
                            acc.validation_ms += outrow.validation_ms;
                            if outrow.accepted {
                                exact_rows += 1;
                                accepted = Some(outrow);
                                break;
                            }
                        }
                        let outrow = accepted
                            .unwrap_or_else(|| exhausted(seed, rho, n, rotation, row, &args));
                        acc.rows += 1;
                        if outrow.accepted {
                            acc.accepted += 1;
                        } else {
                            acc.exhausted += 1;
                        }
                        *statuses.entry(outrow.status.clone()).or_insert(0) += 1;
                        if outrow.rho == 1.0 {
                            if let Some(d) = outrow.quotient_distance {
                                acc.endpoint_distances.push(d);
                            }
                        }
                        if let (Some(a), Some(b)) =
                            (outrow.angle_primitive_corr, outrow.height_primitive_corr)
                        {
                            let key = (
                                outrow.rotation_population.clone(),
                                (outrow.rho * 1000.0) as u64,
                            );
                            let e = acc.controls.entry(key).or_default();
                            e.0.push(a);
                            e.1.push(b);
                            if let Some(d) = outrow.quotient_distance {
                                e.2.push(d);
                            }
                        }
                        if outrow.accepted {
                            if let (Some(q), Some(p)) =
                                (outrow.primitive_q_mean, outrow.primitive_p_mean)
                            {
                                acc.q_uniform.push(q);
                                acc.p_uniform.push(p);
                            }
                        }
                        serde_json::to_writer(&mut out, &outrow).expect("write row");
                        out.write_all(b"\n").expect("newline");
                    }
                }
            }
        }
    }
    out.flush().expect("flush rows");
    let marginal=MarginalControl { retained_rows:acc.accepted, primitive_values:acc.q_uniform.len()*2, q_mean:mean(&acc.q_uniform),p_mean:mean(&acc.p_uniform),q_min:acc.q_uniform.iter().copied().reduce(f64::min),q_max:acc.q_uniform.iter().copied().reduce(f64::max),p_min:acc.p_uniform.iter().copied().reduce(f64::min),p_max:acc.p_uniform.iter().copied().reduce(f64::max),note:"finite retained-row diagnostic only; pre-conditioning uniformity is by construction, post-conditioning equality is not a theorem" };
    let dependence_control = acc
        .controls
        .into_iter()
        .map(|((rotation, rho), (angle, height, d))| {
            let keyrho = rho as f64 / 1000.0;
            DependenceControl {
                rho: keyrho,
                rotation_population: rotation,
                pairs: angle.len(),
                angle_corr_mean: mean(&angle),
                height_corr_mean: mean(&height),
                quotient_distance_mean: mean(&d),
            }
        })
        .collect();
    let corrupted = {
        (0..args.attempts).any(|attempt| {
            let mut rng = ChaCha8Rng::from_seed(seed_for(args.seed, 1.0, 4, "zero", 0, attempt));
            generate(4, 1.0, "zero", &mut rng)
                .map(|mut p| {
                    p.p.heights[0] += 0.02;
                    quotient_distance(&p.q, &p.p).unwrap_or(0.0) > 1e-4
                })
                .unwrap_or(false)
        })
    };
    let report=Report {schema:"generator-factor-coupling-atlas-report-v1",law_version:VERSION,source_revision,source_tree,source_dirty:dirty,source_dirty_scope:"tracked git status captured before output creation",command:std::env::args().collect::<Vec<_>>().join(" "),seed:args.seed,seeds,rho_values:RHO_VALUES.to_vec(),side_counts:SIDES.to_vec(),rotation_populations:ROTATIONS.to_vec(),rows:acc.rows,requested_rows:requested,status_counts:statuses,attempts_total:acc.attempts_total,accepted_rows:acc.accepted,exhausted_rows:acc.exhausted,mean_generation_ms:acc.generation_ms/(acc.rows.max(1) as f64),mean_validation_ms:acc.validation_ms/(acc.rows.max(1) as f64),marginal_control:marginal,dependence_control,endpoint_control:EndpointControl {rho_one_shared_primitives:endpoint_shared,rho_one_max_quotient_distance:mean(&acc.endpoint_distances),corrupted_coupling_rejected:corrupted,exact_product_incidence_rows:exact_rows,note:"endpoint and corruption controls are semantic witnesses, not population tests"},copula_formula:"For each angle and height primitive independently: Z_Q,E ~ iid N(0,1), Z_P = rho Z_Q + sqrt(1-rho^2) E, U_Q=Phi(Z_Q), U_P=Phi(Z_P); rho=1 shares Z exactly.",primitive_marginal_contract:"Before selection, each factor has independent U(0,1) angle primitives and independent U(0,1) height primitives; h=0.8+0.4U and angle=2piU plus a global rotation.",conditioning_contract:"A candidate is retained only after both factor H-reconstructions have all prescribed active facets and exact product incidence/positive volume validates. Gaussian-copula marginal preservation is theorem-by-construction before this selection; conditioning may alter the retained marginal, so only finite diagnostics are reported.",interpretation_boundary:"Target-free construction and geometry evidence only: no sys, exchangeability, rho monotonicity, best-rho choice, or transfer claim."};
    serde_json::to_writer_pretty(File::create(report_path).expect("create report"), &report)
        .expect("write report");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(seed: u64, rho: f64, n: usize, rotation: &str) -> GeneratedPair {
        (0..256)
            .filter_map(|attempt| {
                let mut r = ChaCha8Rng::from_seed(seed_for(seed, rho, n, rotation, 0, attempt));
                generate(n, rho, rotation, &mut r)
            })
            .next()
            .expect("bounded test draw should produce an active fan")
    }

    #[test]
    fn rho_one_shares_both_primitive_streams() {
        let p = draw(1, 1.0, 6, "uniform");
        assert!(p
            .q_angle_u
            .iter()
            .zip(&p.p_angle_u)
            .all(|(a, b)| (a - b).abs() < 1e-14));
        assert!(p
            .q_height_u
            .iter()
            .zip(&p.p_height_u)
            .all(|(a, b)| (a - b).abs() < 1e-14));
    }
    #[test]
    fn rho_zero_is_not_endpoint_copy() {
        let p = draw(1, 0.0, 6, "uniform");
        assert!(p
            .q_angle_u
            .iter()
            .zip(&p.p_angle_u)
            .any(|(a, b)| (a - b).abs() > 1e-5));
    }
    #[test]
    fn fixed_rotation_is_separate_and_valid() {
        let p = draw(2, 0.9, 4, "pi/4");
        assert_eq!(p.q.normals.len(), 4);
        assert!((shoelace(&p.q.vertices).abs() - 1.0).abs() < 1e-8);
    }
    #[test]
    fn corrupted_endpoint_fails_distance_control() {
        let mut p = draw(3, 1.0, 4, "zero");
        assert!(quotient_distance(&p.q, &p.p).unwrap() < 1e-10);
        p.p.heights[0] += 0.05;
        assert!(quotient_distance(&p.q, &p.p).unwrap() > 1e-4);
    }
    #[test]
    fn product_incidence_and_area_validation() {
        let p = draw(4, 0.5, 4, "uniform");
        let poly = SysLandscapePolytopeCache::from_lagrangian_product(
            &p.q.normals,
            &p.q.heights,
            &p.p.normals,
            &p.p.heights,
        )
        .expect("exact product");
        let v = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
        assert!(v.is_finite() && v > 0.0);
    }
}
