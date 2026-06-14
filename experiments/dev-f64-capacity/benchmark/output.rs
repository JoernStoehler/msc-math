use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct BenchmarkRow {
    pub(crate) family: String,
    pub(crate) source_id: String,
    pub(crate) facet_count: usize,
    pub(crate) repetitions: usize,
    pub(crate) f64_min_ms: f64,
    pub(crate) f64_median_ms: f64,
    pub(crate) f64_max_ms: f64,
    pub(crate) f64_capacity: Option<f64>,
    pub(crate) audit_capacity_label: Option<f64>,
    pub(crate) artifact_capacity_label: Option<f64>,
    pub(crate) abs_action_error: Option<f64>,
    pub(crate) rel_action_error: Option<f64>,
    pub(crate) outcome: String,
    pub(crate) agreement_status: String,
    pub(crate) trust_class: String,
    pub(crate) trust_reasons: Vec<String>,
    pub(crate) sigma_count: u64,
    pub(crate) exact_recompute_status: String,
}

pub(crate) fn write_benchmark_rows(output: &Path, rows: &[BenchmarkRow]) {
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create benchmark output directory");
        }
    }

    let file = File::create(output).expect("create benchmark JSONL");
    let mut writer = BufWriter::new(file);
    for row in rows {
        let line = serde_json::to_string(row).expect("serialize benchmark row");
        writeln!(writer, "{line}").expect("write benchmark row");
    }
    writer.flush().expect("flush benchmark JSONL");
}
