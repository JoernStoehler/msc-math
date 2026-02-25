//! Exact rational arithmetic for polytope combinatorial data.
//!
//! A [`RationalPolytope4D`] stores polytope coordinates as exact rationals
//! (not unit normals — just halfspace directions). Its combinatorial data
//! (vertex descriptors, adjacency, ω₀ signs) is computed exactly over Q
//! at construction time.
//!
//! The f64 representation (unit normals, heights) is a derived quantity
//! for fast numerical computation (KKT/SVD). The rational polytope is the
//! source of truth for all discrete/combinatorial decisions.
//!
//! # Mathematical framework
//!
//! A polytope K ⊂ R⁴ is given by halfspaces ⟨nᵢ, x⟩ ≤ hᵢ, where nᵢ ∈ Q⁴
//! (not necessarily unit) and hᵢ ∈ Q₊. The combinatorial data is:
//!
//! - **Vertex descriptors** V = {S₁, …, S_V}: for each vertex vⱼ,
//!   Sⱼ = {i : vⱼ ∈ Fᵢ} is the set of 4 facets through vⱼ.
//! - **Adjacency**: facets i, k are adjacent iff {i,k} ⊂ Sⱼ for some j.
//! - **Sign pattern** D(i,k) = sign(ω₀(nᵢ, nₖ)) ∈ {+, -, 0} for adjacent pairs.
//!
//! All computed exactly over Q — no numerical predicates, no tolerances.
//!
//! # Margins
//!
//! [`Margins`] records the minimum values of key quantities:
//! - `min_gap`: smallest non-incidence gap gⱼⁱ = hᵢ - ⟨nᵢ, vⱼ⟩ > 0
//! - `min_abs_det`: smallest |det(N_S)| for vertex subsets S
//! - `min_omega_nonzero`: smallest |ω₀(nᵢ, nₖ)| for non-zero sign entries
//!
//! These are exact rational values. When margins ≫ f64 rounding error,
//! the f64 polytope has the same combinatorial type.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
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

/// Exact combinatorial data computed over Q.
///
/// Every field is determined exactly from rational coordinates.
/// No numerical predicates, no tolerances.
#[derive(Clone, Debug)]
pub struct CombinatorialData {
    /// Number of facets.
    pub num_facets: usize,
    /// Vertex descriptors: each vertex is the intersection of exactly 4 facets.
    /// Sorted lexicographically.
    pub vertex_descriptors: Vec<BTreeSet<usize>>,
    /// Adjacent facet pairs (i, k) with i < k.
    pub adjacency: BTreeSet<(usize, usize)>,
    /// Sign of ω₀(nᵢ, nₖ) for each adjacent pair (i, k) with i < k.
    pub sign_pattern: BTreeMap<(usize, usize), Sign>,
    /// Exact rational margins.
    pub margins: Margins,
}

/// Exact rational margins — the "safety buffer" for each combinatorial predicate.
///
/// If all margins are much larger than f64 rounding error (~1e-15 × scale),
/// then the f64 polytope has the same combinatorial data.
#[derive(Clone, Debug)]
pub struct Margins {
    /// Smallest non-incidence gap: min over all vertices j and non-incident facets i
    /// of gⱼⁱ = hᵢ - ⟨nᵢ, vⱼ⟩. Always positive for valid polytopes.
    pub min_gap: BigRational,
    /// Smallest absolute determinant |det(N_S)| over vertex subsets S.
    /// Positive iff all vertex matrices are non-singular (which they are).
    pub min_abs_det: BigRational,
    /// Smallest |ω₀(nᵢ, nₖ)| over adjacent pairs with D(i,k) ≠ 0.
    /// None if all adjacent pairs have ω₀ = 0.
    pub min_omega_nonzero: Option<BigRational>,
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
    /// No vertices found — the halfspaces don't form a bounded polytope,
    /// or they are inconsistent.
    NoVertices,
    /// A facet has no incident vertices (redundant facet).
    RedundantFacet(usize),
    /// Polytope is not simple: some vertex lies on more than 4 facets.
    NotSimple {
        vertex_index: usize,
        facets: BTreeSet<usize>,
    },
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
            Self::NoVertices => write!(f, "no vertices found (unbounded or inconsistent)"),
            Self::RedundantFacet(i) => write!(f, "facet {i} has no incident vertices"),
            Self::NotSimple {
                vertex_index,
                facets,
            } => {
                write!(
                    f,
                    "not simple: vertex {vertex_index} on {} facets ({:?})",
                    facets.len(),
                    facets
                )
            }
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
    /// - No vertices found (unbounded or inconsistent)
    /// - Any facet is redundant (no incident vertices)
    /// - Not simple (some vertex on more than 4 facets)
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

        // Enumerate vertices exactly
        let (vertex_descriptors, vertices, all_dets, all_gaps) =
            Self::enumerate_vertices_exact(&normals, &heights)?;

        // Check simplicity: look for points on >4 facets.
        // For each candidate 5-subset, if the intersection point (if unique)
        // lies inside K, then it's on ≥5 facets → not simple.
        // But simpler: we already enumerated all 4-subsets. A vertex on >4 facets
        // would appear as multiple 4-subsets sharing the same point.
        // We check by looking for duplicate vertices among the enumerated ones.
        for (idx, vd) in vertex_descriptors.iter().enumerate() {
            // Count how many facets are incident (gap = 0)
            let mut incident_facets = vd.clone();
            for i in 0..f {
                if vd.contains(&i) {
                    continue;
                }
                let gap = &heights[i] - dot4(&normals[i], &vertices[idx]);
                if gap.is_zero() {
                    incident_facets.insert(i);
                }
            }
            if incident_facets.len() > 4 {
                return Err(RationalConstructionError::NotSimple {
                    vertex_index: idx,
                    facets: incident_facets,
                });
            }
        }

        // Check irredundancy: every facet has at least one vertex
        for i in 0..f {
            if !vertex_descriptors.iter().any(|vd| vd.contains(&i)) {
                return Err(RationalConstructionError::RedundantFacet(i));
            }
        }

        // Compute adjacency
        let mut adjacency = BTreeSet::new();
        for vd in &vertex_descriptors {
            let facets: Vec<_> = vd.iter().copied().collect();
            for a in 0..facets.len() {
                for b in (a + 1)..facets.len() {
                    let (i, k) = (facets[a], facets[b]);
                    adjacency.insert((i.min(k), i.max(k)));
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
            .unwrap_or_else(BigRational::one);

        let min_abs_det = all_dets
            .into_iter()
            .map(|d| d.abs())
            .min()
            .unwrap_or_else(BigRational::one);

        let min_omega_nonzero = sign_pattern
            .values()
            .zip(adjacency.iter())
            .filter(|(s, _)| **s != Sign::Zero)
            .map(|(_, &(i, k))| omega0_rational(&normals[i], &normals[k]).abs())
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
    /// 3. Check all gaps gⱼⁱ = hᵢ - ⟨nᵢ, v⟩ > 0 for i ∉ S (exact).
    /// 4. If all positive → S is a vertex descriptor, v is the vertex.
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

            // Check all gaps
            let mut all_positive = true;
            let mut subset_gaps = Vec::new();
            for i in 0..f {
                if subset.contains(&i) {
                    continue;
                }
                let gap = &heights[i] - dot4(&normals[i], &v);
                if !gap.is_positive() {
                    all_positive = false;
                    break;
                }
                subset_gaps.push(gap);
            }

            if all_positive {
                vertex_descriptors.push(BTreeSet::from(subset));
                vertices.push(v);
                dets.push(d);
                gaps.extend(subset_gaps);
            }
        }

        if vertex_descriptors.is_empty() {
            return Err(RationalConstructionError::NoVertices);
        }

        Ok((vertex_descriptors, vertices, dets, gaps))
    }

    /// Round an f64 polytope to rational coordinates with the given denominator.
    ///
    /// Each coordinate x is mapped to round(x × D) / D. This produces
    /// rational normals (not unit) and rational heights.
    ///
    /// The denominator D controls precision: larger D = closer to f64 values,
    /// but larger rational numerators. D ≤ 100_000 is recommended.
    pub fn from_f64_rounded(
        normals: &[nalgebra::Vector4<f64>],
        heights: &[f64],
        denominator: u64,
    ) -> Result<Self, RationalConstructionError> {
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

    /// Convert to f64 Polytope4D.
    ///
    /// Normalizes each rational normal to a unit vector:
    ///   n̂ᵢ = nᵢ / ‖nᵢ‖,  ĥᵢ = hᵢ / ‖nᵢ‖
    ///
    /// The halfspace ⟨nᵢ, x⟩ ≤ hᵢ is equivalent to ⟨n̂ᵢ, x⟩ ≤ ĥᵢ.
    pub fn to_f64(&self) -> Result<Polytope4D, RationalConstructionError> {
        let mut f64_normals = Vec::with_capacity(self.normals.len());
        let mut f64_heights = Vec::with_capacity(self.heights.len());

        for (n, h) in self.normals.iter().zip(self.heights.iter()) {
            // Compute ‖n‖² exactly, then take f64 sqrt
            let norm_sq: f64 = n
                .iter()
                .map(|c| {
                    let f = rational_to_f64(c);
                    f * f
                })
                .sum();
            let norm = norm_sq.sqrt();

            if norm < 1e-15 {
                return Err(RationalConstructionError::F64Conversion(format!(
                    "normal has near-zero f64 norm: {norm}"
                )));
            }

            let unit_n =
                nalgebra::Vector4::new(
                    rational_to_f64(&n[0]) / norm,
                    rational_to_f64(&n[1]) / norm,
                    rational_to_f64(&n[2]) / norm,
                    rational_to_f64(&n[3]) / norm,
                );
            let unit_h = rational_to_f64(h) / norm;

            f64_normals.push(unit_n);
            f64_heights.push(unit_h);
        }

        Polytope4D::new(f64_normals, f64_heights).map_err(|e| {
            RationalConstructionError::F64Conversion(format!("{e}"))
        })
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
}

/// Convert a BigRational to f64 (best approximation).
fn rational_to_f64(r: &BigRational) -> f64 {
    // num-rational doesn't have a direct to_f64, so we use
    // numer/denom conversion via string parsing or big integer division.
    let numer: f64 = r.numer().to_string().parse().unwrap_or(f64::NAN);
    let denom: f64 = r.denom().to_string().parse().unwrap_or(f64::NAN);
    numer / denom
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
