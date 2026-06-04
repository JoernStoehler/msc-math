//! Shared ascent, row, and shard-output helpers for sys-landscape experiments.

mod cli;
mod compute;
mod rows;
mod runner;
mod shard_io;

#[cfg(test)]
mod tests;

pub use cli::{
    cache_path_for, parse_ascent_args, smoke_output_path, trace_path_for, AscentArgs,
    AscentOutputPaths,
};
pub use compute::{
    apply_dual_step, ascent_direction, compute_active_sys_state, compute_capacity_result,
    compute_sys, compute_sys_from_capacity, dual_vertices_rational_strings,
    orbit_scalars_from_result, rational_vec4_to_strings, ActiveSysState, AscentMode,
};
pub use rows::{SeedResult, SummaryRow, TraceRow};
pub use runner::run_parallel_seeds;
pub use shard_io::{
    finalize_ascent_output, load_completed_names, open_ascent_writers, write_seed_result,
    AscentWriters,
};
