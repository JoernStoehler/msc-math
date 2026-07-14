//! Shared optimizer-suite harness for the literal branch-gradient explorer.
//!
//! The two safeguard policies intentionally differ only in their acceptance
//! predicate.  The maximin and poll policies have explicit radius semantics.
//! Every target (including invalid and rejected targets) increments the exact
//! evaluator counter in the trajectory artifact.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::derivatives::{
    capacity_derivatives_a_from_orbit, systolic_ratio_gradient_a, volume_derivatives_a,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

const MAX_BACKTRACK_HALVINGS: usize = 20;
const MAX_TARGET_EVALUATIONS: usize = 100;
const MAX_RADIUS_SHRINKS: usize = 8;
const NEAR_ACTIVE_RELATIVE_WINDOW: f64 = 1.0e-3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Policy {
    Literal,
    InvalidityOnly,
    Monotone,
    Maximin,
    Poll,
}
impl Policy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Literal => "literal",
            Self::InvalidityOnly => "invalidity_only",
            Self::Monotone => "monotone_backtracking",
            Self::Maximin => "near_active_maximin",
            Self::Poll => "positive_spanning_poll",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "literal" => Self::Literal,
            "invalidity-only" | "invalidity_only" => Self::InvalidityOnly,
            "monotone" | "monotone_backtracking" => Self::Monotone,
            "maximin" | "near_active_maximin" => Self::Maximin,
            "poll" | "positive_spanning_poll" => Self::Poll,
            _ => return None,
        })
    }
}

#[derive(Debug)]
struct Cli {
    polytope_table: PathBuf,
    out_dir: PathBuf,
    facet_count: usize,
    start_count: usize,
    exclude: Vec<String>,
    policies: Vec<Policy>,
    etas: Vec<f64>,
    updates: usize,
    panel_starts: usize,
    panel_updates: usize,
    smoke: bool,
    parallelism: usize,
}
#[derive(Debug, Deserialize, Clone)]
struct PanelRow {
    #[serde(alias = "name")]
    poly_id: String,
    facet_count: Option<usize>,
    #[serde(alias = "dual_vertices")]
    dual_vertices_f64: Vec<[f64; 4]>,
}
#[derive(Debug, Clone)]
struct Start {
    id: String,
    duals: Vec<Vector4<f64>>,
}
#[derive(Debug)]
struct State {
    polytope: SysLandscapePolytopeCache,
    sys: f64,
    volume: f64,
    action: f64,
    sigma: Vec<usize>,
    gradient: Vec<Vector4<f64>>,
    near_gradients: Vec<Vec<Vector4<f64>>>,
}

#[derive(Debug, Serialize)]
struct AttemptRow {
    policy: String,
    start_id: String,
    nominal_eta: f64,
    iteration: usize,
    attempt: usize,
    rate: f64,
    target_evaluations: usize,
    target_valid: bool,
    target_sys: Option<f64>,
    delta: Option<f64>,
    accepted: bool,
    reason: String,
    best_sys: f64,
    best_iteration: usize,
    radius: Option<f64>,
    direction: Option<String>,
}
#[derive(Debug, Serialize)]
struct TrajectorySummary {
    policy: String,
    start_id: String,
    nominal_eta: f64,
    requested_updates: usize,
    committed_updates: usize,
    initial_sys: f64,
    final_sys: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    target_evaluations: usize,
    invalid_attempts: usize,
    rejected_attempts: usize,
    accepted_decreases: usize,
    backtracking_attempts: usize,
    stalls: usize,
    failure: Option<String>,
    final_radius: Option<f64>,
}
#[derive(Debug, Serialize)]
struct Provenance {
    command: Vec<String>,
    source_head: Option<String>,
    source_input: String,
    source_input_blake3: String,
    implementation: String,
    implementation_blake3: String,
    policies: Vec<String>,
    nominal_etas: Vec<f64>,
    updates: usize,
    panel_starts: usize,
    panel_updates: usize,
    max_backtrack_halvings: usize,
    near_active_window_relative: f64,
    poll_slice: String,
    evaluator_accounting: String,
}
#[derive(Debug, Serialize)]
struct RunSummary {
    provenance: String,
    wall_seconds: f64,
    trajectories: Vec<TrajectorySummary>,
    total_target_evaluations: usize,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("create output directory");
    let rows: Vec<PanelRow> = load_jsonl(&cli.polytope_table);
    let starts: Vec<Start> = rows
        .into_iter()
        .filter(|r| r.facet_count == Some(cli.facet_count))
        .filter(|r| !cli.exclude.contains(&r.poly_id))
        .take(if cli.smoke { 1 } else { cli.start_count })
        .map(|r| Start {
            id: r.poly_id,
            duals: r
                .dual_vertices_f64
                .into_iter()
                .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
                .collect(),
        })
        .collect();
    assert_eq!(
        starts.len(),
        if cli.smoke { 1 } else { cli.start_count },
        "frozen source selection did not produce requested starts"
    );
    let implementation =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("literal-naive-gradient/optimizer_suite.rs");
    let provenance = Provenance { command: std::env::args().collect(), source_head: git_output(&["rev-parse","HEAD"]), source_input: cli.polytope_table.display().to_string(), source_input_blake3: hash_file(&cli.polytope_table), implementation: implementation.display().to_string(), implementation_blake3: hash_file(&implementation), policies: cli.policies.iter().map(|p|p.as_str().to_string()).collect(), nominal_etas: cli.etas.clone(), updates: cli.updates, panel_starts: cli.panel_starts, panel_updates: cli.panel_updates, max_backtrack_halvings: MAX_BACKTRACK_HALVINGS, near_active_window_relative: NEAR_ACTIVE_RELATIVE_WINDOW, poll_slice: "first dual vertex, four coordinate axes, +/- deterministic positive-spanning poll".to_string(), evaluator_accounting: "initial state excluded; every target proposal, invalidity retry, rejection, and poll candidate increments target_evaluations; every policy stops at 100 target evaluations and an incomplete poll is censored without acceptance or radius shrink".to_string() };
    write_json(cli.out_dir.join("run-provenance.json"), &provenance);
    let began = Instant::now();
    let mut trajectories = Vec::new();
    let (starts_to_run, updates, etas) = if cli.smoke {
        (&starts[..], 1, vec![1e-3])
    } else {
        (&starts[..], cli.updates, cli.etas.clone())
    };
    for policy in &cli.policies {
        let (local_starts, local_updates, local_etas) = match policy {
            Policy::Maximin | Policy::Poll => (
                &starts_to_run[..cli.panel_starts.min(starts_to_run.len())],
                cli.panel_updates,
                vec![cli.etas[2.min(cli.etas.len() - 1)]],
            ),
            _ => (starts_to_run, updates, etas.clone()),
        };
        for start in local_starts {
            for &eta in &local_etas {
                trajectories.push(run_trajectory(
                    *policy,
                    start,
                    eta,
                    local_updates,
                    &cli.out_dir,
                ));
            }
        }
    }
    let total = trajectories.iter().map(|t| t.target_evaluations).sum();
    write_json(
        cli.out_dir.join("summary.json"),
        &RunSummary {
            provenance: "run-provenance.json".to_string(),
            wall_seconds: began.elapsed().as_secs_f64(),
            trajectories,
            total_target_evaluations: total,
        },
    );
}

fn run_trajectory(
    policy: Policy,
    start: &Start,
    nominal_eta: f64,
    updates: usize,
    out_dir: &Path,
) -> TrajectorySummary {
    let dir = out_dir
        .join("trajectories")
        .join(safe_id(policy.as_str()))
        .join(safe_id(&start.id));
    fs::create_dir_all(&dir).expect("trajectory dir");
    let path = dir.join(format!("eta-{}.jsonl", eta_label(nominal_eta)));
    let mut writer = BufWriter::new(File::create(&path).expect("trajectory"));
    let mut current = compute_state(&start.duals).expect("initial state failed");
    let initial = current.sys;
    let mut best = initial;
    let mut best_iteration = 0;
    let mut committed = 0;
    let mut target_evals = 0;
    let mut invalid = 0;
    let mut rejected = 0;
    let mut accepted_decreases = 0;
    let mut backtracks = 0;
    let mut stalls = 0;
    let mut radius = if matches!(policy, Policy::Maximin | Policy::Poll) {
        Some(nominal_eta)
    } else {
        None
    };
    let mut failure = None;
    write_attempt(
        &mut writer,
        &AttemptRow {
            policy: policy.as_str().to_string(),
            start_id: start.id.clone(),
            nominal_eta,
            iteration: 0,
            attempt: 0,
            rate: 0.0,
            target_evaluations: 0,
            target_valid: true,
            target_sys: Some(initial),
            delta: None,
            accepted: true,
            reason: "initial".to_string(),
            best_sys: best,
            best_iteration,
            radius,
            direction: None,
        },
    );
    for iteration in 1..=updates {
        if target_evals >= MAX_TARGET_EVALUATIONS {
            failure = Some("method_stop_target_evaluation_budget".to_string());
            stalls += 1;
            break;
        }
        match policy {
            Policy::Literal | Policy::InvalidityOnly | Policy::Monotone => {
                let grad = current.gradient.clone();
                let before = current.polytope.dual_vertices_f64.clone();
                let mut rate = nominal_eta;
                let mut accepted_state = None;
                let mut attempt = 0;
                loop {
                    if target_evals >= MAX_TARGET_EVALUATIONS {
                        stalls += 1;
                        failure = Some("method_stop_target_evaluation_budget".to_string());
                        break;
                    }
                    let after = add_step(&before, &grad, rate);
                    target_evals += 1;
                    let target = compute_state(&after);
                    let (valid, sys, delta) = match &target {
                        Ok(s) => (true, Some(s.sys), Some(s.sys - current.sys)),
                        Err(_) => (false, None, None),
                    };
                    let mut accept = valid;
                    let mut reason = if !valid {
                        "invalid".to_string()
                    } else {
                        "valid".to_string()
                    };
                    if !valid {
                        invalid += 1;
                    }
                    if matches!(policy, Policy::Monotone) && valid && delta.unwrap_or(0.0) <= 0.0 {
                        accept = false;
                        rejected += 1;
                        reason = "rejected_nonincrease".to_string();
                    }
                    if accept {
                        let next = target.expect("accepted target");
                        if next.sys < current.sys {
                            accepted_decreases += 1;
                        }
                        accepted_state = Some(next);
                        write_attempt(
                            &mut writer,
                            &AttemptRow {
                                policy: policy.as_str().to_string(),
                                start_id: start.id.clone(),
                                nominal_eta,
                                iteration,
                                attempt,
                                rate,
                                target_evaluations: target_evals,
                                target_valid: true,
                                target_sys: sys,
                                delta,
                                accepted: true,
                                reason,
                                best_sys: best,
                                best_iteration,
                                radius,
                                direction: None,
                            },
                        );
                        break;
                    }
                    write_attempt(
                        &mut writer,
                        &AttemptRow {
                            policy: policy.as_str().to_string(),
                            start_id: start.id.clone(),
                            nominal_eta,
                            iteration,
                            attempt,
                            rate,
                            target_evaluations: target_evals,
                            target_valid: valid,
                            target_sys: sys,
                            delta,
                            accepted: false,
                            reason: reason.clone(),
                            best_sys: best,
                            best_iteration,
                            radius,
                            direction: None,
                        },
                    );
                    if matches!(policy, Policy::InvalidityOnly) && valid {
                        accepted_state = Some(target.expect("valid target"));
                        break;
                    }
                    if matches!(policy, Policy::Literal) {
                        failure = Some(
                            if !valid {
                                "invalid_target"
                            } else {
                                "literal_target_rejected_unexpectedly"
                            }
                            .to_string(),
                        );
                        break;
                    }
                    if attempt >= MAX_BACKTRACK_HALVINGS {
                        stalls += 1;
                        failure = Some("method_stall_backtracking_safety_bound".to_string());
                        break;
                    }
                    attempt += 1;
                    rate *= 0.5;
                    backtracks += 1;
                }
                let Some(next) = accepted_state else { break };
                current = next;
                committed += 1;
                if current.sys > best {
                    best = current.sys;
                    best_iteration = iteration;
                }
            }
            Policy::Maximin => {
                let Some(r) = radius else {
                    failure = Some("method_stop_radius_missing".to_string());
                    break;
                };
                let Some(dir) = maximin_direction(&current.near_gradients) else {
                    failure = Some("method_stop_missing_common_direction".to_string());
                    break;
                };
                let after = add_step(&current.polytope.dual_vertices_f64, &dir, r);
                target_evals += 1;
                let target = compute_state(&after);
                let (valid, sys, delta) = match &target {
                    Ok(s) => (true, Some(s.sys), Some(s.sys - current.sys)),
                    Err(_) => (false, None, None),
                };
                let accept = valid && delta.unwrap_or(0.0) > 0.0;
                if !valid {
                    invalid += 1;
                }
                if !accept {
                    rejected += 1;
                    radius = Some(r * 0.5);
                    stalls += usize::from(radius.unwrap_or(0.0) < 1e-12);
                } else {
                    let next = target.unwrap();
                    current = next;
                    committed += 1;
                    if current.sys > best {
                        best = current.sys;
                        best_iteration = iteration;
                    }
                    radius = Some(if delta.unwrap_or(0.0) > 0.0 {
                        r.min(1.0) * 1.25
                    } else {
                        r * 0.5
                    });
                }
                write_attempt(
                    &mut writer,
                    &AttemptRow {
                        policy: policy.as_str().to_string(),
                        start_id: start.id.clone(),
                        nominal_eta,
                        iteration,
                        attempt: 0,
                        rate: r,
                        target_evaluations: target_evals,
                        target_valid: valid,
                        target_sys: sys,
                        delta,
                        accepted: accept,
                        reason: if accept {
                            "radius_expand".to_string()
                        } else {
                            "radius_shrink_or_stop".to_string()
                        },
                        best_sys: best,
                        best_iteration,
                        radius,
                        direction: Some("near_active_maximin".to_string()),
                    },
                );
                if radius.unwrap_or(0.0) < 1e-12 {
                    failure = Some("method_stop_shrunken_radius".to_string());
                    break;
                }
            }
            Policy::Poll => {
                let Some(r) = radius else {
                    failure = Some("method_stop_radius_missing".to_string());
                    break;
                };
                let dirs = poll_directions(current.polytope.dual_vertices_f64.len());
                let mut candidate = None;
                let mut candidate_delta = f64::NEG_INFINITY;
                let mut poll_complete = true;
                for (label, dir) in dirs {
                    if target_evals >= MAX_TARGET_EVALUATIONS {
                        poll_complete = false;
                        failure = Some("method_stop_target_evaluation_budget".to_string());
                        stalls += 1;
                        break;
                    }
                    let after = add_step(&current.polytope.dual_vertices_f64, &dir, r);
                    target_evals += 1;
                    let target = compute_state(&after);
                    let (valid, sys, delta) = match &target {
                        Ok(s) => (true, Some(s.sys), Some(s.sys - current.sys)),
                        Err(_) => (false, None, None),
                    };
                    if !valid {
                        invalid += 1;
                    }
                    if let Some(d) = delta {
                        if d > candidate_delta {
                            candidate_delta = d;
                            candidate = Some((target.unwrap(), label.clone(), sys, d));
                        }
                    }
                    write_attempt(
                        &mut writer,
                        &AttemptRow {
                            policy: policy.as_str().to_string(),
                            start_id: start.id.clone(),
                            nominal_eta,
                            iteration,
                            attempt: target_evals,
                            rate: r,
                            target_evaluations: target_evals,
                            target_valid: valid,
                            target_sys: sys,
                            delta,
                            accepted: false,
                            reason: "poll_candidate".to_string(),
                            best_sys: best,
                            best_iteration,
                            radius,
                            direction: Some(label),
                        },
                    );
                }
                if !poll_complete {
                    break;
                }
                if let Some((next, label, sys, d)) = candidate.filter(|(_, _, _, d)| *d > 0.0) {
                    current = next;
                    committed += 1;
                    if current.sys > best {
                        best = current.sys;
                        best_iteration = iteration;
                    }
                    radius = Some((r * 1.1).min(1.0));
                    write_attempt(
                        &mut writer,
                        &AttemptRow {
                            policy: policy.as_str().to_string(),
                            start_id: start.id.clone(),
                            nominal_eta,
                            iteration,
                            attempt: target_evals,
                            rate: r,
                            target_evaluations: target_evals,
                            target_valid: true,
                            target_sys: sys,
                            delta: Some(d),
                            accepted: true,
                            reason: "poll_best_improves".to_string(),
                            best_sys: best,
                            best_iteration,
                            radius,
                            direction: Some(label),
                        },
                    );
                } else {
                    rejected += 1;
                    radius = Some(r * 0.5);
                    write_attempt(
                        &mut writer,
                        &AttemptRow {
                            policy: policy.as_str().to_string(),
                            start_id: start.id.clone(),
                            nominal_eta,
                            iteration,
                            attempt: target_evals,
                            rate: r,
                            target_evaluations: target_evals,
                            target_valid: true,
                            target_sys: Some(current.sys),
                            delta: Some(0.0),
                            accepted: false,
                            reason: "poll_no_improvement".to_string(),
                            best_sys: best,
                            best_iteration,
                            radius,
                            direction: None,
                        },
                    );
                    if radius.unwrap_or(0.0) < 1e-12 {
                        failure = Some("method_stop_shrunken_radius".to_string());
                        break;
                    }
                }
            }
        }
    }
    writer.flush().expect("flush trajectory");
    TrajectorySummary {
        policy: policy.as_str().to_string(),
        start_id: start.id.clone(),
        nominal_eta,
        requested_updates: updates,
        committed_updates: committed,
        initial_sys: initial,
        final_sys: failure.is_none().then_some(current.sys),
        best_sys: best,
        best_iteration,
        target_evaluations: target_evals,
        invalid_attempts: invalid,
        rejected_attempts: rejected,
        accepted_decreases,
        backtracking_attempts: backtracks,
        stalls,
        failure,
        final_radius: radius,
    }
}

fn write_attempt(writer: &mut BufWriter<File>, row: &AttemptRow) {
    serde_json::to_writer(&mut *writer, row).expect("serialize attempt");
    writer.write_all(b"\n").expect("write attempt");
}
fn add_step(before: &[Vector4<f64>], grad: &[Vector4<f64>], rate: f64) -> Vec<Vector4<f64>> {
    before.iter().zip(grad).map(|(a, g)| a + rate * g).collect()
}

fn compute_state(duals: &[Vector4<f64>]) -> Result<State, String> {
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(duals.to_vec())
        .ok_or("updated_state_invalid_geometry")?;
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if !volume.is_finite() || volume <= 0.0 {
        return Err("exact_volume_failed".into());
    };
    let capacity =
        capacity_all_safe(&polytope).map_err(|e| format!("exact_full_capacity_failed:{e:?}"))?;
    let sys = symplectic::systolic_ratio(capacity.min_action, volume);
    if !sys.is_finite() {
        return Err("exact_full_sys_computation_failed".into());
    };
    let admissible: Vec<_> = capacity
        .orbits
        .iter()
        .filter(|o| {
            matches!(
                o.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .collect();
    let best = admissible
        .iter()
        .min_by(|a, b| {
            a.action
                .total_cmp(&b.action)
                .then_with(|| a.sigma.cmp(&b.sigma))
        })
        .ok_or("no_admissible_minimizing_sigma_branch")?;
    let best_action = best.action;
    let best_sigma = best.sigma.clone();
    let dvol = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|e| format!("volume_derivative_failed:{e:?}"))?;
    let grad_for = |orbit: &symplectic::OrbitKktData| -> Result<Vec<Vector4<f64>>, String> {
        let dc = capacity_derivatives_a_from_orbit(&polytope.dual_vertices_f64, orbit)
            .map_err(|e| format!("branch_derivative_failed:{e:?}"))?;
        Ok(systolic_ratio_gradient_a(orbit.action, volume, &dc, &dvol))
    };
    let gradient = grad_for(best)?;
    let cutoff = capacity.min_action * (1.0 + NEAR_ACTIVE_RELATIVE_WINDOW);
    let near: Vec<_> = admissible
        .iter()
        .filter(|o| o.action <= cutoff)
        .map(|o| grad_for(o))
        .collect::<Result<_, _>>()?;
    let near_gradients = if near.is_empty() {
        vec![gradient.clone()]
    } else {
        near
    };
    Ok(State {
        polytope,
        sys,
        volume,
        action: best_action,
        sigma: best_sigma,
        gradient,
        near_gradients,
    })
}
fn maximin_direction(grads: &[Vec<Vector4<f64>>]) -> Option<Vec<Vector4<f64>>> {
    let f = grads.first()?.len();
    let dim = f * 4;
    let mut vars = variables!();
    let xs: Vec<_> = (0..dim)
        .map(|_| vars.add(variable().min(-1.).max(1.)))
        .collect();
    let t = vars.add(variable().min(f64::NEG_INFINITY));
    let mut model = vars.maximise(Expression::from(t)).using(default_solver);
    for g in grads {
        let flat = flatten(g);
        let mut lhs = Expression::from(0.);
        for (c, x) in flat.iter().zip(&xs) {
            lhs += *c * *x;
        }
        model = model.with(constraint!(lhs >= t));
    }
    let sol = model.solve().ok()?;
    let d: Vec<f64> = xs.iter().map(|x| sol.value(*x)).collect();
    normalize(&unflatten(&d))
}
fn normalize(v: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let n = v.iter().map(|x| x.dot(x)).sum::<f64>().sqrt();
    (n.is_finite() && n > 1e-14).then(|| v.iter().map(|x| *x / n).collect())
}
fn flatten(v: &[Vector4<f64>]) -> Vec<f64> {
    v.iter().flat_map(|x| [x[0], x[1], x[2], x[3]]).collect()
}
fn unflatten(v: &[f64]) -> Vec<Vector4<f64>> {
    v.chunks_exact(4)
        .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
        .collect()
}
fn poll_directions(facets: usize) -> Vec<(String, Vec<Vector4<f64>>)> {
    let mut out = Vec::new();
    for k in 0..4 {
        let mut p = vec![Vector4::zeros(); facets];
        p[0][k] = 1.;
        out.push((format!("slice+e{k}"), p.clone()));
        p[0][k] = -1.;
        out.push((format!("slice-e{k}"), p));
    }
    out
}
fn capacity_all_safe(
    polytope: &SysLandscapePolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if let Ok(c) = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
        let tr=symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(&polytope.facet_intersection_is_nonempty,&polytope.omega_signs);
        let (o, i) = solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &c.q_indices,
            &c.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &tr,
        )
        .map_err(|_| OrbitSearchError::NumericalFailure)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            o,
            i,
            0.,
            OrbitGuaranteeMode::AllSafe,
        )
    } else {
        let tr=symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(&polytope.facet_intersection_is_nonempty,&polytope.omega_signs);
        let (o, i) = solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &tr)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            o,
            i,
            0.,
            OrbitGuaranteeMode::AllSafe,
        )
    }
}
fn parse_args() -> Cli {
    let mut table = PathBuf::from("experiments/sys-datascience/produce/random.jsonl");
    let mut out =
        PathBuf::from("experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/suite");
    let mut facet = 6;
    let mut starts = 6;
    let mut exclude = vec!["random_F6_s0_1".to_string()];
    let mut policies = vec![
        Policy::InvalidityOnly,
        Policy::Monotone,
        Policy::Maximin,
        Policy::Poll,
    ];
    let mut etas = vec![1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.];
    let mut updates = 100;
    let mut panel_starts = 2;
    let mut panel_updates = 15;
    let mut smoke = false;
    let mut parallelism = 1;
    let mut a = std::env::args().skip(1);
    while let Some(x) = a.next() {
        match x.as_str() {
            "--polytope-table" => table = PathBuf::from(a.next().unwrap()),
            "--out-dir" => out = PathBuf::from(a.next().unwrap()),
            "--facet-count" => facet = a.next().unwrap().parse().unwrap(),
            "--start-count" => starts = a.next().unwrap().parse().unwrap(),
            "--exclude-start-ids" => {
                exclude = a.next().unwrap().split(',').map(str::to_string).collect()
            }
            "--policies" => {
                policies = a
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| Policy::parse(s).unwrap_or_else(|| panic!("unknown policy {s}")))
                    .collect()
            }
            "--etas" => {
                etas = a
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            }
            "--updates" => updates = a.next().unwrap().parse().unwrap(),
            "--panel-starts" => panel_starts = a.next().unwrap().parse().unwrap(),
            "--panel-updates" => panel_updates = a.next().unwrap().parse().unwrap(),
            "--smoke" => smoke = true,
            "--parallelism" => parallelism = a.next().unwrap().parse().unwrap(),
            "--help" => {
                println!("optimizer suite");
                std::process::exit(0)
            }
            _ => panic!("unsupported argument {x}"),
        }
    }
    assert!(parallelism > 0);
    Cli {
        polytope_table: table,
        out_dir: out,
        facet_count: facet,
        start_count: starts,
        exclude,
        policies,
        etas,
        updates,
        panel_starts,
        panel_updates,
        smoke,
        parallelism,
    }
}
fn load_jsonl<T: for<'de> Deserialize<'de>>(p: &Path) -> Vec<T> {
    BufReader::new(File::open(p).unwrap())
        .lines()
        .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
        .collect()
}
fn write_json<T: Serialize>(p: PathBuf, v: &T) {
    serde_json::to_writer_pretty(File::create(p).unwrap(), v).unwrap()
}
fn safe_id(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
fn eta_label(e: f64) -> String {
    format!("{e:.0e}").replace('+', "")
}
fn hash_file(p: &Path) -> String {
    blake3::hash(&fs::read(p).unwrap()).to_hex().to_string()
}
fn git_output(a: &[&str]) -> Option<String> {
    let o = Command::new("git").args(a).output().ok()?;
    o.status
        .success()
        .then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
}
