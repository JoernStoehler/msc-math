use nalgebra::{DMatrix, DVector};

use crate::{row_reduction, ExactScalar};

/// Exact solution set of `A x = b`.
#[derive(Clone, Debug, PartialEq)]
pub enum LinearSystemSolution<T> {
    Inconsistent,
    Consistent {
        particular: DVector<T>,
        kernel_basis: DMatrix<T>,
    },
}

pub fn solve_linear_system<T: ExactScalar + 'static>(
    matrix: &DMatrix<T>,
    rhs: &DVector<T>,
) -> LinearSystemSolution<T> {
    assert_eq!(
        matrix.nrows(),
        rhs.nrows(),
        "matrix has {} rows but rhs has {} rows",
        matrix.nrows(),
        rhs.nrows()
    );

    let augmented = augmented_matrix(matrix, rhs);
    let reduction = row_reduction(&augmented);
    let coefficient_cols = matrix.ncols();

    if has_inconsistent_row(&reduction.rref, coefficient_cols) {
        return LinearSystemSolution::Inconsistent;
    }

    let mut particular = DVector::from_element(coefficient_cols, T::zero());
    for (row, &col) in reduction.pivot_columns.iter().enumerate() {
        if col < coefficient_cols {
            particular[col] = reduction.rref[(row, coefficient_cols)].clone();
        }
    }

    LinearSystemSolution::Consistent {
        particular,
        kernel_basis: kernel_basis(matrix),
    }
}

pub fn kernel_basis<T: ExactScalar + 'static>(matrix: &DMatrix<T>) -> DMatrix<T> {
    let reduction = row_reduction(matrix);
    let cols = matrix.ncols();
    let free_columns = free_columns(cols, &reduction.pivot_columns);
    let mut basis = DMatrix::from_element(cols, free_columns.len(), T::zero());

    for (basis_col, &free_col) in free_columns.iter().enumerate() {
        basis[(free_col, basis_col)] = T::one();

        for (row, &pivot_col) in reduction.pivot_columns.iter().enumerate() {
            basis[(pivot_col, basis_col)] = -reduction.rref[(row, free_col)].clone();
        }
    }

    basis
}

fn augmented_matrix<T: ExactScalar + 'static>(matrix: &DMatrix<T>, rhs: &DVector<T>) -> DMatrix<T> {
    DMatrix::from_fn(matrix.nrows(), matrix.ncols() + 1, |row, col| {
        if col == matrix.ncols() {
            rhs[row].clone()
        } else {
            matrix[(row, col)].clone()
        }
    })
}

fn has_inconsistent_row<T: ExactScalar + 'static>(
    rref: &DMatrix<T>,
    coefficient_cols: usize,
) -> bool {
    for row in 0..rref.nrows() {
        let left_is_zero = (0..coefficient_cols).all(|col| rref[(row, col)].is_zero());
        if left_is_zero && !rref[(row, coefficient_cols)].is_zero() {
            return true;
        }
    }

    false
}

fn free_columns(cols: usize, pivot_columns: &[usize]) -> Vec<usize> {
    (0..cols)
        .filter(|col| !pivot_columns.contains(col))
        .collect()
}
