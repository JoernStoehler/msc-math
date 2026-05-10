//! Shared helpers for combinatorial-cells experiments.
//!
//! Experiments studying the local geometry of combinatorial cells in
//! dual-vertex space: cell widths, boundary characterization, convexity,
//! gradient behavior at boundaries.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

pub mod boundary_events;
pub mod instrumented_capacity;
pub mod records;

pub use boundary_events::{compute_step_bound_detailed, BoundaryEvent, EventType};
pub use instrumented_capacity::{ehz_capacity_instrumented, InstrumentedCapacitySummary};
pub use records::{construct_at_t, name_from_record, source_dataset_from_record};

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}
