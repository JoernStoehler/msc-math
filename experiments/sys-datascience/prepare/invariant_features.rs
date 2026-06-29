//! Full-symmetry invariant feature table for random/product datascience rows.
//!
//! The input convention is `K = {x : <a_i, x> <= 1}`. This module exports
//! identity/target fields plus features whose formulas are invariant under the
//! full target action `Sp(4) x R_+ x R^4 x Perm(F)`, up to the stated f64
//! reconstruction limits. In this normalized-dual representation, a translated
//! body is represented here only when the new origin is still interior, so every
//! translated inequality can be renormalized to right-hand side `1`.
//!
//! - Combinatorial features depend only on the face lattice, so they are
//!   unchanged by invertible affine maps and facet permutation.
//! - For an oriented two-face polygon `P`, `0.5 * |sum omega(v_i, v_{i+1})|`
//!   is translation-invariant because the extra translation term telescopes
//!   around the closed polygon, Sp(4)-invariant by definition, and scales by
//!   `lambda^2` under primal scaling `x -> lambda x`.
//! - Four-dimensional volume scales by `lambda^4`, so dividing symplectic
//!   two-face area statistics by `sqrt(volume)` removes the scale.
//!
//! Cutoff/sign features are intentionally absent from this v1 table. They need
//! ambiguity accounting near classifier boundaries before they should be used
//! as method-facing invariants.

#[path = "features_dual_vertices.rs"]
mod features_dual_vertices;
#[path = "features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "features_helpers.rs"]
mod features_helpers;
#[path = "features_skeleton.rs"]
mod features_skeleton;

use crate::load_caches::LoadedPolytopeRow;
use crate::rows::PolytopeTableRow;
use euclidean_polytopes::{
    edges_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence,
    vertex_facets_from_vertex_facet_incidence,
};
use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::Vector4;
use rayon::prelude::*;

fn divide_by_volume_sqrt(value: f64, volume_sqrt: f64) -> f64 {
    value / volume_sqrt
}

pub fn invariant_row_from_dual_vertices(
    poly_id: String,
    dual_vertices: Vec<Vector4<f64>>,
    volume: f64,
    sys: f64,
) -> PolytopeTableRow {
    assert!(
        volume.is_finite() && volume > 0.0,
        "invariant row needs positive finite volume, got {volume}"
    );
    let volume_sqrt = volume.sqrt();
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
        .unwrap_or_else(|| panic!("reconstruct invariant-feature polytope {poly_id}"));
    let facet_count = polytope.facet_count();
    let incidence = &polytope.vertex_facet_incidence;
    let vertex_facets = vertex_facets_from_vertex_facet_incidence(incidence);
    let edges = edges_from_vertex_facet_incidence(incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(incidence);
    let skeleton_fields =
        features_skeleton::compute_skeleton_fields(&polytope, &vertex_facets, &edges, &two_faces);
    let face_symplectic_fields = features_face_symplectic::compute_face_symplectic_fields(
        &two_faces,
        &polytope.vertices_f64,
        incidence,
    );

    PolytopeTableRow {
        poly_id,
        facet_count,
        capacity_source: "diagnostic_synthetic".to_string(),
        sys,
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
        ridge_symp_area_ordered_face_count: face_symplectic_fields
            .ridge_symp_area_ordered_face_count,
        ridge_symp_area_ordering_failure_count: face_symplectic_fields
            .ridge_symp_area_ordering_failure_count,
        ridge_symp_area_ordered_fraction: face_symplectic_fields.ridge_symp_area_ordered_fraction,
        ridge_symp_area_mean_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_mean,
            volume_sqrt,
        ),
        ridge_symp_area_std_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_std,
            volume_sqrt,
        ),
        ridge_symp_area_min_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_min,
            volume_sqrt,
        ),
        ridge_symp_area_max_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_max,
            volume_sqrt,
        ),
        ridge_symp_area_q25_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_q25,
            volume_sqrt,
        ),
        ridge_symp_area_median_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_median,
            volume_sqrt,
        ),
        ridge_symp_area_q75_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_q75,
            volume_sqrt,
        ),
        ridge_symp_area_q90_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_q90,
            volume_sqrt,
        ),
        ridge_symp_area_q95_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_q95,
            volume_sqrt,
        ),
        ridge_symp_area_sum_over_volume_sqrt: divide_by_volume_sqrt(
            face_symplectic_fields.ridge_symp_area_sum,
            volume_sqrt,
        ),
        ridge_symp_area_max_share: face_symplectic_fields.ridge_symp_area_max_share,
        ridge_symp_area_top3_share: face_symplectic_fields.ridge_symp_area_top3_share,
    }
}

fn invariant_row_from_loaded_row(row: &LoadedPolytopeRow) -> PolytopeTableRow {
    let raw_dual_vertices = features_dual_vertices::raw_dual_vertices_f64(row);
    let mut output = invariant_row_from_dual_vertices(
        row.poly_id.clone(),
        raw_dual_vertices,
        row.volume,
        row.sys,
    );
    output.capacity_source = row.capacity_source.clone();
    output
}

pub fn build_polytope_table(rows: &[LoadedPolytopeRow]) -> Vec<PolytopeTableRow> {
    rows.par_iter().map(invariant_row_from_loaded_row).collect()
}

pub fn exact_invariant_row_fields(row: &PolytopeTableRow) -> Vec<(&'static str, String)> {
    vec![
        ("facet_count", row.facet_count.to_string()),
        ("vertex_count", row.vertex_count.to_string()),
        ("edge_count", row.edge_count.to_string()),
        ("ridge_count", row.ridge_count.to_string()),
        ("is_simple", row.is_simple.to_string()),
        (
            "ridge_symp_area_ordered_face_count",
            row.ridge_symp_area_ordered_face_count.to_string(),
        ),
        (
            "ridge_symp_area_ordering_failure_count",
            row.ridge_symp_area_ordering_failure_count.to_string(),
        ),
    ]
}

pub fn numeric_invariant_row_fields(row: &PolytopeTableRow) -> Vec<(&'static str, f64)> {
    vec![
        ("sys", row.sys),
        ("simple_vertex_fraction", row.simple_vertex_fraction),
        ("edge_density", row.edge_density),
        (
            "vertex_incident_facets_mean",
            row.vertex_incident_facets_mean,
        ),
        ("vertex_incident_facets_std", row.vertex_incident_facets_std),
        ("vertex_incident_facets_min", row.vertex_incident_facets_min),
        ("vertex_incident_facets_max", row.vertex_incident_facets_max),
        ("vertex_degree_mean", row.vertex_degree_mean),
        ("vertex_degree_std", row.vertex_degree_std),
        ("vertex_degree_min", row.vertex_degree_min),
        ("vertex_degree_max", row.vertex_degree_max),
        ("ridge_size_mean", row.ridge_size_mean),
        ("ridge_size_std", row.ridge_size_std),
        ("ridge_size_min", row.ridge_size_min),
        ("ridge_size_max", row.ridge_size_max),
        ("facet_vertex_count_mean", row.facet_vertex_count_mean),
        ("facet_vertex_count_std", row.facet_vertex_count_std),
        ("facet_vertex_count_min", row.facet_vertex_count_min),
        ("facet_vertex_count_max", row.facet_vertex_count_max),
        ("facet_neighbor_count_mean", row.facet_neighbor_count_mean),
        ("facet_neighbor_count_std", row.facet_neighbor_count_std),
        ("facet_neighbor_count_min", row.facet_neighbor_count_min),
        ("facet_neighbor_count_max", row.facet_neighbor_count_max),
        (
            "ridge_symp_area_ordered_fraction",
            row.ridge_symp_area_ordered_fraction,
        ),
        (
            "ridge_symp_area_mean_over_volume_sqrt",
            row.ridge_symp_area_mean_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_std_over_volume_sqrt",
            row.ridge_symp_area_std_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_min_over_volume_sqrt",
            row.ridge_symp_area_min_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_max_over_volume_sqrt",
            row.ridge_symp_area_max_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_q25_over_volume_sqrt",
            row.ridge_symp_area_q25_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_median_over_volume_sqrt",
            row.ridge_symp_area_median_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_q75_over_volume_sqrt",
            row.ridge_symp_area_q75_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_q90_over_volume_sqrt",
            row.ridge_symp_area_q90_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_q95_over_volume_sqrt",
            row.ridge_symp_area_q95_over_volume_sqrt,
        ),
        (
            "ridge_symp_area_sum_over_volume_sqrt",
            row.ridge_symp_area_sum_over_volume_sqrt,
        ),
        ("ridge_symp_area_max_share", row.ridge_symp_area_max_share),
        ("ridge_symp_area_top3_share", row.ridge_symp_area_top3_share),
    ]
}

pub fn relative_residual(left: f64, right: f64) -> f64 {
    (left - right).abs() / left.abs().max(right.abs()).max(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix4, Vector4};

    fn simplex_duals() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(-1.0, -1.0, -1.0, -1.0),
        ]
    }

    fn skewed_box_duals() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(1.2, 0.1, 0.0, 0.0),
            Vector4::new(-0.9, 0.2, 0.0, 0.0),
            Vector4::new(0.0, 1.1, 0.1, 0.0),
            Vector4::new(0.0, -0.8, 0.2, 0.0),
            Vector4::new(0.0, 0.0, 1.3, -0.1),
            Vector4::new(0.0, 0.0, -0.7, 0.1),
            Vector4::new(0.1, 0.0, 0.0, 1.1),
            Vector4::new(-0.1, 0.0, 0.0, -0.9),
        ]
    }

    fn standard_symplectic_matrix() -> Matrix4<f64> {
        Matrix4::new(
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
            -1.0, 0.0, 0.0, 0.0, //
            0.0, -1.0, 0.0, 0.0,
        )
    }

    fn sample_sp4_exp(seed: usize) -> Matrix4<f64> {
        let t = seed as f64 + 1.0;
        let h = Matrix4::new(
            0.08 * t,
            0.03,
            -0.02,
            0.01,
            0.03,
            -0.04 * t,
            0.05,
            -0.01,
            -0.02,
            0.05,
            0.02 * t,
            0.04,
            0.01,
            -0.01,
            0.04,
            -0.03 * t,
        );
        (standard_symplectic_matrix() * h).exp()
    }

    fn transform_duals_by_primal_map(
        duals: &[Vector4<f64>],
        primal_map: &Matrix4<f64>,
    ) -> Vec<Vector4<f64>> {
        let dual_map = primal_map
            .try_inverse()
            .expect("invertible transform")
            .transpose();
        duals.iter().map(|dual| dual_map * dual).collect()
    }

    fn translate_duals(duals: &[Vector4<f64>], shift: Vector4<f64>) -> Vec<Vector4<f64>> {
        duals
            .iter()
            .map(|dual| {
                let denominator = 1.0 + dual.dot(&shift);
                assert!(
                    denominator > 1e-6,
                    "test translation left the origin outside the translated body"
                );
                dual / denominator
            })
            .collect()
    }

    fn permute_duals(mut duals: Vec<Vector4<f64>>) -> Vec<Vector4<f64>> {
        duals.reverse();
        duals
    }

    fn assert_rows_close(base: &PolytopeTableRow, other: &PolytopeTableRow) {
        for ((name, base_value), (_, other_value)) in exact_invariant_row_fields(base)
            .into_iter()
            .zip(exact_invariant_row_fields(other))
        {
            assert_eq!(base_value, other_value, "{name}");
        }
        for ((name, base_value), (_, other_value)) in numeric_invariant_row_fields(base)
            .into_iter()
            .zip(numeric_invariant_row_fields(other))
        {
            let residual = relative_residual(base_value, other_value);
            assert!(
                residual.is_finite() && residual < 1e-8,
                "{name}: base={base_value} other={other_value} residual={residual}"
            );
        }
    }

    #[test]
    fn invariant_features_survive_target_group_transforms() {
        let cases = [
            ("simplex", simplex_duals(), 0.25),
            ("skewed_box", skewed_box_duals(), 3.7),
        ];
        for (case_name, duals, volume) in cases {
            let base =
                invariant_row_from_dual_vertices(case_name.to_string(), duals.clone(), volume, 0.5);

            let scale = 1.7;
            let scaled_duals = duals.iter().map(|dual| dual / scale).collect::<Vec<_>>();
            let scaled = invariant_row_from_dual_vertices(
                format!("{case_name}_scale"),
                scaled_duals,
                volume * scale.powi(4),
                0.5,
            );
            assert_rows_close(&base, &scaled);

            let translated = invariant_row_from_dual_vertices(
                format!("{case_name}_translate"),
                translate_duals(&duals, Vector4::new(0.03, -0.02, 0.01, 0.04)),
                volume,
                0.5,
            );
            assert_rows_close(&base, &translated);

            let permuted = invariant_row_from_dual_vertices(
                format!("{case_name}_permute"),
                permute_duals(duals.clone()),
                volume,
                0.5,
            );
            assert_rows_close(&base, &permuted);

            for seed in 0..3 {
                let symplectic_duals = transform_duals_by_primal_map(&duals, &sample_sp4_exp(seed));
                let symplectic = invariant_row_from_dual_vertices(
                    format!("{case_name}_sp4_{seed}"),
                    symplectic_duals,
                    volume,
                    0.5,
                );
                assert_rows_close(&base, &symplectic);
            }

            let full_primal_map = Matrix4::identity() * 1.4 * sample_sp4_exp(4);
            let full_duals = transform_duals_by_primal_map(
                &translate_duals(
                    &permute_duals(duals.clone()),
                    Vector4::new(0.02, 0.01, -0.02, 0.03),
                ),
                &full_primal_map,
            );
            let full = invariant_row_from_dual_vertices(
                format!("{case_name}_full"),
                full_duals,
                volume * 1.4f64.powi(4),
                0.5,
            );
            assert_rows_close(&base, &full);
        }
    }
}
