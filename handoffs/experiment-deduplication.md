# Task: Extract shared experiment code to library

## Context

Four experiments (gradient-descent, sys-optimization, hko-neighborhood, omega-obstacle) each contain 930-2347 LOC copies of an instrumented KKT solver and derivative computation code. This duplication makes maintenance error-prone — a bug fix in one copy doesn't propagate to others. The code is stable and mature enough for library extraction.

## Scope

1. **Extract instrumented KKT solver** to a shared module (either `crates/src/kkt_instrumented.rs` or a submodule of `crates/src/kkt/`):
   - `build_kkt_system()` — constructs KKT system from (S, σ)
   - `solve_kkt_svd_path()` — SVD-based linear solver
   - `find_positive_beta_nd()` — finds admissible β from KKT solution
   - `ehz_capacity_instrumented()` — extracts orbit metadata (valid orb count, best action, gaps)

2. **Extract derivative computation** to `crates/src/derivatives.rs` or similar:
   - `compute_capacity_derivatives_analytical()` / `_fd()` — height and normal derivatives of c_EHZ
   - `compute_volume_derivatives_analytical()` / `_normal()` — height and normal derivatives of volume
   - The finite-difference variants are cross-checks, not alternatives — keep both

3. **Update experiment binaries** to use shared modules instead of local copies. Verify outputs are unchanged by running each experiment and comparing JSONL output.

4. **Do NOT change** the mathematical formulas, thresholds, or algorithm logic during extraction. This is a pure refactor — same inputs, same outputs.

## Out of scope

- Changing eigenvalue thresholds or KKT solver logic (that's a separate task)
- Changing the (n, h) parameterization to dual vertices (that's a separate task)
- Refactoring adjacency/permutation logic (lower priority, 6+ copies but lightweight code)
- Writing or updating .tex writeups
- Changing any experiment's JSONL output format

## Key files

The four source files with duplicated code:
- `/workspaces/msc-math/experiments/gradient-descent/kkt_instrumented.rs` (930 LOC — the cleanest copy, already a separate module)
- `/workspaces/msc-math/experiments/sys-optimization/sys_optimization.rs` (2347 LOC, KKT code embedded)
- `/workspaces/msc-math/experiments/hko-neighborhood/hko_neighborhood.rs` (2118 LOC, KKT code embedded)
- `/workspaces/msc-math/experiments/omega-obstacle/omega_obstacle.rs` (1028 LOC, KKT code embedded)

The library KKT solver they're based on:
- `/workspaces/msc-math/crates/src/kkt.rs` (the non-instrumented version)

## Prior findings

- `gradient-descent/kkt_instrumented.rs` is the cleanest starting point — it's already a separate module with a public API
- The four copies differ mainly in: parameter naming, which orbits are tracked (best vs all near-optimal), and how results are structured. The core math (KKT construction, SVD solver, derivative formulas) is identical.
- Each copy uses a local `SVD_CONDITION_TAU` matching the library's `EIGEN_CONDITION_TAU = 1e-3`
- Derivative code differences: sys-optimization and hko-neighborhood track near-optimal orbits (action gap < threshold), gradient-descent tracks only the best orbit

## Success criteria

- `cargo test --lib` passes (library tests still work)
- `cargo build` succeeds for all 4 experiment binaries
- Each experiment binary produces identical JSONL output to before extraction (run each and diff)
- No duplicated KKT solver or derivative code remains in experiment files
- The shared module has doc comments explaining its public API
