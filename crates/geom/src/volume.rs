/// 4D polytope volume via the divergence theorem.
///
/// vol(K) = (1/4) Σ_i h_i · vol_3D(F_i)
///
/// Proof sketch: div(x/4) = 1, so by the divergence theorem,
/// ∫_K 1 dV = ∫_∂K ⟨x/4, n⟩ dS. On facet F_i, ⟨x, n_i⟩ = h_i
/// (constant), so each facet integral = (h_i/4) · vol_3D(F_i).
///
/// Reference: Grünbaum, "Convex Polytopes", §14.1.
///
/// Each facet's 3D volume is computed by decomposing it into tetrahedra
/// via its ridge structure (intersections with other facets), using the
/// 4D cross product to compute tetrahedron volumes directly in R^4
/// without projecting to a 3D coordinate system.
use crate::cross_product::cross_product_4d;
use crate::polytope::Polytope4D;
use nalgebra::Vector4;

const EPS_ON_FACET: f64 = 1e-8;

/// Compute volume of a 4D convex polytope (origin must be in interior).
pub fn volume(polytope: &Polytope4D) -> f64 {
    let normals = polytope.normals();
    let heights = polytope.heights();
    let vertices = polytope.vertices();

    if vertices.len() < 5 {
        return 0.0;
    }

    (0..normals.len())
        .map(|i| heights[i] * facet_volume_3d(normals, heights, vertices, i, normals.len()))
        .sum::<f64>()
        / 4.0
}

/// Compute the 3D volume of facet `fi` by decomposing it into tetrahedra.
///
/// Strategy: pick the centroid of facet vertices as apex. For each ridge
/// (2D face = intersection with another facet), sort its vertices into a
/// convex polygon and fan-triangulate. Each triangle + centroid = tetrahedron
/// whose 3D volume is computed via the 4D cross product.
fn facet_volume_3d(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
    fi: usize,
    f: usize,
) -> f64 {
    // Vertices on this facet
    let facet_verts: Vec<Vector4<f64>> = vertices
        .iter()
        .filter(|v| (normals[fi].dot(v) - heights[fi]).abs() < EPS_ON_FACET)
        .cloned()
        .collect();

    if facet_verts.len() < 4 {
        return 0.0;
    }

    // Interior point of the facet
    let centroid = facet_verts.iter().copied().sum::<Vector4<f64>>() / facet_verts.len() as f64;

    // Sum over ridges: each ridge is the intersection of facet fi with another facet fj
    (0..f)
        .filter(|&fj| fj != fi)
        .flat_map(|fj| {
            // Ridge vertices: on both facet fi and facet fj
            let ridge_verts: Vec<Vector4<f64>> = facet_verts
                .iter()
                .filter(|v| (normals[fj].dot(v) - heights[fj]).abs() < EPS_ON_FACET)
                .cloned()
                .collect();

            if ridge_verts.len() < 3 {
                return Vec::new();
            }

            // Sort ridge vertices into convex polygon order and fan-triangulate
            let sorted = sort_polygon_vertices(&ridge_verts);
            (1..sorted.len() - 1)
                .map(|k| {
                    // 3D volume of tetrahedron (centroid, sorted[0], sorted[k], sorted[k+1])
                    // in R^4 via cross product: vol = ||a × b × c|| / 6
                    let a = sorted[0] - centroid;
                    let b = sorted[k] - centroid;
                    let c = sorted[k + 1] - centroid;
                    cross_product_4d(a, b, c).norm() / 6.0
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

/// Sort vertices of a convex polygon in R^4 by angle around their centroid.
///
/// The vertices lie in a 2D affine subspace. We build a 2D basis from the
/// vertex data itself (no normal needed) and sort by atan2.
fn sort_polygon_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    if vertices.len() <= 3 {
        return vertices.to_vec();
    }

    let n = vertices.len() as f64;
    let centroid = vertices.iter().copied().sum::<Vector4<f64>>() / n;

    // First basis direction: centroid → first vertex
    let d1 = (vertices[0] - centroid).normalize();

    // Second basis direction: first direction orthogonal to d1
    let d2 = match vertices.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > 1e-10).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return vertices.to_vec(), // degenerate
    };

    let mut indexed: Vec<(f64, Vector4<f64>)> = vertices
        .iter()
        .map(|v| {
            let rel = *v - centroid;
            let angle = rel.dot(&d2).atan2(rel.dot(&d1));
            (angle, *v)
        })
        .collect();

    indexed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    indexed.into_iter().map(|(_, v)| v).collect()
}

/// Volume of a 4-simplex: |det[v1-v0, v2-v0, v3-v0, v4-v0]| / 24.
pub fn simplex_volume_5(
    v0: Vector4<f64>,
    v1: Vector4<f64>,
    v2: Vector4<f64>,
    v3: Vector4<f64>,
    v4: Vector4<f64>,
) -> f64 {
    let mat = nalgebra::Matrix4::from_columns(&[v1 - v0, v2 - v0, v3 - v0, v4 - v0]);
    mat.determinant().abs() / 24.0
}

/// Compute volume via qconvex FA (direct volume computation).
///
/// Algorithm: Uses qconvex FA flag to compute volume directly from vertices.
///
/// This is simpler than the divergence theorem approach: qconvex handles all
/// the complex geometry internally, giving us a single volume number to parse.
/// The 1:1 correspondence is: "compute convex hull, extract volume".
///
/// Two-pass workflow: qhalf (vertices) → qconvex FA (volume).
///
/// Reference: qconvex FA flag documentation (Qhull manual, §4.3).
pub fn volume_qconvex(polytope: &Polytope4D) -> Result<f64, crate::QhullError> {
    let vertices = polytope.vertices();
    crate::qhull::compute_volume_qconvex(vertices)
}

/// Compute volume with cross-checking (divergence theorem vs qconvex FA).
///
/// Calls both volume algorithms and asserts they agree within tolerance.
/// This is a temporary wrapper for verification during the transition.
///
/// **TODO:** Remove after 1-week cross-check is complete.
pub fn volume_with_cross_check(polytope: &Polytope4D) -> f64 {
    let vol_div = volume(polytope);
    let vol_qhull = volume_qconvex(polytope).expect("qconvex should succeed");

    let rel_error = (vol_div - vol_qhull).abs() / vol_div.max(vol_qhull).max(1e-10);
    assert!(
        rel_error < 1e-6,
        "Volume algorithms disagree: divergence={}, qconvex={}, rel_error={}",
        vol_div,
        vol_qhull,
        rel_error
    );

    vol_div
}

#[cfg(test)]
#[path = "volume_test.rs"]
mod volume_test;
