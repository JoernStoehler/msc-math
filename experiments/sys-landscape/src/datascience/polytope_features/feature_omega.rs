use nalgebra::{DMatrix, Vector4};
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;

pub struct OmegaFields {
    pub transition: DMatrix<bool>,
    pub allpair_abs_omega_vol1_mean: f64,
    pub allpair_abs_omega_vol1_std: f64,
    pub allpair_abs_omega_vol1_min: f64,
    pub allpair_abs_omega_vol1_max: f64,
    pub allpair_zero_fraction: f64,
    pub ridge_abs_omega_vol1_mean: f64,
    pub ridge_abs_omega_vol1_std: f64,
    pub ridge_abs_omega_vol1_min: f64,
    pub ridge_abs_omega_vol1_max: f64,
    pub ridge_zero_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em3_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em2_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em1_fraction: f64,
    pub transition_density: f64,
    pub transition_bidirectional_fraction: f64,
    pub transition_out_degree_mean: f64,
    pub transition_out_degree_std: f64,
    pub transition_out_degree_min: f64,
    pub transition_out_degree_max: f64,
}

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
}

pub fn compute(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    duals: &[Vector4<f64>],
    facet_count: usize,
    omega_scale: f64,
) -> OmegaFields {
    let mut allpair_abs_omegas = Vec::new();
    let mut allpair_zero_count = 0usize;
    for i in 0..facet_count {
        for j in (i + 1)..facet_count {
            let value = omega0(&duals[i], &duals[j]).abs() * omega_scale;
            if polytope.omega_signs()[(i, j)] == 0 {
                allpair_zero_count += 1;
            }
            allpair_abs_omegas.push(value);
        }
    }
    let ridge_abs_omegas = skeleton
        .ridges
        .iter()
        .map(|ridge| omega0(&duals[ridge.facets[0]], &duals[ridge.facets[1]]).abs() * omega_scale)
        .collect::<Vec<_>>();
    let ridge_zero_count = skeleton
        .ridges
        .iter()
        .filter(|ridge| polytope.omega_signs()[(ridge.facets[0], ridge.facets[1])] == 0)
        .count();
    let transition = build_transition_matrix(polytope);
    let mut transition_true_count = 0usize;
    let mut adjacent_pair_count = 0usize;
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
            if polytope.vertex_adjacency()[(i, j)] {
                adjacent_pair_count += 1;
                if transition[(i, j)] && transition[(j, i)] {
                    bidirectional_pair_count += 1;
                }
            }
        }
    }

    let (allpair_abs_omega_vol1_mean, allpair_abs_omega_vol1_std, allpair_abs_omega_vol1_min, allpair_abs_omega_vol1_max) =
        stats_or_zero(&allpair_abs_omegas);
    let (ridge_abs_omega_vol1_mean, ridge_abs_omega_vol1_std, ridge_abs_omega_vol1_min, ridge_abs_omega_vol1_max) =
        stats_or_zero(&ridge_abs_omegas);
    let (transition_out_degree_mean, transition_out_degree_std, transition_out_degree_min, transition_out_degree_max) =
        stats_or_zero(&out_degrees);
    let total_pairs = (facet_count * (facet_count - 1) / 2) as f64;
    let transition_density = if facet_count >= 2 {
        transition_true_count as f64 / (facet_count * (facet_count - 1)) as f64
    } else {
        0.0
    };
    let transition_bidirectional_fraction = if adjacent_pair_count > 0 {
        bidirectional_pair_count as f64 / adjacent_pair_count as f64
    } else {
        0.0
    };

    OmegaFields {
        transition,
        allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max,
        allpair_zero_fraction: if total_pairs > 0.0 {
            allpair_zero_count as f64 / total_pairs
        } else {
            0.0
        },
        ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max,
        ridge_zero_fraction: if skeleton.ridges.is_empty() {
            0.0
        } else {
            ridge_zero_count as f64 / skeleton.ridges.len() as f64
        },
        ridge_abs_omega_vol1_le_1em3_fraction: fraction_at_most(&ridge_abs_omegas, 1e-3),
        ridge_abs_omega_vol1_le_1em2_fraction: fraction_at_most(&ridge_abs_omegas, 1e-2),
        ridge_abs_omega_vol1_le_1em1_fraction: fraction_at_most(&ridge_abs_omegas, 1e-1),
        transition_density,
        transition_bidirectional_fraction,
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    }
}
