/// Investigation: what error does qhull return for unbounded polytopes?
///
/// This test file investigates whether qhull reliably detects unboundedness.
/// If yes, we can remove the O(F³) check_bounded() function in datasets/validation.rs
/// and rely on qhull's error handling instead.

#[cfg(test)]
mod investigation {
    use crate::vertices::compute_vertices;
    use crate::QhullError;
    use nalgebra::Vector4;

    /// Test 1: 4 halfspaces in 4D (underconstrained - unbounded)
    ///
    /// This system has only 4 constraints in 4D, so it's unbounded in at least
    /// one direction. Does qhull detect this and fail?
    #[test]
    #[ignore] // Run manually with: cargo test --package geom unbounded_test1 -- --ignored --nocapture
    fn unbounded_test1_underconstrained() {
        println!("\n=== Test 1: 4 halfspaces in 4D (underconstrained) ===");
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let heights = vec![1.0, 1.0, 1.0, 1.0];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✗ UNEXPECTED SUCCESS");
                println!("  Returned {} vertices", vertices.len());
                panic!("Expected qhull to fail on unbounded polytope, but it succeeded!");
            }
            Err(e) => {
                println!("✓ FAILED AS EXPECTED");
                println!("  Error: {}", e);
                match e {
                    QhullError::ComputationFailed(stderr) => {
                        println!("  Stderr: {}", stderr);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Test 2: 5 halfspaces all pointing roughly the same direction (unbounded)
    ///
    /// These halfspaces all have positive x-component and small perturbations.
    /// The normals don't positively span R^4, so the polytope is unbounded.
    #[test]
    #[ignore] // Run manually
    fn unbounded_test2_unidirectional() {
        println!("\n=== Test 2: 5 halfspaces pointing same direction ===");
        let normals = vec![
            Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
        ];
        let heights = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✗ UNEXPECTED SUCCESS");
                println!("  Returned {} vertices", vertices.len());
                panic!("Expected qhull to fail on unbounded polytope, but it succeeded!");
            }
            Err(e) => {
                println!("✓ FAILED AS EXPECTED");
                println!("  Error: {}", e);
                match e {
                    QhullError::ComputationFailed(stderr) => {
                        println!("  Stderr: {}", stderr);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Test 3: Valid bounded hypercube (control - should succeed)
    #[test]
    #[ignore] // Run manually
    fn bounded_test_hypercube_control() {
        println!("\n=== Test 3: Valid bounded hypercube (control) ===");
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ];
        let heights = vec![1.0; 8];

        match compute_vertices(&normals, &heights) {
            Ok(vertices) => {
                println!("✓ SUCCESS AS EXPECTED");
                println!("  Returned {} vertices", vertices.len());
                assert_eq!(vertices.len(), 16, "4D hypercube should have 16 vertices");
            }
            Err(e) => {
                println!("✗ UNEXPECTED FAILURE");
                println!("  Error: {}", e);
                panic!("Expected bounded hypercube to succeed, but qhull failed!");
            }
        }
    }
}
