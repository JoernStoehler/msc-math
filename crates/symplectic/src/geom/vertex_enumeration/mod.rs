//! Exact vertex enumeration for 4D polytopes over Q.
//!
//! Module architecture and math mapping:
//! - `enumerate`: end-to-end construction pipeline and C(F,4) vertex enumeration
//!   ([lem:vertex-enumeration], [prop:integer-cramer], [cor:prefilter-soundness]).
//! - `boundedness`: boundedness validation via positive spanning
//!   ([lem:positive-span], [lem:bounded-triples]).
//! - `irredundancy`: facet irredundancy checks from incident affine rank
//!   ([lem:irredundancy]).
//! - `exact_linalg`: exact determinant/solve/rank primitives used by all modules.

mod boundedness;
mod enumerate;
mod exact_linalg;
mod irredundancy;

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use std::collections::BTreeSet;

/// Tolerance for duplicate-halfspace detection: ||a_i - a_j|| / max(||a_i||, ||a_j||) < threshold.
///
/// **Why 1e-8:** f64 dual vertices from user inputs differ by at least O(1e-3)
/// in practice (physically distinct facet directions). The 1e-8 relative
/// threshold is tight enough to reject identical or nearly-identical halfspaces
/// while staying far above machine epsilon (~1e-16), avoiding spurious false
/// positives from floating-point rounding.
const EPS_DUPLICATE_RELATIVE: f64 = 1e-8;

/// Near-zero f64 norm threshold for dual vertex validation.
///
/// **Why 1e-15:** f64 machine epsilon is ~2.2e-16; any norm below 1e-15
/// indicates a vector whose direction is lost to rounding and cannot
/// represent a meaningful halfspace constraint.
const EPS_ZERO_NORM: f64 = 1e-15;

/// Errors from exact rational/f64 dual-vertex construction when validation fails.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    /// Fewer than 5 facets (the minimum for a bounded 4D polytope).
    TooFewFacets(usize),
    /// Dual vertex at the given index is the zero vector.
    ZeroDualVertex(usize),
    /// Two halfspaces are duplicates (within relative tolerance).
    DuplicateHalfspaces { i: usize, j: usize },
    /// Dual vertices do not positively span R^4 (polytope is unbounded).
    Unbounded,
    /// No vertices found (inconsistent halfspace system).
    NoVertices,
    /// Facet is redundant: no incident vertices, or incident vertices
    /// have affine rank < 3 (don't span the facet hyperplane).
    RedundantFacet(usize),
    /// Exact-to-f64 or f64-to-exact conversion produced invalid data.
    F64Conversion(String),
}

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFacets(n) => write!(f, "need >=5 facets, got {n}"),
            Self::ZeroDualVertex(i) => write!(f, "dual vertex[{i}] is the zero vector"),
            Self::DuplicateHalfspaces { i, j } => {
                write!(f, "halfspaces[{i}] and [{j}] are duplicates")
            }
            Self::Unbounded => {
                write!(
                    f,
                    "polytope is unbounded (dual vertices do not positively span R^4)"
                )
            }
            Self::NoVertices => write!(f, "no vertices found (inconsistent halfspaces)"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
            Self::F64Conversion(msg) => write!(f, "f64 conversion failed: {msg}"),
        }
    }
}

type RationalPipelineOutput = (Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>);

pub(crate) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<RationalPipelineOutput, ConstructionError> {
    enumerate::construct_rational_pipeline(dual_vertices)
}

pub(crate) fn rationalize_f64_dual_vertices(
    dual_vertices_f64: &[Vector4<f64>],
) -> Result<Vec<[BigRational; 4]>, ConstructionError> {
    let f = dual_vertices_f64.len();
    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }

    for (i, a) in dual_vertices_f64.iter().enumerate() {
        for c in 0..4 {
            let value = a[c];
            if !value.is_finite() {
                return Err(ConstructionError::F64Conversion(format!(
                    "dual vertex[{i}][{c}] is non-finite: {value}"
                )));
            }
        }
        if a.norm() < EPS_ZERO_NORM {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }

    for i in 0..f {
        for j in (i + 1)..f {
            let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
            if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm()
                < EPS_DUPLICATE_RELATIVE * max_norm
            {
                return Err(ConstructionError::DuplicateHalfspaces { i, j });
            }
        }
    }

    Ok(dual_vertices_f64
        .iter()
        .map(|a| std::array::from_fn(|c| super::rational_arithmetic::f64_to_rational(a[c])))
        .collect())
}

pub(crate) fn dual_vertices_f64_from_rational(
    dual_vertices: &[[BigRational; 4]],
) -> Result<Vec<Vector4<f64>>, ConstructionError> {
    dual_vertices
        .iter()
        .enumerate()
        .map(|(i, y)| {
            let v = rational_array_to_f64(y);
            if v.norm() < EPS_ZERO_NORM {
                Err(ConstructionError::F64Conversion(format!(
                    "dual vertex[{i}] has near-zero f64 norm: {}",
                    v.norm()
                )))
            } else {
                Ok(v)
            }
        })
        .collect()
}

pub(crate) fn rational_array_to_f64(v: &[BigRational; 4]) -> Vector4<f64> {
    Vector4::new(
        super::rational_arithmetic::rational_to_f64(&v[0]),
        super::rational_arithmetic::rational_to_f64(&v[1]),
        super::rational_arithmetic::rational_to_f64(&v[2]),
        super::rational_arithmetic::rational_to_f64(&v[3]),
    )
}

pub(crate) fn rational_vertices_to_f64(vertices: &[[BigRational; 4]]) -> Vec<Vector4<f64>> {
    vertices.iter().map(rational_array_to_f64).collect()
}

pub(crate) fn vertex_facet_incidence_from_descriptors(
    vertex_descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> DMatrix<bool> {
    DMatrix::from_fn(vertex_descriptors.len(), facet_count, |v, f| {
        vertex_descriptors[v].contains(&f)
    })
}

pub(crate) fn facet_intersection_is_nonempty_from_incidence(
    vertex_facet_incidence: &DMatrix<bool>,
) -> DMatrix<bool> {
    DMatrix::from_fn(
        vertex_facet_incidence.ncols(),
        vertex_facet_incidence.ncols(),
        |i, k| {
            i != k
                && (0..vertex_facet_incidence.nrows())
                    .any(|v| vertex_facet_incidence[(v, i)] && vertex_facet_incidence[(v, k)])
        },
    )
}

pub(crate) fn omega_signs_from_rational_dual_vertices(
    dual_vertices: &[[BigRational; 4]],
) -> DMatrix<i8> {
    DMatrix::from_fn(dual_vertices.len(), dual_vertices.len(), |i, k| {
        if i == k {
            return 0i8;
        }
        let omega =
            super::rational_arithmetic::omega0_rational(&dual_vertices[i], &dual_vertices[k]);
        match super::rational_arithmetic::Sign::of(&omega) {
            super::rational_arithmetic::Sign::Plus => 1,
            super::rational_arithmetic::Sign::Minus => -1,
            super::rational_arithmetic::Sign::Zero => 0,
        }
    })
}

#[cfg(test)]
mod tests;
