use euclidean_polytopes::{
    all_points_are_extreme_exact, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    origin_in_interior_of_conv_exact, polar_vertices_exact_rational_assuming_origin_interior,
    sample_random_dual_vertices_f64, volume_from_incidence_exact,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::known_polytopes::KnownPolytope;

#[derive(Clone, Debug)]
pub(super) struct LocalPolytopeCache {
    pub(super) dual_vertices: Vec<[BigRational; 4]>,
    pub(super) vertices: Vec<[BigRational; 4]>,
    pub(super) vertex_facet_incidence: DMatrix<bool>,
    pub(super) facet_intersection_is_nonempty: DMatrix<bool>,
    pub(super) omega_signs: DMatrix<i8>,
    pub(super) dual_vertices_f64: Vec<Vector4<f64>>,
    pub(super) vertices_f64: Vec<Vector4<f64>>,
}

impl LocalPolytopeCache {
    pub(super) fn from_f64_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
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

    pub(super) fn from_known(polytope: &KnownPolytope) -> Self {
        Self::assemble(
            polytope.dual_vertices.clone(),
            polytope.vertices.clone(),
            polytope.vertex_facet_incidence.clone(),
            polytope.dual_vertices_f64.clone(),
        )
    }

    pub(super) fn generate_random(
        facet_count: usize,
        h_min: f64,
        h_max: f64,
        master_seed: u64,
    ) -> Option<Self> {
        for attempt in 0..100u64 {
            let mut key_material = [0u8; 16];
            key_material[..8].copy_from_slice(&master_seed.to_le_bytes());
            key_material[8..].copy_from_slice(&attempt.to_le_bytes());
            let seed = blake3::derive_key("local-sys-methods-random-basepoint", &key_material);
            let mut rng = ChaCha8Rng::from_seed(seed);
            let dual_vertices =
                sample_random_dual_vertices_f64(facet_count, h_min, h_max, &mut rng);
            if let Some(polytope) = Self::from_f64_dual_vertices(dual_vertices) {
                return Some(polytope);
            }
        }
        None
    }

    pub(super) fn facet_count(&self) -> usize {
        self.dual_vertices_f64.len()
    }

    pub(super) fn volume(&self) -> f64 {
        let vertices: Vec<Vector4<BigRational>> = self
            .vertices
            .iter()
            .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
            .collect();
        volume_from_incidence_exact(&vertices, &self.vertex_facet_incidence)
            .to_f64()
            .unwrap_or(f64::NAN)
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
            .expect("local sys methods require vertices to be representable as f64");

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
        if !a.iter().all(|value| value.is_finite()) || a.norm() < 1e-12 {
            return None;
        }
    }
    for i in 0..dual_vertices_f64.len() {
        for j in i + 1..dual_vertices_f64.len() {
            let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
            if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm() < 1e-10 * max_norm {
                return None;
            }
        }
    }
    Some(())
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
