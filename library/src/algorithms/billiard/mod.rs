//! Billiard algorithm for EHZ capacity of Lagrangian product polytopes.
//!
//! Computes c_EHZ(K_q x_L K_p) for Lagrangian products where K_q, K_p are
//! convex polygons in R^2. This exploits the block structure of billiard orbits
//! in Lagrangian products to enumerate only O(n^3) candidate permutations
//! instead of the O(n!) of the general HK2017 algorithm.
//!
//! See [thm:billiard-characterization]: c_EHZ equals the minimum K_p-degree-length
//! billiard trajectory in K_q, and [thm:bounce-bound]: the minimiser has at most
//! 3 bounces.
//!
//! Uses `CapacityAccumulator` for the enumerate -> solve -> track pattern,
//! mirroring the hk2017 module's accumulator usage.
//!
//! Submodules:
//! - `block_enumeration` — block structure enumeration for Q/P facets
//! - `facet_classification` — classify facets into q-space and p-space types
//! - `kkt_benchmark` — KKT solver performance measurement
//!
//! Mathematical correspondence: [thm:billiard-characterization], [thm:bounce-bound]

pub mod block_enumeration;
pub mod facet_classification;
pub mod kkt_benchmark;

use crate::algorithms::facet_adjacency::{
    build_transition_matrix, is_feasible_cycle,
};
use crate::algorithms::orbit_search::{
    collect_legacy_capacity,
    collect_orbits,
    OrbitGuaranteeMode,
    OrbitSearchResult,
    OrbitSolveBackend,
};
use crate::geom::polytope::Polytope4D;
use block_enumeration::{enumerate_blocks, enumerate_k_bounce_sigmas};
use facet_classification::classify_facets;

/// Error type for the billiard algorithm.
///
/// Returned when the polytope is not a valid Lagrangian product: either a facet
/// has mixed q/p normal components, or there are too few facets of one type.
#[derive(Debug, Clone)]
pub enum BilliardError {
    /// A facet normal has both q and p components (not a Lagrangian product).
    NotLagrangianProduct {
        /// Index of the offending facet.
        facet: usize,
        /// The facet's normal vector [n0, n1, n2, n3].
        normal: [f64; 4],
    },
    /// Too few facets of a given type (need at least 3 for a polygon).
    TooFewFacets {
        /// Which facet type is deficient ("q" or "p").
        facet_type: &'static str,
        /// How many facets of this type were found.
        count: usize,
    },
}

impl std::fmt::Display for BilliardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BilliardError::NotLagrangianProduct { facet, normal } => {
                write!(
                    f,
                    "facet {} has mixed normal [{:.4}, {:.4}, {:.4}, {:.4}]: not a Lagrangian product",
                    facet, normal[0], normal[1], normal[2], normal[3]
                )
            }
            BilliardError::TooFewFacets { facet_type, count } => {
                write!(
                    f,
                    "only {} {}-facets (need at least 3 for a polygon)",
                    count, facet_type
                )
            }
        }
    }
}

impl std::error::Error for BilliardError {}

/// Error from the shared billiard orbit collector.
#[derive(Debug, Clone)]
pub enum BilliardOrbitSearchError {
    /// Input is not a valid Lagrangian product.
    InvalidInput(BilliardError),
    /// The shared orbit-search layer failed after input validation succeeded.
    Search(crate::algorithms::OrbitSearchError),
}

impl std::fmt::Display for BilliardOrbitSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(err) => err.fmt(f),
            Self::Search(err) => write!(f, "billiard orbit search failed: {err:?}"),
        }
    }
}

impl std::error::Error for BilliardOrbitSearchError {}

/// Result of the billiard capacity computation.
///
/// Wraps [`CapacityResult`](crate::algorithms::capacity_accumulator::CapacityResult)
/// (shared accumulator output) plus the algorithm-specific `bounce_count` field
/// indicating the k value (2 or 3) of the optimal orbit.
///
/// Access capacity fields via `.result.capacity` (no Deref -- explicit field access).
///
/// [thm:billiard-characterization]: result of block-structured enumeration.
#[derive(Clone, Debug)]
pub struct BilliardResult {
    /// Core capacity result from the accumulator: capacity, uncertainty, best
    /// permutation, beta vector, and iteration count.
    pub result: crate::algorithms::capacity_accumulator::CapacityResult,
    /// Number of bounces (k value) of the optimal orbit (2 or 3).
    pub bounce_count: usize,
}

/// Compute c_EHZ for a Lagrangian product polytope.
///
/// Returns error if the polytope is not a Lagrangian product (some facet normal
/// has both q and p components, or too few facets of one type).
///
/// Returns `Ok(None)` if no valid orbit is found (should not happen for valid
/// Lagrangian products, but guards against degenerate input).
///
/// [thm:billiard-characterization], [thm:bounce-bound]: enumerates block permutations
/// sigma = (Q_1 P_1 ... Q_k P_k) for k in {2, 3}.
pub fn billiard_capacity(
    polytope: &Polytope4D,
) -> Result<Option<BilliardResult>, BilliardError> {
    // Step 1: classify facets into q-type and p-type.
    let classification = classify_facets(polytope)?;

    // Step 2: build adjacency matrices.
    // Undirected: for block building (same-type adjacent pairs).
    let adj = polytope.vertex_adjacency();
    // Directed: for cycle pruning (omega_0 transition feasibility).
    let directed_adj = build_transition_matrix(polytope);

    // Step 3: enumerate blocks.
    let q_blocks = enumerate_blocks(&classification.q_indices, adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, adj);

    // Step 4: for k = 2, 3, enumerate sigma sequences and solve KKT.
    let (result, bounce_count) = match collect_legacy_capacity(
        polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| {
            for k in 2..=3 {
                enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
                    // Directed adjacency pruning: skip cycles violating omega_0 condition.
                    if !is_feasible_cycle(sigma, &directed_adj) {
                        return;
                    }
                    visit(sigma, k);
                });
            }
        },
        |_result| 2,
    ) {
        Some(outcome) => outcome,
        None => return Ok(None),
    };

    Ok(Some(BilliardResult {
        result,
        bounce_count,
    }))
}

/// Collect near-minimum billiard orbits for a Lagrangian product polytope.
pub fn billiard_minimum_orbits(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, BilliardOrbitSearchError> {
    let classification = classify_facets(polytope).map_err(BilliardOrbitSearchError::InvalidInput)?;
    let adj = polytope.vertex_adjacency();
    let directed_adj = build_transition_matrix(polytope);
    let q_blocks = enumerate_blocks(&classification.q_indices, adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, adj);

    collect_orbits(polytope, gap, mode, backend, |visit| {
        for k in 2..=3 {
            enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
                if !is_feasible_cycle(sigma, &directed_adj) {
                    return;
                }
                visit(sigma);
            });
        }
    })
    .map_err(BilliardOrbitSearchError::Search)
}

// Tests for billiard capacity: correctness and cross-validation with hk2017.
//
// Proposition: billiard_capacity agrees with known literature values and with
// ehz_capacity on all Lagrangian product test polytopes.
// Reference: [thm:billiard-characterization], [alg:billiard]
//
// Strategy: fixture-based (known polytopes), cross-algorithm (billiard vs hk2017).
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    /// Helper: assert billiard capacity matches expected value within tolerance.
    fn assert_capacity(
        name: &str,
        polytope: &crate::geom::polytope::Polytope4D,
        expected: f64,
        tol: f64,
    ) {
        let result = billiard_capacity(polytope)
            .unwrap_or_else(|e| panic!("{}: billiard_capacity returned error: {}", name, e))
            .unwrap_or_else(|| panic!("{}: billiard_capacity returned None", name));

        let diff = (result.result.capacity - expected).abs();
        assert!(
            diff < tol,
            "{}: capacity {:.10} != expected {:.10} (diff {:.2e}, tol {:.2e})",
            name,
            result.result.capacity,
            expected,
            diff,
            tol,
        );
    }

    // ============================================================
    // Agreement tests: billiard vs known values
    // ============================================================

    /// Verify billiard capacity of the hypercube matches the known value (4.0).
    #[test]
    fn hypercube_capacity() {
        let kp = known_polytopes::hypercube();
        assert_capacity("hypercube", &kp.polytope, 4.0, 1e-8);
    }

    /// Verify billiard capacity of the Lagrangian triangle product matches the known value (1.5).
    #[test]
    fn triangle_product_capacity() {
        let kp = known_polytopes::lagrangian_triangle_product();
        assert_capacity("triangle_product", &kp.polytope, 1.5, 1e-8);
    }

    /// Verify billiard capacity of the Lagrangian triangle-square product matches the known value (1.5).
    #[test]
    fn triangle_square_capacity() {
        let kp = known_polytopes::lagrangian_triangle_square();
        assert_capacity("triangle_square", &kp.polytope, 1.5, 1e-8);
    }

    /// Smoke-test the richer billiard collector on a known Lagrangian product.
    #[test]
    fn triangle_product_minimum_orbits_collector() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let result = billiard_minimum_orbits(
            &kp.polytope,
            0.0,
            crate::algorithms::OrbitGuaranteeMode::BoundSafe,
            crate::algorithms::OrbitSolveBackend::SaddlePoint,
        )
        .expect("billiard minimum-orbit collector should succeed");

        assert!(!result.orbits.is_empty(), "collector must return at least one orbit");
        assert!(result.min_action_lower <= result.min_action_upper);
    }

    /// Verify billiard capacity of the HK-O pentagon counterexample matches the analytic formula.
    ///
    /// **Why release mode:** ~50k KKT solves, too slow for debug suite.
    #[test]
    #[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
    fn hko_pentagon_capacity() {
        let kp = known_polytopes::hko_pentagon();
        let expected = 2.0
            * (std::f64::consts::PI / 10.0).cos()
            * (1.0 + (std::f64::consts::PI / 5.0).cos());
        assert_capacity("hko_pentagon", &kp.polytope, expected, 1e-6);
    }

    // ============================================================
    // Agreement tests: billiard vs hk2017
    // ============================================================

    /// Cross-algorithm check: billiard and hk2017 agree on hypercube capacity.
    #[test]
    #[ignore] // runs hk2017 live -- release-only cross-algorithm check
    fn agrees_with_hk2017_hypercube() {
        let kp = known_polytopes::hypercube();
        let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
        let hk = crate::algorithms::hk2017::ehz_capacity(&kp.polytope).unwrap();
        let diff = (billiard.result.capacity - hk.result.capacity).abs();
        assert!(
            diff < 1e-8,
            "hypercube: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
            billiard.result.capacity,
            hk.result.capacity,
            diff,
        );
    }

    /// Cross-algorithm check: billiard and hk2017 agree on triangle product capacity.
    #[test]
    #[ignore] // runs hk2017 live -- release-only cross-algorithm check
    fn agrees_with_hk2017_triangle_product() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
        let hk = crate::algorithms::hk2017::ehz_capacity(&kp.polytope).unwrap();
        let diff = (billiard.result.capacity - hk.result.capacity).abs();
        assert!(
            diff < 1e-8,
            "triangle_product: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
            billiard.result.capacity,
            hk.result.capacity,
            diff,
        );
    }

    /// Cross-algorithm check: billiard and hk2017 agree on triangle-square product capacity.
    #[test]
    #[ignore] // runs hk2017 live -- release-only cross-algorithm check
    fn agrees_with_hk2017_triangle_square() {
        let kp = known_polytopes::lagrangian_triangle_square();
        let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
        let hk = crate::algorithms::hk2017::ehz_capacity(&kp.polytope).unwrap();
        let diff = (billiard.result.capacity - hk.result.capacity).abs();
        assert!(
            diff < 1e-8,
            "triangle_square: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
            billiard.result.capacity,
            hk.result.capacity,
            diff,
        );
    }

    /// Cross-algorithm check: billiard and hk2017 agree on HK-O pentagon capacity.
    ///
    /// **Why release mode:** hk2017 on 10-facet pentagon is exponential (~60s even in release).
    #[test]
    #[ignore] // hk2017 on 10-facet pentagon takes ~60s; verified against known value instead
    fn agrees_with_hk2017_hko_pentagon() {
        let kp = known_polytopes::hko_pentagon();
        let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
        let hk = crate::algorithms::hk2017::ehz_capacity(&kp.polytope).unwrap();
        let diff = (billiard.result.capacity - hk.result.capacity).abs();
        assert!(
            diff < 1e-8,
            "hko_pentagon: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
            billiard.result.capacity,
            hk.result.capacity,
            diff,
        );
    }

    // ============================================================
    // Error handling tests
    // ============================================================

    /// Verify billiard algorithm rejects the simplex (not a Lagrangian product).
    #[test]
    fn rejects_non_lagrangian_product() {
        let kp = known_polytopes::simplex();
        let result = billiard_capacity(&kp.polytope);
        assert!(result.is_err(), "simplex should not be a Lagrangian product");
    }

    /// Verify billiard algorithm rejects the symplectic triangle product
    /// (normals in symplectic planes, not Lagrangian subspaces).
    #[test]
    fn rejects_symplectic_triangle_product() {
        let kp = known_polytopes::symplectic_triangle_product();
        let result = billiard_capacity(&kp.polytope);
        assert!(
            result.is_err(),
            "symplectic triangle product should not be a Lagrangian product"
        );
    }

    // ============================================================
    // Property tests
    // ============================================================

    /// Verify the billiard iteration count stays within a polynomial bound.
    ///
    /// **Why release mode:** ~50k KKT solves on the 10-facet pentagon.
    #[test]
    #[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
    fn billiard_iterations_polynomial() {
        let kp = known_polytopes::hko_pentagon();
        let result = billiard_capacity(&kp.polytope).unwrap().unwrap();
        // For 5+5 facets, expect on the order of 100k iterations.
        // If it exceeds 1M, something is wrong.
        assert!(
            result.result.iterations < 1_000_000,
            "pentagon: {} iterations exceeds polynomial bound",
            result.result.iterations,
        );
    }

    /// Check structural properties of a BilliardResult.
    fn assert_result_properties(name: &str, result: &BilliardResult) {
        // bounce_count is 2 or 3.
        assert!(
            result.bounce_count == 2 || result.bounce_count == 3,
            "{}: bounce_count = {} (expected 2 or 3)",
            name,
            result.bounce_count,
        );

        // All beta positive.
        for (i, &b) in result.result.best_beta.iter().enumerate() {
            assert!(b > 0.0, "{}: beta[{}] = {:.2e} <= 0", name, i, b);
        }

        // Permutation length matches 2k structure (between 2k and 4k).
        let k = result.bounce_count;
        let len = result.result.best_permutation.len();
        assert!(
            len >= 2 * k && len <= 4 * k,
            "{}: permutation len {} not in [{}, {}] for k={}",
            name,
            len,
            2 * k,
            4 * k,
            k,
        );
    }

    /// Verify structural properties of BilliardResult on small Lagrangian products.
    #[test]
    fn result_properties() {
        for (name, polytope) in lagrangian_test_cases_fast() {
            let result = billiard_capacity(&polytope).unwrap().unwrap();
            assert_result_properties(name, &result);
        }
    }

    /// Verify structural properties of BilliardResult on the HK-O pentagon.
    ///
    /// **Why release mode:** ~50k KKT solves, too slow for debug suite.
    #[test]
    #[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
    fn result_properties_pentagon() {
        let kp = known_polytopes::hko_pentagon();
        let result = billiard_capacity(&kp.polytope).unwrap().unwrap();
        assert_result_properties("hko_pentagon", &result);
    }

    /// Small Lagrangian products: fast in both debug and release.
    fn lagrangian_test_cases_fast() -> Vec<(&'static str, crate::geom::polytope::Polytope4D)> {
        vec![
            ("hypercube", known_polytopes::hypercube().polytope.clone()),
            (
                "triangle_product",
                known_polytopes::lagrangian_triangle_product().polytope.clone(),
            ),
            (
                "triangle_square",
                known_polytopes::lagrangian_triangle_square().polytope.clone(),
            ),
        ]
    }
}
