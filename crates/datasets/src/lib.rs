//! Polytope dataset generation pipeline.
//!
//! Produces JSONL datasets of 4D polytopes with volume, EHZ capacity,
//! systolic ratio, and timing data.

pub mod acceptance_sweep;
pub mod dataset;
pub mod known_polytopes;
pub mod random;
pub mod validation;

#[cfg(test)]
mod sys_properties_test;

#[cfg(test)]
mod lib_test;
