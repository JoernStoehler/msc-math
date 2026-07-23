use crate::{
    analytic_center, standard_symplectic_matrix, translate_duals, volume_one_duals_f64,
    CandidateOutput, CandidateSpec,
};
use nalgebra::{Matrix4, Vector4};
use std::cmp::Ordering;

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "volume_one_symplectic_frame_min",
    canonicalize,
};

const OMEGA_EPS: f64 = 1e-10;
const SYMPLECTIC_DEFECT_EPS: f64 = 1e-7;

pub fn canonicalize(duals: &[Vector4<f64>]) -> CandidateOutput {
    let Some(volume_scaled) = volume_one_duals_f64(duals) else {
        return CandidateOutput {
            duals: duals.to_vec(),
            status: "volume_reconstruction_failed",
        };
    };

    let (center, center_status) = analytic_center(&volume_scaled);
    let shifted = if center_status == "ok" {
        match translate_duals(&volume_scaled, &center) {
            Ok(translated) => translated,
            Err(_) => {
                return CandidateOutput {
                    duals: volume_scaled,
                    status: "translation_failed",
                };
            }
        }
    } else {
        return CandidateOutput {
            duals: volume_scaled,
            status: center_status,
        };
    };

    let Some(best) = best_symplectic_frame_coordinates(&shifted) else {
        return CandidateOutput {
            duals: shifted,
            status: "no_symplectic_frame",
        };
    };

    CandidateOutput {
        duals: best,
        status: "ok",
    }
}

fn best_symplectic_frame_coordinates(duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    if duals.len() < 4 {
        return None;
    }
    let mut best: Option<Vec<Vector4<f64>>> = None;
    for i in 0..duals.len() {
        for j in 0..duals.len() {
            if j == i {
                continue;
            }
            for k in 0..duals.len() {
                if k == i || k == j {
                    continue;
                }
                for l in 0..duals.len() {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    let Some(frame) = symplectic_frame_from_ordered_vectors(
                        duals[i], duals[j], duals[k], duals[l],
                    ) else {
                        continue;
                    };
                    let Some(frame_inverse) = frame.try_inverse() else {
                        continue;
                    };
                    let mut coordinates = duals
                        .iter()
                        .map(|dual| frame_inverse * dual)
                        .collect::<Vec<_>>();
                    if !coordinates
                        .iter()
                        .all(|row| row.iter().all(|value| value.is_finite()))
                    {
                        continue;
                    }
                    coordinates.sort_by(compare_vectors_exactly);
                    if best
                        .as_ref()
                        .map(|current| {
                            compare_representations(&coordinates, current) == Ordering::Less
                        })
                        .unwrap_or(true)
                    {
                        best = Some(coordinates);
                    }
                }
            }
        }
    }
    best
}

fn symplectic_frame_from_ordered_vectors(
    first: Vector4<f64>,
    second: Vector4<f64>,
    third: Vector4<f64>,
    fourth: Vector4<f64>,
) -> Option<Matrix4<f64>> {
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
    (defect < SYMPLECTIC_DEFECT_EPS).then_some(frame)
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

fn compare_representations(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> Ordering {
    for (left_row, right_row) in left.iter().zip(right.iter()) {
        match compare_vectors_exactly(left_row, right_row) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    left.len().cmp(&right.len())
}

fn compare_vectors_exactly(left: &Vector4<f64>, right: &Vector4<f64>) -> Ordering {
    for index in 0..4 {
        match left[index].total_cmp(&right[index]) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    Ordering::Equal
}
