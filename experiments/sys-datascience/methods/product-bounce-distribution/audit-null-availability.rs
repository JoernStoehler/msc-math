//! Bounded, exact audit of the rows whose retained class-minimum artifact has
//! no three-bounce value. This deliberately does not regenerate the full
//! class-minimum artifact.

use exp_sys_landscape::SysLandscapePolytopeCache;
use num_rational::BigRational;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::billiard::{
    bounce_count_from_sigma_for_facets, for_each_sigma_from_facets,
};
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_orbit_sigma_saddle_point, CertifiedOrbitSetMode, OrbitAdmissibility, OrbitKktData,
    OrbitSearchError, OrbitSolveError,
};

#[derive(Deserialize)]
struct RawRow {
    name: String,
    k: usize,
    m: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
}

#[derive(Serialize)]
struct AuditRow {
    name: String,
    k: usize,
    m: usize,
    enumerated_sigma_counts_by_bounces: BTreeMap<String, usize>,
    candidate_orbit_counts_by_bounces: BTreeMap<String, usize>,
    f64_inadmissible_sigma_counts_by_bounces: BTreeMap<String, usize>,
    f64_numerical_failure_counts_by_bounces: BTreeMap<String, usize>,
    exact_admissible_f64_rejected_counts_by_bounces: BTreeMap<String, usize>,
}

struct CandidateStats {
    enumerated: BTreeMap<String, usize>,
    solved: BTreeMap<String, usize>,
    inadmissible: BTreeMap<String, usize>,
    numerical_failure: BTreeMap<String, usize>,
    rejected_sigmas: BTreeMap<String, Vec<Vec<usize>>>,
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

fn exact_rejected_count(duals: &[[BigRational; 4]], sigmas: Vec<Vec<usize>>) -> usize {
    if sigmas.is_empty() {
        return 0;
    }
    let candidates = sigmas
        .into_iter()
        .map(|sigma| OrbitKktData {
            sigma,
            beta: Vec::new(),
            beta_margin: 0.0,
            action: 0.0,
            action_lower: 0.0,
            action_upper: f64::INFINITY,
            q: 0.0,
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility: OrbitAdmissibility::IndeterminateF64,
        })
        .collect();
    match aggregate_certified_orbits_with_dual_vertices_exact(
        duals,
        candidates,
        0,
        BigRational::from_integer(1_000_000_i32.into()),
        CertifiedOrbitSetMode::GapWindow,
    ) {
        Ok(result) => result.orbits.len(),
        Err(OrbitSearchError::NoAdmissibleOrbit) => 0,
        Err(error) => panic!("exact certification of f64-rejected sigmas failed: {error:?}"),
    }
}

fn audit_row(raw: RawRow) -> AuditRow {
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
    let mut stats = CandidateStats {
        enumerated: BTreeMap::new(),
        solved: BTreeMap::new(),
        inadmissible: BTreeMap::new(),
        numerical_failure: BTreeMap::new(),
        rejected_sigmas: BTreeMap::new(),
    };
    for_each_sigma_from_facets(
        &classification.q_indices,
        &classification.p_indices,
        &poly.facet_intersection_is_nonempty,
        &transitions,
        |sigma| {
            let bounce = bounce_count_from_sigma_for_facets(
                &classification.q_indices,
                &classification.p_indices,
                sigma,
            )
            .expect("enumerated sigma must have 2/3-bounce structure");
            let key = bounce.to_string();
            *stats.enumerated.entry(key.clone()).or_default() += 1;
            match solve_orbit_sigma_saddle_point(&poly.dual_vertices_f64, sigma) {
                Ok(_) => *stats.solved.entry(key).or_default() += 1,
                Err(OrbitSolveError::Inadmissible) => {
                    *stats.inadmissible.entry(key.clone()).or_default() += 1;
                    stats
                        .rejected_sigmas
                        .entry(key)
                        .or_default()
                        .push(sigma.to_vec());
                }
                Err(OrbitSolveError::NumericalFailure) => {
                    *stats.numerical_failure.entry(key).or_default() += 1;
                }
            }
        },
    );
    let mut exact_rejected = BTreeMap::new();
    for bounce in [2usize, 3] {
        let key = bounce.to_string();
        stats.enumerated.entry(key.clone()).or_default();
        stats.solved.entry(key.clone()).or_default();
        stats.inadmissible.entry(key.clone()).or_default();
        let sigmas = stats.rejected_sigmas.remove(&key).unwrap_or_default();
        exact_rejected.insert(key, exact_rejected_count(&poly.dual_vertices, sigmas));
    }
    AuditRow {
        name: raw.name,
        k: raw.k,
        m: raw.m,
        enumerated_sigma_counts_by_bounces: stats.enumerated,
        candidate_orbit_counts_by_bounces: stats.solved,
        f64_inadmissible_sigma_counts_by_bounces: stats.inadmissible,
        f64_numerical_failure_counts_by_bounces: stats.numerical_failure,
        exact_admissible_f64_rejected_counts_by_bounces: exact_rejected,
    }
}

struct Args {
    input: PathBuf,
    class_minima: PathBuf,
    output: PathBuf,
}

fn parse_args() -> Args {
    let mut input = None;
    let mut class_minima = None;
    let mut output = None;
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
            "--class-minima" => {
                class_minima = Some(PathBuf::from(value()));
                i += 2;
            }
            "--output" => {
                output = Some(PathBuf::from(value()));
                i += 2;
            }
            "--help" => {
                println!("Usage: sys-datascience-product-bounce-null-audit --input <random-product.jsonl> --class-minima <class-minima.jsonl> --output <audit.jsonl>");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        input: input.expect("--input is required"),
        class_minima: class_minima.expect("--class-minima is required"),
        output: output.expect("--output is required"),
    }
}

fn main() {
    let args = parse_args();
    let null_names: HashSet<String> =
        BufReader::new(File::open(&args.class_minima).expect("open class-minima artifact"))
            .lines()
            .map(|line| {
                serde_json::from_str::<serde_json::Value>(&line.expect("read class-minima line"))
                    .expect("parse class-minima row")
            })
            .filter(|row| row["class_minima"]["3"].is_null())
            .filter_map(|row| row["name"].as_str().map(ToOwned::to_owned))
            .collect();
    assert_eq!(
        null_names.len(),
        785,
        "expected the current 785-row A3-null slice"
    );
    let raws: Vec<RawRow> = BufReader::new(File::open(&args.input).expect("open raw input"))
        .lines()
        .map(|line| serde_json::from_str(&line.expect("read raw line")).expect("parse raw row"))
        .filter(|row: &RawRow| null_names.contains(&row.name))
        .collect();
    assert_eq!(
        raws.len(),
        null_names.len(),
        "null rows must join raw input exactly"
    );
    let rows: Vec<AuditRow> = raws.into_par_iter().map(audit_row).collect();
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent).expect("create output parent");
    }
    let mut output = BufWriter::new(File::create(&args.output).expect("create output"));
    for row in rows {
        writeln!(
            output,
            "{}",
            serde_json::to_string(&row).expect("serialize audit row")
        )
        .expect("write audit row");
    }
    output.flush().expect("flush audit output");
    eprintln!("wrote 785 null-row audit rows to {}", args.output.display());
}
