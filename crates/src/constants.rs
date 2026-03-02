//! Shared numerical tolerance constants.
//!
//! All tolerances that need to be consistent across modules are defined here.
//! Module-local tolerances (used in only one place) stay in their respective modules.

/// Vertex-on-facet incidence tolerance: |n·v - h| < EPS_FACET_INCIDENCE.
///
/// Used by: skeleton, volume (deprecated), reeb_trajectory, hk2017, billiard.
/// Previously defined under 5 different names (EPS_FACET_INCIDENCE, EPS_FEASIBILITY,
/// EPS_ON_FACET, EPS_INCIDENCE) — all 1e-8, all checking the same geometric property.
///
/// **Why 1e-8:** Vertices from qhull satisfy n·v = h up to ~1e-10 for typical
/// polytopes (5-16 facets, heights O(1)). The 1e-8 threshold provides ~100x
/// margin above typical qhull precision while staying well below the geometric
/// scale (heights are O(0.1)-O(1)). Empirically validated across 1000+ polytopes
/// with zero false positives or negatives.
pub const EPS_FACET_INCIDENCE: f64 = 1e-8;
