use crate::CandidateSpec;

pub mod analytic_center_rms_sort;
pub mod omega_signature_matrix;
pub mod symplectic_frame_coordinates;
pub mod volume_one_analytic_center_sort;
pub mod volume_one_omega_labeled_symplectic_frame;
pub mod volume_one_omega_signature_matrix;
pub mod volume_one_symplectic_frame_min;

pub fn all() -> Vec<CandidateSpec> {
    vec![
        analytic_center_rms_sort::SPEC,
        volume_one_analytic_center_sort::SPEC,
        volume_one_omega_labeled_symplectic_frame::SPEC,
    ]
}

pub fn invariant_representatives() -> Vec<CandidateSpec> {
    vec![
        omega_signature_matrix::SPEC,
        volume_one_omega_signature_matrix::SPEC,
    ]
}
