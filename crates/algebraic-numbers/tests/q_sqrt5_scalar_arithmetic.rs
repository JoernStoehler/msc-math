mod common;

use common::{a, ar, q, samples, Qsqrt5};
use num_traits::{One, Zero};

#[test]
fn multiplication_reduces_by_root_squared_equals_five() {
    assert_eq!(Qsqrt5::root() * Qsqrt5::root(), a(5, 0));
}

#[test]
fn converted_rational_arithmetic_is_convenient_without_creating_new_fields() {
    assert_eq!(Qsqrt5::from(2) * Qsqrt5::root(), a(0, 2));
    assert_eq!(Qsqrt5::root() * Qsqrt5::from(q(3)), a(0, 3));
    assert_eq!(Qsqrt5::from(q(3)) * Qsqrt5::root(), a(0, 3));
    assert_eq!(Qsqrt5::root() / Qsqrt5::from(q(5)), ar(0, 1, 5));
    assert_eq!(a(1, 1) / Qsqrt5::root(), ar(1, 1, 5));
}

#[test]
fn zero_one_and_assignment_operators_are_available() {
    let mut value = Qsqrt5::zero();
    value += Qsqrt5::one();
    value *= Qsqrt5::from(5);
    value -= Qsqrt5::from(2);
    value /= Qsqrt5::from(3);

    assert_eq!(value, a(1, 0));
}

#[test]
#[should_panic(expected = "cannot invert zero")]
fn division_by_zero_panics() {
    let _ = Qsqrt5::one() / Qsqrt5::zero();
}

#[test]
#[should_panic]
fn wrong_coefficient_array_length_panics() {
    let _ = Qsqrt5::new([q(0)]);
}

#[test]
fn sampled_values_satisfy_field_laws() {
    let values = samples();
    let zero = Qsqrt5::zero();
    let one = Qsqrt5::one();

    for x in &values {
        assert_eq!(x.clone() + zero.clone(), x.clone());
        assert_eq!(x.clone() - x.clone(), zero);
        assert_eq!(x.clone() * one.clone(), x.clone());
        assert_eq!(x.clone() * zero.clone(), zero);

        if !x.is_zero() {
            assert_eq!(x.clone() / x.clone(), one);
            assert_eq!(x.clone() * (one.clone() / x.clone()), one);
        }
    }

    for x in &values {
        for y in &values {
            assert_eq!(x.clone() + y.clone(), y.clone() + x.clone());
            assert_eq!(x.clone() * y.clone(), y.clone() * x.clone());
            assert_eq!((x.clone() + y.clone()) - y.clone(), x.clone());

            for z in &values {
                assert_eq!(
                    (x.clone() + y.clone()) + z.clone(),
                    x.clone() + (y.clone() + z.clone())
                );
                assert_eq!(
                    (x.clone() * y.clone()) * z.clone(),
                    x.clone() * (y.clone() * z.clone())
                );
                assert_eq!(
                    x.clone() * (y.clone() + z.clone()),
                    x.clone() * y.clone() + x.clone() * z.clone()
                );
            }
        }
    }
}
