/// Polytope validation: thin wrapper around `geom::validation` + `Polytope4D::new()`.
///
/// Since `Polytope4D::new()` now enforces all invariants (boundedness, irredundancy),
/// `validate_polytope` is equivalent to constructing via `new()`. This module is
/// retained for API compatibility and to re-export the error type.
use geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

// Re-export geom's validation utilities for tests that call them directly.
pub use geom::validation::{affine_rank, check_bounded, find_redundant_facet, EPS_FEASIBILITY};

/// Why validation failed.
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    Construction(ConstructionError),
    Unbounded,
    RedundantFacet(usize),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Construction(e) => write!(f, "construction: {e}"),
            Self::Unbounded => write!(f, "polytope is unbounded"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
        }
    }
}

impl From<ConstructionError> for ValidationError {
    fn from(e: ConstructionError) -> Self {
        match e {
            ConstructionError::Unbounded => Self::Unbounded,
            ConstructionError::RedundantFacet(i) => Self::RedundantFacet(i),
            other => Self::Construction(other),
        }
    }
}

/// Full validation: returns a `Polytope4D` or an error explaining why the input
/// does not define a valid irredundant bounded polytope.
///
/// Since `Polytope4D::new()` now enforces all invariants, this delegates directly.
pub fn validate_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Polytope4D, ValidationError> {
    let polytope = Polytope4D::new(normals.to_vec(), heights.to_vec())?;
    Ok(polytope)
}

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;
