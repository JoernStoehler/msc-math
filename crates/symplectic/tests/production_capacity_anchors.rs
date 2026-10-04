//! Independent geometric anchors for the public production capacity dispatcher.
//!
//! These expected values come from planar rectangle areas and capacity axioms,
//! not from a second call to a KKT or candidate-enumeration implementation.
//! Coordinates are (q1, q2, p1, p2), and every input entry is exactly dyadic.

use nalgebra::Vector4;
use num_rational::BigRational;
use symplectic::capacity_4d::{capacity_from_dual_vertices, Capacity4d};

fn box_duals(half_widths: [f64; 4]) -> Vec<Vector4<f64>> {
    (0..4)
        .flat_map(|axis| {
            [-1.0, 1.0].map(move |sign| {
                let mut dual = Vector4::zeros();
                dual[axis] = sign / half_widths[axis];
                dual
            })
        })
        .collect()
}

fn assert_sharp_anchor(duals: &[Vector4<f64>], expected: f64) -> Capacity4d {
    let result = capacity_from_dual_vertices(duals).expect("valid anchor body");
    let bounds = result.bounds();
    assert!(
        bounds.lower() <= expected && expected <= bounds.upper(),
        "expected {expected}, got [{}, {}] via {}",
        bounds.lower(),
        bounds.upper(),
        result.route_name()
    );
    // A broad interval containing the anchor is not enough to pass this check.
    assert!(bounds.upper() - bounds.lower() <= expected * 1e-10);
    result
}

#[test]
fn axis_aligned_boxes_match_independent_planar_area_values() {
    // In the symplectic splitting (q1,p1) x (q2,p2), the capacity is the
    // smaller rectangle area. A planar symplectic rescaling gives the lower
    // bound from an inscribed cube; the smaller projection cylinder gives
    // the matching upper bound. In particular, c([-1,1]^4) = 4.
    for (half_widths, expected) in [([1.0; 4], 4_i64), ([1.0, 2.0, 4.0, 1.0], 8)] {
        let result = assert_sharp_anchor(&box_duals(half_widths), expected as f64);
        let Capacity4d::Product(product) = result else {
            panic!("axis-aligned box must take the structural product route");
        };
        assert_eq!(
            product.capacity_exact(),
            &BigRational::from_integer(expected.into())
        );
    }
}

#[test]
fn conformal_scaling_changes_capacity_quadratically() {
    let unit_cube = box_duals([1.0; 4]);
    for scale in [0.5, 2.0, 4.0] {
        // Scaling the primal body by scale divides every dual row by scale.
        let scaled = unit_cube
            .iter()
            .map(|dual| dual / scale)
            .collect::<Vec<_>>();
        assert_sharp_anchor(&scaled, 4.0 * scale * scale);
    }
}

#[test]
fn dyadic_input_can_have_a_non_dyadic_exact_capacity() {
    // The input rows are the integers +/-3 e_i, so the represented body is
    // [-1/3,1/3]^4. Conformality gives 4/9, whose denominator is not a power
    // of two. Exact dyadic input does not imply dyadic output coordinates
    // or a dyadic capacity.
    let duals = box_duals([1.0; 4])
        .iter()
        .map(|dual| dual * 3.0)
        .collect::<Vec<_>>();
    let Capacity4d::Product(product) = assert_sharp_anchor(&duals, 4.0 / 9.0) else {
        panic!("uniformly scaled cube must use the product route");
    };
    assert_eq!(
        product.capacity_exact(),
        &BigRational::new(4.into(), 9.into())
    );
}

#[test]
fn symplectic_shears_preserve_anchors_across_dispatch_routes() {
    for (half_widths, expected) in [([1.0; 4], 4.0), ([1.0, 2.0, 4.0, 1.0], 8.0)] {
        let original = box_duals(half_widths);
        for shear in [-2.0, -1.0, 1.0, 2.0] {
            // S(q,p)=(q+Bp,p) is symplectic for the symmetric matrix
            // B=[[s,s],[s,0]]. Dual rows transform by S^{-T}, hence
            // (aq,ap) -> (aq,ap-B aq). These integer/dyadic operations
            // introduce no input rounding, and destroy structural q/p form.
            let transformed = original
                .iter()
                .map(|a| {
                    Vector4::new(
                        a[0],
                        a[1],
                        a[2] - shear * (a[0] + a[1]),
                        a[3] - shear * a[0],
                    )
                })
                .collect::<Vec<_>>();
            let result = assert_sharp_anchor(&transformed, expected);
            assert!(matches!(result, Capacity4d::General(_)));
        }
    }
}
