use crate::{
    analytic_center, standard_symplectic_matrix, translate_duals, volume_one_duals_f64,
    CandidateOutput, CandidateSpec,
};
use nalgebra::{Matrix4, Vector4};

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "volume_one_omega_labeled_symplectic_frame",
    canonicalize,
};

const OMEGA_EPS: f64 = 1e-10;
const SYMPLECTIC_DEFECT_EPS: f64 = 1e-7;
const SCORE_SCALE: f64 = 1e12;

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

    let omega = omega_matrix(&shifted);
    let Some(canonical_order) = generic_omega_row_order(&omega) else {
        return CandidateOutput {
            duals: shifted,
            status: "nonunique_omega_signature",
        };
    };
    let ordered = canonical_order
        .iter()
        .map(|&index| shifted[index])
        .collect::<Vec<_>>();

    let Some(frame) = selected_symplectic_frame(&ordered) else {
        return CandidateOutput {
            duals: shifted,
            status: "no_symplectic_frame",
        };
    };
    let Some(frame_inverse) = frame.try_inverse() else {
        return CandidateOutput {
            duals: shifted,
            status: "singular_symplectic_frame",
        };
    };
    let coordinates = ordered
        .iter()
        .map(|dual| frame_inverse * dual)
        .collect::<Vec<_>>();

    CandidateOutput {
        duals: coordinates,
        status: "ok",
    }
}

fn selected_symplectic_frame(duals: &[Vector4<f64>]) -> Option<Matrix4<f64>> {
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
                    let frame = symplectic_frame_from_ordered_vectors(
                        duals[a], duals[b], duals[c], duals[d],
                    );
                    if frame.is_some() {
                        return frame;
                    }
                }
            }
        }
    }
    None
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

fn generic_omega_row_order(omega: &[Vec<f64>]) -> Option<Vec<usize>> {
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
    signatures.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
    if signatures.windows(2).any(|pair| pair[0].1 == pair[1].1) {
        return None;
    }
    Some(
        signatures
            .into_iter()
            .map(|(row_index, _)| row_index)
            .collect(),
    )
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

fn quantize(value: f64) -> i128 {
    (value * SCORE_SCALE).round() as i128
}
