//! Build the sys-landscape datascience tables from producer caches.
//!
//! Input Artifacts: producer JSONL files under `experiments/sys-landscape/datascience/produce/`
//! Output Artifacts: ad hoc output directory passed by `--out-dir`

mod features;
mod features_trace;
mod load_caches;
mod rows;
mod write_database;

use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    let paths = load_caches::parse_args();
    let total_started = Instant::now();
    eprintln!("Loading producer caches");
    let started = Instant::now();
    let caches = load_caches::load_caches(&paths);
    eprintln!(
        "Loaded producer caches in {:.1}s",
        started.elapsed().as_secs_f64()
    );
    eprintln!(
        "Loaded {} polytopes and {} provenance rows",
        caches.polytopes.len(),
        caches.provenance_rows.len()
    );
    eprintln!(
        "Loaded {} computed-polytope observation rows",
        caches.computed_polytope_observations.len()
    );
    eprintln!("Capacity sources: {:?}", capacity_source_counts(&caches));
    eprintln!("Building polytope table");
    let started = Instant::now();
    let polytope_rows = features::build_polytope_table(&caches.polytopes);
    eprintln!(
        "Built polytope table in {:.1}s",
        started.elapsed().as_secs_f64()
    );
    eprintln!("Building provenance/run table");
    let started = Instant::now();
    let provenance_run_rows = features_trace::build_provenance_run_table(&caches.provenance_rows);
    eprintln!(
        "Built provenance/run table in {:.1}s",
        started.elapsed().as_secs_f64()
    );
    eprintln!("Writing tables to {}", paths.out_dir.display());
    let started = Instant::now();
    write_database::write_database(
        &paths.out_dir,
        &polytope_rows,
        &provenance_run_rows,
        &caches.computed_polytope_observations,
    );
    eprintln!("Wrote tables in {:.1}s", started.elapsed().as_secs_f64());
    eprintln!(
        "Total table build time {:.1}s",
        total_started.elapsed().as_secs_f64()
    );

    println!("Wrote {}", paths.out_dir.display());
}

fn capacity_source_counts(caches: &load_caches::LoadedCaches) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for row in &caches.polytopes {
        *counts.entry(row.capacity_source.as_str()).or_default() += 1;
    }
    counts
}
