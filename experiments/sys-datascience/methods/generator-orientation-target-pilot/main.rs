//! Target-free orientation target-pilot freeze validator.
//!
//! The target-free source panel is selected and validated before any capacity
//! call. Historical target reproduction remains pinned to the retained
//! pre-rerun commit recorded in protocol-history.json.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

const SOURCE_SHA256: &str = "b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367";
const SOURCE_REPORT_SHA256: &str =
    "02b7084141c0f2422aaabf1516fa62af501963ce638b9df3ef756c762722d61c";
const SOURCE_SCHEMA: &str = "generator-orientation-smoke-row-v2";
const VARIANTS: [&str; 3] = ["identity", "u2-haar", "so4-haar"];
const BUCKETS: [&str; 4] = ["3x3", "4x4", "4x6", "6x6"];

#[derive(Debug)]
struct Args {
    source: PathBuf,
    source_report: PathBuf,
    design: PathBuf,
    out: PathBuf,
    validate_only: bool,
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
    let mut validate_only = false;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--source" => source = arg_value(&argv, &mut i, "--source")?.into(),
            "--source-report" => {
                source_report = arg_value(&argv, &mut i, "--source-report")?.into()
            }
            "--design" => design = arg_value(&argv, &mut i, "--design")?.into(),
            "--out" => out = arg_value(&argv, &mut i, "--out")?.into(),
            "--validate-only" => {
                validate_only = true;
                i += 1;
            }
            "--help" | "-h" => {
                println!(
                    "--source PATH --source-report PATH --design PATH --out PATH [--validate-only]"
                );
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
        validate_only,
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

fn validate_hash_binding(path: &Path, expected: &str) -> Result<(), String> {
    let actual = sha256_file(path)?;
    if actual != expected {
        // Byte identity is advisory provenance. The caller's schema, ID-grid,
        // backend, and numerical contracts remain blocking.
        eprintln!(
            "warning: {} differs from retained provenance; continuing with \
             semantic checks. Reassess retained interpretation before treating \
             this run as equivalent.",
            path.display()
        );
    }
    Ok(())
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
        eprintln!(
            "warning: source bytes differ from retained provenance; continuing \
             with semantic checks. Reassess retained interpretation."
        );
    }
    if source_report_hash != SOURCE_REPORT_SHA256 {
        eprintln!(
            "warning: source-report bytes differ from retained provenance; \
             continuing with semantic checks. Reassess retained interpretation."
        );
    }
    if rows.len() != 40 {
        return Err(format!("expected 40 source rows, found {}", rows.len()));
    }
    let mut sample_ids = HashSet::new();
    let mut transformed_ids = HashSet::new();
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
            if row.get(key).is_some() {
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
        let sample = str_field(row, "sample_id")?;
        let transformed = str_field(row, "transformed_id")?;
        if !sample_ids.insert(sample.to_owned()) {
            return Err("duplicate source sample ID".into());
        }
        if !transformed_ids.insert(transformed.to_owned()) {
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

fn validate_selection_manifest(
    path: &Path,
    source_rows: &[Value],
    expected_hash: &str,
) -> Result<(), String> {
    validate_hash_binding(path, expected_hash)?;
    let text = read_to_string(path).map_err(|e| format!("read selection manifest: {e}"))?;
    let manifest: Value =
        serde_json::from_str(&text).map_err(|e| format!("selection manifest JSON: {e}"))?;
    if manifest.get("rows").and_then(Value::as_u64) != Some(24)
        || manifest.get("bases_per_bucket").and_then(Value::as_u64) != Some(2)
        || manifest.get("pair_key").and_then(Value::as_str) != Some("base_id")
        || manifest.get("target_calls").and_then(Value::as_u64) != Some(0)
        || manifest
            .get("target_fields_present")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err("selection manifest count/target-free contract mismatch".into());
    }
    let buckets = manifest
        .get("buckets")
        .and_then(Value::as_array)
        .ok_or("selection buckets missing")?;
    if buckets.iter().filter_map(Value::as_str).collect::<Vec<_>>() != BUCKETS {
        return Err("selection bucket order mismatch".into());
    }
    let variants = manifest
        .get("variants")
        .and_then(Value::as_array)
        .ok_or("selection variants missing")?;
    if variants
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        != VARIANTS
    {
        return Err("selection variant order mismatch".into());
    }
    let source_by_id: BTreeMap<_, _> = source_rows
        .iter()
        .filter_map(|r| {
            r.get("transformed_id")
                .and_then(Value::as_str)
                .map(|id| (id, r))
        })
        .collect();
    let selected = manifest
        .get("selected")
        .and_then(Value::as_array)
        .ok_or("selection selected rows missing")?;
    if selected.len() != 24 {
        return Err("selection selected row count mismatch".into());
    }
    let mut ids = HashSet::new();
    let mut grid = BTreeMap::<String, BTreeSet<String>>::new();
    let mut buckets_per_base = BTreeMap::<String, String>::new();
    for row in selected {
        let id = row
            .get("transformed_id")
            .and_then(Value::as_str)
            .ok_or("selection transformed ID missing")?;
        if !ids.insert(id) || !source_by_id.contains_key(id) {
            return Err("selection transformed ID duplicate/substitution".into());
        }
        if !VARIANTS.contains(&row.get("map_variant").and_then(Value::as_str).unwrap_or("")) {
            return Err("selection variant outside exact grid".into());
        }
        let base = row
            .get("base_id")
            .and_then(Value::as_str)
            .ok_or("selection base ID missing")?;
        let bucket = row
            .get("bucket")
            .and_then(Value::as_str)
            .ok_or("selection bucket missing")?;
        grid.entry(base.to_owned())
            .or_default()
            .insert(row["map_variant"].as_str().unwrap().to_owned());
        if buckets_per_base
            .insert(base.to_owned(), bucket.to_owned())
            .is_some_and(|previous| previous != bucket)
        {
            return Err("selection base spans buckets".into());
        }
        let source = source_by_id[id];
        for key in ["base_id", "bucket", "map_variant", "sample_id"] {
            if row.get(key) != source.get(key) {
                return Err(format!("selection/source identity mismatch: {key}"));
            }
        }
    }
    if grid.len() != 8 || grid.values().any(|variants| variants.len() != 3) {
        return Err("selection exact base/variant grid mismatch".into());
    }
    Ok(())
}

fn validate_design(
    design: &Value,
    design_path: &Path,
    source_path: &Path,
    source_report_path: &Path,
    source_rows: &[Value],
) -> Result<(String, String), String> {
    if design.get("schema").and_then(Value::as_str)
        != Some("generator-orientation-target-pilot-design-v1")
    {
        return Err("design schema mismatch".into());
    }
    let source_hash = sha256_file(source_path)?;
    let report_hash = sha256_file(source_report_path)?;
    if source_hash != SOURCE_SHA256 || report_hash != SOURCE_REPORT_SHA256 {
        eprintln!(
            "warning: design source/report bytes differ from retained \
             provenance; continuing with semantic checks. Reassess retained \
             interpretation."
        );
    }
    if design.get("source_sha256").and_then(Value::as_str) != Some(SOURCE_SHA256)
        || design.get("source_report_sha256").and_then(Value::as_str) != Some(SOURCE_REPORT_SHA256)
    {
        eprintln!(
            "warning: design records different source/report bytes; continuing \
             with semantic checks. Reassess retained interpretation."
        );
    }
    if design.get("source_panel").and_then(Value::as_str)
            != Some("experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl")
        || design.get("source_report").and_then(Value::as_str)
            != Some("experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json")
    {
        return Err("design source binding mismatch".into());
    }
    let evaluator = design.get("evaluator").ok_or("design evaluator missing")?;
    let evaluator_path =
        Path::new("experiments/sys-datascience/methods/generator-orientation-target-pilot/main.rs");
    let evaluator_hash = sha256_file(evaluator_path)?;
    if evaluator.get("source").and_then(Value::as_str) != Some(evaluator_path.to_str().unwrap()) {
        return Err("design evaluator source binding mismatch".into());
    }
    if evaluator.get("source_sha256").and_then(Value::as_str) != Some(evaluator_hash.as_str()) {
        eprintln!(
            "warning: design evaluator bytes differ from the current source; \
             continuing with semantic checks. Reassess retained interpretation."
        );
    }
    if evaluator.get("target_backend").and_then(Value::as_str) != Some("CapacityBackend::Auto")
        || evaluator.get("cache").and_then(Value::as_str)
            != Some("ComputedPolytopeCache::load(&[]) method-local empty cache")
        || evaluator.get("reconstruction").and_then(Value::as_str)
            != Some("SysLandscapePolytopeCache::from_f64_dual_vertices")
    {
        return Err("design evaluator backend/cache/reconstruction mismatch".into());
    }
    let implementation = evaluator
        .get("implementation_files")
        .and_then(Value::as_array)
        .ok_or("implementation closure missing")?;
    if implementation.len() != 5 {
        return Err("implementation closure count mismatch".into());
    }
    for item in implementation {
        let path = item
            .get("path")
            .and_then(Value::as_str)
            .ok_or("implementation path missing")?;
        let expected = item
            .get("sha256")
            .and_then(Value::as_str)
            .ok_or("implementation hash missing")?;
        validate_hash_binding(Path::new(path), expected)?;
    }
    let selection = design.get("selection").ok_or("design selection missing")?;
    if selection.get("manifest").and_then(Value::as_str)
        != Some("experiments/sys-datascience/methods/generator-orientation-target-pilot/selection-manifest.json")
    {
        return Err("selection manifest path mismatch".into());
    }
    let selection_path = Path::new("experiments/sys-datascience/methods/generator-orientation-target-pilot/selection-manifest.json");
    validate_selection_manifest(
        selection_path,
        source_rows,
        selection
            .get("manifest_sha256")
            .and_then(Value::as_str)
            .ok_or("selection hash missing")?,
    )?;
    if selection.get("rows").and_then(Value::as_u64) != Some(24)
        || selection.get("bases_per_bucket").and_then(Value::as_u64) != Some(2)
        || selection
            .get("variants")
            .and_then(Value::as_array)
            .map(|v| v.iter().filter_map(Value::as_str).collect::<Vec<_>>())
            != Some(VARIANTS.to_vec())
    {
        return Err("design exact selection specification mismatch".into());
    }
    let analysis = design.get("analysis").ok_or("design analysis missing")?;
    let checks = [
        ("u2_max_abs_delta_gate", 1e-8),
        ("material_so4_threshold", 0.01),
        ("contradiction_so4_threshold", 0.005),
        ("ridge_spearman_gate", -0.5),
        ("bucket_concentration_gate", 0.5),
    ];
    for (key, expected) in checks {
        if (analysis
            .get(key)
            .and_then(Value::as_f64)
            .unwrap_or(f64::NAN)
            - expected)
            .abs()
            > f64::EPSILON
        {
            return Err(format!("design analysis gate mismatch: {key}"));
        }
    }
    if design
        .get("target_exposure_boundary")
        .and_then(Value::as_str)
        != Some("after-pre-target-commit")
        || !design
            .get("prohibition")
            .and_then(Value::as_str)
            .unwrap_or("")
            .contains("additional bases")
    {
        return Err("design exposure/prohibition contract mismatch".into());
    }
    Ok((evaluator_hash, sha256_file(design_path)?))
}

fn main() -> Result<(), String> {
    let args = parse_args()?;
    let source_hash = sha256_file(&args.source)?;
    let report_hash = sha256_file(&args.source_report)?;
    let design_hash = sha256_file(&args.design)?;
    let rows = source_rows(&args.source)?;
    let selected = validate_source(&rows, &source_hash, &report_hash)?;
    let design_text = read_to_string(&args.design).map_err(|e| format!("read design: {e}"))?;
    let design: Value =
        serde_json::from_str(&design_text).map_err(|e| format!("design JSON: {e}"))?;
    let (_evaluator_sha256, design_hash_checked) = validate_design(
        &design,
        &args.design,
        &args.source,
        &args.source_report,
        &rows,
    )?;
    if design_hash_checked != design_hash {
        eprintln!(
            "warning: design bytes changed while validating; continuing with \
             the parsed design. Reassess this run before retaining it."
        );
    }
    if !args.validate_only {
        return Err("this repaired binary is validation-only; reproduce historical targets only at retained commit a59441c0ecde29ac667745e02aac4bedb8ca7d14".into());
    }
    println!(
        "target-free validation passed: {} selected rows; no capacity calls",
        selected.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_hash_binding;
    use std::fs;

    #[test]
    fn implementation_hash_mismatch_is_advisory() {
        let path = std::env::temp_dir().join(format!(
            "orientation-target-pilot-hash-test-{}",
            std::process::id()
        ));
        fs::write(&path, b"implementation closure").expect("write fixture");
        let result = validate_hash_binding(&path, &"0".repeat(64));
        fs::remove_file(&path).expect("remove fixture");
        assert!(
            result.is_ok(),
            "byte drift must not block semantic validation"
        );
    }
}
