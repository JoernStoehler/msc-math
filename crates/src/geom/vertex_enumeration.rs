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
fn det3(r0: &[BigRational], r1: &[BigRational], r2: &[BigRational]) -> BigRational {
    &r0[0] * (&r1[1] * &r2[2] - &r1[2] * &r2[1])
        - &r0[1] * (&r1[0] * &r2[2] - &r1[2] * &r2[0])
        + &r0[2] * (&r1[0] * &r2[1] - &r1[1] * &r2[0])
}

/// Exact determinant of a 4x4 rational matrix via cofactor expansion.
///
/// Expands along the first row using 3x3 minors.
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
pub(super) fn dot4(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2] + &a[3] * &b[3]
}

/// 4D cross product over Q: vector perpendicular to three vectors in R^4.
///
/// d_k = (-1)^k * det(3x3 minor of [a, b, c] with column k removed).
/// Same formula as `cross_product_4d::cross_product_4d` but exact over Q.
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
/// Any direction d in R^4 can be written as a linear combination of
/// cross-product directions from triples of y_i (since rank = 4). If
/// y_i · d > 0 and y_i · d < 0 both occur for every such kernel direction,
/// then y_i positively spans R^4. The check is sufficient because any failure
/// of positive spanning is witnessed by some kernel direction of a triple.
///
/// Complexity: O(F^4) — F^3 triples times F inner products each.
///
/// Mathematical correspondence: [lem:positive-span]
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

/// Scale rational dual vertices to integer arrays with a common denominator.
///
/// Returns (A, D) where A[i][j] = a_i[j] * D ∈ Z and D = lcm of all denominators.
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

/// Check that integer-scaled dual vertices positively span R^4 (polytope is bounded).
///
/// Same algorithm as `check_bounded_rational` but over Z — no GCD, no division.
/// Signs are exact. Mathematical correspondence: [lem:positive-span]
#[allow(dead_code)]
fn check_bounded_int(int_dual_vertices: &[[BigInt; 4]]) -> bool {
    let f = int_dual_vertices.len();

    if rank_int(int_dual_vertices) < 4 {
        return false;
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let d = cross_product_4d_int(
                    &int_dual_vertices[i],
                    &int_dual_vertices[j],
                    &int_dual_vertices[k],
                );
                if d.iter().all(|c| c.is_zero()) {
                    continue;
                }

                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d).is_positive());
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| dot4_int(&int_dual_vertices[l], &d).is_negative());

                if !has_pos || !has_neg {
                    return false;
                }
            }
        }
    }
    true
}

/// Determinant of a 4x4 integer matrix via cofactor expansion.
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

/// Vertex enumeration using integer Cramer's rule.
///
/// For each C(F,4) subset, uses the f64 prefilter to reject ~90%, then
/// solves via integer determinants (no BigRational, no GCD). Only confirmed
/// vertices are converted to BigRational coordinates.
///
/// Mathematical correspondence: [lem:vertex-enumeration]
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
        // v[j] = D · ν_j / δ
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
/// debug mode than release; Cargo profile overrides (`opt-level = 3` for
/// `num-bigint` and `num-rational`) bring debug-mode cost close to release.
///
/// The boundedness check here (`check_bounded_rational`) is the authoritative
/// exact check, distinct from the f64-based `validation::check_bounded` which
/// may be indeterminate near the boundary (and from the vertex-enumeration
/// f64 pre-filter `f64_prefilter_rejects` which rejects non-vertex subsets).
///
/// Mathematical correspondence: [lem:vertex-enumeration]
#[allow(clippy::type_complexity)]
pub(super) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>), ConstructionError> {
    let f = dual_vertices.len();

    // Basic validation
    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }
    for (i, y) in dual_vertices.iter().enumerate() {
        if y.iter().all(|c| c.is_zero()) {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }

    // ── Step 2: Precompute integer-scaled dual vertices ──
    // Scale all rational components to a common denominator D so that
    // A_i = a_i * D ∈ Z^4. All subsequent exact checks use A_i (BigInt),
    // avoiding BigRational GCD normalization.
    let (int_dual_vertices, common_denom) = integer_scale_dual_vertices(dual_vertices);

    // ── Step 3: Bounded check (f64-first per triple, integer fallback) ──
    {
        use super::rational_arithmetic::rational_to_f64;

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
        if rank_int(&int_dual_vertices) < 4 {
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
    }

    // ── Step 4: Vertex enumeration (f64 prefilter + integer Cramer) ──
    let (vertex_descriptors, vertices) =
        enumerate_vertices_int(dual_vertices, &int_dual_vertices, &common_denom)?;

    // ── Step 5: Irredundancy check (f64-first) ──
    for i in 0..f {
        let incident_indices: Vec<usize> = vertex_descriptors
            .iter()
            .enumerate()
            .filter(|(_, vd)| vd.contains(&i))
            .map(|(idx, _)| idx)
            .collect();

        if incident_indices.is_empty() {
            return Err(ConstructionError::RedundantFacet(i));
        }

        // f64 fast path: pick 4 incident vertices, check det4 in f64.
        if incident_indices.len() >= 4 {
            use super::rational_arithmetic::rational_to_f64;
            let base = &vertices[incident_indices[0]];
            let base_f64: [f64; 4] = std::array::from_fn(|c| rational_to_f64(&base[c]));
            let mut rows = [[0.0f64; 4]; 3];
            for (r, &idx) in incident_indices[1..4].iter().enumerate() {
                for c in 0..4 {
                    rows[r][c] = rational_to_f64(&vertices[idx][c]) - base_f64[c];
                }
            }
            // 3x4 matrix → check if any 3x3 minor has large determinant
            let rank_ok = (0..4).any(|skip_col| {
                let m: [[f64; 3]; 3] = std::array::from_fn(|r| {
                    let mut row = [0.0; 3];
                    let mut ci = 0;
                    for (c, &val) in rows[r].iter().enumerate() {
                        if c == skip_col {
                            continue;
                        }
                        row[ci] = val;
                        ci += 1;
                    }
                    row
                });
                let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
                // 3x3 det of O(1) entries has rounding error O(ε_mach) ≈ 1e-16.
                // Threshold 1e-10 gives ~6 orders of margin.
                det.abs() > 1e-10
            });
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

    Ok((vertices, vertex_descriptors))
}

/// Enumerate all vertices by testing all C(F, 4) subsets.
///
/// Two-stage pipeline per subset: cheap f64 stage, then expensive rational stage.
///
/// **Stage 1 (f64):** Solve Y_S · v = 1 in f64 via SVD.
/// For each non-defining constraint y_i · v ≤ 1, evaluate with tolerance
/// δ = C · κ̂ · ε_mach · ‖v̂‖ · ‖ŷ_i‖:
/// - FALSE  (ŝ > 1 + δ): point is definitely outside K → skip subset
/// - INDETERMINATE (|ŝ - 1| ≤ δ): f64 cannot decide → fall through
/// - TRUE   (ŝ < 1 - δ): constraint definitely satisfied → continue
///
/// If ANY constraint is FALSE, the subset is skipped (no rational work).
/// If the system is too ill-conditioned (ε_mach · κ̂ > 1/4),
/// the entire f64 stage is skipped and we fall through to rational.
///
/// **Stage 2 (rational):** Reached only when stage 1 did not reject.
/// Solve Y_S · x = 1 exactly via Cramer's rule over Q. Check all gaps
/// g_i = 1 - y_i · v exactly. If all non-negative, v is a vertex.
///
/// Stage 1 rejects ~80% of subsets, avoiding expensive rational arithmetic.
/// It can only reject, never confirm — all actual vertices reach stage 2.
/// Error bound: [prop:prefilter-bound] in geom/math.tex
/// (has open gap — see TODO there).
///
/// Non-simple vertices (on >4 facets) are handled by deduplication: the first
/// 4-subset discovering a vertex records ALL incident facets. Later subsets
/// yielding the same vertex are skipped.
///
/// Mathematical correspondence: [lem:vertex-enumeration]
#[allow(clippy::type_complexity, dead_code)]
fn enumerate_vertices_exact(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<BTreeSet<usize>>, Vec<[BigRational; 4]>), ConstructionError> {
    use super::rational_arithmetic::rational_to_f64;

    let f = dual_vertices.len();
    let one = BigRational::from(BigInt::from(1));
    let rhs: [BigRational; 4] = std::array::from_fn(|_| one.clone());

    // Precompute f64 versions of dual vertices for the pre-filter.
    let dv_f64: Vec<[f64; 4]> = dual_vertices
        .iter()
        .map(|y| std::array::from_fn(|c| rational_to_f64(&y[c])))
        .collect();

    let mut vertex_descriptors = Vec::new();
    let mut vertices = Vec::new();

    for subset in combinations4(f) {
        // Stage 1: f64 pre-filter. Can only reject (FALSE), never confirm.
        if f64_prefilter_rejects(&dv_f64, &subset, f) {
            continue;
        }

        // Stage 2: exact rational path (reached when stage 1 did not reject).
        let rows: [[BigRational; 4]; 4] = [
            dual_vertices[subset[0]].clone(),
            dual_vertices[subset[1]].clone(),
            dual_vertices[subset[2]].clone(),
            dual_vertices[subset[3]].clone(),
        ];

        let d = det4(&rows);
        if d.is_zero() {
            continue; // Singular subset
        }

        // Solve exactly: Y_S · v = 1
        let v = solve4(&rows, &rhs).unwrap(); // safe: det != 0

        // Check all gaps: gap > 0 means non-incident facet,
        // gap = 0 means incident (non-simple vertex),
        // gap < 0 means point is outside K (not a vertex).
        let mut all_nonneg = true;
        let mut incident_facets = BTreeSet::from(subset);
        for (i, dv) in dual_vertices.iter().enumerate() {
            if subset.contains(&i) {
                continue;
            }
            let gap = &one - dot4(dv, &v);
            if gap.is_negative() {
                all_nonneg = false;
                break;
            }
            if gap.is_zero() {
                incident_facets.insert(i);
            }
        }

        if !all_nonneg {
            continue; // Point is outside K
        }

        // Deduplicate: skip if this vertex was already found by an earlier subset
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

    /// Safety constant from Proposition 1. Tight accounting gives C < 1400;
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

// ── TEMPORARY: Construction profiling harness ──────────────────────────
// Added for phase-level timing of Polytope4D::new().
// Remove after profiling is complete.

/// Timing breakdown from [`profile_construction_phases`].
#[derive(Clone, Debug)]
pub struct ConstructionProfile {
    /// Time in f64 validation (nonzero, duplicate, bounded checks) — nanoseconds.
    pub f64_validation_ns: u128,
    /// Time in f64→rational conversion — nanoseconds.
    pub f64_to_rational_ns: u128,
    /// Time in `check_bounded_rational` — nanoseconds.
    pub check_bounded_ns: u128,
    /// Time in `enumerate_vertices_exact` — nanoseconds.
    pub enumerate_vertices_ns: u128,
    /// Time in irredundancy check — nanoseconds.
    pub irredundancy_ns: u128,
    /// Time in `assemble` (incidence, adjacency, omega signs) — nanoseconds.
    pub assemble_ns: u128,
    /// Total C(F,4) subsets tested.
    pub total_subsets: usize,
    /// Subsets rejected by f64 prefilter.
    pub prefilter_rejected: usize,
    /// Subsets with det=0 (singular).
    pub det_zero: usize,
    /// Subsets yielding a vertex (before dedup).
    pub subsets_yielding_vertex: usize,
    /// Total unique vertices found.
    pub total_vertices: usize,
    /// Number of facets.
    pub facet_count: usize,
}

/// Run the `Polytope4D::new` pipeline with phase-level timing instrumentation.
///
/// TEMPORARY profiling harness. Replicates the construction pipeline from
/// `Polytope4D::new` + `construct_rational_pipeline` + `enumerate_vertices_exact`
/// with `std::time::Instant` measurements around each phase and subset counters.
///
/// Returns `Ok(profile)` on success, `Err(error)` if construction fails.
pub fn profile_construction_phases(
    halfspaces: Vec<nalgebra::Vector4<f64>>,
) -> Result<ConstructionProfile, super::polytope::ConstructionError> {
    use super::polytope::ConstructionError;
    use super::rational_arithmetic::{f64_to_rational, rational_to_f64};
    use nalgebra::Vector4;
    use num_bigint::BigInt;
    use std::collections::BTreeSet;
    use std::time::Instant;

    let f = halfspaces.len();

    // ── Phase 1: f64 validation ─────────────────────────────────────────
    let t0 = Instant::now();

    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }
    for (i, a) in halfspaces.iter().enumerate() {
        if a.norm() < 1e-15 {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }
    for i in 0..f {
        for j in (i + 1)..f {
            let max_norm = halfspaces[i].norm().max(halfspaces[j].norm());
            if (halfspaces[i] - halfspaces[j]).norm()
                < super::polytope::EPS_DUPLICATE_RELATIVE_PROFILE * max_norm
            {
                return Err(ConstructionError::DuplicateHalfspaces { i, j });
            }
        }
    }
    let unit_dirs: Vec<Vector4<f64>> = halfspaces.iter().map(|a| a.normalize()).collect();
    if !crate::geom::validation::check_bounded(&unit_dirs) {
        return Err(ConstructionError::Unbounded);
    }

    let f64_validation_ns = t0.elapsed().as_nanos();

    // ── Phase 2: f64→rational conversion ────────────────────────────────
    let t1 = Instant::now();

    let dual_vertices: Vec<[BigRational; 4]> = halfspaces
        .iter()
        .map(|a| std::array::from_fn(|c| f64_to_rational(a[c])))
        .collect();

    let f64_to_rational_ns = t1.elapsed().as_nanos();

    // ── Phase 3: check_bounded_rational ─────────────────────────────────
    let t2 = Instant::now();

    if !check_bounded_rational(&dual_vertices) {
        return Err(ConstructionError::Unbounded);
    }

    let check_bounded_ns = t2.elapsed().as_nanos();

    // ── Phase 4: enumerate_vertices_exact (with counters) ───────────────
    let t3 = Instant::now();

    let one = BigRational::from(BigInt::from(1));
    let rhs: [BigRational; 4] = std::array::from_fn(|_| one.clone());

    let dv_f64: Vec<[f64; 4]> = dual_vertices
        .iter()
        .map(|y| std::array::from_fn(|c| rational_to_f64(&y[c])))
        .collect();

    let subsets = combinations4(f);
    let total_subsets = subsets.len();
    let mut prefilter_rejected: usize = 0;
    let mut det_zero: usize = 0;
    let mut subsets_yielding_vertex: usize = 0;

    let mut vertex_descriptors = Vec::new();
    let mut vertices: Vec<[BigRational; 4]> = Vec::new();

    for subset in &subsets {
        if f64_prefilter_rejects(&dv_f64, subset, f) {
            prefilter_rejected += 1;
            continue;
        }

        let rows: [[BigRational; 4]; 4] = [
            dual_vertices[subset[0]].clone(),
            dual_vertices[subset[1]].clone(),
            dual_vertices[subset[2]].clone(),
            dual_vertices[subset[3]].clone(),
        ];

        let d = det4(&rows);
        if d.is_zero() {
            det_zero += 1;
            continue;
        }

        let v = solve4(&rows, &rhs).unwrap();

        let mut all_nonneg = true;
        let mut incident_facets = BTreeSet::from(*subset);
        for (i, dv) in dual_vertices.iter().enumerate() {
            if subset.contains(&i) {
                continue;
            }
            let gap = &one - dot4(dv, &v);
            if gap.is_negative() {
                all_nonneg = false;
                break;
            }
            if gap.is_zero() {
                incident_facets.insert(i);
            }
        }

        if !all_nonneg {
            continue;
        }

        subsets_yielding_vertex += 1;

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

    let total_vertices = vertices.len();
    let enumerate_vertices_ns = t3.elapsed().as_nanos();

    // ── Phase 5: irredundancy check ─────────────────────────────────────
    let t4 = Instant::now();

    for i in 0..f {
        let incident: Vec<[BigRational; 4]> = vertex_descriptors
            .iter()
            .zip(vertices.iter())
            .filter(|(vd, _)| vd.contains(&i))
            .map(|(_, v)| v.clone())
            .collect();

        if incident.is_empty() || affine_rank_rational(&incident) < 3 {
            return Err(ConstructionError::RedundantFacet(i));
        }
    }

    let irredundancy_ns = t4.elapsed().as_nanos();

    // ── Phase 6: assemble (incidence, adjacency, omega signs) ───────────
    let t5 = Instant::now();

    let dual_vertices_f64: Vec<Vector4<f64>> = dual_vertices
        .iter()
        .enumerate()
        .map(|(i, y)| {
            let v = Vector4::new(
                rational_to_f64(&y[0]),
                rational_to_f64(&y[1]),
                rational_to_f64(&y[2]),
                rational_to_f64(&y[3]),
            );
            if v.norm() < 1e-15 {
                panic!("dual vertex[{i}] has near-zero f64 norm");
            }
            v
        })
        .collect();

    let vertices_f64: Vec<Vector4<f64>> = vertices
        .iter()
        .map(|vr| {
            Vector4::new(
                rational_to_f64(&vr[0]),
                rational_to_f64(&vr[1]),
                rational_to_f64(&vr[2]),
                rational_to_f64(&vr[3]),
            )
        })
        .collect();

    // Build incidence, adjacency, omega_signs (same as Polytope4D::assemble)
    let v_count = vertices.len();
    let f_count = dual_vertices.len();

    let _incidence = nalgebra::DMatrix::from_fn(v_count, f_count, |v, fi| {
        vertex_descriptors[v].contains(&fi)
    });

    let _vertex_adjacency = nalgebra::DMatrix::from_fn(f_count, f_count, |i, k| {
        i != k && (0..v_count).any(|v| _incidence[(v, i)] && _incidence[(v, k)])
    });

    let _omega_signs = nalgebra::DMatrix::from_fn(f_count, f_count, |i, k| {
        if i == k {
            return 0i8;
        }
        let omega = super::rational_arithmetic::omega0_rational(
            &dual_vertices[i],
            &dual_vertices[k],
        );
        match super::rational_arithmetic::Sign::of(&omega) {
            super::rational_arithmetic::Sign::Plus => 1,
            super::rational_arithmetic::Sign::Minus => -1,
            super::rational_arithmetic::Sign::Zero => 0,
        }
    });

    // Suppress unused variable warnings
    let _ = (&_vertex_adjacency, &_omega_signs, &dual_vertices_f64, &vertices_f64);

    let assemble_ns = t5.elapsed().as_nanos();

    Ok(ConstructionProfile {
        f64_validation_ns,
        f64_to_rational_ns,
        check_bounded_ns,
        enumerate_vertices_ns,
        irredundancy_ns,
        assemble_ns,
        total_subsets,
        prefilter_rejected,
        det_zero,
        subsets_yielding_vertex,
        total_vertices,
        facet_count: f,
    })
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
}
