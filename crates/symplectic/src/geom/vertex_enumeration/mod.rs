//! Exact vertex enumeration for 4D polytopes over Q.
//!
//! Module architecture and math mapping:
//! - `enumerate`: end-to-end construction pipeline and C(F,4) vertex enumeration
//!   ([lem:vertex-enumeration], [prop:integer-cramer], [cor:prefilter-soundness]).
//! - `boundedness`: boundedness validation via positive spanning
//!   ([lem:positive-span], [lem:bounded-triples]).
//! - `irredundancy`: facet irredundancy checks from incident affine rank
//!   ([lem:irredundancy]).
//! - `exact_linalg`: exact determinant/solve/rank primitives used by all modules.

mod boundedness;
mod enumerate;
mod exact_linalg;
mod irredundancy;

type RationalPipelineOutput = (
    Vec<[num_rational::BigRational; 4]>,
    Vec<std::collections::BTreeSet<usize>>,
);

pub(super) fn construct_rational_pipeline(
    dual_vertices: &[[num_rational::BigRational; 4]],
) -> Result<RationalPipelineOutput, crate::geom::polytope::ConstructionError> {
    enumerate::construct_rational_pipeline(dual_vertices)
}

#[cfg(test)]
mod tests;
