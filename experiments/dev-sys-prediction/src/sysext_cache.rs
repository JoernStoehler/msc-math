use exp_sys_landscape::{
    compute_sys_computation, polytope_key, SysComputation, SysLandscapePolytopeCache,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const CURRENT_CAPACITY_METHOD: &str = "legacy-orbit-search-v1";
const CURRENT_VOLUME_METHOD: &str = "f64-from-exact-derived-incidence-v1";
const UNKNOWN_METHOD: &str = "unknown";

fn unknown_method() -> String {
    UNKNOWN_METHOD.to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SysextCacheRow {
    pub(crate) polytope_key: String,
    pub(crate) facet_count: usize,
    #[serde(default = "unknown_method")]
    pub(crate) capacity_method: String,
    #[serde(default = "unknown_method")]
    pub(crate) volume_method: String,
    #[serde(default)]
    pub(crate) geometry: Option<CachedPolytopeGeometry>,
    pub(crate) volume: f64,
    pub(crate) min_action: f64,
    pub(crate) sys: f64,
    pub(crate) iterations: u64,
    pub(crate) sigma_results: Vec<SysextSigmaResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CachedPolytopeGeometry {
    pub(crate) dual_vertices: Vec<[BigRational; 4]>,
    pub(crate) vertices: Vec<[BigRational; 4]>,
    pub(crate) vertex_facet_incidence: DMatrix<bool>,
    pub(crate) facet_intersection_is_nonempty: DMatrix<bool>,
    pub(crate) omega_signs: DMatrix<i8>,
    pub(crate) dual_vertices_f64: Vec<[f64; 4]>,
    pub(crate) vertices_f64: Vec<[f64; 4]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SysextSigmaResult {
    pub(crate) sigma: Vec<usize>,
    pub(crate) action: f64,
    pub(crate) beta_positive: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct CachedSysextState {
    pub(crate) min_action: f64,
    pub(crate) volume: f64,
    pub(crate) sys: f64,
    pub(crate) iterations: u64,
    pub(crate) sigma_results: Vec<SysextSigmaResult>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SysextCacheOutcome {
    Hit,
    MissComputed,
}

#[derive(Clone, Debug)]
pub(crate) struct CachedSysextLookup {
    pub(crate) state: CachedSysextState,
    pub(crate) outcome: SysextCacheOutcome,
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub(crate) struct SysextCacheStats {
    pub(crate) hits: usize,
    pub(crate) misses: usize,
}

impl CachedSysextState {
    pub(crate) fn best_sigma(&self) -> Option<&[usize]> {
        self.sigma_results
            .iter()
            .filter(|result| result.beta_positive)
            .min_by(|a, b| a.action.total_cmp(&b.action))
            .or_else(|| {
                self.sigma_results
                    .iter()
                    .min_by(|a, b| a.action.total_cmp(&b.action))
            })
            .map(|result| result.sigma.as_slice())
    }

    pub(crate) fn cached_scalar_near_active_count(&self, threshold_relative: f64) -> usize {
        // This is counted only inside the scalar sysext payload stored in the
        // cache row. It is not a target action-window enumeration.
        let cutoff = self.min_action * (1.0 + threshold_relative.max(0.0));
        let count = self
            .sigma_results
            .iter()
            .filter(|result| result.beta_positive)
            .filter(|result| result.action <= cutoff)
            .count();
        count.max(1)
    }
}

pub(crate) struct SysextCache {
    rows: Mutex<HashMap<SysextCacheKey, SysextCacheRow>>,
    used_keys: Mutex<HashSet<SysextCacheKey>>,
    writer: Mutex<Option<BufWriter<File>>>,
    stats: Mutex<SysextCacheStats>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SysextCacheKey {
    polytope_key: String,
    capacity_method: String,
    volume_method: String,
}

impl SysextCacheKey {
    fn current(polytope_key: String) -> Self {
        Self {
            polytope_key,
            capacity_method: CURRENT_CAPACITY_METHOD.to_string(),
            volume_method: CURRENT_VOLUME_METHOD.to_string(),
        }
    }

    fn from_row(row: &SysextCacheRow) -> Self {
        Self {
            polytope_key: row.polytope_key.clone(),
            capacity_method: row.capacity_method.clone(),
            volume_method: row.volume_method.clone(),
        }
    }
}

impl SysextCache {
    pub(crate) fn load(input_paths: &[PathBuf], output_path: Option<&Path>) -> Self {
        let mut rows = HashMap::new();
        for path in input_paths {
            load_existing_rows(path, &mut rows);
        }
        if let Some(path) = output_path {
            load_optional_rows(path, &mut rows);
        }
        let writer = output_path.map(open_append_writer);
        Self {
            rows: Mutex::new(rows),
            used_keys: Mutex::new(HashSet::new()),
            writer: Mutex::new(writer),
            stats: Mutex::new(SysextCacheStats::default()),
        }
    }

    pub(crate) fn compute(
        &self,
        polytope: &SysLandscapePolytopeCache,
    ) -> Option<CachedSysextState> {
        let polytope_key = polytope_key(polytope);
        let key = SysextCacheKey::current(polytope_key.clone());
        if let Some(row) = self
            .rows
            .lock()
            .expect("sysext cache rows poisoned")
            .get(&key)
        {
            self.record_hit(&key);
            return Some(cached_state_from_row(row));
        }

        let computation = compute_sys_computation(polytope)?;
        let row = row_from_computation(polytope_key, polytope, computation);
        self.append_new_row(&row);
        self.rows
            .lock()
            .expect("sysext cache rows poisoned")
            .insert(key.clone(), row.clone());
        self.used_keys
            .lock()
            .expect("sysext cache used keys poisoned")
            .insert(key);
        self.stats
            .lock()
            .expect("sysext cache stats poisoned")
            .misses += 1;
        Some(cached_state_from_row(&row))
    }

    pub(crate) fn compute_from_dual_vertices(
        &self,
        dual_vertices: &[Vector4<f64>],
    ) -> Result<CachedSysextLookup, TargetSysextError> {
        let Some(polytope_key) = polytope_key_from_f64_dual_vertices(dual_vertices) else {
            return Err(TargetSysextError::ConstructionFailed);
        };
        let key = SysextCacheKey::current(polytope_key);
        if let Some(row) = self
            .rows
            .lock()
            .expect("sysext cache rows poisoned")
            .get(&key)
        {
            self.record_hit(&key);
            return Ok(CachedSysextLookup {
                state: cached_state_from_row(row),
                outcome: SysextCacheOutcome::Hit,
            });
        }

        let Some(polytope) =
            SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.to_vec())
        else {
            return Err(TargetSysextError::ConstructionFailed);
        };
        let state = self
            .compute(&polytope)
            .ok_or(TargetSysextError::SysFailed)?;
        Ok(CachedSysextLookup {
            state,
            outcome: SysextCacheOutcome::MissComputed,
        })
    }

    pub(crate) fn polytope_from_dual_vertices(
        &self,
        dual_vertices: &[Vector4<f64>],
    ) -> Option<SysLandscapePolytopeCache> {
        let polytope_key = polytope_key_from_f64_dual_vertices(dual_vertices)?;
        let rows = self.rows.lock().expect("sysext cache rows poisoned");
        let current_key = SysextCacheKey::current(polytope_key.clone());
        let geometry = rows
            .get(&current_key)
            .and_then(|row| row.geometry.as_ref())
            .or_else(|| {
                rows.values()
                    .find(|row| row.polytope_key == polytope_key && row.geometry.is_some())
                    .and_then(|row| row.geometry.as_ref())
            });
        if let Some(geometry) = geometry {
            return polytope_from_geometry(geometry);
        }
        drop(rows);
        SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.to_vec())
    }

    pub(crate) fn stats(&self) -> SysextCacheStats {
        *self.stats.lock().expect("sysext cache stats poisoned")
    }

    pub(crate) fn used_rows(&self) -> Vec<SysextCacheRow> {
        let rows = self.rows.lock().expect("sysext cache rows poisoned");
        let used = self
            .used_keys
            .lock()
            .expect("sysext cache used keys poisoned");
        let mut out = used
            .iter()
            .filter_map(|key| rows.get(key).cloned())
            .collect::<Vec<_>>();
        out.sort_by(|a, b| {
            (&a.polytope_key, &a.capacity_method, &a.volume_method).cmp(&(
                &b.polytope_key,
                &b.capacity_method,
                &b.volume_method,
            ))
        });
        out
    }

    fn record_hit(&self, key: &SysextCacheKey) {
        self.used_keys
            .lock()
            .expect("sysext cache used keys poisoned")
            .insert(key.clone());
        self.stats.lock().expect("sysext cache stats poisoned").hits += 1;
    }

    fn append_new_row(&self, row: &SysextCacheRow) {
        let mut writer = self.writer.lock().expect("sysext cache writer poisoned");
        let Some(writer) = writer.as_mut() else {
            return;
        };
        serde_json::to_writer(&mut *writer, row).expect("failed to serialize sysext cache row");
        writeln!(writer).expect("failed to write sysext cache row");
        writer.flush().expect("failed to flush sysext cache row");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TargetSysextError {
    ConstructionFailed,
    SysFailed,
}

fn row_from_computation(
    polytope_key: String,
    polytope: &SysLandscapePolytopeCache,
    computation: SysComputation,
) -> SysextCacheRow {
    SysextCacheRow {
        polytope_key,
        facet_count: polytope.facet_count(),
        capacity_method: CURRENT_CAPACITY_METHOD.to_string(),
        volume_method: CURRENT_VOLUME_METHOD.to_string(),
        geometry: Some(geometry_from_polytope(polytope)),
        volume: computation.vol,
        min_action: computation.capacity.min_action,
        sys: computation.sys,
        iterations: computation.capacity.iterations,
        sigma_results: computation
            .capacity
            .orbits
            .into_iter()
            .map(|orbit| SysextSigmaResult {
                sigma: orbit.sigma,
                action: orbit.action,
                beta_positive: orbit.beta_margin > 0.0,
            })
            .collect(),
    }
}

fn geometry_from_polytope(polytope: &SysLandscapePolytopeCache) -> CachedPolytopeGeometry {
    CachedPolytopeGeometry {
        dual_vertices: polytope.dual_vertices.clone(),
        vertices: polytope.vertices.clone(),
        vertex_facet_incidence: polytope.vertex_facet_incidence.clone(),
        facet_intersection_is_nonempty: polytope.facet_intersection_is_nonempty.clone(),
        omega_signs: polytope.omega_signs.clone(),
        dual_vertices_f64: vec4s_to_arrays(&polytope.dual_vertices_f64),
        vertices_f64: vec4s_to_arrays(&polytope.vertices_f64),
    }
}

fn polytope_from_geometry(geometry: &CachedPolytopeGeometry) -> Option<SysLandscapePolytopeCache> {
    SysLandscapePolytopeCache::from_trusted_parts(
        geometry.dual_vertices.clone(),
        geometry.vertices.clone(),
        geometry.vertex_facet_incidence.clone(),
        geometry.facet_intersection_is_nonempty.clone(),
        geometry.omega_signs.clone(),
        arrays_to_vec4s(&geometry.dual_vertices_f64),
        arrays_to_vec4s(&geometry.vertices_f64),
    )
}

fn polytope_key_from_f64_dual_vertices(dual_vertices: &[Vector4<f64>]) -> Option<String> {
    if !dual_vertices
        .iter()
        .all(|vertex| vertex.iter().all(|value| value.is_finite()))
    {
        return None;
    }
    let rational = dual_vertices
        .iter()
        .map(|a| {
            Some(std::array::from_fn(|c| {
                BigRational::from_float(a[c]).expect("finite f64 was checked")
            }))
        })
        .collect::<Option<Vec<[BigRational; 4]>>>()?;
    Some(
        rational
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| format!("{}/{}", value.numer(), value.denom()))
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join("|"),
    )
}

fn vec4s_to_arrays(values: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    values
        .iter()
        .map(|value| [value[0], value[1], value[2], value[3]])
        .collect()
}

fn arrays_to_vec4s(values: &[[f64; 4]]) -> Vec<Vector4<f64>> {
    values
        .iter()
        .map(|value| Vector4::new(value[0], value[1], value[2], value[3]))
        .collect()
}

fn cached_state_from_row(row: &SysextCacheRow) -> CachedSysextState {
    CachedSysextState {
        min_action: row.min_action,
        volume: row.volume,
        sys: row.sys,
        iterations: row.iterations,
        sigma_results: row.sigma_results.clone(),
    }
}

fn open_append_writer(path: &Path) -> BufWriter<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| {
            panic!(
                "failed to create sysext cache output parent {}: {err}",
                parent.display()
            )
        });
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|err| panic!("failed to open sysext cache {}: {err}", path.display()));
    BufWriter::new(file)
}

fn load_existing_rows(path: &Path, rows: &mut HashMap<SysextCacheKey, SysextCacheRow>) {
    let file = File::open(path)
        .unwrap_or_else(|err| panic!("failed to open sysext cache {}: {err}", path.display()));
    load_rows_from_file(path, file, rows);
}

fn load_optional_rows(path: &Path, rows: &mut HashMap<SysextCacheKey, SysextCacheRow>) {
    let Ok(file) = File::open(path) else {
        return;
    };
    load_rows_from_file(path, file, rows);
}

fn load_rows_from_file(
    path: &Path,
    file: File,
    rows: &mut HashMap<SysextCacheKey, SysextCacheRow>,
) {
    let reader = BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|err| {
            panic!(
                "failed to read sysext cache {:?}:{}: {err}",
                path,
                line_number + 1
            )
        });
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row = serde_json::from_str::<SysextCacheRow>(line).unwrap_or_else(|err| {
            panic!(
                "invalid sysext cache JSON {:?}:{}: {err}",
                path,
                line_number + 1
            )
        });
        for previous in rows
            .values()
            .filter(|previous| previous.polytope_key == row.polytope_key)
        {
            if let (Some(previous_geometry), Some(row_geometry)) =
                (&previous.geometry, &row.geometry)
            {
                assert_eq!(
                    previous_geometry,
                    row_geometry,
                    "conflicting sysext cache geometry for polytope_key {:?} in {:?}:{}",
                    row.polytope_key,
                    path,
                    line_number + 1
                );
            }
        }
        let key = SysextCacheKey::from_row(&row);
        if let Some(previous) = rows.get(&key) {
            assert!(
                rows_match_except_geometry(previous, &row),
                "conflicting sysext cache row for key {:?} in {:?}:{}",
                key,
                path,
                line_number + 1
            );
        }
        rows.entry(key)
            .and_modify(|previous| {
                if previous.geometry.is_none() && row.geometry.is_some() {
                    *previous = row.clone();
                }
            })
            .or_insert(row);
    }
}

fn rows_match_except_geometry(left: &SysextCacheRow, right: &SysextCacheRow) -> bool {
    left.polytope_key == right.polytope_key
        && left.facet_count == right.facet_count
        && left.capacity_method == right.capacity_method
        && left.volume_method == right.volume_method
        && left.volume == right.volume
        && left.min_action == right.min_action
        && left.sys == right.sys
        && left.iterations == right.iterations
        && left.sigma_results == right.sigma_results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn row(polytope_key: &str, capacity_method: &str, volume_method: &str) -> SysextCacheRow {
        SysextCacheRow {
            polytope_key: polytope_key.to_string(),
            facet_count: 4,
            capacity_method: capacity_method.to_string(),
            volume_method: volume_method.to_string(),
            geometry: None,
            volume: 2.0,
            min_action: 1.0,
            sys: 0.25,
            iterations: 3,
            sigma_results: Vec::new(),
        }
    }

    fn temp_path(label: &str) -> PathBuf {
        let serial = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "dev-sys-prediction-sysext-cache-{label}-{}-{serial}.jsonl",
            std::process::id()
        ))
    }

    #[test]
    fn unlabeled_rows_deserialize_with_unknown_method_identity() {
        let value = serde_json::json!({
            "polytope_key": "geometry",
            "facet_count": 4,
            "volume": 2.0,
            "min_action": 1.0,
            "sys": 0.25,
            "iterations": 3,
            "sigma_results": []
        });
        let row: SysextCacheRow = serde_json::from_value(value).unwrap();

        assert_eq!(row.capacity_method, UNKNOWN_METHOD);
        assert_eq!(row.volume_method, UNKNOWN_METHOD);
        assert_ne!(
            SysextCacheKey::from_row(&row),
            SysextCacheKey::current(row.polytope_key.clone())
        );
    }

    #[test]
    fn loader_keeps_legacy_unknown_and_current_rows_in_separate_namespaces() {
        let path = temp_path("method-namespaces");
        let legacy = row("geometry", UNKNOWN_METHOD, UNKNOWN_METHOD);
        let mut current = row("geometry", CURRENT_CAPACITY_METHOD, CURRENT_VOLUME_METHOD);
        current.volume = 2.5;
        current.sys = 0.2;
        let contents = format!(
            "{}\n{}\n",
            serde_json::to_string(&legacy).unwrap(),
            serde_json::to_string(&current).unwrap()
        );
        fs::write(&path, contents).unwrap();

        let mut rows = HashMap::new();
        load_existing_rows(&path, &mut rows);
        fs::remove_file(&path).unwrap();

        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.get(&SysextCacheKey::current("geometry".to_string()))
                .unwrap(),
            &current
        );
        assert_eq!(
            rows.get(&SysextCacheKey::from_row(&legacy)).unwrap(),
            &legacy
        );
    }
}
