//! Boundedness checks for dual vertices.
//!
//! Mathematical correspondence: [lem:positive-span], [lem:bounded-triples].

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

#[cfg(test)]
use super::exact_linalg::rank_over_q;
#[cfg(test)]
use super::exact_linalg::test_support::{cross_product_4d_rational, dot4};
use super::exact_linalg::{cross_product_4d_int, dot4_int, rank_int};
use super::ConstructionError;

/// f64 pre-filter for a single triple in the bounded check.
pub(super) fn bounded_triple_f64_confirms(
    dv_f64: &[nalgebra::Vector4<f64>],
    i: usize,
    j: usize,
    k: usize,
) -> bool {
    use crate::geom::cross_product_4d::cross_product_4d;

    const EPS_DEP: f64 = 1e-12;
    const EPS_SIGN: f64 = 1e-9;

    let d = cross_product_4d(dv_f64[i], dv_f64[j], dv_f64[k]);
    if d.norm() < EPS_DEP {
        return false;
    }
    let d = d.normalize();

    let mut has_pos = false;
    let mut has_neg = false;
    for (l, dv) in dv_f64.iter().enumerate() {
        if l == i || l == j || l == k {
            continue;
        }
        let s = dv.dot(&d);
        if s > EPS_SIGN {
            has_pos = true;
        } else if s < -EPS_SIGN {
            has_neg = true;
        } else {
            return false;
        }
        if has_pos && has_neg {
            return true;
        }
    }
    false
}

/// Check that dual vertices positively span R^4 using f64-first with exact fallback.
pub(super) fn check_bounded_f64_first(
    dual_vertices: &[[BigRational; 4]],
    int_dual_vertices: &[[BigInt; 4]],
) -> Result<(), ConstructionError> {
    use crate::geom::rational_arithmetic::rational_to_f64;

    let f = dual_vertices.len();
    let dv_f64: Vec<nalgebra::Vector4<f64>> = dual_vertices
        .iter()
        .map(|y| {
            nalgebra::Vector4::new(
                rational_to_f64(&y[0]),
                rational_to_f64(&y[1]),
                rational_to_f64(&y[2]),
                rational_to_f64(&y[3]),
            )
        })
        .collect();

    if rank_int(int_dual_vertices) < 4 {
        return Err(ConstructionError::Unbounded);
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                if bounded_triple_f64_confirms(&dv_f64, i, j, k) {
                    continue;
                }
                let d_int = cross_product_4d_int(
                    &int_dual_vertices[i],
                    &int_dual_vertices[j],
                    &int_dual_vertices[k],
                );
                if d_int.iter().all(|c| c.is_zero()) {
                    continue;
                }
                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d_int).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d_int).is_negative());
                if !has_pos || !has_neg {
                    return Err(ConstructionError::Unbounded);
                }
            }
        }
    }

    Ok(())
}

/// Exact boundedness check retained for tests.
#[cfg(test)]
pub(super) fn check_bounded_rational(dual_vertices: &[[BigRational; 4]]) -> bool {
    let f = dual_vertices.len();

    if rank_over_q(dual_vertices) < 4 {
        return false;
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let d = cross_product_4d_rational(
                    &dual_vertices[i],
                    &dual_vertices[j],
                    &dual_vertices[k],
                );
                if d.iter().all(|c| c.is_zero()) {
                    continue;
                }

                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&dual_vertices[l], &d).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&dual_vertices[l], &d).is_negative());

                if !has_pos || !has_neg {
                    return false;
                }
            }
        }
    }
    true
}
