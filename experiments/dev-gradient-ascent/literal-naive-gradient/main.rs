//! Literal unconditional branch-gradient trajectories.
//!
//! This binary intentionally implements only
//! `a <- a + eta * grad_a sys_sigma(a)`.  It does not normalize, project,
//! select a near-active set, accept/reject, or line-search the update.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use symplectic::derivatives::{
    capacity_derivatives_a_from_orbit, systolic_ratio_gradient_a, volume_derivatives_a,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_ETAS: &[f64] = &[1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0];
const DEFAULT_UPDATES: usize = 100;

#[derive(Debug)]
struct Cli {
    polytope_table: PathBuf,
    poly_id: String,
    out_dir: PathBuf,
    etas: Vec<f64>,
    updates: usize,
}

#[derive(Debug, Deserialize)]
struct PanelRow {
    poly_id: String,
    #[allow(dead_code)]
    facet_count: Option<usize>,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Debug)]
struct State {
    polytope: SysLandscapePolytopeCache,
    sys: f64,
    volume: f64,
    action: f64,
    sigma: Vec<usize>,
    gradient: Vec<Vector4<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrajectoryRow {
    eta: f64,
    iteration: usize,
    role: String,
    state_valid: bool,
    failure: Option<String>,
    sys: Option<f64>,
    full_sys_delta: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    selected_sigma: Option<Vec<usize>>,
    selected_action: Option<f64>,
    resulting_sigma: Option<Vec<usize>>,
    gradient: Vec<[f64; 4]>,
    gradient_norm: Option<f64>,
    da: Vec<[f64; 4]>,
    step_norm: Option<f64>,
    dual_vertices_before: Vec<[f64; 4]>,
    dual_vertices_after: Vec<[f64; 4]>,
}

#[derive(Debug, Serialize)]
struct TrajectorySummary {
    eta: f64,
    requested_updates: usize,
    iterations_completed: usize,
    initial_sys: f64,
    final_sys: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    gain_through_iteration_20: Option<f64>,
    additional_best_gain_iterations_21_100: f64,
    full_sys_increases: usize,
    full_sys_decreases: usize,
    full_sys_equal: usize,
    branch_switches: usize,
    failure: Option<String>,
    trajectory_path: String,
}

#[derive(Debug, Serialize)]
struct RunProvenance {
    command: Vec<String>,
    source_repo_head: Option<String>,
    source_worktree_diff_blake3: String,
    implementation_path: String,
    implementation_blake3: String,
    input_path: String,
    input_blake3: String,
    poly_id: String,
    etas: Vec<f64>,
    updates: usize,
    update_rule: String,
    state_validity_rule: String,
}

#[derive(Debug, Serialize)]
struct RunSummary {
    poly_id: String,
    source_repo_head: Option<String>,
    etas: Vec<f64>,
    updates: usize,
    trajectories: Vec<TrajectorySummary>,
}

fn main() {
    let cli = parse_args(std::env::args().skip(1));
    fs::create_dir_all(&cli.out_dir).expect("create output directory");
    let rows: Vec<PanelRow> = load_jsonl(&cli.polytope_table);
    let row = rows
        .into_iter()
        .find(|row| row.poly_id == cli.poly_id)
        .unwrap_or_else(|| {
            panic!(
                "poly_id {} not found in {}",
                cli.poly_id,
                cli.polytope_table.display()
            )
        });
    let initial_duals: Vec<Vector4<f64>> = row
        .dual_vertices_f64
        .iter()
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect();

    let implementation_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("literal-naive-gradient/main.rs");
    let provenance = RunProvenance {
        command: std::env::args().collect(),
        source_repo_head: git_output(&["rev-parse", "HEAD"]),
        source_worktree_diff_blake3: git_diff_hash(),
        implementation_path: implementation_path.display().to_string(),
        implementation_blake3: hash_file(&implementation_path),
        input_path: cli.polytope_table.display().to_string(),
        input_blake3: hash_file(&cli.polytope_table),
        poly_id: cli.poly_id.clone(),
        etas: cli.etas.clone(),
        updates: cli.updates,
        update_rule: "da = eta * grad_a sys_sigma(a); a = a + da".to_string(),
        state_validity_rule: "Stop only when constructing the updated exact-state polytope or its exact full sys/branch gradient fails; no projection, gauge normalization, acceptance, or line search".to_string(),
    };
    write_json(cli.out_dir.join("run-provenance.json"), &provenance);

    let mut summaries = Vec::new();
    for &eta in &cli.etas {
        let summary = run_trajectory(&initial_duals, eta, cli.updates, &cli.out_dir);
        summaries.push(summary);
    }
    let run_summary = RunSummary {
        poly_id: cli.poly_id,
        source_repo_head: provenance.source_repo_head,
        etas: cli.etas,
        updates: cli.updates,
        trajectories: summaries,
    };
    write_json(cli.out_dir.join("summary.json"), &run_summary);
}

fn run_trajectory(
    initial_duals: &[Vector4<f64>],
    eta: f64,
    updates: usize,
    out_dir: &Path,
) -> TrajectorySummary {
    let filename = format!("trajectory-eta-{}.jsonl", eta_label(eta));
    let path = out_dir.join(filename);
    let file = File::create(&path).expect("create trajectory");
    let mut writer = BufWriter::new(file);
    let initial =
        compute_state(initial_duals).unwrap_or_else(|err| panic!("initial state failed: {err}"));
    let initial_sys = initial.sys;
    let mut best_sys = initial.sys;
    let mut best_iteration = 0;
    let mut current = initial;
    let mut previous_sigma = current.sigma.clone();
    let mut increases = 0;
    let mut decreases = 0;
    let mut equal = 0;
    let mut branch_switches = 0;
    write_row(
        &mut writer,
        &make_row(
            eta,
            0,
            "initial",
            &current,
            None,
            best_sys,
            best_iteration,
            vec![Vector4::zeros(); current.polytope.facet_count()],
            true,
            None,
        ),
    );

    let mut failure = None;
    let mut completed = 0usize;
    for iteration in 1..=updates {
        let gradient = current.gradient.clone();
        let da: Vec<Vector4<f64>> = gradient.iter().map(|g| eta * g).collect();
        let before_duals = current.polytope.dual_vertices_f64.clone();
        let after_duals: Vec<Vector4<f64>> =
            before_duals.iter().zip(&da).map(|(a, d)| a + d).collect();
        match compute_state(&after_duals) {
            Ok(next) => {
                let delta = next.sys - current.sys;
                if delta > 0.0 {
                    increases += 1;
                } else if delta < 0.0 {
                    decreases += 1;
                } else {
                    equal += 1;
                }
                if next.sigma != previous_sigma {
                    branch_switches += 1;
                }
                previous_sigma = next.sigma.clone();
                if next.sys > best_sys {
                    best_sys = next.sys;
                    best_iteration = iteration;
                }
                let row = make_update_row(
                    eta,
                    iteration,
                    &current,
                    &next,
                    Some(delta),
                    best_sys,
                    best_iteration,
                    da,
                    (before_duals, after_duals),
                );
                write_row(&mut writer, &row);
                current = next;
                completed = iteration;
            }
            Err(err) => {
                let gradient_norm = l2_norm(&gradient);
                let step_norm = eta * gradient_norm;
                let row = TrajectoryRow {
                    eta,
                    iteration,
                    role: "failure".to_string(),
                    state_valid: false,
                    failure: Some(err.clone()),
                    sys: None,
                    full_sys_delta: None,
                    best_sys,
                    best_iteration,
                    selected_sigma: Some(current.sigma.clone()),
                    selected_action: Some(current.action),
                    resulting_sigma: None,
                    gradient: vectors_to_arrays(&gradient),
                    gradient_norm: Some(gradient_norm),
                    da: vectors_to_arrays(&da),
                    step_norm: Some(step_norm),
                    dual_vertices_before: vectors_to_arrays(&before_duals),
                    dual_vertices_after: vectors_to_arrays(&after_duals),
                };
                write_row(&mut writer, &row);
                failure = Some(err);
                break;
            }
        }
    }
    writer.flush().expect("flush trajectory");
    TrajectorySummary {
        eta,
        requested_updates: updates,
        iterations_completed: completed,
        initial_sys,
        final_sys: (completed > 0).then_some(current.sys).or(Some(initial_sys)),
        best_sys,
        best_iteration,
        gain_through_iteration_20: if completed >= 20 {
            Some(best_sys_through(&path, 20).unwrap_or(initial_sys) - initial_sys)
        } else {
            None
        },
        additional_best_gain_iterations_21_100: if completed >= 20 {
            best_sys - best_sys_through(&path, 20).unwrap_or(initial_sys)
        } else {
            0.0
        },
        full_sys_increases: increases,
        full_sys_decreases: decreases,
        full_sys_equal: equal,
        branch_switches,
        failure,
        trajectory_path: path.display().to_string(),
    }
}

fn compute_state(duals: &[Vector4<f64>]) -> Result<State, String> {
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(duals.to_vec())
        .ok_or_else(|| "updated_state_invalid_geometry".to_string())?;
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if !volume.is_finite() || volume <= 0.0 {
        return Err("exact_volume_failed".to_string());
    }
    let capacity = capacity_all_safe(&polytope)
        .map_err(|err| format!("exact_full_capacity_failed:{err:?}"))?;
    let sys = symplectic::systolic_ratio(capacity.min_action, volume);
    if !sys.is_finite() {
        return Err("exact_full_sys_computation_failed".to_string());
    }
    let orbit = capacity
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .min_by(|a, b| {
            a.action
                .total_cmp(&b.action)
                .then_with(|| a.sigma.cmp(&b.sigma))
        })
        .ok_or_else(|| "no_admissible_minimizing_sigma_branch".to_string())?;
    let d_volume = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|err| format!("volume_derivative_failed:{err:?}"))?;
    let d_capacity = capacity_derivatives_a_from_orbit(&polytope.dual_vertices_f64, orbit)
        .map_err(|err| format!("branch_derivative_failed:{err:?}"))?;
    let gradient = systolic_ratio_gradient_a(orbit.action, volume, &d_capacity, &d_volume);
    if !gradient.iter().all(|g| g.iter().all(|x| x.is_finite())) {
        return Err("nonfinite_branch_gradient".to_string());
    }
    Ok(State {
        polytope,
        sys,
        volume,
        action: orbit.action,
        sigma: orbit.sigma.clone(),
        gradient,
    })
}

fn capacity_all_safe(
    polytope: &SysLandscapePolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if let Ok(classification) = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
        let transition_is_allowed =
            symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            );
        let (orbits, iterations) = solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &transition_is_allowed,
        )
        .map_err(|_| OrbitSearchError::NumericalFailure)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            0.0,
            OrbitGuaranteeMode::AllSafe,
        )
    } else {
        let transition_is_allowed =
            symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
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
            OrbitGuaranteeMode::AllSafe,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn make_row(
    eta: f64,
    iteration: usize,
    role: &str,
    state: &State,
    delta: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    da: Vec<Vector4<f64>>,
    state_valid: bool,
    duals: Option<(Vec<Vector4<f64>>, Vec<Vector4<f64>>)>,
) -> TrajectoryRow {
    let gradient_norm = l2_norm(&state.gradient);
    let before = duals
        .as_ref()
        .map(|(before, _)| vectors_to_arrays(before))
        .unwrap_or_else(|| vectors_to_arrays(&state.polytope.dual_vertices_f64));
    let after = duals
        .as_ref()
        .map(|(_, after)| vectors_to_arrays(after))
        .unwrap_or_else(|| vectors_to_arrays(&state.polytope.dual_vertices_f64));
    TrajectoryRow {
        eta,
        iteration,
        role: role.to_string(),
        state_valid,
        failure: None,
        sys: Some(state.sys),
        full_sys_delta: delta,
        best_sys,
        best_iteration,
        selected_sigma: Some(state.sigma.clone()),
        selected_action: Some(state.action),
        resulting_sigma: Some(state.sigma.clone()),
        gradient: vectors_to_arrays(&state.gradient),
        gradient_norm: Some(gradient_norm),
        da: vectors_to_arrays(&da),
        step_norm: Some(l2_norm(&da)),
        dual_vertices_before: before,
        dual_vertices_after: after,
    }
}

#[allow(clippy::too_many_arguments)]
fn make_update_row(
    eta: f64,
    iteration: usize,
    base: &State,
    next: &State,
    delta: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    da: Vec<Vector4<f64>>,
    duals: (Vec<Vector4<f64>>, Vec<Vector4<f64>>),
) -> TrajectoryRow {
    let gradient_norm = l2_norm(&base.gradient);
    TrajectoryRow {
        eta,
        iteration,
        role: "update".to_string(),
        state_valid: true,
        failure: None,
        sys: Some(next.sys),
        full_sys_delta: delta,
        best_sys,
        best_iteration,
        selected_sigma: Some(base.sigma.clone()),
        selected_action: Some(base.action),
        resulting_sigma: Some(next.sigma.clone()),
        gradient: vectors_to_arrays(&base.gradient),
        gradient_norm: Some(gradient_norm),
        da: vectors_to_arrays(&da),
        step_norm: Some(l2_norm(&da)),
        dual_vertices_before: vectors_to_arrays(&duals.0),
        dual_vertices_after: vectors_to_arrays(&duals.1),
    }
}

fn best_sys_through(path: &Path, iteration_limit: usize) -> Option<f64> {
    let file = File::open(path).ok()?;
    let mut best = f64::NEG_INFINITY;
    for line in BufReader::new(file).lines() {
        let row: TrajectoryRow = serde_json::from_str(&line.ok()?).ok()?;
        if row.iteration <= iteration_limit {
            if let Some(sys) = row.sys {
                best = best.max(sys);
            }
        }
    }
    best.is_finite().then_some(best)
}

fn vectors_to_arrays(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn l2_norm(vectors: &[Vector4<f64>]) -> f64 {
    vectors.iter().map(|v| v.dot(v)).sum::<f64>().sqrt()
}

fn write_row<T: Serialize>(writer: &mut BufWriter<File>, row: &T) {
    serde_json::to_writer(&mut *writer, row).expect("serialize trajectory row");
    writer.write_all(b"\n").expect("write trajectory row");
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(line, text)| {
            serde_json::from_str(
                &text.unwrap_or_else(|err| {
                    panic!("read {} line {}: {err}", path.display(), line + 1)
                }),
            )
            .unwrap_or_else(|err| panic!("parse {} line {}: {err}", path.display(), line + 1))
        })
        .collect()
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    let file = File::create(&path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    serde_json::to_writer_pretty(file, value).expect("write json");
}

fn hash_file(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|err| panic!("hash {}: {err}", path.display()));
    blake3::hash(&bytes).to_hex().to_string()
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_diff_hash() -> String {
    let output = Command::new("git")
        .args(["diff", "--binary"])
        .output()
        .expect("git diff");
    blake3::hash(&output.stdout).to_hex().to_string()
}

fn eta_label(eta: f64) -> String {
    match eta {
        1e-5 => "1e-5".to_string(),
        1e-4 => "1e-4".to_string(),
        1e-3 => "1e-3".to_string(),
        1e-2 => "1e-2".to_string(),
        1e-1 => "1e-1".to_string(),
        1.0 => "1".to_string(),
        _ => format!("{eta:.12e}").replace('+', ""),
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Cli {
    let mut polytope_table = PathBuf::from(
        "experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl",
    );
    let mut poly_id =
        "3daddfde522cb04777d651814d7f88a31f6ec20c1b7ac8fc960efc3e4534104e".to_string();
    let mut out_dir =
        PathBuf::from("/tmp/sys-ds-research-lines/optimizer/literal-naive-gradient-luna-high");
    let mut etas = DEFAULT_ETAS.to_vec();
    let mut updates = DEFAULT_UPDATES;
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--polytope-table" => {
                polytope_table = PathBuf::from(args.next().expect("--polytope-table requires path"))
            }
            "--poly-id" => poly_id = args.next().expect("--poly-id requires value"),
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("--out-dir requires path")),
            "--etas" => {
                etas = args
                    .next()
                    .expect("--etas requires csv")
                    .split(',')
                    .map(|x| x.parse().expect("invalid eta"))
                    .collect();
            }
            "--updates" => {
                updates = args
                    .next()
                    .expect("--updates requires integer")
                    .parse()
                    .expect("invalid updates")
            }
            "--help" => {
                println!("Usage: dev-gradient-ascent-literal-naive-gradient [--polytope-table PATH] [--poly-id ID] [--out-dir PATH] [--etas CSV] [--updates N]");
                std::process::exit(0);
            }
            other => panic!("unsupported argument {other}"),
        }
    }
    assert!(!etas.is_empty(), "at least one eta is required");
    assert!(
        etas.iter().all(|eta| eta.is_finite() && *eta > 0.0),
        "etas must be positive finite"
    );
    Cli {
        polytope_table,
        poly_id,
        out_dir,
        etas,
        updates,
    }
}
