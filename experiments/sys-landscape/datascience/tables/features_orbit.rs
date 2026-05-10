//! Sigma-local orbit and KKT-derived feature columns.

use nalgebra::Vector4;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic_form::omega0;

use crate::load_caches::LoadedPolytopeRow;

use super::features_helpers::{fraction_at_most, stats_or_zero};

pub struct OrbitFields {
    pub orbit_sigma_available: f64,
    pub orbit_sigma_count: f64,
    pub orbit_sigma_gap_cutoff: f64,
    pub orbit_sigma_len: f64,
    pub orbit_sigma_fraction: f64,
    pub orbit_selected_norm_mean: f64,
    pub orbit_selected_norm_std: f64,
    pub orbit_selected_norm_min: f64,
    pub orbit_selected_norm_max: f64,
    pub orbit_cycle_abs_omega_mean: f64,
    pub orbit_cycle_abs_omega_std: f64,
    pub orbit_cycle_abs_omega_min: f64,
    pub orbit_cycle_abs_omega_max: f64,
    pub orbit_cycle_abs_omega_le_1e3_fraction: f64,
    pub orbit_cycle_abs_omega_le_1e2_fraction: f64,
    pub orbit_cycle_abs_omega_le_1e1_fraction: f64,
    pub orbit_cycle_zero_fraction: f64,
    pub orbit_cycle_transition_fraction: f64,
    pub orbit_cycle_bidirectional_fraction: f64,
    pub orbit_cycle_adjacent_fraction: f64,
    pub orbit_selected_out_degree_mean: f64,
    pub orbit_selected_out_degree_std: f64,
    pub orbit_selected_out_degree_min: f64,
    pub orbit_selected_out_degree_max: f64,
    pub orbit_kkt_available: f64,
    pub orbit_search_scalar_available: f64,
    pub orbit_result_iterations_log1p: f64,
    pub orbit_result_returned_orbit_count: f64,
    pub orbit_best_beta_margin: f64,
    pub orbit_best_q_error_bound: f64,
    pub orbit_best_has_mu: f64,
    pub orbit_best_has_xi: f64,
    pub orbit_best_is_admissible_exact: f64,
    pub orbit_best_is_indeterminate_f64: f64,
}

fn zero_orbit_fields() -> OrbitFields {
    OrbitFields {
        orbit_sigma_available: 0.0,
        orbit_sigma_count: 0.0,
        orbit_sigma_gap_cutoff: 0.0,
        orbit_sigma_len: 0.0,
        orbit_sigma_fraction: 0.0,
        orbit_selected_norm_mean: 0.0,
        orbit_selected_norm_std: 0.0,
        orbit_selected_norm_min: 0.0,
        orbit_selected_norm_max: 0.0,
        orbit_cycle_abs_omega_mean: 0.0,
        orbit_cycle_abs_omega_std: 0.0,
        orbit_cycle_abs_omega_min: 0.0,
        orbit_cycle_abs_omega_max: 0.0,
        orbit_cycle_abs_omega_le_1e3_fraction: 0.0,
        orbit_cycle_abs_omega_le_1e2_fraction: 0.0,
        orbit_cycle_abs_omega_le_1e1_fraction: 0.0,
        orbit_cycle_zero_fraction: 0.0,
        orbit_cycle_transition_fraction: 0.0,
        orbit_cycle_bidirectional_fraction: 0.0,
        orbit_cycle_adjacent_fraction: 0.0,
        orbit_selected_out_degree_mean: 0.0,
        orbit_selected_out_degree_std: 0.0,
        orbit_selected_out_degree_min: 0.0,
        orbit_selected_out_degree_max: 0.0,
        orbit_kkt_available: 0.0,
        orbit_search_scalar_available: 0.0,
        orbit_result_iterations_log1p: 0.0,
        orbit_result_returned_orbit_count: 0.0,
        orbit_best_beta_margin: 0.0,
        orbit_best_q_error_bound: 0.0,
        orbit_best_has_mu: 0.0,
        orbit_best_has_xi: 0.0,
        orbit_best_is_admissible_exact: 0.0,
        orbit_best_is_indeterminate_f64: 0.0,
    }
}

pub fn compute_orbit_fields(
    row: &LoadedPolytopeRow,
    polytope: &Polytope4D,
    duals: &[Vector4<f64>],
    facet_count: usize,
    transition: &nalgebra::DMatrix<bool>,
) -> OrbitFields {
    let Some(sigmas) = row.sigmas.as_ref() else {
        return zero_orbit_fields();
    };
    let Some(best_sigma) = sigmas.first() else {
        return zero_orbit_fields();
    };
    let perm = &best_sigma.perm;
    let selected_norms = perm.iter().map(|&facet| duals[facet].norm()).collect::<Vec<_>>();
    let selected_out_degrees = perm
        .iter()
        .map(|&facet| (0..facet_count).filter(|&other| transition[(facet, other)]).count() as f64)
        .collect::<Vec<_>>();
    let mut cycle_abs_omegas = Vec::new();
    let mut cycle_zero_count = 0usize;
    let mut cycle_transition_count = 0usize;
    let mut cycle_bidirectional_count = 0usize;
    let mut cycle_adjacent_count = 0usize;
    if perm.len() >= 2 {
        for idx in 0..perm.len() {
            let i = perm[idx];
            let j = perm[(idx + 1) % perm.len()];
            cycle_abs_omegas.push(omega0(&duals[i], &duals[j]).abs());
            if polytope.omega_signs()[(i, j)] == 0 {
                cycle_zero_count += 1;
            }
            if transition[(i, j)] {
                cycle_transition_count += 1;
            }
            if transition[(i, j)] && transition[(j, i)] {
                cycle_bidirectional_count += 1;
            }
            if polytope.facet_intersection_is_nonempty()[(i, j)] {
                cycle_adjacent_count += 1;
            }
        }
    }
    let (orbit_selected_norm_mean, orbit_selected_norm_std, orbit_selected_norm_min, orbit_selected_norm_max) =
        stats_or_zero(&selected_norms);
    let (orbit_cycle_abs_omega_mean, orbit_cycle_abs_omega_std, orbit_cycle_abs_omega_min, orbit_cycle_abs_omega_max) =
        stats_or_zero(&cycle_abs_omegas);
    let (orbit_selected_out_degree_mean, orbit_selected_out_degree_std, orbit_selected_out_degree_min, orbit_selected_out_degree_max) =
        stats_or_zero(&selected_out_degrees);
    let orbit_scalars = row.orbit_scalars.clone();
    let orbit_search_scalar_available = orbit_scalars
        .as_ref()
        .is_some_and(|scalars| scalars.returned_orbit_count > 0 || scalars.iterations > 0);
    let cycle_len = cycle_abs_omegas.len() as f64;

    OrbitFields {
        orbit_sigma_available: 1.0,
        orbit_sigma_count: sigmas.len() as f64,
        orbit_sigma_gap_cutoff: row.sigma_gap_cutoff.unwrap_or(0.0),
        orbit_sigma_len: perm.len() as f64,
        orbit_sigma_fraction: if facet_count == 0 {
            0.0
        } else {
            perm.len() as f64 / facet_count as f64
        },
        orbit_selected_norm_mean,
        orbit_selected_norm_std,
        orbit_selected_norm_min,
        orbit_selected_norm_max,
        orbit_cycle_abs_omega_mean,
        orbit_cycle_abs_omega_std,
        orbit_cycle_abs_omega_min,
        orbit_cycle_abs_omega_max,
        orbit_cycle_abs_omega_le_1e3_fraction: fraction_at_most(&cycle_abs_omegas, 1e-3),
        orbit_cycle_abs_omega_le_1e2_fraction: fraction_at_most(&cycle_abs_omegas, 1e-2),
        orbit_cycle_abs_omega_le_1e1_fraction: fraction_at_most(&cycle_abs_omegas, 1e-1),
        orbit_cycle_zero_fraction: if cycle_len > 0.0 { cycle_zero_count as f64 / cycle_len } else { 0.0 },
        orbit_cycle_transition_fraction: if cycle_len > 0.0 {
            cycle_transition_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_cycle_bidirectional_fraction: if cycle_len > 0.0 {
            cycle_bidirectional_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_cycle_adjacent_fraction: if cycle_len > 0.0 {
            cycle_adjacent_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std,
        orbit_selected_out_degree_min,
        orbit_selected_out_degree_max,
        orbit_kkt_available: if orbit_scalars.is_some() { 1.0 } else { 0.0 },
        orbit_search_scalar_available: if orbit_search_scalar_available { 1.0 } else { 0.0 },
        orbit_result_iterations_log1p: orbit_scalars
            .as_ref()
            .map(|scalars| (scalars.iterations as f64).ln_1p())
            .unwrap_or(0.0),
        orbit_result_returned_orbit_count: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.returned_orbit_count as f64)
            .unwrap_or(0.0),
        orbit_best_beta_margin: orbit_scalars.as_ref().map(|scalars| scalars.best_beta_margin).unwrap_or(0.0),
        orbit_best_q_error_bound: orbit_scalars.as_ref().map(|scalars| scalars.best_q_error_bound).unwrap_or(0.0),
        orbit_best_has_mu: orbit_scalars.as_ref().map(|scalars| if scalars.best_has_mu { 1.0 } else { 0.0 }).unwrap_or(0.0),
        orbit_best_has_xi: orbit_scalars.as_ref().map(|scalars| if scalars.best_has_xi { 1.0 } else { 0.0 }).unwrap_or(0.0),
        orbit_best_is_admissible_exact: orbit_scalars
            .as_ref()
            .map(|scalars| if scalars.best_is_admissible_exact { 1.0 } else { 0.0 })
            .unwrap_or(0.0),
        orbit_best_is_indeterminate_f64: orbit_scalars
            .as_ref()
            .map(|scalars| if scalars.best_is_indeterminate_f64 { 1.0 } else { 0.0 })
            .unwrap_or(0.0),
    }
}
