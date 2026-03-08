//! Exact rational arithmetic for polytope combinatorial data.
//!
//! Provides the exact arithmetic pipeline used by [`Polytope4D`](super::polytope::Polytope4D)
//! at construction time. The pipeline takes dual vertices y_i ∈ K° (vertices of
//! the polar body) and computes:
//!
//! - **Vertices** of K, by solving y_i · x = 1 for all C(F,4) four-element subsets
//! - **Vertex–facet incidence**: which vertices lie on which facets
//!
//! Adjacency and ω₀ signs are computed by `Polytope4D::from_dual_vertices` from
//! the incidence data and dual vertices.
//!
//! The f64 representation (unit normals, heights, vertices) is derived for
//! numerical algorithms. The rational coordinates are the source of truth
//! for all discrete/combinatorial decisions.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use std::collections::BTreeSet;

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

/// Errors during rational polytope construction.
#[derive(Clone, Debug)]
pub enum RationalConstructionError {
    /// Fewer than 5 facets (cannot form bounded 4D polytope).
    TooFewFacets(usize),
    /// A dual vertex is the zero vector.
    ZeroDualVertex(usize),
    /// Dual vertices do not positively span R^4 (polytope is unbounded).
    Unbounded,
    /// No vertices found — the halfspaces are inconsistent.
    NoVertices,
    /// Facet is redundant: no incident vertices, or incident vertices
    /// have affine rank < 3 (don't span the facet hyperplane).
    RedundantFacet(usize),
    /// Conversion to f64 failed.
    F64Conversion(String),
}

impl std::fmt::Display for RationalConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFacets(n) => write!(f, "need ≥5 facets, got {n}"),
            Self::ZeroDualVertex(i) => write!(f, "dual vertex[{i}] is the zero vector"),
            Self::Unbounded => {
                write!(f, "dual vertices do not positively span R^4 (unbounded)")
            }
            Self::NoVertices => write!(f, "no vertices found (inconsistent halfspaces)"),
            Self::RedundantFacet(i) => {
                write!(f, "facet {i} is redundant (no incident vertices, or affine rank < 3)")
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
    let c00 = det3(
        &[b[1].clone(), b[2].clone(), b[3].clone()],
        &[c[1].clone(), c[2].clone(), c[3].clone()],
        &[d[1].clone(), d[2].clone(), d[3].clone()],
    );
    let c01 = det3(
        &[b[0].clone(), b[2].clone(), b[3].clone()],
        &[c[0].clone(), c[2].clone(), c[3].clone()],
        &[d[0].clone(), d[2].clone(), d[3].clone()],
    );
    let c02 = det3(
        &[b[0].clone(), b[1].clone(), b[3].clone()],
        &[c[0].clone(), c[1].clone(), c[3].clone()],
        &[d[0].clone(), d[1].clone(), d[3].clone()],
    );
    let c03 = det3(
        &[b[0].clone(), b[1].clone(), b[2].clone()],
        &[c[0].clone(), c[1].clone(), c[2].clone()],
        &[d[0].clone(), d[1].clone(), d[2].clone()],
    );

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
/// For dual vertices y_i = n_i/h_i: sign(ω₀(y_i, y_k)) = sign(ω₀(n_i, n_k))
/// since h_i, h_k > 0.
pub(super) fn omega0_rational(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
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

/// Check that directions positively span R^4 (polytope is bounded).
///
/// K bounded ⟺ rec(K) = {0} ⟺ dual vertices positively span R^4.
/// "Positively span" means: for every nonzero d ∈ R^4, some y_i · d > 0.
///
/// Since y_i = n_i / h_i with h_i > 0, positive spanning of y_i is
/// equivalent to positive spanning of n_i.
///
/// Algorithm (exact over Q):
/// 1. Check rank(Y) = 4 via Gaussian elimination.
/// 2. For each triple (i,j,k), compute the 1D kernel direction d via
///    exact 4D cross product. If d = 0 (dependent triple), skip.
///    Check some y_l · d > 0 and some y_l · d < 0 among y_l ∉ {i,j,k}.
///
/// Complexity: O(F⁴) — F³ triples × F inner products each.
pub(super) fn check_bounded_rational(dual_vertices: &[[BigRational; 4]]) -> bool {
    let f = dual_vertices.len();

    if rank_over_q(dual_vertices) < 4 {
        return false;
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let d = cross_product_4d_rational(
                    &dual_vertices[i],
                    &dual_vertices[j],
                    &dual_vertices[k],
                );
                if d.iter().all(|c| c.is_zero()) {
                    continue; // Dependent triple
                }

                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&dual_vertices[l], &d).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4(&dual_vertices[l], &d).is_negative());

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

// ── Construction pipeline (called by Polytope4D constructors) ────────────

/// Run the rational construction pipeline: validate, enumerate vertices,
/// check irredundancy.
///
/// Takes dual vertices y_i ∈ K° and returns (primal_vertices, vertex_descriptors).
/// Each vertex descriptor is the set of facet indices incident to that vertex.
///
/// The halfspace representation is y_i · x ≤ 1 for each dual vertex y_i.
///
/// Non-simple polytopes (vertices on >4 facets) are supported: the vertex
/// descriptor records ALL incident facets, not just the defining 4-subset.
#[allow(clippy::type_complexity)]
pub(super) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>), RationalConstructionError> {
    let f = dual_vertices.len();

    // Basic validation
    if f < 5 {
        return Err(RationalConstructionError::TooFewFacets(f));
    }
    for (i, y) in dual_vertices.iter().enumerate() {
        if y.iter().all(|c| c.is_zero()) {
            return Err(RationalConstructionError::ZeroDualVertex(i));
        }
    }

    // Boundedness: dual vertices must positively span R^4
    if !check_bounded_rational(dual_vertices) {
        return Err(RationalConstructionError::Unbounded);
    }

    // Enumerate vertices exactly (solves y_i · x = 1)
    let (vertex_descriptors, vertices) = enumerate_vertices_exact(dual_vertices)?;

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

    Ok((vertices, vertex_descriptors))
}

/// Enumerate all vertices by testing all C(F, 4) subsets exactly.
///
/// For each 4-element subset S of dual vertices:
/// 1. Build the 4×4 system Y_S · x = 1 (constant RHS).
/// 2. Compute det(Y_S) exactly. If zero, skip (singular).
/// 3. Solve Y_S · x = 1 exactly via Cramer's rule.
/// 4. Check all gaps g_i = 1 - y_i · v ≥ 0 for i ∉ S (exact).
/// 5. If all non-negative → v is a vertex; its descriptor is the set of
///    ALL facets with gap = 0 (including the defining subset S).
///
/// Non-simple vertices (on >4 facets) are handled by merging: the first
/// 4-subset that discovers a vertex records ALL incident facets. Later
/// subsets that yield the same vertex are skipped (already discovered).
#[allow(clippy::type_complexity)]
fn enumerate_vertices_exact(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<BTreeSet<usize>>, Vec<[BigRational; 4]>), RationalConstructionError> {
    let f = dual_vertices.len();
    let one = BigRational::from(BigInt::from(1));
    let rhs: [BigRational; 4] = std::array::from_fn(|_| one.clone());

    let mut vertex_descriptors = Vec::new();
    let mut vertices = Vec::new();

    for subset in combinations4(f) {
        let rows: [[BigRational; 4]; 4] = [
            dual_vertices[subset[0]].clone(),
            dual_vertices[subset[1]].clone(),
            dual_vertices[subset[2]].clone(),
            dual_vertices[subset[3]].clone(),
        ];

        let d = det4(&rows);
        if d.is_zero() {
            continue; // Singular — skip
        }

        // Solve exactly: Y_S · v = 1
        let v = solve4(&rows, &rhs).unwrap(); // safe: det ≠ 0

        // Check all gaps. gap > 0 means the facet is non-incident.
        // gap = 0 means the vertex lies on this facet too (non-simple).
        // gap < 0 means the point is outside K (not a vertex).
        let mut all_nonnegative = true;
        let mut incident_facets = BTreeSet::from(subset);
        for (i, dv) in dual_vertices.iter().enumerate() {
            if subset.contains(&i) {
                continue;
            }
            let gap = &one - dot4(dv, &v);
            if gap.is_negative() {
                all_nonnegative = false;
                break;
            }
            if gap.is_zero() {
                incident_facets.insert(i);
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
    }

    if vertex_descriptors.is_empty() {
        return Err(RationalConstructionError::NoVertices);
    }

    Ok((vertex_descriptors, vertices))
}

// ── f64 conversion utilities (called by Polytope4D constructors) ─────────

/// Convert dual vertices to f64 unit normals and heights.
///
/// For each dual vertex y_i:
///   n̂_i = y_i / ‖y_i‖  (unit normal)
///   ĥ_i = 1 / ‖y_i‖    (positive height)
///
/// ‖y‖² is computed exactly over Q, then converted to f64 for sqrt.
pub(super) fn dual_vertices_to_f64(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<nalgebra::Vector4<f64>>, Vec<f64>), RationalConstructionError> {
    let mut normals = Vec::with_capacity(dual_vertices.len());
    let mut heights = Vec::with_capacity(dual_vertices.len());

    for (i, y) in dual_vertices.iter().enumerate() {
        let norm_sq_exact = dot4(y, y);
        let norm = rational_to_f64(&norm_sq_exact).sqrt();

        if norm < 1e-15 {
            return Err(RationalConstructionError::F64Conversion(format!(
                "dual vertex[{i}] has near-zero f64 norm: {norm}"
            )));
        }

        let unit_n = nalgebra::Vector4::new(
            rational_to_f64(&y[0]) / norm,
            rational_to_f64(&y[1]) / norm,
            rational_to_f64(&y[2]) / norm,
            rational_to_f64(&y[3]) / norm,
        );
        let h = 1.0 / norm;

        normals.push(unit_n);
        heights.push(h);
    }

    Ok((normals, heights))
}

/// Convert exact rational vertices to f64 vectors.
///
/// Each rational coordinate is converted via [`rational_to_f64`] to the
/// nearest f64 approximation. Rounding error is bounded by machine epsilon
/// times the coordinate magnitude (~1e-16 for O(1) coordinates).
pub(super) fn rational_vertices_to_f64(
    vertices: &[[BigRational; 4]],
) -> Vec<nalgebra::Vector4<f64>> {
    vertices
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

// ── Scalar conversion utilities ──────────────────────────────────────────

/// Convert a BigRational to f64 (best approximation).
///
/// For rationals with power-of-2 denominators (the common case from
/// [`f64_to_rational`]), the division is exact in f64 arithmetic and
/// recovers the original value. For general BigRationals with large
/// numerators/denominators, this produces the nearest f64 approximation.
pub(super) fn rational_to_f64(r: &BigRational) -> f64 {
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
    assert!(
        x.is_finite(),
        "f64_to_rational: input must be finite, got {x}"
    );
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
pub(super) fn random_small_rational(rng: &mut impl rand::Rng, bits: u32) -> BigRational {
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
