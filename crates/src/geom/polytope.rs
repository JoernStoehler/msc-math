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
                write!(f, "polytope is unbounded (dual vertices do not positively span R^4)")
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
    // ── Internal construction (single path, no duplication) ──────────────

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

    // ── Public constructors ─────────────────────────────────────────────

    /// Construct from exact rational dual vertices a_i in R^4 \ {0}.
    ///
    /// Each dual vertex defines a halfspace a_i^T x <= 1. Runs the exact
    /// rational pipeline: vertex enumeration, incidence, adjacency, omega signs.
    pub fn new(
        dual_vertices: Vec<[BigRational; 4]>,
    ) -> Result<Self, ConstructionError> {
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

        // Validate: nonzero
        for (i, a) in dual_vertices_f64.iter().enumerate() {
            if a.norm() < EPS_ZERO_NORM {
                return Err(ConstructionError::ZeroDualVertex(i));
            }
        }

        // Validate: no duplicates
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

        // Convert to exact rationals for discrete decisions (vertex-facet
        // incidence, omega signs). f64 cannot reliably decide these near zero.
        let dual_vertices: Vec<[BigRational; 4]> = dual_vertices_f64
            .iter()
            .map(|a| {
                std::array::from_fn(|c| super::rational_arithmetic::f64_to_rational(a[c]))
            })
            .collect();

        Self::build(dual_vertices, Some(dual_vertices_f64))
    }

    // ── Removed constructors ────────────────────────────────────────────
    // from_normals_and_heights, from_rationals, from_f64_rounded were thin
    // wrappers that computed n/h. Callers now inline the division and call
    // new() or from_f64() directly.

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
                        + super::rational_arithmetic::random_small_rational(
                            rng,
                            perturbation_bits,
                        )
                })
            })
            .collect();

        let result = Self::new(perturbed)?;

        // Verify post-condition: no adjacent pair has omega_0 = 0
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

        // Build vertex-facet incidence from the descriptor sets
        let incidence = DMatrix::from_fn(v_count, f_count, |v, f| {
            vertex_descriptors[v].contains(&f)
        });

        // Facets are adjacent iff they share at least one vertex
        let vertex_adjacency = DMatrix::from_fn(f_count, f_count, |i, k| {
            i != k && (0..v_count).any(|v| incidence[(v, i)] && incidence[(v, k)])
        });

        // Precompute sign(omega_0(a_i, a_k)) exactly over Q
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

    // ── Exact rational accessors ──

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

    // ── f64 accessors ──

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

    // ── Derived quantities (computed on the fly from dual vertices) ──

    /// Unit normal n_i = a_i / |a_i| for facet i.
    ///
    /// Computed from `dual_vertices_f64` -- not stored.
    pub fn normal_f64(&self, i: usize) -> Vector4<f64> {
        self.dual_vertices_f64[i].normalize()
    }

    /// Height h_i = 1 / |a_i| for facet i.
    ///
    /// Computed from `dual_vertices_f64` -- not stored.
    /// The halfspace a_i^T x <= 1 is equivalently n_i^T x <= h_i.
    pub fn height_f64(&self, i: usize) -> f64 {
        1.0 / self.dual_vertices_f64[i].norm()
    }

    /// All unit normals, computed on the fly.
    pub fn normals_f64(&self) -> Vec<Vector4<f64>> {
        self.dual_vertices_f64.iter().map(|a| a.normalize()).collect()
    }

    /// All heights, computed on the fly.
    pub fn heights_f64(&self) -> Vec<f64> {
        self.dual_vertices_f64
            .iter()
            .map(|a| 1.0 / a.norm())
            .collect()
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
mod tests {
    use super::*;
    use nalgebra::Vector4;

    // Tests for polytope: construction, accessors, and invariants.
    //
    // Proposition: Polytope4D construction validates inputs (nonzero, non-duplicate,
    // bounded, irredundant) and produces consistent incidence/adjacency/omega data.
    // Reference: [def:polytope-dual], [def:polar-body]
    //
    // Strategy: fixture-based (simplex, hypercube, known polytopes)

    /// 5 halfspaces forming a simplex-like polytope. a_i = n_i/h_i with h_i = 1.
    fn simplex_halfspaces_5() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ]
    }

    /// Verify a valid simplex construction returns correct facet count and accessors.
    #[test]
    fn valid_construction() {
        let halfspaces = simplex_halfspaces_5();
        let p = Polytope4D::from_f64(halfspaces).unwrap();
        assert_eq!(p.facet_count(), 5);
        assert_eq!(p.normals_f64().len(), 5);
        assert_eq!(p.heights_f64().len(), 5);
        assert!(!p.vertices_f64().is_empty(), "vertices should be precomputed");
    }

    /// Verify every vertex satisfies all halfspace inequalities n_i . v <= h_i.
    #[test]
    fn vertices_satisfy_halfspace_inequalities() {
        let halfspaces = simplex_halfspaces_5();
        let p = Polytope4D::from_f64(halfspaces).unwrap();

        const EPS: f64 = 1e-8;
        for v in p.vertices_f64() {
            for (i, (n, &h)) in p.normals_f64().iter().zip(p.heights_f64().iter()).enumerate() {
                let lhs = n.dot(v);
                assert!(
                    lhs <= h + EPS,
                    "vertex {} violates halfspace {}: {} > {}",
                    v,
                    i,
                    lhs,
                    h
                );
            }
        }
    }

    /// Verify that the incidence matrix is consistent with f64 vertex positions.
    ///
    /// For each vertex v and facet f: if incidence[v,f] is true, the f64 vertex
    /// must lie on that facet (within tolerance). If false, strictly interior.
    #[test]
    fn vertex_ordering_matches_incidence() {
        use crate::constants::EPS_FACET_INCIDENCE;
        use crate::geom::known_polytopes;

        for kp in known_polytopes::all_known() {
            let p = &kp.polytope;
            let incidence = p.incidence();
            let v_count = p.vertices_f64().len();

            assert_eq!(
                incidence.nrows(),
                v_count,
                "{}: incidence row count mismatch",
                kp.name
            );

            for vi in 0..v_count {
                let vertex = &p.vertices_f64()[vi];

                for fi in 0..p.facet_count() {
                    if incidence[(vi, fi)] {
                        let residual =
                            (p.normals_f64()[fi].dot(vertex) - p.heights_f64()[fi]).abs();
                        assert!(
                            residual < EPS_FACET_INCIDENCE,
                            "{}: vertex {} should be on facet {} but residual = {:.2e}",
                            kp.name,
                            vi,
                            fi,
                            residual
                        );
                    } else {
                        let slack = p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
                        assert!(
                            slack > EPS_FACET_INCIDENCE,
                            "{}: vertex {} should be interior to facet {} but slack = {:.2e}",
                            kp.name,
                            vi,
                            fi,
                            slack
                        );
                    }
                }
            }
        }
    }

    /// Verify vertex ordering invariant via the rational n/h -> new() construction path.
    #[test]
    fn vertex_ordering_via_from_rationals() {
        use crate::constants::EPS_FACET_INCIDENCE;
        use crate::geom::known_polytopes;
        use crate::geom::rational_arithmetic;

        for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
            let orig = &kp.polytope;

            let rational_normals: Vec<[num_rational::BigRational; 4]> = orig
                .normals_f64()
                .iter()
                .map(|n| std::array::from_fn(|i| rational_arithmetic::f64_to_rational(n[i])))
                .collect();
            let rational_heights: Vec<num_rational::BigRational> = orig
                .heights_f64()
                .iter()
                .map(|&h| rational_arithmetic::f64_to_rational(h))
                .collect();

            let dual_vertices: Vec<[num_rational::BigRational; 4]> = rational_normals
                .iter()
                .zip(rational_heights.iter())
                .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
                .collect();
            let p = Polytope4D::new(dual_vertices)
                .expect("rational n/h construction should succeed");
            let incidence = p.incidence();

            assert_eq!(
                p.vertices_f64().len(),
                incidence.nrows(),
                "{} (from_rationals): vertex count mismatch",
                kp.name
            );

            for vi in 0..p.vertices_f64().len() {
                let vertex = &p.vertices_f64()[vi];
                for fi in 0..p.facet_count() {
                    if incidence[(vi, fi)] {
                        let residual =
                            (p.normals_f64()[fi].dot(vertex) - p.heights_f64()[fi]).abs();
                        assert!(
                            residual < EPS_FACET_INCIDENCE,
                            "{} (from_rationals): vertex {} on facet {} residual = {:.2e}",
                            kp.name,
                            vi,
                            fi,
                            residual
                        );
                    } else {
                        let slack = p.heights_f64()[fi] - p.normals_f64()[fi].dot(vertex);
                        assert!(
                            slack > EPS_FACET_INCIDENCE,
                            "{} (from_rationals): vertex {} interior to facet {} slack = {:.2e}",
                            kp.name,
                            vi,
                            fi,
                            slack
                        );
                    }
                }
            }
        }
    }

    /// Adjacency matrix should be symmetric and have no self-adjacency.
    #[test]
    fn adjacency_matrix_symmetric_no_self_loops() {
        use crate::geom::known_polytopes;

        for kp in known_polytopes::all_known() {
            let p = &kp.polytope;
            let adj = p.vertex_adjacency();
            let f = p.facet_count();

            for i in 0..f {
                assert!(
                    !adj[(i, i)],
                    "{}: facet {} is self-adjacent",
                    kp.name,
                    i
                );
                for j in (i + 1)..f {
                    assert_eq!(
                        adj[(i, j)],
                        adj[(j, i)],
                        "{}: adjacency not symmetric at ({}, {})",
                        kp.name,
                        i,
                        j
                    );
                }
            }
        }
    }

    /// Omega signs matrix should be antisymmetric with zero diagonal.
    #[test]
    fn omega_signs_antisymmetric() {
        use crate::geom::known_polytopes;

        for kp in known_polytopes::all_known() {
            let p = &kp.polytope;
            let omega = p.omega_signs();
            let f = p.facet_count();

            for i in 0..f {
                assert_eq!(
                    omega[(i, i)],
                    0,
                    "{}: diagonal omega[{},{}] should be 0",
                    kp.name,
                    i,
                    i
                );
                for j in (i + 1)..f {
                    assert_eq!(
                        omega[(i, j)],
                        -omega[(j, i)],
                        "{}: omega not antisymmetric at ({}, {})",
                        kp.name,
                        i,
                        j
                    );
                }
            }
        }
    }

    /// The dual vertices accessor returns the right count and nonzero vectors.
    #[test]
    fn dual_vertices_count_and_nonzero() {
        let halfspaces = simplex_halfspaces_5();
        let p = Polytope4D::from_f64(halfspaces).unwrap();

        assert_eq!(p.dual_vertices_f64().len(), 5);
        for (i, dv) in p.dual_vertices_f64().iter().enumerate() {
            assert!(
                dv.norm() > 1e-10,
                "dual vertex[{i}] should be nonzero: {:?}",
                dv
            );
        }
    }

    /// Heights are positive for bounded polytopes.
    #[test]
    fn heights_positive() {
        use crate::geom::known_polytopes;

        for kp in known_polytopes::all_known() {
            for (i, &h) in kp.polytope.heights_f64().iter().enumerate() {
                assert!(
                    h > 0.0,
                    "{}: height[{i}] should be positive, got {h}",
                    kp.name
                );
            }
        }
    }

    /// Normals are unit vectors.
    #[test]
    fn normals_are_unit() {
        use crate::geom::known_polytopes;

        for kp in known_polytopes::all_known() {
            for (i, n) in kp.polytope.normals_f64().iter().enumerate() {
                assert!(
                    (n.norm() - 1.0).abs() < 1e-10,
                    "{}: normal[{i}] should be unit, norm = {}",
                    kp.name,
                    n.norm()
                );
            }
        }
    }

    // ---- Construction validation: edge cases and error paths ----
    //
    // Tests for polytope construction: edge cases and error paths.
    //
    // Proposition: Polytope4D::new rejects invalid inputs with the correct
    // ConstructionError variant: too few facets, zero dual vertex, duplicates,
    // unbounded, and redundant facets.
    // Reference: [def:polytope-dual]
    //
    // Strategy: exhaustive for each error variant

    /// Minimal valid halfspaces for a simplex-like polytope (5 facets).
    fn simplex_halfspaces() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ]
    }

    // ---- TooFewFacets ----

    /// Verify Polytope4D::new rejects 4 facets (minimum is 5 in R^4).
    #[test]
    fn reject_too_few_facets_4() {
        let halfspaces = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::TooFewFacets(4));
    }

    /// Verify Polytope4D::new rejects an empty halfspace list.
    #[test]
    fn reject_too_few_facets_0() {
        let halfspaces: Vec<Vector4<f64>> = vec![];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::TooFewFacets(0));
    }

    /// Verify Polytope4D::new rejects a single halfspace.
    #[test]
    fn reject_too_few_facets_1() {
        let halfspaces = vec![Vector4::x()];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::TooFewFacets(1));
    }

    // ---- ZeroDualVertex ----

    /// Verify Polytope4D::new rejects a zero-vector halfspace.
    #[test]
    fn reject_zero_halfspace() {
        let mut halfspaces = simplex_halfspaces();
        halfspaces[2] = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::ZeroDualVertex(2));
    }

    /// Verify Polytope4D::new rejects a near-zero (sub-epsilon) halfspace.
    #[test]
    fn reject_near_zero_halfspace() {
        let mut halfspaces = simplex_halfspaces();
        halfspaces[0] = Vector4::new(1e-16, 0.0, 0.0, 0.0);
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::ZeroDualVertex(0));
    }

    // ---- DuplicateHalfspaces ----

    /// Verify Polytope4D::new rejects duplicate halfspaces.
    #[test]
    fn reject_duplicate_halfspaces() {
        let halfspaces = vec![
            Vector4::x(),
            Vector4::y(),
            Vector4::z(),
            Vector4::w(),
            Vector4::x(), // duplicate of [0]
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::DuplicateHalfspaces { i: 0, j: 4 });
    }

    // ---- Unbounded ----

    /// Verify Polytope4D::new rejects halfspaces all pointing in roughly +x direction.
    #[test]
    fn reject_unbounded_all_positive_x() {
        // All halfspaces point roughly in +x direction -- unbounded in -x.
        let halfspaces = vec![
            Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::Unbounded);
    }

    /// Verify Polytope4D::new rejects halfspaces missing the -w direction.
    #[test]
    fn reject_unbounded_missing_one_direction() {
        // Bounded in x, y, z but not in w (no -w halfspace).
        let halfspaces = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            // missing -Vector4::w()
            Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        assert_eq!(err, ConstructionError::Unbounded);
    }

    // ---- RedundantFacet ----

    /// Verify Polytope4D::new rejects a redundant diagonal facet on the hypercube.
    #[test]
    fn reject_redundant_diagonal_facet() {
        // Hypercube [-1,1]^4 + one redundant diagonal facet far from the polytope.
        let n_diag = Vector4::new(1.0, 1.0, 0.0, 0.0).normalize();
        let halfspaces = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
            n_diag / 10.0, // x+y <= sqrt(2)*10 -- never active on [-1,1]^4
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        match err {
            ConstructionError::RedundantFacet(idx) => {
                assert_eq!(idx, 8, "the added diagonal facet should be redundant");
            }
            other => panic!("expected RedundantFacet, got {other:?}"),
        }
    }

    /// Verify Polytope4D::new rejects a nearly-parallel far-out redundant facet.
    #[test]
    fn reject_redundant_nearly_parallel_facet() {
        // Hypercube + a nearly parallel facet far from the polytope.
        let n_tilted = Vector4::new(1.0, 0.001, 0.0, 0.0).normalize();
        let halfspaces = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
            n_tilted / 100.0, // nearly +x, far out -- redundant
        ];
        let err = Polytope4D::from_f64(halfspaces).unwrap_err();
        match err {
            ConstructionError::RedundantFacet(idx) => {
                assert_eq!(idx, 8, "the nearly-parallel far facet should be redundant");
            }
            other => panic!("expected RedundantFacet, got {other:?}"),
        }
    }

    // ---- Positive tests: valid inputs are accepted ----

    /// Verify a valid 5-facet simplex is accepted.
    #[test]
    fn simplex_accepted() {
        let halfspaces = simplex_halfspaces();
        let p = Polytope4D::from_f64(halfspaces).unwrap();
        assert_eq!(p.facet_count(), 5);
    }

    /// Verify a valid 8-facet hypercube is accepted.
    #[test]
    fn hypercube_accepted() {
        let halfspaces = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
        ];
        let p = Polytope4D::from_f64(halfspaces).unwrap();
        assert_eq!(p.facet_count(), 8);
    }

    /// Verify from_f64 with n/h dual vertices accepts a valid hypercube.
    #[test]
    fn from_normals_and_heights_accepted() {
        let normals = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
        ];
        let heights = vec![1.0; 8];
        let p = Polytope4D::from_f64(
            normals.iter().zip(heights.iter()).map(|(n, &h)| n / h).collect(),
        ).unwrap();
        assert_eq!(p.facet_count(), 8);
    }

    /// Non-simple polytopes (where more than 4 facets meet at a vertex)
    /// should be accepted. The crosspolytope is a canonical example.
    #[test]
    fn non_simple_polytope_accepted() {
        let p = &crate::geom::known_polytopes::crosspolytope().polytope;
        assert_eq!(p.facet_count(), 16);
        assert!(!p.vertices_f64().is_empty());
    }
}
