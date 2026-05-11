//! Flat known-incidence facet volume helpers for 4D polytopes.
//!
//! The maintained implementation lives in `euclidean-polytopes`. This module
//! exposes the same flat helpers through the symplectic geometry namespace
//! without rebuilding `Polytope4D`-shaped compatibility wrappers.
//!
//! Mathematical correspondence: [def:volume] (per-facet specialization)

pub use euclidean_polytopes::{
    facet_volume_and_centroid_from_incidence_f64, facet_volume_from_incidence_f64,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    // Tests for facet volume: per-facet 3D volumes of 4D polytope facets.
    //
    // Proposition: facet_volume_from_incidence_f64 computes correct 3D volumes
    // for facets of known polytopes. For the hypercube [-1,1]^4, each facet is
    // a cube [-1,1]^3 with volume 8.
    // Reference: [def:volume] (per-facet specialization)
    //
    // Strategy: fixture-based (hypercube: exact volumes known;
    //   crosspolytope: divergence-theorem cross-check with exact Euclidean volume)

    /// Each facet of [-1,1]^4 is a cube [-1,1]^3 with volume 8.
    #[test]
    fn hypercube_facet_volumes() {
        let polytope = &known_polytopes::hypercube().polytope;
        let f = polytope.facet_count();
        assert_eq!(f, 8, "hypercube should have 8 facets");

        for fi in 0..f {
            let vol =
                facet_volume_from_incidence_f64(polytope.vertices_f64(), polytope.incidence(), fi)
                    .expect("valid known polytope fixture");
            assert!(
                (vol - 8.0).abs() < 1e-6,
                "facet {fi}: volume = {vol}, expected 8.0"
            );
        }
    }

    /// Sum of facet volumes × h_i / 4 = polytope volume (divergence theorem).
    /// h_i = 1/|a_i|. For [-1,1]^4: h_i = 1, sum = 8 * (8.0 * 1.0) / 4 = 16.0.
    #[test]
    fn facet_volume_sum_equals_polytope_volume() {
        let polytope = &known_polytopes::hypercube().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| {
                facet_volume_from_incidence_f64(polytope.vertices_f64(), polytope.incidence(), fi)
                    .expect("valid known polytope fixture")
                    * (1.0 / duals[fi].norm())
            })
            .sum::<f64>()
            / 4.0;

        let polytope_volume =
            crate::test_lib::euclidean_volume_f64(polytope.vertices(), polytope.incidence());

        assert!(
            (vol_from_facets - polytope_volume).abs() / polytope_volume < 1e-6,
            "facet sum = {vol_from_facets}, polytope volume = {polytope_volume}"
        );
    }

    /// Facet volume and centroid: centroid should lie on the facet hyperplane.
    #[test]
    fn facet_centroid_on_hyperplane() {
        let polytope = &known_polytopes::hypercube().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        for (fi, dual) in duals.iter().enumerate().take(f) {
            let (vol, centroid) = facet_volume_and_centroid_from_incidence_f64(
                polytope.vertices_f64(),
                polytope.incidence(),
                fi,
            )
            .expect("valid known polytope fixture");
            assert!(vol > 0.0, "facet {fi} should have positive volume");
            let dot = dual.dot(&centroid);
            assert!(
                (dot - 1.0).abs() < 1e-6,
                "facet {fi}: centroid not on hyperplane, a·c = {dot}, expected 1.0",
            );
        }
    }

    /// Cross-validate facet volumes with a non-cubic polytope.
    #[test]
    fn crosspolytope_facet_volume_sum() {
        let polytope = &known_polytopes::crosspolytope().polytope;
        let duals = polytope.dual_vertices_f64();
        let f = polytope.facet_count();

        let vol_from_facets: f64 = (0..f)
            .map(|fi| {
                facet_volume_from_incidence_f64(polytope.vertices_f64(), polytope.incidence(), fi)
                    .expect("valid known polytope fixture")
                    * (1.0 / duals[fi].norm())
            })
            .sum::<f64>()
            / 4.0;

        let polytope_volume =
            crate::test_lib::euclidean_volume_f64(polytope.vertices(), polytope.incidence());

        // Looser than hypercube (1e-6) because the crosspolytope has 16 facets
        // with non-axis-aligned normals, producing more triangulation error in
        // the f64 2-face decomposition.
        assert!(
            (vol_from_facets - polytope_volume).abs() / polytope_volume < 1e-4,
            "facet sum = {vol_from_facets}, polytope volume = {polytope_volume}"
        );
    }

    /// Wiring regression: the symplectic namespace exposes the Euclidean
    /// known-incidence helper directly.
    #[test]
    fn facet_api_matches_euclidean_known_incidence_helper() {
        for kp in known_polytopes::all_known() {
            let polytope = &kp.polytope;
            for facet_index in 0..polytope.facet_count() {
                let symplectic_value = facet_volume_and_centroid_from_incidence_f64(
                    polytope.vertices_f64(),
                    polytope.incidence(),
                    facet_index,
                )
                .expect("valid Polytope4D fixture");
                let euclidean = euclidean_polytopes::facet_volume_and_centroid_from_incidence_f64(
                    polytope.vertices_f64(),
                    polytope.incidence(),
                    facet_index,
                )
                .expect("valid Polytope4D fixture");
                let volume_error = (symplectic_value.0 - euclidean.0).abs();
                let centroid_error = (symplectic_value.1 - euclidean.1).norm();

                assert!(
                    volume_error <= 1.0e-10_f64.max(1.0e-10 * euclidean.0.abs()),
                    "{} facet {facet_index}: symplectic volume = {}, euclidean volume = {}",
                    kp.name,
                    symplectic_value.0,
                    euclidean.0
                );
                assert!(
                    centroid_error <= 1.0e-10,
                    "{} facet {facet_index}: symplectic centroid = {:?}, euclidean centroid = {:?}",
                    kp.name,
                    symplectic_value.1,
                    euclidean.1
                );
            }
        }
    }
}
