//! Diagnose unresolved f64 flow-graph closed words against exact one-sigma QP.
//!
//! This is an experiment, not a correctness resolver. Exact QP on the same
//! sigma is recorded as evidence about an unresolved word; it is not treated
//! here as a proof that the flow-graph tube is empty or nonempty.

use exp_combinatorial_cells::flat_polytope::{rational_arrays_to_vectors, CellPolytopeCache};
use num_rational::BigRational;
use serde::Serialize;
use std::path::PathBuf;
use symplectic::algorithms::flow_graph::{
    diagnose_f64_closed_words,
    exact_tube::{
        resolve_closed_word_exact, ExactClosedTubeError, ExactClosedWordOutcome, ExactFlatTubeInput,
    },
    F64ClosedCycleOutcome, F64TubeError, FlatTubeInput,
};
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify_sigma_beta_action;
use symplectic::exact::solve_orbit_sigma_exact;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
};

const DEFAULT_MASTER_SEED: u64 = 20260605;
const DEFAULT_FACET_COUNT: usize = 7;
const DEFAULT_ATTEMPT_START: u64 = 0;
const DEFAULT_ATTEMPTS: u64 = 40;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

#[derive(Debug)]
struct Args {
    facet_count: usize,
    master_seed: u64,
    attempt_start: u64,
    attempts: u64,
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct Row {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    qp_capacity: Option<f64>,
    qp_best_sigma: Option<Vec<usize>>,
    fg_capacity: Option<f64>,
    checked_closed_cycle_count: usize,
    closed_cycle_error_count: usize,
    unresolved_summary: UnresolvedSummary,
    unresolved: Vec<UnresolvedWord>,
    row_error: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct UnresolvedSummary {
    singular_fixed_point_errors: usize,
    indeterminate_polygon_errors: usize,
    exact_empty_tubes: usize,
    exact_zero_action_no_orbits: usize,
    exact_non_strict_no_orbits: usize,
    exact_positive_orbits: usize,
    exact_unsupported_positive_singular: usize,
    exact_tube_errors: usize,
    exact_positive_below_qp_capacity: usize,
    exact_positive_equal_or_above_qp_capacity: usize,
}

#[derive(Debug, Serialize)]
struct UnresolvedWord {
    sigma: Vec<usize>,
    f64_step: String,
    f64_error: String,
    exact_tube: ExactTubeSummary,
    exact_qp: ExactQpSummary,
    is_qp_best_sigma: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ExactTubeSummary {
    EmptyTube,
    ZeroActionNoOrbit {
        action_decimal: Option<f64>,
        singular_status: Option<String>,
    },
    NonStrictNoOrbit {
        action_decimal: f64,
    },
    PositiveOrbit {
        action_decimal: f64,
        tube_halfspaces: Option<usize>,
        tube_vertices: Option<usize>,
    },
    UnsupportedPositiveSingular {
        singular_status: String,
        min_action_decimal: Option<f64>,
        max_action_decimal: Option<f64>,
    },
    Error {
        error: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ExactQpSummary {
    Orbit {
        action_decimal: f64,
        q_decimal: f64,
        geometric_recovery: Option<GeometricRecoverySummary>,
    },
    NoOrbit,
}

#[derive(Debug, Serialize)]
struct GeometricRecoverySummary {
    max_violation: f64,
    closure_error: f64,
    action: f64,
    solution_dim: usize,
}

fn main() -> Result<(), String> {
    let args = parse_args();
    let mut rows = Vec::new();
    for attempt in args.attempt_start..args.attempt_start + args.attempts {
        rows.push(classify_attempt(&args, attempt));
    }
    write_jsonl(&rows, args.output.as_ref())?;
    Ok(())
}

fn classify_attempt(args: &Args, attempt: u64) -> Row {
    let dual_vertices =
        match generate_dual_vertices(args.facet_count, H_MIN, H_MAX, args.master_seed, attempt) {
            Ok(dual_vertices) => dual_vertices,
            Err(error) => {
                return Row::error(args, attempt, format!("generate_dual_vertices: {error:?}"));
            }
        };
    let Some(polytope) = CellPolytopeCache::from_f64(dual_vertices) else {
        return Row::error(
            args,
            attempt,
            "CellPolytopeCache rejected polytope".to_string(),
        );
    };
    let input = FlatTubeInput::new(
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );

    let (qp_capacity, qp_best_sigma) = qp_reference(&polytope).unwrap_or((None, None));
    let exact_duals = rational_arrays_to_vectors(&polytope.dual_vertices);
    let exact_input = ExactFlatTubeInput {
        dual_vertices: &polytope.dual_vertices,
        facet_intersection_is_nonempty: &polytope.facet_intersection_is_nonempty,
        omega_signs: &polytope.omega_signs,
    };

    let flow = match diagnose_f64_closed_words(&input, 0.0) {
        Ok(flow) => flow,
        Err(error) => {
            return Row {
                facet_count: args.facet_count,
                master_seed: args.master_seed,
                attempt,
                qp_capacity,
                qp_best_sigma,
                fg_capacity: None,
                checked_closed_cycle_count: 0,
                closed_cycle_error_count: 0,
                unresolved_summary: UnresolvedSummary::default(),
                unresolved: Vec::new(),
                row_error: Some(format!("diagnose_f64_closed_words: {error:?}")),
            };
        }
    };

    let mut unresolved_summary = UnresolvedSummary::default();
    let unresolved = flow
        .closed_cycles
        .iter()
        .filter_map(|record| {
            let F64ClosedCycleOutcome::Error(error) = record.outcome else {
                return None;
            };
            if !matches!(
                error.error,
                F64TubeError::NumericallyIndeterminatePolygon | F64TubeError::SingularFixedPointMap
            ) {
                return None;
            }
            unresolved_summary.add_f64_error(error.error);
            let exact_tube = exact_tube_summary(&exact_input, &record.sigma);
            unresolved_summary.add_exact_tube(&exact_tube, qp_capacity);
            let exact_qp = match solve_orbit_sigma_exact(&exact_duals, &record.sigma) {
                Some(orbit) => {
                    let action_decimal = rational_to_f64_lossy(&orbit.action());
                    let beta_decimal: Vec<f64> =
                        orbit.beta.iter().map(rational_to_f64_lossy).collect();
                    let geometric_recovery = recover_and_verify_sigma_beta_action(
                        &polytope.dual_vertices_f64,
                        &record.sigma,
                        &beta_decimal,
                        action_decimal,
                    )
                    .map(|recovery| GeometricRecoverySummary {
                        max_violation: recovery.max_violation,
                        closure_error: recovery.closure_error,
                        action: recovery.action,
                        solution_dim: recovery.solution_dim,
                    });
                    ExactQpSummary::Orbit {
                        action_decimal,
                        q_decimal: rational_to_f64_lossy(&orbit.q),
                        geometric_recovery,
                    }
                }
                None => ExactQpSummary::NoOrbit,
            };
            Some(UnresolvedWord {
                sigma: record.sigma.clone(),
                f64_step: format!("{:?}", error.step),
                f64_error: format!("{:?}", error.error),
                exact_tube,
                exact_qp,
                is_qp_best_sigma: qp_best_sigma.as_ref().map(|sigma| *sigma == record.sigma),
            })
        })
        .collect();

    Row {
        facet_count: args.facet_count,
        master_seed: args.master_seed,
        attempt,
        qp_capacity,
        qp_best_sigma,
        fg_capacity: flow.best_action,
        checked_closed_cycle_count: flow.checked_closed_word_count(),
        closed_cycle_error_count: flow.closed_cycle_error_count(),
        unresolved_summary,
        unresolved,
        row_error: None,
    }
}

impl UnresolvedSummary {
    fn add_f64_error(&mut self, error: F64TubeError) {
        match error {
            F64TubeError::SingularFixedPointMap => self.singular_fixed_point_errors += 1,
            F64TubeError::NumericallyIndeterminatePolygon => {
                self.indeterminate_polygon_errors += 1;
            }
            _ => {}
        }
    }

    fn add_exact_tube(&mut self, exact_tube: &ExactTubeSummary, qp_capacity: Option<f64>) {
        match exact_tube {
            ExactTubeSummary::EmptyTube => self.exact_empty_tubes += 1,
            ExactTubeSummary::ZeroActionNoOrbit { .. } => self.exact_zero_action_no_orbits += 1,
            ExactTubeSummary::NonStrictNoOrbit { .. } => self.exact_non_strict_no_orbits += 1,
            ExactTubeSummary::PositiveOrbit { action_decimal, .. } => {
                self.exact_positive_orbits += 1;
                match qp_capacity {
                    Some(qp_capacity) if *action_decimal + 1e-8 < qp_capacity => {
                        self.exact_positive_below_qp_capacity += 1;
                    }
                    Some(_) => self.exact_positive_equal_or_above_qp_capacity += 1,
                    None => {}
                }
            }
            ExactTubeSummary::UnsupportedPositiveSingular { .. } => {
                self.exact_unsupported_positive_singular += 1;
            }
            ExactTubeSummary::Error { .. } => self.exact_tube_errors += 1,
        }
    }
}

fn exact_tube_summary(input: &ExactFlatTubeInput<'_>, sigma: &[usize]) -> ExactTubeSummary {
    match resolve_closed_word_exact(input, sigma) {
        Ok((result, _metrics)) => match result.outcome {
            ExactClosedWordOutcome::EmptyTube => ExactTubeSummary::EmptyTube,
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                action,
                singular_status,
                ..
            } => ExactTubeSummary::ZeroActionNoOrbit {
                action_decimal: action.as_ref().map(rational_to_f64_lossy),
                singular_status: singular_status.map(str::to_string),
            },
            ExactClosedWordOutcome::NonStrictNoOrbit { action, .. } => {
                ExactTubeSummary::NonStrictNoOrbit {
                    action_decimal: rational_to_f64_lossy(&action),
                }
            }
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                ExactTubeSummary::PositiveOrbit {
                    action_decimal: rational_to_f64_lossy(&action),
                    tube_halfspaces: result.tube_halfspaces,
                    tube_vertices: result.tube_vertices,
                }
            }
            ExactClosedWordOutcome::UnsupportedPositiveSingular {
                singular_status,
                min_action,
                max_action,
            } => ExactTubeSummary::UnsupportedPositiveSingular {
                singular_status: singular_status.to_string(),
                min_action_decimal: min_action.as_ref().map(rational_to_f64_lossy),
                max_action_decimal: max_action.as_ref().map(rational_to_f64_lossy),
            },
        },
        Err(error) => ExactTubeSummary::Error {
            error: exact_tube_error_name(error).to_string(),
        },
    }
}

fn exact_tube_error_name(error: ExactClosedTubeError) -> &'static str {
    match error {
        ExactClosedTubeError::InvalidInput => "invalid_input",
        ExactClosedTubeError::InvalidWord => "invalid_word",
        ExactClosedTubeError::UnsupportedSingularTransition => "unsupported_singular_transition",
        ExactClosedTubeError::InternalInconsistentSingularSolve => {
            "internal_inconsistent_singular_solve"
        }
    }
}

fn qp_reference(polytope: &CellPolytopeCache) -> Result<(Option<f64>, Option<Vec<usize>>), String> {
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)
            .map_err(|error| format!("{error:?}"))?;
    let result = aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(|error| format!("{error:?}"))?;
    Ok((Some(result.min_action), Some(result.best_sigma().to_vec())))
}

fn rational_to_f64_lossy(value: &BigRational) -> f64 {
    use num_traits::ToPrimitive;
    value.to_f64().unwrap_or(f64::NAN)
}

impl Row {
    fn error(args: &Args, attempt: u64, row_error: String) -> Self {
        Self {
            facet_count: args.facet_count,
            master_seed: args.master_seed,
            attempt,
            qp_capacity: None,
            qp_best_sigma: None,
            fg_capacity: None,
            checked_closed_cycle_count: 0,
            closed_cycle_error_count: 0,
            unresolved_summary: UnresolvedSummary::default(),
            unresolved: Vec::new(),
            row_error: Some(row_error),
        }
    }
}

fn write_jsonl(rows: &[Row], output: Option<&PathBuf>) -> Result<(), String> {
    let mut writer: Box<dyn std::io::Write> = match output {
        Some(path) => Box::new(
            std::fs::File::create(path)
                .map_err(|error| format!("create {}: {error}", path.display()))?,
        ),
        None => Box::new(std::io::stdout()),
    };
    for row in rows {
        serde_json::to_writer(&mut writer, row).map_err(|error| format!("serialize: {error}"))?;
        writeln!(writer).map_err(|error| format!("write newline: {error}"))?;
    }
    Ok(())
}

fn parse_args() -> Args {
    let mut args = Args {
        facet_count: DEFAULT_FACET_COUNT,
        master_seed: DEFAULT_MASTER_SEED,
        attempt_start: DEFAULT_ATTEMPT_START,
        attempts: DEFAULT_ATTEMPTS,
        output: None,
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--facet-count" => {
                args.facet_count = iter
                    .next()
                    .expect("--facet-count needs a value")
                    .parse()
                    .expect("--facet-count must be usize");
            }
            "--master-seed" => {
                args.master_seed = iter
                    .next()
                    .expect("--master-seed needs a value")
                    .parse()
                    .expect("--master-seed must be u64");
            }
            "--attempt-start" => {
                args.attempt_start = iter
                    .next()
                    .expect("--attempt-start needs a value")
                    .parse()
                    .expect("--attempt-start must be u64");
            }
            "--attempts" => {
                args.attempts = iter
                    .next()
                    .expect("--attempts needs a value")
                    .parse()
                    .expect("--attempts must be u64");
            }
            "--output" => {
                args.output = Some(PathBuf::from(iter.next().expect("--output needs a path")));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-unresolved-diagnostic [--facet-count N] [--master-seed N] [--attempt-start N] [--attempts N] [--output PATH]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    args
}
