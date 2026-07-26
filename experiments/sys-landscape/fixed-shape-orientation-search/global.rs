//! Global determinant-one linear scan of two retained high-sys bodies.
//!
//! The compact stratum is the existing SO(4)/U(2) orientation experiment.
//! The remaining parameters cover the noncompact directions of
//! Sp(4)\SL(4), represented by normalized nondegenerate skew forms.

use euclidean_polytopes::volume_from_incidence_f64;
use exp_sys_landscape::{capacity_auto, SysLandscapePolytopeCache};
use nalgebra::{Matrix4, Vector4};
use num_rational::BigRational;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_json::{json, Value};
use std::f64::consts::TAU;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::algorithms::capacity_4d::exact_binary64_polytope_geometry;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::symplectic_form::j4;

const DEFAULT_OUTPUT: &str =
    "experiments/sys-landscape/fixed-shape-orientation-search/global-evaluations.jsonl";
const COMPACT_OUTPUT: &str =
    "experiments/sys-landscape/fixed-shape-orientation-search/evaluations.jsonl";
const DEFAULT_SEED: u64 = 0x51_34_4c_34;
const DEFAULT_SAMPLES: usize = 24;
const RADII: [f64; 6] = [0.125, 0.25, 0.5, 1.0, 2.0, 4.0];
const SOURCES: [(&str, &str); 2] = [
    ("generic", "experiments/polytope-datasets/random.jsonl"),
    (
        "product",
        "experiments/polytope-datasets/random-product.jsonl",
    ),
];

#[derive(Clone)]
struct SourceBody {
    kind: &'static str,
    name: String,
    recorded_sys: f64,
    base: SysLandscapePolytopeCache,
    volume: f64,
}

#[derive(Clone, Copy)]
struct CompactBest {
    theta: f64,
    phi: f64,
    sys: f64,
}

#[derive(Clone, Copy)]
struct Point {
    theta: f64,
    phi: f64,
    v_polar: f64,
    v_azimuth: f64,
    radius: f64,
}

#[derive(Clone, Copy)]
struct Best {
    point: Point,
    sys: f64,
}

struct Args {
    output: PathBuf,
    samples: usize,
    seed: u64,
}

fn read_best(kind: &'static str, path: &Path) -> Result<SourceBody, String> {
    let file = File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    let mut best: Option<Value> = None;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line =
            line.map_err(|error| format!("read {}:{}: {error}", path.display(), index + 1))?;
        let row: Value = serde_json::from_str(&line)
            .map_err(|error| format!("parse {}:{}: {error}", path.display(), index + 1))?;
        let sys = row
            .get("sys")
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("missing sys in {}:{}", path.display(), index + 1))?;
        if best
            .as_ref()
            .and_then(|current| current.get("sys"))
            .and_then(Value::as_f64)
            .is_none_or(|current| sys > current)
        {
            best = Some(row);
        }
    }
    let row = best.ok_or_else(|| format!("empty source {}", path.display()))?;
    let values = row
        .get("dual_vertices")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing dual_vertices in {}", path.display()))?;
    let duals = values
        .iter()
        .map(|value| {
            let coordinates = value.as_array().ok_or("dual vertex is not an array")?;
            if coordinates.len() != 4 {
                return Err("dual vertex is not four-dimensional".to_string());
            }
            let x = coordinates
                .iter()
                .map(|coordinate| coordinate.as_f64().ok_or("dual coordinate is not f64"))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Vector4::new(x[0], x[1], x[2], x[3]))
        })
        .collect::<Result<Vec<_>, String>>()?;
    // Match the compact scan's production geometry path. The older cache
    // constructor repeats exact origin/extremality checks before polar
    // construction.
    let geometry = exact_binary64_polytope_geometry(&duals)
        .map_err(|error| format!("base reconstruction failed: {error}"))?;
    let dual_vertices = geometry
        .dual_vertices_exact
        .iter()
        .map(|vertex| std::array::from_fn(|coordinate| vertex[coordinate].clone()))
        .collect();
    let vertices = geometry
        .primal_vertices_exact
        .iter()
        .map(|vertex| std::array::from_fn(|coordinate| vertex[coordinate].clone()))
        .collect();
    let base = SysLandscapePolytopeCache::from_rational_parts(dual_vertices, vertices)
        .ok_or_else(|| format!("base reconstruction failed for {}", path.display()))?;
    let volume = volume_from_incidence_f64(&base.vertices_f64, &base.vertex_facet_incidence)
        .map_err(|error| format!("base f64 volume failed: {error:?}"))?;
    Ok(SourceBody {
        kind,
        name: row
            .get("name")
            .and_then(Value::as_str)
            .ok_or("missing source name")?
            .to_owned(),
        recorded_sys: row.get("sys").and_then(Value::as_f64).unwrap(),
        base,
        volume,
    })
}

fn read_compact_best(kind: &str, path: &Path) -> Result<CompactBest, String> {
    let file = File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    let mut best: Option<CompactBest> = None;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line =
            line.map_err(|error| format!("read {}:{}: {error}", path.display(), index + 1))?;
        let row: Value = serde_json::from_str(&line)
            .map_err(|error| format!("parse {}:{}: {error}", path.display(), index + 1))?;
        if row.get("source_kind").and_then(Value::as_str) != Some(kind) {
            continue;
        }
        let candidate = CompactBest {
            theta: row
                .get("theta")
                .and_then(Value::as_f64)
                .ok_or("missing theta")?,
            phi: row
                .get("phi")
                .and_then(Value::as_f64)
                .ok_or("missing phi")?,
            sys: row
                .get("sys")
                .and_then(Value::as_f64)
                .ok_or("missing sys")?,
        };
        if best.is_none_or(|current| candidate.sys > current.sys) {
            best = Some(candidate);
        }
    }
    best.ok_or_else(|| format!("no compact result for {kind} in {}", path.display()))
}

fn standard_omega_matrix() -> Matrix4<f64> {
    // The crate defines omega(u,v) = <J_0 u,v> = u^T (-J_0) v.
    -j4()
}

fn orientation(theta: f64, phi: f64) -> Matrix4<f64> {
    #[rustfmt::skip]
    let k1 = Matrix4::new(
         0.0, -1.0,  0.0,  0.0,
         1.0,  0.0,  0.0,  0.0,
         0.0,  0.0,  0.0,  1.0,
         0.0,  0.0, -1.0,  0.0,
    );
    #[rustfmt::skip]
    let k2 = Matrix4::new(
         0.0,  0.0,  0.0, -1.0,
         0.0,  0.0,  1.0,  0.0,
         0.0, -1.0,  0.0,  0.0,
         1.0,  0.0,  0.0,  0.0,
    );
    let generator = phi.cos() * k1 + phi.sin() * k2;
    theta.cos() * Matrix4::identity() + theta.sin() * generator
}

fn complementary_form(polar: f64, azimuth: f64) -> Matrix4<f64> {
    #[rustfmt::skip]
    let b1 = Matrix4::new(
         0.0,  1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0,  0.0,
         0.0,  0.0,  0.0,  1.0,
         0.0,  0.0, -1.0,  0.0,
    );
    #[rustfmt::skip]
    let b2 = Matrix4::new(
         0.0,  0.0,  1.0,  0.0,
         0.0,  0.0,  0.0, -1.0,
        -1.0,  0.0,  0.0,  0.0,
         0.0,  1.0,  0.0,  0.0,
    );
    #[rustfmt::skip]
    let b3 = Matrix4::new(
         0.0,  0.0,  0.0,  1.0,
         0.0,  0.0,  1.0,  0.0,
         0.0, -1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0,  0.0,
    );
    polar.cos() * b1 + polar.sin() * (azimuth.cos() * b2 + azimuth.sin() * b3)
}

/// Representative of a point in Sp(4)\SL(4).
///
/// If `u = O^T J O`, then `u` commutes with the complementary unit form `v`.
/// The symmetric involution `a = -uv` has trace zero. Consequently
/// `D = exp((r/2)a)`, `M = OD`, and
/// `M^T J M = cosh(r)u + sinh(r)v`, with `det(M)=1`.
fn linear_map(point: Point) -> Result<(Matrix4<f64>, f64, f64), String> {
    let o = orientation(point.theta, point.phi);
    let omega = standard_omega_matrix();
    let u = o.transpose() * omega * o;
    let v = complementary_form(point.v_polar, point.v_azimuth);
    let a = -u * v;
    let half = point.radius / 2.0;
    let d = half.cosh() * Matrix4::identity() + half.sinh() * a;
    let map = o * d;

    let target = point.radius.cosh() * u + point.radius.sinh() * v;
    let pullback_error = (map.transpose() * omega * map - target).norm();
    let determinant_error = (map.determinant() - 1.0).abs();
    if pullback_error > 2e-10 || determinant_error > 2e-10 {
        return Err(format!(
            "invalid quotient representative: pullback error {pullback_error:e}, determinant error {determinant_error:e}"
        ));
    }
    Ok((map, pullback_error, determinant_error))
}

fn evaluate(
    body: &SourceBody,
    stage: &str,
    point: Point,
    compact_baseline_sys: f64,
    writer: &mut BufWriter<File>,
) -> Result<Best, String> {
    let (map, pullback_error, determinant_error) = linear_map(point)?;
    let duals = body
        .base
        .dual_vertices_f64
        .iter()
        .map(|dual| map * dual)
        .collect::<Vec<_>>();
    let exact_vectors = duals
        .iter()
        .map(|dual| {
            Vector4::new(
                BigRational::from_float(dual[0]).unwrap(),
                BigRational::from_float(dual[1]).unwrap(),
                BigRational::from_float(dual[2]).unwrap(),
                BigRational::from_float(dual[3]).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let exact_arrays = exact_vectors
        .iter()
        .map(|dual| {
            [
                dual[0].clone(),
                dual[1].clone(),
                dual[2].clone(),
                dual[3].clone(),
            ]
        })
        .collect::<Vec<_>>();
    let omega_signs = omega_signs_exact(&exact_vectors);
    let capacity_start = Instant::now();
    let result = capacity_auto(
        &duals,
        &exact_arrays,
        &body.base.facet_intersection_is_nonempty,
        &omega_signs,
    )
    .map_err(|error| {
        format!(
            "capacity failed for {} at radius {}: {error:?}",
            body.name, point.radius
        )
    })?;
    let capacity_runtime_ms = capacity_start.elapsed().as_secs_f64() * 1000.0;
    let capacity = result.min_action;
    let sys = symplectic::systolic_ratio(capacity, body.volume);
    serde_json::to_writer(
        &mut *writer,
        &json!({
            "schema": "fixed-shape-linear-search-v1",
            "source_kind": body.kind,
            "source_name": body.name,
            "source_recorded_sys": body.recorded_sys,
            "facet_count": body.base.dual_vertices_f64.len(),
            "stage": stage,
            "theta": point.theta,
            "phi": point.phi,
            "v_polar": point.v_polar,
            "v_azimuth": point.v_azimuth,
            "radius": point.radius,
            "map_condition_number": point.radius.exp(),
            "sys": sys,
            "delta_from_compact_best": sys - compact_baseline_sys,
            "capacity": capacity,
            "volume": body.volume,
            "capacity_runtime_ms": capacity_runtime_ms,
            "pullback_error": pullback_error,
            "determinant_error": determinant_error,
        }),
    )
    .map_err(|error| format!("serialize output: {error}"))?;
    writeln!(writer).map_err(|error| format!("write output: {error}"))?;
    Ok(Best { point, sys })
}

fn update_best(best: &mut Best, candidate: Best) {
    if candidate.sys > best.sys {
        *best = candidate;
    }
}

fn random_u(rng: &mut ChaCha8Rng) -> (f64, f64) {
    let z = rng.gen_range(-1.0_f64..=1.0);
    (0.5 * z.acos(), rng.gen_range(0.0..TAU))
}

fn random_v(rng: &mut ChaCha8Rng) -> (f64, f64) {
    let z = rng.gen_range(-1.0_f64..=1.0);
    (z.acos(), rng.gen_range(0.0..TAU))
}

fn scan_body(
    body: &SourceBody,
    compact: CompactBest,
    samples: usize,
    seed: u64,
    writer: &mut BufWriter<File>,
) -> Result<(Best, usize), String> {
    let control_point = Point {
        theta: compact.theta,
        phi: compact.phi,
        v_polar: 0.0,
        v_azimuth: 0.0,
        radius: 0.0,
    };
    let mut best = evaluate(body, "compact-control", control_point, compact.sys, writer)?;
    if (best.sys - compact.sys).abs() > 2e-10 {
        return Err(format!(
            "{} compact control changed: artifact {}, recomputed {}",
            body.kind, compact.sys, best.sys
        ));
    }
    let mut evaluations = 1;
    let body_offset = if body.kind == "generic" { 0 } else { 1 };
    let mut rng = ChaCha8Rng::seed_from_u64(seed.wrapping_add(body_offset));

    // Common angular directions across radii expose radial behavior instead of
    // confounding it with a fresh draw at every distortion scale.
    let global_directions = (0..samples)
        .map(|_| {
            let (theta, phi) = random_u(&mut rng);
            let (v_polar, v_azimuth) = random_v(&mut rng);
            (theta, phi, v_polar, v_azimuth)
        })
        .collect::<Vec<_>>();
    let anchored_directions = (0..samples.div_ceil(2))
        .map(|_| random_v(&mut rng))
        .collect::<Vec<_>>();

    for radius in RADII {
        for &(theta, phi, v_polar, v_azimuth) in &global_directions {
            let candidate = evaluate(
                body,
                "global",
                Point {
                    theta,
                    phi,
                    v_polar,
                    v_azimuth,
                    radius,
                },
                compact.sys,
                writer,
            )?;
            update_best(&mut best, candidate);
            evaluations += 1;
        }
        for &(v_polar, v_azimuth) in &anchored_directions {
            let candidate = evaluate(
                body,
                "compact-u",
                Point {
                    theta: compact.theta,
                    phi: compact.phi,
                    v_polar,
                    v_azimuth,
                    radius,
                },
                compact.sys,
                writer,
            )?;
            update_best(&mut best, candidate);
            evaluations += 1;
        }
    }
    Ok((best, evaluations))
}

fn parse_args() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let mut output = PathBuf::from(DEFAULT_OUTPUT);
    let mut samples = DEFAULT_SAMPLES;
    let mut seed = DEFAULT_SEED;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--output" => output = PathBuf::from(args.next().ok_or("--output needs PATH")?),
            "--samples" => {
                samples = args
                    .next()
                    .ok_or("--samples needs N")?
                    .parse()
                    .map_err(|_| "--samples must be an integer")?;
                if samples == 0 {
                    return Err("--samples must be positive".into());
                }
            }
            "--seed" => {
                seed = args
                    .next()
                    .ok_or("--seed needs N")?
                    .parse()
                    .map_err(|_| "--seed must be a u64")?;
            }
            other => return Err(format!("unexpected argument {other}")),
        }
    }
    Ok(Args {
        output,
        samples,
        seed,
    })
}

fn main() -> Result<(), String> {
    let args = parse_args()?;
    let bodies = SOURCES
        .iter()
        .map(|(kind, path)| read_best(kind, Path::new(path)))
        .collect::<Result<Vec<_>, _>>()?;
    let output = File::create(&args.output)
        .map_err(|error| format!("create {}: {error}", args.output.display()))?;
    let mut writer = BufWriter::new(output);
    for body in bodies {
        let compact = read_compact_best(body.kind, Path::new(COMPACT_OUTPUT))?;
        let (best, evaluations) = scan_body(&body, compact, args.samples, args.seed, &mut writer)?;
        println!(
            "{} {}: compact={:.12}, best={:.12}, delta={:+.12}, radius={:.3}, condition={:.1}, evaluations={}",
            body.kind,
            body.name,
            compact.sys,
            best.sys,
            best.sys - compact.sys,
            best.point.radius,
            best.point.radius.exp(),
            evaluations,
        );
    }
    writer
        .flush()
        .map_err(|error| format!("flush {}: {error}", args.output.display()))?;
    println!("wrote {}", args.output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotient_representatives_have_the_claimed_algebra() {
        for point in [
            Point {
                theta: 0.0,
                phi: 0.0,
                v_polar: 0.0,
                v_azimuth: 0.0,
                radius: 0.0,
            },
            Point {
                theta: 0.37,
                phi: 1.2,
                v_polar: 2.1,
                v_azimuth: 5.4,
                radius: 4.0,
            },
        ] {
            let (map, pullback_error, determinant_error) = linear_map(point).unwrap();
            assert!(pullback_error < 2e-10);
            assert!(determinant_error < 2e-10);
            if point.radius == 0.0 {
                assert!((map - orientation(point.theta, point.phi)).norm() < 1e-14);
            }
        }
    }
}
