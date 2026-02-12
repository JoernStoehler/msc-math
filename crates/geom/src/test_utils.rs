/// Convenience constructors returning bare `Polytope4D` for tests.
///
/// These delegate to `known_polytopes` and strip the metadata.
/// For capacity values and literature references, use `known_polytopes` directly.
///
/// **Coordinates**: (q₁, q₂, p₁, p₂). See `symplectic.rs` for J₀ and ω₀.
use crate::polytope::Polytope4D;

#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use rand_distr::StandardNormal;

pub fn simplex() -> Polytope4D {
    crate::known_polytopes::simplex().polytope
}

pub fn hypercube() -> Polytope4D {
    crate::known_polytopes::hypercube().polytope
}

/// Scaled hypercube [-s, s]^4.
///
/// Not in `known_polytopes` because it's parameterized — no single known capacity.
/// Expected: volume = 16s^4, EHZ capacity = 4s.
pub fn scaled_hypercube(s: f64) -> Polytope4D {
    use nalgebra::Vector4;
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

pub fn crosspolytope() -> Polytope4D {
    crate::known_polytopes::crosspolytope().polytope
}

pub fn lagrangian_triangle_product() -> Polytope4D {
    crate::known_polytopes::lagrangian_triangle_product().polytope
}

pub fn symplectic_triangle_product() -> Polytope4D {
    crate::known_polytopes::symplectic_triangle_product().polytope
}

/// Generate a random bounded polytope with specified number of facets.
///
/// Normals are uniformly distributed on S³ (via sampling from 4D standard normal
/// and normalizing). Heights are random in [0.5, 2.0] to ensure 0 ∈ int(K).
///
/// # Panics
/// Panics if Polytope4D::new() fails (unbounded, degenerate, etc.)
/// Also panics if `facet_count < 5` (minimum for bounded 4D polytope).
#[cfg(test)]
pub fn random_bounded_polytope(facet_count: usize, rng: &mut impl Rng) -> Polytope4D {
    use nalgebra::Vector4;
    // Generate random unit vectors on S³
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

    // Random heights ensuring 0 ∈ int(K)
    let heights: Vec<f64> = (0..facet_count)
        .map(|_| rng.gen_range(0.5..2.0))
        .collect();

    Polytope4D::new(normals, heights).expect("random polytope should be valid")
}
