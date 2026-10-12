use crate::{
    capacity_f64_only_with_policy_and_method_profiled, F64CapacityMethod, F64CapacityOutcome,
    F64CapacityReport, F64ValidationPolicy,
};
use symplectic::known_polytopes;

/// Demonstrates that product/billiard routing is a product-specialized
/// enumeration speedup, not a different one-sigma QP solver.
///
/// On this clean product fixture both routes return the same f64 scalar. The
/// product route enumerates fewer sigmas before calling the same f64 KKT
/// candidate logic.
#[test]
fn product_billiard_route_reduces_product_sigma_count() {
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

    let generic_capacity = success_capacity(&generic);
    let product_capacity = success_capacity(&product);
    assert!(
        (generic_capacity - fixture.capacity).abs() < 1e-12,
        "generic HK route should return the product fixture capacity: {generic:?}"
    );
    assert!(
        (product_capacity - fixture.capacity).abs() < 1e-12,
        "product billiard route should return the product fixture capacity: {product:?}"
    );
    assert!(
        (generic_capacity - product_capacity).abs() < 1e-12,
        "this demo is about enumeration count, not a scalar disagreement"
    );
    assert!(
        product.sigma_count < generic.sigma_count,
        "product billiard should visit fewer sigmas on product inputs: generic {}, product {}",
        generic.sigma_count,
        product.sigma_count
    );
}

fn success_capacity(report: &F64CapacityReport) -> f64 {
    match &report.outcome {
        F64CapacityOutcome::Success { capacity, .. } => *capacity,
        F64CapacityOutcome::Failure { .. } => {
            panic!("expected f64 route success, got {report:?}")
        }
    }
}
