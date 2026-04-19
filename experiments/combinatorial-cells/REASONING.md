# Combinatorial Cells Reasoning

## Surface map
`experiments/combinatorial-cells` owns all local reasoning about six binary experiments and their shared data graph:
- `cell-widths` for per-facet boundary-distance probes.
- `boundary-characterization` for anatomy/crossing/gradient at first boundaries.
- `convexity` for midpoint-type stability.
- `multiple-crossings` for iterative boundary traversal.
- `gradient-discontinuity` for how systolic gradient changes at boundaries.
- `omega-hypothesis` for the near-Lagrangian ridge hypothesis.

All shared math helpers are in `src/lib.rs` (`compute_step_bound_detailed`, `compute_sys_gradient_a`, `BoundaryEvent`, etc.), and all binaries are in `experiments/combinatorial-cells/Cargo.toml`.

## Current interpretation from visible artifacts
The topic is now operating on a larger owned cache than the historical 140-polytope snapshot in design notes:
- `polytopes.jsonl`: 953 rows (`source` shows 950 random + 3 known polytopes).
- `boundary-characterization/combinatorial-boundaries-anatomy.jsonl`: 6,671 rows across 953 polytopes.
- `boundary-characterization/combinatorial-boundaries-crossing.jsonl`: 5,633 rows (`construction_ok_after` succeeds for 5,631).
- `multiple-crossings/combinatorial-boundaries-sweep.jsonl`: 3,812 rows.
- `cell-widths/combinatorial-boundaries-profiling.jsonl`: 66,230 rows.
- `convexity/combinatorial-boundaries-convexity.jsonl`: 19,060 rows.
- `omega-hypothesis/omega-obstacle.jsonl`: 953 rows.

This suggests the topic is currently in a “full cache refresh + extended analysis” state, not an initial exploratory pass. Binaries still retain hard-coded experimental defaults (`MAX_FACET_COUNT=10`, `SEED=42`, `MAX_STEP_SIZE=100`) so reruns are designed to be deterministic.

## What the artifacts imply
- `cell-omega` now bootstraps the canonical local cache and is still the only binary that writes `polytopes.jsonl` in this package.
- `cell-widths` and `boundary-characterization` both depend on `polytopes.jsonl`; downstream analyses (`gradient-discontinuity`) depend on those outputs.
- `multiple-crossings` repeatedly consumes `compute_step_bound_detailed` results, so its failure profile is a direct test of numerical robustness of the boundary crossing kernel.
- `convexity` uses the same local perturbation strategy as `cell-widths` before midpoint checks, so differences between “incidence-preserving” and “transition-preserving” success rates are methodological, not dataset-driven.
- `gradient-discontinuity` is interpretive: it interprets gradient rows generated upstream by `boundary-characterization`.
- The old research notes report the hypothesis-as-exploration period (mostly March 2026), while current JSONL sizes show ongoing recomputation and likely additional random draws, so decisions that depended on older counts should be revalidated with fresh commands before being used as quantitative claims.

## Interpretation anchors tied to formal text
- Shared helpers claim correspondence to `lem:step-bound-incidence`, `lem:step-bound-omega`, and `lem:sys-gradient-a` in code comments.
- Cross-checks against conjecture-facing text are in `formal/combinatorial-cells/` and should be updated there when experimental interpretation changes (not in this migration).
