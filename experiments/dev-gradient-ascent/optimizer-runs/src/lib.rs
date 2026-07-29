//! Shared machinery for statistically comparing black-box `sys(a)` optimizers.
//!
//! The evaluator owns every expensive objective call. Optimizers only exchange
//! proposals and observations through the ask/tell interface, which keeps
//! accounting and traces comparable across sequential and population methods.

pub mod algorithm;
pub mod algorithms;
pub mod branch_model;
pub mod dataset;
pub mod evaluator;
pub mod manifest;
pub mod output;
pub mod quotient;
pub mod runner;
pub mod schedule;
pub mod schema;

pub use evaluator::{reconstruct_geometry_and_volume, Evaluation, Evaluator, EvaluatorConfig};
pub use manifest::{
    load_and_resolve, AlgorithmSpec, CandidateAcceptancePolicy, Manifest, ResolvedPlan,
};
pub use runner::run_plan;
