use crate::exact_binary64_dual_vertex_arrays;
use num_traits::ToPrimitive;
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::{known_polytopes, solve_orbit_sigma_saddle_point, OrbitAdmissibility};

/// Demonstrates that the current f64 `q_error_bound` is not a certificate
/// against exact rational arithmetic on the binary64 input.
///
/// The bound stored by the f64 KKT solver measures the residual correction for
/// the floating-point linear solve. It does not include all rounding effects
/// needed to prove that the reported `q` encloses the exact binary64-rational
/// KKT value.
#[test]
fn current_q_error_bound_does_not_cover_exact_binary64_q() {
    let dual_vertices = &known_polytopes::hko_pentagon().dual_vertices_f64;
    let sigma = vec![0, 1, 7, 3, 9, 5];

    let f64_orbit = solve_orbit_sigma_saddle_point(dual_vertices, &sigma)
        .expect("f64 KKT solve should return a candidate");
    assert_eq!(f64_orbit.admissibility, OrbitAdmissibility::AdmissibleF64);
    assert!(
        f64_orbit.beta_margin > 0.1,
        "this is not a beta-margin edge case: {f64_orbit:?}"
    );

    let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
    let exact = solve_kkt_exact(&exact_vertices, &sigma).expect("exact binary64 KKT solve");
    let exact_q = exact
        .q_exact
        .to_f64()
        .expect("fixture q value should fit in f64");
    let q_abs_error = (f64_orbit.q - exact_q).abs();

    assert!(
        q_abs_error > f64_orbit.q_error_bound,
        "the current q_error_bound should not be mistaken for a total binary64-exact error bound: q_abs_error={q_abs_error:.3e}, q_error_bound={:.3e}",
        f64_orbit.q_error_bound
    );
    assert!(
        q_abs_error < 1e-12,
        "the scalar can still be a good heuristic even when the stored bound is not a certificate: q_abs_error={q_abs_error:.3e}"
    );
}
