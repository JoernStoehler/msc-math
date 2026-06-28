use crate::{candidate_canonicalize, CandidateOutput, CandidateSpec};
use nalgebra::Vector4;

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "analytic_center_rms_sort",
    canonicalize,
};

pub fn canonicalize(duals: &[Vector4<f64>]) -> CandidateOutput {
    candidate_canonicalize(duals)
}
