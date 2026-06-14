mod args;
mod output;

#[path = "../scan/input/mod.rs"]
mod input;

use exp_dev_f64_capacity::{capacity_f64_only, classify_report, F64CapacityOutcome, ScanCase};
use output::{write_benchmark_rows, BenchmarkRow};
use std::time::Instant;

fn main() {
    let args = args::parse_args();
    let options = input::LoadCaseOptions {
        input_source: input::InputSource::Artifacts,
        max_rows_per_family: args.max_rows_per_family,
        generated_samples_per_facet: 0,
        generated_seed: 0,
        family_filter: Vec::new(),
        source_id_filter: Vec::new(),
    };
    let cases = input::load_cases(&options);
    let rows = cases
        .into_iter()
        .map(|case| benchmark_case(case, args.repetitions))
        .collect::<Vec<_>>();
    write_benchmark_rows(&args.output, &rows);
    eprintln!(
        "wrote {} benchmark rows to {}",
        rows.len(),
        args.output.display()
    );
}

fn benchmark_case(case: ScanCase, repetitions: usize) -> BenchmarkRow {
    let mut times = Vec::with_capacity(repetitions);
    let mut last_report = None;
    for _ in 0..repetitions {
        let started = Instant::now();
        let report = capacity_f64_only(&case.dual_vertices);
        times.push(started.elapsed().as_secs_f64() * 1000.0);
        last_report = Some(report);
    }
    times.sort_by(f64::total_cmp);

    let report = last_report.expect("benchmark repetitions must be positive");
    let f64_capacity = capacity(&report.outcome);
    let (abs_action_error, rel_action_error) = match (f64_capacity, case.audit_capacity_label) {
        (Some(actual), Some(audit_label)) => {
            let abs = (actual - audit_label).abs();
            let rel = abs / audit_label.abs().max(1.0);
            (Some(abs), Some(rel))
        }
        _ => (None, None),
    };
    let classification = classify_report(
        &report,
        case.audit_capacity_label,
        abs_action_error,
        rel_action_error,
    );

    BenchmarkRow {
        family: case.family,
        source_id: case.source_id,
        facet_count: case.dual_vertices.len(),
        repetitions,
        f64_min_ms: *times.first().expect("nonempty timing list"),
        f64_median_ms: times[times.len() / 2],
        f64_max_ms: *times.last().expect("nonempty timing list"),
        f64_capacity,
        audit_capacity_label: case.audit_capacity_label,
        artifact_capacity_label: case.artifact_capacity_label,
        abs_action_error,
        rel_action_error,
        outcome: outcome_label(&report.outcome).to_string(),
        agreement_status: classification.agreement_status.label().to_string(),
        trust_class: classification.trust_class.label().to_string(),
        trust_reasons: classification.trust_reasons,
        sigma_count: report.sigma_count,
        exact_recompute_status: "not_attempted".to_string(),
    }
}

fn capacity(outcome: &F64CapacityOutcome) -> Option<f64> {
    match outcome {
        F64CapacityOutcome::Success { capacity, .. } => Some(*capacity),
        F64CapacityOutcome::Failure { .. } => None,
    }
}

fn outcome_label(outcome: &F64CapacityOutcome) -> &'static str {
    match outcome {
        F64CapacityOutcome::Success { .. } => "success",
        F64CapacityOutcome::Failure { .. } => "failure",
    }
}
