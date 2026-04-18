//! Normalize the hostile-landscape source packets into core joinable tables.
//!
//! Goal: convert the current random/ascent JSONLs into four durable core tables:
//! `polytopes.jsonl`, `states.jsonl`, `capacity_results.jsonl`, and
//! `step_events.jsonl`.
//! Input Artifacts: experiments/sys-landscape/cache.jsonl
//!         experiments/combinatorial-cells/polytopes.jsonl
//!         experiments/sys-landscape/variable-f-ascent/cache.jsonl
//!         experiments/sys-landscape/random-sample/random-sweep.jsonl
//!         experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl
//!         experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl
//!         experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl
//!         experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl
//!         experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products-trace.jsonl
//!         experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl
//! Output Artifacts: None by default (writes to an untracked temp directory unless `--out-dir` is set)
//!
//! Default dataset boundary:
//! - include random generic samples
//! - include random Lagrangian products
//! - include fixed-F ascent endpoints and their step-event logs
//! - include variable-F continuation endpoints
//! - exclude HKO-near control packets
//!
//! Source-priority rule for exact geometry:
//! 1. exact cache records
//! 2. exact dual-vertex payloads on summary rows, if they identify a cached polytope
//! 3. legacy `f64` dual-vertex matching into the exact caches
//!
//! The converter does not invent intermediate geometry. `step_events.jsonl`
//! stays an event log keyed by endpoint `state_id`.

use blake3::Hasher;
use exp_sys_landscape::{rational_vec4_to_strings, SummaryRow, TraceRow};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use symplectic::database::{load, PolytopeRecord};
use symplectic::ehz_capacity;
use symplectic::geom::volume::volume;

const MATCH_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Deserialize)]
struct RandomSweepRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    #[serde(default)]
    dual_vertices_rational: Vec<[String; 4]>,
    capacity: f64,
    volume: f64,
    sys: f64,
    iterations: u64,
}

#[derive(Debug, Deserialize)]
struct RandomProductRow {
    name: String,
    k: usize,
    m: usize,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    #[serde(default)]
    dual_vertices_rational: Vec<[String; 4]>,
    capacity: f64,
    volume: f64,
    sys: f64,
    iterations: u64,
}

#[derive(Debug, Deserialize)]
struct VariableFRow {
    rq: String,
    path: String,
    name: String,
    #[serde(default)]
    source_name: String,
    #[serde(default)]
    lineage_id: String,
    #[serde(default)]
    direct_parent_trial: Option<String>,
    starting_f: usize,
    starting_sys: f64,
    #[serde(default)]
    sys_after_addition: Option<f64>,
    final_sys: f64,
    delta_vs_source: f64,
    n_iterations: usize,
    n_phases: usize,
    #[serde(default)]
    placement_direction: Option<[f64; 4]>,
    #[serde(default)]
    facet_remained_active: Option<bool>,
    total_time_ms: f64,
    #[serde(default)]
    starting_dual_vertices_rational: Vec<[String; 4]>,
    #[serde(default)]
    after_addition_dual_vertices_rational: Option<Vec<[String; 4]>>,
    #[serde(default)]
    final_dual_vertices_rational: Vec<[String; 4]>,
    final_dual_vertices: Vec<[f64; 4]>,
}

#[derive(Debug, Serialize)]
struct PolytopeRow {
    poly_id: String,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    facet_count: usize,
    geometry_source: String,
}

#[derive(Debug, Serialize)]
struct StateRow {
    state_id: String,
    poly_id: String,
    dataset: String,
    family: String,
    role: String,
    search_space: String,
    optimizer: String,
    backend: String,
    source_name: String,
    root_group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_state_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_f: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reported_final_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reported_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sys_after_addition: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_iterations: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_phases: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_escape_overshoot: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_escape_wiggle: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_direction: Option<[f64; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_remained_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_time_ms: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
struct CapacityResultRow {
    poly_id: String,
    capacity: f64,
    volume: f64,
    sys: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<u64>,
    search_result_source: String,
}

#[derive(Debug, Serialize)]
struct StepEventRow {
    state_id: String,
    phase: usize,
    iteration: usize,
    step_type: String,
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    gradient_norm: f64,
}

#[derive(Clone)]
struct ExactCacheEntry {
    poly_id: String,
    record: PolytopeRecord,
    facet_count: usize,
    dual_vertices_f64: Vec<[f64; 4]>,
    geometry_source: String,
}

struct DatasetPaths {
    random_sample: PathBuf,
    random_product: PathBuf,
    general_summary: PathBuf,
    general_trace: PathBuf,
    products_summary: PathBuf,
    products_trace: PathBuf,
    variable_f: PathBuf,
    out_dir: PathBuf,
}

impl DatasetPaths {
    fn defaults(package_root: &Path) -> Self {
        Self {
            random_sample: package_root.join("random-sample/random-sweep.jsonl"),
            random_product: package_root.join("random-product-sample/random-product-sweep.jsonl"),
            general_summary: package_root
                .join("gradient-ascent-general/gradient-ascent-general.jsonl"),
            general_trace: package_root
                .join("gradient-ascent-general/gradient-ascent-general-trace.jsonl"),
            products_summary: package_root
                .join("gradient-ascent-products/gradient-ascent-products.jsonl"),
            products_trace: package_root
                .join("gradient-ascent-products/gradient-ascent-products-trace.jsonl"),
            variable_f: package_root.join("variable-f-ascent/variable-f-ascent.jsonl"),
            out_dir: smoke_output_dir(),
        }
    }
}

fn smoke_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!(
        "sys-normalized-dataset-{}-{stamp}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp output dir");
    dir
}

fn parse_args(package_root: &Path) -> DatasetPaths {
    let mut paths = DatasetPaths::defaults(package_root);
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out-dir" => {
                let value = args.get(i + 1).expect("--out-dir requires a value");
                paths.out_dir = PathBuf::from(value);
                i += 2;
            }
            "--general-summary" => {
                let value = args.get(i + 1).expect("--general-summary requires a value");
                paths.general_summary = PathBuf::from(value);
                i += 2;
            }
            "--general-trace" => {
                let value = args.get(i + 1).expect("--general-trace requires a value");
                paths.general_trace = PathBuf::from(value);
                i += 2;
            }
            "--products-summary" => {
                let value = args
                    .get(i + 1)
                    .expect("--products-summary requires a value");
                paths.products_summary = PathBuf::from(value);
                i += 2;
            }
            "--products-trace" => {
                let value = args.get(i + 1).expect("--products-trace requires a value");
                paths.products_trace = PathBuf::from(value);
                i += 2;
            }
            "--variable-f" => {
                let value = args.get(i + 1).expect("--variable-f requires a value");
                paths.variable_f = PathBuf::from(value);
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    paths
}

fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<T>(&line)
                .unwrap_or_else(|e| panic!("parse {}: {e}\nline={line}", path.display()))
        })
        .collect()
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(writer).expect("write newline");
    }
    writer.flush().expect("flush output");
}

fn poly_id_from_strings(data: &[[String; 4]]) -> String {
    let canonical = serde_json::to_string(data).expect("serialize canonical duals");
    let mut hasher = Hasher::new();
    hasher.update(canonical.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn poly_id_from_record(record: &PolytopeRecord) -> String {
    poly_id_from_strings(&rational_vec4_to_strings(&record.dual_vertices_rational))
}

fn rational_dual_vertices_to_f64(data: &[[BigRational; 4]]) -> Vec<[f64; 4]> {
    data.iter()
        .map(|row| {
            std::array::from_fn(|i| {
                row[i]
                    .to_f64()
                    .unwrap_or_else(|| panic!("cannot convert rational dual vertex to f64"))
            })
        })
        .collect()
}

fn dual_vertices_to_state_key(prefix: &str, name: &str) -> String {
    format!("{prefix}::{name}")
}

fn max_abs_dual_diff(lhs: &[[f64; 4]], rhs: &[[f64; 4]]) -> f64 {
    lhs.iter()
        .zip(rhs)
        .flat_map(|(a, b)| (0..4).map(move |i| (a[i] - b[i]).abs()))
        .fold(0.0, f64::max)
}

fn approx_match_exact<'a>(
    candidates: &'a [ExactCacheEntry],
    query: &[[f64; 4]],
    label: &str,
) -> &'a ExactCacheEntry {
    let mut best: Option<(&ExactCacheEntry, f64)> = None;
    let mut second: Option<(&ExactCacheEntry, f64)> = None;
    for candidate in candidates {
        if candidate.dual_vertices_f64.len() != query.len() {
            continue;
        }
        let diff = max_abs_dual_diff(&candidate.dual_vertices_f64, query);
        if best.as_ref().is_none_or(|(_, best_diff)| diff < *best_diff) {
            second = best;
            best = Some((candidate, diff));
        } else if second
            .as_ref()
            .is_none_or(|(_, second_diff)| diff < *second_diff)
        {
            second = Some((candidate, diff));
        }
    }
    let (best_candidate, best_diff) =
        best.unwrap_or_else(|| panic!("no cache candidates for {label}"));
    assert!(
        best_diff <= MATCH_TOLERANCE,
        "no exact-cache match for {label}: best max-abs diff = {best_diff:.3e}"
    );
    if let Some((second_candidate, second_diff)) = second {
        assert!(
            second_diff > MATCH_TOLERANCE || second_candidate.poly_id == best_candidate.poly_id,
            "ambiguous exact-cache match for {label}: {} and {} within tolerance",
            best_candidate.poly_id,
            second_candidate.poly_id
        );
    }
    best_candidate
}

fn load_exact_cache(path: &Path, label: &str, entries: &mut HashMap<String, ExactCacheEntry>) {
    if !path.exists() {
        return;
    }
    let db = load(path).unwrap_or_else(|e| panic!("load {}: {e}", path.display()));
    for record in db.into_values() {
        let poly_id = poly_id_from_record(&record);
        if entries.contains_key(&poly_id) {
            continue;
        }
        let dual_vertices_f64 = rational_dual_vertices_to_f64(&record.dual_vertices_rational);
        entries.insert(
            poly_id.clone(),
            ExactCacheEntry {
                poly_id,
                facet_count: record.dual_vertices_rational.len(),
                record,
                dual_vertices_f64,
                geometry_source: label.to_string(),
            },
        );
    }
}

fn build_exact_candidate_index(
    package_root: &Path,
) -> (
    HashMap<String, ExactCacheEntry>,
    HashMap<usize, Vec<ExactCacheEntry>>,
) {
    let repo_root = package_root
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("package root should be experiments/sys-landscape");
    let mut by_poly_id = HashMap::new();
    load_exact_cache(
        &package_root.join("cache.jsonl"),
        "experiments/sys-landscape/cache.jsonl",
        &mut by_poly_id,
    );
    load_exact_cache(
        &repo_root.join("experiments/combinatorial-cells/polytopes.jsonl"),
        "experiments/combinatorial-cells/polytopes.jsonl",
        &mut by_poly_id,
    );
    load_exact_cache(
        &package_root.join("variable-f-ascent/cache.jsonl"),
        "experiments/sys-landscape/variable-f-ascent/cache.jsonl",
        &mut by_poly_id,
    );
    let mut by_facet_count: HashMap<usize, Vec<ExactCacheEntry>> = HashMap::new();
    for entry in by_poly_id.values() {
        by_facet_count
            .entry(entry.facet_count)
            .or_default()
            .push(entry.clone());
    }
    (by_poly_id, by_facet_count)
}

fn infer_variable_f_source_name(row: &VariableFRow) -> String {
    if !row.source_name.is_empty() {
        return row.source_name.clone();
    }
    if row.rq == "rq1" {
        let (prefix, _) = row
            .name
            .rsplit_once("_p")
            .unwrap_or_else(|| panic!("bad rq1 name: {}", row.name));
        return prefix.trim_start_matches("rq1_").to_string();
    }
    row.name
        .split("_path")
        .next()
        .unwrap_or(&row.name)
        .to_string()
}

fn infer_variable_f_lineage_id(row: &VariableFRow, source_name: &str) -> String {
    if !row.lineage_id.is_empty() {
        return row.lineage_id.clone();
    }
    if row.rq == "rq1" {
        return source_name.to_string();
    }
    if row.rq == "rq2" {
        return format!("rq2_seed_{source_name}");
    }
    source_name.to_string()
}

fn infer_root_group_id(
    dataset: &str,
    family: &str,
    source_name: &str,
    role: &str,
) -> String {
    if role == "random_sample" {
        return format!("{dataset}::{source_name}");
    }
    if family == "general" && source_name.starts_with("general_") {
        return format!("general::{source_name}");
    }
    if family == "lagrangian_product" && source_name.starts_with("products_") {
        return format!("lagrangian_product::{source_name}");
    }
    format!("{dataset}::{source_name}")
}

fn infer_variable_f_parent_state_id(row: &VariableFRow) -> Option<String> {
    row.direct_parent_trial
        .as_ref()
        .map(|parent| dual_vertices_to_state_key("variable_f", parent))
}

fn resolve_entry_from_exact_or_f64<'a>(
    exact_by_poly_id: &'a HashMap<String, ExactCacheEntry>,
    by_facet_count: &'a HashMap<usize, Vec<ExactCacheEntry>>,
    exact_duals: &[[String; 4]],
    fallback_duals: &[[f64; 4]],
    label: &str,
) -> &'a ExactCacheEntry {
    if !exact_duals.is_empty() {
        let poly_id = poly_id_from_strings(exact_duals);
        if let Some(entry) = exact_by_poly_id.get(&poly_id) {
            return entry;
        }
    }
    let candidates = by_facet_count
        .get(&fallback_duals.len())
        .unwrap_or_else(|| {
            panic!(
                "no exact cache bucket for {label} with F={}",
                fallback_duals.len()
            )
        });
    approx_match_exact(candidates, fallback_duals, label)
}

fn capacity_from_record(
    entry: &ExactCacheEntry,
    iterations: Option<u64>,
    search_result_source: &str,
) -> CapacityResultRow {
    if let (Some(capacity), Some(volume)) = (entry.record.capacity, entry.record.volume) {
        return CapacityResultRow {
            poly_id: entry.poly_id.clone(),
            capacity,
            volume,
            sys: capacity * capacity / (2.0 * volume),
            iterations,
            search_result_source: search_result_source.to_string(),
        };
    }
    let polytope = entry
        .record
        .to_polytope()
        .expect("reconstruct exact cache row for computed capacity");
    let volume = volume(&polytope).expect("volume from exact cache row");
    let result = ehz_capacity(&polytope).expect("capacity from exact cache row");
    let capacity = result.capacity();
    CapacityResultRow {
        poly_id: entry.poly_id.clone(),
        capacity,
        volume,
        sys: capacity * capacity / (2.0 * volume),
        iterations,
        search_result_source: format!("{search_result_source}+computed"),
    }
}

fn push_polytope_row(output: &mut HashMap<String, PolytopeRow>, entry: &ExactCacheEntry) {
    output
        .entry(entry.poly_id.clone())
        .or_insert_with(|| PolytopeRow {
            poly_id: entry.poly_id.clone(),
            dual_vertices_rational: rational_vec4_to_strings(&entry.record.dual_vertices_rational),
            vertices_rational: rational_vec4_to_strings(&entry.record.vertices_rational),
            facet_count: entry.record.dual_vertices_rational.len(),
            geometry_source: entry.geometry_source.clone(),
        });
}

fn push_capacity_row(
    output: &mut HashMap<String, CapacityResultRow>,
    entry: &ExactCacheEntry,
    iterations: Option<u64>,
    search_result_source: &str,
) {
    let new_row = capacity_from_record(entry, iterations, search_result_source);
    output
        .entry(entry.poly_id.clone())
        .and_modify(|existing| {
            if existing.iterations.is_none() {
                existing.iterations = new_row.iterations;
            }
        })
        .or_insert(new_row);
}

fn main() {
    let package_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let paths = parse_args(package_root);
    std::fs::create_dir_all(&paths.out_dir).expect("create output directory");

    println!("normalized-dataset: Stage 1 converter");
    println!("  out-dir: {}", paths.out_dir.display());
    println!("  general-summary: {}", paths.general_summary.display());
    println!("  products-summary: {}", paths.products_summary.display());
    println!("  variable-f: {}", paths.variable_f.display());

    let (exact_by_poly_id, exact_by_facet_count) = build_exact_candidate_index(package_root);
    println!("Loaded exact cache entries: {}", exact_by_poly_id.len());

    let random_rows: Vec<RandomSweepRow> = read_jsonl(&paths.random_sample);
    let random_product_rows: Vec<RandomProductRow> = read_jsonl(&paths.random_product);
    let general_rows: Vec<SummaryRow> = read_jsonl(&paths.general_summary);
    let general_trace_rows: Vec<TraceRow> = read_jsonl(&paths.general_trace);
    let products_rows: Vec<SummaryRow> = read_jsonl(&paths.products_summary);
    let products_trace_rows: Vec<TraceRow> = read_jsonl(&paths.products_trace);
    let variable_f_rows: Vec<VariableFRow> = read_jsonl(&paths.variable_f);

    let mut polytopes = HashMap::<String, PolytopeRow>::new();
    let mut capacities = HashMap::<String, CapacityResultRow>::new();
    let mut states = Vec::<StateRow>::new();
    let mut step_events = Vec::<StepEventRow>::new();

    for row in &random_rows {
        let label = format!("random sample {}", row.name);
        let entry = resolve_entry_from_exact_or_f64(
            &exact_by_poly_id,
            &exact_by_facet_count,
            &row.dual_vertices_rational,
            &row.dual_vertices,
            &label,
        );
        push_polytope_row(&mut polytopes, entry);
        push_capacity_row(
            &mut capacities,
            entry,
            Some(row.iterations),
            "random-sample/random-sweep.jsonl",
        );
        states.push(StateRow {
            state_id: dual_vertices_to_state_key("random_sample", &row.name),
            poly_id: entry.poly_id.clone(),
            dataset: "random_sample".into(),
            family: "general".into(),
            role: "random_sample".into(),
            search_space: "general".into(),
            optimizer: "none".into(),
            backend: "ehz_capacity".into(),
            source_name: row.name.clone(),
            root_group_id: infer_root_group_id(
                "random_sample",
                "general",
                &row.name,
                "random_sample",
            ),
            seed_index: None,
            lineage_id: None,
            parent_state_id: None,
            rq: None,
            path: None,
            starting_f: Some(row.facet_count),
            starting_sys: Some(row.sys),
            reported_final_sys: Some(row.sys),
            reported_delta: Some(0.0),
            sys_after_addition: None,
            n_iterations: Some(row.iterations as usize),
            n_phases: None,
            best_strategy: None,
            n_escape_overshoot: None,
            n_escape_wiggle: None,
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: None,
        });
        let _ = (row.capacity, row.volume);
    }

    for row in &random_product_rows {
        let label = format!("random product {}", row.name);
        let entry = resolve_entry_from_exact_or_f64(
            &exact_by_poly_id,
            &exact_by_facet_count,
            &row.dual_vertices_rational,
            &row.dual_vertices,
            &label,
        );
        push_polytope_row(&mut polytopes, entry);
        push_capacity_row(
            &mut capacities,
            entry,
            Some(row.iterations),
            "random-product-sample/random-product-sweep.jsonl",
        );
        states.push(StateRow {
            state_id: dual_vertices_to_state_key("random_product", &row.name),
            poly_id: entry.poly_id.clone(),
            dataset: "random_product_sample".into(),
            family: "lagrangian_product".into(),
            role: "random_sample".into(),
            search_space: "lagrangian_product".into(),
            optimizer: "none".into(),
            backend: "ehz_capacity_billiard".into(),
            source_name: row.name.clone(),
            root_group_id: infer_root_group_id(
                "random_product_sample",
                "lagrangian_product",
                &row.name,
                "random_sample",
            ),
            seed_index: None,
            lineage_id: None,
            parent_state_id: None,
            rq: None,
            path: None,
            starting_f: Some(row.facet_count),
            starting_sys: Some(row.sys),
            reported_final_sys: Some(row.sys),
            reported_delta: Some(0.0),
            sys_after_addition: None,
            n_iterations: Some(row.iterations as usize),
            n_phases: None,
            best_strategy: None,
            n_escape_overshoot: None,
            n_escape_wiggle: None,
            placement_direction: None,
            facet_remained_active: None,
            total_time_ms: None,
        });
        let _ = (row.k, row.m, row.capacity, row.volume);
    }

    for row in &general_rows {
        let label = format!("ga general {}", row.name);
        let entry = resolve_entry_from_exact_or_f64(
            &exact_by_poly_id,
            &exact_by_facet_count,
            &row.final_dual_vertices_rational,
            &row.final_dual_vertices,
            &label,
        );
        push_polytope_row(&mut polytopes, entry);
        push_capacity_row(
            &mut capacities,
            entry,
            None,
            "gradient-ascent-general/gradient-ascent-general.jsonl",
        );
        let lineage = if row.lineage_id.is_empty() {
            row.name.clone()
        } else {
            row.lineage_id.clone()
        };
        let source_name = if row.source_name.is_empty() {
            row.name.clone()
        } else {
            row.source_name.clone()
        };
        states.push(StateRow {
            state_id: dual_vertices_to_state_key("ga_general", &row.name),
            poly_id: entry.poly_id.clone(),
            dataset: "gradient_ascent_general".into(),
            family: "general".into(),
            role: "ascent_endpoint".into(),
            search_space: "general".into(),
            optimizer: "gradient_ascent".into(),
            backend: "ehz_capacity".into(),
            root_group_id: infer_root_group_id(
                "gradient_ascent_general",
                "general",
                &source_name,
                "ascent_endpoint",
            ),
            source_name,
            seed_index: Some(row.seed_index),
            lineage_id: Some(lineage),
            parent_state_id: None,
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
        });
    }

    for row in &general_trace_rows {
        step_events.push(StepEventRow {
            state_id: dual_vertices_to_state_key("ga_general", &row.name),
            phase: row.phase,
            iteration: row.iteration,
            step_type: row.step_type.clone(),
            t_fraction: row.t_fraction,
            t_actual: row.t_actual,
            sys_before: row.sys_before,
            sys_after: row.sys_after,
            delta_sys: row.delta_sys,
            gradient_norm: row.gradient_norm,
        });
    }

    for row in &products_rows {
        let label = format!("ga products {}", row.name);
        let entry = resolve_entry_from_exact_or_f64(
            &exact_by_poly_id,
            &exact_by_facet_count,
            &row.final_dual_vertices_rational,
            &row.final_dual_vertices,
            &label,
        );
        push_polytope_row(&mut polytopes, entry);
        push_capacity_row(
            &mut capacities,
            entry,
            None,
            "gradient-ascent-products/gradient-ascent-products.jsonl",
        );
        let lineage = if row.lineage_id.is_empty() {
            row.name.clone()
        } else {
            row.lineage_id.clone()
        };
        let source_name = if row.source_name.is_empty() {
            row.name.clone()
        } else {
            row.source_name.clone()
        };
        states.push(StateRow {
            state_id: dual_vertices_to_state_key("ga_products", &row.name),
            poly_id: entry.poly_id.clone(),
            dataset: "gradient_ascent_products".into(),
            family: "lagrangian_product".into(),
            role: "ascent_endpoint".into(),
            search_space: "lagrangian_product".into(),
            optimizer: "projected_gradient_ascent".into(),
            backend: "ehz_capacity".into(),
            root_group_id: infer_root_group_id(
                "gradient_ascent_products",
                "lagrangian_product",
                &source_name,
                "ascent_endpoint",
            ),
            source_name,
            seed_index: Some(row.seed_index),
            lineage_id: Some(lineage),
            parent_state_id: None,
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
        });
    }

    for row in &products_trace_rows {
        step_events.push(StepEventRow {
            state_id: dual_vertices_to_state_key("ga_products", &row.name),
            phase: row.phase,
            iteration: row.iteration,
            step_type: row.step_type.clone(),
            t_fraction: row.t_fraction,
            t_actual: row.t_actual,
            sys_before: row.sys_before,
            sys_after: row.sys_after,
            delta_sys: row.delta_sys,
            gradient_norm: row.gradient_norm,
        });
    }

    for row in &variable_f_rows {
        let label = format!("variable-f {}", row.name);
        let entry = resolve_entry_from_exact_or_f64(
            &exact_by_poly_id,
            &exact_by_facet_count,
            &row.final_dual_vertices_rational,
            &row.final_dual_vertices,
            &label,
        );
        push_polytope_row(&mut polytopes, entry);
        push_capacity_row(
            &mut capacities,
            entry,
            None,
            "variable-f-ascent/variable-f-ascent.jsonl",
        );
        let source_name = infer_variable_f_source_name(row);
        let lineage_id = infer_variable_f_lineage_id(row, &source_name);
        states.push(StateRow {
            state_id: dual_vertices_to_state_key("variable_f", &row.name),
            poly_id: entry.poly_id.clone(),
            dataset: "variable_f_ascent".into(),
            family: "general".into(),
            role: "continuation_endpoint".into(),
            search_space: row.path.clone(),
            optimizer: "gradient_ascent".into(),
            backend: "ehz_capacity".into(),
            root_group_id: infer_root_group_id(
                "variable_f_ascent",
                "general",
                &source_name,
                "continuation_endpoint",
            ),
            source_name,
            seed_index: None,
            lineage_id: Some(lineage_id),
            parent_state_id: infer_variable_f_parent_state_id(row),
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
        });
    }

    let mut polytope_rows = polytopes.into_values().collect::<Vec<_>>();
    polytope_rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));

    let mut capacity_rows = capacities.into_values().collect::<Vec<_>>();
    capacity_rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));

    states.sort_by(|a, b| a.state_id.cmp(&b.state_id));
    step_events.sort_by(|a, b| {
        a.state_id
            .cmp(&b.state_id)
            .then(a.phase.cmp(&b.phase))
            .then(a.iteration.cmp(&b.iteration))
    });

    write_jsonl(&paths.out_dir.join("polytopes.jsonl"), &polytope_rows);
    write_jsonl(&paths.out_dir.join("states.jsonl"), &states);
    write_jsonl(
        &paths.out_dir.join("capacity_results.jsonl"),
        &capacity_rows,
    );
    write_jsonl(&paths.out_dir.join("step_events.jsonl"), &step_events);

    println!("Wrote {} polytopes", polytope_rows.len());
    println!("Wrote {} states", states.len());
    println!("Wrote {} capacity rows", capacity_rows.len());
    println!("Wrote {} step events", step_events.len());
    println!("Output directory: {}", paths.out_dir.display());
}
