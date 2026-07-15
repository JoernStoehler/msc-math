//! Frozen 24-row orientation target evaluator.
//!
//! The target-free source panel is selected and validated before any capacity
//! call. Each selected f64 dual payload is reconstructed locally and evaluated
//! with an empty method-local ComputedPolytopeCache.

use exp_sys_landscape::{CapacityBackend, ComputedPolytopeCache, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

const SOURCE_SHA256: &str = "b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367";
const SOURCE_REPORT_SHA256: &str =
    "02b7084141c0f2422aaabf1516fa62af501963ce638b9df3ef756c762722d61c";
const SOURCE_SCHEMA: &str = "generator-orientation-smoke-row-v2";
const TARGET_SCHEMA: &str = "generator-orientation-target-pilot-row-v1";
const MANIFEST_SCHEMA: &str = "generator-orientation-target-pilot-manifest-v1";
const COORDINATE_ORDER: &str = "q1,q2,p1,p2";
const VARIANTS: [&str; 3] = ["identity", "u2-haar", "so4-haar"];
const BUCKETS: [&str; 4] = ["3x3", "4x4", "4x6", "6x6"];

const EVALUATOR_SOURCE: &[u8] = include_bytes!("main.rs");

#[derive(Debug)]
struct Args {
    source: PathBuf,
    source_report: PathBuf,
    design: PathBuf,
    out: PathBuf,
}

fn arg_value(args: &[String], index: &mut usize, flag: &str) -> Result<String, String> {
    let value = args
        .get(*index + 1)
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))?;
    *index += 2;
    Ok(value)
}

fn parse_args() -> Result<Args, String> {
    let argv = std::env::args().collect::<Vec<_>>();
    let mut source = PathBuf::from("experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl");
    let mut source_report = PathBuf::from("experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json");
    let mut design = PathBuf::from(
        "experiments/sys-datascience/methods/generator-orientation-target-pilot/design.json",
    );
    let mut out = PathBuf::from("experiments/sys-datascience/methods/generator-orientation-target-pilot/artifacts/target-rows.jsonl");
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--source" => source = arg_value(&argv, &mut i, "--source")?.into(),
            "--source-report" => {
                source_report = arg_value(&argv, &mut i, "--source-report")?.into()
            }
            "--design" => design = arg_value(&argv, &mut i, "--design")?.into(),
            "--out" => out = arg_value(&argv, &mut i, "--out")?.into(),
            "--help" | "-h" => {
                println!("--source PATH --source-report PATH --design PATH --out PATH");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(Args {
        source,
        source_report,
        design,
        out,
    })
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .map_err(|e| format!("sha256sum {}: {e}", path.display()))?;
    if !output.status.success() {
        return Err(format!("sha256sum failed for {}", path.display()));
    }
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| format!("sha256sum returned no digest for {}", path.display()))
}

fn source_rows(path: &Path) -> Result<Vec<Value>, String> {
    let file = File::open(path).map_err(|e| format!("open source {}: {e}", path.display()))?;
    BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(line, line_result)| {
            let text = line_result.map_err(|e| format!("read source line {}: {e}", line + 1))?;
            if text.trim().is_empty() {
                return Err(format!("empty source line {}", line + 1));
            }
            serde_json::from_str(&text).map_err(|e| format!("source line {} JSON: {e}", line + 1))
        })
        .collect()
}

fn str_field<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("source field {key} missing or non-string"))
}

fn validate_source(
    rows: &[Value],
    source_hash: &str,
    source_report_hash: &str,
) -> Result<Vec<Value>, String> {
    if source_hash != SOURCE_SHA256 {
        return Err(format!("source hash mismatch: {source_hash}"));
    }
    if source_report_hash != SOURCE_REPORT_SHA256 {
        return Err(format!("source report hash mismatch: {source_report_hash}"));
    }
    if rows.len() != 40 {
        return Err(format!("expected 40 source rows, found {}", rows.len()));
    }
    let mut ids = HashSet::new();
    let mut bases = BTreeMap::<String, BTreeSet<String>>::new();
    let forbidden = [
        "capacity",
        "sys",
        "target",
        "iterations",
        "iteration",
        "bounce_label",
    ];
    for row in rows {
        if str_field(row, "schema")? != SOURCE_SCHEMA {
            return Err("unexpected source schema".into());
        }
        for key in forbidden {
            if row.get(key).is_some() && !row[key].is_null() {
                return Err(format!("source contains target field {key}"));
            }
        }
        if row.get("base_accepted").and_then(Value::as_bool) != Some(true)
            || row
                .get("semantic_invariants_passed")
                .and_then(Value::as_bool)
                != Some(true)
            || str_field(row, "reconstruction_status")? != "reconstructed"
            || str_field(row, "map_status")? != "generated"
            || row
                .get("invariant_failures")
                .and_then(Value::as_array)
                .map_or(true, |v| !v.is_empty())
        {
            return Err("source reconstruction/semantic status failed".into());
        }
        let base = str_field(row, "base_id")?;
        let _sample = str_field(row, "sample_id")?;
        let transformed = str_field(row, "transformed_id")?;
        if !ids.insert(transformed.to_owned()) {
            return Err("duplicate source transformed ID".into());
        }
        let bucket = str_field(row, "bucket")?;
        if !BUCKETS.contains(&bucket) {
            return Err(format!("unexpected bucket {bucket}"));
        }
        let variant = str_field(row, "map_variant")?;
        bases
            .entry(base.to_owned())
            .or_default()
            .insert(variant.to_owned());
    }
    if bases.len() != 8 {
        return Err("source base count/grid failed".into());
    }
    let mut selected = Vec::new();
    let mut bucket_counts = BTreeMap::<String, usize>::new();
    for (base, variants) in &bases {
        if variants
            != &BTreeSet::from([
                "identity".into(),
                "u2-deterministic".into(),
                "u2-haar".into(),
                "so4-deterministic".into(),
                "so4-haar".into(),
            ])
        {
            return Err(format!("base {base} has wrong variant set"));
        }
        let group = rows
            .iter()
            .filter(|r| r.get("base_id").and_then(Value::as_str) == Some(base));
        let mut bucket = None;
        for row in group {
            bucket.get_or_insert(str_field(row, "bucket")?);
            if bucket != Some(str_field(row, "bucket")?) {
                return Err(format!("base {base} spans buckets"));
            }
            if VARIANTS.contains(&str_field(row, "map_variant")?) {
                selected.push(row.clone());
            }
        }
        *bucket_counts.entry(bucket.unwrap().to_owned()).or_default() += 1;
    }
    if bucket_counts
        != BTreeMap::from([
            ("3x3".into(), 2),
            ("4x4".into(), 2),
            ("4x6".into(), 2),
            ("6x6".into(), 2),
        ])
    {
        return Err(format!("wrong bucket counts {bucket_counts:?}"));
    }
    if selected.len() != 24 {
        return Err(format!(
            "expected 24 selected rows, found {}",
            selected.len()
        ));
    }
    Ok(selected)
}

fn dual_vertices(row: &Value) -> Result<Vec<Vector4<f64>>, String> {
    let values = row
        .get("transformed_dual_vertices_f64")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing transformed dual vertices".to_string())?;
    values
        .iter()
        .map(|v| {
            let a = v
                .as_array()
                .ok_or_else(|| "dual vertex is not array".to_string())?;
            if a.len() != 4 {
                return Err("dual vertex has wrong dimension".into());
            }
            let x = a
                .iter()
                .map(|x| {
                    x.as_f64()
                        .ok_or_else(|| "dual coordinate is non-f64".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            if !x.iter().all(|v| v.is_finite()) {
                return Err("nonfinite dual coordinate".into());
            }
            Ok(Vector4::new(x[0], x[1], x[2], x[3]))
        })
        .collect()
}

fn fail_manifest(
    path: &Path,
    args: &Args,
    source_hash: &str,
    source_report_hash: &str,
    design_hash: &str,
    reason: &str,
    completed: usize,
) -> Result<(), String> {
    let manifest = json!({"schema": MANIFEST_SCHEMA, "status": "failed", "failure": reason, "completed_rows": completed,
        "expected_rows": 24, "source_path": args.source, "source_sha256": source_hash, "source_report_path": args.source_report,
        "source_report_sha256": source_report_hash, "design_path": args.design, "design_sha256": design_hash,
        "evaluator_source_blake3": blake3::hash(EVALUATOR_SOURCE).to_hex().to_string()});
    let path = path.with_file_name("target-manifest.json");
    std::fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap())
        .map_err(|e| format!("write {}: {e}", path.display()))
}

fn main() -> Result<(), String> {
    let args = parse_args()?;
    let source_hash = sha256_file(&args.source)?;
    let report_hash = sha256_file(&args.source_report)?;
    let design_hash = sha256_file(&args.design)?;
    let evaluator_path =
        Path::new("experiments/sys-datascience/methods/generator-orientation-target-pilot/main.rs");
    let evaluator_sha256 = sha256_file(evaluator_path)?;
    let repo_commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("git rev-parse: {e}"))?;
    let repo_commit = String::from_utf8_lossy(&repo_commit.stdout)
        .trim()
        .to_owned();
    let design_text = read_to_string(&args.design).map_err(|e| format!("read design: {e}"))?;
    let design: Value =
        serde_json::from_str(&design_text).map_err(|e| format!("design JSON: {e}"))?;
    if design
        .get("target_exposure_boundary")
        .and_then(Value::as_str)
        != Some("after-pre-target-commit")
    {
        return Err("design target boundary missing".into());
    }
    let rows = source_rows(&args.source)?;
    let selected = match validate_source(&rows, &source_hash, &report_hash) {
        Ok(rows) => rows,
        Err(error) => {
            let _ = fail_manifest(
                &args.out,
                &args,
                &source_hash,
                &report_hash,
                &design_hash,
                &error,
                0,
            );
            return Err(error);
        }
    };
    if let Some(parent) = args.out.parent() {
        create_dir_all(parent).map_err(|e| format!("create output parent: {e}"))?;
    }
    let file = File::create(&args.out).map_err(|e| format!("create target output: {e}"))?;
    let mut writer = BufWriter::new(file);
    let cache = ComputedPolytopeCache::load(&[]);
    let mut completed = 0usize;
    let mut seen = HashSet::new();
    let start = Instant::now();
    for source in selected {
        let source_id = str_field(&source, "transformed_id")?.to_owned();
        if !seen.insert(source_id.clone()) {
            return Err("duplicate selected target ID".into());
        }
        let dual = match dual_vertices(&source) {
            Ok(v) => v,
            Err(error) => {
                let _ = fail_manifest(
                    &args.out,
                    &args,
                    &source_hash,
                    &report_hash,
                    &design_hash,
                    &error,
                    completed,
                );
                return Err(error);
            }
        };
        let poly = match SysLandscapePolytopeCache::from_f64_dual_vertices(dual) {
            Some(v) => v,
            None => {
                let error = format!("reconstruction failed for {source_id}");
                let _ = fail_manifest(
                    &args.out,
                    &args,
                    &source_hash,
                    &report_hash,
                    &design_hash,
                    &error,
                    completed,
                );
                return Err(error);
            }
        };
        let payload = match cache.compute(&poly, CapacityBackend::Auto) {
            Some(v) => v,
            None => {
                let error = format!("capacity failed for {source_id}");
                let _ = fail_manifest(
                    &args.out,
                    &args,
                    &source_hash,
                    &report_hash,
                    &design_hash,
                    &error,
                    completed,
                );
                return Err(error);
            }
        };
        let values = json!({
            "schema": TARGET_SCHEMA, "target_status": "complete", "source_id": source_id,
            "sample_id": source["sample_id"], "transformed_id": source["transformed_id"], "base_id": source["base_id"],
            "bucket": source["bucket"], "q_sides": source["q_sides"], "p_sides": source["p_sides"],
            "map_variant": source["map_variant"], "map_family": source["map_family"], "map_mode": source["map_mode"],
            "map_seed": source["map_seed"], "row_index": source["row_index"], "facet_count": payload.facet_count,
            "poly_id": payload.poly_id, "backend": payload.backend, "exact_volume_as_f64": payload.volume,
            "volume": payload.volume, "capacity": payload.capacity, "sys": payload.sys,
            "sigma_gap_cutoff": payload.sigma_gap_cutoff, "sigmas": payload.sigmas, "orbit_scalars": payload.orbit_scalars,
            "time_volume_ms": payload.time_volume_ms, "time_capacity_ms": payload.time_capacity_ms,
            "source_sha256": source_hash, "source_report_sha256": report_hash, "design_sha256": design_hash,
            "evaluator_source_sha256": evaluator_sha256,
            "coordinate_order": COORDINATE_ORDER, "total_elapsed_ms": start.elapsed().as_secs_f64()*1000.0,
            "source_transformed_dual_vertices_f64": source["transformed_dual_vertices_f64"]
        });
        serde_json::to_writer(&mut writer, &values)
            .map_err(|e| format!("write target row: {e}"))?;
        writeln!(&mut writer).map_err(|e| format!("write target newline: {e}"))?;
        writer
            .flush()
            .map_err(|e| format!("flush target row: {e}"))?;
        completed += 1;
    }
    let manifest = json!({"schema": MANIFEST_SCHEMA, "status": "complete", "completed_rows": completed, "expected_rows": 24,
        "source_path": args.source, "source_sha256": source_hash, "source_report_path": args.source_report,
        "source_report_sha256": report_hash, "design_path": args.design, "design_sha256": design_hash,
        "target_path": args.out, "target_schema": TARGET_SCHEMA, "target_sha256": sha256_file(&args.out)?,
        "evaluator_source_sha256": evaluator_sha256, "backend": "auto", "method_local_cache": true,
        "pre_target_commit": repo_commit, "provenance": {"implementation_files": ["experiments/sys-landscape/src/datascience_cache.rs", "experiments/sys-landscape/src/sys_landscape_cache.rs", "experiments/sys-landscape/src/lib.rs", "experiments/sys-landscape/Cargo.toml", "Cargo.lock"]}});
    let manifest_path = args.out.with_file_name("target-manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .map_err(|e| format!("write manifest: {e}"))?;
    println!(
        "completed {completed} target rows in {:.3}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}
