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

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property test: cross product perpendicularity
        ///
        /// For any three vectors a, b, c in ℝ⁴, the cross product d = a × b × c
        /// must be perpendicular to all three inputs: d·a = d·b = d·c = 0.
        #[test]
        fn cross_product_perpendicular_to_inputs(
            a in prop::array::uniform4(-10.0f64..10.0f64),
            b in prop::array::uniform4(-10.0f64..10.0f64),
            c in prop::array::uniform4(-10.0f64..10.0f64)
        ) {
            let a_vec = Vector4::from(a);
            let b_vec = Vector4::from(b);
            let c_vec = Vector4::from(c);

            let d = cross_product_4d(a_vec, b_vec, c_vec);

            // If inputs are coplanar, cross product is zero - skip those cases
            if d.norm() > 1e-10 {
                let dot_a = d.dot(&a_vec).abs();
                let dot_b = d.dot(&b_vec).abs();
                let dot_c = d.dot(&c_vec).abs();

                // Perpendicularity tolerance scaled by vector magnitudes
                let tol = 1e-10 * d.norm() * a_vec.norm().max(b_vec.norm()).max(c_vec.norm());

                prop_assert!(
                    dot_a < tol,
                    "not perpendicular to a: d·a={}, d={:?}, a={:?}",
                    dot_a,
                    d,
                    a_vec
                );
                prop_assert!(
                    dot_b < tol,
                    "not perpendicular to b: d·b={}, d={:?}, b={:?}",
                    dot_b,
                    d,
                    b_vec
                );
                prop_assert!(
                    dot_c < tol,
                    "not perpendicular to c: d·c={}, d={:?}, c={:?}",
                    dot_c,
                    d,
                    c_vec
                );
            }
        }
    }
}
