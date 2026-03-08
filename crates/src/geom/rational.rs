//! Exact rational arithmetic for polytope combinatorial data.
//!
//! A [`RationalPolytope4D`] stores polytope coordinates as exact rationals
//! (not unit normals — just halfspace directions). The following quantities
//! are computed exactly over Q at construction time
//! (thesis [rem:exact-quantities]):
//!
//! - **Vertex–facet incidence** E ∈ {0,1}^{V×F}: stored as vertex descriptors
//!   Sⱼ = {i : E_{j,i} = 1}, the set of facets through each vertex.
//! - **Symplectic signs** sign(ω₀(nᵢ, nₖ)) ∈ {+, -, 0} for each facet pair.
//!   Stored only for vertex-adjacent pairs as an optimization (non-adjacent
//!   pairs are pruned earlier by the search algorithm).
//!
//! The f64 representation (unit normals, heights) is a derived quantity
//! for fast numerical computation (KKT solver). The rational polytope is the
//! source of truth for all discrete/combinatorial decisions.
//!
//! # Entry points
//!
//! - [`RationalPolytope4D::new`]: from exact rational coordinates (e.g. 1/3).
//! - [`RationalPolytope4D::from_f64`]: from f64 coordinates (lossless — every
//!   f64 is an exact rational m·2^e).
//! - [`RationalPolytope4D::from_f64_rounded`]: from f64 with rounding to a
//!   fixed denominator D (lossy, smaller denominators).
//!
//! # Margins
//!
//! [`Margins`] records each exact predicate's distance from its decision
//! boundary. A polytope is numerically ε-robust when all margins exceed ε.
//! When ε ≫ f64 rounding error, the rounded f64 polytope preserves
//! vertex–facet incidence and nonzero symplectic signs.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use std::collections::{BTreeMap, BTreeSet};

use super::polytope::Polytope4D;

/// Sign of an exact rational value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sign {
    Plus,
    Minus,
    Zero,
}

impl Sign {
    /// Compute the sign of a rational number.
    pub fn of(r: &BigRational) -> Self {
        if r.is_zero() {
            Sign::Zero
        } else if r.is_positive() {
            Sign::Plus
        } else {
            Sign::Minus
        }
    }
}

/// Exact quantities computed over Q from rational polytope coordinates
/// (thesis [rem:exact-quantities]).
///
/// Every field is determined exactly from rational coordinates.
/// No numerical predicates, no tolerances.
#[derive(Clone, Debug)]
pub struct CombinatorialData {
    /// Number of facets.
    #[allow(dead_code)]
    pub(crate) num_facets: usize,
    /// Vertex–facet incidence, stored as vertex descriptors: Sⱼ = {i : E_{j,i} = 1}.
    /// Each vertex is the intersection of ≥4 facets (exactly 4 for simple polytopes,
    /// more for non-simple). Sorted lexicographically.
    pub(crate) vertex_descriptors: Vec<BTreeSet<usize>>,
    /// Vertex-adjacent facet pairs (i, k) with i < k.
    /// Derived from incidence: {i,k} ⊂ Sⱼ for some vertex j.
    pub(crate) adjacency: BTreeSet<(usize, usize)>,
    /// sign(ω₀(nᵢ, nₖ)) for each vertex-adjacent pair (i, k) with i < k.
    /// Stored only for adjacent pairs (non-adjacent pairs are pruned by the
    /// search algorithm before symplectic signs are needed).
    pub(crate) sign_pattern: BTreeMap<(usize, usize), Sign>,
    /// Exact rational margins.
    #[allow(dead_code)]
    pub(crate) margins: Margins,
}

/// Exact rational margins for the ε-robustness conditions.
///
/// A polytope is numerically ε-robust when all three margins exceed ε.
/// When ε ≫ f64 rounding error (~1e-15 × scale),
/// the rounded f64 polytope preserves vertex–facet incidence and
/// nonzero symplectic signs.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Margins {
    /// Smallest non-incidence gap: min over all vertices j and non-incident facets i
    /// of gⱼⁱ = hᵢ - ⟨nᵢ, vⱼ⟩. Always positive for valid polytopes.
    pub(crate) min_gap: BigRational,
    /// Smallest absolute determinant |det(N_S)| over vertex subsets S.
    /// Positive iff all vertex matrices are non-singular (which they are).
    pub(crate) min_abs_det: BigRational,
    /// Smallest |ω₀(nᵢ, nₖ)| over adjacent pairs with ω₀(nᵢ, nₖ) ≠ 0.
    /// None if all adjacent pairs have ω₀ = 0.
    pub(crate) min_omega_nonzero: Option<BigRational>,
}

/// Polytope with exact rational coordinates and precomputed combinatorial data.
///
/// The normals are NOT unit vectors — they are arbitrary nonzero rational
/// directions defining halfspaces ⟨nᵢ, x⟩ ≤ hᵢ. Unit normalization happens
/// only in [`to_f64()`](RationalPolytope4D::to_f64).
#[derive(Clone, Debug)]
pub struct RationalPolytope4D {
    normals: Vec<[BigRational; 4]>,
    heights: Vec<BigRational>,
    /// Exact rational vertices, one per vertex descriptor (same order).
    vertices: Vec<[BigRational; 4]>,
    data: CombinatorialData,
}

/// Errors during rational polytope construction.
#[derive(Clone, Debug)]
pub enum RationalConstructionError {
    /// Normal and height vectors have different lengths.
    LengthMismatch { normals: usize, heights: usize },
    /// Fewer than 5 facets (cannot form bounded 4D polytope).
    TooFewFacets(usize),
    /// A normal vector is the zero vector.
    ZeroNormal(usize),
    /// A height is not strictly positive.
    NonPositiveHeight { index: usize },
    /// Normals do not positively span R^4 (polytope is unbounded).
    Unbounded,
    /// No vertices found — the halfspaces are inconsistent.
    NoVertices,
    /// Facet is redundant: no incident vertices, or incident vertices
    /// have affine rank < 3 (don't span the facet hyperplane).
    RedundantFacet(usize),
    /// Perturbation failed to break all ω₀ = 0 (astronomically unlikely).
    PerturbationFailed,
    /// Conversion to f64 failed.
    F64Conversion(String),
}

impl std::fmt::Display for RationalConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch { normals, heights } => {
                write!(f, "length mismatch: {normals} normals vs {heights} heights")
            }
            Self::TooFewFacets(n) => write!(f, "need ≥5 facets, got {n}"),
            Self::ZeroNormal(i) => write!(f, "normal[{i}] is the zero vector"),
            Self::NonPositiveHeight { index } => {
                write!(f, "height[{index}] is not strictly positive")
            }
            Self::Unbounded => write!(f, "normals do not positively span R^4 (unbounded)"),
            Self::NoVertices => write!(f, "no vertices found (inconsistent halfspaces)"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant (no incident vertices, or affine rank < 3)"),
            Self::PerturbationFailed => write!(
                f,
                "perturbation failed to break all ω₀ = 0 (astronomically unlikely)"
            ),
            Self::F64Conversion(msg) => write!(f, "f64 conversion failed: {msg}"),
        }
    }
}

impl std::error::Error for RationalConstructionError {}

// ── Exact linear algebra over Q ──────────────────────────────────────────

/// Compute the exact determinant of a 4×4 rational matrix.
/// Rows are given as 4-element arrays.
fn det4(rows: &[[BigRational; 4]; 4]) -> BigRational {
    // Leibniz formula: sum over all 24 permutations of S₄.
    // For a 4×4 matrix, we expand along cofactors of the first row.
    let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

    // Cofactors of row 0 (minor matrices with column i removed)
    let c00 = det3(&[b[1].clone(), b[2].clone(), b[3].clone()],
                    &[c[1].clone(), c[2].clone(), c[3].clone()],
                    &[d[1].clone(), d[2].clone(), d[3].clone()]);
    let c01 = det3(&[b[0].clone(), b[2].clone(), b[3].clone()],
                    &[c[0].clone(), c[2].clone(), c[3].clone()],
                    &[d[0].clone(), d[2].clone(), d[3].clone()]);
    let c02 = det3(&[b[0].clone(), b[1].clone(), b[3].clone()],
                    &[c[0].clone(), c[1].clone(), c[3].clone()],
                    &[d[0].clone(), d[1].clone(), d[3].clone()]);
    let c03 = det3(&[b[0].clone(), b[1].clone(), b[2].clone()],
                    &[c[0].clone(), c[1].clone(), c[2].clone()],
                    &[d[0].clone(), d[1].clone(), d[2].clone()]);

    &a[0] * c00 - &a[1] * c01 + &a[2] * c02 - &a[3] * c03
}

/// Determinant of a 3×3 matrix (Sarrus' rule).
/// Rows are given as 3-element slices.
fn det3(r0: &[BigRational], r1: &[BigRational], r2: &[BigRational]) -> BigRational {
    &r0[0] * (&r1[1] * &r2[2] - &r1[2] * &r2[1])
        - &r0[1] * (&r1[0] * &r2[2] - &r1[2] * &r2[0])
        + &r0[2] * (&r1[0] * &r2[1] - &r1[1] * &r2[0])
}

/// Solve a 4×4 linear system N·x = b exactly via Cramer's rule.
/// Returns None if det(N) = 0.
fn solve4(rows: &[[BigRational; 4]; 4], rhs: &[BigRational; 4]) -> Option<[BigRational; 4]> {
    let d = det4(rows);
    if d.is_zero() {
        return None;
    }

    let mut result: [BigRational; 4] = std::array::from_fn(|_| BigRational::zero());

    for col in 0..4 {
        // Replace column `col` of N with rhs
        let mut modified = rows.clone();
        for row in 0..4 {
            modified[row][col] = rhs[row].clone();
        }
        result[col] = det4(&modified) / &d;
    }

    Some(result)
}

/// Inner product ⟨a, b⟩ of two 4-vectors over Q.
fn dot4(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

/// Standard symplectic form ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁.
///
/// Same formula as [`super::symplectic::omega0`] but over Q.
/// Does NOT require unit normals.
fn omega0_rational(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

/// 4D cross product over Q: vector perpendicular to three vectors in R⁴.
///
/// Same formula as [`super::cross_product::cross_product_4d`] but exact.
/// d_k = (-1)^k · det(3×3 minor of [a, b, c] with column k removed).
fn cross_product_4d_rational(
    a: &[BigRational; 4],
    b: &[BigRational; 4],
    c: &[BigRational; 4],
) -> [BigRational; 4] {
    // 2×2 minors of (b, c)
    let bc_01 = &b[0] * &c[1] - &b[1] * &c[0];
    let bc_02 = &b[0] * &c[2] - &b[2] * &c[0];
    let bc_03 = &b[0] * &c[3] - &b[3] * &c[0];
    let bc_12 = &b[1] * &c[2] - &b[2] * &c[1];
    let bc_13 = &b[1] * &c[3] - &b[3] * &c[1];
    let bc_23 = &b[2] * &c[3] - &b[3] * &c[2];

    let d0 = &a[1] * &bc_23 - &a[2] * &bc_13 + &a[3] * &bc_12;
    let d1 = -(&a[0] * &bc_23 - &a[2] * &bc_03 + &a[3] * &bc_02);
    let d2 = &a[0] * &bc_13 - &a[1] * &bc_03 + &a[3] * &bc_01;
    let d3 = -(&a[0] * &bc_12 - &a[1] * &bc_02 + &a[2] * &bc_01);

    [d0, d1, d2, d3]
}

/// Compute the rank of a matrix (rows × 4) over Q via Gaussian elimination.
///
/// Exact — no tolerances, no floating-point rounding.
fn rank_over_q(rows: &[[BigRational; 4]]) -> usize {
    if rows.is_empty() {
        return 0;
    }

    let m = rows.len();
    let n = 4;
    let mut mat: Vec<[BigRational; 4]> = rows.to_vec();

    let mut rank = 0;
    for col in 0..n {
        // Find pivot row with nonzero entry in this column
        let pivot_row = (rank..m).find(|&r| !mat[r][col].is_zero());
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        mat.swap(rank, pivot_row);

        // Eliminate all other rows
        let pivot_val = mat[rank][col].clone();
        for r in 0..m {
            if r == rank || mat[r][col].is_zero() {
                continue;
            }
            let factor = &mat[r][col] / &pivot_val;
            // Clone the pivot row to avoid borrowing mat twice.
            let pivot_row: [BigRational; 4] = mat[rank].clone();
            for (mat_c, pivot_c) in mat[r][col..n].iter_mut().zip(pivot_row[col..n].iter()) {
                *mat_c = &*mat_c - &factor * pivot_c;
            }
        }
        rank += 1;
    }
    rank
}

/// Check that normals positively span R^4 (polytope is bounded).
///
/// K bounded ⟺ rec(K) = {0} ⟺ normals positively span R^4.
/// "Positively span" means: for every nonzero d ∈ R^4, some nᵢ · d > 0.
///
/// Algorithm (exact over Q):
/// 1. Check rank(normals) = 4 via Gaussian elimination.
/// 2. For each triple (i,j,k), compute the 1D kernel direction d via
///    exact 4D cross product. If d = 0 (dependent triple), skip.
///    Check some nₗ · d > 0 and some nₗ · d < 0 among normals ∉ {i,j,k}.
///
/// Complexity: O(F⁴) — F³ triples × F inner products each.
fn check_bounded_rational(normals: &[[BigRational; 4]]) -> bool {
    let f = normals.len();

    if rank_over_q(normals) < 4 {
        return false;
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let d = cross_product_4d_rational(&normals[i], &normals[j], &normals[k]);
                if d.iter().all(|c| c.is_zero()) {
                    continue; // Dependent triple
                }

                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&normals[l], &d).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&normals[l], &d).is_negative());

                if !has_pos || !has_neg {
                    return false;
                }
            }
        }
    }
    true
}

/// Compute the affine rank of a set of 4D rational points.
///
/// Affine rank = dimension of the affine span = rank of centered differences.
fn affine_rank_rational(points: &[[BigRational; 4]]) -> usize {
    if points.len() <= 1 {
        return 0;
    }

    let base = &points[0];
    let centered: Vec<[BigRational; 4]> = points[1..]
        .iter()
        .map(|p| std::array::from_fn(|i| &p[i] - &base[i]))
        .collect();

    rank_over_q(&centered)
}

/// Result of exact vertex enumeration: (descriptors, vertices, determinants, gaps).
type VertexEnumResult = (
    Vec<BTreeSet<usize>>,
    Vec<[BigRational; 4]>,
    Vec<BigRational>,
    Vec<BigRational>,
);

// ── Combinatorial subsets ────────────────────────────────────────────────

/// Enumerate all C(n, 4) four-element subsets of {0, ..., n-1}.
fn combinations4(n: usize) -> Vec<[usize; 4]> {
    let mut result = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    result.push([i, j, k, l]);
                }
            }
        }
    }
    result
}

// ── Construction ─────────────────────────────────────────────────────────

impl RationalPolytope4D {
    /// Construct a rational polytope from exact rational coordinates.
    ///
    /// Computes exact combinatorial data (vertex descriptors, adjacency,
    /// ω₀ signs, margins) at construction time.
    ///
    /// Fails if:
    /// - Fewer than 5 facets
    /// - Any normal is zero or any height is non-positive
    /// - Normals do not positively span R^4 (unbounded)
    /// - No vertices found (inconsistent halfspaces)
    /// - Any facet is redundant (no incident vertices or affine rank < 3)
    ///
    /// Non-simple polytopes (vertices on >4 facets) are supported: the vertex
    /// descriptor records ALL incident facets, not just the defining 4-subset.
    pub fn new(
        normals: Vec<[BigRational; 4]>,
        heights: Vec<BigRational>,
    ) -> Result<Self, RationalConstructionError> {
        let f = normals.len();

        // Basic validation
        if normals.len() != heights.len() {
            return Err(RationalConstructionError::LengthMismatch {
                normals: normals.len(),
                heights: heights.len(),
            });
        }
        if f < 5 {
            return Err(RationalConstructionError::TooFewFacets(f));
        }
        for (i, n) in normals.iter().enumerate() {
            if n.iter().all(|c| c.is_zero()) {
                return Err(RationalConstructionError::ZeroNormal(i));
            }
        }
        for (i, h) in heights.iter().enumerate() {
            if !h.is_positive() {
                return Err(RationalConstructionError::NonPositiveHeight { index: i });
            }
        }

        // Boundedness: normals must positively span R^4
        if !check_bounded_rational(&normals) {
            return Err(RationalConstructionError::Unbounded);
        }

        // Enumerate vertices exactly
        let (vertex_descriptors, vertices, all_dets, all_gaps) =
            Self::enumerate_vertices_exact(&normals, &heights)?;

        // Check irredundancy: every facet has incident vertices spanning a
        // 3D affine subspace (the facet hyperplane).
        for i in 0..f {
            let incident: Vec<[BigRational; 4]> = vertex_descriptors
                .iter()
                .zip(vertices.iter())
                .filter(|(vd, _)| vd.contains(&i))
                .map(|(_, v)| v.clone())
                .collect();

            if incident.is_empty() || affine_rank_rational(&incident) < 3 {
                return Err(RationalConstructionError::RedundantFacet(i));
            }
        }

        // Compute adjacency
        let mut adjacency = BTreeSet::new();
        for vd in &vertex_descriptors {
            let facets: Vec<_> = vd.iter().copied().collect();
            for (a, &fi) in facets.iter().enumerate() {
                for &fj in &facets[(a + 1)..] {
                    adjacency.insert((fi.min(fj), fi.max(fj)));
                }
            }
        }

        // Compute sign pattern
        let mut sign_pattern = BTreeMap::new();
        for &(i, k) in &adjacency {
            let omega = omega0_rational(&normals[i], &normals[k]);
            sign_pattern.insert((i, k), Sign::of(&omega));
        }

        // Compute margins
        let min_gap = all_gaps
            .into_iter()
            .min()
            .expect("valid polytope has at least one vertex gap");

        let min_abs_det = all_dets
            .into_iter()
            .map(|d| d.abs())
            .min()
            .expect("valid polytope has at least one vertex determinant");

        let min_omega_nonzero = sign_pattern
            .iter()
            .filter(|(_, s)| **s != Sign::Zero)
            .map(|(&(i, k), _)| omega0_rational(&normals[i], &normals[k]).abs())
            .min();

        let margins = Margins {
            min_gap,
            min_abs_det,
            min_omega_nonzero,
        };

        let data = CombinatorialData {
            num_facets: f,
            vertex_descriptors: vertex_descriptors.clone(),
            adjacency,
            sign_pattern,
            margins,
        };

        Ok(RationalPolytope4D {
            normals,
            heights,
            vertices,
            data,
        })
    }

    /// Enumerate all vertices by testing all C(F, 4) subsets exactly.
    ///
    /// For each 4-element subset S:
    /// 1. Compute det(N_S) exactly. If zero, skip (singular).
    /// 2. Solve N_S · v = h_S exactly via Cramer's rule.
    /// 3. Check all gaps gⱼⁱ = hᵢ - ⟨nᵢ, v⟩ ≥ 0 for i ∉ S (exact).
    /// 4. If all non-negative → v is a vertex; its descriptor is the set of
    ///    ALL facets with gap = 0 (including the defining subset S).
    ///
    /// Non-simple vertices (on >4 facets) are handled by merging: the first
    /// 4-subset that discovers a vertex records ALL incident facets. Later
    /// subsets that yield the same vertex are skipped (already discovered).
    ///
    /// Returns (vertex_descriptors, vertices, dets_of_vertices, all_positive_gaps).
    fn enumerate_vertices_exact(
        normals: &[[BigRational; 4]],
        heights: &[BigRational],
    ) -> Result<VertexEnumResult, RationalConstructionError> {
        let f = normals.len();
        let mut vertex_descriptors = Vec::new();
        let mut vertices = Vec::new();
        let mut dets = Vec::new();
        let mut gaps = Vec::new();

        for subset in combinations4(f) {
            let rows: [[BigRational; 4]; 4] = [
                normals[subset[0]].clone(),
                normals[subset[1]].clone(),
                normals[subset[2]].clone(),
                normals[subset[3]].clone(),
            ];

            let rhs: [BigRational; 4] = [
                heights[subset[0]].clone(),
                heights[subset[1]].clone(),
                heights[subset[2]].clone(),
                heights[subset[3]].clone(),
            ];

            let d = det4(&rows);
            if d.is_zero() {
                continue; // Singular — skip
            }

            // Solve exactly
            let v = solve4(&rows, &rhs).unwrap(); // safe: det ≠ 0

            // Check all gaps. gap > 0 means the facet is non-incident.
            // gap = 0 means the vertex lies on this facet too (non-simple).
            // gap < 0 means the point is outside K (not a vertex).
            let mut all_nonnegative = true;
            let mut incident_facets = BTreeSet::from(subset);
            let mut subset_gaps = Vec::new();
            for i in 0..f {
                if subset.contains(&i) {
                    continue;
                }
                let gap = &heights[i] - dot4(&normals[i], &v);
                if gap.is_negative() {
                    all_nonnegative = false;
                    break;
                }
                if gap.is_zero() {
                    incident_facets.insert(i);
                } else {
                    subset_gaps.push(gap);
                }
            }

            if !all_nonnegative {
                continue; // Point is outside K
            }

            // Check if this vertex was already discovered by an earlier subset
            // (happens for non-simple polytopes where multiple 4-subsets of the
            // incident facets yield the same vertex).
            let already_found = vertices.iter().any(|existing: &[BigRational; 4]| {
                (0..4).all(|i| existing[i] == v[i])
            });
            if already_found {
                continue;
            }

            vertex_descriptors.push(incident_facets);
            vertices.push(v);
            dets.push(d);
            gaps.extend(subset_gaps);
        }

        if vertex_descriptors.is_empty() {
            return Err(RationalConstructionError::NoVertices);
        }

        Ok((vertex_descriptors, vertices, dets, gaps))
    }

    /// Lossless conversion from f64 polytope to exact rational coordinates.
    ///
    /// Every finite f64 is an exact rational number (m · 2^e for integer m
    /// and exponent e). This method extracts that exact rational from the
    /// IEEE-754 bit representation — no rounding, no precision loss.
    ///
    /// The input normals need not be unit vectors. If they are unit vectors
    /// (as in `Polytope4D`), the resulting rational normals will have
    /// denominators up to 2^52 — large but exact.
    pub fn from_f64(
        normals: &[nalgebra::Vector4<f64>],
        heights: &[f64],
    ) -> Result<Self, RationalConstructionError> {
        let rational_normals: Vec<[BigRational; 4]> = normals
            .iter()
            .map(|n| std::array::from_fn(|i| f64_to_rational(n[i])))
            .collect();

        let rational_heights: Vec<BigRational> =
            heights.iter().map(|&h| f64_to_rational(h)).collect();

        Self::new(rational_normals, rational_heights)
    }

    /// Round an f64 polytope to rational coordinates with the given denominator.
    ///
    /// Each coordinate x is mapped to round(x × D) / D. This is **lossy**:
    /// for D = 1000, only ~3 decimal digits are preserved.
    ///
    /// Prefer [`from_f64`](Self::from_f64) for lossless conversion.
    /// Use this only when you want smaller denominators (e.g. for readable
    /// output or faster exact arithmetic on very large polytopes).
    ///
    /// # Panics
    ///
    /// `denominator` must be ≤ 2^52 (otherwise `round() as i64` overflows
    /// for unit-magnitude coordinates).
    pub fn from_f64_rounded(
        normals: &[nalgebra::Vector4<f64>],
        heights: &[f64],
        denominator: u64,
    ) -> Result<Self, RationalConstructionError> {
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

        Self::new(rational_normals, rational_heights)
    }

    /// Perturb normals to break ω₀ = 0 degeneracies.
    ///
    /// Returns a NEW `RationalPolytope4D` whose normals are randomly perturbed
    /// by magnitude ~2^{-perturbation_bits}. Heights are unchanged. The
    /// perturbed polytope is re-enumerated from scratch (new vertices, incidence,
    /// signs).
    ///
    /// Typical usage: `perturbation_bits = 64` gives perturbations ~2^{-64},
    /// far below f64 epsilon (2^{-52}), so the f64 representation is unchanged.
    ///
    /// Post-condition: all ω₀(nᵢ, nₖ) ≠ 0 (returns `PerturbationFailed` if
    /// not, which is astronomically unlikely for random perturbations).
    pub fn perturbed(
        &self,
        rng: &mut impl rand::Rng,
        perturbation_bits: u32,
    ) -> Result<Self, RationalConstructionError> {
        let perturbed_normals: Vec<[BigRational; 4]> = self.normals
            .iter()
            .map(|n| {
                std::array::from_fn(|i| {
                    &n[i] + random_small_rational(rng, perturbation_bits)
                })
            })
            .collect();

        let result = Self::new(perturbed_normals, self.heights.clone())?;

        // Verify post-condition: no ω₀ = 0
        if result.data.sign_pattern.values().any(|s| *s == Sign::Zero) {
            return Err(RationalConstructionError::PerturbationFailed);
        }

        Ok(result)
    }

    /// Convert to f64 Polytope4D.
    ///
    /// Normalizes each rational normal to a unit vector:
    ///   n̂ᵢ = nᵢ / ‖nᵢ‖,  ĥᵢ = hᵢ / ‖nᵢ‖
    ///
    /// The halfspace ⟨nᵢ, x⟩ ≤ hᵢ is equivalent to ⟨n̂ᵢ, x⟩ ≤ ĥᵢ.
    /// ‖n‖² is computed exactly over Q, then converted to f64 for sqrt.
    /// Convert to f64, running the rational pipeline again via `Polytope4D::new()`.
    ///
    /// This is the simple path: the returned `Polytope4D` recomputes its own
    /// `CombinatorialData` from the f64 values. Use [`to_f64_with_data()`] to
    /// attach this `RationalPolytope4D`'s pre-computed data instead.
    pub fn to_f64(&self) -> Result<Polytope4D, RationalConstructionError> {
        let (f64_normals, f64_heights) = self.to_f64_normals_heights()?;
        Polytope4D::new(f64_normals, f64_heights).map_err(|e| {
            RationalConstructionError::F64Conversion(format!("{e}"))
        })
    }

    /// Convert to f64, attaching this polytope's exact combinatorial data.
    ///
    /// Avoids the redundant `from_f64()` call that `to_f64()` → `Polytope4D::new()`
    /// would trigger. Prefer this when you already have a `RationalPolytope4D`
    /// and want to avoid recomputing the rational pipeline.
    pub fn to_f64_with_data(&self) -> Result<Polytope4D, RationalConstructionError> {
        let (f64_normals, f64_heights) = self.to_f64_normals_heights()?;
        let f64_vertices = self.vertices_to_f64();
        Polytope4D::new_with_exact_data(f64_normals, f64_heights, f64_vertices, self.data.clone())
            .map_err(|e| RationalConstructionError::F64Conversion(format!("{e}")))
    }

    /// Helper: convert rational normals/heights to f64 unit normals and heights.
    fn to_f64_normals_heights(
        &self,
    ) -> Result<(Vec<nalgebra::Vector4<f64>>, Vec<f64>), RationalConstructionError> {
        let mut f64_normals = Vec::with_capacity(self.normals.len());
        let mut f64_heights = Vec::with_capacity(self.heights.len());

        for (n, h) in self.normals.iter().zip(self.heights.iter()) {
            let norm_sq_exact = dot4(n, n);
            let norm = rational_to_f64(&norm_sq_exact).sqrt();

            if norm < 1e-15 {
                return Err(RationalConstructionError::F64Conversion(format!(
                    "normal has near-zero f64 norm: {norm}"
                )));
            }

            let unit_n = nalgebra::Vector4::new(
                rational_to_f64(&n[0]) / norm,
                rational_to_f64(&n[1]) / norm,
                rational_to_f64(&n[2]) / norm,
                rational_to_f64(&n[3]) / norm,
            );
            let unit_h = rational_to_f64(h) / norm;

            f64_normals.push(unit_n);
            f64_heights.push(unit_h);
        }

        Ok((f64_normals, f64_heights))
    }

    /// The exact combinatorial data.
    pub fn combinatorial_data(&self) -> &CombinatorialData {
        &self.data
    }

    /// The exact rational normals (not unit).
    pub fn normals(&self) -> &[[BigRational; 4]] {
        &self.normals
    }

    /// The exact rational heights.
    pub fn heights(&self) -> &[BigRational] {
        &self.heights
    }

    /// The exact rational vertices (same order as vertex descriptors).
    pub fn vertices(&self) -> &[[BigRational; 4]] {
        &self.vertices
    }

    /// Number of facets.
    pub fn num_facets(&self) -> usize {
        self.normals.len()
    }

    /// Number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Convert exact rational vertices to f64 vectors.
    ///
    /// Each rational coordinate is converted via [`rational_to_f64`] to the
    /// nearest f64 approximation. Rounding error is bounded by machine epsilon
    /// times the coordinate magnitude (~1e-16 for O(1) coordinates).
    ///
    /// Note: even when normals/heights originate from `from_f64` (lossless),
    /// vertex coordinates are computed by Cramer's rule and have arbitrary
    /// rational denominators, so this conversion is not lossless in general.
    ///
    /// The returned vectors are in the same order as [`vertices()`](Self::vertices)
    /// and [`combinatorial_data().vertex_descriptors`](CombinatorialData::vertex_descriptors).
    pub fn vertices_to_f64(&self) -> Vec<nalgebra::Vector4<f64>> {
        self.vertices
            .iter()
            .map(|rv| {
                nalgebra::Vector4::new(
                    rational_to_f64(&rv[0]),
                    rational_to_f64(&rv[1]),
                    rational_to_f64(&rv[2]),
                    rational_to_f64(&rv[3]),
                )
            })
            .collect()
    }
}

/// Convert a BigRational to f64 (best approximation).
///
/// For rationals with power-of-2 denominators (the common case from
/// [`f64_to_rational`]), the division is exact in f64 arithmetic and
/// recovers the original value. For general BigRationals with large
/// numerators/denominators, this produces the nearest f64 approximation.
fn rational_to_f64(r: &BigRational) -> f64 {
    // For rationals with power-of-2 denominators (common case: from f64_to_rational),
    // numer / denom is exact when both fit in f64 mantissa.
    // For larger values, the division still produces the best f64 approximation.
    use num_traits::ToPrimitive;
    let numer: f64 = r.numer().to_f64().unwrap_or(f64::NAN);
    let denom: f64 = r.denom().to_f64().unwrap_or(f64::NAN);
    numer / denom
}

/// Lossless conversion from f64 to exact BigRational.
///
/// Every finite f64 is exactly m · 2^e for some integer mantissa m and
/// exponent e. This function extracts (m, e) from the IEEE-754 bit
/// representation and constructs the exact rational m / 2^(-e) or m · 2^e.
///
/// Panics on NaN or infinity.
pub fn f64_to_rational(x: f64) -> BigRational {
    assert!(x.is_finite(), "f64_to_rational: input must be finite, got {x}");
    if x == 0.0 {
        return BigRational::zero();
    }
    let bits = x.to_bits();
    let sign = if bits >> 63 == 0 { 1i64 } else { -1i64 };
    let exponent = ((bits >> 52) & 0x7FF) as i64;
    let mantissa = if exponent == 0 {
        // Subnormal: mantissa without implicit 1
        (bits & 0x000F_FFFF_FFFF_FFFF) as i64
    } else {
        // Normal: mantissa with implicit 1
        ((bits & 0x000F_FFFF_FFFF_FFFF) | 0x0010_0000_0000_0000) as i64
    };
    let e = if exponent == 0 {
        1 - 1023 - 52 // Subnormal exponent
    } else {
        exponent - 1023 - 52 // Normal exponent
    };

    let numer = BigInt::from(sign * mantissa);
    if e >= 0 {
        let scale = BigInt::from(1u64) << (e as u64);
        BigRational::new(numer * scale, BigInt::from(1))
    } else {
        let scale = BigInt::from(1u64) << ((-e) as u64);
        BigRational::new(numer, scale)
    }
}

/// Generate a random rational number with magnitude < 2^{-bits}.
///
/// Uses uniform random numerator in [-2^32, 2^32) and denominator 2^{bits+32}.
/// This gives numbers like k / 2^{bits+32} for random k, which are exact
/// rationals with bounded denominator size.
fn random_small_rational(rng: &mut impl rand::Rng, bits: u32) -> BigRational {
    let numer: i64 = rng.gen_range(-(1i64 << 32)..(1i64 << 32));
    let denom = BigInt::from(1u64) << (bits as u64 + 32);
    BigRational::new(BigInt::from(numer), denom)
}

/// Helper: create a BigRational from an integer.
pub fn rat(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

/// Helper: create a BigRational from a fraction.
pub fn frac(numer: i64, denom: i64) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

#[cfg(test)]
#[path = "rational_test.rs"]
mod rational_test;
