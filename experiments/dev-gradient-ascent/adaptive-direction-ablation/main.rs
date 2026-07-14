//! Adaptive direction-model ablation with strict common exact evaluation.
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

const MAX_TARGET_EVALUATIONS: usize = 100;
const NEAR_ACTIVE_RELATIVE_WINDOW: f64 = 1.0e-3;
const CANDIDATE_WINDOW_RELATIVE_GAP: f64 = 1.0e-2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Policy {
    BranchGradient,
    NearActiveMaximin,
    CandidateWindowMaximin,
    SingleBranchBoxSteepest,
}
impl Policy {
    fn as_str(self) -> &'static str {
        match self {
            Self::BranchGradient => "inf_normalized_branch_gradient",
            Self::NearActiveMaximin => "near_active_box_lp_maximin",
            Self::CandidateWindowMaximin => "candidate_window_box_lp_maximin",
            Self::SingleBranchBoxSteepest => "single_branch_box_steepest",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "branch_gradient" | "normalized_branch_gradient" | "inf_normalized_branch_gradient" => {
                Self::BranchGradient
            }
            "near_active_maximin"
            | "near_active_zero_gap_maximin"
            | "near_active_box_lp_maximin" => Self::NearActiveMaximin,
            "candidate_window_maximin"
            | "candidate_window_gap_aware_maximin"
            | "candidate_window_box_lp_maximin" => Self::CandidateWindowMaximin,
            "single_branch_box_steepest" | "single_branch_sign_box" => {
                Self::SingleBranchBoxSteepest
            }
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
    radii: Vec<f64>,
    budget: usize,
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
    near_sigmas: Vec<Vec<usize>>,
    candidate_gradients: Vec<Vec<Vector4<f64>>>,
    candidate_sigmas: Vec<Vec<usize>>,
    candidate_gaps: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct AttemptRow {
    policy: String,
    start_id: String,
    initial_radius: f64,
    iteration: usize,
    attempt: usize,
    proposal_radius: f64,
    target_evaluations: usize,
    target_valid: bool,
    target_sys: Option<f64>,
    base_sys: f64,
    delta: Option<f64>,
    accepted: bool,
    reason: String,
    best_sys: f64,
    best_iteration: usize,
    current_radius: f64,
    direction_label: String,
    direction_norm_inf: f64,
    direction_flat: Vec<f64>,
    primary_gradient_flat: Vec<f64>,
    base_dual_flat: Vec<f64>,
    target_dual_flat: Vec<f64>,
    base_sigma: Vec<usize>,
    near_active_count: usize,
    near_active_sigmas: Vec<Vec<usize>>,
    candidate_window_count: usize,
    candidate_window_sigmas: Vec<Vec<usize>>,
    genuinely_multi_branch: bool,
    predicted_delta: Option<f64>,
    predicted_branch_values: Vec<f64>,
    predicted_winning_sigma: Option<Vec<usize>>,
    predicted_observed_error: Option<f64>,
    target_sigma: Option<Vec<usize>>,
    target_visible_near: Option<bool>,
    target_visible_candidate: Option<bool>,
}
#[derive(Debug, Serialize)]
struct TrajectorySummary {
    policy: String,
    start_id: String,
    initial_radius: f64,
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
    radius_expansions: usize,
    radius_shrinks: usize,
    stop_reason: String,
    final_radius: f64,
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
    initial_radii: Vec<f64>,
    requested_target_budget: usize,
    post_initial_target_budget: usize,
    near_active_window_relative: f64,
    candidate_window_relative_gap: f64,
    direction_contract: String,
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
        .filter(|r| cli.facet_count == 0 || r.facet_count == Some(cli.facet_count))
        .filter(|r| !cli.exclude.contains(&r.poly_id))
        .take(cli.start_count)
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
        cli.start_count,
        "frozen source selection did not produce requested starts"
    );
    let implementation =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("adaptive-direction-ablation/main.rs");
    let provenance = Provenance { command: std::env::args().collect(), source_head: git_output(&["rev-parse","HEAD"]), source_input: cli.polytope_table.display().to_string(), source_input_blake3: hash_file(&cli.polytope_table), implementation: implementation.display().to_string(), implementation_blake3: hash_file(&implementation), policies: cli.policies.iter().map(|p|p.as_str().to_string()).collect(), initial_radii: cli.radii.clone(), requested_target_budget: cli.budget, post_initial_target_budget: MAX_TARGET_EVALUATIONS, near_active_window_relative: NEAR_ACTIVE_RELATIVE_WINDOW, candidate_window_relative_gap: CANDIDATE_WINDOW_RELATIVE_GAP, direction_contract: "common L-infinity radius semantics: deterministic branch ray scaled to max_abs=1; near-active and candidate-window use box-LP x_j in [-1,1] directly without post-normalization; candidate objective min_sigma(base_gap_sigma + <grad sys_sigma, radius*direction>)".to_string(), evaluator_accounting: "initial state excluded; every target proposal increments target_evaluations; accepted iff valid and target full sys strictly increases; accepted radius expands 1.25, invalid/non-improving shrinks 0.5".to_string() };
    write_json(cli.out_dir.join("run-provenance.json"), &provenance);
    let began = Instant::now();
    let mut trajectories = Vec::new();
    let starts_to_run = &starts[..];
    let radii = if cli.smoke {
        vec![1e-3]
    } else {
        cli.radii.clone()
    };
    for policy in &cli.policies {
        for start in starts_to_run {
            for &radius in &radii {
                trajectories.push(run_trajectory(
                    *policy,
                    start,
                    radius,
                    cli.budget,
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
    initial_radius: f64,
    budget: usize,
    out_dir: &Path,
) -> TrajectorySummary {
    let dir = out_dir
        .join("trajectories")
        .join(safe_id(policy.as_str()))
        .join(safe_id(&start.id));
    fs::create_dir_all(&dir).expect("trajectory dir");
    let path = dir.join(format!("radius-{}.jsonl", eta_label(initial_radius)));
    let mut writer = BufWriter::new(File::create(&path).expect("trajectory"));
    let mut current = compute_state(&start.duals).expect("initial state failed");
    let initial = current.sys;
    let mut best = initial;
    let mut best_iteration = 0usize;
    let mut committed = 0usize;
    let mut target_evals = 0usize;
    let mut invalid = 0usize;
    let mut rejected = 0usize;
    let mut decreases = 0usize;
    let mut expands = 0usize;
    let mut shrinks = 0usize;
    let mut radius = initial_radius;
    let mut stop_reason = "budget".to_string();
    write_attempt(
        &mut writer,
        &AttemptRow::initial(policy, start, initial, radius, &current),
    );
    for iteration in 1..=budget {
        if target_evals >= MAX_TARGET_EVALUATIONS {
            stop_reason = "target_evaluation_budget".into();
            break;
        }
        let (label, direction, predicted, winner, predicted_values) =
            match direction_for(policy, &current, radius) {
                Some(x) => x,
                None => {
                    stop_reason = "direction_construction_failed".into();
                    break;
                }
            };
        let base_sigma = current.sigma.clone();
        let base_near_sigmas = current.near_sigmas.clone();
        let base_candidate_sigmas = current.candidate_sigmas.clone();
        let flat = flatten(&direction);
        let primary_gradient_flat = flatten(&current.gradient);
        let norm = direction
            .iter()
            .flat_map(|v| v.iter())
            .map(|x| x.abs())
            .fold(0.0, f64::max);
        let before = current.polytope.dual_vertices_f64.clone();
        let base_sys = current.sys;
        let proposal_radius = radius;
        let after = add_step(&before, &direction, proposal_radius);
        target_evals += 1;
        let target = compute_state(&after);
        let valid = target.is_ok();
        if !valid {
            invalid += 1;
        }
        let (target_sys, delta, target_sigma) = target
            .as_ref()
            .map(|s| {
                (
                    Some(s.sys),
                    Some(s.sys - current.sys),
                    Some(s.sigma.clone()),
                )
            })
            .unwrap_or((None, None, None));
        let accept = valid && delta.unwrap_or(f64::NEG_INFINITY) > 0.0;
        if !accept {
            rejected += 1;
            shrinks += 1;
            radius *= 0.5;
        } else {
            let next = target.unwrap();
            if next.sys < current.sys {
                decreases += 1;
            }
            current = next;
            committed += 1;
            if current.sys > best {
                best = current.sys;
                best_iteration = iteration;
            }
            expands += 1;
            radius *= 1.25;
        }
        let observed_error = predicted.zip(delta).map(|(p, d)| d - p);
        let target_sigma_ref = target_sigma.clone();
        let row = AttemptRow {
            policy: policy.as_str().into(),
            start_id: start.id.clone(),
            initial_radius,
            iteration,
            attempt: 0,
            target_evaluations: target_evals,
            target_valid: valid,
            target_sys,
            base_sys,
            delta,
            accepted: accept,
            reason: if accept {
                "accepted_radius_expand".into()
            } else if valid {
                "non_improving_radius_shrink".into()
            } else {
                "invalid_radius_shrink".into()
            },
            best_sys: best,
            best_iteration,
            proposal_radius,
            current_radius: radius,
            direction_label: label,
            direction_norm_inf: norm,
            direction_flat: flat,
            primary_gradient_flat,
            base_dual_flat: flatten(&before),
            target_dual_flat: flatten(&after),
            base_sigma: base_sigma.clone(),
            near_active_count: base_near_sigmas.len(),
            near_active_sigmas: base_near_sigmas.clone(),
            candidate_window_count: base_candidate_sigmas.len(),
            candidate_window_sigmas: base_candidate_sigmas.clone(),
            genuinely_multi_branch: base_near_sigmas.len() > 1 || base_candidate_sigmas.len() > 1,
            predicted_delta: predicted,
            predicted_branch_values: predicted_values,
            predicted_winning_sigma: winner,
            predicted_observed_error: observed_error,
            target_sigma: target_sigma_ref.clone(),
            target_visible_near: target_sigma_ref
                .as_ref()
                .map(|x| base_near_sigmas.iter().any(|s| s == x)),
            target_visible_candidate: target_sigma_ref
                .as_ref()
                .map(|x| base_candidate_sigmas.iter().any(|s| s == x)),
        };
        write_attempt(&mut writer, &row);
        if radius < 1e-12 {
            stop_reason = "shrunken_radius".into();
            break;
        }
    }
    if target_evals >= MAX_TARGET_EVALUATIONS {
        stop_reason = "target_evaluation_budget".into();
    }
    writer.flush().expect("flush trajectory");
    TrajectorySummary {
        policy: policy.as_str().into(),
        start_id: start.id.clone(),
        initial_radius,
        requested_updates: budget,
        committed_updates: committed,
        initial_sys: initial,
        final_sys: Some(current.sys),
        best_sys: best,
        best_iteration,
        target_evaluations: target_evals,
        invalid_attempts: invalid,
        rejected_attempts: rejected,
        accepted_decreases: decreases,
        radius_expansions: expands,
        radius_shrinks: shrinks,
        stop_reason,
        final_radius: radius,
    }
}

fn direction_for(
    policy: Policy,
    state: &State,
    radius: f64,
) -> Option<(
    String,
    Vec<Vector4<f64>>,
    Option<f64>,
    Option<Vec<usize>>,
    Vec<f64>,
)> {
    match policy {
        Policy::BranchGradient => {
            let d = normalize_inf(&state.gradient)?;
            let values = vec![radius * gradient_dot(&state.gradient, &d)?];
            Some((
                "inf_normalized_branch_gradient".into(),
                d,
                Some(values[0]),
                Some(state.sigma.clone()),
                values,
            ))
        }
        Policy::NearActiveMaximin => {
            let d = if state.near_gradients.len() == 1 {
                box_steepest_direction(state.near_gradients.first()?)?
            } else {
                maximin_direction(&state.near_gradients)?
            };
            let values: Vec<f64> = state
                .near_gradients
                .iter()
                .map(|g| gradient_dot(g, &d).unwrap_or(f64::INFINITY) * radius)
                .collect();
            let (i, pred) = values
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.total_cmp(b.1))?;
            Some((
                "near_active_box_lp_maximin".into(),
                d,
                Some(*pred),
                Some(state.near_sigmas.get(i)?.clone()),
                values,
            ))
        }
        Policy::CandidateWindowMaximin => {
            let (d, p, w, values) = candidate_window_direction(state, radius)?;
            Some((
                "candidate_window_box_lp_maximin".into(),
                d,
                Some(p),
                Some(w),
                values,
            ))
        }
        Policy::SingleBranchBoxSteepest => {
            let d = box_steepest_direction(&state.gradient)?;
            let values = vec![radius * gradient_dot(&state.gradient, &d)?];
            Some((
                "single_branch_box_steepest".into(),
                d,
                Some(values[0]),
                Some(state.sigma.clone()),
                values,
            ))
        }
    }
}
fn gradient_dot(g: &[Vector4<f64>], d: &[Vector4<f64>]) -> Option<f64> {
    (g.len() == d.len()).then(|| g.iter().zip(d).map(|(a, b)| a.dot(b)).sum())
}

fn write_attempt(writer: &mut BufWriter<File>, row: &AttemptRow) {
    serde_json::to_writer(&mut *writer, row).expect("serialize attempt");
    writer.write_all(b"\n").expect("write attempt");
}
impl AttemptRow {
    fn initial(policy: Policy, start: &Start, sys: f64, radius: f64, state: &State) -> Self {
        Self {
            policy: policy.as_str().into(),
            start_id: start.id.clone(),
            initial_radius: radius,
            iteration: 0,
            attempt: 0,
            target_evaluations: 0,
            target_valid: true,
            target_sys: Some(sys),
            base_sys: sys,
            delta: None,
            accepted: true,
            reason: "initial".into(),
            best_sys: sys,
            best_iteration: 0,
            proposal_radius: radius,
            current_radius: radius,
            direction_label: "initial".into(),
            direction_norm_inf: 0.0,
            direction_flat: Vec::new(),
            primary_gradient_flat: flatten(&state.gradient),
            base_dual_flat: flatten(&state.polytope.dual_vertices_f64),
            target_dual_flat: flatten(&state.polytope.dual_vertices_f64),
            base_sigma: state.sigma.clone(),
            near_active_count: state.near_sigmas.len(),
            near_active_sigmas: state.near_sigmas.clone(),
            candidate_window_count: state.candidate_sigmas.len(),
            candidate_window_sigmas: state.candidate_sigmas.clone(),
            genuinely_multi_branch: state.near_sigmas.len() > 1 || state.candidate_sigmas.len() > 1,
            predicted_delta: None,
            predicted_branch_values: Vec::new(),
            predicted_winning_sigma: None,
            predicted_observed_error: None,
            target_sigma: Some(state.sigma.clone()),
            target_visible_near: Some(true),
            target_visible_candidate: Some(true),
        }
    }
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
    let capacity0 = capacity_all_safe(&polytope, 0.0)
        .map_err(|e| format!("exact_full_capacity_failed:{e:?}"))?;
    let capacity = capacity_all_safe(
        &polytope,
        capacity0.min_action * CANDIDATE_WINDOW_RELATIVE_GAP,
    )
    .map_err(|e| format!("exact_window_capacity_failed:{e:?}"))?;
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
    let near_sigmas: Vec<Vec<usize>> = admissible
        .iter()
        .filter(|o| o.action <= cutoff)
        .map(|o| o.sigma.clone())
        .collect();
    let candidate: Vec<_> = admissible
        .iter()
        .filter(|o| o.action <= capacity.min_action * (1.0 + CANDIDATE_WINDOW_RELATIVE_GAP))
        .collect();
    let candidate_sigmas: Vec<Vec<usize>> = candidate.iter().map(|o| o.sigma.clone()).collect();
    let candidate_gaps: Vec<f64> = candidate
        .iter()
        .map(|o| sys * ((o.action / capacity.min_action).powi(2) - 1.0))
        .collect();
    let candidate_gradients = candidate
        .iter()
        .map(|o| grad_for(o))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(State {
        polytope,
        sys,
        volume,
        action: best_action,
        sigma: best_sigma,
        gradient,
        near_gradients,
        near_sigmas,
        candidate_gradients,
        candidate_sigmas,
        candidate_gaps,
    })
}
fn maximin_direction(grads: &[Vec<Vector4<f64>>]) -> Option<Vec<Vector4<f64>>> {
    if grads.len() == 1 {
        return box_steepest_direction(grads.first()?);
    }
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
    let out = unflatten(&d);
    (d.iter().all(|x| x.is_finite()) && d.iter().any(|x| x.abs() > 1e-14)).then_some(out)
}
fn candidate_window_direction(
    state: &State,
    radius: f64,
) -> Option<(Vec<Vector4<f64>>, f64, Vec<usize>, Vec<f64>)> {
    candidate_window_box_lp_direction(
        &state.candidate_gradients,
        &state.candidate_gaps,
        &state.candidate_sigmas,
        radius,
    )
}
fn candidate_window_box_lp_direction(
    candidate_gradients: &[Vec<Vector4<f64>>],
    candidate_gaps: &[f64],
    candidate_sigmas: &[Vec<usize>],
    radius: f64,
) -> Option<(Vec<Vector4<f64>>, f64, Vec<usize>, Vec<f64>)> {
    let first = candidate_gradients.first()?;
    if candidate_gradients.len() == 1 {
        let d = box_steepest_direction(first)?;
        let values = vec![candidate_gaps.first()? + radius * gradient_dot(first, &d)?];
        return Some((d, values[0], candidate_sigmas.first()?.clone(), values));
    }
    let dim = first.len() * 4;
    let mut vars = variables!();
    let xs: Vec<_> = (0..dim)
        .map(|_| vars.add(variable().min(-1.).max(1.)))
        .collect::<Vec<_>>();
    let t = vars.add(variable().min(f64::NEG_INFINITY));
    let mut model = vars.maximise(Expression::from(t)).using(default_solver);
    for (g, gap) in candidate_gradients.iter().zip(candidate_gaps) {
        let mut lhs = Expression::from(*gap);
        for (c, x) in flatten(g).iter().zip(&xs) {
            lhs += radius * (*c) * (*x);
        }
        model = model.with(constraint!(lhs >= t));
    }
    let sol = model.solve().ok()?;
    let raw: Vec<f64> = xs.iter().map(|x| sol.value(*x)).collect();
    let d = unflatten(&raw);
    if !raw.iter().all(|x| x.is_finite()) || !raw.iter().any(|x| x.abs() > 1e-14) {
        return None;
    }
    let values: Vec<f64> = candidate_gradients
        .iter()
        .zip(candidate_gaps)
        .map(|(g, gap)| *gap + radius * gradient_dot(g, &d).unwrap_or(f64::INFINITY))
        .collect();
    let (i, p) = values
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))?;
    Some((d, *p, candidate_sigmas.get(i)?.clone(), values))
}
fn normalize_inf(v: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let n = v
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x.abs())
        .fold(0.0, f64::max);
    (n.is_finite() && n > 1e-14).then(|| v.iter().map(|x| *x / n).collect())
}
fn box_steepest_direction(v: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let out: Vec<_> = v
        .iter()
        .map(|x| {
            Vector4::new(
                if x[0] > 0.0 {
                    1.0
                } else if x[0] < 0.0 {
                    -1.0
                } else {
                    0.0
                },
                if x[1] > 0.0 {
                    1.0
                } else if x[1] < 0.0 {
                    -1.0
                } else {
                    0.0
                },
                if x[2] > 0.0 {
                    1.0
                } else if x[2] < 0.0 {
                    -1.0
                } else {
                    0.0
                },
                if x[3] > 0.0 {
                    1.0
                } else if x[3] < 0.0 {
                    -1.0
                } else {
                    0.0
                },
            )
        })
        .collect();
    let flat = flatten(&out);
    (flatten(v).iter().all(|x| x.is_finite()) && flat.iter().any(|x| x.abs() > 0.0)).then_some(out)
}
fn flatten(v: &[Vector4<f64>]) -> Vec<f64> {
    v.iter().flat_map(|x| [x[0], x[1], x[2], x[3]]).collect()
}
fn unflatten(v: &[f64]) -> Vec<Vector4<f64>> {
    v.chunks_exact(4)
        .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
        .collect()
}
fn capacity_all_safe(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
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
            action_gap,
            OrbitGuaranteeMode::AllSafe,
        )
    } else {
        let tr=symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(&polytope.facet_intersection_is_nonempty,&polytope.omega_signs);
        let (o, i) = solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &tr)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            o,
            i,
            action_gap,
            OrbitGuaranteeMode::AllSafe,
        )
    }
}
fn parse_args() -> Cli {
    let mut table = PathBuf::from("experiments/sys-datascience/produce/random.jsonl");
    let mut out =
        PathBuf::from("experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts");
    let mut facet = 6;
    let mut starts = 6;
    let mut exclude = vec!["random_F6_s0_1".to_string()];
    let mut policies = vec![
        Policy::BranchGradient,
        Policy::NearActiveMaximin,
        Policy::CandidateWindowMaximin,
        Policy::SingleBranchBoxSteepest,
    ];
    let mut radii = vec![1e-4, 1e-3, 1e-2];
    let mut budget = MAX_TARGET_EVALUATIONS;
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
            "--radii" => {
                radii = a
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            }
            "--budget" => budget = a.next().unwrap().parse().unwrap(),
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
        radii,
        budget,
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

#[cfg(test)]
mod tests {
    use nalgebra::Vector4;

    #[test]
    fn inf_branch_normalization_has_common_radius_semantics() {
        let gradient = vec![Vector4::new(2.0, -1.0, 0.5, 0.0)];
        let direction = super::normalize_inf(&gradient).unwrap();
        assert!(
            (direction
                .iter()
                .flat_map(|x| x.iter())
                .map(|x| x.abs())
                .fold(0.0, f64::max)
                - 1.0)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn candidate_prediction_uses_executed_box_solution_not_euclidean_rescale() {
        let gaps = vec![0.0, 0.25];
        let gradients = vec![
            vec![Vector4::new(-1.0, -1.0, 0.0, 0.0)],
            vec![Vector4::new(1.0, -1.0, 0.0, 0.0)],
        ];
        let sigmas = vec![vec![0], vec![1]];
        let (direction, executed, _, values) =
            super::candidate_window_box_lp_direction(&gradients, &gaps, &sigmas, 1.0).unwrap();
        assert!(direction
            .iter()
            .flat_map(|x| x.iter())
            .all(|x| x.abs() <= 1.0 + 1e-8));
        assert!(direction
            .iter()
            .flat_map(|x| x.iter())
            .any(|x| x.abs() > 1e-8));
        assert!((executed - values.iter().copied().fold(f64::INFINITY, f64::min)).abs() < 1e-12);
    }

    #[test]
    fn single_branch_box_steepest_is_coordinatewise_sign_with_zero_handling() {
        let g = vec![Vector4::new(2.0, -3.0, 0.0, 4.0)];
        let d = super::box_steepest_direction(&g).unwrap();
        assert_eq!(d[0], Vector4::new(1.0, -1.0, 0.0, 1.0));
        assert!((super::gradient_dot(&g, &d).unwrap() - 9.0).abs() < 1e-12);
    }

    #[test]
    fn singleton_maximin_and_candidate_use_the_same_sign_control() {
        let g = vec![Vector4::new(2.0, -3.0, 0.0, 4.0)];
        let sign = super::box_steepest_direction(&g).unwrap();
        let maximin = super::maximin_direction(std::slice::from_ref(&g)).unwrap();
        let (candidate, objective, _, values) = super::candidate_window_box_lp_direction(
            std::slice::from_ref(&g),
            &[0.25],
            &[vec![7]],
            0.5,
        )
        .unwrap();
        assert_eq!(sign, maximin);
        assert_eq!(sign, candidate);
        assert!((objective - (0.25 + 0.5 * 9.0)).abs() < 1e-12);
        assert_eq!(values.len(), 1);
    }
}
