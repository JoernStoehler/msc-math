use crate::geometry::{f64_combinatorics, F64Combinatorics, F64Predicate};
use crate::{
    capacity_f64_only, exact_binary64_dual_vertex_arrays,
    exact_binary64_transition_matrix_assuming_origin_interior,
    solve_exact_capacity_for_transition_pruned_sigmas, F64CapacityOutcome,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::Zero;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;

/// Demonstrates the first repair after literal f64 pruning fails.
///
/// The route below still uses f64, but it no longer prunes a transition merely
/// because an equality-scale predicate is not literally true. Facet
/// intersections classified as `Indeterminate` are kept in the transition
/// graph. This preserves the exact minimizer for the literal-pruning failure
/// fixture, but does not by itself make f64 capacity a trusted route.
#[test]
fn conservative_transition_graph_keeps_literal_pruning_miss() {
    let dual_vertices = pruning_roundoff_fixture();
    let exact = exact_reference(&dual_vertices);
    assert_eq!(exact.minimizers[0].sigma, vec![0, 3, 1, 4, 2]);

    let combinatorics = f64_combinatorics(&dual_vertices).expect("f64 combinatorics");
    let transition = conservative_transition_matrix(&combinatorics, dual_vertices.len());

    assert!(
        transition[(1, 4)],
        "conservative f64 pruning must keep edge 1->4 because this edge is the exact minimizer edge missed by literal f64 pruning; facet_intersection={:?}, omega_sign={}",
        combinatorics.facet_intersections[(1, 4)],
        combinatorics.omega_signs[(1, 4)]
    );
    assert!(
        SimpleDirectedCyclesCanonical::new(&transition)
            .any(|sigma| sigma == exact.minimizers[0].sigma),
        "conservative f64 pruning should enumerate the exact minimizer sigma {:?}",
        exact.minimizers[0].sigma
    );

    let report = capacity_f64_only(&dual_vertices);
    let F64CapacityOutcome::Success {
        capacity,
        sigma: ref reported_sigma,
    } = report.outcome
    else {
        panic!("conservative f64 route should return a value on this fixture: {report:?}");
    };
    assert_eq!(*reported_sigma, exact.minimizers[0].sigma);
    assert!(
        (capacity - exact.capacity).abs() < 1e-12,
        "conservative pruning fixes this pruning miss; any remaining f64 trust issue must come from a different failure class: f64={capacity:.17}, exact={:.17}, report={report:?}",
        exact.capacity
    );
}

fn conservative_transition_matrix(
    combinatorics: &F64Combinatorics,
    facet_count: usize,
) -> DMatrix<bool> {
    DMatrix::from_fn(facet_count, facet_count, |i, j| {
        combinatorics.facet_intersections[(i, j)] != F64Predicate::False
            && combinatorics.omega_signs[(i, j)] >= 0
    })
}

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

fn exact_reference(dual_vertices: &[Vector4<f64>]) -> crate::ExactCapacityReport {
    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let transition = exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    solve_exact_capacity_for_transition_pruned_sigmas(
        &exact_vertices,
        &transition,
        BigRational::zero(),
    )
    .expect("exact reference capacity")
}
