use crate::SysLandscapePolytopeCache;
use euclidean_polytopes::volume_from_incidence_f64;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use symplectic::OrbitSearchResult;

use super::compute::{compute_capacity_result, SysComputation};
use super::dual_vertices_rational_strings;

const CURRENT_VOLUME_METHOD: &str = "f64-from-exact-derived-incidence-v1";
const LEGACY_VOLUME_METHOD: &str = "exact-rational-rounded-f64-v1";
const DERIVED_VALUE_RELATIVE_TOLERANCE: f64 = 1e-12;

fn legacy_volume_method() -> String {
    LEGACY_VOLUME_METHOD.to_string()
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpensiveComputationCacheRow {
    pub polytope_key: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity_result: OrbitSearchResult,
    #[serde(default = "legacy_volume_method")]
    pub volume_method: String,
    pub volume: f64,
    pub sys: f64,
}

#[derive(Default)]
pub struct ExpensiveComputationCache {
    rows: HashMap<String, ExpensiveComputationCacheRow>,
    used_rows: Mutex<HashMap<String, ExpensiveComputationCacheRow>>,
    stats: Mutex<ExpensiveComputationCacheStats>,
}

#[derive(Clone, Copy, Default)]
pub struct ExpensiveComputationCacheStats {
    pub hits: usize,
    pub misses: usize,
}

impl ExpensiveComputationCache {
    pub fn load(paths: &[PathBuf]) -> Self {
        let mut rows = HashMap::new();
        for path in paths {
            load_rows(path, &mut rows);
        }
        Self {
            rows,
            used_rows: Mutex::new(HashMap::new()),
            stats: Mutex::new(ExpensiveComputationCacheStats::default()),
        }
    }

    pub fn empty() -> Self {
        Self::load(&[])
    }

    pub fn compute(&self, polytope: &SysLandscapePolytopeCache) -> Option<SysComputation> {
        let key = polytope_key(polytope);
        if let Some(row) = self.rows.get(&key) {
            {
                let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
                used.entry(key).or_insert_with(|| row.clone());
            }
            let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
            stats.hits += 1;
            return Some(SysComputation {
                capacity: row.capacity_result.clone(),
                vol: row.volume,
                sys: row.sys,
            });
        }
        if let Some(row) = self
            .used_rows
            .lock()
            .expect("used cache mutex poisoned")
            .get(&key)
            .cloned()
        {
            let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
            stats.hits += 1;
            return Some(SysComputation {
                capacity: row.capacity_result,
                vol: row.volume,
                sys: row.sys,
            });
        }

        let volume =
            volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                .ok()?;
        if volume <= 0.0 {
            return None;
        }
        let capacity_result = compute_capacity_result(polytope)?;
        let capacity = capacity_result.min_action;
        let sys = symplectic::systolic_ratio(capacity, volume);
        if !sys.is_finite() {
            return None;
        }

        let row = ExpensiveComputationCacheRow {
            polytope_key: key.clone(),
            dual_vertices_rational: dual_vertices_rational_strings(polytope),
            facet_count: polytope.facet_count(),
            capacity_result: capacity_result.clone(),
            volume_method: CURRENT_VOLUME_METHOD.to_string(),
            volume,
            sys,
        };
        {
            let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
            used.entry(key).or_insert(row);
        }
        {
            let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
            stats.misses += 1;
        }

        Some(SysComputation {
            capacity: capacity_result,
            vol: volume,
            sys,
        })
    }

    pub fn used_rows(&self) -> Vec<ExpensiveComputationCacheRow> {
        let used = self.used_rows.lock().expect("used cache mutex poisoned");
        let mut rows: Vec<_> = used.values().cloned().collect();
        rows.sort_by(|a, b| a.polytope_key.cmp(&b.polytope_key));
        rows
    }

    pub fn stats(&self) -> ExpensiveComputationCacheStats {
        *self.stats.lock().expect("cache stats mutex poisoned")
    }
}

pub fn polytope_key(polytope: &SysLandscapePolytopeCache) -> String {
    dual_vertices_rational_strings(polytope)
        .into_iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("|")
}

fn load_rows(path: &Path, rows: &mut HashMap<String, ExpensiveComputationCacheRow>) {
    let Ok(file) = File::open(path) else {
        return;
    };
    let reader = BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| {
            panic!(
                "failed to read expensive-computation cache {:?}:{}: {e}",
                path,
                line_number + 1
            )
        });
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row = serde_json::from_str::<ExpensiveComputationCacheRow>(line).unwrap_or_else(|e| {
            panic!(
                "invalid expensive-computation cache JSON {:?}:{}: {e}",
                path,
                line_number + 1
            )
        });
        if let Some(previous) = rows.get(&row.polytope_key) {
            assert!(
                semantic_row_eq(previous, &row),
                "conflicting expensive-computation cache row for polytope_key {:?} in {:?}:{}",
                row.polytope_key,
                path,
                line_number + 1
            );
            if previous.volume_method == CURRENT_VOLUME_METHOD
                && row.volume_method != CURRENT_VOLUME_METHOD
            {
                continue;
            }
        }
        rows.insert(row.polytope_key.clone(), row);
    }
}

fn approximately_equal_derived_value(a: f64, b: f64) -> bool {
    a == b
        || ((a - b).abs()
            <= DERIVED_VALUE_RELATIVE_TOLERANCE * a.abs().max(b.abs()).max(f64::MIN_POSITIVE))
}

fn semantic_row_eq(a: &ExpensiveComputationCacheRow, b: &ExpensiveComputationCacheRow) -> bool {
    a.polytope_key == b.polytope_key
        && a.dual_vertices_rational == b.dual_vertices_rational
        && a.facet_count == b.facet_count
        && a.capacity_result == b.capacity_result
        && approximately_equal_derived_value(a.volume, b.volume)
        && approximately_equal_derived_value(a.sys, b.sys)
}

#[cfg(test)]
mod tests {
    use super::approximately_equal_derived_value;

    #[test]
    fn cache_compatibility_accepts_roundoff_but_not_material_change() {
        let value = 2.0_f64;
        assert!(approximately_equal_derived_value(
            value,
            f64::from_bits(value.to_bits() + 1)
        ));
        assert!(!approximately_equal_derived_value(
            value,
            value * (1.0 + 1e-8)
        ));
    }
}
