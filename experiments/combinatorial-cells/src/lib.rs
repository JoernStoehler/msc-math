//! Shared helpers for combinatorial-cells experiments.
//!
//! Experiments studying the local geometry of combinatorial cells in
//! dual-vertex space: cell widths, boundary characterization, convexity,
//! gradient behavior at boundaries.

pub mod boundary_events;
pub mod instrumented_capacity;
pub mod records;

pub use boundary_events::{compute_step_bound_detailed, BoundaryEvent, EventType};
pub use instrumented_capacity::{ehz_capacity_instrumented, InstrumentedCapacitySummary};
pub use records::{construct_at_t, name_from_record, source_dataset_from_record};
