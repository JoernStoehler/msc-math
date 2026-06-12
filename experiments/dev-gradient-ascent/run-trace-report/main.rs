//! Summarize run traces from split local-geometry runs.
//!
//! This command inspects `run-trace.jsonl` from complete local-geometry runs.
//! It records where the retained observed multi-direction policy mattered:
//! accepted negative-predicted steps and accepted steps after earlier candidate
//! directions were rejected.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Cli {
    out_dir: PathBuf,
    run_dirs: Vec<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
struct LocalGeometrySummary {
    degeneracy_counts: BTreeMap<String, usize>,
    out_dir: String,
}

#[derive(Clone, Debug, Deserialize)]
struct RunTraceRow {
    poly_id: String,
    degeneracy_label: String,
    iteration: usize,
    chosen_direction_label: Option<String>,
    attempted_direction_labels: Vec<String>,
    rejected_direction_labels: Vec<String>,
    line_search_attempts: usize,
    rejected_steps: Vec<f64>,
    line_search_status: String,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
    base_sys: f64,
    target_sys: Option<f64>,
    accepted: bool,
    stop_reason: String,
}

#[derive(Default, Serialize)]
struct TraceTotals {
    run_count: usize,
    rows: usize,
    accepted_rows: usize,
    stopped_rows: usize,
    accepted_negative_predicted_rows: usize,
    accepted_after_rejected_direction_rows: usize,
    accepted_with_branch_switching_signal_rows: usize,
    accepted_by_direction: BTreeMap<String, usize>,
    stop_reason_counts: BTreeMap<String, usize>,
    line_search_status_counts: BTreeMap<String, usize>,
    attempted_direction_counts: BTreeMap<String, usize>,
    rejected_direction_counts: BTreeMap<String, usize>,
    total_line_search_attempts: usize,
    total_rejected_steps: usize,
    max_line_search_attempts: usize,
    max_observed_delta_sys: Option<f64>,
}

#[derive(Serialize)]
struct RunTraceReport {
    run_dir: String,
    source_summary_out_dir: String,
    degeneracy_counts: BTreeMap<String, usize>,
    totals: TraceTotals,
}

#[derive(Serialize)]
struct RunTraceSummary {
    command: String,
    run_count: usize,
    totals: TraceTotals,
    totals_by_degeneracy_label: BTreeMap<String, TraceTotals>,
    run_dirs: Vec<String>,
    artifact_files: Vec<String>,
    caveat: String,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");

    let mut run_reports = Vec::new();
    let mut summary = RunTraceSummary {
        command: "dev-gradient-ascent-run-trace-report".to_string(),
        run_count: 0,
        totals: TraceTotals::default(),
        totals_by_degeneracy_label: BTreeMap::new(),
        run_dirs: Vec::new(),
        artifact_files: vec![
            "run-trace-report.jsonl".to_string(),
            "run-trace-summary.json".to_string(),
        ],
        caveat: "run-trace summary only; this reports optimizer behavior but does not certify endpoint local maximality".to_string(),
    };

    for run_dir in &cli.run_dirs {
        let local_summary: LocalGeometrySummary = read_json(&run_dir.join("summary.json"));
        let rows: Vec<RunTraceRow> = load_jsonl(&run_dir.join("run-trace.jsonl"));
        let mut totals = TraceTotals::default();
        totals.run_count = 1;
        for row in &rows {
            update_trace_totals(&mut totals, row);
        }

        let budget_label = budget_label_from_degeneracy_counts(&local_summary.degeneracy_counts);
        add_trace_totals(&mut summary.totals, &totals);
        add_trace_totals(
            summary
                .totals_by_degeneracy_label
                .entry(budget_label)
                .or_default(),
            &totals,
        );
        summary.run_count += 1;
        summary.run_dirs.push(local_summary.out_dir.clone());

        run_reports.push(RunTraceReport {
            run_dir: run_dir.display().to_string(),
            source_summary_out_dir: local_summary.out_dir,
            degeneracy_counts: local_summary.degeneracy_counts,
            totals,
        });
    }

    write_jsonl(cli.out_dir.join("run-trace-report.jsonl"), &run_reports)
        .expect("failed to write run-trace-report.jsonl");
    write_json(cli.out_dir.join("run-trace-summary.json"), &summary)
        .expect("failed to write run-trace-summary.json");
    println!("{}", cli.out_dir.display());
}

fn update_trace_totals(totals: &mut TraceTotals, row: &RunTraceRow) {
    totals.rows += 1;
    totals.total_line_search_attempts += row.line_search_attempts;
    totals.total_rejected_steps += row.rejected_steps.len();
    totals.max_line_search_attempts = totals
        .max_line_search_attempts
        .max(row.line_search_attempts);
    *totals
        .stop_reason_counts
        .entry(row.stop_reason.clone())
        .or_insert(0) += 1;
    *totals
        .line_search_status_counts
        .entry(row.line_search_status.clone())
        .or_insert(0) += 1;
    for direction in &row.attempted_direction_labels {
        *totals
            .attempted_direction_counts
            .entry(direction.clone())
            .or_insert(0) += 1;
    }
    for direction in &row.rejected_direction_labels {
        *totals
            .rejected_direction_counts
            .entry(direction.clone())
            .or_insert(0) += 1;
    }
    if !row.accepted {
        totals.stopped_rows += 1;
        return;
    }

    totals.accepted_rows += 1;
    if let Some(direction) = &row.chosen_direction_label {
        *totals
            .accepted_by_direction
            .entry(direction.clone())
            .or_insert(0) += 1;
    }
    if row.predicted_delta_sys.is_some_and(|delta| delta < 0.0) {
        totals.accepted_negative_predicted_rows += 1;
    }
    if !row.rejected_direction_labels.is_empty() {
        totals.accepted_after_rejected_direction_rows += 1;
    }
    if row
        .target_sys
        .is_some_and(|target_sys| target_sys > row.base_sys)
        && row
            .observed_delta_sys
            .zip(row.predicted_delta_sys)
            .is_some_and(|(observed, predicted)| observed > 0.0 && predicted < 0.0)
    {
        totals.accepted_with_branch_switching_signal_rows += 1;
    }
    if let Some(delta) = row.observed_delta_sys {
        totals.max_observed_delta_sys = Some(match totals.max_observed_delta_sys {
            Some(current) => current.max(delta),
            None => delta,
        });
    }
}

fn add_trace_totals(target: &mut TraceTotals, source: &TraceTotals) {
    target.run_count += source.run_count;
    target.rows += source.rows;
    target.accepted_rows += source.accepted_rows;
    target.stopped_rows += source.stopped_rows;
    target.accepted_negative_predicted_rows += source.accepted_negative_predicted_rows;
    target.accepted_after_rejected_direction_rows += source.accepted_after_rejected_direction_rows;
    target.accepted_with_branch_switching_signal_rows +=
        source.accepted_with_branch_switching_signal_rows;
    add_counts(
        &mut target.accepted_by_direction,
        &source.accepted_by_direction,
    );
    add_counts(&mut target.stop_reason_counts, &source.stop_reason_counts);
    add_counts(
        &mut target.line_search_status_counts,
        &source.line_search_status_counts,
    );
    add_counts(
        &mut target.attempted_direction_counts,
        &source.attempted_direction_counts,
    );
    add_counts(
        &mut target.rejected_direction_counts,
        &source.rejected_direction_counts,
    );
    target.total_line_search_attempts += source.total_line_search_attempts;
    target.total_rejected_steps += source.total_rejected_steps;
    target.max_line_search_attempts = target
        .max_line_search_attempts
        .max(source.max_line_search_attempts);
    if let Some(delta) = source.max_observed_delta_sys {
        target.max_observed_delta_sys = Some(match target.max_observed_delta_sys {
            Some(current) => current.max(delta),
            None => delta,
        });
    }
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
    eprintln!("Usage: dev-gradient-ascent-run-trace-report [--out-dir PATH] RUN_DIR...");
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-run-trace-report-{}-{stamp}",
        std::process::id()
    ))
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap_or_else(|err| {
                panic!("failed to read {}:{}: {err}", path.display(), idx + 1)
            });
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("failed to parse {}:{}: {err}", path.display(), idx + 1)
                })
            })
        })
        .collect()
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    serde_json::from_reader(file)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
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
