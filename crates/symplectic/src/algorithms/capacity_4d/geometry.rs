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

/// Exact geometry of the binary64 polytope
/// `{x in R^4 : <dual_vertices[i], x> <= 1}`.
///
/// The exact coordinates are the dyadic rationals represented by the input
/// binary64 values, not unavailable source rationals or algebraic numbers.
/// This is a plain data object: constructing it proves exact polytope validity,
/// but does not establish the separate numerical-size policy required by the
/// certified f64 capacity route.
#[derive(Clone, Debug, PartialEq)]
pub struct PolytopeGeometry4d {
    pub dual_vertices: Vec<Vector4<f64>>,
    pub dual_vertices_exact: Vec<Vector4<BigRational>>,
    pub primal_vertices_exact: Vec<Vector4<BigRational>>,
    pub vertex_facet_incidence: DMatrix<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolytopeGeometryError4d {
    NonFiniteCoordinate { facet: usize, coordinate: usize },
    InvalidExactPolytope(ExactPolytopeError),
}

impl std::fmt::Display for PolytopeGeometryError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteCoordinate { facet, coordinate } => write!(
                formatter,
                "dual facet {facet} has a non-finite coordinate at index {coordinate}"
            ),
            Self::InvalidExactPolytope(error) => {
                write!(formatter, "invalid exact binary64 polytope: {error:?}")
            }
        }
    }
}

impl std::error::Error for PolytopeGeometryError4d {}

/// Construct exact binary64 geometry and check boundedness, nonemptiness, and
/// facet irredundancy.
///
/// This function deliberately does not apply the f64 capacity route's
/// numerical-size policy and does not enumerate QP candidates.
pub fn exact_binary64_polytope_geometry(
    dual_vertices: &[Vector4<f64>],
) -> Result<PolytopeGeometry4d, PolytopeGeometryError4d> {
    check_finite_dual_vertices(dual_vertices)?;

    let dual_vertices_exact = dual_vertices
        .iter()
        .map(|vertex| {
            Vector4::new(
                f64_to_rational(vertex[0]),
                f64_to_rational(vertex[1]),
                f64_to_rational(vertex[2]),
                f64_to_rational(vertex[3]),
            )
        })
        .collect::<Vec<_>>();
    let exact_geometry = exact_vertices_with_incidence(&dual_vertices_exact)
        .map_err(PolytopeGeometryError4d::InvalidExactPolytope)?;

    Ok(PolytopeGeometry4d {
        dual_vertices: dual_vertices.to_vec(),
        dual_vertices_exact,
        primal_vertices_exact: exact_geometry.vertices,
        vertex_facet_incidence: exact_geometry.vertex_facet_incidence,
    })
}

/// Check that every supplied dual-vertex coordinate is finite.
pub fn check_finite_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Result<(), PolytopeGeometryError4d> {
    for (facet, vertex) in dual_vertices.iter().enumerate() {
        for (coordinate, value) in vertex.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(PolytopeGeometryError4d::NonFiniteCoordinate { facet, coordinate });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapacityInputBoundsError4d {
    TooManyFacets { count: usize, maximum: usize },
    DualNormOutOfRange { facet: usize },
    PrimalNormOutOfRange { vertex: usize },
}

impl std::fmt::Display for CapacityInputBoundsError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyFacets { count, maximum } => write!(
                formatter,
                "{count} facets exceed the supported maximum {maximum}"
            ),
            Self::DualNormOutOfRange { facet } => write!(
                formatter,
                "dual facet {facet} has infinity norm outside [{MIN_INPUT_NORM_INF}, {MAX_INPUT_NORM_INF}]"
            ),
            Self::PrimalNormOutOfRange { vertex } => write!(
                formatter,
                "primal vertex {vertex} has infinity norm outside [{MIN_INPUT_NORM_INF}, {MAX_INPUT_NORM_INF}]"
            ),
        }
    }
}

impl std::error::Error for CapacityInputBoundsError4d {}

/// Cheaply check the facet-count limit of the current production route.
pub fn check_facet_count(facet_count: usize) -> Result<(), CapacityInputBoundsError4d> {
    if facet_count > MAX_INPUT_FACETS {
        return Err(CapacityInputBoundsError4d::TooManyFacets {
            count: facet_count,
            maximum: MAX_INPUT_FACETS,
        });
    }
    Ok(())
}

/// Cheaply check the dual-vertex part of the certified f64 route's
/// numerical-size policy.
pub fn check_dual_vertex_norm_bounds(
    dual_vertices: &[Vector4<f64>],
) -> Result<(), CapacityInputBoundsError4d> {
    for (facet, vertex) in dual_vertices.iter().enumerate() {
        let norm = norm_inf_f64(vertex);
        if !(MIN_INPUT_NORM_INF..=MAX_INPUT_NORM_INF).contains(&norm) {
            return Err(CapacityInputBoundsError4d::DualNormOutOfRange { facet });
        }
    }
    Ok(())
}

/// Check the primal-vertex part of the certified f64 route's numerical-size
/// policy after exact geometry construction.
pub fn check_primal_vertex_norm_bounds(
    geometry: &PolytopeGeometry4d,
) -> Result<(), CapacityInputBoundsError4d> {
    let minimum = BigRational::new(1.into(), 1000.into());
    let maximum = BigRational::from_integer(1000.into());
    for (vertex, coordinates) in geometry.primal_vertices_exact.iter().enumerate() {
        let norm = coordinates
            .iter()
            .map(Signed::abs)
            .max()
            .expect("a four-dimensional vertex has coordinates");
        if norm < minimum || norm > maximum {
            return Err(CapacityInputBoundsError4d::PrimalNormOutOfRange { vertex });
        }
    }
    Ok(())
}

pub(super) fn check_vertex_norm_bounds(
    geometry: &PolytopeGeometry4d,
) -> Result<(), CapacityInputBoundsError4d> {
    check_dual_vertex_norm_bounds(&geometry.dual_vertices)?;
    check_primal_vertex_norm_bounds(geometry)
}

/// Build the exact incidence- and symplectic-sign-pruned transition graph.
pub fn capacity_transition_graph(geometry: &PolytopeGeometry4d) -> DMatrix<bool> {
    let intersections = facet_intersection_is_nonempty_exact(&geometry.vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&geometry.dual_vertices_exact);
    build_transition_matrix_from_facet_intersections_and_omega(&intersections, &omega_signs)
}

/// Recognize exact structural `q`/`p` products.
pub fn classify_lagrangian_product(geometry: &PolytopeGeometry4d) -> Option<FacetClassification> {
    classify_exact_product_facets(&geometry.dual_vertices_exact)
}

fn classify_exact_product_facets(vertices: &[Vector4<BigRational>]) -> Option<FacetClassification> {
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
