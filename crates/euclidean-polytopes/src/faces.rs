//! Incidence-only face combinatorics for 4D convex polytopes.
//!
//! These helpers use only a vertex-facet incidence matrix. They do not inspect
//! coordinates and do not order 2-face vertices cyclically.

use nalgebra::DMatrix;

/// A 2-face described by its two containing facets and incident vertices.
///
/// The facet indices are sorted as `facets[0] < facets[1]`. The vertex list is
/// deterministic and sorted by increasing vertex index, not by polygon order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoFace {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

/// Return sorted incident-facet lists for every vertex in an incidence matrix.
///
/// `incidence[(v, f)]` is interpreted as vertex `v` lying on facet `f`.
pub fn vertex_facets_from_incidence(incidence: &DMatrix<bool>) -> Vec<Vec<usize>> {
    (0..incidence.nrows())
        .map(|vertex_index| {
            (0..incidence.ncols())
                .filter(|&facet_index| incidence[(vertex_index, facet_index)])
                .collect()
        })
        .collect()
}

/// Return 4D edges inferred from vertex-facet incidence.
///
/// A vertex pair is an edge when the two vertices share at least three incident
/// facets. Returned pairs satisfy `edge[0] < edge[1]` and are deterministic.
pub fn edges_from_incidence(incidence: &DMatrix<bool>) -> Vec<[usize; 2]> {
    let vertex_facets = vertex_facets_from_incidence(incidence);
    let mut edges = Vec::new();

    for left_vertex in 0..vertex_facets.len() {
        for right_vertex in left_vertex + 1..vertex_facets.len() {
            if count_common_sorted(&vertex_facets[left_vertex], &vertex_facets[right_vertex]) >= 3 {
                edges.push([left_vertex, right_vertex]);
            }
        }
    }

    edges
}

/// Return unordered 2-faces inferred from vertex-facet incidence.
///
/// A facet pair defines a 2-face candidate when at least three vertices are
/// incident to both facets. Returned facet pairs satisfy `facets[0] <
/// facets[1]`; each vertex list is sorted by increasing vertex index.
pub fn two_faces_from_incidence(incidence: &DMatrix<bool>) -> Vec<TwoFace> {
    let mut two_faces = Vec::new();

    for left_facet in 0..incidence.ncols() {
        for right_facet in left_facet + 1..incidence.ncols() {
            let vertices: Vec<usize> = (0..incidence.nrows())
                .filter(|&vertex_index| {
                    incidence[(vertex_index, left_facet)] && incidence[(vertex_index, right_facet)]
                })
                .collect();

            if vertices.len() >= 3 {
                two_faces.push(TwoFace {
                    facets: [left_facet, right_facet],
                    vertices,
                });
            }
        }
    }

    two_faces
}

fn count_common_sorted(left: &[usize], right: &[usize]) -> usize {
    let mut count = 0;
    let (mut left_index, mut right_index) = (0, 0);
    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => left_index += 1,
            std::cmp::Ordering::Greater => right_index += 1,
            std::cmp::Ordering::Equal => {
                count += 1;
                left_index += 1;
                right_index += 1;
            }
        }
    }
    count
}
