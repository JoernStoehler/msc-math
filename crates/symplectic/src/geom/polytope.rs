//! Convex polytope K in R^4 via dual (polar) representation.
//!
//! The central type [`Polytope4D`] represents a bounded, irredundant convex polytope
//! K = { x in R^4 | a_i^T x <= 1 for all i }, where the a_i are vertices of the
//! polar body K°. All combinatorial data (vertices, incidence, adjacency, omega signs)
//! is precomputed exactly over Q at construction time; f64 copies are kept for
//! numerical algorithms.
//!
//! Mathematical correspondence: [def:polytope-dual], [def:polar-body]

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use std::collections::BTreeSet;

/// Tolerance for duplicate-halfspace detection: ||a_i - a_j|| / max(||a_i||, ||a_j||) < threshold.
///
/// **Why 1e-8:** f64 dual vertices from user inputs differ by at least O(1e-3)
/// in practice (physically distinct facet directions). The 1e-8 relative
/// threshold is tight enough to reject identical or nearly-identical halfspaces
/// while staying far above machine epsilon (~1e-16), avoiding spurious false
/// positives from floating-point rounding.
const EPS_DUPLICATE_RELATIVE: f64 = 1e-8;

/// Near-zero f64 norm threshold for dual vertex validation.
///
/// **Why 1e-15:** f64 machine epsilon is ~2.2e-16; any norm below 1e-15
/// indicates a vector whose direction is lost to rounding and cannot
/// represent a meaningful halfspace constraint.
const EPS_ZERO_NORM: f64 = 1e-15;

/// Convex polytope K in R^4 via dual (polar) representation.
///
/// K = { x in R^4 | a_i^T x <= 1 for all i = 1, ..., F }
///
/// where a_i in R^4 \ {0} are the vertices of the polar body K°.
///
/// # Invariants (enforced by constructor)
///
/// - F >= 5 (minimum facets for a bounded 4D polytope)
/// - All dual vertices a_i are nonzero
/// - **Bounded**: dual vertices positively span R^4
/// - **Irredundant**: every facet has incident vertices of affine rank 3
/// - Vertices, incidence, adjacency, and omega_0 signs are precomputed exactly over Q
///
/// # Representations
///
/// Exact rational data (`dual_vertices`, `vertices`, `incidence`, `adjacency`,
/// `omega_signs`) is the source of truth for all discrete/combinatorial decisions.
/// The f64 data (`dual_vertices_f64`, `vertices_f64`) is for numerical algorithms.
#[derive(Clone, Debug)]
pub struct Polytope4D {
    /// Vertices of the polar body K°: a_i in R^4 \ {0}.
    /// Halfspace i is a_i^T x <= 1.
    dual_vertices: Vec<[BigRational; 4]>,

    /// Vertices of K, computed exactly over Q.
    vertices: Vec<[BigRational; 4]>,

    /// Vertex-facet incidence matrix E in {0,1}^{V x F}.
    /// E[v,f] = true iff vertex v lies on facet f.
    incidence: DMatrix<bool>,

    /// Vertex-sharing adjacency matrix A in {0,1}^{F x F}.
    /// A[i,k] = true iff facets i and k share at least one vertex.
    vertex_adjacency: DMatrix<bool>,

    /// Symplectic sign matrix omega in {-1,0,+1}^{F x F}, antisymmetric.
    /// omega[i,k] = sign(omega_0(a_i, a_k)). Zero only for non-generic polytopes.
    omega_signs: DMatrix<i8>,

    /// Dual vertices as f64. The native numerical representation.
    dual_vertices_f64: Vec<Vector4<f64>>,

    /// Vertices of K rounded to f64.
    vertices_f64: Vec<Vector4<f64>>,
}

/// Errors from [`Polytope4D`] construction when invariants are violated.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionError {
    /// Fewer than 5 facets (the minimum for a bounded 4D polytope).
    TooFewFacets(usize),
    /// Dual vertex at the given index is the zero vector.
    ZeroDualVertex(usize),
    /// Two halfspaces are duplicates (within relative tolerance).
    DuplicateHalfspaces { i: usize, j: usize },
    /// Dual vertices do not positively span R^4 (polytope is unbounded).
    Unbounded,
    /// No vertices found (inconsistent halfspace system).
    NoVertices,
    /// Facet is redundant: no incident vertices, or incident vertices
    /// have affine rank < 3 (don't span the facet hyperplane).
    RedundantFacet(usize),
    /// Exact-to-f64 conversion produced degenerate result.
    F64Conversion(String),
    /// Perturbation failed to break all omega_0 = 0 degeneracies.
    PerturbationFailed,
}

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFacets(n) => write!(f, "need >=5 facets, got {n}"),
            Self::ZeroDualVertex(i) => write!(f, "dual vertex[{i}] is the zero vector"),
            Self::DuplicateHalfspaces { i, j } => {
                write!(f, "halfspaces[{i}] and [{j}] are duplicates")
            }
            Self::Unbounded => {
                write!(
                    f,
                    "polytope is unbounded (dual vertices do not positively span R^4)"
                )
            }
            Self::NoVertices => write!(f, "no vertices found (inconsistent halfspaces)"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
            Self::F64Conversion(msg) => write!(f, "f64 conversion failed: {msg}"),
            Self::PerturbationFailed => {
                write!(
                    f,
                    "perturbation failed to break all omega_0 = 0 (astronomically unlikely)"
                )
            }
        }
    }
}

impl Polytope4D {
    /// Core constructor. All public constructors converge here.
    ///
    /// Takes exact rational dual vertices (source of truth for all discrete
    /// decisions) and optional pre-computed f64 dual vertices (kept as-is
    /// to avoid round-trip loss; computed from rationals if not provided).
    ///
    /// Delegates bounded/irredundancy/vertex checks to `construct_rational_pipeline`.
    /// Correctness: [lem:vertex-enumeration] (vertex enumeration via 4-subsets),
    /// [lem:positive-span] + [lem:bounded-triples] (bounded check),
    /// [lem:irredundancy] (irredundancy check), [lem:rational-pipeline] (exact Q arithmetic).
    fn build(
        dual_vertices: Vec<[BigRational; 4]>,
        dual_vertices_f64: Option<Vec<Vector4<f64>>>,
    ) -> Result<Self, ConstructionError> {
        let (vertices, vertex_descriptors) =
            super::vertex_enumeration::construct_rational_pipeline(&dual_vertices)?;

        let dual_vertices_f64 = match dual_vertices_f64 {
            Some(dv) => dv,
            None => dual_vertices
                .iter()
                .enumerate()
                .map(|(i, y)| {
                    let v = rational_array_to_f64(y);
                    if v.norm() < EPS_ZERO_NORM {
                        Err(ConstructionError::F64Conversion(format!(
                            "dual vertex[{i}] has near-zero f64 norm: {}",
                            v.norm()
                        )))
                    } else {
                        Ok(v)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        };

        let vertices_f64 = rational_verts_to_f64(&vertices);

        Ok(Self::assemble(
            dual_vertices,
            vertices,
            &vertex_descriptors,
            dual_vertices_f64,
            vertices_f64,
        ))
    }

    /// Construct from exact rational dual vertices a_i in R^4 \ {0}.
    ///
    /// Each dual vertex defines a halfspace a_i^T x <= 1. Runs the exact
    /// rational pipeline: vertex enumeration, incidence, adjacency, omega signs.
    pub fn new(dual_vertices: Vec<[BigRational; 4]>) -> Result<Self, ConstructionError> {
        Self::build(dual_vertices, None)
    }

    /// Construct from f64 dual vertices a_i in R^4 \ {0}.
    ///
    /// Validates inputs (nonzero, no duplicates), converts to exact rationals
    /// via `f64_to_rational`, then runs the rational pipeline. The original f64
    /// inputs are kept directly (no round-trip through rational).
    ///
    /// Correctness: [lem:rational-pipeline] (exact Q arithmetic for discrete
    /// decisions), [lem:vertex-enumeration], [lem:positive-span],
    /// [lem:bounded-triples], [lem:irredundancy].
    pub fn from_f64(dual_vertices_f64: Vec<Vector4<f64>>) -> Result<Self, ConstructionError> {
        let f = dual_vertices_f64.len();
        if f < 5 {
            return Err(ConstructionError::TooFewFacets(f));
        }

        for (i, a) in dual_vertices_f64.iter().enumerate() {
            for c in 0..4 {
                let value = a[c];
                if !value.is_finite() {
                    return Err(ConstructionError::F64Conversion(format!(
                        "dual vertex[{i}][{c}] is non-finite: {value}"
                    )));
                }
            }
            if a.norm() < EPS_ZERO_NORM {
                return Err(ConstructionError::ZeroDualVertex(i));
            }
        }

        for i in 0..f {
            for j in (i + 1)..f {
                let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
                if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm()
                    < EPS_DUPLICATE_RELATIVE * max_norm
                {
                    return Err(ConstructionError::DuplicateHalfspaces { i, j });
                }
            }
        }

        let dual_vertices: Vec<[BigRational; 4]> = dual_vertices_f64
            .iter()
            .map(|a| std::array::from_fn(|c| super::rational_arithmetic::f64_to_rational(a[c])))
            .collect();

        Self::build(dual_vertices, Some(dual_vertices_f64))
    }

    /// Construct from pre-computed rational dual vertices and vertices.
    ///
    /// Recomputes vertex_descriptors by checking, for each vertex v and facet f,
    /// whether a_f · v = 1 (exact rational dot product). This is O(V·F) — much
    /// cheaper than vertex enumeration which is O(C(F,4)).
    ///
    /// Then calls assemble() to build incidence, omega_signs, vertex_adjacency,
    /// and f64 copies.
    pub fn from_rational_parts(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
    ) -> Result<Self, ConstructionError> {
        use num_traits::One;

        let one = BigRational::one();

        let vertex_descriptors: Vec<BTreeSet<usize>> = vertices
            .iter()
            .map(|v| {
                (0..dual_vertices.len())
                    .filter(|&f| {
                        let dot = &dual_vertices[f][0] * &v[0]
                            + &dual_vertices[f][1] * &v[1]
                            + &dual_vertices[f][2] * &v[2]
                            + &dual_vertices[f][3] * &v[3];
                        dot == one
                    })
                    .collect()
            })
            .collect();

        let dual_vertices_f64 = dual_vertices
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let v = rational_array_to_f64(y);
                if v.norm() < EPS_ZERO_NORM {
                    Err(ConstructionError::F64Conversion(format!(
                        "dual vertex[{i}] has near-zero f64 norm: {}",
                        v.norm()
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

    /// Perturb dual vertices to break omega_0 = 0 degeneracies.
    ///
    /// Returns a new `Polytope4D` whose dual vertices are randomly perturbed
    /// by magnitude ~2^{-perturbation_bits}.
    ///
    /// Post-condition: all adjacent pairs have omega_0 != 0.
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
                    &y[c]
                        + super::rational_arithmetic::random_small_rational(rng, perturbation_bits)
                })
            })
            .collect();

        let result = Self::new(perturbed)?;

        let f = result.facet_count();
        for i in 0..f {
            for k in (i + 1)..f {
                if result.vertex_adjacency[(i, k)] && result.omega_signs[(i, k)] == 0 {
                    return Err(ConstructionError::PerturbationFailed);
                }
            }
        }

        Ok(result)
    }

    /// Assemble from pre-computed components (internal).
    ///
    /// Builds incidence, adjacency, and omega sign matrices from the vertex
    /// descriptors produced by the rational pipeline.
    ///
    /// Omega sign computation uses exact rational arithmetic: [lem:rational-pipeline]
    /// (sign(omega_0(a_i, a_k)) computed over Q to prevent misclassification).
    fn assemble(
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_descriptors: &[BTreeSet<usize>],
        dual_vertices_f64: Vec<Vector4<f64>>,
        vertices_f64: Vec<Vector4<f64>>,
    ) -> Self {
        let v_count = vertices.len();
        let f_count = dual_vertices.len();

        let incidence =
            DMatrix::from_fn(v_count, f_count, |v, f| vertex_descriptors[v].contains(&f));

        let vertex_adjacency = DMatrix::from_fn(f_count, f_count, |i, k| {
            i != k && (0..v_count).any(|v| incidence[(v, i)] && incidence[(v, k)])
        });

        let omega_signs = DMatrix::from_fn(f_count, f_count, |i, k| {
            if i == k {
                return 0i8;
            }
            let omega =
                super::rational_arithmetic::omega0_rational(&dual_vertices[i], &dual_vertices[k]);
            match super::rational_arithmetic::Sign::of(&omega) {
                super::rational_arithmetic::Sign::Plus => 1,
                super::rational_arithmetic::Sign::Minus => -1,
                super::rational_arithmetic::Sign::Zero => 0,
            }
        });

        Self {
            dual_vertices,
            vertices,
            incidence,
            vertex_adjacency,
            omega_signs,
            dual_vertices_f64,
            vertices_f64,
        }
    }

    /// Dual vertices a_i, vertices of the polar body K°.
    pub fn dual_vertices(&self) -> &[[BigRational; 4]] {
        &self.dual_vertices
    }

    /// Exact rational vertices of K.
    pub fn vertices(&self) -> &[[BigRational; 4]] {
        &self.vertices
    }

    /// Vertex-facet incidence matrix E in {0,1}^{V x F}.
    ///
    /// `incidence[(v, f)]` is true iff vertex v lies on facet f.
    pub fn incidence(&self) -> &DMatrix<bool> {
        &self.incidence
    }

    /// Vertex-sharing adjacency matrix A in {0,1}^{F x F}.
    ///
    /// `vertex_adjacency[(i, k)]` is true iff facets i and k share at least one vertex.
    pub fn vertex_adjacency(&self) -> &DMatrix<bool> {
        &self.vertex_adjacency
    }

    /// Symplectic sign matrix omega in {-1,0,+1}^{F x F}, antisymmetric.
    ///
    /// `omega_signs[(i, k)]` = sign(omega_0(a_i, a_k)), computed exactly over Q.
    /// Zero only for non-generic polytopes.
    pub fn omega_signs(&self) -> &DMatrix<i8> {
        &self.omega_signs
    }

    /// Dual vertices a_i as f64 vectors. Native numerical representation.
    ///
    /// Halfspace i is a_i^T x <= 1.
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
}

/// Convert exact rational 4-vector to f64.
fn rational_array_to_f64(v: &[BigRational; 4]) -> Vector4<f64> {
    Vector4::new(
        super::rational_arithmetic::rational_to_f64(&v[0]),
        super::rational_arithmetic::rational_to_f64(&v[1]),
        super::rational_arithmetic::rational_to_f64(&v[2]),
        super::rational_arithmetic::rational_to_f64(&v[3]),
    )
}

/// Convert exact rational vertices to f64 vectors.
fn rational_verts_to_f64(vertices: &[[BigRational; 4]]) -> Vec<Vector4<f64>> {
    vertices.iter().map(rational_array_to_f64).collect()
}

#[cfg(test)]
mod tests;
