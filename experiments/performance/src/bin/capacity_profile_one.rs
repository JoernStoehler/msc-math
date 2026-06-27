use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, F64CapacityMethod, F64CapacityOutcome,
    F64ValidationPolicy,
};
use exp_performance::args::{split_inline_arg, take_value};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use std::env;
use std::hint::black_box;
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

const TARGET_NAME: &str = "capacity-profile-one";
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;
const MAX_ATTEMPTS_PER_SAMPLE: u64 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CapacityPath {
    F64TransitionPrunedHk,
    PrunedHkExactFallback,
}

impl CapacityPath {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "f64_transition_pruned_hk" | "f64" => Ok(Self::F64TransitionPrunedHk),
            "pruned_hk_exact_fallback" | "fallback" => Ok(Self::PrunedHkExactFallback),
            other => Err(format!(
                "--path must be f64_transition_pruned_hk/f64 or pruned_hk_exact_fallback/fallback, got {other}"
            )),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::F64TransitionPrunedHk => "f64_transition_pruned_hk",
            Self::PrunedHkExactFallback => "pruned_hk_exact_fallback",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Config {
    path: CapacityPath,
    facet_count: usize,
    sample: usize,
    repetitions: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    out_dir: Option<PathBuf>,
}

struct Geometry {
    dual_vertices_f64: Vec<Vector4<f64>>,
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: DMatrix<bool>,
    omega_signs: DMatrix<i8>,
}

#[derive(Serialize)]
struct SummaryRow {
    target: &'static str,
    path: &'static str,
    facet_count: usize,
    sample: usize,
    repetitions: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    elapsed_ms: f64,
    per_repetition_ms: f64,
    last_capacity: f64,
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
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.path.label())?;
    let dual_vertices = accepted_fixture(&config)?;
    let geometry = exact_geometry(dual_vertices);

    let started = Instant::now();
    let mut last = None;
    for _ in 0..config.repetitions {
        last = Some(match config.path {
            CapacityPath::F64TransitionPrunedHk => f64_hk(&geometry.dual_vertices_f64),
            CapacityPath::PrunedHkExactFallback => pruned_hk_exact(&geometry),
        });
        black_box(last);
    }
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;

    let mut writer = JsonlWriter::create(&out_dir.join("profile-summary.jsonl"))?;
    writer.write(&SummaryRow {
        target: TARGET_NAME,
        path: config.path.label(),
        facet_count: config.facet_count,
        sample: config.sample,
        repetitions: config.repetitions,
        seed: config.seed,
        h_min: config.h_min,
        h_max: config.h_max,
        elapsed_ms,
        per_repetition_ms: elapsed_ms / config.repetitions as f64,
        last_capacity: last.expect("repetitions is positive"),
    })?;
    writer.flush()?;
    Ok(out_dir)
}

fn accepted_fixture(config: &Config) -> Result<Vec<Vector4<f64>>, String> {
    let first_attempt =
        config.facet_count as u64 * 1_000_000 + config.sample as u64 * MAX_ATTEMPTS_PER_SAMPLE;
    for offset in 0..MAX_ATTEMPTS_PER_SAMPLE {
        if let Ok(dual_vertices) = generate_dual_vertices(
            config.facet_count,
            config.h_min,
            config.h_max,
            config.seed,
            first_attempt + offset,
        ) {
            return Ok(dual_vertices);
        }
    }
    Err(format!(
        "no accepted fixture for F={}, sample={}",
        config.facet_count, config.sample
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

fn pruned_hk_exact(geometry: &Geometry) -> f64 {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&geometry.dual_vertices_f64, &transition_is_allowed)
            .expect("pruned candidate solve");
    aggregate(geometry, orbits, iterations)
}

fn aggregate(geometry: &Geometry, orbits: Vec<OrbitKktData>, iterations: u64) -> f64 {
    aggregate_orbits_with_dual_vertices_exact(
        &geometry.dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("aggregate")
    .min_action
}

fn f64_hk(dual_vertices: &[Vector4<f64>]) -> f64 {
    let (report, _) = capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    match report.outcome {
        F64CapacityOutcome::Success { capacity, .. } => capacity,
        F64CapacityOutcome::Failure { reason } => panic!("f64 route failed: {reason:?}"),
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

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        path: CapacityPath::PrunedHkExactFallback,
        facet_count: 10,
        sample: 1,
        repetitions: 100,
        seed: SEED,
        h_min: H_MIN,
        h_max: H_MAX,
        out_dir: None,
    };
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--path" => {
                config.path = CapacityPath::parse(&take_value("--path", inline_value, &mut args)?)?
            }
            "--facet-count" => {
                config.facet_count = take_value("--facet-count", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--facet-count must be a positive integer".to_string())?
            }
            "--sample" => {
                config.sample = take_value("--sample", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--sample must be a nonnegative integer".to_string())?
            }
            "--repetitions" => {
                config.repetitions = take_value("--repetitions", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--repetitions must be a positive integer".to_string())?
            }
            "--out-dir" => {
                config.out_dir = Some(PathBuf::from(take_value(
                    "--out-dir",
                    inline_value,
                    &mut args,
                )?))
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

fn validate_config(config: &Config) -> Result<(), String> {
    if config.facet_count < 5 {
        return Err("--facet-count must be at least 5".to_string());
    }
    if config.repetitions == 0 {
        return Err("--repetitions must be positive".to_string());
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
    "Usage: cargo run -p exp-performance --release --bin capacity-profile-one -- \\
        --path f64 --facet-count 10 --sample 1 --repetitions 100 --out-dir /tmp/capacity-profile-one-f64\n\
\n\
Options:\n\
  --path PATH           f64_transition_pruned_hk/f64 or pruned_hk_exact_fallback/fallback [default: fallback]\n\
  --facet-count N      Random fixture facet count [default: 10]\n\
  --sample N           Deterministic random fixture sample index [default: 1]\n\
  --repetitions N      Repetitions after fixture construction [default: 100]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<path>-<time>-pid<PID>]\n\
  --help               Print this help text"
}
