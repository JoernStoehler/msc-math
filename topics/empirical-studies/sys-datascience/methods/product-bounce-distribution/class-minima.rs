//! Exact class-specific billiard minima on retained random-product rows.
//!
//! This is a retained-data audit.  For each stored product polytope it
//! runs the existing f64 billiard candidate stream used by the producer,
//! separates its admissible solved candidates by their 2/3-bounce word
//! structure, and exactly certifies the minimizers within each class. The JSONL output is deliberately detailed;
//! the Python summary consumer owns aggregate descriptive statistics.

use exp_sys_landscape::SysLandscapePolytopeCache;
use num_rational::BigRational;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, CertifiedOrbitSetMode,
};

#[derive(Deserialize)]
struct RawRow {
    name: String,
    k: usize,
    m: usize,
    bounces: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    volume: f64,
    capacity: f64,
    sys: f64,
}

#[derive(Serialize)]
struct ClassMinimum {
    action: f64,
    action_exact: String,
    minimizer_count: usize,
    minimizer_sigmas: Vec<Vec<usize>>,
    exact_resolutions: usize,
}

#[derive(Serialize)]
struct OutputRow {
    name: String,
    k: usize,
    m: usize,
    producer_bounces: usize,
    stored_volume: f64,
    stored_capacity: f64,
    stored_sys: f64,
    candidate_iterations: u64,
    candidate_orbit_count: usize,
    candidate_orbit_counts_by_bounces: BTreeMap<String, usize>,
    active_vertex_counts: BTreeMap<String, Vec<usize>>,
    /// Null means this class had no admissible solved candidate for this row;
    /// it is not a zero or an inferred minimum.
    class_minima: BTreeMap<String, Option<ClassMinimum>>,
    /// `(A3 - A2) / A2`, where A_b is the exact b-bounce class minimum.
    normalized_three_minus_two_gap: Option<f64>,
    /// Minimum cyclic normal-angle gap in each factor, in radians.
    q_min_angular_gap_rad: f64,
    p_min_angular_gap_rad: f64,
    product_min_angular_gap_rad: f64,
}

struct Args {
    input: PathBuf,
    output: PathBuf,
    limit: Option<usize>,
}

fn parse_args() -> Args {
    let mut input = None;
    let mut output = None;
    let mut limit = None;
    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        let value = || {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{} requires a value", argv[i]))
        };
        match argv[i].as_str() {
            "--input" => {
                input = Some(PathBuf::from(value()));
                i += 2;
            }
            "--output" => {
                output = Some(PathBuf::from(value()));
                i += 2;
            }
            "--limit" => {
                limit = Some(value().parse().expect("--limit must be usize"));
                i += 2;
            }
            "--help" => {
                println!("Usage: sys-datascience-product-bounce-class-minima --input <random-product.jsonl> --output <class-minima.jsonl> [--limit <n>]");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        input: input.expect("--input is required"),
        output: output.expect("--output is required"),
        limit,
    }
}

fn rational(value: &str) -> BigRational {
    let (numerator, denominator) = value.split_once('/').expect("rational must contain /");
    BigRational::new(
        numerator.parse().expect("rational numerator"),
        denominator.parse().expect("rational denominator"),
    )
}

fn rational_vecs(rows: Vec<[String; 4]>) -> Vec<[BigRational; 4]> {
    rows.into_iter()
        .map(|row| row.map(|x| rational(&x)))
        .collect()
}

fn minimum_angle_gap(
    duals: &[nalgebra::Vector4<f64>],
    indices: &[usize],
    first_coordinate: usize,
) -> f64 {
    let mut angles: Vec<f64> = indices
        .iter()
        .map(|&i| {
            let a = duals[i];
            a[first_coordinate + 1]
                .atan2(a[first_coordinate])
                .rem_euclid(std::f64::consts::TAU)
        })
        .collect();
    angles.sort_by(|a, b| a.total_cmp(b));
    (0..angles.len())
        .map(|i| (angles[(i + 1) % angles.len()] - angles[i]).rem_euclid(std::f64::consts::TAU))
        .fold(f64::INFINITY, f64::min)
}

fn solve_row(raw: RawRow) -> OutputRow {
    let poly = SysLandscapePolytopeCache::from_rational_parts(
        rational_vecs(raw.dual_vertices_rational),
        rational_vecs(raw.vertices_rational),
    )
    .expect("stored rational geometry must reconstruct");
    let classification = classify_facets_from_dual_vertices(&poly.dual_vertices_f64)
        .expect("stored random-product row must classify");
    let transitions = build_transition_matrix_from_facet_intersections_and_omega(
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    );
    let (orbits, iterations) = solve_billiard_candidates(
        &poly.dual_vertices_f64,
        &classification.q_indices,
        &classification.p_indices,
        &poly.facet_intersection_is_nonempty,
        &transitions,
    )
    .expect("billiard candidates must solve");

    let mut by_bounce = BTreeMap::<usize, Vec<_>>::new();
    for orbit in &orbits {
        let bounces = bounce_count_from_sigma_for_facets(
            &classification.q_indices,
            &classification.p_indices,
            &orbit.sigma,
        )
        .expect("billiard enumeration must have 2/3-bounce structure");
        by_bounce.entry(bounces).or_default().push(orbit.clone());
    }
    let mut class_minima = BTreeMap::new();
    let mut active_vertex_counts = BTreeMap::new();
    let mut candidate_orbit_counts_by_bounces = BTreeMap::new();
    for bounces in [2usize, 3] {
        let candidates = by_bounce.remove(&bounces).unwrap_or_default();
        candidate_orbit_counts_by_bounces.insert(bounces.to_string(), candidates.len());
        if candidates.is_empty() {
            active_vertex_counts.insert(bounces.to_string(), Vec::new());
            class_minima.insert(bounces.to_string(), None);
            continue;
        }
        let result = aggregate_certified_orbits_with_dual_vertices_exact(
            &poly.dual_vertices,
            candidates,
            iterations,
            BigRational::from_integer(0.into()),
            CertifiedOrbitSetMode::MinimizersOnly,
        )
        .expect("each bounce class must have an exact minimum");
        active_vertex_counts.insert(
            bounces.to_string(),
            result.minimizers.iter().map(|o| o.sigma.len()).collect(),
        );
        class_minima.insert(
            bounces.to_string(),
            Some(ClassMinimum {
                action: result.capacity,
                action_exact: result.capacity_exact.to_string(),
                minimizer_count: result.minimizers.len(),
                minimizer_sigmas: result.minimizers.into_iter().map(|o| o.sigma).collect(),
                exact_resolutions: result.exact_resolutions,
            }),
        );
    }
    let normalized_three_minus_two_gap = match (&class_minima["2"], &class_minima["3"]) {
        (Some(a2), Some(a3)) => Some((a3.action - a2.action) / a2.action),
        _ => None,
    };
    let q_gap = minimum_angle_gap(&poly.dual_vertices_f64, &classification.q_indices, 0);
    let p_gap = minimum_angle_gap(&poly.dual_vertices_f64, &classification.p_indices, 2);
    OutputRow {
        name: raw.name,
        k: raw.k,
        m: raw.m,
        producer_bounces: raw.bounces,
        stored_volume: raw.volume,
        stored_capacity: raw.capacity,
        stored_sys: raw.sys,
        candidate_iterations: iterations,
        candidate_orbit_count: orbits.len(),
        candidate_orbit_counts_by_bounces,
        active_vertex_counts,
        class_minima,
        normalized_three_minus_two_gap,
        q_min_angular_gap_rad: q_gap,
        p_min_angular_gap_rad: p_gap,
        product_min_angular_gap_rad: q_gap.min(p_gap),
    }
}

fn main() {
    let args = parse_args();
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent).expect("create output parent");
    }
    let input = BufReader::new(File::open(&args.input).expect("open input"));
    let mut output = BufWriter::new(File::create(&args.output).expect("create output"));
    let raws: Vec<RawRow> = input
        .lines()
        .take(args.limit.unwrap_or(usize::MAX))
        .map(|line| serde_json::from_str(&line.expect("read input line")).expect("parse raw row"))
        .collect();
    let count = raws.len();
    let rows: Vec<OutputRow> = raws.into_par_iter().map(solve_row).collect();
    for row in rows {
        writeln!(
            output,
            "{}",
            serde_json::to_string(&row).expect("serialize output row")
        )
        .expect("write output row");
    }
    output.flush().expect("flush output");
    eprintln!("wrote {count} rows to {}", args.output.display());
}
