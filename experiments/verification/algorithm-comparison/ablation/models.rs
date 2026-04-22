use serde::{Deserialize, Serialize};
use symplectic::geom::polytope::Polytope4D;

pub const SEED: u64 = 42;
pub const H_MIN: f64 = 0.5;
pub const H_MAX: f64 = 2.0;
pub const N_PER_GROUP: usize = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct AblationEntry {
    pub polytope_name: String,
    pub variant: String,
    pub group: String,
    pub facet_count: usize,
    pub dual_vertices: Vec<[f64; 4]>,
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub iterations: u64,
    pub time_ms: f64,
}

#[derive(Debug)]
pub struct AblationCapacityResult {
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub best_permutation: Vec<usize>,
    pub best_beta: Vec<f64>,
    pub iterations: u64,
}

#[derive(Debug)]
pub struct AblationResult {
    pub result: AblationCapacityResult,
    pub best_subset: Vec<usize>,
}

pub type VariantRunner = fn(&Polytope4D) -> Option<AblationResult>;

pub struct Variant {
    pub name: &'static str,
    pub run: VariantRunner,
}

pub struct AblationFixture {
    pub name: String,
    pub group: String,
    pub polytope: Polytope4D,
    pub expected_capacity: Option<f64>,
}
