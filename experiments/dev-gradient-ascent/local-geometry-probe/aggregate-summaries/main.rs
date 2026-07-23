//! Aggregate split local-geometry runs.
//!
//! This command is intentionally small: it reads complete per-run
//! `summary.json` and `compute-budget-report.json` files, writes one JSONL row
//! per run, and writes aggregate counts. It does not inspect or reinterpret
//! trace/probe rows.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Cli {
    out_dir: PathBuf,
    run_dirs: Vec<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
struct LocalGeometrySummary {
    method: String,
    selection_threshold_relative: Option<f64>,
    max_fixtures_per_label: Option<usize>,
    skip_fixtures_per_label: Option<usize>,
    degeneracy_labels: Option<Vec<String>>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: Option<usize>,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: Option<usize>,
    degeneracy_counts: BTreeMap<String, usize>,
    status_counts: BTreeMap<String, usize>,
    trace_stop_reason_counts: BTreeMap<String, usize>,
    trace_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_status_counts: BTreeMap<String, usize>,
    endpoint_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_status_counts: Option<BTreeMap<String, usize>>,
    endpoint_direction_scan_threshold_counts: Option<BTreeMap<String, usize>>,
    out_dir: String,
    caveat: String,
}

#[derive(Serialize)]
struct RunSummaryRow {
    run_dir: String,
    method: String,
    selection_threshold_relative: Option<f64>,
    max_fixtures_per_label: Option<usize>,
    skip_fixtures_per_label: Option<usize>,
    degeneracy_labels: Option<Vec<String>>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    degeneracy_counts: BTreeMap<String, usize>,
    trace_stop_reason_counts: BTreeMap<String, usize>,
    endpoint_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_threshold_counts: BTreeMap<String, usize>,
    caveat: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ComputeBudgetReport {
    command: String,
    diagnostic_dir: Option<String>,
    polytope_table: Option<String>,
    selection_threshold_relative: Option<f64>,
    max_fixtures_per_label: Option<usize>,
    skip_fixtures_per_label: Option<usize>,
    degeneracy_labels: Option<Vec<String>>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: Option<usize>,
    base_orbit_iterations: Option<u64>,
    target_orbit_iterations: Option<u64>,
    trace_base_orbit_iterations: Option<u64>,
    trace_target_orbit_iterations: Option<u64>,
    endpoint_base_orbit_iterations: Option<u64>,
    endpoint_target_orbit_iterations: Option<u64>,
    endpoint_scan_base_orbit_iterations: Option<u64>,
    endpoint_scan_target_orbit_iterations: Option<u64>,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: Option<usize>,
    elapsed_ms: Option<f64>,
}

#[derive(Serialize)]
struct BudgetSummaryRow {
    run_dir: String,
    command: String,
    diagnostic_dir: Option<String>,
    polytope_table: Option<String>,
    selection_threshold_relative: Option<f64>,
    max_fixtures_per_label: Option<usize>,
    skip_fixtures_per_label: Option<usize>,
    degeneracy_labels: Option<Vec<String>>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: u64,
    trace_base_orbit_iterations: u64,
    trace_target_orbit_iterations: u64,
    endpoint_base_orbit_iterations: u64,
    endpoint_target_orbit_iterations: u64,
    endpoint_scan_base_orbit_iterations: u64,
    endpoint_scan_target_orbit_iterations: u64,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    elapsed_ms: Option<f64>,
}

#[derive(Default, Serialize)]
struct BudgetTotals {
    run_count: usize,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: u64,
    trace_base_orbit_iterations: u64,
    trace_target_orbit_iterations: u64,
    endpoint_base_orbit_iterations: u64,
    endpoint_target_orbit_iterations: u64,
    endpoint_scan_base_orbit_iterations: u64,
    endpoint_scan_target_orbit_iterations: u64,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    elapsed_ms: Option<f64>,
}

#[derive(Serialize)]
struct AggregateSummary {
    command: String,
    run_count: usize,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    degeneracy_counts: BTreeMap<String, usize>,
    status_counts: BTreeMap<String, usize>,
    trace_stop_reason_counts: BTreeMap<String, usize>,
    trace_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_status_counts: BTreeMap<String, usize>,
    endpoint_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_threshold_counts: BTreeMap<String, usize>,
    budget_report_count: usize,
    budget_selected_fixtures: usize,
    budget_probe_rows: usize,
    budget_run_trace_rows: usize,
    budget_endpoint_diagnostic_rows: usize,
    budget_endpoint_direction_scan_rows: usize,
    budget_base_orbit_iterations: u64,
    budget_target_orbit_iterations: u64,
    budget_trace_base_orbit_iterations: u64,
    budget_trace_target_orbit_iterations: u64,
    budget_endpoint_base_orbit_iterations: u64,
    budget_endpoint_target_orbit_iterations: u64,
    budget_endpoint_scan_base_orbit_iterations: u64,
    budget_endpoint_scan_target_orbit_iterations: u64,
    budget_failed_probe_rows: usize,
    budget_failed_endpoint_direction_scan_rows: usize,
    budget_elapsed_ms: Option<f64>,
    budget_by_degeneracy_label: BTreeMap<String, BudgetTotals>,
    run_dirs: Vec<String>,
    artifact_files: Vec<String>,
    caveat: String,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");

    let mut run_rows = Vec::new();
    let mut budget_rows = Vec::new();
    let mut aggregate = AggregateSummary {
        command: "dev-gradient-ascent-aggregate-summaries".to_string(),
        run_count: 0,
        selected_fixtures: 0,
        probe_rows: 0,
        run_trace_rows: 0,
        endpoint_diagnostic_rows: 0,
        endpoint_direction_scan_rows: 0,
        failed_probe_rows: 0,
        failed_endpoint_direction_scan_rows: 0,
        degeneracy_counts: BTreeMap::new(),
        status_counts: BTreeMap::new(),
        trace_stop_reason_counts: BTreeMap::new(),
        trace_line_search_status_counts: BTreeMap::new(),
        endpoint_status_counts: BTreeMap::new(),
        endpoint_line_search_status_counts: BTreeMap::new(),
        endpoint_direction_scan_status_counts: BTreeMap::new(),
        endpoint_direction_scan_threshold_counts: BTreeMap::new(),
        budget_report_count: 0,
        budget_selected_fixtures: 0,
        budget_probe_rows: 0,
        budget_run_trace_rows: 0,
        budget_endpoint_diagnostic_rows: 0,
        budget_endpoint_direction_scan_rows: 0,
        budget_base_orbit_iterations: 0,
        budget_target_orbit_iterations: 0,
        budget_trace_base_orbit_iterations: 0,
        budget_trace_target_orbit_iterations: 0,
        budget_endpoint_base_orbit_iterations: 0,
        budget_endpoint_target_orbit_iterations: 0,
        budget_endpoint_scan_base_orbit_iterations: 0,
        budget_endpoint_scan_target_orbit_iterations: 0,
        budget_failed_probe_rows: 0,
        budget_failed_endpoint_direction_scan_rows: 0,
        budget_elapsed_ms: None,
        budget_by_degeneracy_label: BTreeMap::new(),
        run_dirs: Vec::new(),
        artifact_files: vec![
            "run-summary.jsonl".to_string(),
            "budget-summary.jsonl".to_string(),
            "aggregate-summary.json".to_string(),
        ],
        caveat: "summary and budget aggregation only; this does not inspect trace rows or certify endpoint local maximality".to_string(),
    };

    for run_dir in &cli.run_dirs {
        let summary: LocalGeometrySummary = read_json(&run_dir.join("summary.json"));
        let endpoint_direction_scan_rows = summary.endpoint_direction_scan_rows.unwrap_or(0);
        let failed_endpoint_direction_scan_rows =
            summary.failed_endpoint_direction_scan_rows.unwrap_or(0);
        let endpoint_direction_scan_threshold_counts = summary
            .endpoint_direction_scan_threshold_counts
            .clone()
            .unwrap_or_default();

        aggregate.run_count += 1;
        aggregate.selected_fixtures += summary.selected_fixtures;
        aggregate.probe_rows += summary.probe_rows;
        aggregate.run_trace_rows += summary.run_trace_rows;
        aggregate.endpoint_diagnostic_rows += summary.endpoint_diagnostic_rows;
        aggregate.endpoint_direction_scan_rows += endpoint_direction_scan_rows;
        aggregate.failed_probe_rows += summary.failed_probe_rows;
        aggregate.failed_endpoint_direction_scan_rows += failed_endpoint_direction_scan_rows;
        aggregate.run_dirs.push(summary.out_dir.clone());
        add_counts(&mut aggregate.degeneracy_counts, &summary.degeneracy_counts);
        add_counts(&mut aggregate.status_counts, &summary.status_counts);
        add_counts(
            &mut aggregate.trace_stop_reason_counts,
            &summary.trace_stop_reason_counts,
        );
        add_counts(
            &mut aggregate.trace_line_search_status_counts,
            &summary.trace_line_search_status_counts,
        );
        add_counts(
            &mut aggregate.endpoint_status_counts,
            &summary.endpoint_status_counts,
        );
        add_counts(
            &mut aggregate.endpoint_line_search_status_counts,
            &summary.endpoint_line_search_status_counts,
        );
        if let Some(counts) = &summary.endpoint_direction_scan_status_counts {
            add_counts(&mut aggregate.endpoint_direction_scan_status_counts, counts);
        }
        add_counts(
            &mut aggregate.endpoint_direction_scan_threshold_counts,
            &endpoint_direction_scan_threshold_counts,
        );
        let budget_label = budget_label_from_degeneracy_counts(&summary.degeneracy_counts);

        run_rows.push(RunSummaryRow {
            run_dir: run_dir.display().to_string(),
            method: summary.method,
            selection_threshold_relative: summary.selection_threshold_relative,
            max_fixtures_per_label: summary.max_fixtures_per_label,
            skip_fixtures_per_label: summary.skip_fixtures_per_label,
            degeneracy_labels: summary.degeneracy_labels,
            selected_fixtures: summary.selected_fixtures,
            probe_rows: summary.probe_rows,
            run_trace_rows: summary.run_trace_rows,
            endpoint_diagnostic_rows: summary.endpoint_diagnostic_rows,
            endpoint_direction_scan_rows,
            failed_probe_rows: summary.failed_probe_rows,
            failed_endpoint_direction_scan_rows,
            degeneracy_counts: summary.degeneracy_counts,
            trace_stop_reason_counts: summary.trace_stop_reason_counts,
            endpoint_status_counts: summary.endpoint_status_counts,
            endpoint_direction_scan_threshold_counts,
            caveat: summary.caveat,
        });

        if let Some(budget) =
            read_optional_json::<ComputeBudgetReport>(&run_dir.join("compute-budget-report.json"))
        {
            let endpoint_direction_scan_rows = budget.endpoint_direction_scan_rows.unwrap_or(0);
            let base_orbit_iterations = budget.base_orbit_iterations.unwrap_or(0);
            let target_orbit_iterations = budget.target_orbit_iterations.unwrap_or(0);
            let trace_base_orbit_iterations = budget.trace_base_orbit_iterations.unwrap_or(0);
            let trace_target_orbit_iterations = budget.trace_target_orbit_iterations.unwrap_or(0);
            let endpoint_base_orbit_iterations = budget.endpoint_base_orbit_iterations.unwrap_or(0);
            let endpoint_target_orbit_iterations =
                budget.endpoint_target_orbit_iterations.unwrap_or(0);
            let endpoint_scan_base_orbit_iterations =
                budget.endpoint_scan_base_orbit_iterations.unwrap_or(0);
            let endpoint_scan_target_orbit_iterations =
                budget.endpoint_scan_target_orbit_iterations.unwrap_or(0);
            let failed_endpoint_direction_scan_rows =
                budget.failed_endpoint_direction_scan_rows.unwrap_or(0);

            aggregate.budget_report_count += 1;
            aggregate.budget_selected_fixtures += budget.selected_fixtures;
            aggregate.budget_probe_rows += budget.probe_rows;
            aggregate.budget_run_trace_rows += budget.run_trace_rows;
            aggregate.budget_endpoint_diagnostic_rows += budget.endpoint_diagnostic_rows;
            aggregate.budget_endpoint_direction_scan_rows += endpoint_direction_scan_rows;
            aggregate.budget_base_orbit_iterations += base_orbit_iterations;
            aggregate.budget_target_orbit_iterations += target_orbit_iterations;
            aggregate.budget_trace_base_orbit_iterations += trace_base_orbit_iterations;
            aggregate.budget_trace_target_orbit_iterations += trace_target_orbit_iterations;
            aggregate.budget_endpoint_base_orbit_iterations += endpoint_base_orbit_iterations;
            aggregate.budget_endpoint_target_orbit_iterations += endpoint_target_orbit_iterations;
            aggregate.budget_endpoint_scan_base_orbit_iterations +=
                endpoint_scan_base_orbit_iterations;
            aggregate.budget_endpoint_scan_target_orbit_iterations +=
                endpoint_scan_target_orbit_iterations;
            aggregate.budget_failed_probe_rows += budget.failed_probe_rows;
            aggregate.budget_failed_endpoint_direction_scan_rows +=
                failed_endpoint_direction_scan_rows;
            aggregate.budget_elapsed_ms =
                add_optional_f64(aggregate.budget_elapsed_ms, budget.elapsed_ms);

            let label_totals = aggregate
                .budget_by_degeneracy_label
                .entry(budget_label)
                .or_default();
            label_totals.run_count += 1;
            label_totals.selected_fixtures += budget.selected_fixtures;
            label_totals.probe_rows += budget.probe_rows;
            label_totals.run_trace_rows += budget.run_trace_rows;
            label_totals.endpoint_diagnostic_rows += budget.endpoint_diagnostic_rows;
            label_totals.endpoint_direction_scan_rows += endpoint_direction_scan_rows;
            label_totals.base_orbit_iterations += base_orbit_iterations;
            label_totals.target_orbit_iterations += target_orbit_iterations;
            label_totals.trace_base_orbit_iterations += trace_base_orbit_iterations;
            label_totals.trace_target_orbit_iterations += trace_target_orbit_iterations;
            label_totals.endpoint_base_orbit_iterations += endpoint_base_orbit_iterations;
            label_totals.endpoint_target_orbit_iterations += endpoint_target_orbit_iterations;
            label_totals.endpoint_scan_base_orbit_iterations += endpoint_scan_base_orbit_iterations;
            label_totals.endpoint_scan_target_orbit_iterations +=
                endpoint_scan_target_orbit_iterations;
            label_totals.failed_probe_rows += budget.failed_probe_rows;
            label_totals.failed_endpoint_direction_scan_rows += failed_endpoint_direction_scan_rows;
            label_totals.elapsed_ms = add_optional_f64(label_totals.elapsed_ms, budget.elapsed_ms);

            budget_rows.push(BudgetSummaryRow {
                run_dir: run_dir.display().to_string(),
                command: budget.command,
                diagnostic_dir: budget.diagnostic_dir,
                polytope_table: budget.polytope_table,
                selection_threshold_relative: budget.selection_threshold_relative,
                max_fixtures_per_label: budget.max_fixtures_per_label,
                skip_fixtures_per_label: budget.skip_fixtures_per_label,
                degeneracy_labels: budget.degeneracy_labels,
                selected_fixtures: budget.selected_fixtures,
                probe_rows: budget.probe_rows,
                run_trace_rows: budget.run_trace_rows,
                endpoint_diagnostic_rows: budget.endpoint_diagnostic_rows,
                endpoint_direction_scan_rows,
                base_orbit_iterations,
                target_orbit_iterations,
                trace_base_orbit_iterations,
                trace_target_orbit_iterations,
                endpoint_base_orbit_iterations,
                endpoint_target_orbit_iterations,
                endpoint_scan_base_orbit_iterations,
                endpoint_scan_target_orbit_iterations,
                failed_probe_rows: budget.failed_probe_rows,
                failed_endpoint_direction_scan_rows,
                elapsed_ms: budget.elapsed_ms,
            });
        }
    }

    write_jsonl(cli.out_dir.join("run-summary.jsonl"), &run_rows)
        .expect("failed to write run-summary.jsonl");
    write_jsonl(cli.out_dir.join("budget-summary.jsonl"), &budget_rows)
        .expect("failed to write budget-summary.jsonl");
    write_json(cli.out_dir.join("aggregate-summary.json"), &aggregate)
        .expect("failed to write aggregate-summary.json");
    println!("{}", cli.out_dir.display());
}

fn add_counts(target: &mut BTreeMap<String, usize>, source: &BTreeMap<String, usize>) {
    for (key, value) in source {
        *target.entry(key.clone()).or_insert(0) += value;
    }
}

fn budget_label_from_degeneracy_counts(counts: &BTreeMap<String, usize>) -> String {
    let mut nonzero = counts.iter().filter(|(_, count)| **count > 0);
    let Some((label, _)) = nonzero.next() else {
        return "mixed_or_unknown".to_string();
    };
    if nonzero.next().is_some() {
        "mixed_or_unknown".to_string()
    } else {
        label.clone()
    }
}

fn add_optional_f64(lhs: Option<f64>, rhs: Option<f64>) -> Option<f64> {
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => Some(lhs + rhs),
        (Some(lhs), None) => Some(lhs),
        (None, Some(rhs)) => Some(rhs),
        (None, None) => None,
    }
}

fn parse_args() -> Cli {
    let mut out_dir = default_output_dir();
    let mut run_dirs = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(args.next().expect("--out-dir requires a path"));
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other if other.starts_with('-') => panic!("unsupported argument: {other}"),
            other => run_dirs.push(PathBuf::from(other)),
        }
    }

    if run_dirs.is_empty() {
        print_usage();
        panic!("at least one run directory is required");
    }

    Cli { out_dir, run_dirs }
}

fn print_usage() {
    eprintln!("Usage: dev-gradient-ascent-aggregate-summaries [--out-dir PATH] RUN_DIR...");
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-aggregate-summaries-{}-{stamp}",
        std::process::id()
    ))
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    serde_json::from_reader(file)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn read_optional_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Option<T> {
    if !path.exists() {
        return None;
    }
    Some(read_json(path))
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
