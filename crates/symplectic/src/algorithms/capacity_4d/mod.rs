//! Certified EHZ capacities and exact minimizing words of validated
//! four-dimensional polytopes.
//!
//! This API is separate from [`crate::algorithms::orbit_search`]. Scalar
//! functions return only the capacity certificate. [`qp_minimizers`]
//! additionally exact-resolves every tied minimizing word in its stated finite
//! candidate family. [`general_qp_action_window`] returns one full exact KKT
//! witness for every general-HK word within an exact capacity multiple, and
//! [`solve_sigma_exact`] obtains the full exact KKT payload for one requested
//! word.
//!
//! Most callers should use [`capacity_from_dual_vertices`]. Callers that need
//! minimizing words should use [`qp_minimizers_from_dual_vertices`]. Callers
//! that need intermediate geometry can instead use
//! [`exact_binary64_polytope_geometry`], the explicit input checks, and then
//! [`capacity`] or [`qp_minimizers`]. Product results include an exact
//! dyadic-rational value; general results include outward binary64 bounds.

mod general;
mod geometry;
mod product;

pub use geometry::{
    capacity_transition_graph, check_dual_vertex_norm_bounds, check_facet_count,
    check_finite_dual_vertices, check_primal_vertex_norm_bounds, classify_lagrangian_product,
    exact_binary64_polytope_geometry, CapacityInputBoundsError4d, PolytopeGeometry4d,
    PolytopeGeometryError4d, MAX_INPUT_FACETS, MAX_INPUT_NORM_INF, MIN_INPUT_NORM_INF,
};

use crate::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use crate::exact::{solve_orbit_sigma_exact_rational, ExactOrbitKktData};
use geometry::check_vertex_norm_bounds;
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;

/// Maximum general-route candidate count accepted before materialization.
pub const MAX_GENERAL_CANDIDATES: usize = 100_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapacityBounds4d {
    lower: f64,
    upper: f64,
}

impl CapacityBounds4d {
    pub fn lower(self) -> f64 {
        self.lower
    }

    pub fn upper(self) -> f64 {
        self.upper
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneralCapacity4d {
    bounds: CapacityBounds4d,
}

impl GeneralCapacity4d {
    pub fn bounds(&self) -> CapacityBounds4d {
        self.bounds
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductCapacity4d {
    capacity_exact: BigRational,
    bounds: CapacityBounds4d,
}

impl ProductCapacity4d {
    pub fn capacity_exact(&self) -> &BigRational {
        &self.capacity_exact
    }

    pub fn bounds(&self) -> CapacityBounds4d {
        self.bounds
    }
}

/// The finite candidate family certified by a QP minimizer search.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QpCandidateFamily4d {
    /// Simple cyclic HK words from the complete transition-pruned general
    /// candidate family.
    GeneralHk,
    /// Sparse closure-vertex words from the product six-facet theorem.
    ///
    /// This is not a claim to enumerate every physical orbit in a degenerate
    /// within-word solution family.
    ProductClosureVertex,
}

/// One exactly admissible word selected by a production QP request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertifiedQpCandidate4d {
    sigma: Vec<usize>,
    action_exact: BigRational,
}

impl CertifiedQpCandidate4d {
    pub fn sigma(&self) -> &[usize] {
        &self.sigma
    }

    pub fn action_exact(&self) -> &BigRational {
        &self.action_exact
    }
}

/// Every tied minimizing word in one stated finite candidate family.
#[derive(Clone, Debug, PartialEq)]
pub struct QpMinimizers4d {
    family: QpCandidateFamily4d,
    bounds: CapacityBounds4d,
    candidates: Vec<CertifiedQpCandidate4d>,
}

impl QpMinimizers4d {
    pub fn family(&self) -> QpCandidateFamily4d {
        self.family
    }

    pub fn bounds(&self) -> CapacityBounds4d {
        self.bounds
    }

    pub fn candidates(&self) -> &[CertifiedQpCandidate4d] {
        &self.candidates
    }
}

/// Every exactly admissible general-HK word whose action is at most a
/// caller-supplied exact multiple of capacity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QpActionWindow4d {
    family: QpCandidateFamily4d,
    capacity_exact: BigRational,
    maximum_action_multiple: BigRational,
    witnesses: Vec<ExactOrbitKktData<BigRational>>,
}

impl QpActionWindow4d {
    pub fn family(&self) -> QpCandidateFamily4d {
        self.family
    }

    pub fn capacity_exact(&self) -> &BigRational {
        &self.capacity_exact
    }

    pub fn maximum_action_multiple(&self) -> &BigRational {
        &self.maximum_action_multiple
    }

    /// One exact positive KKT witness for each returned discrete word.
    ///
    /// A rank-deficient word may have more than one positive solution; this
    /// result selects one witness and does not parameterize that family.
    pub fn witnesses(&self) -> &[ExactOrbitKktData<BigRational>] {
        &self.witnesses
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Capacity4d {
    General(GeneralCapacity4d),
    Product(ProductCapacity4d),
}

impl Capacity4d {
    pub fn bounds(&self) -> CapacityBounds4d {
        match self {
            Self::General(result) => result.bounds(),
            Self::Product(result) => result.bounds(),
        }
    }

    pub fn route_name(&self) -> &'static str {
        match self {
            Self::General(_) => "general",
            Self::Product(_) => "product",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CapacityValueError4d {
    InvalidRelativeTolerance,
    InvalidCapacityBounds,
    BoundsTooWide {
        certified_relative_error: f64,
        maximum_relative_error: f64,
    },
}

impl std::fmt::Display for CapacityValueError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRelativeTolerance => {
                formatter.write_str("relative tolerance must be finite and nonnegative")
            }
            Self::InvalidCapacityBounds => {
                formatter.write_str("capacity bounds must be finite, positive, and ordered")
            }
            Self::BoundsTooWide {
                certified_relative_error,
                maximum_relative_error,
            } => write!(
                formatter,
                "certified relative error {certified_relative_error:e} exceeds requested maximum {maximum_relative_error:e}"
            ),
        }
    }
}

impl std::error::Error for CapacityValueError4d {}

/// Return a representative capacity only when its relative error is certified.
///
/// If the exact capacity `c` lies in `[lower, upper]`, the returned midpoint
/// `value` satisfies
///
/// `|value - c| / c <= certified_relative_error <= maximum_relative_error`.
///
/// The computed error bound is rounded outward.
pub fn capacity_value(
    capacity: &Capacity4d,
    maximum_relative_error: f64,
) -> Result<f64, CapacityValueError4d> {
    if !maximum_relative_error.is_finite() || maximum_relative_error < 0.0 {
        return Err(CapacityValueError4d::InvalidRelativeTolerance);
    }
    let bounds = capacity.bounds();
    if !bounds.lower.is_finite()
        || !bounds.upper.is_finite()
        || bounds.lower <= 0.0
        || bounds.lower > bounds.upper
    {
        return Err(CapacityValueError4d::InvalidCapacityBounds);
    }

    let value = bounds.lower + (bounds.upper - bounds.lower) * 0.5;
    let lower_distance = value - bounds.lower;
    let upper_distance = bounds.upper - value;
    let maximum_distance = lower_distance.max(upper_distance);
    let certified_relative_error = if maximum_distance == 0.0 {
        0.0
    } else {
        next_up(next_up(maximum_distance) / bounds.lower)
    };
    if certified_relative_error > maximum_relative_error {
        return Err(CapacityValueError4d::BoundsTooWide {
            certified_relative_error,
            maximum_relative_error,
        });
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapacityError4d {
    ProductRouteRequiresStructuralProduct,
    ProductRouteFailed(ProductRouteFailure4d),
    InvalidMaximumActionMultiple,
    GeneralCandidateLimitExceeded { limit: usize },
    NoPositiveGeneralCandidate,
    GeneralExactContenderResolutionFailed { sigma: Vec<usize> },
}

impl std::fmt::Display for CapacityError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductRouteRequiresStructuralProduct => {
                formatter.write_str("the product route requires exact structural q/p blocks")
            }
            Self::ProductRouteFailed(error) => {
                write!(
                    formatter,
                    "validated product-route invariant failed: {error:?}"
                )
            }
            Self::InvalidMaximumActionMultiple => {
                formatter.write_str("maximum action multiple must be at least one")
            }
            Self::GeneralCandidateLimitExceeded { limit } => write!(
                formatter,
                "forced general route has more than the supported {limit} candidate cycles"
            ),
            Self::NoPositiveGeneralCandidate => {
                formatter.write_str("validated general route found no positive capacity candidate")
            }
            Self::GeneralExactContenderResolutionFailed { sigma } => write!(
                formatter,
                "certified general contender {sigma:?} failed exact KKT resolution"
            ),
        }
    }
}

impl std::error::Error for CapacityError4d {}

/// Errors from the one-shot [`capacity_from_dual_vertices`] convenience API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapacityFromDualVerticesError4d {
    Geometry(PolytopeGeometryError4d),
    InputBounds(CapacityInputBoundsError4d),
    Capacity(CapacityError4d),
}

impl std::fmt::Display for CapacityFromDualVerticesError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Geometry(error) => write!(formatter, "{error}"),
            Self::InputBounds(error) => write!(formatter, "{error}"),
            Self::Capacity(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for CapacityFromDualVerticesError4d {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExactSigmaInputError4d {
    Empty,
    FacetOutOfRange {
        position: usize,
        facet: usize,
        facet_count: usize,
    },
    RepeatedFacet {
        facet: usize,
    },
}

impl std::fmt::Display for ExactSigmaInputError4d {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => formatter.write_str("sigma must contain at least one facet"),
            Self::FacetOutOfRange {
                position,
                facet,
                facet_count,
            } => write!(
                formatter,
                "sigma position {position} uses facet {facet}, but the input has {facet_count} facets"
            ),
            Self::RepeatedFacet { facet } => {
                write!(formatter, "sigma repeats facet {facet}")
            }
        }
    }
}

impl std::error::Error for ExactSigmaInputError4d {}

/// A failure of the product route after shared input validation succeeded.
///
/// These cases indicate a violated implementation or mathematical invariant,
/// rather than a recoverable input-validation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductRouteFailure4d {
    InvalidValidatedFacet { facet: usize },
    TooFewFacets { factor: &'static str, count: usize },
    NoClosureVertex { factor: &'static str },
    NoPositiveCandidate,
    InternalSupportMismatch,
}

impl From<product::ProductClosureError> for ProductRouteFailure4d {
    fn from(error: product::ProductClosureError) -> Self {
        match error {
            product::ProductClosureError::NonFiniteFacet { facet }
            | product::ProductClosureError::NotStructuralProduct { facet }
            | product::ProductClosureError::ZeroFacet { facet } => {
                Self::InvalidValidatedFacet { facet }
            }
            product::ProductClosureError::TooFewFacets { factor, count } => {
                Self::TooFewFacets { factor, count }
            }
            product::ProductClosureError::NoClosureVertex { factor } => {
                Self::NoClosureVertex { factor }
            }
            product::ProductClosureError::NoPositiveCandidate => Self::NoPositiveCandidate,
            product::ProductClosureError::InternalSupportMismatch => Self::InternalSupportMismatch,
        }
    }
}

/// Validate exact binary64 geometry and numerical-size bounds, then compute the
/// scalar EHZ capacity using exact product dispatch.
pub fn capacity_from_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Result<Capacity4d, CapacityFromDualVerticesError4d> {
    let geometry = checked_geometry_from_dual_vertices(dual_vertices)?;
    capacity_assuming_checked(&geometry).map_err(CapacityFromDualVerticesError4d::Capacity)
}

/// Validate exact binary64 geometry and numerical-size bounds, then return
/// every tied minimizing word in the automatically selected candidate family.
pub fn qp_minimizers_from_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Result<QpMinimizers4d, CapacityFromDualVerticesError4d> {
    let geometry = checked_geometry_from_dual_vertices(dual_vertices)?;
    qp_minimizers_assuming_checked(&geometry).map_err(CapacityFromDualVerticesError4d::Capacity)
}

fn checked_geometry_from_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Result<PolytopeGeometry4d, CapacityFromDualVerticesError4d> {
    check_facet_count(dual_vertices.len()).map_err(CapacityFromDualVerticesError4d::InputBounds)?;
    check_finite_dual_vertices(dual_vertices).map_err(CapacityFromDualVerticesError4d::Geometry)?;
    check_dual_vertex_norm_bounds(dual_vertices)
        .map_err(CapacityFromDualVerticesError4d::InputBounds)?;
    let geometry = exact_binary64_polytope_geometry(dual_vertices)
        .map_err(CapacityFromDualVerticesError4d::Geometry)?;
    check_primal_vertex_norm_bounds(&geometry)
        .map_err(CapacityFromDualVerticesError4d::InputBounds)?;
    Ok(geometry)
}

/// Compute the scalar EHZ capacity of checked exact binary64 geometry.
///
/// Call [`check_facet_count`], [`check_dual_vertex_norm_bounds`], and
/// [`check_primal_vertex_norm_bounds`] first. Skipping those checks is a caller
/// bug and panics.
pub fn capacity(geometry: &PolytopeGeometry4d) -> Result<Capacity4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    capacity_assuming_checked(geometry)
}

fn capacity_assuming_checked(geometry: &PolytopeGeometry4d) -> Result<Capacity4d, CapacityError4d> {
    if classify_lagrangian_product(geometry).is_some() {
        product_capacity_assuming_checked(geometry).map(Capacity4d::Product)
    } else {
        general_capacity_assuming_checked(geometry).map(Capacity4d::General)
    }
}

/// Force the certified general route, including for structural products.
///
/// The production input checks required by [`capacity`] also apply here.
pub fn general_capacity(
    geometry: &PolytopeGeometry4d,
) -> Result<GeneralCapacity4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    general_capacity_assuming_checked(geometry)
}

fn general_capacity_assuming_checked(
    geometry: &PolytopeGeometry4d,
) -> Result<GeneralCapacity4d, CapacityError4d> {
    let words = general_words(geometry)?;
    let (lower, upper) = general::solve_selected_general(&geometry.dual_vertices, words)
        .ok_or(CapacityError4d::NoPositiveGeneralCandidate)?;
    Ok(GeneralCapacity4d {
        bounds: CapacityBounds4d { lower, upper },
    })
}

/// Use the KKT-free product route.
///
/// The production input checks required by [`capacity`] also apply here. Returns
/// [`CapacityError4d::ProductRouteRequiresStructuralProduct`] when exact
/// binary64-rational classification did not establish a q/p product.
pub fn product_capacity(
    geometry: &PolytopeGeometry4d,
) -> Result<ProductCapacity4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    product_capacity_assuming_checked(geometry)
}

fn product_capacity_assuming_checked(
    geometry: &PolytopeGeometry4d,
) -> Result<ProductCapacity4d, CapacityError4d> {
    if classify_lagrangian_product(geometry).is_none() {
        return Err(CapacityError4d::ProductRouteRequiresStructuralProduct);
    }
    let report = product::solve_product_closure_capacity_hybrid(&geometry.dual_vertices)
        .map_err(|error| CapacityError4d::ProductRouteFailed(error.into()))?;
    Ok(ProductCapacity4d {
        bounds: rational_bounds(&report.capacity_exact),
        capacity_exact: report.capacity_exact,
    })
}

/// Return every tied minimizing word in the automatically selected finite
/// candidate family.
pub fn qp_minimizers(geometry: &PolytopeGeometry4d) -> Result<QpMinimizers4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    qp_minimizers_assuming_checked(geometry)
}

fn qp_minimizers_assuming_checked(
    geometry: &PolytopeGeometry4d,
) -> Result<QpMinimizers4d, CapacityError4d> {
    if classify_lagrangian_product(geometry).is_some() {
        product_qp_minimizers_assuming_checked(geometry)
    } else {
        general_qp_minimizers_assuming_checked(geometry)
    }
}

/// Force exact minimizer materialization from the general HK family.
pub fn general_qp_minimizers(
    geometry: &PolytopeGeometry4d,
) -> Result<QpMinimizers4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    general_qp_minimizers_assuming_checked(geometry)
}

fn general_qp_minimizers_assuming_checked(
    geometry: &PolytopeGeometry4d,
) -> Result<QpMinimizers4d, CapacityError4d> {
    let words = general_words(geometry)?;
    let report = general::solve_selected_general_minimizers(&geometry.dual_vertices, words)
        .map_err(|sigma| CapacityError4d::GeneralExactContenderResolutionFailed { sigma })?
        .ok_or(CapacityError4d::NoPositiveGeneralCandidate)?;
    let selection = report
        .exact_selection
        .expect("the rich general request materializes exact candidates");
    debug_assert!(selection
        .candidates
        .iter()
        .all(|candidate| candidate.witness.action() == selection.capacity_exact));
    Ok(QpMinimizers4d {
        family: QpCandidateFamily4d::GeneralHk,
        bounds: CapacityBounds4d {
            lower: report.bounds.0,
            upper: report.bounds.1,
        },
        candidates: selection
            .candidates
            .into_iter()
            .map(|candidate| {
                let action_exact = candidate.witness.action();
                CertifiedQpCandidate4d {
                    sigma: candidate.witness.sigma,
                    action_exact,
                }
            })
            .collect(),
    })
}

/// Return every exactly admissible general-HK word whose exact action is at
/// most `maximum_action_multiple * capacity`.
///
/// The multiple is exact, inclusive, and must be at least one. This forces the
/// complete transition-pruned general candidate family even when the input is
/// a structural product.
pub fn general_qp_action_window(
    geometry: &PolytopeGeometry4d,
    maximum_action_multiple: BigRational,
) -> Result<QpActionWindow4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    if maximum_action_multiple < BigRational::from_integer(1.into()) {
        return Err(CapacityError4d::InvalidMaximumActionMultiple);
    }
    let words = general_words(geometry)?;
    let report = general::solve_selected_general_action_window(
        &geometry.dual_vertices,
        words,
        maximum_action_multiple.clone(),
    )
    .map_err(|sigma| CapacityError4d::GeneralExactContenderResolutionFailed { sigma })?
    .ok_or(CapacityError4d::NoPositiveGeneralCandidate)?;
    let selection = report
        .exact_selection
        .expect("the action-window request materializes exact candidates");
    Ok(QpActionWindow4d {
        family: QpCandidateFamily4d::GeneralHk,
        capacity_exact: selection.capacity_exact,
        maximum_action_multiple,
        witnesses: selection
            .candidates
            .into_iter()
            .map(|candidate| candidate.witness)
            .collect(),
    })
}

/// Return sparse exact closure-vertex minimizers for a structural product.
pub fn product_qp_minimizers(
    geometry: &PolytopeGeometry4d,
) -> Result<QpMinimizers4d, CapacityError4d> {
    assert_capacity_input_bounds(geometry);
    product_qp_minimizers_assuming_checked(geometry)
}

fn product_qp_minimizers_assuming_checked(
    geometry: &PolytopeGeometry4d,
) -> Result<QpMinimizers4d, CapacityError4d> {
    if classify_lagrangian_product(geometry).is_none() {
        return Err(CapacityError4d::ProductRouteRequiresStructuralProduct);
    }
    let report = product::solve_product_closure_capacity_hybrid(&geometry.dual_vertices)
        .map_err(|error| CapacityError4d::ProductRouteFailed(error.into()))?;
    let bounds = rational_bounds(&report.capacity_exact);
    let action_exact = report.capacity_exact;
    Ok(QpMinimizers4d {
        family: QpCandidateFamily4d::ProductClosureVertex,
        bounds,
        candidates: report
            .winners
            .into_iter()
            .map(|winner| CertifiedQpCandidate4d {
                sigma: winner.sigma,
                action_exact: action_exact.clone(),
            })
            .collect(),
    })
}

/// Solve the exact KKT system for one caller-supplied word.
///
/// `Ok(None)` means the valid word has no solution with strictly positive beta
/// and q. Invalid facet indices or repeated facets are soft input errors.
/// Numerical-size bounds are irrelevant because this function is exact.
pub fn solve_sigma_exact(
    geometry: &PolytopeGeometry4d,
    sigma: &[usize],
) -> Result<Option<ExactOrbitKktData<BigRational>>, ExactSigmaInputError4d> {
    validate_sigma(sigma, geometry.dual_vertices.len())?;
    Ok(solve_orbit_sigma_exact_rational(
        &geometry.dual_vertices_exact,
        sigma,
    ))
}

fn general_words(geometry: &PolytopeGeometry4d) -> Result<Vec<Vec<usize>>, CapacityError4d> {
    let transition_is_allowed = capacity_transition_graph(geometry);
    let words = SimpleDirectedCyclesCanonical::new(&transition_is_allowed)
        .take(MAX_GENERAL_CANDIDATES + 1)
        .collect::<Vec<_>>();
    if words.len() > MAX_GENERAL_CANDIDATES {
        return Err(CapacityError4d::GeneralCandidateLimitExceeded {
            limit: MAX_GENERAL_CANDIDATES,
        });
    }
    Ok(words)
}

fn assert_capacity_input_bounds(geometry: &PolytopeGeometry4d) {
    assert!(
        check_facet_count(geometry.dual_vertices.len()).is_ok(),
        "capacity_4d requires a successful check_facet_count call"
    );
    assert!(
        check_vertex_norm_bounds(geometry).is_ok(),
        "capacity_4d requires successful dual- and primal-vertex norm checks"
    );
}

fn validate_sigma(sigma: &[usize], facet_count: usize) -> Result<(), ExactSigmaInputError4d> {
    if sigma.is_empty() {
        return Err(ExactSigmaInputError4d::Empty);
    }
    let mut seen = vec![false; facet_count];
    for (position, &facet) in sigma.iter().enumerate() {
        if facet >= facet_count {
            return Err(ExactSigmaInputError4d::FacetOutOfRange {
                position,
                facet,
                facet_count,
            });
        }
        if std::mem::replace(&mut seen[facet], true) {
            return Err(ExactSigmaInputError4d::RepeatedFacet { facet });
        }
    }
    Ok(())
}

fn rational_bounds(value: &BigRational) -> CapacityBounds4d {
    let rounded = value.to_f64().unwrap_or(f64::INFINITY);
    if !rounded.is_finite() {
        return CapacityBounds4d {
            lower: 0.0,
            upper: f64::INFINITY,
        };
    }
    let rounded_exact =
        BigRational::from_float(rounded).expect("a finite binary64 value is rational");
    match rounded_exact.cmp(value) {
        std::cmp::Ordering::Less => CapacityBounds4d {
            lower: rounded,
            upper: next_up(rounded),
        },
        std::cmp::Ordering::Equal => CapacityBounds4d {
            lower: rounded,
            upper: rounded,
        },
        std::cmp::Ordering::Greater => CapacityBounds4d {
            lower: next_down(rounded),
            upper: rounded,
        },
    }
}

fn next_up(value: f64) -> f64 {
    if value == f64::INFINITY {
        return value;
    }
    if value == 0.0 {
        return f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits + 1 } else { bits - 1 })
}

fn next_down(value: f64) -> f64 {
    if value == f64::NEG_INFINITY {
        return value;
    }
    if value == 0.0 {
        return -f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits - 1 } else { bits + 1 })
}
