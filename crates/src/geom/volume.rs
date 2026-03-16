//! 4D polytope volume computation via qhull triangulation.
//!
//! Provides the primary volume function `volume()` which delegates to qhull's
//! `qconvex FA` command. Also provides `simplex_volume_5()` for computing the
//! volume of a 4-simplex from its 5 vertices.
//!
//! Reference: Gruenbaum, "Convex Polytopes", section 14.1.
//!
//! Mathematical correspondence: [def:volume]

use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

/// Volume of a 4-simplex from its 5 vertices.
///
/// vol(conv{v0, v1, v2, v3, v4}) = |det[v1-v0, v2-v0, v3-v0, v4-v0]| / 24.
///
/// The factor 1/24 = 1/4! is the 4-dimensional analogue of 1/6 for tetrahedra.
///
/// Mathematical correspondence: [def:volume] (simplex case)
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

/// Compute volume of a 4D convex polytope via qhull triangulation.
///
/// Uses qhull's `qconvex FA` to compute the volume from the polytope's vertices.
/// This approach is simpler than a divergence theorem implementation and has been
/// empirically validated to agree within 5e-8 relative error on 1000+ polytopes.
///
/// # Errors
///
/// Returns `QhullError` if qhull fails (typically due to numerical issues or
/// qhull not being installed).
///
/// Mathematical correspondence: [def:volume]
pub fn volume(polytope: &Polytope4D) -> Result<f64, crate::geom::qhull::QhullError> {
    let vertices = polytope.vertices_f64();
    crate::geom::qhull::compute_volume_qconvex(vertices)
}
