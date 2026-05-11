//! Record naming and polytope reconstruction helpers.

use crate::CellPolytopeCache;
use nalgebra::Vector4;
use symplectic::database::{PolytopeRecord, Source};

/// Derive a human-readable name from a database record's `Source`.
pub fn name_from_record(record: &PolytopeRecord, index: usize) -> String {
    match &record.source {
        Some(Source::Random {
            facet_count_target,
            attempt,
            ..
        }) => {
            format!("random_F{facet_count_target}_a{attempt}")
        }
        Some(Source::LagrangianProduct { n1, n2, .. }) => {
            format!("product_{n1}x{n2}_{index}")
        }
        Some(Source::Known { name }) => name.clone(),
        None => format!("polytope_{index}"),
    }
}

/// Derive a source dataset string from a database record's `Source`.
pub fn source_dataset_from_record(record: &PolytopeRecord) -> String {
    match &record.source {
        Some(Source::Random { .. }) => "random-sample".to_string(),
        Some(Source::LagrangianProduct { .. }) => "random-product-sample".to_string(),
        Some(Source::Known { .. }) => "known".to_string(),
        None => "unknown".to_string(),
    }
}

/// Construct a polytope at `a'_k = a_k + t*d_k`.
pub fn construct_at_t(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<CellPolytopeCache> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();
    CellPolytopeCache::from_f64(new_duals)
}

pub fn cache_from_record(record: &PolytopeRecord) -> Option<CellPolytopeCache> {
    CellPolytopeCache::from_rational_parts(
        record.dual_vertices_rational.clone(),
        record.vertices_rational.clone(),
    )
}

pub fn record_from_cache(polytope: &CellPolytopeCache) -> PolytopeRecord {
    PolytopeRecord {
        dual_vertices_rational: polytope.dual_vertices.clone(),
        vertices_rational: polytope.vertices.clone(),
        source: None,
        volume: None,
        volume_err: None,
        capacity: None,
        capacity_err: None,
        sigma_gap_cutoff: None,
        sigmas: None,
        orbit_scalars: None,
    }
}
