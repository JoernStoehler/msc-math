/// JSONL row types for the polytope dataset and acceptance sweep.
use crate::geom::polytope::Polytope4D;
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
    /// Time to compute volume via qhull (milliseconds).
    pub time_volume_ms: f64,
    /// Time to compute EHZ capacity via HK2017 algorithm (milliseconds).
    pub time_capacity_ms: f64,
    /// Time to generate/validate the polytope (milliseconds).
    pub time_creation_ms: f64,
    /// Number of (subset, permutation) pairs where KKT was solved (after adjacency pruning).
    /// 0 if capacity was skipped (e.g., too many facets).
    #[serde(default)]
    pub iterations: u64,

    // ---- Billiard algorithm fields (present only for Lagrangian products) ----

    /// EHZ capacity from the billiard algorithm.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_billiard: Option<f64>,
    /// Time to compute billiard capacity (milliseconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_billiard_ms: Option<f64>,
    /// Number of KKT solves in the billiard algorithm.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iterations_billiard: Option<u64>,
    /// Bounce count (k) of the billiard optimal orbit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounces: Option<usize>,
    /// Whether HK2017 and billiard agree within tolerance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithms_agree: Option<bool>,
}

impl PolytopeRow {
    /// Build a row from a polytope and its computed values.
    #[allow(clippy::too_many_arguments)]
    pub fn from_polytope(
        polytope: &Polytope4D,
        source: String,
        volume: f64,
        capacity: f64,
        iterations: u64,
        time_volume_ms: f64,
        time_capacity_ms: f64,
        time_creation_ms: f64,
    ) -> Self {
        let normals: Vec<[f64; 4]> = polytope
            .normals_f64()
            .iter()
            .map(|n| [n[0], n[1], n[2], n[3]])
            .collect();
        let sys = capacity * capacity / (2.0 * volume);
        Self {
            source,
            facet_count: polytope.facet_count(),
            normals,
            heights: polytope.heights_f64().to_vec(),
            volume,
            capacity,
            sys,
            time_volume_ms,
            time_capacity_ms,
            time_creation_ms,
            iterations,
            capacity_billiard: None,
            time_billiard_ms: None,
            iterations_billiard: None,
            bounces: None,
            algorithms_agree: None,
        }
    }
}

/// A single row in the acceptance sweep dataset (dataset 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceRow {
    /// Number of halfspaces in the sampled polytopes.
    pub facet_count: usize,
    /// Minimum height parameter for rejection sampling.
    pub h_min: f64,
    /// Maximum height parameter for rejection sampling.
    pub h_max: f64,
    /// Total number of sampling attempts.
    pub n_total: usize,
    /// Number of attempts that produced a valid polytope.
    pub n_accepted: usize,
    /// Fraction of accepted samples: n_accepted / n_total.
    pub acceptance_ratio: f64,
    /// Mean validation time for accepted samples (milliseconds).
    pub avg_time_accepted_ms: f64,
    /// Mean validation time for rejected samples (milliseconds).
    pub avg_time_rejected_ms: f64,
}

#[cfg(test)]
#[path = "dataset_test.rs"]
mod dataset_test;
