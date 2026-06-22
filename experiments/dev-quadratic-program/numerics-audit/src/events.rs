use crate::args::RunMode;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct RunStarted {
    pub event: &'static str,
    pub target: &'static str,
    pub mode: RunMode,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextStarted {
    pub event: &'static str,
    pub mode: RunMode,
    pub context_id: String,
    pub context_kind: &'static str,
    pub object_id: String,
    pub object_family: &'static str,
    pub input_pair_kind: &'static str,
    pub sigma: Vec<usize>,
    pub sample_policy: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub event: &'static str,
    pub mode: RunMode,
    pub algorithm: &'static str,
    pub stage: &'static str,
    pub context_id: String,
    pub context_kind: &'static str,
    pub object_id: String,
    pub object_family: &'static str,
    pub input_pair_kind: &'static str,
    pub sigma: Vec<usize>,
    pub variable: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<usize>,
    pub sample_policy: &'static str,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f64: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_f64: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abs_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PredicateObservation {
    pub event: &'static str,
    pub mode: RunMode,
    pub algorithm: &'static str,
    pub stage: &'static str,
    pub context_id: String,
    pub context_kind: &'static str,
    pub object_id: String,
    pub object_family: &'static str,
    pub input_pair_kind: &'static str,
    pub sigma: Vec<usize>,
    pub predicate: &'static str,
    pub sample_policy: &'static str,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f64_trinary: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disagrees_with_oracle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextFinished {
    pub event: &'static str,
    pub mode: RunMode,
    pub context_id: String,
    pub status: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct RunFinished {
    pub event: &'static str,
    pub mode: RunMode,
    pub contexts: usize,
    pub status: &'static str,
}
