//! Test-only flat polytope data constructors and helpers.
//!
//! Convenience wrappers around `known_polytopes`. For capacity values and
//! literature references, use `known_polytopes` directly.
//!
//! Coordinates: (q_1, q_2, p_1, p_2). See `symplectic_form` module for J_0 and omega_0.

use crate::geom::known_polytopes::KnownPolytope;
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;
use rand::Rng;
use rand_distr::StandardNormal;

/// 4-simplex (5 facets). Delegates to `known_polytopes::simplex()`.
pub fn simplex() -> &'static KnownPolytope {
    crate::geom::known_polytopes::simplex()
}

/// Hypercube [-1,1]^4 (8 facets). Delegates to `known_polytopes::hypercube()`.
pub fn hypercube() -> &'static KnownPolytope {
    crate::geom::known_polytopes::hypercube()
}

/// Scaled hypercube [-s, s]^4, returned as f64 dual vertices.
///
/// Not in `known_polytopes` because it is parameterized (no single known capacity).
/// Expected: volume = 16*s^4, EHZ capacity = 4*s.
pub fn scaled_hypercube_dual_vertices_f64(s: f64) -> Vec<Vector4<f64>> {
    assert!(s > 0.0);
    vec![
        Vector4::x() / s,
        -Vector4::x() / s,
        Vector4::y() / s,
        -Vector4::y() / s,
        Vector4::z() / s,
        -Vector4::z() / s,
        Vector4::w() / s,
        -Vector4::w() / s,
    ]
}

/// 4D crosspolytope (16 facets). Delegates to `known_polytopes::crosspolytope()`.
pub fn crosspolytope() -> &'static KnownPolytope {
    crate::geom::known_polytopes::crosspolytope()
}

/// Lagrangian triangle product (6 facets). Delegates to `known_polytopes`.
pub fn lagrangian_triangle_product() -> &'static KnownPolytope {
    crate::geom::known_polytopes::lagrangian_triangle_product()
}

/// Symplectic triangle product (6 facets). Delegates to `known_polytopes`.
pub fn symplectic_triangle_product() -> &'static KnownPolytope {
    crate::geom::known_polytopes::symplectic_triangle_product()
}

/// Generate accepted random f64 dual vertices with specified number of facets.
///
/// Normals are uniformly distributed on S^3 (via 4D standard normal, normalized).
/// Heights are random in [0.5, 2.0] to ensure 0 is in int(K).
/// Retries up to 100 times if the random configuration is unbounded.
///
/// # Panics
///
/// Panics if no valid dual-vertex set is found in 100 attempts.
pub fn random_bounded_dual_vertices_f64(
    facet_count: usize,
    rng: &mut impl Rng,
) -> Vec<Vector4<f64>> {
    for _ in 0..100 {
        let halfspaces: Vec<Vector4<f64>> = (0..facet_count)
            .map(|_| {
                let v = Vector4::new(
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                );
                let n = v.normalize();
                let h: f64 = rng.gen_range(0.5..2.0);
                n / h
            })
            .collect();

        if Polytope4D::from_f64(halfspaces.clone()).is_ok() {
            return halfspaces;
        }
    }

    panic!(
        "Failed to generate bounded {}-facet dual vertices in 100 attempts",
        facet_count
    );
}
