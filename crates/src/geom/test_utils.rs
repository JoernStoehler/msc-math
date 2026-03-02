/// Convenience constructors for tests and tooling.
///
/// Known polytopes delegate to `known_polytopes` and strip the metadata.
/// For capacity values and literature references, use `known_polytopes` directly.
///
/// **Coordinates**: (q₁, q₂, p₁, p₂). See `symplectic.rs` for J₀ and ω₀.
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
/// Not in `known_polytopes` because it's parameterized — no single known capacity.
/// Expected: volume = 16s^4, EHZ capacity = 4s.
pub fn scaled_hypercube(s: f64) -> Polytope4D {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![s; 8];
    Polytope4D::new(normals, heights).expect("scaled hypercube")
}

/// 4D crosspolytope (16 facets). Delegates to `known_polytopes::crosspolytope()`.
pub fn crosspolytope() -> Polytope4D {
    crate::geom::known_polytopes::crosspolytope().polytope
}

/// Lagrangian triangle product (6 facets). Delegates to `known_polytopes::lagrangian_triangle_product()`.
pub fn lagrangian_triangle_product() -> Polytope4D {
    crate::geom::known_polytopes::lagrangian_triangle_product().polytope
}

/// Symplectic triangle product (6 facets). Delegates to `known_polytopes::symplectic_triangle_product()`.
pub fn symplectic_triangle_product() -> Polytope4D {
    crate::geom::known_polytopes::symplectic_triangle_product().polytope
}

/// Generate a random bounded polytope with specified number of facets.
///
/// Normals are uniformly distributed on S³ (via 4D standard normal, normalized).
/// Heights are random in [0.5, 2.0] to ensure 0 ∈ int(K).
/// Retries up to 100 times if the random configuration is unbounded.
///
/// # Panics
/// Panics if no valid polytope is found in 100 attempts.
pub fn random_bounded_polytope(facet_count: usize, rng: &mut impl Rng) -> Polytope4D {
    for _ in 0..100 {
        let normals: Vec<Vector4<f64>> = (0..facet_count)
            .map(|_| {
                let v = Vector4::new(
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                    rng.sample(StandardNormal),
                );
                v.normalize()
            })
            .collect();

        let heights: Vec<f64> = (0..facet_count)
            .map(|_| rng.gen_range(0.5..2.0))
            .collect();

        if let Ok(polytope) = Polytope4D::new(normals, heights) {
            return polytope;
        }
    }

    panic!(
        "Failed to generate bounded {}-facet polytope in 100 attempts",
        facet_count
    );
}
