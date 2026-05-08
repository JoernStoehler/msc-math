mod common;

use common::{a, ar, q, Qsqrt5};
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

    assert_eq!(value, a(5, 0));
}
