//! Explicit-format entrypoint for the invariant table builder.

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
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "Build invariant tables. Usage: {} --input-format <retained|run-local> --out-dir <dir> [input options]\n\
             Retained: --produce-dir <dir> or --random <path> --random-product <path>; optional --random-only-size <smoke|method|full>.\n\
             Run-local: --produce-dir <dir> containing computed-polytopes.jsonl and sample metadata.",
            args[0]
        );
        return;
    }
    let position = args
        .iter()
        .position(|arg| arg == "--input-format")
        .expect("--input-format <retained|run-local> is required");
    let format = args
        .get(position + 1)
        .expect("--input-format requires a value")
        .clone();
    args.drain(position..=position + 1);
    match format.as_str() {
        "retained" => {
            let paths = load_caches::parse_args_from(&args);
            table_builder::run(table_builder::InputFormat::Retained(paths));
        }
        "run-local" => {
            let input = run_local::parse_args_from(&args);
            table_builder::run(table_builder::InputFormat::RunLocal {
                produce_dir: input.produce_dir,
                out_dir: input.out_dir,
            });
        }
        other => panic!("unknown --input-format {other}; expected retained or run-local"),
    }
}
