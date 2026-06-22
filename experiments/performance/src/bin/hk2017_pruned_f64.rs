use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use exp_performance::timing::{timed, timed_result};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices;
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
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    trace: bool,
    out_dir: Option<PathBuf>,
}

#[derive(Serialize)]
struct PhaseEvent {
    target: &'static str,
    mode: RunMode,
    facet_count: usize,
    sample: usize,
    seed: u64,
    phase: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixture_attempts: Option<u64>,
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
    let config = parse_args(env::args().skip(1))?;
    validate_config(&config)?;
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.mode.as_str())?;
    if config.trace {
        init_tracing()?;
    }
    let phase_events_path = out_dir.join("phase-events.jsonl");

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
        timed_result(|| phase_accepted_fixture_acquisition(config, facet_count, sample));
    let (dual_vertices, fixture_attempts) = match fixture_result {
        Ok(value) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "accepted_fixture_acquisition")
                    .elapsed(fixture_ms)
                    .fixture_attempts(value.1),
            )?;
            value
        }
        Err(error) => {
            phase_events.write(
                &base_event(config, facet_count, sample, "accepted_fixture_acquisition")
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
                    .capacity(result.min_action)
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
fn phase_accepted_fixture_acquisition(
    config: &Config,
    facet_count: usize,
    sample: usize,
) -> Result<(Vec<Vector4<f64>>, u64), String> {
    let first_attempt = facet_count as u64 * 1_000_000 + sample as u64 * MAX_ATTEMPTS_PER_SAMPLE;
    for offset in 0..MAX_ATTEMPTS_PER_SAMPLE {
        match generate_dual_vertices(
            facet_count,
            config.h_min,
            config.h_max,
            config.seed,
            first_attempt + offset,
        ) {
            Ok(dual_vertices) => return Ok((dual_vertices, offset + 1)),
            Err(_) => continue,
        }
    }

    Err(format!(
        "no accepted fixture after {MAX_ATTEMPTS_PER_SAMPLE} attempts"
    ))
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
        mode: config.mode,
        facet_count,
        sample,
        seed: config.seed,
        phase,
        elapsed_ms: 0.0,
        status: "ok",
        fixture_attempts: None,
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

fn config_for_mode(mode: RunMode) -> Config {
    let (facet_counts, samples) = match mode {
        RunMode::Smoke => (vec![5], 1),
        RunMode::Production => (vec![10, 11, 12], 3),
    };
    Config {
        mode,
        facet_counts,
        samples,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        trace: false,
        out_dir: None,
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("mode sample count must be at least 1".to_owned());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 5)
    {
        return Err("mode facet-count entries must be at least 5".to_owned());
    }
    if !config.h_min.is_finite()
        || !config.h_max.is_finite()
        || config.h_min <= 0.0
        || config.h_min >= config.h_max
    {
        return Err(format!(
            "mode h_min/h_max must satisfy finite 0 < h_min < h_max, got {} and {}",
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
        --mode production --out-dir /tmp/perf-hk2017\n\
\n\
Options:\n\
  --mode MODE          Named run mode: smoke or production [default: smoke]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<mode>-<time>-pid<PID>]\n\
  --trace              Emit tracing span close events to stderr\n\
  --help               Print this help text"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(values: &[&str]) -> Config {
        parse_args(values.iter().map(|value| value.to_string())).unwrap()
    }

    #[test]
    fn smoke_mode_is_default() {
        let config = parse(&[]);
        assert_eq!(config.mode, RunMode::Smoke);
        assert_eq!(config.facet_counts, vec![5]);
        assert_eq!(config.samples, 1);
    }

    #[test]
    fn production_mode_selects_documented_profile_size() {
        let config = parse(&["--mode", "production"]);
        assert_eq!(config.mode, RunMode::Production);
        assert_eq!(config.facet_counts, vec![10, 11, 12]);
        assert_eq!(config.samples, 3);
    }

    #[test]
    fn ad_hoc_input_selector_flags_are_rejected() {
        for flag in [
            "--seed",
            "--facet-counts",
            "--samples",
            "--h-min",
            "--h-max",
        ] {
            assert!(parse_args([flag.to_string(), "1".to_string()].into_iter()).is_err());
        }
    }
}
