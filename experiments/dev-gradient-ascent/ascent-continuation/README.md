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

Because the retained packet predates the removal of redundant model
construction, this is a semantic rerun route, not a byte-for-byte or
runtime-identical regeneration command.

The four fixed states are one endpoint known to improve under a larger budget,
the later version of that endpoint with a remaining positive basis slope, one
endpoint whose basis slopes remain negative across three radii, and HKO as a
local-maximum control.

A stop is not a local-maximality result: the finite-step models and one signed
basis only cover a restricted direction set.

## Retained result

The retained four-state packet is
`artifacts/four-state-full-20260729/`. Its generated report records ten
accepted moves on each of three optimizer endpoints and no accepted move from
the HKO control. One endpoint for which all signed basis directions decreased
still gained `0.00223027` along a branch-informed path of normalized length
`0.01`, with positive tested slope at the step cap. This establishes only that
the selected finite-budget endpoint had a sustained untested ascent path.

The HKO development calibration is retained under
`artifacts/hko-one-step-development-panel-20260729/`. It tests four
predeclared directions at four distances, with HKO as a false-positive
control. Every perturbation improved and recovered `0.934`--`1.000` of its
known gap in one validated move; HKO accepted no move. This is four-direction
development evidence, not a miss-rate estimate.

The outcome-selected top-eight check is retained under
`artifacts/top8-tuning-endpoints-one-step-20260729/`. Five of eight endpoints
improved, none crossed `sys = 1`, and the cohort exposed systematic
finite-distance affine failures. The causal decomposition of three proposals
belongs to [`../endpoint-model-audit/`](../endpoint-model-audit/).

Post-process a raw output directory with:

```bash
uv run --script experiments/dev-gradient-ascent/ascent-continuation/analyze.py \
  RAW_OUTPUT ANALYSIS_OUTPUT
```

The analysis writes a reader-facing report, CSV summaries, and plots of
cumulative gain against path length, slope by accepted step, candidate gain by
distance, and measured runtime components. The retained helper
`analyze_optimizer_endpoints.py` owns the top-eight packet, while
`analyze_hko_calibration.py` owns the HKO control panel.
