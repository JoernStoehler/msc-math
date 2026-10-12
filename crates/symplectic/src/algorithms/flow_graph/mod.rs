//! Flow-graph capacity algorithm work surface.
//!
//! The local README routes to the maintained topic account and implementation
//! contract in `topics/flow-graphs/`. The idealized theorem and rational Rust
//! rejection boundary have distinct hypotheses and evidence.

pub mod exact_search;
pub mod exact_tube;
mod words;

pub use exact_tube::{
    closed_tube_visualization_snapshot_exact, face_polygon_snapshot_exact,
    ExactAffineScalarSnapshot, ExactAffineSnapshot, ExactClosedOrbitSnapshot,
    ExactClosedTubeMetrics, ExactFacePolygonSnapshot, ExactFixedPointSnapshot, ExactFlatTubeInput,
    ExactHalfspaceSnapshot, ExactTubeFaceFixedPointSnapshot, ExactTubeFaceSnapshot,
    ExactTubeVisualizationSnapshot,
};
pub use words::*;
