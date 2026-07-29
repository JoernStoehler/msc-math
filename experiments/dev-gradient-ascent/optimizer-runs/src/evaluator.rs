use crate::quotient::flatten;
use crate::schema::EvaluationRow;
use euclidean_polytopes::volume_from_incidence_f64;
use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays, f64_geometry_payload, F64GeometryPayload,
};
use exp_sys_landscape::{reference::exact_volume_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::time::Instant;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitGuaranteeMode, OrbitKktData,
    OrbitSearchError, OrbitSearchResult,
};

const SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GeometryMode {
    /// Heuristic binary64 geometry with explicit indeterminate-predicate counts.
    F64,
    /// Exact geometry of the binary64 input coordinates.
    Exact,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VolumeMode {
    F64,
    Exact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EvaluatorConfig {
    pub geometry_mode: GeometryMode,
    pub volume_mode: VolumeMode,
    pub accept_indeterminate_geometry: bool,
    pub exact_geometry_fallback: bool,
    pub cache_within_run: bool,
}

impl Default for EvaluatorConfig {
    fn default() -> Self {
        Self {
            geometry_mode: GeometryMode::Exact,
            volume_mode: VolumeMode::F64,
            accept_indeterminate_geometry: true,
            exact_geometry_fallback: true,
            cache_within_run: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Evaluation {
    pub row: EvaluationRow,
    pub duals: Vec<Vector4<f64>>,
    pub physical_evaluation: bool,
    pub context: Option<Arc<EvaluationContext>>,
}

/// Non-serialized state shared with structure-aware optimizers.
#[derive(Clone, Debug)]
pub struct EvaluationContext {
    pub polytope: Arc<SysLandscapePolytopeCache>,
    pub volume: f64,
    pub min_action: f64,
    pub winning_orbit: OrbitKktData,
}

#[derive(Clone, Debug)]
struct CachedOutcome {
    context: Option<Arc<EvaluationContext>>,
    status: String,
    geometry_route: String,
    fallback_reason: Option<String>,
    usable_by_optimizer: bool,
    error: Option<String>,
    sys: Option<f64>,
    capacity: Option<f64>,
    volume: Option<f64>,
    winning_sigma: Option<Vec<usize>>,
    winning_beta_margin: Option<f64>,
    orbit_count: Option<usize>,
    sigma_iterations: Option<u64>,
    geometry_indeterminate_count: usize,
    vertex_indeterminate_count: usize,
    bounded_near_singular_vertex_count: usize,
    ambiguous_vertex_incidence_count: usize,
    facet_intersection_indeterminate_count: usize,
    omega_indeterminate_count: usize,
    geometry_ms: f64,
    volume_ms: f64,
    capacity_ms: f64,
    total_ms: f64,
}

pub struct Evaluator {
    config: EvaluatorConfig,
    cache: HashMap<String, CachedOutcome>,
}

impl Evaluator {
    pub fn new(config: EvaluatorConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        &mut self,
        run_id: &str,
        evaluation_id: String,
        proposal_id: Option<String>,
        role: &str,
        logical_call: usize,
        charged: bool,
        duals: Vec<Vector4<f64>>,
    ) -> Evaluation {
        let key = point_key(&duals);
        if self.config.cache_within_run {
            if let Some(cached) = self.cache.get(&key).cloned() {
                let context = cached.context.clone();
                return Evaluation {
                    row: row_from_outcome(
                        run_id,
                        evaluation_id,
                        proposal_id,
                        role,
                        logical_call,
                        charged,
                        &key,
                        "hit",
                        &duals,
                        cached_without_cost(cached),
                    ),
                    duals,
                    physical_evaluation: false,
                    context,
                };
            }
        }
        let outcome = compute_outcome(&duals, &self.config);
        let context = outcome.context.clone();
        if self.config.cache_within_run {
            self.cache.insert(key.clone(), outcome.clone());
        }
        Evaluation {
            row: row_from_outcome(
                run_id,
                evaluation_id,
                proposal_id,
                role,
                logical_call,
                charged,
                &key,
                "miss",
                &duals,
                outcome,
            ),
            duals,
            physical_evaluation: true,
            context,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn row_from_outcome(
    run_id: &str,
    evaluation_id: String,
    proposal_id: Option<String>,
    role: &str,
    logical_call: usize,
    charged: bool,
    point_key: &str,
    cache_status: &str,
    duals: &[Vector4<f64>],
    outcome: CachedOutcome,
) -> EvaluationRow {
    EvaluationRow {
        schema_version: SCHEMA_VERSION,
        run_id: run_id.to_string(),
        evaluation_id,
        proposal_id,
        role: role.to_string(),
        logical_call,
        charged,
        point_key: point_key.to_string(),
        cache_status: cache_status.to_string(),
        status: outcome.status,
        geometry_route: outcome.geometry_route,
        fallback_reason: outcome.fallback_reason,
        usable_by_optimizer: outcome.usable_by_optimizer,
        error: outcome.error,
        facet_count: duals.len(),
        dual_flat: flatten(duals),
        sys: outcome.sys,
        capacity: outcome.capacity,
        volume: outcome.volume,
        winning_sigma: outcome.winning_sigma,
        winning_beta_margin: outcome.winning_beta_margin,
        orbit_count: outcome.orbit_count,
        sigma_iterations: outcome.sigma_iterations,
        geometry_indeterminate_count: outcome.geometry_indeterminate_count,
        vertex_indeterminate_count: outcome.vertex_indeterminate_count,
        bounded_near_singular_vertex_count: outcome.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: outcome.ambiguous_vertex_incidence_count,
        facet_intersection_indeterminate_count: outcome.facet_intersection_indeterminate_count,
        omega_indeterminate_count: outcome.omega_indeterminate_count,
        geometry_ms: outcome.geometry_ms,
        volume_ms: outcome.volume_ms,
        capacity_ms: outcome.capacity_ms,
        total_ms: outcome.total_ms,
    }
}

fn cached_without_cost(mut cached: CachedOutcome) -> CachedOutcome {
    cached.geometry_ms = 0.0;
    cached.volume_ms = 0.0;
    cached.capacity_ms = 0.0;
    cached.total_ms = 0.0;
    cached
}

fn compute_outcome(duals: &[Vector4<f64>], config: &EvaluatorConfig) -> CachedOutcome {
    let total_started = Instant::now();
    let geometry_started = Instant::now();
    let geometry = build_geometry(duals, config.geometry_mode);
    let (mut polytope, counts, mut geometry_route, mut fallback_reason) = match geometry {
        Ok((polytope, counts)) => (
            polytope,
            counts,
            match config.geometry_mode {
                GeometryMode::F64 => "f64",
                GeometryMode::Exact => "exact",
            }
            .to_string(),
            None,
        ),
        Err(error)
            if config.geometry_mode == GeometryMode::F64 && config.exact_geometry_fallback =>
        {
            match build_geometry(duals, GeometryMode::Exact) {
                Ok((polytope, counts)) => (
                    polytope,
                    counts,
                    "exact_fallback".to_string(),
                    Some(format!("f64_geometry:{error}")),
                ),
                Err(exact_error) => {
                    return failed_outcome(
                        format!("f64_geometry:{error};exact_geometry:{exact_error}"),
                        geometry_started.elapsed().as_secs_f64() * 1000.0,
                        total_started.elapsed().as_secs_f64() * 1000.0,
                    )
                }
            }
        }
        Err(error) => {
            return failed_outcome(
                error,
                geometry_started.elapsed().as_secs_f64() * 1000.0,
                total_started.elapsed().as_secs_f64() * 1000.0,
            )
        }
    };
    let mut geometry_ms = geometry_started.elapsed().as_secs_f64() * 1000.0;
    let geometry_indeterminate_count = counts.total();
    let volume_started = Instant::now();
    let mut volume = compute_volume(&polytope, config.volume_mode);
    let mut volume_ms = volume_started.elapsed().as_secs_f64() * 1000.0;
    if volume.is_err() && geometry_route == "f64" && config.exact_geometry_fallback {
        let fast_error = volume.as_ref().expect_err("checked error").to_string();
        let fallback_geometry_started = Instant::now();
        match build_geometry(duals, GeometryMode::Exact) {
            Ok((exact_polytope, _)) => {
                geometry_ms += fallback_geometry_started.elapsed().as_secs_f64() * 1000.0;
                polytope = exact_polytope;
                geometry_route = "exact_fallback".to_string();
                fallback_reason = Some(format!("f64_volume:{fast_error}"));
                let fallback_volume_started = Instant::now();
                volume = compute_volume(&polytope, config.volume_mode);
                volume_ms += fallback_volume_started.elapsed().as_secs_f64() * 1000.0;
            }
            Err(exact_error) => {
                geometry_ms += fallback_geometry_started.elapsed().as_secs_f64() * 1000.0;
                volume = Err(format!(
                    "f64_volume:{fast_error};exact_geometry:{exact_error}"
                ));
            }
        }
    }
    let volume = match volume {
        Ok(value) if value.is_finite() && value > 0.0 => value,
        Ok(_) => {
            return failed_with_route(
                "nonpositive_or_nonfinite_volume".to_string(),
                &counts,
                &geometry_route,
                fallback_reason.clone(),
                geometry_ms,
                volume_ms,
                0.0,
                total_started.elapsed().as_secs_f64() * 1000.0,
            )
        }
        Err(error) => {
            return failed_with_route(
                error,
                &counts,
                &geometry_route,
                fallback_reason.clone(),
                geometry_ms,
                volume_ms,
                0.0,
                total_started.elapsed().as_secs_f64() * 1000.0,
            )
        }
    };
    let capacity_started = Instant::now();
    let capacity = capacity_minima_safe(&polytope);
    let capacity_ms = capacity_started.elapsed().as_secs_f64() * 1000.0;
    let capacity = match capacity {
        Ok(value) => value,
        Err(error) => {
            return failed_with_route(
                format!("capacity_failed:{error:?}"),
                &counts,
                &geometry_route,
                fallback_reason.clone(),
                geometry_ms,
                volume_ms,
                capacity_ms,
                total_started.elapsed().as_secs_f64() * 1000.0,
            )
        }
    };
    let sys = symplectic::systolic_ratio(capacity.min_action, volume);
    if !sys.is_finite() {
        return failed_with_route(
            "nonfinite_systolic_ratio".to_string(),
            &counts,
            &geometry_route,
            fallback_reason.clone(),
            geometry_ms,
            volume_ms,
            capacity_ms,
            total_started.elapsed().as_secs_f64() * 1000.0,
        );
    }
    let indeterminate = geometry_indeterminate_count > 0;
    let best = capacity.best_orbit();
    CachedOutcome {
        context: Some(Arc::new(EvaluationContext {
            polytope: Arc::new(polytope),
            volume,
            min_action: capacity.min_action,
            winning_orbit: best.clone(),
        })),
        status: if geometry_route == "exact_fallback" {
            "exact_fallback".to_string()
        } else if indeterminate {
            "indeterminate_geometry".to_string()
        } else {
            "ok".to_string()
        },
        geometry_route,
        fallback_reason,
        usable_by_optimizer: !indeterminate || config.accept_indeterminate_geometry,
        error: None,
        sys: Some(sys),
        capacity: Some(capacity.min_action),
        volume: Some(volume),
        winning_sigma: Some(best.sigma.clone()),
        winning_beta_margin: Some(best.beta_margin),
        orbit_count: Some(capacity.orbits.len()),
        sigma_iterations: Some(capacity.iterations),
        geometry_indeterminate_count,
        vertex_indeterminate_count: counts.vertex,
        bounded_near_singular_vertex_count: counts.near_singular,
        ambiguous_vertex_incidence_count: counts.incidence,
        facet_intersection_indeterminate_count: counts.intersection,
        omega_indeterminate_count: counts.omega,
        geometry_ms,
        volume_ms,
        capacity_ms,
        total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PredicateCounts {
    vertex: usize,
    near_singular: usize,
    incidence: usize,
    intersection: usize,
    omega: usize,
}

impl PredicateCounts {
    pub(crate) fn total(&self) -> usize {
        self.vertex + self.near_singular + self.incidence + self.intersection + self.omega
    }
}

pub(crate) fn build_geometry(
    duals: &[Vector4<f64>],
    mode: GeometryMode,
) -> Result<(SysLandscapePolytopeCache, PredicateCounts), String> {
    match mode {
        GeometryMode::Exact => SysLandscapePolytopeCache::from_f64_dual_vertices(duals.to_vec())
            .map(|polytope| (polytope, PredicateCounts::default()))
            .ok_or_else(|| "invalid_exact_geometry".to_string()),
        GeometryMode::F64 => {
            let payload =
                f64_geometry_payload(duals).map_err(|_| "invalid_f64_geometry".to_string())?;
            build_from_f64_payload(duals, payload)
        }
    }
}

fn build_from_f64_payload(
    duals: &[Vector4<f64>],
    payload: F64GeometryPayload,
) -> Result<(SysLandscapePolytopeCache, PredicateCounts), String> {
    let counts = PredicateCounts {
        vertex: payload.vertex_indeterminate_count,
        near_singular: payload.bounded_near_singular_vertex_count,
        incidence: payload.ambiguous_vertex_incidence_count,
        intersection: payload.facet_intersection_indeterminate_count,
        omega: payload.omega_indeterminate_count,
    };
    let exact_duals = exact_binary64_dual_vertex_arrays(duals);
    let exact_vertices = exact_binary64_dual_vertex_arrays(&payload.vertices);
    let polytope = SysLandscapePolytopeCache::from_trusted_parts(
        exact_duals,
        exact_vertices,
        payload.vertex_facet_incidence,
        payload.facet_intersection_is_nonempty,
        payload.omega_signs,
        duals.to_vec(),
        payload.vertices,
    )
    .ok_or_else(|| "invalid_f64_geometry_payload_shape".to_string())?;
    Ok((polytope, counts))
}

pub(crate) fn compute_volume(
    polytope: &SysLandscapePolytopeCache,
    mode: VolumeMode,
) -> Result<f64, String> {
    match mode {
        VolumeMode::Exact => Ok(exact_volume_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        )),
        VolumeMode::F64 => {
            validate_volume_incidence(&polytope.vertex_facet_incidence)?;
            if let Some(facet) = (0..polytope.vertex_facet_incidence.ncols()).find(|&facet| {
                (0..polytope.vertex_facet_incidence.nrows())
                    .filter(|&vertex| polytope.vertex_facet_incidence[(vertex, facet)])
                    .count()
                    < 4
            }) {
                return Err(format!(
                    "invalid_f64_volume_incidence:facet_{facet}_has_fewer_than_four_vertices"
                ));
            }
            std::panic::catch_unwind(AssertUnwindSafe(|| {
                volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
            }))
            .map_err(|_| "f64_volume_decomposition_panicked".to_string())?
            .map_err(|error| format!("f64_volume_failed:{error:?}"))
        }
    }
}

/// Reconstructs the configured geometry and volume without running the orbit
/// search.
///
/// The clean runner supports exact binary64 geometry only. This helper exists
/// for diagnostic evaluation of already named branches; it does not establish
/// that the resulting point has a valid `sys` value.
pub fn reconstruct_geometry_and_volume(
    duals: &[Vector4<f64>],
    config: &EvaluatorConfig,
) -> Result<(SysLandscapePolytopeCache, f64), String> {
    let (polytope, _) = build_geometry(duals, config.geometry_mode)?;
    let volume = compute_volume(&polytope, config.volume_mode)?;
    Ok((polytope, volume))
}

fn validate_volume_incidence(incidence: &nalgebra::DMatrix<bool>) -> Result<(), String> {
    for facet_i in 0..incidence.ncols() {
        for facet_j in facet_i + 1..incidence.ncols() {
            let shared = (0..incidence.nrows())
                .filter(|&vertex| incidence[(vertex, facet_i)] && incidence[(vertex, facet_j)])
                .collect::<Vec<_>>();
            if shared.len() <= 2 {
                continue;
            }
            let degrees = shared
                .iter()
                .map(|&left| {
                    shared
                        .iter()
                        .filter(|&&right| {
                            left != right
                                && (0..incidence.ncols()).any(|facet| {
                                    facet != facet_i
                                        && facet != facet_j
                                        && incidence[(left, facet)]
                                        && incidence[(right, facet)]
                                })
                        })
                        .count()
                })
                .collect::<Vec<_>>();
            if degrees.iter().any(|degree| *degree != 2) {
                return Err(format!(
                    "invalid_f64_volume_incidence:facets_{facet_i}_{facet_j}_degrees_{degrees:?}"
                ));
            }
        }
    }
    Ok(())
}

fn capacity_minima_safe(
    polytope: &SysLandscapePolytopeCache,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    capacity_search(polytope, 0.0, OrbitGuaranteeMode::MinimaSafe)
}

pub(crate) fn capacity_with_action_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    capacity_search(polytope, action_gap, OrbitGuaranteeMode::AllSafe)
}

fn capacity_search(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
    guarantee: OrbitGuaranteeMode,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
    if let Ok(classification) = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
        let (orbits, iterations) = solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &transition,
        )
        .map_err(|_| OrbitSearchError::NumericalFailure)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap,
            guarantee,
        )
    } else {
        let (orbits, iterations) =
            solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition)?;
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap,
            guarantee,
        )
    }
}

fn failed_outcome(error: String, geometry_ms: f64, total_ms: f64) -> CachedOutcome {
    let mut outcome = failed_after_geometry(
        error,
        &PredicateCounts::default(),
        geometry_ms,
        0.0,
        0.0,
        total_ms,
    );
    if outcome
        .error
        .as_deref()
        .is_some_and(|error| error.contains(";exact_geometry:"))
    {
        outcome.geometry_route = "f64_then_exact_failed".to_string();
        outcome.fallback_reason = outcome.error.clone();
    }
    outcome
}

fn failed_after_geometry(
    error: String,
    counts: &PredicateCounts,
    geometry_ms: f64,
    volume_ms: f64,
    capacity_ms: f64,
    total_ms: f64,
) -> CachedOutcome {
    CachedOutcome {
        context: None,
        status: "invalid".to_string(),
        geometry_route: "failed".to_string(),
        fallback_reason: None,
        usable_by_optimizer: false,
        error: Some(error),
        sys: None,
        capacity: None,
        volume: None,
        winning_sigma: None,
        winning_beta_margin: None,
        orbit_count: None,
        sigma_iterations: None,
        geometry_indeterminate_count: counts.total(),
        vertex_indeterminate_count: counts.vertex,
        bounded_near_singular_vertex_count: counts.near_singular,
        ambiguous_vertex_incidence_count: counts.incidence,
        facet_intersection_indeterminate_count: counts.intersection,
        omega_indeterminate_count: counts.omega,
        geometry_ms,
        volume_ms,
        capacity_ms,
        total_ms,
    }
}

#[allow(clippy::too_many_arguments)]
fn failed_with_route(
    error: String,
    counts: &PredicateCounts,
    geometry_route: &str,
    fallback_reason: Option<String>,
    geometry_ms: f64,
    volume_ms: f64,
    capacity_ms: f64,
    total_ms: f64,
) -> CachedOutcome {
    let mut outcome =
        failed_after_geometry(error, counts, geometry_ms, volume_ms, capacity_ms, total_ms);
    outcome.geometry_route = geometry_route.to_string();
    outcome.fallback_reason = fallback_reason.or_else(|| {
        outcome
            .error
            .as_deref()
            .filter(|error| error.contains(";exact_geometry:"))
            .map(str::to_string)
    });
    if outcome.fallback_reason.is_some() && outcome.geometry_route == "f64" {
        outcome.geometry_route = "f64_then_exact_failed".to_string();
    }
    outcome
}

pub fn point_key(duals: &[Vector4<f64>]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"sys-optimizer-study-point-v1");
    for dual in duals {
        for coordinate in dual.iter() {
            hasher.update(&coordinate.to_bits().to_le_bytes());
        }
    }
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluator_computes_cross_polytope() {
        let duals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ];
        let mut evaluator = Evaluator::new(EvaluatorConfig::default());
        let evaluation = evaluator.evaluate("run", "eval".into(), None, "initial", 0, false, duals);
        assert!(evaluation.row.usable_by_optimizer, "{:?}", evaluation.row);
        assert!(evaluation.row.sys.unwrap() > 0.0);
        assert!(evaluation.row.total_ms >= evaluation.row.capacity_ms);
    }
}
