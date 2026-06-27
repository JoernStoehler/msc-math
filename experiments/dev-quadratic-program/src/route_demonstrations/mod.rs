//! Executable demonstrations of why tempting simpler capacity routes fail.
//!
//! Nobody currently plans to import this code. Future consumers are expected to
//! read and run these tests, then copy-edit the small route fragments if they
//! need a simpler heuristic and do not care about the missing numerical
//! guarantees.
//!
//! Keep each file focused on one simplification or failure class. Do not build
//! a route-by-fixture dashboard here; add an ad-hoc test when a new interaction
//! matters.
//!
//! Current executable rungs:
//!
//! - `literal_f64_pruning`: literal f64 predicates can silently prune a real
//!   transition and return the wrong capacity.
//! - `conservative_pruning_still_f64`: keeping indeterminate transitions fixes
//!   that pruning miss on the same fixture.
//! - `conservative_pruning_count_blowup`: keeping indeterminate transitions can
//!   substantially increase the sigma stream before any KKT solve runs.
//! - `beta_margin_indeterminate`: a literal f64 `beta > 0` check can reject an
//!   exactly positive KKT point whose smallest beta is below the route's f64
//!   decision scale.
//! - `f64_value_not_certificate`: a correct-looking f64 scalar can still leave
//!   the minimizing set undecided.
//! - `retained_candidate_fallback_limit`: exact fallback over retained
//!   candidates is exact only for the candidate set it receives.
//! - `guarded_route_safe_refusal`: guarded routes should reject or request
//!   fallback rather than inventing a scalar on invalid/ambiguous inputs.
//!
//! The cost demonstration for the exact transition-pruned reference route lives
//! in `experiments/performance/src/bin/capacity_route_costs.rs`, because it
//! needs runtime and hardware context instead of a unit-test assertion.

mod beta_margin_indeterminate;
mod conservative_pruning_count_blowup;
mod conservative_pruning_still_f64;
mod f64_value_not_certificate;
mod guarded_route_safe_refusal;
mod literal_f64_pruning;
mod retained_candidate_fallback_limit;

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
