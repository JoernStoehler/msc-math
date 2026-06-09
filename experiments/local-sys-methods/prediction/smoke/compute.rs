use super::cache::LocalPolytopeCache;
use super::row::{PredictionError, PredictionRow};
use nalgebra::{DMatrix, Vector4};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::facet_adjacency::is_feasible_cycle;
use symplectic::algorithms::{OrbitKktData, OrbitSearchResult};
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::geom::known_polytopes;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_orbit_sigma_saddle_point,
    solve_pruned_hk2017_candidates, systolic_ratio, OrbitGuaranteeMode,
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

pub(super) fn prediction_rows() -> Result<Vec<PredictionRow>, PredictionError> {
    let basepoints = vec![
        (
            "random_f10_seed_5a51_2026".to_string(),
            LocalPolytopeCache::generate_random(
                RANDOM_FACET_COUNT,
                RANDOM_H_MIN,
                RANDOM_H_MAX,
                RANDOM_BASEPOINT_MASTER_SEED,
            )
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

fn compute_base_state(
    polytope: LocalPolytopeCache,
    action_gap: f64,
) -> Result<BaseState, PredictionError> {
    let capacity = compute_capacity_result(&polytope, action_gap)?;
    let volume = polytope.volume();
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
    // This is a deterministic probe direction, not "the gradient" at a
    // multi-active basepoint. The row prediction still uses the full Clarke
    // subdifferential through `clarke_directional_derivative_a`.
    let first_returned_gradient = base
        .sys_subgradients
        .first()
        .ok_or_else(|| PredictionError::Derivative("empty sys subgradient set".to_string()))?;
    let gradient_direction = normalize_direction(first_returned_gradient).ok_or_else(|| {
        PredictionError::Derivative("zero first returned active gradient".to_string())
    })?;
    let negative_gradient_direction: Vec<Vector4<f64>> =
        gradient_direction.iter().map(|v| -*v).collect();

    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_DIRECTION_SEED);
    let random_a = random_unit_direction(base.polytope.facet_count(), &mut rng)
        .ok_or_else(|| PredictionError::Derivative("random direction A was zero".to_string()))?;
    let random_b = random_unit_direction(base.polytope.facet_count(), &mut rng)
        .ok_or_else(|| PredictionError::Derivative("random direction B was zero".to_string()))?;

    Ok(vec![
        (
            "first_returned_active_gradient".to_string(),
            gradient_direction,
        ),
        (
            "negative_first_returned_active_gradient".to_string(),
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
        target_best_sigma_in_base_active_set: None,
        target_best_sigma_in_base_candidate_window: None,
        target_best_sigma_base_transition_allowed: None,
        target_best_sigma_base_solve_status: None,
        target_best_sigma_base_action_gap: None,
        target_best_sigma_transitions_opened: None,
        active_orbit_count: base.active_orbits.len(),
        base_candidate_orbit_count: base.capacity.orbits.len(),
        base_candidate_action_gap: base.candidate_action_gap,
        active_action_spread: base.action_spread,
        active_min_beta_margin: base.active_min_beta_margin,
        active_max_q_error_bound: base.active_max_q_error_bound,
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
    let target_volume = target_polytope.volume();
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
    let base_transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &base.polytope.facet_intersection_is_nonempty,
        &base.polytope.omega_signs,
    );
    row.target_best_sigma_base_transition_allowed = Some(is_feasible_cycle(
        &target_best_sigma,
        &base_transition_is_allowed,
    ));
    let target_transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &target_polytope.facet_intersection_is_nonempty,
        &target_polytope.omega_signs,
    );
    row.target_best_sigma_transitions_opened = Some(transitions_rejected_then_allowed(
        &target_best_sigma,
        &base_transition_is_allowed,
        &target_transition_is_allowed,
    ));
    match solve_orbit_sigma_saddle_point(&base.polytope.dual_vertices_f64, &target_best_sigma) {
        Ok(base_target_orbit) => {
            row.target_best_sigma_base_solve_status =
                Some(format!("ok:{:?}", base_target_orbit.admissibility));
            row.target_best_sigma_base_action_gap =
                Some(base_target_orbit.action - base.capacity.min_action);
        }
        Err(err) => {
            row.target_best_sigma_base_solve_status = Some(format!("{err:?}"));
        }
    }
    row.target_best_sigma = Some(target_best_sigma);
    Ok(row)
}

fn transitions_rejected_then_allowed(
    sigma: &[usize],
    base_transition_is_allowed: &DMatrix<bool>,
    target_transition_is_allowed: &DMatrix<bool>,
) -> Vec<[usize; 2]> {
    sigma
        .iter()
        .copied()
        .zip(sigma.iter().copied().cycle().skip(1))
        .take(sigma.len())
        .filter(|&(from, to)| {
            !base_transition_is_allowed[(from, to)] && target_transition_is_allowed[(from, to)]
        })
        .map(|(from, to)| [from, to])
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_helper_detects_opened_edges() {
        let base = DMatrix::from_row_slice(
            3,
            3,
            &[true, false, true, true, true, true, false, true, true],
        );
        let target = DMatrix::from_row_slice(
            3,
            3,
            &[true, true, true, true, true, true, false, true, true],
        );
        let sigma = vec![0, 1, 2];

        assert_eq!(
            transitions_rejected_then_allowed(&sigma, &base, &target),
            vec![[0, 1]]
        );
    }

    #[test]
    #[ignore = "runs live HK2017 recomputation on HKO; use the smoke binary for routine checks"]
    fn smoke_rows_cover_generic_and_hko_basepoints() {
        let random = LocalPolytopeCache::generate_random(
            RANDOM_FACET_COUNT,
            RANDOM_H_MIN,
            RANDOM_H_MAX,
            RANDOM_BASEPOINT_MASTER_SEED,
        )
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
                assert!(row.target_best_sigma_base_transition_allowed.is_some());
                assert!(row.target_best_sigma_base_solve_status.is_some());
                assert!(row.target_best_sigma_transitions_opened.is_some());
            }
        }
    }
}
