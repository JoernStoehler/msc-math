use exp_local_sys_methods::{
    default_base_candidate_action_gap, default_output_path,
    run_prediction_smoke_with_base_candidate_action_gap,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Default)]
struct BasepointSummary {
    rows: usize,
    successful: usize,
    switches: usize,
    target_outside_base_active: usize,
    target_outside_base_candidate_window: usize,
    max_abs_error: f64,
    active_orbit_counts: BTreeSet<usize>,
    base_candidate_orbit_counts: BTreeSet<usize>,
    base_candidate_action_gaps: BTreeSet<String>,
    max_active_q_error_bound: f64,
}

struct CliOptions {
    output_path: PathBuf,
    base_candidate_action_gap: f64,
}

fn parse_options() -> CliOptions {
    let mut args = std::env::args().skip(1);
    let mut output = PathBuf::from(default_output_path());
    let mut base_candidate_action_gap = default_base_candidate_action_gap();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(path) = args.next() else {
                    panic!("--output requires a path");
                };
                output = PathBuf::from(path);
            }
            "--base-candidate-action-gap" => {
                let Some(value) = args.next() else {
                    panic!("--base-candidate-action-gap requires a nonnegative finite f64");
                };
                base_candidate_action_gap = value
                    .parse::<f64>()
                    .expect("--base-candidate-action-gap requires a nonnegative finite f64");
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: local-sys-prediction-smoke [--output <path>] \
                     [--base-candidate-action-gap <f64>]\n\
                     Default output: {}\n\
                     Default base candidate action gap: {:.3e}",
                    default_output_path(),
                    default_base_candidate_action_gap()
                );
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    if !base_candidate_action_gap.is_finite() || base_candidate_action_gap < 0.0 {
        panic!("--base-candidate-action-gap requires a nonnegative finite f64");
    }
    CliOptions {
        output_path: output,
        base_candidate_action_gap,
    }
}

fn main() {
    let options = parse_options();
    let rows = run_prediction_smoke_with_base_candidate_action_gap(
        &options.output_path,
        options.base_candidate_action_gap,
    )
    .unwrap_or_else(|err| panic!("local sys prediction smoke failed: {err:?}"));
    let successful = rows.iter().filter(|row| row.status == "ok").count();
    let generic_success = rows
        .iter()
        .any(|row| row.basepoint_name.starts_with("random_f10") && row.status == "ok");
    println!(
        "local-sys-prediction-smoke: wrote {} rows to {}",
        rows.len(),
        options.output_path.display()
    );
    println!("  successful rows: {successful}");
    println!("  generic basepoint success: {generic_success}");
    for (basepoint, summary) in summarize_by_basepoint(&rows) {
        println!(
            "  {basepoint}: ok {}/{}, switches {}, target outside base active {}, \
             target outside base candidate window {}, max abs error {:.3e}, \
             active orbit counts {:?}, base candidate counts {:?}, \
             base candidate gaps {:?}, \
             max active q error {:.3e}",
            summary.successful,
            summary.rows,
            summary.switches,
            summary.target_outside_base_active,
            summary.target_outside_base_candidate_window,
            summary.max_abs_error,
            summary.active_orbit_counts,
            summary.base_candidate_orbit_counts,
            summary.base_candidate_action_gaps,
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
        if row.target_best_sigma_in_base_candidate_window == Some(false) {
            summary.target_outside_base_candidate_window += 1;
        }
        if let Some(error) = row.abs_prediction_error {
            summary.max_abs_error = summary.max_abs_error.max(error);
        }
        summary.active_orbit_counts.insert(row.active_orbit_count);
        summary
            .base_candidate_orbit_counts
            .insert(row.base_candidate_orbit_count);
        summary
            .base_candidate_action_gaps
            .insert(format!("{:.3e}", row.base_candidate_action_gap));
        summary.max_active_q_error_bound = summary
            .max_active_q_error_bound
            .max(row.active_max_q_error_bound);
    }
    summaries
}
