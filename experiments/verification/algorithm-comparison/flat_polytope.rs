//! Experiment-local flat geometry cache for algorithm-comparison binaries.
//!
//! The ablation and benchmark workflows need repeated access to exact dual
//! vertices, primal vertices, incidence, omega signs, and f64 dual vertices.
//! Keep that cache local to the experiment instead of depending on the private
//! `symplectic` polytope wrapper.

use euclidean_polytopes::{
    all_points_are_extreme_exact, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    origin_in_interior_of_conv_exact, polar_vertices_exact, sample_random_dual_vertices_f64,
};
use nalgebra::{DMatrix, Vector2, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::Rng;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::known_polytopes::KnownPolytope;
use symplectic::geom::lagrangian_product::lagrangian_product;

const EPS_ZERO_NORM: f64 = 1e-12;
const EPS_DUPLICATE_RELATIVE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct FlatPolytopeCache {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
}

impl FlatPolytopeCache {
    pub fn from_f64_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
        validate_f64_dual_vertices(&dual_vertices_f64)?;
        let dual_vertices = dual_vertices_f64
            .iter()
            .map(|a| {
                std::array::from_fn(|c| {
                    BigRational::from_float(a[c]).expect("finite f64 was validated")
                })
            })
            .collect::<Vec<_>>();
        let dual_vectors = vectors_from_arrays(&dual_vertices);
        if !origin_in_interior_of_conv_exact(&dual_vectors)
            || !all_points_are_extreme_exact(&dual_vectors)
        {
            return None;
        }
        let polar = polar_vertices_exact(&dual_vectors);
        Some(Self::assemble(
            dual_vertices,
            arrays_from_vectors(&polar.vertices),
            polar.vertex_facet_incidence,
            dual_vertices_f64,
        ))
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

    pub fn from_lagrangian_product(
        q_normals: &[Vector2<f64>],
        q_heights: &[f64],
        p_normals: &[Vector2<f64>],
        p_heights: &[f64],
    ) -> Option<Self> {
        let dual_vertices = lagrangian_product(q_normals, q_heights, p_normals, p_heights).ok()?;
        Self::from_f64_dual_vertices(dual_vertices)
    }

    pub fn from_known(known: &KnownPolytope) -> Self {
        Self {
            dual_vertices: known.dual_vertices.clone(),
            vertices: known.vertices.clone(),
            vertex_facet_incidence: known.vertex_facet_incidence.clone(),
            facet_intersection_is_nonempty: known.facet_intersection_is_nonempty.clone(),
            omega_signs: known.omega_signs.clone(),
            dual_vertices_f64: known.dual_vertices_f64.clone(),
        }
    }

    pub fn sample_random<R: Rng + ?Sized>(
        facet_count: usize,
        h_min: f64,
        h_max: f64,
        rng: &mut R,
    ) -> Option<Self> {
        (facet_count >= 5
            && h_min.is_finite()
            && h_max.is_finite()
            && h_min > 0.0
            && h_min < h_max)
            .then_some(())?;
        Self::from_f64_dual_vertices(sample_random_dual_vertices_f64(
            facet_count,
            h_min,
            h_max,
            rng,
        ))
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
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
        Self {
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
            dual_vertices_f64,
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
