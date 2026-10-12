//! Write the final datascience tables.

use crate::rows::{PolytopeTableRow, ProvenanceRunRow};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(writer).expect("write newline");
    }
    writer.flush().expect("flush output");
}

pub fn write_database(
    out_dir: &Path,
    polytope_rows: &[PolytopeTableRow],
    provenance_run_rows: &[ProvenanceRunRow],
) {
    std::fs::create_dir_all(out_dir)
        .unwrap_or_else(|e| panic!("create {}: {e}", out_dir.display()));
    let provenance_rows = provenance_run_rows
        .iter()
        .map(PolytopeProvenanceTableRow::from)
        .collect::<Vec<_>>();
    write_jsonl(&out_dir.join("polytope-table.jsonl"), polytope_rows);
    write_jsonl(
        &out_dir.join("polytope-provenance-table.jsonl"),
        &provenance_rows,
    );
}

#[derive(Serialize)]
struct PolytopeProvenanceTableRow<'a> {
    provenance_id: &'a str,
    poly_id: &'a str,
    dataset: &'a str,
    family: &'a str,
    role: &'a str,
    search_space: &'a str,
    optimizer: &'a str,
    backend: &'a str,
    source_name: &'a str,
    root_group_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: &'a Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_attempt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_h_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_h_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_m: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_bounces: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_id: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_time_ms: Option<f64>,
}

impl<'a> From<&'a ProvenanceRunRow> for PolytopeProvenanceTableRow<'a> {
    fn from(row: &'a ProvenanceRunRow) -> Self {
        Self {
            provenance_id: &row.provenance_id,
            poly_id: &row.poly_id,
            dataset: &row.dataset,
            family: &row.family,
            role: &row.role,
            search_space: &row.search_space,
            optimizer: &row.optimizer,
            backend: &row.backend,
            source_name: &row.source_name,
            root_group_id: &row.root_group_id,
            source: &row.source,
            sample_seed: row.sample_seed,
            sample_attempt: row.sample_attempt,
            sample_h_min: row.sample_h_min,
            sample_h_max: row.sample_h_max,
            product_k: row.product_k,
            product_m: row.product_m,
            product_bounces: row.product_bounces,
            seed_index: row.seed_index,
            lineage_id: &row.lineage_id,
            path: &row.path,
            total_time_ms: row.total_time_ms,
        }
    }
}
