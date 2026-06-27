//! Fixed-sigma `sysext_sigma` line probe.
//!
//! This command evaluates raw KKT critical points for already chosen sigmas
//! along a finite line `a0 + t u`. It is deliberately separate from full `sys`
//! recomputation: the goal is to inspect individual branch action and beta
//! margin behavior without enumerating all candidate branches at every sample.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{DVector, Vector4};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::derivatives::{
    capacity_subgradients_a, systolic_ratio_gradient_a, volume_derivatives_a,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitKktData, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_SELECTION_THRESHOLD_RELATIVE: f64 = 1.0e-2;
const DEFAULT_ACTION_WINDOW_RELATIVE: f64 = 1.0e-2;
const DEFAULT_STEPS: &[f64] = &[-1.0e-3, -3.0e-4, -1.0e-4, 0.0, 1.0e-4, 3.0e-4, 1.0e-3];
const EPS_EIGEN_FLOOR: f64 = 1.0e-12;
const EPS_KKT_RESIDUAL: f64 = 1.0e-6;
const EPS_Q_POSITIVE: f64 = 1.0e-15;

#[derive(Debug)]
struct Cli {
    diagnostic_dir: PathBuf,
    polytope_table: PathBuf,
    out_dir: PathBuf,
    selection_threshold_relative: f64,
    action_window_relative: f64,
    degeneracy_label: String,
    steps: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct DiagnosticRow {
    poly_id: String,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    threshold_relative: f64,
    near_active_count: Option<usize>,
    degeneracy_label: String,
    failure: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    capacity: f64,
    sys: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Clone, Debug)]
struct Fixture {
    diagnostic: DiagnosticRow,
    polytope: PolytopeRow,
    selection_rank_within_label: usize,
}

#[derive(Clone, Debug)]
struct BaseState {
    polytope: SysLandscapePolytopeCache,
    capacity: OrbitSearchResult,
    sys: f64,
    near_active_orbits: Vec<OrbitKktData>,
    sys_gradients: Vec<Vec<Vector4<f64>>>,
}

#[derive(Clone, Debug)]
struct RawKktResult {
    beta: Vec<f64>,
    mu: Vec<f64>,
    q_corrected: f64,
    residual_norm: f64,
    n_positive: usize,
    n_negative: usize,
    n_zero: usize,
}

#[derive(Serialize)]
struct FixtureRow {
    poly_id: String,
    degeneracy_label: String,
    selection_rank_within_label: usize,
    threshold_relative: f64,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    input_near_active_count: usize,
}

#[derive(Serialize)]
struct LineProbeRow {
    poly_id: String,
    degeneracy_label: String,
    direction_label: String,
    sigma_source: String,
    sigma: Vec<usize>,
    t: f64,
    status: String,
    action: Option<f64>,
    beta_margin: Option<f64>,
    beta_positive: Option<bool>,
    q_corrected: Option<f64>,
    residual_norm: Option<f64>,
    n_positive: Option<usize>,
    n_negative: Option<usize>,
    n_zero: Option<usize>,
    mu: Option<[f64; 4]>,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    selected_fixture: usize,
    rows: usize,
    status_counts: BTreeMap<String, usize>,
    out_dir: String,
    elapsed_ms: f64,
    caveat: String,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");
    let t0 = Instant::now();

    let diagnostic_rows: Vec<DiagnosticRow> =
        load_jsonl(&cli.diagnostic_dir.join("branch-set-diagnostic.jsonl"));
    let polytope_rows: Vec<PolytopeRow> = load_jsonl(&cli.polytope_table);
    let polytope_by_id: HashMap<String, PolytopeRow> = polytope_rows
        .into_iter()
        .map(|row| (row.poly_id.clone(), row))
        .collect();
    let fixture = select_fixture(
        &diagnostic_rows,
        &polytope_by_id,
        cli.selection_threshold_relative,
        &cli.degeneracy_label,
    )
    .expect("no matching fixture selected");
    let fixture_rows = vec![fixture_row(&fixture)];

    let base = compute_base_state_from_row(
        &fixture.polytope,
        cli.action_window_relative,
        cli.selection_threshold_relative,
    )
    .expect("failed to compute base state");
    let direction = normalize_direction(
        base.sys_gradients
            .first()
            .expect("base state has no near-active gradient"),
    )
    .expect("failed to normalize first near-active gradient");
    let sigmas = sigmas_to_probe(&base);

    let mut rows = Vec::new();
    for (source, sigma) in sigmas {
        for &t in &cli.steps {
            rows.push(line_probe_row(
                &fixture,
                &base,
                "first_near_active_gradient",
                &source,
                &sigma,
                &direction,
                t,
            ));
        }
    }

    write_jsonl(cli.out_dir.join("fixture-selection.jsonl"), &fixture_rows)
        .expect("failed to write fixture-selection.jsonl");
    write_jsonl(cli.out_dir.join("sysext-sigma-line-probe.jsonl"), &rows)
        .expect("failed to write sysext-sigma-line-probe.jsonl");
    let summary = Summary {
        method: "dev-sysext-sigma-line-probe".to_string(),
        selected_fixture: 1,
        rows: rows.len(),
        status_counts: count_statuses(&rows),
        out_dir: cli.out_dir.display().to_string(),
        elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
        caveat: "fixed-sigma raw KKT probe only; it does not recompute sys or enumerate target candidate sets".to_string(),
    };
    write_json(cli.out_dir.join("summary.json"), &summary).expect("failed to write summary.json");

    println!("{}", cli.out_dir.display());
}

fn sigmas_to_probe(base: &BaseState) -> Vec<(String, Vec<usize>)> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let best = base.capacity.best_sigma().to_vec();
    seen.insert(best.clone());
    out.push(("base_best".to_string(), best));
    for (idx, orbit) in base.near_active_orbits.iter().enumerate() {
        if seen.insert(orbit.sigma.clone()) {
            out.push((format!("near_active_{idx}"), orbit.sigma.clone()));
        }
    }
    out
}

fn line_probe_row(
    fixture: &Fixture,
    base: &BaseState,
    direction_label: &str,
    sigma_source: &str,
    sigma: &[usize],
    direction: &[Vector4<f64>],
    t: f64,
) -> LineProbeRow {
    let target_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(direction)
        .map(|(dual, delta)| dual + t * delta)
        .collect();
    match solve_raw_sysext_kkt_for_dual_vertices(&target_duals, sigma) {
        Ok(raw) => {
            let action = 0.5 / raw.q_corrected;
            let beta_margin = raw.beta.iter().copied().fold(f64::INFINITY, f64::min);
            let mu = <[f64; 4]>::try_from(raw.mu.as_slice()).ok();
            LineProbeRow {
                poly_id: fixture.polytope.poly_id.clone(),
                degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
                direction_label: direction_label.to_string(),
                sigma_source: sigma_source.to_string(),
                sigma: sigma.to_vec(),
                t,
                status: "ok".to_string(),
                action: Some(action),
                beta_margin: Some(beta_margin),
                beta_positive: Some(beta_margin > 0.0),
                q_corrected: Some(raw.q_corrected),
                residual_norm: Some(raw.residual_norm),
                n_positive: Some(raw.n_positive),
                n_negative: Some(raw.n_negative),
                n_zero: Some(raw.n_zero),
                mu,
            }
        }
        Err(status) => LineProbeRow {
            poly_id: fixture.polytope.poly_id.clone(),
            degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
            direction_label: direction_label.to_string(),
            sigma_source: sigma_source.to_string(),
            sigma: sigma.to_vec(),
            t,
            status,
            action: None,
            beta_margin: None,
            beta_positive: None,
            q_corrected: None,
            residual_norm: None,
            n_positive: None,
            n_negative: None,
            n_zero: None,
            mu: None,
        },
    }
}

fn solve_raw_sysext_kkt_for_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<RawKktResult, String> {
    let (kkt, rhs) = symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices(
        dual_vertices,
        sigma,
    );
    let m = rhs.len() - 5;
    let size = rhs.len();
    let eig = kkt.clone().symmetric_eigen();
    let max_abs_ev = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    if max_abs_ev < EPS_EIGEN_FLOOR {
        return Err("singular_matrix".to_string());
    }

    let mut x0 = DVector::zeros(size);
    for i in 0..size {
        if eig.eigenvalues[i].abs() > EPS_EIGEN_FLOOR {
            let coeff = eig.eigenvectors.column(i).dot(&rhs) / eig.eigenvalues[i];
            for j in 0..size {
                x0[j] += coeff * eig.eigenvectors[(j, i)];
            }
        }
    }
    let residual = &kkt * &x0 - rhs;
    let residual_norm = residual.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return Err("residual_too_large".to_string());
    }
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual[i] * x0[i]).sum();
    let q_correction = r2_dot_mu + residual[m + 4] * x0[m + 4];
    let beta: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    let mu: Vec<f64> = (m..m + 4).map(|i| x0[i]).collect();
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;
    let q_corrected = q_raw + q_correction;
    if q_corrected <= EPS_Q_POSITIVE {
        return Err("nonpositive_q".to_string());
    }
    let strict_threshold = max_abs_ev * 1.0e-3;
    let n_positive = eig
        .eigenvalues
        .iter()
        .filter(|&&e| e > strict_threshold)
        .count();
    let n_negative = eig
        .eigenvalues
        .iter()
        .filter(|&&e| e < -strict_threshold)
        .count();
    let n_zero = size - n_positive - n_negative;
    Ok(RawKktResult {
        beta,
        mu,
        q_corrected,
        residual_norm,
        n_positive,
        n_negative,
        n_zero,
    })
}

fn compute_base_state_from_row(
    row: &PolytopeRow,
    action_window_relative: f64,
    branch_threshold_relative: f64,
) -> Result<BaseState, String> {
    let polytope = polytope_from_row(row)?;
    let capacity = capacity_auto_with_gap(&polytope, row.capacity * action_window_relative)
        .map_err(|err| format!("base_capacity_failed:{err:?}"))?;
    let vol =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if !vol.is_finite() || vol <= 0.0 {
        return Err("base_volume_failed".to_string());
    }
    let sys = symplectic::systolic_ratio(capacity.min_action, vol);
    let near_active_orbits = near_active_orbits(&capacity, branch_threshold_relative);
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|err| format!("volume_derivative_failed:{err:?}"))?;
    let d_capacity_da = capacity_subgradients_a(&polytope.dual_vertices_f64, &near_active_orbits)
        .map_err(|err| format!("capacity_derivative_failed:{err:?}"))?;
    let sys_gradients = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(capacity.min_action, vol, capacity_gradient, &d_volume_da)
        })
        .collect();
    Ok(BaseState {
        polytope,
        capacity,
        sys,
        near_active_orbits,
        sys_gradients,
    })
}

fn near_active_orbits(result: &OrbitSearchResult, threshold_relative: f64) -> Vec<OrbitKktData> {
    let cutoff = result.min_action * (1.0 + threshold_relative.max(0.0));
    let mut active: Vec<OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| orbit.action <= cutoff)
        .cloned()
        .collect();
    if active.is_empty() {
        active.push(result.best_orbit().clone());
    }
    active
}

fn capacity_auto_with_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
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
        )?;
        return aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap.max(0.0),
            OrbitGuaranteeMode::AllSafe,
        );
    }

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
        action_gap.max(0.0),
        OrbitGuaranteeMode::AllSafe,
    )
}

fn select_fixture(
    diagnostic_rows: &[DiagnosticRow],
    polytopes: &HashMap<String, PolytopeRow>,
    threshold: f64,
    degeneracy_label: &str,
) -> Option<Fixture> {
    let mut rows: Vec<&DiagnosticRow> = diagnostic_rows
        .iter()
        .filter(|row| row.failure.is_none())
        .filter(|row| row.degeneracy_label == degeneracy_label)
        .filter(|row| (row.threshold_relative - threshold).abs() <= 1.0e-15)
        .collect();
    rows.sort_by(|a, b| {
        b.input_sys
            .total_cmp(&a.input_sys)
            .then_with(|| a.poly_id.cmp(&b.poly_id))
    });
    rows.into_iter().enumerate().find_map(|(rank, row)| {
        polytopes.get(&row.poly_id).map(|polytope| Fixture {
            diagnostic: row.clone(),
            polytope: polytope.clone(),
            selection_rank_within_label: rank,
        })
    })
}

fn polytope_from_row(row: &PolytopeRow) -> Result<SysLandscapePolytopeCache, String> {
    let dual_vertices: Vec<Vector4<f64>> = row
        .dual_vertices_f64
        .iter()
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect();
    SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
        .ok_or_else(|| "failed_to_construct_polytope".to_string())
}

fn normalize_direction(direction: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let norm = direction
        .iter()
        .flat_map(|v| [v[0], v[1], v[2], v[3]])
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();
    (norm > 0.0 && norm.is_finite()).then(|| direction.iter().map(|v| v / norm).collect())
}

fn fixture_row(fixture: &Fixture) -> FixtureRow {
    FixtureRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        selection_rank_within_label: fixture.selection_rank_within_label,
        threshold_relative: fixture.diagnostic.threshold_relative,
        selection_buckets: fixture.diagnostic.selection_buckets.clone(),
        datasets: fixture.diagnostic.datasets.clone(),
        input_facet_count: fixture.diagnostic.input_facet_count,
        input_sys: fixture.diagnostic.input_sys,
        input_near_active_count: fixture.diagnostic.near_active_count.unwrap_or(0),
    }
}

fn count_statuses(rows: &[LineProbeRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.status.clone()).or_insert(0) += 1;
    }
    counts
}

fn parse_args() -> Cli {
    let mut cli = Cli {
        diagnostic_dir: PathBuf::new(),
        polytope_table: PathBuf::from("experiments/sys-datascience/prepare/polytope-table.jsonl"),
        out_dir: default_output_dir(),
        selection_threshold_relative: DEFAULT_SELECTION_THRESHOLD_RELATIVE,
        action_window_relative: DEFAULT_ACTION_WINDOW_RELATIVE,
        degeneracy_label: "high_degeneracy".to_string(),
        steps: DEFAULT_STEPS.to_vec(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--diagnostic-dir" => {
                cli.diagnostic_dir =
                    PathBuf::from(args.next().expect("--diagnostic-dir requires a path"));
            }
            "--polytope-table" => {
                cli.polytope_table =
                    PathBuf::from(args.next().expect("--polytope-table requires a path"));
            }
            "--out-dir" => {
                cli.out_dir = PathBuf::from(args.next().expect("--out-dir requires a path"));
            }
            "--selection-threshold-relative" => {
                cli.selection_threshold_relative = parse_f64_arg(&mut args, &arg);
            }
            "--action-window-relative" => {
                cli.action_window_relative = parse_f64_arg(&mut args, &arg);
            }
            "--degeneracy-label" => {
                cli.degeneracy_label = args.next().expect("--degeneracy-label requires a label");
            }
            "--steps" => {
                cli.steps = args
                    .next()
                    .expect("--steps requires comma-separated f64 values")
                    .split(',')
                    .map(|value| value.parse().expect("--steps entries must be f64"))
                    .collect();
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    if cli.diagnostic_dir.as_os_str().is_empty() {
        print_usage();
        panic!("--diagnostic-dir is required");
    }
    cli
}

fn parse_f64_arg(args: &mut impl Iterator<Item = String>, name: &str) -> f64 {
    args.next()
        .unwrap_or_else(|| panic!("{name} requires an f64"))
        .parse()
        .unwrap_or_else(|_| panic!("{name} must be an f64"))
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_secs();
    PathBuf::from(format!("/tmp/dev-sysext-sigma-line-probe-{stamp}"))
}

fn print_usage() {
    eprintln!(
        "Usage: dev-sysext-sigma-line-probe --diagnostic-dir PATH \
         [--polytope-table PATH] [--out-dir PATH] \
         [--selection-threshold-relative F64] [--action-window-relative F64] \
         [--degeneracy-label LABEL] [--steps CSV]"
    );
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|err| panic!("failed to open {path:?}: {err}"));
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read JSONL line"))
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(&line).expect("failed to parse JSONL row"))
        .collect()
}

fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writeln!(writer)?;
    }
    Ok(())
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, value)?;
    writeln!(writer)?;
    Ok(())
}
