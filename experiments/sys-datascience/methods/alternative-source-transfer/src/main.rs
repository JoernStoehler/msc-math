//! Target-free producer and selector for the reviewed factorial-both transfer packet.
//!
//! This binary intentionally has no capacity or `sys` code path.  `produce`
//! writes exact reconstructed product geometry, `features` computes the two
//! frozen feature families from that geometry, and `select` freezes the
//! selected/control union.  Validation is a separate manifest gate.

#[path = "../../../../polytope-invariant-table/features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "../../../../polytope-invariant-table/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::{two_faces_from_vertex_facet_incidence, volume_from_incidence_exact};
use exp_sys_landscape::{rational_vec4_to_strings, SysLandscapePolytopeCache};
use nalgebra::{DMatrix, Matrix4, SymmetricEigen, Vector2, Vector4};
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::geom::polygon::{polygon_area, random_polygon_2d};

const MASTER_SEED: u64 = 2_026_071_601;
const CONTROL_SEED: u64 = 2_026_071_299;
const IDENTITY_SCOPE: &str = "alternative-source-transfer-v1";
const LAW: &str = "factorial-both";
const ROW_TARGET: usize = 3_200;
const ROW_CAP: usize = 4_000;
const ATTEMPT_CAP: usize = 128;
const RHO_FRACTION: f64 = 0.005;
const RIDGE_PRIMARY_FRACTION: f64 = 0.01;
const RIDGE_SECONDARY_FRACTION: f64 = 0.5;
const CONTROL_COUNT: usize = 16;

#[derive(Clone)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SourceRow {
    schema: String,
    candidate_id: String,
    logical_cell: String,
    identity_scope: String,
    law: String,
    law_version: String,
    seed: u64,
    bucket: String,
    k: usize,
    m: usize,
    row_index: usize,
    attempt: usize,
    accepted: bool,
    validation_status: String,
    exact_dual_vertices: Vec<[String; 4]>,
    exact_primal_vertices: Vec<[String; 4]>,
    vertex_facet_incidence: Vec<Vec<bool>>,
    exact_volume: String,
    volume: f64,
    geometry_fingerprint: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FeatureRow {
    schema: String,
    candidate_id: String,
    logical_cell: String,
    identity_scope: String,
    law: String,
    seed: u64,
    bucket: String,
    k: usize,
    m: usize,
    row_index: usize,
    attempt: usize,
    vertex_covariance_status: String,
    vertex_covariance_rho: Option<f64>,
    ridge_symp_area_sum_over_volume_sqrt: f64,
    ridge_symp_area_max_share: f64,
    ridge_symp_area_ordering_failure_count: usize,
    source_geometry_fingerprint: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct SelectionRow {
    candidate_id: String,
    logical_cell: String,
    bucket: String,
    row_index: usize,
    attempt: usize,
    memberships: Vec<String>,
    geometry_fingerprint: String,
}

#[derive(Serialize, Deserialize)]
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
    membership_counts: BTreeMap<String, BTreeMap<String, usize>>,
    unique_target_rows: usize,
    arm_overlap_rows: usize,
    target_free: bool,
    clean_commit: String,
    lock_hash: String,
}

fn bucket_name(k: usize, m: usize) -> String {
    format!("{k}x{m}")
}

fn hash_bytes(path: &Path) -> String {
    let bytes = std::fs::read(path).expect("artifact exists");
    let mut hash = Sha256::new();
    hash.update(bytes);
    format!("{:x}", hash.finalize())
}

fn law_seed(row: usize, attempt: usize, k: usize, m: usize) -> [u8; 32] {
    let mut key = Vec::new();
    key.extend_from_slice(b"factorial-base");
    key.push(0);
    key.extend_from_slice(b"paired-current");
    key.push(0);
    key.extend_from_slice(&MASTER_SEED.to_le_bytes());
    key.extend_from_slice(&(k as u64).to_le_bytes());
    key.extend_from_slice(&(m as u64).to_le_bytes());
    key.extend_from_slice(&(row as u64).to_le_bytes());
    key.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&key).as_bytes()
}

fn active(f: &Factor) -> bool {
    if f.normals.len() != f.heights.len() || f.normals.len() < 3 {
        return false;
    }
    for i in 0..f.normals.len() {
        let j = (i + 1) % f.normals.len();
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / det;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / det;
        if f.normals
            .iter()
            .zip(&f.heights)
            .any(|(n, h)| n[0] * x + n[1] * y > *h + 1e-9)
        {
            return false;
        }
    }
    true
}

fn normalize(mut f: Factor) -> Option<Factor> {
    if !active(&f) {
        return None;
    }
    let area = polygon_area(&f.normals, &f.heights)?;
    if !(area.is_finite() && area > 0.0) {
        return None;
    }
    let s = area.sqrt().recip();
    for h in &mut f.heights {
        *h *= s;
    }
    Some(f)
}

fn latent(k: usize, m: usize, row: usize, attempt: usize) -> Option<(Factor, Factor)> {
    let mut rng = ChaCha8Rng::from_seed(law_seed(row, attempt, k, m));
    let (q_normals, q_heights) = random_polygon_2d(k, 0.8, 1.2, &mut rng);
    let (p_normals, p_heights) = random_polygon_2d(m, 0.8, 1.2, &mut rng);
    let q = Factor {
        normals: q_normals,
        heights: q_heights,
    };
    let p = Factor {
        normals: p_normals,
        heights: p_heights,
    };
    let _q_base = normalize(q.clone())?;
    let _p_base = normalize(p.clone())?;
    let q_t = normalize(Factor {
        normals: q.normals.clone(),
        heights: vec![1.0; k],
    })?;
    let p_t = normalize(Factor {
        normals: p.normals.clone(),
        heights: vec![1.0; m],
    })?;
    Some((q_t, p_t))
}

fn exact_fingerprint(row: &SourceRow) -> String {
    let mut bytes = Vec::new();
    for v in &row.exact_dual_vertices {
        for c in v {
            bytes.extend_from_slice(c.as_bytes());
            bytes.push(0);
        }
    }
    for v in &row.exact_primal_vertices {
        for c in v {
            bytes.extend_from_slice(c.as_bytes());
            bytes.push(0);
        }
    }
    for r in &row.vertex_facet_incidence {
        for b in r {
            bytes.push(*b as u8);
        }
    }
    blake3::hash(&bytes).to_hex().to_string()
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let mut w = BufWriter::new(File::create(path).unwrap());
    for r in rows {
        serde_json::to_writer(&mut w, r).unwrap();
        w.write_all(b"\n").unwrap();
    }
}
fn read_jsonl<T: for<'a> Deserialize<'a>>(path: &Path) -> Vec<T> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
        .collect()
}

fn rational_f64(value: &str) -> f64 {
    if let Some((numerator, denominator)) = value.split_once('/') {
        numerator.parse::<f64>().unwrap() / denominator.parse::<f64>().unwrap()
    } else {
        value.parse::<f64>().unwrap()
    }
}

fn source_row(k: usize, m: usize, row: usize) -> Option<SourceRow> {
    for attempt in 0..ATTEMPT_CAP {
        let Some((q, p)) = latent(k, m, row, attempt) else {
            continue;
        };
        let Some(poly) = SysLandscapePolytopeCache::from_lagrangian_product(
            &q.normals, &q.heights, &p.normals, &p.heights,
        ) else {
            continue;
        };
        let exact_dual = rational_vec4_to_strings(&poly.dual_vertices);
        let exact_primal = rational_vec4_to_strings(&poly.vertices);
        let incidence = (0..poly.vertex_facet_incidence.nrows())
            .map(|i| {
                (0..poly.vertex_facet_incidence.ncols())
                    .map(|j| poly.vertex_facet_incidence[(i, j)])
                    .collect()
            })
            .collect();
        let exact_vertices: Vec<Vector4<_>> = poly
            .vertices
            .iter()
            .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
            .collect();
        let exact_volume =
            volume_from_incidence_exact(&exact_vertices, &poly.vertex_facet_incidence);
        let vol = exact_volume
            .to_f64()
            .expect("exact source volume must be representable as f64");
        let mut sr = SourceRow {
            schema: "alternative-source-transfer-source-v1".into(),
            candidate_id: format!(
                "{IDENTITY_SCOPE}/{LAW}/seed={MASTER_SEED}/row={row}/attempt={attempt}/{k}x{m}"
            ),
            logical_cell: format!(
                "seed={MASTER_SEED}/bucket={k}x{m}/row={row}/attempt={attempt}/law={LAW}"
            ),
            identity_scope: IDENTITY_SCOPE.into(),
            law: LAW.into(),
            law_version: "wishlist-2026-07-14-v2".into(),
            seed: MASTER_SEED,
            bucket: bucket_name(k, m),
            k,
            m,
            row_index: row,
            attempt,
            accepted: true,
            validation_status: "eligible".into(),
            exact_dual_vertices: exact_dual,
            exact_primal_vertices: exact_primal,
            vertex_facet_incidence: incidence,
            exact_volume: exact_volume.to_string(),
            volume: vol,
            geometry_fingerprint: String::new(),
        };
        sr.geometry_fingerprint = exact_fingerprint(&sr);
        return Some(sr);
    }
    None
}

fn produce(out: &Path) {
    create_dir_all(out).unwrap();
    let path = out.join("source.jsonl");
    let mut w = BufWriter::new(File::create(&path).unwrap());
    let mut counts = BTreeMap::new();
    let started = Instant::now();
    for (k, m) in [(4usize, 6usize), (6, 6)] {
        let mut rows: Vec<SourceRow> = (0..ROW_CAP)
            .into_par_iter()
            .filter_map(|row| source_row(k, m, row))
            .collect();
        rows.sort_by_key(|r| r.row_index);
        rows.truncate(ROW_TARGET);
        let accepted = rows.len();
        if accepted < ROW_TARGET {
            panic!("incomplete source bucket {k}x{m}: {accepted}/{ROW_TARGET}");
        }
        for row in &rows {
            serde_json::to_writer(&mut w, row).unwrap();
            w.write_all(b"\n").unwrap();
        }
        counts.insert(bucket_name(k, m), accepted);
    }
    w.flush().unwrap();
    let report = serde_json::json!({"schema":"alternative-source-transfer-production-v1","counts":counts,"elapsed_seconds":started.elapsed().as_secs_f64(),"target_free":true,"source_sha256":hash_bytes(&path)});
    std::fs::write(
        out.join("production-report.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
}

#[derive(Clone)]
struct Cov {
    rho: Option<f64>,
    status: String,
}
fn cov(vertices: &[Vector4<f64>], expected: usize) -> Cov {
    let mut map = BTreeMap::<[u64; 4], Vector4<f64>>::new();
    for v in vertices {
        map.entry(std::array::from_fn(|i| {
            if v[i] == 0.0 {
                0
            } else {
                v[i].to_bits()
            }
        }))
        .or_insert(*v);
    }
    let vs: Vec<_> = map.into_values().collect();
    if vs.len() != expected {
        return Cov {
            rho: None,
            status: "unexpected_distinct_vertex_count".into(),
        };
    }
    let mean = vs.iter().fold(Vector4::zeros(), |s, v| s + v) / (vs.len() as f64);
    let c = vs.iter().fold(Matrix4::zeros(), |s, v| {
        let d = v - mean;
        s + d * d.transpose()
    }) / (vs.len() as f64);
    let eig = SymmetricEigen::new(c);
    let lo = eig
        .eigenvalues
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let hi = eig
        .eigenvalues
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if !(lo > 0.0 && hi / lo <= 1e10) {
        return Cov {
            rho: None,
            status: "covariance_ineligible".into(),
        };
    }
    let j = Matrix4::new(
        0., 0., 1., 0., 0., 0., 0., 1., -1., 0., 0., 0., 0., -1., 0., 0.,
    );
    let jc = j * c;
    let s = -0.5 * (jc * jc).trace();
    let p = c.determinant();
    let d = (s * s - 4. * p).max(0.).sqrt();
    let n2 = 0.5 * (s + d);
    let n1 = p / n2;
    if !(n1 > 0. && n2 >= n1) {
        return Cov {
            rho: None,
            status: "williamson_ineligible".into(),
        };
    }
    Cov {
        rho: Some((n2 / n1).sqrt()),
        status: "eligible".into(),
    }
}

fn features(out: &Path) {
    let source: Vec<SourceRow> = read_jsonl(&out.join("source.jsonl"));
    let mut rows = Vec::new();
    for s in &source {
        let incidence = DMatrix::from_fn(
            s.vertex_facet_incidence.len(),
            s.vertex_facet_incidence[0].len(),
            |i, j| s.vertex_facet_incidence[i][j],
        );
        let vertices: Vec<Vector4<f64>> = s
            .exact_primal_vertices
            .iter()
            .map(|v| {
                Vector4::new(
                    rational_f64(&v[0]),
                    rational_f64(&v[1]),
                    rational_f64(&v[2]),
                    rational_f64(&v[3]),
                )
            })
            .collect();
        let faces = two_faces_from_vertex_facet_incidence(&incidence);
        let f = features_face_symplectic::compute_face_symplectic_fields(
            &faces,
            &vertices,
            &incidence,
            s.volume.sqrt(),
        );
        let c = cov(&vertices, s.k * s.m);
        rows.push(FeatureRow {
            schema: "alternative-source-transfer-feature-v1".into(),
            candidate_id: s.candidate_id.clone(),
            logical_cell: s.logical_cell.clone(),
            identity_scope: s.identity_scope.clone(),
            law: s.law.clone(),
            seed: s.seed,
            bucket: s.bucket.clone(),
            k: s.k,
            m: s.m,
            row_index: s.row_index,
            attempt: s.attempt,
            vertex_covariance_status: c.status,
            vertex_covariance_rho: c.rho,
            ridge_symp_area_sum_over_volume_sqrt: f.ridge_symp_area_sum / s.volume.sqrt(),
            ridge_symp_area_max_share: f.ridge_symp_area_max_share,
            ridge_symp_area_ordering_failure_count: f.ridge_symp_area_ordering_failure_count,
            source_geometry_fingerprint: s.geometry_fingerprint.clone(),
        });
    }
    write_jsonl(&out.join("features.jsonl"), &rows);
    std::fs::write(out.join("feature-report.json"),serde_json::to_vec_pretty(&serde_json::json!({"schema":"alternative-source-transfer-feature-report-v1","count":rows.len(),"feature_sha256":hash_bytes(&out.join("features.jsonl")),"target_free":true})).unwrap()).unwrap();
}

fn ceil_min(frac: f64, n: usize) -> usize {
    ((frac * n as f64).ceil() as usize).max(1)
}
fn select(out: &Path) {
    let source: Vec<SourceRow> = read_jsonl(&out.join("source.jsonl"));
    let features: Vec<FeatureRow> = read_jsonl(&out.join("features.jsonl"));
    assert_eq!(source.len(), features.len());
    let mut selected: Vec<SelectionRow> = Vec::new();
    let mut membership_counts: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for bucket in ["4x6", "6x6"] {
        let mut ix: Vec<usize> = (0..features.len())
            .filter(|i| {
                features[*i].bucket == bucket && features[*i].vertex_covariance_status == "eligible"
            })
            .collect();
        assert_eq!(ix.len(), ROW_TARGET);
        ix.sort_by(|a, b| {
            features[*a]
                .vertex_covariance_rho
                .partial_cmp(&features[*b].vertex_covariance_rho)
                .unwrap()
                .then_with(|| features[*a].candidate_id.cmp(&features[*b].candidate_id))
        });
        let rho = &ix[..ceil_min(RHO_FRACTION, ix.len())];
        let mut ridge = ix.to_vec();
        ridge.sort_by(|a, b| {
            features[*a]
                .ridge_symp_area_sum_over_volume_sqrt
                .partial_cmp(&features[*b].ridge_symp_area_sum_over_volume_sqrt)
                .unwrap()
                .then_with(|| features[*a].candidate_id.cmp(&features[*b].candidate_id))
        });
        ridge.truncate(ceil_min(RIDGE_PRIMARY_FRACTION, ix.len()));
        ridge.sort_by(|a, b| {
            features[*a]
                .ridge_symp_area_max_share
                .partial_cmp(&features[*b].ridge_symp_area_max_share)
                .unwrap()
                .then_with(|| features[*a].candidate_id.cmp(&features[*b].candidate_id))
        });
        ridge.truncate(ceil_min(
            RIDGE_SECONDARY_FRACTION,
            ceil_min(RIDGE_PRIMARY_FRACTION, ix.len()),
        ));
        let arm_union: BTreeSet<usize> = rho.iter().chain(&ridge).copied().collect();
        let mut rest: Vec<usize> = ix
            .iter()
            .copied()
            .filter(|i| !arm_union.contains(i))
            .collect();
        rest.sort_by(|a, b| {
            let mut ha = blake3::Hasher::new();
            ha.update(b"frozen-canonical-vertex-covariance-control-v1");
            ha.update(&CONTROL_SEED.to_le_bytes());
            ha.update(features[*a].candidate_id.as_bytes());
            let mut hb = blake3::Hasher::new();
            hb.update(b"frozen-canonical-vertex-covariance-control-v1");
            hb.update(&CONTROL_SEED.to_le_bytes());
            hb.update(features[*b].candidate_id.as_bytes());
            ha.finalize()
                .as_bytes()
                .cmp(hb.finalize().as_bytes())
                .then_with(|| features[*a].candidate_id.cmp(&features[*b].candidate_id))
        });
        let control = &rest[..CONTROL_COUNT];
        let mut add = |i: usize, name: &str| {
            if let Some(r) = selected
                .iter_mut()
                .find(|r| r.candidate_id == source[i].candidate_id)
            {
                r.memberships.push(name.into());
            } else {
                selected.push(SelectionRow {
                    candidate_id: source[i].candidate_id.clone(),
                    logical_cell: source[i].logical_cell.clone(),
                    bucket: bucket.into(),
                    row_index: source[i].row_index,
                    attempt: source[i].attempt,
                    memberships: vec![name.into()],
                    geometry_fingerprint: source[i].geometry_fingerprint.clone(),
                });
            }
        };
        for i in rho {
            add(*i, "rho");
        }
        for i in &ridge {
            add(*i, "ridge");
        }
        for i in control {
            add(*i, "control");
        }
        let mut c = BTreeMap::new();
        c.insert("rho".into(), rho.len());
        c.insert("ridge".into(), ridge.len());
        c.insert("control".into(), control.len());
        membership_counts.insert(bucket.into(), c);
    }
    write_jsonl(&out.join("selection.jsonl"), &selected);
    let report = serde_json::json!({"schema":"alternative-source-transfer-selection-v1","membership_counts":membership_counts,"selection_count":selected.len(),"target_free":true,"source_sha256":hash_bytes(&out.join("source.jsonl")),"feature_sha256":hash_bytes(&out.join("features.jsonl")),"selection_sha256":hash_bytes(&out.join("selection.jsonl"))});
    std::fs::write(
        out.join("selection-report.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
}

fn validate(out: &Path) {
    let source: Vec<SourceRow> = read_jsonl(&out.join("source.jsonl"));
    let feature: Vec<FeatureRow> = read_jsonl(&out.join("features.jsonl"));
    let selection: Vec<SelectionRow> = read_jsonl(&out.join("selection.jsonl"));
    assert_eq!(source.len(), 6400);
    assert_eq!(feature.len(), 6400);
    assert!(selection.len() <= 96);
    let mut ids = BTreeSet::new();
    let mut cells = BTreeSet::new();
    let mut fps = BTreeSet::new();
    for s in &source {
        assert!(ids.insert(s.candidate_id.clone()));
        assert!(cells.insert(s.logical_cell.clone()));
    }
    for r in &selection {
        assert!(fps.insert(r.geometry_fingerprint.clone()));
    }
    let mut m = BTreeMap::new();
    for b in ["4x6", "6x6"] {
        let rows = selection.iter().filter(|r| r.bucket == b);
        let mut c = BTreeMap::new();
        for arm in ["rho", "ridge", "control"] {
            c.insert(
                arm.into(),
                rows.clone()
                    .filter(|r| r.memberships.iter().any(|x| x == arm))
                    .count(),
            );
        }
        assert!(c.values().all(|x| *x == 16));
        m.insert(b.into(), c);
    }
    let overlap = selection.iter().filter(|r| r.memberships.len() > 1).count();
    let manifest = Manifest {
        schema: "alternative-source-transfer-manifest-v1".into(),
        identity_scope: IDENTITY_SCOPE.into(),
        master_seed: MASTER_SEED,
        control_seed: CONTROL_SEED,
        law: LAW.into(),
        buckets: vec!["4x6".into(), "6x6".into()],
        row_target_per_bucket: ROW_TARGET,
        row_cap_per_bucket: ROW_CAP,
        attempt_cap: ATTEMPT_CAP,
        source_sha256: hash_bytes(&out.join("source.jsonl")),
        feature_sha256: hash_bytes(&out.join("features.jsonl")),
        selection_sha256: hash_bytes(&out.join("selection.jsonl")),
        source_count: source.len(),
        feature_count: feature.len(),
        selection_count: selection.len(),
        membership_counts: m,
        unique_target_rows: selection.len(),
        arm_overlap_rows: overlap,
        target_free: true,
        clean_commit: std::env::var("GIT_COMMIT").unwrap_or_else(|_| "unrecorded-precommit".into()),
        lock_hash: hash_bytes(Path::new("Cargo.lock")),
    };
    std::fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
}

fn main() {
    let mut a = std::env::args().skip(1);
    let cmd = a.next().unwrap_or_else(|| "help".into());
    let out = PathBuf::from(a.next().unwrap_or_else(|| "artifacts/transfer-v1".into()));
    match cmd.as_str() {
        "produce" => produce(&out),
        "features" => features(&out),
        "select" => select(&out),
        "validate" => validate(&out),
        "help" => eprintln!(
            "usage: alternative-source-transfer <produce|features|select|validate> OUT_DIR"
        ),
        other => panic!("unknown command {other}"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reviewed_owner_seed(row: usize, attempt: usize, k: usize, m: usize) -> [u8; 32] {
        let mut key = Vec::new();
        key.extend_from_slice(&MASTER_SEED.to_le_bytes());
        key.extend_from_slice(b"factorial-base");
        key.push(0);
        key.extend_from_slice(b"paired-current");
        key.push(0);
        key.extend_from_slice(&(k as u64).to_le_bytes());
        key.extend_from_slice(&(m as u64).to_le_bytes());
        key.extend_from_slice(&(row as u64).to_le_bytes());
        key.extend_from_slice(&(attempt as u64).to_le_bytes());
        *blake3::hash(&key).as_bytes()
    }

    #[test]
    fn seed_translation_is_explicit_and_law_semantics_survive() {
        assert_ne!(law_seed(0, 0, 4, 6), reviewed_owner_seed(0, 0, 4, 6));
        let (q, p) = (0..ATTEMPT_CAP)
            .find_map(|attempt| latent(4, 6, 0, attempt))
            .expect("reviewed admissible source draw");
        assert!(active(&q) && active(&p));
        assert!(q.heights.iter().all(|height| *height > 0.0));
        assert!(p.heights.iter().all(|height| *height > 0.0));
    }
}
