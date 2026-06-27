use crate::fallback_route::aggregate_certified_orbits_with_local_exact_fallback;
use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
    solve_exact_capacity_for_transition_pruned_sigmas,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::Zero;
use symplectic::{solve_orbit_sigma_saddle_point, CertifiedOrbitSetMode};

/// Demonstrates the candidate-retention limit of exact fallback.
///
/// Exact fallback can certify every sigma it is given and still fail to compute
/// the global capacity if the f64 candidate filter never retained the exact
/// minimizer. The fallback is exact over the retained set, not over a sigma
/// stream it never sees.
#[test]
fn exact_fallback_over_retained_candidates_is_not_global_certification() {
    let dual_vertices = pruning_roundoff_fixture();
    let exact_vertices = exact_binary64_dual_vertex_arrays(&dual_vertices);
    let exact_transition =
        exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
    let exact_all_visited = solve_exact_capacity_for_transition_pruned_sigmas(
        &exact_vertices,
        &exact_transition,
        BigRational::zero(),
    )
    .expect("small exact reference");
    assert_eq!(exact_all_visited.minimizers[0].sigma, vec![0, 3, 1, 4, 2]);

    let retained_sigma = vec![0, 4, 3, 1, 2];
    let retained_candidate = solve_orbit_sigma_saddle_point(&dual_vertices, &retained_sigma)
        .expect("retained candidate solves in f64");
    let retained_exact = aggregate_certified_orbits_with_local_exact_fallback(
        &exact_vertices,
        vec![retained_candidate],
        1,
        BigRational::zero(),
        CertifiedOrbitSetMode::MinimizersOnly,
    )
    .expect("exact fallback certifies the retained candidate set");

    assert_eq!(retained_exact.minimizers[0].sigma, retained_sigma);
    assert!(
        retained_exact.capacity > exact_all_visited.capacity + 1.0,
        "retained-set exact fallback should be exact but not global here: retained_set={:.17}, exact_all_visited={:.17}",
        retained_exact.capacity,
        exact_all_visited.capacity
    );
    assert_eq!(
        retained_exact.exact_resolutions, 1,
        "the retained-set fallback had no way to discover the missing exact minimizer"
    );
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
