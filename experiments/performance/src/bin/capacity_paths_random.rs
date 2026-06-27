use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, F64CapacityMethod, F64CapacityOutcome,
    F64ValidationPolicy,
};
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
    OrbitKktData,
};

const TARGET_NAME: &str = "capacity-paths-random";
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;
const MAX_ATTEMPTS_PER_SAMPLE: u64 = 10_000;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    out_dir: Option<PathBuf>,
}

#[derive(Clone)]
struct Geometry {
    dual_vertices_f64: Vec<Vector4<f64>>,
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: DMatrix<bool>,
    omega_signs: DMatrix<i8>,
}

#[derive(Serialize)]
struct MetadataRow {
    target: &'static str,
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
}

#[derive(Serialize)]
struct PathEvent {
    target: &'static str,
    mode: RunMode,
    facet_count: usize,
    sample: usize,
    seed: u64,
    path: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sigma_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indeterminate_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_abs_diff_from_fallback: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(out_dir) => {
            println!("{}", out_dir.display());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<PathBuf, String> {
    let config = parse_args(env::args().skip(1))?;
    validate_config(&config)?;
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.mode.as_str())?;
    write_metadata(&config, &out_dir)?;

    let mut events = JsonlWriter::create(&out_dir.join("path-events.jsonl"))?;
    for &facet_count in &config.facet_counts {
        for sample in 0..config.samples {
            profile_sample(&config, facet_count, sample, &mut events)?;
        }
    }
    events.flush()?;
    Ok(out_dir)
}

fn write_metadata(config: &Config, out_dir: &std::path::Path) -> Result<(), String> {
    let mut writer = JsonlWriter::create(&out_dir.join("metadata.jsonl"))?;
    writer.write(&MetadataRow {
        target: TARGET_NAME,
        mode: config.mode,
        facet_counts: config.facet_counts.clone(),
        samples: config.samples,
        seed: config.seed,
        h_min: config.h_min,
        h_max: config.h_max,
    })?;
    writer.flush()
}

fn profile_sample(
    config: &Config,
    facet_count: usize,
    sample: usize,
    events: &mut JsonlWriter,
) -> Result<(), String> {
    let dual_vertices = accepted_fixture(config, facet_count, sample)?;
    let geometry = exact_geometry(dual_vertices);
    let fallback = pruned_hk_exact(config, facet_count, sample, &geometry);
    let fallback_capacity = fallback.capacity;
    events.write(&fallback)?;

    let mut f64_event = f64_hk(config, facet_count, sample, &geometry.dual_vertices_f64);
    f64_event.capacity_abs_diff_from_fallback = match (f64_event.capacity, fallback_capacity) {
        (Some(left), Some(right)) => Some((left - right).abs()),
        _ => None,
    };
    events.write(&f64_event)
}

fn accepted_fixture(
    config: &Config,
    facet_count: usize,
    sample: usize,
) -> Result<Vec<Vector4<f64>>, String> {
    let first_attempt = facet_count as u64 * 1_000_000 + sample as u64 * MAX_ATTEMPTS_PER_SAMPLE;
    for offset in 0..MAX_ATTEMPTS_PER_SAMPLE {
        if let Ok(dual_vertices) = generate_dual_vertices(
            facet_count,
            config.h_min,
            config.h_max,
            config.seed,
            first_attempt + offset,
        ) {
            return Ok(dual_vertices);
        }
    }
    Err(format!(
        "no accepted fixture for F={facet_count}, sample={sample}"
    ))
}

fn exact_geometry(dual_vertices_f64: Vec<Vector4<f64>>) -> Geometry {
    let dual_vertices_exact = exact_dual_vertex_arrays(&dual_vertices_f64);
    let dual_vertices_exact_vectors = exact_dual_vertex_vectors(&dual_vertices_exact);
    let PolarVerticesExact {
        vertex_facet_incidence,
        ..
    } = polar_vertices_exact_rational_assuming_origin_interior(&dual_vertices_exact_vectors);
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&dual_vertices_exact_vectors);
    Geometry {
        dual_vertices_f64,
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
    }
}

fn pruned_hk_exact(
    config: &Config,
    facet_count: usize,
    sample: usize,
    geometry: &Geometry,
) -> PathEvent {
    let started = Instant::now();
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    match solve_pruned_hk2017_candidates(&geometry.dual_vertices_f64, &transition_is_allowed)
        .and_then(|(orbits, iterations)| aggregate(geometry, orbits, iterations))
    {
        Ok((capacity, iterations, raw_orbits, returned_orbits)) => PathEvent {
            target: TARGET_NAME,
            mode: config.mode,
            facet_count,
            sample,
            seed: config.seed,
            path: "pruned_hk_exact_fallback",
            elapsed_ms: elapsed_ms(started),
            status: "ok",
            capacity: Some(capacity),
            iterations: Some(iterations),
            raw_orbits: Some(raw_orbits),
            returned_orbits: Some(returned_orbits),
            sigma_count: None,
            admissible_f64_count: None,
            indeterminate_f64_count: None,
            capacity_abs_diff_from_fallback: None,
            error: None,
        },
        Err(error) => error_event(
            config,
            facet_count,
            sample,
            "pruned_hk_exact_fallback",
            elapsed_ms(started),
            format!("{error:?}"),
        ),
    }
}

fn aggregate(
    geometry: &Geometry,
    orbits: Vec<OrbitKktData>,
    iterations: u64,
) -> Result<(f64, u64, usize, usize), symplectic::OrbitSearchError> {
    let raw_orbits = orbits.len();
    let result = aggregate_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )?;
    Ok((
        result.min_action,
        iterations,
        raw_orbits,
        result.orbits.len(),
    ))
}

fn f64_hk(
    config: &Config,
    facet_count: usize,
    sample: usize,
    dual_vertices: &[Vector4<f64>],
) -> PathEvent {
    let started = Instant::now();
    let (report, _) = capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    let elapsed_ms = elapsed_ms(started);
    match report.outcome {
        F64CapacityOutcome::Success { capacity, .. } => PathEvent {
            target: TARGET_NAME,
            mode: config.mode,
            facet_count,
            sample,
            seed: config.seed,
            path: "f64_transition_pruned_hk",
            elapsed_ms,
            status: "ok",
            capacity: Some(capacity),
            iterations: None,
            raw_orbits: None,
            returned_orbits: None,
            sigma_count: Some(report.sigma_count),
            admissible_f64_count: Some(report.admissible_f64_count),
            indeterminate_f64_count: Some(report.indeterminate_f64_count),
            capacity_abs_diff_from_fallback: None,
            error: None,
        },
        F64CapacityOutcome::Failure { reason } => error_event(
            config,
            facet_count,
            sample,
            "f64_transition_pruned_hk",
            elapsed_ms,
            format!("{reason:?}"),
        ),
    }
}

fn error_event(
    config: &Config,
    facet_count: usize,
    sample: usize,
    path: &'static str,
    elapsed_ms: f64,
    error: String,
) -> PathEvent {
    PathEvent {
        target: TARGET_NAME,
        mode: config.mode,
        facet_count,
        sample,
        seed: config.seed,
        path,
        elapsed_ms,
        status: "error",
        capacity: None,
        iterations: None,
        raw_orbits: None,
        returned_orbits: None,
        sigma_count: None,
        admissible_f64_count: None,
        indeterminate_f64_count: None,
        capacity_abs_diff_from_fallback: None,
        error: Some(error),
    }
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn exact_dual_vertex_arrays(dual_vertices: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
    dual_vertices
        .iter()
        .map(|a| {
            [
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            ]
        })
        .collect()
}

fn exact_dual_vertex_vectors(
    dual_vertices_exact: &[[BigRational; 4]],
) -> Vec<Vector4<BigRational>> {
    dual_vertices_exact
        .iter()
        .map(|a| Vector4::new(a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()))
        .collect()
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let args: Vec<String> = args.collect();
    let mut config = config_for_mode(selected_run_mode(&args)?);
    let mut args = args.into_iter().peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--mode" => {
                let _ = take_value("--mode", inline_value, &mut args)?;
            }
            "--out-dir" => {
                config.out_dir = Some(PathBuf::from(take_value(
                    "--out-dir",
                    inline_value,
                    &mut args,
                )?));
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(config)
}

fn config_for_mode(mode: RunMode) -> Config {
    let (facet_counts, samples) = match mode {
        RunMode::Smoke => (vec![6], 1),
        RunMode::Production => (vec![6, 10], 5),
    };
    Config {
        mode,
        facet_counts,
        samples,
        seed: SEED,
        h_min: H_MIN,
        h_max: H_MAX,
        out_dir: None,
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("sample count must be positive".to_string());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 5)
    {
        return Err("facet counts must be at least 5".to_string());
    }
    if !config.h_min.is_finite()
        || !config.h_max.is_finite()
        || config.h_min <= 0.0
        || config.h_min >= config.h_max
    {
        return Err("height range must satisfy finite 0 < h_min < h_max".to_string());
    }
    Ok(())
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-performance --release --bin capacity-paths-random -- \\
        --mode production --out-dir /tmp/capacity-paths-random\n\
\n\
Options:\n\
  --mode MODE          Named run mode: smoke or production [default: smoke]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<mode>-<time>-pid<PID>]\n\
  --help               Print this help text"
}
