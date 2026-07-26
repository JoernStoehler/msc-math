//! Shared computed-polytope cache for datascience producers.
//!
//! The cache is keyed by canonical f64 dual-vertex bits. Producer metadata should
//! point at `poly_id`; this row owns the expensive capacity/minimizer payload.

use crate::SysLandscapePolytopeCache;
use euclidean_polytopes::volume_from_incidence_f64;
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use symplectic::capacity_4d::{
    check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
    check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, qp_minimizers,
    PolytopeGeometry4d, QpCandidateFamily4d,
};
use symplectic::database::{OrbitScalars, SigmaAction};

const CURRENT_VOLUME_METHOD: &str = "f64-from-exact-derived-incidence-v1";
const LEGACY_VOLUME_METHOD: &str = "exact-rational-rounded-f64-v1";
const CURRENT_CAPACITY_METHOD: &str = "certified-qp-minimizers-v1";
const LEGACY_CAPACITY_METHOD: &str = "legacy-orbit-search-v1";
const MAXIMUM_CAPACITY_RELATIVE_ERROR: f64 = 1e-10;
const DERIVED_VALUE_RELATIVE_TOLERANCE: f64 = 1e-12;

fn legacy_volume_method() -> String {
    LEGACY_VOLUME_METHOD.to_string()
}

fn legacy_capacity_method() -> String {
    LEGACY_CAPACITY_METHOD.to_string()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapacityBackend {
    Auto,
    Product,
}

impl CapacityBackend {
    fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Product => "product",
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedPolytopePayloadRow {
    pub poly_id: String,
    pub dual_vertices: Vec<[f64; 4]>,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub backend: String,
    #[serde(default = "legacy_capacity_method")]
    pub capacity_method: String,
    #[serde(default = "legacy_volume_method")]
    pub volume_method: String,
    pub volume: f64,
    pub capacity: f64,
    #[serde(default)]
    pub capacity_lower: Option<f64>,
    #[serde(default)]
    pub capacity_upper: Option<f64>,
    #[serde(default)]
    pub capacity_exact: Option<String>,
    #[serde(default)]
    pub candidate_family: Option<String>,
    pub sys: f64,
    pub sigma_gap_cutoff: f64,
    pub sigmas: Vec<SigmaAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_scalars: Option<OrbitScalars>,
    pub time_volume_ms: f64,
    pub time_capacity_ms: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComputeCacheStats {
    pub hits: usize,
    pub misses: usize,
    pub miss_volume_ms: f64,
    pub miss_capacity_ms: f64,
}

#[derive(Default)]
pub struct ComputedPolytopeCache {
    rows: HashMap<String, ComputedPolytopePayloadRow>,
    used_rows: Mutex<HashMap<String, ComputedPolytopePayloadRow>>,
    in_flight: Mutex<HashMap<String, Arc<Mutex<()>>>>,
    stats: Mutex<ComputeCacheStats>,
    wal: Option<Mutex<BufWriter<File>>>,
}

impl ComputedPolytopeCache {
    pub fn load(paths: &[PathBuf]) -> Self {
        Self::load_with_wal(paths, None)
    }

    pub fn load_with_wal(paths: &[PathBuf], wal_path: Option<PathBuf>) -> Self {
        let mut rows = HashMap::new();
        for path in paths {
            load_payload_rows(path, &mut rows);
        }
        let wal = wal_path.map(|path| {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                        panic!(
                            "create computed-polytope WAL parent {}: {e}",
                            parent.display()
                        )
                    });
                }
            }
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .unwrap_or_else(|e| panic!("open computed-polytope WAL {}: {e}", path.display()));
            Mutex::new(BufWriter::new(file))
        });
        Self {
            rows,
            used_rows: Mutex::new(HashMap::new()),
            in_flight: Mutex::new(HashMap::new()),
            stats: Mutex::new(ComputeCacheStats::default()),
            wal,
        }
    }

    pub fn compute(
        &self,
        polytope: &SysLandscapePolytopeCache,
        backend: CapacityBackend,
    ) -> Option<ComputedPolytopePayloadRow> {
        let poly_id = poly_id(polytope);
        if let Some(row) = self
            .rows
            .get(&poly_id)
            .filter(|row| row.capacity_method == CURRENT_CAPACITY_METHOD)
        {
            self.record_hit(&poly_id, row.clone());
            return Some(row.clone());
        }
        if let Some(row) = self
            .used_rows
            .lock()
            .expect("used cache mutex poisoned")
            .get(&poly_id)
            .cloned()
        {
            self.record_used_hit(row.clone());
            return Some(row);
        }

        let key_lock = {
            let mut in_flight = self
                .in_flight
                .lock()
                .expect("in-flight cache mutex poisoned");
            // Keep per-key locks for the run; pruning them while waiters exist
            // can split one key across two locks after a failed computation.
            Arc::clone(
                in_flight
                    .entry(poly_id.clone())
                    .or_insert_with(|| Arc::new(Mutex::new(()))),
            )
        };
        let _key_guard = key_lock.lock().expect("in-flight key mutex poisoned");

        if let Some(row) = self
            .used_rows
            .lock()
            .expect("used cache mutex poisoned")
            .get(&poly_id)
            .cloned()
        {
            self.record_used_hit(row.clone());
            return Some(row);
        }

        let start_volume = Instant::now();
        let volume =
            volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                .ok()?;
        let time_volume_ms = start_volume.elapsed().as_secs_f64() * 1000.0;
        if volume <= 0.0 {
            return None;
        }

        let geometry = checked_capacity_geometry(polytope)?;
        let start_capacity = Instant::now();
        let minimizers = qp_minimizers(&geometry).ok()?;
        if backend == CapacityBackend::Product
            && minimizers.family() != QpCandidateFamily4d::ProductClosureVertex
        {
            return None;
        }
        let time_capacity_ms = start_capacity.elapsed().as_secs_f64() * 1000.0;

        let bounds = minimizers.bounds();
        let capacity = bounds.value(MAXIMUM_CAPACITY_RELATIVE_ERROR).ok()?;
        let exact_capacity = minimizers.candidates().first()?.action_exact().clone();
        assert!(minimizers
            .candidates()
            .iter()
            .all(|candidate| candidate.action_exact() == &exact_capacity));
        let sys = symplectic::systolic_ratio(capacity, volume);
        if !sys.is_finite() {
            return None;
        }
        if let Some(legacy) = self
            .rows
            .get(&poly_id)
            .filter(|row| row.capacity_method != CURRENT_CAPACITY_METHOD)
        {
            assert!(
                capacity_within_bounds_with_tolerance(
                    legacy.capacity,
                    bounds.lower(),
                    bounds.upper()
                ),
                "legacy capacity {} lies outside current certified bounds [{}, {}] for poly_id {}",
                legacy.capacity,
                bounds.lower(),
                bounds.upper(),
                poly_id
            );
        }

        let row = ComputedPolytopePayloadRow {
            poly_id: poly_id.clone(),
            dual_vertices: f64_dual_vertices(polytope),
            dual_vertices_rational: rational_vec4_to_strings(&polytope.dual_vertices),
            vertices_rational: rational_vec4_to_strings(&polytope.vertices),
            facet_count: polytope.facet_count(),
            backend: backend.name().to_string(),
            capacity_method: CURRENT_CAPACITY_METHOD.to_string(),
            volume_method: CURRENT_VOLUME_METHOD.to_string(),
            volume,
            capacity,
            capacity_lower: Some(bounds.lower()),
            capacity_upper: Some(bounds.upper()),
            capacity_exact: Some(rational_text(&exact_capacity)),
            candidate_family: Some(candidate_family_name(minimizers.family()).to_string()),
            sys,
            sigma_gap_cutoff: 0.0,
            sigmas: minimizers
                .candidates()
                .iter()
                .map(|candidate| SigmaAction {
                    perm: candidate.sigma().to_vec(),
                    action: candidate
                        .action_exact()
                        .to_f64()
                        .expect("production capacity must fit in f64"),
                })
                .collect(),
            orbit_scalars: None,
            time_volume_ms,
            time_capacity_ms,
        };

        {
            let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
            if !used.contains_key(&poly_id) {
                self.append_wal(&row);
                used.insert(poly_id.clone(), row.clone());
            }
        }
        {
            let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
            stats.misses += 1;
            stats.miss_volume_ms += time_volume_ms;
            stats.miss_capacity_ms += time_capacity_ms;
        }
        Some(row)
    }

    pub fn used_rows(&self) -> Vec<ComputedPolytopePayloadRow> {
        let used = self.used_rows.lock().expect("used cache mutex poisoned");
        let mut rows: Vec<_> = used.values().cloned().collect();
        rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
        rows
    }

    pub fn stats(&self) -> ComputeCacheStats {
        *self.stats.lock().expect("cache stats mutex poisoned")
    }

    fn record_hit(&self, poly_id: &str, row: ComputedPolytopePayloadRow) {
        {
            let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
            used.entry(poly_id.to_string()).or_insert(row);
        }
        self.record_used_hit_count();
    }

    fn record_used_hit(&self, row: ComputedPolytopePayloadRow) {
        let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
        used.entry(row.poly_id.clone()).or_insert(row);
        drop(used);
        self.record_used_hit_count();
    }

    fn record_used_hit_count(&self) {
        let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
        stats.hits += 1;
    }

    fn append_wal(&self, row: &ComputedPolytopePayloadRow) {
        let Some(wal) = &self.wal else {
            return;
        };
        let mut writer = wal.lock().expect("computed-polytope WAL mutex poisoned");
        serde_json::to_writer(&mut *writer, row).expect("write computed-polytope WAL JSON");
        writeln!(&mut *writer).expect("write computed-polytope WAL newline");
        writer.flush().expect("flush computed-polytope WAL");
    }
}

/// Reuse already validated exact geometry when this cache describes the exact
/// dyadic values of its binary64 dual vertices. Source-rational fixtures take
/// the ordinary exact-binary64 reconstruction path instead.
fn checked_capacity_geometry(polytope: &SysLandscapePolytopeCache) -> Option<PolytopeGeometry4d> {
    check_facet_count(polytope.facet_count()).ok()?;
    check_finite_dual_vertices(&polytope.dual_vertices_f64).ok()?;
    check_dual_vertex_norm_bounds(&polytope.dual_vertices_f64).ok()?;

    let dual_vertices_exact = polytope
        .dual_vertices_f64
        .iter()
        .map(|vertex| {
            Some(Vector4::new(
                BigRational::from_float(vertex[0])?,
                BigRational::from_float(vertex[1])?,
                BigRational::from_float(vertex[2])?,
                BigRational::from_float(vertex[3])?,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    let cached_dual_vertices_exact = polytope
        .dual_vertices
        .iter()
        .map(|vertex| {
            Vector4::new(
                vertex[0].clone(),
                vertex[1].clone(),
                vertex[2].clone(),
                vertex[3].clone(),
            )
        })
        .collect::<Vec<_>>();

    let geometry = if dual_vertices_exact == cached_dual_vertices_exact {
        PolytopeGeometry4d {
            dual_vertices: polytope.dual_vertices_f64.clone(),
            dual_vertices_exact,
            primal_vertices_exact: polytope
                .vertices
                .iter()
                .map(|vertex| {
                    Vector4::new(
                        vertex[0].clone(),
                        vertex[1].clone(),
                        vertex[2].clone(),
                        vertex[3].clone(),
                    )
                })
                .collect(),
            vertex_facet_incidence: polytope.vertex_facet_incidence.clone(),
        }
    } else {
        exact_binary64_polytope_geometry(&polytope.dual_vertices_f64).ok()?
    };
    check_primal_vertex_norm_bounds(&geometry).ok()?;
    Some(geometry)
}

fn candidate_family_name(family: QpCandidateFamily4d) -> &'static str {
    match family {
        QpCandidateFamily4d::GeneralHk => "general-hk",
        QpCandidateFamily4d::ProductClosureVertex => "product-closure-vertex",
    }
}

fn rational_text(value: &BigRational) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

pub fn poly_id(polytope: &SysLandscapePolytopeCache) -> String {
    poly_id_from_dual_vertices(&polytope.dual_vertices_f64)
}

pub fn poly_id_from_dual_vertices(dual_vertices: &[nalgebra::Vector4<f64>]) -> String {
    let mut hasher = blake3::Hasher::new();
    for vertex in dual_vertices {
        for coord in vertex.iter() {
            let normalized = if *coord == 0.0 { 0.0 } else { *coord };
            assert!(
                normalized.is_finite(),
                "poly_id requires finite f64 dual vertices"
            );
            hasher.update(&normalized.to_bits().to_le_bytes());
        }
    }
    hasher.finalize().to_hex().to_string()
}

pub fn f64_dual_vertices(polytope: &SysLandscapePolytopeCache) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

pub fn rational_vec4_to_strings(data: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
        .collect()
}

fn load_payload_rows(path: &Path, rows: &mut HashMap<String, ComputedPolytopePayloadRow>) {
    let Ok(file) = File::open(path) else {
        return;
    };
    let reader = BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| {
            panic!(
                "failed to read computed-polytope cache {:?}:{}: {e}",
                path,
                line_number + 1
            )
        });
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row = serde_json::from_str::<ComputedPolytopePayloadRow>(line).unwrap_or_else(|e| {
            panic!(
                "invalid computed-polytope cache JSON {:?}:{}: {e}",
                path,
                line_number + 1
            )
        });
        assert!(
            row.capacity_method != CURRENT_CAPACITY_METHOD
                || current_capacity_payload_is_valid(&row),
            "incomplete current capacity payload for poly_id {:?} in {:?}:{}",
            row.poly_id,
            path,
            line_number + 1
        );
        if let Some(previous) = rows.get(&row.poly_id) {
            if previous.capacity_method == row.capacity_method {
                assert!(
                    semantic_payload_eq(previous, &row),
                    "conflicting computed-polytope cache row for poly_id {:?} in {:?}:{}",
                    row.poly_id,
                    path,
                    line_number + 1
                );
                if previous.volume_method == CURRENT_VOLUME_METHOD
                    && row.volume_method != CURRENT_VOLUME_METHOD
                {
                    continue;
                }
            } else {
                let (current, legacy) = if previous.capacity_method == CURRENT_CAPACITY_METHOD {
                    (previous, &row)
                } else if row.capacity_method == CURRENT_CAPACITY_METHOD {
                    (&row, previous)
                } else {
                    panic!(
                        "unknown competing capacity methods {:?} and {:?} for poly_id {:?}",
                        previous.capacity_method, row.capacity_method, row.poly_id
                    );
                };
                assert!(
                    geometry_payload_eq(current, legacy)
                        && approximately_equal_derived_value(current.volume, legacy.volume)
                        && current_capacity_contains_legacy(current, legacy.capacity),
                    "legacy/current capacity payload disagreement for poly_id {:?} in {:?}:{}",
                    row.poly_id,
                    path,
                    line_number + 1
                );
                if previous.capacity_method == CURRENT_CAPACITY_METHOD {
                    continue;
                }
            }
        }
        rows.insert(row.poly_id.clone(), row);
    }
}

fn approximately_equal_derived_value(a: f64, b: f64) -> bool {
    a == b
        || ((a - b).abs()
            <= DERIVED_VALUE_RELATIVE_TOLERANCE * a.abs().max(b.abs()).max(f64::MIN_POSITIVE))
}

fn capacity_within_bounds_with_tolerance(value: f64, lower: f64, upper: f64) -> bool {
    let margin = MAXIMUM_CAPACITY_RELATIVE_ERROR
        * value
            .abs()
            .max(lower.abs())
            .max(upper.abs())
            .max(f64::MIN_POSITIVE);
    value >= lower - margin && value <= upper + margin
}

fn current_capacity_contains_legacy(
    current: &ComputedPolytopePayloadRow,
    legacy_capacity: f64,
) -> bool {
    match (current.capacity_lower, current.capacity_upper) {
        (Some(lower), Some(upper)) => {
            capacity_within_bounds_with_tolerance(legacy_capacity, lower, upper)
        }
        _ => false,
    }
}

fn current_capacity_payload_is_valid(row: &ComputedPolytopePayloadRow) -> bool {
    let bounds_are_valid = match (row.capacity_lower, row.capacity_upper) {
        (Some(lower), Some(upper)) => {
            lower.is_finite()
                && upper.is_finite()
                && row.capacity.is_finite()
                && 0.0 < lower
                && lower <= row.capacity
                && row.capacity <= upper
        }
        _ => false,
    };
    bounds_are_valid
        && row
            .capacity_exact
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        && matches!(
            row.candidate_family.as_deref(),
            Some("general-hk" | "product-closure-vertex")
        )
        && !row.sigmas.is_empty()
        && row
            .sigmas
            .iter()
            .all(|sigma| sigma.action.is_finite() && sigma.action > 0.0)
}

fn geometry_payload_eq(a: &ComputedPolytopePayloadRow, b: &ComputedPolytopePayloadRow) -> bool {
    a.poly_id == b.poly_id
        && a.dual_vertices == b.dual_vertices
        && a.dual_vertices_rational == b.dual_vertices_rational
        && a.vertices_rational == b.vertices_rational
        && a.facet_count == b.facet_count
}

fn semantic_payload_eq(a: &ComputedPolytopePayloadRow, b: &ComputedPolytopePayloadRow) -> bool {
    geometry_payload_eq(a, b)
        && a.capacity_method == b.capacity_method
        && approximately_equal_derived_value(a.volume, b.volume)
        && a.capacity == b.capacity
        && a.capacity_lower == b.capacity_lower
        && a.capacity_upper == b.capacity_upper
        && a.capacity_exact == b.capacity_exact
        && a.candidate_family == b.candidate_family
        && approximately_equal_derived_value(a.sys, b.sys)
        && a.sigma_gap_cutoff == b.sigma_gap_cutoff
        && a.sigmas == b.sigmas
        && a.orbit_scalars == b.orbit_scalars
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector4;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use symplectic::geom::polygon::regular_polygon_2d;

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn poly_id_normalizes_signed_zero() {
        let a = vec![Vector4::new(0.0, -0.0, 1.0, -2.0)];
        let b = vec![Vector4::new(-0.0, 0.0, 1.0, -2.0)];
        assert_eq!(
            poly_id_from_dual_vertices(&a),
            poly_id_from_dual_vertices(&b)
        );
    }

    #[test]
    fn poly_id_depends_on_facet_order() {
        let a = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
        ];
        let b = vec![
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, 0.0, 0.0),
        ];
        assert_ne!(
            poly_id_from_dual_vertices(&a),
            poly_id_from_dual_vertices(&b)
        );
    }

    fn test_payload() -> ComputedPolytopePayloadRow {
        ComputedPolytopePayloadRow {
            poly_id: "poly-a".to_string(),
            dual_vertices: vec![[1.0, 0.0, 0.0, 0.0]],
            dual_vertices_rational: vec![[
                "1/1".to_string(),
                "0/1".to_string(),
                "0/1".to_string(),
                "0/1".to_string(),
            ]],
            vertices_rational: vec![[
                "1/1".to_string(),
                "0/1".to_string(),
                "0/1".to_string(),
                "0/1".to_string(),
            ]],
            facet_count: 1,
            backend: "auto".to_string(),
            capacity_method: CURRENT_CAPACITY_METHOD.to_string(),
            volume_method: CURRENT_VOLUME_METHOD.to_string(),
            volume: 2.0,
            capacity: 1.0,
            capacity_lower: Some(0.999_999_999_999),
            capacity_upper: Some(1.000_000_000_001),
            capacity_exact: Some("1/1".to_string()),
            candidate_family: Some("general-hk".to_string()),
            sys: 0.25,
            sigma_gap_cutoff: 0.0,
            sigmas: vec![SigmaAction {
                perm: vec![0],
                action: 1.0,
            }],
            orbit_scalars: Some(OrbitScalars {
                iterations: 1,
                returned_orbit_count: 1,
                best_beta_margin: 0.5,
                best_q_error_bound: 0.0,
                best_has_mu: true,
                best_has_xi: true,
                best_is_admissible_exact: false,
                best_is_indeterminate_f64: false,
            }),
            time_volume_ms: 1.0,
            time_capacity_ms: 2.0,
        }
    }

    fn temp_cache_path() -> PathBuf {
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "datascience-cache-test-{}-{counter}.jsonl",
            std::process::id()
        ))
    }

    fn write_payloads(rows: &[ComputedPolytopePayloadRow]) -> PathBuf {
        let path = temp_cache_path();
        let mut file = File::create(&path).expect("create temp cache file");
        for row in rows {
            serde_json::to_writer(&mut file, row).expect("write payload JSON");
            writeln!(file).expect("write payload newline");
        }
        path
    }

    fn triangle_product() -> SysLandscapePolytopeCache {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("triangle product should construct")
    }

    #[test]
    fn current_compute_emits_exact_minimizer_certificate_without_orbit_scalars() {
        let cache = ComputedPolytopeCache::load(&[]);
        let row = cache
            .compute(&triangle_product(), CapacityBackend::Auto)
            .expect("triangle product should compute");

        assert_eq!(row.capacity_method, CURRENT_CAPACITY_METHOD);
        assert_eq!(
            row.candidate_family.as_deref(),
            Some("product-closure-vertex")
        );
        assert!(row
            .capacity_lower
            .is_some_and(|lower| lower <= row.capacity));
        assert!(row
            .capacity_upper
            .is_some_and(|upper| row.capacity <= upper));
        assert!(row.capacity_exact.is_some());
        assert!(!row.sigmas.is_empty());
        assert!(row.orbit_scalars.is_none());
    }

    #[test]
    fn legacy_capacity_row_is_recomputed_instead_of_counted_as_a_hit() {
        let polytope = triangle_product();
        let current = ComputedPolytopeCache::load(&[])
            .compute(&polytope, CapacityBackend::Auto)
            .expect("triangle product should compute");
        let mut legacy = current;
        legacy.capacity_method = LEGACY_CAPACITY_METHOD.to_string();
        legacy.capacity_lower = None;
        legacy.capacity_upper = None;
        legacy.capacity_exact = None;
        legacy.candidate_family = None;
        let path = write_payloads(&[legacy]);

        let cache = ComputedPolytopeCache::load(std::slice::from_ref(&path));
        let upgraded = cache
            .compute(&polytope, CapacityBackend::Auto)
            .expect("legacy row should recompute");
        std::fs::remove_file(&path).expect("remove temp cache file");

        assert_eq!(upgraded.capacity_method, CURRENT_CAPACITY_METHOD);
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn cache_loader_accepts_execution_metadata_differences() {
        let a = test_payload();
        let mut b = a.clone();
        b.backend = "billiard".to_string();
        b.time_volume_ms = 10.0;
        b.time_capacity_ms = 20.0;
        let path = write_payloads(&[a.clone(), b]);

        let cache = ComputedPolytopeCache::load(std::slice::from_ref(&path));
        std::fs::remove_file(&path).expect("remove temp cache file");

        assert_eq!(cache.rows.len(), 1);
        assert_eq!(
            cache.rows.get("poly-a").expect("payload").capacity,
            a.capacity
        );
    }

    #[test]
    fn cache_loader_accepts_legacy_exact_volume_and_prefers_current_f64_row() {
        let mut exact = test_payload();
        exact.volume_method = LEGACY_VOLUME_METHOD.to_string();
        exact.volume = f64::from_bits(exact.volume.to_bits() + 1);
        exact.sys = f64::from_bits(exact.sys.to_bits() - 1);
        let current = test_payload();
        let path = write_payloads(&[exact, current.clone()]);

        let cache = ComputedPolytopeCache::load(std::slice::from_ref(&path));
        std::fs::remove_file(&path).expect("remove temp cache file");

        let loaded = cache.rows.get("poly-a").expect("payload");
        assert_eq!(loaded.volume_method, CURRENT_VOLUME_METHOD);
        assert_eq!(loaded.volume, current.volume);
        assert_eq!(loaded.sys, current.sys);
    }

    #[test]
    fn cache_loader_accepts_payload_without_volume_method_as_legacy() {
        let row = test_payload();
        let mut value = serde_json::to_value(&row).expect("serialize payload");
        value
            .as_object_mut()
            .expect("payload object")
            .remove("volume_method");

        let loaded: ComputedPolytopePayloadRow =
            serde_json::from_value(value).expect("deserialize legacy payload");
        assert_eq!(loaded.volume_method, LEGACY_VOLUME_METHOD);
    }

    #[test]
    fn cache_loader_accepts_payload_without_capacity_method_as_legacy() {
        let row = test_payload();
        let mut value = serde_json::to_value(&row).expect("serialize payload");
        value
            .as_object_mut()
            .expect("payload object")
            .remove("capacity_method");

        let loaded: ComputedPolytopePayloadRow =
            serde_json::from_value(value).expect("deserialize legacy payload");
        assert_eq!(loaded.capacity_method, LEGACY_CAPACITY_METHOD);
    }

    #[test]
    fn cache_loader_prefers_current_capacity_certificate_over_compatible_legacy_row() {
        let current = test_payload();
        let mut legacy = current.clone();
        legacy.capacity_method = LEGACY_CAPACITY_METHOD.to_string();
        legacy.capacity_lower = None;
        legacy.capacity_upper = None;
        legacy.capacity_exact = None;
        legacy.candidate_family = None;
        let path = write_payloads(&[legacy, current.clone()]);

        let cache = ComputedPolytopeCache::load(std::slice::from_ref(&path));
        std::fs::remove_file(&path).expect("remove temp cache file");

        let loaded = cache.rows.get("poly-a").expect("payload");
        assert_eq!(loaded.capacity_method, CURRENT_CAPACITY_METHOD);
        assert_eq!(loaded.capacity_exact.as_deref(), Some("1/1"));
    }

    #[test]
    #[should_panic(expected = "incomplete current capacity payload")]
    fn cache_loader_rejects_current_method_without_its_certificate_fields() {
        let mut row = test_payload();
        row.capacity_lower = None;
        let path = write_payloads(&[row]);

        let _ = ComputedPolytopeCache::load(std::slice::from_ref(&path));
    }

    #[test]
    fn cache_wal_appends_loadable_payload_rows() {
        let path = temp_cache_path();
        let row = test_payload();
        let cache = ComputedPolytopeCache::load_with_wal(&[], Some(path.clone()));

        cache.append_wal(&row);
        drop(cache);

        let loaded = ComputedPolytopeCache::load(std::slice::from_ref(&path));
        std::fs::remove_file(&path).expect("remove temp cache file");

        assert_eq!(loaded.rows.len(), 1);
        assert_eq!(loaded.rows.get("poly-a").expect("payload").sys, row.sys);
    }

    #[test]
    #[should_panic(expected = "conflicting computed-polytope cache row")]
    fn cache_loader_rejects_semantic_payload_conflicts() {
        let a = test_payload();
        let mut b = a.clone();
        b.capacity = 1.25;
        b.capacity_lower = Some(1.249_999_999_999);
        b.capacity_upper = Some(1.250_000_000_001);
        b.capacity_exact = Some("5/4".to_string());
        let path = write_payloads(&[a, b]);

        let _ = ComputedPolytopeCache::load(std::slice::from_ref(&path));
    }

    #[test]
    #[should_panic(expected = "conflicting computed-polytope cache row")]
    fn cache_loader_rejects_material_volume_conflict() {
        let a = test_payload();
        let mut b = a.clone();
        b.volume *= 1.0 + 1e-8;
        let path = write_payloads(&[a, b]);

        let _ = ComputedPolytopeCache::load(std::slice::from_ref(&path));
    }
}
