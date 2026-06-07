//! E2E prediction smoke tests for the f64 flow-graph work surface.
//!
//! Re-reasoning rule for expected outputs:
//! 1. Identify the fixed polytope fixture or deterministic generated input.
//! 2. Check whether the current flow-graph input assumptions should hold.
//! 3. Check whether f64 diagnostics should be stable.
//! 4. Expected output is one of:
//!    - agrees with QP/HK2017 within the declared tolerance;
//!    - rejects with a typed reason;
//!    - returns indeterminate or another typed non-success;
//!    - diagnostic closed-word error behavior, kept out of agreement buckets.
//!
//! Do not update expected outputs merely because current flow-graph output
//! changed. Update them only after re-running the reasoning above.

use super::{
    capacity_f64, closed_tube_for_sigma_f64, diagnose_f64_closed_words,
    exact::{resolve_closed_word_exact, ExactClosedWordOutcome, ExactFlatTubeInput},
    CapacityF64Error, F64ClosedCycleOutcome, F64TubeError, FlatTubeInput,
};
use crate::algorithms::test_helpers::{flat_facet_data_from_dual_vertices, pruned_capacity};
use crate::geom::known_polytopes::{self, KnownPolytope};
use crate::random::generate_dual_vertices;
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::fmt::Write;

const RANDOM_MASTER_SEED: u64 = 20260605;
const RANDOM_H_MIN: f64 = 0.5;
const RANDOM_H_MAX: f64 = 2.0;
const AGREEMENT_TOLERANCE: f64 = 1e-8;

#[derive(Clone, Copy)]
struct FixtureCase {
    name: &'static str,
    fixture: fn() -> &'static KnownPolytope,
    reason: &'static str,
}

#[derive(Clone, Copy)]
struct RejectionCase {
    fixture: FixtureCase,
    expected_error: F64TubeError,
}

#[derive(Clone, Copy)]
struct GeneratedCase {
    name: &'static str,
    facet_count: usize,
    attempt: u64,
    reason: &'static str,
    expected_closed_word_errors: ExpectedClosedWordErrorSummary,
}

#[derive(Clone, Copy)]
struct ExpectedClosedWordErrorSummary {
    closed_cycle_errors: usize,
    singular_fixed_point_errors: usize,
    indeterminate_polygon_errors: usize,
    exact_empty_tubes: usize,
    exact_zero_action_no_orbits: usize,
    exact_positive_orbits: usize,
}

struct GeneratedCaseData {
    dual_vertices_exact: Vec<[BigRational; 4]>,
    dual_vertices_f64: Vec<Vector4<f64>>,
    facet_intersection_is_nonempty: nalgebra::DMatrix<bool>,
    omega_signs: nalgebra::DMatrix<i8>,
}

fn input_for_fixture(fixture: &KnownPolytope) -> FlatTubeInput<'_> {
    FlatTubeInput::new(
        &fixture.dual_vertices_f64,
        &fixture.facet_intersection_is_nonempty,
        &fixture.omega_signs,
    )
}

fn unsupported_cases() -> Vec<RejectionCase> {
    vec![
        RejectionCase {
            fixture: FixtureCase {
                name: "simplex",
                fixture: known_polytopes::simplex,
                reason: "minimal known fixture; current affine f64 tube path rejects because a geometric transition has exact zero omega",
            },
            expected_error: F64TubeError::UnsupportedZeroOmegaTransition,
        },
        RejectionCase {
            fixture: FixtureCase {
                name: "hypercube",
                fixture: known_polytopes::hypercube,
                reason: "axis-aligned known fixture; current affine f64 tube path rejects zero-omega geometric transitions",
            },
            expected_error: F64TubeError::UnsupportedZeroOmegaTransition,
        },
        RejectionCase {
            fixture: FixtureCase {
                name: "hko_pentagon",
                fixture: known_polytopes::hko_pentagon,
                reason: "HKO has geometrically possible zero-omega transitions, so the affine f64 tube path must reject before returning a capacity-like value",
            },
            expected_error: F64TubeError::UnsupportedZeroOmegaTransition,
        },
        RejectionCase {
            fixture: FixtureCase {
                name: "lagrangian_triangle_product",
                fixture: known_polytopes::lagrangian_triangle_product,
                reason: "Lagrangian product structure creates zero-omega geometric transitions; current flow-graph f64 path intentionally rejects it",
            },
            expected_error: F64TubeError::UnsupportedZeroOmegaTransition,
        },
        RejectionCase {
            fixture: FixtureCase {
                name: "lagrangian_triangle_square",
                fixture: known_polytopes::lagrangian_triangle_square,
                reason: "Lagrangian product structure creates zero-omega geometric transitions; current flow-graph f64 path intentionally rejects it",
            },
            expected_error: F64TubeError::UnsupportedZeroOmegaTransition,
        },
    ]
}

fn generated_near_qp_with_closed_word_error_smoke_cases() -> Vec<GeneratedCase> {
    vec![
        GeneratedCase {
            name: "generated_F5_seed20260605_attempt60",
            facet_count: 5,
            attempt: 60,
            reason: "fresh deterministic random polytope: QP and diagnostic f64 flow-graph currently agree, but diagnostic f64 closed-word errors remain",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 8,
                singular_fixed_point_errors: 4,
                indeterminate_polygon_errors: 4,
                exact_empty_tubes: 1,
                exact_zero_action_no_orbits: 7,
                exact_positive_orbits: 0,
            },
        },
        GeneratedCase {
            name: "generated_F5_seed20260605_attempt73",
            facet_count: 5,
            attempt: 73,
            reason: "second deterministic random polytope with the same near-equality plus diagnostic closed-word error signal",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 11,
                singular_fixed_point_errors: 5,
                indeterminate_polygon_errors: 6,
                exact_empty_tubes: 1,
                exact_zero_action_no_orbits: 10,
                exact_positive_orbits: 0,
            },
        },
        GeneratedCase {
            name: "generated_F5_seed20260605_attempt77",
            facet_count: 5,
            attempt: 77,
            reason: "third deterministic random polytope; guards that near-equality alone is not treated as acceptance while closed-cycle errors remain",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 10,
                singular_fixed_point_errors: 5,
                indeterminate_polygon_errors: 5,
                exact_empty_tubes: 0,
                exact_zero_action_no_orbits: 10,
                exact_positive_orbits: 0,
            },
        },
        GeneratedCase {
            name: "generated_F7_seed20260605_attempt31",
            facet_count: 7,
            attempt: 31,
            reason: "regression polytope: trinary polygon predicates prevent the former false tiny-action candidate; diagnostic f64 closed-word errors remain",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 15,
                singular_fixed_point_errors: 8,
                indeterminate_polygon_errors: 7,
                exact_empty_tubes: 1,
                exact_zero_action_no_orbits: 13,
                exact_positive_orbits: 1,
            },
        },
    ]
}

fn generated_near_qp_with_closed_word_error_f10_cases() -> Vec<GeneratedCase> {
    vec![
        GeneratedCase {
            name: "generated_F10_seed20260605_attempt1",
            facet_count: 10,
            attempt: 1,
            reason: "F10 verification target case: diagnostic f64 and QP capacities agree closely, but near-singular and polygon-indeterminate closed-word errors remain",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 17,
                singular_fixed_point_errors: 8,
                indeterminate_polygon_errors: 9,
                exact_empty_tubes: 4,
                exact_zero_action_no_orbits: 13,
                exact_positive_orbits: 0,
            },
        },
        GeneratedCase {
            name: "generated_F10_seed20260605_attempt2",
            facet_count: 10,
            attempt: 2,
            reason: "second F10 verification target case with close diagnostic f64/QP capacity and diagnostic closed-word errors",
            expected_closed_word_errors: ExpectedClosedWordErrorSummary {
                closed_cycle_errors: 17,
                singular_fixed_point_errors: 8,
                indeterminate_polygon_errors: 9,
                exact_empty_tubes: 1,
                exact_zero_action_no_orbits: 16,
                exact_positive_orbits: 0,
            },
        },
    ]
}

#[test]
fn generated_near_qp_polytopes_match_qp_with_capacity_f64() {
    for case in generated_near_qp_with_closed_word_error_smoke_cases() {
        assert_f64_exact_resolution_matches_qp_for_closed_word_error_case(case);
    }
}

#[test]
#[ignore = "F10 capacity_f64 verification is part of the deliberate verification suite, not the default dev loop"]
fn generated_f10_near_qp_polytopes_match_qp_with_capacity_f64() {
    for case in generated_near_qp_with_closed_word_error_f10_cases() {
        assert_f64_exact_resolution_matches_qp_for_closed_word_error_case(case);
    }
}

fn assert_f64_exact_resolution_matches_qp_for_closed_word_error_case(case: GeneratedCase) {
    let generated = generate_case_data(case);
    let input = input_for_generated_case(&generated);
    let exact_input = exact_input_for_generated_case(&generated);
    input
        .validate_no_geometric_zero_omega_transitions()
        .unwrap_or_else(|error| {
            panic!(
                "{}: expected current generic input check to pass, got {error:?}",
                case.name
            )
        });
    let qp = pruned_capacity(
        &generated.dual_vertices_f64,
        &generated.dual_vertices_exact,
        &generated.facet_intersection_is_nonempty,
        &generated.omega_signs,
    )
    .unwrap_or_else(|error| panic!("{}: QP reference failed: {error:?}", case.name));
    let flow = diagnose_f64_closed_words(&input, 0.0).unwrap_or_else(|error| {
        panic!(
            "{}: expected f64 diagnostic search to return a candidate plus closed-cycle diagnostics, got polytope rejection {error:?}",
            case.name
        )
    });
    let qp_capacity = qp.capacity();
    let fg_capacity = flow.best_action.unwrap_or_else(|| {
        panic!(
            "{}: expected current f64 search to expose a candidate action",
            case.name
        )
    });
    let relative_error = ((fg_capacity - qp_capacity) / qp_capacity).abs();
    assert!(
        relative_error <= AGREEMENT_TOLERANCE,
        "{}: expected near-QP equality before final acceptance; qp={qp_capacity}, fg={fg_capacity}, relative_error={relative_error}. reason: {}",
        case.name,
        case.reason
    );
    assert!(
        flow.closed_cycle_error_count() > 0,
        "{}: this polytope is intentionally not an accepted agreement case until closed-cycle errors are resolved. reason: {}",
        case.name,
        case.reason
    );
    assert_eq!(
        flow.closed_cycle_error_count(),
        case.expected_closed_word_errors.closed_cycle_errors,
        "{}: closed-cycle error count changed. This may be an improvement or a regression, but the prediction bucket must be reinterpreted before updating the expected count.",
        case.name
    );
    let resolved = capacity_f64(&input, &exact_input, 0.0)
        .unwrap_or_else(|error| panic!("{}: capacity_f64 rejected: {error:?}", case.name));
    let resolved_relative_error = ((resolved.capacity_action - qp_capacity) / qp_capacity).abs();
    assert!(
        resolved_relative_error <= AGREEMENT_TOLERANCE,
        "{}: capacity_f64 does not match QP; qp={qp_capacity}, resolved={}, relative_error={resolved_relative_error}",
        case.name,
        resolved.capacity_action
    );
}

#[test]
fn exact_closed_word_resolver_clears_generated_closed_word_error_polytopes() {
    for case in generated_near_qp_with_closed_word_error_smoke_cases() {
        assert_exact_closed_word_resolver_clears_generated_closed_word_error_polytope(case);
    }
}

#[test]
#[ignore = "F10 exact resolution verification is part of the deliberate verification suite, not the default dev loop"]
fn exact_closed_word_resolver_clears_generated_f10_closed_word_error_polytopes() {
    for case in generated_near_qp_with_closed_word_error_f10_cases() {
        assert_exact_closed_word_resolver_clears_generated_closed_word_error_polytope(case);
    }
}

fn assert_exact_closed_word_resolver_clears_generated_closed_word_error_polytope(
    case: GeneratedCase,
) {
    let generated = generate_case_data(case);
    let input = input_for_generated_case(&generated);
    let exact_input = exact_input_for_generated_case(&generated);
    let qp = pruned_capacity(
        &generated.dual_vertices_f64,
        &generated.dual_vertices_exact,
        &generated.facet_intersection_is_nonempty,
        &generated.omega_signs,
    )
    .unwrap_or_else(|error| panic!("{}: QP reference failed: {error:?}", case.name));
    let qp_capacity = qp.capacity();
    let flow = diagnose_f64_closed_words(&input, 0.0).unwrap_or_else(|error| {
        panic!(
            "{}: expected f64 search to produce diagnostics, got {error:?}",
            case.name
        )
    });

    let best_orbit = flow
        .orbits
        .iter()
        .min_by(|left, right| left.action.total_cmp(&right.action))
        .unwrap_or_else(|| panic!("{}: expected a f64 best orbit", case.name));
    assert_exact_positive_action_matches(case.name, &exact_input, &best_orbit.facets, qp_capacity);

    let mut resolved_errors = 0usize;
    let mut summary = ObservedClosedWordErrorSummary::default();
    for record in &flow.closed_cycles {
        let F64ClosedCycleOutcome::Error(error) = record.outcome else {
            continue;
        };
        resolved_errors += 1;
        summary.add_f64_error(error.error);
        let (result, _) =
            resolve_closed_word_exact(&exact_input, &record.sigma).unwrap_or_else(|error| {
                panic!(
                    "{}: exact closed-word resolver failed for {:?}: {error:?}",
                    case.name, record.sigma
                )
            });
        match result.outcome {
            ExactClosedWordOutcome::EmptyTube => {
                summary.exact_empty_tubes += 1;
            }
            ExactClosedWordOutcome::ZeroActionNoOrbit { .. } => {
                summary.exact_zero_action_no_orbits += 1;
            }
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                summary.exact_positive_orbits += 1;
                let action = action.to_f64().expect("exact action to f64");
                assert!(
                        action + AGREEMENT_TOLERANCE >= qp_capacity,
                        "{}: diagnostic f64 error word {:?} exact-resolved to lower action {action} than QP capacity {qp_capacity}",
                        case.name,
                        record.sigma
                    );
            }
            ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => {
                panic!(
                    "{}: diagnostic f64 error word {:?} exact-resolved to unsupported positive singular",
                    case.name, record.sigma
                );
            }
        }
    }
    assert!(
        resolved_errors > 0,
        "{}: expected this prediction bucket to contain f64 closed-cycle errors",
        case.name
    );
    assert_eq!(
        summary,
        ObservedClosedWordErrorSummary::from_expected(case.expected_closed_word_errors),
        "{}: diagnostic f64 error words exact-resolved with an unexpected composition.\nExpected: {}\nObserved: {}\nThis may be an implementation improvement, a regression, or a changed fixture. Reinterpret before updating the test.",
        case.name,
        ObservedClosedWordErrorSummary::from_expected(case.expected_closed_word_errors).display(),
        summary.display()
    );
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ObservedClosedWordErrorSummary {
    singular_fixed_point_errors: usize,
    indeterminate_polygon_errors: usize,
    exact_empty_tubes: usize,
    exact_zero_action_no_orbits: usize,
    exact_positive_orbits: usize,
}

impl ObservedClosedWordErrorSummary {
    fn from_expected(expected: ExpectedClosedWordErrorSummary) -> Self {
        Self {
            singular_fixed_point_errors: expected.singular_fixed_point_errors,
            indeterminate_polygon_errors: expected.indeterminate_polygon_errors,
            exact_empty_tubes: expected.exact_empty_tubes,
            exact_zero_action_no_orbits: expected.exact_zero_action_no_orbits,
            exact_positive_orbits: expected.exact_positive_orbits,
        }
    }

    fn add_f64_error(&mut self, error: F64TubeError) {
        match error {
            F64TubeError::SingularFixedPointMap => self.singular_fixed_point_errors += 1,
            F64TubeError::NumericallyIndeterminatePolygon => {
                self.indeterminate_polygon_errors += 1;
            }
            other => panic!("unexpected f64 closed-cycle error in prediction bucket: {other:?}"),
        }
    }

    fn display(self) -> String {
        let mut text = String::new();
        write!(
            &mut text,
            "f64 singular={}, f64 polygon_indeterminate={}, exact_empty={}, exact_zero={}, exact_positive={}",
            self.singular_fixed_point_errors,
            self.indeterminate_polygon_errors,
            self.exact_empty_tubes,
            self.exact_zero_action_no_orbits,
            self.exact_positive_orbits
        )
        .expect("write to String");
        text
    }
}

#[test]
fn former_false_tiny_action_word_is_indeterminate() {
    let case = GeneratedCase {
        name: "generated_F7_seed20260605_attempt31_sigma_0_4_2_6",
        facet_count: 7,
        attempt: 31,
        reason: "this closed word formerly produced a false action near 1e-8 from a numerically indeterminate polygon",
        expected_closed_word_errors: ExpectedClosedWordErrorSummary {
            closed_cycle_errors: 15,
            singular_fixed_point_errors: 8,
            indeterminate_polygon_errors: 7,
            exact_empty_tubes: 1,
            exact_zero_action_no_orbits: 13,
            exact_positive_orbits: 1,
        },
    };
    let generated = generate_case_data(case);
    let input = input_for_generated_case(&generated);

    assert_eq!(
        closed_tube_for_sigma_f64(&input, &[0, 4, 2, 6], f64::INFINITY),
        Err(F64TubeError::NumericallyIndeterminatePolygon),
        "{}: reason: {}",
        case.name,
        case.reason
    );
}

fn generate_case_data(case: GeneratedCase) -> GeneratedCaseData {
    let dual_vertices_f64 = generate_dual_vertices(
        case.facet_count,
        RANDOM_H_MIN,
        RANDOM_H_MAX,
        RANDOM_MASTER_SEED,
        case.attempt,
    )
    .unwrap_or_else(|error| panic!("{}: random generator failed: {error:?}", case.name));
    let (dual_vertices_exact, facet_intersection_is_nonempty, omega_signs) =
        flat_facet_data_from_dual_vertices(&dual_vertices_f64);
    GeneratedCaseData {
        dual_vertices_exact,
        dual_vertices_f64,
        facet_intersection_is_nonempty,
        omega_signs,
    }
}

fn input_for_generated_case(generated: &GeneratedCaseData) -> FlatTubeInput<'_> {
    FlatTubeInput::new(
        &generated.dual_vertices_f64,
        &generated.facet_intersection_is_nonempty,
        &generated.omega_signs,
    )
}

fn exact_input_for_generated_case(generated: &GeneratedCaseData) -> ExactFlatTubeInput<'_> {
    ExactFlatTubeInput {
        dual_vertices: &generated.dual_vertices_exact,
        facet_intersection_is_nonempty: &generated.facet_intersection_is_nonempty,
        omega_signs: &generated.omega_signs,
    }
}

fn assert_exact_positive_action_matches(
    case_name: &str,
    exact_input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    expected_capacity: f64,
) {
    let (result, _) = resolve_closed_word_exact(exact_input, sigma).unwrap_or_else(|error| {
        panic!("{case_name}: exact best-word resolver failed for {sigma:?}: {error:?}")
    });
    match result.outcome {
        ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
            let action = action.to_f64().expect("exact action to f64");
            let relative_error = ((action - expected_capacity) / expected_capacity).abs();
            assert!(
                relative_error <= AGREEMENT_TOLERANCE,
                "{case_name}: exact best-word action {action} does not match QP capacity {expected_capacity}; relative_error={relative_error}"
            );
        }
        other => {
            panic!("{case_name}: expected exact orbit with action > 0 for {sigma:?}, got {other:?}")
        }
    }
}

#[test]
fn unsupported_cases_reject_with_predicted_reason() {
    for case in unsupported_cases() {
        let fixture = (case.fixture.fixture)();
        let input = input_for_fixture(fixture);
        let exact_input = exact_input_for_fixture(fixture);
        assert_eq!(
            capacity_f64(&input, &exact_input, 0.0),
            Err(CapacityF64Error::Numerical(case.expected_error)),
            "{}: reason: {}",
            case.fixture.name,
            case.fixture.reason
        );
        crate::algorithms::test_helpers::pruned_capacity_for_fixture(fixture).unwrap_or_else(
            |error| {
                panic!(
                    "{}: QP should still provide the reference output even when f64 flow-graph rejects: {error:?}",
                    case.fixture.name
                )
            },
        );
    }
}

fn exact_input_for_fixture(fixture: &KnownPolytope) -> ExactFlatTubeInput<'_> {
    ExactFlatTubeInput {
        dual_vertices: &fixture.dual_vertices,
        facet_intersection_is_nonempty: &fixture.facet_intersection_is_nonempty,
        omega_signs: &fixture.omega_signs,
    }
}
