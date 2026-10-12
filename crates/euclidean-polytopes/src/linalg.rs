use algebraic_numbers::{solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Vector4};
use num_bigint::BigInt;

pub(crate) fn combinations3(n: usize) -> Vec<[usize; 3]> {
    let mut result = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                result.push([i, j, k]);
            }
        }
    }
    result
}

pub(crate) fn combinations4(n: usize) -> Vec<[usize; 4]> {
    let mut result = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    result.push([i, j, k, l]);
                }
            }
        }
    }
    result
}

pub(crate) fn dot4_exact<T: ExactScalar>(left: &Vector4<T>, right: &Vector4<T>) -> T {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

pub(crate) fn cross_product_4d_exact<T: ExactScalar>(
    a: &Vector4<T>,
    b: &Vector4<T>,
    c: &Vector4<T>,
) -> Vector4<T> {
    let bc_01 = b[0].clone() * c[1].clone() - b[1].clone() * c[0].clone();
    let bc_02 = b[0].clone() * c[2].clone() - b[2].clone() * c[0].clone();
    let bc_03 = b[0].clone() * c[3].clone() - b[3].clone() * c[0].clone();
    let bc_12 = b[1].clone() * c[2].clone() - b[2].clone() * c[1].clone();
    let bc_13 = b[1].clone() * c[3].clone() - b[3].clone() * c[1].clone();
    let bc_23 = b[2].clone() * c[3].clone() - b[3].clone() * c[2].clone();

    Vector4::new(
        a[1].clone() * bc_23.clone() - a[2].clone() * bc_13.clone() + a[3].clone() * bc_12.clone(),
        -(a[0].clone() * bc_23.clone() - a[2].clone() * bc_03.clone()
            + a[3].clone() * bc_02.clone()),
        a[0].clone() * bc_13 - a[1].clone() * bc_03.clone() + a[3].clone() * bc_01.clone(),
        -(a[0].clone() * bc_12 - a[1].clone() * bc_02 + a[2].clone() * bc_01),
    )
}

pub(crate) fn is_zero_vector_exact<T: ExactScalar>(vector: &Vector4<T>) -> bool {
    (0..4).all(|coordinate| vector[coordinate].is_zero())
}

pub(crate) fn solve4_exact<T: ExactScalar + 'static>(
    rows: &[Vector4<T>; 4],
    rhs: &Vector4<T>,
) -> Option<Vector4<T>> {
    let matrix = DMatrix::from_fn(4, 4, |row, col| rows[row][col].clone());
    let rhs = DVector::from_fn(4, |row, _| rhs[row].clone());
    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => Some(Vector4::new(
            particular[0].clone(),
            particular[1].clone(),
            particular[2].clone(),
            particular[3].clone(),
        )),
        _ => None,
    }
}

/// Exact determinant of a four-by-four integer matrix, supplied as ordered rows.
///
/// The sign uses ordinary row/column orientation. Singular inputs return zero;
/// no rank, halfspace, boundedness or facet validation is performed. Arithmetic
/// uses `BigInt` throughout. This is the determinant kernel in the rational
/// polar/Cramer pipelines described by [prop:integer-cramer] in
/// `topics/polytope-geometry/exact-polar-geometry.tex`.
pub fn determinant_4d_integer(rows: &[[BigInt; 4]; 4]) -> BigInt {
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
