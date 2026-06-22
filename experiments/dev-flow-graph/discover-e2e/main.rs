//! Discover high-VoI flow-graph e2e smoke cases on fresh random polytopes.
//!
//! This is a case-finding tool, not a test. Promote selected rows into fixed
//! tests only after reviewing the prediction label.

use exp_combinatorial_cells::flat_polytope::{rational_arrays_to_vectors, CellPolytopeCache};
use num_rational::BigRational;
use serde::Serialize;
use std::collections::BTreeMap;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{
    closed_tube_for_sigma_f64, diagnose_f64_closed_words, F64ClosedCycleOutcome, F64TubeError,
    FlatTubeInput,
};
use symplectic::exact::solve_orbit_sigma_exact;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_orbit_sigma_saddle_point,
    solve_pruned_hk2017_candidates, OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_MASTER_SEED: u64 = 20260605;
const DEFAULT_MAX_ATTEMPTS_PER_F: u64 = 200;
const DEFAULT_WANTED_PER_BUCKET: usize = 3;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

#[derive(Debug)]
struct Args {
    facet_counts: Vec<usize>,
    attempt_start: u64,
    max_attempts_per_f: u64,
    wanted_per_bucket: usize,
    master_seed: u64,
    trace: bool,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Bucket {
    ApproxEqualClean,
    ApproxEqualWithClosedCycleErrors,
    CapacityMismatch,
    NoCandidate,
    RejectZeroOmega,
    RejectNearZeroOmega,
    RejectOther,
    QpError,
}

#[derive(Debug, Serialize)]
struct Row {
    bucket: Bucket,
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_exact: Vec<[BigRational; 4]>,
    facet_intersection_is_nonempty: Vec<Vec<bool>>,
    omega_signs: Vec<Vec<i8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qp_capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qp_best_sigma: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_facets: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_segment_times: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_breakpoints: Option<Vec<[f64; 4]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_active_facets: Option<Vec<Vec<usize>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_max_facet_violation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_qp_single_sigma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_exact_qp_single_sigma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_closed_tube_infinite_cutoff_debug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_best_closed_tube_best_cutoff_debug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_orbit_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_cycle_error_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checked_closed_cycle_count: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    closed_cycle_errors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qp_error: Option<String>,
}

fn main() {
    let args = parse_args();
    if args.trace {
        init_tracing().expect("initialize tracing");
    }
    let mut found_by_bucket: BTreeMap<Bucket, usize> = BTreeMap::new();

    for &facet_count in &args.facet_counts {
        for attempt in args.attempt_start..args.max_attempts_per_f {
            let Some(row) = classify_attempt(&args, facet_count, attempt) else {
                continue;
            };
            let found = found_by_bucket.entry(row.bucket).or_default();
            if *found >= args.wanted_per_bucket {
                continue;
            }
            serde_json::to_writer(std::io::stdout(), &row).expect("write row");
            println!();
            *found += 1;
        }
    }
}

fn classify_attempt(args: &Args, facet_count: usize, attempt: u64) -> Option<Row> {
    let dual_vertices =
        generate_dual_vertices(facet_count, H_MIN, H_MAX, args.master_seed, attempt).ok()?;
    let polytope = CellPolytopeCache::from_f64(dual_vertices)?;
    let input = FlatTubeInput::new(
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let qp = match pruned_capacity(&polytope) {
        Ok(result) => result,
        Err(error) => {
            return Some(Row {
                bucket: Bucket::QpError,
                facet_count,
                master_seed: args.master_seed,
                attempt,
                dual_vertices_f64: vector4_rows(&polytope.dual_vertices_f64),
                dual_vertices_exact: polytope.dual_vertices.clone(),
                facet_intersection_is_nonempty: matrix_rows(
                    &polytope.facet_intersection_is_nonempty,
                ),
                omega_signs: matrix_rows(&polytope.omega_signs),
                qp_capacity: None,
                qp_best_sigma: None,
                fg_capacity: None,
                fg_best_facets: None,
                fg_best_segment_times: None,
                fg_best_breakpoints: None,
                fg_best_active_facets: None,
                fg_best_max_facet_violation: None,
                fg_best_qp_single_sigma: None,
                fg_best_exact_qp_single_sigma: None,
                fg_best_closed_tube_infinite_cutoff_debug: None,
                fg_best_closed_tube_best_cutoff_debug: None,
                relative_error: None,
                fg_orbit_count: None,
                closed_cycle_error_count: None,
                checked_closed_cycle_count: None,
                closed_cycle_errors: Vec::new(),
                fg_error: None,
                qp_error: Some(format!("{error:?}")),
            });
        }
    };
    let qp_best_sigma = qp.best_sigma().to_vec();
    let qp_capacity = qp.min_action;

    match diagnose_f64_closed_words(&input, 0.0) {
        Ok(flow) => {
            let Some(fg_capacity) = flow.best_action else {
                return Some(Row {
                    bucket: Bucket::NoCandidate,
                    facet_count,
                    master_seed: args.master_seed,
                    attempt,
                    dual_vertices_f64: vector4_rows(&polytope.dual_vertices_f64),
                    dual_vertices_exact: polytope.dual_vertices.clone(),
                    facet_intersection_is_nonempty: matrix_rows(
                        &polytope.facet_intersection_is_nonempty,
                    ),
                    omega_signs: matrix_rows(&polytope.omega_signs),
                    qp_capacity: Some(qp_capacity),
                    qp_best_sigma: Some(qp_best_sigma),
                    fg_capacity: None,
                    fg_best_facets: None,
                    fg_best_segment_times: None,
                    fg_best_breakpoints: None,
                    fg_best_active_facets: None,
                    fg_best_max_facet_violation: None,
                    fg_best_qp_single_sigma: None,
                    fg_best_exact_qp_single_sigma: None,
                    fg_best_closed_tube_infinite_cutoff_debug: None,
                    fg_best_closed_tube_best_cutoff_debug: None,
                    relative_error: None,
                    fg_orbit_count: Some(flow.orbits.len()),
                    closed_cycle_error_count: Some(flow.closed_cycle_error_count()),
                    checked_closed_cycle_count: Some(flow.checked_closed_word_count()),
                    closed_cycle_errors: closed_cycle_error_summaries(&flow.closed_cycles),
                    fg_error: None,
                    qp_error: None,
                });
            };
            let fg_best = flow
                .orbits
                .iter()
                .min_by(|left, right| left.action.total_cmp(&right.action));
            let fg_best_breakpoints = fg_best.map(|orbit| vector4_rows(&orbit.breakpoints));
            let fg_best_active_facets =
                fg_best.map(|orbit| active_facets_at_breakpoints(&polytope, &orbit.breakpoints));
            let fg_best_max_facet_violation =
                fg_best.map(|orbit| max_facet_violation(&polytope, &orbit.breakpoints));
            let fg_best_qp_single_sigma = fg_best
                .map(|orbit| qp_single_sigma_summary(&polytope.dual_vertices_f64, &orbit.facets));
            let fg_best_exact_qp_single_sigma = fg_best
                .map(|orbit| exact_qp_single_sigma_summary(&polytope.dual_vertices, &orbit.facets));
            let fg_best_closed_tube_infinite_cutoff_debug =
                fg_best.map(|orbit| closed_tube_debug(&input, &orbit.facets, f64::INFINITY));
            let fg_best_closed_tube_best_cutoff_debug =
                fg_best.map(|orbit| closed_tube_debug(&input, &orbit.facets, fg_capacity));
            let relative_error = ((fg_capacity - qp_capacity) / qp_capacity).abs();
            let bucket = if relative_error <= 1e-8 && !flow.has_closed_cycle_errors() {
                Bucket::ApproxEqualClean
            } else if relative_error <= 1e-8 {
                Bucket::ApproxEqualWithClosedCycleErrors
            } else {
                Bucket::CapacityMismatch
            };
            Some(Row {
                bucket,
                facet_count,
                master_seed: args.master_seed,
                attempt,
                dual_vertices_f64: vector4_rows(&polytope.dual_vertices_f64),
                dual_vertices_exact: polytope.dual_vertices.clone(),
                facet_intersection_is_nonempty: matrix_rows(
                    &polytope.facet_intersection_is_nonempty,
                ),
                omega_signs: matrix_rows(&polytope.omega_signs),
                qp_capacity: Some(qp_capacity),
                qp_best_sigma: Some(qp_best_sigma),
                fg_capacity: Some(fg_capacity),
                fg_best_facets: fg_best.map(|orbit| orbit.facets.clone()),
                fg_best_segment_times: fg_best.map(|orbit| orbit.segment_times.clone()),
                fg_best_breakpoints,
                fg_best_active_facets,
                fg_best_max_facet_violation,
                fg_best_qp_single_sigma,
                fg_best_exact_qp_single_sigma,
                fg_best_closed_tube_infinite_cutoff_debug,
                fg_best_closed_tube_best_cutoff_debug,
                relative_error: Some(relative_error),
                fg_orbit_count: Some(flow.orbits.len()),
                closed_cycle_error_count: Some(flow.closed_cycle_error_count()),
                checked_closed_cycle_count: Some(flow.checked_closed_word_count()),
                closed_cycle_errors: closed_cycle_error_summaries(&flow.closed_cycles),
                fg_error: None,
                qp_error: None,
            })
        }
        Err(error) => Some(Row {
            bucket: rejection_bucket(error),
            facet_count,
            master_seed: args.master_seed,
            attempt,
            dual_vertices_f64: vector4_rows(&polytope.dual_vertices_f64),
            dual_vertices_exact: polytope.dual_vertices.clone(),
            facet_intersection_is_nonempty: matrix_rows(&polytope.facet_intersection_is_nonempty),
            omega_signs: matrix_rows(&polytope.omega_signs),
            qp_capacity: Some(qp_capacity),
            qp_best_sigma: Some(qp_best_sigma),
            fg_capacity: None,
            fg_best_facets: None,
            fg_best_segment_times: None,
            fg_best_breakpoints: None,
            fg_best_active_facets: None,
            fg_best_max_facet_violation: None,
            fg_best_qp_single_sigma: None,
            fg_best_exact_qp_single_sigma: None,
            fg_best_closed_tube_infinite_cutoff_debug: None,
            fg_best_closed_tube_best_cutoff_debug: None,
            relative_error: None,
            fg_orbit_count: None,
            closed_cycle_error_count: None,
            checked_closed_cycle_count: None,
            closed_cycle_errors: Vec::new(),
            fg_error: Some(format!("{error:?}")),
            qp_error: None,
        }),
    }
}

fn matrix_rows<T: Copy>(matrix: &nalgebra::DMatrix<T>) -> Vec<Vec<T>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn pruned_capacity(polytope: &CellPolytopeCache) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn vector4_rows(points: &[nalgebra::Vector4<f64>]) -> Vec<[f64; 4]> {
    points
        .iter()
        .map(|point| [point[0], point[1], point[2], point[3]])
        .collect()
}

fn qp_single_sigma_summary(dual_vertices: &[nalgebra::Vector4<f64>], sigma: &[usize]) -> String {
    match solve_orbit_sigma_saddle_point(dual_vertices, sigma) {
        Ok(orbit) => format!(
            "ok action={} beta={:?} beta_margin={} q={} q_error_bound={} admissibility={:?} mu={:?} xi={:?}",
            orbit.action,
            orbit.beta,
            orbit.beta_margin,
            orbit.q,
            orbit.q_error_bound,
            orbit.admissibility,
            orbit.mu,
            orbit.xi
        ),
        Err(error) => format!("err {error:?}"),
    }
}

fn exact_qp_single_sigma_summary(dual_vertices: &[[BigRational; 4]], sigma: &[usize]) -> String {
    let dual_vertex_vectors = rational_arrays_to_vectors(dual_vertices);
    match solve_orbit_sigma_exact(&dual_vertex_vectors, sigma) {
        Some(orbit) => format!(
            "ok action={:?} beta={:?} q={:?} mu={:?} xi={:?}",
            orbit.action(),
            orbit.beta,
            orbit.q,
            orbit.mu,
            orbit.xi
        ),
        None => "none".to_string(),
    }
}

fn closed_tube_debug(input: &FlatTubeInput<'_>, sigma: &[usize], cutoff: f64) -> String {
    match closed_tube_for_sigma_f64(input, sigma, cutoff) {
        Ok(Some(tube)) => format!("{tube:#?}"),
        Ok(None) => "Ok(None)".to_string(),
        Err(error) => format!("Err({error:?})"),
    }
}

fn active_facets_at_breakpoints(
    polytope: &CellPolytopeCache,
    breakpoints: &[nalgebra::Vector4<f64>],
) -> Vec<Vec<usize>> {
    breakpoints
        .iter()
        .map(|point| {
            polytope
                .dual_vertices_f64
                .iter()
                .enumerate()
                .filter_map(|(facet, dual)| {
                    (dual.dot(point) - 1.0).abs().lt(&1e-6).then_some(facet)
                })
                .collect()
        })
        .collect()
}

fn max_facet_violation(
    polytope: &CellPolytopeCache,
    breakpoints: &[nalgebra::Vector4<f64>],
) -> f64 {
    breakpoints
        .iter()
        .flat_map(|point| {
            polytope
                .dual_vertices_f64
                .iter()
                .map(move |dual| dual.dot(point) - 1.0)
        })
        .fold(0.0, f64::max)
}

fn closed_cycle_error_summaries(
    records: &[symplectic::algorithms::flow_graph::F64ClosedCycleRecord],
) -> Vec<String> {
    records
        .iter()
        .filter_map(|record| match record.outcome {
            F64ClosedCycleOutcome::Error(error) => Some(format!(
                "{:?}:{:?}:{:?}",
                record.sigma, error.step, error.error
            )),
            _ => None,
        })
        .collect()
}

fn rejection_bucket(error: F64TubeError) -> Bucket {
    match error {
        F64TubeError::UnsupportedZeroOmegaTransition => Bucket::RejectZeroOmega,
        F64TubeError::NumericallyUnstableOmegaTransition => Bucket::RejectNearZeroOmega,
        _ => Bucket::RejectOther,
    }
}

fn parse_args() -> Args {
    let mut args = Args {
        facet_counts: vec![5, 6, 7, 8],
        attempt_start: 0,
        max_attempts_per_f: DEFAULT_MAX_ATTEMPTS_PER_F,
        wanted_per_bucket: DEFAULT_WANTED_PER_BUCKET,
        master_seed: DEFAULT_MASTER_SEED,
        trace: false,
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--facet-counts" => {
                args.facet_counts = iter
                    .next()
                    .expect("--facet-counts needs a value")
                    .split(',')
                    .map(|part| part.parse().expect("facet count must be usize"))
                    .collect();
            }
            "--max-attempts-per-f" => {
                args.max_attempts_per_f = iter
                    .next()
                    .expect("--max-attempts-per-f needs a value")
                    .parse()
                    .expect("--max-attempts-per-f must be u64");
            }
            "--attempt-start" => {
                args.attempt_start = iter
                    .next()
                    .expect("--attempt-start needs a value")
                    .parse()
                    .expect("--attempt-start must be u64");
            }
            "--wanted-per-bucket" => {
                args.wanted_per_bucket = iter
                    .next()
                    .expect("--wanted-per-bucket needs a value")
                    .parse()
                    .expect("--wanted-per-bucket must be usize");
            }
            "--master-seed" => {
                args.master_seed = iter
                    .next()
                    .expect("--master-seed needs a value")
                    .parse()
                    .expect("--master-seed must be u64");
            }
            "--trace" => args.trace = true,
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-discover-e2e [--facet-counts 5,6,7,8] [--max-attempts-per-f N] [--wanted-per-bucket N] [--master-seed N] [--trace]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    args
}

fn init_tracing() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|error| format!("initialize tracing subscriber: {error}"))
}
