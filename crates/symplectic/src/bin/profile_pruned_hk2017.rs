use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence, polar_vertices_exact,
    PolarVerticesExact,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use std::env;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
};

const DEFAULT_SEED: u64 = 42;
const DEFAULT_H_MIN: f64 = 0.5;
const DEFAULT_H_MAX: f64 = 2.0;
const MAX_ATTEMPTS_PER_SAMPLE: u64 = 10_000;

#[derive(Clone, Debug)]
struct Config {
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    jsonl: bool,
    trace: bool,
}

#[derive(Serialize)]
struct ProfileRow {
    facet_count: usize,
    sample: usize,
    seed: u64,
    fixture_attempts: u64,
    status: &'static str,
    phases_ms: PhaseTimingsMs,
    counts: ProfileCounts,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_action_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_action_upper: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_sigma_len: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Default, Serialize)]
struct PhaseTimingsMs {
    fixture_generation: f64,
    exact_geometry: f64,
    transition_matrix: f64,
    solve_candidates: f64,
    aggregate: f64,
    total: f64,
}

#[derive(Default, Serialize)]
struct ProfileCounts {
    vertices: Option<usize>,
    allowed_transitions: Option<usize>,
    raw_orbits: Option<usize>,
    returned_orbits: Option<usize>,
    iterations: Option<u64>,
}

struct FlatGeometry {
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: DMatrix<bool>,
    omega_signs: DMatrix<i8>,
    vertex_count: usize,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let config = parse_args(env::args().skip(1))?;
    validate_config(&config)?;
    if config.trace {
        init_tracing()?;
    }

    for &facet_count in &config.facet_counts {
        for sample in 0..config.samples {
            let row = profile_sample(&config, facet_count, sample);
            emit_row(&row, config.jsonl)?;
        }
    }

    Ok(())
}

fn profile_sample(config: &Config, facet_count: usize, sample: usize) -> ProfileRow {
    let total_start = Instant::now();
    let mut phases = PhaseTimingsMs::default();
    let mut counts = ProfileCounts::default();

    let fixture_start = Instant::now();
    let (dual_vertices, fixture_attempts) = match accepted_fixture(config, facet_count, sample) {
        Ok(value) => value,
        Err(error) => {
            phases.fixture_generation = ms(fixture_start.elapsed());
            phases.total = ms(total_start.elapsed());
            return error_row(
                facet_count,
                sample,
                config.seed,
                MAX_ATTEMPTS_PER_SAMPLE,
                phases,
                counts,
                error,
            );
        }
    };
    phases.fixture_generation = ms(fixture_start.elapsed());

    let geometry_start = Instant::now();
    let geometry = build_flat_geometry(&dual_vertices);
    phases.exact_geometry = ms(geometry_start.elapsed());
    counts.vertices = Some(geometry.vertex_count);

    let transition_start = Instant::now();
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    phases.transition_matrix = ms(transition_start.elapsed());
    counts.allowed_transitions = Some(
        transition_is_allowed
            .iter()
            .filter(|&&allowed| allowed)
            .count(),
    );

    let solve_start = Instant::now();
    let (orbits, iterations) =
        match solve_pruned_hk2017_candidates(&dual_vertices, &transition_is_allowed) {
            Ok(value) => value,
            Err(error) => {
                phases.solve_candidates = ms(solve_start.elapsed());
                phases.total = ms(total_start.elapsed());
                return error_row(
                    facet_count,
                    sample,
                    config.seed,
                    fixture_attempts,
                    phases,
                    counts,
                    format!("solve_candidates: {error:?}"),
                );
            }
        };
    phases.solve_candidates = ms(solve_start.elapsed());
    counts.iterations = Some(iterations);
    counts.raw_orbits = Some(orbits.len());

    let aggregate_start = Instant::now();
    let result = match aggregate_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    ) {
        Ok(value) => value,
        Err(error) => {
            phases.aggregate = ms(aggregate_start.elapsed());
            phases.total = ms(total_start.elapsed());
            return error_row(
                facet_count,
                sample,
                config.seed,
                fixture_attempts,
                phases,
                counts,
                format!("aggregate: {error:?}"),
            );
        }
    };
    phases.aggregate = ms(aggregate_start.elapsed());
    phases.total = ms(total_start.elapsed());
    counts.returned_orbits = Some(result.orbits.len());

    ProfileRow {
        facet_count,
        sample,
        seed: config.seed,
        fixture_attempts,
        status: "ok",
        phases_ms: phases,
        counts,
        capacity: Some(result.capacity()),
        min_action_lower: Some(result.min_action_lower),
        min_action_upper: Some(result.min_action_upper),
        best_sigma_len: Some(result.best_sigma().len()),
        error: None,
    }
}

fn accepted_fixture(
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
        "fixture_generation: no accepted fixture after {MAX_ATTEMPTS_PER_SAMPLE} attempts"
    ))
}

fn build_flat_geometry(dual_vertices: &[Vector4<f64>]) -> FlatGeometry {
    let dual_vertices_exact = exact_dual_vertex_arrays(dual_vertices);
    let dual_vertices_exact_vectors = exact_dual_vertex_vectors(&dual_vertices_exact);
    let PolarVerticesExact {
        vertices,
        vertex_facet_incidence,
    } = polar_vertices_exact(&dual_vertices_exact_vectors);
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&dual_vertices_exact_vectors);

    FlatGeometry {
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
        vertex_count: vertices.len(),
    }
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

fn error_row(
    facet_count: usize,
    sample: usize,
    seed: u64,
    fixture_attempts: u64,
    phases: PhaseTimingsMs,
    counts: ProfileCounts,
    error: String,
) -> ProfileRow {
    ProfileRow {
        facet_count,
        sample,
        seed,
        fixture_attempts,
        status: "error",
        phases_ms: phases,
        counts,
        capacity: None,
        min_action_lower: None,
        min_action_upper: None,
        best_sigma_len: None,
        error: Some(error),
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        facet_counts: vec![5, 6, 7, 8],
        samples: 3,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        jsonl: false,
        trace: false,
    };

    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = match arg.split_once('=') {
            Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
            None => (arg, None),
        };

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
            "--jsonl" => {
                if inline_value.is_some() {
                    return Err("--jsonl does not take a value".to_owned());
                }
                config.jsonl = true;
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

fn emit_row(row: &ProfileRow, jsonl: bool) -> Result<(), String> {
    if jsonl {
        println!(
            "{}",
            serde_json::to_string(row).map_err(|error| format!("serialize row: {error}"))?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(row).map_err(|error| format!("serialize row: {error}"))?
        );
    }
    Ok(())
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn init_tracing() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|error| format!("initialize tracing subscriber: {error}"))
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p symplectic --release --bin profile-pruned-hk2017 -- \\
        --facet-counts 5,6,7,8 --samples 3 --jsonl\n\
\n\
Options:\n\
  --facet-counts LIST  Comma-separated facet counts, each at least 5 [default: 5,6,7,8]\n\
  --samples N          Accepted deterministic fixtures per facet count [default: 3]\n\
  --seed N             Master seed for deterministic fixture attempts [default: 42]\n\
  --h-min X            Random fixture minimum support height [default: 0.5]\n\
  --h-max X            Random fixture maximum support height [default: 2.0]\n\
  --jsonl              Emit one compact JSON object per row\n\
  --trace              Emit opt-in tracing diagnostics to stderr"
}
