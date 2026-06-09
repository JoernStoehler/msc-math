use algebraic_numbers::{rank, solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Vector4};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use crate::linalg::{combinations3, cross_product_4d_exact, dot4_exact, is_zero_vector_exact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSign {
    Positive,
    Negative,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OriginInteriorF64 {
    True,
    False,
    Indeterminate,
}

const F64_UNIT_ROUNDOFF: f64 = f64::EPSILON / 2.0;
const ORIENT4_ERROR_SAFETY_FACTOR: f64 = 16.0;

const ORIENT4_PERMUTATIONS: [([usize; 4], f64); 24] = [
    ([0, 1, 2, 3], 1.0),
    ([0, 1, 3, 2], -1.0),
    ([0, 2, 1, 3], -1.0),
    ([0, 2, 3, 1], 1.0),
    ([0, 3, 1, 2], 1.0),
    ([0, 3, 2, 1], -1.0),
    ([1, 0, 2, 3], -1.0),
    ([1, 0, 3, 2], 1.0),
    ([1, 2, 0, 3], 1.0),
    ([1, 2, 3, 0], -1.0),
    ([1, 3, 0, 2], -1.0),
    ([1, 3, 2, 0], 1.0),
    ([2, 0, 1, 3], 1.0),
    ([2, 0, 3, 1], -1.0),
    ([2, 1, 0, 3], -1.0),
    ([2, 1, 3, 0], 1.0),
    ([2, 3, 0, 1], 1.0),
    ([2, 3, 1, 0], -1.0),
    ([3, 0, 1, 2], -1.0),
    ([3, 0, 2, 1], 1.0),
    ([3, 1, 0, 2], 1.0),
    ([3, 1, 2, 0], -1.0),
    ([3, 2, 0, 1], -1.0),
    ([3, 2, 1, 0], 1.0),
];

/// Return the certified sign of the exact determinant of an f64 `4 x 4` matrix.
///
/// The input f64 values are interpreted as exact real numbers. `Positive` and
/// `Negative` certify the exact determinant sign; `Indeterminate` makes no
/// claim. Non-finite values and operations that leave the normal f64 range are
/// treated as indeterminate rather than relying on underflow/overflow behavior.
pub fn orient4_sign_f64(rows: [Vector4<f64>; 4]) -> CertifiedSign {
    let Some((determinant, error_bound)) = orient4_det_and_error_bound_f64(rows) else {
        return CertifiedSign::Indeterminate;
    };

    if determinant > error_bound {
        CertifiedSign::Positive
    } else if determinant < -error_bound {
        CertifiedSign::Negative
    } else {
        CertifiedSign::Indeterminate
    }
}

/// Return a diagnostic origin-interior answer over exact f64 input values.
///
/// This is a sufficient f64 filter, not a complete predicate. It returns
/// `False` when a triple-normal separating hyperplane is certified and `True`
/// when every triple is certified nonseparating. Degenerate or numerically
/// narrow cases return `Indeterminate` for exact fallback.
pub fn origin_in_interior_of_conv_f64(points: &[Vector4<f64>]) -> OriginInteriorF64 {
    if points.len() < 5 || points.iter().any(|point| !valid_f64_input_vector(point)) {
        return OriginInteriorF64::Indeterminate;
    }

    let mut every_triple_certified_nonseparating = true;
    for triple in combinations3(points.len()) {
        let mut has_positive = false;
        let mut has_negative = false;
        let mut all_positive = true;
        let mut all_negative = true;

        for (point_index, point) in points.iter().enumerate() {
            if triple.contains(&point_index) {
                continue;
            }

            match orient4_sign_f64([
                points[triple[0]],
                points[triple[1]],
                points[triple[2]],
                *point,
            ]) {
                CertifiedSign::Positive => {
                    has_positive = true;
                    all_negative = false;
                }
                CertifiedSign::Negative => {
                    has_negative = true;
                    all_positive = false;
                }
                CertifiedSign::Indeterminate => {
                    all_positive = false;
                    all_negative = false;
                }
            }
        }

        if all_positive || all_negative {
            return OriginInteriorF64::False;
        }
        if !(has_positive && has_negative) {
            every_triple_certified_nonseparating = false;
        }
    }

    if every_triple_certified_nonseparating {
        OriginInteriorF64::True
    } else {
        OriginInteriorF64::Indeterminate
    }
}

fn orient4_det_and_error_bound_f64(rows: [Vector4<f64>; 4]) -> Option<(f64, f64)> {
    if rows.iter().any(|row| !valid_f64_input_vector(row)) {
        return None;
    }

    let mut determinant = 0.0;
    let mut magnitude_sum = 0.0;

    for (permutation, sign) in ORIENT4_PERMUTATIONS {
        let signed_term = checked_mul4_f64(
            rows[0][permutation[0]],
            rows[1][permutation[1]],
            rows[2][permutation[2]],
            rows[3][permutation[3]],
        )? * sign;
        determinant = checked_add_f64(determinant, signed_term)?;

        let magnitude_term = checked_mul4_f64(
            rows[0][permutation[0]].abs(),
            rows[1][permutation[1]].abs(),
            rows[2][permutation[2]].abs(),
            rows[3][permutation[3]].abs(),
        )?;
        magnitude_sum = checked_add_f64(magnitude_sum, magnitude_term)?;
    }

    let error_bound = orient4_error_bound_f64(magnitude_sum)?;
    Some((determinant, error_bound))
}

fn orient4_error_bound_f64(magnitude_sum: f64) -> Option<f64> {
    if magnitude_sum == 0.0 {
        return Some(0.0);
    }

    let rounded_error_bound = ORIENT4_ERROR_SAFETY_FACTOR * gamma_f64(26.0) * magnitude_sum;
    if !rounded_error_bound.is_finite() {
        return None;
    }
    if rounded_error_bound == 0.0 || rounded_error_bound.is_subnormal() {
        return Some(f64::MIN_POSITIVE);
    }
    let padded_error_bound = rounded_error_bound.next_up();
    padded_error_bound.is_finite().then_some(padded_error_bound)
}

pub(crate) fn gamma_f64(k: f64) -> f64 {
    let ku = k * F64_UNIT_ROUNDOFF;
    ku / (1.0 - ku)
}

fn valid_f64_input_vector(vector: &Vector4<f64>) -> bool {
    vector.iter().all(|&entry| valid_normal_or_zero_f64(entry))
}

fn valid_normal_or_zero_f64(value: f64) -> bool {
    value == 0.0 || (value.is_finite() && value.is_normal())
}

fn checked_mul4_f64(a: f64, b: f64, c: f64, d: f64) -> Option<f64> {
    let ab = checked_mul_f64(a, b)?;
    let abc = checked_mul_f64(ab, c)?;
    checked_mul_f64(abc, d)
}

fn checked_mul_f64(left: f64, right: f64) -> Option<f64> {
    let result = left * right;
    if !result.is_finite() {
        return None;
    }
    if result == 0.0 {
        return (left == 0.0 || right == 0.0).then_some(0.0);
    }
    result.is_normal().then_some(result)
}

fn checked_add_f64(left: f64, right: f64) -> Option<f64> {
    let result = left + right;
    if !result.is_finite() {
        return None;
    }
    if result == 0.0 {
        return (left == -right).then_some(0.0);
    }
    result.is_normal().then_some(result)
}

/// Return whether `0` lies in the interior of `conv(points)` in ambient `R^4`.
///
/// This is an exact positive-spanning test. It returns `false` for lower-rank
/// input, including empty and lower-dimensional point sets.
pub fn origin_in_interior_of_conv_exact<T: ExactScalar + 'static>(points: &[Vector4<T>]) -> bool {
    if points.len() < 5 {
        return false;
    }

    origin_in_interior_of_conv_exact_slow(points)
}

fn origin_in_interior_of_conv_exact_slow<T: ExactScalar + 'static>(points: &[Vector4<T>]) -> bool {
    let matrix = DMatrix::from_fn(points.len(), 4, |row, col| points[row][col].clone());
    if rank(&matrix) < 4 {
        return false;
    }

    for triple in combinations3(points.len()) {
        let normal =
            cross_product_4d_exact(&points[triple[0]], &points[triple[1]], &points[triple[2]]);
        if is_zero_vector_exact(&normal) {
            continue;
        }

        let has_positive = points
            .iter()
            .any(|point| dot4_exact(point, &normal) > T::zero());
        let has_negative = points
            .iter()
            .any(|point| dot4_exact(point, &normal) < T::zero());

        if !has_positive || !has_negative {
            return false;
        }
    }

    true
}

/// Return whether `0` lies in the interior of `conv(points)` for rational input.
///
/// This is the same exact positive-spanning test as
/// [`origin_in_interior_of_conv_exact`], specialized by scaling all coordinates
/// to a common integer denominator before the triple-normal separation pass.
pub(crate) fn origin_in_interior_of_conv_exact_rational(points: &[Vector4<BigRational>]) -> bool {
    if points.len() < 5 {
        return false;
    }

    let matrix = DMatrix::from_fn(points.len(), 4, |row, col| points[row][col].clone());
    if rank(&matrix) < 4 {
        return false;
    }

    let integer_points = integer_scale_rational_points(points);
    origin_in_interior_of_integer_scaled_conv(&integer_points)
}

fn integer_scale_rational_points(points: &[Vector4<BigRational>]) -> Vec<[BigInt; 4]> {
    let mut common_denominator = BigInt::one();
    for point in points {
        for coordinate in point.iter() {
            common_denominator = common_denominator.lcm(coordinate.denom());
        }
    }

    points
        .iter()
        .map(|point| {
            std::array::from_fn(|coordinate| {
                let scale = &common_denominator / point[coordinate].denom();
                point[coordinate].numer() * scale
            })
        })
        .collect()
}

fn origin_in_interior_of_integer_scaled_conv(points: &[[BigInt; 4]]) -> bool {
    for triple in combinations3(points.len()) {
        let normal =
            cross_product_4d_int(&points[triple[0]], &points[triple[1]], &points[triple[2]]);
        if normal.iter().all(BigInt::is_zero) {
            continue;
        }

        let mut has_positive = false;
        let mut has_negative = false;
        for point in points {
            let dot = dot4_int(point, &normal);
            has_positive |= dot.is_positive();
            has_negative |= dot.is_negative();
            if has_positive && has_negative {
                break;
            }
        }

        if !has_positive || !has_negative {
            return false;
        }
    }

    true
}

fn cross_product_4d_int(a: &[BigInt; 4], b: &[BigInt; 4], c: &[BigInt; 4]) -> [BigInt; 4] {
    let bc_01 = &b[0] * &c[1] - &b[1] * &c[0];
    let bc_02 = &b[0] * &c[2] - &b[2] * &c[0];
    let bc_03 = &b[0] * &c[3] - &b[3] * &c[0];
    let bc_12 = &b[1] * &c[2] - &b[2] * &c[1];
    let bc_13 = &b[1] * &c[3] - &b[3] * &c[1];
    let bc_23 = &b[2] * &c[3] - &b[3] * &c[2];

    [
        &a[1] * &bc_23 - &a[2] * &bc_13 + &a[3] * &bc_12,
        -(&a[0] * &bc_23 - &a[2] * &bc_03 + &a[3] * &bc_02),
        &a[0] * &bc_13 - &a[1] * &bc_03 + &a[3] * &bc_01,
        -(&a[0] * &bc_12 - &a[1] * &bc_02 + &a[2] * &bc_01),
    ]
}

fn dot4_int(left: &[BigInt; 4], right: &[BigInt; 4]) -> BigInt {
    &left[0] * &right[0] + &left[1] * &right[1] + &left[2] * &right[2] + &left[3] * &right[3]
}

/// Return whether every input point is an extreme point of `conv(points)`.
///
/// This exact predicate works in ambient `R^4`, including lower-dimensional
/// point sets. It returns `false` when any point lies in the convex hull of the
/// remaining input points; exact duplicate points are therefore non-extreme.
pub fn all_points_are_extreme_exact<T: ExactScalar + 'static>(points: &[Vector4<T>]) -> bool {
    for target_index in 0..points.len() {
        if point_lies_in_conv_of_others(points, target_index) {
            return false;
        }
    }

    true
}

fn point_lies_in_conv_of_others<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
    target_index: usize,
) -> bool {
    let max_subset_size = 5.min(points.len().saturating_sub(1));
    for subset_size in 1..=max_subset_size {
        for witness_indices in witness_subsets_excluding(points.len(), target_index, subset_size) {
            if has_nonnegative_barycentric_witness(points, target_index, &witness_indices) {
                return true;
            }
        }
    }

    false
}

fn witness_subsets_excluding(n: usize, excluded: usize, subset_size: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::with_capacity(subset_size);
    extend_witness_subsets(n, excluded, subset_size, 0, &mut current, &mut result);
    result
}

fn extend_witness_subsets(
    n: usize,
    excluded: usize,
    subset_size: usize,
    next_index: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if current.len() == subset_size {
        result.push(current.clone());
        return;
    }

    for index in next_index..n {
        if index == excluded {
            continue;
        }

        current.push(index);
        extend_witness_subsets(n, excluded, subset_size, index + 1, current, result);
        current.pop();
    }
}

fn has_nonnegative_barycentric_witness<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
    target_index: usize,
    witness_indices: &[usize],
) -> bool {
    let Some(witness_indices) =
        reduce_witness_by_coordinate_bounds(points, target_index, witness_indices)
    else {
        return false;
    };

    let matrix = DMatrix::from_fn(5, witness_indices.len(), |row, col| {
        if row < 4 {
            points[witness_indices[col]][row].clone()
        } else {
            T::one()
        }
    });
    let rhs = DVector::from_fn(5, |row, _| {
        if row < 4 {
            points[target_index][row].clone()
        } else {
            T::one()
        }
    });

    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => particular.iter().all(|weight| weight >= &T::zero()),
        _ => false,
    }
}

fn reduce_witness_by_coordinate_bounds<T: ExactScalar>(
    points: &[Vector4<T>],
    target_index: usize,
    witness_indices: &[usize],
) -> Option<Vec<usize>> {
    let mut active_indices = witness_indices.to_vec();

    for (coordinate, target_coordinate) in points[target_index].iter().enumerate() {
        let min_coordinate = active_indices
            .iter()
            .map(|&idx| &points[idx][coordinate])
            .min()
            .expect("witness subsets are nonempty");
        let max_coordinate = active_indices
            .iter()
            .map(|&idx| &points[idx][coordinate])
            .max()
            .expect("witness subsets are nonempty");

        if target_coordinate < min_coordinate || target_coordinate > max_coordinate {
            return None;
        }

        if target_coordinate == min_coordinate || target_coordinate == max_coordinate {
            active_indices.retain(|&idx| points[idx][coordinate] == *target_coordinate);
            if active_indices.is_empty() {
                return None;
            }
        }
    }

    Some(active_indices)
}

#[cfg(test)]
mod tests {
    use super::{
        all_points_are_extreme_exact, has_nonnegative_barycentric_witness,
        orient4_det_and_error_bound_f64, orient4_sign_f64, origin_in_interior_of_conv_exact,
        origin_in_interior_of_conv_exact_rational, origin_in_interior_of_conv_f64, CertifiedSign,
        OriginInteriorF64,
    };
    use nalgebra::Vector4;
    use num_rational::BigRational;

    type Q = BigRational;

    fn q(n: i64) -> Q {
        Q::from_integer(n.into())
    }

    fn vq(entries: [i64; 4]) -> Vector4<Q> {
        Vector4::new(q(entries[0]), q(entries[1]), q(entries[2]), q(entries[3]))
    }

    fn vf(entries: [f64; 4]) -> Vector4<f64> {
        Vector4::new(entries[0], entries[1], entries[2], entries[3])
    }

    fn det4_i64(rows: [[i64; 4]; 4]) -> i128 {
        let mut determinant = 0i128;
        for (permutation, sign) in super::ORIENT4_PERMUTATIONS {
            let term = (rows[0][permutation[0]] as i128)
                * (rows[1][permutation[1]] as i128)
                * (rows[2][permutation[2]] as i128)
                * (rows[3][permutation[3]] as i128);
            if sign > 0.0 {
                determinant += term;
            } else {
                determinant -= term;
            }
        }
        determinant
    }

    fn f64_rows_from_i64(rows: [[i64; 4]; 4]) -> [Vector4<f64>; 4] {
        rows.map(|row| vf(row.map(|entry| entry as f64)))
    }

    #[test]
    fn orient4_f64_certifies_well_separated_signs_and_tiny_positive() {
        let identity = [
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        assert_eq!(orient4_sign_f64(identity), CertifiedSign::Positive);

        let row_swap = [identity[1], identity[0], identity[2], identity[3]];
        assert_eq!(orient4_sign_f64(row_swap), CertifiedSign::Negative);

        let tiny_positive = [
            vf([1.0e-300, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        assert_eq!(orient4_sign_f64(tiny_positive), CertifiedSign::Positive);

        let rounded_products = [
            vf([1.1, 0.0, 0.0, 0.0]),
            vf([0.0, 1.3, 0.0, 0.0]),
            vf([0.0, 0.0, 1.7, 0.0]),
            vf([0.0, 0.0, 0.0, 1.9]),
        ];
        assert_eq!(orient4_sign_f64(rounded_products), CertifiedSign::Positive);

        let scaled_rounded_products = [
            vf([1.1e-80, 0.0, 0.0, 0.0]),
            vf([0.0, 1.3e20, 0.0, 0.0]),
            vf([0.0, 0.0, 1.7e30, 0.0]),
            vf([0.0, 0.0, 0.0, 1.9e40]),
        ];
        assert_eq!(
            orient4_sign_f64(scaled_rounded_products),
            CertifiedSign::Positive
        );
    }

    #[test]
    fn orient4_f64_uses_error_bound_instead_of_static_margin() {
        let small_positive = [
            vf([1.0e-10, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        let (determinant, error_bound) =
            orient4_det_and_error_bound_f64(small_positive).expect("normal finite inputs");

        assert!(determinant < 1.0e-8);
        assert!(determinant > error_bound);
        assert_eq!(orient4_sign_f64(small_positive), CertifiedSign::Positive);
    }

    #[test]
    fn orient4_f64_certified_signs_match_exact_integer_determinants() {
        let mut state = 0x8f4a_7c15_d3e2_b901_u64;

        for _ in 0..512 {
            let mut rows = [[0i64; 4]; 4];
            for row in &mut rows {
                for entry in row {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    *entry = ((state >> 32) % 7) as i64 - 3;
                }
            }

            let exact_determinant = det4_i64(rows);
            match orient4_sign_f64(f64_rows_from_i64(rows)) {
                CertifiedSign::Positive => assert!(
                    exact_determinant > 0,
                    "certified positive for exact determinant {exact_determinant} and rows {rows:?}"
                ),
                CertifiedSign::Negative => assert!(
                    exact_determinant < 0,
                    "certified negative for exact determinant {exact_determinant} and rows {rows:?}"
                ),
                CertifiedSign::Indeterminate => {}
            }
        }
    }

    #[test]
    fn orient4_f64_keeps_cancellation_cases_indeterminate() {
        let exact_zero_with_large_terms = [
            [1_000_000, 1_000_001, 999_999, 1_000_002],
            [999_998, 1_000_003, 1_000_004, 999_997],
            [1_999_998, 2_000_004, 2_000_003, 1_999_999],
            [3, 5, 7, 11],
        ];
        assert_eq!(det4_i64(exact_zero_with_large_terms), 0);
        assert_eq!(
            orient4_sign_f64(f64_rows_from_i64(exact_zero_with_large_terms)),
            CertifiedSign::Indeterminate
        );

        let x: f64 = 1.0e8;
        let next_x = f64::from_bits(x.to_bits() + 1);
        let near_cancellation = [
            vf([x, x, 0.0, 0.0]),
            vf([x, next_x, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        let (determinant, error_bound) =
            orient4_det_and_error_bound_f64(near_cancellation).expect("normal finite inputs");
        assert!(determinant.abs() <= error_bound);
        assert_eq!(
            orient4_sign_f64(near_cancellation),
            CertifiedSign::Indeterminate
        );
    }

    #[test]
    fn orient4_f64_refuses_zero_subnormal_and_overflow_cases() {
        let zero = [
            vf([0.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        assert_eq!(orient4_sign_f64(zero), CertifiedSign::Indeterminate);

        let subnormal_input = [
            vf([f64::MIN_POSITIVE / 2.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        assert_eq!(
            orient4_sign_f64(subnormal_input),
            CertifiedSign::Indeterminate
        );

        let overflow = [
            vf([1.0e200, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0e200, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0e200, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0e200]),
        ];
        assert_eq!(orient4_sign_f64(overflow), CertifiedSign::Indeterminate);
    }

    #[test]
    fn origin_interior_f64_certifies_robust_true_false_and_boundary_indeterminate() {
        let simplex = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
            vf([-1.0, -1.0, -1.0, -1.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&simplex),
            OriginInteriorF64::True
        );

        let outside = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
            vf([1.0, 1.0, 1.0, 1.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&outside),
            OriginInteriorF64::False
        );

        let boundary = vec![
            vf([0.0, 0.0, 0.0, 0.0]),
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&boundary),
            OriginInteriorF64::Indeterminate
        );

        let outside_with_more_points = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
            vf([1.0, 1.0, 1.0, 1.0]),
            vf([2.0, 1.0, 1.0, 1.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&outside_with_more_points),
            OriginInteriorF64::False
        );
    }

    #[test]
    fn origin_interior_f64_leaves_boundary_and_degenerate_cases_indeterminate() {
        let lower_rank_without_origin_point = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([-1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, -1.0, 0.0, 0.0]),
            vf([1.0, 1.0, 0.0, 0.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&lower_rank_without_origin_point),
            OriginInteriorF64::Indeterminate
        );

        let boundary_without_origin_point = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
            vf([0.0, -1.0, -1.0, -1.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&boundary_without_origin_point),
            OriginInteriorF64::Indeterminate
        );

        let full_dimensional_interior_with_duplicate = vec![
            vf([1.0, 0.0, 0.0, 0.0]),
            vf([-1.0, 0.0, 0.0, 0.0]),
            vf([0.0, 1.0, 0.0, 0.0]),
            vf([0.0, -1.0, 0.0, 0.0]),
            vf([0.0, 0.0, 1.0, 0.0]),
            vf([0.0, 0.0, -1.0, 0.0]),
            vf([0.0, 0.0, 0.0, 1.0]),
            vf([0.0, 0.0, 0.0, -1.0]),
            vf([1.0, 0.0, 0.0, 0.0]),
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&full_dimensional_interior_with_duplicate),
            OriginInteriorF64::Indeterminate
        );
    }

    #[test]
    fn exact_origin_predicate_handles_boundary_and_full_dimensional_interior() {
        let simplex = vec![
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
            vq([-1, -1, -1, -1]),
        ];
        assert!(origin_in_interior_of_conv_exact(&simplex));
        assert!(origin_in_interior_of_conv_exact_rational(&simplex));

        let outside = vec![
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
            vq([1, 1, 1, 1]),
        ];
        assert!(!origin_in_interior_of_conv_exact(&outside));
        assert!(!origin_in_interior_of_conv_exact_rational(&outside));

        let origin_on_boundary = vec![
            vq([0, 0, 0, 0]),
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
        ];
        assert!(!origin_in_interior_of_conv_exact(&origin_on_boundary));
        assert!(!origin_in_interior_of_conv_exact_rational(
            &origin_on_boundary
        ));

        let crosspolytope = vec![
            vq([1, 0, 0, 0]),
            vq([-1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, -1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, -1, 0]),
            vq([0, 0, 0, 1]),
            vq([0, 0, 0, -1]),
        ];
        assert!(origin_in_interior_of_conv_exact(&crosspolytope));
        assert!(origin_in_interior_of_conv_exact_rational(&crosspolytope));
    }

    #[test]
    fn affinely_dependent_witness_is_rejected_but_smaller_witness_decides() {
        let points = vec![
            vq([1, 1, 0, 0]),
            vq([0, 0, 0, 0]),
            vq([2, 0, 0, 0]),
            vq([2, 2, 0, 0]),
            vq([0, 2, 0, 0]),
        ];

        assert!(!has_nonnegative_barycentric_witness(
            &points,
            0,
            &[1, 2, 3, 4]
        ));
        assert!(!all_points_are_extreme_exact(&points));
    }
}
