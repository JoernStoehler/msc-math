# Witness-Search Plan

Scope:
- implement the next extension after current gradient/ascent baselines: reusable witness oracles and witness-guided continuation.

Program owner: this package.

Execution state mapping:
- reference: `research/sys-landscape/design/witness-search-program.md`
- this is the successor to `variable-f-ascent/` and the HKO continuation baseline.

Phase 1 — Witness oracle instrumentation
- add explicit witness-bundle outputs in `experiments/sys-landscape/gradient-ascent-general` and `gradient-ascent-products` loops (top-`m`, within-gap set, incumbent carry-over).
- add diagnostics required for exact-call accounting (runtime, rejected-by-upper-bound events, incumbent hit rates).
- keep all outputs in the existing `gradient-ascent-*.jsonl` schema plus explicit oracle sections to avoid schema churn.

Phase 2 — Reuse and safe prefilter
- add benchmark harness around cached witness sets in `variable-f-ascent/` lineups:
  - minimizer-only baseline
  - top-`m` witness sets
  - within-gap witness sets
  - parent-child continuation cache.
- primary acceptance outputs:
  - hit rate for cached minimizers
  - safe reject rate from an upper bound `U_A(K) < 1`
  - exact-call reduction factor.

Phase 3 — Reduced model ascent
- use witness-reduced cheap model in the same seed set as current baselines:
  - smooth surrogate on top witness set (soft-min / log-sum-exp)
  - periodic exact-check fallback.
- report exact-call count and best `sys` against current exact-every-step baselines.

Phase 4 — Witness-guided continuation
- replace random facet-add continuation in `cut-and-ascent/`-style style with witness-guided facet splitting.
- use witness lifting from parent to child `F+1` problems.
- direct comparison target: `variable-f-ascent/` and `experiments/hko-local-maximum/cut-and-ascent/`.

Phase 5 — Structured families + shutdown policy
- only after phases 1–4 validate, add a symmetry-constrained low-dimensional search lane.
- stop early if this package fails to produce reusable oracle/pruning signal with clear reproducible artifacts; do not keep opening new method lines from this task.

Acceptance targets:
- one artifactized witness file format for all ascent runs.
- one benchmark table comparing exact-call reduction and best `sys` against current phase baselines.
- explicit "no-new-sys>1" and "new-positive-signal" outcomes documented in matching task notes.
