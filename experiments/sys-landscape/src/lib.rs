//! Shared helpers for sys-landscape experiments.
//!
//! Experiments studying the systolic ratio as a global function on polytope
//! space: random-sample, random-product-sample, gradient-ascent-general,
//! gradient-ascent-products, rotated-regular-products, rejection-calibration.

pub mod ascent;
pub mod datasets;

pub use ascent::{
    apply_dual_step, ascent_direction, compute_active_sys_state, compute_capacity_result,
    compute_step_bound, compute_step_bound_detailed, compute_sys, compute_sys_from_capacity,
    dual_vertices_rational_strings, finalize_ascent_output, load_completed_names,
    open_ascent_writers, orbit_scalars_from_result, parse_ascent_args, rational_vec4_to_strings,
    run_parallel_seeds, smoke_output_path, trace_path_for, write_result, ActiveSysState,
    AscentArgs, AscentMode, BoundaryEvent, EventType, SeedResult, SummaryRow, TraceRow,
    MAX_STEP_SIZE,
};
pub use datasets::{
    continuation_cache_path, experiment_path, package_root, raw_dataset_cache_path,
    raw_dataset_path, raw_dataset_trace_path, raw_root, shared_family_cache_path,
    CONTINUATION_EXPERIMENT_DIR, GRADIENT_ASCENT_GENERAL_DIR,
};
