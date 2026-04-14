//! Exact candidate enumeration stage.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use std::collections::BTreeSet;

use super::prefilter::f64_prefilter_rejects;
use crate::geom::polytope::ConstructionError;

fn combinations4(n: usize) -> Vec<[usize; 4]> {
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

fn dot4_int(a: &[BigInt; 4], b: &[BigInt; 4]) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

fn det4_int(rows: &[[BigInt; 4]; 4]) -> BigInt {
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

#[allow(clippy::type_complexity)]
pub(super) fn enumerate_vertices_int(
    dual_vertices: &[[BigRational; 4]],
    int_dual_vertices: &[[BigInt; 4]],
    common_denom: &BigInt,
) -> Result<(Vec<BTreeSet<usize>>, Vec<[BigRational; 4]>), ConstructionError> {
    use crate::geom::rational_arithmetic::rational_to_f64;

    let f = dual_vertices.len();
    let one_int = [
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(1),
    ];
    let dv_f64: Vec<[f64; 4]> = dual_vertices
        .iter()
        .map(|y| std::array::from_fn(|c| rational_to_f64(&y[c])))
        .collect();

    let mut vertex_descriptors = Vec::new();
    let mut vertices = Vec::new();

    for subset in combinations4(f) {
        if f64_prefilter_rejects(&dv_f64, &subset, f) {
            continue;
        }

        let m_s_owned: [[BigInt; 4]; 4] = [
            int_dual_vertices[subset[0]].clone(),
            int_dual_vertices[subset[1]].clone(),
            int_dual_vertices[subset[2]].clone(),
            int_dual_vertices[subset[3]].clone(),
        ];

        let delta = det4_int(&m_s_owned);
        if delta.is_zero() {
            continue;
        }
        let delta_positive = delta.is_positive();

        let mut nu = [
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(0),
        ];
        for j in 0..4 {
            let mut modified = m_s_owned.clone();
            for row in 0..4 {
                modified[row][j] = one_int[row].clone();
            }
            nu[j] = det4_int(&modified);
        }

        let mut all_ok = true;
        let mut incident_facets = BTreeSet::from(subset);
        for (i, a_i) in int_dual_vertices.iter().enumerate() {
            if subset.contains(&i) {
                continue;
            }
            let gap_numer = &delta - dot4_int(a_i, &nu);
            if gap_numer.is_zero() {
                incident_facets.insert(i);
            } else if gap_numer.is_positive() != delta_positive {
                all_ok = false;
                break;
            }
        }
        if !all_ok {
            continue;
        }

        let v: [BigRational; 4] =
            std::array::from_fn(|j| BigRational::new(common_denom * &nu[j], delta.clone()));
        let already_found = vertices
            .iter()
            .any(|existing: &[BigRational; 4]| (0..4).all(|i| existing[i] == v[i]));
        if already_found {
            continue;
        }
        vertex_descriptors.push(incident_facets);
        vertices.push(v);
    }

    if vertex_descriptors.is_empty() {
        return Err(ConstructionError::NoVertices);
    }
    Ok((vertex_descriptors, vertices))
}
