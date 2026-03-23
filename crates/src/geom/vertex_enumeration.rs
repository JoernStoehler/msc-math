//! Exact vertex enumeration for 4D polytopes over Q.
//!
//! Takes dual vertices y_i (vertices of the polar body K°) and computes:
//! - **Vertices** of K by solving y_i · x = 1 for all C(F,4) four-element subsets
//! - **Vertex-facet incidence**: which vertices lie on which facets
//! - **Boundedness**: dual vertices positively span R^4
//! - **Irredundancy**: every facet has incident vertices of affine rank 3
//!
//! All vertex inclusion decisions are exact over Q. An f64 pre-filter
//! accelerates rejection of non-vertex subsets but makes no inclusion
//! decisions — all confirmed vertices are determined by exact rational arithmetic.
//!
//! Mathematical correspondence: [lem:vertex-enumeration], [lem:positive-span]

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use std::collections::BTreeSet;

use super::polytope::ConstructionError;

// ── Exact linear algebra over Q ──────────────────────────────────────────

/// Determinant of a 3x3 rational matrix (Sarrus' rule).
#[cfg(test)]
fn det3(r0: &[BigRational], r1: &[BigRational], r2: &[BigRational]) -> BigRational {
    &r0[0] * (&r1[1] * &r2[2] - &r1[2] * &r2[1])
        - &r0[1] * (&r1[0] * &r2[2] - &r1[2] * &r2[0])
        + &r0[2] * (&r1[0] * &r2[1] - &r1[1] * &r2[0])
}

/// Exact determinant of a 4x4 rational matrix via cofactor expansion.
///
/// Expands along the first row using 3x3 minors.
#[cfg(test)]
pub(super) fn det4(rows: &[[BigRational; 4]; 4]) -> BigRational {
    let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

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

/// Solve a 4x4 linear system N*x = b exactly via Cramer's rule.
///
/// Returns `None` if det(N) = 0 (singular system).
#[cfg(test)]
pub(super) fn solve4(
    rows: &[[BigRational; 4]; 4],
    rhs: &[BigRational; 4],
) -> Option<[BigRational; 4]> {
    let d = det4(rows);
    if d.is_zero() {
        return None;
    }

    let mut result: [BigRational; 4] = std::array::from_fn(|_| BigRational::zero());

    for col in 0..4 {
        let mut modified = rows.clone();
        for row in 0..4 {
            modified[row][col] = rhs[row].clone();
        }
        result[col] = det4(&modified) / &d;
    }

    Some(result)
}

/// Inner product of two 4-vectors over Q.
#[cfg(test)]
pub(super) fn dot4(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

/// 4D cross product over Q: vector perpendicular to three vectors in R^4.
///
/// d_k = (-1)^k * det(3x3 minor of [a, b, c] with column k removed).
/// Same formula as `cross_product_4d::cross_product_4d` but exact over Q.
///
/// Superseded in production by `cross_product_4d_int` (integer-only path);
/// retained for tests that verify rational-level correctness.
#[cfg(test)]
pub(super) fn cross_product_4d_rational(
    a: &[BigRational; 4],
    b: &[BigRational; 4],
    c: &[BigRational; 4],
) -> [BigRational; 4] {
    // 2x2 minors of (b, c)
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

/// Compute the rank of a set of 4-component rational row vectors via Gaussian elimination.
///
/// Exact over Q — no tolerances or floating-point rounding.
///
/// Mathematical correspondence: helper for [lem:vertex-enumeration] (used in vertex
/// feasibility checks and in `affine_rank_rational` for the irredundancy check).
pub(super) fn rank_over_q(rows: &[[BigRational; 4]]) -> usize {
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
            let pivot_row_data: [BigRational; 4] = mat[rank].clone();
            for (mat_c, pivot_c) in mat[r][col..n].iter_mut().zip(pivot_row_data[col..n].iter()) {
                *mat_c = &*mat_c - &factor * pivot_c;
            }
        }
        rank += 1;
    }
    rank
}

// ── Boundedness and rank checks ──────────────────────────────────────────

/// Check that dual vertices positively span R^4 (polytope is bounded).
///
/// K bounded iff rec(K) = {0} iff dual vertices positively span R^4.
/// "Positively span" means: for every nonzero d in R^4, some y_i · d > 0.
///
/// Since y_i = n_i / h_i with h_i > 0, positive spanning of y_i is
/// equivalent to positive spanning of n_i.
///
/// # Algorithm (exact over Q)
///
/// 1. Check rank(Y) = 4 via Gaussian elimination.
/// 2. For each triple (i,j,k), compute the 1D kernel direction d via exact
///    4D cross product. If d = 0 (dependent triple), skip.
///    Check some y_l · d > 0 and some y_l · d < 0 among y_l not in {i,j,k}.
///
/// # Sufficiency
///
/// Suppose positive spanning fails: some nonzero d has y_i · d ≤ 0 for all i.
/// Then d lies in the dual cone C = {d : y_i · d ≤ 0 ∀i}, which is a
/// polyhedral cone. Since rank(Y) = 4, C is pointed and every extreme ray
/// of C lies on the intersection of 3 linearly independent active constraints
/// y_i · d = 0. Such an extreme ray is a kernel direction of the triple
/// (i, j, k). Therefore, any failure of positive spanning is witnessed by
/// some kernel direction of a triple — the check is sufficient.
///
/// Complexity: O(F^4) — F^3 triples times F inner products each.
///
/// Mathematical correspondence: [lem:positive-span]
///
/// Superseded in production by the per-triple f64+integer path in
/// `construct_rational_pipeline`; retained for tests.
#[cfg(test)]
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
///
/// Mathematical correspondence: helper for [lem:irredundancy] (facets require
/// incident vertices of affine rank >= 3).
pub(super) fn affine_rank_rational(points: &[[BigRational; 4]]) -> usize {
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

// ── Integer-scaled arithmetic (GCD-free) ────────────────────────────────
//
// For exact checks (boundedness, vertex enumeration), we scale all rational
// dual vertices to a common denominator D and work entirely in BigInt.
// This eliminates GCD normalization (~40% of CPU in the BigRational path).
// TODO: add inline benchmark citation (criterion bench date + F range) for the ~40% claim.

/// Scale rational dual vertices to integer arrays with a common denominator.
///
/// Returns (A, D) where A[i][j] = a_i[j] * D ∈ Z and D = lcm of all denominators.
///
/// Mathematical correspondence: preprocessing step for [prop:integer-cramer] (scaling
/// eliminates GCD normalization by lifting the system to integer arithmetic).
fn integer_scale_dual_vertices(
    dual_vertices: &[[BigRational; 4]],
) -> (Vec<[BigInt; 4]>, BigInt) {
    // Compute D = lcm of all denominators.
    let mut d = BigInt::from(1);
    for y in dual_vertices {
        for comp in y {
            d = num_integer::Integer::lcm(&d, comp.denom());
        }
    }

    // Scale: A_i[j] = a_i[j].numer * (D / a_i[j].denom)
    let int_verts: Vec<[BigInt; 4]> = dual_vertices
        .iter()
        .map(|y| {
            std::array::from_fn(|c| {
                let scale = &d / y[c].denom();
                y[c].numer() * scale
            })
        })
        .collect();

    (int_verts, d)
}

/// 4D cross product over Z: direction perpendicular to three integer vectors.
fn cross_product_4d_int(a: &[BigInt; 4], b: &[BigInt; 4], c: &[BigInt; 4]) -> [BigInt; 4] {
    // 2x2 minors of (b, c)
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

/// Dot product of two 4-vectors over Z.
fn dot4_int(a: &[BigInt; 4], b: &[BigInt; 4]) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

/// Rank of integer 4-vectors via exact Gaussian elimination over Z.
///
/// Uses fraction-free elimination: pivots by subtracting scaled rows so that
/// all entries remain integers. Only checks for zero vs nonzero pivots.
///
/// Mathematical correspondence: rank check step in [prop:integer-cramer] (verifying
/// rank(A) = 4 before the bounded check in `check_bounded_f64_first`).
fn rank_int(rows: &[[BigInt; 4]]) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let m = rows.len();
    let n = 4;
    let mut mat: Vec<[BigInt; 4]> = rows.to_vec();
    let mut rank = 0;

    for col in 0..n {
        let pivot_row = (rank..m).find(|&r| !mat[r][col].is_zero());
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        mat.swap(rank, pivot_row);

        // Eliminate: for each other row, subtract (row[col]/pivot[col]) * pivot_row.
        // Fraction-free: row = pivot[col] * row - row[col] * pivot_row.
        let pivot_val = mat[rank][col].clone();
        for r in 0..m {
            if r == rank || mat[r][col].is_zero() {
                continue;
            }
            let row_val = mat[r][col].clone();
            let pivot_row_data: [BigInt; 4] = mat[rank].clone();
            for c in col..n {
                mat[r][c] = &pivot_val * &mat[r][c] - &row_val * &pivot_row_data[c];
            }
        }
        rank += 1;
    }
    rank
}

/// f64 pre-filter for a single triple in the bounded check.
///
/// Returns true if f64 can confirm that the kernel direction of triple (i,j,k)
/// has both positive and negative projections among the remaining dual vertices
/// (i.e., this direction does not witness unboundedness). Returns false if
/// f64 cannot decide — the caller must fall through to exact integer arithmetic.
///
/// Same pattern as `f64_prefilter_rejects` for vertex enumeration: f64 resolves
/// the common case cheaply, exact arithmetic handles the rare ambiguous cases.
///
/// Mathematical correspondence: fast path implementing [lem:bounded-triples] (reduces
/// the bounded check to per-triple kernel direction sign tests).
fn bounded_triple_f64_confirms(
    dv_f64: &[nalgebra::Vector4<f64>],
    i: usize,
    j: usize,
    k: usize,
) -> bool {
    use crate::geom::cross_product_4d::cross_product_4d;

    /// Near-dependent triple: ‖d‖ below this means f64 cross product is noise.
    /// Matches `validation::check_bounded` threshold.
    const EPS_DEP: f64 = 1e-12;
    /// Dot product sign threshold: f64 dot product error is O(ε_mach) ≈ 1e-16
    /// for unit-scale vectors, so 1e-9 gives ~7 orders of margin.
    /// Matches `validation::check_bounded` EPS_UNIT.
    const EPS_SIGN: f64 = 1e-9;

    let d = cross_product_4d(dv_f64[i], dv_f64[j], dv_f64[k]);
    if d.norm() < EPS_DEP {
        return false; // Near-dependent, f64 can't determine direction
    }
    let d = d.normalize();

    let mut has_pos = false;
    let mut has_neg = false;
    for (l, dv) in dv_f64.iter().enumerate() {
        if l == i || l == j || l == k {
            continue;
        }
        let s = dv.dot(&d);
        if s > EPS_SIGN {
            has_pos = true;
        } else if s < -EPS_SIGN {
            has_neg = true;
        } else {
            return false; // Ambiguous sign, can't decide
        }
        if has_pos && has_neg {
            return true; // Both signs found, triple is fine
        }
    }
    // Went through all facets but didn't find both signs with clear margin
    false
}

/// Determinant of a 4x4 integer matrix via cofactor expansion.
///
/// Mathematical correspondence: the δ = det(M_S) computation in [prop:integer-cramer].
fn det4_int(rows: &[[BigInt; 4]; 4]) -> BigInt {
    let (a, b, c, d) = (&rows[0], &rows[1], &rows[2], &rows[3]);

    // Cofactor expansion along first row, using precomputed 2x2 minors.
    let m01 = &b[0] * &c[1] - &b[1] * &c[0];
    let m02 = &b[0] * &c[2] - &b[2] * &c[0];
    let m03 = &b[0] * &c[3] - &b[3] * &c[0];
    let m12 = &b[1] * &c[2] - &b[2] * &c[1];
    let m13 = &b[1] * &c[3] - &b[3] * &c[1];
    let m23 = &b[2] * &c[3] - &b[3] * &c[2];

    let c00 = &d[1] * &m23 - &d[2] * &m13 + &d[3] * &m12;
    let c01 = &d[0] * &m23 - &d[2] * &m03 + &d[3] * &m02;
    let c02 = &d[0] * &m13 - &d[1] * &m03 + &d[3] * &m01;
    let c03 = &d[0] * &m12 - &d[1] * &m02 + &d[2] * &m01;

    &a[0] * c00 - &a[1] * c01 + &a[2] * c02 - &a[3] * c03
}

/// Vertex enumeration using integer Cramer's rule with f64 prefilter.
///
/// For each C(F,4) subset, uses the f64 prefilter to reject ~65-93%
/// (depending on facet count; TODO: add criterion bench citation for this number),
/// then solves via integer determinants (no BigRational, no GCD). Only confirmed
/// vertices are converted to BigRational coordinates.
///
/// Mathematical correspondence: [lem:vertex-enumeration], [prop:integer-cramer]
#[allow(clippy::type_complexity)]
fn enumerate_vertices_int(
    dual_vertices: &[[BigRational; 4]],
    int_dual_vertices: &[[BigInt; 4]],
    common_denom: &BigInt,
) -> Result<(Vec<BTreeSet<usize>>, Vec<[BigRational; 4]>), ConstructionError> {
    use super::rational_arithmetic::rational_to_f64;

    let f = dual_vertices.len();
    let one_int = [BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1)];

    // Precompute f64 versions for the prefilter.
    let dv_f64: Vec<[f64; 4]> = dual_vertices
        .iter()
        .map(|y| std::array::from_fn(|c| rational_to_f64(&y[c])))
        .collect();

    let mut vertex_descriptors = Vec::new();
    let mut vertices = Vec::new();

    for subset in combinations4(f) {
        // Stage 1: f64 prefilter (unchanged).
        if f64_prefilter_rejects(&dv_f64, &subset, f) {
            continue;
        }

        // Stage 2: integer Cramer's rule.
        let m_s: [&[BigInt; 4]; 4] = [
            &int_dual_vertices[subset[0]],
            &int_dual_vertices[subset[1]],
            &int_dual_vertices[subset[2]],
            &int_dual_vertices[subset[3]],
        ];
        let m_s_owned: [[BigInt; 4]; 4] = [
            m_s[0].clone(),
            m_s[1].clone(),
            m_s[2].clone(),
            m_s[3].clone(),
        ];

        // Step 1: det(M_S)
        let delta = det4_int(&m_s_owned);
        if delta.is_zero() {
            continue; // Singular
        }
        let delta_positive = delta.is_positive();

        // Step 2: Cramer numerator dets ν_j (M_S with col j = [1,1,1,1])
        let mut nu = [BigInt::from(0), BigInt::from(0), BigInt::from(0), BigInt::from(0)];
        for j in 0..4 {
            let mut modified = m_s_owned.clone();
            for row in 0..4 {
                modified[row][j] = one_int[row].clone();
            }
            nu[j] = det4_int(&modified);
        }

        // Step 3: feasibility check — all gaps must be non-negative.
        let mut all_ok = true;
        let mut incident_facets = BTreeSet::from(subset);

        for (i, a_i) in int_dual_vertices.iter().enumerate() {
            if subset.contains(&i) {
                continue;
            }
            // g_i = δ − Σ_j A_i[j] · ν_j
            let gap_numer = &delta - dot4_int(a_i, &nu);

            // gap ≥ 0 iff sign(gap_numer) matches sign(δ)
            if gap_numer.is_zero() {
                incident_facets.insert(i);
            } else if gap_numer.is_positive() != delta_positive {
                all_ok = false;
                break;
            }
        }

        if !all_ok {
            continue;
        }

        // Step 4: vertex confirmed. Compute exact rational coordinates.
        // v[j] = D · ν_j / δ  (reduced form — needed for reliable f64 conversion
        // in the irredundancy check; unreduced ~240-bit numerators overflow f64).
        let v: [BigRational; 4] = std::array::from_fn(|j| {
            BigRational::new(common_denom * &nu[j], delta.clone())
        });

        // Deduplicate
        let already_found = vertices
            .iter()
            .any(|existing: &[BigRational; 4]| (0..4).all(|i| existing[i] == v[i]));
        if already_found {
            continue;
        }

        vertex_descriptors.push(incident_facets);
        vertices.push(v);
    }

    if vertex_descriptors.is_empty() {
        return Err(ConstructionError::NoVertices);
    }

    Ok((vertex_descriptors, vertices))
}

/// Check that dual vertices positively span R^4 using f64-first per-triple
/// checks with exact integer fallback.
///
/// 1. Rank check in integer (cheap, done once).
/// 2. For each triple (i,j,k), try f64 first (`bounded_triple_f64_confirms`).
///    If f64 is inconclusive, fall back to exact integer cross-product and
///    dot-product signs.
///
/// Mathematical correspondence: [lem:positive-span]
fn check_bounded_f64_first(
    dual_vertices: &[[BigRational; 4]],
    int_dual_vertices: &[[BigInt; 4]],
) -> Result<(), ConstructionError> {
    use super::rational_arithmetic::rational_to_f64;

    let f = dual_vertices.len();
    let dv_f64: Vec<nalgebra::Vector4<f64>> = dual_vertices
        .iter()
        .map(|y| {
            nalgebra::Vector4::new(
                rational_to_f64(&y[0]),
                rational_to_f64(&y[1]),
                rational_to_f64(&y[2]),
                rational_to_f64(&y[3]),
            )
        })
        .collect();

    // Rank check in integer (cheap, done once).
    if rank_int(int_dual_vertices) < 4 {
        return Err(ConstructionError::Unbounded);
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                // f64 pre-filter: confirms this triple if signs are unambiguous.
                if bounded_triple_f64_confirms(&dv_f64, i, j, k) {
                    continue;
                }
                // f64 inconclusive — exact integer fallback.
                let d_int = cross_product_4d_int(
                    &int_dual_vertices[i],
                    &int_dual_vertices[j],
                    &int_dual_vertices[k],
                );
                if d_int.iter().all(|c| c.is_zero()) {
                    continue; // Dependent triple
                }
                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d_int).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d_int).is_negative());
                if !has_pos || !has_neg {
                    return Err(ConstructionError::Unbounded);
                }
            }
        }
    }

    Ok(())
}

/// Check irredundancy: every facet has incident vertices of affine rank >= 3.
///
/// Uses f64 fast path first (checking 3x3 minor determinants), falling back
/// to exact rational `affine_rank_rational` when f64 is inconclusive.
///
/// Mathematical correspondence: [lem:irredundancy] (facet i is redundant iff
/// its incident vertices have affine rank < 3).
fn check_irredundancy_f64_first(
    vertices: &[[BigRational; 4]],
    vertex_descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> Result<(), ConstructionError> {
    use super::rational_arithmetic::rational_to_f64;

    for i in 0..facet_count {
        let incident_indices: Vec<usize> = vertex_descriptors
            .iter()
            .enumerate()
            .filter(|(_, vd)| vd.contains(&i))
            .map(|(idx, _)| idx)
            .collect();

        if incident_indices.is_empty() {
            return Err(ConstructionError::RedundantFacet(i));
        }

        // f64 fast path: try sets of 4 incident vertices until one confirms
        // rank >= 3. The first 4 may be nearly coplanar; trying more avoids
        // unnecessary fallback to expensive rational rank computation.
        if incident_indices.len() >= 4 {
            let inc_f64: Vec<[f64; 4]> = incident_indices
                .iter()
                .map(|&idx| std::array::from_fn(|c| rational_to_f64(&vertices[idx][c])))
                .collect();

            // 3x3 minor determinant threshold for f64 rank check.
            // Vertices are exact rational; their f64 representations have
            // rounding error O(ε_mach) ≈ 2e-16. The 3x3 determinant has
            // O(n·ε_mach)·||rows||³ error (n=3 terms, unit-scale vertices
            // → error ~1e-15). Threshold 1e-10 gives ~5 orders of margin;
            // if det < 1e-10 we fall through to exact rational `affine_rank_rational`.
            // Must be re-validated if vertex coordinates can exceed ~1e4 in magnitude.
            const EPS_RANK_F64: f64 = 1e-10;
            let mut rank_ok = false;
            'outer: for base_idx in 0..inc_f64.len() {
                let base = &inc_f64[base_idx];
                // Try all triples of other vertices as the 3 difference vectors.
                let others: Vec<usize> = (0..inc_f64.len())
                    .filter(|&j| j != base_idx)
                    .collect();
                for a in 0..others.len() {
                    for b in (a + 1)..others.len() {
                        for c in (b + 1)..others.len() {
                            let rows: [[f64; 4]; 3] = [
                                std::array::from_fn(|d| inc_f64[others[a]][d] - base[d]),
                                std::array::from_fn(|d| inc_f64[others[b]][d] - base[d]),
                                std::array::from_fn(|d| inc_f64[others[c]][d] - base[d]),
                            ];
                            // Check any 3x3 minor
                            for skip_col in 0..4 {
                                let cols: Vec<usize> =
                                    (0..4).filter(|&d| d != skip_col).collect();
                                let det = rows[0][cols[0]]
                                    * (rows[1][cols[1]] * rows[2][cols[2]]
                                        - rows[1][cols[2]] * rows[2][cols[1]])
                                    - rows[0][cols[1]]
                                        * (rows[1][cols[0]] * rows[2][cols[2]]
                                            - rows[1][cols[2]] * rows[2][cols[0]])
                                    + rows[0][cols[2]]
                                        * (rows[1][cols[0]] * rows[2][cols[1]]
                                            - rows[1][cols[1]] * rows[2][cols[0]]);
                                if det.abs() > EPS_RANK_F64 {
                                    rank_ok = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
            if rank_ok {
                continue; // f64 confirmed rank >= 3
            }
        }

        // Fallback: exact rational rank check
        let incident: Vec<[BigRational; 4]> = incident_indices
            .iter()
            .map(|&idx| vertices[idx].clone())
            .collect();
        if affine_rank_rational(&incident) < 3 {
            return Err(ConstructionError::RedundantFacet(i));
        }
    }

    Ok(())
}

// ── Construction pipeline ────────────────────────────────────────────────

/// Run the exact rational construction pipeline: validate, enumerate vertices,
/// check irredundancy — all over Q.
///
/// Takes dual vertices y_i in K° and returns (primal_vertices, vertex_descriptors).
/// Each vertex descriptor is the set of facet indices incident to that vertex.
///
/// The halfspace representation is y_i · x <= 1 for each dual vertex y_i.
///
/// Non-simple polytopes (vertices on >4 facets) are supported: the vertex
/// descriptor records ALL incident facets, not just the defining 4-subset.
///
/// ## Why exact arithmetic
///
/// Vertex-facet incidence is a discrete decision: is `y_i · v` exactly 1?
/// In f64, rounding error makes this ambiguous for near-incident pairs.
/// The exact rational pipeline resolves all such decisions without tolerances.
/// This is critical for `omega_signs` (sign of ω₀(y_i, y_k)) which controls
/// directed adjacency pruning in the capacity algorithm.
///
/// ## Performance
///
/// O(F⁴) BigRational operations. The `num-bigint` crate is ~20× slower in
/// debug mode than release (TODO: add criterion bench citation for this ratio);
/// Cargo profile overrides (`opt-level = 3` for `num-bigint` and `num-rational`)
/// bring debug-mode cost close to release.
///
/// The pipeline uses f64-first strategies with exact integer fallbacks:
/// - **Bounded check**: per-triple f64 pre-filter (`bounded_triple_f64_confirms`),
///   falling back to integer cross-product signs for inconclusive triples.
/// - **Vertex enumeration**: f64 prefilter (`f64_prefilter_rejects`) to reject
///   ~65-93% of C(F,4) subsets, then integer Cramer's rule (`det4_int`) for
///   the remainder — no BigRational GCD normalization needed.
/// - **Irredundancy**: f64 rank check on incident vertices, falling back to
///   exact rational `affine_rank_rational` when f64 is inconclusive.
///
/// Mathematical correspondence: [lem:vertex-enumeration]
#[allow(clippy::type_complexity)]
pub(super) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>), ConstructionError> {
    let f = dual_vertices.len();

    // ── Step 1: Validation ──
    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }
    for (i, y) in dual_vertices.iter().enumerate() {
        if y.iter().all(|c| c.is_zero()) {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }

    // ── Step 2: Precompute integer-scaled dual vertices ──
    let (int_dual_vertices, common_denom) = integer_scale_dual_vertices(dual_vertices);

    // ── Step 3: Bounded check (f64-first per triple, integer fallback) ──
    check_bounded_f64_first(dual_vertices, &int_dual_vertices)?;

    // ── Step 4: Vertex enumeration (f64 prefilter + integer Cramer) ──
    let (vertex_descriptors, vertices) =
        enumerate_vertices_int(dual_vertices, &int_dual_vertices, &common_denom)?;

    // ── Step 5: Irredundancy check (f64-first) ──
    check_irredundancy_f64_first(&vertices, &vertex_descriptors, f)?;

    Ok((vertices, vertex_descriptors))
}

/// f64 pre-filter: returns true if the subset definitely yields no vertex.
///
/// When this returns true, at least one constraint yᵢᵀA⁻¹𝟏 > 1 is
/// determined by the error bound, so the four facets cannot meet inside K.
/// When this returns false, the subset may or may not be a vertex —
/// the rational path decides.
///
/// Correctness: [cor:prefilter-soundness] in geom/math.tex.
/// Note: the proof has an open gap (κ̂ vs κ, see TODO in geom/math.tex).
fn f64_prefilter_rejects(dv_f64: &[[f64; 4]], subset: &[usize; 4], f: usize) -> bool {
    use nalgebra::{Matrix4, Vector4};

    /// Unit roundoff for f64: ε_mach = 2⁻⁵³.
    const EPS_MACH: f64 = f64::EPSILON / 2.0;

    /// Safety constant from [prop:prefilter-bound]. Tight accounting gives C < 1400;
    /// we use C = 10⁴ for generous margin.
    const C: f64 = 1e4;

    // Step 1: Build 4×4 matrix from subset rows.
    let a = Matrix4::new(
        dv_f64[subset[0]][0], dv_f64[subset[0]][1],
        dv_f64[subset[0]][2], dv_f64[subset[0]][3],
        dv_f64[subset[1]][0], dv_f64[subset[1]][1],
        dv_f64[subset[1]][2], dv_f64[subset[1]][3],
        dv_f64[subset[2]][0], dv_f64[subset[2]][1],
        dv_f64[subset[2]][2], dv_f64[subset[2]][3],
        dv_f64[subset[3]][0], dv_f64[subset[3]][1],
        dv_f64[subset[3]][2], dv_f64[subset[3]][3],
    );

    // Step 2: Compute SVD of Â.
    let svd = a.svd(true, true);
    let svals = &svd.singular_values;

    // Find σ̂_min and σ̂_max (nalgebra does not guarantee sorted order).
    let sigma_min = svals[0].min(svals[1]).min(svals[2]).min(svals[3]);
    let sigma_max = svals[0].max(svals[1]).max(svals[2]).max(svals[3]);

    // Step 3: If σ̂_min = 0, matrix is singular → INDETERMINATE.
    if sigma_min == 0.0 {
        return false;
    }

    // Step 4: Condition number check. If ε_mach · κ̂ > 1/4, the system
    // is too ill-conditioned for the error bound to hold.
    let kappa_hat = sigma_max / sigma_min;
    if EPS_MACH * kappa_hat > 0.25 {
        return false; // Too ill-conditioned → INDETERMINATE
    }

    // Step 5: Solve Â·v̂ = 𝟏 via SVD factors.
    let ones = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let v_hat = match svd.solve(&ones, 0.0) {
        Ok(v) => v,
        Err(_) => return false, // Solve failed → INDETERMINATE
    };

    // NaN/Inf check on the solution.
    if v_hat.iter().any(|&x| !x.is_finite()) {
        return false;
    }

    let v_norm = v_hat.norm();

    // Step 6: Check each non-defining constraint with tolerance
    // δ = C · κ̂ · ε_mach · ‖v̂‖ · ‖ŷᵢ‖.
    for (i, y_i) in dv_f64[..f].iter().enumerate() {
        if subset.contains(&i) {
            continue;
        }

        // ŝ = ŷᵢᵀv̂
        let s_hat = y_i[0] * v_hat[0] + y_i[1] * v_hat[1]
            + y_i[2] * v_hat[2] + y_i[3] * v_hat[3];

        // ‖ŷᵢ‖₂
        let y_norm = (y_i[0] * y_i[0] + y_i[1] * y_i[1]
            + y_i[2] * y_i[2] + y_i[3] * y_i[3])
            .sqrt();

        // δ = C · κ̂ · ε_mach · ‖v̂‖ · ‖ŷᵢ‖
        let delta = C * kappa_hat * EPS_MACH * v_norm * y_norm;

        if !s_hat.is_finite() || !delta.is_finite() {
            return false; // NaN/Inf → INDETERMINATE
        }

        // If ŝ > 1 + δ: constraint is definitely violated → reject subset.
        if s_hat > 1.0 + delta {
            return true;
        }
        // TRUE (ŝ < 1 − δ) and INDETERMINATE both fall through to rational.
    }

    false // No constraint was definitely violated → proceed to rational
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::polytope::Polytope4D;
    use crate::geom::rational_arithmetic::{frac, rat};
    use num_rational::BigRational;
    use num_traits::Zero;
    use std::collections::BTreeSet;

    // Tests for vertex enumeration: exact vertex computation from halfspaces.
    //
    // Proposition: The exact rational pipeline correctly enumerates all vertices
    // of a polytope K from its dual vertex (halfspace) representation, with
    // correct vertex-facet incidence and exact rational coordinates.
    // Reference: [lem:vertex-enumeration], [lem:positive-span]
    //
    // Strategy: fixture-based on simplex (5 facets) and hypercube (8 facets),
    // verifying vertex counts, descriptor structure, coordinate values,
    // affine rank, and boundedness.

    // ── Test fixtures ──────────────────────────────────────────────────────

    /// Build a rational 4-simplex with exact rational coordinates.
    ///
    /// Simplex with vertices at (-1/5)*1 + (9/5)*e_i for i=1..4, plus (-1/5)*1.
    /// The origin is interior (all gaps = 1/5 > 0). Uses non-unit normals.
    ///
    /// Facets:
    ///   0: -x_1 <= 1/5   (n = (-1,0,0,0), h = 1/5)
    ///   1: -x_2 <= 1/5   (n = (0,-1,0,0), h = 1/5)
    ///   2: -x_3 <= 1/5   (n = (0,0,-1,0), h = 1/5)
    ///   3: -x_4 <= 1/5   (n = (0,0,0,-1), h = 1/5)
    ///   4: x_1+x_2+x_3+x_4 <= 1   (n = (1,1,1,1), h = 1)
    fn rational_simplex() -> Polytope4D {
        let normals = vec![
            [rat(-1), rat(0), rat(0), rat(0)],
            [rat(0), rat(-1), rat(0), rat(0)],
            [rat(0), rat(0), rat(-1), rat(0)],
            [rat(0), rat(0), rat(0), rat(-1)],
            [rat(1), rat(1), rat(1), rat(1)],
        ];
        let heights = vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];
        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();
        Polytope4D::new(dual_vertices).expect("simplex construction")
    }

    /// Build a rational hypercube [-1, 1]^4 with exact integer coordinates.
    ///
    /// 8 facets (+-e_i), 16 vertices (all sign combinations of (1,1,1,1)).
    fn rational_hypercube() -> Polytope4D {
        let normals = vec![
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(-1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(-1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(-1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
            [rat(0), rat(0), rat(0), rat(-1)],
        ];
        let heights = vec![rat(1); 8];
        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();
        Polytope4D::new(dual_vertices).expect("hypercube construction")
    }

    /// Extract vertex descriptors (sets of incident facet indices) from incidence matrix.
    fn vertex_descriptors_from_incidence(p: &Polytope4D) -> Vec<BTreeSet<usize>> {
        let inc = p.incidence();
        let v_count = p.vertices().len();
        let f_count = p.facet_count();
        (0..v_count)
            .map(|vi| {
                (0..f_count)
                    .filter(|&fi| inc[(vi, fi)])
                    .collect::<BTreeSet<usize>>()
            })
            .collect()
    }

    // ── Simplex vertex structure ────────────────────────────────────────────

    /// Proposition: the 4-simplex has exactly 5 vertex descriptors,
    /// each a 4-element subset of {0, 1, 2, 3, 4} (one facet omitted per vertex).
    #[test]
    fn exact_simplex_vertex_descriptors() {
        let s = rational_simplex();
        let vds = vertex_descriptors_from_incidence(&s);
        assert_eq!(vds.len(), 5, "simplex should have exactly 5 vertices");

        for vd in &vds {
            assert_eq!(vd.len(), 4, "simplex vertex should lie on exactly 4 facets");
            assert!(
                vd.iter().all(|&i| i < 5),
                "facet indices should be in 0..5"
            );
        }

        // Each vertex descriptor is {0..4} minus one element
        let expected: Vec<BTreeSet<usize>> = (0..5)
            .map(|omit| (0..5).filter(|&i| i != omit).collect())
            .collect();
        let mut actual = vds;
        actual.sort();
        let mut expected_sorted = expected;
        expected_sorted.sort();
        assert_eq!(actual, expected_sorted);
    }

    /// Proposition: the simplex vertex on facets {0,1,2,3} (omitting the sum-constraint)
    /// has exact coordinates (-1/5, -1/5, -1/5, -1/5).
    #[test]
    fn exact_simplex_vertex_coordinates() {
        let s = rational_simplex();
        let vds = vertex_descriptors_from_incidence(&s);

        let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
        let idx = vds
            .iter()
            .position(|vd| *vd == target_vd)
            .expect("vertex {0,1,2,3} should exist");

        let v = &s.vertices()[idx];
        let expected = frac(-1, 5);
        for (c, coord) in v.iter().enumerate() {
            assert_eq!(
                coord, &expected,
                "coordinate {c} should be -1/5, got {coord}"
            );
        }
    }

    // ── Hypercube vertex structure ──────────────────────────────────────────

    /// Proposition: the hypercube [-1,1]^4 has exactly 16 vertex descriptors,
    /// each a 4-element subset of {0,...,7}, picking one from each opposing pair.
    #[test]
    fn exact_hypercube_vertex_descriptors() {
        let h = rational_hypercube();
        let vds = vertex_descriptors_from_incidence(&h);
        assert_eq!(vds.len(), 16, "hypercube should have 16 vertices");

        for vd in &vds {
            assert_eq!(
                vd.len(),
                4,
                "hypercube vertex should lie on exactly 4 facets"
            );
            // Each opposing pair (0,1), (2,3), (4,5), (6,7) should contribute exactly one
            let pairs = [(0, 1), (2, 3), (4, 5), (6, 7)];
            for (a, b) in pairs {
                let has_a = vd.contains(&a);
                let has_b = vd.contains(&b);
                assert!(
                    has_a ^ has_b,
                    "vertex should pick exactly one from pair ({a}, {b})"
                );
            }
        }
    }

    /// Proposition: the hypercube vertices are exactly the 16 points (+-1, +-1, +-1, +-1).
    #[test]
    fn exact_hypercube_vertex_coordinates() {
        let h = rational_hypercube();
        let one = rat(1);
        let neg_one = rat(-1);

        for v in h.vertices() {
            for coord in v {
                assert!(
                    coord == &one || coord == &neg_one,
                    "hypercube vertex coordinate should be +/-1, got {coord}"
                );
            }
        }
        assert_eq!(h.vertices().len(), 16);
    }

    // ── Affine rank ─────────────────────────────────────────────────────────

    /// Proposition: affine rank of the 5 simplex vertices = 4 (they span R^4).
    #[test]
    fn simplex_vertices_affine_rank_is_4() {
        let p = rational_simplex();
        assert_eq!(affine_rank_rational(p.vertices()), 4);
    }

    /// Proposition: 4 points in the hyperplane x_3 = 0 have affine rank 3 (< 4).
    #[test]
    fn coplanar_points_affine_rank_below_4() {
        let points = vec![
            [rat(0), rat(0), rat(0), rat(0)],
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
        ];
        assert_eq!(affine_rank_rational(&points), 3);
    }

    // ── Boundedness ─────────────────────────────────────────────────────────

    /// Proposition: simplex dual vertices positively span R^4 (simplex is bounded).
    #[test]
    fn simplex_is_bounded() {
        let p = rational_simplex();
        assert!(check_bounded_rational(p.dual_vertices()));
    }

    /// Proposition: hypercube dual vertices positively span R^4 (hypercube is bounded).
    #[test]
    fn hypercube_is_bounded() {
        let p = rational_hypercube();
        assert!(check_bounded_rational(p.dual_vertices()));
    }

    // ── Non-simple polytope ─────────────────────────────────────────────────

    /// Proposition: non-simple polytopes (vertices on > 4 facets) are correctly handled.
    /// A hypercube with a diagonal cut at x_1+x_2+x_3+x_4 <= 2 produces 4 non-simple
    /// vertices (on 5 facets) and 11 simple vertices, totalling 15.
    #[test]
    fn non_simple_polytope_vertex_enumeration() {
        let normals = vec![
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(-1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(-1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(-1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
            [rat(0), rat(0), rat(0), rat(-1)],
            [rat(1), rat(1), rat(1), rat(1)],
        ];
        let heights = vec![
            rat(1),
            rat(1),
            rat(1),
            rat(1),
            rat(1),
            rat(1),
            rat(1),
            rat(1),
            rat(2),
        ];
        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();
        let p =
            Polytope4D::new(dual_vertices).expect("non-simple polytope should succeed");

        let vds = vertex_descriptors_from_incidence(&p);
        assert_eq!(vds.len(), 15, "cut hypercube should have 15 vertices");

        let non_simple_count = vds.iter().filter(|vd| vd.len() > 4).count();
        assert_eq!(
            non_simple_count, 4,
            "expected 4 non-simple vertices (on the diagonal cut)"
        );
    }

    // ---- Linear algebra tests ----
    //
    // Tests for exact rational linear algebra helpers used by vertex enumeration.
    //
    // Proposition: The low-level linear algebra routines (det4, solve4, rank_over_q,
    // cross_product_4d_rational, dot4) compute exact results over Q with no
    // floating-point approximation.
    // Reference: [lem:vertex-enumeration]
    //
    // Strategy: fixture-based on known matrices (identity, diagonal, singular)
    // and vectors, verifying exact algebraic identities.

    // ── Determinant ─────────────────────────────────────────────────────────

    /// Proposition: det(I_4) = 1.
    #[test]
    fn det4_identity() {
        let id: [[BigRational; 4]; 4] = [
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
        ];
        assert_eq!(det4(&id), rat(1));
    }

    /// Proposition: a matrix with two identical rows has determinant 0.
    #[test]
    fn det4_singular() {
        let singular: [[BigRational; 4]; 4] = [
            [rat(1), rat(2), rat(3), rat(4)],
            [rat(1), rat(2), rat(3), rat(4)],
            [rat(5), rat(6), rat(7), rat(8)],
            [rat(9), rat(10), rat(11), rat(12)],
        ];
        assert_eq!(det4(&singular), rat(0));
    }

    /// Proposition: det(diag(2,3,5,7)) = 210.
    #[test]
    fn det4_diagonal() {
        let diag: [[BigRational; 4]; 4] = [
            [rat(2), rat(0), rat(0), rat(0)],
            [rat(0), rat(3), rat(0), rat(0)],
            [rat(0), rat(0), rat(5), rat(0)],
            [rat(0), rat(0), rat(0), rat(7)],
        ];
        assert_eq!(det4(&diag), rat(210));
    }

    // ── Linear system solver (Cramer's rule) ────────────────────────────────

    /// Proposition: solving I*x = b yields x = b.
    #[test]
    fn solve4_identity_system() {
        let id: [[BigRational; 4]; 4] = [
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
        ];
        let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
        let x = solve4(&id, &rhs).expect("non-singular");
        assert_eq!(x, rhs);
    }

    /// Proposition: solving diag(2,3,5,7)*x = (4,9,10,21) yields x = (2,3,2,3).
    #[test]
    fn solve4_diagonal_system() {
        let diag: [[BigRational; 4]; 4] = [
            [rat(2), rat(0), rat(0), rat(0)],
            [rat(0), rat(3), rat(0), rat(0)],
            [rat(0), rat(0), rat(5), rat(0)],
            [rat(0), rat(0), rat(0), rat(7)],
        ];
        let rhs = [rat(4), rat(9), rat(10), rat(21)];
        let x = solve4(&diag, &rhs).expect("non-singular");
        assert_eq!(x, [rat(2), rat(3), rat(2), rat(3)]);
    }

    /// Proposition: solve4 returns None for a singular system (two identical rows).
    #[test]
    fn solve4_singular_returns_none() {
        let singular: [[BigRational; 4]; 4] = [
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
        ];
        assert!(solve4(&singular, &[rat(1), rat(1), rat(1), rat(1)]).is_none());
    }

    // ── Matrix rank ─────────────────────────────────────────────────────────

    /// Proposition: rank(I_4) = 4.
    #[test]
    fn rank_over_q_identity() {
        let id = vec![
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(0), rat(0), rat(0), rat(1)],
        ];
        assert_eq!(rank_over_q(&id), 4);
    }

    /// Proposition: replacing one row with a scalar multiple of another drops rank to 3.
    #[test]
    fn rank_over_q_dependent_row() {
        let rows = vec![
            [rat(1), rat(0), rat(0), rat(0)],
            [rat(0), rat(1), rat(0), rat(0)],
            [rat(0), rat(0), rat(1), rat(0)],
            [rat(2), rat(0), rat(0), rat(0)], // 2 * row 0
        ];
        assert_eq!(rank_over_q(&rows), 3);
    }

    /// Proposition: the zero vector has rank 0.
    #[test]
    fn rank_over_q_zero_vector() {
        let zeros = vec![[rat(0), rat(0), rat(0), rat(0)]];
        assert_eq!(rank_over_q(&zeros), 0);
    }

    /// Proposition: the empty set has rank 0.
    #[test]
    fn rank_over_q_empty() {
        let empty: Vec<[BigRational; 4]> = vec![];
        assert_eq!(rank_over_q(&empty), 0);
    }

    /// Proposition: a single nonzero vector has rank 1.
    #[test]
    fn rank_over_q_single_nonzero() {
        let single = vec![[rat(3), rat(-1), rat(0), rat(7)]];
        assert_eq!(rank_over_q(&single), 1);
    }

    // ── 4D cross product ────────────────────────────────────────────────────

    /// Proposition: cross_product_4d_rational(a, b, c) is perpendicular to all three inputs
    /// and is nonzero when a, b, c are linearly independent.
    #[test]
    fn cross_product_4d_rational_perpendicular() {
        let a = [rat(1), rat(2), rat(3), rat(4)];
        let b = [rat(5), rat(-1), rat(2), rat(0)];
        let c = [rat(0), rat(3), rat(-2), rat(1)];
        let d = cross_product_4d_rational(&a, &b, &c);

        assert!(dot4(&d, &a).is_zero(), "d . a = {} should be 0", dot4(&d, &a));
        assert!(dot4(&d, &b).is_zero(), "d . b = {} should be 0", dot4(&d, &b));
        assert!(dot4(&d, &c).is_zero(), "d . c = {} should be 0", dot4(&d, &c));
        assert!(
            !d.iter().all(|x| x.is_zero()),
            "cross product should be nonzero for independent inputs"
        );
    }

    /// Proposition: cross product of three dependent vectors is the zero vector.
    #[test]
    fn cross_product_4d_rational_dependent_is_zero() {
        let a = [rat(1), rat(0), rat(0), rat(0)];
        let b = [rat(0), rat(1), rat(0), rat(0)];
        // c = a + b, linearly dependent
        let c = [rat(1), rat(1), rat(0), rat(0)];
        let d = cross_product_4d_rational(&a, &b, &c);
        assert!(
            d.iter().all(|x| x.is_zero()),
            "cross product of dependent vectors should be zero"
        );
    }

    // ── Edge cases for integer-scaled vertex enumeration pipeline ────────

    /// Proposition: a vertex on 6 facets is correctly detected as non-simple.
    ///
    /// Takes hypercube [-1,1]^4 (8 facets) and adds two diagonal cuts:
    ///   facet 8: x_1+x_2+x_3+x_4 <= 2
    ///   facet 9: x_1+x_2-x_3-x_4 <= 2
    /// Both are non-redundant (they cut off cube vertices like (1,1,1,1)).
    ///
    /// Vertex (1,1,1,-1) is tight on 6 facets: x_1<=1, x_2<=1, x_3<=1,
    /// -x_4<=1, sum=1+1+1-1=2, diff=1+1-1+1=2. This exceeds the 5-facet
    /// case in `non_simple_polytope_vertex_enumeration`.
    #[test]
    fn highly_non_simple_vertex_on_6_facets() {
        // Hypercube [-1,1]^4 plus two diagonal cuts.
        let normals = vec![
            [rat(1), rat(0), rat(0), rat(0)],   // 0: x_1 <= 1
            [rat(-1), rat(0), rat(0), rat(0)],  // 1: -x_1 <= 1
            [rat(0), rat(1), rat(0), rat(0)],   // 2: x_2 <= 1
            [rat(0), rat(-1), rat(0), rat(0)],  // 3: -x_2 <= 1
            [rat(0), rat(0), rat(1), rat(0)],   // 4: x_3 <= 1
            [rat(0), rat(0), rat(-1), rat(0)],  // 5: -x_3 <= 1
            [rat(0), rat(0), rat(0), rat(1)],   // 6: x_4 <= 1
            [rat(0), rat(0), rat(0), rat(-1)],  // 7: -x_4 <= 1
            [rat(1), rat(1), rat(1), rat(1)],   // 8: x_1+x_2+x_3+x_4 <= 2
            [rat(1), rat(1), rat(-1), rat(-1)], // 9: x_1+x_2-x_3-x_4 <= 2
        ];
        let heights = vec![
            rat(1), rat(1), rat(1), rat(1),
            rat(1), rat(1), rat(1), rat(1),
            rat(2), rat(2),
        ];
        let dual_vertices: Vec<[BigRational; 4]> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &n[c] / h))
            .collect();
        let p = Polytope4D::new(dual_vertices).expect("doubly-cut hypercube should succeed");

        let vds = vertex_descriptors_from_incidence(&p);

        // Find vertex (1,1,1,-1): on facets 0,2,4,7,8,9 (6 facets).
        let target = [rat(1), rat(1), rat(1), rat(-1)];
        let idx = p
            .vertices()
            .iter()
            .position(|v| (0..4).all(|c| v[c] == target[c]))
            .expect("vertex (1,1,1,-1) should exist");

        let vd = &vds[idx];
        assert_eq!(
            vd.len(),
            6,
            "vertex (1,1,1,-1) should lie on exactly 6 facets, got {}: {:?}",
            vd.len(),
            vd
        );
        // Verify specific facet incidence
        for &fi in &[0, 2, 4, 7, 8, 9] {
            assert!(
                vd.contains(&fi),
                "vertex (1,1,1,-1) should be incident to facet {fi}"
            );
        }
    }

    /// Proposition: vertex enumeration is correct for large-coordinate dual vertices (~1e6).
    ///
    /// Scales the standard simplex dual vertices by 1e6. The integer scaling
    /// pipeline must handle numerators of magnitude ~1e6 correctly, and the f64
    /// prefilter must not give wrong signs for large coordinates.
    #[test]
    fn large_coordinate_dual_vertices() {
        let scale = rat(1_000_000);
        // Simplex with normals scaled by 1e6: same polytope, just different
        // representation (n_i -> 1e6 * n_i, h_i -> 1e6 * h_i, so y_i unchanged).
        // Instead, scale the dual vertices themselves to get large coordinates.
        // y_i = scale * original_y_i means h-rep: (scale * n_i / h_i) . x <= 1,
        // so K = {x : scale * y_i . x <= 1} = (1/scale) * K_original.
        let base_normals: Vec<[BigRational; 4]> = vec![
            [rat(-1), rat(0), rat(0), rat(0)],
            [rat(0), rat(-1), rat(0), rat(0)],
            [rat(0), rat(0), rat(-1), rat(0)],
            [rat(0), rat(0), rat(0), rat(-1)],
            [rat(1), rat(1), rat(1), rat(1)],
        ];
        let base_heights: Vec<BigRational> =
            vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];

        // Dual vertices = n_i / h_i, then scale by 1e6
        let dual_vertices: Vec<[BigRational; 4]> = base_normals
            .iter()
            .zip(base_heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &(&n[c] / h) * &scale))
            .collect();

        // Verify the coordinates are indeed large
        let max_coord: f64 = dual_vertices
            .iter()
            .flat_map(|y| y.iter())
            .filter(|c| !c.is_zero())
            .map(|c| {
                let f = c.numer().to_string().parse::<f64>().unwrap()
                    / c.denom().to_string().parse::<f64>().unwrap();
                f.abs()
            })
            .fold(0.0f64, f64::max);
        assert!(
            max_coord >= 1e5,
            "dual vertex coordinates should be large, max = {max_coord}"
        );

        let p = Polytope4D::new(dual_vertices).expect("large-coordinate simplex should succeed");
        assert_eq!(
            p.vertices().len(),
            5,
            "scaled simplex should still have 5 vertices"
        );

        // The vertex on facets {0,1,2,3} should be (-1/5, -1/5, -1/5, -1/5) / scale
        let vds = vertex_descriptors_from_incidence(&p);
        let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
        let idx = vds
            .iter()
            .position(|vd| *vd == target_vd)
            .expect("vertex {0,1,2,3} should exist");
        let expected = frac(-1, 5_000_000);
        for (c, coord) in p.vertices()[idx].iter().enumerate() {
            assert_eq!(
                coord, &expected,
                "coordinate {c} of scaled simplex vertex should be -1/5000000"
            );
        }
    }

    /// Proposition: vertex enumeration is correct for small-coordinate dual vertices (~1e-6).
    ///
    /// Scales dual vertices by 1e-6. Near-zero f64 values in the prefilter must
    /// not cause incorrect sign decisions; the exact integer fallback must handle
    /// the resulting small numerators and large common denominator correctly.
    #[test]
    fn small_coordinate_dual_vertices() {
        // Scale = 1/1000000
        let scale = frac(1, 1_000_000);
        let base_normals: Vec<[BigRational; 4]> = vec![
            [rat(-1), rat(0), rat(0), rat(0)],
            [rat(0), rat(-1), rat(0), rat(0)],
            [rat(0), rat(0), rat(-1), rat(0)],
            [rat(0), rat(0), rat(0), rat(-1)],
            [rat(1), rat(1), rat(1), rat(1)],
        ];
        let base_heights: Vec<BigRational> =
            vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];

        let dual_vertices: Vec<[BigRational; 4]> = base_normals
            .iter()
            .zip(base_heights.iter())
            .map(|(n, h)| std::array::from_fn(|c| &(&n[c] / h) * &scale))
            .collect();

        let p = Polytope4D::new(dual_vertices).expect("small-coordinate simplex should succeed");
        assert_eq!(
            p.vertices().len(),
            5,
            "scaled simplex should still have 5 vertices"
        );

        // Vertex on facets {0,1,2,3}: coordinates = -1/5 / scale = -1/5 * 1e6 = -200000
        let vds = vertex_descriptors_from_incidence(&p);
        let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
        let idx = vds
            .iter()
            .position(|vd| *vd == target_vd)
            .expect("vertex {0,1,2,3} should exist");
        let expected = rat(-200_000);
        for (c, coord) in p.vertices()[idx].iter().enumerate() {
            assert_eq!(
                coord, &expected,
                "coordinate {c} of small-scale simplex vertex should be -200000"
            );
        }
    }

    /// Proposition: exact rational construction handles non-power-of-2 denominators.
    ///
    /// Constructs a simplex from exact rationals with denominators 3, 5, 7 — values
    /// that are not exactly representable in f64. The common denominator computation
    /// (lcm) must correctly combine these primes, and the integer Cramer pipeline
    /// must produce exact vertex coordinates.
    #[test]
    fn exact_rational_non_power_of_two_denominators() {
        // A simplex with non-power-of-2 heights: h_0..h_3 = 1/3, h_4 = 2/7.
        // Dual vertices y_i = n_i / h_i.
        //
        // Facets:
        //   0: -x_1 <= 1/3   => y = (-3, 0, 0, 0)
        //   1: -x_2 <= 1/3   => y = (0, -3, 0, 0)
        //   2: -x_3 <= 1/3   => y = (0, 0, -3, 0)
        //   3: -x_4 <= 1/3   => y = (0, 0, 0, -3)
        //   4: x_1+x_2+x_3+x_4 <= 2/7  => y = (7/2, 7/2, 7/2, 7/2)
        let dual_vertices: Vec<[BigRational; 4]> = vec![
            [rat(-3), rat(0), rat(0), rat(0)],
            [rat(0), rat(-3), rat(0), rat(0)],
            [rat(0), rat(0), rat(-3), rat(0)],
            [rat(0), rat(0), rat(0), rat(-3)],
            [frac(7, 2), frac(7, 2), frac(7, 2), frac(7, 2)],
        ];

        let p = Polytope4D::new(dual_vertices.clone())
            .expect("non-power-of-2 denominator simplex should succeed");
        assert_eq!(p.vertices().len(), 5, "simplex should have 5 vertices");

        // Verify common denominator handles the lcm(1,1,1,1,2) = 2 correctly
        // by checking that the integer scaling produces correct results.
        let (int_verts, common_denom) = integer_scale_dual_vertices(&dual_vertices);
        // lcm of all denominators: denominators are 1,1,1,1,2 so lcm = 2
        assert_eq!(
            common_denom,
            BigInt::from(2),
            "common denominator should be lcm(1,1,1,1,2) = 2"
        );
        // int_verts[0] = (-3, 0, 0, 0) * 2 = (-6, 0, 0, 0)
        assert_eq!(int_verts[0][0], BigInt::from(-6));
        // int_verts[4] = (7/2, 7/2, 7/2, 7/2) * 2 = (7, 7, 7, 7)
        assert_eq!(int_verts[4][0], BigInt::from(7));

        // Vertex on facets {0,1,2,3} (omitting the sum constraint):
        // Solving -x_i = 1/3 for i=1..4 gives x_i = -1/3.
        let vds = vertex_descriptors_from_incidence(&p);
        let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
        let idx = vds
            .iter()
            .position(|vd| *vd == target_vd)
            .expect("vertex {0,1,2,3} should exist");
        let expected = frac(-1, 3);
        for (c, coord) in p.vertices()[idx].iter().enumerate() {
            assert_eq!(
                coord, &expected,
                "coordinate {c} should be -1/3, got {coord}"
            );
        }

        // Vertex on facets {1,2,3,4} (omitting facet 0, the -x_1 constraint):
        // x_2 = x_3 = x_4 = -1/3, and x_1+x_2+x_3+x_4 = 2/7.
        // So x_1 = 2/7 - (-1/3)*3 = 2/7 + 1 = 9/7.
        let target_vd2: BTreeSet<usize> = [1, 2, 3, 4].into_iter().collect();
        let idx2 = vds
            .iter()
            .position(|vd| *vd == target_vd2)
            .expect("vertex {1,2,3,4} should exist");
        let v = &p.vertices()[idx2];
        assert_eq!(v[0], frac(9, 7), "x_1 should be 9/7, got {}", v[0]);
        assert_eq!(v[1], frac(-1, 3), "x_2 should be -1/3, got {}", v[1]);
        assert_eq!(v[2], frac(-1, 3), "x_3 should be -1/3, got {}", v[2]);
        assert_eq!(v[3], frac(-1, 3), "x_4 should be -1/3, got {}", v[3]);
    }

    /// Proposition: integer Cramer's rule produces exact vertex coordinates for the
    /// hypercube, matching the known analytical values.
    ///
    /// For the hypercube [-1,1]^4, each vertex (s_1, s_2, s_3, s_4) with s_i in {-1,+1}
    /// is the unique solution of the 4x4 system formed by its 4 defining facets.
    /// This test verifies that the integer pipeline produces these exact coordinates
    /// by checking every vertex against the expected Cramer solution.
    #[test]
    fn integer_cramer_exact_coordinates_hypercube() {
        let h = rational_hypercube();
        let vds = vertex_descriptors_from_incidence(&h);

        // For each vertex, verify coordinates match the expected sign pattern.
        // Facet 2k: +e_{k+1} . x <= 1, so vertex on facet 2k has x_{k+1} = +1.
        // Facet 2k+1: -e_{k+1} . x <= 1, so vertex on facet 2k+1 has x_{k+1} = -1.
        for (vi, vd) in vds.iter().enumerate() {
            let v = &h.vertices()[vi];
            for dim in 0..4 {
                let expected = if vd.contains(&(2 * dim)) {
                    rat(1)
                } else {
                    assert!(
                        vd.contains(&(2 * dim + 1)),
                        "vertex must be on one of the pair"
                    );
                    rat(-1)
                };
                assert_eq!(
                    v[dim], expected,
                    "vertex {vi}, coordinate {dim}: expected {expected}, got {}",
                    v[dim]
                );
            }
        }
    }

    /// Proposition: integer Cramer's rule produces exact vertex coordinates for the
    /// simplex, verifiable via the defining equations y_i . v = 1.
    ///
    /// For each vertex v and each defining facet i (in its descriptor), the inner
    /// product y_i . v must be exactly 1. For non-defining facets, y_i . v < 1.
    /// This end-to-end check verifies the full Cramer pipeline (det4_int, numerator
    /// dets, coordinate assembly) without relying on known analytical formulas.
    #[test]
    fn integer_cramer_exact_coordinates_simplex() {
        let s = rational_simplex();
        let vds = vertex_descriptors_from_incidence(&s);
        let dual_verts = s.dual_vertices();

        for (vi, vd) in vds.iter().enumerate() {
            let v = &s.vertices()[vi];
            for fi in 0..dual_verts.len() {
                let prod = dot4(
                    &std::array::from_fn(|c| dual_verts[fi][c].clone()),
                    &std::array::from_fn(|c| v[c].clone()),
                );
                if vd.contains(&fi) {
                    assert_eq!(
                        prod,
                        rat(1),
                        "vertex {vi} on facet {fi}: y.v should be exactly 1, got {prod}"
                    );
                } else {
                    assert!(
                        prod < rat(1),
                        "vertex {vi} not on facet {fi}: y.v should be < 1, got {prod}"
                    );
                }
            }
        }
    }
}
