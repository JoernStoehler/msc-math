//! Compatibility command for retained random/product producer rows.

mod features_trace;
mod invariant_features;
mod load_caches;
#[path = "../polytope-datasets/rows.rs"]
mod producer_rows;
mod rows;
mod run_local;
mod table_builder;
mod write_database;

fn main() {
    let paths = load_caches::parse_args_from(&std::env::args().collect::<Vec<_>>());
    table_builder::run(table_builder::InputFormat::Retained(paths));
}
