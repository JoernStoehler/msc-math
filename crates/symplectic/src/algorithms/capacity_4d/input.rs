use crate::algorithms::billiard::facet_classification::FacetClassification;
use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use crate::exact::{
    exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega_signs_exact,
    ExactPolytopeError,
};
use crate::geom::rational_arithmetic::f64_to_rational;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// Inclusive lower bound on every primal and dual vertex infinity norm.
pub const MIN_INPUT_NORM_INF: f64 = 1e-3;
/// Inclusive upper bound on every primal and dual vertex infinity norm.
pub const MAX_INPUT_NORM_INF: f64 = 1e3;
/// Maximum facet count accepted by the current four-dimensional route.
pub const MAX_INPUT_FACETS: usize = 16;

/// A four-dimensional dual-vertex input after exact binary64 validation.
///
/// The exact data are the dyadic rationals represented by `dual_vertices`,
/// not source rationals or unavailable algebraic coordinates.
#[derive(Clone, Debug)]
pub struct CapacityInput4d {
    pub(crate) dual_vertices: Vec<Vector4<f64>>,
    pub(crate) transition_is_allowed: DMatrix<bool>,
    pub(crate) product_facets: Option<FacetClassification>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapacityInputError {
    TooManyFacets { count: usize, maximum: usize },
    NonFiniteCoordinate { facet: usize, coordinate: usize },
    DualNormOutOfRange { facet: usize },
    InvalidExactPolytope(ExactPolytopeError),
    PrimalNormOutOfRange { vertex: usize },
}

impl std::fmt::Display for CapacityInputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyFacets { count, maximum } => {
                write!(formatter, "{count} facets exceed the supported maximum {maximum}")
            }
            Self::NonFiniteCoordinate { facet, coordinate } => write!(
                formatter,
                "dual facet {facet} has a non-finite coordinate at index {coordinate}"
            ),
            Self::DualNormOutOfRange { facet } => write!(
                formatter,
                "dual facet {facet} has infinity norm outside [{MIN_INPUT_NORM_INF}, {MAX_INPUT_NORM_INF}]"
            ),
            Self::InvalidExactPolytope(error) => {
                write!(formatter, "invalid exact binary64 polytope: {error:?}")
            }
            Self::PrimalNormOutOfRange { vertex } => write!(
                formatter,
                "primal vertex {vertex} has infinity norm outside [{MIN_INPUT_NORM_INF}, {MAX_INPUT_NORM_INF}]"
            ),
        }
    }
}

impl std::error::Error for CapacityInputError {}

impl CapacityInput4d {
    pub fn try_from_dual_vertices(
        dual_vertices: &[Vector4<f64>],
    ) -> Result<Self, CapacityInputError> {
        if dual_vertices.len() > MAX_INPUT_FACETS {
            return Err(CapacityInputError::TooManyFacets {
                count: dual_vertices.len(),
                maximum: MAX_INPUT_FACETS,
            });
        }
        for (facet, vertex) in dual_vertices.iter().enumerate() {
            for (coordinate, value) in vertex.iter().copied().enumerate() {
                if !value.is_finite() {
                    return Err(CapacityInputError::NonFiniteCoordinate { facet, coordinate });
                }
            }
            let norm = norm_inf_f64(vertex);
            if !(MIN_INPUT_NORM_INF..=MAX_INPUT_NORM_INF).contains(&norm) {
                return Err(CapacityInputError::DualNormOutOfRange { facet });
            }
        }

        let dual_vertices_exact = dual_vertices
            .iter()
            .map(|vertex| {
                [
                    f64_to_rational(vertex[0]),
                    f64_to_rational(vertex[1]),
                    f64_to_rational(vertex[2]),
                    f64_to_rational(vertex[3]),
                ]
            })
            .collect::<Vec<_>>();
        let exact_vectors = dual_vertices_exact
            .iter()
            .map(|vertex| {
                Vector4::new(
                    vertex[0].clone(),
                    vertex[1].clone(),
                    vertex[2].clone(),
                    vertex[3].clone(),
                )
            })
            .collect::<Vec<_>>();
        let exact_geometry = exact_vertices_with_incidence(&exact_vectors)
            .map_err(CapacityInputError::InvalidExactPolytope)?;
        let minimum = BigRational::new(1.into(), 1000.into());
        let maximum = BigRational::from_integer(1000.into());
        for (vertex_index, vertex) in exact_geometry.vertices.iter().enumerate() {
            let norm = vertex
                .iter()
                .map(Signed::abs)
                .max()
                .expect("a four-dimensional vertex has coordinates");
            if norm < minimum || norm > maximum {
                return Err(CapacityInputError::PrimalNormOutOfRange {
                    vertex: vertex_index,
                });
            }
        }

        let intersections =
            facet_intersection_is_nonempty_exact(&exact_geometry.vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&exact_vectors);
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &intersections,
            &omega_signs,
        );
        let product_facets = classify_exact_product_facets(&dual_vertices_exact);

        Ok(Self {
            dual_vertices: dual_vertices.to_vec(),
            transition_is_allowed,
            product_facets,
        })
    }

    pub fn dual_vertices(&self) -> &[Vector4<f64>] {
        &self.dual_vertices
    }

    pub fn is_structural_product(&self) -> bool {
        self.product_facets.is_some()
    }
}

fn classify_exact_product_facets(vertices: &[[BigRational; 4]]) -> Option<FacetClassification> {
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();
    for (index, vertex) in vertices.iter().enumerate() {
        let q_zero = vertex[0].is_zero() && vertex[1].is_zero();
        let p_zero = vertex[2].is_zero() && vertex[3].is_zero();
        match (q_zero, p_zero) {
            (false, true) => q_indices.push(index),
            (true, false) => p_indices.push(index),
            _ => return None,
        }
    }
    if q_indices.len() < 3 || p_indices.len() < 3 {
        return None;
    }
    Some(FacetClassification {
        q_indices,
        p_indices,
    })
}

fn norm_inf_f64(vector: &Vector4<f64>) -> f64 {
    vector.iter().copied().map(f64::abs).fold(0.0, f64::max)
}
