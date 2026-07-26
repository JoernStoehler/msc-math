//! Sys-landscape workflow cache for repeated experiment geometry.
//!
//! The cache is exported only so the sys-landscape package's many binaries can
//! share reconstruction and database plumbing. Capacity, KKT, derivative, and
//! feature code still consumes the explicit fields rather than a shared
//! geometry abstraction.

use euclidean_polytopes::{
    all_points_are_extreme_exact, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    origin_in_interior_of_conv_exact, polar_vertices_exact_rational_assuming_origin_interior,
    sample_random_dual_vertices_f64,
};
use nalgebra::{DMatrix, Vector2, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use symplectic::capacity_4d::{
    exact_binary64_polytope_geometry, PolytopeGeometry4d, PolytopeGeometryError4d,
};
use symplectic::database::PolytopeRecord;
use symplectic::exact::omega_signs_exact;

const EPS_ZERO_NORM: f64 = 1e-12;
const EPS_DUPLICATE_RELATIVE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct SysLandscapePolytopeCache {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub vertices_f64: Vec<Vector4<f64>>,
}

/// Obtain the production route's exact-binary64 geometry from an already
/// validated sys-landscape cache.
///
/// Generated binary64 caches already store the same dyadic rationals and can
/// reuse their primal vertices and incidence. Source-rational fixtures may
/// describe a different exact rational object; those are reconstructed from
/// their binary64 dual vertices so the production contract remains unchanged.
///
/// This function establishes geometry only. Capacity callers still apply
/// `check_facet_count`, `check_dual_vertex_norm_bounds`, and
/// `check_primal_vertex_norm_bounds` explicitly.
pub fn exact_binary64_geometry_from_cache(
    polytope: &SysLandscapePolytopeCache,
) -> Result<PolytopeGeometry4d, PolytopeGeometryError4d> {
    let dual_vertices_exact = polytope
        .dual_vertices_f64
        .iter()
        .map(|vertex| {
            Some(Vector4::new(
                BigRational::from_float(vertex[0])?,
                BigRational::from_float(vertex[1])?,
                BigRational::from_float(vertex[2])?,
                BigRational::from_float(vertex[3])?,
            ))
        })
        .collect::<Option<Vec<_>>>();
    let cached_dual_vertices_exact = vectors_from_arrays(&polytope.dual_vertices);

    if dual_vertices_exact.as_ref() == Some(&cached_dual_vertices_exact) {
        return Ok(PolytopeGeometry4d {
            dual_vertices: polytope.dual_vertices_f64.clone(),
            dual_vertices_exact: cached_dual_vertices_exact,
            primal_vertices_exact: vectors_from_arrays(&polytope.vertices),
            vertex_facet_incidence: polytope.vertex_facet_incidence.clone(),
        });
    }

    exact_binary64_polytope_geometry(&polytope.dual_vertices_f64)
}

impl SysLandscapePolytopeCache {
    pub fn from_f64_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
        validate_f64_dual_vertices(&dual_vertices_f64)?;
        let dual_vertices = dual_vertices_f64
            .iter()
            .map(|a| {
                Some(std::array::from_fn(|c| {
                    BigRational::from_float(a[c]).expect("finite f64 was validated")
                }))
            })
            .collect::<Option<Vec<_>>>()?;
        let dual_vectors = vectors_from_arrays(&dual_vertices);

        if !origin_in_interior_of_conv_exact(&dual_vectors)
            || !all_points_are_extreme_exact(&dual_vectors)
        {
            return None;
        }

        let polar = polar_vertices_exact_rational_assuming_origin_interior(&dual_vectors);
        let vertices = arrays_from_vectors(&polar.vertices);
        Some(Self::assemble(
            dual_vertices,
            vertices,
            polar.vertex_facet_incidence,
            dual_vertices_f64,
        ))
    }

    pub fn sample_random(
        facet_count: usize,
        h_min: f64,
        h_max: f64,
        rng: &mut ChaCha8Rng,
    ) -> Option<Self> {
        validate_sampling_parameters(facet_count, h_min, h_max)?;
        Self::from_f64_dual_vertices(sample_random_dual_vertices_f64(
            facet_count,
            h_min,
            h_max,
            rng,
        ))
    }

    pub fn generate_random(
        facet_count: usize,
        h_min: f64,
        h_max: f64,
        master_seed: u64,
        attempt: u64,
    ) -> Option<Self> {
        let mut key_material = [0u8; 16];
        key_material[..8].copy_from_slice(&master_seed.to_le_bytes());
        key_material[8..].copy_from_slice(&attempt.to_le_bytes());
        let seed = blake3::derive_key("polytope-gen", &key_material);
        let mut rng = ChaCha8Rng::from_seed(seed);
        Self::sample_random(facet_count, h_min, h_max, &mut rng)
    }

    pub fn from_lagrangian_product(
        q_normals: &[Vector2<f64>],
        q_heights: &[f64],
        p_normals: &[Vector2<f64>],
        p_heights: &[f64],
    ) -> Option<Self> {
        if q_normals.len() != q_heights.len() || p_normals.len() != p_heights.len() {
            return None;
        }
        let mut dual_vertices = Vec::with_capacity(q_normals.len() + p_normals.len());
        for (normal, height) in q_normals.iter().zip(q_heights) {
            if !height.is_finite() || *height <= 0.0 {
                return None;
            }
            dual_vertices.push(Vector4::new(
                normal[0] / height,
                normal[1] / height,
                0.0,
                0.0,
            ));
        }
        for (normal, height) in p_normals.iter().zip(p_heights) {
            if !height.is_finite() || *height <= 0.0 {
                return None;
            }
            dual_vertices.push(Vector4::new(
                0.0,
                0.0,
                normal[0] / height,
                normal[1] / height,
            ));
        }
        Self::from_f64_dual_vertices(dual_vertices)
    }

    pub fn from_rational_parts(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
    ) -> Option<Self> {
        let dual_vertices_f64 = dual_vertices
            .iter()
            .map(vector_f64_from_array)
            .collect::<Option<Vec<_>>>()?;
        let vertex_facet_incidence = incidence_from_rational_parts(&dual_vertices, &vertices);
        Some(Self::assemble(
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            dual_vertices_f64,
        ))
    }

    /// Reconstructs a cached polytope after a previous validated construction.
    ///
    /// This is a shape-only trust boundary: it checks matrix/vector dimensions
    /// but does not recompute vertices, incidence, omega signs, or rational/f64
    /// agreement. Use only for cache payloads produced from an existing
    /// `SysLandscapePolytopeCache`.
    pub fn from_trusted_parts(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_facet_incidence: DMatrix<bool>,
        facet_intersection_is_nonempty: DMatrix<bool>,
        omega_signs: DMatrix<i8>,
        dual_vertices_f64: Vec<Vector4<f64>>,
        vertices_f64: Vec<Vector4<f64>>,
    ) -> Option<Self> {
        if dual_vertices.len() != dual_vertices_f64.len()
            || vertices.len() != vertices_f64.len()
            || vertex_facet_incidence.ncols() != dual_vertices.len()
            || vertex_facet_incidence.nrows() != vertices.len()
            || facet_intersection_is_nonempty.ncols() != dual_vertices.len()
            || facet_intersection_is_nonempty.nrows() != dual_vertices.len()
            || omega_signs.ncols() != dual_vertices.len()
            || omega_signs.nrows() != dual_vertices.len()
        {
            return None;
        }
        Some(Self {
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
            dual_vertices_f64,
            vertices_f64,
        })
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }

    pub fn to_record(&self) -> PolytopeRecord {
        PolytopeRecord {
            dual_vertices_rational: self.dual_vertices.clone(),
            vertices_rational: self.vertices.clone(),
            source: None,
            volume: None,
            volume_err: None,
            capacity: None,
            capacity_err: None,
            sigma_gap_cutoff: None,
            sigmas: None,
            orbit_scalars: None,
        }
    }

    fn assemble(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_facet_incidence: DMatrix<bool>,
        dual_vertices_f64: Vec<Vector4<f64>>,
    ) -> Self {
        let dual_vectors = vectors_from_arrays(&dual_vertices);
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vectors);
        let vertices_f64 = vertices
            .iter()
            .map(vector_f64_from_array)
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();

        Self {
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
            dual_vertices_f64,
            vertices_f64,
        }
    }
}

fn validate_f64_dual_vertices(dual_vertices_f64: &[Vector4<f64>]) -> Option<()> {
    if dual_vertices_f64.len() < 5 {
        return None;
    }
    for a in dual_vertices_f64 {
        if !a.iter().all(|value| value.is_finite()) || a.norm() < EPS_ZERO_NORM {
            return None;
        }
    }
    for i in 0..dual_vertices_f64.len() {
        for j in i + 1..dual_vertices_f64.len() {
            let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
            if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm()
                < EPS_DUPLICATE_RELATIVE * max_norm
            {
                return None;
            }
        }
    }
    Some(())
}

fn validate_sampling_parameters(facet_count: usize, h_min: f64, h_max: f64) -> Option<()> {
    (facet_count >= 5 && h_min.is_finite() && h_max.is_finite() && h_min > 0.0 && h_min < h_max)
        .then_some(())
}

fn incidence_from_rational_parts(
    dual_vertices: &[[BigRational; 4]],
    vertices: &[[BigRational; 4]],
) -> DMatrix<bool> {
    DMatrix::from_fn(
        vertices.len(),
        dual_vertices.len(),
        |vertex_index, facet_index| {
            let a = &dual_vertices[facet_index];
            let v = &vertices[vertex_index];
            &a[0] * &v[0] + &a[1] * &v[1] + &a[2] * &v[2] + &a[3] * &v[3]
                == BigRational::from_integer(1.into())
        },
    )
}

fn vectors_from_arrays(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect()
}

fn arrays_from_vectors(data: &[Vector4<BigRational>]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|v| [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
        .collect()
}

fn vector_f64_from_array(v: &[BigRational; 4]) -> Option<Vector4<f64>> {
    Some(Vector4::new(
        v[0].to_f64()?,
        v[1].to_f64()?,
        v[2].to_f64()?,
        v[3].to_f64()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_binary64_geometry_ignores_distinct_source_rationals() {
        let normals = [
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(-1.0, 0.0),
            Vector2::new(0.0, -1.0),
        ];
        let heights = [1.0; 4];
        let mut cache = SysLandscapePolytopeCache::from_lagrangian_product(
            &normals, &heights, &normals, &heights,
        )
        .expect("square product");
        let binary64_coordinate =
            BigRational::from_float(cache.dual_vertices_f64[0][0]).expect("finite coordinate");
        cache.dual_vertices[0][0] =
            &binary64_coordinate + BigRational::new(1.into(), (1_u64 << 60).into());

        let geometry =
            exact_binary64_geometry_from_cache(&cache).expect("reconstruct binary64 geometry");

        assert_eq!(geometry.dual_vertices_exact[0][0], binary64_coordinate);
        assert_ne!(
            geometry.dual_vertices_exact[0][0],
            cache.dual_vertices[0][0]
        );
    }
}
