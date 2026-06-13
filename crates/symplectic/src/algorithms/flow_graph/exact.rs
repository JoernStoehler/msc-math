//! Public exact flow-graph facade.
//!
//! Internally, exact one-word tube resolution and exact exhaustive search are
//! separate concerns. This module preserves the historical
//! `flow_graph::exact::*` import path for callers.

pub use super::exact_search::*;
pub use super::exact_tube::*;
