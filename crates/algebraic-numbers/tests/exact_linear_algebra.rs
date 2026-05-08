mod common;

use algebraic_numbers::{
    is_negative_definite, kernel_basis, rank, row_reduction, solve_linear_system,
    LinearSystemSolution,
};
use common::{a, Qsqrt5};
use nalgebra::{DMatrix, DVector};
use num_rational::BigRational;
use proptest::prelude::*;

fn br(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn rational_matrix(rows: usize, cols: usize, entries: &[i64]) -> DMatrix<BigRational> {
    DMatrix::from_row_slice(
        rows,
        cols,
        &entries.iter().copied().map(br).collect::<Vec<_>>(),
    )
}

fn rational_vector(entries: &[i64]) -> DVector<BigRational> {
    DVector::from_column_slice(&entries.iter().copied().map(br).collect::<Vec<_>>())
}

#[test]
fn row_reduction_and_rank_over_bigrational_are_plain_nalgebra_calls() {
    let matrix = rational_matrix(3, 3, &[1, 2, 3, 2, 4, 6, 1, 1, 0]);

    let reduction = row_reduction(&matrix);

    assert_eq!(rank(&matrix), 2);
    assert_eq!(reduction.pivot_columns, vec![0, 1]);
    assert_eq!(
        reduction.rref,
        rational_matrix(3, 3, &[1, 0, -3, 0, 1, 3, 0, 0, 0])
    );
}

#[test]
fn row_reduction_swaps_rows_to_use_first_nonzero_pivot() {
    let matrix = rational_matrix(2, 2, &[0, 1, 2, 3]);

    let reduction = row_reduction(&matrix);

    assert_eq!(reduction.pivot_columns, vec![0, 1]);
    assert_eq!(reduction.rref, rational_matrix(2, 2, &[1, 0, 0, 1]));
}

#[test]
fn kernel_basis_handles_free_column_before_later_pivot() {
    let matrix = rational_matrix(2, 3, &[0, 1, 2, 0, 2, 4]);
    let reduction = row_reduction(&matrix);
    let basis = kernel_basis(&matrix);

    assert_eq!(reduction.pivot_columns, vec![1]);
    assert_eq!(reduction.rref, rational_matrix(2, 3, &[0, 1, 2, 0, 0, 0]));
    assert_eq!(basis, rational_matrix(3, 2, &[1, 0, 0, -2, 0, 1]));

    for col in 0..basis.ncols() {
        let vector = basis.column(col).into_owned();
        assert_eq!(&matrix * &vector, DVector::zeros(2));
    }
}

#[test]
fn solve_unique_system_returns_consistent_solution_with_empty_kernel() {
    let matrix = rational_matrix(2, 2, &[2, 1, 1, -1]);
    let rhs = rational_vector(&[5, 1]);

    let solution = solve_linear_system(&matrix, &rhs);

    match solution {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            assert_eq!(particular, rational_vector(&[2, 1]));
            assert_eq!(kernel_basis.nrows(), 2);
            assert_eq!(kernel_basis.ncols(), 0);
        }
        LinearSystemSolution::Inconsistent => panic!("expected consistent system"),
    }
}

#[test]
fn solve_inconsistent_system_returns_inconsistent() {
    let matrix = rational_matrix(2, 2, &[1, 1, 2, 2]);
    let rhs = rational_vector(&[1, 3]);

    assert_eq!(
        solve_linear_system(&matrix, &rhs),
        LinearSystemSolution::Inconsistent
    );
}

#[test]
fn solve_underdetermined_system_returns_particular_and_kernel_basis() {
    let matrix = rational_matrix(1, 3, &[1, 1, 1]);
    let rhs = rational_vector(&[3]);

    let solution = solve_linear_system(&matrix, &rhs);

    match solution {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            assert_eq!(particular, rational_vector(&[3, 0, 0]));
            assert_eq!(kernel_basis, rational_matrix(3, 2, &[-1, -1, 1, 0, 0, 1]));
            assert_eq!(&matrix * &particular, rhs);
            for col in 0..kernel_basis.ncols() {
                let vector = kernel_basis.column(col).into_owned();
                assert_eq!(&matrix * &vector, DVector::zeros(1));
            }
        }
        LinearSystemSolution::Inconsistent => panic!("expected consistent system"),
    }
}

#[test]
fn q_sqrt5_matrices_use_same_public_api() {
    let root = Qsqrt5::root();
    let matrix = DMatrix::from_row_slice(2, 2, &[a(1, 0), root.clone(), a(0, 0), a(1, 0)]);
    let rhs = DVector::from_column_slice(&[a(3, 1), a(2, 0)]);

    assert_eq!(rank(&matrix), 2);

    let solution = solve_linear_system(&matrix, &rhs);

    match solution {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            assert_eq!(particular, DVector::from_column_slice(&[a(3, -1), a(2, 0)]));
            assert_eq!(kernel_basis.ncols(), 0);
            assert_eq!(&matrix * &particular, rhs);
        }
        LinearSystemSolution::Inconsistent => panic!("expected consistent system"),
    }
}

#[test]
#[should_panic(expected = "matrix has 2 rows but rhs has 3 rows")]
fn solve_panics_for_dimension_mismatch() {
    let matrix = rational_matrix(2, 2, &[1, 0, 0, 1]);
    let rhs = rational_vector(&[1, 2, 3]);

    let _ = solve_linear_system(&matrix, &rhs);
}

#[test]
fn kernel_basis_columns_are_exact_kernel_vectors() {
    let matrix = rational_matrix(2, 4, &[1, 0, 1, 0, 0, 1, 0, 1]);
    let basis = kernel_basis(&matrix);

    assert_eq!(basis, rational_matrix(4, 2, &[-1, 0, 0, -1, 1, 0, 0, 1]));

    for col in 0..basis.ncols() {
        let vector = basis.column(col).into_owned();
        assert_eq!(&matrix * &vector, DVector::zeros(2));
    }
}

#[test]
fn negative_definite_bigrational_cases_are_exact() {
    assert!(is_negative_definite(&rational_matrix(
        2,
        2,
        &[-1, 0, 0, -2]
    )));
    assert!(is_negative_definite(&rational_matrix(
        2,
        2,
        &[-2, 1, 1, -2]
    )));
    assert!(is_negative_definite(&DMatrix::<BigRational>::zeros(0, 0)));
    assert!(!is_negative_definite(&rational_matrix(
        2,
        2,
        &[-1, 0, 0, 1]
    )));
    assert!(!is_negative_definite(&rational_matrix(
        2,
        2,
        &[-1, 2, 2, -1]
    )));
    assert!(!is_negative_definite(&rational_matrix(
        2,
        2,
        &[-1, 0, 0, 0]
    )));
}

#[test]
fn negative_definite_q_sqrt5_cases_are_exact() {
    let negative = DMatrix::from_row_slice(2, 2, &[a(-3, 1), a(0, 0), a(0, 0), a(-1, 0)]);
    let non_diagonal_negative =
        DMatrix::from_row_slice(2, 2, &[a(-4, 1), a(1, 0), a(1, 0), a(-1, 0)]);
    let indefinite = DMatrix::from_row_slice(2, 2, &[a(-1, 0), a(0, 0), a(0, 0), a(0, 1)]);
    let non_diagonal_indefinite =
        DMatrix::from_row_slice(2, 2, &[a(-1, 0), a(0, 1), a(0, 1), a(-1, 0)]);

    assert!(is_negative_definite(&negative));
    assert!(is_negative_definite(&non_diagonal_negative));
    assert!(!is_negative_definite(&indefinite));
    assert!(!is_negative_definite(&non_diagonal_indefinite));
}

#[test]
#[should_panic(expected = "negative-definite check requires a square matrix")]
fn negative_definite_panics_for_nonsquare_matrix() {
    let _ = is_negative_definite(&rational_matrix(2, 3, &[1, 0, 0, 0, 1, 0]));
}

#[test]
#[should_panic(expected = "negative-definite check requires a symmetric matrix")]
fn negative_definite_panics_for_nonsymmetric_matrix() {
    let _ = is_negative_definite(&rational_matrix(2, 2, &[-1, 1, 0, -1]));
}

proptest! {
    #[test]
    fn row_reduction_is_idempotent(entries in proptest::collection::vec(-3_i64..=3, 9)) {
        let matrix = rational_matrix(3, 3, &entries);

        let once = row_reduction(&matrix).rref;
        let twice = row_reduction(&once).rref;

        prop_assert_eq!(once, twice);
    }

    #[test]
    fn rank_matches_rank_of_rref(entries in proptest::collection::vec(-3_i64..=3, 9)) {
        let matrix = rational_matrix(3, 3, &entries);
        let rref = row_reduction(&matrix).rref;

        prop_assert_eq!(rank(&matrix), rank(&rref));
    }

    #[test]
    fn generated_consistent_systems_satisfy_solution(
        entries in proptest::collection::vec(-3_i64..=3, 6),
        solution_entries in proptest::collection::vec(-3_i64..=3, 3),
    ) {
        let matrix = rational_matrix(2, 3, &entries);
        let expected_solution = rational_vector(&solution_entries);
        let rhs = &matrix * &expected_solution;

        let solution = solve_linear_system(&matrix, &rhs);

        match solution {
            LinearSystemSolution::Consistent { particular, kernel_basis } => {
                prop_assert_eq!(&matrix * &particular, rhs);
                for col in 0..kernel_basis.ncols() {
                    let vector = kernel_basis.column(col).into_owned();
                    prop_assert_eq!(&matrix * &vector, DVector::zeros(2));
                }
            }
            LinearSystemSolution::Inconsistent => {
                prop_assert!(false, "generated system should be consistent");
            }
        }
    }
}
