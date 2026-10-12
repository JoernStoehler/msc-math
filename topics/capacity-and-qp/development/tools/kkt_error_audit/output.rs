use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KktErrorAuditRow {
    pub(crate) event: &'static str,
    pub(crate) family: String,
    pub(crate) source_id: String,
    pub(crate) input_source: String,
    pub(crate) enumeration: &'static str,
    pub(crate) facet_count: usize,
    pub(crate) iterations: u64,
    pub(crate) action_rank: usize,
    pub(crate) sigma: Vec<usize>,
    pub(crate) status: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_admissibility: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) current_f64_verdict: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_inverse_beta_radius_verdict: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_positive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_q: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_action: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_beta_margin: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_q: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_action: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_action_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_action_upper: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_beta_margin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) f64_beta_inf_norm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) current_q_error_bound: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) current_q_bound_covers_exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_inverse_beta_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_inverse_beta_radius_covers_exact_beta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_inverse_beta_radius_q_bound: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_inverse_beta_radius_q_bound_covers_exact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) kkt_residual_inf_norm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_kkt_residual_inf_norm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) kkt_inverse_inf_norm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exact_inverse_residual_inf_norm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified_kkt_inverse_inf_norm_bound: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) note: Option<&'static str>,
}

pub(crate) fn write_rows(path: &Path, rows: &[KktErrorAuditRow]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)
            .unwrap_or_else(|e| panic!("serialize row for {}: {e}", path.display()));
        writer
            .write_all(b"\n")
            .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
    writer
        .flush()
        .unwrap_or_else(|e| panic!("flush {}: {e}", path.display()));
}
