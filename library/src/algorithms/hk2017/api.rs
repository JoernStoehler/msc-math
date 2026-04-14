//! Public HK2017 entry points and result container.

use crate::algorithms::capacity_accumulator::CapacityResult;
use crate::geom::polytope::Polytope4D;

use super::enumeration::{enumerate_pruned, enumerate_unpruned};

/// Result of the EHZ capacity computation.
#[derive(Clone, Debug)]
pub struct EhzResult {
    /// Core capacity result from the shared accumulator.
    pub result: CapacityResult,
    /// Facet indices S participating in the optimal orbit (unordered).
    pub best_subset: Vec<usize>,
}

/// Compute c_EHZ(K) for a convex polytope K in R^4 without pruning.
pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult> {
    enumerate_unpruned(polytope).map(|out| EhzResult {
        result: out.result,
        best_subset: out.best_subset,
    })
}

/// Compute c_EHZ(K) for a convex polytope K in R^4 with adjacency pruning.
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    enumerate_pruned(polytope).map(|out| EhzResult {
        result: out.result,
        best_subset: out.best_subset,
    })
}
