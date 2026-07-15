//! Frozen 91-row target evaluator.
//!
//! This is intentionally a one-shot command: it validates immutable inputs,
//! reconstructs only stored exact geometry, evaluates each selected ID once,
//! and atomically finalizes one complete JSONL artifact. There is no cache or
//! resume path. Tests exercise the join and write gates with a fake function;
//! the real capacity backend is only reached by the future `evaluate` command.

use exp_sys_landscape::{capacity_auto, compute_sys_from_capacity, SysLandscapePolytopeCache};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

const SOURCE_SHA256: &str = "161f6361fd9c99b1b86a863c3cdb7db438fd76329392992f6212e37c83e69963";
const FEATURE_SHA256: &str = "8a87ef1a050cd9b3a717c85a43b0577f9e72c308e635fcc93defed58ec8883a5";
const SELECTION_SHA256: &str = "2e4953cc61fa3eb02405c2fff9844c842c7813fd05edb7a741413574b794a168";
const EVALUATOR_IDENTITY_SCHEMA: &str = "alternative-source-transfer-evaluator-identity-v1";
const EVALUATOR_SOURCE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../sys-datascience/methods/alternative-source-transfer/src/evaluator.rs"
));
const EVALUATOR_LOCK_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock"));
const EVALUATOR_BACKEND_LIB_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
const EVALUATOR_BACKEND_CACHE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/sys_landscape_cache.rs"
));
const EVALUATOR_BACKEND_COMPUTE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/ascent/compute.rs"
));

#[derive(Clone, Deserialize)]
struct SourceRow {
    schema: String,
    candidate_id: String,
    logical_cell: String,
    bucket: String,
    exact_dual_vertices: Vec<[String; 4]>,
    exact_primal_vertices: Vec<[String; 4]>,
    vertex_facet_incidence: Vec<Vec<bool>>,
    volume: f64,
    geometry_fingerprint: String,
}

#[derive(Clone, Deserialize)]
struct SelectionRow {
    candidate_id: String,
    logical_cell: String,
    bucket: String,
    memberships: Vec<String>,
    geometry_fingerprint: String,
}

#[derive(Deserialize)]
struct Manifest {
    schema: String,
    identity_scope: String,
    master_seed: u64,
    control_seed: u64,
    law: String,
    buckets: Vec<String>,
    row_target_per_bucket: usize,
    row_cap_per_bucket: usize,
    attempt_cap: usize,
    source_sha256: String,
    feature_sha256: String,
    selection_sha256: String,
    source_count: usize,
    feature_count: usize,
    selection_count: usize,
    unique_target_rows: usize,
    arm_overlap_rows: usize,
    target_free: bool,
}

#[derive(Clone, Debug, Serialize)]
struct TargetRow {
    schema: &'static str,
    candidate_id: String,
    logical_cell: String,
    bucket: String,
    selection_memberships: Vec<String>,
    geometry_fingerprint: String,
    source_sha256: &'static str,
    feature_sha256: &'static str,
    selection_sha256: &'static str,
    evaluator_identity_schema: &'static str,
    evaluator_source_sha256: String,
    evaluator_lock_sha256: String,
    evaluator_backend_sha256: String,
    evaluator_git_commit: String,
    evaluator_git_clean: bool,
    volume: f64,
    capacity: f64,
    sys: f64,
}

fn digest(path: &Path) -> String {
    let mut h = Sha256::new();
    h.update(fs::read(path).expect("artifact exists"));
    format!("{:x}", h.finalize())
}

fn digest_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn digest_backend() -> String {
    let mut h = Sha256::new();
    for (name, bytes) in [
        ("src/lib.rs", EVALUATOR_BACKEND_LIB_BYTES),
        ("src/sys_landscape_cache.rs", EVALUATOR_BACKEND_CACHE_BYTES),
        ("src/ascent/compute.rs", EVALUATOR_BACKEND_COMPUTE_BYTES),
    ] {
        h.update(name.as_bytes());
        h.update([0]);
        h.update(bytes);
        h.update([0]);
    }
    format!("{:x}", h.finalize())
}

fn evaluator_identity() -> Result<(String, String, String, String, bool), String> {
    let output = Command::new("git")
        .args(["-C", env!("CARGO_MANIFEST_DIR"), "rev-parse", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;
    let commit = String::from_utf8(output.stdout)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();
    if commit.is_empty() {
        return Err("cannot determine repository HEAD".into());
    }
    let status = Command::new("git")
        .args([
            "-C",
            env!("CARGO_MANIFEST_DIR"),
            "status",
            "--porcelain",
            "--untracked-files=all",
        ])
        .output()
        .map_err(|e| e.to_string())?;
    let clean = status.status.success() && status.stdout.is_empty() && status.stderr.is_empty();
    Ok((
        digest_bytes(EVALUATOR_SOURCE_BYTES),
        digest_bytes(EVALUATOR_LOCK_BYTES),
        digest_backend(),
        commit,
        clean,
    ))
}

fn read_jsonl<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<Vec<T>, String> {
    BufReader::new(File::open(path).map_err(|e| e.to_string())?)
        .lines()
        .map(|line| {
            serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())
        })
        .collect()
}

fn rational(value: &str) -> Result<BigRational, String> {
    BigRational::from_str(value).map_err(|e| format!("invalid exact coordinate {value}: {e}"))
}

fn geometry(row: &SourceRow) -> Result<SysLandscapePolytopeCache, String> {
    let dual = row
        .exact_dual_vertices
        .iter()
        .map(|v| {
            Ok([
                rational(&v[0])?,
                rational(&v[1])?,
                rational(&v[2])?,
                rational(&v[3])?,
            ])
        })
        .collect::<Result<Vec<_>, String>>()?;
    let primal = row
        .exact_primal_vertices
        .iter()
        .map(|v| {
            Ok([
                rational(&v[0])?,
                rational(&v[1])?,
                rational(&v[2])?,
                rational(&v[3])?,
            ])
        })
        .collect::<Result<Vec<_>, String>>()?;
    SysLandscapePolytopeCache::from_rational_parts(dual, primal)
        .ok_or_else(|| "stored exact geometry reconstruction failed".into())
}

fn load_inputs(out: &Path) -> Result<Vec<(SourceRow, SelectionRow)>, String> {
    let manifest: Manifest = serde_json::from_str(
        &fs::read_to_string(out.join("manifest.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if manifest.schema != "alternative-source-transfer-manifest-v1"
        || !manifest.target_free
        || manifest.identity_scope != "alternative-source-transfer-v1"
        || manifest.master_seed != 2026071601
        || manifest.control_seed != 2026071299
        || manifest.law != "factorial-both"
        || manifest.buckets != vec!["4x6", "6x6"]
        || manifest.row_target_per_bucket != 3200
        || manifest.row_cap_per_bucket != 4000
        || manifest.attempt_cap != 128
        || manifest.unique_target_rows != 91
        || manifest.arm_overlap_rows != 5
    {
        return Err("manifest schema/constants gate failed".into());
    }
    if manifest.source_sha256 != SOURCE_SHA256
        || manifest.feature_sha256 != FEATURE_SHA256
        || manifest.selection_sha256 != SELECTION_SHA256
    {
        return Err("frozen artifact hash gate failed".into());
    }
    if digest(&out.join("source.jsonl")) != SOURCE_SHA256
        || digest(&out.join("features.jsonl")) != FEATURE_SHA256
        || digest(&out.join("selection.jsonl")) != SELECTION_SHA256
    {
        return Err("artifact bytes do not match frozen hashes".into());
    }
    if manifest.source_count != 6400
        || manifest.feature_count != 6400
        || manifest.selection_count != 91
    {
        return Err("manifest count gate failed".into());
    }
    let source: Vec<SourceRow> = read_jsonl(&out.join("source.jsonl"))?;
    let selection: Vec<SelectionRow> = read_jsonl(&out.join("selection.jsonl"))?;
    if source.len() != 6400 || selection.len() != 91 {
        return Err("artifact count gate failed".into());
    }
    let mut source_by_id = BTreeMap::new();
    for row in source {
        if row.schema != "alternative-source-transfer-source-v1"
            || source_by_id.insert(row.candidate_id.clone(), row).is_some()
        {
            return Err("source identity/schema gate failed".into());
        }
    }
    let mut seen = BTreeMap::new();
    let mut joined = Vec::with_capacity(selection.len());
    for pick in selection {
        if seen.insert(pick.candidate_id.clone(), true).is_some() {
            return Err("duplicate selected ID".into());
        }
        let source = source_by_id
            .get(&pick.candidate_id)
            .ok_or("selection references unknown source")?;
        if source.logical_cell != pick.logical_cell
            || source.bucket != pick.bucket
            || source.geometry_fingerprint != pick.geometry_fingerprint
        {
            return Err("selection/source identity or geometry mismatch".into());
        }
        joined.push((source.clone(), pick));
    }
    Ok(joined)
}

fn evaluate_rows<F>(
    joined: &[(SourceRow, SelectionRow)],
    mut target: F,
    identity: &(String, String, String, String, bool),
) -> Result<Vec<TargetRow>, String>
where
    F: FnMut(&SourceRow) -> Result<(f64, f64, f64), String>,
{
    if joined.len() != 91 {
        return Err("evaluator requires exactly 91 selected rows".into());
    }
    if !identity.4 {
        return Err("evaluator requires a clean Git checkout".into());
    }
    let mut output = Vec::with_capacity(joined.len());
    for (source, pick) in joined {
        let (volume, capacity, sys) = target(source)?;
        if !volume.is_finite() || !capacity.is_finite() || !sys.is_finite() {
            return Err("target result is nonfinite".into());
        }
        output.push(TargetRow {
            schema: "alternative-source-transfer-target-v1",
            candidate_id: source.candidate_id.clone(),
            logical_cell: source.logical_cell.clone(),
            bucket: source.bucket.clone(),
            selection_memberships: pick.memberships.clone(),
            geometry_fingerprint: source.geometry_fingerprint.clone(),
            source_sha256: SOURCE_SHA256,
            feature_sha256: FEATURE_SHA256,
            selection_sha256: SELECTION_SHA256,
            evaluator_identity_schema: EVALUATOR_IDENTITY_SCHEMA,
            evaluator_source_sha256: identity.0.clone(),
            evaluator_lock_sha256: identity.1.clone(),
            evaluator_backend_sha256: identity.2.clone(),
            evaluator_git_commit: identity.3.clone(),
            evaluator_git_clean: identity.4,
            volume,
            capacity,
            sys,
        });
    }
    Ok(output)
}

fn write_atomic(path: &Path, rows: &[TargetRow]) -> Result<(), String> {
    if path.exists() {
        return Err("refusing to overwrite existing finalized target output".into());
    }
    let tmp = path.with_extension("jsonl.tmp");
    let result = (|| {
        let mut w = BufWriter::new(File::create(&tmp).map_err(|e| e.to_string())?);
        for row in rows {
            serde_json::to_writer(&mut w, row).map_err(|e| e.to_string())?;
            w.write_all(b"\n").map_err(|e| e.to_string())?;
        }
        w.flush().map_err(|e| e.to_string())?;
        fs::rename(&tmp, path).map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

fn real_target(source: &SourceRow) -> Result<(f64, f64, f64), String> {
    let poly = geometry(source)?;
    let capacity = capacity_auto(
        &poly.dual_vertices_f64,
        &poly.dual_vertices,
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    )
    .map_err(|e| format!("capacity failed: {e:?}"))?;
    let sys = compute_sys_from_capacity(&poly, &capacity)
        .ok_or("sys computation returned no finite value")?;
    Ok((source.volume, capacity.min_action, sys))
}

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    if cmd != "evaluate" {
        eprintln!("future command: evaluate OUT_DIR TARGET_OUT");
        std::process::exit(2);
    }
    let out = PathBuf::from(args.next().expect("OUT_DIR"));
    let target = PathBuf::from(args.next().expect("TARGET_OUT"));
    let joined = load_inputs(&out).unwrap_or_else(|e| panic!("refusing target evaluation: {e}"));
    let identity =
        evaluator_identity().unwrap_or_else(|e| panic!("cannot establish evaluator identity: {e}"));
    let rows = evaluate_rows(&joined, real_target, &identity)
        .unwrap_or_else(|e| panic!("target evaluation failed: {e}"));
    write_atomic(&target, &rows).unwrap_or_else(|e| panic!("target output not finalized: {e}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fake_target_emits_one_row_per_unique_id() {
        let mut joined = Vec::new();
        for i in 0..91 {
            let id = i.to_string();
            joined.push((
                SourceRow {
                    schema: "alternative-source-transfer-source-v1".into(),
                    candidate_id: id.clone(),
                    logical_cell: id.clone(),
                    bucket: "4x6".into(),
                    exact_dual_vertices: vec![],
                    exact_primal_vertices: vec![],
                    vertex_facet_incidence: vec![],
                    volume: 1.,
                    geometry_fingerprint: id.clone(),
                },
                SelectionRow {
                    candidate_id: id.clone(),
                    logical_cell: id.clone(),
                    bucket: "4x6".into(),
                    memberships: vec!["rho".into()],
                    geometry_fingerprint: id,
                },
            ));
        }
        let identity = (
            "source".into(),
            "lock".into(),
            "backend".into(),
            "commit".into(),
            true,
        );
        let rows = evaluate_rows(&joined, |_| Ok((1., 2., 0.5)), &identity).unwrap();
        assert_eq!(rows.len(), 91);
        assert_eq!(
            rows.iter()
                .map(|row| &row.candidate_id)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            91
        );
    }
    #[test]
    fn fake_target_rejects_nonfinite_result() {
        let mut joined = Vec::new();
        for i in 0..91 {
            let id = i.to_string();
            joined.push((
                SourceRow {
                    schema: "alternative-source-transfer-source-v1".into(),
                    candidate_id: id.clone(),
                    logical_cell: id.clone(),
                    bucket: "4x6".into(),
                    exact_dual_vertices: vec![],
                    exact_primal_vertices: vec![],
                    vertex_facet_incidence: vec![],
                    volume: 1.,
                    geometry_fingerprint: id.clone(),
                },
                SelectionRow {
                    candidate_id: id.clone(),
                    logical_cell: id.clone(),
                    bucket: "4x6".into(),
                    memberships: vec!["rho".into()],
                    geometry_fingerprint: id,
                },
            ));
        }
        let identity = (
            "source".into(),
            "lock".into(),
            "backend".into(),
            "commit".into(),
            true,
        );
        let err = evaluate_rows(&joined, |_| Ok((1., 2., f64::NAN)), &identity).unwrap_err();
        assert!(err.contains("nonfinite"));
    }

    #[test]
    fn fake_target_requires_clean_identity() {
        let joined = (0..91)
            .map(|i| {
                let id = i.to_string();
                (
                    SourceRow {
                        schema: "alternative-source-transfer-source-v1".into(),
                        candidate_id: id.clone(),
                        logical_cell: id.clone(),
                        bucket: "4x6".into(),
                        exact_dual_vertices: vec![],
                        exact_primal_vertices: vec![],
                        vertex_facet_incidence: vec![],
                        volume: 1.,
                        geometry_fingerprint: id.clone(),
                    },
                    SelectionRow {
                        candidate_id: id.clone(),
                        logical_cell: id.clone(),
                        bucket: "4x6".into(),
                        memberships: vec!["rho".into()],
                        geometry_fingerprint: id,
                    },
                )
            })
            .collect::<Vec<_>>();
        let identity = (
            "source".into(),
            "lock".into(),
            "backend".into(),
            "commit".into(),
            false,
        );
        let err = evaluate_rows(&joined, |_| Ok((1., 2., 0.5)), &identity).unwrap_err();
        assert!(err.contains("clean Git"));
    }

    #[test]
    fn write_atomic_refuses_overwrite() {
        let dir = std::env::temp_dir().join(format!(
            "alternative-source-transfer-overwrite-test-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("target.jsonl");
        fs::write(&path, b"existing\n").unwrap();
        let err = write_atomic(&path, &[]).unwrap_err();
        assert!(err.contains("overwrite"));
        assert_eq!(fs::read_to_string(path).unwrap(), "existing\n");
        fs::remove_dir_all(dir).unwrap();
    }
}
