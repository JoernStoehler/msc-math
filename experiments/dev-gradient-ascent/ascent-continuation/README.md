# Repeated ascent continuation

This diagnostic asks whether a frozen optimizer endpoint still lies at the
start of a sustained improving path. The full four-state command is an
exploratory run, not a smoke test: its first retained execution took about nine
minutes before redundant model construction was removed.

At every accepted state it independently rebuilds two finite-step branch
models (relative candidate windows `0.1` and `1.0`) and the gradient of the
currently minimizing branch. It proposes one move from each model at five
relative distances, fully recomputes `sys` at every proposal, accepts the best
improvement, and repeats. If all model moves fail, it checks every signed
coordinate of the local symmetry-transverse basis at relative distance
`1e-5`.

The output records every tested proposal in `candidates.jsonl`, accepted moves
in `steps.jsonl`, and per-state path length, gain, final slope, evaluator count,
and stop reason in `summary.json`. It also flushes each completed state to
`state-summaries.jsonl`, so an interrupted panel retains complete state-level
results even though it has no final `summary.json`.

The development loop is a one-state, one-step debug run with two distances:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode debug --out-dir /tmp/ascent-continuation-debug
```

The four-state, ten-step scientific run is explicit:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode full --out-dir /tmp/ascent-continuation-full
```

The four fixed states are one endpoint known to improve under a larger budget,
the later version of that endpoint with a remaining positive basis slope, one
endpoint whose basis slopes remain negative across three radii, and HKO as a
local-maximum control.

A stop is not a local-maximality result: the finite-step models and one signed
basis only cover a restricted direction set.

## Calibration against the known HKO maximum

The HKO quotient-ray packet supplies controlled states whose improving target
is known: HKO is a proved local maximum, and the retained points are explicit
perturbations of it. This separates “the diagnostic found some improvement”
from “it recovered a substantial fraction of a known gap and distance.”

`make_hko_calibration.py` freezes four development directions at distances
`1e-4`, `1e-3`, `1e-2`, and `1e-1`. The directions are one slice-basis
sentinel, the projected rotated-pentagon tangent sentinel, and two random
rays. The remaining random rays are not exposed and can later provide a
held-out miss-rate estimate after the proposal rules are frozen.

Generate the short debug input and run it:

```bash
uv run --script experiments/dev-gradient-ascent/ascent-continuation/make_hko_calibration.py \
  experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-quotient-ray/evaluations.jsonl \
  /tmp/hko-calibration-debug.json --profile debug
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode debug --states-json /tmp/hko-calibration-debug.json \
  --out-dir /tmp/hko-calibration-debug
```

The panel run uses `--profile panel` and the same executable. Each perturbation
gets one accepted move, proposed at distances equal to `1`, `0.3`, and `0.1`
times its known return distance. This directly measures one-step recovery
without paying to rebuild large branch windows after a state is already
numerically close to HKO. Residual states, rather than the whole panel, are the
appropriate population for any multi-step follow-up. Post-process it with
`analyze_hko_calibration.py`.

The predeclared failure readings are retained in the generated input packet.
In particular, an accepted HKO move is a false-positive failure; a large known
gap with no accepted move is a directly observed false negative; and strong
direction dependence prevents treating a stop as globally calibrated evidence
of proximity.

The retained development result is
`artifacts/hko-one-step-development-panel-20260729/analysis/REPORT.md`. All 16
perturbations produced a validated improving move, recovering
`0.934`–`1.000` of the known `sys` gap. The `0.1` and `1.0` candidate windows
were empirically indistinguishable on this panel, while the current
minimizing-branch gradient recovered only `-0.221`–`0.414` of the gap.
Branch-extension enumeration dominated runtime. These are development-panel
results; the unexposed random rays are reserved for a held-out miss-rate test.

## Highest retained optimizer endpoints

`make_optimizer_endpoint_packet.py` selects completed optimizer endpoints by
recorded rank for late-stage continuation. The retained outcome-selected check
under `artifacts/top8-tuning-endpoints-one-step-20260729/` applies one
continuation step to the eight highest four-anchor endpoints from the
128-start tuning dataset.

Five of eight endpoints improved, but none crossed `sys = 1`. The highest
endpoint, at `0.9999624776406894`, had no improving max--min branch-model,
current-winning-branch gradient, or signed-basis move. At that state the
eventual target winner was already present in the base branch set, yet its
affine prediction had the wrong sign with a roughly constant error per distance
over radii `1e-3` through `1e-5`; both signs of the proposed direction
decreased. The
base had one indeterminate vertex count, but a follow-up audit found that it
came from an unbounded nearly singular four-facet system and did not make the
recorded primal incidence uncertain. The immediate failure is therefore an
unresolved branch-derivative, KKT-regime, volume-derivative, or bookkeeping
problem rather than a missing target word. It does not classify the endpoint
as a local maximum. See [`../endpoint-model-audit/`](../endpoint-model-audit/)
for the technical account.

Post-process a raw output directory with:

```bash
uv run --script experiments/dev-gradient-ascent/ascent-continuation/analyze.py \
  RAW_OUTPUT ANALYSIS_OUTPUT
```

The analysis writes a reader-facing report, CSV summaries, and plots of
cumulative gain against path length, slope by accepted step, candidate gain by
distance, and measured runtime components.

For optimizer-endpoint packets, use
`analyze_optimizer_endpoints.py`; the ordinary `analyze.py` is specialized to
the fixed four-state continuation panel.
