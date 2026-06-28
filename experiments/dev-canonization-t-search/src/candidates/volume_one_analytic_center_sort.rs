use crate::{
    analytic_center, compare_vectors_lexicographically_for_candidates, translate_duals,
    volume_one_duals_f64, CandidateOutput, CandidateSpec,
};
use nalgebra::Vector4;

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "volume_one_analytic_center_sort",
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

    let mut sorted = shifted;
    sorted.sort_by(compare_vectors_lexicographically_for_candidates);
    CandidateOutput {
        duals: sorted,
        status: "ok",
    }
}
