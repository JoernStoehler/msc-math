use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Vector4};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::time::Instant;
use tracing::{info, info_span};

// Internal sufficient-condition filter constants. These bounds intentionally
// overestimate f64 arithmetic/input error because an inconclusive test only
// sends the tuple to exact integer arithmetic.
const F64_DOT_ERROR_SAFETY_FACTOR: f64 = 32.0;
const F64_DET_ERROR_SAFETY_FACTOR: f64 = 256.0;

use crate::linalg::{combinations4, dot4_exact, solve4_exact};
use crate::predicates::{
    gamma_f64, origin_in_interior_of_conv_exact, origin_in_interior_of_conv_exact_rational,
};

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
    let tuples = combinations4(vertices.len());
    let candidate_4sets = tuples.len();
    let mut polar_vertices = Vec::new();

    let mut exact_solve_attempts = 0usize;
    let mut singular_exact_solves = 0usize;
    let mut exact_feasibility_checks = 0usize;
    let mut feasible_candidates = 0usize;
    let mut duplicate_exact_comparisons = 0usize;
    let mut duplicate_candidates = 0usize;

    let enumeration_start = Instant::now();
    for tuple in tuples {
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
    let origin_is_interior = origin_in_interior_of_conv_exact_rational(vertices);
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
    let vertices_f64 = rational_vertices_to_f64_approximations(vertices);
    let tuples = combinations4(vertices.len());
    let candidate_4sets = tuples.len();
    let mut scaled_candidates = Vec::new();

    let mut exact_solve_attempts = 0usize;
    let mut f64_prefilter_rejections = 0usize;
    let mut singular_exact_solves = 0usize;
    let mut exact_feasibility_checks = 0usize;
    let mut feasible_candidates = 0usize;
    let mut duplicate_exact_comparisons = 0usize;
    let mut duplicate_candidates = 0usize;

    let enumeration_start = Instant::now();
    for tuple in tuples {
        if let Some(vertices_f64) = &vertices_f64 {
            if f64_prefilter_certifies_rejection(vertices_f64, &tuple) {
                f64_prefilter_rejections += 1;
                continue;
            }
        }

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
        exact_solve_attempts,
        f64_prefilter_rejections,
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

#[derive(Clone, Copy, Debug)]
struct F64ApproxVector4 {
    values: [f64; 4],
    errors: [f64; 4],
}

impl F64ApproxVector4 {
    fn has_input_error(&self) -> bool {
        self.errors.iter().any(|&error| error != 0.0)
    }
}

fn rational_vertices_to_f64_approximations(
    vertices: &[Vector4<BigRational>],
) -> Option<Vec<F64ApproxVector4>> {
    vertices
        .iter()
        .map(|vertex| {
            let converted: [Option<(f64, f64)>; 4] = std::array::from_fn(|coordinate| {
                f64_approximation_for_rational(&vertex[coordinate])
            });
            Some(F64ApproxVector4 {
                values: [
                    converted[0]?.0,
                    converted[1]?.0,
                    converted[2]?.0,
                    converted[3]?.0,
                ],
                errors: [
                    converted[0]?.1,
                    converted[1]?.1,
                    converted[2]?.1,
                    converted[3]?.1,
                ],
            })
        })
        .collect()
}

fn f64_approximation_for_rational(value: &BigRational) -> Option<(f64, f64)> {
    let f = value.to_f64()?;
    if !normal_or_zero_f64(f) {
        return None;
    }

    let rounded = BigRational::from_float(f)?;
    let exact_error = (value - rounded).abs();
    if exact_error.is_zero() {
        return Some((f, 0.0));
    }
    let error = exact_error.to_f64()?.next_up();
    normal_or_zero_f64(error).then_some((f, error))
}

fn normal_or_zero_f64(value: f64) -> bool {
    value == 0.0 || (value.is_finite() && value.is_normal())
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

fn f64_prefilter_certifies_rejection(
    vertices_f64: &[F64ApproxVector4],
    tuple: &[usize; 4],
) -> bool {
    // Contract: `true` means exact arithmetic would reject this tuple as
    // infeasible. `false` means inconclusive.
    // Rejection-only filter. For a tuple matrix A, Cramer's rule gives
    // y_j = nu_j / det(A). For a row a, a*y <= 1 is equivalent to the signed
    // gap condition det(A) and h = det(A) - a*nu having compatible signs. If
    // either sign is numerically narrow, exact integer arithmetic decides it.
    let rows = [
        vertices_f64[tuple[0]],
        vertices_f64[tuple[1]],
        vertices_f64[tuple[2]],
        vertices_f64[tuple[3]],
    ];
    let Some((determinant, determinant_error)) = det4_f64_with_error(rows) else {
        return false;
    };
    let determinant_sign = if determinant > determinant_error {
        1.0
    } else if determinant < -determinant_error {
        -1.0
    } else {
        return false;
    };

    let mut numerators = [0.0; 4];
    let mut numerator_errors = [0.0; 4];
    for coordinate in 0..4 {
        let Some((numerator, numerator_error)) =
            det4_f64_with_error(replace_column_with_ones(rows, coordinate))
        else {
            return false;
        };
        numerators[coordinate] = numerator;
        numerator_errors[coordinate] = numerator_error;
    }

    for (index, vertex) in vertices_f64.iter().enumerate() {
        if tuple.contains(&index) {
            continue;
        }

        let Some((gap, gap_error)) = signed_cramer_feasibility_gap_f64(
            *vertex,
            determinant,
            determinant_error,
            numerators,
            numerator_errors,
        ) else {
            return false;
        };
        if determinant_sign * gap < -gap_error {
            return true;
        }
    }

    false
}

fn signed_cramer_feasibility_gap_f64(
    row: F64ApproxVector4,
    determinant: f64,
    determinant_error: f64,
    numerators: [f64; 4],
    numerator_errors: [f64; 4],
) -> Option<(f64, f64)> {
    // `None` means the f64 filter is inconclusive; exact arithmetic decides.
    let products: [Option<f64>; 4] = std::array::from_fn(|coordinate| {
        checked_mul_f64(row.values[coordinate], numerators[coordinate])
    });

    let dot = checked_add_f64(
        checked_add_f64(products[0]?, products[1]?)?,
        checked_add_f64(products[2]?, products[3]?)?,
    )?;
    let gap = checked_sub_f64(determinant, dot)?;

    let mut numerator_input_error = 0.0;
    let mut row_input_error = 0.0;
    let mut product_magnitude_sum = 0.0;
    for coordinate in 0..4 {
        numerator_input_error += row.values[coordinate].abs() * numerator_errors[coordinate];
        row_input_error += row.errors[coordinate] * numerators[coordinate].abs();
        row_input_error += row.errors[coordinate] * numerator_errors[coordinate];
        product_magnitude_sum =
            checked_add_f64(product_magnitude_sum, products[coordinate]?.abs().next_up())?;
    }

    let dot_rounding_error = F64_DOT_ERROR_SAFETY_FACTOR * gamma_f64(7.0) * product_magnitude_sum;
    let subtraction_error =
        F64_DOT_ERROR_SAFETY_FACTOR * gamma_f64(1.0) * (determinant.abs() + product_magnitude_sum);
    let error_bound = determinant_error
        + numerator_input_error
        + row_input_error
        + dot_rounding_error
        + subtraction_error;
    if !error_bound.is_finite() {
        return None;
    }
    padded_positive_error_bound(error_bound).map(|error_bound| (gap, error_bound))
}

fn checked_add_f64(left: f64, right: f64) -> Option<f64> {
    checked_normal_or_zero_f64(left + right)
}

fn checked_sub_f64(left: f64, right: f64) -> Option<f64> {
    checked_normal_or_zero_f64(left - right)
}

fn checked_mul_f64(left: f64, right: f64) -> Option<f64> {
    checked_normal_or_zero_f64(left * right)
}

fn checked_normal_or_zero_f64(value: f64) -> Option<f64> {
    normal_or_zero_f64(value).then_some(value)
}

fn replace_column_with_ones(
    mut rows: [F64ApproxVector4; 4],
    column: usize,
) -> [F64ApproxVector4; 4] {
    for row in &mut rows {
        row.values[column] = 1.0;
        row.errors[column] = 0.0;
    }
    rows
}

fn det4_f64_with_error(rows: [F64ApproxVector4; 4]) -> Option<(f64, f64)> {
    let determinant = det4_value_f64(rows)?;
    let max_value = max_abs_f64_entry(rows)?;
    let magnitude_bound = 24.0 * max_value * max_value * max_value * max_value;
    // Cofactor evaluation is not the 24-term Leibniz computation used by the
    // public orient4 diagnostic, so this local bound uses a larger operation
    // count and safety factor. It supports a sufficient-condition rejection
    // test, not an exact determinant API.
    let arithmetic_error = F64_DET_ERROR_SAFETY_FACTOR * gamma_f64(50.0) * magnitude_bound;

    let input_error = if rows.iter().any(F64ApproxVector4::has_input_error) {
        det4_input_error_bound(rows, max_value)?
    } else {
        0.0
    };
    let error_bound = padded_positive_error_bound(arithmetic_error + input_error)?;
    Some((determinant, error_bound))
}

fn det4_value_f64(rows: [F64ApproxVector4; 4]) -> Option<f64> {
    let a = rows[0].values;
    let b = rows[1].values;
    let c = rows[2].values;
    let d = rows[3].values;

    let m01 = checked_sub_f64(checked_mul_f64(b[0], c[1])?, checked_mul_f64(b[1], c[0])?)?;
    let m02 = checked_sub_f64(checked_mul_f64(b[0], c[2])?, checked_mul_f64(b[2], c[0])?)?;
    let m03 = checked_sub_f64(checked_mul_f64(b[0], c[3])?, checked_mul_f64(b[3], c[0])?)?;
    let m12 = checked_sub_f64(checked_mul_f64(b[1], c[2])?, checked_mul_f64(b[2], c[1])?)?;
    let m13 = checked_sub_f64(checked_mul_f64(b[1], c[3])?, checked_mul_f64(b[3], c[1])?)?;
    let m23 = checked_sub_f64(checked_mul_f64(b[2], c[3])?, checked_mul_f64(b[3], c[2])?)?;

    let c00 = checked_add_f64(
        checked_sub_f64(checked_mul_f64(d[1], m23)?, checked_mul_f64(d[2], m13)?)?,
        checked_mul_f64(d[3], m12)?,
    )?;
    let c01 = checked_add_f64(
        checked_sub_f64(checked_mul_f64(d[0], m23)?, checked_mul_f64(d[2], m03)?)?,
        checked_mul_f64(d[3], m02)?,
    )?;
    let c02 = checked_add_f64(
        checked_sub_f64(checked_mul_f64(d[0], m13)?, checked_mul_f64(d[1], m03)?)?,
        checked_mul_f64(d[3], m01)?,
    )?;
    let c03 = checked_add_f64(
        checked_sub_f64(checked_mul_f64(d[0], m12)?, checked_mul_f64(d[1], m02)?)?,
        checked_mul_f64(d[2], m01)?,
    )?;

    checked_sub_f64(
        checked_add_f64(
            checked_sub_f64(checked_mul_f64(a[0], c00)?, checked_mul_f64(a[1], c01)?)?,
            checked_mul_f64(a[2], c02)?,
        )?,
        checked_mul_f64(a[3], c03)?,
    )
}

fn max_abs_f64_entry(rows: [F64ApproxVector4; 4]) -> Option<f64> {
    let mut max_value = 0.0_f64;
    let mut min_nonzero_value = f64::INFINITY;
    for row in rows {
        for coordinate in 0..4 {
            if !normal_or_zero_f64(row.values[coordinate])
                || !normal_or_zero_f64(row.errors[coordinate])
            {
                return None;
            }
            let abs_value = row.values[coordinate].abs();
            max_value = max_value.max(abs_value);
            if abs_value != 0.0 {
                min_nonzero_value = min_nonzero_value.min(abs_value);
            }
        }
    }
    let max_squared = max_value * max_value;
    let max_fourth = max_squared * max_squared;
    if !max_squared.is_finite() || !max_fourth.is_finite() {
        return None;
    }
    if min_nonzero_value.is_finite() {
        let min_squared = min_nonzero_value * min_nonzero_value;
        let min_fourth = min_squared * min_squared;
        if !min_fourth.is_normal() {
            return None;
        }
    }
    Some(max_value)
}

fn det4_input_error_bound(rows: [F64ApproxVector4; 4], max_value: f64) -> Option<f64> {
    let mut max_error = 0.0_f64;
    for row in rows {
        for coordinate in 0..4 {
            max_error = max_error.max(row.errors[coordinate]);
        }
    }
    if max_error == 0.0 {
        return Some(0.0);
    }

    // For each Leibniz product, replacing exact inputs by f64 approximations
    // changes the product by at most (A + E)^4 - A^4. Use the expanded form
    // to avoid cancellation when E is much smaller than ulp(A).
    let a2 = max_value * max_value;
    let a3 = a2 * max_value;
    let e2 = max_error * max_error;
    let e3 = e2 * max_error;
    let e4 = e2 * e2;
    let error_bound = 24.0 * (4.0 * a3 * max_error + 6.0 * a2 * e2 + 4.0 * max_value * e3 + e4);
    padded_positive_error_bound(error_bound)
}

fn padded_positive_error_bound(error_bound: f64) -> Option<f64> {
    if !error_bound.is_finite() {
        return None;
    }
    if error_bound == 0.0 || error_bound.is_subnormal() {
        return Some(f64::MIN_POSITIVE);
    }
    let padded = error_bound.next_up();
    padded.is_finite().then_some(padded)
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

#[cfg(test)]
mod tests {
    use super::{
        det4_f64_with_error, f64_prefilter_certifies_rejection, integer_scale_rational_vertices,
        integer_scaled_feasibility_gap, integer_scaled_polar_candidate,
        rational_vertices_to_f64_approximations, signed_cramer_feasibility_gap_f64,
        F64ApproxVector4,
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

    #[test]
    fn f64_prefilter_can_reject_non_exact_rational_inputs() {
        let vertices = vec![
            Vector4::new(q(1), q(0), q(0), q(0)),
            Vector4::new(q(-1), q(0), q(0), q(0)),
            Vector4::new(q(0), q(1), q(0), q(0)),
            Vector4::new(q(0), q(-1), q(0), q(0)),
            Vector4::new(q(0), q(0), q(1), q(0)),
            Vector4::new(q(0), q(0), q(-1), q(0)),
            Vector4::new(q(0), q(0), q(0), q(1)),
            Vector4::new(q(0), q(0), q(0), q(-1)),
            Vector4::new(qf(1, 3), qf(1, 3), qf(1, 3), qf(1, 3)),
            Vector4::new(qf(-1, 3), qf(-1, 3), qf(-1, 3), qf(-1, 3)),
        ];
        let approximations =
            rational_vertices_to_f64_approximations(&vertices).expect("finite rational fixture");

        assert!(
            approximations
                .iter()
                .flat_map(|point| point.errors)
                .any(|error| error > 0.0),
            "fixture should exercise rational-to-f64 input error bounds"
        );

        let mut rejected = 0;
        for a in 0..vertices.len() {
            for b in a + 1..vertices.len() {
                for c in b + 1..vertices.len() {
                    for d in c + 1..vertices.len() {
                        if f64_prefilter_certifies_rejection(&approximations, &[a, b, c, d]) {
                            rejected += 1;
                        }
                    }
                }
            }
        }

        assert!(rejected > 0);
    }

    #[test]
    fn f64_prefilter_rejections_are_exactly_infeasible_on_fixture() {
        let vertices = vec![
            Vector4::new(q(1), q(0), q(0), q(0)),
            Vector4::new(q(-1), q(0), q(0), q(0)),
            Vector4::new(q(0), q(1), q(0), q(0)),
            Vector4::new(q(0), q(-1), q(0), q(0)),
            Vector4::new(q(0), q(0), q(1), q(0)),
            Vector4::new(q(0), q(0), q(-1), q(0)),
            Vector4::new(q(0), q(0), q(0), q(1)),
            Vector4::new(q(0), q(0), q(0), q(-1)),
            Vector4::new(qf(1, 3), qf(1, 3), qf(1, 3), qf(1, 3)),
            Vector4::new(qf(-1, 3), qf(-1, 3), qf(-1, 3), qf(-1, 3)),
            Vector4::new(qf(7, 10), qf(-1, 5), qf(3, 10), qf(-2, 5)),
        ];
        let approximations =
            rational_vertices_to_f64_approximations(&vertices).expect("finite rational fixture");
        let (integer_vertices, common_denominator) = integer_scale_rational_vertices(&vertices);

        let mut rejected = 0;
        for a in 0..vertices.len() {
            for b in a + 1..vertices.len() {
                for c in b + 1..vertices.len() {
                    for d in c + 1..vertices.len() {
                        let tuple = [a, b, c, d];
                        if !f64_prefilter_certifies_rejection(&approximations, &tuple) {
                            continue;
                        }
                        rejected += 1;
                        let candidate = integer_scaled_polar_candidate(
                            &integer_vertices,
                            &common_denominator,
                            &tuple,
                        )
                        .expect("certified nonzero f64 determinant must be exact nonsingular");
                        assert!(
                            integer_vertices.iter().any(|row| {
                                integer_scaled_feasibility_gap(row, &common_denominator, &candidate)
                                    .is_negative()
                            }),
                            "f64 prefilter rejected an exactly feasible tuple {tuple:?}"
                        );
                    }
                }
            }
        }

        assert!(rejected > 0);
    }

    #[test]
    fn f64_gap_filter_is_inconclusive_on_nonzero_product_underflow() {
        let row = F64ApproxVector4 {
            values: [f64::MIN_POSITIVE, 0.0, 0.0, 0.0],
            errors: [0.0; 4],
        };

        assert_eq!(
            signed_cramer_feasibility_gap_f64(row, 1.0, f64::MIN_POSITIVE, [0.5; 4], [0.0; 4]),
            None
        );
    }

    #[test]
    fn f64_determinant_filter_is_inconclusive_on_intermediate_underflow() {
        let rows = [
            F64ApproxVector4 {
                values: [1.0, 0.0, 0.0, 0.0],
                errors: [0.0; 4],
            },
            F64ApproxVector4 {
                values: [f64::MIN_POSITIVE, 0.0, 0.0, 0.0],
                errors: [0.0; 4],
            },
            F64ApproxVector4 {
                values: [0.0, 0.5, 0.0, 0.0],
                errors: [0.0; 4],
            },
            F64ApproxVector4 {
                values: [0.0, 0.0, 1.0, 1.0],
                errors: [0.0; 4],
            },
        ];

        assert_eq!(det4_f64_with_error(rows), None);
    }

    #[test]
    fn f64_approximation_rejects_non_normal_values() {
        let subnormal =
            BigRational::from_float(f64::MIN_POSITIVE / 2.0).expect("subnormal f64 is finite");

        assert_eq!(super::f64_approximation_for_rational(&subnormal), None);
    }
}
