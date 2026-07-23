//! Summarize endpoint direction-scan rows from split local-geometry runs.
//!
//! This command inspects `endpoint-direction-scan.jsonl` and
//! `endpoint-diagnostic.jsonl` from complete local-geometry runs. It quantifies
//! the positive-below-threshold caveat that remains after the endpoint
//! diagnostic stops.

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
struct EndpointDiagnosticRow {
    poly_id: String,
    effective_min_observed_delta: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct EndpointDirectionScanRow {
    poly_id: String,
    degeneracy_label: String,
    direction_label: String,
    step: f64,
    status: String,
    base_sys: f64,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
struct BestPositiveRow {
    run_dir: String,
    poly_id: String,
    degeneracy_label: String,
    direction_label: String,
    step: f64,
    base_sys: f64,
    effective_min_observed_delta: Option<f64>,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: f64,
    observed_delta_relative_to_threshold: Option<f64>,
    observed_delta_relative_to_base_sys: f64,
}

#[derive(Clone, Debug, Default, Serialize)]
struct EndpointScanTotals {
    run_count: usize,
    total_rows: usize,
    ok_rows: usize,
    failed_rows: usize,
    above_threshold_rows: usize,
    positive_below_threshold_rows: usize,
    nonpositive_rows: usize,
    missing_observed_delta_rows: usize,
    missing_threshold_rows: usize,
    best_positive_by_delta: Option<BestPositiveRow>,
    best_positive_by_threshold_ratio: Option<BestPositiveRow>,
}

#[derive(Serialize)]
struct RunEndpointScanReport {
    run_dir: String,
    source_summary_out_dir: String,
    degeneracy_counts: BTreeMap<String, usize>,
    totals: EndpointScanTotals,
}

#[derive(Serialize)]
struct EndpointScanSummary {
    command: String,
    run_count: usize,
    totals: EndpointScanTotals,
    totals_by_degeneracy_label: BTreeMap<String, EndpointScanTotals>,
    run_dirs: Vec<String>,
    artifact_files: Vec<String>,
    caveat: String,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");

    let mut run_reports = Vec::new();
    let mut summary = EndpointScanSummary {
        command: "dev-gradient-ascent-endpoint-scan-report".to_string(),
        run_count: 0,
        totals: EndpointScanTotals::default(),
        totals_by_degeneracy_label: BTreeMap::new(),
        run_dirs: Vec::new(),
        artifact_files: vec![
            "run-endpoint-scan-report.jsonl".to_string(),
            "endpoint-scan-summary.json".to_string(),
        ],
        caveat: "finite endpoint direction-scan summary; this does not certify endpoint local maximality".to_string(),
    };

    for run_dir in &cli.run_dirs {
        let local_summary: LocalGeometrySummary = read_json(&run_dir.join("summary.json"));
        let endpoint_rows: Vec<EndpointDiagnosticRow> =
            load_jsonl(&run_dir.join("endpoint-diagnostic.jsonl"));
        let endpoint_thresholds: BTreeMap<String, f64> = endpoint_rows
            .into_iter()
            .filter_map(|row| {
                row.effective_min_observed_delta
                    .map(|delta| (row.poly_id, delta))
            })
            .collect();
        let scan_rows: Vec<EndpointDirectionScanRow> =
            load_jsonl(&run_dir.join("endpoint-direction-scan.jsonl"));
        let mut totals = EndpointScanTotals::default();
        totals.run_count = 1;

        for row in &scan_rows {
            update_totals_from_row(
                &mut totals,
                run_dir,
                row,
                endpoint_thresholds.get(&row.poly_id).copied(),
            );
        }

        let budget_label = budget_label_from_degeneracy_counts(&local_summary.degeneracy_counts);
        add_totals(&mut summary.totals, &totals);
        add_totals(
            summary
                .totals_by_degeneracy_label
                .entry(budget_label)
                .or_default(),
            &totals,
        );
        summary.run_count += 1;
        summary.run_dirs.push(local_summary.out_dir.clone());

        run_reports.push(RunEndpointScanReport {
            run_dir: run_dir.display().to_string(),
            source_summary_out_dir: local_summary.out_dir,
            degeneracy_counts: local_summary.degeneracy_counts,
            totals,
        });
    }

    write_jsonl(
        cli.out_dir.join("run-endpoint-scan-report.jsonl"),
        &run_reports,
    )
    .expect("failed to write run-endpoint-scan-report.jsonl");
    write_json(cli.out_dir.join("endpoint-scan-summary.json"), &summary)
        .expect("failed to write endpoint-scan-summary.json");
    println!("{}", cli.out_dir.display());
}

fn update_totals_from_row(
    totals: &mut EndpointScanTotals,
    run_dir: &Path,
    row: &EndpointDirectionScanRow,
    threshold: Option<f64>,
) {
    totals.total_rows += 1;
    if row.status.as_str() != "ok" {
        totals.failed_rows += 1;
        return;
    }
    totals.ok_rows += 1;

    let Some(delta) = row.observed_delta_sys else {
        totals.missing_observed_delta_rows += 1;
        return;
    };
    if delta <= 0.0 {
        totals.nonpositive_rows += 1;
        return;
    }

    match threshold {
        Some(threshold) if delta > threshold => totals.above_threshold_rows += 1,
        Some(_) => totals.positive_below_threshold_rows += 1,
        None => totals.missing_threshold_rows += 1,
    }

    let candidate = BestPositiveRow {
        run_dir: run_dir.display().to_string(),
        poly_id: row.poly_id.clone(),
        degeneracy_label: row.degeneracy_label.clone(),
        direction_label: row.direction_label.clone(),
        step: row.step,
        base_sys: row.base_sys,
        effective_min_observed_delta: threshold,
        predicted_delta_sys: row.predicted_delta_sys,
        observed_delta_sys: delta,
        observed_delta_relative_to_threshold: threshold
            .filter(|threshold| *threshold > 0.0)
            .map(|threshold| delta / threshold),
        observed_delta_relative_to_base_sys: delta / row.base_sys.abs(),
    };
    replace_if_larger_delta(&mut totals.best_positive_by_delta, &candidate);
    replace_if_larger_threshold_ratio(&mut totals.best_positive_by_threshold_ratio, &candidate);
}

fn add_totals(target: &mut EndpointScanTotals, source: &EndpointScanTotals) {
    target.run_count += source.run_count;
    target.total_rows += source.total_rows;
    target.ok_rows += source.ok_rows;
    target.failed_rows += source.failed_rows;
    target.above_threshold_rows += source.above_threshold_rows;
    target.positive_below_threshold_rows += source.positive_below_threshold_rows;
    target.nonpositive_rows += source.nonpositive_rows;
    target.missing_observed_delta_rows += source.missing_observed_delta_rows;
    target.missing_threshold_rows += source.missing_threshold_rows;
    if let Some(candidate) = &source.best_positive_by_delta {
        replace_if_larger_delta(&mut target.best_positive_by_delta, candidate);
    }
    if let Some(candidate) = &source.best_positive_by_threshold_ratio {
        replace_if_larger_threshold_ratio(&mut target.best_positive_by_threshold_ratio, candidate);
    }
}

fn replace_if_larger_delta(target: &mut Option<BestPositiveRow>, candidate: &BestPositiveRow) {
    let should_replace = target
        .as_ref()
        .is_none_or(|current| candidate.observed_delta_sys > current.observed_delta_sys);
    if should_replace {
        *target = Some(candidate.clone());
    }
}

fn replace_if_larger_threshold_ratio(
    target: &mut Option<BestPositiveRow>,
    candidate: &BestPositiveRow,
) {
    let Some(candidate_ratio) = candidate.observed_delta_relative_to_threshold else {
        return;
    };
    let should_replace = target.as_ref().is_none_or(|current| {
        current
            .observed_delta_relative_to_threshold
            .is_none_or(|current_ratio| candidate_ratio > current_ratio)
    });
    if should_replace {
        *target = Some(candidate.clone());
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
    eprintln!("Usage: dev-gradient-ascent-endpoint-scan-report [--out-dir PATH] RUN_DIR...");
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-endpoint-scan-report-{}-{stamp}",
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
