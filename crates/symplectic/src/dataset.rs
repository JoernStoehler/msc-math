//! JSONL dataset row types for polytope datasets and acceptance sweeps.
//!
//! Defines the serialization schema for two dataset types:
//! - [`PolytopeRow`]: one polytope with computed capacity, volume, and systolic ratio
//! - [`AcceptanceRow`]: rejection sampling statistics for a given facet count
//!
//! Each row serializes to a single JSON line (JSONL format) for streaming I/O.
//!
//! Mathematical correspondence: [def:systolic-ratio]

use crate::{geom::polytope::Polytope4D, systolic_ratio};
use serde::{Deserialize, Serialize};

/// A single polytope row in the main dataset.
///
/// Contains the polytope definition (normals, heights), computed geometric
/// quantities (volume, capacity, systolic ratio), and timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolytopeRow {
    /// How this polytope was generated (e.g., "random_F8", "hko_pentagon").
    pub source: String,
    /// Number of facets (halfspaces).
    pub facet_count: usize,
    /// Dual vertices a_i (halfspace a_i^T x <= 1), one per facet.
    pub dual_vertices: Vec<[f64; 4]>,
    /// 4D volume computed by the canonical pure-Rust volume backend.
    pub volume: f64,
    /// EHZ capacity computed via the HK2017 algorithm.
    pub capacity: f64,
    /// Systolic ratio: sys = capacity^2 / (2 * volume).
    ///
    /// Mathematical correspondence: [def:systolic-ratio]
    pub sys: f64,
    /// Time to compute volume (milliseconds).
    pub time_volume_ms: f64,
    /// Time to compute EHZ capacity (milliseconds).
    pub time_capacity_ms: f64,
    /// Time to generate/validate the polytope (milliseconds).
    pub time_creation_ms: f64,
    /// Number of (subset, permutation) pairs where KKT was solved.
    /// Zero if capacity was skipped (e.g., too many facets).
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
    ///
    /// Computes the systolic ratio sys = capacity^2 / (2 * volume).
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
        let dual_vertices: Vec<[f64; 4]> = polytope
            .dual_vertices_f64()
            .iter()
            .map(|a| [a[0], a[1], a[2], a[3]])
            .collect();
        let sys = systolic_ratio(capacity, volume);
        Self {
            source,
            facet_count: polytope.facet_count(),
            dual_vertices,
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

/// A single row in the acceptance sweep dataset.
///
/// Records rejection sampling statistics for random polytope generation
/// at a given facet count and height range.
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
mod tests {
    use super::*;
    use crate::geom::polytope::Polytope4D;
    use nalgebra::Vector4;

    // Tests for dataset: serialization roundtrip for JSONL row types.
    //
    // Proposition: PolytopeRow and AcceptanceRow serialize to valid JSONL
    // (single-line JSON) and deserialize back without data loss.
    //
    // Strategy: fixture-based on hypercube and synthetic acceptance data.

    /// Build a dummy hypercube for testing (8 facets, unit halfspaces).
    fn dummy_polytope() -> Polytope4D {
        let normals = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
        ];
        Polytope4D::from_f64(normals).unwrap()
    }

    /// Verify PolytopeRow serializes to JSON and deserializes back without data loss.
    #[test]
    fn polytope_row_round_trip() {
        let p = dummy_polytope();
        let row = PolytopeRow::from_polytope(&p, "test".into(), 2.0, 3.0, 0, 1.0, 1.5, 0.1);

        let json = serde_json::to_string(&row).unwrap();
        let parsed: PolytopeRow = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.source, "test");
        assert_eq!(parsed.facet_count, 8);
        assert!((parsed.volume - 2.0).abs() < 1e-12);
        assert!((parsed.capacity - 3.0).abs() < 1e-12);
    }

    /// Verify sys = c^2 / (2*vol) is computed correctly from capacity and volume.
    #[test]
    fn sys_computation() {
        let p = dummy_polytope();
        let row = PolytopeRow::from_polytope(&p, "test".into(), 2.0, 3.0, 0, 0.0, 0.0, 0.0);
        // sys = 3^2 / (2 * 2) = 9/4 = 2.25
        assert!((row.sys - 2.25).abs() < 1e-12);
    }

    /// Verify AcceptanceRow serializes to JSON and deserializes back without data loss.
    #[test]
    fn acceptance_row_round_trip() {
        let row = AcceptanceRow {
            facet_count: 5,
            h_min: 0.5,
            h_max: 2.0,
            n_total: 1000,
            n_accepted: 342,
            acceptance_ratio: 0.342,
            avg_time_accepted_ms: 0.5,
            avg_time_rejected_ms: 0.1,
        };

        let json = serde_json::to_string(&row).unwrap();
        let parsed: AcceptanceRow = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.facet_count, 5);
        assert_eq!(parsed.n_total, 1000);
        assert_eq!(parsed.n_accepted, 342);
        assert!((parsed.acceptance_ratio - 0.342).abs() < 1e-12);
    }

    /// Verify serialized JSON output contains no embedded newlines (valid JSONL).
    #[test]
    fn jsonl_format_no_newlines() {
        let p = dummy_polytope();
        let row = PolytopeRow::from_polytope(&p, "test".into(), 1.0, 1.0, 0, 0.0, 0.0, 0.0);
        let json = serde_json::to_string(&row).unwrap();
        assert!(!json.contains('\n'), "JSONL line must not contain newlines");
    }
}
