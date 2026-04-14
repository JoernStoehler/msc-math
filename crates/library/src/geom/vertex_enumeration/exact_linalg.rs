//! Exact linear algebra helpers for vertex enumeration.
//!
//! This module contains deterministic arithmetic primitives shared by boundedness,
//! enumeration, and irredundancy checks.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

/// Compute the rank of rational 4-vectors via exact Gaussian elimination.
///
/// Mathematical correspondence: helper for [lem:vertex-enumeration].
pub(super) fn rank_over_q(rows: &[[BigRational; 4]]) -> usize {
    if rows.is_empty() {
        return 0;
    }

    let m = rows.len();
    let n = 4;
    let mut mat: Vec<[BigRational; 4]> = rows.to_vec();

    let mut rank = 0;
    for col in 0..n {
        let pivot_row = (rank..m).find(|&r| !mat[r][col].is_zero());
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        mat.swap(rank, pivot_row);

        let pivot_val = mat[rank][col].clone();
        for r in 0..m {
            if r == rank || mat[r][col].is_zero() {
                continue;
            }
            let factor = &mat[r][col] / &pivot_val;
            let pivot_row_data: [BigRational; 4] = mat[rank].clone();
            for (mat_c, pivot_c) in mat[r][col..n].iter_mut().zip(pivot_row_data[col..n].iter()) {
                *mat_c = &*mat_c - &factor * pivot_c;
            }
        }
        rank += 1;
    }
    rank
}

/// Rank of integer 4-vectors via fraction-free elimination.
///
/// Mathematical correspondence: rank check in [prop:integer-cramer].
pub(super) fn rank_int(rows: &[[BigInt; 4]]) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let m = rows.len();
    let n = 4;
    let mut mat: Vec<[BigInt; 4]> = rows.to_vec();
    let mut rank = 0;

    for col in 0..n {
        let pivot_row = (rank..m).find(|&r| !mat[r][col].is_zero());
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        mat.swap(rank, pivot_row);

        let pivot_val = mat[rank][col].clone();
        for r in 0..m {
            if r == rank || mat[r][col].is_zero() {
                continue;
            }
            let row_val = mat[r][col].clone();
            let pivot_row_data: [BigInt; 4] = mat[rank].clone();
            for c in col..n {
                mat[r][c] = &pivot_val * &mat[r][c] - &row_val * &pivot_row_data[c];
            }
        }
        rank += 1;
    }
    rank
}

/// Determinant of a 4x4 integer matrix via cofactor expansion.
///
/// Mathematical correspondence: δ = det(M_S) in [prop:integer-cramer].
pub(crate) fn det4_int(rows: &[[BigInt; 4]; 4]) -> BigInt {
    let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

    let m01 = &b[0] * &c[1] - &b[1] * &c[0];
    let m02 = &b[0] * &c[2] - &b[2] * &c[0];
    let m03 = &b[0] * &c[3] - &b[3] * &c[0];
    let m12 = &b[1] * &c[2] - &b[2] * &c[1];
    let m13 = &b[1] * &c[3] - &b[3] * &c[1];
    let m23 = &b[2] * &c[3] - &b[3] * &c[2];

    let c00 = &d[1] * &m23 - &d[2] * &m13 + &d[3] * &m12;
    let c01 = &d[0] * &m23 - &d[2] * &m03 + &d[3] * &m02;
    let c02 = &d[0] * &m13 - &d[1] * &m03 + &d[3] * &m01;
    let c03 = &d[0] * &m12 - &d[1] * &m02 + &d[2] * &m01;

    &a[0] * c00 - &a[1] * c01 + &a[2] * c02 - &a[3] * c03
}

/// Dot product of two 4-vectors over Z.
pub(crate) fn dot4_int(a: &[BigInt; 4], b: &[BigInt; 4]) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

/// 4D cross product over Z: direction perpendicular to three integer vectors.
pub(super) fn cross_product_4d_int(
    a: &[BigInt; 4],
    b: &[BigInt; 4],
    c: &[BigInt; 4],
) -> [BigInt; 4] {
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

/// Scale rational dual vertices to integer arrays with a common denominator.
///
/// Mathematical correspondence: preprocessing for [prop:integer-cramer].
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

#[cfg(test)]
pub(super) mod test_support {
    use num_rational::BigRational;
    use num_traits::Zero;

    /// Determinant of a 3x3 rational matrix (Sarrus' rule).
    pub(crate) fn det3(r0: &[BigRational], r1: &[BigRational], r2: &[BigRational]) -> BigRational {
        &r0[0] * (&r1[1] * &r2[2] - &r1[2] * &r2[1]) - &r0[1] * (&r1[0] * &r2[2] - &r1[2] * &r2[0])
            + &r0[2] * (&r1[0] * &r2[1] - &r1[1] * &r2[0])
    }

    /// Exact determinant of a 4x4 rational matrix via cofactor expansion.
    pub(crate) fn det4(rows: &[[BigRational; 4]; 4]) -> BigRational {
        let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

        let c00 = det3(
            &[b[1].clone(), b[2].clone(), b[3].clone()],
            &[c[1].clone(), c[2].clone(), c[3].clone()],
            &[d[1].clone(), d[2].clone(), d[3].clone()],
        );
        let c01 = det3(
            &[b[0].clone(), b[2].clone(), b[3].clone()],
            &[c[0].clone(), c[2].clone(), c[3].clone()],
            &[d[0].clone(), d[2].clone(), d[3].clone()],
        );
        let c02 = det3(
            &[b[0].clone(), b[1].clone(), b[3].clone()],
            &[c[0].clone(), c[1].clone(), c[3].clone()],
            &[d[0].clone(), d[1].clone(), d[3].clone()],
        );
        let c03 = det3(
            &[b[0].clone(), b[1].clone(), b[2].clone()],
            &[c[0].clone(), c[1].clone(), c[2].clone()],
            &[d[0].clone(), d[1].clone(), d[2].clone()],
        );

        &a[0] * c00 - &a[1] * c01 + &a[2] * c02 - &a[3] * c03
    }

    /// Solve a 4x4 linear system N*x = b exactly via Cramer's rule.
    pub(crate) fn solve4(
        rows: &[[BigRational; 4]; 4],
        rhs: &[BigRational; 4],
    ) -> Option<[BigRational; 4]> {
        let d = det4(rows);
        if d.is_zero() {
            return None;
        }

        let mut result: [BigRational; 4] = std::array::from_fn(|_| BigRational::zero());
        for col in 0..4 {
            let mut modified = rows.clone();
            for row in 0..4 {
                modified[row][col] = rhs[row].clone();
            }
            result[col] = det4(&modified) / &d;
        }

        Some(result)
    }

    /// Inner product of two 4-vectors over Q.
    pub(crate) fn dot4(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
        &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
    }

    /// 4D cross product over Q.
    pub(crate) fn cross_product_4d_rational(
        a: &[BigRational; 4],
        b: &[BigRational; 4],
        c: &[BigRational; 4],
    ) -> [BigRational; 4] {
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
}
