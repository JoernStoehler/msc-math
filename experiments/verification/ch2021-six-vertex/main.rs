//! Exact reproduction gate for the CH2021 rational six-vertex equality body.
//!
//! The exhaustive exact HK2017 active-word calculation is authoritative.
//! Current pruned and unpruned f64 routes are secondary regression checks.

use euclidean_polytopes::{
    all_points_are_extreme_exact, origin_in_interior_of_conv_exact, polar_vertices_exact_rational,
    two_faces_from_vertex_facet_incidence, volume_from_incidence_exact, volume_from_incidence_f64,
};
use nalgebra::{DMatrix, Vector4};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::{
    for_each_sigma_pruned_by_transition, for_each_sigma_unpruned_facet_count,
    solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates,
};
use symplectic::algorithms::{aggregate_orbits_with_dual_vertices_exact, OrbitGuaranteeMode};
use symplectic::exact::{
    exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega0, omega_signs_exact,
};
use symplectic::kkt::rational_solver::solve_kkt_exact;

const EXPECTED_EXACT_WORDS: u64 = 125_664;
const F64_RELATIVE_TOLERANCE: f64 = 1.0e-9;
const VOLUME_RELATIVE_TOLERANCE: f64 = 1.0e-12;
const PRODUCER_COMMAND: &str =
    "cargo run -p dev-capacity-validation --release --bin ch2021-six-vertex";

#[derive(Serialize)]
struct Report {
    schema_version: u32,
    experiment_id: &'static str,
    producer: Producer,
    sources: Sources,
    conventions: Conventions,
    geometry: Geometry,
    face_diagnostics: FaceDiagnostics,
    exact_hk2017: ExactEnumeration,
    f64_checks: F64Checks,
    gates: Gates,
    claim_boundary: ClaimBoundary,
}

#[derive(Serialize)]
struct Producer {
    command: &'static str,
    git_revision: String,
    git_dirty: bool,
    git_status_porcelain: Vec<String>,
}

#[derive(Serialize)]
struct Sources {
    ch_coordinates: &'static str,
    ch_two_face_criterion: &'static str,
    hk_capacity_formula: &'static str,
    exact_problem_contract: &'static str,
}

#[derive(Serialize)]
struct Conventions {
    coordinate_order: &'static str,
    symplectic_form: &'static str,
    translation: &'static str,
    polar_inequality: &'static str,
    exact_value_encoding: &'static str,
    exact_capacity_rule: &'static str,
    f64_role: &'static str,
    f64_relative_tolerance: f64,
}

#[derive(Serialize)]
struct Geometry {
    source_vertices: Vec<Vec<String>>,
    source_coordinates_finite: bool,
    affine_rank: usize,
    all_source_points_extreme: bool,
    centroid: Vec<String>,
    translated_vertices: Vec<Vec<String>>,
    translated_origin_interior: bool,
    facet_count: usize,
    dual_vertices: Vec<Vec<String>>,
    distinct_dual_vertices: bool,
    vertex_facet_incidence: Vec<Vec<bool>>,
    incidence_shape: [usize; 2],
    exact_round_trip_vertices: bool,
    exact_round_trip_incidence: bool,
    volume_exact: String,
    volume_f64: f64,
    volume_exact_f64: f64,
    volume_absolute_difference: f64,
    volume_relative_tolerance: f64,
    volume_crosscheck_passed: bool,
}

#[derive(Serialize)]
struct FaceDiagnostics {
    omega_sign_matrix: Vec<Vec<i8>>,
    actual_two_face_count: usize,
    zero_actual_ridges: Vec<FaceRecord>,
    nonzero_actual_ridges: Vec<FaceRecord>,
    zero_non_ridge_pairs: Vec<NonRidgeZeroPair>,
    ordinary_ch_combinatorial_flow_well_posed: bool,
}

#[derive(Serialize)]
struct FaceRecord {
    facets: [usize; 2],
    vertices: Vec<usize>,
    omega_exact: String,
    omega_sign: i8,
}

#[derive(Serialize)]
struct NonRidgeZeroPair {
    facets: [usize; 2],
    shared_vertices: Vec<usize>,
    intersection_nonempty: bool,
    omega_exact: String,
}

#[derive(Serialize)]
struct ExactEnumeration {
    expected_word_count: u64,
    visited_word_count: u64,
    visited_by_length: Vec<u64>,
    solver_error_count: u64,
    no_positive_kkt_witness_count: u64,
    kkt_witness_count: u64,
    positive_q_witness_count: u64,
    zero_q_witness_count: u64,
    negative_q_witness_count: u64,
    q_max_exact: String,
    capacity_exact: String,
    capacity_exact_f64: f64,
    capacity_squared_exact: String,
    twice_volume_exact: String,
    systolic_ratio_exact: String,
    maximizer_count: usize,
    maximizers: Vec<ExactMaximizer>,
    visit_count_passed: bool,
    positive_witness_passed: bool,
    capacity_volume_identity_passed: bool,
}

#[derive(Clone, Serialize)]
struct ExactMaximizer {
    sigma: Vec<usize>,
    beta_exact: Vec<String>,
    q_exact: String,
    action_exact: String,
}

struct RawExactMaximizer {
    sigma: Vec<usize>,
    beta: Vec<BigRational>,
    q: BigRational,
}

#[derive(Serialize)]
struct F64Checks {
    role: &'static str,
    tolerance_rule: &'static str,
    pruned: F64Route,
    unpruned: F64Route,
}

#[derive(Serialize)]
struct F64Route {
    status: String,
    independently_enumerated_words: u64,
    solver_iterations: Option<u64>,
    raw_retained_candidates: Option<usize>,
    aggregated_retained_candidates: Option<usize>,
    admissible_f64_candidates: Option<usize>,
    admissible_exact_candidates: Option<usize>,
    indeterminate_candidates: Option<usize>,
    min_action: Option<f64>,
    min_action_lower: Option<f64>,
    min_action_upper: Option<f64>,
    best_sigma: Option<Vec<usize>>,
    absolute_difference_from_exact: Option<f64>,
    agrees_with_exact: bool,
}

#[derive(Serialize)]
struct Gates {
    source_geometry: bool,
    exact_polar_and_round_trip: bool,
    exact_volume_and_f64_crosscheck: bool,
    exhaustive_exact_hk2017: bool,
    exact_capacity_volume_identity: bool,
    pruned_f64_secondary_check: bool,
    unpruned_f64_secondary_check: bool,
    overall_passed: bool,
}

#[derive(Serialize)]
struct ClaimBoundary {
    certifies: Vec<&'static str>,
    does_not_certify: Vec<&'static str>,
    lagrangian_two_face_effect: &'static str,
    stop_rule: &'static str,
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = manifest_dir.join("ch2021-six-vertex/report.json");
    let producer = git_provenance(manifest_dir);

    let source_vertices = source_vertices();
    let source_coordinates_finite = source_vertices
        .iter()
        .flat_map(|vertex| vertex.iter())
        .all(|coordinate| !coordinate.denom().is_zero());
    let affine_rank = affine_rank(&source_vertices);
    let all_source_points_extreme = all_points_are_extreme_exact(&source_vertices);
    let centroid = arithmetic_centroid(&source_vertices);
    let translated_vertices: Vec<_> = source_vertices
        .iter()
        .map(|vertex| vertex - &centroid)
        .collect();
    let translated_origin_interior = origin_in_interior_of_conv_exact(&translated_vertices);

    assert_eq!(affine_rank, 4, "source vertices must have affine rank four");
    assert!(
        all_source_points_extreme,
        "all six source points must be extreme"
    );
    assert!(
        translated_origin_interior,
        "translated origin must be interior"
    );

    let polar = polar_vertices_exact_rational(&translated_vertices);
    let facet_count = polar.vertices.len();
    let distinct_dual_vertices = all_vectors_distinct(&polar.vertices);
    assert_eq!(facet_count, 9, "CH body must reconstruct with nine facets");
    assert!(
        distinct_dual_vertices,
        "polar dual vertices must be distinct"
    );

    let incidence = DMatrix::from_fn(source_vertices.len(), facet_count, |vertex, facet| {
        polar.vertex_facet_incidence[(facet, vertex)]
    });
    let round_trip = exact_vertices_with_incidence(&polar.vertices)
        .unwrap_or_else(|error| panic!("exact dual round-trip failed: {error:?}"));
    let (exact_round_trip_vertices, exact_round_trip_incidence) = compare_round_trip(
        &translated_vertices,
        &incidence,
        &round_trip.vertices,
        &round_trip.vertex_facet_incidence,
    );

    let volume_exact = volume_from_incidence_exact(&translated_vertices, &incidence);
    assert!(volume_exact.is_positive(), "exact volume must be positive");
    let translated_vertices_f64 = vectors_to_f64(&translated_vertices);
    let volume_f64 = volume_from_incidence_f64(&translated_vertices_f64, &incidence)
        .expect("f64 incidence volume must be finite");
    let volume_exact_f64 = rational_to_f64(&volume_exact);
    let volume_absolute_difference = (volume_f64 - volume_exact_f64).abs();
    let volume_crosscheck_passed =
        relative_agreement(volume_f64, volume_exact_f64, VOLUME_RELATIVE_TOLERANCE);

    let omega_signs = omega_signs_exact(&polar.vertices);
    let facet_intersections = facet_intersection_is_nonempty_exact(&incidence);
    let (zero_actual_ridges, nonzero_actual_ridges, zero_non_ridge_pairs) = face_diagnostics(
        &polar.vertices,
        &incidence,
        &omega_signs,
        &facet_intersections,
    );

    let dual_arrays: Vec<[BigRational; 4]> = polar
        .vertices
        .iter()
        .map(|vertex| std::array::from_fn(|coordinate| vertex[coordinate].clone()))
        .collect();
    let (exact_hk2017, capacity_exact) = exhaustive_exact_hk2017(&dual_arrays, &volume_exact);

    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &facet_intersections,
        &omega_signs,
    );
    let dual_f64 = vectors_to_f64(&polar.vertices);
    let exact_capacity_f64 = rational_to_f64(&capacity_exact);
    let pruned_enumerated_words = count_pruned_words(&transition);
    let pruned = run_f64_route(
        &dual_arrays,
        pruned_enumerated_words,
        exact_capacity_f64,
        || solve_pruned_hk2017_candidates(&dual_f64, &transition),
    );
    let unpruned = run_f64_route(
        &dual_arrays,
        EXPECTED_EXACT_WORDS,
        exact_capacity_f64,
        || solve_unpruned_hk2017_candidates(&dual_f64),
    );

    let source_geometry = source_coordinates_finite
        && affine_rank == 4
        && all_source_points_extreme
        && translated_origin_interior;
    let exact_polar_and_round_trip = facet_count == 9
        && distinct_dual_vertices
        && incidence.shape() == (6, 9)
        && exact_round_trip_vertices
        && exact_round_trip_incidence;
    let exact_volume_and_f64_crosscheck = volume_exact.is_positive() && volume_crosscheck_passed;
    let exhaustive_exact_hk2017 =
        exact_hk2017.visit_count_passed && exact_hk2017.positive_witness_passed;
    let exact_capacity_volume_identity = exact_hk2017.capacity_volume_identity_passed;
    let pruned_f64_secondary_check = pruned.agrees_with_exact;
    let unpruned_f64_secondary_check = unpruned.agrees_with_exact;
    let overall_passed = source_geometry
        && exact_polar_and_round_trip
        && exact_volume_and_f64_crosscheck
        && exhaustive_exact_hk2017
        && exact_capacity_volume_identity
        && pruned_f64_secondary_check
        && unpruned_f64_secondary_check;

    let report = Report {
        schema_version: 1,
        experiment_id: "E3-CH2021-six-vertex-exact-reproduction",
        producer,
        sources: Sources {
            ch_coordinates: "papers/ch2021/s1_introduction_and_main_results.tex (6-vertex polytopes)",
            ch_two_face_criterion: "papers/ch2021/s3_reeb_dynamics_on_polytopes.tex (Lagrangian 2-face criterion)",
            hk_capacity_formula: "papers/hk2017/EHZ-polytopes.tex (Theorem 1.1)",
            exact_problem_contract: "formal/hk2017-qp-core.tex (def:hk2017-problem)",
        },
        conventions: Conventions {
            coordinate_order: "(q1,q2,p1,p2), directly corresponding to CH's (x1,x2,y1,y2)",
            symplectic_form: "omega0(u,v)=u_q1*v_p1-u_p1*v_q1+u_q2*v_p2-u_p2*v_q2",
            translation: "subtract the exact arithmetic mean of the six displayed vertices",
            polar_inequality: "<a_i,x> <= 1 with returned polar vertices used without negation",
            exact_value_encoding: "canonical BigRational display strings",
            exact_capacity_rule: "visit all active words, Qmax=max positive exact KKT witness, c=1/(2*Qmax)",
            f64_role: "secondary current-evaluator regression check only",
            f64_relative_tolerance: F64_RELATIVE_TOLERANCE,
        },
        geometry: Geometry {
            source_vertices: serialize_vectors(&source_vertices),
            source_coordinates_finite,
            affine_rank,
            all_source_points_extreme,
            centroid: serialize_vector(&centroid),
            translated_vertices: serialize_vectors(&translated_vertices),
            translated_origin_interior,
            facet_count,
            dual_vertices: serialize_vectors(&polar.vertices),
            distinct_dual_vertices,
            vertex_facet_incidence: serialize_matrix(&incidence),
            incidence_shape: [incidence.nrows(), incidence.ncols()],
            exact_round_trip_vertices,
            exact_round_trip_incidence,
            volume_exact: ratio_string(&volume_exact),
            volume_f64,
            volume_exact_f64,
            volume_absolute_difference,
            volume_relative_tolerance: VOLUME_RELATIVE_TOLERANCE,
            volume_crosscheck_passed,
        },
        face_diagnostics: FaceDiagnostics {
            omega_sign_matrix: serialize_matrix(&omega_signs),
            actual_two_face_count: zero_actual_ridges.len() + nonzero_actual_ridges.len(),
            ordinary_ch_combinatorial_flow_well_posed: zero_actual_ridges.is_empty(),
            zero_actual_ridges,
            nonzero_actual_ridges,
            zero_non_ridge_pairs,
        },
        exact_hk2017,
        f64_checks: F64Checks {
            role: "secondary checks against exhaustive exact HK2017",
            tolerance_rule: "abs(f64-exact) <= 1e-9 * max(1, abs(exact))",
            pruned,
            unpruned,
        },
        gates: Gates {
            source_geometry,
            exact_polar_and_round_trip,
            exact_volume_and_f64_crosscheck,
            exhaustive_exact_hk2017,
            exact_capacity_volume_identity,
            pruned_f64_secondary_check,
            unpruned_f64_secondary_check,
            overall_passed,
        },
        claim_boundary: ClaimBoundary {
            certifies: vec![
                "the displayed rational body is a full-dimensional six-vertex, nine-facet convex polytope with the reported exact incidence and volume",
                "its exact EHZ capacity is the reported value by exhaustive active-word HK2017 Theorem 1.1 enumeration",
                "its systolic ratio is exactly one when the exact capacity-volume identity passes",
            ],
            does_not_certify: vec![
                "the paper's claimed families",
                "combinatorial Zollness or open-dense coverage by minimizing orbits",
                "local maximality, a stabilizer, orientation response, or a nearby landscape",
                "well-posedness of CH2021's ordinary combinatorial Reeb flow when an actual Lagrangian 2-face exists",
            ],
            lagrangian_two_face_effect: "zero pairing on an actual 2-face limits the CH ordinary-flow claim but does not invalidate the general HK2017 capacity formula",
            stop_rule: "stop after this one body; do not run orientation, perturbation, family, or adaptive successor evaluations",
        },
    };

    write_report(&output_path, &report);
    println!("wrote {}", output_path.display());
    println!(
        "exact words={} Qmax={} capacity={} volume={} c^2=2V={} overall_passed={}",
        report.exact_hk2017.visited_word_count,
        report.exact_hk2017.q_max_exact,
        report.exact_hk2017.capacity_exact,
        report.geometry.volume_exact,
        report.exact_hk2017.capacity_volume_identity_passed,
        overall_passed,
    );

    if !overall_passed {
        std::process::exit(1);
    }
}

fn source_vertices() -> Vec<Vector4<BigRational>> {
    [
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
        [0, -1, 1, 0],
        [-1, -1, 0, 1],
    ]
    .into_iter()
    .map(|coordinates| {
        Vector4::new(
            integer_ratio(coordinates[0]),
            integer_ratio(coordinates[1]),
            integer_ratio(coordinates[2]),
            integer_ratio(coordinates[3]),
        )
    })
    .collect()
}

fn integer_ratio(value: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(value))
}

fn arithmetic_centroid(vertices: &[Vector4<BigRational>]) -> Vector4<BigRational> {
    let mut sum = Vector4::from_element(BigRational::zero());
    for vertex in vertices {
        sum += vertex;
    }
    sum / integer_ratio(vertices.len() as i64)
}

fn affine_rank(vertices: &[Vector4<BigRational>]) -> usize {
    let mut matrix: Vec<Vec<BigRational>> = (0..4)
        .map(|row| {
            (1..vertices.len())
                .map(|column| vertices[column][row].clone() - vertices[0][row].clone())
                .collect()
        })
        .collect();
    let (rows, columns) = (matrix.len(), matrix[0].len());
    let mut pivot_row = 0;
    for column in 0..columns {
        let Some(pivot) = (pivot_row..rows).find(|&row| !matrix[row][column].is_zero()) else {
            continue;
        };
        matrix.swap(pivot_row, pivot);
        let pivot_value = matrix[pivot_row][column].clone();
        for entry in &mut matrix[pivot_row][column..] {
            *entry /= pivot_value.clone();
        }
        for row in 0..rows {
            if row == pivot_row || matrix[row][column].is_zero() {
                continue;
            }
            let factor = matrix[row][column].clone();
            for current_column in column..columns {
                let reduction = factor.clone() * matrix[pivot_row][current_column].clone();
                matrix[row][current_column] -= reduction;
            }
        }
        pivot_row += 1;
        if pivot_row == rows {
            break;
        }
    }
    pivot_row
}

fn all_vectors_distinct(vertices: &[Vector4<BigRational>]) -> bool {
    (0..vertices.len())
        .all(|left| (left + 1..vertices.len()).all(|right| vertices[left] != vertices[right]))
}

fn compare_round_trip(
    expected_vertices: &[Vector4<BigRational>],
    expected_incidence: &DMatrix<bool>,
    actual_vertices: &[Vector4<BigRational>],
    actual_incidence: &DMatrix<bool>,
) -> (bool, bool) {
    if expected_vertices.len() != actual_vertices.len()
        || expected_incidence.shape() != actual_incidence.shape()
    {
        return (false, false);
    }

    let mut actual_rows = BTreeSet::new();
    let mut vertices_match = true;
    let mut incidence_match = true;
    for (expected_row, expected_vertex) in expected_vertices.iter().enumerate() {
        let positions: Vec<_> = actual_vertices
            .iter()
            .enumerate()
            .filter_map(|(row, vertex)| (vertex == expected_vertex).then_some(row))
            .collect();
        if positions.len() != 1 {
            vertices_match = false;
            incidence_match = false;
            continue;
        }
        let actual_row = positions[0];
        actual_rows.insert(actual_row);
        for facet in 0..expected_incidence.ncols() {
            incidence_match &=
                expected_incidence[(expected_row, facet)] == actual_incidence[(actual_row, facet)];
        }
    }
    vertices_match &= actual_rows.len() == actual_vertices.len();
    (vertices_match, incidence_match)
}

fn face_diagnostics(
    dual_vertices: &[Vector4<BigRational>],
    incidence: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
    intersections: &DMatrix<bool>,
) -> (Vec<FaceRecord>, Vec<FaceRecord>, Vec<NonRidgeZeroPair>) {
    let two_faces = two_faces_from_vertex_facet_incidence(incidence);
    let ridge_pairs: BTreeSet<[usize; 2]> = two_faces.iter().map(|face| face.facets).collect();
    let mut zero_actual_ridges = Vec::new();
    let mut nonzero_actual_ridges = Vec::new();

    for face in two_faces {
        let pairing = omega0(
            &dual_vertices[face.facets[0]],
            &dual_vertices[face.facets[1]],
        );
        let record = FaceRecord {
            facets: face.facets,
            vertices: face.vertices,
            omega_exact: ratio_string(&pairing),
            omega_sign: omega_signs[(face.facets[0], face.facets[1])],
        };
        if pairing.is_zero() {
            zero_actual_ridges.push(record);
        } else {
            nonzero_actual_ridges.push(record);
        }
    }

    let mut zero_non_ridge_pairs = Vec::new();
    for left in 0..dual_vertices.len() {
        for right in left + 1..dual_vertices.len() {
            if omega_signs[(left, right)] != 0 || ridge_pairs.contains(&[left, right]) {
                continue;
            }
            let shared_vertices = (0..incidence.nrows())
                .filter(|&vertex| incidence[(vertex, left)] && incidence[(vertex, right)])
                .collect();
            zero_non_ridge_pairs.push(NonRidgeZeroPair {
                facets: [left, right],
                shared_vertices,
                intersection_nonempty: intersections[(left, right)],
                omega_exact: ratio_string(&omega0(&dual_vertices[left], &dual_vertices[right])),
            });
        }
    }

    (
        zero_actual_ridges,
        nonzero_actual_ridges,
        zero_non_ridge_pairs,
    )
}

fn exhaustive_exact_hk2017(
    dual_vertices: &[[BigRational; 4]],
    volume_exact: &BigRational,
) -> (ExactEnumeration, BigRational) {
    let mut visited_word_count = 0u64;
    let mut visited_by_length = vec![0u64; dual_vertices.len() + 1];
    let mut kkt_witness_count = 0u64;
    let mut positive_q_witness_count = 0u64;
    let mut zero_q_witness_count = 0u64;
    let mut negative_q_witness_count = 0u64;
    let mut q_max: Option<BigRational> = None;
    let mut maximizers = Vec::<RawExactMaximizer>::new();

    for_each_sigma_unpruned_facet_count(dual_vertices.len(), |sigma| {
        visited_word_count += 1;
        visited_by_length[sigma.len()] += 1;
        let Some(witness) = solve_kkt_exact(dual_vertices, sigma) else {
            return;
        };
        kkt_witness_count += 1;
        if witness.q_exact.is_positive() {
            positive_q_witness_count += 1;
            match &q_max {
                None => {
                    q_max = Some(witness.q_exact.clone());
                    maximizers.push(RawExactMaximizer {
                        sigma: sigma.to_vec(),
                        beta: witness.beta,
                        q: witness.q_exact,
                    });
                }
                Some(current) if witness.q_exact > *current => {
                    q_max = Some(witness.q_exact.clone());
                    maximizers.clear();
                    maximizers.push(RawExactMaximizer {
                        sigma: sigma.to_vec(),
                        beta: witness.beta,
                        q: witness.q_exact,
                    });
                }
                Some(current) if witness.q_exact == *current => {
                    maximizers.push(RawExactMaximizer {
                        sigma: sigma.to_vec(),
                        beta: witness.beta,
                        q: witness.q_exact,
                    });
                }
                Some(_) => {}
            }
        } else if witness.q_exact.is_zero() {
            zero_q_witness_count += 1;
        } else {
            negative_q_witness_count += 1;
        }
    });

    let q_max = q_max.expect("exhaustive exact enumeration found no positive KKT witness");
    let capacity_exact = BigRational::one() / (integer_ratio(2) * q_max.clone());
    maximizers.sort_by(|left, right| left.sigma.cmp(&right.sigma));
    let maximizers: Vec<_> = maximizers
        .into_iter()
        .map(|maximizer| ExactMaximizer {
            sigma: maximizer.sigma,
            beta_exact: maximizer.beta.iter().map(ratio_string).collect(),
            q_exact: ratio_string(&maximizer.q),
            action_exact: ratio_string(&capacity_exact),
        })
        .collect();
    let capacity_squared = capacity_exact.clone() * capacity_exact.clone();
    let twice_volume = integer_ratio(2) * volume_exact.clone();
    let systolic_ratio = capacity_squared.clone() / twice_volume.clone();
    let visit_count_passed = visited_word_count == EXPECTED_EXACT_WORDS;
    let positive_witness_passed = positive_q_witness_count > 0;
    let capacity_volume_identity_passed = capacity_squared == twice_volume;
    let no_positive_kkt_witness_count = visited_word_count - positive_q_witness_count;

    (
        ExactEnumeration {
            expected_word_count: EXPECTED_EXACT_WORDS,
            visited_word_count,
            visited_by_length,
            solver_error_count: 0,
            no_positive_kkt_witness_count,
            kkt_witness_count,
            positive_q_witness_count,
            zero_q_witness_count,
            negative_q_witness_count,
            q_max_exact: ratio_string(&q_max),
            capacity_exact: ratio_string(&capacity_exact),
            capacity_exact_f64: rational_to_f64(&capacity_exact),
            capacity_squared_exact: ratio_string(&capacity_squared),
            twice_volume_exact: ratio_string(&twice_volume),
            systolic_ratio_exact: ratio_string(&systolic_ratio),
            maximizer_count: maximizers.len(),
            maximizers,
            visit_count_passed,
            positive_witness_passed,
            capacity_volume_identity_passed,
        },
        capacity_exact,
    )
}

fn count_pruned_words(transition: &DMatrix<bool>) -> u64 {
    let mut count = 0u64;
    for_each_sigma_pruned_by_transition(transition, |_| count += 1);
    count
}

fn run_f64_route<F>(
    dual_vertices_exact: &[[BigRational; 4]],
    independently_enumerated_words: u64,
    capacity_exact_f64: f64,
    solve: F,
) -> F64Route
where
    F: FnOnce() -> Result<(Vec<symplectic::OrbitKktData>, u64), symplectic::OrbitSearchError>,
{
    let (raw_orbits, iterations) = match solve() {
        Ok(result) => result,
        Err(error) => {
            return F64Route {
                status: format!("solve_error:{error:?}"),
                independently_enumerated_words,
                solver_iterations: None,
                raw_retained_candidates: None,
                aggregated_retained_candidates: None,
                admissible_f64_candidates: None,
                admissible_exact_candidates: None,
                indeterminate_candidates: None,
                min_action: None,
                min_action_lower: None,
                min_action_upper: None,
                best_sigma: None,
                absolute_difference_from_exact: None,
                agrees_with_exact: false,
            };
        }
    };
    let raw_retained_candidates = raw_orbits.len();
    let aggregated = match aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        raw_orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    ) {
        Ok(result) => result,
        Err(error) => {
            return F64Route {
                status: format!("aggregation_error:{error:?}"),
                independently_enumerated_words,
                solver_iterations: Some(iterations),
                raw_retained_candidates: Some(raw_retained_candidates),
                aggregated_retained_candidates: None,
                admissible_f64_candidates: None,
                admissible_exact_candidates: None,
                indeterminate_candidates: None,
                min_action: None,
                min_action_lower: None,
                min_action_upper: None,
                best_sigma: None,
                absolute_difference_from_exact: None,
                agrees_with_exact: false,
            };
        }
    };

    let admissible_f64_candidates = aggregated
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                symplectic::OrbitAdmissibility::AdmissibleF64
            )
        })
        .count();
    let admissible_exact_candidates = aggregated
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                symplectic::OrbitAdmissibility::AdmissibleExact
            )
        })
        .count();
    let indeterminate_candidates = aggregated
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                symplectic::OrbitAdmissibility::IndeterminateF64
            )
        })
        .count();
    let difference = (aggregated.min_action - capacity_exact_f64).abs();
    let agrees_with_exact = relative_agreement(
        aggregated.min_action,
        capacity_exact_f64,
        F64_RELATIVE_TOLERANCE,
    ) && indeterminate_candidates == 0
        && iterations == independently_enumerated_words;

    F64Route {
        status: "ok".to_owned(),
        independently_enumerated_words,
        solver_iterations: Some(iterations),
        raw_retained_candidates: Some(raw_retained_candidates),
        aggregated_retained_candidates: Some(aggregated.orbits.len()),
        admissible_f64_candidates: Some(admissible_f64_candidates),
        admissible_exact_candidates: Some(admissible_exact_candidates),
        indeterminate_candidates: Some(indeterminate_candidates),
        min_action: Some(aggregated.min_action),
        min_action_lower: Some(aggregated.min_action_lower),
        min_action_upper: Some(aggregated.min_action_upper),
        best_sigma: Some(aggregated.best_sigma().to_vec()),
        absolute_difference_from_exact: Some(difference),
        agrees_with_exact,
    }
}

fn vectors_to_f64(vertices: &[Vector4<BigRational>]) -> Vec<Vector4<f64>> {
    vertices
        .iter()
        .map(|vertex| {
            Vector4::new(
                rational_to_f64(&vertex[0]),
                rational_to_f64(&vertex[1]),
                rational_to_f64(&vertex[2]),
                rational_to_f64(&vertex[3]),
            )
        })
        .collect()
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().expect("small rational must convert to f64")
}

fn relative_agreement(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance * right.abs().max(1.0)
}

fn ratio_string(value: &BigRational) -> String {
    value.to_string()
}

fn serialize_vector(vector: &Vector4<BigRational>) -> Vec<String> {
    vector.iter().map(ratio_string).collect()
}

fn serialize_vectors(vectors: &[Vector4<BigRational>]) -> Vec<Vec<String>> {
    vectors.iter().map(serialize_vector).collect()
}

fn serialize_matrix<T: Clone>(matrix: &DMatrix<T>) -> Vec<Vec<T>> {
    (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols())
                .map(|column| matrix[(row, column)].clone())
                .collect()
        })
        .collect()
}

fn git_provenance(manifest_dir: &Path) -> Producer {
    let revision = git_output(manifest_dir, &["rev-parse", "HEAD"]);
    let status = git_output(
        manifest_dir,
        &["status", "--porcelain", "--untracked-files=all"],
    );
    let git_status_porcelain: Vec<_> = status
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    Producer {
        command: PRODUCER_COMMAND,
        git_revision: revision.trim().to_owned(),
        git_dirty: !git_status_porcelain.is_empty(),
        git_status_porcelain,
    }
}

fn git_output(manifest_dir: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(manifest_dir)
        .args(arguments)
        .output()
        .expect("run git for report provenance");
    assert!(output.status.success(), "git provenance command failed");
    String::from_utf8(output.stdout).expect("git output must be UTF-8")
}

fn write_report(path: &PathBuf, report: &Report) {
    let file = File::create(path).expect("create CH report");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, report).expect("serialize CH report");
    writer.write_all(b"\n").expect("terminate CH report");
    writer.flush().expect("flush CH report");
}
