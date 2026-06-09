use crate::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use symplectic::OrbitSearchResult;

use super::compute::{compute_capacity_result, SysComputation};
use super::dual_vertices_rational_strings;

#[derive(Clone, Serialize, Deserialize)]
pub struct ExpensiveComputationCacheRow {
    pub polytope_key: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity_result: OrbitSearchResult,
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

        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        if volume <= 0.0 {
            return None;
        }
        let capacity_result = compute_capacity_result(polytope)?;
        let capacity = capacity_result.capacity();
        let sys = symplectic::systolic_ratio(capacity, volume);
        if !sys.is_finite() {
            return None;
        }

        let row = ExpensiveComputationCacheRow {
            polytope_key: key.clone(),
            dual_vertices_rational: dual_vertices_rational_strings(polytope),
            facet_count: polytope.facet_count(),
            capacity_result: capacity_result.clone(),
            volume,
            sys,
        };
        {
            let mut used = self
                .used_rows
                .lock()
                .expect("emitted cache mutex poisoned");
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
        let used = self
            .used_rows
            .lock()
            .expect("used cache mutex poisoned");
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
    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(row) = serde_json::from_str::<ExpensiveComputationCacheRow>(line) else {
            continue;
        };
        rows.entry(row.polytope_key.clone()).or_insert(row);
    }
}
