//! Frozen generic ridge-tail stage-one target evaluation.
//!
//! This sibling packet is the only stage-one component allowed to call the
//! capacity backend.  It consumes the immutable 200-row panel and never
//! generates or reranks candidates.

#[path = "../../../prepare/features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "../../../prepare/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::volume_from_incidence_f64;
use exp_sys_landscape::{capacity_auto, poly_id, SysLandscapePolytopeCache};
use rayon::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::systolic_ratio;

const FROZEN_VALIDATION_SHA256: &str =
    "524dce5b2e93ba1090f6a648f87793fee0a8665d64ae929a326d1ceee2cd6bc3";
const FROZEN_MANIFEST_SHA256: &str =
    "b57fdba2afde97a2bec8644d4dc02a00104ec2bc088fdae66d52539a4ceea936";
const FROZEN_SELECTION_SHA256: &str =
    "f155c6868c3414af1e33c78aff6dc4eaf0156d92577f915db4a29ca6ebfbc23f";
const FROZEN_PANEL_SHA256: &str =
    "a08c846d412ab77a7974b23b13cac6be0d7eb0418617cc551a8ecbb0a0f6a379";
const FROZEN_SELECTED_HASH: &str =
    "7536b8e6d12691a12878f3f17bf8ce0bcdab910697ad581459713242a310b68b";
const FROZEN_BASELINE_HASH: &str =
    "69ec2feb35faa19c8ed9fc31b4dd4a674dac30ea63b78431f47677dcda11f763";
const FROZEN_PANEL_HASH: &str = "af5492cea21899774b47cbb578c316972fdee8285fbd7bd61d608adb5ac3c708";
const TARGET_RUN_SOURCE_SHA256: &str =
    "c2441988497351f719a6b59bccc37133912eb0b95d07a0b942c58b74b2645ede";
const THRESHOLD: f64 = 0.5949424195457518;
const MAX_ROWS: usize = 200;

#[derive(Clone, Debug, Deserialize)]
struct FullValidation {
    valid: bool,
    candidate_population_hash: String,
    selected_hash: String,
    baseline_hash: String,
    panel_hash: String,
    checks: BTreeMap<String, bool>,
}

#[derive(Clone, Debug, Deserialize)]
struct Manifest {
    candidate_population_hash: String,
    selected_hash: String,
    baseline_hash: String,
    panel_hash: String,
    counts: Counts,
    target_exposure: TargetExposure,
}
#[derive(Clone, Debug, Deserialize)]
struct Counts {
    accepted_candidates: usize,
    selected: usize,
    baseline: usize,
    panel_union: usize,
}
#[derive(Clone, Debug, Deserialize)]
struct TargetExposure {
    capacity_computed_for_new_population: bool,
    sys_computed_for_new_population: bool,
    target_fields_present_in_stage_one_artifacts: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct PanelRow {
    schema: String,
    candidate_id: String,
    poly_id: String,
    sample_index: usize,
    rejection_attempt: u64,
    facet_count: usize,
    height_min: f64,
    height_max: f64,
    selection_ids: Vec<String>,
    baseline_ids: Vec<String>,
    evaluation_roles: Vec<String>,
    future_band: String,
    proxy: String,
    proxy_value_f64: f64,
    f64_rank: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    stage_order: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Preflight {
    schema: String,
    valid: bool,
    target_calls: bool,
    full_validation_sha256: String,
    artifact_sha256: BTreeMap<String, String>,
    selected_hash: String,
    baseline_hash: String,
    panel_hash: String,
    panel_rows: usize,
    selected_rows: usize,
    baseline_rows: usize,
    checks: BTreeMap<String, bool>,
}

#[derive(Clone, Debug, Serialize)]
struct TargetRow {
    candidate_id: String,
    poly_id: String,
    sample_index: usize,
    rejection_attempt: u64,
    role: String,
    future_band: String,
    f64_rank: usize,
    proxy: f64,
    ridge_count: usize,
    ridge_symp_area_mean: f64,
    f64_volume: f64,
    capacity: f64,
    sys: f64,
    backend: String,
    capacity_iterations: u64,
    returned_orbit_count: usize,
    best_sigma: Vec<usize>,
    best_action: f64,
    best_beta_margin: f64,
    best_admissibility: String,
    time_reconstruct_ms: f64,
    time_capacity_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EvaluationManifest {
    schema: String,
    status: String,
    row_count: usize,
    selected_count: usize,
    baseline_count: usize,
    threshold: f64,
    volume_definition: String,
    capacity_route: String,
    preflight_sha256: String,
    evaluator_source_sha256: String,
    #[serde(default)]
    target_evaluator_source_sha256: String,
    #[serde(default)]
    manifest_repair_source_sha256: String,
    target_free_full_validation_sha256: String,
    target_calls_for_new_population: bool,
    rows_sha256: String,
    #[serde(default)]
    rows_blake3: String,
    wall_ms: f64,
    process_user_cpu_seconds: f64,
    process_system_cpu_seconds: f64,
    max_rss_kib: i64,
}

#[derive(Clone, Copy)]
struct Usage {
    user: f64,
    system: f64,
    rss: i64,
}
fn usage() -> Usage {
    unsafe {
        let mut r: libc::rusage = std::mem::zeroed();
        assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut r), 0);
        let s = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 / 1e6;
        Usage {
            user: s(r.ru_utime),
            system: s(r.ru_stime),
            rss: r.ru_maxrss,
        }
    }
}

fn sha256(path: &Path) -> String {
    let out = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum");
    assert!(
        out.status.success(),
        "sha256sum failed for {}",
        path.display()
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
fn blake3_file(path: &Path) -> String {
    let mut file = File::open(path).expect("open hash file");
    let mut h = blake3::Hasher::new();
    let mut b = [0u8; 1 << 20];
    loop {
        let n = file.read(&mut b).expect("read hash file");
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    h.finalize().to_hex().to_string()
}
fn read_json<T: DeserializeOwned>(path: &Path) -> T {
    serde_json::from_reader(File::open(path).expect("open JSON")).expect("parse JSON")
}
fn read_jsonl<T: for<'a> Deserialize<'a>>(path: &Path) -> Vec<T> {
    BufReader::new(File::open(path).expect("open JSONL"))
        .lines()
        .map(|l| serde_json::from_str(&l.expect("read JSONL")).expect("parse JSONL"))
        .collect()
}
fn write_json(path: &Path, value: &impl Serialize) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    serde_json::to_writer_pretty(BufWriter::new(File::create(path).unwrap()), value).unwrap();
}
fn write_jsonl(path: &Path, rows: &[TargetRow]) {
    let mut w = BufWriter::new(File::create(path).unwrap());
    for row in rows {
        serde_json::to_writer(&mut w, row).unwrap();
        writeln!(w).unwrap();
    }
    w.flush().unwrap();
}
fn parse_rational(s: &str) -> num_rational::BigRational {
    s.parse().unwrap()
}
fn parse_vectors(rows: &[[String; 4]]) -> Vec<[num_rational::BigRational; 4]> {
    rows.iter()
        .map(|r| std::array::from_fn(|i| parse_rational(&r[i])))
        .collect()
}

fn packet_paths(out_dir: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    (
        out_dir.join("manifest.json"),
        out_dir.join("selection.jsonl"),
        out_dir.join("panel-geometries.jsonl"),
        out_dir.join("full-validation.json"),
    )
}

fn preflight(args: &Args) {
    let (manifest_path, selection_path, panel_path, full_path) = packet_paths(&args.out_dir);
    let manifest: Manifest = read_json(&manifest_path);
    let full: FullValidation = read_json(&full_path);
    let selection: Vec<serde_json::Value> = read_jsonl(&selection_path);
    let panel: Vec<PanelRow> = read_jsonl(&panel_path);
    let mut hashes = BTreeMap::new();
    for (name, p) in [
        ("manifest.json", &manifest_path),
        ("selection.jsonl", &selection_path),
        ("panel-geometries.jsonl", &panel_path),
    ] {
        hashes.insert(name.to_string(), sha256(p));
    }
    let selected = panel
        .iter()
        .filter(|r| r.evaluation_roles == ["selected"])
        .count();
    let baseline = panel
        .iter()
        .filter(|r| r.evaluation_roles == ["baseline"])
        .count();
    let mut checks = BTreeMap::new();
    checks.insert(
        "full_validation_identity".into(),
        sha256(&full_path) == FROZEN_VALIDATION_SHA256
            && full.valid
            && full.checks.values().all(|v| *v),
    );
    checks.insert(
        "frozen_artifact_bytes".into(),
        hashes["manifest.json"] == FROZEN_MANIFEST_SHA256
            && hashes["selection.jsonl"] == FROZEN_SELECTION_SHA256
            && hashes["panel-geometries.jsonl"] == FROZEN_PANEL_SHA256,
    );
    checks.insert(
        "semantic_hashes".into(),
        full.candidate_population_hash
            == "447bc752a968a9d37e219cc820bb433b72ec1d6379cf9b10d24230c5c65ebeea"
            && full.selected_hash == FROZEN_SELECTED_HASH
            && full.baseline_hash == FROZEN_BASELINE_HASH
            && full.panel_hash == FROZEN_PANEL_HASH
            && manifest.selected_hash == FROZEN_SELECTED_HASH
            && manifest.baseline_hash == FROZEN_BASELINE_HASH
            && manifest.panel_hash == FROZEN_PANEL_HASH,
    );
    checks.insert(
        "counts_and_roles".into(),
        selection.len() == 100
            && panel.len() == MAX_ROWS
            && selected == 100
            && baseline == 100
            && panel
                .iter()
                .map(|r| r.candidate_id.as_str())
                .collect::<BTreeSet<_>>()
                .len()
                == MAX_ROWS,
    );
    checks.insert(
        "target_free_input".into(),
        !manifest
            .target_exposure
            .capacity_computed_for_new_population
            && !manifest.target_exposure.sys_computed_for_new_population
            && !manifest
                .target_exposure
                .target_fields_present_in_stage_one_artifacts,
    );
    checks.insert(
        "manifest_counts".into(),
        manifest.counts.accepted_candidates == 10000
            && manifest.counts.selected == 100
            && manifest.counts.baseline == 100
            && manifest.counts.panel_union == 200,
    );
    checks.insert("selection_input_present".into(), selection.len() == 100);
    let valid = checks.values().all(|v| *v);
    let summary = Preflight {
        schema: "sys-datascience.generic-ridge-tail-stage1-target.preflight.v1".to_string(),
        valid,
        target_calls: false,
        full_validation_sha256: sha256(&full_path),
        artifact_sha256: hashes,
        selected_hash: manifest.selected_hash,
        baseline_hash: manifest.baseline_hash,
        panel_hash: manifest.panel_hash,
        panel_rows: panel.len(),
        selected_rows: selected,
        baseline_rows: baseline,
        checks,
    };
    assert!(valid, "target preflight failed: {:?}", summary.checks);
    write_json(&args.out_dir.join("preflight.json"), &summary);
    println!("preflight passed: exactly 200 frozen rows; no target calls");
}

fn evaluate(args: &Args) {
    assert!(args.workers > 0 && args.workers <= 12);
    let started = Instant::now();
    let u0 = usage();
    let (manifest_path, selection_path, panel_path, full_path) = packet_paths(&args.out_dir);
    let preflight_path = args.out_dir.join("preflight.json");
    let pre: Preflight = read_json(&preflight_path);
    assert!(
        pre.valid && !pre.target_calls,
        "run passing target-free preflight first"
    );
    assert_eq!(sha256(&full_path), FROZEN_VALIDATION_SHA256);
    assert_eq!(sha256(&manifest_path), FROZEN_MANIFEST_SHA256);
    assert_eq!(sha256(&selection_path), FROZEN_SELECTION_SHA256);
    assert_eq!(sha256(&panel_path), FROZEN_PANEL_SHA256);
    let _manifest: Manifest = read_json(&manifest_path);
    let panel: Vec<PanelRow> = read_jsonl(&panel_path);
    assert_eq!(
        panel.len(),
        MAX_ROWS,
        "evaluator is capped at exactly 200 rows"
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.workers)
        .build_global()
        .expect("rayon pool");
    let rows = panel.par_iter().map(evaluate_row).collect::<Vec<_>>();
    assert_eq!(rows.len(), MAX_ROWS);
    let mut rows = rows;
    rows.sort_by(|a, b| {
        a.role
            .cmp(&b.role)
            .then(a.f64_rank.cmp(&b.f64_rank))
            .then(a.candidate_id.cmp(&b.candidate_id))
    });
    let selected = rows.iter().filter(|r| r.role == "selected").count();
    let baseline = rows.iter().filter(|r| r.role == "baseline").count();
    assert_eq!((selected, baseline), (100, 100));
    let output = args.out_dir.join("target-rows.jsonl");
    write_jsonl(&output, &rows);
    let u1 = usage();
    let source_hash = sha256(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/main.rs")
            .as_path(),
    );
    let manifest = EvaluationManifest { schema: "sys-datascience.generic-ridge-tail-stage1-target.evaluation.v1".to_string(), status: "evaluated-frozen-200-target-free-panel".to_string(), row_count: rows.len(), selected_count: selected, baseline_count: baseline, threshold: THRESHOLD, volume_definition: "euclidean_polytopes::volume_from_incidence_f64 on exact-derived incidence".to_string(), capacity_route: "exp_sys_landscape::capacity_auto (billiard when Lagrangian-product classification succeeds; otherwise pruned HK2017)".to_string(), preflight_sha256: sha256(&preflight_path), evaluator_source_sha256: source_hash.clone(), target_evaluator_source_sha256: source_hash, manifest_repair_source_sha256: String::new(), target_free_full_validation_sha256: FROZEN_VALIDATION_SHA256.to_string(), target_calls_for_new_population: true, rows_sha256: sha256(&output), rows_blake3: blake3_file(&output), wall_ms: started.elapsed().as_secs_f64()*1000.0, process_user_cpu_seconds: u1.user-u0.user, process_system_cpu_seconds: u1.system-u0.system, max_rss_kib: u1.rss };
    write_json(&args.out_dir.join("evaluation-manifest.json"), &manifest);
    println!(
        "evaluated exactly 200 frozen rows; target rows at {}",
        output.display()
    );
}

/// Regenerate only the provenance manifest from retained target rows. This
/// repair path performs no capacity/sys calls and exists so a provenance-field
/// correction never requires repeating irreversible target evaluations.
fn finalize_manifest(args: &Args) {
    let output = args.out_dir.join("target-rows.jsonl");
    let path = args.out_dir.join("evaluation-manifest.json");
    let mut manifest: EvaluationManifest = read_json(&path);
    manifest.rows_sha256 = sha256(&output);
    manifest.rows_blake3 = blake3_file(&output);
    manifest.evaluator_source_sha256 = TARGET_RUN_SOURCE_SHA256.to_string();
    manifest.target_evaluator_source_sha256 = TARGET_RUN_SOURCE_SHA256.to_string();
    manifest.manifest_repair_source_sha256 = sha256(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/main.rs")
            .as_path(),
    );
    write_json(&path, &manifest);
    println!("finalized evaluation provenance without target calls");
}

fn evaluate_row(row: &PanelRow) -> TargetRow {
    assert_eq!(
        row.schema,
        "sys-datascience.generic-ridge-tail-stage1.panel-geometry.v1"
    );
    assert_eq!(row.facet_count, 10);
    assert_eq!(row.height_min.to_bits(), 0.8f64.to_bits());
    assert_eq!(row.height_max.to_bits(), 1.2f64.to_bits());
    assert_eq!(row.proxy, "ridge_symp_area_mean_over_volume_sqrt");
    assert_eq!(row.evaluation_roles.len(), 1);
    let dual = parse_vectors(&row.dual_vertices_rational);
    let vertices = parse_vectors(&row.vertices_rational);
    let started = Instant::now();
    let poly = SysLandscapePolytopeCache::from_rational_parts(dual, vertices)
        .expect("reconstruct panel geometry");
    assert_eq!(poly_id(&poly), row.poly_id);
    let volume = volume_from_incidence_f64(&poly.vertices_f64, &poly.vertex_facet_incidence)
        .expect("f64 incidence volume");
    let (ridge_count, ridge_mean) = ridge_mean(&poly);
    let proxy = ridge_mean / volume.sqrt();
    assert_eq!(proxy.to_bits(), row.proxy_value_f64.to_bits());
    assert!(volume.is_finite() && volume > 0.0 && proxy.is_finite());
    let reconstruct_ms = started.elapsed().as_secs_f64() * 1000.0;
    let cap_started = Instant::now();
    let result = capacity_auto(
        &poly.dual_vertices_f64,
        &poly.dual_vertices,
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    )
    .expect("automatic/HK2017 capacity route");
    let cap_ms = cap_started.elapsed().as_secs_f64() * 1000.0;
    let capacity = result.min_action;
    let sys = systolic_ratio(capacity, volume);
    assert!(
        sys.is_finite() && capacity.is_finite() && capacity > 0.0,
        "invalid target result"
    );
    let best = result.best_orbit();
    TargetRow {
        candidate_id: row.candidate_id.clone(),
        poly_id: row.poly_id.clone(),
        sample_index: row.sample_index,
        rejection_attempt: row.rejection_attempt,
        role: row.evaluation_roles[0].clone(),
        future_band: row.future_band.clone(),
        f64_rank: row.f64_rank,
        proxy,
        ridge_count,
        ridge_symp_area_mean: ridge_mean,
        f64_volume: volume,
        capacity,
        sys,
        backend: if symplectic::classify_facets_from_dual_vertices(&poly.dual_vertices_f64).is_ok()
        {
            "auto->billiard".into()
        } else {
            "auto->pruned_hk2017".into()
        },
        capacity_iterations: result.iterations,
        returned_orbit_count: result.orbits.len(),
        best_sigma: best.sigma.clone(),
        best_action: best.action,
        best_beta_margin: best.beta_margin,
        best_admissibility: format!("{:?}", best.admissibility),
        time_reconstruct_ms: reconstruct_ms,
        time_capacity_ms: cap_ms,
    }
}

fn ridge_mean(poly: &SysLandscapePolytopeCache) -> (usize, f64) {
    let faces =
        euclidean_polytopes::two_faces_from_vertex_facet_incidence(&poly.vertex_facet_incidence);
    let fields = features_face_symplectic::compute_face_symplectic_fields(
        &faces,
        &poly.vertices_f64,
        &poly.vertex_facet_incidence,
        1.0,
    );
    assert_eq!(fields.ridge_symp_area_ordering_failure_count, 0);
    (faces.len(), fields.ridge_symp_area_mean)
}

struct Args {
    command: String,
    out_dir: PathBuf,
    workers: usize,
}
fn args() -> Args {
    let a: Vec<_> = std::env::args().collect();
    assert!(
        a.len() >= 2,
        "usage: preflight|evaluate --out-dir PATH [--workers N]"
    );
    let mut out_dir = PathBuf::from(
        "experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1",
    );
    let mut workers = 12;
    let mut i = 2;
    while i < a.len() {
        match a[i].as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(&a[i + 1]);
                i += 2;
            }
            "--workers" => {
                workers = a[i + 1].parse().unwrap();
                i += 2;
            }
            x => panic!("unknown flag {x}"),
        }
    }
    Args {
        command: a[1].clone(),
        out_dir,
        workers,
    }
}
fn main() {
    let a = args();
    match a.command.as_str() {
        "preflight" => preflight(&a),
        "evaluate" => evaluate(&a),
        "finalize" => finalize_manifest(&a),
        x => panic!("unknown command {x}"),
    }
}
