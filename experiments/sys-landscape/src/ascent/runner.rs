use super::cli::AscentArgs;
use super::rows::SeedResult;
use super::shard_io::{write_seed_result, AscentWriters};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Parallel seed loop with per-seed RNG streams.
///
/// Invariants:
/// - Seed i is identified by global index; the closure MUST use `seed_i`
///   (= `args.seed.wrapping_add(i as u64)`) to construct its RNG and do all
///   per-seed work. Precondition: `args.seed + args.n_start + args.n` must not
///   overflow u64; `wrapping_add` only aliases seed streams across the global
///   batch at `seed ≈ u64::MAX`, far above any realistic ascent run.
///   The per-seed JSON payloads for index i are byte-reproducible
///   regardless of which thread processes it. **File-level byte reproducibility
///   requires the caller to invoke `finalize_ascent_output` after this function
///   returns** — rayon scheduling determines the append order within both
///   output files, and `finalize_ascent_output` is what canonicalizes row order.
/// - `completed` is checked before calling `process`; resume semantics
///   therefore hold across crashes (see `write_result` for the on-disk
///   ordering invariant that backs this).
/// - Writers are locked only during append (ms); contention is negligible
///   against per-seed ascent cost (~seconds).
/// - Seed name format is `"{prefix}_{i}"`, matching the historical naming
///   used by both ascent binaries before the refactor.
///
/// Lock acquisition order inside the rayon closure is strictly:
/// `db` (outside `write_result`, in the per-experiment closure) → `trace`
/// (inside `write_result`) → `summary` (inside `write_result`) → `best`
/// (here, after `write_result` returns). Each lock is released before the
/// next is acquired — no nesting — so two threads cannot form a deadlock
/// cycle regardless of which seed each is processing.
///
/// Panic propagation: a panic inside `process` (or inside `write_result`)
/// poisons any mutex held across the panic point. Subsequent seeds that
/// call `.lock().expect("... poisoned")` will then fan the panic out and
/// crash the binary. This is **intended**: on LICCA, a panicking seed
/// crashes the slurm job, which requeues, and `load_completed_names`
/// resumes by skipping seeds already written to the summary file. Do not
/// convert these `.expect` calls to recover-and-continue without also
/// updating the resume story.
pub fn run_parallel_seeds<F>(
    args: &AscentArgs,
    completed: &HashSet<String>,
    writers: &AscentWriters,
    best: &Arc<Mutex<(f64, String)>>,
    process: F,
) where
    F: Fn(usize, u64) -> Option<SeedResult> + Send + Sync,
{
    let end = args.n_start + args.n;
    (args.n_start..end).into_par_iter().for_each(|i| {
        let name = format!("{}_{}", args.prefix, i);
        if completed.contains(&name) {
            return;
        }
        let seed_i = args.seed.wrapping_add(i as u64);
        if let Some(result) = process(i, seed_i) {
            write_seed_result(&result, writers);
            let mut b = best.lock().expect("best-tracker mutex poisoned");
            // Strict `>`: on ties, the first-to-arrive winner is kept. With rayon
            // scheduling the arrival order is non-deterministic, so the reported
            // "best" name can vary between runs when multiple seeds hit the same
            // `final_sys`. Cosmetic only — `best` is printed to stdout and never
            // written to JSONL. The canonicalized summary file (sorted by name
            // in `finalize_ascent_output`) remains byte-reproducible.
            if result.summary.final_sys > b.0 {
                *b = (result.summary.final_sys, result.summary.name.clone());
            }
        }
    });
}
