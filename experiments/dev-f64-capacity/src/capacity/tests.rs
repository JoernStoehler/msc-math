use super::*;
use crate::geometry::f64_combinatorics;
use crate::{product, F64ValidationPolicy};
use nalgebra::Vector4;
use symplectic::known_polytopes;

#[test]
fn generic_random_row_has_pure_f64_capacity() {
    let dual_vertices = vec![
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
    ];
    let report = capacity_f64_only(&dual_vertices);
    assert!(
        matches!(report.outcome, F64CapacityOutcome::Success { .. }),
        "generic datascience random row should be a basic pure-f64 success case: {report:?}"
    );
}

#[test]
fn hko_is_reported_as_f64_ambiguous() {
    let fixture = known_polytopes::hko_pentagon();
    let combinatorics = f64_combinatorics(&fixture.dual_vertices_f64).expect("HKO f64 geometry");
    assert!(
        combinatorics.omega_indeterminate_count > 0
            || combinatorics.facet_intersection_indeterminate_count > 0,
        "HKO should expose the expected f64 hard-case geometry signal: {combinatorics:?}"
    );
}

#[test]
fn product_billiard_method_reduces_product_sigma_count() {
    let fixture = known_polytopes::lagrangian_triangle_product();
    let dual_vertices = &fixture.dual_vertices_f64;
    let (generic, _) = capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    let (product, _) = capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::ProductBilliardOrHk,
    );

    assert!(matches!(
        generic.outcome,
        F64CapacityOutcome::Success { .. }
    ));
    assert!(matches!(
        product.outcome,
        F64CapacityOutcome::Success { .. }
    ));
    assert!(
        product.sigma_count < generic.sigma_count,
        "product billiard should cut product sigma count: generic {}, product {}",
        generic.sigma_count,
        product.sigma_count
    );
}

#[test]
fn product_billiard_method_tolerates_tiny_off_block_drift() {
    let fixture = known_polytopes::lagrangian_triangle_product();
    let mut dual_vertices = fixture.dual_vertices_f64.clone();
    for vertex in &mut dual_vertices {
        if vertex[2] == 0.0 && vertex[3] == 0.0 {
            vertex[2] = 1e-14;
            vertex[3] = -2e-14;
        } else {
            vertex[0] = -1e-14;
            vertex[1] = 2e-14;
        }
    }

    let (generic, _) = capacity_f64_only_with_policy_and_method_profiled(
        &dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    let rounded = product::round_blocks(&dual_vertices);
    assert!(rounded.should_use_rounded_vertices());
    let (product, _) = capacity_f64_only_with_policy_and_method_profiled(
        &rounded.rounded_dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::ProductBilliardOrHk,
    );

    assert!(matches!(
        product.outcome,
        F64CapacityOutcome::Success { .. }
    ));
    assert!(
        product.sigma_count < generic.sigma_count,
        "tolerant product routing should cut sigma count: generic {}, product {}",
        generic.sigma_count,
        product.sigma_count
    );
}
