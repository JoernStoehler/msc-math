//! Symplectic-form and transition-graph feature columns.

use euclidean_polytopes::TwoFace;
use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::DMatrix;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::geom::symplectic_form::omega0;

use super::features_helpers::{fraction_at_most, quantile_or_zero, stats_or_zero, top_k_share};

pub struct OmegaFields {
    pub transition: DMatrix<bool>,
    pub omega_matrix_vol1_frobenius_norm: f64,
    pub omega_matrix_vol1_spectral_norm: f64,
    pub omega_matrix_vol1_stable_rank: f64,
    pub omega_matrix_vol1_rank_1em10: f64,
    pub omega_matrix_vol1_nullity_1em10: f64,
    pub omega_sign_out_degree_mean: f64,
    pub omega_sign_out_degree_std: f64,
    pub omega_sign_out_degree_min: f64,
    pub omega_sign_out_degree_max: f64,
    pub allpair_abs_omega_vol1_mean: f64,
    pub allpair_abs_omega_vol1_std: f64,
    pub allpair_abs_omega_vol1_min: f64,
    pub allpair_abs_omega_vol1_max: f64,
    pub allpair_abs_omega_vol1_q25: f64,
    pub allpair_abs_omega_vol1_median: f64,
    pub allpair_abs_omega_vol1_q75: f64,
    pub allpair_abs_omega_vol1_q90: f64,
    pub allpair_abs_omega_vol1_top3_share: f64,
    pub allpair_zero_fraction: f64,
    pub allpair_abs_normalized_omega_mean: f64,
    pub allpair_abs_normalized_omega_std: f64,
    pub allpair_abs_normalized_omega_min: f64,
    pub allpair_abs_normalized_omega_max: f64,
    pub ridge_abs_omega_vol1_mean: f64,
    pub ridge_abs_omega_vol1_std: f64,
    pub ridge_abs_omega_vol1_min: f64,
    pub ridge_abs_omega_vol1_max: f64,
    pub ridge_abs_omega_vol1_q25: f64,
    pub ridge_abs_omega_vol1_median: f64,
    pub ridge_abs_omega_vol1_q75: f64,
    pub ridge_abs_omega_vol1_q90: f64,
    pub ridge_abs_omega_vol1_top3_share: f64,
    pub ridge_zero_fraction: f64,
    pub ridge_abs_normalized_omega_mean: f64,
    pub ridge_abs_normalized_omega_std: f64,
    pub ridge_abs_normalized_omega_min: f64,
    pub ridge_abs_normalized_omega_max: f64,
    pub ridge_abs_omega_vol1_le_1em3_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em2_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em1_fraction: f64,
    pub transition_density: f64,
    pub transition_bidirectional_given_facet_intersection_fraction: f64,
    pub transition_out_degree_mean: f64,
    pub transition_out_degree_std: f64,
    pub transition_out_degree_min: f64,
    pub transition_out_degree_max: f64,
}

pub fn compute_omega_fields(
    polytope: &SysLandscapePolytopeCache,
    two_faces: &[TwoFace],
    duals: &[nalgebra::Vector4<f64>],
    facet_count: usize,
    omega_scale: f64,
) -> OmegaFields {
    let omega_matrix = DMatrix::from_fn(facet_count, facet_count, |i, j| {
        omega0(&duals[i], &duals[j]) * omega_scale
    });
    let omega_svd = omega_matrix.clone().svd(false, false);
    let singular_values = omega_svd
        .singular_values
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let omega_matrix_vol1_frobenius_norm = singular_values
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    let omega_matrix_vol1_spectral_norm = singular_values.iter().copied().fold(0.0, f64::max);
    let omega_matrix_vol1_stable_rank = if omega_matrix_vol1_spectral_norm > 0.0 {
        omega_matrix_vol1_frobenius_norm.powi(2) / omega_matrix_vol1_spectral_norm.powi(2)
    } else {
        0.0
    };
    let omega_matrix_vol1_rank_1em10 = singular_values
        .iter()
        .filter(|&&value| value > 1e-10)
        .count() as f64;
    let omega_matrix_vol1_nullity_1em10 = facet_count as f64 - omega_matrix_vol1_rank_1em10;

    let mut allpair_abs_omegas = Vec::new();
    let mut allpair_abs_normalized_omegas = Vec::new();
    let mut allpair_zero_count = 0usize;
    let mut omega_sign_out_degrees = vec![0.0; facet_count];
    for i in 0..facet_count {
        for j in (i + 1)..facet_count {
            let raw_omega = omega0(&duals[i], &duals[j]);
            let value = raw_omega.abs() * omega_scale;
            let sign = polytope.omega_signs[(i, j)];
            if sign == 0 {
                allpair_zero_count += 1;
            } else if sign > 0 {
                omega_sign_out_degrees[i] += 1.0;
            } else {
                omega_sign_out_degrees[j] += 1.0;
            }
            let denom = duals[i].norm() * duals[j].norm();
            if denom > 0.0 {
                allpair_abs_normalized_omegas.push((raw_omega / denom).abs());
            }
            allpair_abs_omegas.push(value);
        }
    }
    let ridge_abs_omegas = two_faces
        .iter()
        .map(|two_face| {
            omega0(&duals[two_face.facets[0]], &duals[two_face.facets[1]]).abs() * omega_scale
        })
        .collect::<Vec<_>>();
    let ridge_abs_normalized_omegas = two_faces
        .iter()
        .filter_map(|two_face| {
            let i = two_face.facets[0];
            let j = two_face.facets[1];
            let denom = duals[i].norm() * duals[j].norm();
            (denom > 0.0).then(|| (omega0(&duals[i], &duals[j]) / denom).abs())
        })
        .collect::<Vec<_>>();
    let ridge_zero_count = two_faces
        .iter()
        .filter(|two_face| polytope.omega_signs[(two_face.facets[0], two_face.facets[1])] == 0)
        .count();
    let facet_intersection_is_nonempty = &polytope.facet_intersection_is_nonempty;
    let omega_signs = &polytope.omega_signs;
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        facet_intersection_is_nonempty,
        omega_signs,
    );
    let mut transition_true_count = 0usize;
    let mut facet_intersection_pair_count = 0usize;
    let mut bidirectional_pair_count = 0usize;
    let mut out_degrees = Vec::new();
    for i in 0..facet_count {
        let mut out = 0usize;
        for j in 0..facet_count {
            if transition[(i, j)] {
                transition_true_count += 1;
                out += 1;
            }
        }
        out_degrees.push(out as f64);
    }
    for i in 0..facet_count {
        for j in (i + 1)..facet_count {
            if facet_intersection_is_nonempty[(i, j)] {
                facet_intersection_pair_count += 1;
                if transition[(i, j)] && transition[(j, i)] {
                    bidirectional_pair_count += 1;
                }
            }
        }
    }

    let (
        allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max,
    ) = stats_or_zero(&allpair_abs_omegas);
    let (
        allpair_abs_normalized_omega_mean,
        allpair_abs_normalized_omega_std,
        allpair_abs_normalized_omega_min,
        allpair_abs_normalized_omega_max,
    ) = stats_or_zero(&allpair_abs_normalized_omegas);
    let (
        ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max,
    ) = stats_or_zero(&ridge_abs_omegas);
    let (
        ridge_abs_normalized_omega_mean,
        ridge_abs_normalized_omega_std,
        ridge_abs_normalized_omega_min,
        ridge_abs_normalized_omega_max,
    ) = stats_or_zero(&ridge_abs_normalized_omegas);
    let (
        omega_sign_out_degree_mean,
        omega_sign_out_degree_std,
        omega_sign_out_degree_min,
        omega_sign_out_degree_max,
    ) = stats_or_zero(&omega_sign_out_degrees);
    let (
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    ) = stats_or_zero(&out_degrees);
    let total_pairs = (facet_count * (facet_count - 1) / 2) as f64;

    OmegaFields {
        transition,
        omega_matrix_vol1_frobenius_norm,
        omega_matrix_vol1_spectral_norm,
        omega_matrix_vol1_stable_rank,
        omega_matrix_vol1_rank_1em10,
        omega_matrix_vol1_nullity_1em10,
        omega_sign_out_degree_mean,
        omega_sign_out_degree_std,
        omega_sign_out_degree_min,
        omega_sign_out_degree_max,
        allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max,
        allpair_abs_omega_vol1_q25: quantile_or_zero(&allpair_abs_omegas, 0.25),
        allpair_abs_omega_vol1_median: quantile_or_zero(&allpair_abs_omegas, 0.50),
        allpair_abs_omega_vol1_q75: quantile_or_zero(&allpair_abs_omegas, 0.75),
        allpair_abs_omega_vol1_q90: quantile_or_zero(&allpair_abs_omegas, 0.90),
        allpair_abs_omega_vol1_top3_share: top_k_share(&allpair_abs_omegas, 3),
        allpair_zero_fraction: if total_pairs > 0.0 {
            allpair_zero_count as f64 / total_pairs
        } else {
            0.0
        },
        allpair_abs_normalized_omega_mean,
        allpair_abs_normalized_omega_std,
        allpair_abs_normalized_omega_min,
        allpair_abs_normalized_omega_max,
        ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max,
        ridge_abs_omega_vol1_q25: quantile_or_zero(&ridge_abs_omegas, 0.25),
        ridge_abs_omega_vol1_median: quantile_or_zero(&ridge_abs_omegas, 0.50),
        ridge_abs_omega_vol1_q75: quantile_or_zero(&ridge_abs_omegas, 0.75),
        ridge_abs_omega_vol1_q90: quantile_or_zero(&ridge_abs_omegas, 0.90),
        ridge_abs_omega_vol1_top3_share: top_k_share(&ridge_abs_omegas, 3),
        ridge_zero_fraction: if two_faces.is_empty() {
            0.0
        } else {
            ridge_zero_count as f64 / two_faces.len() as f64
        },
        ridge_abs_normalized_omega_mean,
        ridge_abs_normalized_omega_std,
        ridge_abs_normalized_omega_min,
        ridge_abs_normalized_omega_max,
        ridge_abs_omega_vol1_le_1em3_fraction: fraction_at_most(&ridge_abs_omegas, 1e-3),
        ridge_abs_omega_vol1_le_1em2_fraction: fraction_at_most(&ridge_abs_omegas, 1e-2),
        ridge_abs_omega_vol1_le_1em1_fraction: fraction_at_most(&ridge_abs_omegas, 1e-1),
        transition_density: if facet_count >= 2 {
            transition_true_count as f64 / (facet_count * (facet_count - 1)) as f64
        } else {
            0.0
        },
        transition_bidirectional_given_facet_intersection_fraction: if facet_intersection_pair_count
            > 0
        {
            bidirectional_pair_count as f64 / facet_intersection_pair_count as f64
        } else {
            0.0
        },
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    }
}
