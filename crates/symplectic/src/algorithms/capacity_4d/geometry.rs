use crate::algorithms::billiard::facet_classification::FacetClassification;
use crate::algorithms::capacity_4d::geometry_rank::affine_rank_at_least_three_exact;
use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use crate::exact::{facet_intersection_is_nonempty_exact, omega_signs_exact, ExactPolytopeError};
use crate::geom::rational_arithmetic::f64_to_rational;
use euclidean_polytopes::{
    origin_in_interior_of_conv_exact_rational, origin_in_interior_of_conv_f64,
    polar_vertices_exact_rational_assuming_origin_interior, OriginInteriorF64,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::Signed;

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
/// The exact output contract does not require generic exact arithmetic on every
/// four-facet subset. Boundedness is first decided by the certified trinary
/// predicate in `euclidean_polytopes`; only an indeterminate result is resolved
/// exactly. Polar vertices use a one-sided f64 rejection filter followed by
/// exact integer arithmetic for every survivor. See
/// `crates/euclidean-polytopes/src/polar.rs` and its comparison tests.
///
/// This function deliberately does not apply the f64 capacity route's
/// numerical-size policy and does not enumerate QP candidates.
pub fn exact_binary64_polytope_geometry(
    dual_vertices: &[Vector4<f64>],
) -> Result<PolytopeGeometry4d, PolytopeGeometryError4d> {
    check_finite_dual_vertices(dual_vertices)?;
    check_basic_exact_polytope_input(dual_vertices)?;

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

    // For inequalities <a_i, x> <= 1, boundedness is equivalent to
    // 0 belonging to the interior of conv{a_i}. The f64 predicate is
    // one-sided: True and False are certificates for the exact binary64
    // inputs; Indeterminate alone needs the integer-scaled exact predicate.
    let origin_is_interior = match origin_in_interior_of_conv_f64(dual_vertices) {
        OriginInteriorF64::True => true,
        OriginInteriorF64::False => false,
        OriginInteriorF64::Indeterminate => {
            origin_in_interior_of_conv_exact_rational(&dual_vertices_exact)
        }
    };
    if !origin_is_interior {
        return Err(PolytopeGeometryError4d::InvalidExactPolytope(
            ExactPolytopeError::Unbounded,
        ));
    }

    let exact_geometry =
        polar_vertices_exact_rational_assuming_origin_interior(&dual_vertices_exact);
    check_exact_facet_irredundancy(
        &exact_geometry.vertices,
        &exact_geometry.vertex_facet_incidence,
    )
    .map_err(PolytopeGeometryError4d::InvalidExactPolytope)?;

    Ok(PolytopeGeometry4d {
        dual_vertices: dual_vertices.to_vec(),
        dual_vertices_exact,
        primal_vertices_exact: exact_geometry.vertices,
        vertex_facet_incidence: exact_geometry.vertex_facet_incidence,
    })
}

fn check_basic_exact_polytope_input(
    dual_vertices: &[Vector4<f64>],
) -> Result<(), PolytopeGeometryError4d> {
    if dual_vertices.len() < 5 {
        return Err(PolytopeGeometryError4d::InvalidExactPolytope(
            ExactPolytopeError::TooFewFacets(dual_vertices.len()),
        ));
    }
    for (facet, dual) in dual_vertices.iter().enumerate() {
        if dual.iter().all(|entry| *entry == 0.0) {
            return Err(PolytopeGeometryError4d::InvalidExactPolytope(
                ExactPolytopeError::ZeroDualVertex(facet),
            ));
        }
    }
    Ok(())
}

fn check_exact_facet_irredundancy(
    vertices: &[Vector4<BigRational>],
    incidence: &DMatrix<bool>,
) -> Result<(), ExactPolytopeError> {
    for facet in 0..incidence.ncols() {
        let incident = (0..incidence.nrows())
            .filter(|&vertex| incidence[(vertex, facet)])
            .map(|vertex| vertices[vertex].clone())
            .collect::<Vec<_>>();
        if !affine_rank_at_least_three_exact(&incident) {
            return Err(ExactPolytopeError::RedundantFacet(facet));
        }
    }
    Ok(())
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
///
/// A forbidden edge removes words from the complete search, so the selected
/// implementation uses exact incidence and exact omega signs. A proved
/// conservative f64 supergraph could replace this work; none is currently
/// selected. See `crates/symplectic/DEVELOPMENT.md`.
pub fn capacity_transition_graph(geometry: &PolytopeGeometry4d) -> DMatrix<bool> {
    let intersections = facet_intersection_is_nonempty_exact(&geometry.vertex_facet_incidence);
    let omega_signs = omega_signs_exact(&geometry.dual_vertices_exact);
    build_transition_matrix_from_facet_intersections_and_omega(&intersections, &omega_signs)
}

/// Recognize exact structural `q`/`p` products.
pub fn classify_lagrangian_product(geometry: &PolytopeGeometry4d) -> Option<FacetClassification> {
    // Binary64 zero is already exact for the dyadic input represented by
    // `PolytopeGeometry4d`; converting these block-zero tests to rationals
    // would add no information.
    classify_product_facets_f64(&geometry.dual_vertices)
}

fn classify_product_facets_f64(vertices: &[Vector4<f64>]) -> Option<FacetClassification> {
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();
    for (index, vertex) in vertices.iter().enumerate() {
        let q_zero = vertex[0] == 0.0 && vertex[1] == 0.0;
        let p_zero = vertex[2] == 0.0 && vertex[3] == 0.0;
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

#[cfg(test)]
mod tests {
    use super::{exact_binary64_polytope_geometry, PolytopeGeometryError4d};
    use crate::exact::{exact_vertices_with_incidence, ExactPolytopeError};
    use crate::geom::rational_arithmetic::f64_to_rational;
    use crate::known_polytopes;
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    type VertexWithIncidence = ([BigRational; 4], Vec<bool>);

    fn sorted_geometry_rows(
        vertices: &[Vector4<BigRational>],
        incidence: &nalgebra::DMatrix<bool>,
    ) -> Vec<VertexWithIncidence> {
        let mut rows = vertices
            .iter()
            .enumerate()
            .map(|(row, vertex)| {
                (
                    std::array::from_fn(|coordinate| vertex[coordinate].clone()),
                    incidence.row(row).iter().copied().collect(),
                )
            })
            .collect::<Vec<_>>();
        rows.sort();
        rows
    }

    #[test]
    fn optimized_binary64_geometry_matches_generic_exact_constructor() {
        for dual_vertices in [
            known_polytopes::simplex().dual_vertices_f64.clone(),
            known_polytopes::hypercube().dual_vertices_f64.clone(),
            known_polytopes::crosspolytope().dual_vertices_f64.clone(),
            known_polytopes::lagrangian_triangle_product()
                .dual_vertices_f64
                .clone(),
        ] {
            let optimized =
                exact_binary64_polytope_geometry(&dual_vertices).expect("valid exact geometry");
            let exact_duals = dual_vertices
                .iter()
                .map(|vertex| vertex.map(f64_to_rational))
                .collect::<Vec<_>>();
            let generic =
                exact_vertices_with_incidence(&exact_duals).expect("generic exact geometry");

            assert_eq!(optimized.dual_vertices_exact, exact_duals);
            assert_eq!(
                sorted_geometry_rows(
                    &optimized.primal_vertices_exact,
                    &optimized.vertex_facet_incidence
                ),
                sorted_geometry_rows(&generic.vertices, &generic.vertex_facet_incidence)
            );
        }
    }

    #[test]
    fn optimized_binary64_geometry_preserves_exact_input_errors() {
        let too_few = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        assert_eq!(
            exact_binary64_polytope_geometry(&too_few),
            Err(PolytopeGeometryError4d::InvalidExactPolytope(
                ExactPolytopeError::TooFewFacets(4)
            ))
        );

        let mut zero = known_polytopes::simplex().dual_vertices_f64.clone();
        zero[2] = Vector4::zeros();
        assert_eq!(
            exact_binary64_polytope_geometry(&zero),
            Err(PolytopeGeometryError4d::InvalidExactPolytope(
                ExactPolytopeError::ZeroDualVertex(2)
            ))
        );

        let mut unbounded = too_few;
        unbounded.push(Vector4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(
            exact_binary64_polytope_geometry(&unbounded),
            Err(PolytopeGeometryError4d::InvalidExactPolytope(
                ExactPolytopeError::Unbounded
            ))
        );

        let mut redundant = known_polytopes::hypercube().dual_vertices_f64.clone();
        redundant.push(Vector4::new(0.5, 0.0, 0.0, 0.0));
        assert_eq!(
            exact_binary64_polytope_geometry(&redundant),
            Err(PolytopeGeometryError4d::InvalidExactPolytope(
                ExactPolytopeError::RedundantFacet(8)
            ))
        );
    }

    #[test]
    fn optimized_binary64_geometry_matches_generic_exact_on_generated_f10_inputs() {
        let mut rng = ChaCha8Rng::seed_from_u64(0x6578_6163_745f_6175);
        let mut valid_cases = 0;
        for _ in 0..8 {
            let dual_vertices =
                euclidean_polytopes::sample_random_dual_vertices_f64(10, 0.8, 1.2, &mut rng);
            let exact_duals = dual_vertices
                .iter()
                .map(|vertex| vertex.map(f64_to_rational))
                .collect::<Vec<_>>();
            let generic = exact_vertices_with_incidence(&exact_duals);
            let optimized = exact_binary64_polytope_geometry(&dual_vertices);
            match generic {
                Ok(generic) => {
                    let optimized = optimized.expect("optimized constructor rejected valid input");
                    assert_eq!(
                        sorted_geometry_rows(
                            &optimized.primal_vertices_exact,
                            &optimized.vertex_facet_incidence
                        ),
                        sorted_geometry_rows(&generic.vertices, &generic.vertex_facet_incidence)
                    );
                    valid_cases += 1;
                }
                Err(error) => assert_eq!(
                    optimized,
                    Err(PolytopeGeometryError4d::InvalidExactPolytope(error))
                ),
            }
        }
        assert!(
            valid_cases > 0,
            "generated comparison exercised no valid input"
        );
    }
}
