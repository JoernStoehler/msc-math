use algebraic_numbers::{Algebraic, RationalInterval, RealAlgebraicField, Sign};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::{One, Zero};

enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-5).into()),
            BigRational::zero(),
            BigRational::one(),
        ]
    }

    fn isolating_interval() -> RationalInterval {
        RationalInterval::new(q(2), q(3))
    }
}

type Qsqrt5 = Algebraic<Sqrt5>;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn a(rational: i64, sqrt5_coeff: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![q(rational), q(sqrt5_coeff)]).unwrap()
}

fn ar(rational: i64, sqrt5_numer: i64, sqrt5_denom: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![
        q(rational),
        BigRational::new(sqrt5_numer.into(), sqrt5_denom.into()),
    ])
    .unwrap()
}

#[test]
fn vector4_addition_over_q_sqrt5_is_plain_nalgebra_syntax() {
    let left = Vector4::new(a(1, 1), a(2, 0), a(0, 3), a(-1, 1));
    let right = Vector4::new(a(4, -1), a(0, 5), a(7, 0), a(1, 2));

    let sum = left + right;

    assert_eq!(sum, Vector4::new(a(5, 0), a(2, 5), a(7, 3), a(0, 3)));
}

#[test]
fn multiplication_reduces_by_alpha_squared_equals_five() {
    let alpha = a(0, 1);

    assert_eq!(alpha.clone() * alpha, a(5, 0));
}

#[test]
fn scalar_arithmetic_is_convenient_without_creating_new_fields() {
    let alpha = Qsqrt5::alpha();

    assert_eq!(2 * alpha.clone(), a(0, 2));
    assert_eq!(alpha.clone() * q(3), a(0, 3));
    assert_eq!(q(3) * alpha.clone(), a(0, 3));
    assert_eq!(alpha.clone() / q(5), ar(0, 1, 5));
    assert_eq!(a(1, 1) / alpha, ar(1, 1, 5));
}

#[test]
fn sign_and_order_are_exact_for_q_sqrt5_examples() {
    let alpha = Qsqrt5::alpha();

    assert_eq!((alpha.clone() - 2).sign(), Sign::Positive);
    assert_eq!((alpha.clone() - 3).sign(), Sign::Negative);
    assert!(alpha.clone() > a(2, 0));
    assert!(alpha < a(3, 0));
}
