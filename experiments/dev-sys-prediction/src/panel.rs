//! Config-driven local panel runner for sys-prediction experiments.
//!
//! The public data flow is basepoints -> states -> perturbation events -> observations/reports.
//! Computation caches are acceleration artifacts, not dataset identity.
//!
//! Realized sampling law, not an iid population model:
//!
//! ```text
//! for each configured facet bucket F:
//!   choose n_F basepoints from the prepared table rows with facet_count = F
//!   and capacity_source = source, using deterministic seeded ordering;
//! for each selected basepoint a0:
//!   annotate the branch window at a0;
//!   build deterministic probe directions from branch-gradient geometry plus
//!   fixed pseudo-random controls;
//!   evaluate a = a0 + t u for configured radii t.
//! ```
//!
//! Therefore `F`, `a0`, and `u` are not independent random variables here.
//! In particular, the conditional distribution of `a0 | F` is whatever the
//! configured basepoint selector realizes for that facet bucket, and `u` is an
//! algorithmic direction set conditional on the branch geometry of `a0`.
//! Population claims such as `a0 | F ~ Random(F)` or `u ~ Uniform(S^{4F-1})`
//! require a different producer or an explicit interpretation layer.

use crate::basepoints::{provenance_rows, select_basepoints, BasepointSelectionFacetSummary};
use crate::panel_analysis::{
    summarize_beta_scan, summarize_prediction_probe, BetaFacetSummary, PredictionHighlight,
};
use crate::panel_cache::sys_cache_paths;
use crate::panel_io::{read_json, read_required_json, require_nonempty, write_json, write_jsonl};
use crate::{prediction_cloud, sysext_beta_boundary_scan};
use exp_dev_gradient_ascent::branch_diagnostic;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

const DEFAULT_POLYTOPE_TABLE: &str = "experiments/sys-datascience/prepare/polytope-table.jsonl";
const DEFAULT_BRANCH_THRESHOLD_RELATIVE: f64 = 0.01;
const DEFAULT_ACTION_WINDOW_RELATIVE: f64 = 0.01;
const DEFAULT_SELECTION_SEED: &str = "dev-sys-prediction-panel-v1";

#[derive(Debug)]
struct Cli {
    config: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct PanelConfig {
    #[serde(default = "default_polytope_table")]
    polytope_table: String,
    #[serde(default = "default_source")]
    source: String,
    #[serde(default)]
    sys_cache_inputs: Vec<String>,
    #[serde(default)]
    sys_cache_output: Option<String>,
    buckets: Vec<FacetBucketConfig>,
    steps: Vec<f64>,
    #[serde(default)]
    trace_iterations: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct FacetBucketConfig {
    facet_count: usize,
    #[serde(default)]
    basepoints: usize,
    #[serde(default)]
    beta_boundary_rows: usize,
}

#[derive(Debug, Deserialize)]
struct BranchSummary {
    selected_rows: usize,
    successful_recomputations: usize,
    failed_recomputations: usize,
    degeneracy_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct ProducerSummary {
    selected_fixtures: usize,
    selected_counts_by_label: BTreeMap<String, usize>,
    requested_labels_with_no_selected_fixture: Vec<String>,
    probe_rows: usize,
    failed_probe_rows: usize,
}

#[derive(Serialize)]
struct DatasetSummary {
    method: String,
    config: PanelConfig,
    out_dir: String,
    basepoint_event_panel: Option<BasepointEventPanelSummary>,
    reports: ReportSummary,
    stage_costs: Vec<StageCost>,
    elapsed_s: f64,
}

#[derive(Serialize)]
struct ReportSummary {
    beta_boundary: Vec<BetaFacetSummary>,
}

#[derive(Serialize)]
struct BasepointEventPanelSummary {
    selected_basepoints: usize,
    basepoint_selection_method: String,
    basepoint_selection_seed: String,
    basepoint_selection_by_facet: BTreeMap<usize, BasepointSelectionFacetSummary>,
    actual_selected_fixtures: usize,
    selected_counts_by_label: BTreeMap<String, usize>,
    requested_labels_with_no_selected_fixture: Vec<String>,
    probe_rows: usize,
    branch_selected_rows: usize,
    branch_successful_recomputations: usize,
    branch_degeneracy_counts: BTreeMap<String, usize>,
    sys_cache_inputs: Vec<String>,
    sys_cache_output: String,
    panel_path: String,
    provenance_path: String,
    branch_dir: String,
    perturbation_dir: String,
    prediction_highlights: Vec<PredictionHighlight>,
}

#[derive(Serialize)]
struct StageCost {
    stage: String,
    elapsed_s: f64,
    argv: Vec<String>,
}

pub fn main_from_env() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");
    let started = Instant::now();
    let config: PanelConfig = read_json(&cli.config);
    validate_config(&config);
    let mut stage_costs = Vec::new();

    let basepoint_event_panel = if config.buckets.iter().any(|bucket| bucket.basepoints > 0) {
        assert!(
            !config.steps.is_empty(),
            "config with basepoints must provide at least one step"
        );
        Some(run_basepoint_event_panel(&cli, &config, &mut stage_costs))
    } else {
        None
    };
    let beta_boundary = run_beta_boundary_report(&cli, &config, &mut stage_costs);

    let summary = DatasetSummary {
        method: "dev-sys-prediction-panel".to_string(),
        config,
        out_dir: cli.out_dir.display().to_string(),
        basepoint_event_panel,
        reports: ReportSummary { beta_boundary },
        stage_costs,
        elapsed_s: started.elapsed().as_secs_f64(),
    };
    write_json(&cli.out_dir.join("dataset-summary.json"), &summary);
    println!("{}", cli.out_dir.join("dataset-summary.json").display());
}

fn run_basepoint_event_panel(
    cli: &Cli,
    config: &PanelConfig,
    stage_costs: &mut Vec<StageCost>,
) -> BasepointEventPanelSummary {
    let panel_dir = cli.out_dir.join("basepoint-event-panel");
    let panel_path = panel_dir.join("basepoint-polytope-panel.jsonl");
    let provenance_path = panel_dir.join("basepoint-provenance-panel.jsonl");
    let branch_dir = panel_dir.join("branch-annotation");
    let perturbation_dir = panel_dir.join("perturbation-cloud");
    let cache_paths = sys_cache_paths(
        &cli.out_dir,
        &config.sys_cache_inputs,
        config.sys_cache_output.as_ref(),
    );
    if let Some(parent) = cache_paths.output.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| {
            panic!(
                "failed to create sys cache output parent {}: {err}",
                parent.display()
            )
        });
    }

    let basepoint_counts = basepoint_counts_by_facet(config);
    let table_path = PathBuf::from(&config.polytope_table);
    let basepoint_selection = select_basepoints(
        &table_path,
        &basepoint_counts,
        &config.source,
        DEFAULT_SELECTION_SEED,
    );
    let panel_rows = basepoint_selection.rows;
    write_jsonl(&panel_path, &panel_rows);
    write_jsonl(
        &provenance_path,
        &provenance_rows(&panel_rows, &config.source),
    );

    let selected_count = panel_rows.len();
    let threshold = DEFAULT_BRANCH_THRESHOLD_RELATIVE.to_string();
    let branch_argv = producer_argv(
        "dev-gradient-ascent-branch-diagnostic",
        &[
            "--polytope-table",
            panel_path.to_str().expect("utf8 path"),
            "--provenance-table",
            provenance_path.to_str().expect("utf8 path"),
            "--out-dir",
            branch_dir.to_str().expect("utf8 path"),
            "--max-rows",
            &(selected_count * 4).to_string(),
            "--thresholds-relative",
            &threshold,
        ],
    );
    run_stage(
        "basepoint_branch_annotation",
        branch_argv,
        stage_costs,
        branch_diagnostic::run_from_args,
    );

    let branch_summary: BranchSummary = read_required_json(&branch_dir.join("summary.json"));
    require_nonempty(&branch_dir.join("branch-set-diagnostic.jsonl"));
    assert_eq!(
        branch_summary.failed_recomputations, 0,
        "branch diagnostic had failed recomputations"
    );

    let steps = join_f64_csv(&config.steps);
    let labels = default_degeneracy_labels().join(",");
    let selection_threshold = DEFAULT_BRANCH_THRESHOLD_RELATIVE.to_string();
    let action_window = DEFAULT_ACTION_WINDOW_RELATIVE.to_string();
    let trace_iterations = config.trace_iterations.to_string();
    let mut perturbation_argv = producer_argv(
        "dev-sys-prediction-panel:perturbation-cloud",
        &[
            "--diagnostic-dir",
            branch_dir.to_str().expect("utf8 path"),
            "--polytope-table",
            panel_path.to_str().expect("utf8 path"),
            "--out-dir",
            perturbation_dir.to_str().expect("utf8 path"),
            "--selection-threshold-relative",
            &selection_threshold,
            "--action-window-relative",
            &action_window,
            "--degeneracy-labels",
            &labels,
            "--max-fixtures-per-label",
            &selected_count.to_string(),
            "--steps",
            &steps,
            "--trace-iterations",
            &trace_iterations,
            "--skip-endpoint-diagnostics",
        ],
    );
    for path in &cache_paths.inputs {
        perturbation_argv.push("--sys-cache-input".to_string());
        perturbation_argv.push(path.display().to_string());
    }
    perturbation_argv.push("--sys-cache-output".to_string());
    perturbation_argv.push(cache_paths.output.display().to_string());
    run_stage(
        "perturbation_cloud",
        perturbation_argv,
        stage_costs,
        prediction_cloud::run_from_args,
    );

    let perturbation_summary: ProducerSummary =
        read_required_json(&perturbation_dir.join("summary.json"));
    let probe_path = perturbation_dir.join("local-geometry-probe.jsonl");
    require_nonempty(&probe_path);
    publish_core_identity_rows(&perturbation_dir, &cli.out_dir);
    assert!(
        perturbation_summary.selected_fixtures > 0,
        "perturbation cloud selected zero basepoints"
    );
    assert_eq!(
        perturbation_summary.failed_probe_rows, 0,
        "perturbation cloud probe rows failed"
    );
    let poly_id_to_facet = panel_rows
        .iter()
        .map(|row| (row.poly_id.clone(), row.facet_count))
        .collect::<BTreeMap<_, _>>();

    BasepointEventPanelSummary {
        selected_basepoints: selected_count,
        basepoint_selection_method: "seeded_hash".to_string(),
        basepoint_selection_seed: DEFAULT_SELECTION_SEED.to_string(),
        basepoint_selection_by_facet: basepoint_selection.summary_by_facet,
        actual_selected_fixtures: perturbation_summary.selected_fixtures,
        selected_counts_by_label: perturbation_summary.selected_counts_by_label,
        requested_labels_with_no_selected_fixture: perturbation_summary
            .requested_labels_with_no_selected_fixture,
        probe_rows: perturbation_summary.probe_rows,
        branch_selected_rows: branch_summary.selected_rows,
        branch_successful_recomputations: branch_summary.successful_recomputations,
        branch_degeneracy_counts: branch_summary.degeneracy_counts,
        sys_cache_inputs: cache_paths
            .inputs
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        sys_cache_output: cache_paths.output.display().to_string(),
        panel_path: panel_path.display().to_string(),
        provenance_path: provenance_path.display().to_string(),
        branch_dir: branch_dir.display().to_string(),
        perturbation_dir: perturbation_dir.display().to_string(),
        prediction_highlights: summarize_prediction_probe(&probe_path, &poly_id_to_facet),
    }
}

fn publish_core_identity_rows(source_dir: &Path, out_dir: &Path) {
    for file_name in ["basepoints.jsonl", "states.jsonl", "events.jsonl"] {
        let source = source_dir.join(file_name);
        require_nonempty(&source);
        fs::copy(&source, out_dir.join(file_name)).unwrap_or_else(|err| {
            panic!(
                "failed to publish {} from {}: {err}",
                file_name,
                source.display()
            )
        });
    }
}

fn run_beta_boundary_report(
    cli: &Cli,
    config: &PanelConfig,
    stage_costs: &mut Vec<StageCost>,
) -> Vec<BetaFacetSummary> {
    let beta_dir = cli.out_dir.join("beta-boundary-scan");
    fs::create_dir_all(&beta_dir).expect("failed to create beta scan output directory");
    let mut summaries = Vec::new();
    for bucket in config
        .buckets
        .iter()
        .filter(|bucket| bucket.beta_boundary_rows > 0)
    {
        let facet_count = bucket.facet_count;
        let out_path = beta_dir.join(format!("facet-{facet_count}.jsonl"));
        let facet = facet_count.to_string();
        let max_rows = bucket.beta_boundary_rows.to_string();
        let argv = producer_argv(
            "dev-sys-prediction-panel:beta-boundary-report",
            &[
                "--polytope-table",
                &config.polytope_table,
                "--out",
                out_path.to_str().expect("utf8 path"),
                "--capacity-source",
                &config.source,
                "--facet-counts",
                &facet,
                "--max-rows",
                &max_rows,
            ],
        );
        run_stage(
            &format!("beta_boundary_scan_F{facet_count}"),
            argv,
            stage_costs,
            sysext_beta_boundary_scan::run_from_args,
        );
        require_nonempty(&out_path);
        summaries.push(summarize_beta_scan(facet_count, &out_path));
    }
    summaries
}

fn basepoint_counts_by_facet(config: &PanelConfig) -> BTreeMap<usize, usize> {
    config
        .buckets
        .iter()
        .filter(|bucket| bucket.basepoints > 0)
        .map(|bucket| (bucket.facet_count, bucket.basepoints))
        .collect()
}

fn validate_config(config: &PanelConfig) {
    assert!(
        !config.buckets.is_empty(),
        "config must provide at least one facet bucket"
    );
    let mut seen = BTreeSet::new();
    for bucket in &config.buckets {
        assert!(
            seen.insert(bucket.facet_count),
            "duplicate facet_count {} in config buckets",
            bucket.facet_count
        );
    }
}

fn run_stage<F>(stage: &str, argv: Vec<String>, stage_costs: &mut Vec<StageCost>, run: F)
where
    F: FnOnce(Vec<String>),
{
    let started = Instant::now();
    run(argv.clone());
    stage_costs.push(StageCost {
        stage: stage.to_string(),
        elapsed_s: started.elapsed().as_secs_f64(),
        argv,
    });
}

fn producer_argv(program_name: &str, args: &[&str]) -> Vec<String> {
    let mut argv = vec![program_name.to_string()];
    argv.extend(args.iter().map(|arg| (*arg).to_string()));
    argv
}

fn join_f64_csv(values: &[f64]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_args() -> Cli {
    let mut config = None;
    let mut out_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" => {
                config = Some(PathBuf::from(
                    args.next().expect("--config requires a path"),
                ));
            }
            "--out-dir" => {
                out_dir = Some(PathBuf::from(
                    args.next().expect("--out-dir requires a path"),
                ));
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    Cli {
        config: config.expect("--config is required"),
        out_dir: out_dir.expect("--out-dir is required"),
    }
}

fn print_usage() {
    eprintln!("Usage: dev-sys-prediction-panel --config PATH --out-dir PATH");
}

fn default_polytope_table() -> String {
    DEFAULT_POLYTOPE_TABLE.to_string()
}

fn default_source() -> String {
    "random_sample".to_string()
}

fn default_degeneracy_labels() -> Vec<String> {
    vec![
        "large_gap".to_string(),
        "narrow_gap".to_string(),
        "high_degeneracy".to_string(),
    ]
}
