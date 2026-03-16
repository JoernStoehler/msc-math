//! `symplectic` crate: EHZ capacity algorithms for 4D polytopes.
//!
//! Modules:
//! - `geom` — polytope types, geometry primitives, symplectic form
//! - `kkt` — KKT solver for constrained quadratic optimization
//! - `algorithms` — EHZ capacity algorithms (hk2017, billiard, tube)
//! - `constants` — shared numerical tolerance constants
//! - `dataset` — JSONL row types for polytope dataset serialization
//! - `random` — random polytope generation via rejection sampling

pub mod geom;
pub mod kkt;
pub mod algorithms;
pub mod constants;
pub mod dataset;
pub mod random;

#[cfg(test)]
#[path = "dataset_test.rs"]
mod dataset_test;

#[cfg(test)]
#[path = "random_test.rs"]
mod random_test;
