//! Checkpoint serialization for resumable crosspolytope search runs.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CandidateSer {
    pub(crate) action: f64,
    pub(crate) subset: Vec<usize>,
    pub(crate) permutation: Vec<usize>,
    pub(crate) beta: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Checkpoint {
    pub(crate) completed_m: usize,
    pub(crate) iterations: u64,
    pub(crate) elapsed_secs: f64,
    pub(crate) best_certified: Option<CandidateSer>,
    pub(crate) best_uncertain: Option<CandidateSer>,
}

pub(crate) fn checkpoint_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("main/checkpoint.json")
}

pub(crate) fn save_checkpoint(cp: &Checkpoint) {
    let path = checkpoint_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create checkpoint directory");
    }
    let file = File::create(&path).expect("failed to create checkpoint");
    serde_json::to_writer_pretty(file, cp).expect("failed to write checkpoint");
    println!(
        "  [checkpoint] m={}, iterations={}, {:.1}s elapsed",
        cp.completed_m, cp.iterations, cp.elapsed_secs
    );
}

pub(crate) fn load_checkpoint() -> Option<Checkpoint> {
    let path = checkpoint_path();
    if !path.exists() {
        return None;
    }
    let file = File::open(&path).ok()?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).ok()
}
