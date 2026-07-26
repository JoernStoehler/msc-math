//! Retained four-dimensional QP route experiment.
//!
//! The comparison contains scalar-interval, batched, normwise, and staged
//! inverse-defect enclosures plus an empirical control. The verified variants
//! implement Lemmas `lem:kkt-verified-inverse-defect`,
//! `lem:kkt-batched-defect-enclosure`, and
//! `lem:kkt-normwise-defect-enclosure` from
//! `formal/hk2017-qp-precision.tex`.
//! Curvature discovery and cyclic inheritance implement
//! `lem:kkt-certified-curvature-direction` and
//! `lem:kkt-cyclic-obstruction-inheritance` from the same file.
//! The cheap residual/inverse-norm control uses unverified error estimates;
//! exact fallback on its indeterminate cases does not make its determinate
//! decisions sound.
//!
//! nalgebra 0.35 is intentionally an experiment-only second dependency: it
//! supplies the pivoted Bunch--Kaufman LBL^T factorization missing from the
//! repository's current nalgebra 0.33 dependency.

use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, edge_fixture_cases,
    exact_binary64_dual_vertex_arrays, generated_f64_cases,
    selected_route::general::solve_selected_general,
    try_exact_binary64_transition_matrix_assuming_origin_interior, validate_f64_polytope_input,
    F64CapacityMethod, F64CapacityOutcome, F64ValidationPolicy,
};
use nalgebra::{DMatrix, DVector, Vector4};
use nalgebra035::{DMatrix as DMatrix35, DVector as DVector35};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::{BTreeMap, BTreeSet};
use std::hint::black_box;
use std::time::{Duration, Instant};
use symplectic::algorithms::billiard::{
    facet_classification::classify_facets_from_dual_vertices, for_each_sigma_from_facets,
};
use symplectic::algorithms::capacity_4d::{
    check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
    check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, general_capacity,
    PolytopeGeometry4d,
};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::geom::known_polytopes;
use symplectic::geom::known_polytopes::hko_pentagon;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::kkt::projection_solver::solve_projected;
use symplectic::kkt::qp_assembly::{
    build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices,
};
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::kkt::saddle_point_solver::{solve_saddle_point, KktOutcome};
use symplectic::kkt::Verdict;
use symplectic::solve_pruned_hk2017_candidates;

const DEFAULT_SEED: u64 = 99_599_604;
const INERTIA_RELATIVE_FLOOR: f64 = 1e-12;

fn checked_production_geometry(dual_vertices: &[Vector4<f64>]) -> PolytopeGeometry4d {
    check_facet_count(dual_vertices.len()).expect("capacity facet-count bound");
    check_finite_dual_vertices(dual_vertices).expect("finite dual vertices");
    check_dual_vertex_norm_bounds(dual_vertices).expect("capacity dual-vertex norm bounds");
    let geometry =
        exact_binary64_polytope_geometry(dual_vertices).expect("exact polytope geometry");
    check_primal_vertex_norm_bounds(&geometry).expect("capacity primal-vertex norm bounds");
    geometry
}

// These files form one logical module. Keeping the shared experiment state in
// one module avoids public visibility plumbing between tightly coupled
// algorithm variants, while the file boundaries keep each review surface
// small and named by responsibility.
include!("harness/shared.rs");
include!("harness/commands.rs");
include!("harness/heuristic_controls.rs");
include!("harness/numerical_audits.rs");
include!("harness/benchmarks.rs");
include!("harness/general_route.rs");
include!("harness/exact_agreement.rs");
include!("harness/adversarial.rs");
include!("harness/tests.rs");
