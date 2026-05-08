mod common;

use common::a;
use nalgebra::Vector4;

#[test]
fn vector4_addition_over_q_sqrt5_is_plain_nalgebra_syntax() {
    let left = Vector4::new(a(1, 1), a(2, 0), a(0, 3), a(-1, 1));
    let right = Vector4::new(a(4, -1), a(0, 5), a(7, 0), a(1, 2));

    let sum = left + right;

    assert_eq!(sum, Vector4::new(a(5, 0), a(2, 5), a(7, 3), a(0, 3)));
}
