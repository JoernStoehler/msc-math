//! Prepare method-facing datascience tables from run-local producer outputs.
//!
//! This command consumes producer metadata plus computed-polytope payloads. It
//! does not run capacity search.

mod canonize;
mod features_trace;
mod invariant_features;
mod load_caches;
mod rows;
mod write_database;

#[path = "../produce/rows.rs"]
mod producer_rows;

use exp_sys_landscape::ComputedPolytopePayloadRow;
use load_caches::{LoadedCaches, LoadedPolytopeRow, LoadedProvenanceRow};
use producer_rows::{
    DatascienceRandomProductSampleRow, DatascienceRandomSampleRow, DatascienceSampleSource,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Instant;

struct Args {
    produce_dir: PathBuf,
    out_dir: PathBuf,
}

#[derive(Serialize)]
struct PrepareStatsRow {
    produce_dir: String,
    out_dir: String,
    polytope_rows: usize,
    provenance_rows: usize,
    max_sys: Option<f64>,
    sys_gt_one: usize,
    build_polytope_table_ms: f64,
    wall_time_ms: f64,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help(
            argv.first()
                .map(String::as_str)
                .unwrap_or("sys-datascience-prepare"),
        );
        std::process::exit(0);
    }

    let mut produce_dir = None;
    let mut out_dir = None;
    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .unwrap_or_else(|| panic!("{flag} requires a value"));
        match flag {
            "--produce-dir" => {
                produce_dir = Some(PathBuf::from(value));
                i += 2;
            }
            "--out-dir" => {
                out_dir = Some(PathBuf::from(value));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        produce_dir: produce_dir.expect("--produce-dir is required"),
        out_dir: out_dir.expect("--out-dir is required"),
    }
}

fn print_help(program: &str) {
    println!(
        "\
Prepare datascience tables from run-local producer outputs.

Usage:
  {program} --produce-dir <dir> --out-dir <prepare-dir>

Inputs in <dir>:
  computed-polytopes.jsonl
  random-samples.jsonl and/or random-product-samples.jsonl
"
    );
}

fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let line = line
                .unwrap_or_else(|e| panic!("read {} line {}: {e}", path.display(), line_idx + 1));
            if line.trim().is_empty() {
                None
            } else {
                Some(serde_json::from_str::<T>(&line).unwrap_or_else(|e| {
                    panic!("parse {} line {}: {e}", path.display(), line_idx + 1)
                }))
            }
        })
        .collect()
}

fn read_jsonl_if_exists<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    if path.exists() {
        read_jsonl(path)
    } else {
        Vec::new()
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create stats parent");
    }
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

fn payloads_by_poly_id(path: &Path) -> HashMap<String, ComputedPolytopePayloadRow> {
    let mut out = HashMap::new();
    for row in read_jsonl::<ComputedPolytopePayloadRow>(path) {
        assert!(
            row.sys <= 1.0,
            "computed payload {} has sys > 1: {}",
            row.poly_id,
            row.sys
        );
        if let Some(previous) = out.insert(row.poly_id.clone(), row) {
            panic!(
                "duplicate computed payload for poly_id {}",
                previous.poly_id
            );
        }
    }
    out
}

fn ensure_polytope(
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    payload: &ComputedPolytopePayloadRow,
    capacity_source: &str,
) {
    polytopes
        .entry(payload.poly_id.clone())
        .or_insert_with(|| LoadedPolytopeRow {
            poly_id: payload.poly_id.clone(),
            dual_vertices_rational: payload.dual_vertices_rational.clone(),
            facet_count: payload.facet_count,
            capacity: payload.capacity,
            volume: payload.volume,
            sys: payload.sys,
            capacity_source: capacity_source.to_string(),
        });
}

fn require_payload<'a>(
    payloads: &'a HashMap<String, ComputedPolytopePayloadRow>,
    dataset: &str,
    name: &str,
    poly_id: &str,
    reported_sys: f64,
) -> &'a ComputedPolytopePayloadRow {
    let payload = payloads
        .get(poly_id)
        .unwrap_or_else(|| panic!("{dataset}:{name}: missing computed payload {poly_id}"));
    if (payload.sys - reported_sys).abs() > 1e-8 {
        panic!(
            "{dataset}:{name}: sample sys {} disagrees with payload sys {} for {poly_id}",
            reported_sys, payload.sys
        );
    }
    payload
}

fn provenance_id(dataset: &str, name: &str) -> String {
    format!("{dataset}:{name}")
}

fn source_value(source: &DatascienceSampleSource) -> serde_json::Value {
    serde_json::to_value(source).expect("serialize datascience sample source")
}

fn load_new_producer_outputs(produce_dir: &Path) -> LoadedCaches {
    let payloads = payloads_by_poly_id(&produce_dir.join("computed-polytopes.jsonl"));
    let random_rows = read_jsonl_if_exists::<DatascienceRandomSampleRow>(
        &produce_dir.join("random-samples.jsonl"),
    );
    let product_rows = read_jsonl_if_exists::<DatascienceRandomProductSampleRow>(
        &produce_dir.join("random-product-samples.jsonl"),
    );

    let mut polytopes = HashMap::new();
    let mut provenance_rows = Vec::new();
    let mut sample_poly_ids = HashSet::new();
    let mut provenance_ids = HashSet::new();

    for row in random_rows {
        sample_poly_ids.insert(row.poly_id.clone());
        let provenance_id = provenance_id("random_sample", &row.name);
        assert!(
            provenance_ids.insert(provenance_id.clone()),
            "duplicate provenance_id {provenance_id}"
        );
        let payload = require_payload(&payloads, "random_sample", &row.name, &row.poly_id, row.sys);
        ensure_polytope(&mut polytopes, payload, "random_sample");
        let DatascienceSampleSource::Random {
            facet_count: source_facet_count,
            h_min,
            h_max,
            seed,
            sample_index,
            attempt,
        } = row.source
        else {
            panic!("random sample {} has non-random source", row.name);
        };
        assert_eq!(
            source_facet_count, payload.facet_count,
            "random sample {} source facet_count disagrees with payload",
            row.name
        );
        let source = DatascienceSampleSource::Random {
            facet_count: source_facet_count,
            h_min,
            h_max,
            seed,
            sample_index,
            attempt,
        };
        let seed = seed.expect("run-local random source must include seed");
        let attempt = attempt.expect("run-local random source must include attempt");
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id,
            poly_id: row.poly_id,
            dataset: "random_sample".to_string(),
            family: "general".to_string(),
            role: "random_sample".to_string(),
            search_space: "general".to_string(),
            optimizer: "none".to_string(),
            backend: payload.backend.clone(),
            source_name: row.name.clone(),
            root_group_id: format!("random_sample:{}", row.name),
            source: Some(source_value(&source)),
            sample_seed: Some(seed),
            sample_attempt: Some(attempt),
            sample_h_min: Some(h_min),
            sample_h_max: Some(h_max),
            product_k: None,
            product_m: None,
            product_bounces: None,
            seed_index: Some(attempt as usize),
            lineage_id: Some(format!("seed:{seed}:attempt:{attempt}")),
            path: None,
            total_time_ms: None,
        });
    }

    for row in product_rows {
        sample_poly_ids.insert(row.poly_id.clone());
        let provenance_id = provenance_id("random_product_sample", &row.name);
        assert!(
            provenance_ids.insert(provenance_id.clone()),
            "duplicate provenance_id {provenance_id}"
        );
        let payload = require_payload(
            &payloads,
            "random_product_sample",
            &row.name,
            &row.poly_id,
            row.sys,
        );
        ensure_polytope(&mut polytopes, payload, "random_product_sample");
        let DatascienceSampleSource::RandomProduct {
            k,
            m,
            h_min,
            h_max,
            seed,
            sample_index,
            attempt,
            bounces,
        } = row.source
        else {
            panic!("random-product sample {} has non-product source", row.name);
        };
        let source = DatascienceSampleSource::RandomProduct {
            k,
            m,
            h_min,
            h_max,
            seed,
            sample_index,
            attempt,
            bounces,
        };
        let seed = seed.expect("run-local random-product source must include seed");
        let attempt = attempt.expect("run-local random-product source must include attempt");
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id,
            poly_id: row.poly_id,
            dataset: "random_product_sample".to_string(),
            family: "lagrangian_product".to_string(),
            role: "random_product_sample".to_string(),
            search_space: "lagrangian_product".to_string(),
            optimizer: "none".to_string(),
            backend: payload.backend.clone(),
            source_name: row.name.clone(),
            root_group_id: format!("random_product_sample:{}", row.name),
            source: Some(source_value(&source)),
            sample_seed: Some(seed),
            sample_attempt: Some(attempt),
            sample_h_min: Some(h_min),
            sample_h_max: Some(h_max),
            product_k: Some(k),
            product_m: Some(m),
            product_bounces: Some(bounces),
            seed_index: Some(attempt as usize),
            lineage_id: Some(format!("seed:{seed}:{k}x{m}:attempt:{attempt}")),
            path: Some(format!("lp_{k}x{m}")),
            total_time_ms: None,
        });
    }

    let mut polytope_rows = polytopes.into_values().collect::<Vec<_>>();
    polytope_rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    provenance_rows.sort_by(|a, b| a.provenance_id.cmp(&b.provenance_id));
    assert!(
        !provenance_rows.is_empty(),
        "prepare input has no producer metadata rows"
    );
    assert_eq!(
        payloads.len(),
        sample_poly_ids.len(),
        "computed payload count must match unique sample poly_id count"
    );

    LoadedCaches {
        polytopes: polytope_rows,
        provenance_rows,
    }
}

fn main() {
    let args = parse_args();
    let total_started = Instant::now();
    eprintln!(
        "Loading produced artifacts from {}",
        args.produce_dir.display()
    );
    let caches = load_new_producer_outputs(&args.produce_dir);
    eprintln!(
        "Loaded {} polytopes and {} provenance rows",
        caches.polytopes.len(),
        caches.provenance_rows.len()
    );

    let started = Instant::now();
    let polytope_rows = invariant_features::build_polytope_table(&caches.polytopes);
    let build_polytope_table_ms = started.elapsed().as_secs_f64() * 1000.0;
    eprintln!(
        "Built invariant polytope table in {:.1}s",
        build_polytope_table_ms / 1000.0
    );

    let provenance_run_rows = features_trace::build_provenance_run_table(&caches.provenance_rows);
    write_database::write_database(&args.out_dir, &polytope_rows, &provenance_run_rows);
    let wall_time_ms = total_started.elapsed().as_secs_f64() * 1000.0;
    let stats = PrepareStatsRow {
        produce_dir: args.produce_dir.display().to_string(),
        out_dir: args.out_dir.display().to_string(),
        polytope_rows: polytope_rows.len(),
        provenance_rows: caches.provenance_rows.len(),
        max_sys: polytope_rows.iter().map(|row| row.sys).reduce(f64::max),
        sys_gt_one: polytope_rows.iter().filter(|row| row.sys > 1.0).count(),
        build_polytope_table_ms,
        wall_time_ms,
    };
    write_json(&args.out_dir.join("prepare-stats.json"), &stats);
    eprintln!("Total prepare time {:.1}s", wall_time_ms / 1000.0);
    println!("Wrote {}", args.out_dir.display());
}
