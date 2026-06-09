//! Local method-development helpers for first-order `sys` prediction.
//!
//! This package compares local Clarke first-order predictions at a base
//! polytope with full HK2017 recomputation at nearby dual-vertex states. It is
//! method-development code, not thesis evidence by itself.

use euclidean_polytopes::{
    all_points_are_extreme_exact, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    origin_in_interior_of_conv_exact, polar_vertices_exact_rational_assuming_origin_interior,
    sample_random_dual_vertices_f64, volume_from_incidence_exact,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::{OrbitKktData, OrbitSearchError, OrbitSearchResult};
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::exact::omega_signs_exact;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, systolic_ratio,
    OrbitGuaranteeMode,
};

const ACTIVE_ORBIT_RTOL: f64 = 1e-9;
const RANDOM_BASEPOINT_MASTER_SEED: u64 = 0x5a51_2026;
const RANDOM_DIRECTION_SEED: u64 = 0x5a51_2026_0000_0001;
const RANDOM_FACET_COUNT: usize = 10;
const RANDOM_H_MIN: f64 = 0.5;
const RANDOM_H_MAX: f64 = 2.0;
const STEP_VALUES: [f64; 3] = [1e-4, 1e-3, 1e-2];
// Base-only collection window. This is a heuristic diagnostic, and also lets
// exact minimizers survive f64 lower-bound trimming before the active filter.
const BASE_CANDIDATE_ACTION_GAP: f64 = 1e-2;

#[derive(Clone, Debug)]
pub struct LocalPolytopeCache {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub vertices_f64: Vec<Vector4<f64>>,
}

#[derive(Clone, Debug)]
struct BaseState {
    polytope: LocalPolytopeCache,
    capacity: OrbitSearchResult,
    volume: f64,
    sys: f64,
    active_orbits: Vec<OrbitKktData>,
    sys_subgradients: Vec<Vec<Vector4<f64>>>,
    action_spread: f64,
    candidate_action_gap: f64,
    active_min_beta_margin: f64,
    active_max_q_error_bound: f64,
}

#[derive(Debug, Serialize)]
pub struct PredictionRow {
    pub basepoint_name: String,
    pub facet_count: usize,
    pub direction_label: String,
    pub step: f64,
    pub status: String,
    pub sys0: f64,
    pub predicted_sys: f64,
    pub recomputed_sys: Option<f64>,
    pub abs_prediction_error: Option<f64>,
    pub rel_prediction_error: Option<f64>,
    pub base_best_sigma: Vec<usize>,
    pub target_best_sigma: Option<Vec<usize>>,
    pub best_sigma_changed: Option<bool>,
    pub target_best_sigma_in_base_active_set: Option<bool>,
    pub target_best_sigma_in_base_candidate_window: Option<bool>,
    pub active_orbit_count: usize,
    pub base_candidate_orbit_count: usize,
    pub base_candidate_action_gap: f64,
    pub active_action_spread: f64,
    pub active_min_beta_margin: f64,
    pub active_max_q_error_bound: f64,
    pub best_beta_margin: f64,
    pub best_q_error_bound: f64,
    pub step_norm: f64,
}

#[derive(Debug)]
pub enum PredictionError {
    Geometry(String),
    Capacity(OrbitSearchError),
    Derivative(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for PredictionError {
    fn from(err: std::io::Error) -> Self {
        PredictionError::Io(err)
    }
}

impl From<serde_json::Error> for PredictionError {
    fn from(err: serde_json::Error) -> Self {
        PredictionError::Json(err)
    }
}

impl LocalPolytopeCache {
    pub fn from_f64_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
        validate_f64_dual_vertices(&dual_vertices_f64)?;
        let dual_vertices = dual_vertices_f64
            .iter()
            .map(|a| {
                Some(std::array::from_fn(|c| {
                    BigRational::from_float(a[c]).expect("finite f64 was validated")
                }))
            })
            .collect::<Option<Vec<_>>>()?;
        let dual_vectors = vectors_from_arrays(&dual_vertices);

        if !origin_in_interior_of_conv_exact(&dual_vectors)
            || !all_points_are_extreme_exact(&dual_vectors)
        {
            return None;
        }

        let polar = polar_vertices_exact_rational_assuming_origin_interior(&dual_vectors);
        let vertices = arrays_from_vectors(&polar.vertices);
        Some(Self::assemble(
            dual_vertices,
            vertices,
            polar.vertex_facet_incidence,
            dual_vertices_f64,
        ))
    }

    pub fn from_known(polytope: &KnownPolytope) -> Self {
        Self::assemble(
            polytope.dual_vertices.clone(),
            polytope.vertices.clone(),
            polytope.vertex_facet_incidence.clone(),
            polytope.dual_vertices_f64.clone(),
        )
    }

    pub fn generate_random(facet_count: usize, h_min: f64, h_max: f64) -> Option<Self> {
        for attempt in 0..100u64 {
            let mut key_material = [0u8; 16];
            key_material[..8].copy_from_slice(&RANDOM_BASEPOINT_MASTER_SEED.to_le_bytes());
            key_material[8..].copy_from_slice(&attempt.to_le_bytes());
            let seed = blake3::derive_key("local-sys-methods-random-basepoint", &key_material);
            let mut rng = ChaCha8Rng::from_seed(seed);
            let dual_vertices =
                sample_random_dual_vertices_f64(facet_count, h_min, h_max, &mut rng);
            if let Some(polytope) = Self::from_f64_dual_vertices(dual_vertices) {
                return Some(polytope);
            }
        }
        None
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices_f64.len()
    }

    fn assemble(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_facet_incidence: DMatrix<bool>,
        dual_vertices_f64: Vec<Vector4<f64>>,
    ) -> Self {
        let dual_vectors = vectors_from_arrays(&dual_vertices);
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vectors);
        let vertices_f64 = vertices
            .iter()
            .map(vector_f64_from_array)
            .collect::<Option<Vec<_>>>()
            .expect("local sys methods require vertices to be representable as f64");

        Self {
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
            dual_vertices_f64,
            vertices_f64,
        }
    }
}

pub fn default_output_path() -> &'static str {
    "/tmp/local-sys-methods/smoke-local-prediction.jsonl"
}

pub fn run_prediction_smoke(output_path: &Path) -> Result<Vec<PredictionRow>, PredictionError> {
    let basepoints = vec![
        (
            "random_f10_seed_5a51_2026".to_string(),
            LocalPolytopeCache::generate_random(RANDOM_FACET_COUNT, RANDOM_H_MIN, RANDOM_H_MAX)
                .ok_or_else(|| {
                    PredictionError::Geometry("random basepoint generation failed".to_string())
                })?,
        ),
        (
            "hko_pentagon".to_string(),
            LocalPolytopeCache::from_known(known_polytopes::hko_pentagon()),
        ),
    ];

    let mut rows = Vec::new();
    for (name, polytope) in basepoints {
        let base = compute_base_state(polytope, BASE_CANDIDATE_ACTION_GAP)?;
        let directions = prediction_directions(&base)?;
        for (direction_label, direction) in directions {
            for step in STEP_VALUES {
                rows.push(prediction_row(
                    &name,
                    &base,
                    &direction_label,
                    &direction,
                    step,
                )?);
            }
        }
    }

    write_jsonl(output_path, &rows)?;
    Ok(rows)
}

fn compute_capacity_result(
    polytope: &LocalPolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, PredictionError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)
            .map_err(PredictionError::Capacity)?;
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(PredictionError::Capacity)
}

fn compute_volume(polytope: &LocalPolytopeCache) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = polytope
        .vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    volume_from_incidence_exact(&vertices, &polytope.vertex_facet_incidence)
        .to_f64()
        .unwrap_or(f64::NAN)
}

fn compute_base_state(
    polytope: LocalPolytopeCache,
    action_gap: f64,
) -> Result<BaseState, PredictionError> {
    let capacity = compute_capacity_result(&polytope, action_gap)?;
    let volume = compute_volume(&polytope);
    if !volume.is_finite() || volume <= 0.0 {
        return Err(PredictionError::Geometry(
            "base volume is not positive finite".to_string(),
        ));
    }
    let sys = systolic_ratio(capacity.capacity(), volume);
    if !sys.is_finite() {
        return Err(PredictionError::Geometry(
            "base sys is not finite".to_string(),
        ));
    }

    let active_orbits = admissible_active_orbits(&capacity);
    let active_actions: Vec<f64> = active_orbits.iter().map(|orbit| orbit.action).collect();
    let action_spread = spread_or_zero(&active_actions);
    let active_min_beta_margin = active_orbits
        .iter()
        .map(|orbit| orbit.beta_margin)
        .fold(f64::INFINITY, f64::min);
    let active_max_q_error_bound = active_orbits
        .iter()
        .map(|orbit| orbit.q_error_bound)
        .fold(0.0, f64::max);
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|err| PredictionError::Derivative(format!("volume derivative failed: {err:?}")))?;
    let d_capacity_da = capacity_subgradients_a(&polytope.dual_vertices_f64, &active_orbits)
        .map_err(|err| {
            PredictionError::Derivative(format!("capacity derivative failed: {err:?}"))
        })?;
    let sys_subgradients = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(capacity.capacity(), volume, capacity_gradient, &d_volume_da)
        })
        .collect();

    Ok(BaseState {
        polytope,
        capacity,
        volume,
        sys,
        active_orbits,
        sys_subgradients,
        action_spread,
        candidate_action_gap: action_gap,
        active_min_beta_margin,
        active_max_q_error_bound,
    })
}

fn prediction_directions(
    base: &BaseState,
) -> Result<Vec<(String, Vec<Vector4<f64>>)>, PredictionError> {
    let first_gradient = base
        .sys_subgradients
        .first()
        .ok_or_else(|| PredictionError::Derivative("empty sys subgradient set".to_string()))?;
    let gradient_direction = normalize_direction(first_gradient)
        .ok_or_else(|| PredictionError::Derivative("zero first active gradient".to_string()))?;
    let negative_gradient_direction: Vec<Vector4<f64>> =
        gradient_direction.iter().map(|v| -*v).collect();

    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_DIRECTION_SEED);
    let random_a = random_unit_direction(base.polytope.facet_count(), &mut rng)
        .ok_or_else(|| PredictionError::Derivative("random direction A was zero".to_string()))?;
    let random_b = random_unit_direction(base.polytope.facet_count(), &mut rng)
        .ok_or_else(|| PredictionError::Derivative("random direction B was zero".to_string()))?;

    Ok(vec![
        ("first_active_gradient".to_string(), gradient_direction),
        (
            "negative_first_active_gradient".to_string(),
            negative_gradient_direction,
        ),
        ("random_unit_a".to_string(), random_a),
        ("random_unit_b".to_string(), random_b),
    ])
}

fn prediction_row(
    basepoint_name: &str,
    base: &BaseState,
    direction_label: &str,
    direction: &[Vector4<f64>],
    step: f64,
) -> Result<PredictionRow, PredictionError> {
    let directional_derivative = clarke_directional_derivative_a(&base.sys_subgradients, direction)
        .map_err(|err| PredictionError::Derivative(format!("Clarke derivative failed: {err:?}")))?;
    let predicted_sys = base.sys + step * directional_derivative;
    let step_norm = step * flat_norm(direction);
    let best = base.capacity.best_orbit();
    let target_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(direction)
        .map(|(dual, delta)| dual + step * delta)
        .collect();

    let mut row = PredictionRow {
        basepoint_name: basepoint_name.to_string(),
        facet_count: base.polytope.facet_count(),
        direction_label: direction_label.to_string(),
        step,
        status: "ok".to_string(),
        sys0: base.sys,
        predicted_sys,
        recomputed_sys: None,
        abs_prediction_error: None,
        rel_prediction_error: None,
        base_best_sigma: best.sigma.clone(),
        target_best_sigma: None,
        best_sigma_changed: None,
        target_best_sigma_in_base_active_set: None,
        target_best_sigma_in_base_candidate_window: None,
        active_orbit_count: base.active_orbits.len(),
        base_candidate_orbit_count: base.capacity.orbits.len(),
        base_candidate_action_gap: base.candidate_action_gap,
        active_action_spread: base.action_spread,
        active_min_beta_margin: base.active_min_beta_margin,
        active_max_q_error_bound: base.active_max_q_error_bound,
        best_beta_margin: best.beta_margin,
        best_q_error_bound: best.q_error_bound,
        step_norm,
    };

    let Some(target_polytope) = LocalPolytopeCache::from_f64_dual_vertices(target_duals) else {
        row.status = "target_polytope_construction_failed".to_string();
        return Ok(row);
    };
    let target_capacity = match compute_capacity_result(&target_polytope, 0.0) {
        Ok(capacity) => capacity,
        Err(err) => {
            row.status = format!("target_capacity_failed:{err:?}");
            return Ok(row);
        }
    };
    let target_volume = compute_volume(&target_polytope);
    if !target_volume.is_finite() || target_volume <= 0.0 {
        row.status = "target_volume_failed".to_string();
        return Ok(row);
    }
    let recomputed_sys = systolic_ratio(target_capacity.capacity(), target_volume);
    if !recomputed_sys.is_finite() {
        row.status = "target_sys_not_finite".to_string();
        return Ok(row);
    }

    let error = (predicted_sys - recomputed_sys).abs();
    let relative_error = error / recomputed_sys.abs().max(1e-15);
    let target_best_sigma = target_capacity.best_orbit().sigma.clone();
    row.recomputed_sys = Some(recomputed_sys);
    row.abs_prediction_error = Some(error);
    row.rel_prediction_error = Some(relative_error);
    row.best_sigma_changed = Some(row.base_best_sigma != target_best_sigma);
    row.target_best_sigma_in_base_active_set = Some(
        base.active_orbits
            .iter()
            .any(|orbit| orbit.sigma == target_best_sigma),
    );
    row.target_best_sigma_in_base_candidate_window = Some(
        base.capacity
            .orbits
            .iter()
            .any(|orbit| orbit.sigma == target_best_sigma),
    );
    row.target_best_sigma = Some(target_best_sigma);
    Ok(row)
}

fn admissible_active_orbits(result: &OrbitSearchResult) -> Vec<OrbitKktData> {
    let tol = ACTIVE_ORBIT_RTOL * result.min_action.abs().max(1.0);
    let active: Vec<OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                symplectic::OrbitAdmissibility::AdmissibleF64
                    | symplectic::OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| (orbit.action - result.min_action).abs() <= tol)
        .cloned()
        .collect();

    if active.is_empty() {
        vec![result.best_orbit().clone()]
    } else {
        active
    }
}

fn normalize_direction(direction: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let norm = flat_norm(direction);
    (norm > 0.0 && norm.is_finite()).then(|| direction.iter().map(|v| v / norm).collect())
}

fn random_unit_direction(facet_count: usize, rng: &mut ChaCha8Rng) -> Option<Vec<Vector4<f64>>> {
    let direction: Vec<Vector4<f64>> = (0..facet_count)
        .map(|_| {
            Vector4::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
            )
        })
        .collect();
    normalize_direction(&direction)
}

fn flat_norm(direction: &[Vector4<f64>]) -> f64 {
    direction
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt()
}

fn spread_or_zero(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

fn write_jsonl(path: &Path, rows: &[PredictionRow]) -> Result<(), PredictionError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writeln!(writer)?;
    }
    writer.flush()?;
    Ok(())
}

fn validate_f64_dual_vertices(dual_vertices_f64: &[Vector4<f64>]) -> Option<()> {
    if dual_vertices_f64.len() < 5 {
        return None;
    }
    for a in dual_vertices_f64 {
        if !a.iter().all(|value| value.is_finite()) || a.norm() < 1e-12 {
            return None;
        }
    }
    for i in 0..dual_vertices_f64.len() {
        for j in i + 1..dual_vertices_f64.len() {
            let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
            if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm() < 1e-10 * max_norm {
                return None;
            }
        }
    }
    Some(())
}

fn vectors_from_arrays(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect()
}

fn arrays_from_vectors(data: &[Vector4<BigRational>]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|v| [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
        .collect()
}

fn vector_f64_from_array(v: &[BigRational; 4]) -> Option<Vector4<f64>> {
    Some(Vector4::new(
        v[0].to_f64()?,
        v[1].to_f64()?,
        v[2].to_f64()?,
        v[3].to_f64()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_rows_cover_generic_and_hko_basepoints() {
        let random =
            LocalPolytopeCache::generate_random(RANDOM_FACET_COUNT, RANDOM_H_MIN, RANDOM_H_MAX)
                .expect("deterministic random basepoint");
        let hko = LocalPolytopeCache::from_known(known_polytopes::hko_pentagon());

        for (name, polytope) in [("random", random), ("hko", hko)] {
            let base =
                compute_base_state(polytope, BASE_CANDIDATE_ACTION_GAP).unwrap_or_else(|err| {
                    panic!("{name} base state failed: {err:?}");
                });
            let directions = prediction_directions(&base).unwrap_or_else(|err| {
                panic!("{name} directions failed: {err:?}");
            });
            let row = prediction_row(name, &base, &directions[0].0, &directions[0].1, 1e-4)
                .unwrap_or_else(|err| panic!("{name} prediction row failed: {err:?}"));
            assert!(row.predicted_sys.is_finite());
            assert!(row.sys0.is_finite());
            assert!(row.active_min_beta_margin.is_finite());
            assert!(row.active_max_q_error_bound.is_finite());
            assert!(row.base_candidate_orbit_count >= row.active_orbit_count);
            assert_eq!(row.base_candidate_action_gap, BASE_CANDIDATE_ACTION_GAP);
            if name == "random" {
                assert_eq!(row.status, "ok");
            }
            if row.status == "ok" {
                assert!(row.recomputed_sys.expect("recomputed sys").is_finite());
                assert!(row.target_best_sigma_in_base_candidate_window.is_some());
            }
        }
    }
}
