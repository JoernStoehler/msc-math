//! Flow-graph capacity algorithm work surface.
//!
//! The local README is the status and contract surface for this unfinished but
//! thesis-facing CH2021-style algorithm packet.

pub mod exact_search;
pub mod exact_tube;
mod f64_tube_search;
mod words;

pub use f64_tube_search::*;
pub use words::*;
