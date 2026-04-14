//! Exact vertex enumeration pipeline for 4D polytopes over Q.
//!
//! Stage order is explicit and fixed:
//! input normalization -> prefilter -> exact candidate enumeration
//! -> irredundancy/boundedness checks -> output assembly.

mod boundedness;
mod enumerate;
mod linear_algebra;
mod pipeline;
mod prefilter;

pub(in crate::geom) use pipeline::construct_rational_pipeline;

#[cfg(test)]
mod tests;
