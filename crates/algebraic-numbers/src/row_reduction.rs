use nalgebra::DMatrix;

use crate::ExactScalar;

/// Reduced row-echelon form of a dense exact matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct RowReduction<T> {
    pub rref: DMatrix<T>,
    pub pivot_columns: Vec<usize>,
}

/// Compute reduced row-echelon form using exact Gaussian elimination.
pub fn row_reduction<T: ExactScalar + 'static>(matrix: &DMatrix<T>) -> RowReduction<T> {
    let mut rref = matrix.clone();
    let mut pivot_columns = Vec::new();
    let mut pivot_row = 0;

    for col in 0..rref.ncols() {
        let Some(source_row) = find_pivot_row(&rref, pivot_row, col) else {
            continue;
        };

        if source_row != pivot_row {
            rref.swap_rows(source_row, pivot_row);
        }

        normalize_pivot_row(&mut rref, pivot_row, col);
        eliminate_pivot_column(&mut rref, pivot_row, col);
        pivot_columns.push(col);
        pivot_row += 1;

        if pivot_row == rref.nrows() {
            break;
        }
    }

    RowReduction {
        rref,
        pivot_columns,
    }
}

pub fn rank<T: ExactScalar + 'static>(matrix: &DMatrix<T>) -> usize {
    row_reduction(matrix).pivot_columns.len()
}

fn find_pivot_row<T: ExactScalar + 'static>(
    matrix: &DMatrix<T>,
    start_row: usize,
    col: usize,
) -> Option<usize> {
    (start_row..matrix.nrows()).find(|&row| !matrix[(row, col)].is_zero())
}

fn normalize_pivot_row<T: ExactScalar + 'static>(
    matrix: &mut DMatrix<T>,
    pivot_row: usize,
    pivot_col: usize,
) {
    let pivot = matrix[(pivot_row, pivot_col)].clone();
    if pivot.is_one() {
        return;
    }

    for col in 0..matrix.ncols() {
        matrix[(pivot_row, col)] = matrix[(pivot_row, col)].clone() / pivot.clone();
    }
}

fn eliminate_pivot_column<T: ExactScalar + 'static>(
    matrix: &mut DMatrix<T>,
    pivot_row: usize,
    pivot_col: usize,
) {
    for row in 0..matrix.nrows() {
        if row == pivot_row {
            continue;
        }

        let factor = matrix[(row, pivot_col)].clone();
        if factor.is_zero() {
            continue;
        }

        for col in 0..matrix.ncols() {
            matrix[(row, col)] =
                matrix[(row, col)].clone() - factor.clone() * matrix[(pivot_row, col)].clone();
        }
    }
}
