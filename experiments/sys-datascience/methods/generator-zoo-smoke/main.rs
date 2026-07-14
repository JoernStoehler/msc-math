//! Breadth-first smoke for explicit planar factor laws.
//!
//! This is deliberately a local experiment owner.  It emits shape-only factor
//! rows and validates a tiny product sample at the existing exact product
//! boundary; it does not evaluate `sys` or make a transfer claim.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Gamma, Normal};
use serde::Serialize;
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

const LAW_VERSION: &str = "generator-zoo-v1";
const DEFAULT_SEED: u64 = 20260714;
const DEFAULT_ATTEMPTS: usize = 64;
const PAIRS: &[(usize, usize)] = &[(3, 3), (4, 6), (6, 6)];

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
    attempts: usize,
    rows_per_law: usize,
    only_law: Option<String>,
}

#[derive(Clone, Debug)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
    vertices: Vec<Vector2<f64>>,
}

#[derive(Serialize)]
struct FactorShapeRow {
    schema: &'static str,
    sample_id: String,
    law: String,
    population: String,
    law_version: &'static str,
    parameter: String,
    seed: u64,
    row_index: usize,
    attempt: usize,
    pair_bucket: String,
    factor_role: &'static str,
    side_count: usize,
    area_normalized: bool,
    vertices_ccw: Vec<[f64; 2]>,
}

#[derive(Serialize)]
struct ProductSmokeRow {
    schema: &'static str,
    sample_id: String,
    law: String,
    law_version: &'static str,
    parameter: String,
    seed: u64,
    row_index: usize,
    attempt: usize,
    pair_bucket: String,
    accepted: bool,
    q_side_count: usize,
    p_side_count: usize,
    q_area: Option<f64>,
    p_area: Option<f64>,
    product_volume: Option<f64>,
    generation_ms: f64,
    validation_ms: f64,
    rejection_reason: Option<String>,
}

#[derive(Serialize)]
struct LawSummary {
    law: String,
    parameter: String,
    rows: usize,
    accepted: usize,
    exhausted: usize,
    total_generation_ms: f64,
    total_validation_ms: f64,
    max_attempts_observed: usize,
}

#[derive(Serialize)]
struct Disposition {
    law: &'static str,
    status: &'static str,
    formula: &'static str,
    note: &'static str,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    law_version: &'static str,
    seed: u64,
    max_attempts_per_row: usize,
    rows_per_law: usize,
    product_rows: usize,
    factor_rows: usize,
    status_counts: BTreeMap<String, usize>,
    per_law: Vec<LawSummary>,
    dispositions: Vec<Disposition>,
    command: String,
    source_revision: String,
    source_dirty: bool,
    interpretation_boundary: &'static str,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from("experiments/sys-datascience/methods/generator-zoo-smoke/artifacts"),
        seed: DEFAULT_SEED,
        attempts: DEFAULT_ATTEMPTS,
        rows_per_law: 1,
        only_law: None,
    };
    let mut i = 1;
    while i < argv.len() {
        let next = |flag: &str| {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} needs a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(next("--out-dir"));
                i += 2;
            }
            "--seed" => {
                args.seed = next("--seed").parse().expect("seed must be u64");
                i += 2;
            }
            "--attempts" => {
                args.attempts = next("--attempts").parse().expect("attempts must be usize");
                i += 2;
            }
            "--rows-per-law" => {
                args.rows_per_law = next("--rows-per-law").parse().expect("rows must be usize");
                i += 2;
            }
            "--only-law" => {
                args.only_law = Some(next("--only-law").to_string());
                i += 2;
            }
            "--help" | "-h" => {
                println!("--out-dir DIR --seed N --attempts N --rows-per-law N [--only-law LAW]");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.rows_per_law > 0);
    args
}

fn law_seed(
    seed: u64,
    law: &str,
    parameter: &str,
    bucket: (usize, usize),
    row: usize,
    attempt: usize,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&seed.to_le_bytes());
    for text in [law, parameter] {
        bytes.extend_from_slice(text.as_bytes());
        bytes.push(0);
    }
    bytes.extend_from_slice(&(bucket.0 as u64).to_le_bytes());
    bytes.extend_from_slice(&(bucket.1 as u64).to_le_bytes());
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

/// Convert a CCW vertex cycle to the outward-unit-normal H representation.
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
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        let edge = b - a;
        let length = edge.norm();
        if !length.is_finite() || length <= 1e-12 {
            return None;
        }
        let normal = Vector2::new(edge[1] / length, -edge[0] / length);
        let height = normal.dot(&a);
        if !height.is_finite() || height <= 1e-12 {
            return None;
        }
        normals.push(normal);
        heights.push(height);
    }
    // Every prescribed edge must remain on the boundary of the intersection;
    // this is the local all-active-facet conditioning witness.
    for vertex in &vertices {
        if normals
            .iter()
            .zip(&heights)
            .any(|(normal, height)| normal.dot(vertex) > *height + 1e-9)
        {
            return None;
        }
    }
    Some(Factor {
        normals,
        heights,
        vertices,
    })
}

fn normalize(mut factor: Factor) -> Option<Factor> {
    let area = shoelace(&factor.vertices).abs();
    if !area.is_finite() || area <= 1e-12 {
        return None;
    }
    let scale = area.sqrt().recip();
    for point in &mut factor.vertices {
        *point *= scale;
    }
    for height in &mut factor.heights {
        *height *= scale;
    }
    let normalized = shoelace(&factor.vertices).abs();
    if (normalized - 1.0).abs() > 1e-8 {
        return None;
    }
    Some(factor)
}

fn random_angles(n: usize, rng: &mut ChaCha8Rng, period: f64) -> Vec<f64> {
    let mut angles: Vec<_> = (0..n).map(|_| rng.gen::<f64>() * period).collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles
}

/// Copy of the current random-factor law: IID normal angles and IID supports
/// in `[1-delta, 1+delta)`, conditioned on all prescribed facets being active.
fn current_baseline(n: usize, delta: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let angles = random_angles(n, rng, TAU);
    let heights = (0..n)
        .map(|_| 1.0 + delta * (2.0 * rng.gen::<f64>() - 1.0))
        .collect();
    angle_factor(&angles, heights)
}

fn angle_factor(angles: &[f64], heights: Vec<f64>) -> Option<Factor> {
    let vertices = (0..angles.len())
        .map(|i| {
            let j = (i + 1) % angles.len();
            let ni = Vector2::new(angles[i].cos(), angles[i].sin());
            let nj = Vector2::new(angles[j].cos(), angles[j].sin());
            let det = ni[0] * nj[1] - ni[1] * nj[0];
            Vector2::new(
                (heights[i] * nj[1] - heights[j] * ni[1]) / det,
                (ni[0] * heights[j] - nj[0] * heights[i]) / det,
            )
        })
        .collect();
    from_vertices(vertices)
}

/// Zonogon: `sum_j [-ell_j v_j, ell_j v_j]`, represented by its edge walk.
fn zonogon(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    if n < 4 || n % 2 != 0 {
        return None;
    }
    let r = n / 2;
    let angles = random_angles(r, rng, PI);
    let lengths: Vec<f64> = (0..r).map(|_| 0.5 + rng.gen::<f64>()).collect();
    let mut edges = Vec::with_capacity(n);
    let mut start = Vector2::new(0.0, 0.0);
    for (&angle, &length) in angles.iter().zip(&lengths) {
        let v = Vector2::new(angle.cos(), angle.sin());
        start -= length * v;
        edges.push((angle, 2.0 * length * v));
        edges.push((angle + PI, -2.0 * length * v));
    }
    edges.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    let mut vertices = Vec::with_capacity(n);
    let mut point = start;
    for (_, edge) in edges {
        vertices.push(point);
        point += edge;
    }
    from_vertices(vertices)
}

fn cross(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>) -> f64 {
    let ab = b - a;
    let ac = c - a;
    ab[0] * ac[1] - ab[1] * ac[0]
}

fn convex_hull(mut points: Vec<Vector2<f64>>) -> Vec<Vector2<f64>> {
    points.sort_by(|a, b| {
        a[0].partial_cmp(&b[0])
            .unwrap()
            .then(a[1].partial_cmp(&b[1]).unwrap())
    });
    points.dedup_by(|a, b| (*a - *b).norm() < 1e-14);
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    for point in &points {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], *point) <= 1e-12
        {
            lower.pop();
        }
        lower.push(*point);
    }
    let mut upper = Vec::new();
    for point in points.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], *point) <= 1e-12
        {
            upper.pop();
        }
        upper.push(*point);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn origin_interior(vertices: &[Vector2<f64>]) -> bool {
    vertices.iter().enumerate().all(|(i, a)| {
        let b = vertices[(i + 1) % vertices.len()];
        cross(*a, b, Vector2::new(0.0, 0.0)) > 1e-10
    })
}

/// IID points uniformly distributed in the unit disk, conditioned on exactly
/// `n` hull vertices and strict origin/interior containment.  The small named
/// sample size keeps this breadth smoke's bounded rejection practical.
fn primal_hull(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let points = (0..(n + 4))
        .map(|_| {
            let radius = rng.gen::<f64>().sqrt();
            let angle = rng.gen::<f64>() * TAU;
            Vector2::new(radius * angle.cos(), radius * angle.sin())
        })
        .collect();
    let hull = convex_hull(points);
    if hull.len() != n || !origin_interior(&hull) {
        return None;
    }
    from_vertices(hull)
}

fn dirichlet_gaps(n: usize, alpha: f64, rng: &mut ChaCha8Rng) -> Option<Vec<f64>> {
    let gamma = Gamma::new(alpha, 1.0).ok()?;
    let mut values: Vec<f64> = (0..n).map(|_| gamma.sample(rng)).collect();
    let sum: f64 = values.iter().sum();
    for value in &mut values {
        *value *= TAU / sum;
    }
    if values.iter().any(|gap| *gap >= PI) {
        return None;
    }
    Some(values)
}

/// Named approximation: a Dirichlet gap law, not a circular-beta/CUE sample.
fn repulsive_gap(n: usize, parameter: &str, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let (angles, heights) = if parameter == "regular" {
        let rotation = rng.gen::<f64>() * TAU;
        (
            (0..n)
                .map(|i| rotation + TAU * i as f64 / n as f64)
                .collect(),
            vec![1.0; n],
        )
    } else {
        let alpha: f64 = parameter
            .strip_prefix("alpha=")
            .unwrap_or(parameter)
            .parse()
            .ok()?;
        let gaps = dirichlet_gaps(n, alpha, rng)?;
        let mut angles = Vec::with_capacity(n);
        let mut theta = rng.gen::<f64>() * TAU;
        for gap in gaps {
            angles.push(theta);
            theta += gap;
        }
        (angles, vec![1.0; n])
    };
    angle_factor(&angles, heights)
}

/// Controlled mutation chain: start regular, then make bounded angular and
/// support mutations at each named step, rejecting order/active-facet failure.
fn regular_mutation(n: usize, steps: usize, scale: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let step = TAU / n as f64;
    let normal = Normal::new(0.0, scale).ok()?;
    let rotation = rng.gen::<f64>() * TAU;
    let mut angles: Vec<f64> = (0..n).map(|i| rotation + i as f64 * step).collect();
    let mut heights = vec![1.0; n];
    for _ in 0..steps {
        for angle in &mut angles {
            *angle += normal.sample(rng).clamp(-0.2 * step, 0.2 * step);
        }
        for height in &mut heights {
            *height *= (normal.sample(rng) * 0.5).exp();
        }
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let gaps: Vec<f64> = (0..n)
            .map(|i| {
                let next = if i + 1 == n {
                    angles[0] + TAU
                } else {
                    angles[i + 1]
                };
                next - angles[i]
            })
            .collect();
        if gaps.iter().any(|gap| *gap < 0.2 * step || *gap >= PI) {
            return None;
        }
    }
    angle_factor(&angles, heights)
}

fn generate(law: &str, parameter: &str, n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    match law {
        "current-baseline" => {
            current_baseline(n, parameter.strip_prefix("delta=")?.parse().ok()?, rng)
        }
        "zonogon" => zonogon(n, rng),
        "primal-hull-uniform-disk" => primal_hull(n, rng),
        "repulsive-gap" => repulsive_gap(n, parameter, rng),
        "regular-mutation" => {
            let (steps, scale) = parameter.strip_prefix("steps=")?.split_once(",scale=")?;
            regular_mutation(n, steps.parse().ok()?, scale.parse().ok()?, rng)
        }
        _ => None,
    }
}

fn factor_shape_row(
    law: &str,
    parameter: &str,
    seed: u64,
    row: usize,
    attempt: usize,
    bucket: (usize, usize),
    role: &'static str,
    factor: &Factor,
) -> FactorShapeRow {
    FactorShapeRow {
        schema: "factor-shape-row-v1",
        sample_id: format!("generator-zoo-v1/{law}/param={parameter}/seed={seed}/row={row}/attempt={attempt}/{}x{}/factor={role}", bucket.0, bucket.1),
        law: law.to_string(), population: format!("{law}[{parameter}]"), law_version: LAW_VERSION, parameter: parameter.to_string(), seed,
        row_index: row, attempt, pair_bucket: format!("{}x{}", bucket.0, bucket.1), factor_role: role,
        side_count: factor.vertices.len(), area_normalized: (shoelace(&factor.vertices).abs() - 1.0).abs() < 1e-8,
        vertices_ccw: factor.vertices.iter().map(|v| [v[0], v[1]]).collect(),
    }
}

fn validate_product(q: &Factor, p: &Factor) -> Option<f64> {
    let poly = SysLandscapePolytopeCache::from_lagrangian_product(
        &q.normals, &q.heights, &p.normals, &p.heights,
    )?;
    Some(exact_volume_from_incidence_as_f64(
        &poly.vertices,
        &poly.vertex_facet_incidence,
    ))
}

fn dispositions() -> Vec<Disposition> {
    vec![
        Disposition { law: "current-baseline", status: "implemented-control", formula: "theta_i iid U(0,2pi), h_i iid U(1-delta,1+delta)", note: "condition on the resulting n-gon retaining all prescribed facets; delta=0.2 matches the current random-factor law" },
        Disposition { law: "zonogon", status: "implemented", formula: "sum_j [-ell_j v_j, ell_j v_j]", note: "even side counts only; distinct unoriented directions and positive uniform lengths" },
        Disposition { law: "primal-hull-uniform-disk", status: "implemented", formula: "conv{X_1,...,X_N}, X_i iid uniform disk", note: "accept exactly n hull vertices with strict origin/interior; area-normalize" },
        Disposition { law: "repulsive-gap", status: "implemented", formula: "g/(sum g) with g_i iid Gamma(alpha,1), theta cumulative", note: "named Dirichlet repulsive-gap approximation; alpha=1 IID control and regular control, not circular beta" },
        Disposition { law: "regular-mutation", status: "implemented", formula: "bounded mutation chain from regular fan", note: "each step clips angular perturbations and checks cyclic gaps/active facets" },
        Disposition { law: "surface-area closure", status: "abandoned", formula: "sum ell_i u_i=0", note: "not attempted: a faithful edge-measure sampler and closure conditioning would exceed this local owner" },
    ]
}

fn main() {
    let args = parse_args();
    create_dir_all(&args.out_dir).expect("create output directory");
    let shape_path = args.out_dir.join("factor-shapes.jsonl");
    let product_path = args.out_dir.join("product-smoke.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut shapes = BufWriter::new(File::create(&shape_path).expect("create shapes"));
    let mut products = BufWriter::new(File::create(&product_path).expect("create products"));
    let jobs: &[(&str, &[&str])] = &[
        ("current-baseline", &["delta=0.2"]),
        ("zonogon", &["lengths=uniform(0.5,1.5)"]),
        ("primal-hull-uniform-disk", &["points=n+4,origin=interior"]),
        (
            "repulsive-gap",
            &["alpha=1", "alpha=4", "alpha=16", "regular"],
        ),
        ("regular-mutation", &["steps=4,scale=0.03"]),
    ];
    let mut summaries: BTreeMap<(String, String), LawSummary> = BTreeMap::new();
    let mut status_counts = BTreeMap::new();
    let mut product_rows = 0;
    let mut factor_rows = 0;
    for &(law, parameters) in jobs {
        if args.only_law.as_deref().is_some_and(|wanted| wanted != law) {
            continue;
        }
        for &parameter in parameters {
            for &bucket in PAIRS {
                if law == "zonogon" && (bucket.0 % 2 != 0 || bucket.1 % 2 != 0) {
                    continue;
                }
                for row in 0..args.rows_per_law {
                    let mut accepted = None;
                    let mut generation_ms = 0.0;
                    let mut validation_ms = 0.0;
                    for attempt in 0..args.attempts {
                        let seed = law_seed(args.seed, law, parameter, bucket, row, attempt);
                        let mut rng = ChaCha8Rng::from_seed(seed);
                        let started = Instant::now();
                        let generated =
                            generate(law, parameter, bucket.0, &mut rng).and_then(|q| {
                                let p = generate(law, parameter, bucket.1, &mut rng)?;
                                Some((normalize(q)?, normalize(p)?))
                            });
                        generation_ms += started.elapsed().as_secs_f64() * 1000.0;
                        let Some((q, p)) = generated else {
                            continue;
                        };
                        let validate_start = Instant::now();
                        let product_volume = validate_product(&q, &p);
                        validation_ms += validate_start.elapsed().as_secs_f64() * 1000.0;
                        if product_volume.is_none() {
                            continue;
                        }
                        let sample = format!("generator-zoo-v1/{law}/param={parameter}/seed={}/row={row}/attempt={attempt}/{}x{}", args.seed, bucket.0, bucket.1);
                        for (role, factor) in [("q", &q), ("p", &p)] {
                            serde_json::to_writer(
                                &mut shapes,
                                &factor_shape_row(
                                    law, parameter, args.seed, row, attempt, bucket, role, factor,
                                ),
                            )
                            .unwrap();
                            shapes.write_all(b"\n").unwrap();
                            factor_rows += 1;
                        }
                        let prod = ProductSmokeRow {
                            schema: "generator-zoo-product-row-v1",
                            sample_id: sample,
                            law: law.to_string(),
                            law_version: LAW_VERSION,
                            parameter: parameter.to_string(),
                            seed: args.seed,
                            row_index: row,
                            attempt,
                            pair_bucket: format!("{}x{}", bucket.0, bucket.1),
                            accepted: true,
                            q_side_count: q.vertices.len(),
                            p_side_count: p.vertices.len(),
                            q_area: Some(shoelace(&q.vertices).abs()),
                            p_area: Some(shoelace(&p.vertices).abs()),
                            product_volume,
                            generation_ms,
                            validation_ms,
                            rejection_reason: None,
                        };
                        serde_json::to_writer(&mut products, &prod).unwrap();
                        products.write_all(b"\n").unwrap();
                        product_rows += 1;
                        accepted = Some(attempt);
                        break;
                    }
                    let key = (law.to_string(), parameter.to_string());
                    let summary = summaries.entry(key).or_insert_with(|| LawSummary {
                        law: law.to_string(),
                        parameter: parameter.to_string(),
                        rows: 0,
                        accepted: 0,
                        exhausted: 0,
                        total_generation_ms: 0.0,
                        total_validation_ms: 0.0,
                        max_attempts_observed: 0,
                    });
                    summary.rows += 1;
                    if let Some(attempt) = accepted {
                        summary.accepted += 1;
                        summary.max_attempts_observed =
                            summary.max_attempts_observed.max(attempt + 1);
                        *status_counts.entry("accepted".to_string()).or_insert(0) += 1;
                    } else {
                        summary.exhausted += 1;
                        summary.max_attempts_observed =
                            summary.max_attempts_observed.max(args.attempts);
                        *status_counts.entry("exhausted".to_string()).or_insert(0) += 1;
                        let failure = ProductSmokeRow { schema: "generator-zoo-product-row-v1", sample_id: format!("generator-zoo-v1/{law}/param={parameter}/seed={}/row={row}/outcome=exhausted/{}x{}", args.seed, bucket.0, bucket.1), law: law.to_string(), law_version: LAW_VERSION, parameter: parameter.to_string(), seed: args.seed, row_index: row, attempt: args.attempts - 1, pair_bucket: format!("{}x{}", bucket.0, bucket.1), accepted: false, q_side_count: bucket.0, p_side_count: bucket.1, q_area: None, p_area: None, product_volume: None, generation_ms, validation_ms, rejection_reason: Some(format!("no accepted product in {} bounded attempts", args.attempts)) };
                        serde_json::to_writer(&mut products, &failure).unwrap();
                        products.write_all(b"\n").unwrap();
                        product_rows += 1;
                    }
                    summary.total_generation_ms += generation_ms;
                    summary.total_validation_ms += validation_ms;
                }
            }
        }
    }
    shapes.flush().unwrap();
    products.flush().unwrap();
    let source_revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let source_dirty = match Command::new("git")
        .args([
            "status",
            "--porcelain",
            "--",
            "experiments/sys-datascience/methods/generator-zoo-smoke/main.rs",
        ])
        .output()
    {
        Ok(output) if output.status.success() => !output.stdout.is_empty(),
        _ => true,
    };
    let report = Report {
        schema: "generator-zoo-report-v1",
        law_version: LAW_VERSION,
        seed: args.seed,
        max_attempts_per_row: args.attempts,
        rows_per_law: args.rows_per_law,
        product_rows,
        factor_rows,
        status_counts,
        per_law: summaries.into_values().collect(),
        dispositions: dispositions(),
        command: std::env::args().collect::<Vec<_>>().join(" "),
        source_revision,
        source_dirty,
        interpretation_boundary: "Tiny product smoke validates construction and provenance only; it does not estimate law distributions or establish transfer.",
    };
    serde_json::to_writer_pretty(File::create(report_path).unwrap(), &report).unwrap();
    println!("wrote {factor_rows} factor rows and {product_rows} product rows");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zonogon_is_centrally_symmetric() {
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let f = zonogon(6, &mut rng).unwrap();
        assert_eq!(f.vertices.len(), 6);
        for i in 0..3 {
            assert!((f.vertices[i] + f.vertices[i + 3]).norm() < 1e-10);
        }
    }

    #[test]
    fn current_baseline_retains_active_facets() {
        let mut found = None;
        for seed in 0..512 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            if let Some(factor) = current_baseline(6, 0.2, &mut rng) {
                found = Some(factor);
                break;
            }
        }
        let factor = found.expect("bounded baseline smoke should find an active hexagon");
        assert_eq!(factor.vertices.len(), 6);
        assert!(factor.heights.iter().all(|height| *height > 0.0));
    }

    #[test]
    fn primal_hull_has_requested_sides_and_contains_origin() {
        let mut found = None;
        for seed in 0..512 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            if let Some(f) = primal_hull(4, &mut rng) {
                found = Some(f);
                break;
            }
        }
        let f = found.expect("bounded hull smoke should find a quadrilateral");
        assert_eq!(f.vertices.len(), 4);
        assert!(origin_interior(&f.vertices));
    }

    #[test]
    fn repulsive_gap_has_lower_gap_cv_than_iid_and_regular_limit() {
        fn cv(alpha: &str) -> f64 {
            let mut values = Vec::new();
            for seed in 0..256 {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                if let Some(f) = repulsive_gap(6, alpha, &mut rng) {
                    let gaps: Vec<_> = (0..6)
                        .map(|i| {
                            (f.normals[(i + 1) % 6].y.atan2(f.normals[(i + 1) % 6].x)
                                - f.normals[i].y.atan2(f.normals[i].x))
                            .rem_euclid(TAU)
                        })
                        .collect();
                    let mean = gaps.iter().sum::<f64>() / 6.0;
                    values.push(
                        (gaps.iter().map(|g| (g - mean).powi(2)).sum::<f64>() / 6.0).sqrt() / mean,
                    );
                }
            }
            values.iter().sum::<f64>() / values.len() as f64
        }
        assert!(cv("16") < cv("1"));
        let mut rng = ChaCha8Rng::seed_from_u64(99);
        let regular = repulsive_gap(6, "regular", &mut rng).unwrap();
        let gaps: Vec<_> = (0..6)
            .map(|i| {
                (regular.normals[(i + 1) % 6]
                    .y
                    .atan2(regular.normals[(i + 1) % 6].x)
                    - regular.normals[i].y.atan2(regular.normals[i].x))
                .rem_euclid(TAU)
            })
            .collect();
        assert!(
            gaps.iter()
                .fold(0.0_f64, |m, x| m.max((*x - gaps[0]).abs()))
                < 1e-12
        );
    }

    #[test]
    fn all_normalized_factors_have_unit_area() {
        let mut rng = ChaCha8Rng::seed_from_u64(11);
        let f = normalize(regular_mutation(6, 4, 0.03, &mut rng).unwrap()).unwrap();
        assert!((shoelace(&f.vertices).abs() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn mutation_chain_keeps_the_named_fan_and_positive_supports() {
        let mut rng = ChaCha8Rng::seed_from_u64(12);
        let f = regular_mutation(6, 4, 0.03, &mut rng).unwrap();
        assert_eq!(f.normals.len(), 6);
        assert!(f.heights.iter().all(|height| *height > 0.0));
    }
}
