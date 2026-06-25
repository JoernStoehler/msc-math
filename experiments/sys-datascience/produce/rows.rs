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
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub attempt: Option<u64>,
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
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub attempt: Option<u64>,
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
