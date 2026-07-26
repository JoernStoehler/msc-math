//! One-start vertical smoke for the observed multi-direction ascent candidate.
//!
//! This is deliberately a plumbing smoke, not a retained panel or evidence for
//! endpoint local maximality. It follows the current `dev-gradient-ascent`
//! near-active candidate and records the exact geometry/provenance needed to
//! make the wiring inspectable. Schema v2 uses the production exact general
//! action window for basepoint branches and the production scalar route for
//! finite-step targets; the retained v1 panel remains historical evidence for
//! its original legacy branch selector.

use exp_sys_landscape::{
    dual_vertices_rational_strings, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::capacity_4d::{
    capacity, capacity_value, check_dual_vertex_norm_bounds, check_facet_count,
    check_finite_dual_vertices, check_primal_vertex_norm_bounds, general_qp_action_window,
    PolytopeGeometry4d,
};
use symplectic::derivatives::{
    capacity_subgradients_a_from_exact_orbits, clarke_directional_derivative_a,
    systolic_ratio_gradient_a, volume_derivatives_a,
};

const FACET_COUNT: usize = 10;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const MAX_GENERATION_ATTEMPTS: u64 = 1_000;
const BRANCH_THRESHOLD_RELATIVE: f64 = 1.0e-3;
const ACTION_WINDOW_RELATIVE: f64 = 1.0e-2;
const TRACE_STEP: f64 = 1.0e-3;
const ENDPOINT_STEP: f64 = 1.0e-3;
const TRACE_CAP: usize = 1;
const RETAINED_TRACE_STEPS: &[f64] = &[1.0e-3, 1.0e-4];
const RETAINED_ENDPOINT_STEPS: &[f64] = &[1.0e-3, 1.0e-4, 1.0e-5, 1.0e-6];
const RETAINED_TRACE_CAP: usize = 8;
const MIN_OBSERVED_DELTA: f64 = 0.0;
const MIN_OBSERVED_RELATIVE_DELTA: f64 = 1.0e-3;

#[derive(Debug)]
struct Args {
    out: PathBuf,
    seed: u64,
    retained_preflight: bool,
}

#[derive(Clone, Serialize)]
struct GeometrySnapshot {
    capacity: f64,
    volume: f64,
    sys: f64,
    dual_vertices_rational: Vec<[String; 4]>,
    capacity_certificate: CapacityCertificateSnapshot,
}

#[derive(Clone, Serialize)]
#[serde(tag = "contract", rename_all = "snake_case")]
enum CapacityCertificateSnapshot {
    GeneralExactWindow {
        capacity_exact: String,
        maximum_action_multiple: String,
        returned_window_count: usize,
        near_active_count: usize,
    },
    ScalarBounds {
        lower: f64,
        upper: f64,
        maximum_relative_error: f64,
    },
}

#[derive(Serialize)]
struct GeneratorIdentity {
    kind: String,
    master_seed: u64,
    generation_attempt: Option<u64>,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
}

#[derive(Serialize)]
struct DirectionAttempt {
    direction_label: String,
    step: f64,
    predicted_delta_sys: Option<f64>,
    status: String,
    observed_delta_sys: Option<f64>,
    target_sys: Option<f64>,
}

#[derive(Serialize)]
struct TraceRow {
    iteration: usize,
    base_sys: f64,
    base_near_active_count: usize,
    effective_min_observed_delta: f64,
    generated_direction_labels: Vec<String>,
    attempted_direction_labels: Vec<String>,
    attempts: Vec<DirectionAttempt>,
    chosen_direction_label: Option<String>,
    chosen_step: Option<f64>,
    accepted: bool,
    stop_reason: String,
}

#[derive(Serialize)]
struct EndpointResult {
    status: String,
    trace_termination: String,
    endpoint_condition_status: String,
    base_sys: Option<f64>,
    base_near_active_count: Option<usize>,
    endpoint_steps: Vec<f64>,
    generated_direction_labels: Vec<String>,
    attempts: Vec<DirectionAttempt>,
    finite_positive_improvement_found: Option<bool>,
    threshold_improvement_found: Option<bool>,
    caveat: String,
}

#[derive(Serialize)]
struct ComputeBudget {
    base_state_evaluations: usize,
    finite_step_evaluations: usize,
    elapsed_ms: f64,
}

#[derive(Serialize)]
struct Configuration {
    run_mode: String,
    seeds: Vec<u64>,
    method_variant: String,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    trace_steps: Vec<f64>,
    trace_iteration_cap: usize,
    endpoint_steps: Vec<f64>,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
}

#[derive(Serialize)]
struct SmokeRow {
    schema: String,
    run_id: String,
    purpose: String,
    status: String,
    failure: Option<String>,
    generator: GeneratorIdentity,
    configuration: Configuration,
    start: Option<GeometrySnapshot>,
    trace: Vec<TraceRow>,
    final_state: Option<GeometrySnapshot>,
    endpoint: EndpointResult,
    compute_budget: ComputeBudget,
    caveat: String,
}

#[derive(Clone)]
struct BaseState {
    polytope: SysLandscapePolytopeCache,
    snapshot: GeometrySnapshot,
    sys_gradients: Vec<Vec<Vector4<f64>>>,
}

struct Direction {
    label: &'static str,
    vector: Vec<Vector4<f64>>,
    predicted_directional_derivative: Option<f64>,
}

struct FiniteEvaluation {
    polytope: SysLandscapePolytopeCache,
    snapshot: GeometrySnapshot,
}

struct StepOutcome {
    evaluation: Option<FiniteEvaluation>,
    failure: Option<String>,
}

fn main() {
    let args = parse_args();
    if let Some(parent) = args.out.parent() {
        fs::create_dir_all(parent).expect("create smoke output directory");
    }
    let row = run_one(args.seed, args.retained_preflight);
    let file = File::create(&args.out).expect("create smoke JSONL");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &row).expect("serialize smoke row");
    writer.write_all(b"\n").expect("write smoke row");
    writer.flush().expect("flush smoke row");
    println!("{}", args.out.display());
}

fn run_one(seed: u64, retained_preflight: bool) -> SmokeRow {
    let started = Instant::now();
    let configuration = configuration(seed, retained_preflight);
    let mut budget = ComputeBudget {
        base_state_evaluations: 0,
        finite_step_evaluations: 0,
        elapsed_ms: 0.0,
    };
    let mut generator = GeneratorIdentity {
        kind: "SysLandscapePolytopeCache::generate_random".to_string(),
        master_seed: seed,
        generation_attempt: None,
        facet_count: FACET_COUNT,
        h_min: H_MIN,
        h_max: H_MAX,
    };
    let Some((start_polytope, generation_attempt)) = random_start(seed) else {
        return failed_row(
            seed,
            generator,
            configuration,
            budget,
            started.elapsed().as_secs_f64() * 1_000.0,
            "random_start_generation_failed".to_string(),
        );
    };
    generator.generation_attempt = Some(generation_attempt);
    let initial = match compute_base_state(start_polytope) {
        Ok(base) => base,
        Err(err) => {
            return failed_row(
                seed,
                generator,
                configuration,
                budget,
                started.elapsed().as_secs_f64() * 1_000.0,
                err,
            )
        }
    };
    budget.base_state_evaluations += 1;
    let start_snapshot = initial.snapshot.clone();
    let mut current_polytope = initial.polytope.clone();
    let mut final_snapshot = initial.snapshot.clone();
    let mut trace = Vec::new();
    let mut trace_termination = "trace_iteration_cap".to_string();
    for iteration in 0..configuration.trace_iteration_cap {
        let base = if iteration == 0 {
            initial.clone()
        } else {
            match compute_base_state(current_polytope.clone()) {
                Ok(base) => {
                    budget.base_state_evaluations += 1;
                    base
                }
                Err(err) => {
                    trace.push(trace_failure(
                        iteration,
                        &final_snapshot,
                        format!("base_state_failed:{err}"),
                    ));
                    trace_termination = "trace_base_state_failed".to_string();
                    break;
                }
            }
        };
        let effective_threshold = threshold(base.snapshot.sys);
        let mut directions = match candidate_directions(&base) {
            Ok(directions) => directions,
            Err(err) => {
                trace.push(trace_failure(
                    iteration,
                    &base.snapshot,
                    format!("direction_generation_failed:{err}"),
                ));
                trace_termination = "trace_direction_generation_failed".to_string();
                break;
            }
        };
        directions.sort_by(|left, right| {
            right
                .predicted_directional_derivative
                .unwrap_or(f64::NEG_INFINITY)
                .total_cmp(
                    &left
                        .predicted_directional_derivative
                        .unwrap_or(f64::NEG_INFINITY),
                )
        });
        let generated_direction_labels = directions.iter().map(|d| d.label.to_string()).collect();
        let mut attempts = Vec::new();
        let mut chosen_direction_label = None;
        let mut chosen_step = None;
        let mut accepted = false;
        'directions: for direction in directions {
            for &step in &configuration.trace_steps {
                let outcome = evaluate_step(&base, &direction.vector, step);
                let predicted_delta_sys = direction
                    .predicted_directional_derivative
                    .map(|derivative| derivative * step);
                if let Some(evaluation) = outcome.evaluation {
                    budget.finite_step_evaluations += 1;
                    let observed_delta = evaluation.snapshot.sys - base.snapshot.sys;
                    let is_accepted = observed_delta > effective_threshold;
                    attempts.push(DirectionAttempt {
                        direction_label: direction.label.to_string(),
                        step,
                        predicted_delta_sys,
                        status: if is_accepted {
                            "accepted"
                        } else {
                            "below_threshold"
                        }
                        .to_string(),
                        observed_delta_sys: Some(observed_delta),
                        target_sys: Some(evaluation.snapshot.sys),
                    });
                    if is_accepted {
                        chosen_direction_label = Some(direction.label.to_string());
                        chosen_step = Some(step);
                        current_polytope = evaluation.polytope;
                        final_snapshot = evaluation.snapshot;
                        accepted = true;
                        break 'directions;
                    }
                } else {
                    attempts.push(DirectionAttempt {
                        direction_label: direction.label.to_string(),
                        step,
                        predicted_delta_sys,
                        status: outcome.failure.expect("failed evaluation has reason"),
                        observed_delta_sys: None,
                        target_sys: None,
                    });
                }
            }
        }
        trace.push(TraceRow {
            iteration,
            base_sys: base.snapshot.sys,
            base_near_active_count: base.sys_gradients.len(),
            effective_min_observed_delta: effective_threshold,
            generated_direction_labels,
            attempted_direction_labels: attempts
                .iter()
                .map(|a| a.direction_label.clone())
                .collect(),
            attempts,
            chosen_direction_label,
            chosen_step,
            accepted,
            stop_reason: if accepted {
                "accepted_observed_delta_above_threshold"
            } else {
                "all_generated_trace_steps_failed_observed_threshold"
            }
            .to_string(),
        });
        if !accepted {
            trace_termination = "all_generated_trace_steps_failed_observed_threshold".to_string();
            break;
        }
    }
    let endpoint = endpoint_scan(
        &current_polytope,
        &configuration,
        &trace_termination,
        &mut budget,
    );
    budget.elapsed_ms = started.elapsed().as_secs_f64() * 1_000.0;
    SmokeRow {
        schema: "gradient_ascent_observed_general_smoke_v2".to_string(),
        run_id: format!("observed-general-smoke-seed-{seed}"),
        purpose: if retained_preflight { "retained_mode_one_seed_preflight" } else { "plumbing_smoke_one_random_F10_start" }.to_string(),
        status: "completed".to_string(),
        failure: None,
        generator,
        configuration,
        start: Some(start_snapshot),
        trace,
        final_state: Some(final_snapshot),
        endpoint,
        compute_budget: budget,
        caveat: if retained_preflight {
            "One-seed retained-mode preflight only. It checks the retained producer path and records no 12-seed-panel evidence. It is not a local-maximum certificate, a candidate proposer, or thesis evidence."
        } else {
            "One deterministic plumbing smoke only. It is not a retained 12-seed run, endpoint-local-maximality evidence, a local-maximum certificate, a candidate proposer, or thesis evidence."
        }.to_string(),
    }
}

fn trace_failure(iteration: usize, base: &GeometrySnapshot, stop_reason: String) -> TraceRow {
    TraceRow {
        iteration,
        base_sys: base.sys,
        base_near_active_count: 0,
        effective_min_observed_delta: threshold(base.sys),
        generated_direction_labels: Vec::new(),
        attempted_direction_labels: Vec::new(),
        attempts: Vec::new(),
        chosen_direction_label: None,
        chosen_step: None,
        accepted: false,
        stop_reason,
    }
}

fn endpoint_scan(
    polytope: &SysLandscapePolytopeCache,
    configuration: &Configuration,
    trace_termination: &str,
    budget: &mut ComputeBudget,
) -> EndpointResult {
    let base = match compute_base_state(polytope.clone()) {
        Ok(base) => base,
        Err(err) => {
            return endpoint_failure(
                trace_termination,
                format!("endpoint_base_state_failed:{err}"),
            )
        }
    };
    budget.base_state_evaluations += 1;
    let threshold = threshold(base.snapshot.sys);
    let mut directions = match candidate_directions(&base) {
        Ok(directions) => directions,
        Err(err) => {
            return endpoint_failure(
                trace_termination,
                format!("endpoint_direction_generation_failed:{err}"),
            )
        }
    };
    directions.sort_by(|left, right| {
        right
            .predicted_directional_derivative
            .unwrap_or(f64::NEG_INFINITY)
            .total_cmp(
                &left
                    .predicted_directional_derivative
                    .unwrap_or(f64::NEG_INFINITY),
            )
    });
    let generated_direction_labels = directions.iter().map(|d| d.label.to_string()).collect();
    let mut attempts = Vec::new();
    let mut positive = false;
    let mut above_threshold = false;
    for direction in directions {
        for &step in &configuration.endpoint_steps {
            let outcome = evaluate_step(&base, &direction.vector, step);
            let predicted_delta_sys = direction
                .predicted_directional_derivative
                .map(|derivative| derivative * step);
            if let Some(evaluation) = outcome.evaluation {
                budget.finite_step_evaluations += 1;
                let observed_delta = evaluation.snapshot.sys - base.snapshot.sys;
                positive |= observed_delta > 0.0;
                above_threshold |= observed_delta > threshold;
                attempts.push(DirectionAttempt {
                    direction_label: direction.label.to_string(),
                    step,
                    predicted_delta_sys,
                    status: if observed_delta > threshold {
                        "above_threshold"
                    } else if observed_delta > 0.0 {
                        "positive_below_threshold"
                    } else {
                        "nonpositive"
                    }
                    .to_string(),
                    observed_delta_sys: Some(observed_delta),
                    target_sys: Some(evaluation.snapshot.sys),
                });
            } else {
                attempts.push(DirectionAttempt {
                    direction_label: direction.label.to_string(),
                    step,
                    predicted_delta_sys,
                    status: outcome.failure.expect("failed evaluation has reason"),
                    observed_delta_sys: None,
                    target_sys: None,
                });
            }
        }
    }
    let endpoint_condition_status =
        if trace_termination == "all_generated_trace_steps_failed_observed_threshold" {
            if above_threshold {
                "fails_checked_finite_grid"
            } else {
                "passes_checked_finite_grid"
            }
        } else {
            "not_evaluable_trace_did_not_stop"
        };
    EndpointResult {
        status: "completed_finite_direction_scan".to_string(),
        trace_termination: trace_termination.to_string(),
        endpoint_condition_status: endpoint_condition_status.to_string(),
        base_sys: Some(base.snapshot.sys),
        base_near_active_count: Some(base.sys_gradients.len()),
        endpoint_steps: configuration.endpoint_steps.clone(),
        generated_direction_labels,
        attempts,
        finite_positive_improvement_found: Some(positive),
        threshold_improvement_found: Some(above_threshold),
        caveat: "The scanned finite directions are only the candidate's generated set on the configured endpoint grid. Neither a negative result nor a positive result certifies endpoint local maximality.".to_string(),
    }
}

fn endpoint_failure(trace_termination: &str, status: String) -> EndpointResult {
    EndpointResult {
        status,
        trace_termination: trace_termination.to_string(),
        endpoint_condition_status: "not_evaluable_endpoint_scan_failed".to_string(),
        base_sys: None,
        base_near_active_count: None,
        endpoint_steps: Vec::new(),
        generated_direction_labels: Vec::new(),
        attempts: Vec::new(),
        finite_positive_improvement_found: None,
        threshold_improvement_found: None,
        caveat: "Endpoint scan did not complete; this does not imply an endpoint condition."
            .to_string(),
    }
}

fn failed_row(
    seed: u64,
    generator: GeneratorIdentity,
    configuration: Configuration,
    mut budget: ComputeBudget,
    elapsed_ms: f64,
    failure: String,
) -> SmokeRow {
    budget.elapsed_ms = elapsed_ms;
    SmokeRow {
        schema: "gradient_ascent_observed_general_smoke_v2".to_string(),
        run_id: format!("observed-general-smoke-seed-{seed}"),
        purpose: "plumbing_smoke_one_random_F10_start".to_string(),
        status: "failed".to_string(),
        failure: Some(failure),
        generator,
        configuration,
        start: None,
        trace: Vec::new(),
        final_state: None,
        endpoint: endpoint_failure(
            "not_run_after_initialization_failure",
            "not_run_after_initialization_failure".to_string(),
        ),
        compute_budget: budget,
        caveat: "A failed smoke is plumbing information only; it supplies no research or thesis evidence.".to_string(),
    }
}

fn configuration(seed: u64, retained_preflight: bool) -> Configuration {
    Configuration {
        run_mode: if retained_preflight {
            "retained_preflight"
        } else {
            "smoke"
        }
        .to_string(),
        seeds: vec![seed],
        method_variant: "iterative_exact_window_multi_direction_probe".to_string(),
        branch_threshold_relative: BRANCH_THRESHOLD_RELATIVE,
        action_window_relative: ACTION_WINDOW_RELATIVE,
        trace_steps: if retained_preflight {
            RETAINED_TRACE_STEPS.to_vec()
        } else {
            vec![TRACE_STEP]
        },
        trace_iteration_cap: if retained_preflight {
            RETAINED_TRACE_CAP
        } else {
            TRACE_CAP
        },
        endpoint_steps: if retained_preflight {
            RETAINED_ENDPOINT_STEPS.to_vec()
        } else {
            vec![ENDPOINT_STEP]
        },
        min_observed_delta: MIN_OBSERVED_DELTA,
        min_observed_relative_delta: MIN_OBSERVED_RELATIVE_DELTA,
    }
}

fn random_start(seed: u64) -> Option<(SysLandscapePolytopeCache, u64)> {
    (0..MAX_GENERATION_ATTEMPTS).find_map(|attempt| {
        SysLandscapePolytopeCache::generate_random(FACET_COUNT, H_MIN, H_MAX, seed, attempt)
            .map(|polytope| (polytope, attempt))
    })
}

fn threshold(base_sys: f64) -> f64 {
    MIN_OBSERVED_DELTA.max(MIN_OBSERVED_RELATIVE_DELTA * base_sys.abs())
}

fn compute_base_state(polytope: SysLandscapePolytopeCache) -> Result<BaseState, String> {
    let geometry = checked_capacity_geometry(&polytope)?;
    let maximum_action_multiple = BigRational::new(101.into(), 100.into());
    let window = general_qp_action_window(&geometry, maximum_action_multiple.clone())
        .map_err(|err| format!("near_active_capacity_failed:{err:?}"))?;
    let capacity_exact = window.capacity_exact().clone();
    let capacity = capacity_exact
        .to_f64()
        .filter(|value| value.is_finite() && *value > 0.0)
        .ok_or_else(|| "capacity_not_representable_as_f64".to_string())?;
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if !volume.is_finite() || volume <= 0.0 {
        return Err("volume_failed".to_string());
    }
    let sys = symplectic::systolic_ratio(capacity, volume);
    if !sys.is_finite() {
        return Err("sys_failed".to_string());
    }
    let near_active_multiple = BigRational::new(1001.into(), 1000.into());
    let near_active_cutoff = &capacity_exact * near_active_multiple;
    let near_active = window
        .witnesses()
        .iter()
        .filter(|witness| witness.action() <= near_active_cutoff)
        .cloned()
        .collect::<Vec<_>>();
    if near_active.is_empty() {
        return Err("empty_near_active_window".to_string());
    }
    let volume_gradients = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|err| format!("volume_derivative_failed:{err:?}"))?;
    let capacity_gradients =
        capacity_subgradients_a_from_exact_orbits(&polytope.dual_vertices_f64, &near_active)
            .map_err(|err| format!("capacity_derivative_failed:{err:?}"))?;
    let sys_gradients = capacity_gradients
        .iter()
        .map(|gradient| systolic_ratio_gradient_a(capacity, volume, gradient, &volume_gradients))
        .collect();
    Ok(BaseState {
        snapshot: GeometrySnapshot {
            capacity,
            volume,
            sys,
            dual_vertices_rational: dual_vertices_rational_strings(&polytope),
            capacity_certificate: CapacityCertificateSnapshot::GeneralExactWindow {
                capacity_exact: capacity_exact.to_string(),
                maximum_action_multiple: maximum_action_multiple.to_string(),
                returned_window_count: window.witnesses().len(),
                near_active_count: near_active.len(),
            },
        },
        polytope,
        sys_gradients,
    })
}

fn candidate_directions(base: &BaseState) -> Result<Vec<Direction>, String> {
    let first = base
        .sys_gradients
        .first()
        .ok_or_else(|| "empty_near_active_gradient_set".to_string())?;
    let single = normalize(first).ok_or_else(|| "zero_single_gradient".to_string())?;
    let mut directions = vec![
        Direction {
            label: "single_near_active_gradient",
            vector: single.clone(),
            predicted_directional_derivative: None,
        },
        Direction {
            label: "negative_single_near_active_gradient",
            vector: single.iter().map(|vector| -*vector).collect(),
            predicted_directional_derivative: None,
        },
    ];
    if base.sys_gradients.len() > 1 {
        if let Some(vector) = maximin_direction(&base.sys_gradients) {
            directions.push(Direction {
                label: "near_active_maximin_direction",
                vector,
                predicted_directional_derivative: None,
            });
        }
    }
    for direction in &mut directions {
        direction.predicted_directional_derivative =
            clarke_directional_derivative_a(&base.sys_gradients, &direction.vector).ok();
    }
    Ok(directions)
}

fn evaluate_step(base: &BaseState, direction: &[Vector4<f64>], step: f64) -> StepOutcome {
    let duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(direction)
        .map(|(dual, delta)| dual + step * delta)
        .collect();
    let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
        return StepOutcome {
            evaluation: None,
            failure: Some("target_polytope_construction_failed".to_string()),
        };
    };
    let geometry = match checked_capacity_geometry(&polytope) {
        Ok(geometry) => geometry,
        Err(err) => {
            return StepOutcome {
                evaluation: None,
                failure: Some(format!("target_capacity_geometry_failed:{err}")),
            }
        }
    };
    let capacity_result = match capacity(&geometry) {
        Ok(capacity) => capacity,
        Err(err) => {
            return StepOutcome {
                evaluation: None,
                failure: Some(format!("target_capacity_failed:{err:?}")),
            }
        }
    };
    let maximum_relative_error = 1e-10;
    let capacity = match capacity_value(&capacity_result, maximum_relative_error) {
        Ok(value) => value,
        Err(err) => {
            return StepOutcome {
                evaluation: None,
                failure: Some(format!("target_capacity_bounds_too_wide:{err}")),
            }
        }
    };
    let bounds = capacity_result.bounds();
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    let sys = symplectic::systolic_ratio(capacity, volume);
    if !volume.is_finite() || volume <= 0.0 || !sys.is_finite() {
        return StepOutcome {
            evaluation: None,
            failure: Some("target_volume_or_sys_failed".to_string()),
        };
    }
    StepOutcome {
        evaluation: Some(FiniteEvaluation {
            snapshot: GeometrySnapshot {
                capacity,
                volume,
                sys,
                dual_vertices_rational: dual_vertices_rational_strings(&polytope),
                capacity_certificate: CapacityCertificateSnapshot::ScalarBounds {
                    lower: bounds.lower(),
                    upper: bounds.upper(),
                    maximum_relative_error,
                },
            },
            polytope,
        }),
        failure: None,
    }
}

fn maximin_direction(gradients: &[Vec<Vector4<f64>>]) -> Option<Vec<Vector4<f64>>> {
    let facet_count = gradients.first()?.len();
    let mut vars = variables!();
    let direction_vars: Vec<_> = (0..facet_count * 4)
        .map(|_| vars.add(variable().min(-1.0).max(1.0)))
        .collect();
    let t = vars.add(variable().min(f64::NEG_INFINITY));
    let mut model = vars.maximise(Expression::from(t)).using(default_solver);
    for gradient in gradients {
        let mut lhs = Expression::from(0.0);
        for (coefficient, variable) in flatten(gradient).iter().zip(&direction_vars) {
            if *coefficient != 0.0 {
                lhs += *coefficient * *variable;
            }
        }
        model = model.with(constraint!(lhs >= t));
    }
    let solution = model.solve().ok()?;
    let flat: Vec<f64> = direction_vars
        .iter()
        .map(|variable| solution.value(*variable))
        .collect();
    normalize(&unflatten(&flat))
}

fn checked_capacity_geometry(
    polytope: &SysLandscapePolytopeCache,
) -> Result<PolytopeGeometry4d, String> {
    check_facet_count(polytope.dual_vertices_f64.len())
        .map_err(|error| format!("facet-count check failed: {error}"))?;
    check_finite_dual_vertices(&polytope.dual_vertices_f64)
        .map_err(|error| format!("finite-coordinate check failed: {error}"))?;
    check_dual_vertex_norm_bounds(&polytope.dual_vertices_f64)
        .map_err(|error| format!("dual-vertex norm check failed: {error}"))?;
    let geometry = PolytopeGeometry4d {
        dual_vertices: polytope.dual_vertices_f64.clone(),
        dual_vertices_exact: vectors_from_arrays(&polytope.dual_vertices),
        primal_vertices_exact: vectors_from_arrays(&polytope.vertices),
        vertex_facet_incidence: polytope.vertex_facet_incidence.clone(),
    };
    check_primal_vertex_norm_bounds(&geometry)
        .map_err(|error| format!("primal-vertex norm check failed: {error}"))?;
    Ok(geometry)
}

fn vectors_from_arrays(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|row| {
            Vector4::new(
                row[0].clone(),
                row[1].clone(),
                row[2].clone(),
                row[3].clone(),
            )
        })
        .collect()
}

fn flatten(vectors: &[Vector4<f64>]) -> Vec<f64> {
    vectors
        .iter()
        .flat_map(|v| [v[0], v[1], v[2], v[3]])
        .collect()
}

fn unflatten(values: &[f64]) -> Vec<Vector4<f64>> {
    values
        .chunks_exact(4)
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect()
}

fn normalize(vectors: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let norm = vectors.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
    (norm > 0.0 && norm.is_finite()).then(|| vectors.iter().map(|v| v / norm).collect())
}

fn parse_args() -> Args {
    let mut out = None;
    let mut seed = 42u64;
    let mut retained_preflight = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = Some(PathBuf::from(args.next().expect("--out requires a path"))),
            "--seed" => {
                seed = args
                    .next()
                    .expect("--seed requires a u64")
                    .parse()
                    .expect("--seed must be a u64")
            }
            "--retained-preflight" => retained_preflight = true,
            "--help" | "-h" => {
                eprintln!(
                    "Usage: sys-gradient-ascent-observed-general [--seed U64] [--out PATH] [--retained-preflight]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        out: out.unwrap_or_else(|| default_output_path(retained_preflight)),
        seed,
        retained_preflight,
    }
}

fn default_output_path(retained_preflight: bool) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_millis();
    std::env::temp_dir()
        .join(format!(
            "sys-gradient-ascent-observed-general-{}-{}-{stamp}",
            if retained_preflight {
                "retained-preflight"
            } else {
                "smoke"
            },
            std::process::id()
        ))
        .join("smoke.jsonl")
}
