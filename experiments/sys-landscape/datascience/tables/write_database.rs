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
    let ascent_run_rows = provenance_run_rows
        .iter()
        .filter(|row| row.optimizer.contains("gradient_ascent"))
        .map(PolytopeAscentRunTableRow::from)
        .collect::<Vec<_>>();
    write_jsonl(&out_dir.join("polytope-table.jsonl"), polytope_rows);
    write_jsonl(
        &out_dir.join("polytope-provenance-table.jsonl"),
        &provenance_rows,
    );
    write_jsonl(
        &out_dir.join("polytope-ascent-run-table.jsonl"),
        &ascent_run_rows,
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
    seed_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_id: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_provenance_id: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rq: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: &'a Option<String>,
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
            seed_index: row.seed_index,
            lineage_id: &row.lineage_id,
            parent_provenance_id: &row.parent_provenance_id,
            rq: &row.rq,
            path: &row.path,
        }
    }
}

#[derive(Serialize)]
struct PolytopeAscentRunTableRow<'a> {
    provenance_id: &'a str,
    poly_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_f: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reported_final_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reported_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sys_after_addition: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_iterations: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_phases: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_strategy: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_escape_overshoot: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_escape_wiggle: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_direction: Option<[f64; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_remained_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_time_ms: Option<f64>,
    trajectory_trace_available: f64,
    trajectory_event_count: usize,
    trajectory_phase_count: usize,
    trajectory_mean_iters_per_phase: f64,
    trajectory_overshoot_fraction: f64,
    trajectory_overshoot_15_fraction: f64,
    trajectory_overshoot_2_fraction: f64,
    trajectory_overshoot_3_fraction: f64,
    trajectory_t_fraction_mean: f64,
    trajectory_t_fraction_std: f64,
    trajectory_t_fraction_max: f64,
    trajectory_t_actual_mean: f64,
    trajectory_t_actual_std: f64,
    trajectory_t_actual_max: f64,
    trajectory_gradient_norm_mean: f64,
    trajectory_gradient_norm_std: f64,
    trajectory_gradient_norm_max: f64,
    trajectory_delta_share_top1: f64,
    trajectory_delta_share_top3: f64,
    trajectory_restart_drop_mean: f64,
    trajectory_restart_drop_max: f64,
    trajectory_restart_drop_fraction: f64,
    trajectory_efficiency_mean: f64,
    trajectory_efficiency_std: f64,
    trajectory_efficiency_max: f64,
}

impl<'a> From<&'a ProvenanceRunRow> for PolytopeAscentRunTableRow<'a> {
    fn from(row: &'a ProvenanceRunRow) -> Self {
        Self {
            provenance_id: &row.provenance_id,
            poly_id: &row.poly_id,
            starting_f: row.starting_f,
            starting_sys: row.starting_sys,
            reported_final_sys: row.reported_final_sys,
            reported_delta: row.reported_delta,
            sys_after_addition: row.sys_after_addition,
            n_iterations: row.n_iterations,
            n_phases: row.n_phases,
            best_strategy: &row.best_strategy,
            n_escape_overshoot: row.n_escape_overshoot,
            n_escape_wiggle: row.n_escape_wiggle,
            placement_direction: row.placement_direction,
            facet_remained_active: row.facet_remained_active,
            total_time_ms: row.total_time_ms,
            trajectory_trace_available: row.trajectory_trace_available,
            trajectory_event_count: row.trajectory_event_count,
            trajectory_phase_count: row.trajectory_phase_count,
            trajectory_mean_iters_per_phase: row.trajectory_mean_iters_per_phase,
            trajectory_overshoot_fraction: row.trajectory_overshoot_fraction,
            trajectory_overshoot_15_fraction: row.trajectory_overshoot_15_fraction,
            trajectory_overshoot_2_fraction: row.trajectory_overshoot_2_fraction,
            trajectory_overshoot_3_fraction: row.trajectory_overshoot_3_fraction,
            trajectory_t_fraction_mean: row.trajectory_t_fraction_mean,
            trajectory_t_fraction_std: row.trajectory_t_fraction_std,
            trajectory_t_fraction_max: row.trajectory_t_fraction_max,
            trajectory_t_actual_mean: row.trajectory_t_actual_mean,
            trajectory_t_actual_std: row.trajectory_t_actual_std,
            trajectory_t_actual_max: row.trajectory_t_actual_max,
            trajectory_gradient_norm_mean: row.trajectory_gradient_norm_mean,
            trajectory_gradient_norm_std: row.trajectory_gradient_norm_std,
            trajectory_gradient_norm_max: row.trajectory_gradient_norm_max,
            trajectory_delta_share_top1: row.trajectory_delta_share_top1,
            trajectory_delta_share_top3: row.trajectory_delta_share_top3,
            trajectory_restart_drop_mean: row.trajectory_restart_drop_mean,
            trajectory_restart_drop_max: row.trajectory_restart_drop_max,
            trajectory_restart_drop_fraction: row.trajectory_restart_drop_fraction,
            trajectory_efficiency_mean: row.trajectory_efficiency_mean,
            trajectory_efficiency_std: row.trajectory_efficiency_std,
            trajectory_efficiency_max: row.trajectory_efficiency_max,
        }
    }
}
