/// Convex polytope in R^4 via halfspace representation.
///
/// See `[def:polytope]` in thesis/general-case-algorithm.tex:
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
/// - No two normals are (near-)identical: `‖n_i - n_j‖ > ε` for i ≠ j
/// - **Bounded**: normals positively span R^4 (checked via O(F³) kernel enumeration)
/// - **Irredundant**: every facet has incident vertices of affine rank 3
/// - Vertices are precomputed via exact rational arithmetic from the H-representation
use nalgebra::Vector4;

/// Tolerance for unit-normal check: |(‖n‖ - 1)| < EPS_UNIT.
///
/// **Why 1e-9:** nalgebra's `normalize()` achieves ~1e-15 relative error on f64.
/// The 1e-9 threshold is conservative (6 orders above typical error), catching
/// genuinely un-normalized inputs while allowing standard numerical noise.
const EPS_UNIT: f64 = 1e-9;

/// Tolerance for duplicate-normal detection: ‖n_i - n_j‖ < EPS_DUPLICATE_NORMAL.
///
/// **Why 1e-8:** Slightly looser than EPS_UNIT (1e-9) because two normals from
/// different constructions may accumulate rounding independently. Tight enough
/// to catch copy-paste duplicates, loose enough to avoid false positives on
/// normals that differ by O(1e-10) from separate trigonometric computations.
const EPS_DUPLICATE_NORMAL: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Polytope4D {
    normals: Vec<Vector4<f64>>,
    heights: Vec<f64>,
    vertices: Vec<Vector4<f64>>,
    /// Exact combinatorial data from the rational pipeline.
    ///
    /// This is the authoritative source for discrete decisions (vertex-facet
    /// incidence, ω₀ signs, adjacency). Algorithms use this instead of
    /// recomputing from f64 with tolerances.
    ///
    /// Computed at construction time via [`RationalPolytope4D::from_f64`].
    exact_data: super::rational::CombinatorialData,
}

/// Errors from [`Polytope4D::new()`] when invariants are violated.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    LengthMismatch { normals: usize, heights: usize },
    TooFewFacets(usize),
    NonUnitNormal { index: usize, norm: f64 },
    NonPositiveHeight { index: usize, value: f64 },
    DuplicateHalfspaces { i: usize, j: usize },
    Unbounded,
    VertexEnumerationFailed(String),
    RedundantFacet(usize),
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
            Self::Unbounded => write!(f, "polytope is unbounded (normals do not positively span R^4)"),
            Self::VertexEnumerationFailed(msg) => {
                write!(f, "vertex enumeration failed: {msg}")
            }
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant (incident vertices have affine rank < 3)"),
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
            if h <= 0.0 || !h.is_finite() { // also rejects NaN and infinity
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

        // Boundedness: normals must positively span R^4
        if !crate::geom::validation::check_bounded(&normals) {
            return Err(ConstructionError::Unbounded);
        }

        // Compute exact combinatorial data via rational pipeline.
        // from_f64 reinterprets the f64 normals/heights as exact rationals (lossless),
        // then computes exact vertices, incidence, adjacency, and ω₀ signs over Q.
        // Also checks irredundancy exactly.
        //
        // TODO(perf): This is the dominant cost of Polytope4D::new() — for F=16 it
        // solves C(16,4) = 1820 exact rational linear systems via Cramer's rule.
        // Causes ~12x slowdown in test suite vs main. The hot path is det4()/solve4()
        // in rational.rs. Possible approaches: (1) sparse Cramer — skip C(F,4) subsets
        // known to be infeasible via f64 pre-filter, (2) incremental determinant updates,
        // (3) exact integer arithmetic instead of BigRational.
        let rp = super::rational::RationalPolytope4D::from_f64(&normals, &heights)
            .map_err(|e| match e {
                super::rational::RationalConstructionError::RedundantFacet(i) => {
                    ConstructionError::RedundantFacet(i)
                }
                other => ConstructionError::VertexEnumerationFailed(
                    format!("rational pipeline failed: {other}")
                ),
            })?;

        // Vertices come from the rational pipeline, converted to f64.
        // No reordering needed: vertices_to_f64() preserves the rational pipeline's
        // lexicographic vertex descriptor ordering.
        let vertices = rp.vertices_to_f64();

        Ok(Self {
            normals,
            heights,
            vertices,
            exact_data: rp.combinatorial_data().clone(),
        })
    }

    /// Outward unit normal vectors (n̂ᵢ ∈ S³) defining the halfspaces.
    pub fn normals(&self) -> &[Vector4<f64>] {
        &self.normals
    }

    /// Positive heights (ĥᵢ > 0) defining the halfspaces n̂ᵢ · x ≤ ĥᵢ.
    pub fn heights(&self) -> &[f64] {
        &self.heights
    }

    /// Vertices of the polytope, computed exactly via rational arithmetic at construction time.
    pub fn vertices(&self) -> &[Vector4<f64>] {
        &self.vertices
    }

    /// Number of facets F = number of halfspaces in the H-representation.
    pub fn facet_count(&self) -> usize {
        self.normals.len()
    }

    /// Exact combinatorial data from the rational pipeline.
    ///
    /// Always available — computed at construction time for every `Polytope4D`.
    /// This is the authoritative source for vertex-facet incidence, facet
    /// adjacency, and ω₀ sign decisions.
    pub fn exact_data(&self) -> &super::rational::CombinatorialData {
        &self.exact_data
    }

    /// Construct from a [`RationalPolytope4D`](super::rational::RationalPolytope4D).
    ///
    /// Converts rational normals to f64 unit vectors and attaches the exact
    /// combinatorial data from `rp`. The result carries:
    /// - f64 unit normals, heights, vertices (for KKT solver, numerics)
    /// - Exact incidence, adjacency, ω₀ signs (for discrete decisions)
    ///
    /// Prefer this over `new()` when you already have a `RationalPolytope4D`,
    /// since `new()` would redundantly recompute the rational representation.
    pub fn from_rational(
        rp: &super::rational::RationalPolytope4D,
    ) -> Result<Self, ConstructionError> {
        rp.to_f64_with_data().map_err(|e| {
            ConstructionError::VertexEnumerationFailed(format!("rational→f64 failed: {e}"))
        })
    }

    /// Private constructor that validates f64 invariants and attaches
    /// pre-computed exact combinatorial data, skipping the rational pipeline.
    ///
    /// Used by `RationalPolytope4D::to_f64_with_data()` to avoid redundantly
    /// recomputing the rational pipeline that `new()` would trigger.
    ///
    /// `vertices` must be in the same order as `exact_data.vertex_descriptors`.
    pub(super) fn new_with_exact_data(
        normals: Vec<Vector4<f64>>,
        heights: Vec<f64>,
        vertices: Vec<Vector4<f64>>,
        exact_data: super::rational::CombinatorialData,
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

        let f = normals.len();
        for i in 0..f {
            for j in (i + 1)..f {
                if (normals[i] - normals[j]).norm() < EPS_DUPLICATE_NORMAL {
                    return Err(ConstructionError::DuplicateHalfspaces { i, j });
                }
            }
        }

        if !crate::geom::validation::check_bounded(&normals) {
            return Err(ConstructionError::Unbounded);
        }

        debug_assert_eq!(
            exact_data.vertex_descriptors.len(), vertices.len(),
            "new_with_exact_data: vertex count mismatch: exact_data={}, vertices={}",
            exact_data.vertex_descriptors.len(), vertices.len()
        );

        Ok(Self {
            normals,
            heights,
            vertices,
            exact_data,
        })
    }
}

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;
