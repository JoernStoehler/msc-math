use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence, origin_in_interior_of_conv_exact,
    polar_vertices_exact_rational_assuming_origin_interior,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive};
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;

#[derive(Clone, Debug)]
pub struct CellPolytopeCache {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub vertices_f64: Vec<Vector4<f64>>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
}

impl CellPolytopeCache {
    pub fn from_f64(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
        if !dual_vertices_f64_are_valid(&dual_vertices_f64) {
            return None;
        }

        let dual_vertices: Vec<[BigRational; 4]> = dual_vertices_f64
            .iter()
            .map(|a| std::array::from_fn(|c| f64_to_rational(a[c])))
            .collect();
        Self::new(dual_vertices, Some(dual_vertices_f64))
    }

    pub fn new(
        dual_vertices: Vec<[BigRational; 4]>,
        dual_vertices_f64: Option<Vec<Vector4<f64>>>,
    ) -> Option<Self> {
        if dual_vertices.len() < 5 {
            return None;
        }

        let dual_vertex_vectors = rational_arrays_to_vectors(&dual_vertices);
        if !origin_in_interior_of_conv_exact(&dual_vertex_vectors) {
            return None;
        }

        let polar = polar_vertices_exact_rational_assuming_origin_interior(&dual_vertex_vectors);
        let vertices = rational_vectors_to_arrays(&polar.vertices);
        let vertices_f64 = rational_vectors_to_f64(&polar.vertices);
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(
                &polar.vertex_facet_incidence,
            );
        let omega_signs = omega_signs_exact(&dual_vertex_vectors);
        let dual_vertices_f64 =
            dual_vertices_f64.unwrap_or_else(|| rational_vectors_to_f64(&dual_vertex_vectors));

        Some(Self {
            dual_vertices,
            vertices,
            dual_vertices_f64,
            vertices_f64,
            vertex_facet_incidence: polar.vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
        })
    }

    pub fn from_rational_parts(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
    ) -> Option<Self> {
        if dual_vertices.len() < 5 || vertices.is_empty() {
            return None;
        }

        let dual_vertex_vectors = rational_arrays_to_vectors(&dual_vertices);
        let vertex_vectors = rational_arrays_to_vectors(&vertices);
        let vertex_facet_incidence = DMatrix::from_fn(
            vertex_vectors.len(),
            dual_vertices.len(),
            |vertex, facet| {
                dual_vertex_vectors[facet].dot(&vertex_vectors[vertex]) == BigRational::one()
            },
        );
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vertex_vectors);
        let dual_vertices_f64 = rational_vectors_to_f64(&dual_vertex_vectors);
        let vertices_f64 = rational_vectors_to_f64(&vertex_vectors);

        Some(Self {
            dual_vertices,
            vertices,
            dual_vertices_f64,
            vertices_f64,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
        })
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }
}

pub fn rational_arrays_to_vectors(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|row| {
            Vector4::new(
                row[0].clone(),
                row[1].clone(),
                row[2].clone(),
                row[3].clone(),
            )
        })
        .collect()
}

fn rational_vectors_to_arrays(data: &[Vector4<BigRational>]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|v| [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
        .collect()
}

fn rational_vectors_to_f64(data: &[Vector4<BigRational>]) -> Vec<Vector4<f64>> {
    data.iter()
        .map(|v| {
            Vector4::new(
                v[0].to_f64().unwrap_or(f64::NAN),
                v[1].to_f64().unwrap_or(f64::NAN),
                v[2].to_f64().unwrap_or(f64::NAN),
                v[3].to_f64().unwrap_or(f64::NAN),
            )
        })
        .collect()
}

fn dual_vertices_f64_are_valid(dual_vertices: &[Vector4<f64>]) -> bool {
    if dual_vertices.len() < 5 {
        return false;
    }
    if dual_vertices
        .iter()
        .any(|a| !a.iter().all(|value| value.is_finite()) || a.norm() < 1e-15)
    {
        return false;
    }
    for i in 0..dual_vertices.len() {
        for j in (i + 1)..dual_vertices.len() {
            let max_norm = dual_vertices[i].norm().max(dual_vertices[j].norm());
            if (dual_vertices[i] - dual_vertices[j]).norm() < 1e-8 * max_norm {
                return false;
            }
        }
    }
    true
}
