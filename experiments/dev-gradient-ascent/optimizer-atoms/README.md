# Optimizer atom analysis

This packet explains complete optimizer trajectories produced by
[`../optimizer-runs`](../optimizer-runs/README.md). Its ordinary input is one
immutable trajectory dataset. It does not produce or rank complete optimizers.

`analyze_trace.py` uses only recorded on-trajectory proposals. It separates
prediction error, winner identification, realized distance, acceptance,
validated gain, trajectory phase, and measured component cost. These rows are
the primary population because seeded random perturbations previously
understated on-trajectory prediction errors.

A companion replay producer may add matched off-trajectory alternatives at
selected saved states: proposals from other step rules, several distances
along one direction, alternative candidate sets, and target-winner oracle
controls. The `optimizer-atoms` Rust executable implements the replay
surface: several distances along recorded proposal directions, action-window
selectors, the anchor winner, all transition-feasible anchor words, and target
winner/effective-all oracle controls. It compares nonlinear named-branch
reevaluation at the target with affine branch models built at the anchor.
Schema version 2 additionally records:

- the sigma selected by each predictor and its physical status at the target;
- why a future full-sys winner was absent from the anchor candidate universe;
- direct candidate-set reevaluation along later accepted trajectory states;
- retroactive checks of a newly observed winner at earlier trajectory states;
- ambient and symmetry-transverse normalized distances; and
- invalid-target geometry diagnostics.

Those extra KKT evaluations are written as a separate immutable dataset; they
are not silently added by the trace analyzer.

```bash
cargo run -p optimizer-atoms --release -- \
  --config /path/to/replay-config.json \
  --out /tmp/optimizer-atom-replay

uv run --script \
  experiments/dev-gradient-ascent/optimizer-atoms/diagnose_replay.py \
  --dataset /tmp/optimizer-atom-replay \
  --out /tmp/optimizer-atom-replay-evidence
```

The target-effective-all nonlinear row is the pipeline sanity check: after
target transition and beta predicates are applied, it must reproduce the full
target `sys`. `anchor_transition_feasible_all` is different and may miss words
that become transition-feasible only after moving.

```bash
uv run --script \
  experiments/dev-gradient-ascent/optimizer-atoms/analyze_trace.py \
  --dataset /path/to/optimizer-runs-output \
  --out /tmp/optimizer-atoms
```

The output is diagnostic evidence. Smaller pointwise error does not by itself
establish a better optimizer; complete performance remains in
`optimizer-comparison/`.

The current retained schema-v2 replay is
`artifacts/development-f10-16-replay/`. Its question-oriented report and
figures are in `artifacts/development-f10-16-replay-evidence/`. The report
makes the replay population, both distance definitions, candidate-omission
causes, affine-error causes, candidate lifetime, rollback opportunity,
gain-relative error, denominators, and claim boundary explicit:

```bash
uv run --script \
  experiments/dev-gradient-ascent/optimizer-atoms/diagnose_replay.py \
  --dataset experiments/dev-gradient-ascent/optimizer-atoms/artifacts/development-f10-16-replay \
  --out /tmp/predictor-replay-evidence
```

The retained output is
`artifacts/development-f10-16-replay-evidence/REPORT.md`.

`analyze_replay.py`, `explain_replay.py`, and
`artifacts/development-f10-16-replay-analysis/` are retained schema-v1
analysis surfaces. `diagnose_replay.py` is authoritative for schema-v2 replay
data; it imports plotting and tabulation helpers from `explain_replay.py`.
