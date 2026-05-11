use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Vector4};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::time::Instant;
use tracing::{info, info_span};

use crate::linalg::{combinations4, dot4_exact, solve4_exact};
use crate::predicates::origin_in_interior_of_conv_exact;

/// Exact vertices of a normalized polar polytope and their input-facet incidence.
#[derive(Clone, Debug, PartialEq)]
pub struct PolarVerticesExact<T: ExactScalar + 'static> {
    pub vertices: Vec<Vector4<T>>,
    pub vertex_facet_incidence: DMatrix<bool>,
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
) -> PolarVerticesExact<T> {
    let span = info_span!("polar_vertices_exact", input_points = vertices.len());
    let _span_guard = span.enter();

    let validation_start = Instant::now();
    let origin_is_interior = origin_in_interior_of_conv_exact(vertices);
    let validation_ms = ms(validation_start);
    assert!(
        origin_is_interior,
        "polar_vertices_exact requires 0 in int conv(vertices)"
    );

    polar_vertices_exact_generic_impl(vertices, validation_ms)
}

/// Enumerate polar vertices exactly after a trusted origin-interior boundary.
///
/// Assumed precondition: the caller has just validated, constructed, or loaded
/// from trusted data that `0 in int conv(vertices)`. This skips the positive
/// spanning validation performed by [`polar_vertices_exact`] but otherwise uses
/// the same generic exact enumeration semantics.
///
/// Panics if the assumed origin-interior contract is false enough that exact
/// enumeration produces no vertices.
pub fn polar_vertices_exact_assuming_origin_interior<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
) -> PolarVerticesExact<T> {
    let span = info_span!(
        "polar_vertices_exact",
        input_points = vertices.len(),
        origin_contract = "assumed"
    );
    let _span_guard = span.enter();

    polar_vertices_exact_generic_impl(vertices, 0.0)
}

fn polar_vertices_exact_generic_impl<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    validation_ms: f64,
) -> PolarVerticesExact<T> {
    let one = T::one();
    let rhs = Vector4::new(one.clone(), one.clone(), one.clone(), one.clone());
    let vertices_f64 = f64_prefilter_vertices(vertices);
    let tuples = combinations4(vertices.len());
    let candidate_4sets = tuples.len();
    let mut polar_vertices = Vec::new();

    let mut f64_prefilter_rejected = 0usize;
    let mut exact_solve_attempts = 0usize;
    let mut singular_exact_solves = 0usize;
    let mut exact_feasibility_checks = 0usize;
    let mut feasible_candidates = 0usize;
    let mut duplicate_exact_comparisons = 0usize;
    let mut duplicate_candidates = 0usize;

    let enumeration_start = Instant::now();
    for tuple in tuples {
        if let Some(vertices_f64) = &vertices_f64 {
            if f64_prefilter_rejects(vertices_f64, &tuple) {
                f64_prefilter_rejected += 1;
                continue;
            }
        }

        exact_solve_attempts += 1;
        let rows = tuple.map(|idx| vertices[idx].clone());
        let Some(candidate) = solve4_exact(&rows, &rhs) else {
            singular_exact_solves += 1;
            continue;
        };

        let mut is_feasible = true;
        for vertex in vertices {
            exact_feasibility_checks += 1;
            if dot4_exact(vertex, &candidate) > one {
                is_feasible = false;
                break;
            }
        }
        if !is_feasible {
            continue;
        }
        feasible_candidates += 1;

        let mut is_duplicate = false;
        for known in &polar_vertices {
            duplicate_exact_comparisons += 1;
            if known == &candidate {
                is_duplicate = true;
                break;
            }
        }
        if is_duplicate {
            duplicate_candidates += 1;
        } else {
            polar_vertices.push(candidate);
        }
    }
    let enumeration_ms = ms(enumeration_start);

    assert!(
        !polar_vertices.is_empty(),
        "origin-interior polar input produced no exact vertices"
    );

    let incidence_start = Instant::now();
    let vertex_facet_incidence =
        DMatrix::from_fn(polar_vertices.len(), vertices.len(), |row, col| {
            dot4_exact(&vertices[col], &polar_vertices[row]) == one
        });
    let incidence_ms = ms(incidence_start);

    info!(
        path = "generic_exact",
        validation_ms,
        enumeration_ms,
        incidence_ms,
        input_points = vertices.len(),
        candidate_4sets,
        f64_prefilter_rejected,
        exact_solve_attempts,
        singular_exact_solves,
        exact_feasibility_checks,
        feasible_candidates,
        duplicate_exact_comparisons,
        duplicate_candidates,
        returned_vertices = polar_vertices.len(),
        incidence_rows = vertex_facet_incidence.nrows(),
        incidence_cols = vertex_facet_incidence.ncols(),
        "polar vertex enumeration"
    );

    PolarVerticesExact {
        vertices: polar_vertices,
        vertex_facet_incidence,
    }
}

/// Enumerate BigRational polar vertices with the checked origin-interior API.
///
/// This is the rational-specific counterpart of [`polar_vertices_exact`]. It
/// keeps the same checked precondition and panic behavior, then uses the
/// integer-scaled determinant/gap path instead of generic exact linear solves.
pub fn polar_vertices_exact_rational(
    vertices: &[Vector4<BigRational>],
) -> PolarVerticesExact<BigRational> {
    let span = info_span!("polar_vertices_exact", input_points = vertices.len());
    let _span_guard = span.enter();

    let validation_start = Instant::now();
    let origin_is_interior = origin_in_interior_of_conv_exact(vertices);
    let validation_ms = ms(validation_start);
    assert!(
        origin_is_interior,
        "polar_vertices_exact_rational requires 0 in int conv(vertices)"
    );

    polar_vertices_exact_rational_impl(vertices, validation_ms)
}

/// Enumerate BigRational polar vertices after a trusted origin-interior boundary.
///
/// Assumed precondition: the caller has just validated, constructed, or loaded
/// from trusted data that `0 in int conv(vertices)`. This avoids repeating the
/// positive-spanning test and uses integer-scaled determinant solves and exact
/// integer feasibility gaps for the hot BigRational path.
///
/// Panics if the assumed origin-interior contract is false enough that exact
/// enumeration produces no vertices.
pub fn polar_vertices_exact_rational_assuming_origin_interior(
    vertices: &[Vector4<BigRational>],
) -> PolarVerticesExact<BigRational> {
    let span = info_span!(
        "polar_vertices_exact",
        input_points = vertices.len(),
        origin_contract = "assumed"
    );
    let _span_guard = span.enter();

    polar_vertices_exact_rational_impl(vertices, 0.0)
}

fn polar_vertices_exact_rational_impl(
    vertices: &[Vector4<BigRational>],
    validation_ms: f64,
) -> PolarVerticesExact<BigRational> {
    let (integer_vertices, common_denominator) = integer_scale_rational_vertices(vertices);
    let tuples = combinations4(vertices.len());
    let candidate_4sets = tuples.len();
    let mut scaled_candidates = Vec::new();

    let f64_prefilter_rejected = 0usize;
    let mut exact_solve_attempts = 0usize;
    let mut singular_exact_solves = 0usize;
    let mut exact_feasibility_checks = 0usize;
    let mut feasible_candidates = 0usize;
    let mut duplicate_exact_comparisons = 0usize;
    let mut duplicate_candidates = 0usize;

    let enumeration_start = Instant::now();
    for tuple in tuples {
        exact_solve_attempts += 1;
        let Some(candidate) =
            integer_scaled_polar_candidate(&integer_vertices, &common_denominator, &tuple)
        else {
            singular_exact_solves += 1;
            continue;
        };

        let mut is_feasible = true;
        for row in &integer_vertices {
            exact_feasibility_checks += 1;
            if integer_scaled_feasibility_gap(row, &common_denominator, &candidate).is_negative() {
                is_feasible = false;
                break;
            }
        }
        if !is_feasible {
            continue;
        }
        feasible_candidates += 1;

        let mut is_duplicate = false;
        for known in &scaled_candidates {
            duplicate_exact_comparisons += 1;
            if known == &candidate {
                is_duplicate = true;
                break;
            }
        }
        if is_duplicate {
            duplicate_candidates += 1;
        } else {
            scaled_candidates.push(candidate);
        }
    }
    let enumeration_ms = ms(enumeration_start);

    assert!(
        !scaled_candidates.is_empty(),
        "origin-interior polar input produced no exact vertices"
    );

    let polar_vertices: Vec<_> = scaled_candidates
        .iter()
        .map(ScaledRationalCandidate::to_vector)
        .collect();

    let incidence_start = Instant::now();
    let vertex_facet_incidence =
        DMatrix::from_fn(scaled_candidates.len(), vertices.len(), |row, col| {
            integer_scaled_feasibility_gap(
                &integer_vertices[col],
                &common_denominator,
                &scaled_candidates[row],
            )
            .is_zero()
        });
    let incidence_ms = ms(incidence_start);

    info!(
        path = "integer_scaled_big_rational",
        validation_ms,
        enumeration_ms,
        incidence_ms,
        input_points = vertices.len(),
        candidate_4sets,
        f64_prefilter_rejected,
        exact_solve_attempts,
        singular_exact_solves,
        exact_feasibility_checks,
        feasible_candidates,
        duplicate_exact_comparisons,
        duplicate_candidates,
        returned_vertices = polar_vertices.len(),
        incidence_rows = vertex_facet_incidence.nrows(),
        incidence_cols = vertex_facet_incidence.ncols(),
        "polar vertex enumeration"
    );

    PolarVerticesExact {
        vertices: polar_vertices,
        vertex_facet_incidence,
    }
}

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScaledRationalCandidate {
    denominator: BigInt,
    numerators: [BigInt; 4],
}

impl ScaledRationalCandidate {
    fn new(mut denominator: BigInt, mut numerators: [BigInt; 4]) -> Self {
        assert!(
            !denominator.is_zero(),
            "scaled rational candidate denominator must be nonzero"
        );
        if denominator.is_negative() {
            denominator = -denominator;
            for numerator in &mut numerators {
                *numerator = -numerator.clone();
            }
        }

        let mut divisor = denominator.clone();
        for numerator in &numerators {
            divisor = divisor.gcd(&numerator.abs());
        }
        if divisor > BigInt::one() {
            denominator /= &divisor;
            for numerator in &mut numerators {
                *numerator /= &divisor;
            }
        }

        Self {
            denominator,
            numerators,
        }
    }

    fn to_vector(&self) -> Vector4<BigRational> {
        Vector4::new(
            BigRational::new(self.numerators[0].clone(), self.denominator.clone()),
            BigRational::new(self.numerators[1].clone(), self.denominator.clone()),
            BigRational::new(self.numerators[2].clone(), self.denominator.clone()),
            BigRational::new(self.numerators[3].clone(), self.denominator.clone()),
        )
    }
}

fn integer_scale_rational_vertices(
    vertices: &[Vector4<BigRational>],
) -> (Vec<[BigInt; 4]>, BigInt) {
    let mut common_denominator = BigInt::one();
    for vertex in vertices {
        for coordinate in vertex.iter() {
            common_denominator = common_denominator.lcm(coordinate.denom());
        }
    }

    let integer_vertices = vertices
        .iter()
        .map(|vertex| {
            std::array::from_fn(|coordinate| {
                let scale = &common_denominator / vertex[coordinate].denom();
                vertex[coordinate].numer() * scale
            })
        })
        .collect();

    (integer_vertices, common_denominator)
}

fn integer_scaled_polar_candidate(
    integer_vertices: &[[BigInt; 4]],
    common_denominator: &BigInt,
    tuple: &[usize; 4],
) -> Option<ScaledRationalCandidate> {
    let rows: [[BigInt; 4]; 4] = [
        integer_vertices[tuple[0]].clone(),
        integer_vertices[tuple[1]].clone(),
        integer_vertices[tuple[2]].clone(),
        integer_vertices[tuple[3]].clone(),
    ];
    let determinant = det4_int(&rows);
    if determinant.is_zero() {
        return None;
    }

    let mut numerators = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];
    for coordinate in 0..4 {
        let mut modified = rows.clone();
        for row in &mut modified {
            row[coordinate] = BigInt::one();
        }
        numerators[coordinate] = common_denominator * det4_int(&modified);
    }

    Some(ScaledRationalCandidate::new(determinant, numerators))
}

fn integer_scaled_feasibility_gap(
    integer_row: &[BigInt; 4],
    common_denominator: &BigInt,
    candidate: &ScaledRationalCandidate,
) -> BigInt {
    common_denominator * &candidate.denominator - dot4_int(integer_row, &candidate.numerators)
}

fn dot4_int(left: &[BigInt; 4], right: &[BigInt; 4]) -> BigInt {
    &left[0] * &right[0] + &left[1] * &right[1] + &left[2] * &right[2] + &left[3] * &right[3]
}

fn det4_int(rows: &[[BigInt; 4]; 4]) -> BigInt {
    let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

    let m01 = &b[0] * &c[1] - &b[1] * &c[0];
    let m02 = &b[0] * &c[2] - &b[2] * &c[0];
    let m03 = &b[0] * &c[3] - &b[3] * &c[0];
    let m12 = &b[1] * &c[2] - &b[2] * &c[1];
    let m13 = &b[1] * &c[3] - &b[3] * &c[1];
    let m23 = &b[2] * &c[3] - &b[3] * &c[2];

    let c00 = &d[1] * &m23 - &d[2] * &m13 + &d[3] * &m12;
    let c01 = &d[0] * &m23 - &d[2] * &m03 + &d[3] * &m02;
    let c02 = &d[0] * &m13 - &d[1] * &m03 + &d[3] * &m01;
    let c03 = &d[0] * &m12 - &d[1] * &m02 + &d[2] * &m01;

    &a[0] * c00 - &a[1] * c01 + &a[2] * c02 - &a[3] * c03
}

fn f64_prefilter_vertices<T: ExactScalar>(vertices: &[Vector4<T>]) -> Option<Vec<[f64; 4]>> {
    vertices
        .iter()
        .map(|vertex| {
            let coordinates = std::array::from_fn(|coordinate| vertex[coordinate].round_to_f64());
            if coordinates.iter().all(|coordinate| coordinate.is_finite()) {
                Some(coordinates)
            } else {
                None
            }
        })
        .collect()
}

fn f64_prefilter_rejects(vertices_f64: &[[f64; 4]], tuple: &[usize; 4]) -> bool {
    use nalgebra::{Matrix4, Vector4};

    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const CONDITIONING_SAFETY_FACTOR: f64 = 1e4;

    let matrix = Matrix4::new(
        vertices_f64[tuple[0]][0],
        vertices_f64[tuple[0]][1],
        vertices_f64[tuple[0]][2],
        vertices_f64[tuple[0]][3],
        vertices_f64[tuple[1]][0],
        vertices_f64[tuple[1]][1],
        vertices_f64[tuple[1]][2],
        vertices_f64[tuple[1]][3],
        vertices_f64[tuple[2]][0],
        vertices_f64[tuple[2]][1],
        vertices_f64[tuple[2]][2],
        vertices_f64[tuple[2]][3],
        vertices_f64[tuple[3]][0],
        vertices_f64[tuple[3]][1],
        vertices_f64[tuple[3]][2],
        vertices_f64[tuple[3]][3],
    );

    let svd = matrix.svd(true, true);
    let singular_values = &svd.singular_values;
    let sigma_min = singular_values[0]
        .min(singular_values[1])
        .min(singular_values[2])
        .min(singular_values[3]);
    let sigma_max = singular_values[0]
        .max(singular_values[1])
        .max(singular_values[2])
        .max(singular_values[3]);

    if sigma_min == 0.0 {
        return false;
    }

    let condition_number_bound = sigma_max / sigma_min;
    if EPS_MACH * condition_number_bound > 0.25 {
        return false;
    }

    let rhs = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let candidate = match svd.solve(&rhs, 0.0) {
        Ok(candidate) => candidate,
        Err(_) => return false,
    };

    if candidate.iter().any(|&coordinate| !coordinate.is_finite()) {
        return false;
    }

    let candidate_norm = candidate.norm();

    for (point_index, point) in vertices_f64.iter().enumerate() {
        if tuple.contains(&point_index) {
            continue;
        }

        let dot = point[0] * candidate[0]
            + point[1] * candidate[1]
            + point[2] * candidate[2]
            + point[3] * candidate[3];
        let point_norm =
            (point[0] * point[0] + point[1] * point[1] + point[2] * point[2] + point[3] * point[3])
                .sqrt();
        let absolute_error_bound = CONDITIONING_SAFETY_FACTOR
            * condition_number_bound
            * EPS_MACH
            * candidate_norm
            * point_norm;

        if !dot.is_finite() || !absolute_error_bound.is_finite() {
            return false;
        }

        if dot > 1.0 + absolute_error_bound {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{
        f64_prefilter_rejects, integer_scale_rational_vertices, integer_scaled_feasibility_gap,
        integer_scaled_polar_candidate,
    };
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use num_traits::{Signed, Zero};

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    fn qf(numer: i64, denom: i64) -> BigRational {
        BigRational::new(numer.into(), denom.into())
    }

    #[test]
    fn f64_prefilter_rejects_clearly_outside_candidate() {
        let vertices = vec![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 2.0, 2.0, 2.0],
        ];

        assert!(f64_prefilter_rejects(&vertices, &[0, 1, 2, 3]));
    }

    #[test]
    fn f64_prefilter_keeps_plausible_candidate_for_exact_resolution() {
        let vertices = vec![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [-0.25, -0.25, -0.25, -0.25],
        ];

        assert!(!f64_prefilter_rejects(&vertices, &[0, 1, 2, 3]));
    }

    #[test]
    fn integer_scaled_candidate_solves_fractional_diagonal_system() {
        let vertices = vec![
            Vector4::new(qf(1, 2), q(0), q(0), q(0)),
            Vector4::new(q(0), qf(2, 3), q(0), q(0)),
            Vector4::new(q(0), q(0), qf(-3, 4), q(0)),
            Vector4::new(q(0), q(0), q(0), qf(5, 6)),
        ];
        let (integer_vertices, common_denominator) = integer_scale_rational_vertices(&vertices);

        let candidate =
            integer_scaled_polar_candidate(&integer_vertices, &common_denominator, &[0, 1, 2, 3])
                .expect("diagonal system is nonsingular");

        assert_eq!(
            candidate.to_vector(),
            Vector4::new(q(2), qf(3, 2), qf(-4, 3), qf(6, 5))
        );
        for row in &integer_vertices {
            assert!(integer_scaled_feasibility_gap(row, &common_denominator, &candidate).is_zero());
        }
    }

    #[test]
    fn integer_scaled_gap_distinguishes_feasible_incident_and_outside_rows() {
        let vertices = vec![
            Vector4::new(qf(1, 2), q(0), q(0), q(0)),
            Vector4::new(q(0), qf(2, 3), q(0), q(0)),
            Vector4::new(q(0), q(0), qf(3, 4), q(0)),
            Vector4::new(q(0), q(0), q(0), qf(5, 6)),
            Vector4::new(qf(1, 4), q(0), q(0), q(0)),
            Vector4::new(q(1), q(0), q(0), q(0)),
        ];
        let (integer_vertices, common_denominator) = integer_scale_rational_vertices(&vertices);
        let candidate =
            integer_scaled_polar_candidate(&integer_vertices, &common_denominator, &[0, 1, 2, 3])
                .expect("diagonal system is nonsingular");

        assert!(integer_scaled_feasibility_gap(
            &integer_vertices[0],
            &common_denominator,
            &candidate
        )
        .is_zero());
        assert!(integer_scaled_feasibility_gap(
            &integer_vertices[4],
            &common_denominator,
            &candidate
        )
        .is_positive());
        assert!(integer_scaled_feasibility_gap(
            &integer_vertices[5],
            &common_denominator,
            &candidate
        )
        .is_negative());
    }
}
