use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_performance::{
    prepare_out_dir, run_environment, timed, timed_result, unix_timestamp_secs, write_json_file,
    JsonlWriter, RunEnvironment,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use std::env::{self, ArgsOs};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices_profiled;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
    OrbitKktData,
};
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

const TARGET_NAME: &str = "hk2017-pruned-f64";
const DEFAULT_SEED: u64 = 42;
const DEFAULT_H_MIN: f64 = 0.5;
const DEFAULT_H_MAX: f64 = 2.0;
const MAX_ATTEMPTS_PER_SAMPLE: u64 = 10_000;

#[derive(Clone, Debug, Serialize)]
struct Config {
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    trace: bool,
    out_dir: Option<PathBuf>,
}

#[derive(Clone, Debug)]
struct Invocation {
    program: OsString,
    args: Vec<String>,
}

#[derive(Serialize)]
struct RunMetadata {
    target: &'static str,
    started_unix_secs: u64,
    cwd: String,
    command: Vec<String>,
    environment: RunEnvironment,
    config: ConfigForMetadata,
    files: OutputFiles,
}

#[derive(Serialize)]
struct ConfigForMetadata {
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    max_attempts_per_sample: u64,
    trace: bool,
}

#[derive(Serialize)]
struct OutputFiles {
    phase_events_jsonl: String,
    run_metadata_json: String,
}

#[derive(Serialize)]
struct PhaseEvent {
    target: &'static str,
    facet_count: usize,
    sample: usize,
    seed: u64,
    phase: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixture_attempts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertex_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_transitions: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_sigma_len: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

struct FlatGeometry {
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: DMatrix<bool>,
    omega_signs: DMatrix<i8>,
    vertex_count: usize,
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
    let invocation = Invocation::from_env(env::args_os())?;
    let config = parse_args(invocation.args.iter().cloned())?;
    validate_config(&config)?;
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME)?;
    if config.trace {
        init_tracing()?;
    }
    let phase_events_path = out_dir.join("phase-events.jsonl");
    let metadata_path = out_dir.join("run-metadata.json");

    let metadata = RunMetadata {
        target: TARGET_NAME,
        started_unix_secs: unix_timestamp_secs()?,
        cwd: env::current_dir()
            .map_err(|error| format!("read current directory: {error}"))?
            .display()
            .to_string(),
        command: invocation.command_for_metadata(),
        environment: run_environment(),
        config: ConfigForMetadata {
            facet_counts: config.facet_counts.clone(),
            samples: config.samples,
            seed: config.seed,
            h_min: config.h_min,
            h_max: config.h_max,
            max_attempts_per_sample: MAX_ATTEMPTS_PER_SAMPLE,
            trace: config.trace,
        },
        files: OutputFiles {
            phase_events_jsonl: phase_events_path.display().to_string(),
            run_metadata_json: metadata_path.display().to_string(),
        },
    };
    write_json_file(&metadata_path, &metadata)?;

    let mut phase_events = JsonlWriter::create(&phase_events_path)?;
    for &facet_count in &config.facet_counts {
        for sample in 0..config.samples {
            profile_sample(&config, facet_count, sample, &mut phase_events)?;
        }
    }
    phase_events.flush()?;

    Ok(out_dir)
}

fn profile_sample(
    config: &Config,
    facet_count: usize,
    sample: usize,
    phase_events: &mut JsonlWriter,
) -> Result<(), String> {
    let _span = info_span!(
        "performance_sample",
        target = TARGET_NAME,
        facet_count,
        sample
    )
    .entered();
    let (fixture_result, fixture_ms) =
        timed_result(|| phase_fixture_generation(config, facet_count, sample, phase_events));
    let (dual_vertices, fixture_attempts) = match fixture_result {
        Ok(value) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "fixture_generation")
                    .elapsed(fixture_ms)
                    .fixture_attempts(value.1),
            )?;
            value
        }
        Err(error) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "fixture_generation")
                    .elapsed(fixture_ms)
                    .error(error),
            )?;
            return Ok(());
        }
    };

    let (geometry, exact_geometry_ms) = timed(|| phase_exact_geometry(&dual_vertices));
    phase_events.write(
        &base_event(config, facet_count, sample, "exact_geometry")
            .elapsed(exact_geometry_ms)
            .vertex_count(geometry.vertex_count),
    )?;

    let (transition_is_allowed, transition_ms) = timed(|| phase_transition_matrix(&geometry));
    let allowed_transitions = transition_is_allowed
        .iter()
        .filter(|&&allowed| allowed)
        .count();
    phase_events.write(
        &base_event(config, facet_count, sample, "transition_matrix")
            .elapsed(transition_ms)
            .allowed_transitions(allowed_transitions),
    )?;

    let (solve_result, solve_ms) =
        timed_result(|| phase_solve_candidates(&dual_vertices, &transition_is_allowed));
    let (orbits, iterations) = match solve_result {
        Ok(value) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "solve_candidates")
                    .elapsed(solve_ms)
                    .iterations(value.1)
                    .raw_orbits(value.0.len()),
            )?;
            value
        }
        Err(error) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "solve_candidates")
                    .elapsed(solve_ms)
                    .error(format!("{error:?}")),
            )?;
            return Ok(());
        }
    };

    let (aggregate_result, aggregate_ms) =
        timed_result(|| phase_aggregate_minima(&geometry, orbits, iterations));
    match aggregate_result {
        Ok(result) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "aggregate_minima")
                    .elapsed(aggregate_ms)
                    .iterations(iterations)
                    .returned_orbits(result.orbits.len())
                    .capacity(result.capacity())
                    .best_sigma_len(result.best_sigma().len()),
            )?;
        }
        Err(error) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "aggregate_minima")
                    .elapsed(aggregate_ms)
                    .iterations(iterations)
                    .error(format!("{error:?}")),
            )?;
        }
    }

    Ok(())
}

#[inline(never)]
fn phase_fixture_generation(
    config: &Config,
    facet_count: usize,
    sample: usize,
    phase_events: &mut JsonlWriter,
) -> Result<(Vec<Vector4<f64>>, u64), String> {
    let first_attempt = facet_count as u64 * 1_000_000 + sample as u64 * MAX_ATTEMPTS_PER_SAMPLE;
    for offset in 0..MAX_ATTEMPTS_PER_SAMPLE {
        let attempt = first_attempt + offset;
        match profile_fixture_attempt(config, facet_count, sample, attempt, phase_events) {
            Ok(dual_vertices) => return Ok((dual_vertices, offset + 1)),
            Err(_) => continue,
        }
    }

    Err(format!(
        "no accepted fixture after {MAX_ATTEMPTS_PER_SAMPLE} attempts"
    ))
}

fn profile_fixture_attempt(
    config: &Config,
    facet_count: usize,
    sample: usize,
    attempt: u64,
    phase_events: &mut JsonlWriter,
) -> Result<Vec<Vector4<f64>>, String> {
    let (result, profile) = generate_dual_vertices_profiled(
        facet_count,
        config.h_min,
        config.h_max,
        config.seed,
        attempt,
    );
    phase_events.write(
        &base_event(config, facet_count, sample, "fixture_seed_setup")
            .elapsed(profile.seed_setup_ms)
            .attempt_index(attempt),
    )?;

    phase_events.write(
        &base_event(config, facet_count, sample, "fixture_raw_sampling")
            .elapsed(profile.raw_sampling_ms)
            .attempt_index(attempt),
    )?;

    match result {
        Ok(dual_vertices) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "fixture_acceptance_validation")
                    .elapsed(profile.validation_ms)
                    .attempt_index(attempt)
                    .accepted(true),
            )?;
            Ok(dual_vertices)
        }
        Err(error) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "fixture_acceptance_validation")
                    .elapsed(profile.validation_ms)
                    .attempt_index(attempt)
                    .accepted(false)
                    .error(format!("{error:?}")),
            )?;
            Err("rejected fixture attempt".to_string())
        }
    }
}

#[inline(never)]
fn phase_exact_geometry(dual_vertices: &[Vector4<f64>]) -> FlatGeometry {
    let _span = info_span!("exact_geometry", facet_count = dual_vertices.len()).entered();
    let (dual_vertices_exact, dual_vertices_exact_vectors) = {
        let _span = info_span!("exact_rational_conversion").entered();
        let dual_vertices_exact = exact_dual_vertex_arrays(dual_vertices);
        let dual_vertices_exact_vectors = exact_dual_vertex_vectors(&dual_vertices_exact);
        (dual_vertices_exact, dual_vertices_exact_vectors)
    };
    let PolarVerticesExact {
        vertices,
        vertex_facet_incidence,
    } = {
        let _span = info_span!("exact_polar_vertices").entered();
        polar_vertices_exact_rational_assuming_origin_interior(&dual_vertices_exact_vectors)
    };
    let facet_intersection_is_nonempty = {
        let _span = info_span!("exact_facet_intersections").entered();
        facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence)
    };
    let omega_signs = {
        let _span = info_span!("exact_omega_signs").entered();
        omega_signs_exact(&dual_vertices_exact_vectors)
    };

    FlatGeometry {
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
        vertex_count: vertices.len(),
    }
}

#[inline(never)]
fn phase_transition_matrix(geometry: &FlatGeometry) -> DMatrix<bool> {
    build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    )
}

#[inline(never)]
fn phase_solve_candidates(
    dual_vertices: &[Vector4<f64>],
    transition_is_allowed: &DMatrix<bool>,
) -> Result<(Vec<OrbitKktData>, u64), symplectic::OrbitSearchError> {
    solve_pruned_hk2017_candidates(dual_vertices, transition_is_allowed)
}

#[inline(never)]
fn phase_aggregate_minima(
    geometry: &FlatGeometry,
    orbits: Vec<OrbitKktData>,
    iterations: u64,
) -> Result<symplectic::OrbitSearchResult, symplectic::OrbitSearchError> {
    aggregate_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
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

fn base_event(
    config: &Config,
    facet_count: usize,
    sample: usize,
    phase: &'static str,
) -> PhaseEvent {
    PhaseEvent {
        target: TARGET_NAME,
        facet_count,
        sample,
        seed: config.seed,
        phase,
        elapsed_ms: 0.0,
        status: "ok",
        fixture_attempts: None,
        attempt_index: None,
        accepted: None,
        vertex_count: None,
        allowed_transitions: None,
        iterations: None,
        raw_orbits: None,
        returned_orbits: None,
        capacity: None,
        best_sigma_len: None,
        error: None,
    }
}

impl PhaseEvent {
    fn elapsed(mut self, elapsed_ms: f64) -> Self {
        self.elapsed_ms = elapsed_ms;
        self
    }

    fn fixture_attempts(mut self, fixture_attempts: u64) -> Self {
        self.fixture_attempts = Some(fixture_attempts);
        self
    }

    fn attempt_index(mut self, attempt_index: u64) -> Self {
        self.attempt_index = Some(attempt_index);
        self
    }

    fn accepted(mut self, accepted: bool) -> Self {
        self.accepted = Some(accepted);
        self
    }

    fn vertex_count(mut self, vertex_count: usize) -> Self {
        self.vertex_count = Some(vertex_count);
        self
    }

    fn allowed_transitions(mut self, allowed_transitions: usize) -> Self {
        self.allowed_transitions = Some(allowed_transitions);
        self
    }

    fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = Some(iterations);
        self
    }

    fn raw_orbits(mut self, raw_orbits: usize) -> Self {
        self.raw_orbits = Some(raw_orbits);
        self
    }

    fn returned_orbits(mut self, returned_orbits: usize) -> Self {
        self.returned_orbits = Some(returned_orbits);
        self
    }

    fn capacity(mut self, capacity: f64) -> Self {
        self.capacity = Some(capacity);
        self
    }

    fn best_sigma_len(mut self, best_sigma_len: usize) -> Self {
        self.best_sigma_len = Some(best_sigma_len);
        self
    }

    fn error(mut self, error: impl Into<String>) -> Self {
        self.status = "error";
        self.error = Some(error.into());
        self
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        facet_counts: vec![10],
        samples: 3,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        trace: false,
        out_dir: None,
    };

    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--facet-counts" => {
                let value = take_value("--facet-counts", inline_value, &mut args)?;
                config.facet_counts = parse_facet_counts(&value)?;
            }
            "--samples" => {
                let value = take_value("--samples", inline_value, &mut args)?;
                config.samples = value
                    .parse()
                    .map_err(|_| format!("--samples must be a positive integer, got {value}"))?;
            }
            "--seed" => {
                let value = take_value("--seed", inline_value, &mut args)?;
                config.seed = value
                    .parse()
                    .map_err(|_| format!("--seed must be a u64, got {value}"))?;
            }
            "--h-min" => {
                let value = take_value("--h-min", inline_value, &mut args)?;
                config.h_min = value
                    .parse()
                    .map_err(|_| format!("--h-min must be a finite f64, got {value}"))?;
            }
            "--h-max" => {
                let value = take_value("--h-max", inline_value, &mut args)?;
                config.h_max = value
                    .parse()
                    .map_err(|_| format!("--h-max must be a finite f64, got {value}"))?;
            }
            "--out-dir" => {
                let value = take_value("--out-dir", inline_value, &mut args)?;
                config.out_dir = Some(PathBuf::from(value));
            }
            "--trace" => {
                if inline_value.is_some() {
                    return Err("--trace does not take a value".to_owned());
                }
                config.trace = true;
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

impl Invocation {
    fn from_env(args: ArgsOs) -> Result<Self, String> {
        let mut args = args.into_iter();
        let program = args
            .next()
            .ok_or_else(|| "missing argv[0] program name".to_owned())?;
        let args = args
            .map(|arg| {
                arg.into_string().map_err(|arg| {
                    format!("non-utf8 command argument: {}", Path::new(&arg).display())
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { program, args })
    }

    fn command_for_metadata(&self) -> Vec<String> {
        let mut command = Vec::with_capacity(self.args.len() + 1);
        command.push(Path::new(&self.program).display().to_string());
        command.extend(self.args.iter().cloned());
        command
    }
}

fn split_inline_arg(arg: String) -> (String, Option<String>) {
    match arg.split_once('=') {
        Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
        None => (arg, None),
    }
}

fn take_value(
    flag: &str,
    inline_value: Option<String>,
    args: &mut impl Iterator<Item = String>,
) -> Result<String, String> {
    match inline_value {
        Some(value) => Ok(value),
        None => args
            .next()
            .ok_or_else(|| format!("{flag} requires a value")),
    }
}

fn parse_facet_counts(value: &str) -> Result<Vec<usize>, String> {
    let facet_counts: Result<Vec<_>, _> = value
        .split(',')
        .map(|part| {
            let trimmed = part.trim();
            trimmed
                .parse::<usize>()
                .map_err(|_| format!("invalid facet count: {trimmed}"))
        })
        .collect();
    let facet_counts = facet_counts?;
    if facet_counts.is_empty() {
        return Err("--facet-counts must contain at least one count".to_owned());
    }
    Ok(facet_counts)
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("--samples must be at least 1".to_owned());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 5)
    {
        return Err("--facet-counts entries must be at least 5".to_owned());
    }
    if !config.h_min.is_finite()
        || !config.h_max.is_finite()
        || config.h_min <= 0.0
        || config.h_min >= config.h_max
    {
        return Err(format!(
            "--h-min/--h-max must satisfy finite 0 < h-min < h-max, got {} and {}",
            config.h_min, config.h_max
        ));
    }
    Ok(())
}

fn init_tracing() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .try_init()
        .map_err(|error| format!("initialize tracing subscriber: {error}"))
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-performance --release --bin hk2017-pruned-f64 -- \\
        --facet-counts 10,11,12 --samples 3 --out-dir /tmp/perf-hk2017\n\
\n\
Options:\n\
  --facet-counts LIST  Comma-separated facet counts, each at least 5 [default: 10]\n\
  --samples N          Accepted deterministic fixtures per facet count [default: 3]\n\
  --seed N             Master seed for deterministic fixture attempts [default: 42]\n\
  --h-min X            Random fixture minimum support height [default: 0.5]\n\
  --h-max X            Random fixture maximum support height [default: 2.0]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<time>-pid<PID>]\n\
  --trace              Emit tracing span close events to stderr\n\
  --help               Print this help text"
}
