use algebraic_numbers::{rank, ExactScalar};
use nalgebra::{DMatrix, Vector4};

use crate::linalg::{combinations3, cross_product_4d_exact, dot4_exact, is_zero_vector_exact};

/// Return whether `0` lies in the interior of `conv(points)` in ambient `R^4`.
///
/// This is an exact positive-spanning test. It returns `false` for lower-rank
/// input, including empty and lower-dimensional point sets.
pub fn origin_in_interior_of_conv_exact<T: ExactScalar + 'static>(points: &[Vector4<T>]) -> bool {
    if points.len() < 5 {
        return false;
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
