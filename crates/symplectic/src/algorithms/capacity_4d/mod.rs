//! Certified EHZ capacities and exact minimizing words of validated
//! four-dimensional polytopes.
//!
//! This API is separate from [`crate::algorithms::orbit_search`]. Scalar
//! methods return only the capacity certificate. [`CapacityInput4d::qp_minimizers`]
//! additionally exact-resolves every tied minimizing word in its stated finite
//! candidate family, and [`CapacityInput4d::solve_sigma_exact`] obtains the
//! full exact KKT payload for one requested word.
//!
//! Call [`CapacityInput4d::try_from_dual_vertices`] once, then call
//! [`CapacityInput4d::capacity`] for exact product dispatch and the certified
//! general fallback. Product results include an exact dyadic-rational value;
//! general results include outward binary64 bounds.

mod general;
mod input;
mod product;

pub use input::{
    CapacityInput4d, CapacityInputError, MAX_GENERAL_CANDIDATES, MAX_INPUT_FACETS,
    MAX_INPUT_NORM_INF, MIN_INPUT_NORM_INF,
};

use crate::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use crate::exact::{solve_orbit_sigma_exact_rational, ExactOrbitKktData};
use num_rational::BigRational;
use num_traits::ToPrimitive;

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

/// One exactly admissible minimizing word for the exact binary64 input.
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapacityError4d {
    ProductRouteRequiresStructuralProduct,
    ProductRouteFailed(ProductRouteFailure4d),
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

impl CapacityInput4d {
    /// Compute the scalar EHZ capacity using exact product dispatch.
    pub fn capacity(&self) -> Result<Capacity4d, CapacityError4d> {
        if self.product_facets.is_some() {
            self.product_capacity().map(Capacity4d::Product)
        } else {
            self.general_capacity().map(Capacity4d::General)
        }
    }

    /// Force the certified general route, including for structural products.
    pub fn general_capacity(&self) -> Result<GeneralCapacity4d, CapacityError4d> {
        let words = self.general_words()?;
        let (lower, upper) = general::solve_selected_general(&self.dual_vertices, words)
            .ok_or(CapacityError4d::NoPositiveGeneralCandidate)?;
        Ok(GeneralCapacity4d {
            bounds: CapacityBounds4d { lower, upper },
        })
    }

    /// Use the KKT-free product route.
    ///
    /// Returns [`CapacityError4d::ProductRouteRequiresStructuralProduct`] when
    /// exact binary64-rational classification did not establish a q/p product.
    pub fn product_capacity(&self) -> Result<ProductCapacity4d, CapacityError4d> {
        if self.product_facets.is_none() {
            return Err(CapacityError4d::ProductRouteRequiresStructuralProduct);
        }
        let report = product::solve_product_closure_capacity_hybrid(&self.dual_vertices)
            .map_err(|error| CapacityError4d::ProductRouteFailed(error.into()))?;
        Ok(ProductCapacity4d {
            bounds: rational_bounds(&report.capacity_exact),
            capacity_exact: report.capacity_exact,
        })
    }

    /// Return every tied minimizing word in the automatically selected finite
    /// candidate family.
    pub fn qp_minimizers(&self) -> Result<QpMinimizers4d, CapacityError4d> {
        if self.product_facets.is_some() {
            self.product_qp_minimizers()
        } else {
            self.general_qp_minimizers()
        }
    }

    /// Force exact minimizer materialization from the general HK family.
    pub fn general_qp_minimizers(&self) -> Result<QpMinimizers4d, CapacityError4d> {
        let words = self.general_words()?;
        let report = general::solve_selected_general_minimizers(&self.dual_vertices, words)
            .map_err(|sigma| CapacityError4d::GeneralExactContenderResolutionFailed { sigma })?
            .ok_or(CapacityError4d::NoPositiveGeneralCandidate)?;
        Ok(QpMinimizers4d {
            family: QpCandidateFamily4d::GeneralHk,
            bounds: CapacityBounds4d {
                lower: report.bounds.0,
                upper: report.bounds.1,
            },
            candidates: report
                .minimizers
                .expect("the rich general request materializes minimizers")
                .into_iter()
                .map(|candidate| CertifiedQpCandidate4d {
                    sigma: candidate.sigma,
                    action_exact: candidate.action_exact,
                })
                .collect(),
        })
    }

    /// Return sparse exact closure-vertex minimizers for a structural product.
    pub fn product_qp_minimizers(&self) -> Result<QpMinimizers4d, CapacityError4d> {
        if self.product_facets.is_none() {
            return Err(CapacityError4d::ProductRouteRequiresStructuralProduct);
        }
        let report = product::solve_product_closure_capacity_hybrid(&self.dual_vertices)
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
    /// `Ok(None)` means the valid word has no solution with strictly positive
    /// beta and q. Invalid facet indices or repeated facets are soft input
    /// errors.
    pub fn solve_sigma_exact(
        &self,
        sigma: &[usize],
    ) -> Result<Option<ExactOrbitKktData<BigRational>>, ExactSigmaInputError4d> {
        validate_sigma(sigma, self.dual_vertices.len())?;
        Ok(solve_orbit_sigma_exact_rational(
            &self.dual_vertices_exact,
            sigma,
        ))
    }

    fn general_words(&self) -> Result<Vec<Vec<usize>>, CapacityError4d> {
        let words = SimpleDirectedCyclesCanonical::new(&self.transition_is_allowed)
            .take(MAX_GENERAL_CANDIDATES + 1)
            .collect::<Vec<_>>();
        if words.len() > MAX_GENERAL_CANDIDATES {
            return Err(CapacityError4d::GeneralCandidateLimitExceeded {
                limit: MAX_GENERAL_CANDIDATES,
            });
        }
        Ok(words)
    }
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
