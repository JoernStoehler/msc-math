/// Convex polytope in R^4 via halfspace representation.
///
/// K = { x ∈ R^4 | n_i · x ≤ h_i for all i = 1, ..., F }
///
/// # Invariants (enforced by constructor)
///
/// - `normals.len() == heights.len() >= 5` (minimum facets for a bounded 4D polytope)
/// - All normals are unit vectors: `‖n_i‖ = 1`
/// - All heights are strictly positive: `h_i > 0` (origin is in the interior)
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Polytope4D {
    normals: Vec<Vector4<f64>>,
    heights: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    LengthMismatch { normals: usize, heights: usize },
    TooFewFacets(usize),
    NonUnitNormal { index: usize, norm: f64 },
    NonPositiveHeight { index: usize, value: f64 },
}

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch { normals, heights } => {
                write!(f, "length mismatch: {normals} normals vs {heights} heights")
            }
            Self::TooFewFacets(n) => write!(f, "need ≥5 facets, got {n}"),
            Self::NonUnitNormal { index, norm } => {
                write!(f, "normal[{index}] has norm {norm}, expected 1.0")
            }
            Self::NonPositiveHeight { index, value } => {
                write!(f, "height[{index}] = {value}, expected > 0")
            }
        }
    }
}

impl Polytope4D {
    /// Construct a polytope from outward unit normals and positive heights.
    pub fn new(
        normals: Vec<Vector4<f64>>,
        heights: Vec<f64>,
    ) -> Result<Self, ConstructionError> {
        if normals.len() != heights.len() {
            return Err(ConstructionError::LengthMismatch {
                normals: normals.len(),
                heights: heights.len(),
            });
        }
        if normals.len() < 5 {
            return Err(ConstructionError::TooFewFacets(normals.len()));
        }
        for (i, n) in normals.iter().enumerate() {
            let norm = n.norm();
            if (norm - 1.0).abs() > 1e-9 {
                return Err(ConstructionError::NonUnitNormal { index: i, norm });
            }
        }
        for (i, &h) in heights.iter().enumerate() {
            if h <= 0.0 || !h.is_finite() {
                return Err(ConstructionError::NonPositiveHeight { index: i, value: h });
            }
        }
        Ok(Self { normals, heights })
    }

    pub fn normals(&self) -> &[Vector4<f64>] {
        &self.normals
    }

    pub fn heights(&self) -> &[f64] {
        &self.heights
    }

    pub fn facet_count(&self) -> usize {
        self.normals.len()
    }
}

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;
