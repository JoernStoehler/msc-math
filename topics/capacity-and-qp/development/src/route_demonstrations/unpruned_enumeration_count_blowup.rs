use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
};
use symplectic::algorithms::hk2017::{
    for_each_sigma_unpruned_facet_count, SimpleDirectedCyclesCanonical,
};
use symplectic::known_polytopes;

/// Demonstrates why unpruned HK enumeration is only a reference route.
///
/// This is deliberately count-only: wall-clock timing is machine-dependent.
/// The blowup happens before any KKT solve, exact fallback, or numerical
/// decision logic can help.
#[test]
fn unpruned_enumeration_count_explodes_before_kkt() {
    assert_eq!(unpruned_active_word_count(6), 409);
    assert_eq!(unpruned_active_word_count(8), 16_064);
    assert_eq!(unpruned_active_word_count(10), 1_112_073);
    assert_eq!(unpruned_active_word_count(12), 119_481_284);
    assert_eq!(unpruned_active_word_count(16), 3_809_950_976_992);

    let enumerated_f6 = count_by_enumerating_unpruned(6);
    assert_eq!(
        enumerated_f6,
        unpruned_active_word_count(6),
        "the closed-form count should match the public unpruned enumerator on a cheap facet count"
    );

    let hko = known_polytopes::hko_pentagon();
    let dual_vertices = &hko.dual_vertices_f64;
    assert_eq!(dual_vertices.len(), 10);

    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let exact_transition =
        exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    let pruned_count = SimpleDirectedCyclesCanonical::new(&exact_transition).count() as u128;
    let unpruned_count = unpruned_active_word_count(dual_vertices.len());

    assert_eq!(pruned_count, 7_606);
    assert_eq!(unpruned_count, 1_112_073);
    assert!(
        unpruned_count > 100 * pruned_count,
        "HKO F=10 count-only comparison: unpruned {unpruned_count}, exact-pruned {pruned_count}"
    );
}

fn count_by_enumerating_unpruned(facet_count: usize) -> u128 {
    let mut count = 0u128;
    for_each_sigma_unpruned_facet_count(facet_count, |_| count += 1);
    count
}

fn unpruned_active_word_count(facet_count: usize) -> u128 {
    (2..=facet_count)
        .map(|word_len| binomial(facet_count, word_len) * factorial(word_len - 1))
        .sum()
}

fn binomial(n: usize, k: usize) -> u128 {
    let k = k.min(n - k);
    (0..k).fold(1u128, |acc, i| acc * (n - i) as u128 / (i + 1) as u128)
}

fn factorial(n: usize) -> u128 {
    (1..=n).fold(1u128, |acc, item| acc * item as u128)
}
