//! Instrumentable copies of the selected QP implementations.
//!
//! These concrete files are the copy-editable starting point for experiments.
//! Their headers name the corresponding production files and intentional
//! instrumentation differences. Correspondence tests prevent silent semantic
//! drift.

pub mod general;
pub mod product;
