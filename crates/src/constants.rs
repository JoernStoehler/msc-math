//! Shared numerical tolerance constants.
//!
//! All tolerances that need to be consistent across modules are defined here.
//! Module-local tolerances (used in only one place) stay in their respective modules.

/// Vertex-on-facet incidence tolerance: |n·v - h| < EPS_FACET_INCIDENCE.
///
/// Used by: skeleton, volume (deprecated), reeb_trajectory, hk2017, billiard.
/// Previously defined under 5 different names (EPS_FACET_INCIDENCE, EPS_FEASIBILITY,
/// EPS_ON_FACET, EPS_INCIDENCE) — all 1e-8, all checking the same geometric property.
pub const EPS_FACET_INCIDENCE: f64 = 1e-8;
