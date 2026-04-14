//! Boundedness and normalization stages.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

use crate::geom::polytope::ConstructionError;

pub(super) fn integer_scale_dual_vertices(
    dual_vertices: &[[BigRational; 4]],
) -> (Vec<[BigInt; 4]>, BigInt) {
    let mut d = BigInt::from(1);
    for y in dual_vertices {
        for comp in y {
            d = num_integer::Integer::lcm(&d, comp.denom());
        }
    }

    let int_verts: Vec<[BigInt; 4]> = dual_vertices
        .iter()
        .map(|y| {
            std::array::from_fn(|c| {
                let scale = &d / y[c].denom();
                y[c].numer() * scale
            })
        })
        .collect();

    (int_verts, d)
}

fn cross_product_4d_int(a: &[BigInt; 4], b: &[BigInt; 4], c: &[BigInt; 4]) -> [BigInt; 4] {
    let bc_01 = &b[0] * &c[1] - &b[1] * &c[0];
    let bc_02 = &b[0] * &c[2] - &b[2] * &c[0];
    let bc_03 = &b[0] * &c[3] - &b[3] * &c[0];
    let bc_12 = &b[1] * &c[2] - &b[2] * &c[1];
    let bc_13 = &b[1] * &c[3] - &b[3] * &c[1];
    let bc_23 = &b[2] * &c[3] - &b[3] * &c[2];

    let d0 = &a[1] * &bc_23 - &a[2] * &bc_13 + &a[3] * &bc_12;
    let d1 = -(&a[0] * &bc_23 - &a[2] * &bc_03 + &a[3] * &bc_02);
    let d2 = &a[0] * &bc_13 - &a[1] * &bc_03 + &a[3] * &bc_01;
    let d3 = -(&a[0] * &bc_12 - &a[1] * &bc_02 + &a[2] * &bc_01);
    [d0, d1, d2, d3]
}

fn dot4_int(a: &[BigInt; 4], b: &[BigInt; 4]) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

fn rank_int(rows: &[[BigInt; 4]]) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let m = rows.len();
    let mut mat: Vec<[BigInt; 4]> = rows.to_vec();
    let mut rank = 0;

    for col in 0..4 {
        let Some(pivot_row) = (rank..m).find(|&r| !mat[r][col].is_zero()) else {
            continue;
        };
        mat.swap(rank, pivot_row);
        let pivot_val = mat[rank][col].clone();
        for r in 0..m {
            if r == rank || mat[r][col].is_zero() {
                continue;
            }
            let row_val = mat[r][col].clone();
            let pivot = mat[rank].clone();
            for c in col..4 {
                mat[r][c] = &pivot_val * &mat[r][c] - &row_val * &pivot[c];
            }
        }
        rank += 1;
    }
    rank
}

fn bounded_triple_f64_confirms(
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

#[cfg(test)]
use super::linear_algebra::{cross_product_4d_rational, dot4, rank_over_q};

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
