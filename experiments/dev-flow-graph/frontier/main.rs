//! Flow-graph frontier counts.
//!
//! Input artifact: experiments/combinatorial-cells/polytopes.jsonl by default.
//! Output artifact: JSONL to stdout, or to `--output <path>` when provided.
//!
//! Purpose: measure the combinatorial size of the transition-pruned half-cache
//! and closed-word frontier without depending on tube arithmetic.

use exp_combinatorial_cells::flat_polytope::CellPolytopeCache;
use exp_combinatorial_cells::name_from_record;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{
    counts_by_plus_depth, enumerate_transition_pruned_words, half_cache_depth,
    split_closed_word_into_half_words,
};
use symplectic::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use symplectic::database;

#[derive(Debug)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    max_facets: Option<usize>,
}

#[derive(Debug, Serialize)]
struct FrontierRow {
    polytope_name: String,
    facet_count: usize,
    half_depth: usize,
    directed_transition_edges: usize,
    half_cache_total: usize,
    half_cache_counts_by_plus_depth: Vec<usize>,
    closed_cycle_count: usize,
    closed_split_missing_count: usize,
    supported_no_geometric_zero_omega: bool,
}

fn parse_args() -> Args {
    let mut input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../combinatorial-cells/polytopes.jsonl");
    let mut output = None;
    let mut max_facets = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                input = args
                    .next()
                    .map(PathBuf::from)
                    .expect("--input requires a path");
            }
            "--output" => {
                output = Some(
                    args.next()
                        .map(PathBuf::from)
                        .expect("--output requires a path"),
                );
            }
            "--max-facets" => {
                max_facets = Some(
                    args.next()
                        .expect("--max-facets requires a value")
                        .parse()
                        .expect("--max-facets must be a usize"),
                );
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-frontier [--input PATH] [--output PATH] [--max-facets N]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }

    Args {
        input,
        output,
        max_facets,
    }
}

fn closed_raw_word(sigma: &[usize]) -> Vec<usize> {
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    word
}

fn frontier_row(polytope_name: String, polytope: &CellPolytopeCache) -> FrontierRow {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let facet_count = polytope.facet_count();
    let half_depth = half_cache_depth(facet_count);
    let cached_words = enumerate_transition_pruned_words(&transition_is_allowed, half_depth);
    let counts_by_plus = counts_by_plus_depth(&cached_words, half_depth);
    let cached_word_set: HashSet<Vec<usize>> = cached_words
        .iter()
        .map(|word| word.facets.clone())
        .collect();
    let directed_transition_edges = transition_is_allowed
        .iter()
        .filter(|is_allowed| **is_allowed)
        .count();

    let mut closed_cycle_count = 0usize;
    let mut closed_split_missing_count = 0usize;
    for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
        closed_cycle_count += 1;
        let closed = closed_raw_word(sigma);
        match split_closed_word_into_half_words(&closed, half_depth) {
            Some((left, right))
                if cached_word_set.contains(&left) && cached_word_set.contains(&right) => {}
            _ => closed_split_missing_count += 1,
        }
    });
    let supported_no_geometric_zero_omega = (0..facet_count).all(|first| {
        (0..facet_count).all(|second| {
            first == second
                || !polytope.facet_intersection_is_nonempty[(first, second)]
                || polytope.omega_signs[(first, second)] != 0
        })
    });

    FrontierRow {
        polytope_name,
        facet_count,
        half_depth,
        directed_transition_edges,
        half_cache_total: cached_words.len(),
        half_cache_counts_by_plus_depth: counts_by_plus,
        closed_cycle_count,
        closed_split_missing_count,
        supported_no_geometric_zero_omega,
    }
}

fn main() {
    let args = parse_args();
    let db = database::load_many(&[args.input.as_path()]).expect("load polytope database");

    let writer: Box<dyn Write> = match &args.output {
        Some(path) => {
            Box::new(BufWriter::new(File::create(path).unwrap_or_else(|err| {
                panic!("create {}: {err}", path.display())
            })))
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };
    let mut writer = writer;

    for (idx, (_, record)) in db.iter().enumerate() {
        let facet_count = record.dual_vertices_rational.len();
        if args
            .max_facets
            .is_some_and(|max_facets| facet_count > max_facets)
        {
            continue;
        }
        let Some(polytope) = CellPolytopeCache::from_rational_parts(
            record.dual_vertices_rational.clone(),
            record.vertices_rational.clone(),
        ) else {
            eprintln!("skip row {idx}: could not reconstruct polytope");
            continue;
        };
        let row = frontier_row(name_from_record(record, idx), &polytope);
        serde_json::to_writer(&mut writer, &row).expect("write frontier row");
        writeln!(&mut writer).expect("write newline");
    }
}
