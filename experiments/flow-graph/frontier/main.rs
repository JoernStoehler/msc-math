//! Flow-graph frontier counts.
//!
//! Input artifact: experiments/combinatorial-cells/polytopes.jsonl by default.
//! Output artifact: JSONL to stdout, or to `--output <path>` when provided.
//!
//! Purpose: measure the combinatorial size of the transition-pruned half-cache
//! before implementing polygon/tube numerics.

use exp_combinatorial_cells::flat_polytope::CellPolytopeCache;
use exp_combinatorial_cells::name_from_record;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{
    build_tube_for_word_f64, counts_by_plus_depth, diagnose_f64_closed_words,
    enumerate_transition_pruned_words, half_cache_depth, reset_f64_polygon_metrics,
    split_closed_word_into_half_words, take_f64_polygon_metrics, F64TubeError, FlatTubeInput,
};
use symplectic::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use symplectic::database;

#[derive(Debug)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    max_facets: Option<usize>,
    build_f64_tubes: bool,
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
    f64_tube_live_count: Option<usize>,
    f64_tube_empty_count: Option<usize>,
    f64_tube_unsupported_count: Option<usize>,
    f64_tube_unexpected_error_count: Option<usize>,
    f64_closed_cycle_checked_count: Option<usize>,
    f64_closed_cycle_no_orbit_count: Option<usize>,
    f64_closed_cycle_error_count: Option<usize>,
    f64_candidate_orbit_count: Option<usize>,
    f64_candidate_best_action: Option<f64>,
    f64_live_start_inequality_sum: Option<usize>,
    f64_live_end_inequality_sum: Option<usize>,
    f64_live_start_inequality_max: Option<usize>,
    f64_live_end_inequality_max: Option<usize>,
    f64_polygon_is_empty_calls: Option<u64>,
    f64_polygon_contains_calls: Option<u64>,
    f64_polygon_contains_halfspace_checks: Option<u64>,
    f64_polygon_intersect_calls: Option<u64>,
    f64_polygon_with_halfspace_calls: Option<u64>,
    f64_polygon_pullback_calls: Option<u64>,
    f64_polygon_image_calls: Option<u64>,
    f64_polygon_vertices_calls: Option<u64>,
    f64_polygon_vertex_pair_checks: Option<u64>,
}

fn parse_args() -> Args {
    let mut input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../combinatorial-cells/polytopes.jsonl");
    let mut output = None;
    let mut max_facets = None;
    let mut build_f64_tubes = false;
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
            "--build-f64-tubes" => build_f64_tubes = true,
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-frontier [--input PATH] [--output PATH] [--max-facets N] [--build-f64-tubes]"
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
        build_f64_tubes,
    }
}

fn closed_raw_word(sigma: &[usize]) -> Vec<usize> {
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    word
}

fn frontier_row(
    polytope_name: String,
    polytope: &CellPolytopeCache,
    build_f64_tubes: bool,
) -> FrontierRow {
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
    let tube_input = FlatTubeInput::new(
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let supported_no_geometric_zero_omega = tube_input
        .validate_no_geometric_zero_omega_transitions()
        .is_ok();

    let (
        f64_tube_live_count,
        f64_tube_empty_count,
        f64_tube_unsupported_count,
        f64_tube_unexpected_error_count,
        f64_closed_cycle_checked_count,
        f64_closed_cycle_no_orbit_count,
        f64_closed_cycle_error_count,
        f64_candidate_orbit_count,
        f64_candidate_best_action,
        f64_live_start_inequality_sum,
        f64_live_end_inequality_sum,
        f64_live_start_inequality_max,
        f64_live_end_inequality_max,
        f64_polygon_is_empty_calls,
        f64_polygon_contains_calls,
        f64_polygon_contains_halfspace_checks,
        f64_polygon_intersect_calls,
        f64_polygon_with_halfspace_calls,
        f64_polygon_pullback_calls,
        f64_polygon_image_calls,
        f64_polygon_vertices_calls,
        f64_polygon_vertex_pair_checks,
    ) = if build_f64_tubes {
        reset_f64_polygon_metrics();
        let mut live = 0usize;
        let mut empty = 0usize;
        let mut unsupported = 0usize;
        let mut unexpected = 0usize;
        let mut start_inequality_sum = 0usize;
        let mut end_inequality_sum = 0usize;
        let mut start_inequality_max = 0usize;
        let mut end_inequality_max = 0usize;
        for word in &cached_words {
            match build_tube_for_word_f64(&tube_input, &word.facets, f64::INFINITY) {
                Ok(Some(tube)) => {
                    live += 1;
                    let start_count = tube.start_polygon().inequality_count();
                    let end_count = tube.end_polygon().inequality_count();
                    start_inequality_sum += start_count;
                    end_inequality_sum += end_count;
                    start_inequality_max = start_inequality_max.max(start_count);
                    end_inequality_max = end_inequality_max.max(end_count);
                }
                Ok(None) => empty += 1,
                Err(
                    F64TubeError::SingularTubeMap
                    | F64TubeError::UnsupportedDegenerateTransition
                    | F64TubeError::NumericallyUnstableOmegaTransition,
                ) => {
                    unsupported += 1;
                }
                Err(_) => unexpected += 1,
            }
        }
        let metrics = take_f64_polygon_metrics();
        let search = diagnose_f64_closed_words(&tube_input, 0.0);
        let (
            closed_cycle_checked_count,
            closed_cycle_no_orbit_count,
            closed_cycle_error_count,
            candidate_orbit_count,
            candidate_best_action,
        ) = match search {
            Ok(search) => (
                Some(search.checked_closed_word_count()),
                Some(search.no_orbit_count()),
                Some(search.closed_cycle_error_count()),
                Some(search.orbits.len()),
                search.best_action,
            ),
            Err(_) => (None, None, None, None, None),
        };
        (
            Some(live),
            Some(empty),
            Some(unsupported),
            Some(unexpected),
            closed_cycle_checked_count,
            closed_cycle_no_orbit_count,
            closed_cycle_error_count,
            candidate_orbit_count,
            candidate_best_action,
            Some(start_inequality_sum),
            Some(end_inequality_sum),
            Some(start_inequality_max),
            Some(end_inequality_max),
            Some(metrics.is_empty_calls),
            Some(metrics.contains_calls),
            Some(metrics.contains_halfspace_checks),
            Some(metrics.intersect_calls),
            Some(metrics.with_halfspace_calls),
            Some(metrics.pullback_calls),
            Some(metrics.image_calls),
            Some(metrics.vertices_calls),
            Some(metrics.vertex_pair_checks),
        )
    } else {
        (
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        )
    };

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
        f64_tube_live_count,
        f64_tube_empty_count,
        f64_tube_unsupported_count,
        f64_tube_unexpected_error_count,
        f64_closed_cycle_checked_count,
        f64_closed_cycle_no_orbit_count,
        f64_closed_cycle_error_count,
        f64_candidate_orbit_count,
        f64_candidate_best_action,
        f64_live_start_inequality_sum,
        f64_live_end_inequality_sum,
        f64_live_start_inequality_max,
        f64_live_end_inequality_max,
        f64_polygon_is_empty_calls,
        f64_polygon_contains_calls,
        f64_polygon_contains_halfspace_checks,
        f64_polygon_intersect_calls,
        f64_polygon_with_halfspace_calls,
        f64_polygon_pullback_calls,
        f64_polygon_image_calls,
        f64_polygon_vertices_calls,
        f64_polygon_vertex_pair_checks,
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
        let row = frontier_row(
            name_from_record(record, idx),
            &polytope,
            args.build_f64_tubes,
        );
        serde_json::to_writer(&mut writer, &row).expect("write frontier row");
        writeln!(&mut writer).expect("write newline");
    }
}
