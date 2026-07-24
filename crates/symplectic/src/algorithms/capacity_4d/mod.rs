//! Certified scalar EHZ capacities of validated four-dimensional polytopes.
//!
//! This API is separate from [`crate::algorithms::orbit_search`]: it returns a
//! scalar certificate and does not promise all minimizing or near-minimizing
//! orbit branches.
//!
//! Call [`CapacityInput4d::try_from_dual_vertices`] once, then call
//! [`CapacityInput4d::capacity`] for exact product dispatch and the certified
//! general fallback. Product results include an exact dyadic-rational value;
//! general results include outward binary64 bounds.

mod general;
mod input;
mod product;

pub use input::{
    CapacityInput4d, CapacityInputError, MAX_INPUT_FACETS, MAX_INPUT_NORM_INF, MIN_INPUT_NORM_INF,
};

use crate::algorithms::hk2017::SimpleDirectedCyclesCanonical;
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
pub struct ProductCapacityWinner4d {
    sigma: Vec<usize>,
    beta_exact: Vec<BigRational>,
}

impl ProductCapacityWinner4d {
    pub fn sigma(&self) -> &[usize] {
        &self.sigma
    }

    pub fn beta_exact(&self) -> &[BigRational] {
        &self.beta_exact
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductCapacity4d {
    capacity_exact: BigRational,
    bounds: CapacityBounds4d,
    winners: Vec<ProductCapacityWinner4d>,
}

impl ProductCapacity4d {
    pub fn capacity_exact(&self) -> &BigRational {
        &self.capacity_exact
    }

    pub fn bounds(&self) -> CapacityBounds4d {
        self.bounds
    }

    pub fn winners(&self) -> &[ProductCapacityWinner4d] {
        &self.winners
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
    NoPositiveGeneralCandidate,
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
            Self::NoPositiveGeneralCandidate => {
                formatter.write_str("validated general route found no positive capacity candidate")
            }
        }
    }
}

impl std::error::Error for CapacityError4d {}

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
        let words =
            SimpleDirectedCyclesCanonical::new(&self.transition_is_allowed).collect::<Vec<_>>();
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
            winners: report
                .winners
                .into_iter()
                .map(|winner| ProductCapacityWinner4d {
                    sigma: winner.sigma,
                    beta_exact: winner.beta_exact,
                })
                .collect(),
        })
    }
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
