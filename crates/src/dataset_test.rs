//! Tests for dataset: serialization roundtrip for JSONL row types.
//!
//! Proposition: PolytopeRow and AcceptanceRow serialize to valid JSONL
//! (single-line JSON) and deserialize back without data loss.
//!
//! Strategy: fixture-based on hypercube and synthetic acceptance data.

use super::dataset::*;
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

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
    Polytope4D::new(normals).unwrap()
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
