//! Target-free smoke for the missing natural planar coupling laws.
//!
//! This owner intentionally implements only the two laws whose mathematical
//! construction was missing from the line branch: a correlated latent law for
//! two equal-sided factors and a centroid-centred polar coupling.  Existing
//! factor laws are named as reused dispositions in the report rather than
//! copied into another producer.  The smoke performs exact product
//! reconstruction and volume validation, but never evaluates `sys`.

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

const LAW_VERSION: &str = "generator-natural-law-expansion-v1";
const DEFAULT_SEED: u64 = 20260715;
const DEFAULT_ATTEMPTS: usize = 128;
const PAIRS: &[(usize, usize)] = &[(3, 3), (4, 4), (6, 6)];

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
struct SmokeRow {
    schema: &'static str,
    sample_id: String,
    pairing_id: Option<String>,
    law: String,
    wishlist_item: u8,
    law_version: &'static str,
    parameter: String,
    seed: u64,
    row_index: usize,
    attempt: usize,
    attempts: usize,
    pair_bucket: String,
    accepted: bool,
    status: String,
    rejection_reason: Option<String>,
    q_area: Option<f64>,
    p_area: Option<f64>,
    q_support_cv: Option<f64>,
    p_support_cv: Option<f64>,
    q_gap_cv: Option<f64>,
    p_gap_cv: Option<f64>,
    product_volume: Option<f64>,
    generation_ms: f64,
    validation_ms: f64,
}

#[derive(Serialize)]
struct Disposition {
    wishlist_item: u8,
    law: &'static str,
    disposition: &'static str,
    evidence: &'static str,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    law_version: &'static str,
    seed: u64,
    max_attempts_per_row: usize,
    rows_per_law: usize,
    rows: usize,
    requested_rows: usize,
    all_requested_rows_terminal: bool,
    status_counts: BTreeMap<String, usize>,
    dispositions: Vec<Disposition>,
    command: String,
    source_revision: String,
    source_tree: String,
    source_dirty: bool,
    source_dirty_scope: &'static str,
    interpretation_boundary: &'static str,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/generator-natural-law-expansion/artifacts",
        ),
        seed: DEFAULT_SEED,
        attempts: DEFAULT_ATTEMPTS,
        rows_per_law: 2,
        only_law: None,
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
            "--seed" => {
                args.seed = value("--seed").parse().expect("seed must be u64");
                i += 2;
            }
            "--attempts" => {
                args.attempts = value("--attempts").parse().expect("attempts must be usize");
                i += 2;
            }
            "--rows-per-law" => {
                args.rows_per_law = value("--rows-per-law").parse().expect("rows must be usize");
                i += 2;
            }
            "--only-law" => {
                args.only_law = Some(value("--only-law").to_owned());
                i += 2;
            }
            "--help" | "-h" => {
                println!("--out-dir DIR --seed N --attempts N --rows-per-law N [--only-law shared-latent|polar-coupled]");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.rows_per_law > 0);
    if let Some(law) = &args.only_law {
        assert!(matches!(law.as_str(), "shared-latent" | "polar-coupled"));
    }
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
    for value in [law, parameter] {
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(0);
    }
    bytes.extend_from_slice(&(bucket.0 as u64).to_le_bytes());
    bytes.extend_from_slice(&(bucket.1 as u64).to_le_bytes());
    bytes.extend_from_slice(&(row as u64).to_le_bytes());
    bytes.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&bytes).as_bytes()
}

fn sample_id(
    law: &str,
    parameter: &str,
    seed: u64,
    row: usize,
    attempt: usize,
    bucket: (usize, usize),
) -> String {
    format!(
        "{LAW_VERSION}/{law}/param={parameter}/seed={seed}/row={row}/attempt={attempt}/{}x{}",
        bucket.0, bucket.1
    )
}

fn pairing_id(
    law: &str,
    parameter: &str,
    seed: u64,
    row: usize,
    attempt: usize,
    n: usize,
) -> String {
    format!(
        "{LAW_VERSION}/{law}/param={parameter}/seed={seed}/row={row}/attempt={attempt}/pair={n}x{n}"
    )
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
    points.dedup_by(|a, b| (*a - *b).norm() < 1e-13);
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    for &point in &points {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 1e-12
        {
            lower.pop();
        }
        lower.push(point);
    }
    let mut upper = Vec::new();
    for &point in points.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 1e-12
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

/// Build the outward-unit-normal H representation from a CCW vertex cycle.
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
        if !height.is_finite() || height <= 1e-10 {
            return None;
        }
        normals.push(normal);
        heights.push(height);
    }
    if vertices.iter().any(|point| {
        normals
            .iter()
            .zip(&heights)
            .any(|(normal, height)| normal.dot(point) > *height + 1e-9)
    }) {
        return None;
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
    if (shoelace(&factor.vertices).abs() - 1.0).abs() > 1e-8 {
        return None;
    }
    Some(factor)
}

fn angle_factor(angles: &[f64], heights: &[f64]) -> Option<Factor> {
    if angles.len() != heights.len() || angles.len() < 3 {
        return None;
    }
    let vertices = (0..angles.len())
        .map(|i| {
            let j = (i + 1) % angles.len();
            let ni = Vector2::new(angles[i].cos(), angles[i].sin());
            let nj = Vector2::new(angles[j].cos(), angles[j].sin());
            let det = ni[0] * nj[1] - ni[1] * nj[0];
            if det.abs() < 1e-12 {
                return None;
            }
            Some(Vector2::new(
                (heights[i] * nj[1] - heights[j] * ni[1]) / det,
                (ni[0] * heights[j] - nj[0] * heights[i]) / det,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    from_vertices(vertices)
}

fn coefficient_of_variation(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    if !mean.is_finite() || mean <= 0.0 {
        return None;
    }
    let variance = values.iter().map(|x| (*x - mean).powi(2)).sum::<f64>() / values.len() as f64;
    Some(variance.sqrt() / mean)
}

fn metrics(factor: &Factor) -> (Option<f64>, Option<f64>) {
    let area = shoelace(&factor.vertices).abs();
    let support_cv = coefficient_of_variation(&factor.heights);
    let angles: Vec<f64> = factor
        .normals
        .iter()
        .map(|u| u[1].atan2(u[0]).rem_euclid(TAU))
        .collect();
    let mut sorted = angles;
    sorted.sort_by(f64::total_cmp);
    let gaps: Vec<f64> = (0..sorted.len())
        .map(|i| {
            sorted[(i + 1) % sorted.len()] - sorted[i]
                + if i + 1 == sorted.len() { TAU } else { 0.0 }
        })
        .collect();
    (Some(area), coefficient_of_variation(&gaps))
}

fn current_baseline(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let mut angles: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * TAU).collect();
    angles.sort_by(f64::total_cmp);
    let heights: Vec<f64> = (0..n).map(|_| 0.8 + 0.4 * rng.gen::<f64>()).collect();
    angle_factor(&angles, &heights)
}

fn centered_gaussian(n: usize, rng: &mut ChaCha8Rng) -> Vec<f64> {
    let normal = Normal::new(0.0, 1.0).expect("unit normal");
    let mut values: Vec<f64> = (0..n).map(|_| normal.sample(rng)).collect();
    let mean = values.iter().sum::<f64>() / n as f64;
    for value in &mut values {
        *value -= mean;
    }
    values
}

fn parse_shared_parameter(parameter: &str) -> Option<(f64, f64)> {
    let mut rho = None;
    let mut sigma = None;
    for field in parameter.split(',') {
        let (key, value) = field.split_once('=')?;
        match key {
            "rho" => rho = Some(value.parse::<f64>().ok()?),
            "sigma" => sigma = Some(value.parse::<f64>().ok()?),
            _ => return None,
        }
    }
    let (rho, sigma) = (rho?, sigma?);
    (rho.is_finite() && (0.0..=1.0).contains(&rho) && sigma.is_finite() && sigma >= 0.0)
        .then_some((rho, sigma))
}

/// Logistic-normal shared latent law.  The same correlated latent Gaussian
/// controls normalized angular gaps and log supports; a common global
/// rotation is a gauge and is therefore shared.  `rho=0` has independent
/// factor shapes, while `rho=1` has identical shape coordinates before the
/// common area normalization.
fn shared_latent(n: usize, parameter: &str, rng: &mut ChaCha8Rng) -> Option<(Factor, Factor)> {
    let (rho, sigma) = parse_shared_parameter(parameter)?;
    let root = (1.0 - rho * rho).sqrt();
    let q_gap = centered_gaussian(n, rng);
    let p_gap_noise = centered_gaussian(n, rng);
    let q_support = centered_gaussian(n, rng);
    let p_support_noise = centered_gaussian(n, rng);
    let p_gap: Vec<f64> = q_gap
        .iter()
        .zip(&p_gap_noise)
        .map(|(q, z)| rho * q + root * z)
        .collect();
    let p_support: Vec<f64> = q_support
        .iter()
        .zip(&p_support_noise)
        .map(|(q, z)| rho * q + root * z)
        .collect();
    let to_angles = |z: &[f64], rotation: f64| {
        let weights: Vec<f64> = z.iter().map(|x| x.exp()).collect();
        let sum = weights.iter().sum::<f64>();
        let mut theta = rotation;
        let mut angles = Vec::with_capacity(n);
        for weight in weights {
            angles.push(theta);
            theta += TAU * weight / sum;
        }
        (angles, theta - rotation)
    };
    let rotation = rng.gen::<f64>() * TAU;
    let (q_angles, q_total) = to_angles(&q_gap, rotation);
    let (p_angles, p_total) = to_angles(&p_gap, rotation);
    if q_total <= 0.0 || p_total <= 0.0 {
        return None;
    }
    let q_heights: Vec<f64> = q_support.iter().map(|z| (sigma * z).exp()).collect();
    let p_heights: Vec<f64> = p_support.iter().map(|z| (sigma * z).exp()).collect();
    let q = normalize(angle_factor(&q_angles, &q_heights)?)?;
    let p = normalize(angle_factor(&p_angles, &p_heights)?)?;
    Some((q, p))
}

fn polygon_centroid(vertices: &[Vector2<f64>]) -> Option<Vector2<f64>> {
    let mut twice_area = 0.0;
    let mut sum = Vector2::new(0.0, 0.0);
    for i in 0..vertices.len() {
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        let cross = a[0] * b[1] - b[0] * a[1];
        twice_area += cross;
        sum += (a + b) * cross;
    }
    if twice_area.abs() <= 1e-12 {
        return None;
    }
    Some(sum / (3.0 * twice_area))
}

/// Origin-polar helper used only by the mathematical calibration test.  The
/// production polar law uses `centroid` explicitly; this helper makes the
/// standard `(K^o)^o=K` incidence witness visible without changing that law.
fn origin_polar(factor: &Factor) -> Option<Factor> {
    let vertices = factor
        .normals
        .iter()
        .zip(&factor.heights)
        .map(|(normal, height)| *normal / *height)
        .collect::<Vec<_>>();
    from_vertices(convex_hull(vertices))
}

/// Polar coupling about the area centroid.  For `Q-c`, the polar has vertices
/// `u_i/(h_i-u_i·c)`.  This is translation-aware; raw-origin polarity is not
/// used.  The relative rotation is explicit and is not a common gauge.
fn polar_coupled(n: usize, parameter: &str, rng: &mut ChaCha8Rng) -> Option<(Factor, Factor)> {
    let phi = match parameter {
        "center=centroid,phi=0" => 0.0,
        "center=centroid,phi=pi/4" => PI / 4.0,
        _ => return None,
    };
    let q = normalize(current_baseline(n, rng)?)?;
    let center = polygon_centroid(&q.vertices)?;
    let mut polar_vertices = Vec::with_capacity(n);
    for (normal, height) in q.normals.iter().zip(&q.heights) {
        let centered_support = *height - normal.dot(&center);
        if !centered_support.is_finite() || centered_support <= 1e-10 {
            return None;
        }
        polar_vertices.push(*normal / centered_support);
    }
    let mut p_vertices = convex_hull(polar_vertices);
    if p_vertices.len() != n {
        return None;
    }
    for vertex in &mut p_vertices {
        let rotated = Vector2::new(
            phi.cos() * vertex[0] - phi.sin() * vertex[1],
            phi.sin() * vertex[0] + phi.cos() * vertex[1],
        );
        *vertex = rotated;
    }
    let p = normalize(from_vertices(p_vertices)?)?;
    Some((q, p))
}

fn make_pair(
    law: &str,
    parameter: &str,
    bucket: (usize, usize),
    rng: &mut ChaCha8Rng,
) -> Option<(Factor, Factor)> {
    if bucket.0 != bucket.1 {
        return None;
    }
    match law {
        "shared-latent" => shared_latent(bucket.0, parameter, rng),
        "polar-coupled" => polar_coupled(bucket.0, parameter, rng),
        _ => None,
    }
}

fn evaluate(
    law: &str,
    item: u8,
    parameter: &str,
    args: &Args,
    bucket: (usize, usize),
    row: usize,
    attempt: usize,
    q: Factor,
    p: Factor,
    generation_ms: f64,
) -> SmokeRow {
    let sample_id = sample_id(law, parameter, args.seed, row, attempt, bucket);
    let pairing = pairing_id(law, parameter, args.seed, row, attempt, bucket.0);
    let (q_area, q_gap_cv) = metrics(&q);
    let (p_area, p_gap_cv) = metrics(&p);
    let q_support_cv = coefficient_of_variation(&q.heights);
    let p_support_cv = coefficient_of_variation(&p.heights);
    let start = Instant::now();
    let poly = SysLandscapePolytopeCache::from_lagrangian_product(
        &q.normals, &q.heights, &p.normals, &p.heights,
    );
    let mut validation_ms = start.elapsed().as_secs_f64() * 1000.0;
    let Some(poly) = poly else {
        return SmokeRow {
            schema: "generator-natural-law-expansion-row-v1",
            sample_id,
            pairing_id: Some(pairing),
            law: law.to_owned(),
            wishlist_item: item,
            law_version: LAW_VERSION,
            parameter: parameter.to_owned(),
            seed: args.seed,
            row_index: row,
            attempt,
            attempts: attempt + 1,
            pair_bucket: format!("{}x{}", bucket.0, bucket.1),
            accepted: false,
            status: "invalid".into(),
            rejection_reason: Some("exact product reconstruction rejected geometry".into()),
            q_area,
            p_area,
            q_support_cv,
            p_support_cv,
            q_gap_cv,
            p_gap_cv,
            product_volume: None,
            generation_ms,
            validation_ms,
        };
    };
    let volume_start = Instant::now();
    let product_volume =
        exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
    validation_ms += volume_start.elapsed().as_secs_f64() * 1000.0;
    let valid_volume = product_volume.is_finite() && product_volume > 0.0;
    SmokeRow {
        schema: "generator-natural-law-expansion-row-v1",
        sample_id,
        pairing_id: Some(pairing),
        law: law.to_owned(),
        wishlist_item: item,
        law_version: LAW_VERSION,
        parameter: parameter.to_owned(),
        seed: args.seed,
        row_index: row,
        attempt,
        attempts: attempt + 1,
        pair_bucket: format!("{}x{}", bucket.0, bucket.1),
        accepted: valid_volume,
        status: if valid_volume { "survived" } else { "invalid" }.into(),
        rejection_reason: (!valid_volume).then(|| "non-positive or non-finite exact volume".into()),
        q_area,
        p_area,
        q_support_cv,
        p_support_cv,
        q_gap_cv,
        p_gap_cv,
        product_volume: valid_volume.then_some(product_volume),
        generation_ms,
        validation_ms,
    }
}

fn dispositions() -> Vec<Disposition> {
    vec![
        Disposition { wishlist_item: 1, law: "fresh baseline", disposition: "already faithful", evidence: "generator-zoo-smoke current-baseline arm" },
        Disposition { wishlist_item: 2, law: "equal-support tangential", disposition: "already faithful", evidence: "alternative-generator-smoke equal-support and factorial arms" },
        Disposition { wishlist_item: 3, law: "support-variance ladder", disposition: "already faithful", evidence: "alternative-generator-smoke log-support ladder" },
        Disposition { wishlist_item: 4, law: "smooth support field", disposition: "already faithful", evidence: "alternative-generator-smoke smooth-support-r2/r3; no translation claim" },
        Disposition { wishlist_item: 5, law: "shape-cell conditional", disposition: "dependency expansion", evidence: "requires matched population design, not a standalone unconditional sampler" },
        Disposition { wishlist_item: 6, law: "one-factor factorial", disposition: "already faithful", evidence: "paired baseline fans and explicit pairing IDs" },
        Disposition { wishlist_item: 7, law: "Dirichlet angular gaps", disposition: "already faithful", evidence: "generator-zoo-smoke repulsive-gap arm" },
        Disposition { wishlist_item: 8, law: "jittered regular", disposition: "already faithful", evidence: "alternative-generator-smoke jittered-regular arm" },
        Disposition { wishlist_item: 9, law: "centrally symmetric strips", disposition: "already faithful", evidence: "alternative-generator-smoke symmetric-strip arms" },
        Disposition { wishlist_item: 10, law: "broken antipodal supports", disposition: "already faithful", evidence: "paired shared lines and preserved strip widths" },
        Disposition { wishlist_item: 11, law: "random zonogon", disposition: "already faithful", evidence: "generator-zoo-smoke zonogon edge-walk arm" },
        Disposition { wishlist_item: 12, law: "congruent factors", disposition: "already faithful", evidence: "alternative-generator-smoke explicit relative rotations" },
        Disposition { wishlist_item: 13, law: "shared-latent factors", disposition: "implemented", evidence: "correlated logistic-normal gaps and centered log supports; rho=0,1 are independent/congruent-shape endpoints" },
        Disposition { wishlist_item: 14, law: "polar coupling", disposition: "implemented", evidence: "polar of Q minus its area centroid, then explicit relative rotation and area normalization" },
        Disposition { wishlist_item: 15, law: "primal IID-point hull", disposition: "already faithful", evidence: "generator-zoo-smoke uniform-disk hull conditioned on side count" },
        Disposition { wishlist_item: 16, law: "inscribed polygon", disposition: "already faithful", evidence: "alternative-generator-smoke circle-hull support formula" },
        Disposition { wishlist_item: 17, law: "Poisson-line/Crofton cell", disposition: "dependency expansion", evidence: "faithful stationary line process plus side-count conditioning needs a finite-window/conditional sampler; no shortcut used" },
        Disposition { wishlist_item: 18, law: "SO(4)/U(2) orientation", disposition: "dependency expansion", evidence: "orbit intervention explicitly outside this planar owner" },
        Disposition { wishlist_item: 19, law: "quotient-transverse perturbation", disposition: "dependency expansion", evidence: "orbit intervention explicitly outside this planar owner" },
        Disposition { wishlist_item: 20, law: "generic centrally symmetric 4-polytopes", disposition: "dependency expansion", evidence: "generic 4D exact reconstruction owner required" },
        Disposition { wishlist_item: 21, law: "SL(4) structured images", disposition: "dependency expansion", evidence: "generic 4D image law and quotient measure are outside this owner" },
    ]
}

fn git_revision() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "unknown".into())
}

fn git_tree() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD^{tree}"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "unknown".into())
}

fn tracked_source_dirty_from_status(status: &str) -> bool {
    status.lines().any(|line| {
        let code = line.as_bytes();
        code.len() >= 2 && !line.starts_with("??") && !line.starts_with("!!")
    })
}

/// Capture the whole repository snapshot before creating output files. The
/// status command ignores untracked/ignored files, so a generated artifact
/// cannot make its own report dirty; every tracked transitive dependency does.
fn source_provenance() -> (String, String, bool) {
    let revision = git_revision();
    let tree = git_tree();
    let dirty = Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=no"])
        .output()
        .map(|output| {
            !output.status.success()
                || tracked_source_dirty_from_status(&String::from_utf8_lossy(&output.stdout))
        })
        .unwrap_or(true);
    (revision, tree, dirty)
}

fn terminal_status(status: &str) -> bool {
    matches!(status, "survived" | "exhausted")
}

fn terminal_contract(
    requested_rows: usize,
    rows: usize,
    status_counts: &BTreeMap<String, usize>,
) -> bool {
    requested_rows > 0
        && rows == requested_rows
        && status_counts
            .iter()
            .all(|(status, count)| *count > 0 && terminal_status(status))
}

fn exhausted_row(
    law: &str,
    item: u8,
    parameter: &str,
    args: &Args,
    bucket: (usize, usize),
    row: usize,
) -> SmokeRow {
    let attempt = args.attempts.saturating_sub(1);
    SmokeRow {
        schema: "generator-natural-law-expansion-row-v1",
        sample_id: sample_id(law, parameter, args.seed, row, attempt, bucket),
        pairing_id: None,
        law: law.to_owned(),
        wishlist_item: item,
        law_version: LAW_VERSION,
        parameter: parameter.to_string(),
        seed: args.seed,
        row_index: row,
        attempt,
        attempts: args.attempts,
        pair_bucket: format!("{}x{}", bucket.0, bucket.1),
        accepted: false,
        status: "exhausted".into(),
        rejection_reason: Some("no exact-valid pair in bounded attempts".into()),
        q_area: None,
        p_area: None,
        q_support_cv: None,
        p_support_cv: None,
        q_gap_cv: None,
        p_gap_cv: None,
        product_volume: None,
        generation_ms: 0.0,
        validation_ms: 0.0,
    }
}

fn main() {
    let args = parse_args();
    // Pin the producer source before creating or overwriting any output.
    let (source_revision, source_tree, source_dirty) = source_provenance();
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("smoke-rows.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut rows_out = BufWriter::new(File::create(&rows_path).expect("create rows"));
    let jobs = [
        (
            "shared-latent",
            13u8,
            ["rho=0,sigma=0.2", "rho=0.5,sigma=0.2", "rho=1,sigma=0.2"],
        ),
        (
            "polar-coupled",
            14u8,
            ["center=centroid,phi=0", "center=centroid,phi=pi/4", ""],
        ),
    ];
    let requested_rows = jobs
        .iter()
        .filter(|(law, _, _)| args.only_law.as_deref().is_none_or(|wanted| wanted == *law))
        .map(|(_, _, parameters)| {
            parameters
                .iter()
                .filter(|parameter| !parameter.is_empty())
                .count()
                * PAIRS.len()
                * args.rows_per_law
        })
        .sum();
    let mut status_counts = BTreeMap::new();
    let mut rows = 0usize;
    for (law, item, parameters) in jobs {
        if args.only_law.as_deref().is_some_and(|wanted| wanted != law) {
            continue;
        }
        for parameter in parameters.iter().filter(|parameter| !parameter.is_empty()) {
            for &bucket in PAIRS {
                for row in 0..args.rows_per_law {
                    let mut accepted = None;
                    for attempt in 0..args.attempts {
                        let mut rng = ChaCha8Rng::from_seed(law_seed(
                            args.seed, law, parameter, bucket, row, attempt,
                        ));
                        let generation_start = Instant::now();
                        let pair = make_pair(law, parameter, bucket, &mut rng);
                        let generation_ms = generation_start.elapsed().as_secs_f64() * 1000.0;
                        let Some((q, p)) = pair else { continue };
                        let result = evaluate(
                            law,
                            item,
                            parameter,
                            &args,
                            bucket,
                            row,
                            attempt,
                            q,
                            p,
                            generation_ms,
                        );
                        if result.accepted {
                            accepted = Some(result);
                            break;
                        }
                    }
                    let result = accepted
                        .unwrap_or_else(|| exhausted_row(law, item, parameter, &args, bucket, row));
                    *status_counts.entry(result.status.clone()).or_insert(0) += 1;
                    serde_json::to_writer(&mut rows_out, &result).expect("write row");
                    rows_out.write_all(b"\n").expect("newline");
                    rows += 1;
                }
            }
        }
    }
    rows_out.flush().expect("flush rows");
    let command = std::env::args().collect::<Vec<_>>().join(" ");
    let all_requested_rows_terminal = terminal_contract(requested_rows, rows, &status_counts);
    let report = Report { schema: "generator-natural-law-expansion-report-v1", law_version: LAW_VERSION, seed: args.seed, max_attempts_per_row: args.attempts, rows_per_law: args.rows_per_law, rows, requested_rows, all_requested_rows_terminal, status_counts, dispositions: dispositions(), command, source_revision, source_tree, source_dirty, source_dirty_scope: "git status --porcelain=v1 --untracked-files=no (repository-wide tracked files; snapshot captured before output creation)", interpretation_boundary: "Exact geometry and finite-volume smoke evidence only; no target, ranking, population estimate, or transfer claim." };
    serde_json::to_writer_pretty(File::create(report_path).expect("create report"), &report)
        .expect("write report");
    if !all_requested_rows_terminal {
        eprintln!("requested row contract failed; inspect the written report");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_latent_rho_one_shares_normals() {
        let (q, p) = (0..128)
            .filter_map(|attempt| {
                let mut rng = ChaCha8Rng::from_seed(law_seed(
                    1,
                    "shared-latent",
                    "rho=1,sigma=0.2",
                    (5, 5),
                    0,
                    attempt,
                ));
                shared_latent(5, "rho=1,sigma=0.2", &mut rng)
            })
            .next()
            .expect("shared-latent should have a bounded valid smoke draw");
        assert_eq!(q.normals.len(), p.normals.len());
        for (a, b) in q.normals.iter().zip(&p.normals) {
            assert!((*a - *b).norm() < 1e-12);
        }
    }

    #[test]
    fn polar_uses_centroid_and_normalizes_area() {
        let (q, p) = (0..128)
            .filter_map(|attempt| {
                let mut rng = ChaCha8Rng::from_seed(law_seed(
                    2,
                    "polar-coupled",
                    "center=centroid,phi=pi/4",
                    (5, 5),
                    0,
                    attempt,
                ));
                polar_coupled(5, "center=centroid,phi=pi/4", &mut rng)
            })
            .next()
            .expect("polar coupling should have a bounded valid smoke draw");
        assert_eq!(q.normals.len(), p.normals.len());
        assert!((shoelace(&q.vertices).abs() - 1.0).abs() < 1e-10);
        assert!((shoelace(&p.vertices).abs() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn origin_double_polar_preserves_incidence_and_mahler_product_is_finite() {
        let q = (0..128)
            .filter_map(|attempt| {
                let mut rng =
                    ChaCha8Rng::from_seed(law_seed(3, "baseline", "delta=0.2", (5, 5), 0, attempt));
                current_baseline(5, &mut rng).and_then(normalize)
            })
            .next()
            .expect("baseline should have a bounded valid smoke draw");
        let polar = origin_polar(&q).expect("origin polar exists");
        let double = origin_polar(&polar).expect("double polar exists");
        assert_eq!(q.normals.len(), polar.normals.len());
        assert_eq!(q.normals.len(), double.normals.len());
        let mahler = shoelace(&q.vertices).abs() * shoelace(&polar.vertices).abs();
        assert!(mahler.is_finite() && mahler > 0.0);
        let q_area = shoelace(&q.vertices).abs();
        let double_area = shoelace(&double.vertices).abs();
        assert!((q_area - double_area).abs() < 1e-8);
    }

    #[test]
    fn cross_seed_same_law_smoke_has_unique_ids_and_valid_conditioning() {
        let mut ids = std::collections::BTreeSet::new();
        let mut accepted = 0usize;
        for seed in 0..16 {
            for row in 0..2 {
                let id = sample_id("shared-latent", "rho=0.5,sigma=0.2", seed, row, 0, (4, 4));
                assert!(ids.insert(id));
                for attempt in 0..32 {
                    let mut rng = ChaCha8Rng::from_seed(law_seed(
                        seed,
                        "shared-latent",
                        "rho=0.5,sigma=0.2",
                        (4, 4),
                        row,
                        attempt,
                    ));
                    if let Some((q, p)) = shared_latent(4, "rho=0.5,sigma=0.2", &mut rng) {
                        assert!(q.heights.iter().all(|h| *h > 0.0));
                        assert!(p.heights.iter().all(|h| *h > 0.0));
                        accepted += 1;
                        break;
                    }
                }
            }
        }
        assert!(accepted >= 16);
    }

    #[test]
    fn deterministic_attempt_seed_repeats() {
        let seed = law_seed(99, "shared-latent", "rho=0.5,sigma=0.2", (4, 4), 2, 3);
        let mut a = ChaCha8Rng::from_seed(seed);
        let mut b = ChaCha8Rng::from_seed(seed);
        let pa = shared_latent(4, "rho=0.5,sigma=0.2", &mut a).unwrap();
        let pb = shared_latent(4, "rho=0.5,sigma=0.2", &mut b).unwrap();
        assert_eq!(pa.0.vertices, pb.0.vertices);
        assert_eq!(pa.1.vertices, pb.1.vertices);
    }

    #[test]
    fn forced_exhaustion_writes_report_with_terminal_status() {
        let args = Args {
            out_dir: std::env::temp_dir(),
            seed: 17,
            attempts: 1,
            rows_per_law: 1,
            only_law: Some("shared-latent".into()),
        };
        let row = exhausted_row("shared-latent", 13, "rho=0,sigma=0.2", &args, (3, 3), 0);
        assert_eq!(row.status, "exhausted");
        assert_eq!(row.attempts, 1);
        let mut status_counts = BTreeMap::new();
        status_counts.insert(row.status.clone(), 1);
        assert!(terminal_contract(1, 1, &status_counts));
        let report_path = std::env::temp_dir().join(format!(
            "generator-natural-law-expansion-forced-exhaustion-{}.json",
            std::process::id()
        ));
        let report = Report {
            schema: "generator-natural-law-expansion-report-v1",
            law_version: LAW_VERSION,
            seed: args.seed,
            max_attempts_per_row: args.attempts,
            rows_per_law: args.rows_per_law,
            rows: 1,
            requested_rows: 1,
            all_requested_rows_terminal: true,
            status_counts,
            dispositions: dispositions(),
            command: "forced-exhaustion-test".into(),
            source_revision: "test-revision".into(),
            source_tree: "test-tree".into(),
            source_dirty: false,
            source_dirty_scope: "test fixture",
            interpretation_boundary: "test",
        };
        serde_json::to_writer_pretty(
            File::create(&report_path).expect("create forced-exhaustion report"),
            &report,
        )
        .expect("write forced-exhaustion report");
        let value: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&report_path).expect("read forced-exhaustion report"),
        )
        .expect("parse forced-exhaustion report");
        assert_eq!(value["status_counts"]["exhausted"], 1);
        assert_eq!(value["all_requested_rows_terminal"], true);
        std::fs::remove_file(report_path).expect("remove forced-exhaustion report");
    }

    #[test]
    fn dirty_tracked_dependency_invalidates_source_provenance() {
        assert!(tracked_source_dirty_from_status(
            " M crates/symplectic/src/geom/polygon.rs\n"
        ));
        assert!(!tracked_source_dirty_from_status(
            "?? disposable-smoke-output.json\n"
        ));
        let mut invalid = BTreeMap::new();
        invalid.insert("invalid".into(), 1);
        assert!(!terminal_contract(1, 1, &invalid));
    }
}
