//! Diagnostic invariance report for the invariant-feature table.
//!
//! This is a smoke/report command, not the retained table builder. It checks
//! the exported invariant row on small synthetic polytopes under representatives
//! of scale, translation, facet permutation, Sp(4), and their composition.

mod invariant_features;
mod load_caches;
#[path = "../polytope-datasets/rows.rs"]
mod producer_rows;
mod rows;

use nalgebra::{Matrix4, Vector4};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct Report {
    tolerance: f64,
    cases: usize,
    transforms_per_case: usize,
    max_relative_residual: f64,
    failures: usize,
    summaries: BTreeMap<String, TransformSummary>,
}

#[derive(Default, Serialize)]
struct TransformSummary {
    samples: usize,
    max_relative_residual: f64,
    failures: usize,
    worst_field: Option<String>,
    worst_case: Option<String>,
}

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
            assert!(denominator > 1e-6);
            dual / denominator
        })
        .collect()
}

fn permute_duals(mut duals: Vec<Vector4<f64>>) -> Vec<Vector4<f64>> {
    duals.rotate_left(1);
    duals
}

fn compare_rows(
    base: &rows::PolytopeTableRow,
    other: &rows::PolytopeTableRow,
) -> (String, f64, usize) {
    let mut worst = ("".to_string(), 0.0);
    let mut field_failures = 0usize;
    for ((name, base_value), (_, other_value)) in
        invariant_features::exact_invariant_row_fields(base)
            .into_iter()
            .zip(invariant_features::exact_invariant_row_fields(other))
    {
        if base_value != other_value {
            field_failures += 1;
            if worst.1 == 0.0 {
                worst = (name.to_string(), f64::INFINITY);
            }
        }
    }
    for ((name, base_value), (_, other_value)) in
        invariant_features::numeric_invariant_row_fields(base)
            .into_iter()
            .zip(invariant_features::numeric_invariant_row_fields(other))
    {
        let residual = invariant_features::relative_residual(base_value, other_value);
        if !residual.is_finite() {
            field_failures += 1;
            if worst.1.is_finite() {
                worst = (name.to_string(), residual);
            }
            continue;
        }
        if residual > worst.1 {
            worst = (name.to_string(), residual);
        }
    }
    (worst.0, worst.1, field_failures)
}

fn main() {
    let tolerance = 1e-8;
    let cases = [
        ("simplex", simplex_duals(), 0.25),
        ("skewed_box", skewed_box_duals(), 3.7),
    ];
    let mut summaries = BTreeMap::<String, TransformSummary>::new();
    for (case_name, duals, volume) in cases {
        let base = invariant_features::invariant_row_from_dual_vertices(
            case_name.to_string(),
            duals.clone(),
            volume,
            0.5,
        );
        let scale = 1.7f64;
        let scaled_duals = duals.iter().map(|dual| dual / scale).collect::<Vec<_>>();
        let transformed_cases = [
            ("scale".to_string(), scaled_duals, volume * scale.powi(4)),
            (
                "translation".to_string(),
                translate_duals(&duals, Vector4::new(0.03, -0.02, 0.01, 0.04)),
                volume,
            ),
            (
                "permutation".to_string(),
                permute_duals(duals.clone()),
                volume,
            ),
            (
                "sampled_sp4_exp".to_string(),
                transform_duals_by_primal_map(&duals, &sample_sp4_exp(2)),
                volume,
            ),
            (
                "sampled_full_group".to_string(),
                transform_duals_by_primal_map(
                    &translate_duals(
                        &permute_duals(duals.clone()),
                        Vector4::new(0.02, 0.01, -0.02, 0.03),
                    ),
                    &(Matrix4::identity() * 1.4 * sample_sp4_exp(4)),
                ),
                volume * 1.4f64.powi(4),
            ),
        ];

        for (transform_name, transformed_duals, transformed_volume) in transformed_cases {
            let transformed = invariant_features::invariant_row_from_dual_vertices(
                format!("{case_name}_{transform_name}"),
                transformed_duals,
                transformed_volume,
                0.5,
            );
            let (worst_field, residual, field_failures) = compare_rows(&base, &transformed);
            let summary = summaries.entry(transform_name).or_default();
            summary.samples += 1;
            if residual > summary.max_relative_residual {
                summary.max_relative_residual = residual;
                summary.worst_field = Some(worst_field);
                summary.worst_case = Some(case_name.to_string());
            }
            if field_failures > 0 || !residual.is_finite() || residual > tolerance {
                summary.failures += 1;
            }
        }
    }
    let max_relative_residual = summaries
        .values()
        .map(|summary| summary.max_relative_residual)
        .fold(0.0, f64::max);
    let failures = summaries.values().map(|summary| summary.failures).sum();
    let report = Report {
        tolerance,
        cases: 2,
        transforms_per_case: 5,
        max_relative_residual,
        failures,
        summaries,
    };
    serde_json::to_writer_pretty(std::io::stdout(), &report).expect("write report");
    println!();
    if failures > 0 {
        std::process::exit(1);
    }
}
