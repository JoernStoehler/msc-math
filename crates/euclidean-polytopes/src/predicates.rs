use algebraic_numbers::{rank, solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Matrix5, Vector4, Vector5};

use crate::linalg::{
    combinations3, combinations5, cross_product_4d_exact, dot4_exact, is_zero_vector_exact,
};

/// Return whether `0` lies in the interior of `conv(points)` in ambient `R^4`.
///
/// This is an exact positive-spanning test. It returns `false` for lower-rank
/// input, including empty and lower-dimensional point sets.
pub fn origin_in_interior_of_conv_exact<T: ExactScalar + 'static>(points: &[Vector4<T>]) -> bool {
    if points.len() < 5 {
        return false;
    }

    if let Some(true_or_unknown) = origin_in_interior_of_conv_exact_from_f64_candidates(points) {
        return true_or_unknown;
    }

    origin_in_interior_of_conv_exact_slow(points)
}

/// Validation predicate for checked polar APIs.
///
/// This accepts f64-decided `true`/`false` cases under the local margin
/// contract, and uses exact work only for f64-indeterminate inputs. Keep
/// [`origin_in_interior_of_conv_exact`] exact; this helper is a polar validation
/// policy, not a reusable exact predicate.
pub(crate) fn origin_in_interior_of_conv_f64_first_for_polar_validation<
    T: ExactScalar + 'static,
>(
    points: &[Vector4<T>],
) -> bool {
    if points.len() < 5 {
        return false;
    }

    let Some(points_f64) = round_points_to_f64(points) else {
        return origin_in_interior_of_conv_exact_slow(points);
    };

    match origin_in_interior_of_conv_f64(&points_f64) {
        OriginInteriorF64::True(_) => true,
        OriginInteriorF64::False => false,
        OriginInteriorF64::Indeterminate(witnesses_to_check) => {
            for witness_indices in witnesses_to_check {
                if exact_origin_simplex_witness(points, &witness_indices) {
                    return true;
                }
            }
            origin_in_interior_of_conv_exact_slow(points)
        }
    }
}

fn origin_in_interior_of_conv_exact_from_f64_candidates<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
) -> Option<bool> {
    let points_f64 = round_points_to_f64(points)?;
    match origin_in_interior_of_conv_f64(&points_f64) {
        OriginInteriorF64::True(witness_indices) => {
            exact_origin_simplex_witness(points, &witness_indices).then_some(true)
        }
        OriginInteriorF64::False => None,
        OriginInteriorF64::Indeterminate(witnesses_to_check) => {
            for witness_indices in witnesses_to_check {
                if exact_origin_simplex_witness(points, &witness_indices) {
                    return Some(true);
                }
            }
            None
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
enum OriginInteriorF64 {
    True([usize; 5]),
    False,
    Indeterminate(Vec<[usize; 5]>),
}

fn round_points_to_f64<T: ExactScalar>(points: &[Vector4<T>]) -> Option<Vec<[f64; 4]>> {
    points
        .iter()
        .map(|point| {
            let coordinates: [f64; 4] =
                std::array::from_fn(|coordinate| point[coordinate].round_to_f64());
            coordinates
                .iter()
                .all(|coordinate| coordinate.is_finite())
                .then_some(coordinates)
        })
        .collect()
}

fn origin_in_interior_of_conv_f64(points: &[[f64; 4]]) -> OriginInteriorF64 {
    const MIN_BARYCENTRIC_WEIGHT: f64 = 1e-8;

    if points.len() < 5 {
        return OriginInteriorF64::False;
    }

    let mut witnesses_to_check = Vec::new();
    for indices in combinations5(points.len()) {
        let matrix = Matrix5::from_fn(|row, col| {
            if row < 4 {
                points[indices[col]][row]
            } else {
                1.0
            }
        });
        let rhs = Vector5::new(0.0, 0.0, 0.0, 0.0, 1.0);
        let Some(weights) = matrix.lu().solve(&rhs) else {
            witnesses_to_check.push(indices);
            continue;
        };
        if weights.iter().any(|weight| !weight.is_finite()) {
            witnesses_to_check.push(indices);
            continue;
        }
        if weights
            .iter()
            .all(|weight| *weight > MIN_BARYCENTRIC_WEIGHT)
        {
            return OriginInteriorF64::True(indices);
        }
        if weights
            .iter()
            .all(|weight| *weight >= -MIN_BARYCENTRIC_WEIGHT)
        {
            witnesses_to_check.push(indices);
        }
    }

    if witnesses_to_check.is_empty() {
        OriginInteriorF64::False
    } else {
        OriginInteriorF64::Indeterminate(witnesses_to_check)
    }
}

fn exact_origin_simplex_witness<T: ExactScalar + 'static>(
    points: &[Vector4<T>],
    indices: &[usize; 5],
) -> bool {
    let matrix = DMatrix::from_fn(5, 5, |row, col| {
        if row < 4 {
            points[indices[col]][row].clone()
        } else {
            T::one()
        }
    });
    let rhs = DVector::from_fn(5, |row, _| if row < 4 { T::zero() } else { T::one() });

    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => particular.iter().all(|weight| weight > &T::zero()),
        _ => false,
    }
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
        origin_in_interior_of_conv_exact, origin_in_interior_of_conv_f64,
        origin_in_interior_of_conv_f64_first_for_polar_validation, OriginInteriorF64,
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

    #[test]
    fn f64_origin_diagnostic_separates_simplex_witness_from_indeterminate() {
        let too_few_points = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&too_few_points),
            OriginInteriorF64::False
        );

        let simplex = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [-1.0, -1.0, -1.0, -1.0],
        ];
        assert!(matches!(
            origin_in_interior_of_conv_f64(&simplex),
            OriginInteriorF64::True([0, 1, 2, 3, 4])
        ));

        let origin_on_boundary = [
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(matches!(
            origin_in_interior_of_conv_f64(&origin_on_boundary),
            OriginInteriorF64::Indeterminate(witnesses) if !witnesses.is_empty()
        ));

        let robustly_outside = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        ];
        assert_eq!(
            origin_in_interior_of_conv_f64(&robustly_outside),
            OriginInteriorF64::False
        );

        let crosspolytope = [
            [1.0, 0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, -1.0],
        ];
        assert!(matches!(
            origin_in_interior_of_conv_f64(&crosspolytope),
            OriginInteriorF64::Indeterminate(witnesses) if !witnesses.is_empty()
        ));
    }

    #[test]
    fn origin_validation_accepts_f64_decisions_but_exact_predicate_stays_exact() {
        let simplex = vec![
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
            vq([-1, -1, -1, -1]),
        ];
        assert!(origin_in_interior_of_conv_f64_first_for_polar_validation(
            &simplex
        ));

        let outside = vec![
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
            vq([1, 1, 1, 1]),
        ];
        assert!(!origin_in_interior_of_conv_f64_first_for_polar_validation(
            &outside
        ));
        assert!(!origin_in_interior_of_conv_exact(&outside));

        let origin_on_boundary = vec![
            vq([0, 0, 0, 0]),
            vq([1, 0, 0, 0]),
            vq([0, 1, 0, 0]),
            vq([0, 0, 1, 0]),
            vq([0, 0, 0, 1]),
        ];
        assert!(!origin_in_interior_of_conv_f64_first_for_polar_validation(
            &origin_on_boundary
        ));
        assert!(!origin_in_interior_of_conv_exact(&origin_on_boundary));

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
        assert!(origin_in_interior_of_conv_f64_first_for_polar_validation(
            &crosspolytope
        ));
        assert!(origin_in_interior_of_conv_exact(&crosspolytope));
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
