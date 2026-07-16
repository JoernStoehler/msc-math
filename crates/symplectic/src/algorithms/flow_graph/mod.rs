//! Flow-graph capacity algorithm work surface.
//!
//! The local README is the status and contract surface for this unfinished but
//! thesis-facing CH2021-style algorithm packet.

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
