//! Load producer outputs and merge them into unified datascience input rows.

use blake3::Hasher;
use exp_sys_landscape::{package_root, rational_vec4_to_strings, SummaryRow, TraceRow};
#[path = "../produce/rows.rs"]
mod rows;
use rows::{RandomProductRow, RandomSweepRow, ResultRow};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use symplectic::database::{load_many, OrbitScalars, PolytopeRecord, SigmaAction};

#[derive(Clone)]
pub struct LoadedPolytopeRow {
    pub poly_id: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub capacity_iterations: Option<u64>,
    pub capacity_source: String,
    pub sigma_gap_cutoff: Option<f64>,
    pub sigmas: Option<Vec<SigmaAction>>,
    pub orbit_scalars: Option<OrbitScalars>,
}

#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub phase: usize,
    pub iteration: usize,
    pub step_type: String,
    pub t_fraction: f64,
    pub t_actual: f64,
    pub sys_before: f64,
    pub sys_after: f64,
    pub delta_sys: f64,
    pub gradient_norm: f64,
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
    pub seed_index: Option<usize>,
    pub lineage_id: Option<String>,
    pub parent_provenance_id: Option<String>,
    pub rq: Option<String>,
    pub path: Option<String>,
    pub starting_f: Option<usize>,
    pub starting_sys: Option<f64>,
    pub reported_final_sys: Option<f64>,
    pub reported_delta: Option<f64>,
    pub sys_after_addition: Option<f64>,
    pub n_iterations: Option<usize>,
    pub n_phases: Option<usize>,
    pub best_strategy: Option<String>,
    pub n_escape_overshoot: Option<usize>,
    pub n_escape_wiggle: Option<usize>,
    pub placement_direction: Option<[f64; 4]>,
    pub facet_remained_active: Option<bool>,
    pub total_time_ms: Option<f64>,
    pub trace_events: Vec<TraceEvent>,
}

pub struct LoadedCaches {
    pub polytopes: Vec<LoadedPolytopeRow>,
    pub provenance_rows: Vec<LoadedProvenanceRow>,
}

#[derive(Clone)]
struct OrbitPayload {
    capacity: Option<f64>,
    volume: Option<f64>,
    sigmas: Option<Vec<SigmaAction>>,
    sigma_gap_cutoff: Option<f64>,
    orbit_scalars: Option<OrbitScalars>,
}

pub struct DatasetPaths {
    pub random_sample: PathBuf,
    pub random_product: PathBuf,
    pub ascent_summary: PathBuf,
    pub ascent_trace: PathBuf,
    pub ascent_cache: PathBuf,
    pub ascent_product_summary: PathBuf,
    pub ascent_product_trace: PathBuf,
    pub ascent_product_cache: PathBuf,
    pub continuation_summary: PathBuf,
    pub shared_cache: PathBuf,
    pub continuation_cache: PathBuf,
    pub out_dir: PathBuf,
}

fn print_help(program: &str) {
    println!(
        "\
Build sys-landscape datascience tables from producer caches.

Usage:
  {program} --out-dir <tables-dir> [options]

Method-wave output:
  experiments/sys-landscape/datascience/tables

Options:
  --produce-dir <dir>              Read canonical producer filenames from <dir>
  --random <path>                  Override random.jsonl
  --random-product <path>          Override random-product.jsonl
  --ascent <path>                  Override ascent-general-endpoints.jsonl
  --ascent-trace <path>            Override ascent-general-trace.jsonl
  --ascent-cache <path>            Override ascent-general-cache.jsonl
  --ascent-product <path>          Override ascent-product-endpoints.jsonl
  --ascent-product-trace <path>    Override ascent-product-trace.jsonl
  --ascent-product-cache <path>    Override ascent-product-cache.jsonl
  --continuation <path>            Override continuation.jsonl
  --shared-cache <path>            Override shared-cache.jsonl
  --continuation-cache <path>      Override continuation-cache.jsonl
  --out-dir <tables-dir>           Output directory for retained table files
  --help                           Show this help

If --out-dir is omitted, this command writes to a temporary smoke directory.
Use that only for one-off scratch. For method waves, use an owned path under
experiments/sys-landscape/datascience/tables/.
"
    );
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
    let produce_dir = package_root().join("datascience/produce");
    DatasetPaths {
        random_sample: produce_dir.join("random.jsonl"),
        random_product: produce_dir.join("random-product.jsonl"),
        ascent_summary: produce_dir.join("ascent-general-endpoints.jsonl"),
        ascent_trace: produce_dir.join("ascent-general-trace.jsonl"),
        ascent_cache: produce_dir.join("ascent-general-cache.jsonl"),
        ascent_product_summary: produce_dir.join("ascent-product-endpoints.jsonl"),
        ascent_product_trace: produce_dir.join("ascent-product-trace.jsonl"),
        ascent_product_cache: produce_dir.join("ascent-product-cache.jsonl"),
        continuation_summary: produce_dir.join("continuation.jsonl"),
        shared_cache: produce_dir.join("shared-cache.jsonl"),
        continuation_cache: produce_dir.join("continuation-cache.jsonl"),
        out_dir: smoke_output_dir(),
    }
}

pub fn parse_args() -> DatasetPaths {
    let defaults = default_paths();
    let mut random_sample = defaults.random_sample;
    let mut random_product = defaults.random_product;
    let mut ascent_summary = defaults.ascent_summary;
    let mut ascent_trace = defaults.ascent_trace;
    let mut ascent_cache = defaults.ascent_cache;
    let mut ascent_product_summary = defaults.ascent_product_summary;
    let mut ascent_product_trace = defaults.ascent_product_trace;
    let mut ascent_product_cache = defaults.ascent_product_cache;
    let mut continuation_summary = defaults.continuation_summary;
    let mut shared_cache = defaults.shared_cache;
    let mut continuation_cache = defaults.continuation_cache;
    let mut out_dir = defaults.out_dir;

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help(args.first().map(String::as_str).unwrap_or("sys-dataset"));
        std::process::exit(0);
    }
    let mut i = 1usize;
    while i < args.len() {
        let flag = args[i].as_str();
        let value = args
            .get(i + 1)
            .unwrap_or_else(|| panic!("{flag} requires a value"));
        match flag {
            "--produce-dir" => {
                let dir = PathBuf::from(value);
                random_sample = dir.join("random.jsonl");
                random_product = dir.join("random-product.jsonl");
                ascent_summary = dir.join("ascent-general-endpoints.jsonl");
                ascent_trace = dir.join("ascent-general-trace.jsonl");
                ascent_cache = dir.join("ascent-general-cache.jsonl");
                ascent_product_summary = dir.join("ascent-product-endpoints.jsonl");
                ascent_product_trace = dir.join("ascent-product-trace.jsonl");
                ascent_product_cache = dir.join("ascent-product-cache.jsonl");
                continuation_summary = dir.join("continuation.jsonl");
                shared_cache = dir.join("shared-cache.jsonl");
                continuation_cache = dir.join("continuation-cache.jsonl");
                i += 2;
            }
            "--random" => {
                random_sample = PathBuf::from(value);
                i += 2;
            }
            "--random-product" => {
                random_product = PathBuf::from(value);
                i += 2;
            }
            "--ascent" => {
                ascent_summary = PathBuf::from(value);
                i += 2;
            }
            "--ascent-trace" => {
                ascent_trace = PathBuf::from(value);
                i += 2;
            }
            "--ascent-cache" => {
                ascent_cache = PathBuf::from(value);
                i += 2;
            }
            "--ascent-product" => {
                ascent_product_summary = PathBuf::from(value);
                i += 2;
            }
            "--ascent-product-trace" => {
                ascent_product_trace = PathBuf::from(value);
                i += 2;
            }
            "--ascent-product-cache" => {
                ascent_product_cache = PathBuf::from(value);
                i += 2;
            }
            "--continuation" => {
                continuation_summary = PathBuf::from(value);
                i += 2;
            }
            "--shared-cache" => {
                shared_cache = PathBuf::from(value);
                i += 2;
            }
            "--continuation-cache" => {
                continuation_cache = PathBuf::from(value);
                i += 2;
            }
            "--out-dir" => {
                out_dir = PathBuf::from(value);
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    DatasetPaths {
        random_sample,
        random_product,
        ascent_summary,
        ascent_trace,
        ascent_cache,
        ascent_product_summary,
        ascent_product_trace,
        ascent_product_cache,
        continuation_summary,
        shared_cache,
        continuation_cache,
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

fn read_jsonl_if_exists<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    if path.exists() {
        read_jsonl(path)
    } else {
        Vec::new()
    }
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

fn orbit_payloads_from_paths(cache_paths: &[&Path]) -> HashMap<String, OrbitPayload> {
    if cache_paths.is_empty() {
        return HashMap::new();
    }

    let db = load_many(cache_paths).expect("load producer caches");
    let mut out = HashMap::new();
    for (_key, record) in db {
        let poly_id =
            poly_id_from_dual_vertices(&rational_vec4_to_strings(&record.dual_vertices_rational));
        let sigma_gap_cutoff = sigma_gap_cutoff(&record);
        out.insert(
            poly_id,
            OrbitPayload {
                capacity: record.capacity,
                volume: record.volume,
                sigmas: record.sigmas,
                sigma_gap_cutoff,
                orbit_scalars: record.orbit_scalars,
            },
        );
    }
    out
}

fn existing_paths<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Vec<&'a Path> {
    paths.into_iter().filter(|path| path.exists()).collect()
}

fn orbit_payloads(paths: &DatasetPaths) -> HashMap<String, OrbitPayload> {
    let cache_paths = existing_paths([
        paths.shared_cache.as_path(),
        paths.ascent_cache.as_path(),
        paths.ascent_product_cache.as_path(),
        paths.continuation_cache.as_path(),
    ]);
    orbit_payloads_from_paths(&cache_paths)
}

fn orbit_payloads_for_path(path: &Path) -> HashMap<String, OrbitPayload> {
    let cache_paths = existing_paths([path]);
    orbit_payloads_from_paths(&cache_paths)
}

fn sigma_gap_cutoff(record: &PolytopeRecord) -> Option<f64> {
    let sigmas = record.sigmas.as_ref()?;
    if sigmas.len() < 2 {
        return Some(0.0);
    }
    let best = sigmas.first()?.action;
    let next = sigmas.get(1)?.action;
    Some(next - best)
}

fn ensure_polytope(
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
    dual_vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
    reported_capacity: f64,
    volume: f64,
    sys: f64,
    capacity_iterations: Option<u64>,
    capacity_source: &str,
) -> String {
    let poly_id = poly_id_from_dual_vertices(&dual_vertices_rational);
    let orbit_payload = orbit_payloads.get(&poly_id);
    let capacity = orbit_payload
        .and_then(|row| row.capacity)
        .unwrap_or(reported_capacity);
    let volume = orbit_payload.and_then(|row| row.volume).unwrap_or(volume);
    match polytopes.get_mut(&poly_id) {
        Some(existing) => {
            if existing.capacity_iterations.is_none() {
                existing.capacity_iterations = capacity_iterations;
            }
            if existing.capacity <= 0.0 {
                existing.capacity = capacity;
            }
            if existing.sigmas.is_none() {
                existing.sigmas = orbit_payload.and_then(|row| row.sigmas.clone());
            }
            if existing.orbit_scalars.is_none() {
                existing.orbit_scalars = orbit_payload.and_then(|row| row.orbit_scalars.clone());
            }
        }
        None => {
            polytopes.insert(
                poly_id.clone(),
                LoadedPolytopeRow {
                    poly_id: poly_id.clone(),
                    dual_vertices_rational,
                    facet_count,
                    capacity,
                    volume,
                    sys,
                    capacity_iterations,
                    capacity_source: capacity_source.to_string(),
                    sigma_gap_cutoff: orbit_payload.and_then(|row| row.sigma_gap_cutoff),
                    sigmas: orbit_payload.and_then(|row| row.sigmas.clone()),
                    orbit_scalars: orbit_payload.and_then(|row| row.orbit_scalars.clone()),
                },
            );
        }
    }
    poly_id
}

fn root_group(dataset: &str, source_name: &str, lineage_id: &str, name: &str) -> String {
    if !lineage_id.is_empty() {
        return lineage_id.to_string();
    }
    if !source_name.is_empty() {
        return source_name.to_string();
    }
    format!("{dataset}:{name}")
}

fn provenance_id(dataset: &str, name: &str) -> String {
    format!("{dataset}:{name}")
}

fn load_random_sample_rows(
    path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
) {
    for row in read_jsonl::<RandomSweepRow>(path) {
        let poly_id = ensure_polytope(
            polytopes,
            orbit_payloads,
            row.dual_vertices_rational,
            row.facet_count,
            row.capacity,
            row.volume,
            row.sys,
            Some(row.iterations),
            "random_sample",
        );
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id: provenance_id("random_sample", &row.name),
            poly_id,
            dataset: "random_sample".to_string(),
            family: "general".to_string(),
            role: "random_sample".to_string(),
            search_space: "general".to_string(),
            optimizer: "none".to_string(),
            backend: "ehz_capacity".to_string(),
            source_name: row.name.clone(),
            root_group_id: root_group("random_sample", &row.name, "", &row.name),
            seed_index: None,
            lineage_id: None,
            parent_provenance_id: None,
            rq: None,
            path: None,
            starting_f: None,
            starting_sys: None,
            reported_final_sys: None,
            reported_delta: None,
            sys_after_addition: None,
            n_iterations: None,
            n_phases: None,
            best_strategy: None,
            n_escape_overshoot: None,
            n_escape_wiggle: None,
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: None,
            trace_events: Vec::new(),
        });
    }
}

fn load_random_product_rows(
    path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
) {
    for row in read_jsonl::<RandomProductRow>(path) {
        let poly_id = ensure_polytope(
            polytopes,
            orbit_payloads,
            row.dual_vertices_rational,
            row.facet_count,
            row.capacity,
            row.volume,
            row.sys,
            Some(row.iterations),
            "random_product_sample",
        );
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id: provenance_id("random_product_sample", &row.name),
            poly_id,
            dataset: "random_product_sample".to_string(),
            family: "lagrangian_product".to_string(),
            role: "random_sample".to_string(),
            search_space: "lagrangian_product".to_string(),
            optimizer: "none".to_string(),
            backend: "ehz_capacity_billiard".to_string(),
            source_name: row.name.clone(),
            root_group_id: root_group("random_product_sample", &row.name, "", &row.name),
            seed_index: None,
            lineage_id: None,
            parent_provenance_id: None,
            rq: None,
            path: Some(format!("lp_{}x{}", row.k, row.m)),
            starting_f: None,
            starting_sys: None,
            reported_final_sys: None,
            reported_delta: None,
            sys_after_addition: None,
            n_iterations: None,
            n_phases: None,
            best_strategy: None,
            n_escape_overshoot: None,
            n_escape_wiggle: None,
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: None,
            trace_events: Vec::new(),
        });
    }
}

fn trace_events_by_name(path: &Path) -> HashMap<String, Vec<TraceEvent>> {
    read_jsonl_if_exists::<TraceRow>(path).into_iter().fold(
        HashMap::<String, Vec<TraceEvent>>::new(),
        |mut acc, row| {
            acc.entry(row.name.clone()).or_default().push(TraceEvent {
                phase: row.phase,
                iteration: row.iteration,
                step_type: row.step_type,
                t_fraction: row.t_fraction,
                t_actual: row.t_actual,
                sys_before: row.sys_before,
                sys_after: row.sys_after,
                delta_sys: row.delta_sys,
                gradient_norm: row.gradient_norm,
            });
            acc
        },
    )
}

fn load_ascent_rows(
    summary_path: &Path,
    trace_path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
    ascent_orbit_payloads: &HashMap<String, OrbitPayload>,
) {
    let trace_events = trace_events_by_name(trace_path);
    for row in read_jsonl::<SummaryRow>(summary_path) {
        let payload = require_endpoint_payload(
            "gradient_ascent_general",
            &row.name,
            &row.final_dual_vertices_rational,
            row.final_capacity,
            row.final_volume,
            row.final_sys,
            ascent_orbit_payloads,
        );
        let poly_id = ensure_polytope(
            polytopes,
            orbit_payloads,
            row.final_dual_vertices_rational.clone(),
            row.facet_count,
            payload.capacity,
            payload.volume,
            row.final_sys,
            None,
            "gradient_ascent_general",
        );
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id: provenance_id("gradient_ascent_general", &row.name),
            poly_id,
            dataset: "gradient_ascent_general".to_string(),
            family: "general".to_string(),
            role: "ascent_endpoint".to_string(),
            search_space: "general".to_string(),
            optimizer: "gradient_ascent".to_string(),
            backend: "ehz_capacity".to_string(),
            source_name: row.source_name.clone(),
            root_group_id: root_group(
                "gradient_ascent_general",
                &row.source_name,
                &row.lineage_id,
                &row.name,
            ),
            seed_index: Some(row.seed_index),
            lineage_id: (!row.lineage_id.is_empty()).then_some(row.lineage_id.clone()),
            parent_provenance_id: None,
            rq: None,
            path: None,
            starting_f: Some(row.facet_count),
            starting_sys: Some(row.starting_sys),
            reported_final_sys: Some(row.final_sys),
            reported_delta: Some(row.total_delta),
            sys_after_addition: None,
            n_iterations: Some(row.n_gradient_iters_total),
            n_phases: Some(row.n_ascent_phases),
            best_strategy: Some(row.best_strategy.clone()),
            n_escape_overshoot: Some(row.n_escape_overshoot),
            n_escape_wiggle: Some(row.n_escape_wiggle),
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: Some(row.total_time_ms),
            trace_events: trace_events.get(&row.name).cloned().unwrap_or_default(),
        });
    }
}

fn load_ascent_product_rows(
    summary_path: &Path,
    trace_path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
    ascent_product_orbit_payloads: &HashMap<String, OrbitPayload>,
) {
    let trace_events = trace_events_by_name(trace_path);
    for row in read_jsonl::<SummaryRow>(summary_path) {
        let payload = require_endpoint_payload(
            "gradient_ascent_products",
            &row.name,
            &row.final_dual_vertices_rational,
            row.final_capacity,
            row.final_volume,
            row.final_sys,
            ascent_product_orbit_payloads,
        );
        let poly_id = ensure_polytope(
            polytopes,
            orbit_payloads,
            row.final_dual_vertices_rational.clone(),
            row.facet_count,
            payload.capacity,
            payload.volume,
            row.final_sys,
            None,
            "gradient_ascent_products",
        );
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id: provenance_id("gradient_ascent_products", &row.name),
            poly_id,
            dataset: "gradient_ascent_products".to_string(),
            family: "lagrangian_product".to_string(),
            role: "ascent_endpoint".to_string(),
            search_space: "lagrangian_product".to_string(),
            optimizer: "projected_gradient_ascent".to_string(),
            backend: "ehz_capacity".to_string(),
            source_name: row.source_name.clone(),
            root_group_id: root_group(
                "gradient_ascent_products",
                &row.source_name,
                &row.lineage_id,
                &row.name,
            ),
            seed_index: Some(row.seed_index),
            lineage_id: (!row.lineage_id.is_empty()).then_some(row.lineage_id.clone()),
            parent_provenance_id: None,
            rq: None,
            path: None,
            starting_f: Some(row.facet_count),
            starting_sys: Some(row.starting_sys),
            reported_final_sys: Some(row.final_sys),
            reported_delta: Some(row.total_delta),
            sys_after_addition: None,
            n_iterations: Some(row.n_gradient_iters_total),
            n_phases: Some(row.n_ascent_phases),
            best_strategy: Some(row.best_strategy.clone()),
            n_escape_overshoot: Some(row.n_escape_overshoot),
            n_escape_wiggle: Some(row.n_escape_wiggle),
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: Some(row.total_time_ms),
            trace_events: trace_events.get(&row.name).cloned().unwrap_or_default(),
        });
    }
}

fn load_continuation_rows(
    path: &Path,
    polytopes: &mut HashMap<String, LoadedPolytopeRow>,
    provenance_rows: &mut Vec<LoadedProvenanceRow>,
    orbit_payloads: &HashMap<String, OrbitPayload>,
) {
    for row in read_jsonl::<ResultRow>(path) {
        let poly_id = ensure_polytope(
            polytopes,
            orbit_payloads,
            row.final_dual_vertices_rational.clone(),
            row.final_dual_vertices_rational.len(),
            0.0,
            0.0,
            row.final_sys,
            None,
            "variable_f_ascent",
        );
        let parent_provenance_id = row
            .direct_parent_trial
            .as_ref()
            .map(|parent| provenance_id("variable_f_ascent", parent));
        provenance_rows.push(LoadedProvenanceRow {
            provenance_id: provenance_id("variable_f_ascent", &row.name),
            poly_id,
            dataset: "variable_f_ascent".to_string(),
            family: "general".to_string(),
            role: "continuation_endpoint".to_string(),
            search_space: "general".to_string(),
            optimizer: "gradient_ascent".to_string(),
            backend: "ehz_capacity".to_string(),
            source_name: row.source_name.clone(),
            root_group_id: root_group(
                "variable_f_ascent",
                &row.source_name,
                &row.lineage_id,
                &row.name,
            ),
            seed_index: None,
            lineage_id: (!row.lineage_id.is_empty()).then_some(row.lineage_id.clone()),
            parent_provenance_id,
            rq: Some(row.rq.clone()),
            path: Some(row.path.clone()),
            starting_f: Some(row.starting_f),
            starting_sys: Some(row.starting_sys),
            reported_final_sys: Some(row.final_sys),
            reported_delta: Some(row.delta_vs_source),
            sys_after_addition: row.sys_after_addition,
            n_iterations: Some(row.n_iterations),
            n_phases: Some(row.n_phases),
            best_strategy: None,
            n_escape_overshoot: None,
            n_escape_wiggle: None,
            placement_direction: row.placement_direction,
            facet_remained_active: row.facet_remained_active,
            total_time_ms: Some(row.total_time_ms),
            trace_events: Vec::new(),
        });
    }
}

pub fn load_caches(paths: &DatasetPaths) -> LoadedCaches {
    let orbit_payloads = orbit_payloads(paths);
    let ascent_orbit_payloads = orbit_payloads_for_path(&paths.ascent_cache);
    let ascent_product_orbit_payloads = orbit_payloads_for_path(&paths.ascent_product_cache);
    let mut polytopes = HashMap::<String, LoadedPolytopeRow>::new();
    let mut provenance_rows = Vec::<LoadedProvenanceRow>::new();

    load_random_sample_rows(
        &paths.random_sample,
        &mut polytopes,
        &mut provenance_rows,
        &orbit_payloads,
    );
    load_random_product_rows(
        &paths.random_product,
        &mut polytopes,
        &mut provenance_rows,
        &orbit_payloads,
    );
    load_ascent_rows(
        &paths.ascent_summary,
        &paths.ascent_trace,
        &mut polytopes,
        &mut provenance_rows,
        &orbit_payloads,
        &ascent_orbit_payloads,
    );
    load_ascent_product_rows(
        &paths.ascent_product_summary,
        &paths.ascent_product_trace,
        &mut polytopes,
        &mut provenance_rows,
        &orbit_payloads,
        &ascent_product_orbit_payloads,
    );
    load_continuation_rows(
        &paths.continuation_summary,
        &mut polytopes,
        &mut provenance_rows,
        &orbit_payloads,
    );

    let mut polytope_rows = polytopes.into_values().collect::<Vec<_>>();
    polytope_rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    provenance_rows.sort_by(|a, b| a.provenance_id.cmp(&b.provenance_id));

    LoadedCaches {
        polytopes: polytope_rows,
        provenance_rows,
    }
}

struct RequiredEndpointPayload {
    capacity: f64,
    volume: f64,
}

fn require_endpoint_payload(
    dataset: &str,
    name: &str,
    final_dual_vertices_rational: &[[String; 4]],
    final_capacity: f64,
    final_volume: f64,
    final_sys: f64,
    orbit_payloads: &HashMap<String, OrbitPayload>,
) -> RequiredEndpointPayload {
    let poly_id = poly_id_from_dual_vertices(final_dual_vertices_rational);
    let payload = orbit_payloads.get(&poly_id).unwrap_or_else(|| {
        panic!("{dataset}:{name}: missing producer-cache row for endpoint {poly_id}")
    });
    let capacity = payload
        .capacity
        .unwrap_or_else(|| panic!("{dataset}:{name}: producer-cache row lacks capacity"));
    let volume = payload
        .volume
        .unwrap_or_else(|| panic!("{dataset}:{name}: producer-cache row lacks volume"));
    if payload.sigmas.is_none() {
        panic!("{dataset}:{name}: producer-cache row lacks sigmas");
    }
    if payload.orbit_scalars.is_none() {
        panic!("{dataset}:{name}: producer-cache row lacks orbit scalars");
    }
    if final_capacity > 0.0 && (final_capacity - capacity).abs() > 1e-9 {
        panic!(
            "{dataset}:{name}: summary final_capacity {final_capacity} disagrees with cache capacity {capacity}"
        );
    }
    if final_volume > 0.0 && (final_volume - volume).abs() > 1e-9 {
        panic!(
            "{dataset}:{name}: summary final_volume {final_volume} disagrees with cache volume {volume}"
        );
    }
    let sys = capacity * capacity / (2.0 * volume);
    if (sys - final_sys).abs() > 1e-8 {
        panic!(
            "{dataset}:{name}: summary final_sys {final_sys} disagrees with cache-derived sys {sys}"
        );
    }
    RequiredEndpointPayload { capacity, volume }
}
