//! Facet sampling, resume helpers, and small experiment-local utilities.

use crate::flat_polytope::HkoPolytopeCache;
use euclidean_polytopes::vertex_facets_from_vertex_facet_incidence;
use nalgebra::Vector4;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Add a barely-non-redundant facet to a polytope.
///
/// New dual vertex: a_{F+1} = n / (h_K(n) - ε) where h_K(n) = max_v ⟨n,v⟩.
// TODO: add [lem:facet-addition] to formal math (dual vertex ↔ halfspace correspondence)
pub(crate) fn add_facet(
    polytope: &HkoPolytopeCache,
    direction: &Vector4<f64>,
    epsilon: f64,
) -> Option<HkoPolytopeCache> {
    let vertices = &polytope.vertices_f64;
    let h_k_n = vertices
        .iter()
        .map(|v| direction.dot(v))
        .fold(f64::NEG_INFINITY, f64::max);
    let new_h = h_k_n - epsilon;
    if new_h <= 0.0 {
        return None;
    }
    let mut new_duals: Vec<Vector4<f64>> = polytope.dual_vertices_f64.to_vec();
    new_duals.push(direction / new_h);
    HkoPolytopeCache::from_f64(new_duals)
}

pub(crate) fn random_direction(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    let x: f64 = StandardNormal.sample(rng);
    let y: f64 = StandardNormal.sample(rng);
    let z: f64 = StandardNormal.sample(rng);
    let w: f64 = StandardNormal.sample(rng);
    Vector4::new(x, y, z, w).normalize()
}

pub(crate) fn last_facet_active(polytope: &HkoPolytopeCache) -> bool {
    let last_idx = polytope.facet_count() - 1;
    vertex_facets_from_vertex_facet_incidence(&polytope.vertex_facet_incidence)
        .iter()
        .any(|facets| facets.contains(&last_idx))
}

pub(crate) fn load_completed_names(path: &Path) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

pub(crate) fn dvs_to_array(polytope: &HkoPolytopeCache) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}
