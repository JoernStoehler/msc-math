//! Small workflow smoke tests for the real-algebraic crate.

use real_algebraic::{dot, Algebraic, OrderedField, TanPiFifth};

type TanPiFifthField = Algebraic<TanPiFifth>;

#[test]
fn tan_pi_fifth_smoke() {
    let t = TanPiFifthField::generator();
    let lhs = dot(
        &[TanPiFifthField::one(), t.clone()],
        &[TanPiFifthField::one(), TanPiFifthField::from_i64(2)],
    );
    assert!(lhs.to_f64() > 2.0);
}
