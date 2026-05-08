use nalgebra::DMatrix;

use crate::ExactScalar;

pub fn is_negative_definite<T: ExactScalar + 'static>(matrix: &DMatrix<T>) -> bool {
    assert_eq!(
        matrix.nrows(),
        matrix.ncols(),
        "negative-definite check requires a square matrix, got {}x{}",
        matrix.nrows(),
        matrix.ncols()
    );
    assert_symmetric(matrix);

    for size in 1..=matrix.nrows() {
        let minor = leading_principal_minor(matrix, size);
        let det = determinant(&minor);
        let has_expected_sign = if size % 2 == 1 {
            det < T::zero()
        } else {
            det > T::zero()
        };

        if !has_expected_sign {
            return false;
        }
    }

    true
}

fn assert_symmetric<T: ExactScalar + 'static>(matrix: &DMatrix<T>) {
    for row in 0..matrix.nrows() {
        for col in 0..row {
            assert_eq!(
                matrix[(row, col)],
                matrix[(col, row)],
                "negative-definite check requires a symmetric matrix"
            );
        }
    }
}

fn leading_principal_minor<T: ExactScalar + 'static>(
    matrix: &DMatrix<T>,
    size: usize,
) -> DMatrix<T> {
    DMatrix::from_fn(size, size, |row, col| matrix[(row, col)].clone())
}

fn determinant<T: ExactScalar + 'static>(matrix: &DMatrix<T>) -> T {
    debug_assert_eq!(matrix.nrows(), matrix.ncols());

    let n = matrix.nrows();
    let mut work = matrix.clone();
    let mut det = T::one();

    for col in 0..n {
        let Some(pivot_row) = (col..n).find(|&row| !work[(row, col)].is_zero()) else {
            return T::zero();
        };

        if pivot_row != col {
            work.swap_rows(pivot_row, col);
            det = -det;
        }

        let pivot = work[(col, col)].clone();
        det *= pivot.clone();

        for row in (col + 1)..n {
            let factor = work[(row, col)].clone() / pivot.clone();
            if factor.is_zero() {
                continue;
            }

            for target_col in col..n {
                work[(row, target_col)] = work[(row, target_col)].clone()
                    - factor.clone() * work[(col, target_col)].clone();
            }
        }
    }

    det
}
