//! Shared helpers for sys-landscape experiments.
//!
//! Experiments studying the systolic ratio as a global function on polytope
//! space: random-sample, random-product-sample, gradient-ascent-general,
//! gradient-ascent-products, rotated-regular-products, rejection-calibration.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

pub mod ascent;
pub mod datasets;
pub mod step_bound;

pub use ascent::{
    apply_dual_step, ascent_direction, compute_active_sys_state, compute_capacity_result,
    compute_sys, compute_sys_from_capacity, dual_vertices_rational_strings, finalize_ascent_output,
    load_completed_names, open_ascent_writers, orbit_scalars_from_result, parse_ascent_args,
    rational_vec4_to_strings, run_parallel_seeds, smoke_output_path, trace_path_for, write_result,
    ActiveSysState, AscentArgs, AscentMode, SeedResult, SummaryRow, TraceRow,
};
pub use datasets::{
    continuation_cache_path, experiment_path, package_root, raw_dataset_cache_path,
    raw_dataset_path, raw_dataset_trace_path, raw_root, shared_family_cache_path,
    CONTINUATION_EXPERIMENT_DIR, GRADIENT_ASCENT_GENERAL_DIR,
};
pub use step_bound::{
    compute_step_bound, compute_step_bound_detailed, BoundaryEvent, EventType, MAX_STEP_SIZE,
};

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}
