//! Purpose: small linear-algebra helpers over ordered scalar types.
//! Context: the crate intentionally ships only the routines that downstream
//! geometry and KKT code reaches for repeatedly.

use crate::field::OrderedField;

/// Result of solving one square linear system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveResult<T> {
    Unique(T),
    Singular,
}

/// Solve a dense `N x N` linear system by Gaussian elimination.
pub fn solve_square<const N: usize, F: OrderedField>(
    matrix: &[[F; N]; N],
    rhs: &[F; N],
) -> SolveResult<[F; N]> {
    let mut aug: Vec<Vec<F>> = (0..N)
        .map(|row| {
            let mut line: Vec<F> = (0..N).map(|col| matrix[row][col].clone()).collect();
            line.push(rhs[row].clone());
            line
        })
        .collect();

    for col in 0..N {
        let Some(pivot_row) = (col..N).find(|&row| !aug[row][col].is_zero()) else {
            return SolveResult::Singular;
        };
        aug.swap(col, pivot_row);
        let pivot = aug[col][col].clone();
        for row in (col + 1)..N {
            if aug[row][col].is_zero() {
                continue;
            }
            let factor = aug[row][col].clone() / pivot.clone();
            for j in col..=N {
                let correction = aug[col][j].clone() * factor.clone();
                aug[row][j] = aug[row][j].clone() - correction;
            }
        }
    }

    let mut solution = std::array::from_fn(|_| F::zero());
    for row in (0..N).rev() {
        if aug[row][row].is_zero() {
            return SolveResult::Singular;
        }
        let mut rhs_val = aug[row][N].clone();
        for col in (row + 1)..N {
            rhs_val = rhs_val - aug[row][col].clone() * solution[col].clone();
        }
        solution[row] = rhs_val / aug[row][row].clone();
    }

    SolveResult::Unique(solution)
}

/// Row rank of a dense matrix given as owned rows.
pub fn rank_rows<F: OrderedField>(rows: &[Vec<F>]) -> usize {
    if rows.is_empty() {
        return 0;
    }

    let mut mat = rows.to_vec();
    let m = mat.len();
    let ncols = mat[0].len();
    let mut rank = 0usize;

    for col in 0..ncols {
        let Some(pivot_row) = (rank..m).find(|&row| !mat[row][col].is_zero()) else {
            continue;
        };
        mat.swap(rank, pivot_row);
        let pivot = mat[rank][col].clone();
        for row in 0..m {
            if row == rank || mat[row][col].is_zero() {
                continue;
            }
            let factor = mat[row][col].clone() / pivot.clone();
            for j in col..ncols {
                let correction = factor.clone() * mat[rank][j].clone();
                mat[row][j] = mat[row][j].clone() - correction;
            }
        }
        rank += 1;
    }

    rank
}

#[cfg(test)]
#[path = "test_linear.rs"]
mod test_linear;
