//! Load random/product producer outputs into unified datascience input rows.

use crate::producer_rows::{DatascienceSampleSource, RandomProductRow, RandomSweepRow};
use blake3::Hasher;
use exp_sys_landscape::package_root;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct LoadedPolytopeRow {
    pub poly_id: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub capacity_source: String,
}

#[derive(Clone)]
pub struct LoadedProvenanceRow {
    pub provenance_id: String,
    pub poly_id: String,
    pub dataset: String,
    pub family: String,
    pub role: String,
    pub search_space: String,
    pub optimizer: String,
    pub backend: String,
    pub source_name: String,
    pub root_group_id: String,
    pub source: Option<serde_json::Value>,
    pub sample_seed: Option<u64>,
    pub sample_attempt: Option<u64>,
    pub sample_h_min: Option<f64>,
    pub sample_h_max: Option<f64>,
    pub product_k: Option<usize>,
    pub product_m: Option<usize>,
    pub product_bounces: Option<usize>,
    pub seed_index: Option<usize>,
    pub lineage_id: Option<String>,
    pub path: Option<String>,
    pub total_time_ms: Option<f64>,
}

pub struct LoadedCaches {
    pub polytopes: Vec<LoadedPolytopeRow>,
    pub provenance_rows: Vec<LoadedProvenanceRow>,
}

pub struct DatasetPaths {
    pub max_random_rows: Option<usize>,
    pub max_random_product_rows: Option<usize>,
    pub random_sample: PathBuf,
    pub random_product: PathBuf,
    pub out_dir: PathBuf,
}

fn print_help(program: &str) {
    println!(
        "\
Build random/product sys-landscape datascience prepared tables.

Usage:
  {program} --out-dir <prepare-dir> [options]

Options:
  --random-only                   Accepted for compatibility; this loader is always random/product only
  --random-only-size <name>       Named size: smoke, method, or full
  --max-random-rows <n>           Override random_sample row count for development
  --max-random-product-rows <n>   Override random_product_sample row count for development
  --produce-dir <dir>             Read random.jsonl and random-product.jsonl from <dir>
  --random <path>                 Override random.jsonl
  --random-product <path>         Override random-product.jsonl
  --out-dir <prepare-dir>         Output directory for prepared tables
  --help                          Show this help

If --out-dir is omitted, this command writes to a temporary smoke directory.
"
    );
}

fn random_only_size_limits(name: &str) -> (Option<usize>, Option<usize>) {
    match name {
        "smoke" => (Some(8), Some(20)),
        "method" => (Some(512), Some(1024)),
        "full" => (None, None),
        other => panic!("unknown --random-only-size {other}; expected smoke, method, or full"),
    }
}

fn smoke_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("sys-dataset-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp output dir");
    dir
}

fn default_paths() -> DatasetPaths {
    let produce_dir = package_root().join("../sys-datascience/produce");
    DatasetPaths {
        max_random_rows: None,
        max_random_product_rows: None,
        random_sample: produce_dir.join("random.jsonl"),
        random_product: produce_dir.join("random-product.jsonl"),
        out_dir: smoke_output_dir(),
    }
}

pub fn parse_args() -> DatasetPaths {
    let defaults = default_paths();
    let mut max_random_rows = defaults.max_random_rows;
    let mut max_random_product_rows = defaults.max_random_product_rows;
    let mut random_sample = defaults.random_sample;
    let mut random_product = defaults.random_product;
    let mut out_dir = defaults.out_dir;

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help(args.first().map(String::as_str).unwrap_or("sys-dataset"));
        std::process::exit(0);
    }
    let mut i = 1usize;
    while i < args.len() {
        let flag = args[i].as_str();
        match flag {
            "--random-only" => {
                i += 1;
            }
            "--random-only-size" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                (max_random_rows, max_random_product_rows) = random_only_size_limits(value);
                i += 2;
            }
            "--max-random-rows" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                max_random_rows = Some(
                    value
                        .parse()
                        .unwrap_or_else(|e| panic!("parse --max-random-rows {value}: {e}")),
                );
                i += 2;
            }
            "--max-random-product-rows" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                max_random_product_rows =
                    Some(value.parse().unwrap_or_else(|e| {
                        panic!("parse --max-random-product-rows {value}: {e}")
                    }));
                i += 2;
            }
            "--produce-dir" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                let dir = PathBuf::from(value);
                random_sample = dir.join("random.jsonl");
                random_product = dir.join("random-product.jsonl");
                i += 2;
            }
            "--random" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                random_sample = PathBuf::from(value);
                i += 2;
            }
            "--random-product" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                random_product = PathBuf::from(value);
                i += 2;
            }
            "--out-dir" => {
                let value = args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("{flag} requires a value"));
                out_dir = PathBuf::from(value);
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    DatasetPaths {
        max_random_rows,
        max_random_product_rows,
        random_sample,
        random_product,
        out_dir,
    }
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

fn poly_id_from_dual_vertices(dual_vertices_rational: &[[String; 4]]) -> String {
    let mut hasher = Hasher::new();
    for row in dual_vertices_rational {
        for coord in row {
            hasher.update(coord.as_bytes());
            hasher.update(&[0]);
        }
    }
    hasher.finalize().to_hex().to_string()
}

fn ensure_polytope(
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    dual_vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
    capacity: f64,
    volume: f64,
    sys: f64,
    capacity_source: &str,
) -> String {
    let poly_id = poly_id_from_dual_vertices(&dual_vertices_rational);
    polytopes
        .entry(poly_id.clone())
        .or_insert_with(|| LoadedPolytopeRow {
            poly_id: poly_id.clone(),
            dual_vertices_rational,
            facet_count,
            capacity,
            volume,
            sys,
            capacity_source: capacity_source.to_string(),
        });
    poly_id
}

fn provenance_id(dataset: &str, name: &str) -> String {
    format!("{dataset}:{name}")
}

fn empty_provenance(
    provenance_id: String,
    poly_id: String,
    dataset: &str,
    family: &str,
    search_space: &str,
    backend: &str,
    source_name: String,
) -> LoadedProvenanceRow {
    LoadedProvenanceRow {
        provenance_id,
        poly_id,
        dataset: dataset.to_string(),
        family: family.to_string(),
        role: "random_sample".to_string(),
        search_space: search_space.to_string(),
        optimizer: "none".to_string(),
        backend: backend.to_string(),
        source_name: source_name.clone(),
        root_group_id: source_name,
        source: None,
        sample_seed: None,
        sample_attempt: None,
        sample_h_min: None,
        sample_h_max: None,
        product_k: None,
        product_m: None,
        product_bounces: None,
        seed_index: None,
        lineage_id: None,
        path: None,
        total_time_ms: None,
    }
}

fn source_value(source: &DatascienceSampleSource) -> serde_json::Value {
    serde_json::to_value(source).expect("serialize datascience sample source")
}

fn load_random_sample_rows(
    path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    limit: Option<usize>,
) {
    let mut rows = read_jsonl::<RandomSweepRow>(path);
    if let Some(limit) = limit {
        stratified_prefix_sample(&mut rows, limit, |row| row.facet_count, |row| &row.name);
    }
    for row in rows {
        let name = row.name.clone();
        let h_min = row.h_min;
        let h_max = row.h_max;
        let poly_id = ensure_polytope(
            polytopes,
            row.dual_vertices_rational,
            row.facet_count,
            row.capacity,
            row.volume,
            row.sys,
            "random_sample",
        );
        let mut provenance = empty_provenance(
            provenance_id("random_sample", &name),
            poly_id,
            "random_sample",
            "general",
            "general",
            "ehz_capacity",
            name,
        );
        provenance.sample_seed = row.seed;
        provenance.sample_attempt = row.attempt;
        provenance.sample_h_min = Some(h_min);
        provenance.sample_h_max = Some(h_max);
        provenance.source = Some(source_value(&DatascienceSampleSource::Random {
            facet_count: row.facet_count,
            h_min,
            h_max,
            seed: row.seed,
            sample_index: None,
            attempt: row.attempt,
        }));
        provenance.seed_index = row.attempt.map(|attempt| attempt as usize);
        if let (Some(seed), Some(attempt)) = (row.seed, row.attempt) {
            provenance.lineage_id = Some(format!("seed:{seed}:attempt:{attempt}"));
        }
        provenance_rows.push(provenance);
    }
}

fn load_random_product_rows(
    path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    limit: Option<usize>,
) {
    let mut rows = read_jsonl::<RandomProductRow>(path);
    if let Some(limit) = limit {
        stratified_prefix_sample(
            &mut rows,
            limit,
            |row| (row.k, row.m, row.bounces),
            |row| &row.name,
        );
    }
    for row in rows {
        let name = row.name.clone();
        let h_min = row.h_min;
        let h_max = row.h_max;
        let k = row.k;
        let m = row.m;
        let bounces = row.bounces;
        let poly_id = ensure_polytope(
            polytopes,
            row.dual_vertices_rational,
            row.facet_count,
            row.capacity,
            row.volume,
            row.sys,
            "random_product_sample",
        );
        let mut provenance = empty_provenance(
            provenance_id("random_product_sample", &name),
            poly_id,
            "random_product_sample",
            "lagrangian_product",
            "lagrangian_product",
            "ehz_capacity_billiard",
            name,
        );
        provenance.role = "random_product_sample".to_string();
        provenance.sample_seed = row.seed;
        provenance.sample_attempt = row.attempt;
        provenance.sample_h_min = Some(h_min);
        provenance.sample_h_max = Some(h_max);
        provenance.product_k = Some(k);
        provenance.product_m = Some(m);
        provenance.product_bounces = Some(bounces);
        provenance.source = Some(source_value(&DatascienceSampleSource::RandomProduct {
            k,
            m,
            h_min,
            h_max,
            seed: row.seed,
            sample_index: None,
            attempt: row.attempt,
            bounces,
        }));
        provenance.seed_index = row.attempt.map(|attempt| attempt as usize);
        if let (Some(seed), Some(attempt)) = (row.seed, row.attempt) {
            provenance.lineage_id = Some(format!("seed:{seed}:attempt:{attempt}"));
        }
        provenance.path = Some(format!("lp_{k}x{m}"));
        provenance_rows.push(provenance);
    }
}

fn stratified_prefix_sample<T, K>(
    rows: &mut Vec<T>,
    limit: usize,
    key: impl Fn(&T) -> K,
    name: impl Fn(&T) -> &str,
) where
    K: Ord,
{
    if rows.len() <= limit {
        return;
    }
    let mut strata = BTreeMap::<K, Vec<T>>::new();
    for row in rows.drain(..) {
        strata.entry(key(&row)).or_default().push(row);
    }
    for stratum in strata.values_mut() {
        stratum.sort_by(|left, right| name(left).cmp(name(right)));
    }

    let stratum_count = strata.len().max(1);
    let base = limit / stratum_count;
    let remainder = limit % stratum_count;
    let mut sampled = Vec::with_capacity(limit);
    for (index, (_, mut stratum)) in strata.into_iter().enumerate() {
        let take = (base + usize::from(index < remainder)).min(stratum.len());
        sampled.extend(stratum.drain(..take));
    }
    sampled.sort_by(|left, right| name(left).cmp(name(right)));
    rows.extend(sampled);
}

pub fn load_caches(paths: &DatasetPaths) -> LoadedCaches {
    let mut polytopes = HashMap::new();
    let mut provenance_rows = Vec::new();
    load_random_sample_rows(
        &paths.random_sample,
        &mut polytopes,
        &mut provenance_rows,
        paths.max_random_rows,
    );
    load_random_product_rows(
        &paths.random_product,
        &mut polytopes,
        &mut provenance_rows,
        paths.max_random_product_rows,
    );

    let mut polytope_rows = polytopes.into_values().collect::<Vec<_>>();
    polytope_rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    provenance_rows.sort_by(|a, b| a.provenance_id.cmp(&b.provenance_id));

    LoadedCaches {
        polytopes: polytope_rows,
        provenance_rows,
    }
}
