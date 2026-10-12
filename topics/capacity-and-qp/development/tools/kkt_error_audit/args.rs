use std::path::PathBuf;

use crate::input::InputSource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Enumeration {
    PrunedExactBinary64,
    Unpruned,
}

#[derive(Clone, Debug)]
pub(crate) struct Args {
    pub(crate) output: PathBuf,
    pub(crate) input_source: InputSource,
    pub(crate) max_rows_per_family: usize,
    pub(crate) generated_samples_per_facet: usize,
    pub(crate) generated_seed: u64,
    pub(crate) family_filter: Vec<String>,
    pub(crate) source_id_filter: Vec<String>,
    pub(crate) max_candidates_per_case: usize,
    pub(crate) enumeration: Enumeration,
}

pub(crate) fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut output = PathBuf::from("/tmp/qp-kkt-error-audit.jsonl");
    let mut input_source = InputSource::Artifacts;
    let mut max_rows_per_family = 1usize;
    let mut generated_samples_per_facet = 1usize;
    let mut generated_seed = 0x5eed_f64_u64;
    let mut family_filter = Vec::new();
    let mut source_id_filter = Vec::new();
    let mut max_candidates_per_case = 64usize;
    let mut enumeration = Enumeration::PrunedExactBinary64;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--output" => {
                output = PathBuf::from(value(&argv, i, "--output"));
                i += 2;
            }
            "--input-source" => {
                input_source = match value(&argv, i, "--input-source") {
                    "all" => InputSource::All,
                    "generated" => InputSource::Generated,
                    "artifacts" => InputSource::Artifacts,
                    "edge-fixtures" => InputSource::EdgeFixtures,
                    other => panic!(
                        "--input-source must be all, generated, artifacts, or edge-fixtures, got {other}"
                    ),
                };
                i += 2;
            }
            "--max-rows-per-family" => {
                max_rows_per_family = value(&argv, i, "--max-rows-per-family")
                    .parse()
                    .expect("--max-rows-per-family must be a non-negative integer");
                i += 2;
            }
            "--generated-samples-per-facet" => {
                generated_samples_per_facet = value(&argv, i, "--generated-samples-per-facet")
                    .parse()
                    .expect("--generated-samples-per-facet must be a non-negative integer");
                i += 2;
            }
            "--generated-seed" => {
                generated_seed = value(&argv, i, "--generated-seed")
                    .parse()
                    .expect("--generated-seed must be a u64");
                i += 2;
            }
            "--family-filter" => {
                family_filter.extend(
                    value(&argv, i, "--family-filter")
                        .split(',')
                        .filter(|family| !family.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--source-id-filter" => {
                source_id_filter.extend(
                    value(&argv, i, "--source-id-filter")
                        .split(',')
                        .filter(|source_id| !source_id.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--max-candidates-per-case" => {
                max_candidates_per_case = value(&argv, i, "--max-candidates-per-case")
                    .parse()
                    .expect("--max-candidates-per-case must be a non-negative integer");
                i += 2;
            }
            "--enumeration" => {
                enumeration = match value(&argv, i, "--enumeration") {
                    "pruned-exact-binary64" => Enumeration::PrunedExactBinary64,
                    "unpruned" => Enumeration::Unpruned,
                    other => panic!(
                        "--enumeration must be pruned-exact-binary64 or unpruned, got {other}"
                    ),
                };
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        output,
        input_source,
        max_rows_per_family,
        generated_samples_per_facet,
        generated_seed,
        family_filter,
        source_id_filter,
        max_candidates_per_case,
        enumeration,
    }
}

fn value<'a>(argv: &'a [String], i: usize, flag: &str) -> &'a str {
    argv.get(i + 1)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{flag} requires a value"))
}

fn print_help() {
    println!(
        "Usage: qp-kkt-error-audit [--output PATH] [--input-source all|generated|artifacts|edge-fixtures]\n\
         [--max-rows-per-family N] [--generated-samples-per-facet N] [--generated-seed U64]\n\
         [--family-filter FAMILY[,FAMILY...]] [--source-id-filter SOURCE_ID[,SOURCE_ID...]]\n\
         [--max-candidates-per-case N] [--enumeration pruned-exact-binary64|unpruned]\n\
         N=0 means no cap for rows or candidates."
    );
}
