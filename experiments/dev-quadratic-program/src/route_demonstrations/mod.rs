//! Executable demonstrations of why tempting simpler capacity routes fail.
//!
//! See `README.md` in this directory for the packet purpose, inclusion rule,
//! non-goals, current coverage list, and fallback-mode framing.

mod beta_margin_indeterminate;
mod conservative_pruning_count_blowup;
mod conservative_pruning_still_f64;
mod f64_value_not_certificate;
mod fallback_guarantee_modes_have_different_scopes;
mod guarded_route_safe_refusal;
mod literal_f64_pruning;
mod lp_transition_policy_no_edge_advantage;
mod near_redundant_removal_is_bounded_surrogate;
mod near_singular_kkt_false_positive;
mod product_billiard_reduces_product_sigma_count;
mod product_rounding_changes_input;
mod q_error_bound_not_certificate;
mod retained_candidate_fallback_limit;
mod unpruned_enumeration_count_blowup;

use nalgebra::Vector4;

fn pruning_roundoff_fixture() -> Vec<Vector4<f64>> {
    vec![
        Vector4::new(
            -0.7609176562997226,
            -0.5842245470076217,
            -0.6093220693528425,
            0.07216780853507296,
        ),
        Vector4::new(
            0.784069284213464,
            -0.5531443877418841,
            0.18211913477611671,
            -0.36079445513926356,
        ),
        Vector4::new(
            -0.043547885416314415,
            0.8556529705333096,
            0.8361784175796745,
            0.2857765173406991,
        ),
        Vector4::new(
            -0.2753007640820361,
            -0.48381690655215637,
            -0.8235951274500787,
            0.35426171198575546,
        ),
        Vector4::new(
            -0.12602783596581424,
            0.6516682410783413,
            0.1098373351502524,
            -0.5152232850628169,
        ),
    ]
}
