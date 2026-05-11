//! Record naming and polytope reconstruction helpers.

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
