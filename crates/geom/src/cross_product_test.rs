use super::*;
use nalgebra::Vector4;

#[test]
fn perpendicular_to_all_inputs() {
    let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
    let c = Vector4::new(9.0, 10.0, 11.0, 13.0); // not coplanar with a,b
    let d = cross_product_4d(a, b, c);
    assert!(d.dot(&a).abs() < 1e-10, "not perpendicular to a: {}", d.dot(&a));
    assert!(d.dot(&b).abs() < 1e-10, "not perpendicular to b: {}", d.dot(&b));
    assert!(d.dot(&c).abs() < 1e-10, "not perpendicular to c: {}", d.dot(&c));
}

#[test]
fn standard_basis_norm() {
    // cross(e1, e2, e3) should be ±e4 with norm 1
    let e1 = Vector4::x();
    let e2 = Vector4::y();
    let e3 = Vector4::z();
    let d = cross_product_4d(e1, e2, e3);
    assert!((d.norm() - 1.0).abs() < 1e-10, "norm should be 1, got {}", d.norm());
    // d should be parallel to e4
    assert!(
        (d[3].abs() - 1.0).abs() < 1e-10,
        "should be ±e4, got {:?}",
        d
    );
}

#[test]
fn zero_for_coplanar_inputs() {
    let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let c = Vector4::new(1.0, 1.0, 0.0, 0.0); // c = a + b, linearly dependent
    let d = cross_product_4d(a, b, c);
    assert!(d.norm() < 1e-10, "should be zero for coplanar inputs, got {:?}", d);
}
