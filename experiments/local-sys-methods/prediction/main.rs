use exp_local_sys_methods::{default_output_path, run_prediction_smoke};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Default)]
struct BasepointSummary {
    rows: usize,
    successful: usize,
    switches: usize,
    target_outside_base_active: usize,
    max_abs_error: f64,
    active_orbit_counts: BTreeSet<usize>,
    max_active_q_error_bound: f64,
}

fn parse_output_path() -> PathBuf {
    let mut args = std::env::args().skip(1);
    let mut output = PathBuf::from(default_output_path());
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(path) = args.next() else {
                    panic!("--output requires a path");
                };
                output = PathBuf::from(path);
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: local-sys-prediction-smoke [--output <path>]\n\
                     Default output: {}",
                    default_output_path()
                );
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    output
}

fn main() {
    let output_path = parse_output_path();
    let rows = run_prediction_smoke(&output_path)
        .unwrap_or_else(|err| panic!("local sys prediction smoke failed: {err:?}"));
    let successful = rows.iter().filter(|row| row.status == "ok").count();
    let generic_success = rows
        .iter()
        .any(|row| row.basepoint_name.starts_with("random_f10") && row.status == "ok");
    println!(
        "local-sys-prediction-smoke: wrote {} rows to {}",
        rows.len(),
        output_path.display()
    );
    println!("  successful rows: {successful}");
    println!("  generic basepoint success: {generic_success}");
    for (basepoint, summary) in summarize_by_basepoint(&rows) {
        println!(
            "  {basepoint}: ok {}/{}, switches {}, target outside base active {}, \
             max abs error {:.3e}, active orbit counts {:?}, max active q error {:.3e}",
            summary.successful,
            summary.rows,
            summary.switches,
            summary.target_outside_base_active,
            summary.max_abs_error,
            summary.active_orbit_counts,
            summary.max_active_q_error_bound,
        );
    }
    if successful == 0 || !generic_success {
        eprintln!("local-sys-prediction-smoke did not produce a successful generic row");
        std::process::exit(2);
    }
}

fn summarize_by_basepoint(
    rows: &[exp_local_sys_methods::PredictionRow],
) -> BTreeMap<String, BasepointSummary> {
    let mut summaries = BTreeMap::new();
    for row in rows {
        let summary = summaries
            .entry(row.basepoint_name.clone())
            .or_insert_with(BasepointSummary::default);
        summary.rows += 1;
        if row.status == "ok" {
            summary.successful += 1;
        }
        if row.best_sigma_changed == Some(true) {
            summary.switches += 1;
        }
        if row.target_best_sigma_in_base_active_set == Some(false) {
            summary.target_outside_base_active += 1;
        }
        if let Some(error) = row.abs_prediction_error {
            summary.max_abs_error = summary.max_abs_error.max(error);
        }
        summary.active_orbit_counts.insert(row.active_orbit_count);
        summary.max_active_q_error_bound = summary
            .max_active_q_error_bound
            .max(row.active_max_q_error_bound);
    }
    summaries
}
