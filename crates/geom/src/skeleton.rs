/// Combinatorial skeleton (face lattice) of a 4D convex polytope.
///
/// Computes the full face structure from vertex-facet incidence:
/// - 0-faces (vertices): stored on `Polytope4D`
/// - 1-faces (edges): pairs of vertices sharing ≥3 common facets
/// - 2-faces (ridges): pairs of facets sharing ≥3 vertices, forming convex polygons
/// - 3-faces (facets): stored on `Polytope4D` as normals/heights
///
/// The skeleton is NOT stored on `Polytope4D` to keep that struct lightweight.
/// Compute on demand via `Skeleton::compute()`.
use crate::polytope::Polytope4D;
use nalgebra::Vector4;

/// Vertex-on-facet incidence tolerance. Matches `EPS_FACET_INCIDENCE` in hk2017.
const EPS_INCIDENCE: f64 = 1e-8;

/// Full combinatorial skeleton of a 4D polytope.
#[derive(Clone, Debug)]
pub struct Skeleton {
    /// `vertex_facets[v]` = sorted facet indices incident to vertex `v`.
    pub vertex_facets: Vec<Vec<usize>>,
    /// Edges as pairs `[i, j]` of vertex indices with `i < j`.
    pub edges: Vec<[usize; 2]>,
    /// 2-faces (ridges): each is the intersection of two facets.
    pub ridges: Vec<Ridge>,
}

/// A 2-dimensional face (ridge) of a 4D polytope.
///
/// The intersection of two facets. Vertices are sorted into convex polygon
/// order within their 2D affine hull.
#[derive(Clone, Debug)]
pub struct Ridge {
    /// Two facet indices with `facets[0] < facets[1]`.
    pub facets: [usize; 2],
    /// Vertex indices forming the polygon boundary, in convex order.
    pub vertices: Vec<usize>,
}

impl Skeleton {
    /// Compute the skeleton from vertex-facet incidence.
    ///
    /// Complexity: O(V² · F) for edges, O(F² · V) for ridges.
    /// Fine for our polytopes (V ≤ 200, F ≤ 16).
    pub fn compute(polytope: &Polytope4D) -> Self {
        let normals = polytope.normals();
        let heights = polytope.heights();
        let vertices = polytope.vertices();
        let f = normals.len();
        let v_count = vertices.len();

        // Step 1: vertex-facet incidence
        let vertex_facets: Vec<Vec<usize>> = (0..v_count)
            .map(|vi| {
                (0..f)
                    .filter(|&fi| {
                        (normals[fi].dot(&vertices[vi]) - heights[fi]).abs() < EPS_INCIDENCE
                    })
                    .collect()
            })
            .collect();

        // Step 2: edges — vertex pairs sharing ≥3 common facets
        let mut edges = Vec::new();
        for i in 0..v_count {
            for j in (i + 1)..v_count {
                let common = count_common(&vertex_facets[i], &vertex_facets[j]);
                if common >= 3 {
                    edges.push([i, j]);
                }
            }
        }

        // Step 3: ridges — facet pairs sharing ≥3 vertices
        let mut ridges = Vec::new();
        for fi in 0..f {
            for fj in (fi + 1)..f {
                let ridge_verts: Vec<usize> = (0..v_count)
                    .filter(|&vi| {
                        vertex_facets[vi].contains(&fi) && vertex_facets[vi].contains(&fj)
                    })
                    .collect();
                if ridge_verts.len() >= 3 {
                    let sorted = sort_polygon_vertices(vertices, &ridge_verts);
                    ridges.push(Ridge {
                        facets: [fi, fj],
                        vertices: sorted,
                    });
                }
            }
        }

        Self {
            vertex_facets,
            edges,
            ridges,
        }
    }
}

/// Count elements common to two sorted slices.
fn count_common(a: &[usize], b: &[usize]) -> usize {
    let mut count = 0;
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
        }
    }
    count
}

/// Sort vertex indices into convex polygon order within their 2D affine hull.
///
/// Finds the centroid, builds a 2D basis via Gram-Schmidt, then sorts by angle.
fn sort_polygon_vertices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() <= 2 {
        return indices.to_vec();
    }

    let coords: Vec<Vector4<f64>> = indices.iter().map(|&i| all_vertices[i]).collect();
    let centroid = coords.iter().sum::<Vector4<f64>>() / coords.len() as f64;

    // First basis vector: centroid → first vertex
    let d1_raw = coords[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < 1e-12 {
        return indices.to_vec();
    }
    let d1 = d1_raw / d1_norm;

    // Second basis vector via Gram-Schmidt on remaining vertices
    let d2 = match coords.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > 1e-10).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return indices.to_vec(), // degenerate (collinear)
    };

    let mut indexed: Vec<(f64, usize)> = indices
        .iter()
        .map(|&idx| {
            let rel = all_vertices[idx] - centroid;
            let angle = rel.dot(&d2).atan2(rel.dot(&d1));
            (angle, idx)
        })
        .collect();
    indexed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    indexed.into_iter().map(|(_, idx)| idx).collect()
}

#[cfg(test)]
#[path = "skeleton_test.rs"]
mod skeleton_test;
