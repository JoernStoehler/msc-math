//! Build the enriched polytope table from unified producer rows.

use crate::load_caches::LoadedPolytopeRow;
use crate::rows::PolytopeTableRow;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::algorithms::solve_orbit_sigma;
use symplectic::database::OrbitScalars;
use symplectic::ehz_capacity;
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::{OrbitAdmissibility, OrbitSolveBackend};

struct GeometryFields {
    geom_vol1_norm_mean: f64,
    geom_vol1_norm_std: f64,
    geom_vol1_norm_min: f64,
    geom_vol1_norm_max: f64,
    geom_vol1_centroid_norm: f64,
    geom_vol1_coord_std_x: f64,
    geom_vol1_coord_std_y: f64,
    geom_vol1_coord_std_z: f64,
    geom_vol1_coord_std_w: f64,
    geom_cosine_mean: f64,
    geom_cosine_std: f64,
    geom_cosine_min: f64,
    geom_cosine_max: f64,
    geom_vol1_pairwise_dist_mean: f64,
    geom_vol1_pairwise_dist_std: f64,
    geom_vol1_pairwise_dist_min: f64,
    geom_vol1_pairwise_dist_max: f64,
    geom_vol1_sval_1: f64,
    geom_vol1_sval_2: f64,
    geom_vol1_sval_3: f64,
    geom_vol1_sval_4: f64,
}

struct DualVertexFields {
    dual_vertex_count: usize,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_flat_f64: Vec<f64>,
}

struct SkeletonFields {
    vertex_count: usize,
    edge_count: usize,
    ridge_count: usize,
    is_simple: bool,
    simple_vertex_fraction: f64,
    edge_density: f64,
    vertex_incident_facets_mean: f64,
    vertex_incident_facets_std: f64,
    vertex_incident_facets_min: f64,
    vertex_incident_facets_max: f64,
    vertex_degree_mean: f64,
    vertex_degree_std: f64,
    vertex_degree_min: f64,
    vertex_degree_max: f64,
    ridge_size_mean: f64,
    ridge_size_std: f64,
    ridge_size_min: f64,
    ridge_size_max: f64,
    facet_vertex_count_mean: f64,
    facet_vertex_count_std: f64,
    facet_vertex_count_min: f64,
    facet_vertex_count_max: f64,
    facet_neighbor_count_mean: f64,
    facet_neighbor_count_std: f64,
    facet_neighbor_count_min: f64,
    facet_neighbor_count_max: f64,
}

struct FaceGeometryFields {
    edge_length_vol1_mean: f64,
    edge_length_vol1_std: f64,
    edge_length_vol1_min: f64,
    edge_length_vol1_max: f64,
    edge_length_vol1_max_share: f64,
    facet_volume_vol1_mean: f64,
    facet_volume_vol1_std: f64,
    facet_volume_vol1_min: f64,
    facet_volume_vol1_max: f64,
    facet_volume_vol1_sum: f64,
    facet_volume_vol1_max_share: f64,
}

struct FaceSymplecticFields {
    ridge_symp_area_volnorm_mean: f64,
    ridge_symp_area_volnorm_std: f64,
    ridge_symp_area_volnorm_min: f64,
    ridge_symp_area_volnorm_max: f64,
    ridge_symp_area_volnorm_sum: f64,
    ridge_symp_area_volnorm_max_share: f64,
    ridge_symp_area_volnorm_zero_fraction: f64,
    ridge_symp_area_volnorm_le_1em3_fraction: f64,
    ridge_symp_area_volnorm_le_1em2_fraction: f64,
    ridge_symp_area_volnorm_le_1em1_fraction: f64,
}

struct OmegaFields {
    transition: DMatrix<bool>,
    allpair_abs_omega_vol1_mean: f64,
    allpair_abs_omega_vol1_std: f64,
    allpair_abs_omega_vol1_min: f64,
    allpair_abs_omega_vol1_max: f64,
    allpair_zero_fraction: f64,
    ridge_abs_omega_vol1_mean: f64,
    ridge_abs_omega_vol1_std: f64,
    ridge_abs_omega_vol1_min: f64,
    ridge_abs_omega_vol1_max: f64,
    ridge_zero_fraction: f64,
    ridge_abs_omega_vol1_le_1em3_fraction: f64,
    ridge_abs_omega_vol1_le_1em2_fraction: f64,
    ridge_abs_omega_vol1_le_1em1_fraction: f64,
    transition_density: f64,
    transition_bidirectional_fraction: f64,
    transition_out_degree_mean: f64,
    transition_out_degree_std: f64,
    transition_out_degree_min: f64,
    transition_out_degree_max: f64,
}

struct OrbitFields {
    orbit_sigma_available: f64,
    orbit_sigma_count: f64,
    orbit_sigma_gap_cutoff: f64,
    orbit_sigma_len: f64,
    orbit_sigma_fraction: f64,
    orbit_selected_norm_mean: f64,
    orbit_selected_norm_std: f64,
    orbit_selected_norm_min: f64,
    orbit_selected_norm_max: f64,
    orbit_cycle_abs_omega_mean: f64,
    orbit_cycle_abs_omega_std: f64,
    orbit_cycle_abs_omega_min: f64,
    orbit_cycle_abs_omega_max: f64,
    orbit_cycle_abs_omega_le_1e3_fraction: f64,
    orbit_cycle_abs_omega_le_1e2_fraction: f64,
    orbit_cycle_abs_omega_le_1e1_fraction: f64,
    orbit_cycle_zero_fraction: f64,
    orbit_cycle_transition_fraction: f64,
    orbit_cycle_bidirectional_fraction: f64,
    orbit_cycle_adjacent_fraction: f64,
    orbit_selected_out_degree_mean: f64,
    orbit_selected_out_degree_std: f64,
    orbit_selected_out_degree_min: f64,
    orbit_selected_out_degree_max: f64,
    orbit_kkt_available: f64,
    orbit_search_scalar_available: f64,
    orbit_result_iterations_log1p: f64,
    orbit_result_returned_orbit_count: f64,
    orbit_best_beta_margin: f64,
    orbit_best_q_error_bound: f64,
    orbit_best_has_mu: f64,
    orbit_best_has_xi: f64,
    orbit_best_is_admissible_exact: f64,
    orbit_best_is_indeterminate_f64: f64,
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
}

fn parse_rational(token: &str) -> BigRational {
    token.parse()
        .unwrap_or_else(|e| panic!("parse rational {token}: {e}"))
}

fn dual_vertices_big(row: &LoadedPolytopeRow) -> Vec<[BigRational; 4]> {
    row.dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| parse_rational(&vertex[i])))
        .collect()
}

fn dual_vertices_f64(row: &LoadedPolytopeRow) -> (Vec<Vector4<f64>>, DualVertexFields) {
    let rationals = dual_vertices_big(row);
    let arrays = rationals
        .iter()
        .map(|vertex| std::array::from_fn(|i| rational_to_f64(&vertex[i])))
        .collect::<Vec<_>>();
    let vectors = arrays
        .iter()
        .map(|vertex| Vector4::from_row_slice(vertex))
        .collect::<Vec<_>>();
    let flat = arrays
        .iter()
        .flat_map(|vertex| vertex.iter().copied())
        .collect::<Vec<_>>();
    (
        vectors,
        DualVertexFields {
            dual_vertex_count: arrays.len(),
            dual_vertices_f64: arrays,
            dual_vertices_flat_f64: flat,
        },
    )
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

fn stats3_or_zero(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
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
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), max)
}

fn max_share(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    if total <= 0.0 {
        0.0
    } else {
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max) / total
    }
}

fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
    }
}

fn centered_singular_values(vertices: &[[f64; 4]]) -> [f64; 4] {
    let arr = nalgebra::DMatrix::from_fn(vertices.len(), 4, |r, c| vertices[r][c]);
    let mut centered = arr.clone();
    for c in 0..4 {
        let mean = (0..vertices.len()).map(|r| arr[(r, c)]).sum::<f64>() / vertices.len() as f64;
        for r in 0..vertices.len() {
            centered[(r, c)] -= mean;
        }
    }
    let svd = centered.svd(false, false);
    let mut out = [0.0; 4];
    for (idx, value) in svd.singular_values.iter().take(4).enumerate() {
        out[idx] = *value;
    }
    out
}

fn compute_geometry_fields(dual_vertices: &[[f64; 4]], volume_value: f64) -> GeometryFields {
    let scale = volume_value.powf(0.25);
    let scaled = dual_vertices
        .iter()
        .map(|vertex| std::array::from_fn(|i| vertex[i] * scale))
        .collect::<Vec<[f64; 4]>>();
    let norms = scaled
        .iter()
        .map(|vertex| vertex.iter().map(|coord| coord * coord).sum::<f64>().sqrt())
        .collect::<Vec<_>>();
    let centroid: [f64; 4] =
        std::array::from_fn(|i| scaled.iter().map(|vertex| vertex[i]).sum::<f64>() / scaled.len() as f64);
    let centroid_norm = centroid.iter().map(|coord| coord * coord).sum::<f64>().sqrt();
    let coord_std: [f64; 4] = std::array::from_fn(|i| {
        let mean = centroid[i];
        let var = scaled
            .iter()
            .map(|vertex| {
                let delta = vertex[i] - mean;
                delta * delta
            })
            .sum::<f64>()
            / scaled.len() as f64;
        var.sqrt()
    });
    let mut cosines = Vec::new();
    let mut pairwise_distances = Vec::new();
    for i in 0..scaled.len() {
        for j in (i + 1)..scaled.len() {
            let dot = (0..4).map(|k| scaled[i][k] * scaled[j][k]).sum::<f64>();
            let denom = norms[i] * norms[j];
            if denom > 0.0 {
                cosines.push(dot / denom);
            }
            pairwise_distances.push(
                (0..4)
                    .map(|k| {
                        let delta = scaled[i][k] - scaled[j][k];
                        delta * delta
                    })
                    .sum::<f64>()
                    .sqrt(),
            );
        }
    }
    let (geom_vol1_norm_mean, geom_vol1_norm_std, geom_vol1_norm_min, geom_vol1_norm_max) =
        stats_or_zero(&norms);
    let (geom_cosine_mean, geom_cosine_std, geom_cosine_min, geom_cosine_max) =
        stats_or_zero(&cosines);
    let (
        geom_vol1_pairwise_dist_mean,
        geom_vol1_pairwise_dist_std,
        geom_vol1_pairwise_dist_min,
        geom_vol1_pairwise_dist_max,
    ) = stats_or_zero(&pairwise_distances);
    let singular_values = centered_singular_values(&scaled);

    GeometryFields {
        geom_vol1_norm_mean,
        geom_vol1_norm_std,
        geom_vol1_norm_min,
        geom_vol1_norm_max,
        geom_vol1_centroid_norm: centroid_norm,
        geom_vol1_coord_std_x: coord_std[0],
        geom_vol1_coord_std_y: coord_std[1],
        geom_vol1_coord_std_z: coord_std[2],
        geom_vol1_coord_std_w: coord_std[3],
        geom_cosine_mean,
        geom_cosine_std,
        geom_cosine_min,
        geom_cosine_max,
        geom_vol1_pairwise_dist_mean,
        geom_vol1_pairwise_dist_std,
        geom_vol1_pairwise_dist_min,
        geom_vol1_pairwise_dist_max,
        geom_vol1_sval_1: singular_values[0],
        geom_vol1_sval_2: singular_values[1],
        geom_vol1_sval_3: singular_values[2],
        geom_vol1_sval_4: singular_values[3],
    }
}

fn compute_skeleton_fields(polytope: &Polytope4D, skeleton: &Skeleton, facet_count: usize) -> SkeletonFields {
    let vertex_count = polytope.vertices().len();
    let edge_count = skeleton.edges.len();
    let ridge_count = skeleton.ridges.len();
    let vertex_incident_facets = skeleton
        .vertex_facets
        .iter()
        .map(|facets| facets.len() as f64)
        .collect::<Vec<_>>();
    let simple_vertices = vertex_incident_facets
        .iter()
        .filter(|&&count| (count - 4.0).abs() < f64::EPSILON)
        .count();
    let simple_vertex_fraction = if vertex_count == 0 {
        0.0
    } else {
        simple_vertices as f64 / vertex_count as f64
    };
    let mut vertex_degrees = vec![0usize; vertex_count];
    for edge in &skeleton.edges {
        vertex_degrees[edge[0]] += 1;
        vertex_degrees[edge[1]] += 1;
    }
    let vertex_degrees = vertex_degrees
        .into_iter()
        .map(|degree| degree as f64)
        .collect::<Vec<_>>();
    let ridge_sizes = skeleton
        .ridges
        .iter()
        .map(|ridge| ridge.vertices.len() as f64)
        .collect::<Vec<_>>();
    let mut facet_vertex_counts = vec![0usize; facet_count];
    for facets in &skeleton.vertex_facets {
        for &facet in facets {
            facet_vertex_counts[facet] += 1;
        }
    }
    let facet_vertex_counts = facet_vertex_counts
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let facet_neighbor_counts = (0..facet_count)
        .map(|facet| {
            (0..facet_count)
                .filter(|&other| polytope.vertex_adjacency()[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();
    let edge_density = if vertex_count >= 2 {
        (2.0 * edge_count as f64) / ((vertex_count * (vertex_count - 1)) as f64)
    } else {
        0.0
    };
    let (
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
    ) = stats_or_zero(&vertex_incident_facets);
    let (vertex_degree_mean, vertex_degree_std, vertex_degree_min, vertex_degree_max) =
        stats_or_zero(&vertex_degrees);
    let (ridge_size_mean, ridge_size_std, ridge_size_min, ridge_size_max) =
        stats_or_zero(&ridge_sizes);
    let (
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
    ) = stats_or_zero(&facet_vertex_counts);
    let (
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    ) = stats_or_zero(&facet_neighbor_counts);

    SkeletonFields {
        vertex_count,
        edge_count,
        ridge_count,
        is_simple: simple_vertices == vertex_count,
        simple_vertex_fraction,
        edge_density,
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
        vertex_degree_mean,
        vertex_degree_std,
        vertex_degree_min,
        vertex_degree_max,
        ridge_size_mean,
        ridge_size_std,
        ridge_size_min,
        ridge_size_max,
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    }
}

fn compute_face_geometry_fields(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    vertices: &[Vector4<f64>],
    facet_count: usize,
    linear_scale: f64,
    facet_scale: f64,
) -> FaceGeometryFields {
    let edge_lengths = skeleton
        .edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm() / linear_scale)
        .collect::<Vec<_>>();
    let facet_volumes = (0..facet_count)
        .map(|facet| facet_volume_3d(polytope, facet) / facet_scale)
        .collect::<Vec<_>>();
    let (edge_length_vol1_mean, edge_length_vol1_std, edge_length_vol1_min, edge_length_vol1_max) =
        stats_or_zero(&edge_lengths);
    let (facet_volume_vol1_mean, facet_volume_vol1_std, facet_volume_vol1_min, facet_volume_vol1_max) =
        stats_or_zero(&facet_volumes);

    FaceGeometryFields {
        edge_length_vol1_mean,
        edge_length_vol1_std,
        edge_length_vol1_min,
        edge_length_vol1_max,
        edge_length_vol1_max_share: max_share(&edge_lengths),
        facet_volume_vol1_mean,
        facet_volume_vol1_std,
        facet_volume_vol1_min,
        facet_volume_vol1_max,
        facet_volume_vol1_sum: facet_volumes.iter().sum::<f64>(),
        facet_volume_vol1_max_share: max_share(&facet_volumes),
    }
}

fn ridge_symplectic_area(vertices: &[Vector4<f64>]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }
    let doubled_area = (0..vertices.len())
        .map(|idx| {
            let next = (idx + 1) % vertices.len();
            omega0(&vertices[idx], &vertices[next])
        })
        .sum::<f64>();
    0.5 * doubled_area.abs()
}

fn compute_face_symplectic_fields(
    skeleton: &Skeleton,
    vertices: &[Vector4<f64>],
    volume_scale: f64,
) -> FaceSymplecticFields {
    let ridge_symp_areas = skeleton
        .ridges
        .iter()
        .map(|ridge| {
            let ridge_vertices = ridge
                .vertices
                .iter()
                .map(|&vertex| vertices[vertex])
                .collect::<Vec<_>>();
            ridge_symplectic_area(&ridge_vertices) / volume_scale
        })
        .collect::<Vec<_>>();
    let (
        ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max,
    ) = stats_or_zero(&ridge_symp_areas);

    FaceSymplecticFields {
        ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max,
        ridge_symp_area_volnorm_sum: ridge_symp_areas.iter().sum::<f64>(),
        ridge_symp_area_volnorm_max_share: max_share(&ridge_symp_areas),
        ridge_symp_area_volnorm_zero_fraction: fraction_at_most(&ridge_symp_areas, 1e-12),
        ridge_symp_area_volnorm_le_1em3_fraction: fraction_at_most(&ridge_symp_areas, 1e-3),
        ridge_symp_area_volnorm_le_1em2_fraction: fraction_at_most(&ridge_symp_areas, 1e-2),
        ridge_symp_area_volnorm_le_1em1_fraction: fraction_at_most(&ridge_symp_areas, 1e-1),
    }
}

fn compute_omega_fields(
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
        transition_density: if facet_count >= 2 {
            transition_true_count as f64 / (facet_count * (facet_count - 1)) as f64
        } else {
            0.0
        },
        transition_bidirectional_fraction: if adjacent_pair_count > 0 {
            bidirectional_pair_count as f64 / adjacent_pair_count as f64
        } else {
            0.0
        },
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    }
}

fn fallback_orbit_scalars(polytope: &Polytope4D, row: &LoadedPolytopeRow) -> Option<OrbitScalars> {
    let best_sigma = row.sigmas.as_ref()?.first()?;
    let orbit = solve_orbit_sigma(polytope, &best_sigma.perm, OrbitSolveBackend::SaddlePoint).ok()?;
    Some(OrbitScalars {
        iterations: 0,
        returned_orbit_count: 0,
        best_beta_margin: orbit.beta_margin,
        best_q_error_bound: orbit.q_error_bound,
        best_has_mu: orbit.mu.is_some(),
        best_has_xi: orbit.xi.is_some(),
        best_is_admissible_exact: matches!(orbit.admissibility, OrbitAdmissibility::AdmissibleExact),
        best_is_indeterminate_f64: matches!(orbit.admissibility, OrbitAdmissibility::IndeterminateF64),
    })
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

fn compute_orbit_fields(
    row: &LoadedPolytopeRow,
    polytope: &Polytope4D,
    duals: &[Vector4<f64>],
    facet_count: usize,
    transition: &DMatrix<bool>,
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
            if polytope.vertex_adjacency()[(i, j)] {
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
    let orbit_scalars = row
        .orbit_scalars
        .clone()
        .or_else(|| fallback_orbit_scalars(polytope, row));
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
        orbit_cycle_transition_fraction: if cycle_len > 0.0 { cycle_transition_count as f64 / cycle_len } else { 0.0 },
        orbit_cycle_bidirectional_fraction: if cycle_len > 0.0 { cycle_bidirectional_count as f64 / cycle_len } else { 0.0 },
        orbit_cycle_adjacent_fraction: if cycle_len > 0.0 { cycle_adjacent_count as f64 / cycle_len } else { 0.0 },
        orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std,
        orbit_selected_out_degree_min,
        orbit_selected_out_degree_max,
        orbit_kkt_available: if orbit_scalars.is_some() { 1.0 } else { 0.0 },
        orbit_search_scalar_available: if orbit_search_scalar_available { 1.0 } else { 0.0 },
        orbit_result_iterations_log1p: orbit_scalars.as_ref().map(|scalars| (scalars.iterations as f64).ln_1p()).unwrap_or(0.0),
        orbit_result_returned_orbit_count: orbit_scalars.as_ref().map(|scalars| scalars.returned_orbit_count as f64).unwrap_or(0.0),
        orbit_best_beta_margin: orbit_scalars.as_ref().map(|scalars| scalars.best_beta_margin).unwrap_or(0.0),
        orbit_best_q_error_bound: orbit_scalars.as_ref().map(|scalars| scalars.best_q_error_bound).unwrap_or(0.0),
        orbit_best_has_mu: orbit_scalars.as_ref().map(|scalars| if scalars.best_has_mu { 1.0 } else { 0.0 }).unwrap_or(0.0),
        orbit_best_has_xi: orbit_scalars.as_ref().map(|scalars| if scalars.best_has_xi { 1.0 } else { 0.0 }).unwrap_or(0.0),
        orbit_best_is_admissible_exact: orbit_scalars.as_ref().map(|scalars| if scalars.best_is_admissible_exact { 1.0 } else { 0.0 }).unwrap_or(0.0),
        orbit_best_is_indeterminate_f64: orbit_scalars.as_ref().map(|scalars| if scalars.best_is_indeterminate_f64 { 1.0 } else { 0.0 }).unwrap_or(0.0),
    }
}

fn enrich_row(row: &LoadedPolytopeRow) -> PolytopeTableRow {
    let (dual_vectors, dual_vertex_fields) = dual_vertices_f64(row);
    let polytope = Polytope4D::from_f64(dual_vectors.clone())
        .unwrap_or_else(|e| panic!("reconstruct {}: {e}", row.poly_id));
    let polytope_volume = volume(&polytope);
    let actual_capacity = if row.capacity > 0.0 {
        row.capacity
    } else {
        ehz_capacity(&polytope)
            .unwrap_or_else(|e| panic!("capacity {}: {:?}", row.poly_id, e))
            .capacity()
    };
    let sys_value = actual_capacity * actual_capacity / (2.0 * polytope_volume);
    let facet_count = polytope.facet_count();
    let geometry_fields = compute_geometry_fields(&dual_vertex_fields.dual_vertices_f64, polytope_volume);
    let linear_scale = polytope_volume.powf(0.25);
    let facet_scale = polytope_volume.powf(0.75);
    let omega_scale = polytope_volume.sqrt();
    let volume_scale = polytope_volume.sqrt();
    let skeleton = Skeleton::compute(&polytope);
    let vertices = polytope.vertices_f64();
    let duals = polytope.dual_vertices_f64();
    let skeleton_fields = compute_skeleton_fields(&polytope, &skeleton, facet_count);
    let face_geometry_fields = compute_face_geometry_fields(
        &polytope,
        &skeleton,
        &vertices,
        facet_count,
        linear_scale,
        facet_scale,
    );
    let face_symplectic_fields =
        compute_face_symplectic_fields(&skeleton, &vertices, volume_scale);
    let omega_fields = compute_omega_fields(&polytope, &skeleton, &duals, facet_count, omega_scale);
    let orbit_fields = compute_orbit_fields(row, &polytope, &duals, facet_count, &omega_fields.transition);

    PolytopeTableRow {
        poly_id: row.poly_id.clone(),
        dual_vertices_rational: row.dual_vertices_rational.clone(),
        facet_count,
        capacity: actual_capacity,
        capacity_iterations: row.capacity_iterations,
        capacity_source: row.capacity_source.clone(),
        volume: polytope_volume,
        sys: sys_value,
        sigma_gap_cutoff: row.sigma_gap_cutoff,
        sigmas: row.sigmas.clone(),
        raw_orbit_scalars: row.orbit_scalars.clone(),
        geom_vol1_norm_mean: geometry_fields.geom_vol1_norm_mean,
        geom_vol1_norm_std: geometry_fields.geom_vol1_norm_std,
        geom_vol1_norm_min: geometry_fields.geom_vol1_norm_min,
        geom_vol1_norm_max: geometry_fields.geom_vol1_norm_max,
        geom_vol1_centroid_norm: geometry_fields.geom_vol1_centroid_norm,
        geom_vol1_coord_std_x: geometry_fields.geom_vol1_coord_std_x,
        geom_vol1_coord_std_y: geometry_fields.geom_vol1_coord_std_y,
        geom_vol1_coord_std_z: geometry_fields.geom_vol1_coord_std_z,
        geom_vol1_coord_std_w: geometry_fields.geom_vol1_coord_std_w,
        geom_cosine_mean: geometry_fields.geom_cosine_mean,
        geom_cosine_std: geometry_fields.geom_cosine_std,
        geom_cosine_min: geometry_fields.geom_cosine_min,
        geom_cosine_max: geometry_fields.geom_cosine_max,
        geom_vol1_pairwise_dist_mean: geometry_fields.geom_vol1_pairwise_dist_mean,
        geom_vol1_pairwise_dist_std: geometry_fields.geom_vol1_pairwise_dist_std,
        geom_vol1_pairwise_dist_min: geometry_fields.geom_vol1_pairwise_dist_min,
        geom_vol1_pairwise_dist_max: geometry_fields.geom_vol1_pairwise_dist_max,
        geom_vol1_sval_1: geometry_fields.geom_vol1_sval_1,
        geom_vol1_sval_2: geometry_fields.geom_vol1_sval_2,
        geom_vol1_sval_3: geometry_fields.geom_vol1_sval_3,
        geom_vol1_sval_4: geometry_fields.geom_vol1_sval_4,
        dual_vertex_count: dual_vertex_fields.dual_vertex_count,
        dual_vertices_f64: dual_vertex_fields.dual_vertices_f64,
        dual_vertices_flat_f64: dual_vertex_fields.dual_vertices_flat_f64,
        vertex_count: skeleton_fields.vertex_count,
        edge_count: skeleton_fields.edge_count,
        ridge_count: skeleton_fields.ridge_count,
        is_simple: skeleton_fields.is_simple,
        simple_vertex_fraction: skeleton_fields.simple_vertex_fraction,
        edge_density: skeleton_fields.edge_density,
        vertex_incident_facets_mean: skeleton_fields.vertex_incident_facets_mean,
        vertex_incident_facets_std: skeleton_fields.vertex_incident_facets_std,
        vertex_incident_facets_min: skeleton_fields.vertex_incident_facets_min,
        vertex_incident_facets_max: skeleton_fields.vertex_incident_facets_max,
        vertex_degree_mean: skeleton_fields.vertex_degree_mean,
        vertex_degree_std: skeleton_fields.vertex_degree_std,
        vertex_degree_min: skeleton_fields.vertex_degree_min,
        vertex_degree_max: skeleton_fields.vertex_degree_max,
        ridge_size_mean: skeleton_fields.ridge_size_mean,
        ridge_size_std: skeleton_fields.ridge_size_std,
        ridge_size_min: skeleton_fields.ridge_size_min,
        ridge_size_max: skeleton_fields.ridge_size_max,
        facet_vertex_count_mean: skeleton_fields.facet_vertex_count_mean,
        facet_vertex_count_std: skeleton_fields.facet_vertex_count_std,
        facet_vertex_count_min: skeleton_fields.facet_vertex_count_min,
        facet_vertex_count_max: skeleton_fields.facet_vertex_count_max,
        facet_neighbor_count_mean: skeleton_fields.facet_neighbor_count_mean,
        facet_neighbor_count_std: skeleton_fields.facet_neighbor_count_std,
        facet_neighbor_count_min: skeleton_fields.facet_neighbor_count_min,
        facet_neighbor_count_max: skeleton_fields.facet_neighbor_count_max,
        edge_length_vol1_mean: face_geometry_fields.edge_length_vol1_mean,
        edge_length_vol1_std: face_geometry_fields.edge_length_vol1_std,
        edge_length_vol1_min: face_geometry_fields.edge_length_vol1_min,
        edge_length_vol1_max: face_geometry_fields.edge_length_vol1_max,
        edge_length_vol1_max_share: face_geometry_fields.edge_length_vol1_max_share,
        facet_volume_vol1_mean: face_geometry_fields.facet_volume_vol1_mean,
        facet_volume_vol1_std: face_geometry_fields.facet_volume_vol1_std,
        facet_volume_vol1_min: face_geometry_fields.facet_volume_vol1_min,
        facet_volume_vol1_max: face_geometry_fields.facet_volume_vol1_max,
        facet_volume_vol1_sum: face_geometry_fields.facet_volume_vol1_sum,
        facet_volume_vol1_max_share: face_geometry_fields.facet_volume_vol1_max_share,
        ridge_symp_area_volnorm_mean: face_symplectic_fields.ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std: face_symplectic_fields.ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min: face_symplectic_fields.ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max: face_symplectic_fields.ridge_symp_area_volnorm_max,
        ridge_symp_area_volnorm_sum: face_symplectic_fields.ridge_symp_area_volnorm_sum,
        ridge_symp_area_volnorm_max_share: face_symplectic_fields.ridge_symp_area_volnorm_max_share,
        ridge_symp_area_volnorm_zero_fraction: face_symplectic_fields.ridge_symp_area_volnorm_zero_fraction,
        ridge_symp_area_volnorm_le_1em3_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em3_fraction,
        ridge_symp_area_volnorm_le_1em2_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em2_fraction,
        ridge_symp_area_volnorm_le_1em1_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em1_fraction,
        allpair_abs_omega_vol1_mean: omega_fields.allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std: omega_fields.allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min: omega_fields.allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max: omega_fields.allpair_abs_omega_vol1_max,
        allpair_zero_fraction: omega_fields.allpair_zero_fraction,
        ridge_abs_omega_vol1_mean: omega_fields.ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std: omega_fields.ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min: omega_fields.ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max: omega_fields.ridge_abs_omega_vol1_max,
        ridge_zero_fraction: omega_fields.ridge_zero_fraction,
        ridge_abs_omega_vol1_le_1em3_fraction: omega_fields.ridge_abs_omega_vol1_le_1em3_fraction,
        ridge_abs_omega_vol1_le_1em2_fraction: omega_fields.ridge_abs_omega_vol1_le_1em2_fraction,
        ridge_abs_omega_vol1_le_1em1_fraction: omega_fields.ridge_abs_omega_vol1_le_1em1_fraction,
        transition_density: omega_fields.transition_density,
        transition_bidirectional_fraction: omega_fields.transition_bidirectional_fraction,
        transition_out_degree_mean: omega_fields.transition_out_degree_mean,
        transition_out_degree_std: omega_fields.transition_out_degree_std,
        transition_out_degree_min: omega_fields.transition_out_degree_min,
        transition_out_degree_max: omega_fields.transition_out_degree_max,
        orbit_sigma_available: orbit_fields.orbit_sigma_available,
        orbit_sigma_count: orbit_fields.orbit_sigma_count,
        orbit_sigma_gap_cutoff: orbit_fields.orbit_sigma_gap_cutoff,
        orbit_sigma_len: orbit_fields.orbit_sigma_len,
        orbit_sigma_fraction: orbit_fields.orbit_sigma_fraction,
        orbit_selected_norm_mean: orbit_fields.orbit_selected_norm_mean,
        orbit_selected_norm_std: orbit_fields.orbit_selected_norm_std,
        orbit_selected_norm_min: orbit_fields.orbit_selected_norm_min,
        orbit_selected_norm_max: orbit_fields.orbit_selected_norm_max,
        orbit_cycle_abs_omega_mean: orbit_fields.orbit_cycle_abs_omega_mean,
        orbit_cycle_abs_omega_std: orbit_fields.orbit_cycle_abs_omega_std,
        orbit_cycle_abs_omega_min: orbit_fields.orbit_cycle_abs_omega_min,
        orbit_cycle_abs_omega_max: orbit_fields.orbit_cycle_abs_omega_max,
        orbit_cycle_abs_omega_le_1e3_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e3_fraction,
        orbit_cycle_abs_omega_le_1e2_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e2_fraction,
        orbit_cycle_abs_omega_le_1e1_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e1_fraction,
        orbit_cycle_zero_fraction: orbit_fields.orbit_cycle_zero_fraction,
        orbit_cycle_transition_fraction: orbit_fields.orbit_cycle_transition_fraction,
        orbit_cycle_bidirectional_fraction: orbit_fields.orbit_cycle_bidirectional_fraction,
        orbit_cycle_adjacent_fraction: orbit_fields.orbit_cycle_adjacent_fraction,
        orbit_selected_out_degree_mean: orbit_fields.orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std: orbit_fields.orbit_selected_out_degree_std,
        orbit_selected_out_degree_min: orbit_fields.orbit_selected_out_degree_min,
        orbit_selected_out_degree_max: orbit_fields.orbit_selected_out_degree_max,
        orbit_kkt_available: orbit_fields.orbit_kkt_available,
        orbit_search_scalar_available: orbit_fields.orbit_search_scalar_available,
        orbit_result_iterations_log1p: orbit_fields.orbit_result_iterations_log1p,
        orbit_result_returned_orbit_count: orbit_fields.orbit_result_returned_orbit_count,
        orbit_best_beta_margin: orbit_fields.orbit_best_beta_margin,
        orbit_best_q_error_bound: orbit_fields.orbit_best_q_error_bound,
        orbit_best_has_mu: orbit_fields.orbit_best_has_mu,
        orbit_best_has_xi: orbit_fields.orbit_best_has_xi,
        orbit_best_is_admissible_exact: orbit_fields.orbit_best_is_admissible_exact,
        orbit_best_is_indeterminate_f64: orbit_fields.orbit_best_is_indeterminate_f64,
    }
}

pub fn build_polytope_table(rows: &[LoadedPolytopeRow]) -> Vec<PolytopeTableRow> {
    rows.iter().map(enrich_row).collect()
}
