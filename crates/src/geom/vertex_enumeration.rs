//! Exact vertex enumeration for 4D polytopes over Q.
//!
//! Takes dual vertices y_i (vertices of the polar body K°) and computes:
//! - **Vertices** of K by solving y_i · x = 1 for all C(F,4) four-element subsets
//! - **Vertex-facet incidence**: which vertices lie on which facets
//! - **Boundedness**: dual vertices positively span R^4
//! - **Irredundancy**: every facet has incident vertices of affine rank 3
//!
//! All arithmetic is exact over Q — no floating-point tolerances.
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
/// The boundedness check here is the authoritative exact check (distinct from
/// the f64 pre-filter in `validation::check_bounded` which may be indeterminate
/// near the boundary).
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

    // Boundedness: dual vertices must positively span R^4
    if !check_bounded_rational(dual_vertices) {
        return Err(ConstructionError::Unbounded);
    }

    // Enumerate vertices exactly (solves y_i · x = 1 for all C(F,4) subsets)
    let (vertex_descriptors, vertices) = enumerate_vertices_exact(dual_vertices)?;

    // Irredundancy: every facet must have incident vertices spanning a 3D
    // affine subspace (the facet hyperplane).
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

    Ok((vertices, vertex_descriptors))
}

/// Enumerate all vertices by testing all C(F, 4) subsets exactly.
///
/// For each 4-element subset S of dual vertices:
/// 1. Build the 4x4 system Y_S · x = 1 (constant RHS).
/// 2. Compute det(Y_S) exactly. If zero, skip (singular).
/// 3. Solve Y_S · x = 1 exactly via Cramer's rule.
/// 4. Check all gaps g_i = 1 - y_i · v >= 0 for i not in S (exact).
/// 5. If all non-negative, v is a vertex; its descriptor is the set of ALL
///    facets with gap = 0 (including the defining subset S).
///
/// Non-simple vertices (on >4 facets) are handled by deduplication: the first
/// 4-subset discovering a vertex records ALL incident facets. Later subsets
/// yielding the same vertex are skipped.
///
/// Mathematical correspondence: [lem:vertex-enumeration]
#[allow(clippy::type_complexity)]
fn enumerate_vertices_exact(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<BTreeSet<usize>>, Vec<[BigRational; 4]>), ConstructionError> {
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
