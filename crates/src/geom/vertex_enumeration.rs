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
#[allow(clippy::type_complexity)]
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
        Polytope4D::from_rationals(normals, heights).expect("simplex construction")
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
        Polytope4D::from_rationals(normals, heights).expect("hypercube construction")
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
        let p =
            Polytope4D::from_rationals(normals, heights).expect("non-simple polytope should succeed");

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
