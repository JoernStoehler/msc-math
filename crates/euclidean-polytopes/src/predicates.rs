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

    if let Some(witness_indices) = f64_origin_simplex_witness(points) {
        if exact_origin_simplex_witness(points, &witness_indices) {
            return true;
        }
    }

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

fn f64_origin_simplex_witness<T: ExactScalar>(points: &[Vector4<T>]) -> Option<[usize; 5]> {
    const MIN_BARYCENTRIC_WEIGHT: f64 = 1e-8;

    let points_f64 = points
        .iter()
        .map(|point| {
            let coordinates: [f64; 4] =
                std::array::from_fn(|coordinate| point[coordinate].round_to_f64());
            coordinates
                .iter()
                .all(|coordinate| coordinate.is_finite())
                .then_some(coordinates)
        })
        .collect::<Option<Vec<_>>>()?;

    for indices in combinations5(points.len()) {
        let matrix = Matrix5::from_fn(|row, col| {
            if row < 4 {
                points_f64[indices[col]][row]
            } else {
                1.0
            }
        });
        let rhs = Vector5::new(0.0, 0.0, 0.0, 0.0, 1.0);
        let Some(weights) = matrix.lu().solve(&rhs) else {
            continue;
        };
        if weights
            .iter()
            .all(|weight| weight.is_finite() && *weight > MIN_BARYCENTRIC_WEIGHT)
        {
            return Some(indices);
        }
    }

    None
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
    use super::{all_points_are_extreme_exact, has_nonnegative_barycentric_witness};
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
