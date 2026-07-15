//! Copy-local generic symplectic-section adapter.
//!
//! The section follows `formal/generic-coordinate-canonization.tex` and the
//! frozen `volume_one_omega_labeled_symplectic_frame` candidate.  It is a
//! partial f64 construction: callers must inspect `SectionOutput::status` and
//! may use coordinates only when the status is `ok`.

use euclidean_polytopes::volume_from_incidence_f64;
use nalgebra::{DMatrix, Matrix4, Vector4};
use serde::Serialize;
use std::panic::{catch_unwind, AssertUnwindSafe};

const ANALYTIC_CENTER_MAX_ITER: usize = 50;
const ANALYTIC_CENTER_TOL: f64 = 1e-12;
const MIN_SLACK: f64 = 1e-10;
const OMEGA_EPS: f64 = 1e-10;
const SYMPLECTIC_DEFECT_EPS: f64 = 1e-7;
const SCORE_SCALE: f64 = 1e12;

#[derive(Clone, Debug, Default, Serialize)]
pub struct SectionDiagnostics {
    pub reconstructed_volume: Option<f64>,
    pub volume_one_algebraic_residual: Option<f64>,
    pub analytic_center_gradient_norm: Option<f64>,
    pub analytic_center_newton_decrement: Option<f64>,
    pub minimum_quantized_signature_linf_gap: Option<u128>,
    pub selected_quadruple: Option<[usize; 4]>,
    pub frame_symplectic_defect_frobenius: Option<f64>,
    pub frame_solve_max_relative_residual: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct SectionOutput {
    pub status: &'static str,
    pub coordinates: Option<Vec<Vector4<f64>>>,
    pub diagnostics: SectionDiagnostics,
}

#[derive(Clone, Debug)]
struct CenterOutput {
    center: Vector4<f64>,
    status: &'static str,
    gradient_norm: Option<f64>,
    newton_decrement: Option<f64>,
}

/// Apply volume-one scaling, analytic-center translation, omega-signature
/// labeling, and the first successful symplectic Gram--Schmidt frame.
pub fn canonicalize(duals: &[Vector4<f64>]) -> SectionOutput {
    let mut diagnostics = SectionDiagnostics::default();
    let Some(volume) = volume_from_normalized_duals_f64(duals) else {
        return non_success("volume_reconstruction_failed", diagnostics);
    };
    if !volume.is_finite() || volume <= 0.0 {
        return non_success("volume_reconstruction_failed", diagnostics);
    }
    diagnostics.reconstructed_volume = Some(volume);
    let dual_scale = volume.powf(0.25);
    let volume_scaled = duals
        .iter()
        .map(|dual| dual * dual_scale)
        .collect::<Vec<_>>();
    diagnostics.volume_one_algebraic_residual = Some((volume / dual_scale.powi(4) - 1.0).abs());

    let center = analytic_center(&volume_scaled);
    diagnostics.analytic_center_gradient_norm = center.gradient_norm;
    diagnostics.analytic_center_newton_decrement = center.newton_decrement;
    if center.status != "ok" {
        return non_success(center.status, diagnostics);
    }
    let Ok(shifted) = translate_duals(&volume_scaled, &center.center) else {
        return non_success("translation_failed", diagnostics);
    };

    let omega = omega_matrix(&shifted);
    let Some((canonical_order, signature_gap)) = generic_omega_row_order(&omega) else {
        diagnostics.minimum_quantized_signature_linf_gap = Some(0);
        return non_success("nonunique_omega_signature", diagnostics);
    };
    diagnostics.minimum_quantized_signature_linf_gap = Some(signature_gap);
    let ordered = canonical_order
        .iter()
        .map(|&index| shifted[index])
        .collect::<Vec<_>>();

    let Some((frame, selected_quadruple, frame_defect)) = selected_symplectic_frame(&ordered)
    else {
        return non_success("no_symplectic_frame", diagnostics);
    };
    diagnostics.selected_quadruple = Some(selected_quadruple);
    diagnostics.frame_symplectic_defect_frobenius = Some(frame_defect);
    let Some(frame_inverse) = frame.try_inverse() else {
        return non_success("singular_symplectic_frame", diagnostics);
    };
    let coordinates = ordered
        .iter()
        .map(|dual| frame_inverse * dual)
        .collect::<Vec<_>>();
    let scale = rms_row_norm(&ordered).max(1.0);
    diagnostics.frame_solve_max_relative_residual = ordered
        .iter()
        .zip(&coordinates)
        .map(|(dual, coordinate)| (frame * coordinate - dual).norm() / scale)
        .reduce(f64::max);

    SectionOutput {
        status: "ok",
        coordinates: Some(coordinates),
        diagnostics,
    }
}

fn non_success(status: &'static str, diagnostics: SectionDiagnostics) -> SectionOutput {
    SectionOutput {
        status,
        coordinates: None,
        diagnostics,
    }
}

fn analytic_center(duals: &[Vector4<f64>]) -> CenterOutput {
    let mut center = Vector4::zeros();
    for _ in 0..ANALYTIC_CENTER_MAX_ITER {
        let mut gradient = Vector4::zeros();
        let mut hessian = Matrix4::zeros();
        for dual in duals {
            let slack = 1.0 - dual.dot(&center);
            if slack <= MIN_SLACK {
                return CenterOutput {
                    center: Vector4::zeros(),
                    status: "nonpositive_slack",
                    gradient_norm: None,
                    newton_decrement: None,
                };
            }
            let weighted = dual / slack;
            gradient += weighted;
            hessian += weighted * weighted.transpose();
        }
        let Some(step) = hessian.lu().solve(&gradient) else {
            return CenterOutput {
                center: Vector4::zeros(),
                status: "singular_hessian",
                gradient_norm: Some(gradient.norm()),
                newton_decrement: None,
            };
        };
        let decrement = gradient.dot(&step);
        if !decrement.is_finite() {
            return CenterOutput {
                center: Vector4::zeros(),
                status: "nonfinite_newton",
                gradient_norm: Some(gradient.norm()),
                newton_decrement: None,
            };
        }
        if decrement < ANALYTIC_CENTER_TOL {
            return CenterOutput {
                center,
                status: "ok",
                gradient_norm: Some(gradient.norm()),
                newton_decrement: Some(decrement),
            };
        }
        let mut step_size = 1.0;
        loop {
            let candidate = center - step_size * step;
            let candidate_min_slack = duals
                .iter()
                .map(|dual| 1.0 - dual.dot(&candidate))
                .fold(f64::INFINITY, f64::min);
            if candidate_min_slack > MIN_SLACK {
                center = candidate;
                break;
            }
            step_size *= 0.5;
            if step_size <= 1e-12 {
                return CenterOutput {
                    center: Vector4::zeros(),
                    status: "line_search_failed",
                    gradient_norm: Some(gradient.norm()),
                    newton_decrement: Some(decrement),
                };
            }
        }
    }
    CenterOutput {
        center,
        status: "max_iter",
        gradient_norm: None,
        newton_decrement: None,
    }
}

fn translate_duals(duals: &[Vector4<f64>], center: &Vector4<f64>) -> Result<Vec<Vector4<f64>>, ()> {
    let mut translated = Vec::with_capacity(duals.len());
    for dual in duals {
        let denominator = 1.0 - dual.dot(center);
        if !denominator.is_finite() || denominator <= MIN_SLACK {
            return Err(());
        }
        translated.push(dual / denominator);
    }
    Ok(translated)
}

fn omega_matrix(duals: &[Vector4<f64>]) -> Vec<Vec<f64>> {
    let j = standard_symplectic_matrix();
    let mut matrix = vec![vec![0.0; duals.len()]; duals.len()];
    for row in 0..duals.len() {
        for col in 0..duals.len() {
            matrix[row][col] = duals[row].dot(&(j * duals[col]));
        }
    }
    matrix
}

fn generic_omega_row_order(omega: &[Vec<f64>]) -> Option<(Vec<usize>, u128)> {
    let mut signatures = (0..omega.len())
        .map(|row_index| {
            let mut sorted_row = omega[row_index]
                .iter()
                .map(|value| quantize(*value))
                .collect::<Vec<_>>();
            sorted_row.sort_by(|left, right| right.cmp(left));
            (row_index, sorted_row)
        })
        .collect::<Vec<_>>();
    let mut minimum_gap = u128::MAX;
    for left in 0..signatures.len() {
        for right in (left + 1)..signatures.len() {
            minimum_gap = minimum_gap.min(signature_linf_gap(
                &signatures[left].1,
                &signatures[right].1,
            ));
        }
    }
    signatures.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
    if minimum_gap == 0 {
        return None;
    }
    Some((
        signatures
            .into_iter()
            .map(|(row_index, _)| row_index)
            .collect(),
        minimum_gap,
    ))
}

fn signature_linf_gap(left: &[i128], right: &[i128]) -> u128 {
    left.iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap_or(0)
}

fn selected_symplectic_frame(duals: &[Vector4<f64>]) -> Option<(Matrix4<f64>, [usize; 4], f64)> {
    for a in 0..duals.len() {
        for b in 0..duals.len() {
            if b == a {
                continue;
            }
            for c in 0..duals.len() {
                if c == a || c == b {
                    continue;
                }
                for d in 0..duals.len() {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    if let Some((frame, defect)) = symplectic_frame_from_ordered_vectors(
                        duals[a], duals[b], duals[c], duals[d],
                    ) {
                        return Some((frame, [a, b, c, d], defect));
                    }
                }
            }
        }
    }
    None
}

fn symplectic_frame_from_ordered_vectors(
    first: Vector4<f64>,
    second: Vector4<f64>,
    third: Vector4<f64>,
    fourth: Vector4<f64>,
) -> Option<(Matrix4<f64>, f64)> {
    let q1 = first;
    let p1_pairing = omega(&q1, &second);
    if !p1_pairing.is_finite() || p1_pairing.abs() < OMEGA_EPS {
        return None;
    }
    let p1 = second / p1_pairing;
    let q2 = symplectic_orthogonal_projection(third, &q1, &p1);
    let fourth_projected = symplectic_orthogonal_projection(fourth, &q1, &p1);
    let p2_pairing = omega(&q2, &fourth_projected);
    if !p2_pairing.is_finite() || p2_pairing.abs() < OMEGA_EPS {
        return None;
    }
    let p2 = fourth_projected / p2_pairing;
    let frame = Matrix4::from_columns(&[q1, q2, p1, p2]);
    if !frame.iter().all(|value| value.is_finite()) {
        return None;
    }
    let defect = (frame.transpose() * standard_symplectic_matrix() * frame
        - standard_symplectic_matrix())
    .norm();
    (defect < SYMPLECTIC_DEFECT_EPS).then_some((frame, defect))
}

fn symplectic_orthogonal_projection(
    vector: Vector4<f64>,
    q1: &Vector4<f64>,
    p1: &Vector4<f64>,
) -> Vector4<f64> {
    vector - omega(&vector, p1) * q1 + omega(&vector, q1) * p1
}

fn omega(left: &Vector4<f64>, right: &Vector4<f64>) -> f64 {
    left.dot(&(standard_symplectic_matrix() * right))
}

fn quantize(value: f64) -> i128 {
    (value * SCORE_SCALE).round() as i128
}

pub fn standard_symplectic_matrix() -> Matrix4<f64> {
    Matrix4::new(
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
        -1.0, 0.0, 0.0, 0.0, //
        0.0, -1.0, 0.0, 0.0,
    )
}

/// Exact minimum-cost assignment RMS on equal-cardinality row multisets.
///
/// This is row-permutation insensitive. It does not quotient coordinate maps,
/// translation, or scale; those properties belong to the representation it is
/// applied to.
pub fn unordered_row_assignment_rms(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> Option<f64> {
    if left.len() != right.len() || left.is_empty() {
        return None;
    }
    let n = left.len();
    let mut cost = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            cost[i][j] = (left[i] - right[j]).norm_squared();
        }
    }
    Some((hungarian_min_cost(&cost) / n as f64).sqrt())
}

fn hungarian_min_cost(cost: &[Vec<f64>]) -> f64 {
    let n = cost.len();
    let mut u = vec![0.0; n + 1];
    let mut v = vec![0.0; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0;
        let mut minv = vec![f64::INFINITY; n + 1];
        let mut used = vec![false; n + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = f64::INFINITY;
            let mut j1 = 0;
            for j in 1..=n {
                if used[j] {
                    continue;
                }
                let current = cost[i0 - 1][j - 1] - u[i0] - v[j];
                if current < minv[j] {
                    minv[j] = current;
                    way[j] = j0;
                }
                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    (-v[0]).max(0.0)
}

fn rms_row_norm(rows: &[Vector4<f64>]) -> f64 {
    (rows.iter().map(Vector4::norm_squared).sum::<f64>() / rows.len() as f64).sqrt()
}

fn volume_from_normalized_duals_f64(duals: &[Vector4<f64>]) -> Option<f64> {
    let vertices = enumerate_vertices_f64(duals)?;
    if vertices.len() < 5 {
        return None;
    }
    let incidence = approximate_incidence(duals, &vertices);
    catch_unwind(AssertUnwindSafe(|| {
        volume_from_incidence_f64(&vertices, &incidence).ok()
    }))
    .ok()
    .flatten()
}

fn enumerate_vertices_f64(duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    if duals.len() < 5
        || !duals
            .iter()
            .all(|dual| dual.iter().all(|value| value.is_finite()))
    {
        return None;
    }
    let mut vertices = Vec::new();
    for i in 0..duals.len() {
        for j in (i + 1)..duals.len() {
            for k in (j + 1)..duals.len() {
                for l in (k + 1)..duals.len() {
                    let matrix = Matrix4::new(
                        duals[i][0],
                        duals[i][1],
                        duals[i][2],
                        duals[i][3],
                        duals[j][0],
                        duals[j][1],
                        duals[j][2],
                        duals[j][3],
                        duals[k][0],
                        duals[k][1],
                        duals[k][2],
                        duals[k][3],
                        duals[l][0],
                        duals[l][1],
                        duals[l][2],
                        duals[l][3],
                    );
                    if matrix.determinant().abs() < 1e-12 {
                        continue;
                    }
                    let Some(candidate) = matrix.lu().solve(&Vector4::repeat(1.0)) else {
                        continue;
                    };
                    if !candidate.iter().all(|value| value.is_finite()) {
                        continue;
                    }
                    let feasible = duals.iter().all(|dual| {
                        let tolerance = 1e-8 * (1.0 + dual.norm() * candidate.norm());
                        dual.dot(&candidate) <= 1.0 + tolerance
                    });
                    if feasible
                        && vertices.iter().all(|known: &Vector4<f64>| {
                            (known - candidate).norm() > 1e-7 * (1.0 + known.norm())
                        })
                    {
                        vertices.push(candidate);
                    }
                }
            }
        }
    }
    Some(vertices)
}

fn approximate_incidence(duals: &[Vector4<f64>], vertices: &[Vector4<f64>]) -> DMatrix<bool> {
    DMatrix::from_fn(vertices.len(), duals.len(), |row, col| {
        let value = duals[col].dot(&vertices[row]);
        let tolerance = 1e-7 * (1.0 + duals[col].norm() * vertices[row].norm());
        (value - 1.0).abs() <= tolerance
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asymmetric_box_duals() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(1.00, 0.0, 0.0, 0.0),
            Vector4::new(-1.11, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.23, 0.0, 0.0),
            Vector4::new(0.0, -1.37, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.51, 0.0),
            Vector4::new(0.0, 0.0, -1.69, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.91),
            Vector4::new(0.0, 0.0, 0.0, -2.17),
        ]
    }

    #[test]
    fn assignment_distance_ignores_row_permutation() {
        let rows = asymmetric_box_duals();
        let mut permuted = rows.clone();
        permuted.rotate_left(3);
        assert_eq!(unordered_row_assignment_rms(&rows, &permuted), Some(0.0));
    }

    #[test]
    fn symmetric_cube_is_observable_non_success() {
        let rows = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ];
        let output = canonicalize(&rows);
        assert_eq!(output.status, "nonunique_omega_signature");
        assert!(output.coordinates.is_none());
    }

    #[test]
    fn asymmetric_box_has_a_section() {
        let output = canonicalize(&asymmetric_box_duals());
        assert_eq!(output.status, "ok");
        assert!(output.coordinates.is_some());
        assert!(
            output
                .diagnostics
                .frame_symplectic_defect_frobenius
                .unwrap()
                < 1e-7
        );
    }
}
