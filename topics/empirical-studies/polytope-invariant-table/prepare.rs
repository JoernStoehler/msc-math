//! Compatibility command for preparing run-local producer outputs.

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
    let args = run_local::parse_args_from(&std::env::args().collect::<Vec<_>>());
    table_builder::run(table_builder::InputFormat::RunLocal {
        produce_dir: args.produce_dir,
        out_dir: args.out_dir,
    });
}
