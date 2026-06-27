use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, exact_binary64_dual_vertex_arrays,
    solve_exact_capacity_for_transition_pruned_sigmas,
    try_exact_binary64_transition_matrix_assuming_origin_interior, ExactCapacityReport,
    F64CapacityMethod, F64CapacityReport, F64ValidationPolicy,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use serde::Serialize;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_pruned_hk2017_candidates, OrbitGuaranteeMode,
};

pub const DEFAULT_SEED: u64 = 42;
pub const DEFAULT_H_MIN: f64 = 0.5;
pub const DEFAULT_H_MAX: f64 = 2.0;
const MAX_ATTEMPTS_PER_SAMPLE: u64 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapacityPath {
    F64TransitionPrunedHk,
    ExactTransitionPrunedF64ThenExactFallback,
    ExactTransitionPrunedSigmas,
}

impl CapacityPath {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "f64_transition_pruned_hk" | "f64" => Ok(Self::F64TransitionPrunedHk),
            "exact_transition_pruned_f64_then_exact_fallback"
            | "pruned_f64_then_exact_fallback"
            | "pruned_hk_exact_fallback"
            | "fallback" => Ok(Self::ExactTransitionPrunedF64ThenExactFallback),
            "exact_transition_pruned_sigmas" | "exact" => Ok(Self::ExactTransitionPrunedSigmas),
            other => Err(format!(
                "--path must be f64_transition_pruned_hk/f64, exact_transition_pruned_f64_then_exact_fallback/fallback, or exact_transition_pruned_sigmas/exact, got {other}"
            )),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::F64TransitionPrunedHk => "f64_transition_pruned_hk",
            Self::ExactTransitionPrunedF64ThenExactFallback => {
                "exact_transition_pruned_f64_then_exact_fallback"
            }
            Self::ExactTransitionPrunedSigmas => "exact_transition_pruned_sigmas",
        }
    }
}

pub struct AcceptedFixture {
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub fixture_attempts: u64,
}

pub struct CapacityFixture {
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub dual_vertices_exact: Vec<[BigRational; 4]>,
    pub transition_is_allowed: DMatrix<bool>,
}

pub struct ExactGeometry {
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub dual_vertices_exact: Vec<[BigRational; 4]>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
}

pub struct FallbackRouteResult {
    pub capacity: f64,
    pub iterations: u64,
    pub raw_orbits: usize,
    pub returned_orbits: usize,
}

pub fn accepted_fixture(
    facet_count: usize,
    sample: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
) -> Result<AcceptedFixture, String> {
    let first_attempt = facet_count as u64 * 1_000_000 + sample as u64 * MAX_ATTEMPTS_PER_SAMPLE;
    for offset in 0..MAX_ATTEMPTS_PER_SAMPLE {
        if let Ok(dual_vertices_f64) =
            generate_dual_vertices(facet_count, h_min, h_max, seed, first_attempt + offset)
        {
            return Ok(AcceptedFixture {
                dual_vertices_f64,
                fixture_attempts: offset + 1,
            });
        }
    }
    Err(format!(
        "no accepted fixture for F={facet_count}, sample={sample}"
    ))
}

pub fn capacity_fixture_from_dual_vertices(
    dual_vertices_f64: Vec<Vector4<f64>>,
) -> Result<CapacityFixture, String> {
    let dual_vertices_exact = exact_binary64_dual_vertex_arrays(&dual_vertices_f64);
    let transition_is_allowed =
        try_exact_binary64_transition_matrix_assuming_origin_interior(&dual_vertices_exact)?;
    Ok(CapacityFixture {
        dual_vertices_f64,
        dual_vertices_exact,
        transition_is_allowed,
    })
}

pub fn exact_geometry_from_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> ExactGeometry {
    let dual_vertices_exact = exact_dual_vertex_arrays(&dual_vertices_f64);
    let dual_vertices_exact_vectors = exact_dual_vertex_vectors(&dual_vertices_exact);
    let PolarVerticesExact {
        vertex_facet_incidence,
        ..
    } = polar_vertices_exact_rational_assuming_origin_interior(&dual_vertices_exact_vectors);
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&dual_vertices_exact_vectors);
    ExactGeometry {
        dual_vertices_f64,
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
    }
}

pub fn exact_transition_pruned_once(
    fixture: &CapacityFixture,
) -> Result<ExactCapacityReport, exp_dev_quadratic_program::ExactCapacityError> {
    solve_exact_capacity_for_transition_pruned_sigmas(
        &fixture.dual_vertices_exact,
        &fixture.transition_is_allowed,
        BigRational::from_integer(0.into()),
    )
}

pub fn pruned_f64_then_exact_once(
    fixture: &CapacityFixture,
) -> Result<FallbackRouteResult, symplectic::OrbitSearchError> {
    pruned_f64_then_exact_with_transition(
        &fixture.dual_vertices_f64,
        &fixture.dual_vertices_exact,
        &fixture.transition_is_allowed,
    )
}

pub fn pruned_f64_then_exact_with_transition(
    dual_vertices_f64: &[Vector4<f64>],
    dual_vertices_exact: &[[BigRational; 4]],
    transition_is_allowed: &DMatrix<bool>,
) -> Result<FallbackRouteResult, symplectic::OrbitSearchError> {
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(dual_vertices_f64, transition_is_allowed)?;
    let raw_orbits = orbits.len();
    let result = aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )?;
    Ok(FallbackRouteResult {
        capacity: result.min_action,
        iterations,
        raw_orbits,
        returned_orbits: result.orbits.len(),
    })
}

pub fn pruned_f64_then_exact_from_geometry(geometry: &ExactGeometry) -> f64 {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &geometry.facet_intersection_is_nonempty,
        &geometry.omega_signs,
    );
    pruned_f64_then_exact_with_transition(
        &geometry.dual_vertices_f64,
        &geometry.dual_vertices_exact,
        &transition_is_allowed,
    )
    .expect("pruned f64 then exact fallback")
    .capacity
}

pub fn f64_transition_pruned_once(fixture: &CapacityFixture) -> F64CapacityReport {
    capacity_f64_only_with_policy_and_method_profiled(
        &fixture.dual_vertices_f64,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    )
    .0
}

pub fn f64_transition_pruned_from_dual_vertices(dual_vertices: &[Vector4<f64>]) -> f64 {
    let (report, _) = capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    match report.outcome {
        exp_dev_quadratic_program::F64CapacityOutcome::Success { capacity, .. } => capacity,
        exp_dev_quadratic_program::F64CapacityOutcome::Failure { reason } => {
            panic!("f64 route failed: {reason:?}")
        }
    }
}

fn exact_dual_vertex_arrays(dual_vertices: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
    dual_vertices
        .iter()
        .map(|a| {
            [
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            ]
        })
        .collect()
}

fn exact_dual_vertex_vectors(
    dual_vertices_exact: &[[BigRational; 4]],
) -> Vec<Vector4<BigRational>> {
    dual_vertices_exact
        .iter()
        .map(|a| Vector4::new(a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()))
        .collect()
}
