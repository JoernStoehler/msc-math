//! Vertex enumeration and construction pipeline.
//!
//! Mathematical correspondence: [lem:vertex-enumeration], [prop:integer-cramer],
//! [cor:prefilter-soundness].

use std::collections::BTreeSet;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

use super::boundedness::check_bounded_f64_first;
use super::exact_linalg::{det4_int, dot4_int, integer_scale_dual_vertices};
use super::irredundancy::check_irredundancy_f64_first;
use super::ConstructionError;

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

#[allow(clippy::type_complexity)]
fn enumerate_vertices_int(
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

        let m_s: [&[BigInt; 4]; 4] = [
            &int_dual_vertices[subset[0]],
            &int_dual_vertices[subset[1]],
            &int_dual_vertices[subset[2]],
            &int_dual_vertices[subset[3]],
        ];
        let m_s_owned: [[BigInt; 4]; 4] = [
            m_s[0].clone(),
            m_s[1].clone(),
            m_s[2].clone(),
            m_s[3].clone(),
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

#[allow(clippy::type_complexity)]
pub(crate) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>), ConstructionError> {
    let f = dual_vertices.len();

    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }
    for (i, y) in dual_vertices.iter().enumerate() {
        if y.iter().all(|c| c.is_zero()) {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }

    let (int_dual_vertices, common_denom) = integer_scale_dual_vertices(dual_vertices);

    check_bounded_f64_first(dual_vertices, &int_dual_vertices)?;

    let (vertex_descriptors, vertices) =
        enumerate_vertices_int(dual_vertices, &int_dual_vertices, &common_denom)?;

    check_irredundancy_f64_first(&vertices, &vertex_descriptors, f)?;

    Ok((vertices, vertex_descriptors))
}

fn f64_prefilter_rejects(dv_f64: &[[f64; 4]], subset: &[usize; 4], f: usize) -> bool {
    use nalgebra::{Matrix4, Vector4};

    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const C: f64 = 1e4;

    let a = Matrix4::new(
        dv_f64[subset[0]][0],
        dv_f64[subset[0]][1],
        dv_f64[subset[0]][2],
        dv_f64[subset[0]][3],
        dv_f64[subset[1]][0],
        dv_f64[subset[1]][1],
        dv_f64[subset[1]][2],
        dv_f64[subset[1]][3],
        dv_f64[subset[2]][0],
        dv_f64[subset[2]][1],
        dv_f64[subset[2]][2],
        dv_f64[subset[2]][3],
        dv_f64[subset[3]][0],
        dv_f64[subset[3]][1],
        dv_f64[subset[3]][2],
        dv_f64[subset[3]][3],
    );

    let svd = a.svd(true, true);
    let svals = &svd.singular_values;

    let sigma_min = svals[0].min(svals[1]).min(svals[2]).min(svals[3]);
    let sigma_max = svals[0].max(svals[1]).max(svals[2]).max(svals[3]);

    if sigma_min == 0.0 {
        return false;
    }

    let kappa_hat = sigma_max / sigma_min;
    if EPS_MACH * kappa_hat > 0.25 {
        return false;
    }

    let ones = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let v_hat = match svd.solve(&ones, 0.0) {
        Ok(v) => v,
        Err(_) => return false,
    };

    if v_hat.iter().any(|&x| !x.is_finite()) {
        return false;
    }

    let v_norm = v_hat.norm();

    for (i, y_i) in dv_f64[..f].iter().enumerate() {
        if subset.contains(&i) {
            continue;
        }

        let s_hat = y_i[0] * v_hat[0] + y_i[1] * v_hat[1] + y_i[2] * v_hat[2] + y_i[3] * v_hat[3];

        let y_norm = (y_i[0] * y_i[0] + y_i[1] * y_i[1] + y_i[2] * y_i[2] + y_i[3] * y_i[3]).sqrt();

        let delta = C * kappa_hat * EPS_MACH * v_norm * y_norm;

        if !s_hat.is_finite() || !delta.is_finite() {
            return false;
        }

        if s_hat > 1.0 + delta {
            return true;
        }
    }

    false
}
