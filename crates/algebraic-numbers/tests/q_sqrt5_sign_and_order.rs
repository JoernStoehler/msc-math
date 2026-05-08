mod common;

use algebraic_numbers::Sign;
use common::{a, Qsqrt5};

#[test]
fn sign_and_order_are_exact_for_q_sqrt5_examples() {
    let alpha = Qsqrt5::alpha();

    assert_eq!((alpha.clone() - 2).sign(), Sign::Positive);
    assert_eq!((alpha.clone() - 3).sign(), Sign::Negative);
    assert!(alpha.clone() > a(2, 0));
    assert!(alpha < a(3, 0));
}
