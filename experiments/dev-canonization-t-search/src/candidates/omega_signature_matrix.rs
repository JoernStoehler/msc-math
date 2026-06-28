use crate::{
    analytic_center, standard_symplectic_matrix, translate_duals, CandidateOutput, CandidateSpec,
};
use nalgebra::Vector4;

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "omega_signature_matrix",
    canonicalize,
};

const SCORE_SCALE: f64 = 1e12;

pub fn canonicalize(duals: &[Vector4<f64>]) -> CandidateOutput {
    let (center, center_status) = analytic_center(duals);
    let shifted = if center_status == "ok" {
        match translate_duals(duals, &center) {
            Ok(translated) => translated,
            Err(_) => {
                return CandidateOutput {
                    duals: duals.to_vec(),
                    status: "translation_failed",
                };
            }
        }
    } else {
        return CandidateOutput {
            duals: duals.to_vec(),
            status: center_status,
        };
    };

    let omega = normalized_omega_matrix(&shifted);
    let order = signature_order(&omega);
    let flattened = flatten_ordered_matrix(&omega, &order);

    CandidateOutput {
        duals: pack_scalars_as_vectors(&flattened),
        status: "ok",
    }
}

pub(crate) fn normalized_omega_matrix(duals: &[Vector4<f64>]) -> Vec<Vec<f64>> {
    let j = standard_symplectic_matrix();
    let mut omega = vec![vec![0.0; duals.len()]; duals.len()];
    let mut max_abs = 0.0_f64;
    for left in 0..duals.len() {
        for right in 0..duals.len() {
            let value = duals[left].dot(&(j * duals[right]));
            omega[left][right] = value;
            max_abs = max_abs.max(value.abs());
        }
    }
    if max_abs > 0.0 {
        for row in &mut omega {
            for value in row {
                *value /= max_abs;
            }
        }
    }
    omega
}

pub(crate) fn raw_omega_matrix(duals: &[Vector4<f64>]) -> Vec<Vec<f64>> {
    let j = standard_symplectic_matrix();
    let mut omega = vec![vec![0.0; duals.len()]; duals.len()];
    for left in 0..duals.len() {
        for right in 0..duals.len() {
            omega[left][right] = duals[left].dot(&(j * duals[right]));
        }
    }
    omega
}

pub(crate) fn signature_order(omega: &[Vec<f64>]) -> Vec<usize> {
    let mut signatures = (0..omega.len())
        .map(|row_index| {
            let mut row_values = omega[row_index]
                .iter()
                .map(|value| quantize(*value))
                .collect::<Vec<_>>();
            row_values.sort_by(|left, right| right.cmp(left));
            (row_index, row_values)
        })
        .collect::<Vec<_>>();
    signatures.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
    signatures
        .into_iter()
        .map(|(row_index, _)| row_index)
        .collect()
}

pub(crate) fn flatten_ordered_matrix(omega: &[Vec<f64>], order: &[usize]) -> Vec<f64> {
    let mut flattened = Vec::with_capacity(order.len() * order.len());
    for &row in order {
        for &col in order {
            flattened.push(omega[row][col]);
        }
    }
    flattened
}

pub(crate) fn pack_scalars_as_vectors(values: &[f64]) -> Vec<Vector4<f64>> {
    values
        .chunks(4)
        .map(|chunk| {
            let mut row = Vector4::zeros();
            for (index, value) in chunk.iter().enumerate() {
                row[index] = *value;
            }
            row
        })
        .collect()
}

fn quantize(value: f64) -> i128 {
    (value * SCORE_SCALE).round() as i128
}
