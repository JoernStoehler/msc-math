use super::*;
use crate::field::OrderedField;
use num_rational::BigRational;

#[test]
fn solve_square_finds_unique_rational_solution() {
    let matrix = [
        [BigRational::from_i64(2), BigRational::from_i64(1)],
        [BigRational::from_i64(1), BigRational::from_i64(1)],
    ];
    let rhs = [BigRational::from_i64(1), BigRational::from_i64(0)];
    let SolveResult::Unique(solution) = solve_square(&matrix, &rhs) else {
        panic!("expected unique solution");
    };
    assert_eq!(solution[0], BigRational::from_i64(1));
    assert_eq!(solution[1], BigRational::from_i64(-1));
}

#[test]
fn solve_square_detects_singular_system() {
    let matrix = [
        [BigRational::from_i64(1), BigRational::from_i64(2)],
        [BigRational::from_i64(2), BigRational::from_i64(4)],
    ];
    let rhs = [BigRational::from_i64(1), BigRational::from_i64(2)];
    assert_eq!(solve_square(&matrix, &rhs), SolveResult::Singular);
}

#[test]
fn rank_rows_counts_full_and_deficient_cases() {
    let full_rank = vec![
        vec![BigRational::from_i64(1), BigRational::from_i64(0)],
        vec![BigRational::from_i64(0), BigRational::from_i64(1)],
    ];
    let rank_deficient = vec![
        vec![BigRational::from_i64(1), BigRational::from_i64(2)],
        vec![BigRational::from_i64(2), BigRational::from_i64(4)],
    ];
    assert_eq!(rank_rows(&full_rank), 2);
    assert_eq!(rank_rows(&rank_deficient), 1);
}

#[test]
fn solve_square_swaps_rows_when_the_diagonal_entry_is_zero() {
    let matrix = [
        [BigRational::from_i64(0), BigRational::from_i64(1)],
        [BigRational::from_i64(2), BigRational::from_i64(3)],
    ];
    let rhs = [BigRational::from_i64(1), BigRational::from_i64(5)];
    let SolveResult::Unique(solution) = solve_square(&matrix, &rhs) else {
        panic!("expected unique solution after row swap");
    };
    assert_eq!(solution[0], BigRational::from_i64(1));
    assert_eq!(solution[1], BigRational::from_i64(1));
}

#[test]
fn rank_rows_is_invariant_under_row_operations() {
    let original = vec![
        vec![
            BigRational::from_i64(1),
            BigRational::from_i64(2),
            BigRational::from_i64(3),
        ],
        vec![
            BigRational::from_i64(0),
            BigRational::from_i64(1),
            BigRational::from_i64(4),
        ],
        vec![
            BigRational::from_i64(1),
            BigRational::from_i64(3),
            BigRational::from_i64(7),
        ],
    ];
    let row_equivalent = vec![
        original[2].clone(),
        original[1].clone(),
        vec![
            original[0][0].clone() - original[1][0].clone(),
            original[0][1].clone() - original[1][1].clone(),
            original[0][2].clone() - original[1][2].clone(),
        ],
    ];

    assert_eq!(rank_rows(&original), 2);
    assert_eq!(rank_rows(&row_equivalent), 2);
}
