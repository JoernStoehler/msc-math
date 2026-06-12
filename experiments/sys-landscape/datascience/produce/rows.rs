//! Shared producer JSONL row schemas for sys-landscape datascience outputs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatascienceRandomSampleRow {
    pub name: String,
    pub poly_id: String,
    pub facet_count: usize,
    pub seed: u64,
    pub attempt: u64,
    pub h_min: f64,
    pub h_max: f64,
    pub sys: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatascienceRandomProductSampleRow {
    pub name: String,
    pub poly_id: String,
    pub k: usize,
    pub m: usize,
    pub facet_count: usize,
    pub seed: u64,
    pub attempt: u64,
    pub h_min: f64,
    pub h_max: f64,
    pub sys: f64,
    pub bounces: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomSweepRow {
    pub name: String,
    pub facet_count: usize,
    pub dual_vertices: Vec<[f64; 4]>,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub vertices_rational: Vec<[String; 4]>,
    pub h_min: f64,
    pub h_max: f64,
    pub volume: f64,
    pub capacity: f64,
    pub sys: f64,
    pub iterations: u64,
    pub time_volume_ms: f64,
    pub time_capacity_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomProductRow {
    pub name: String,
    pub k: usize,
    pub m: usize,
    pub facet_count: usize,
    pub dual_vertices: Vec<[f64; 4]>,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub vertices_rational: Vec<[String; 4]>,
    pub h_min: f64,
    pub h_max: f64,
    pub volume: f64,
    pub capacity: f64,
    pub sys: f64,
    pub iterations: u64,
    pub bounces: usize,
    pub time_volume_ms: f64,
    pub time_capacity_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultRow {
    /// "rq1" or "rq2"
    pub rq: String,
    /// "f10_localmax_then_f11", "f10_ascent", "f10_add_then_f11", "random_f11", "f10_ascent_then_f11"
    pub path: String,
    /// Seed/source identifier
    pub name: String,
    /// External source or seed group that defines the lineage
    #[serde(default)]
    pub source_name: String,
    /// Stable lineage identifier across related paths or placements
    #[serde(default)]
    pub lineage_id: String,
    /// Parent trial row when one exists in this dataset
    #[serde(default)]
    pub direct_parent_trial: Option<String>,
    /// Facet count at start of gradient ascent
    pub starting_f: usize,
    /// sys before any optimization in this trial
    pub starting_sys: f64,
    /// sys immediately after facet addition (before ascent), or null
    #[serde(default)]
    pub sys_after_addition: Option<f64>,
    /// sys after gradient ascent
    pub final_sys: f64,
    /// final_sys - starting_sys (of the source F=10 polytope for RQ1, of start for RQ2)
    pub delta_vs_source: f64,
    /// Total gradient iterations across all phases
    pub n_iterations: usize,
    /// Number of ascent phases (initial + escape rounds)
    pub n_phases: usize,
    /// Facet placement direction (unit vector), or null
    #[serde(default)]
    pub placement_direction: Option<[f64; 4]>,
    /// Whether the added facet is still non-redundant at the end
    #[serde(default)]
    pub facet_remained_active: Option<bool>,
    /// Wall-clock time for this trial
    pub total_time_ms: f64,
    /// Exact dual vertices at the start of the ascent stage
    pub starting_dual_vertices_rational: Vec<[String; 4]>,
    /// Exact dual vertices immediately after facet addition, if applicable
    #[serde(default)]
    pub after_addition_dual_vertices_rational: Option<Vec<[String; 4]>>,
    /// Exact dual vertices at the endpoint
    pub final_dual_vertices_rational: Vec<[String; 4]>,
    /// Final dual vertices
    pub final_dual_vertices: Vec<[f64; 4]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradientAscentRow {
    pub name: String,
    pub final_sys: f64,
    pub final_dual_vertices: Vec<[f64; 4]>,
}
