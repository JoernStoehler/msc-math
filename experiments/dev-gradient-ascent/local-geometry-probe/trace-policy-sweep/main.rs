//! Reclassify local-geometry trace and endpoint rows under stop-threshold policies.
//!
//! This command consumes an existing `dev-gradient-ascent-local-geometry-probe`
//! output directory. It does not replay ascent paths or recompute `sys(a)`.
//! It only asks how already-observed finite probe deltas are labeled under
//! different absolute/relative stop thresholds.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_ABSOLUTE_THRESHOLDS: &[f64] = &[0.0, 1.0e-4, 1.0e-3];
const DEFAULT_RELATIVE_THRESHOLDS: &[f64] = &[0.0, 1.0e-3];

#[derive(Debug)]
struct Cli {
    geometry_dir: PathBuf,
    out_dir: PathBuf,
    absolute_thresholds: Vec<f64>,
    relative_thresholds: Vec<f64>,
}

#[derive(Clone, Copy, Debug)]
struct Policy {
    absolute_delta: f64,
    relative_delta: f64,
}

impl Policy {
    fn label(self) -> String {
        format!(
            "abs={:.3e};rel={:.3e}",
            self.absolute_delta, self.relative_delta
        )
    }

    fn effective_delta(self, base_sys: f64) -> f64 {
        self.absolute_delta
            .max(self.relative_delta * base_sys.abs())
    }
}

#[derive(Debug, Deserialize)]
struct RunTraceRow {
    degeneracy_label: String,
    base_sys: f64,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct EndpointDiagnosticRow {
    degeneracy_label: String,
    final_sys: Option<f64>,
    post_stop_predicted_delta_sys: Option<f64>,
    post_stop_observed_delta_sys: Option<f64>,
}

#[derive(Default)]
struct Counts {
    rows: usize,
    positive_predicted_rows: usize,
    observed_rows: usize,
    above_threshold_rows: usize,
    positive_below_threshold_rows: usize,
    nonpositive_observed_rows: usize,
    missing_observed_rows: usize,
}

#[derive(Serialize)]
struct PolicySweepRow {
    source_file: String,
    policy_label: String,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
    degeneracy_label: String,
    rows: usize,
    positive_predicted_rows: usize,
    observed_rows: usize,
    above_threshold_rows: usize,
    positive_below_threshold_rows: usize,
    nonpositive_observed_rows: usize,
    missing_observed_rows: usize,
    caveat: String,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    geometry_dir: String,
    out_dir: String,
    policies: usize,
    run_trace_rows: usize,
    endpoint_rows: usize,
    sweep_rows: usize,
    caveat: String,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");

    let run_trace_rows: Vec<RunTraceRow> = load_jsonl(&cli.geometry_dir.join("run-trace.jsonl"));
    let endpoint_rows: Vec<EndpointDiagnosticRow> =
        load_jsonl(&cli.geometry_dir.join("endpoint-diagnostic.jsonl"));
    let policies = policy_grid(&cli);

    let mut rows = Vec::new();
    for policy in &policies {
        append_run_trace_sweep_rows(&mut rows, *policy, &run_trace_rows);
        append_endpoint_sweep_rows(&mut rows, *policy, &endpoint_rows);
    }

    write_jsonl(cli.out_dir.join("trace-policy-sweep.jsonl"), &rows)
        .expect("failed to write trace-policy-sweep.jsonl");
    let summary = Summary {
        method: "dev-gradient-ascent-trace-policy-sweep".to_string(),
        geometry_dir: cli.geometry_dir.display().to_string(),
        out_dir: cli.out_dir.display().to_string(),
        policies: policies.len(),
        run_trace_rows: run_trace_rows.len(),
        endpoint_rows: endpoint_rows.len(),
        sweep_rows: rows.len(),
        caveat: "reclassifies observed finite probes only; does not replay optimizer paths"
            .to_string(),
    };
    write_json(cli.out_dir.join("summary.json"), &summary).expect("failed to write summary.json");

    println!("{}", cli.out_dir.display());
}

fn append_run_trace_sweep_rows(
    output: &mut Vec<PolicySweepRow>,
    policy: Policy,
    rows: &[RunTraceRow],
) {
    let mut counts_by_label: BTreeMap<String, Counts> = BTreeMap::new();
    for row in rows {
        let counts = counts_by_label
            .entry(row.degeneracy_label.clone())
            .or_default();
        classify_observation(
            counts,
            policy,
            row.base_sys,
            row.predicted_delta_sys,
            row.observed_delta_sys,
        );
    }
    append_counts(output, "run-trace.jsonl", policy, counts_by_label);
}

fn append_endpoint_sweep_rows(
    output: &mut Vec<PolicySweepRow>,
    policy: Policy,
    rows: &[EndpointDiagnosticRow],
) {
    let mut counts_by_label: BTreeMap<String, Counts> = BTreeMap::new();
    for row in rows {
        let counts = counts_by_label
            .entry(row.degeneracy_label.clone())
            .or_default();
        match row.final_sys {
            Some(final_sys) => classify_observation(
                counts,
                policy,
                final_sys,
                row.post_stop_predicted_delta_sys,
                row.post_stop_observed_delta_sys,
            ),
            None => {
                counts.rows += 1;
                counts.missing_observed_rows += 1;
            }
        }
    }
    append_counts(output, "endpoint-diagnostic.jsonl", policy, counts_by_label);
}

fn classify_observation(
    counts: &mut Counts,
    policy: Policy,
    base_sys: f64,
    predicted_delta: Option<f64>,
    observed_delta: Option<f64>,
) {
    counts.rows += 1;
    if predicted_delta.is_some_and(|delta| delta > 0.0) {
        counts.positive_predicted_rows += 1;
    }
    let Some(observed_delta) = observed_delta else {
        counts.missing_observed_rows += 1;
        return;
    };
    counts.observed_rows += 1;
    if observed_delta > policy.effective_delta(base_sys) {
        counts.above_threshold_rows += 1;
    } else if observed_delta > 0.0 {
        counts.positive_below_threshold_rows += 1;
    } else {
        counts.nonpositive_observed_rows += 1;
    }
}

fn append_counts(
    output: &mut Vec<PolicySweepRow>,
    source_file: &str,
    policy: Policy,
    counts_by_label: BTreeMap<String, Counts>,
) {
    for (degeneracy_label, counts) in counts_by_label {
        output.push(PolicySweepRow {
            source_file: source_file.to_string(),
            policy_label: policy.label(),
            min_observed_delta: policy.absolute_delta,
            min_observed_relative_delta: policy.relative_delta,
            degeneracy_label,
            rows: counts.rows,
            positive_predicted_rows: counts.positive_predicted_rows,
            observed_rows: counts.observed_rows,
            above_threshold_rows: counts.above_threshold_rows,
            positive_below_threshold_rows: counts.positive_below_threshold_rows,
            nonpositive_observed_rows: counts.nonpositive_observed_rows,
            missing_observed_rows: counts.missing_observed_rows,
            caveat: "threshold reclassification of already-observed finite probes".to_string(),
        });
    }
}

fn policy_grid(cli: &Cli) -> Vec<Policy> {
    let mut policies = Vec::new();
    for &absolute_delta in &cli.absolute_thresholds {
        for &relative_delta in &cli.relative_thresholds {
            policies.push(Policy {
                absolute_delta,
                relative_delta,
            });
        }
    }
    policies
}

fn parse_args() -> Cli {
    let mut cli = Cli {
        geometry_dir: PathBuf::new(),
        out_dir: default_output_dir(),
        absolute_thresholds: DEFAULT_ABSOLUTE_THRESHOLDS.to_vec(),
        relative_thresholds: DEFAULT_RELATIVE_THRESHOLDS.to_vec(),
    };

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--geometry-dir" => {
                cli.geometry_dir =
                    PathBuf::from(args.next().expect("--geometry-dir requires a path"));
            }
            "--out-dir" => {
                cli.out_dir = PathBuf::from(args.next().expect("--out-dir requires a path"));
            }
            "--absolute-thresholds" => {
                cli.absolute_thresholds = parse_csv_f64(
                    &args
                        .next()
                        .expect("--absolute-thresholds requires comma-separated f64 values"),
                    "--absolute-thresholds",
                );
            }
            "--relative-thresholds" => {
                cli.relative_thresholds = parse_csv_f64(
                    &args
                        .next()
                        .expect("--relative-thresholds requires comma-separated f64 values"),
                    "--relative-thresholds",
                );
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }

    if cli.geometry_dir.as_os_str().is_empty() {
        print_usage();
        panic!("--geometry-dir is required");
    }
    cli
}

fn parse_csv_f64(input: &str, argument_name: &str) -> Vec<f64> {
    input
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("{argument_name} entries must be f64"))
        })
        .collect()
}

fn print_usage() {
    eprintln!(
        "Usage: dev-gradient-ascent-trace-policy-sweep --geometry-dir PATH \
         [--out-dir PATH] [--absolute-thresholds CSV] [--relative-thresholds CSV]"
    );
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-trace-policy-sweep-{}-{stamp}",
        std::process::id()
    ))
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line =
                line.unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
            serde_json::from_str(&line)
                .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
        })
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
    serde_json::to_writer_pretty(BufWriter::new(file), value)?;
    Ok(())
}
