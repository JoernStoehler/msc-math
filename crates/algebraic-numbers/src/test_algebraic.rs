use super::*;
use crate::named_fields::TanPiFifth;
use std::cmp::Ordering;

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

struct NonMonicSqrtTwo;

impl StaticFieldSpec for NonMonicSqrtTwo {
    fn name() -> &'static str {
        "Q(sqrt(2)) with non-monic polynomial"
    }

    fn generator_name() -> &'static str {
        "s"
    }

    fn minimal_polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-4).into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(2.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
        ]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (
            BigRational::from_integer(1.into()),
            BigRational::from_integer(2.into()),
        )
    }
}

type TanPiFifthField = Algebraic<TanPiFifth>;
type SqrtTwoField = Algebraic<SqrtTwo>;
type NonMonicSqrtTwoField = Algebraic<NonMonicSqrtTwo>;

#[test]
fn tan_pi_fifth_generator_satisfies_defining_polynomial() {
    let t = TanPiFifthField::generator();
    let poly = TanPiFifth::minimal_polynomial();
    let value = poly
        .iter()
        .enumerate()
        .fold(TanPiFifthField::zero(), |acc, (idx, coeff)| {
            let mut term = TanPiFifthField::from_rational(coeff.clone());
            for _ in 0..idx {
                term = term * t.clone();
            }
            acc + term
        });
    assert!(value.is_zero());
}

#[test]
fn inverse_recovers_one() {
    let t = TanPiFifthField::generator();
    let value = TanPiFifthField::one() + TanPiFifthField::from_frac(1, 2) * t;
    let inv = value.inverse();
    assert_eq!(value * inv, TanPiFifthField::one());
}

#[test]
fn comparison_orders_values_in_the_real_embedding() {
    let t = TanPiFifthField::generator();
    assert!(t > TanPiFifthField::from_frac(1, 2));
    assert!(t < TanPiFifthField::one());
}

#[test]
fn multiplication_reduces_to_the_canonical_basis() {
    let t = TanPiFifthField::generator();
    let t4 = t.clone() * t.clone() * t.clone() * t.clone();
    let expected = TanPiFifthField::from_i64(10) * t.clone() * t.clone() - TanPiFifthField::from_i64(5);
    assert_eq!(t4, expected);
}

#[test]
fn sign_classification_distinguishes_negative_zero_and_positive() {
    let t = TanPiFifthField::generator();
    assert_eq!(TanPiFifthField::zero().sign(), Sign::Zero);
    assert_eq!(t.sign(), Sign::Positive);
    assert_eq!((-t).sign(), Sign::Negative);
}

#[test]
fn equivalent_representations_canonicalize_to_the_same_value() {
    let t = TanPiFifthField::generator();
    let left = t.clone() * t.clone() * t.clone() * t.clone();
    let right = TanPiFifthField::from_i64(10) * t.clone() * t.clone() - TanPiFifthField::from_i64(5);
    assert_eq!(left.coeffs(), right.coeffs());
}

#[test]
fn non_monic_polynomial_with_trailing_zeros_reduces_correctly() {
    let s = NonMonicSqrtTwoField::generator();
    assert_eq!(s.clone() * s.clone(), NonMonicSqrtTwoField::from_i64(2));

    let overlong = NonMonicSqrtTwoField::from_coeffs(vec![
        BigRational::from_integer(0.into()),
        BigRational::from_integer(0.into()),
        BigRational::from_integer(1.into()),
        BigRational::from_integer(0.into()),
        BigRational::from_integer(0.into()),
    ]);
    assert_eq!(overlong, NonMonicSqrtTwoField::from_i64(2));
    assert_eq!(
        overlong.coeffs(),
        &[
            BigRational::from_integer(2.into()),
            BigRational::from_integer(0.into())
        ]
    );
}

#[test]
fn sign_handles_values_very_close_to_the_chosen_root() {
    let s = SqrtTwoField::generator();
    let positive = SqrtTwoField::from_i64(99) - SqrtTwoField::from_i64(70) * s.clone();
    let negative = SqrtTwoField::from_i64(239) - SqrtTwoField::from_i64(169) * s;

    assert_eq!(positive.sign(), Sign::Positive);
    assert_eq!(negative.sign(), Sign::Negative);
    assert_eq!(positive.cmp_real(&SqrtTwoField::zero()), Ordering::Greater);
    assert_eq!(negative.cmp_real(&SqrtTwoField::zero()), Ordering::Less);
}

#[test]
fn sign_handles_large_convergents_without_hitting_a_fixed_iteration_ceiling() {
    let s = SqrtTwoField::generator();
    let (numer, denom) = sqrt_two_convergent_with_denominator_bits(700);
    let approximation = SqrtTwoField::from_rational(BigRational::new(numer, denom));
    let difference = s - approximation;
    assert_eq!(difference.sign(), Sign::Positive);
}

fn sqrt_two_convergent_with_denominator_bits(
    target_bits: usize,
) -> (num_bigint::BigInt, num_bigint::BigInt) {
    let mut p_prev = num_bigint::BigInt::from(1);
    let mut p_curr = num_bigint::BigInt::from(3);
    let mut q_prev = num_bigint::BigInt::from(1);
    let mut q_curr = num_bigint::BigInt::from(2);

    while bigint_height_bits(&q_curr) < target_bits {
        let next_p = &p_curr * 2 + &p_prev;
        let next_q = &q_curr * 2 + &q_prev;
        p_prev = p_curr;
        p_curr = next_p;
        q_prev = q_curr;
        q_curr = next_q;
    }

    (p_curr, q_curr)
}
