//! Property-style tests for arithmetic and tiny linear algebra.

use algebraic_numbers::{solve_square, Algebraic, OrderedField, SolveResult, StaticFieldSpec};
use num_rational::BigRational;
use proptest::prelude::*;

struct SqrtTwo;

impl StaticFieldSpec for SqrtTwo {
    fn name() -> &'static str {
        "Q(sqrt(2))"
    }

    fn generator_name() -> &'static str {
        "s"
    }

    fn minimal_polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-2).into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(1.into()),
        ]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (
            BigRational::from_integer(1.into()),
            BigRational::from_integer(2.into()),
        )
    }
}

type SqrtTwoField = Algebraic<SqrtTwo>;

fn rational_from_i16(value: i16) -> BigRational {
    BigRational::from_integer((value as i64).into())
}

fn sqrt_two_field_from_pair(coeffs: (i16, i16)) -> SqrtTwoField {
    SqrtTwoField::from_coeffs(vec![
        rational_from_i16(coeffs.0),
        rational_from_i16(coeffs.1),
    ])
}

prop_compose! {
    fn sqrt_two_elem()(a in -5i16..=5i16, b in -5i16..=5i16) -> SqrtTwoField {
        sqrt_two_field_from_pair((a, b))
    }
}

fn invertible_rational_matrix_2x2(
) -> impl Strategy<Value = ([[BigRational; 2]; 2], [BigRational; 2])> {
    (-5i16..=5i16, -5i16..=5i16, -5i16..=5i16, -5i16..=5i16)
        .prop_filter("matrix must be invertible", |(a, b, c, d)| {
            (*a as i32) * (*d as i32) - (*b as i32) * (*c as i32) != 0
        })
        .prop_map(|(a, b, c, d)| {
            (
                [
                    [rational_from_i16(a), rational_from_i16(b)],
                    [rational_from_i16(c), rational_from_i16(d)],
                ],
                [BigRational::from_i64(1), BigRational::from_i64(-1)],
            )
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn addition_is_commutative(x in sqrt_two_elem(), y in sqrt_two_elem()) {
        prop_assert_eq!(x.clone() + y.clone(), y + x);
    }

    #[test]
    fn multiplication_is_commutative(x in sqrt_two_elem(), y in sqrt_two_elem()) {
        prop_assert_eq!(x.clone() * y.clone(), y * x);
    }

    #[test]
    fn distributivity_holds(x in sqrt_two_elem(), y in sqrt_two_elem(), z in sqrt_two_elem()) {
        prop_assert_eq!(x.clone() * (y.clone() + z.clone()), x.clone() * y + x * z);
    }

    #[test]
    fn dividing_by_self_recovers_one(x in sqrt_two_elem()) {
        prop_assume!(!x.is_zero());
        prop_assert_eq!(x.clone() / x, SqrtTwoField::one());
    }

    #[test]
    fn rational_solve_square_has_zero_residual((matrix, rhs) in invertible_rational_matrix_2x2()) {
        let SolveResult::Unique(solution) = solve_square(&matrix, &rhs) else {
            prop_assert!(false, "expected unique solution");
            unreachable!();
        };

        let residual0 = matrix[0][0].clone() * solution[0].clone()
            + matrix[0][1].clone() * solution[1].clone();
        let residual1 = matrix[1][0].clone() * solution[0].clone()
            + matrix[1][1].clone() * solution[1].clone();

        prop_assert_eq!(residual0, rhs[0].clone());
        prop_assert_eq!(residual1, rhs[1].clone());
    }
}
