//! Build random/product provenance table rows.

use crate::load_caches::LoadedProvenanceRow;
use crate::rows::ProvenanceRunRow;

pub fn build_provenance_run_table(rows: &[LoadedProvenanceRow]) -> Vec<ProvenanceRunRow> {
    rows.iter()
        .map(|row| ProvenanceRunRow {
            provenance_id: row.provenance_id.clone(),
            poly_id: row.poly_id.clone(),
            dataset: row.dataset.clone(),
            family: row.family.clone(),
            role: row.role.clone(),
            search_space: row.search_space.clone(),
            optimizer: row.optimizer.clone(),
            backend: row.backend.clone(),
            source_name: row.source_name.clone(),
            root_group_id: row.root_group_id.clone(),
            sample_seed: row.sample_seed,
            sample_attempt: row.sample_attempt,
            sample_h_min: row.sample_h_min,
            sample_h_max: row.sample_h_max,
            product_k: row.product_k,
            product_m: row.product_m,
            product_bounces: row.product_bounces,
            seed_index: row.seed_index,
            lineage_id: row.lineage_id.clone(),
            path: row.path.clone(),
            total_time_ms: row.total_time_ms,
        })
        .collect()
}
