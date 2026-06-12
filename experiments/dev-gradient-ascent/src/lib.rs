//! Artifact schemas and smoke helpers for gradient-ascent method development.
//!
//! This crate is the top-level development surface for a heuristic ascent
//! method for `sys(a)`. The current code validates artifact shape on synthetic
//! spectra; it does not claim to optimize real polytopes.

use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const THRESHOLDS_RELATIVE: &[f64] = &[1.0e-9, 1.0e-6, 1.0e-3, 1.0e-2];

#[derive(Clone, Debug)]
pub struct SmokeConfig {
    pub out_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct SyntheticFixture {
    fixture_id: &'static str,
    source_role: &'static str,
    action_values: &'static [f64],
    expected_regime: &'static str,
}

#[derive(Serialize)]
pub struct RunTraceRow {
    fixture_id: String,
    iteration: usize,
    method_variant: String,
    degeneracy_regime: String,
    active_window_relative: f64,
    near_active_count: usize,
    step_policy: String,
    step_size: f64,
    predicted_common_ascent: Option<f64>,
    observed_delta_sys: Option<f64>,
    stop_reason: String,
}

#[derive(Serialize)]
pub struct BranchSetDiagnosticRow {
    fixture_id: String,
    source_role: String,
    expected_regime: String,
    min_action: f64,
    active_window_relative: f64,
    near_active_count: usize,
    action_gap_to_second: Option<f64>,
    action_gap_to_last_near_active: Option<f64>,
    diagnostic_role: String,
}

#[derive(Serialize)]
pub struct LocalGeometryProbeRow {
    fixture_id: String,
    direction_id: String,
    radius: f64,
    branch_set_source: String,
    predicted_delta_sys: f64,
    observed_delta_sys: f64,
    model_status: String,
}

#[derive(Serialize)]
pub struct EndpointDiagnosticRow {
    fixture_id: String,
    degeneracy_regime: String,
    quotient_model: String,
    common_ascent_status: String,
    contradiction_probe_status: String,
    heuristic_local_max_status: String,
    caveat: String,
}

#[derive(Serialize)]
pub struct ComputeBudgetReport {
    command: String,
    fixture_count: usize,
    synthetic: bool,
    exact_evaluation_count: usize,
    branch_set_diagnostic_rows: usize,
    run_trace_rows: usize,
    local_geometry_probe_rows: usize,
    endpoint_diagnostic_rows: usize,
}

#[derive(Serialize)]
pub struct SmokeSummary {
    pub purpose: String,
    pub out_dir: String,
    pub artifact_files: Vec<String>,
    pub status: String,
    pub caveat: String,
}

pub fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-smoke-{}-{stamp}",
        std::process::id()
    ))
}

pub fn run_smoke(config: &SmokeConfig) -> std::io::Result<SmokeSummary> {
    fs::create_dir_all(&config.out_dir)?;

    let fixtures = synthetic_fixtures();
    let mut branch_rows = Vec::new();
    let mut trace_rows = Vec::new();
    let mut probe_rows = Vec::new();
    let mut endpoint_rows = Vec::new();

    for fixture in &fixtures {
        for &threshold in THRESHOLDS_RELATIVE {
            branch_rows.push(branch_set_row(fixture, threshold));
        }
        trace_rows.extend(run_trace_rows(fixture));
        probe_rows.extend(local_probe_rows(fixture));
        endpoint_rows.push(endpoint_row(fixture));
    }

    write_jsonl(
        config.out_dir.join("branch-set-diagnostic.jsonl"),
        &branch_rows,
    )?;
    write_jsonl(config.out_dir.join("run-trace.jsonl"), &trace_rows)?;
    write_jsonl(
        config.out_dir.join("local-geometry-probe.jsonl"),
        &probe_rows,
    )?;
    write_jsonl(
        config.out_dir.join("endpoint-diagnostic.jsonl"),
        &endpoint_rows,
    )?;

    let report = ComputeBudgetReport {
        command: "dev-gradient-ascent-smoke".to_string(),
        fixture_count: fixtures.len(),
        synthetic: true,
        exact_evaluation_count: 0,
        branch_set_diagnostic_rows: branch_rows.len(),
        run_trace_rows: trace_rows.len(),
        local_geometry_probe_rows: probe_rows.len(),
        endpoint_diagnostic_rows: endpoint_rows.len(),
    };
    write_json(config.out_dir.join("compute-budget-report.json"), &report)?;

    let summary = SmokeSummary {
        purpose: "validate dev-gradient-ascent artifact surface".to_string(),
        out_dir: config.out_dir.display().to_string(),
        artifact_files: vec![
            "branch-set-diagnostic.jsonl".to_string(),
            "run-trace.jsonl".to_string(),
            "local-geometry-probe.jsonl".to_string(),
            "endpoint-diagnostic.jsonl".to_string(),
            "compute-budget-report.json".to_string(),
            "summary.json".to_string(),
        ],
        status: "smoke_complete".to_string(),
        caveat: "synthetic spectra only; not a sys(a) method result".to_string(),
    };
    write_json(config.out_dir.join("summary.json"), &summary)?;
    Ok(summary)
}

fn synthetic_fixtures() -> Vec<SyntheticFixture> {
    vec![
        SyntheticFixture {
            fixture_id: "large-gap-synthetic",
            source_role: "synthetic_random_start_like",
            action_values: &[1.0, 1.12, 1.35, 1.8],
            expected_regime: "large_gap",
        },
        SyntheticFixture {
            fixture_id: "narrow-gap-synthetic",
            source_role: "synthetic_ridge_like",
            action_values: &[1.0, 1.0008, 1.004, 1.1],
            expected_regime: "narrow_gap",
        },
        SyntheticFixture {
            fixture_id: "high-degeneracy-synthetic",
            source_role: "synthetic_endpoint_candidate_like",
            action_values: &[1.0, 1.0 + 5.0e-10, 1.0 + 9.0e-10, 1.000001, 1.0002],
            expected_regime: "high_degeneracy",
        },
    ]
}

fn branch_set_row(fixture: &SyntheticFixture, threshold: f64) -> BranchSetDiagnosticRow {
    let min_action = fixture
        .action_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let cutoff = min_action * (1.0 + threshold);
    let mut near_active: Vec<f64> = fixture
        .action_values
        .iter()
        .copied()
        .filter(|action| *action <= cutoff)
        .collect();
    near_active.sort_by(|a, b| a.total_cmp(b));

    let mut sorted = fixture.action_values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let action_gap_to_second = sorted.get(1).map(|second| second - min_action);
    let action_gap_to_last_near_active = near_active.last().map(|last| last - min_action);

    BranchSetDiagnosticRow {
        fixture_id: fixture.fixture_id.to_string(),
        source_role: fixture.source_role.to_string(),
        expected_regime: fixture.expected_regime.to_string(),
        min_action,
        active_window_relative: threshold,
        near_active_count: near_active.len(),
        action_gap_to_second,
        action_gap_to_last_near_active,
        diagnostic_role: "threshold_sweep_schema_smoke".to_string(),
    }
}

fn run_trace_rows(fixture: &SyntheticFixture) -> Vec<RunTraceRow> {
    let (variant, step_policy, stop_reason, predicted, observed) = match fixture.expected_regime {
        "large_gap" => (
            "single_branch_placeholder",
            "large_smooth_step",
            "schema_smoke_step_accepted",
            Some(0.11),
            Some(0.09),
        ),
        "narrow_gap" => (
            "near_active_maximin_placeholder",
            "adaptive_ridge_step",
            "schema_smoke_needs_real_probe",
            Some(0.025),
            Some(0.018),
        ),
        _ => (
            "endpoint_diagnostic_placeholder",
            "tiny_contradiction_probe",
            "schema_smoke_heuristic_stop",
            Some(0.0),
            Some(0.0),
        ),
    };

    vec![RunTraceRow {
        fixture_id: fixture.fixture_id.to_string(),
        iteration: 0,
        method_variant: variant.to_string(),
        degeneracy_regime: fixture.expected_regime.to_string(),
        active_window_relative: 1.0e-6,
        near_active_count: branch_set_row(fixture, 1.0e-6).near_active_count,
        step_policy: step_policy.to_string(),
        step_size: match fixture.expected_regime {
            "large_gap" => 0.1,
            "narrow_gap" => 0.01,
            _ => 1.0e-5,
        },
        predicted_common_ascent: predicted,
        observed_delta_sys: observed,
        stop_reason: stop_reason.to_string(),
    }]
}

fn local_probe_rows(fixture: &SyntheticFixture) -> Vec<LocalGeometryProbeRow> {
    [1.0e-4, 1.0e-3, 1.0e-2]
        .into_iter()
        .map(|radius| {
            let predicted_delta_sys = match fixture.expected_regime {
                "large_gap" => 0.8 * radius,
                "narrow_gap" => 0.25 * radius,
                _ => 0.02 * radius,
            };
            let observed_delta_sys = match fixture.expected_regime {
                "large_gap" => 0.78 * radius,
                "narrow_gap" => 0.18 * radius,
                _ => -0.01 * radius,
            };
            LocalGeometryProbeRow {
                fixture_id: fixture.fixture_id.to_string(),
                direction_id: "synthetic-direction-0".to_string(),
                radius,
                branch_set_source: "synthetic_action_spectrum".to_string(),
                predicted_delta_sys,
                observed_delta_sys,
                model_status: "schema_smoke_not_real_sys".to_string(),
            }
        })
        .collect()
}

fn endpoint_row(fixture: &SyntheticFixture) -> EndpointDiagnosticRow {
    let (common_ascent_status, contradiction_probe_status, local_max_status, caveat) =
        match fixture.expected_regime {
            "large_gap" => (
                "common_ascent_available",
                "not_run_not_endpoint_like",
                "not_endpoint_candidate",
                "large-gap smoke fixture is for smooth-step schema only",
            ),
            "narrow_gap" => (
                "common_ascent_depends_on_branch_window",
                "probe_required",
                "inconclusive",
                "narrow-gap smoke fixture represents ridge-development questions",
            ),
            _ => (
                "no_common_ascent_in_placeholder_model",
                "no_improvement_in_placeholder_probe",
                "heuristic_pass_synthetic_only",
                "synthetic endpoint status does not certify any real local maximum",
            ),
        };
    EndpointDiagnosticRow {
        fixture_id: fixture.fixture_id.to_string(),
        degeneracy_regime: fixture.expected_regime.to_string(),
        quotient_model: "not_constructed_in_schema_smoke".to_string(),
        common_ascent_status: common_ascent_status.to_string(),
        contradiction_probe_status: contradiction_probe_status.to_string(),
        heuristic_local_max_status: local_max_status.to_string(),
        caveat: caveat.to_string(),
    }
}

fn write_jsonl<P: AsRef<Path>, T: Serialize>(path: P, rows: &[T]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}
