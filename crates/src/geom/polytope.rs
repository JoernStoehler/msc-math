/// Convex polytope K ⊂ R⁴ via dual representation.
///
/// K = { x ∈ R⁴ | aᵢᵀ x ≤ 1 for all i = 1, ..., F }
///
/// where aᵢ ∈ R⁴\{0} are the vertices of the polar body K°.
///
/// # Invariants (enforced by constructor)
///
/// - F ≥ 5 (minimum facets for a bounded 4D polytope)
/// - All dual vertices aᵢ are nonzero
/// - **Bounded**: dual vertices positively span R⁴
/// - **Irredundant**: every facet has incident vertices of affine rank 3
/// - Vertices, incidence, adjacency, and ω₀ signs are precomputed exactly over Q
///
/// # Representations
///
/// Exact rational data (dual_vertices, vertices, incidence, adjacency, omega_signs)
/// is the source of truth for all discrete/combinatorial decisions.
/// The f64 data (dual_vertices_f64, vertices_f64) is for numerical algorithms.
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use std::collections::BTreeSet;

/// Tolerance for duplicate-halfspace detection: ‖aᵢ - aⱼ‖ / max(‖aᵢ‖, ‖aⱼ‖) < threshold.
const EPS_DUPLICATE_RELATIVE: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Polytope4D {
    /// Vertices of the polar body K°: aᵢ ∈ R⁴\{0}.
    /// Halfspace i is aᵢᵀ x ≤ 1.
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
    /// ω[i,k] = sign(ω₀(aᵢ, aₖ)). Zero only for non-generic polytopes.
    omega_signs: DMatrix<i8>,

    /// Dual vertices as f64. The native numerical representation.
    dual_vertices_f64: Vec<Vector4<f64>>,

    /// Vertices of K rounded to f64.
    vertices_f64: Vec<Vector4<f64>>,
}

/// Errors from [`Polytope4D`] construction when invariants are violated.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    TooFewFacets(usize),
    ZeroDualVertex(usize),
    DuplicateHalfspaces { i: usize, j: usize },
    Unbounded,
    NoVertices,
    /// Facet is redundant: no incident vertices, or incident vertices
    /// have affine rank < 3 (don't span the facet hyperplane).
    RedundantFacet(usize),
    F64Conversion(String),
    PerturbationFailed,
}

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFacets(n) => write!(f, "need ≥5 facets, got {n}"),
            Self::ZeroDualVertex(i) => write!(f, "dual vertex[{i}] is the zero vector"),
            Self::DuplicateHalfspaces { i, j } => {
                write!(f, "halfspaces[{i}] and [{j}] are duplicates")
            }
            Self::Unbounded => {
                write!(f, "polytope is unbounded (dual vertices do not positively span R⁴)")
            }
            Self::NoVertices => write!(f, "no vertices found (inconsistent halfspaces)"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
            Self::F64Conversion(msg) => write!(f, "f64 conversion failed: {msg}"),
            Self::PerturbationFailed => {
                write!(f, "perturbation failed to break all ω₀ = 0 (astronomically unlikely)")
            }
        }
    }
}


impl Polytope4D {
    /// Construct from exact rational dual vertices aᵢ ∈ R⁴\{0}.
    ///
    /// Each dual vertex defines a halfspace aᵢᵀ x ≤ 1.
    /// This is the primary constructor — all other constructors convert to
    /// rational dual vertices and call this.
    pub fn from_dual_vertices(
        dual_vertices: Vec<[BigRational; 4]>,
    ) -> Result<Self, ConstructionError> {
        let (vertices, vertex_descriptors) =
            super::vertex_enumeration::construct_rational_pipeline(&dual_vertices)?;

        let dual_vertices_f64 = dual_vertices
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let v = rational_array_to_f64(y);
                if v.norm() < 1e-15 {
                    Err(ConstructionError::F64Conversion(format!(
                        "dual vertex[{i}] has near-zero f64 norm: {}", v.norm()
                    )))
                } else {
                    Ok(v)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let vertices_f64 = rational_verts_to_f64(&vertices);

        Ok(Self::assemble(
            dual_vertices,
            vertices,
            &vertex_descriptors,
            dual_vertices_f64,
            vertices_f64,
        ))
    }

    /// Construct from f64 dual vertices (halfspaces aᵢᵀ x ≤ 1).
    ///
    /// Converts to exact rationals via f64_to_rational, then runs the
    /// rational pipeline. The f64 inputs are kept directly (no round-trip).
    pub fn new(halfspaces: Vec<Vector4<f64>>) -> Result<Self, ConstructionError> {
        if halfspaces.len() < 5 {
            return Err(ConstructionError::TooFewFacets(halfspaces.len()));
        }

        // Validate: nonzero, no duplicates
        for (i, a) in halfspaces.iter().enumerate() {
            if a.norm() < 1e-15 {
                return Err(ConstructionError::ZeroDualVertex(i));
            }
        }
        let f = halfspaces.len();
        for i in 0..f {
            for j in (i + 1)..f {
                let max_norm = halfspaces[i].norm().max(halfspaces[j].norm());
                if (halfspaces[i] - halfspaces[j]).norm() < EPS_DUPLICATE_RELATIVE * max_norm {
                    return Err(ConstructionError::DuplicateHalfspaces { i, j });
                }
            }
        }

        // Check bounded (dual vertices positively span R⁴)
        let unit_dirs: Vec<Vector4<f64>> = halfspaces.iter().map(|a| a.normalize()).collect();
        if !crate::geom::validation::check_bounded(&unit_dirs) {
            return Err(ConstructionError::Unbounded);
        }

        // Convert to rational
        let dual_vertices: Vec<[BigRational; 4]> = halfspaces
            .iter()
            .map(|a| {
                std::array::from_fn(|c| super::rational::f64_to_rational(a[c]))
            })
            .collect();

        // Run exact pipeline
        let (vertices, vertex_descriptors) =
            super::vertex_enumeration::construct_rational_pipeline(&dual_vertices)?;

        let vertices_f64 = rational_verts_to_f64(&vertices);

        // Keep original f64 dual vertices (no round-trip through rational)
        Ok(Self::assemble(
            dual_vertices,
            vertices,
            &vertex_descriptors,
            halfspaces,
            vertices_f64,
        ))
    }

    /// Construct from f64 normals and heights (legacy interface).
    ///
    /// Computes dual vertices aᵢ = nᵢ / hᵢ, then delegates to `new()`.
    /// Normals need not be unit vectors.
    pub fn from_normals_and_heights(
        normals: Vec<Vector4<f64>>,
        heights: Vec<f64>,
    ) -> Result<Self, ConstructionError> {
        if normals.len() != heights.len() {
            return Err(ConstructionError::TooFewFacets(0)); // TODO: better error
        }
        let halfspaces: Vec<Vector4<f64>> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect();
        Self::new(halfspaces)
    }

    /// Construct from exact rational normals and heights.
    ///
    /// Computes dual vertices aᵢ = nᵢ / hᵢ.
    pub fn from_rationals(
        normals: Vec<[BigRational; 4]>,
        heights: Vec<BigRational>,
    ) -> Result<Self, ConstructionError> {
        use num_traits::Signed;

        if normals.len() != heights.len() {
            return Err(ConstructionError::TooFewFacets(0)); // TODO: better error
        }
        for (i, h) in heights.iter().enumerate() {
            if !h.is_positive() {
                return Err(ConstructionError::ZeroDualVertex(i));
            }
        }

        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();

        Self::from_dual_vertices(dual_vertices)
    }

    /// Round f64 normals/heights to rational, then construct.
    ///
    /// Each coordinate x is mapped to round(x × D) / D.
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
        assert!(
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
    /// by magnitude ~2^{-perturbation_bits}.
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

        // Verify post-condition
        let f = result.facet_count();
        for i in 0..f {
            for k in (i + 1)..f {
                if result.adjacency[(i, k)] && result.omega_signs[(i, k)] == 0 {
                    return Err(ConstructionError::PerturbationFailed);
                }
            }
        }

        Ok(result)
    }

    /// Assemble from pre-computed components.
    fn assemble(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_descriptors: &[BTreeSet<usize>],
        dual_vertices_f64: Vec<Vector4<f64>>,
        vertices_f64: Vec<Vector4<f64>>,
    ) -> Self {
        let v_count = vertices.len();
        let f_count = dual_vertices.len();

        let incidence = DMatrix::from_fn(v_count, f_count, |v, f| {
            vertex_descriptors[v].contains(&f)
        });

        let adjacency = DMatrix::from_fn(f_count, f_count, |i, k| {
            i != k && (0..v_count).any(|v| incidence[(v, i)] && incidence[(v, k)])
        });

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

        Self {
            dual_vertices,
            vertices,
            incidence,
            adjacency,
            omega_signs,
            dual_vertices_f64,
            vertices_f64,
        }
    }

    // ── Exact rational accessors ──

    /// Dual vertices aᵢ, vertices of the polar body K°.
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

    /// Dual vertices aᵢ as f64 vectors. Native numerical representation.
    /// Halfspace i is aᵢᵀ x ≤ 1.
    pub fn dual_vertices_f64(&self) -> &[Vector4<f64>] {
        &self.dual_vertices_f64
    }

    /// Vertices of K rounded to f64.
    pub fn vertices_f64(&self) -> &[Vector4<f64>] {
        &self.vertices_f64
    }

    /// Number of facets F.
    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }

    // ── Derived quantities (computed on the fly) ──

    /// Unit normal n̂ᵢ = aᵢ / |aᵢ| for facet i.
    /// Computed from dual_vertices_f64 — not stored.
    pub fn normal_f64(&self, i: usize) -> Vector4<f64> {
        self.dual_vertices_f64[i].normalize()
    }

    /// Height hᵢ = 1 / |aᵢ| for facet i.
    /// Computed from dual_vertices_f64 — not stored.
    pub fn height_f64(&self, i: usize) -> f64 {
        1.0 / self.dual_vertices_f64[i].norm()
    }

    /// All unit normals, computed on the fly. For migration convenience.
    pub fn normals_f64(&self) -> Vec<Vector4<f64>> {
        self.dual_vertices_f64.iter().map(|a| a.normalize()).collect()
    }

    /// All heights, computed on the fly. For migration convenience.
    pub fn heights_f64(&self) -> Vec<f64> {
        self.dual_vertices_f64.iter().map(|a| 1.0 / a.norm()).collect()
    }
}

/// Convert exact rational 4-vector to f64.
fn rational_array_to_f64(v: &[BigRational; 4]) -> Vector4<f64> {
    Vector4::new(
        super::rational::rational_to_f64(&v[0]),
        super::rational::rational_to_f64(&v[1]),
        super::rational::rational_to_f64(&v[2]),
        super::rational::rational_to_f64(&v[3]),
    )
}

/// Convert exact rational vertices to f64 vectors.
fn rational_verts_to_f64(vertices: &[[BigRational; 4]]) -> Vec<Vector4<f64>> {
    vertices.iter().map(rational_array_to_f64).collect()
}

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;
