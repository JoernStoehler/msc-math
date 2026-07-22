//! Cheap discovery scan over the orientation of two retained high-sys bodies.
//!
//! The Euclidean body is fixed.  The two parameters cover SO(4)/U(2), hence
//! change only its orientation relative to the standard symplectic form.

use exp_sys_landscape::{
    capacity_auto, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use nalgebra::{Matrix4, Vector4};
use num_rational::BigRational;
use serde_json::{json, Value};
use std::f64::consts::{PI, TAU};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::exact::omega_signs_exact;

const DEFAULT_OUTPUT: &str =
    "experiments/sys-landscape/fixed-shape-orientation-search/evaluations.jsonl";
const SOURCES: [(&str, &str); 2] = [
    (
        "generic",
        "experiments/sys-datascience/produce/random.jsonl",
    ),
    (
        "product",
        "experiments/sys-datascience/produce/random-product.jsonl",
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
struct Best {
    theta: f64,
    phi: f64,
    sys: f64,
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
    let base = SysLandscapePolytopeCache::from_f64_dual_vertices(duals)
        .ok_or_else(|| format!("base reconstruction failed for {}", path.display()))?;
    let volume = exact_volume_from_incidence_as_f64(&base.vertices, &base.vertex_facet_incidence);
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

fn evaluate(
    body: &SourceBody,
    stage: &str,
    theta: f64,
    phi: f64,
    baseline_sys: f64,
    writer: &mut BufWriter<File>,
) -> Result<Best, String> {
    let map = orientation(theta, phi);
    let orthogonality_error = (map.transpose() * map - Matrix4::identity()).norm();
    if orthogonality_error > 1e-12 {
        return Err(format!(
            "orientation is not orthogonal: {orthogonality_error}"
        ));
    }
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
            "capacity failed for {} at ({theta},{phi}): {error:?}",
            body.name
        )
    })?;
    let capacity_runtime_ms = capacity_start.elapsed().as_secs_f64() * 1000.0;
    let capacity = result.min_action;
    let sys = symplectic::systolic_ratio(capacity, body.volume);
    serde_json::to_writer(
        &mut *writer,
        &json!({
            "schema": "fixed-shape-orientation-search-v1",
            "source_kind": body.kind,
            "source_name": body.name,
            "source_recorded_sys": body.recorded_sys,
            "facet_count": body.base.dual_vertices_f64.len(),
            "stage": stage,
            "theta": theta,
            "phi": phi,
            "sys": sys,
            "delta_from_recomputed_identity": if stage == "identity" { 0.0 } else { sys - baseline_sys },
            "capacity": capacity,
            "volume": body.volume,
            "capacity_runtime_ms": capacity_runtime_ms,
            "orthogonality_error": orthogonality_error,
        }),
    )
    .map_err(|error| format!("serialize output: {error}"))?;
    writeln!(writer).map_err(|error| format!("write output: {error}"))?;
    Ok(Best { theta, phi, sys })
}

fn update_best(best: &mut Best, candidate: Best) {
    if candidate.sys > best.sys {
        *best = candidate;
    }
}

fn scan_body(
    body: &SourceBody,
    writer: &mut BufWriter<File>,
) -> Result<(Best, f64, usize), String> {
    let identity = evaluate(body, "identity", 0.0, 0.0, 0.0, writer)?;
    let baseline_sys = identity.sys;
    let mut best = identity;
    let mut evaluations = 1;

    for theta_index in 1..=8 {
        let theta = (theta_index as f64) * PI / 16.0;
        let phi_count = if theta_index == 8 { 1 } else { 16 };
        for phi_index in 0..phi_count {
            let phi = (phi_index as f64) * TAU / (phi_count as f64);
            let candidate = evaluate(body, "coarse", theta, phi, baseline_sys, writer)?;
            update_best(&mut best, candidate);
            evaluations += 1;
        }
    }

    let mut theta_step = PI / 64.0;
    let mut phi_step = TAU / 64.0;
    for round in 0..2 {
        let center = best;
        for theta_offset in -2..=2 {
            for phi_offset in -2..=2 {
                let theta =
                    (center.theta + (theta_offset as f64) * theta_step).clamp(0.0, PI / 2.0);
                let phi = (center.phi + (phi_offset as f64) * phi_step).rem_euclid(TAU);
                let stage = if round == 0 { "refine-1" } else { "refine-2" };
                let candidate = evaluate(body, stage, theta, phi, baseline_sys, writer)?;
                update_best(&mut best, candidate);
                evaluations += 1;
            }
        }
        theta_step /= 4.0;
        phi_step /= 4.0;
    }
    Ok((best, baseline_sys, evaluations))
}

fn output_path() -> Result<PathBuf, String> {
    let mut args = std::env::args().skip(1);
    match args.next() {
        None => Ok(PathBuf::from(DEFAULT_OUTPUT)),
        Some(flag) if flag == "--output" => {
            let path = args.next().ok_or("--output needs PATH")?;
            if args.next().is_some() {
                return Err("unexpected arguments after --output PATH".into());
            }
            Ok(PathBuf::from(path))
        }
        Some(other) => Err(format!(
            "unexpected argument {other}; expected --output PATH"
        )),
    }
}

fn main() -> Result<(), String> {
    let bodies = SOURCES
        .iter()
        .map(|(kind, path)| read_best(kind, Path::new(path)))
        .collect::<Result<Vec<_>, _>>()?;
    let output_path = output_path()?;
    let output = File::create(&output_path)
        .map_err(|error| format!("create {}: {error}", output_path.display()))?;
    let mut writer = BufWriter::new(output);
    for body in bodies {
        let (best, baseline, evaluations) = scan_body(&body, &mut writer)?;
        println!(
            "{} {}: identity={:.12}, best={:.12}, delta={:+.12}, theta={:.6}, phi={:.6}, evaluations={}",
            body.kind,
            body.name,
            baseline,
            best.sys,
            best.sys - baseline,
            best.theta,
            best.phi,
            evaluations,
        );
    }
    writer
        .flush()
        .map_err(|error| format!("flush {}: {error}", output_path.display()))?;
    println!("wrote {}", output_path.display());
    Ok(())
}
