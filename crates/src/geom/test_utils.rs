//! Test-only polytope constructors and helpers.
//!
//! Convenience wrappers around `known_polytopes` that strip metadata and return
//! bare `Polytope4D` instances. For capacity values and literature references,
//! use `known_polytopes` directly.
//!
//! Coordinates: (q_1, q_2, p_1, p_2). See `symplectic_form` module for J_0 and omega_0.

use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;
use rand::Rng;
use rand_distr::StandardNormal;

/// 4-simplex (5 facets). Delegates to `known_polytopes::simplex()`.
pub fn simplex() -> Polytope4D {
    crate::geom::known_polytopes::simplex().polytope
}

/// Hypercube [-1,1]^4 (8 facets). Delegates to `known_polytopes::hypercube()`.
pub fn hypercube() -> Polytope4D {
    crate::geom::known_polytopes::hypercube().polytope
}

/// Scaled hypercube [-s, s]^4.
///
/// Not in `known_polytopes` because it is parameterized (no single known capacity).
/// Expected: volume = 16*s^4, EHZ capacity = 4*s.
pub fn scaled_hypercube(s: f64) -> Polytope4D {
    let halfspaces = vec![
        Vector4::x() / s,
        -Vector4::x() / s,
        Vector4::y() / s,
        -Vector4::y() / s,
        Vector4::z() / s,
        -Vector4::z() / s,
        Vector4::w() / s,
        -Vector4::w() / s,
    ];
    Polytope4D::new(halfspaces).expect("scaled hypercube")
}

/// 4D crosspolytope (16 facets). Delegates to `known_polytopes::crosspolytope()`.
pub fn crosspolytope() -> Polytope4D {
    crate::geom::known_polytopes::crosspolytope().polytope
}

/// Lagrangian triangle product (6 facets). Delegates to `known_polytopes`.
pub fn lagrangian_triangle_product() -> Polytope4D {
    crate::geom::known_polytopes::lagrangian_triangle_product().polytope
}

/// Symplectic triangle product (6 facets). Delegates to `known_polytopes`.
pub fn symplectic_triangle_product() -> Polytope4D {
    crate::geom::known_polytopes::symplectic_triangle_product().polytope
}

/// Generate a random bounded polytope with specified number of facets.
///
/// Normals are uniformly distributed on S^3 (via 4D standard normal, normalized).
/// Heights are random in [0.5, 2.0] to ensure 0 is in int(K).
/// Retries up to 100 times if the random configuration is unbounded.
///
/// # Panics
///
/// Panics if no valid polytope is found in 100 attempts.
pub fn random_bounded_polytope(facet_count: usize, rng: &mut impl Rng) -> Polytope4D {
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

        if let Ok(polytope) = Polytope4D::new(halfspaces) {
            return polytope;
        }
    }

    panic!(
        "Failed to generate bounded {}-facet polytope in 100 attempts",
        facet_count
    );
}
