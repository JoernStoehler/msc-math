//! Full-dimensional `R^4` volume for normalized polar pairs.
//!
//! This module keeps the operation flat: callers pass dual facet normals and
//! primal vertices directly. The duals determine vertex-facet incidence, and
//! the primal vertices determine Euclidean determinant geometry.

use nalgebra::{Matrix4, Vector4};

use crate::f64_geometry::{signed_gap_abs_error_bound, validate_finite_vectors4, F64GeometryError};
use crate::polar::IncidenceF64;

/// Diagnostic result for approximate full-dimensional `R^4` volume.
///
/// `Decided` currently reports only the computed `f64` volume. It deliberately
/// does not expose a `volume_abs_error_bound`, because this first slice only
/// bounds the incidence decisions and does not yet carry a rigorous rounding
/// analysis through the determinant sum.
#[derive(Clone, Debug, PartialEq)]
pub enum VolumeF64 {
    Decided {
        volume: f64,
    },
    Indeterminate {
        indeterminate_incidence: Vec<IncidenceF64>,
    },
}

/// Compute full-dimensional Euclidean volume from normalized dual and primal vertices.
///
/// Contract: `dual_vertices` are the normalized facet normals of
/// `K = { x in R^4 : <a_i, x> <= 1 }`, and `vertices` are exactly the primal
/// vertices of the same bounded full-dimensional polytope. The origin is then
/// strictly inside `K`.
///
/// This function validates finite coordinates. It recovers incidence from
/// `signed_gap = 1 - <a_i, v>` without tolerance guessing:
///
/// - `signed_gap == 0.0` is accepted as an exact `f64` active relation;
/// - `signed_gap > signed_gap_abs_error_bound` is accepted as non-incidence;
/// - `signed_gap < -signed_gap_abs_error_bound` panics as a caller contract
///   violation, because a provided vertex lies outside a provided halfspace;
/// - every remaining relation is returned in `VolumeF64::Indeterminate`.
///
/// Once incidence is decided, the implementation triangulates each 3-facet
/// from its vertex centroid, triangulates each ridge polygon by angle ordering
/// in its affine plane, and cones each tetrahedron to the origin.
pub fn volume_f64(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
) -> Result<VolumeF64, F64GeometryError> {
    validate_finite_vectors4("dual_vertices", dual_vertices)?;
    validate_finite_vectors4("vertices", vertices)?;

    assert!(
        dual_vertices.len() >= 5,
        "volume_f64 requires at least five facets for a full-dimensional bounded R^4 polytope"
    );
    assert!(
        vertices.len() >= 5,
        "volume_f64 requires at least five vertices for a full-dimensional R^4 polytope"
    );

    let incidence = decide_incidence_f64(dual_vertices, vertices);
    if !incidence.indeterminate.is_empty() {
        return Ok(VolumeF64::Indeterminate {
            indeterminate_incidence: incidence.indeterminate,
        });
    }

    let volume = origin_star_volume(vertices, &incidence.facet_vertices);
    Ok(VolumeF64::Decided { volume })
}

#[derive(Clone, Debug)]
struct DecidedIncidence {
    facet_vertices: Vec<Vec<usize>>,
    indeterminate: Vec<IncidenceF64>,
}

fn decide_incidence_f64(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
) -> DecidedIncidence {
    let mut facet_vertices = vec![Vec::new(); dual_vertices.len()];
    let mut indeterminate = Vec::new();

    for (vertex_index, vertex) in vertices.iter().enumerate() {
        for (facet_index, facet) in dual_vertices.iter().enumerate() {
            let signed_gap = 1.0 - facet.dot(vertex);
            let signed_gap_abs_error_bound = signed_gap_abs_error_bound(facet, vertex);
            assert!(
                signed_gap.is_finite() && signed_gap_abs_error_bound.is_finite(),
                "volume_f64 incidence arithmetic produced a non-finite diagnostic"
            );

            if signed_gap == 0.0 {
                facet_vertices[facet_index].push(vertex_index);
            } else if signed_gap > signed_gap_abs_error_bound {
                continue;
            } else if signed_gap < -signed_gap_abs_error_bound {
                panic!(
                    "volume_f64 contract violation: vertex {vertex_index} lies outside facet \
                     {facet_index} by signed_gap {signed_gap} with local abs error bound \
                     {signed_gap_abs_error_bound}"
                );
            } else {
                indeterminate.push(IncidenceF64 {
                    vertex_index,
                    facet_index,
                    signed_gap,
                    signed_gap_abs_error_bound,
                });
            }
        }
    }

    DecidedIncidence {
        facet_vertices,
        indeterminate,
    }
}

fn origin_star_volume(vertices: &[Vector4<f64>], facet_vertices: &[Vec<usize>]) -> f64 {
    for (facet_index, indices) in facet_vertices.iter().enumerate() {
        assert!(
            indices.len() >= 4,
            "volume_f64 full-dimensional facet {facet_index} has fewer than four vertices"
        );
    }

    let facet_centroids: Vec<Vector4<f64>> = facet_vertices
        .iter()
        .map(|indices| mean_vertex(vertices, indices))
        .collect();

    let mut total = 0.0;
    for facet_index in 0..facet_vertices.len() {
        for neighbor_index in 0..facet_vertices.len() {
            if facet_index == neighbor_index {
                continue;
            }

            let ridge = intersect_sorted(
                &facet_vertices[facet_index],
                &facet_vertices[neighbor_index],
            );
            if ridge.len() < 3 {
                continue;
            }

            let ordered = order_polygon_vertex_indices(vertices, &ridge);
            for k in 1..ordered.len() - 1 {
                total += simplex_volume_5(
                    Vector4::zeros(),
                    facet_centroids[facet_index],
                    vertices[ordered[0]],
                    vertices[ordered[k]],
                    vertices[ordered[k + 1]],
                );
            }
        }
    }

    total
}

fn simplex_volume_5(
    v0: Vector4<f64>,
    v1: Vector4<f64>,
    v2: Vector4<f64>,
    v3: Vector4<f64>,
    v4: Vector4<f64>,
) -> f64 {
    Matrix4::from_columns(&[v1 - v0, v2 - v0, v3 - v0, v4 - v0])
        .determinant()
        .abs()
        / 24.0
}

fn mean_vertex(vertices: &[Vector4<f64>], indices: &[usize]) -> Vector4<f64> {
    indices
        .iter()
        .map(|&idx| vertices[idx])
        .sum::<Vector4<f64>>()
        / indices.len() as f64
}

fn intersect_sorted(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < lhs.len() && j < rhs.len() {
        match lhs[i].cmp(&rhs[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(lhs[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}

fn order_polygon_vertex_indices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() <= 2 {
        return indices.to_vec();
    }

    let ridge_vertices: Vec<Vector4<f64>> = indices.iter().map(|&idx| all_vertices[idx]).collect();
    let Some(order) = sort_polygon_order(&ridge_vertices) else {
        panic!("volume_f64 could not order a nondegenerate ridge polygon");
    };
    order
        .into_iter()
        .map(|position| indices[position])
        .collect()
}

fn sort_polygon_order(vertices: &[Vector4<f64>]) -> Option<Vec<usize>> {
    const EPS_BASIS_DEGENERATE: f64 = 1e-12;
    const EPS_COLLINEAR: f64 = 1e-10;

    if vertices.len() < 3 {
        return Some((0..vertices.len()).collect());
    }

    let centroid = vertices.iter().copied().sum::<Vector4<f64>>() / vertices.len() as f64;
    let d1_raw = vertices[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < EPS_BASIS_DEGENERATE {
        return None;
    }
    let d1 = d1_raw / d1_norm;

    let d2 = vertices.iter().skip(1).find_map(|vertex| {
        let relative = *vertex - centroid;
        let projection = relative - d1 * relative.dot(&d1);
        (projection.norm() > EPS_COLLINEAR).then(|| projection.normalize())
    })?;

    let mut indexed: Vec<(f64, usize)> = vertices
        .iter()
        .enumerate()
        .map(|(index, vertex)| {
            let relative = *vertex - centroid;
            let angle = relative.dot(&d2).atan2(relative.dot(&d1));
            (angle, index)
        })
        .collect();

    indexed.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap());
    Some(indexed.into_iter().map(|(_, index)| index).collect())
}
