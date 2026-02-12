/// Polytope validation: checks whether (normals, heights) define a valid irredundant
/// bounded convex polytope in R^4 with origin in its interior.
///
/// # Validation pipeline
///
/// 1. Basic checks + duplicate detection: via `Polytope4D::new()`
///    (lengths match, ≥5 facets, normals unit, heights > 0, no duplicate normals)
/// 2. Boundedness: normals positively span R^4 (via triple kernel enumeration)
/// 3. Irredundancy: each facet has incident vertices of affine rank 3
///    (vertices already computed by `Polytope4D::new()`)
use geom::cross_product::cross_product_4d;
use geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

/// Why validation failed.
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    Construction(ConstructionError),
    Unbounded,
    RedundantFacet(usize),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Construction(e) => write!(f, "construction: {e}"),
            Self::Unbounded => write!(f, "polytope is unbounded"),
            Self::RedundantFacet(i) => write!(f, "facet {i} is redundant"),
        }
    }
}

impl From<ConstructionError> for ValidationError {
    fn from(e: ConstructionError) -> Self {
        Self::Construction(e)
    }
}

const EPS_UNIT: f64 = 1e-9;
const EPS_FEASIBILITY: f64 = 1e-8;

/// Full validation: returns a `Polytope4D` or an error explaining why the input
/// does not define a valid irredundant bounded polytope.
pub fn validate_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Polytope4D, ValidationError> {
    // 1. Basic construction checks + duplicate detection + vertex enumeration
    let polytope = Polytope4D::new(normals.to_vec(), heights.to_vec())?;

    // 2. Boundedness
    check_bounded(normals)?;

    // 3. Irredundancy (uses precomputed vertices)
    check_irredundant(normals, heights, polytope.vertices())?;

    Ok(polytope)
}

/// Check that the normals positively span R^4, i.e., the polytope is bounded.
///
/// # Mathematical Background
///
/// **Proposition.** Let K = {x : Ax ≤ h} where A has rows n_1, …, n_F and h_ℓ > 0
/// for all ℓ (so 0 ∈ int K). Then K is bounded if and only if the normals span ℝ⁴
/// and, for every triple (i,j,k) whose normals have a 1-dimensional kernel
/// direction d, some normal has positive inner product with d and some has negative.
///
/// *Proof.* The recession cone rec(K) = {d : Ad ≤ 0} satisfies rec(K) ⊂ K (since
/// 0 ∈ K). So K bounded ⟺ rec(K) = {0} ⟺ normals positively span ℝ⁴, where
/// "positively span" means: no nonzero d has n_ℓ · d ≤ 0 for all ℓ.
///
/// (⇒) Positive span → all triples pass: immediate, since kernel directions are
/// nonzero and positive span gives a blocking normal outside the triple.
///
/// (⇐) Contrapositive: if rec(K) ≠ {0}, some triple fails. Pick d ∈ rec(K) (unit)
/// maximizing |S₀| where S₀ = {ℓ : n_ℓ · d = 0}. If S₀ has < 3 independent
/// normals, (span S₀)⊥ has dim ≥ 2, contains d, so ∃ v ∈ (span S₀)⊥ not ∥ d.
/// Perturbing d → d + tv stays in rec(K) for small |t| (S₀ stays tight, strict
/// inequalities stay negative). Increasing |t|, a new constraint becomes tight,
/// contradicting maximality of |S₀|. So d ⊥ three independent normals n_i, n_j, n_k,
/// d ∈ ker(n_i, n_j, n_k), and n_ℓ · d ≤ 0 ∀ℓ — triple (i,j,k) fails. ∎
///
/// # Algorithm
///
/// 1. Check rank(A) = 4 (normals span ℝ⁴). Without this, the triple check below
///    is vacuously true when rank ≤ 2, yet K is unbounded.
/// 2. Enumerate all triples (i, j, k) and compute their 1D kernel d (via 4D cross
///    product). For each kernel direction d, check that some normal has n_ℓ · d > ε
///    and some has n_ℓ · d < −ε.
///
/// **Complexity:** O(F³) for F facets (all triples), with O(1) kernel computation per triple.
fn check_bounded(normals: &[Vector4<f64>]) -> Result<(), ValidationError> {
    let f = normals.len();

    // Quick rank check: normals must span R^4.
    // Without this, the triple check below is vacuously true when rank ≤ 2
    // (no triple has a 1D kernel), yet K is unbounded.
    // Probability zero for random normals on S^3, but checked for correctness.
    let mat = nalgebra::DMatrix::from_fn(f, 4, |r, c| normals[r][c]);
    let svd = mat.svd(false, false);
    let rank = svd.singular_values.iter().filter(|&&s| s > 1e-8).count();
    if rank < 4 {
        return Err(ValidationError::Unbounded);
    }

    // For each triple, compute the kernel of the 3×4 matrix.
    // The kernel is the intersection of 3 hyperplanes through the origin,
    // which is generically a 1D subspace in R^4.
    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                // Build 3×4 matrix and find its kernel (null space).
                // We use a 4×4 system approach: find d such that n_i·d = n_j·d = n_k·d = 0.
                // The kernel has dimension ≥ 4-3 = 1.
                let kernel_dirs = kernel_of_three(normals[i], normals[j], normals[k]);

                for d in &kernel_dirs {
                    if d.norm() < 1e-12 {
                        continue;
                    }
                    // Check: is there any normal outside {i,j,k} with positive dot product with d?
                    let has_pos = (0..f)
                        .filter(|&l| l != i && l != j && l != k)
                        .any(|l| normals[l].dot(d) > EPS_UNIT);
                    // Same for -d
                    let has_neg = (0..f)
                        .filter(|&l| l != i && l != j && l != k)
                        .any(|l| normals[l].dot(d) < -EPS_UNIT);

                    if !has_pos || !has_neg {
                        return Err(ValidationError::Unbounded);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Compute kernel direction(s) of three vectors in R^4.
///
/// Given n1, n2, n3 ∈ R^4, finds vector(s) d such that n1·d = n2·d = n3·d = 0.
/// Generically the kernel is 1D; returns up to 1 direction (both signs checked by caller).
fn kernel_of_three(n1: Vector4<f64>, n2: Vector4<f64>, n3: Vector4<f64>) -> Vec<Vector4<f64>> {
    // Construct a 4×4 matrix where the first 3 rows are n1, n2, n3
    // and find the null space of the 3×4 submatrix.
    //
    // Approach: use the 4D cross product (determinant expansion).
    // d = n1 × n2 × n3 (generalized cross product in R^4).
    let d = cross_product_4d(n1, n2, n3);
    if d.norm() < 1e-12 {
        // Degenerate: normals are not linearly independent.
        // Kernel is ≥2D; we'd need a more sophisticated approach.
        // For our use case (random normals), this is extremely unlikely.
        // Return empty — this triple doesn't constrain boundedness.
        return vec![];
    }
    vec![d.normalize()]
}

/// Check that every facet is irredundant: its incident vertices affinely span
/// a 3D subspace (the facet hyperplane).
fn check_irredundant(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
) -> Result<(), ValidationError> {
    let f = normals.len();

    for i in 0..f {
        // Collect vertices incident to facet i: n_i · v ≈ h_i
        let incident: Vec<Vector4<f64>> = vertices
            .iter()
            .filter(|v| (normals[i].dot(v) - heights[i]).abs() < EPS_FEASIBILITY)
            .cloned()
            .collect();

        if incident.is_empty() {
            return Err(ValidationError::RedundantFacet(i));
        }

        // Check affine rank = 3 (i.e., the incident vertices span the facet hyperplane).
        // Center the vertices and compute SVD to find rank.
        let rank = affine_rank(&incident);
        if rank < 3 {
            return Err(ValidationError::RedundantFacet(i));
        }
    }
    Ok(())
}

/// Compute the affine rank of a set of points.
/// Affine rank = dimension of their affine span = rank of centered points matrix.
fn affine_rank(points: &[Vector4<f64>]) -> usize {
    if points.len() <= 1 {
        return 0;
    }

    // Center around first point
    let base = points[0];
    let centered: Vec<Vector4<f64>> = points[1..].iter().map(|p| p - base).collect();

    // Build matrix with centered points as rows
    let n = centered.len();
    let mat = nalgebra::DMatrix::from_fn(n, 4, |r, c| centered[r][c]);

    // SVD and count significant singular values
    let svd = mat.svd(false, false);
    let threshold = 1e-8;
    svd.singular_values
        .iter()
        .filter(|&&s| s > threshold)
        .count()
}

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;
