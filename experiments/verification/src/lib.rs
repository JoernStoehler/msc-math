//! Shared helpers for verification experiments.
//!
//! Purpose: keep target-pool selection and shared run plumbing consistent
//! across the minimum-set and orbit-recovery validation binaries.

pub mod io;
pub mod target_pool;

pub use io::{
    create_jsonl_writer, mode_output_path, parse_run_mode, run_mode_label, write_json_line,
    RunMode, RunModeArgError,
};
pub use target_pool::{
    build_target_pool, target_map, Target, ACTION_TOL, EXCLUDED_KNOWN_NAMES, GEOMETRY_TOL,
    MINIMUM_ACTION_GAP_TOL, SCALAR_TOL, SMOKE_TARGET_NAMES,
};
