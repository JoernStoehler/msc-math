//! Profile prepare-stage invariant feature cost by polytope and bucket.
//!
//! This command is sequential on purpose. It measures small batches with simple
//! `Instant` boundaries around the same helper calls used by the prepared
//! invariant table builder.

mod invariant_features;
mod load_caches;
#[path = "../polytope-datasets/rows.rs"]
mod producer_rows;
mod rows;

use exp_sys_landscape::{
    dual_vertices_rational_strings, exact_volume_from_incidence_as_f64, package_root,
    SysLandscapePolytopeCache,
};
use invariant_features::ProfiledInvariantFeatureRow;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::geom::polygon::random_polygon_2d;

const SYNTHETIC_SMOKE_SEED: u64 = 20260701;
const SYNTHETIC_H_MIN: f64 = 0.8;
const SYNTHETIC_H_MAX: f64 = 1.2;

struct Args {
    input_mode: InputMode,
    out_dir: PathBuf,
    max_polytopes: Option<usize>,
}

enum InputMode {
    SyntheticSmoke,
    ProducerCaches(load_caches::DatasetPaths),
}

#[derive(Clone, Serialize)]
struct FeatureCostRow {
    schema: &'static str,
    poly_id: String,
    capacity_source: String,
    bucket: String,
    facet_count: usize,
    product_k: Option<usize>,
    product_m: Option<usize>,
    source_name: Option<String>,
    sys: f64,
    cached_volume: f64,
    recomputed_volume: f64,
    volume_relative_residual: f64,
    vertex_count: usize,
    edge_count: usize,
    ridge_count: usize,
    ridge_symp_area_ordered_face_count: usize,
    ridge_symp_area_ordering_failure_count: usize,
    ridge_symp_area_sum_over_volume_sqrt: f64,
    ridge_symp_area_mean_over_volume_sqrt: f64,
    ridge_symp_area_q95_over_volume_sqrt: f64,
    decode_dual_vertices_ms: f64,
    reconstruct_polytope_ms: f64,
    volume_recompute_ms: f64,
    cached_volume_sqrt_ms: f64,
    face_lattice_ms: f64,
    skeleton_summary_ms: f64,
    ridge_symplectic_area_summary_ms: f64,
    row_assembly_ms: f64,
    standard_prepare_feature_time_ms: f64,
    feature_first_total_with_volume_recompute_ms: f64,
    total_measured_feature_time_ms: f64,
}

#[derive(Serialize)]
struct GroupSummaryRow {
    schema: &'static str,
    bucket: String,
    group: String,
    polytope_count: usize,
    total_ms: f64,
    mean_ms: f64,
    median_ms: f64,
    p90_ms: f64,
    max_ms: f64,
}

#[derive(Serialize)]
struct RunSummary {
    schema: &'static str,
    input_mode: String,
    out_dir: String,
    max_polytopes: Option<usize>,
    polytope_rows: usize,
    provenance_rows: usize,
    profiled_polytope_rows: usize,
    per_polytope_jsonl: String,
    group_summary_tsv: String,
    boundary_note: &'static str,
}

fn parse_args() -> Args {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help(
            argv.first()
                .map(String::as_str)
                .unwrap_or("sys-datascience-feature-cost"),
        );
        std::process::exit(0);
    }

    let mut synthetic_smoke = false;
    let mut out_dir = None;
    let mut max_polytopes = Some(10usize);
    let mut max_random_rows = None;
    let mut max_random_product_rows = None;
    let produce_dir = package_root().join("../polytope-datasets");
    let mut random_sample = produce_dir.join("random.jsonl");
    let mut random_product = produce_dir.join("random-product.jsonl");

    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = || {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match flag {
            "--synthetic-smoke" => {
                synthetic_smoke = true;
                i += 1;
            }
            "--out-dir" => {
                out_dir = Some(PathBuf::from(value()));
                i += 2;
            }
            "--max-polytopes" => {
                let value = value();
                max_polytopes = if value == "all" {
                    None
                } else {
                    Some(
                        value
                            .parse()
                            .expect("--max-polytopes must be a usize or all"),
                    )
                };
                i += 2;
            }
            "--random-only-size" => {
                let (random_limit, product_limit) = random_only_size_limits(value());
                max_random_rows = random_limit;
                max_random_product_rows = product_limit;
                i += 2;
            }
            "--produce-dir" | "--retained-produce-dir" => {
                let dir = PathBuf::from(value());
                random_sample = dir.join("random.jsonl");
                random_product = dir.join("random-product.jsonl");
                i += 2;
            }
            "--random" => {
                random_sample = PathBuf::from(value());
                i += 2;
            }
            "--random-product" => {
                random_product = PathBuf::from(value());
                i += 2;
            }
            "--max-random-rows" => {
                max_random_rows = Some(value().parse().expect("--max-random-rows must be a usize"));
                i += 2;
            }
            "--max-random-product-rows" => {
                max_random_product_rows = Some(
                    value()
                        .parse()
                        .expect("--max-random-product-rows must be a usize"),
                );
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    let out_dir = out_dir.unwrap_or_else(|| {
        std::env::temp_dir().join(format!("sys-ds-feature-cost-{}", std::process::id()))
    });
    let input_mode = if synthetic_smoke {
        InputMode::SyntheticSmoke
    } else {
        InputMode::ProducerCaches(load_caches::DatasetPaths {
            max_random_rows,
            max_random_product_rows,
            random_sample,
            random_product,
            out_dir: out_dir.clone(),
        })
    };

    Args {
        input_mode,
        out_dir,
        max_polytopes,
    }
}

fn print_help(program: &str) {
    println!(
        "\
Profile sys-datascience prepare invariant feature costs.

Usage:
  {program} --synthetic-smoke --out-dir <dir>
  {program} --random-only-size smoke --out-dir <dir> [--max-polytopes 10]

Inputs:
  --synthetic-smoke                Generate a tiny deterministic smoke batch in memory
  --retained-produce-dir <dir>     Read retained random.jsonl and random-product.jsonl from <dir>
  --produce-dir <dir>              Alias for --retained-produce-dir
  --random <path>                  Override canonical random.jsonl
  --random-product <path>          Override canonical random-product.jsonl
  --random-only-size <name>        Named retained producer limits: smoke, method, full
  --max-random-rows <n>            Override retained random row count
  --max-random-product-rows <n>    Override retained product row count
  --max-polytopes <n|all>          Profile first n loaded unique polytopes; default 10
  --out-dir <dir>                  Output directory for cost JSONL/TSV
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

fn synthetic_smoke_caches() -> load_caches::LoadedCaches {
    let mut polytopes = Vec::new();
    let mut provenance_rows = Vec::new();
    let mut random_count = 0usize;
    for facet_count in [5usize, 6, 7] {
        let polytope = (0..)
            .find_map(|attempt| {
                SysLandscapePolytopeCache::generate_random(
                    facet_count,
                    SYNTHETIC_H_MIN,
                    SYNTHETIC_H_MAX,
                    SYNTHETIC_SMOKE_SEED,
                    attempt,
                )
            })
            .expect("synthetic random polytope");
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let poly_id = format!("synthetic_random_f{facet_count}_{random_count}");
        polytopes.push(load_caches::LoadedPolytopeRow {
            poly_id: poly_id.clone(),
            dual_vertices_rational: dual_vertices_rational_strings(&polytope),
            facet_count,
            capacity: 0.0,
            volume,
            sys: 0.0,
            capacity_source: "synthetic_random_sample".to_string(),
        });
        provenance_rows.push(synthetic_provenance(
            &poly_id,
            "synthetic_random_sample",
            "general",
            format!("synthetic_random_f{facet_count}_{random_count}"),
            Some(facet_count),
            None,
        ));
        random_count += 1;
    }

    for (index, (k, m)) in [(3usize, 3usize), (3, 4), (4, 4), (5, 6)]
        .into_iter()
        .enumerate()
    {
        let polytope = synthetic_product_polytope(k, m, index);
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let poly_id = format!("synthetic_product_{k}x{m}_{index}");
        polytopes.push(load_caches::LoadedPolytopeRow {
            poly_id: poly_id.clone(),
            dual_vertices_rational: dual_vertices_rational_strings(&polytope),
            facet_count: polytope.facet_count(),
            capacity: 0.0,
            volume,
            sys: 0.0,
            capacity_source: "synthetic_random_product_sample".to_string(),
        });
        provenance_rows.push(synthetic_provenance(
            &poly_id,
            "synthetic_random_product_sample",
            "lagrangian_product",
            format!("synthetic_product_{k}x{m}_{index}"),
            None,
            Some((k, m)),
        ));
    }

    load_caches::LoadedCaches {
        polytopes,
        provenance_rows,
    }
}

fn synthetic_product_polytope(
    k: usize,
    m: usize,
    sample_index: usize,
) -> SysLandscapePolytopeCache {
    for attempt in 0.. {
        let mut material = Vec::new();
        material.extend_from_slice(&SYNTHETIC_SMOKE_SEED.to_le_bytes());
        material.extend_from_slice(&(k as u64).to_le_bytes());
        material.extend_from_slice(&(m as u64).to_le_bytes());
        material.extend_from_slice(&(sample_index as u64).to_le_bytes());
        material.extend_from_slice(&(attempt as u64).to_le_bytes());
        let seed = blake3::derive_key("sys-ds-feature-cost-synthetic-product", &material);
        let mut rng = ChaCha8Rng::from_seed(seed);
        let (qn, qh) = random_polygon_2d(k, SYNTHETIC_H_MIN, SYNTHETIC_H_MAX, &mut rng);
        let (pn, ph) = random_polygon_2d(m, SYNTHETIC_H_MIN, SYNTHETIC_H_MAX, &mut rng);
        if let Some(polytope) =
            SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
        {
            return polytope;
        }
    }
    unreachable!("unbounded synthetic product generation loop")
}

fn synthetic_provenance(
    poly_id: &str,
    dataset: &str,
    family: &str,
    source_name: String,
    facet_count: Option<usize>,
    product: Option<(usize, usize)>,
) -> load_caches::LoadedProvenanceRow {
    let source = match product {
        Some((k, m)) => Some(serde_json::json!({
            "producer": "random-product",
            "k": k,
            "m": m,
            "h_min": SYNTHETIC_H_MIN,
            "h_max": SYNTHETIC_H_MAX,
            "seed": SYNTHETIC_SMOKE_SEED,
            "sample_index": 0,
            "attempt": 0,
            "bounces": 0
        })),
        None => Some(serde_json::json!({
            "producer": "random",
            "facet_count": facet_count,
            "h_min": SYNTHETIC_H_MIN,
            "h_max": SYNTHETIC_H_MAX,
            "seed": SYNTHETIC_SMOKE_SEED,
            "sample_index": 0,
            "attempt": 0
        })),
    };
    load_caches::LoadedProvenanceRow {
        provenance_id: format!("{dataset}:{source_name}"),
        poly_id: poly_id.to_string(),
        dataset: dataset.to_string(),
        family: family.to_string(),
        role: dataset.to_string(),
        search_space: family.to_string(),
        optimizer: "none".to_string(),
        backend: "synthetic_feature_cost".to_string(),
        source_name,
        root_group_id: poly_id.to_string(),
        source,
        sample_seed: Some(SYNTHETIC_SMOKE_SEED),
        sample_attempt: Some(0),
        sample_h_min: Some(SYNTHETIC_H_MIN),
        sample_h_max: Some(SYNTHETIC_H_MAX),
        product_k: product.map(|(k, _)| k),
        product_m: product.map(|(_, m)| m),
        product_bounces: product.map(|_| 0),
        seed_index: Some(0),
        lineage_id: Some(poly_id.to_string()),
        path: product.map(|(k, m)| format!("lp_{k}x{m}")),
        total_time_ms: None,
    }
}

fn load_input(args: &Args) -> (String, load_caches::LoadedCaches) {
    match &args.input_mode {
        InputMode::SyntheticSmoke => ("synthetic_smoke".to_string(), synthetic_smoke_caches()),
        InputMode::ProducerCaches(paths) => (
            "retained_producer_caches".to_string(),
            load_caches::load_caches(paths),
        ),
    }
}

fn provenance_by_poly_id(
    rows: &[load_caches::LoadedProvenanceRow],
) -> BTreeMap<String, load_caches::LoadedProvenanceRow> {
    let mut out = BTreeMap::new();
    for row in rows {
        out.entry(row.poly_id.clone())
            .or_insert_with(|| row.clone());
    }
    out
}

fn bucket_for(
    feature_row: &ProfiledInvariantFeatureRow,
    provenance: Option<&load_caches::LoadedProvenanceRow>,
) -> String {
    if let Some(row) = provenance {
        if let (Some(k), Some(m)) = (row.product_k, row.product_m) {
            return format!("product_{k}x{m}");
        }
    }
    format!("facet_{}", feature_row.row.facet_count)
}

fn relative_residual(left: f64, right: f64) -> f64 {
    (left - right).abs() / left.abs().max(right.abs()).max(1.0)
}

fn feature_cost_row(
    profiled: ProfiledInvariantFeatureRow,
    provenance: Option<&load_caches::LoadedProvenanceRow>,
) -> FeatureCostRow {
    let bucket = bucket_for(&profiled, provenance);
    let row = profiled.row;
    let timings = profiled.timings;
    let standard_prepare_feature_time_ms = timings.standard_prepare_feature_time_ms();
    let feature_first_total_with_volume_recompute_ms =
        timings.feature_first_total_with_volume_recompute_ms();
    FeatureCostRow {
        schema: "sys_datascience_feature_cost_per_polytope_v1",
        poly_id: row.poly_id,
        capacity_source: row.capacity_source,
        bucket,
        facet_count: row.facet_count,
        product_k: provenance.and_then(|row| row.product_k),
        product_m: provenance.and_then(|row| row.product_m),
        source_name: provenance.map(|row| row.source_name.clone()),
        sys: row.sys,
        cached_volume: profiled.cached_volume,
        recomputed_volume: profiled.recomputed_volume,
        volume_relative_residual: relative_residual(
            profiled.cached_volume,
            profiled.recomputed_volume,
        ),
        vertex_count: row.vertex_count,
        edge_count: row.edge_count,
        ridge_count: row.ridge_count,
        ridge_symp_area_ordered_face_count: row.ridge_symp_area_ordered_face_count,
        ridge_symp_area_ordering_failure_count: row.ridge_symp_area_ordering_failure_count,
        ridge_symp_area_sum_over_volume_sqrt: row.ridge_symp_area_sum_over_volume_sqrt,
        ridge_symp_area_mean_over_volume_sqrt: row.ridge_symp_area_mean_over_volume_sqrt,
        ridge_symp_area_q95_over_volume_sqrt: row.ridge_symp_area_q95_over_volume_sqrt,
        decode_dual_vertices_ms: timings.decode_dual_vertices_ms,
        reconstruct_polytope_ms: timings.reconstruct_polytope_ms,
        volume_recompute_ms: timings.volume_recompute_ms,
        cached_volume_sqrt_ms: timings.cached_volume_sqrt_ms,
        face_lattice_ms: timings.face_lattice_ms,
        skeleton_summary_ms: timings.skeleton_summary_ms,
        ridge_symplectic_area_summary_ms: timings.ridge_symplectic_area_summary_ms,
        row_assembly_ms: timings.row_assembly_ms,
        standard_prepare_feature_time_ms,
        feature_first_total_with_volume_recompute_ms,
        total_measured_feature_time_ms: feature_first_total_with_volume_recompute_ms,
    }
}

fn write_jsonl(path: &Path, rows: &[FeatureCostRow]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)
            .unwrap_or_else(|e| panic!("serialize {}: {e}", path.display()));
        writeln!(&mut writer).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

fn write_group_summary_tsv(path: &Path, rows: &[GroupSummaryRow]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    writeln!(
        &mut writer,
        "schema\tbucket\tgroup\tpolytope_count\ttotal_ms\tmean_ms\tmedian_ms\tp90_ms\tmax_ms"
    )
    .expect("write group summary header");
    for row in rows {
        writeln!(
            &mut writer,
            "{}\t{}\t{}\t{}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}",
            row.schema,
            row.bucket,
            row.group,
            row.polytope_count,
            row.total_ms,
            row.mean_ms,
            row.median_ms,
            row.p90_ms,
            row.max_ms
        )
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
}

fn group_values(row: &FeatureCostRow) -> [(&'static str, f64); 10] {
    [
        ("decode_dual_vertices", row.decode_dual_vertices_ms),
        ("reconstruct_polytope", row.reconstruct_polytope_ms),
        (
            "volume_recompute_from_dual_vertices",
            row.volume_recompute_ms,
        ),
        ("cached_volume_sqrt", row.cached_volume_sqrt_ms),
        ("face_lattice_enumeration", row.face_lattice_ms),
        ("skeleton_summary", row.skeleton_summary_ms),
        (
            "ridge_symplectic_area_summary",
            row.ridge_symplectic_area_summary_ms,
        ),
        ("row_assembly", row.row_assembly_ms),
        (
            "standard_prepare_feature_total",
            row.standard_prepare_feature_time_ms,
        ),
        (
            "feature_first_total_with_volume_recompute",
            row.feature_first_total_with_volume_recompute_ms,
        ),
    ]
}

fn group_summaries(rows: &[FeatureCostRow]) -> Vec<GroupSummaryRow> {
    let mut values = BTreeMap::<(String, String), Vec<f64>>::new();
    for row in rows {
        for (group, value) in group_values(row) {
            values
                .entry(("all".to_string(), group.to_string()))
                .or_default()
                .push(value);
            values
                .entry((row.bucket.clone(), group.to_string()))
                .or_default()
                .push(value);
        }
    }
    values
        .into_iter()
        .map(|((bucket, group), values)| summarize_group(bucket, group, values))
        .collect()
}

fn summarize_group(bucket: String, group: String, mut values: Vec<f64>) -> GroupSummaryRow {
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    let polytope_count = values.len();
    let total_ms = values.iter().sum::<f64>();
    let mean_ms = total_ms / polytope_count as f64;
    GroupSummaryRow {
        schema: "sys_datascience_feature_cost_group_summary_v1",
        bucket,
        group,
        polytope_count,
        total_ms,
        mean_ms,
        median_ms: quantile_sorted(&values, 0.5),
        p90_ms: quantile_sorted(&values, 0.9),
        max_ms: values.last().copied().unwrap_or(0.0),
    }
}

fn quantile_sorted(values: &[f64], q: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    if values.len() == 1 {
        return values[0];
    }
    let position = q * (values.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    if lower == upper {
        values[lower]
    } else {
        let weight = position - lower as f64;
        values[lower] * (1.0 - weight) + values[upper] * weight
    }
}

fn print_compact_summary(rows: &[GroupSummaryRow]) {
    println!("bucket\tgroup\tcount\tmean_ms\tp90_ms\ttotal_ms");
    let mut selected = rows
        .iter()
        .filter(|row| row.bucket == "all")
        .filter(|row| {
            row.group != "standard_prepare_feature_total"
                && row.group != "feature_first_total_with_volume_recompute"
        })
        .collect::<Vec<_>>();
    selected.sort_by(|left, right| {
        right
            .mean_ms
            .partial_cmp(&left.mean_ms)
            .unwrap_or(Ordering::Equal)
    });
    for row in selected {
        println!(
            "{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}",
            row.bucket, row.group, row.polytope_count, row.mean_ms, row.p90_ms, row.total_ms
        );
    }
}

fn main() {
    let args = parse_args();
    std::fs::create_dir_all(&args.out_dir)
        .unwrap_or_else(|e| panic!("create {}: {e}", args.out_dir.display()));

    let (input_mode, mut caches) = load_input(&args);
    if let Some(max_polytopes) = args.max_polytopes {
        caches.polytopes.truncate(max_polytopes);
    }
    let provenance = provenance_by_poly_id(&caches.provenance_rows);
    let rows = caches
        .polytopes
        .iter()
        .map(|row| {
            let profiled = invariant_features::profile_invariant_row_from_loaded_row(row);
            feature_cost_row(profiled, provenance.get(&row.poly_id))
        })
        .collect::<Vec<_>>();
    let summaries = group_summaries(&rows);

    let per_polytope_jsonl = args.out_dir.join("feature-cost-per-polytope.jsonl");
    let group_summary_tsv = args.out_dir.join("feature-cost-group-summary.tsv");
    let run_summary_json = args.out_dir.join("feature-cost-run-summary.json");
    write_jsonl(&per_polytope_jsonl, &rows);
    write_group_summary_tsv(&group_summary_tsv, &summaries);
    write_json(
        &run_summary_json,
        &RunSummary {
            schema: "sys_datascience_feature_cost_run_summary_v1",
            input_mode,
            out_dir: args.out_dir.display().to_string(),
            max_polytopes: args.max_polytopes,
            polytope_rows: caches.polytopes.len(),
            provenance_rows: caches.provenance_rows.len(),
            profiled_polytope_rows: rows.len(),
            per_polytope_jsonl: per_polytope_jsonl.display().to_string(),
            group_summary_tsv: group_summary_tsv.display().to_string(),
            boundary_note: "standard_prepare_feature_total uses the current prepare feature path and cached producer volume; feature_first_total_with_volume_recompute adds exact volume recomputation from reconstructed dual-vertex geometry.",
        },
    );

    print_compact_summary(&summaries);
    println!("Wrote {}", args.out_dir.display());
}
