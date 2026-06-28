use crate::{
    analytic_center, translate_duals, volume_one_duals_f64, CandidateOutput, CandidateSpec,
};
use nalgebra::Vector4;

use super::omega_signature_matrix::{
    flatten_ordered_matrix, pack_scalars_as_vectors, raw_omega_matrix, signature_order,
};

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "volume_one_omega_signature_matrix",
    canonicalize,
};

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

    let omega = raw_omega_matrix(&shifted);
    let order = signature_order(&omega);
    let flattened = flatten_ordered_matrix(&omega, &order);

    CandidateOutput {
        duals: pack_scalars_as_vectors(&flattened),
        status: "ok",
    }
}
