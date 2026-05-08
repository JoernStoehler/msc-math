mod common;

use common::{a, ar, q, Qsqrt5};

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
