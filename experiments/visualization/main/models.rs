//! JSON export models shared by the visualization export pipeline.

use nalgebra::Vector4;
use serde::Serialize;

/// Top-level JSON structure for visualization.
#[derive(Serialize)]
pub struct VizExport {
    pub name: String,
    pub source: String,
    pub capacity: f64,
    pub facet_count: usize,
    pub vertex_count: usize,
    pub edge_count: usize,
    #[serde(rename = "ridge_count")]
    pub two_face_count: usize,
    /// Dual vertices a_i where K = {x : a_i^T x <= 1}, each `[a₁, a₂, a₃, a₄]`.
    pub dual_vertices: Vec<[f64; 4]>,
    /// Reeb flow directions per facet: `reeb_vectors[i] = J₀ · a_i`.
    pub reeb_vectors: Vec<[f64; 4]>,
    /// Vertices, each `[x₁, x₂, x₃, x₄]`.
    pub vertices: Vec<[f64; 4]>,
    /// Edges as pairs of vertex indices.
    pub edges: Vec<[usize; 2]>,
    /// 2-faces.
    #[serde(rename = "ridges")]
    pub two_faces: Vec<VizTwoFace>,
    /// Per-vertex facet incidence.
    pub vertex_facets: Vec<Vec<usize>>,
    /// Reeb trajectories: recovered orbits and displaced variants.
    pub trajectories: Vec<VizTrajectory>,
    /// Volume of the polytope.
    pub volume: f64,
    /// Systolic ratio: c_EHZ² / (2 · volume).
    pub systolic_ratio: f64,
}

#[derive(Serialize)]
pub struct VizTwoFace {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

#[derive(Serialize)]
pub struct VizTrajectory {
    /// Human-readable label for UI display.
    pub label: String,
    pub start_facet: usize,
    pub closed: bool,
    pub segments: Vec<VizSegment>,
}

#[derive(Serialize)]
pub struct VizSegment {
    pub start: [f64; 4],
    pub end: [f64; 4],
    pub facet: usize,
}

pub fn v4_to_array(v: &Vector4<f64>) -> [f64; 4] {
    [v[0], v[1], v[2], v[3]]
}
