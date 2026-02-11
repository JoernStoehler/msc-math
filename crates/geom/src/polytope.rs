/// Convex polytope in R^4 via halfspace representation.
///
/// See Definition 3.2 (Polytope) in thesis/chapter-algorithm.tex:
///   A polytope is a bounded convex subset K ⊂ ℝ⁴ that contains 0 in its
///   interior and is an intersection of finitely many closed half-spaces.
///
/// K = { x ∈ R^4 | n_i · x ≤ h_i for all i = 1, ..., F }
///
/// # Invariants (enforced by constructor)
///
/// - `normals.len() == heights.len() >= 5` (minimum facets for a bounded 4D polytope)
/// - All normals are unit vectors: `‖n_i‖ = 1` (n_i ∈ S³)
/// - All heights are strictly positive: `h_i > 0`
/// - No two normals are (near-)identical: `‖n_i - n_j‖ > ε` for i ≠ j (irredundancy)
/// - Vertices are precomputed via qhull from the H-representation
///
/// Note: h_i > 0 ensures 0 ∈ int(K) but does NOT imply boundedness.
/// Boundedness is part of the polytope definition itself (Definition 3.2).
/// Qhull checks boundedness during vertex enumeration.
use nalgebra::Vector4;

const EPS_UNIT: f64 = 1e-9;
const EPS_DUPLICATE_NORMAL: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Polytope4D {
    normals: Vec<Vector4<f64>>,
    heights: Vec<f64>,
    vertices: Vec<Vector4<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    LengthMismatch { normals: usize, heights: usize },
    TooFewFacets(usize),
    NonUnitNormal { index: usize, norm: f64 },
    NonPositiveHeight { index: usize, value: f64 },
    DuplicateHalfspaces { i: usize, j: usize },
    VertexEnumerationFailed(String),
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
            Self::DuplicateHalfspaces { i, j } => {
                write!(f, "normals[{i}] and normals[{j}] are duplicates")
            }
            Self::VertexEnumerationFailed(msg) => {
                write!(f, "vertex enumeration failed: {msg}")
            }
        }
    }
}

impl Polytope4D {
    /// Construct a polytope from outward unit normals and positive heights.
    ///
    /// Validates all invariants and precomputes vertices.
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
            if (norm - 1.0).abs() > EPS_UNIT {
                return Err(ConstructionError::NonUnitNormal { index: i, norm });
            }
        }
        for (i, &h) in heights.iter().enumerate() {
            if h <= 0.0 || !h.is_finite() {
                return Err(ConstructionError::NonPositiveHeight { index: i, value: h });
            }
        }

        // Check no two normals are (near-)identical
        let f = normals.len();
        for i in 0..f {
            for j in (i + 1)..f {
                if (normals[i] - normals[j]).norm() < EPS_DUPLICATE_NORMAL {
                    return Err(ConstructionError::DuplicateHalfspaces { i, j });
                }
            }
        }

        let vertices = crate::vertices::compute_vertices(&normals, &heights)
            .map_err(|e| ConstructionError::VertexEnumerationFailed(e.to_string()))?;

        Ok(Self {
            normals,
            heights,
            vertices,
        })
    }

    pub fn normals(&self) -> &[Vector4<f64>] {
        &self.normals
    }

    pub fn heights(&self) -> &[f64] {
        &self.heights
    }

    pub fn vertices(&self) -> &[Vector4<f64>] {
        &self.vertices
    }

    pub fn facet_count(&self) -> usize {
        self.normals.len()
    }
}

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;
