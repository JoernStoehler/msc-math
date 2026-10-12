//! Shared feature, provenance, and write stages for both supported input formats.

use crate::{features_trace, invariant_features, load_caches, run_local, write_database};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Input policy is selected before loading. Retained rows have no evaluator
/// labels unless the historical source recorded them; run-local rows project
/// labels from computed payloads.
pub enum InputFormat {
    Retained(load_caches::DatasetPaths),
    RunLocal {
        produce_dir: PathBuf,
        out_dir: PathBuf,
    },
}

#[derive(Serialize)]
struct PrepareStatsRow {
    produce_dir: String,
    out_dir: String,
    polytope_rows: usize,
    provenance_rows: usize,
    max_sys: Option<f64>,
    sys_gt_one: usize,
    build_polytope_table_ms: f64,
    wall_time_ms: f64,
}

fn write_json<T: Serialize>(path: &Path, value: &T) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create stats parent");
    }
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

pub fn run(input: InputFormat) {
    let total_started = Instant::now();
    let out_dir = match &input {
        InputFormat::Retained(paths) => &paths.out_dir,
        InputFormat::RunLocal { out_dir, .. } => out_dir,
    };
    let load_started = Instant::now();
    let caches = match &input {
        InputFormat::Retained(paths) => {
            eprintln!("Loading retained random/product producer rows");
            load_caches::load_caches(paths)
        }
        InputFormat::RunLocal { produce_dir, .. } => {
            eprintln!("Loading produced artifacts from {}", produce_dir.display());
            run_local::load_new_producer_outputs(produce_dir)
        }
    };
    eprintln!(
        "Loaded {} polytopes and {} provenance rows",
        caches.polytopes.len(),
        caches.provenance_rows.len()
    );
    if matches!(&input, InputFormat::Retained(_)) {
        eprintln!(
            "Loaded producer caches in {:.1}s",
            load_started.elapsed().as_secs_f64()
        );
        let mut counts = BTreeMap::<&str, usize>::new();
        for row in &caches.polytopes {
            *counts.entry(row.capacity_source.as_str()).or_default() += 1;
        }
        eprintln!("Capacity sources: {counts:?}");
    }

    eprintln!("Building invariant polytope table");
    let started = Instant::now();
    let polytope_rows = invariant_features::build_polytope_table(&caches.polytopes);
    let build_polytope_table_ms = started.elapsed().as_secs_f64() * 1000.0;
    eprintln!(
        "Built invariant polytope table in {:.1}s",
        build_polytope_table_ms / 1000.0
    );
    eprintln!("Building provenance/run table");
    let started = Instant::now();
    let provenance_rows = features_trace::build_provenance_run_table(&caches.provenance_rows);
    eprintln!(
        "Built provenance/run table in {:.1}s",
        started.elapsed().as_secs_f64()
    );
    eprintln!("Writing tables to {}", out_dir.display());
    let started = Instant::now();
    write_database::write_database(out_dir, &polytope_rows, &provenance_rows);
    eprintln!(
        "Wrote prepared tables in {:.1}s",
        started.elapsed().as_secs_f64()
    );

    if let InputFormat::RunLocal { produce_dir, .. } = &input {
        let wall_time_ms = total_started.elapsed().as_secs_f64() * 1000.0;
        let stats = PrepareStatsRow {
            produce_dir: produce_dir.display().to_string(),
            out_dir: out_dir.display().to_string(),
            polytope_rows: polytope_rows.len(),
            provenance_rows: caches.provenance_rows.len(),
            max_sys: polytope_rows.iter().map(|row| row.sys).reduce(f64::max),
            sys_gt_one: polytope_rows.iter().filter(|row| row.sys > 1.0).count(),
            build_polytope_table_ms,
            wall_time_ms,
        };
        write_json(&out_dir.join("prepare-stats.json"), &stats);
        eprintln!("Total prepare time {:.1}s", wall_time_ms / 1000.0);
    } else {
        eprintln!(
            "Total table build time {:.1}s",
            total_started.elapsed().as_secs_f64()
        );
    }
    println!("Wrote {}", out_dir.display());
}
