//! Breadth-first smoke executor for the alternative polygon-generator wishlist.
//!
//! The binary deliberately keeps each law isolated: a failed law records a
//! disposition and the remaining laws continue.  It is a feasibility packet,
//! not a production sampler or a transfer claim.

use exp_sys_landscape::{
    capacity_auto, compute_sys_from_capacity, exact_volume_from_incidence_as_f64,
    SysLandscapePolytopeCache,
};
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
use std::time::Instant;
use symplectic::geom::polygon::{polygon_area, random_polygon_2d};

const DEFAULT_SEED: u64 = 20260714;
const DEFAULT_ATTEMPTS: usize = 128;
const DEFAULT_RUNTIME_CAP_MS: f64 = 2_000.0;
const PAIRS: &[(usize, usize)] = &[(3, 3), (4, 6), (6, 6)];

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
    attempts: usize,
    runtime_cap_ms: f64,
    rows_per_law: usize,
    target_backend: bool,
    only_law: Option<String>,
    only_family: Option<String>,
    identity_scope: Option<String>,
}

#[derive(Clone, Debug)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}

#[derive(Clone, Serialize)]
struct SmokeRow {
    schema: &'static str,
    sample_id: String,
    law: String,
    wishlist_item: u8,
    law_version: &'static str,
    identity_scope: Option<String>,
    seed: u64,
    row_index: usize,
    attempt: usize,
    attempts: usize,
    rejections: usize,
    parameter: String,
    pair_bucket: String,
    facet_count: usize,
    accepted: bool,
    validation_status: String,
    rejection_reason: Option<String>,
    factor_q_area: Option<f64>,
    factor_p_area: Option<f64>,
    factor_q_support_cv: Option<f64>,
    factor_p_support_cv: Option<f64>,
    factor_q_gap_cv: Option<f64>,
    factor_p_gap_cv: Option<f64>,
    factor_q_isoperimetric_ratio: Option<f64>,
    factor_p_isoperimetric_ratio: Option<f64>,
    pairing_id: Option<String>,
    volume: Option<f64>,
    capacity: Option<f64>,
    sys: Option<f64>,
    iterations: Option<u64>,
    generation_ms: f64,
    validation_ms: f64,
    target_ms: f64,
}

#[derive(Clone, Serialize)]
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
    identity_scope: Option<String>,
    seed: u64,
    max_attempts_per_row: usize,
    runtime_cap_ms: f64,
    pairs: Vec<String>,
    rows: usize,
    command: String,
    source_revision: String,
    status_counts: BTreeMap<String, usize>,
    per_arm: Vec<LawSummary>,
    dispositions: Vec<Disposition>,
    interpretation_boundary: &'static str,
}

#[derive(Serialize)]
struct LawSummary {
    law: String,
    parameter: String,
    rows: usize,
    accepted_rows: usize,
    survived_rows: usize,
    total_generation_ms: f64,
    total_validation_ms: f64,
    total_target_ms: f64,
    max_attempts_observed: usize,
    factor_metric_count: usize,
    mean_support_cv: Option<f64>,
    mean_gap_cv: Option<f64>,
    mean_isoperimetric_ratio: Option<f64>,
    #[serde(skip)]
    total_support_cv: f64,
    #[serde(skip)]
    total_gap_cv: f64,
    #[serde(skip)]
    total_isoperimetric_ratio: f64,
}

#[derive(Clone, Copy)]
struct FactorMetrics {
    area: f64,
    support_cv: f64,
    gap_cv: f64,
    isoperimetric_ratio: f64,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/alternative-generator-smoke/artifacts",
        ),
        seed: DEFAULT_SEED,
        attempts: DEFAULT_ATTEMPTS,
        runtime_cap_ms: DEFAULT_RUNTIME_CAP_MS,
        rows_per_law: 1,
        target_backend: false,
        only_law: None,
        only_family: None,
        identity_scope: None,
    };
    let mut i = 1;
    while i < argv.len() {
        let value = |flag: &str| {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(value("--out-dir"));
                i += 2;
            }
            "--seed" => {
                args.seed = value("--seed").parse().expect("--seed must be u64");
                i += 2;
            }
            "--attempts" => {
                args.attempts = value("--attempts")
                    .parse()
                    .expect("--attempts must be usize");
                i += 2;
            }
            "--runtime-cap-ms" => {
                args.runtime_cap_ms = value("--runtime-cap-ms")
                    .parse()
                    .expect("--runtime-cap-ms must be f64");
                i += 2;
            }
            "--rows-per-law" => {
                args.rows_per_law = value("--rows-per-law")
                    .parse()
                    .expect("--rows-per-law must be usize");
                i += 2;
            }
            "--target" => {
                args.target_backend = true;
                i += 1;
            }
            "--only-law" => {
                args.only_law = Some(value("--only-law").to_string());
                i += 2;
            }
            "--only-family" => {
                args.only_family = Some(value("--only-family").to_string());
                i += 2;
            }
            "--identity-scope" => {
                args.identity_scope = Some(value("--identity-scope").to_string());
                i += 2;
            }
            "--help" | "-h" => {
                println!(
                    "--out-dir DIR --seed N --attempts N --runtime-cap-ms MS --rows-per-law N [--only-law LAW | --only-family factorial --identity-scope ID] [--target]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.runtime_cap_ms.is_finite() && args.runtime_cap_ms > 0.0);
    assert!(
        args.only_law.is_none() || args.only_family.is_none(),
        "--only-law and --only-family are mutually exclusive"
    );
    if let Some(family) = &args.only_family {
        assert_eq!(family, "factorial", "only the factorial family is reviewed");
        assert!(
            !args.target_backend,
            "--only-family is geometry-only and cannot be combined with --target"
        );
        assert!(
            args.identity_scope.is_some(),
            "--only-family requires --identity-scope to prevent artifact aliasing"
        );
    }
    if let Some(scope) = &args.identity_scope {
        assert!(
            !scope.is_empty()
                && scope
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')),
            "--identity-scope must use nonempty ASCII letters, digits, '.', '-', or '_'"
        );
    }
    args
}

fn law_seed(
    seed: u64,
    law: &str,
    parameter: &str,
    bucket: (usize, usize),
    row_index: usize,
    attempt: usize,
) -> [u8; 32] {
    let mut key = Vec::new();
    key.extend_from_slice(&seed.to_le_bytes());
    key.extend_from_slice(law.as_bytes());
    key.push(0);
    key.extend_from_slice(parameter.as_bytes());
    key.push(0);
    key.extend_from_slice(&(bucket.0 as u64).to_le_bytes());
    key.extend_from_slice(&(bucket.1 as u64).to_le_bytes());
    key.extend_from_slice(&(row_index as u64).to_le_bytes());
    key.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&key).as_bytes()
}

fn latent_identity<'a>(law: &'a str, parameter: &'a str) -> (&'a str, &'a str) {
    if matches!(
        law,
        "factorial-baseline" | "factorial-q" | "factorial-p" | "factorial-both"
    ) {
        ("factorial-base", "paired-current")
    } else if matches!(law, "broken-antipodal" | "broken-symmetric-control") {
        ("antipodal-pair", "paired-opposite-supports")
    } else {
        (law, parameter)
    }
}

fn law_family(law: &str) -> Option<&'static str> {
    law.starts_with("factorial-").then_some("factorial")
}

fn scoped_identity_prefix(identity_scope: Option<&str>) -> String {
    identity_scope.map_or_else(
        || "altgen-v2".to_string(),
        |scope| format!("altgen-v2/scope={scope}"),
    )
}

fn pairing_id(
    law: &str,
    identity_scope: Option<&str>,
    seed: u64,
    row_index: usize,
    attempt: usize,
    bucket: (usize, usize),
) -> Option<String> {
    let family = if law.starts_with("factorial-") {
        "factorial"
    } else if matches!(law, "broken-antipodal" | "broken-symmetric-control") {
        "antipodal"
    } else {
        return None;
    };
    Some(format!(
        "{}/{family}/seed={seed}/row={row_index}/attempt={attempt}/{}x{}",
        scoped_identity_prefix(identity_scope),
        bucket.0,
        bucket.1
    ))
}

fn area_normalize(mut f: Factor) -> Option<Factor> {
    if !all_facets_active(&f) {
        return None;
    }
    let area = polygon_area(&f.normals, &f.heights)?;
    if !area.is_finite() || area <= 0.0 {
        return None;
    }
    let scale = area.sqrt().recip();
    for h in &mut f.heights {
        *h *= scale;
    }
    Some(f)
}

/// Cheap H-representation witness used before the exact 4D boundary.  Each
/// adjacent edge intersection must satisfy every half-plane; this rejects
/// inactive facets without spending the much slower rational reconstruction.
fn all_facets_active(f: &Factor) -> bool {
    let n = f.normals.len();
    if n < 3 || f.heights.len() != n {
        return false;
    }
    for i in 0..n {
        let j = (i + 1) % n;
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / det;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / det;
        for (normal, height) in f.normals.iter().zip(&f.heights) {
            if normal[0] * x + normal[1] * y > *height + 1e-9 {
                return false;
            }
        }
    }
    true
}

fn coefficient_of_variation(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    if !mean.is_finite() || mean <= 0.0 {
        return None;
    }
    let variance = values
        .iter()
        .map(|x| {
            let d = *x - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    Some(variance.sqrt() / mean)
}

fn factor_metrics(f: &Factor) -> Option<FactorMetrics> {
    let area = polygon_area(&f.normals, &f.heights)?;
    let support_cv = coefficient_of_variation(&f.heights)?;
    let mut angles: Vec<f64> = f
        .normals
        .iter()
        .map(|n| n[1].atan2(n[0]).rem_euclid(TAU))
        .collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let gaps: Vec<f64> = (0..angles.len())
        .map(|i| {
            let next = if i + 1 == angles.len() {
                angles[0] + TAU
            } else {
                angles[i + 1]
            };
            next - angles[i]
        })
        .collect();
    let gap_cv = coefficient_of_variation(&gaps)?;

    let mut vertices = Vec::with_capacity(f.normals.len());
    for i in 0..f.normals.len() {
        let j = (i + 1) % f.normals.len();
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return None;
        }
        vertices.push(Vector2::new(
            (f.heights[i] * b[1] - f.heights[j] * a[1]) / det,
            (a[0] * f.heights[j] - b[0] * f.heights[i]) / det,
        ));
    }
    let perimeter = (0..vertices.len())
        .map(|i| (vertices[(i + 1) % vertices.len()] - vertices[i]).norm())
        .sum::<f64>();
    let isoperimetric_ratio = 4.0 * PI * area / (perimeter * perimeter);
    if !isoperimetric_ratio.is_finite() {
        return None;
    }
    Some(FactorMetrics {
        area,
        support_cv,
        gap_cv,
        isoperimetric_ratio,
    })
}

fn random_angles<R: Rng>(n: usize, rng: &mut R, period: f64) -> Vec<f64> {
    let mut a: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * period).collect();
    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    a
}

fn from_angles(angles: &[f64], heights: Vec<f64>) -> Factor {
    Factor {
        normals: angles
            .iter()
            .map(|a| Vector2::new(a.cos(), a.sin()))
            .collect(),
        heights,
    }
}

fn baseline(n: usize, rng: &mut ChaCha8Rng) -> Factor {
    let (normals, heights) = random_polygon_2d(n, 0.8, 1.2, rng);
    Factor { normals, heights }
}

fn equal_support(n: usize, rng: &mut ChaCha8Rng) -> Factor {
    let angles = random_angles(n, rng, TAU);
    from_angles(&angles, vec![1.0; n])
}

fn tangentialize(f: &Factor) -> Factor {
    Factor {
        normals: f.normals.clone(),
        heights: vec![1.0; f.heights.len()],
    }
}

fn log_support(n: usize, sigma: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let normal = Normal::new(0.0, 1.0).ok()?;
    let mut z: Vec<f64> = (0..n)
        .map(|_| (normal.sample(rng) as f64).clamp(-2.0, 2.0))
        .collect();
    let mean = z.iter().sum::<f64>() / n as f64;
    for x in &mut z {
        *x = sigma * (*x - mean);
    }
    let angles = random_angles(n, rng, TAU);
    Some(from_angles(&angles, z.into_iter().map(f64::exp).collect()))
}

fn smooth_support(n: usize, modes: usize, amplitude: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let normal = Normal::new(0.0, 1.0).ok()?;
    let coefficients: Vec<(f64, f64)> = (0..modes)
        .map(|_| (normal.sample(rng), normal.sample(rng)))
        .collect();
    let angles = random_angles(n, rng, TAU);
    let mut g: Vec<f64> = angles
        .iter()
        .map(|theta| {
            coefficients
                .iter()
                .enumerate()
                .map(|(index, (a, b))| {
                    let r = (index + 1) as f64;
                    (a * (r * theta).cos() + b * (r * theta).sin()) / r
                })
                .sum::<f64>()
        })
        .collect();
    let mean = g.iter().sum::<f64>() / g.len() as f64;
    for value in &mut g {
        *value -= mean;
    }
    let sd = (g.iter().map(|x| x * x).sum::<f64>() / g.len() as f64).sqrt();
    if !sd.is_finite() || sd < 1e-12 {
        return None;
    }
    for value in &mut g {
        *value *= amplitude / sd;
    }
    Some(from_angles(&angles, g.into_iter().map(f64::exp).collect()))
}

fn dirichlet(n: usize, alpha: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let gamma = Gamma::new(alpha, 1.0).ok()?;
    let mut g: Vec<f64> = (0..n).map(|_| gamma.sample(rng)).collect();
    let sum: f64 = g.iter().sum();
    for x in &mut g {
        *x = TAU * *x / sum;
    }
    if g.iter().any(|x| *x >= PI) {
        return None;
    }
    let rotation = rng.gen::<f64>() * TAU;
    let mut angles = Vec::with_capacity(n);
    let mut t = rotation;
    for gap in g {
        angles.push(t);
        t += gap;
    }
    Some(from_angles(&angles, vec![1.0; n]))
}

fn jittered_regular(n: usize, jitter: f64, rng: &mut ChaCha8Rng) -> Factor {
    let base = PI / 2.0;
    let step = TAU / n as f64;
    let mut angles: Vec<f64> = (0..n)
        .map(|i| base + step * i as f64 + (rng.gen::<f64>() - 0.5) * jitter * step)
        .collect();
    angles.sort_by(|x, y| x.partial_cmp(y).unwrap());
    from_angles(&angles, vec![1.0; n])
}

fn sort_factor_by_normal_angle(normals: Vec<Vector2<f64>>, heights: Vec<f64>) -> Factor {
    let mut ix: Vec<usize> = (0..normals.len()).collect();
    ix.sort_by(|i, j| {
        let ai = normals[*i][1].atan2(normals[*i][0]);
        let aj = normals[*j][1].atan2(normals[*j][0]);
        ai.partial_cmp(&aj).unwrap()
    });
    Factor {
        normals: ix.iter().map(|i| normals[*i]).collect(),
        heights: ix.iter().map(|i| heights[*i]).collect(),
    }
}

fn symmetric_strips(n: usize, iid_widths: bool, rng: &mut ChaCha8Rng) -> Option<Factor> {
    if n % 2 != 0 {
        return None;
    }
    let r = n / 2;
    let lines = random_angles(r, rng, PI);
    let mut normals = Vec::with_capacity(n);
    let mut heights = Vec::with_capacity(n);
    for a in lines {
        let u = Vector2::new(a.cos(), a.sin());
        let width = if iid_widths {
            0.8 + 0.4 * rng.gen::<f64>()
        } else {
            1.0
        };
        normals.push(u);
        heights.push(width / 2.0);
        normals.push(-u);
        heights.push(width / 2.0);
    }
    Some(sort_factor_by_normal_angle(normals, heights))
}

fn antipodal_broken_and_control(n: usize, rng: &mut ChaCha8Rng) -> Option<(Factor, Factor)> {
    if n % 2 != 0 {
        return None;
    }
    let lines = random_angles(n / 2, rng, PI);
    let mut normals = Vec::with_capacity(n);
    let mut broken_heights = Vec::with_capacity(n);
    let mut control_heights = Vec::with_capacity(n);
    for a in lines {
        let u = Vector2::new(a.cos(), a.sin());
        let plus = 0.8 + 0.4 * rng.gen::<f64>();
        let minus = 0.8 + 0.4 * rng.gen::<f64>();
        let control = (plus + minus) / 2.0;
        normals.push(u);
        broken_heights.push(plus);
        control_heights.push(control);
        normals.push(-u);
        broken_heights.push(minus);
        control_heights.push(control);
    }
    Some((
        sort_factor_by_normal_angle(normals.clone(), broken_heights),
        sort_factor_by_normal_angle(normals, control_heights),
    ))
}

fn congruent(n: usize, phi: f64, rng: &mut ChaCha8Rng) -> (Factor, Factor) {
    let q = baseline(n, rng);
    let (pn, ph) = symplectic::geom::polygon::rotate_polygon_2d(&q.normals, &q.heights, phi);
    (
        q,
        Factor {
            normals: pn,
            heights: ph,
        },
    )
}

fn inscribed(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let angles = random_angles(n, rng, TAU);
    let mut gaps = Vec::with_capacity(n);
    for i in 0..n {
        let next = if i + 1 == n {
            angles[0] + TAU
        } else {
            angles[i + 1]
        };
        gaps.push(next - angles[i]);
    }
    if gaps.iter().any(|g| *g >= PI) {
        return None;
    }
    let normals: Vec<Vector2<f64>> = (0..n)
        .map(|i| {
            let mid = angles[i] + gaps[i] / 2.0;
            Vector2::new(mid.cos(), mid.sin())
        })
        .collect();
    let heights = gaps.iter().map(|g| (g / 2.0).cos()).collect();
    Some(Factor { normals, heights })
}

fn make_pair(
    law: &str,
    parameter: &str,
    k: usize,
    m: usize,
    rng: &mut ChaCha8Rng,
) -> Option<(Factor, Factor)> {
    if matches!(
        law,
        "factorial-baseline" | "factorial-q" | "factorial-p" | "factorial-both"
    ) {
        let base_q = baseline(k, rng);
        let base_p = baseline(m, rng);
        // Reject the latent baseline jointly so all four factorial arms use the
        // same accepted normal fans rather than drifting to different attempts.
        area_normalize(base_q.clone())?;
        area_normalize(base_p.clone())?;
        area_normalize(tangentialize(&base_q))?;
        area_normalize(tangentialize(&base_p))?;
        let mut q = base_q.clone();
        let mut p = base_p.clone();
        if matches!(law, "factorial-q" | "factorial-both") {
            q = tangentialize(&base_q);
        }
        if matches!(law, "factorial-p" | "factorial-both") {
            p = tangentialize(&base_p);
        }
        return Some((q, p));
    }
    if matches!(law, "broken-antipodal" | "broken-symmetric-control") {
        let (q_broken, q_control) = antipodal_broken_and_control(k, rng)?;
        let (p_broken, p_control) = antipodal_broken_and_control(m, rng)?;
        // The broken/control comparison is paired only on latent draws for
        // which both arms have all prescribed facets active.
        area_normalize(q_broken.clone())?;
        area_normalize(q_control.clone())?;
        area_normalize(p_broken.clone())?;
        area_normalize(p_control.clone())?;
        return if law == "broken-antipodal" {
            Some((q_broken, p_broken))
        } else {
            Some((q_control, p_control))
        };
    }
    let f = |n: usize, rng: &mut ChaCha8Rng| -> Option<Factor> {
        match law {
            "baseline" => Some(baseline(n, rng)),
            "equal-support" => Some(equal_support(n, rng)),
            "log-support" => log_support(n, parameter.parse().ok()?, rng),
            "smooth-support-r2" => smooth_support(n, 2, parameter.parse().ok()?, rng),
            "smooth-support-r3" => smooth_support(n, 3, parameter.parse().ok()?, rng),
            "dirichlet-gap" => dirichlet(n, parameter.parse().ok()?, rng),
            "jittered-regular" => Some(jittered_regular(n, parameter.parse().ok()?, rng)),
            "symmetric-strips-constant" => symmetric_strips(n, false, rng),
            "symmetric-strips-iid" => symmetric_strips(n, true, rng),
            "inscribed" => inscribed(n, rng),
            _ => None,
        }
    };
    if law == "congruent" {
        if k != m {
            return None;
        }
        let phi = match parameter {
            "zero" => 0.0,
            "half-step" => PI / (2.0 * k as f64),
            "full-step" => PI / k as f64,
            _ => return None,
        };
        let (q, p) = congruent(k, phi, rng);
        return Some((q, p));
    }
    let q = f(k, rng)?;
    let p = f(m, rng)?;
    Some((q, p))
}

fn evaluate_pair(
    q: Factor,
    p: Factor,
    args: &Args,
    law: &str,
    item: u8,
    parameter: &str,
    bucket: (usize, usize),
    seed: u64,
    row_index: usize,
    attempt: usize,
    generation_ms: f64,
    validation_ms_offset: f64,
) -> SmokeRow {
    let sample_id = format!(
        "{}/{law}/param={parameter}/seed={seed}/row={row_index}/attempt={attempt}/{}x{}",
        scoped_identity_prefix(args.identity_scope.as_deref()),
        bucket.0,
        bucket.1
    );
    let q_metrics = factor_metrics(&q);
    let p_metrics = factor_metrics(&p);
    let paired = pairing_id(
        law,
        args.identity_scope.as_deref(),
        seed,
        row_index,
        attempt,
        bucket,
    );
    let tv = Instant::now();
    let poly = SysLandscapePolytopeCache::from_lagrangian_product(
        &q.normals, &q.heights, &p.normals, &p.heights,
    );
    let mut validation_ms = validation_ms_offset + tv.elapsed().as_secs_f64() * 1000.0;
    let bucket_name = format!("{}x{}", bucket.0, bucket.1);
    let Some(poly) = poly else {
        return SmokeRow {
            schema: "alternative-generator-smoke-row-v2",
            sample_id: sample_id.clone(),
            law: law.to_string(),
            wishlist_item: item,
            law_version: "wishlist-2026-07-14-v2",
            identity_scope: args.identity_scope.clone(),
            seed,
            row_index,
            attempt,
            attempts: attempt + 1,
            rejections: attempt,
            parameter: parameter.to_string(),
            pair_bucket: bucket_name,
            facet_count: bucket.0 + bucket.1,
            accepted: false,
            validation_status: "invalid".into(),
            rejection_reason: Some("exact product validation rejected geometry".into()),
            factor_q_area: q_metrics.map(|x| x.area),
            factor_p_area: p_metrics.map(|x| x.area),
            factor_q_support_cv: q_metrics.map(|x| x.support_cv),
            factor_p_support_cv: p_metrics.map(|x| x.support_cv),
            factor_q_gap_cv: q_metrics.map(|x| x.gap_cv),
            factor_p_gap_cv: p_metrics.map(|x| x.gap_cv),
            factor_q_isoperimetric_ratio: q_metrics.map(|x| x.isoperimetric_ratio),
            factor_p_isoperimetric_ratio: p_metrics.map(|x| x.isoperimetric_ratio),
            pairing_id: paired.clone(),
            volume: None,
            capacity: None,
            sys: None,
            iterations: None,
            generation_ms,
            validation_ms,
            target_ms: 0.0,
        };
    };
    let volume_start = Instant::now();
    let volume = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
    validation_ms += volume_start.elapsed().as_secs_f64() * 1000.0;
    // Retain geometry/validation evidence when target evaluation is disabled.
    // Above ten facets the current in-process backend has no cancellable time
    // limit, so target mode records the predeclared cap rather than entering it.
    if !args.target_backend || poly.facet_count() > 10 {
        return SmokeRow {
            schema: "alternative-generator-smoke-row-v2",
            sample_id: sample_id.clone(),
            law: law.to_string(),
            wishlist_item: item,
            law_version: "wishlist-2026-07-14-v2",
            identity_scope: args.identity_scope.clone(),
            seed,
            row_index,
            attempt,
            attempts: attempt + 1,
            rejections: attempt,
            parameter: parameter.to_string(),
            pair_bucket: bucket_name,
            facet_count: poly.facet_count(),
            accepted: true,
            validation_status: if args.target_backend {
                "runtime_cap"
            } else {
                "survived"
            }
            .into(),
            rejection_reason: Some(if args.target_backend {
                "target backend skipped above predeclared facet-count cap 10".into()
            } else {
                "target backend disabled for breadth-first geometry smoke".into()
            }),
            factor_q_area: q_metrics.map(|x| x.area),
            factor_p_area: p_metrics.map(|x| x.area),
            factor_q_support_cv: q_metrics.map(|x| x.support_cv),
            factor_p_support_cv: p_metrics.map(|x| x.support_cv),
            factor_q_gap_cv: q_metrics.map(|x| x.gap_cv),
            factor_p_gap_cv: p_metrics.map(|x| x.gap_cv),
            factor_q_isoperimetric_ratio: q_metrics.map(|x| x.isoperimetric_ratio),
            factor_p_isoperimetric_ratio: p_metrics.map(|x| x.isoperimetric_ratio),
            pairing_id: paired.clone(),
            volume: Some(volume),
            capacity: None,
            sys: None,
            iterations: None,
            generation_ms,
            validation_ms,
            target_ms: 0.0,
        };
    }
    let tt = Instant::now();
    let target = capacity_auto(
        &poly.dual_vertices_f64,
        &poly.dual_vertices,
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    )
    .ok();
    let (capacity, sys, iterations) = target
        .as_ref()
        .map(|c| {
            (
                Some(c.min_action),
                compute_sys_from_capacity(&poly, c),
                Some(c.iterations),
            )
        })
        .unwrap_or((None, None, None));
    let target_ms = tt.elapsed().as_secs_f64() * 1000.0;
    // `runtime_cap_ms` is a post-hoc classification threshold.  The existing
    // target API is synchronous and cannot enforce a wall-clock kill safely.
    let status = if target.is_some() {
        if generation_ms + validation_ms + target_ms > args.runtime_cap_ms {
            "runtime_cap"
        } else {
            "survived"
        }
    } else {
        "target_failed"
    };
    SmokeRow {
        schema: "alternative-generator-smoke-row-v2",
        sample_id,
        law: law.to_string(),
        wishlist_item: item,
        law_version: "wishlist-2026-07-14-v2",
        identity_scope: args.identity_scope.clone(),
        seed,
        row_index,
        attempt,
        attempts: attempt + 1,
        rejections: attempt,
        parameter: parameter.to_string(),
        pair_bucket: bucket_name,
        facet_count: poly.facet_count(),
        accepted: true,
        validation_status: status.into(),
        rejection_reason: None,
        factor_q_area: q_metrics.map(|x| x.area),
        factor_p_area: p_metrics.map(|x| x.area),
        factor_q_support_cv: q_metrics.map(|x| x.support_cv),
        factor_p_support_cv: p_metrics.map(|x| x.support_cv),
        factor_q_gap_cv: q_metrics.map(|x| x.gap_cv),
        factor_p_gap_cv: p_metrics.map(|x| x.gap_cv),
        factor_q_isoperimetric_ratio: q_metrics.map(|x| x.isoperimetric_ratio),
        factor_p_isoperimetric_ratio: p_metrics.map(|x| x.isoperimetric_ratio),
        pairing_id: paired,
        volume: Some(volume),
        capacity,
        sys,
        iterations,
        generation_ms,
        validation_ms,
        target_ms,
    }
}

fn dispositions() -> Vec<Disposition> {
    vec![
        Disposition {
            wishlist_item: 1,
            law: "fresh baseline",
            disposition: "survived",
            evidence: "reuses random polygon kernel with explicit law/seed/attempt identity",
        },
        Disposition {
            wishlist_item: 2,
            law: "equal-support",
            disposition: "survived",
            evidence: "unit supports and area normalization are local",
        },
        Disposition {
            wishlist_item: 3,
            law: "log-support ladder",
            disposition: "survived",
            evidence: "bounded centered Gaussian support ladder",
        },
        Disposition {
            wishlist_item: 4,
            law: "smooth support field",
            disposition: "survived",
            evidence: "R=2,3 inverse-frequency Fourier fields with empirical log-support SD 0.1 use the local active-facet boundary",
        },
        Disposition {
            wishlist_item: 5,
            law: "shape-cell conditional",
            disposition: "backend_or_schema_expansion",
            evidence: "comparison design requires matched retained populations",
        },
        Disposition {
            wishlist_item: 6,
            law: "one-factor factorial",
            disposition: "survived",
            evidence: "all four arms share each baseline product's exact normal fans through a pairing identity",
        },
        Disposition {
            wishlist_item: 7,
            law: "Dirichlet angular gaps",
            disposition: "survived",
            evidence: "Gamma simplex draw with max-gap rejection",
        },
        Disposition {
            wishlist_item: 8,
            law: "jittered regular",
            disposition: "survived",
            evidence: "regular fan plus bounded order-preserving jitter",
        },
        Disposition {
            wishlist_item: 9,
            law: "symmetric strips",
            disposition: "survived",
            evidence: "constant-width and IID-width antipodal strip laws, even side counts",
        },
        Disposition {
            wishlist_item: 10,
            law: "broken antipodal",
            disposition: "survived",
            evidence: "broken and symmetric-control arms share lines and preserve each sampled strip width",
        },
        Disposition {
            wishlist_item: 11,
            law: "zonogon",
            disposition: "backend_or_schema_expansion",
            evidence: "not attempted after the direct H-representation laws filled the pass; faithful Minkowski-sum conversion needs a new local geometry path",
        },
        Disposition {
            wishlist_item: 12,
            law: "congruent factors",
            disposition: "survived",
            evidence: "same factor with explicit relative rotation",
        },
        Disposition {
            wishlist_item: 13,
            law: "shared latent",
            disposition: "compile_or_api_block",
            evidence: "coupling angular simplex and support laws is unresolved",
        },
        Disposition {
            wishlist_item: 14,
            law: "polar coupled",
            disposition: "compile_or_api_block",
            evidence: "canonical-center polarity would need a named exact center",
        },
        Disposition {
            wishlist_item: 15,
            law: "IID point hull",
            disposition: "backend_or_schema_expansion",
            evidence: "hull-side-count conditioning not available in current narrow API",
        },
        Disposition {
            wishlist_item: 16,
            law: "inscribed polygon",
            disposition: "survived",
            evidence: "circle hull has direct edge-normal/support formula",
        },
        Disposition {
            wishlist_item: 17,
            law: "Poisson line cell",
            disposition: "backend_or_schema_expansion",
            evidence: "not runtime-tested: faithful stationary-line windowing and side-count conditioning need a new sampler",
        },
        Disposition {
            wishlist_item: 18,
            law: "SO(4)/U(2) orientation",
            disposition: "backend_or_schema_expansion",
            evidence: "mixed-coordinate target requires generic backend/cache identity",
        },
        Disposition {
            wishlist_item: 19,
            law: "quotient-transverse",
            disposition: "backend_or_schema_expansion",
            evidence: "actual Sp(4) orbit tangent projection is not a local helper",
        },
        Disposition {
            wishlist_item: 20,
            law: "generic centrally symmetric",
            disposition: "backend_or_schema_expansion",
            evidence: "generic exact capacity path is a separate owner",
        },
        Disposition {
            wishlist_item: 21,
            law: "SL(4) structured images",
            disposition: "backend_or_schema_expansion",
            evidence: "law on determinant-one matrices and generic reconstruction are separate",
        },
    ]
}

fn main() {
    let args = parse_args();
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("smoke-rows.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut rows_out = BufWriter::new(File::create(&rows_path).expect("create rows"));
    let mut rows_count = 0usize;
    let mut all_rows = Vec::new();
    let jobs: &[(&str, u8, &[&str])] = &[
        ("baseline", 1, &["0.2"]),
        ("equal-support", 2, &["area=1"]),
        ("log-support", 3, &["0.0", "0.1", "0.2"]),
        ("smooth-support-r2", 4, &["0.1"]),
        ("smooth-support-r3", 4, &["0.1"]),
        ("factorial-baseline", 6, &["current"]),
        ("factorial-q", 6, &["q=tangential"]),
        ("factorial-p", 6, &["p=tangential"]),
        ("factorial-both", 6, &["q,p=tangential"]),
        ("dirichlet-gap", 7, &["0.5", "1.0", "2.0", "10.0"]),
        ("jittered-regular", 8, &["0.0", "0.1"]),
        ("symmetric-strips-constant", 9, &["constant-width"]),
        ("symmetric-strips-iid", 9, &["iid-width"]),
        ("broken-symmetric-control", 10, &["paired-width-control"]),
        ("broken-antipodal", 10, &["independent-supports"]),
        ("congruent", 12, &["zero", "half-step", "full-step"]),
        ("inscribed", 16, &["circle-radius=1"]),
    ];
    for &(law, item, params) in jobs {
        if args.only_law.as_deref().is_some_and(|wanted| wanted != law) {
            continue;
        }
        if args
            .only_family
            .as_deref()
            .is_some_and(|wanted| law_family(law) != Some(wanted))
        {
            continue;
        }
        for &parameter in params {
            for &bucket in PAIRS {
                if (law.starts_with("symmetric-strips")
                    || matches!(law, "broken-antipodal" | "broken-symmetric-control"))
                    && (bucket.0 % 2 != 0 || bucket.1 % 2 != 0)
                {
                    continue;
                }
                if law == "congruent" && bucket.0 != bucket.1 {
                    continue;
                }
                for row_index in 0..args.rows_per_law {
                    let mut accepted = None;
                    let mut generation_ms = 0.0;
                    let mut validation_ms = 0.0;
                    for attempt in 0..args.attempts {
                        let generation_start = Instant::now();
                        let (latent_law, latent_parameter) = latent_identity(law, parameter);
                        let mut rng = ChaCha8Rng::from_seed(law_seed(
                            args.seed,
                            latent_law,
                            latent_parameter,
                            bucket,
                            row_index,
                            attempt,
                        ));
                        let generated = make_pair(law, parameter, bucket.0, bucket.1, &mut rng)
                            .and_then(|(q, p)| Some((area_normalize(q)?, area_normalize(p)?)));
                        generation_ms += generation_start.elapsed().as_secs_f64() * 1000.0;
                        if let Some((q, p)) = generated {
                            let row = evaluate_pair(
                                q,
                                p,
                                &args,
                                law,
                                item,
                                parameter,
                                bucket,
                                args.seed,
                                row_index,
                                attempt,
                                generation_ms,
                                validation_ms,
                            );
                            if row.validation_status != "invalid" {
                                accepted = Some(row);
                                break;
                            }
                            validation_ms = row.validation_ms;
                        }
                    }
                    let row = accepted.unwrap_or_else(|| SmokeRow {
                        schema: "alternative-generator-smoke-row-v2",
                        sample_id: format!(
                            "{}/{law}/param={parameter}/seed={}/row={row_index}/outcome=exhausted/{}x{}",
                            scoped_identity_prefix(args.identity_scope.as_deref()),
                            args.seed, bucket.0, bucket.1
                        ),
                        law: law.to_string(),
                        wishlist_item: item,
                        law_version: "wishlist-2026-07-14-v2",
                        identity_scope: args.identity_scope.clone(),
                        seed: args.seed,
                        row_index,
                        attempt: args.attempts - 1,
                        attempts: args.attempts,
                        rejections: args.attempts,
                        parameter: parameter.to_string(),
                        pair_bucket: format!("{}x{}", bucket.0, bucket.1),
                        facet_count: bucket.0 + bucket.1,
                        accepted: false,
                        validation_status: "invalid_or_low_acceptance".into(),
                        rejection_reason: Some(format!(
                            "no accepted geometry in {} bounded attempts",
                            args.attempts
                        )),
                        factor_q_area: None,
                        factor_p_area: None,
                        factor_q_support_cv: None,
                        factor_p_support_cv: None,
                        factor_q_gap_cv: None,
                        factor_p_gap_cv: None,
                        factor_q_isoperimetric_ratio: None,
                        factor_p_isoperimetric_ratio: None,
                        pairing_id: pairing_id(
                            law,
                            args.identity_scope.as_deref(),
                            args.seed,
                            row_index,
                            args.attempts - 1,
                            bucket,
                        ),
                        volume: None,
                        capacity: None,
                        sys: None,
                        iterations: None,
                        generation_ms,
                        validation_ms,
                        target_ms: 0.0,
                    });
                    serde_json::to_writer(&mut rows_out, &row).expect("write row");
                    rows_out.write_all(b"\n").expect("newline");
                    rows_count += 1;
                    all_rows.push(row);
                }
            }
        }
    }
    rows_out.flush().expect("flush rows");
    let mut status_counts = BTreeMap::new();
    let mut law_map: BTreeMap<(String, String), LawSummary> = BTreeMap::new();
    for row in &all_rows {
        *status_counts
            .entry(row.validation_status.clone())
            .or_insert(0) += 1;
        let summary = law_map
            .entry((row.law.clone(), row.parameter.clone()))
            .or_insert_with(|| LawSummary {
                law: row.law.clone(),
                parameter: row.parameter.clone(),
                rows: 0,
                accepted_rows: 0,
                survived_rows: 0,
                total_generation_ms: 0.0,
                total_validation_ms: 0.0,
                total_target_ms: 0.0,
                max_attempts_observed: 0,
                factor_metric_count: 0,
                mean_support_cv: None,
                mean_gap_cv: None,
                mean_isoperimetric_ratio: None,
                total_support_cv: 0.0,
                total_gap_cv: 0.0,
                total_isoperimetric_ratio: 0.0,
            });
        summary.rows += 1;
        summary.accepted_rows += usize::from(row.accepted);
        summary.survived_rows += usize::from(row.validation_status == "survived");
        summary.total_generation_ms += row.generation_ms;
        summary.total_validation_ms += row.validation_ms;
        summary.total_target_ms += row.target_ms;
        summary.max_attempts_observed = summary.max_attempts_observed.max(row.attempts);
        for metrics in [
            (
                row.factor_q_support_cv,
                row.factor_q_gap_cv,
                row.factor_q_isoperimetric_ratio,
            ),
            (
                row.factor_p_support_cv,
                row.factor_p_gap_cv,
                row.factor_p_isoperimetric_ratio,
            ),
        ] {
            if let (Some(support_cv), Some(gap_cv), Some(isoperimetric_ratio)) = metrics {
                summary.factor_metric_count += 1;
                summary.total_support_cv += support_cv;
                summary.total_gap_cv += gap_cv;
                summary.total_isoperimetric_ratio += isoperimetric_ratio;
            }
        }
    }
    for summary in law_map.values_mut() {
        if summary.factor_metric_count > 0 {
            let count = summary.factor_metric_count as f64;
            summary.mean_support_cv = Some(summary.total_support_cv / count);
            summary.mean_gap_cv = Some(summary.total_gap_cv / count);
            summary.mean_isoperimetric_ratio = Some(summary.total_isoperimetric_ratio / count);
        }
    }
    let source_revision = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    let report = Report {
        schema: "alternative-generator-smoke-report-v2",
        law_version: "wishlist-2026-07-14-v2",
        identity_scope: args.identity_scope.clone(),
        seed: args.seed,
        max_attempts_per_row: args.attempts,
        runtime_cap_ms: args.runtime_cap_ms,
        pairs: PAIRS.iter().map(|(k, m)| format!("{k}x{m}")).collect(),
        rows: rows_count,
        command: std::env::args().collect::<Vec<_>>().join(" "),
        source_revision,
        status_counts,
        per_arm: law_map.into_values().collect(),
        dispositions: dispositions(),
        interpretation_boundary: "Tiny geometry smoke is plumbing, feasibility, and coarse separation evidence only; it does not establish a target transfer conclusion.",
    };
    serde_json::to_writer_pretty(File::create(&report_path).expect("create report"), &report)
        .expect("write report");
    println!(
        "wrote {} rows to {} and report to {}",
        rows_count,
        rows_path.display(),
        report_path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_seed(seed: u64, law: &str, parameter: &str) -> [u8; 32] {
        law_seed(seed, law, parameter, (4, 6), 0, 0)
    }

    #[test]
    fn equal_support_has_equal_heights_and_positive_area() {
        let mut rng = ChaCha8Rng::from_seed(test_seed(7, "equal-support", "area=1"));
        let f = area_normalize(equal_support(5, &mut rng)).unwrap();
        assert!(f.heights.iter().all(|h| (*h - f.heights[0]).abs() < 1e-12));
        assert!((polygon_area(&f.normals, &f.heights).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn log_support_has_unit_geometric_mean_before_area_normalization() {
        let mut rng = ChaCha8Rng::from_seed(test_seed(7, "log-support", "0.2"));
        let f = log_support(6, 0.2, &mut rng).unwrap();
        assert!(f.heights.iter().product::<f64>().ln().abs() < 1e-12);
    }

    #[test]
    fn dirichlet_and_inscribed_have_distinct_support_laws() {
        let da = (0..128)
            .find_map(|attempt| {
                let mut rng =
                    ChaCha8Rng::from_seed(law_seed(8, "dirichlet-gap", "1.0", (4, 6), 0, attempt));
                area_normalize(dirichlet(5, 1.0, &mut rng)?)
            })
            .unwrap();
        let ib = (0..128)
            .find_map(|attempt| {
                let mut rng = ChaCha8Rng::from_seed(law_seed(
                    8,
                    "inscribed",
                    "circle-radius=1",
                    (4, 6),
                    0,
                    attempt,
                ));
                area_normalize(inscribed(5, &mut rng)?)
            })
            .unwrap();
        assert!(da
            .heights
            .iter()
            .zip(ib.heights.iter())
            .any(|(x, y)| (x - y).abs() > 1e-4));
    }

    #[test]
    fn symmetric_strip_supports_are_antipodal_pairs() {
        let mut rng =
            ChaCha8Rng::from_seed(test_seed(9, "symmetric-strips-constant", "constant-width"));
        let f = symmetric_strips(6, false, &mut rng).unwrap();
        for i in 0..f.normals.len() {
            let has_opposite = f.normals.iter().zip(&f.heights).any(|(n, h)| {
                (n + f.normals[i]).norm() < 1e-12 && (*h - f.heights[i]).abs() < 1e-12
            });
            assert!(has_opposite);
        }
    }

    #[test]
    fn broken_control_preserves_each_strip_width() {
        let mut rng =
            ChaCha8Rng::from_seed(test_seed(9, "antipodal-pair", "paired-opposite-supports"));
        let (broken, control) = antipodal_broken_and_control(6, &mut rng).unwrap();
        for i in 0..broken.normals.len() {
            let j = broken
                .normals
                .iter()
                .position(|n| (n + broken.normals[i]).norm() < 1e-12)
                .unwrap();
            assert!((control.heights[i] - control.heights[j]).abs() < 1e-12);
            assert!(
                (2.0 * control.heights[i] - broken.heights[i] - broken.heights[j]).abs() < 1e-12
            );
        }
    }

    #[test]
    fn factorial_arms_keep_the_exact_baseline_normal_fans() {
        let seed = (0..128)
            .find_map(|attempt| {
                let seed = law_seed(10, "factorial-base", "paired-current", (4, 6), 0, attempt);
                let mut rng = ChaCha8Rng::from_seed(seed);
                make_pair("factorial-baseline", "current", 4, 6, &mut rng).map(|_| seed)
            })
            .unwrap();
        let mut baseline_rng = ChaCha8Rng::from_seed(seed);
        let mut both_rng = ChaCha8Rng::from_seed(seed);
        let (base_q, base_p) =
            make_pair("factorial-baseline", "current", 4, 6, &mut baseline_rng).unwrap();
        let (tan_q, tan_p) =
            make_pair("factorial-both", "q,p=tangential", 4, 6, &mut both_rng).unwrap();
        assert_eq!(base_q.normals, tan_q.normals);
        assert_eq!(base_p.normals, tan_p.normals);
        assert!(tan_q.heights.iter().all(|h| (*h - 1.0).abs() < 1e-12));
        assert!(tan_p.heights.iter().all(|h| (*h - 1.0).abs() < 1e-12));
    }

    #[test]
    fn congruent_rotation_preserves_factor_area() {
        let mut rng = ChaCha8Rng::from_seed(test_seed(10, "congruent", "half-step"));
        let (q, p) = congruent(5, 0.2, &mut rng);
        let aq = polygon_area(&q.normals, &q.heights).unwrap();
        let ap = polygon_area(&p.normals, &p.heights).unwrap();
        assert!((aq - ap).abs() < 1e-10);
    }

    #[test]
    fn inscribed_support_formula_is_positive() {
        let mut rng = ChaCha8Rng::from_seed(test_seed(11, "inscribed", "circle-radius=1"));
        let f = inscribed(6, &mut rng).unwrap();
        assert!(f.heights.iter().all(|h| *h > 0.0 && *h <= 1.0));
    }

    #[test]
    fn row_and_bucket_change_the_seed_identity() {
        let a = law_seed(12, "baseline", "0.2", (3, 3), 0, 0);
        let b = law_seed(12, "baseline", "0.2", (3, 3), 1, 0);
        let c = law_seed(12, "baseline", "0.2", (4, 6), 0, 0);
        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn factorial_family_filter_is_exact() {
        for law in [
            "factorial-baseline",
            "factorial-q",
            "factorial-p",
            "factorial-both",
        ] {
            assert_eq!(law_family(law), Some("factorial"));
        }
        assert_eq!(law_family("baseline"), None);
        assert_eq!(law_family("broken-antipodal"), None);
    }

    #[test]
    fn scoped_factorial_pairing_identity_is_complete_and_nonaliasing() {
        let prior = pairing_id("factorial-baseline", None, 17, 3, 5, (4, 6)).unwrap();
        let scoped = [
            "factorial-baseline",
            "factorial-q",
            "factorial-p",
            "factorial-both",
        ]
        .map(|law| pairing_id(law, Some("tangential-matchability-v1"), 17, 3, 5, (4, 6)).unwrap());
        assert!(scoped.iter().all(|identity| identity == &scoped[0]));
        assert_ne!(prior, scoped[0]);
        assert!(scoped[0].contains("scope=tangential-matchability-v1"));
    }
}
