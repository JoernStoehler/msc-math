# Combinatorial Cells Research Note

## Scope
- Coordinates local experiments under `experiments/combinatorial-cells` that probe boundary events and transitions for combinatorial-cell behavior across four-dimensional polytopes.
- The topic scope spans six binaries and shared helpers in `crates/`-like layout:
  - `cell-omega`
  - `cell-widths`
  - `cell-boundary-characterization`
  - `cell-multiple-crossings`
  - `cell-convexity`
  - `cell-gradient-discontinuity`
- The note is for research interpretation and cross-refs; raw artifacts remain owned by their experiment directories under `experiments/`.

## Current State
- Current cache scale is substantially larger than historical design notes: `polytopes.jsonl` has 953 rows (950 random + 3 known polytopes).
- Downstream artifact row counts are in the same snapshot:
  - `boundary-characterization/anatomy`: 6,671 rows
  - `boundary-characterization/crossing`: 5,633 rows (`construction_ok_after` succeeds for 5,631)
  - `multiple-crossings/sweep`: 3,812 rows
  - `cell-widths/profiling`: 66,230 rows
  - `convexity/convexity`: 19,060 rows
  - `omega-hypothesis/obstacle`: 953 rows
- Binaries still use deterministic defaults such as `MAX_FACET_COUNT=10`, `SEED=42`, `MAX_STEP_SIZE=100.0`; reruns are therefore comparable and intentionally bounded.
- `cell-omega` still bootstrap-writes `polytopes.jsonl` and remains the upstream dependency for `cell-widths` and `cell-boundary-characterization`; `gradient-discontinuity` reads from boundary outputs and is interpretive by design.
- `compute_step_bound_detailed` is now the shared core for crossing logic across multiple binaries, with downstream behavior dominated by boundary-crossing robustness.

## Evidence And Interpretation
- Current counts indicate an active recomputation phase, not only the earlier exploratory period.
- The shared helper set (`compute_step_bound_detailed`, `compute_sys_gradient_a`, `BoundaryEvent`) in package `src/lib.rs` directly maps to the same conceptual roles as:
  - `[lem:step-bound-incidence]`
  - `[lem:step-bound-omega]`
  - `[lem:sys-gradient-a]`
- `multiple-crossings` stress-tests the stepping kernel by repeatedly consuming `compute_step_bound_detailed`; repeated construction failures are therefore strong evidence about numerical robustness.
- `convexity` uses the same perturbation strategy as `cell-widths`, so contrast between incidence-only and transition-aware checks should be interpreted as a methodological difference, not an artifact drift difference.
- The near-Lagrangian-ridge hypothesis is retained as falsified negative evidence from existing runs.
- Older counts and conclusions from early notes should be revalidated against current artifacts before reused as numeric claims.

## Decisions
- Retained experiment boundaries inside `experiments/combinatorial-cells`; exploration code is kept local during this migration period.
- Keep deterministic runtime controls (`SEED=42`, fixed facet-direction limits, `MAX_FACET_COUNT=10`) to preserve comparability of new outputs.
- Treat `polytopes.jsonl` as the canonical local cache expected by all binaries in this package.
- Preserve `MAX_STEP_SIZE=100.0` and explicit `Unbounded` classification to avoid false boundary hits in histograms.
- Keep conservative stepping tolerances (`EPS_NUMERICAL_ZERO = 1e-15`) with explicit `EPS_FLOOR` behavior.
- Keep transition-matrix checks in convexity (`midpoint_same_transitions`) as the primary structural criterion.
- Keep old hypothesis file as a negative baseline until formalized evidence replaces it.
- Rejected routes and constraints remain in force:
  - no global convexity assumption in dual-vertex space
  - no continuity-from-sampling assumption for first-boundary `sys`
  - no single-boundary model for multi-step behavior
  - no monotonicity assumption for repeated `sys` improvements

## History
- Legacy `README.md` and `RESEARCH.md` were routed as provisional guidance and became stale as caches grew.
- The three separate notes (`REASONING`, `DECISIONS`, `NEXT-STEPS`) captured:
  - operational state and artifact interpretation,
  - non-obvious design choices and explicit rejections,
  - blocker list and follow-up commands.
- This migration condenses those claims into one research-facing, continuously readable note without moving experiment artifacts or changing package behavior.

## Next Steps
- Refresh the quantitative baseline from current artifacts and compare against historical 140-polytope values.
- Decide whether sweep robustness is safe before increasing run scale for larger `MAX_FACET_COUNT`.
- Close the two explicit blockers: formal continuity of `sys` at boundaries, and systematic handling of `multiple-crossings` failures.
- Maintain evidence-level checkpoints from full reruns:
  - `cargo run -p exp-combinatorial-cells --release --bin cell-omega`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-widths`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-boundary-characterization`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-convexity`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-multiple-crossings`
- Stop when refreshed JSONL + plots are archived and interpreted against the old 140-polytope snapshot, or when both blockers are formally resolved.
