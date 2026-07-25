use nalgebra::{DMatrix, DVector};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// Solve a full-rank square rational system by fraction-free elimination.
///
/// This is a fast path for exact binary floating-point inputs. It scales each
/// augmented row to integers, applies Bareiss elimination, and verifies the
/// resulting rational solution against the original system. `None` means the
/// matrix is singular, the row denominators are not nested as they are for
/// dyadic rationals, exact division failed, or verification failed. Callers
/// needing a complete solver must then use the generic exact rank/kernel path.
pub fn solve_dyadic_rational_system_full_rank(
    matrix: &DMatrix<BigRational>,
    rhs: &DVector<BigRational>,
) -> Option<DVector<BigRational>> {
    let n = matrix.nrows();
    assert_eq!(matrix.ncols(), n, "fraction-free solve requires square A");
    assert_eq!(rhs.len(), n, "A and b row counts differ");
    if n == 0 {
        return Some(DVector::from_vec(Vec::new()));
    }

    let augmented = DMatrix::from_fn(n, n + 1, |row, col| {
        if col == n {
            rhs[row].clone()
        } else {
            matrix[(row, col)].clone()
        }
    });
    let mut integer = scale_dyadic_rows_to_integers(&augmented)?;
    let mut previous_pivot = BigInt::one();

    for pivot_col in 0..n.saturating_sub(1) {
        let pivot_row = (pivot_col..n).find(|&row| !integer[(row, pivot_col)].is_zero())?;
        if pivot_row != pivot_col {
            integer.swap_rows(pivot_row, pivot_col);
        }
        let pivot = integer[(pivot_col, pivot_col)].clone();
        for row in pivot_col + 1..n {
            for col in pivot_col + 1..=n {
                let numerator = &integer[(row, col)] * &pivot
                    - &integer[(row, pivot_col)] * &integer[(pivot_col, col)];
                if (&numerator % &previous_pivot) != BigInt::zero() {
                    return None;
                }
                integer[(row, col)] = numerator / &previous_pivot;
            }
            integer[(row, pivot_col)] = BigInt::zero();
        }
        previous_pivot = pivot;
    }
    if integer[(n - 1, n - 1)].is_zero() {
        return None;
    }

    let mut solution = DVector::from_element(n, BigRational::zero());
    for row in (0..n).rev() {
        let mut value = BigRational::from_integer(integer[(row, n)].clone());
        for col in row + 1..n {
            value -= BigRational::from_integer(integer[(row, col)].clone()) * solution[col].clone();
        }
        solution[row] = value / BigRational::from_integer(integer[(row, row)].clone());
    }

    exact_solution_holds(matrix, rhs, &solution).then_some(solution)
}

fn scale_dyadic_rows_to_integers(matrix: &DMatrix<BigRational>) -> Option<DMatrix<BigInt>> {
    let mut result = DMatrix::from_element(matrix.nrows(), matrix.ncols(), BigInt::zero());
    for row in 0..matrix.nrows() {
        let scale = (0..matrix.ncols())
            .map(|col| matrix[(row, col)].denom())
            .max()
            .cloned()
            .unwrap_or_else(BigInt::one);
        for col in 0..matrix.ncols() {
            let value = &matrix[(row, col)];
            if (&scale % value.denom()) != BigInt::zero() {
                return None;
            }
            result[(row, col)] = value.numer() * (&scale / value.denom());
        }
    }
    Some(result)
}

fn exact_solution_holds(
    matrix: &DMatrix<BigRational>,
    rhs: &DVector<BigRational>,
    solution: &DVector<BigRational>,
) -> bool {
    matrix * solution == rhs.clone()
}

#[cfg(test)]
mod tests {
    use super::solve_dyadic_rational_system_full_rank;
    use nalgebra::{dmatrix, dvector};
    use num_rational::BigRational;

    fn rational(numerator: i64, denominator: i64) -> BigRational {
        BigRational::new(numerator.into(), denominator.into())
    }

    #[test]
    fn solves_full_rank_dyadic_system_exactly() {
        let matrix = dmatrix![
            rational(1, 2), rational(1, 4);
            rational(3, 8), rational(-1, 2)
        ];
        let expected = dvector![rational(3, 2), rational(-5, 4)];
        let rhs = &matrix * &expected;

        let observed =
            solve_dyadic_rational_system_full_rank(&matrix, &rhs).expect("dyadic full-rank system");
        assert_eq!(observed, expected);
    }

    #[test]
    fn solves_after_exact_row_swap() {
        let matrix = dmatrix![
            rational(0, 1), rational(1, 2);
            rational(1, 4), rational(1, 8)
        ];
        let expected = dvector![rational(3, 2), rational(-5, 4)];
        let rhs = &matrix * &expected;

        let observed =
            solve_dyadic_rational_system_full_rank(&matrix, &rhs).expect("row-swapped system");
        assert_eq!(observed, expected);
    }

    #[test]
    fn returns_none_for_singular_system() {
        let matrix = dmatrix![
            rational(1, 2), rational(1, 4);
            rational(1, 1), rational(1, 2)
        ];
        let rhs = dvector![rational(1, 1), rational(2, 1)];
        assert!(solve_dyadic_rational_system_full_rank(&matrix, &rhs).is_none());
    }

    #[test]
    fn returns_none_when_row_denominators_are_not_dyadic_nested() {
        let matrix = dmatrix![
            rational(1, 2), rational(1, 3);
            rational(0, 1), rational(1, 1)
        ];
        let rhs = dvector![rational(1, 1), rational(1, 1)];
        assert!(solve_dyadic_rational_system_full_rank(&matrix, &rhs).is_none());
    }
}
