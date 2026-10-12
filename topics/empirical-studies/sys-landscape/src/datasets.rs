//! Plain path helpers for the current sys-landscape experiment layout.

use std::path::PathBuf;

pub const RAW_DIR: &str = "../polytope-datasets";
pub const CONTINUATION_EXPERIMENT_DIR: &str = "variable-f-ascent";
pub const GRADIENT_ASCENT_GENERAL_DIR: &str = "gradient-ascent-general";
pub const SHARED_CACHE_FILE: &str = "shared-cache.jsonl";
pub const CONTINUATION_CACHE_FILE: &str = "cache.jsonl";

pub fn package_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn experiment_path(experiment_dir: &str, file_name: &str) -> PathBuf {
    package_root().join(experiment_dir).join(file_name)
}

pub fn shared_family_cache_path() -> PathBuf {
    raw_root().join(SHARED_CACHE_FILE)
}

pub fn continuation_cache_path() -> PathBuf {
    experiment_path(CONTINUATION_EXPERIMENT_DIR, CONTINUATION_CACHE_FILE)
}

pub fn raw_root() -> PathBuf {
    package_root().join(RAW_DIR)
}

pub fn raw_dataset_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}.jsonl"))
}

pub fn raw_dataset_trace_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}-trace.jsonl"))
}

pub fn raw_dataset_cache_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}-cache.jsonl"))
}
