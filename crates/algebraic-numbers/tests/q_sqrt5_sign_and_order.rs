mod common;

use algebraic_numbers::Sign;
use common::{a, Qsqrt5};

#[test]
fn sign_and_order_are_exact_for_q_sqrt5_examples() {
    assert_eq!((Qsqrt5::root() - Qsqrt5::from(2)).sign(), Sign::Positive);
    assert_eq!((Qsqrt5::root() - Qsqrt5::from(3)).sign(), Sign::Negative);
    assert!(Qsqrt5::root() > a(2, 0));
    assert!(Qsqrt5::root() < a(3, 0));
}
