//! Shared computed-polytope cache for datascience producers.
//!
//! The cache is keyed by canonical f64 dual-vertex bits. Producer metadata should
//! point at `poly_id`; this row owns the expensive capacity/orbit payload.

use crate::{
    capacity_auto, capacity_billiard, exact_volume_from_incidence_as_f64,
    orbit_scalars_from_result, SysLandscapePolytopeCache,
};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use symplectic::database::{OrbitScalars, SigmaAction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapacityBackend {
    Auto,
    Billiard,
}

impl CapacityBackend {
    fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Billiard => "billiard",
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
    pub volume: f64,
    pub capacity: f64,
    pub sys: f64,
    pub sigma_gap_cutoff: f64,
    pub sigmas: Vec<SigmaAction>,
    pub orbit_scalars: OrbitScalars,
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
    stats: Mutex<ComputeCacheStats>,
}

impl ComputedPolytopeCache {
    pub fn load(paths: &[PathBuf]) -> Self {
        let mut rows = HashMap::new();
        for path in paths {
            load_payload_rows(path, &mut rows);
        }
        Self {
            rows,
            used_rows: Mutex::new(HashMap::new()),
            stats: Mutex::new(ComputeCacheStats::default()),
        }
    }

    pub fn compute(
        &self,
        polytope: &SysLandscapePolytopeCache,
        backend: CapacityBackend,
    ) -> Option<ComputedPolytopePayloadRow> {
        let poly_id = poly_id(polytope);
        if let Some(row) = self.rows.get(&poly_id) {
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
            let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
            stats.hits += 1;
            return Some(row);
        }

        let start_volume = Instant::now();
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let time_volume_ms = start_volume.elapsed().as_secs_f64() * 1000.0;
        if volume <= 0.0 {
            return None;
        }

        let start_capacity = Instant::now();
        let capacity_result = match backend {
            CapacityBackend::Auto => capacity_auto(
                &polytope.dual_vertices_f64,
                &polytope.dual_vertices,
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            )
            .ok()?,
            CapacityBackend::Billiard => capacity_billiard(
                &polytope.dual_vertices_f64,
                &polytope.dual_vertices,
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            )
            .ok()?,
        };
        let time_capacity_ms = start_capacity.elapsed().as_secs_f64() * 1000.0;

        let capacity = capacity_result.capacity();
        let sys = symplectic::systolic_ratio(capacity, volume);
        if !sys.is_finite() {
            return None;
        }

        let row = ComputedPolytopePayloadRow {
            poly_id: poly_id.clone(),
            dual_vertices: f64_dual_vertices(polytope),
            dual_vertices_rational: rational_vec4_to_strings(&polytope.dual_vertices),
            vertices_rational: rational_vec4_to_strings(&polytope.vertices),
            facet_count: polytope.facet_count(),
            backend: backend.name().to_string(),
            volume,
            capacity,
            sys,
            sigma_gap_cutoff: 0.0,
            sigmas: vec![SigmaAction {
                perm: capacity_result.best_sigma().to_vec(),
                action: capacity,
            }],
            orbit_scalars: orbit_scalars_from_result(&capacity_result),
            time_volume_ms,
            time_capacity_ms,
        };

        {
            let mut used = self.used_rows.lock().expect("used cache mutex poisoned");
            used.entry(poly_id).or_insert_with(|| row.clone());
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
        let mut stats = self.stats.lock().expect("cache stats mutex poisoned");
        stats.hits += 1;
    }
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
        if let Some(previous) = rows.get(&row.poly_id) {
            assert!(
                semantic_payload_eq(previous, &row),
                "conflicting computed-polytope cache row for poly_id {:?} in {:?}:{}",
                row.poly_id,
                path,
                line_number + 1
            );
        }
        rows.insert(row.poly_id.clone(), row);
    }
}

fn semantic_payload_eq(a: &ComputedPolytopePayloadRow, b: &ComputedPolytopePayloadRow) -> bool {
    a.poly_id == b.poly_id
        && a.dual_vertices == b.dual_vertices
        && a.dual_vertices_rational == b.dual_vertices_rational
        && a.vertices_rational == b.vertices_rational
        && a.facet_count == b.facet_count
        && a.volume == b.volume
        && a.capacity == b.capacity
        && a.sys == b.sys
        && a.sigma_gap_cutoff == b.sigma_gap_cutoff
        && a.sigmas == b.sigmas
        && a.orbit_scalars == b.orbit_scalars
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector4;

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
}
