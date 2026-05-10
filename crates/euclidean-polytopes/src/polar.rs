use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Matrix4, Vector4};

use crate::f64_geometry::{validate_finite_vectors4, F64GeometryError};
use crate::linalg::{combinations4, dot4_exact, solve4_exact};
use crate::predicates::origin_in_interior_of_conv_exact;

/// Exact polar vertices and exact vertex-facet incidence.
#[derive(Clone, Debug, PartialEq)]
pub struct PolarVertexData<T> {
    pub vertices: Vec<Vector4<T>>,
    pub incidence: DMatrix<bool>,
}

/// Approximate incidence relation for an accepted `f64` vertex.
#[derive(Clone, Debug, PartialEq)]
pub struct IncidenceF64 {
    pub vertex_index: usize,
    pub facet_index: usize,
    pub signed_gap: f64,
    pub signed_gap_abs_error_bound: f64,
}

/// Approximate candidate whose vertex status cannot be decided in `f64`.
#[derive(Clone, Debug, PartialEq)]
pub struct IndeterminatePolarCandidateF64 {
    pub tuple: [usize; 4],
    pub vertex: Option<Vector4<f64>>,
    pub coordinate_abs_error_bound: f64,
}

/// Diagnostic result for approximate polar vertex enumeration.
///
/// `vertices` is partial when `indeterminate_candidates` is non-empty. A
/// candidate is indeterminate when the solve is near singular, a
/// halfspace-membership sign is inside its absolute error bound, or duplicate
/// detection would depend on that bound.
#[derive(Clone, Debug, PartialEq)]
pub struct PolarVerticesF64 {
    pub vertices: Vec<Vector4<f64>>,
    pub coordinate_abs_error_bound: f64,
    pub incidence: Vec<IncidenceF64>,
    pub indeterminate_candidates: Vec<IndeterminatePolarCandidateF64>,
}

/// Enumerate vertices of `{ y in R^4 : <v_i, y> <= 1 }` exactly.
///
/// Checked precondition: `0 in int conv(vertices)`. This condition makes the
/// normalized polar full-dimensional and bounded. The input points do not have
/// to be non-redundant; redundant points add redundant inequalities and do not
/// change the returned exact vertex set.
///
/// Panics when the origin-interior contract is violated.
pub fn polar_vertices_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
) -> PolarVertexData<T> {
    assert!(
        origin_in_interior_of_conv_exact(vertices),
        "polar_vertices_exact requires 0 in int conv(vertices)"
    );

    let one = T::one();
    let rhs = Vector4::new(one.clone(), one.clone(), one.clone(), one.clone());
    let mut polar_vertices = Vec::new();

    for tuple in combinations4(vertices.len()) {
        let rows = tuple.map(|idx| vertices[idx].clone());
        let Some(candidate) = solve4_exact(&rows, &rhs) else {
            continue;
        };

        if vertices
            .iter()
            .all(|vertex| dot4_exact(vertex, &candidate) <= one)
            && !polar_vertices.iter().any(|known| known == &candidate)
        {
            polar_vertices.push(candidate);
        }
    }

    assert!(
        !polar_vertices.is_empty(),
        "origin-interior polar input produced no exact vertices"
    );

    let incidence = DMatrix::from_fn(polar_vertices.len(), vertices.len(), |row, col| {
        dot4_exact(&vertices[col], &polar_vertices[row]) == one
    });

    PolarVertexData {
        vertices: polar_vertices,
        incidence,
    }
}

/// Enumerate well-conditioned polar vertex candidates from `f64` inequalities.
///
/// This diagnostic routine validates finite input and then enumerates 4-tuples
/// of active inequalities. It does not guess near floating-point boundaries:
/// tuples with a near-singular solve, uncertain halfspace membership, or
/// uncertain duplicate classification are reported in
/// `indeterminate_candidates`.
pub fn polar_vertices_f64(vertices: &[Vector4<f64>]) -> Result<PolarVerticesF64, F64GeometryError> {
    validate_finite_vectors4("vertices", vertices)?;

    let mut polar_vertices = Vec::new();
    let mut incidence = Vec::new();
    let mut indeterminate_candidates = Vec::new();
    let mut coordinate_abs_error_bound: f64 = 0.0;

    for tuple in combinations4(vertices.len()) {
        let Some(candidate) = solve_tuple_f64(vertices, tuple) else {
            indeterminate_candidates.push(IndeterminatePolarCandidateF64 {
                tuple,
                vertex: None,
                coordinate_abs_error_bound: 0.0,
            });
            continue;
        };

        coordinate_abs_error_bound =
            coordinate_abs_error_bound.max(candidate.coordinate_abs_error_bound);

        match classify_candidate_f64(vertices, tuple, &candidate.vertex) {
            CandidateClassification::Rejected => {}
            CandidateClassification::Indeterminate => {
                indeterminate_candidates.push(IndeterminatePolarCandidateF64 {
                    tuple,
                    vertex: Some(candidate.vertex),
                    coordinate_abs_error_bound: candidate.coordinate_abs_error_bound,
                });
            }
            CandidateClassification::Accepted(active_incidence) => {
                if duplicate_is_indeterminate(
                    &polar_vertices,
                    &candidate.vertex,
                    candidate
                        .coordinate_abs_error_bound
                        .max(coordinate_abs_error_bound),
                ) {
                    indeterminate_candidates.push(IndeterminatePolarCandidateF64 {
                        tuple,
                        vertex: Some(candidate.vertex),
                        coordinate_abs_error_bound: candidate.coordinate_abs_error_bound,
                    });
                    continue;
                }

                if polar_vertices
                    .iter()
                    .any(|known| known == &candidate.vertex)
                {
                    continue;
                }

                let vertex_index = polar_vertices.len();
                polar_vertices.push(candidate.vertex);
                incidence.extend(active_incidence.into_iter().map(|facet| IncidenceF64 {
                    vertex_index,
                    facet_index: facet,
                    signed_gap: 0.0,
                    signed_gap_abs_error_bound: candidate.coordinate_abs_error_bound,
                }));
            }
        }
    }

    Ok(PolarVerticesF64 {
        vertices: polar_vertices,
        coordinate_abs_error_bound,
        incidence,
        indeterminate_candidates,
    })
}

#[derive(Clone, Debug)]
struct CandidateF64 {
    vertex: Vector4<f64>,
    coordinate_abs_error_bound: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CandidateClassification {
    Accepted(Vec<usize>),
    Rejected,
    Indeterminate,
}

fn solve_tuple_f64(vertices: &[Vector4<f64>], tuple: [usize; 4]) -> Option<CandidateF64> {
    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const ERROR_SCALE: f64 = 1.0e4;

    let matrix = Matrix4::new(
        vertices[tuple[0]][0],
        vertices[tuple[0]][1],
        vertices[tuple[0]][2],
        vertices[tuple[0]][3],
        vertices[tuple[1]][0],
        vertices[tuple[1]][1],
        vertices[tuple[1]][2],
        vertices[tuple[1]][3],
        vertices[tuple[2]][0],
        vertices[tuple[2]][1],
        vertices[tuple[2]][2],
        vertices[tuple[2]][3],
        vertices[tuple[3]][0],
        vertices[tuple[3]][1],
        vertices[tuple[3]][2],
        vertices[tuple[3]][3],
    );
    let svd = matrix.svd(true, true);
    let singular_values = svd.singular_values;
    let sigma_min = singular_values[0]
        .min(singular_values[1])
        .min(singular_values[2])
        .min(singular_values[3]);
    let sigma_max = singular_values[0]
        .max(singular_values[1])
        .max(singular_values[2])
        .max(singular_values[3]);

    if sigma_min == 0.0 {
        return None;
    }

    let kappa = sigma_max / sigma_min;
    if !kappa.is_finite() || EPS_MACH * kappa > 0.25 {
        return None;
    }

    let ones = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let Ok(vertex) = svd.solve(&ones, 0.0) else {
        return None;
    };
    if vertex.iter().any(|coordinate| !coordinate.is_finite()) {
        return None;
    }

    let coordinate_abs_error_bound = ERROR_SCALE * kappa * EPS_MACH * (vertex.norm() + 1.0);
    if !coordinate_abs_error_bound.is_finite() {
        return None;
    }

    Some(CandidateF64 {
        vertex,
        coordinate_abs_error_bound,
    })
}

fn classify_candidate_f64(
    vertices: &[Vector4<f64>],
    tuple: [usize; 4],
    candidate: &Vector4<f64>,
) -> CandidateClassification {
    let mut active = tuple.to_vec();

    for (facet, vertex) in vertices.iter().enumerate() {
        if tuple.contains(&facet) {
            continue;
        }

        let signed_gap = 1.0 - vertex.dot(candidate);
        let signed_gap_abs_error_bound = signed_gap_abs_error_bound(vertex, candidate);
        if !signed_gap.is_finite() || !signed_gap_abs_error_bound.is_finite() {
            return CandidateClassification::Indeterminate;
        }

        if signed_gap + signed_gap_abs_error_bound < 0.0 {
            return CandidateClassification::Rejected;
        }
        if signed_gap - signed_gap_abs_error_bound <= 0.0 {
            return CandidateClassification::Indeterminate;
        }
    }

    active.sort_unstable();
    CandidateClassification::Accepted(active)
}

fn signed_gap_abs_error_bound(facet: &Vector4<f64>, candidate: &Vector4<f64>) -> f64 {
    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const ERROR_SCALE: f64 = 1.0e4;

    ERROR_SCALE * EPS_MACH * (facet.norm() * candidate.norm() + facet.dot(candidate).abs() + 1.0)
}

fn duplicate_is_indeterminate(
    known_vertices: &[Vector4<f64>],
    candidate: &Vector4<f64>,
    coordinate_abs_error_bound: f64,
) -> bool {
    known_vertices.iter().any(|known| {
        if known == candidate {
            return false;
        }

        (0..4)
            .map(|coordinate| (known[coordinate] - candidate[coordinate]).abs())
            .fold(0.0, f64::max)
            <= coordinate_abs_error_bound
    })
}
