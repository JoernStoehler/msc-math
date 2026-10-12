//! Run-local computed-payload and sample-metadata input adapter.

use crate::load_caches::{LoadedCaches, LoadedPolytopeRow, LoadedProvenanceRow};
use crate::producer_rows::{
    DatascienceRandomProductSampleRow, DatascienceRandomSampleRow, DatascienceReferenceSampleRow,
    DatascienceSampleSource,
};
use exp_sys_landscape::ComputedPolytopePayloadRow;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct Args {
    pub produce_dir: PathBuf,
    pub out_dir: PathBuf,
}

pub fn parse_args_from(argv: &[String]) -> Args {
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
  reference-samples.jsonl

Output target metadata distinguishes dataset origin (`capacity_source`), the
requested producer backend, the actual capacity candidate family, and the
capacity/volume arithmetic methods. It does not certify historical inputs.
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

fn payloads_by_poly_id(path: &Path) -> HashMap<String, ComputedPolytopePayloadRow> {
    let mut out = HashMap::new();
    for row in read_jsonl::<ComputedPolytopePayloadRow>(path) {
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
    if let Some(existing) = polytopes.get(&payload.poly_id) {
        assert_eq!(
            existing.capacity_source, capacity_source,
            "poly_id {} appears under multiple capacity sources: {} and {}",
            payload.poly_id, existing.capacity_source, capacity_source
        );
    } else {
        polytopes.insert(
            payload.poly_id.clone(),
            LoadedPolytopeRow {
                poly_id: payload.poly_id.clone(),
                dual_vertices_rational: payload.dual_vertices_rational.clone(),
                facet_count: payload.facet_count,
                capacity: payload.capacity,
                volume: payload.volume,
                sys: payload.sys,
                capacity_source: capacity_source.to_string(),
                capacity_method: Some(payload.capacity_method.clone()),
                capacity_candidate_family: payload.candidate_family.clone(),
                volume_method: Some(payload.volume_method.clone()),
            },
        );
    }
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

pub fn load_new_producer_outputs(produce_dir: &Path) -> LoadedCaches {
    let payloads = payloads_by_poly_id(&produce_dir.join("computed-polytopes.jsonl"));
    let random_rows = read_jsonl_if_exists::<DatascienceRandomSampleRow>(
        &produce_dir.join("random-samples.jsonl"),
    );
    let product_rows = read_jsonl_if_exists::<DatascienceRandomProductSampleRow>(
        &produce_dir.join("random-product-samples.jsonl"),
    );
    let reference_rows = read_jsonl_if_exists::<DatascienceReferenceSampleRow>(
        &produce_dir.join("reference-samples.jsonl"),
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
        sample_index.expect("run-local random source must include sample_index");
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
        assert_eq!(
            row.facet_count, payload.facet_count,
            "random-product sample {} facet_count disagrees with payload",
            row.name
        );
        assert_eq!(
            k.checked_add(m)
                .expect("product source facet_count overflow"),
            payload.facet_count,
            "random-product sample {} source k+m disagrees with payload facet_count",
            row.name
        );
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
        sample_index.expect("run-local random-product source must include sample_index");
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

    for row in reference_rows {
        sample_poly_ids.insert(row.poly_id.clone());
        let provenance_id = provenance_id("known_hko_reference", &row.name);
        assert!(
            provenance_ids.insert(provenance_id.clone()),
            "duplicate provenance_id {provenance_id}"
        );
        let payload = require_payload(
            &payloads,
            "known_hko_reference",
            &row.name,
            &row.poly_id,
            row.sys,
        );
        ensure_polytope(&mut polytopes, payload, "known_hko_reference");
        let DatascienceSampleSource::KnownHkoReference {
            fixture,
            source,
            role,
        } = row.source
        else {
            panic!("reference sample {} has non-reference source", row.name);
        };
        assert_eq!(fixture, "hko_pentagon", "unexpected reference fixture");
        assert_eq!(role, "reference_holdout", "unexpected reference role");
        let source_row = DatascienceSampleSource::KnownHkoReference {
            fixture,
            source,
            role: role.clone(),
        };
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id,
            poly_id: row.poly_id,
            dataset: "known_hko_reference".to_string(),
            family: "known_reference".to_string(),
            role,
            search_space: "hko_pentagon".to_string(),
            optimizer: "none".to_string(),
            backend: payload.backend.clone(),
            source_name: row.name.clone(),
            root_group_id: "known_hko_reference:hko_pentagon".to_string(),
            source: Some(source_value(&source_row)),
            sample_seed: None,
            sample_attempt: None,
            sample_h_min: None,
            sample_h_max: None,
            product_k: None,
            product_m: None,
            product_bounces: None,
            seed_index: None,
            lineage_id: Some("known_hko_reference:hko_pentagon".to_string()),
            path: Some("known_hko_reference/hko_pentagon".to_string()),
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
