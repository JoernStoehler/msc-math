use super::*;
use nalgebra::Matrix2;

#[test]
fn j2_squared_is_minus_identity() {
    let j = j2();
    let j_sq = j * j;
    assert_eq!(j_sq, -Matrix2::identity());
}
