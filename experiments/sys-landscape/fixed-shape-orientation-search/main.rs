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
use symplectic::algorithms::capacity_4d::{
    capacity, capacity_transition_graph, capacity_value, check_dual_vertex_norm_bounds,
    check_facet_count, check_finite_dual_vertices, check_primal_vertex_norm_bounds,
    exact_binary64_polytope_geometry,
};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::exact::omega_signs_exact;

const DEFAULT_OUTPUT: &str =
    "experiments/sys-landscape/fixed-shape-orientation-search/evaluations.jsonl";
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
struct Best {
    theta: f64,
    phi: f64,
    sys: f64,
}

#[derive(Clone, Copy, Debug)]
enum CapacityBackend {
    Legacy,
    Production,
}

impl CapacityBackend {
    fn name(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::Production => "production",
        }
    }
}

struct Args {
    output: PathBuf,
    source_kind: Option<String>,
    maximum_evaluations: Option<usize>,
    backend: CapacityBackend,
    profile_stages: bool,
}

#[derive(Clone, Copy, Default)]
struct EvaluationTiming {
    preparation_ms: f64,
    candidate_diagnostic_ms: f64,
    capacity_ms: f64,
    total_ms: f64,
    candidate_count: Option<usize>,
}

impl std::ops::AddAssign for EvaluationTiming {
    fn add_assign(&mut self, other: Self) {
        self.preparation_ms += other.preparation_ms;
        self.candidate_diagnostic_ms += other.candidate_diagnostic_ms;
        self.capacity_ms += other.capacity_ms;
        self.total_ms += other.total_ms;
        self.candidate_count = match (self.candidate_count, other.candidate_count) {
            (Some(left), Some(right)) => Some(left + right),
            (None, Some(right)) => Some(right),
            _ => None,
        };
    }
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
    backend: CapacityBackend,
    profile_stages: bool,
    writer: &mut BufWriter<File>,
) -> Result<(Best, EvaluationTiming), String> {
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
    let (capacity_value_observed, timing) = match backend {
        CapacityBackend::Legacy => {
            let preparation_started = Instant::now();
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
            let preparation_ms = preparation_started.elapsed().as_secs_f64() * 1000.0;
            let capacity_started = Instant::now();
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
            let capacity_ms = capacity_started.elapsed().as_secs_f64() * 1000.0;
            (
                result.min_action,
                EvaluationTiming {
                    preparation_ms,
                    capacity_ms,
                    total_ms: preparation_ms + capacity_ms,
                    ..EvaluationTiming::default()
                },
            )
        }
        CapacityBackend::Production => {
            let preparation_started = Instant::now();
            check_facet_count(duals.len()).map_err(|error| error.to_string())?;
            check_finite_dual_vertices(&duals).map_err(|error| error.to_string())?;
            check_dual_vertex_norm_bounds(&duals).map_err(|error| error.to_string())?;
            let geometry =
                exact_binary64_polytope_geometry(&duals).map_err(|error| error.to_string())?;
            check_primal_vertex_norm_bounds(&geometry).map_err(|error| error.to_string())?;
            let preparation_ms = preparation_started.elapsed().as_secs_f64() * 1000.0;

            let (candidate_diagnostic_ms, candidate_count) = if profile_stages {
                let candidate_started = Instant::now();
                let transition = capacity_transition_graph(&geometry);
                let count = SimpleDirectedCyclesCanonical::new(&transition).count();
                (
                    candidate_started.elapsed().as_secs_f64() * 1000.0,
                    Some(count),
                )
            } else {
                (0.0, None)
            };

            let capacity_started = Instant::now();
            let result = capacity(&geometry).map_err(|error| error.to_string())?;
            let observed = capacity_value(&result, 1e-10).map_err(|error| error.to_string())?;
            let capacity_ms = capacity_started.elapsed().as_secs_f64() * 1000.0;
            (
                observed,
                EvaluationTiming {
                    preparation_ms,
                    candidate_diagnostic_ms,
                    capacity_ms,
                    total_ms: preparation_ms + capacity_ms,
                    candidate_count,
                },
            )
        }
    };
    let capacity = capacity_value_observed;
    let sys = symplectic::systolic_ratio(capacity, body.volume);
    serde_json::to_writer(
        &mut *writer,
        &json!({
            "schema": "fixed-shape-orientation-search-v1",
            "source_kind": body.kind,
            "source_name": body.name,
            "source_recorded_sys": body.recorded_sys,
            "facet_count": body.base.dual_vertices_f64.len(),
            "capacity_backend": backend.name(),
            "stage": stage,
            "theta": theta,
            "phi": phi,
            "sys": sys,
            "delta_from_recomputed_identity": if stage == "identity" { 0.0 } else { sys - baseline_sys },
            "capacity": capacity,
            "volume": body.volume,
            "preparation_runtime_ms": timing.preparation_ms,
            "candidate_diagnostic_runtime_ms": timing.candidate_diagnostic_ms,
            "candidate_count": timing.candidate_count,
            "capacity_runtime_ms": timing.capacity_ms,
            "capacity_pipeline_runtime_ms": timing.total_ms,
            "orthogonality_error": orthogonality_error,
        }),
    )
    .map_err(|error| format!("serialize output: {error}"))?;
    writeln!(writer).map_err(|error| format!("write output: {error}"))?;
    Ok((Best { theta, phi, sys }, timing))
}

fn update_best(best: &mut Best, candidate: Best) {
    if candidate.sys > best.sys {
        *best = candidate;
    }
}

fn scan_body(
    body: &SourceBody,
    args: &Args,
    writer: &mut BufWriter<File>,
) -> Result<(Best, f64, usize, EvaluationTiming), String> {
    let (identity, identity_timing) = evaluate(
        body,
        "identity",
        0.0,
        0.0,
        0.0,
        args.backend,
        args.profile_stages,
        writer,
    )?;
    let baseline_sys = identity.sys;
    let mut best = identity;
    let mut evaluations = 1;
    let mut timing = identity_timing;
    if args.maximum_evaluations == Some(evaluations) {
        return Ok((best, baseline_sys, evaluations, timing));
    }

    for theta_index in 1..=8 {
        let theta = (theta_index as f64) * PI / 16.0;
        let phi_count = if theta_index == 8 { 1 } else { 16 };
        for phi_index in 0..phi_count {
            if args.maximum_evaluations == Some(evaluations) {
                return Ok((best, baseline_sys, evaluations, timing));
            }
            let phi = (phi_index as f64) * TAU / (phi_count as f64);
            let (candidate, evaluation_timing) = evaluate(
                body,
                "coarse",
                theta,
                phi,
                baseline_sys,
                args.backend,
                args.profile_stages,
                writer,
            )?;
            update_best(&mut best, candidate);
            timing += evaluation_timing;
            evaluations += 1;
        }
    }

    let mut theta_step = PI / 64.0;
    let mut phi_step = TAU / 64.0;
    for round in 0..2 {
        let center = best;
        for theta_offset in -2..=2 {
            for phi_offset in -2..=2 {
                if args.maximum_evaluations == Some(evaluations) {
                    return Ok((best, baseline_sys, evaluations, timing));
                }
                let theta =
                    (center.theta + (theta_offset as f64) * theta_step).clamp(0.0, PI / 2.0);
                let phi = (center.phi + (phi_offset as f64) * phi_step).rem_euclid(TAU);
                let stage = if round == 0 { "refine-1" } else { "refine-2" };
                let (candidate, evaluation_timing) = evaluate(
                    body,
                    stage,
                    theta,
                    phi,
                    baseline_sys,
                    args.backend,
                    args.profile_stages,
                    writer,
                )?;
                update_best(&mut best, candidate);
                timing += evaluation_timing;
                evaluations += 1;
            }
        }
        theta_step /= 4.0;
        phi_step /= 4.0;
    }
    Ok((best, baseline_sys, evaluations, timing))
}

fn parse_args() -> Result<Args, String> {
    let mut output = PathBuf::from(DEFAULT_OUTPUT);
    let mut source_kind = None;
    let mut maximum_evaluations = None;
    let mut backend = CapacityBackend::Legacy;
    let mut profile_stages = false;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--output" => {
                output = PathBuf::from(args.next().ok_or("--output needs PATH")?);
            }
            "--source-kind" => {
                let value = args
                    .next()
                    .ok_or("--source-kind needs generic or product")?;
                if value != "generic" && value != "product" {
                    return Err("--source-kind needs generic or product".into());
                }
                source_kind = Some(value);
            }
            "--maximum-evaluations" => {
                let value = args
                    .next()
                    .ok_or("--maximum-evaluations needs a positive integer")?
                    .parse::<usize>()
                    .map_err(|_| "--maximum-evaluations needs a positive integer")?;
                if value == 0 {
                    return Err("--maximum-evaluations needs a positive integer".into());
                }
                maximum_evaluations = Some(value);
            }
            "--capacity-backend" => {
                backend = match args.next().as_deref() {
                    Some("legacy") => CapacityBackend::Legacy,
                    Some("production") => CapacityBackend::Production,
                    _ => return Err("--capacity-backend needs legacy or production".into()),
                };
            }
            "--profile-stages" => profile_stages = true,
            "--help" => {
                println!(
                    "usage: sys-fixed-shape-orientation-search \
                     [--output PATH] [--source-kind generic|product] \
                     [--maximum-evaluations N] \
                     [--capacity-backend legacy|production] [--profile-stages]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unexpected argument {other}")),
        }
    }
    Ok(Args {
        output,
        source_kind,
        maximum_evaluations,
        backend,
        profile_stages,
    })
}

fn main() -> Result<(), String> {
    let args = parse_args()?;
    let bodies = SOURCES
        .iter()
        .filter(|(kind, _)| {
            args.source_kind
                .as_deref()
                .is_none_or(|wanted| wanted == *kind)
        })
        .map(|(kind, path)| read_best(kind, Path::new(path)))
        .collect::<Result<Vec<_>, _>>()?;
    let output = File::create(&args.output)
        .map_err(|error| format!("create {}: {error}", args.output.display()))?;
    let mut writer = BufWriter::new(output);
    for body in bodies {
        let (best, baseline, evaluations, timing) = scan_body(&body, &args, &mut writer)?;
        println!(
            "{} {}: backend={}, identity={:.12}, best={:.12}, delta={:+.12}, theta={:.6}, phi={:.6}, evaluations={}, preparation_ms={:.3}, candidate_diagnostic_ms={:.3}, capacity_ms={:.3}, pipeline_ms={:.3}, candidates={:?}",
            body.kind,
            body.name,
            args.backend.name(),
            baseline,
            best.sys,
            best.sys - baseline,
            best.theta,
            best.phi,
            evaluations,
            timing.preparation_ms,
            timing.candidate_diagnostic_ms,
            timing.capacity_ms,
            timing.total_ms,
            timing.candidate_count,
        );
    }
    writer
        .flush()
        .map_err(|error| format!("flush {}: {error}", args.output.display()))?;
    println!("wrote {}", args.output.display());
    Ok(())
}
