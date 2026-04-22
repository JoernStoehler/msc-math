//! Build the sys-landscape datascience tables from producer caches.
//!
//! Input Artifacts: producer JSONL files under `experiments/sys-landscape/datascience/produce/`
//! Output Artifacts: ad hoc output directory passed by `--out-dir`

mod features;
mod features_trace;
mod load_caches;
mod rows;
mod write_database;

fn main() {
    let paths = load_caches::parse_args();
    let caches = load_caches::load_caches(&paths);
    let polytope_rows = features::build_polytope_table(&caches.polytopes);
    let observation_rows = features_trace::build_observation_table(&caches.observations);
    write_database::write_database(&paths.out_dir, &polytope_rows, &observation_rows);

    println!("Wrote {}", paths.out_dir.display());
}
