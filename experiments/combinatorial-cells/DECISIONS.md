# Combinatorial Cells Decisions

## Non-obvious retained choices
- Keep experiment package boundaries local to `experiments/combinatorial-cells`; do not route these explorations back to `research/` once migrated.
- Preserve deterministic generation and run configuration (`SEED=42`, fixed `N_FACET_DIRS`, fixed `MAX_FACET_COUNT=10`) to keep JSONL artifacts comparable across reruns.
- Treat `polytopes.jsonl` as the canonical local artifact cache; every binary assumes this cache exists in package root before rerun.
- Keep `MAX_STEP_SIZE=100.0` and classify `Unbounded` explicitly; this preserves comparability of event histograms across runs and avoids false boundary hits.
- Keep epsilon choices conservative and low (`EPS_NUMERICAL_ZERO = 1e-15`) with explicit `EPS_FLOOR` for stepping past boundaries; these were selected to avoid classifying noise as actual crossings.
- Keep transition-matrix checks in convexity (`midpoint_same_transitions`) because they directly encode feasible HK2017 orbit transitions, not just incidence structure.
- Keep old design hypothesis file as negative evidence: the near-Lagrangian-ridge obstacle hypothesis did not hold under observed data and was retained as a falsified baseline.

## Rejected routes and constraints
- Do not assume convexity of combinatorial-type cells in dual-vertex space: midpoint checks show frequent transition-matrix changes.
- Do not assume first-boundary `sys` continuity from sampling alone: continuity is strong numerically but not yet fully formal in the visible artifacts.
- Do not assume a single-boundary model can capture multi-step behavior: multiple-crossings shows repeated boundary encounters and explicit sweep failures.
- Do not assume `sys` improvements are monotone along repeated gradient steps: the sweep analysis documents non-monotonic trajectories.

## Why these notes replaced legacy docs
- The old `README.md` and `RESEARCH.md` were provisional routing documents; they duplicated static map/commands and became stale as outputs grew.
- Historical design notes are superseded by local operational docs; readers should use `REASONING.md`, `DECISIONS.md`, and `NEXT-STEPS.md` for current claims and next steps.
