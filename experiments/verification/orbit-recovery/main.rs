//! Geometric orbit recovery validation for trusted minimum sigma rows.
//!
//! Goal: given a trusted all-minimum sigma dataset, rebuild one-sigma KKT data,
//! recover each orbit geometrically, and validate closure / on-facet / inside-K
//! / action propositions without retesting sigma enumeration.
//!
//! Input Artifacts: experiments/verification/all-minimum/all-minimum-orbits.jsonl,
//!                  experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl
//! Output Artifacts: experiments/verification/orbit-recovery/orbit-recovery.jsonl,
//!                   experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl,
//!                   experiments/verification/orbit-recovery/smoke-orbit-recovery.jsonl,
//!                   experiments/verification/orbit-recovery/smoke-orbit-recovery-orbits.jsonl

use dev_capacity_validation::{
    create_jsonl_writer, mode_output_path, parse_run_mode, run_mode_label, target_map,
    write_json_line, RunMode, RunModeArgError, Target, ACTION_TOL, GEOMETRY_TOL,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::algorithms::hk2017::orbit_recovery::{recover_and_verify, GeometricOrbit};
use symplectic::algorithms::{solve_orbit_sigma, OrbitKktData, OrbitSolveBackend, OrbitSolveError};

#[derive(Debug, Clone, Deserialize)]
struct TrustedOrbitRow {
    polytope_name: String,
    orbit_index: usize,
    admissibility: String,
    sigma: Vec<usize>,
    subset: Vec<usize>,
    action: f64,
    total_segments: usize,
}

#[derive(Debug, Serialize)]
struct OrbitRecoverySummaryRow {
    name: String,
    family: String,
    source_kind: String,
    facet_count: usize,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_stage: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    failure_reasons: Vec<String>,
    trusted_min_orbit_count: usize,
    recovered_orbits: usize,
    failed_solves: usize,
    failed_recoveries: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_dims: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_max_violation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_closure_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_on_facet_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_inside_k_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_action_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worst_sigma_action_error: Option<f64>,
    time_rebuild_ms: f64,
    time_recovery_ms: f64,
    passes_validation: bool,
}

#[derive(Debug, Serialize)]
struct OrbitRecoveryDetailRow {
    polytope_name: String,
    family: String,
    source_kind: String,
    orbit_index: usize,
    trusted_admissibility: String,
    sigma: Vec<usize>,
    subset: Vec<usize>,
    total_segments: usize,
    trusted_action: f64,
    rebuilt_action: Option<f64>,
    sigma_action_error: Option<f64>,
    recovery_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_facets: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_dim: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_violation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closure_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_facet_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_k_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    computed_action: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passes_geometric_checks: Option<bool>,
}

struct RunPaths {
    mode: RunMode,
    trusted_orbits_path: PathBuf,
    summary_path: PathBuf,
    detail_path: PathBuf,
}

fn main() {
    let t0 = Instant::now();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let run_paths = parse_run_paths(manifest_dir);
    let targets = target_map(manifest_dir, run_paths.mode);
    let trusted_orbits = load_trusted_orbits(&run_paths.trusted_orbits_path);
    let mut summary_writer = create_jsonl_writer(&run_paths.summary_path);
    let mut detail_writer = create_jsonl_writer(&run_paths.detail_path);

    eprintln!("Mode: {}", run_mode_label(run_paths.mode));
    eprintln!(
        "Trusted sigma rows: {}",
        trusted_orbits.values().map(Vec::len).sum::<usize>()
    );

    let mut failures = 0usize;
    let mut families = BTreeMap::<String, usize>::new();

    for (name, target) in &targets {
        *families.entry(target.family.clone()).or_insert(0) += 1;
        let rows = trusted_orbits.get(name).cloned().unwrap_or_default();
        let (summary, details) = validate_target(target, rows);
        log_summary(&summary);
        if !summary.passes_validation {
            failures += 1;
        }
        write_json_line(&mut summary_writer, &summary);
        for detail in &details {
            write_json_line(&mut detail_writer, detail);
        }
    }

    summary_writer.flush().expect("flush recovery summary");
    detail_writer.flush().expect("flush recovery details");

    eprintln!("\nFamilies:");
    for (family, count) in families {
        eprintln!("  {family}: {count}");
    }
    eprintln!(
        "\nDone: {} targets, {} failures, {:.1}s total",
        targets.len(),
        failures,
        t0.elapsed().as_secs_f64()
    );
    eprintln!(
        "Summary: {}\nDetails: {}",
        run_paths.summary_path.display(),
        run_paths.detail_path.display()
    );

    if failures > 0 {
        std::process::exit(1);
    }
}

fn parse_run_paths(manifest_dir: &Path) -> RunPaths {
    let mode = match parse_run_mode(env::args().skip(1)) {
        Ok(mode) => mode,
        Err(RunModeArgError::Help) => print_help_and_exit(),
        Err(RunModeArgError::Unknown(other)) => {
            eprintln!("unknown argument: {other}");
            print_help_and_exit();
        }
    };

    let minimum_dir = manifest_dir.join("all-minimum");
    let summary_path = mode_output_path(
        manifest_dir,
        "orbit-recovery",
        "smoke-orbit-recovery.jsonl",
        "orbit-recovery.jsonl",
        mode,
    );
    let detail_path = mode_output_path(
        manifest_dir,
        "orbit-recovery",
        "smoke-orbit-recovery-orbits.jsonl",
        "orbit-recovery-orbits.jsonl",
        mode,
    );

    RunPaths {
        mode,
        trusted_orbits_path: if matches!(mode, RunMode::Full) {
            minimum_dir.join("all-minimum-orbits.jsonl")
        } else {
            minimum_dir.join("smoke-all-minimum-orbits.jsonl")
        },
        summary_path,
        detail_path,
    }
}

fn print_help_and_exit() -> ! {
    eprintln!("Usage: cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery [--full]");
    eprintln!("  default: consume smoke all-minimum rows and write smoke recovery outputs");
    eprintln!("  --full: consume canonical all-minimum rows and refresh orbit-recovery outputs");
    std::process::exit(2);
}

fn load_trusted_orbits(path: &Path) -> HashMap<String, Vec<TrustedOrbitRow>> {
    let file = File::open(path).unwrap_or_else(|err| {
        panic!(
            "failed to open trusted sigma rows {}: {err}\n\
             run `cargo run -p dev-capacity-validation --release --bin axioms-all-minimum{}` first",
            path.display(),
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("smoke-"))
            {
                ""
            } else {
                " -- --full"
            }
        )
    });
    let reader = BufReader::new(file);
    let mut grouped = HashMap::<String, Vec<TrustedOrbitRow>>::new();

    for line in reader.lines() {
        let line = line.expect("failed to read trusted sigma row");
        if line.trim().is_empty() {
            continue;
        }
        let row: TrustedOrbitRow =
            serde_json::from_str(&line).expect("failed to parse trusted sigma row");
        grouped
            .entry(row.polytope_name.clone())
            .or_default()
            .push(row);
    }

    for rows in grouped.values_mut() {
        rows.sort_by_key(|row| row.orbit_index);
    }

    grouped
}

fn validate_target(
    target: &Target,
    rows: Vec<TrustedOrbitRow>,
) -> (OrbitRecoverySummaryRow, Vec<OrbitRecoveryDetailRow>) {
    let mut summary = OrbitRecoverySummaryRow {
        name: target.name.clone(),
        family: target.family.clone(),
        source_kind: target.source_kind.clone(),
        facet_count: target.polytope.facet_count(),
        status: "failed".to_string(),
        failure_stage: None,
        failure_reasons: Vec::new(),
        trusted_min_orbit_count: rows.len(),
        recovered_orbits: 0,
        failed_solves: 0,
        failed_recoveries: 0,
        solution_dims: None,
        worst_max_violation: None,
        worst_closure_error: None,
        worst_on_facet_error: None,
        worst_inside_k_error: None,
        worst_action_error: None,
        worst_sigma_action_error: None,
        time_rebuild_ms: 0.0,
        time_recovery_ms: 0.0,
        passes_validation: false,
    };

    if rows.is_empty() {
        summary.failure_stage = Some("input".to_string());
        summary
            .failure_reasons
            .push("missing trusted minimum sigma rows".to_string());
        return (summary, Vec::new());
    }

    let mut details = Vec::new();
    let t_rebuild = Instant::now();
    let mut rebuilt_orbits = Vec::<(TrustedOrbitRow, OrbitKktData)>::new();

    for row in rows {
        match solve_orbit_sigma(&target.polytope, &row.sigma, OrbitSolveBackend::SaddlePoint) {
            Ok(orbit) => rebuilt_orbits.push((row, orbit)),
            Err(err) => {
                summary.failed_solves += 1;
                details.push(OrbitRecoveryDetailRow {
                    polytope_name: target.name.clone(),
                    family: target.family.clone(),
                    source_kind: target.source_kind.clone(),
                    orbit_index: row.orbit_index,
                    trusted_admissibility: row.admissibility,
                    sigma: row.sigma,
                    subset: row.subset,
                    total_segments: row.total_segments,
                    trusted_action: row.action,
                    rebuilt_action: None,
                    sigma_action_error: None,
                    recovery_status: solve_status(err).to_string(),
                    active_facets: None,
                    solution_dim: None,
                    max_violation: None,
                    closure_error: None,
                    on_facet_error: None,
                    inside_k_error: None,
                    computed_action: None,
                    action_error: None,
                    passes_geometric_checks: None,
                });
            }
        }
    }
    summary.time_rebuild_ms = t_rebuild.elapsed().as_secs_f64() * 1000.0;

    let t_recovery = Instant::now();
    for (row, orbit) in rebuilt_orbits {
        details.push(recover_trusted_orbit(target, row, orbit));
    }
    summary.time_recovery_ms = t_recovery.elapsed().as_secs_f64() * 1000.0;

    let successful = details
        .iter()
        .filter(|detail| detail.recovery_status == "ok")
        .collect::<Vec<_>>();
    let failed_recoveries = details
        .iter()
        .filter(|detail| detail.recovery_status == "recovery_failed")
        .count();
    let invalid = successful
        .iter()
        .filter(|detail| detail.passes_geometric_checks == Some(false))
        .count();

    summary.recovered_orbits = successful.len();
    summary.failed_recoveries = failed_recoveries;
    summary.solution_dims = Some(
        successful
            .iter()
            .filter_map(|detail| detail.solution_dim)
            .collect(),
    );
    summary.worst_max_violation = successful
        .iter()
        .filter_map(|detail| detail.max_violation)
        .max_by(f64::total_cmp);
    summary.worst_closure_error = successful
        .iter()
        .filter_map(|detail| detail.closure_error)
        .max_by(f64::total_cmp);
    summary.worst_on_facet_error = successful
        .iter()
        .filter_map(|detail| detail.on_facet_error)
        .max_by(f64::total_cmp);
    summary.worst_inside_k_error = successful
        .iter()
        .filter_map(|detail| detail.inside_k_error)
        .max_by(f64::total_cmp);
    summary.worst_action_error = successful
        .iter()
        .filter_map(|detail| detail.action_error)
        .max_by(f64::total_cmp);
    summary.worst_sigma_action_error = details
        .iter()
        .filter_map(|detail| detail.sigma_action_error)
        .max_by(f64::total_cmp);

    if summary.failed_solves > 0 {
        summary
            .failure_stage
            .get_or_insert_with(|| "kkt".to_string());
        summary.failure_reasons.push(format!(
            "{} trusted sigmas failed one-sigma solve",
            summary.failed_solves
        ));
    }
    if failed_recoveries > 0 {
        summary
            .failure_stage
            .get_or_insert_with(|| "recovery".to_string());
        summary.failure_reasons.push(format!(
            "{failed_recoveries} trusted sigmas failed geometric recovery"
        ));
    }
    if invalid > 0 {
        summary
            .failure_stage
            .get_or_insert_with(|| "validation".to_string());
        summary.failure_reasons.push(format!(
            "{invalid} recovered orbits violated geometric thresholds"
        ));
    }
    if let Some(worst_sigma_action_error) = summary.worst_sigma_action_error {
        if worst_sigma_action_error > ACTION_TOL {
            summary
                .failure_stage
                .get_or_insert_with(|| "trusted_input".to_string());
            summary.failure_reasons.push(format!(
                "trusted sigma rows disagree with rebuilt one-sigma solve: max action drift {:.2e}",
                worst_sigma_action_error
            ));
        }
    }

    summary.passes_validation = summary.failure_stage.is_none();
    summary.status = if summary.passes_validation {
        "ok".to_string()
    } else {
        "failed".to_string()
    };

    (summary, details)
}

fn recover_trusted_orbit(
    target: &Target,
    row: TrustedOrbitRow,
    orbit: OrbitKktData,
) -> OrbitRecoveryDetailRow {
    let sigma_action_error = (orbit.action - row.action).abs();
    let mut detail = OrbitRecoveryDetailRow {
        polytope_name: target.name.clone(),
        family: target.family.clone(),
        source_kind: target.source_kind.clone(),
        orbit_index: row.orbit_index,
        trusted_admissibility: row.admissibility,
        sigma: row.sigma,
        subset: row.subset,
        total_segments: row.total_segments,
        trusted_action: row.action,
        rebuilt_action: Some(orbit.action),
        sigma_action_error: Some(sigma_action_error),
        recovery_status: "recovery_failed".to_string(),
        active_facets: None,
        solution_dim: None,
        max_violation: None,
        closure_error: None,
        on_facet_error: None,
        inside_k_error: None,
        computed_action: None,
        action_error: None,
        passes_geometric_checks: None,
    };

    let recovery = match recover_and_verify(&target.polytope, &orbit) {
        Some(recovery) => recovery,
        None => return detail,
    };

    let on_facet_error = compute_on_facet_error(&target.polytope, &orbit.sigma, &recovery);
    let action_error = (recovery.action - orbit.action).abs();
    let passes = recovery.closure_error < GEOMETRY_TOL
        && on_facet_error < GEOMETRY_TOL
        && recovery.max_violation < GEOMETRY_TOL
        && action_error < ACTION_TOL;

    detail.recovery_status = "ok".to_string();
    detail.active_facets = Some(
        recovery
            .dwell_times
            .iter()
            .filter(|&&tau| tau > 0.0)
            .count(),
    );
    detail.solution_dim = Some(recovery.solution_dim);
    detail.max_violation = Some(recovery.max_violation);
    detail.closure_error = Some(recovery.closure_error);
    detail.on_facet_error = Some(on_facet_error);
    detail.inside_k_error = Some(recovery.max_violation);
    detail.computed_action = Some(recovery.action);
    detail.action_error = Some(action_error);
    detail.passes_geometric_checks = Some(passes);
    detail
}

fn solve_status(err: OrbitSolveError) -> &'static str {
    match err {
        OrbitSolveError::UnsupportedBackend => "solve_unsupported_backend",
        OrbitSolveError::Inadmissible => "solve_inadmissible",
        OrbitSolveError::NumericalFailure => "solve_numerical_failure",
    }
}

fn compute_on_facet_error(
    polytope: &symplectic::Polytope4D,
    sigma: &[usize],
    recovery: &GeometricOrbit,
) -> f64 {
    let duals = polytope.dual_vertices_f64();
    (0..sigma.len())
        .filter(|&k| recovery.dwell_times[k] > 0.0)
        .map(|k| {
            let facet = &duals[sigma[k]];
            (facet.dot(&recovery.breakpoints[k]) - 1.0).abs()
        })
        .fold(0.0_f64, f64::max)
}

fn log_summary(summary: &OrbitRecoverySummaryRow) {
    if summary.passes_validation {
        eprintln!(
            "  {} [{}] trusted={} recovered={} OK",
            summary.name, summary.family, summary.trusted_min_orbit_count, summary.recovered_orbits,
        );
        return;
    }

    eprintln!(
        "  FAIL {} [{}] stage={} {}",
        summary.name,
        summary.family,
        summary.failure_stage.as_deref().unwrap_or("unknown"),
        summary.failure_reasons.join("; "),
    );
}
