/// JSONL row types for the polytope dataset and acceptance sweep.
use geom::polytope::Polytope4D;
use serde::{Deserialize, Serialize};

/// A single polytope row in the main dataset (dataset 1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolytopeRow {
    pub source: String,
    pub facet_count: usize,
    pub normals: Vec<[f64; 4]>,
    pub heights: Vec<f64>,
    pub volume: f64,
    pub capacity: f64,
    /// Systolic ratio: sys = capacity² / (2 · volume).
    pub sys: f64,
    pub time_volume_ms: f64,
    pub time_capacity_ms: f64,
    pub time_creation_ms: f64,
}

impl PolytopeRow {
    /// Build a row from a polytope and its computed values.
    pub fn from_polytope(
        polytope: &Polytope4D,
        source: String,
        volume: f64,
        capacity: f64,
        time_volume_ms: f64,
        time_capacity_ms: f64,
        time_creation_ms: f64,
    ) -> Self {
        let normals: Vec<[f64; 4]> = polytope
            .normals()
            .iter()
            .map(|n| [n[0], n[1], n[2], n[3]])
            .collect();
        let sys = capacity * capacity / (2.0 * volume);
        Self {
            source,
            facet_count: polytope.facet_count(),
            normals,
            heights: polytope.heights().to_vec(),
            volume,
            capacity,
            sys,
            time_volume_ms,
            time_capacity_ms,
            time_creation_ms,
        }
    }
}

/// A single row in the acceptance sweep dataset (dataset 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceRow {
    pub facet_count: usize,
    pub h_min: f64,
    pub h_max: f64,
    pub n_total: usize,
    pub n_accepted: usize,
    pub acceptance_ratio: f64,
    pub avg_time_accepted_ms: f64,
    pub avg_time_rejected_ms: f64,
}

#[cfg(test)]
#[path = "dataset_test.rs"]
mod dataset_test;
