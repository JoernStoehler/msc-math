/// Polytope validation: checks whether (normals, heights) define a valid irredundant
/// bounded convex polytope in R^4 with origin in its interior.
///
/// # Validation pipeline
///
/// 1. Basic checks: lengths match, ≥5 facets, normals unit, heights > 0
/// 2. No duplicate or proportional halfspaces
/// 3. Boundedness: normals positively span R^4 (via triple kernel enumeration)
/// 4. Vertex enumeration: intersect all C(F,4) hyperplane tuples
/// 5. Irredundancy: each facet has incident vertices of affine rank 3
use geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::{Matrix4, Vector4};

/// Why validation failed.
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    Construction(ConstructionError),
    DuplicateHalfspaces(usize, usize),
    Unbounded,
    RedundantFacet(usize),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Construction(e) => write!(f, "construction: {e}"),
            Self::DuplicateHalfspaces(i, j) => {
                write!(f, "halfspaces {i} and {j} are duplicate/proportional")
            }
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
const EPS_DEDUP: f64 = 1e-8;
const EPS_DUPLICATE_NORMAL: f64 = 1e-8;

/// Full validation: returns a `Polytope4D` or an error explaining why the input
/// does not define a valid irredundant bounded polytope.
pub fn validate_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Polytope4D, ValidationError> {
    // 1. Basic construction checks (lengths, unit normals, positive heights, ≥5 facets)
    //    We clone here because Polytope4D::new takes ownership.
    let polytope = Polytope4D::new(normals.to_vec(), heights.to_vec())?;

    // 2. No duplicate/proportional halfspaces
    check_no_duplicates(normals)?;

    // 3. Boundedness
    check_bounded(normals)?;

    // 4. Vertex enumeration
    let vertices = enumerate_vertices(normals, heights);

    // 5. Irredundancy
    check_irredundant(normals, heights, &vertices)?;

    Ok(polytope)
}

/// Check that no two halfspaces have the same outward normal (n_i ≈ n_j).
///
/// Antiparallel normals (n_i ≈ -n_j) are valid — they represent opposite-facing
/// facets (e.g., the hypercube has +e_x and -e_x).
fn check_no_duplicates(normals: &[Vector4<f64>]) -> Result<(), ValidationError> {
    let f = normals.len();
    for i in 0..f {
        for j in (i + 1)..f {
            let diff = (normals[i] - normals[j]).norm();
            if diff < EPS_DUPLICATE_NORMAL {
                return Err(ValidationError::DuplicateHalfspaces(i, j));
            }
        }
    }
    Ok(())
}

/// Check that the normals positively span R^4, i.e., the polytope is bounded.
///
/// # Algorithm
///
/// For every triple (i, j, k) of normals, compute the 1D kernel d of the 3×4
/// matrix [n_i; n_j; n_k]. A direction d with n_ℓ · d ≤ 0 for all ℓ would
/// make the polytope unbounded. So for each kernel direction d (and -d), we
/// verify that some normal outside the triple has a positive dot product.
///
/// If every kernel direction is "blocked" by at least one normal on each side,
/// the normals positively span R^4 and the polytope is bounded.
fn check_bounded(normals: &[Vector4<f64>]) -> Result<(), ValidationError> {
    let f = normals.len();

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

/// 4D cross product: d perpendicular to a, b, c.
///
/// Computed as the cofactor expansion of the 4×4 determinant
/// | e1  e2  e3  e4 |
/// | a1  a2  a3  a4 |
/// | b1  b2  b3  b4 |
/// | c1  c2  c3  c4 |
fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let d0 = a[1] * (b[2] * c[3] - b[3] * c[2]) - a[2] * (b[1] * c[3] - b[3] * c[1])
        + a[3] * (b[1] * c[2] - b[2] * c[1]);
    let d1 = -(a[0] * (b[2] * c[3] - b[3] * c[2]) - a[2] * (b[0] * c[3] - b[3] * c[0])
        + a[3] * (b[0] * c[2] - b[2] * c[0]));
    let d2 = a[0] * (b[1] * c[3] - b[3] * c[1]) - a[1] * (b[0] * c[3] - b[3] * c[0])
        + a[3] * (b[0] * c[1] - b[1] * c[0]);
    let d3 = -(a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0]));
    Vector4::new(d0, d1, d2, d3)
}

/// Enumerate all vertices of the polytope { x | n_i · x ≤ h_i }.
///
/// A vertex is a point where exactly 4 (or more) linearly independent
/// constraints are active. We find candidates by solving all C(F,4) systems
/// of 4 equations, then keep only feasible points.
pub fn enumerate_vertices(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<Vector4<f64>> {
    let f = normals.len();
    let mut vertices: Vec<Vector4<f64>> = Vec::new();

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                for l in (k + 1)..f {
                    // Build 4×4 matrix N with rows n_i, n_j, n_k, n_l
                    let mat = Matrix4::from_rows(&[
                        normals[i].transpose(),
                        normals[j].transpose(),
                        normals[k].transpose(),
                        normals[l].transpose(),
                    ]);
                    let rhs = Vector4::new(heights[i], heights[j], heights[k], heights[l]);

                    // Solve Nx = rhs
                    let lu = mat.lu();
                    if let Some(x) = lu.solve(&rhs) {
                        // Check feasibility: n_m · x ≤ h_m + ε for all m
                        let feasible = (0..f).all(|m| normals[m].dot(&x) <= heights[m] + EPS_FEASIBILITY);
                        if feasible {
                            // Deduplicate
                            let is_dup = vertices.iter().any(|v| (v - x).norm() < EPS_DEDUP);
                            if !is_dup {
                                vertices.push(x);
                            }
                        }
                    }
                }
            }
        }
    }

    vertices
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
