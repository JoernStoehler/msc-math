//! Public HK2017 orbit-collector entry points.

use crate::algorithms::{
    OrbitGuaranteeMode,
    OrbitSearchError,
    OrbitSearchResult,
    OrbitSolveBackend,
};
use crate::geom::polytope::Polytope4D;

use super::enumeration::{collect_pruned, collect_unpruned};

/// Collect near-minimum HK2017 orbits with adjacency pruning.
pub fn hk2017_minimum_orbits(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    collect_pruned(polytope, gap, mode, backend)
}

/// Collect near-minimum HK2017 orbits without adjacency pruning.
pub fn hk2017_minimum_orbits_unpruned(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    collect_unpruned(polytope, gap, mode, backend)
}
