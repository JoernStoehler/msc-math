use crate::geometry::{f64_combinatorics, F64Combinatorics, F64Predicate};
use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
};
use nalgebra::DMatrix;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::known_polytopes;

/// Demonstrates the operational limit of conservative f64 transition pruning.
///
/// Keeping f64-indeterminate transitions is the right local repair for the
/// literal-pruning failure, but it is not free. On this HKO fixture, f64 omega
/// signs are ambiguous on many ordered facet pairs. Treating those pairs as
/// allowed expands the transition graph and the HK sigma stream before any KKT
/// solve or exact fallback can help.
#[test]
fn conservative_pruning_can_expand_sigma_stream_before_kkt() {
    let dual_vertices = &known_polytopes::hko_pentagon().dual_vertices_f64;
    let combinatorics = f64_combinatorics(dual_vertices).expect("HKO f64 combinatorics");
    assert_eq!(combinatorics.facet_intersection_indeterminate_count, 0);
    assert_eq!(combinatorics.omega_indeterminate_count, 30);

    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let exact_transition =
        exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    let conservative_transition =
        conservative_transition_matrix(&combinatorics, dual_vertices.len());

    assert_eq!(true_count(&exact_transition), 47);
    assert_eq!(true_count(&conservative_transition), 60);

    let exact_cycles = cycle_count(&exact_transition);
    let conservative_cycles = cycle_count(&conservative_transition);
    assert_eq!(exact_cycles, 7_606);
    assert_eq!(conservative_cycles, 11_862);
    assert!(
        conservative_cycles > exact_cycles + 4_000,
        "conservative pruning preserved ambiguous transitions but enlarged the sigma stream: exact={exact_cycles}, conservative={conservative_cycles}"
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

fn true_count(matrix: &DMatrix<bool>) -> usize {
    matrix.iter().filter(|&&entry| entry).count()
}

fn cycle_count(transition: &DMatrix<bool>) -> usize {
    SimpleDirectedCyclesCanonical::new(transition).count()
}
