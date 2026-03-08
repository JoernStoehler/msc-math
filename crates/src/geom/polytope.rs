/// Convex polytope K ⊂ R^4 via dual representation.
///
/// K = { x ∈ R^4 | y_i · x ≤ 1 for all i = 1, ..., F }
///
/// where y_i = n_i / h_i ∈ R^4 are the vertices of the polar body K°.
///
/// # Invariants (enforced by constructor)
///
/// - F ≥ 5 (minimum facets for a bounded 4D polytope)
/// - All dual vertices y_i are nonzero
/// - **Bounded**: dual vertices positively span R^4
/// - **Irredundant**: every facet has incident vertices of affine rank 3
/// - Vertices, incidence, adjacency, and ω₀ signs are precomputed exactly over Q
///
/// # Representations
///
/// Exact rational data (dual_vertices, vertices, incidence, adjacency, omega_signs)
/// is the source of truth for all discrete/combinatorial decisions.
/// The f64 data (normals_f64, heights_f64, vertices_f64) is derived for
/// numerical algorithms like the KKT solver.
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;

/// Tolerance for unit-normal check: |(‖n‖ - 1)| < EPS_UNIT.
///
/// **Why 1e-9:** nalgebra's `normalize()` achieves ~1e-15 relative error on f64.
/// The 1e-9 threshold is conservative (6 orders above typical error), catching
/// genuinely un-normalized inputs while allowing standard numerical noise.
const EPS_UNIT: f64 = 1e-9;

/// Tolerance for duplicate-normal detection: ‖n_i - n_j‖ < EPS_DUPLICATE_NORMAL.
///
/// **Why 1e-8:** Slightly looser than EPS_UNIT (1e-9) because two normals from
/// different constructions may accumulate rounding independently.
const EPS_DUPLICATE_NORMAL: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Polytope4D {
    /// Vertices of the polar body K°: y_i = n_i / h_i ∈ R^4.
    /// The halfspace representation is y_i · x ≤ 1.
    dual_vertices: Vec<[BigRational; 4]>,

    /// Vertices of K, computed exactly over Q.
    vertices: Vec<[BigRational; 4]>,

    /// Vertex–facet incidence matrix E ∈ {0,1}^{V×F}.
    /// E[v,f] = true iff vertex v lies on facet f.
    incidence: DMatrix<bool>,

    /// Facet adjacency matrix A ∈ {0,1}^{F×F}.
    /// A[i,k] = true iff facets i and k share a vertex.
    adjacency: DMatrix<bool>,

    /// Symplectic sign matrix ω ∈ {-1,0,+1}^{F×F}, antisymmetric.
    /// ω[i,k] = sign(ω₀(y_i, y_k)). Zero only for non-generic polytopes.
    omega_signs: DMatrix<i8>,

    /// Outward unit normals n̂_i = y_i / |y_i|.
    normals_f64: Vec<Vector4<f64>>,

    /// Heights ĥ_i = 1 / |y_i| > 0.
    heights_f64: Vec<f64>,

    /// Vertices of K rounded to f64.
    vertices_f64: Vec<Vector4<f64>>,
}

/// Errors from [`Polytope4D`] construction when invariants are violated.
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
            Self::Unbounded => {
                write!(f, "polytope is unbounded (dual vertices do not positively span R^4)")
            }
            Self::VertexEnumerationFailed(msg) => {
                write!(f, "vertex enumeration failed: {msg}")
            }
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
        }
    }
}

/// Map rational pipeline errors to construction errors.
fn map_rational_error(e: super::rational::RationalConstructionError) -> ConstructionError {
    use super::rational::RationalConstructionError;
    match e {
        RationalConstructionError::RedundantFacet(i) => ConstructionError::RedundantFacet(i),
        RationalConstructionError::Unbounded => ConstructionError::Unbounded,
        RationalConstructionError::TooFewFacets(n) => ConstructionError::TooFewFacets(n),
        other => ConstructionError::VertexEnumerationFailed(format!("{other}")),
    }
}

impl Polytope4D {
    /// Construct a polytope from f64 outward unit normals and positive heights.
    ///
    /// Validates f64 inputs, converts to exact rational dual vertices
    /// y_i = n_i / h_i, then runs the rational pipeline to compute
    /// vertices, incidence, adjacency, and ω₀ signs.
    pub fn new(
        normals: Vec<Vector4<f64>>,
        heights: Vec<f64>,
    ) -> Result<Self, ConstructionError> {
        // ── f64 pre-validation ──
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

        // ── Convert to rational dual vertices: y_i = n_i / h_i ──
        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| {
                let rh = super::rational::f64_to_rational(h);
                std::array::from_fn(|c| super::rational::f64_to_rational(n[c]) / &rh)
            })
            .collect();

        Self::from_dual_vertices(dual_vertices)
    }

    /// Construct a polytope from exact rational normals and heights.
    ///
    /// The normals need not be unit vectors — they are arbitrary nonzero rational
    /// directions. Heights must be strictly positive. Internally computes
    /// dual vertices y_i = n_i / h_i.
    pub fn from_rationals(
        normals: Vec<[BigRational; 4]>,
        heights: Vec<BigRational>,
    ) -> Result<Self, ConstructionError> {
        use num_traits::Signed;

        if normals.len() != heights.len() {
            return Err(ConstructionError::LengthMismatch {
                normals: normals.len(),
                heights: heights.len(),
            });
        }
        for (i, h) in heights.iter().enumerate() {
            if !h.is_positive() {
                return Err(ConstructionError::NonPositiveHeight {
                    index: i,
                    value: super::rational::rational_to_f64(h),
                });
            }
        }

        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();

        Self::from_dual_vertices(dual_vertices)
    }

    /// Construct a polytope from exact rational dual vertices y_i ∈ K°.
    ///
    /// Each dual vertex defines a halfspace y_i · x ≤ 1.
    pub fn from_dual_vertices(
        dual_vertices: Vec<[BigRational; 4]>,
    ) -> Result<Self, ConstructionError> {
        let (vertices, vertex_descriptors) =
            super::rational::construct_rational_pipeline(&dual_vertices)
                .map_err(map_rational_error)?;

        let v_count = vertices.len();
        let f_count = dual_vertices.len();

        // Build incidence matrix V×F
        let incidence = DMatrix::from_fn(v_count, f_count, |v, f| {
            vertex_descriptors[v].contains(&f)
        });

        // Build adjacency matrix F×F
        let adjacency = DMatrix::from_fn(f_count, f_count, |i, k| {
            i != k && (0..v_count).any(|v| incidence[(v, i)] && incidence[(v, k)])
        });

        // Build omega signs matrix F×F (antisymmetric)
        let omega_signs = DMatrix::from_fn(f_count, f_count, |i, k| {
            if i == k {
                return 0i8;
            }
            let omega =
                super::rational::omega0_rational(&dual_vertices[i], &dual_vertices[k]);
            match super::rational::Sign::of(&omega) {
                super::rational::Sign::Plus => 1,
                super::rational::Sign::Minus => -1,
                super::rational::Sign::Zero => 0,
            }
        });

        // Compute f64 representation
        let (normals_f64, heights_f64) =
            super::rational::dual_vertices_to_f64(&dual_vertices)
                .map_err(|e| ConstructionError::VertexEnumerationFailed(format!("{e}")))?;
        let vertices_f64 = super::rational::rational_vertices_to_f64(&vertices);

        Ok(Self {
            dual_vertices,
            vertices,
            incidence,
            adjacency,
            omega_signs,
            normals_f64,
            heights_f64,
            vertices_f64,
        })
    }

    /// Round f64 normals/heights to rational with the given denominator,
    /// then construct.
    ///
    /// Each coordinate x is mapped to round(x × D) / D. Lossy:
    /// ~3 decimal digits for D = 1000.
    ///
    /// # Panics
    ///
    /// `denominator` must be ≤ 2^52.
    pub fn from_f64_rounded(
        normals: &[Vector4<f64>],
        heights: &[f64],
        denominator: u64,
    ) -> Result<Self, ConstructionError> {
        use num_bigint::BigInt;
        debug_assert!(
            denominator <= 1u64 << 52,
            "denominator {denominator} exceeds 2^52; round() as i64 may overflow"
        );
        let d = BigInt::from(denominator);

        let rational_normals: Vec<[BigRational; 4]> = normals
            .iter()
            .map(|n| {
                std::array::from_fn(|i| {
                    let rounded = (n[i] * denominator as f64).round() as i64;
                    BigRational::new(BigInt::from(rounded), d.clone())
                })
            })
            .collect();

        let rational_heights: Vec<BigRational> = heights
            .iter()
            .map(|&h| {
                let rounded = (h * denominator as f64).round() as i64;
                BigRational::new(BigInt::from(rounded), d.clone())
            })
            .collect();

        Self::from_rationals(rational_normals, rational_heights)
    }

    /// Perturb dual vertices to break ω₀ = 0 degeneracies.
    ///
    /// Returns a new `Polytope4D` whose dual vertices are randomly perturbed
    /// by magnitude ~2^{-perturbation_bits}. The perturbed polytope is
    /// re-enumerated from scratch.
    ///
    /// Post-condition: all adjacent pairs have ω₀ ≠ 0.
    pub fn perturbed(
        &self,
        rng: &mut impl rand::Rng,
        perturbation_bits: u32,
    ) -> Result<Self, ConstructionError> {
        let perturbed: Vec<[BigRational; 4]> = self
            .dual_vertices
            .iter()
            .map(|y| {
                std::array::from_fn(|c| {
                    &y[c] + super::rational::random_small_rational(rng, perturbation_bits)
                })
            })
            .collect();

        let result = Self::from_dual_vertices(perturbed)?;

        // Verify post-condition: no adjacent pair has ω₀ = 0
        let f = result.facet_count();
        for i in 0..f {
            for k in (i + 1)..f {
                if result.adjacency[(i, k)] && result.omega_signs[(i, k)] == 0 {
                    return Err(ConstructionError::VertexEnumerationFailed(
                        "perturbation failed to break all ω₀ = 0 (astronomically unlikely)"
                            .into(),
                    ));
                }
            }
        }

        Ok(result)
    }

    // ── Exact rational accessors ──

    /// Dual vertices y_i = n_i / h_i, vertices of the polar body K°.
    pub fn dual_vertices(&self) -> &[[BigRational; 4]] {
        &self.dual_vertices
    }

    /// Exact rational vertices of K.
    pub fn vertices(&self) -> &[[BigRational; 4]] {
        &self.vertices
    }

    /// Vertex–facet incidence matrix E ∈ {0,1}^{V×F}.
    pub fn incidence(&self) -> &DMatrix<bool> {
        &self.incidence
    }

    /// Facet adjacency matrix A ∈ {0,1}^{F×F}.
    pub fn adjacency(&self) -> &DMatrix<bool> {
        &self.adjacency
    }

    /// Symplectic sign matrix ω ∈ {-1,0,+1}^{F×F}, antisymmetric.
    pub fn omega_signs(&self) -> &DMatrix<i8> {
        &self.omega_signs
    }

    // ── f64 accessors ──

    /// Outward unit normals n̂_i = y_i / |y_i| ∈ S³.
    pub fn normals_f64(&self) -> &[Vector4<f64>] {
        &self.normals_f64
    }

    /// Positive heights ĥ_i = 1 / |y_i| > 0.
    pub fn heights_f64(&self) -> &[f64] {
        &self.heights_f64
    }

    /// Vertices of K rounded to f64.
    pub fn vertices_f64(&self) -> &[Vector4<f64>] {
        &self.vertices_f64
    }

    /// Number of facets F.
    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }
}

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;
