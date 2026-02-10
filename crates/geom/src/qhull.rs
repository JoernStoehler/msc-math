/// Qhull C library FFI wrapper for halfspace intersection.
///
/// **STATUS:** Interface defined, implementation TODO for next agent session.
///
/// This module isolates all qhull C API calls so future code doesn't need
/// to deal with raw FFI. The C API is weird (command-line args baked in,
/// multi-step initialization, void functions that may call exit(), etc.).
///
/// # Requirements
///
/// The `halfspace_intersection_4d` function must:
/// 1. Accept halfspaces in format {x ∈ ℝ⁴ : n·x ≤ h} with unit normals
/// 2. Return ALL vertices of the polytope defined by the intersection
/// 3. Vertices must satisfy n·v ≤ h + ε for all halfspaces (ε ≈ 1e-6)
/// 4. Correctly handle 4D hypercube (16 vertices), cross-polytope (8 vertices), simplex (5 vertices)
///
/// # Next Steps for Implementation
///
/// 1. Study reference: `~/.cargo/registry/.../qhull-sys-0.4.0/qhull/src/qhalf/qhalf_r.c`
/// 2. Understand qhull duality: in halfspace mode, what list contains the output vertices?
/// 3. Clarify dimension parameter: does qh_init_B expect dim=4 (geometric) or dim=5 (data format)?
/// 4. Run tests incrementally: check vertex count first, then verify coordinates
/// 5. See function-level TODO comment for detailed findings from previous attempt
use nalgebra::Vector4;
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
pub enum QhullError {
    /// Qhull computation failed (details printed to stderr by qhull)
    ComputationFailed,
}

impl std::fmt::Display for QhullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputationFailed => write!(f, "qhull halfspace intersection failed"),
        }
    }
}

impl std::error::Error for QhullError {}

/// Compute vertices of 4D polytope from halfspace intersection.
///
/// Given halfspaces { x : n·x ≤ h } where n ∈ S³, h > 0, computes vertices
/// of the polytope K = ⋂ { x : nᵢ·x ≤ hᵢ }.
///
/// Uses qhull C library. Assumes origin [0,0,0,0] is in the interior (guaranteed
/// by h > 0 and unit normals for convex polytopes).
///
/// # Arguments
/// * `normals` - Unit normal vectors (n̂ᵢ ∈ S³)
/// * `heights` - Positive heights (ĥᵢ > 0)
///
/// # Returns
/// Vec of vertices as Vector4<f64>
pub(crate) fn halfspace_intersection_4d(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Vec<Vector4<f64>>, QhullError> {
    // Convert to qhull format: a₁x₁ + a₂x₂ + a₃x₃ + a₄x₄ + b ≤ 0
    // Our format: n·x ≤ h → n·x - h ≤ 0
    // So: [n₁, n₂, n₃, n₄, -h]
    let num_halfspaces = normals.len();
    let dim = 4;
    let coords_per_halfspace = dim + 1; // 5

    let mut halfspaces: Vec<f64> = Vec::with_capacity(num_halfspaces * coords_per_halfspace);

    for (n, &h) in normals.iter().zip(heights.iter()) {
        halfspaces.push(n[0]);
        halfspaces.push(n[1]);
        halfspaces.push(n[2]);
        halfspaces.push(n[3]);
        halfspaces.push(-h);
    }

    // TODO: Implement qhull C FFI wrapper for halfspace intersection
    //
    // REQUIREMENTS:
    // - Convert halfspaces from format {x : n·x ≤ h} to qhull format
    // - Call qhull C library (qhull-sys crate) in halfspace intersection mode
    // - Extract output vertices as Vec<Vector4<f64>>
    // - Handle all memory management correctly (qhull uses malloc/free)
    //
    // KNOWN ISSUES WITH PREVIOUS ATTEMPT (commit c7392e0):
    // - Returns 7 vertices for hypercube [-1,1]^4 (expected 16)
    // - Returns 7 vertices for other test cases (all wrong)
    // - Volume tests fail with volume=0
    //
    // INVESTIGATION FINDINGS:
    // - qhull C API is complex: requires qh_init_A, qh_init_B, qh_qhull, qh_prepare_output
    // - Halfspace mode uses duality: need to understand vertex vs facet lists
    // - Dimension parameter semantics unclear: dim=4 or dim=5 for 4D halfspaces?
    // - Reference implementation: ~/.cargo/registry/.../qhull-sys-0.4.0/qhull/src/qhalf/qhalf_r.c
    //
    // TESTS:
    // - See test module below for required behavior
    // - Must pass: hypercube (16 vertices), cross-polytope (8 vertices), simplex (5 vertices)
    // - All vertices must satisfy n·v ≤ h for all input halfspaces
    //
    // APPROACH SUGGESTIONS:
    // 1. Study qhalf_r.c main() function completely before coding
    // 2. Test incrementally: print vertex count first, then coordinates
    // 3. Verify against known polytopes (hypercube is simplest test case)
    // 4. Consider using qhull Rust crate as reference (but it has bugs in halfspace mode)
    unimplemented!(
        "qhull halfspace intersection not yet implemented - see TODO comment above"
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::Vector4;

    /// Test case: 4D hypercube [-1,1]^4
    /// Expected: 16 vertices at all combinations of (±1, ±1, ±1, ±1)
    #[test]
    #[ignore] // TODO: Enable when qhull wrapper is correctly implemented
    fn hypercube_vertices() {
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),   // x₁ ≤ 1
            Vector4::new(-1.0, 0.0, 0.0, 0.0),  // -x₁ ≤ 1
            Vector4::new(0.0, 1.0, 0.0, 0.0),   // x₂ ≤ 1
            Vector4::new(0.0, -1.0, 0.0, 0.0),  // -x₂ ≤ 1
            Vector4::new(0.0, 0.0, 1.0, 0.0),   // x₃ ≤ 1
            Vector4::new(0.0, 0.0, -1.0, 0.0),  // -x₃ ≤ 1
            Vector4::new(0.0, 0.0, 0.0, 1.0),   // x₄ ≤ 1
            Vector4::new(0.0, 0.0, 0.0, -1.0),  // -x₄ ≤ 1
        ];
        let heights = vec![1.0; 8];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("hypercube halfspace intersection should succeed");

        assert_eq!(vertices.len(), 16, "hypercube [-1,1]^4 has 16 vertices");

        // All vertices should satisfy all halfspace constraints: n·v ≤ h
        for v in &vertices {
            for (n, &h) in normals.iter().zip(&heights) {
                assert!(
                    n.dot(v) <= h + 1e-6,
                    "vertex {:?} violates constraint {:?} · x ≤ {}",
                    v,
                    n,
                    h
                );
            }
        }

        // All vertices should be on the boundary (at least one constraint tight)
        for v in &vertices {
            let on_boundary = normals
                .iter()
                .zip(&heights)
                .any(|(n, &h)| (n.dot(v) - h).abs() < 1e-6);
            assert!(
                on_boundary,
                "vertex {:?} is not on any facet boundary",
                v
            );
        }
    }

    /// Test case: 4D cross-polytope (±2·eᵢ for i=1,2,3,4)
    /// Defined by: (±1,±1,±1,±1)/2 · x ≤ 1 (16 facets, 8 vertices)
    #[test]
    #[ignore] // TODO: Enable when qhull wrapper is correctly implemented
    fn crosspolytope_vertices() {
        let mut normals = Vec::with_capacity(16);
        for s0 in [-1.0_f64, 1.0] {
            for s1 in [-1.0_f64, 1.0] {
                for s2 in [-1.0_f64, 1.0] {
                    for s3 in [-1.0_f64, 1.0] {
                        normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                    }
                }
            }
        }
        let heights = vec![1.0; 16];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("crosspolytope halfspace intersection should succeed");

        assert_eq!(vertices.len(), 8, "4D cross-polytope has 8 vertices");

        // Check vertices satisfy constraints
        for v in &vertices {
            for (n, &h) in normals.iter().zip(&heights) {
                assert!(
                    n.dot(v) <= h + 1e-6,
                    "vertex {:?} violates constraint {:?} · x ≤ {}",
                    v,
                    n,
                    h
                );
            }
        }
    }

    /// Test case: 4D simplex with 5 vertices
    /// Standard simplex conv{0, e₁, e₂, e₃, e₄}
    #[test]
    #[ignore] // TODO: Enable when qhull wrapper is correctly implemented
    fn simplex_vertices() {
        let normals = vec![
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
            Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ];
        let heights = vec![0.0, 0.0, 0.0, 0.0, 0.5];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("simplex halfspace intersection should succeed");

        assert_eq!(vertices.len(), 5, "4D simplex has 5 vertices");
    }
}
