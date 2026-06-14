# Combinatorial Cells

This package owns combinatorial-boundary exploration outputs and local
interpretation for boundary events and transitions across four-dimensional
polytopes.

## Rust Command Contract

- `cell-omega` is the upstream producer. It reads and updates the package-local
  canonical cache `polytopes.jsonl` and writes
  `omega-hypothesis/omega-obstacle.jsonl`.
- `cell-widths`, `cell-boundary-characterization`, `cell-convexity`, and
  `cell-multiple-crossings` read `polytopes.jsonl` and write tracked evidence
  JSONL files beside each binary.
- `gradient-discontinuity/analyze.py` is an analyzer, not a Rust producer
  binary. It reads boundary-characterization and cell-width outputs and writes
  interpretive figures under `gradient-discontinuity/`.
- These binaries do not currently have smoke modes. Do not run them as quick
  command checks unless intentionally refreshing the tracked artifacts.
- For compile-only checks, use `cargo test -p exp-combinatorial-cells
  --all-targets` or `cargo clippy -p exp-combinatorial-cells --all-targets`.

## Current Artifact Snapshot

- `polytopes.jsonl`: 953 rows (950 random + 3 known polytopes).
- `boundary-characterization/combinatorial-boundaries-anatomy.jsonl`: 6,671
  rows.
- `boundary-characterization/combinatorial-boundaries-crossing.jsonl`: 5,633
  rows; `construction_ok_after` succeeds for 5,631.
- `multiple-crossings/combinatorial-boundaries-sweep.jsonl`: 3,812 rows.
- `cell-widths/combinatorial-boundaries-profiling.jsonl`: 66,230 rows.
- `convexity/combinatorial-boundaries-convexity.jsonl`: 19,060 rows.
- `omega-hypothesis/omega-obstacle.jsonl`: 953 rows.

Older counts and conclusions from early notes must be revalidated against
current artifacts before reuse as numeric claims.

## Interpretation And Constraints

- `cell-omega` bootstrap-writes `polytopes.jsonl` and remains the upstream
  dependency for `cell-widths` and `cell-boundary-characterization`.
- `compute_step_bound_detailed` is the shared core for crossing logic across
  multiple binaries. Downstream behavior is dominated by boundary-crossing
  robustness.
- `multiple-crossings` stress-tests the stepping kernel by repeatedly consuming
  `compute_step_bound_detailed`; repeated construction failures are evidence
  about numerical robustness, not a separate theorem.
- `convexity` uses the same perturbation strategy as `cell-widths`, so contrast
  between incidence-only and transition-aware checks is a methodological
  difference, not artifact drift by itself.
- The near-Lagrangian-ridge hypothesis is retained as falsified negative
  evidence from existing runs.
- Do not assume global convexity in dual-vertex space.
- Do not infer continuity of first-boundary `sys` from samples alone.
- Do not assume a single-boundary model for multi-step behavior.
- Do not assume monotonicity for repeated `sys` improvements.

## Revive Conditions

Treat this package as inactive unless a thesis section or method row needs it.
If revived:

- refresh the quantitative baseline from current artifacts;
- compare against the historical 140-polytope values only after rechecking the
  current artifact schema;
- decide whether sweep robustness is safe before increasing run scale for
  larger `MAX_FACET_COUNT`;
- close the two blockers before using this as strong thesis evidence: formal
  continuity of `sys` at boundaries, and systematic handling of
  `multiple-crossings` failures.
