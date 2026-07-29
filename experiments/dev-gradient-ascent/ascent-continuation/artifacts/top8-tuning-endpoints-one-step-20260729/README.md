# Top-eight retained endpoint continuation

Status: retained outcome-selected discovery check. This is not a held-out
optimizer comparison or local-maximality classification.

## Question

Do the eight highest four-anchor branch-history endpoints from the retained
128-start tuning dataset admit one validated move from the branch-informed
continuation diagnostic? In particular, can the highest endpoint at
`sys = 0.9999624776406894` cross `sys = 1`?

## Result

The answer to the crossing question is no. The highest endpoint had no
improvement among either finite-gap branch model at five radii, the five
current-winning-branch gradient moves, or the 50 signed quotient-basis fallback
directions. Its least negative tested change was `-3.24515353e-6`.

Five of the eight endpoints did have one validated branch-model improvement,
with gains from `1.57114391e-6` to `4.60807882e-5`. None crossed one. The
question-oriented report and plot are in
[`analysis/REPORT.md`](analysis/REPORT.md).

At the highest endpoint, the evaluator's displayed target winner at every
proposed point was already in the base branch set. Its affine model predicted
an increase, but evaluator recomputation decreased with a roughly constant
error per distance over radii `1e-3` through `1e-5`. The opposite direction
also decreased. The base had one indeterminate geometry and vertex count; the
detailed curve is
`analysis/top-endpoint-model-error.png`. A follow-up audit showed that this
count came from an unbounded nearly singular four-facet system, not uncertain
recorded primal incidence. The immediate cause is therefore unresolved; see
[`../../../endpoint-model-audit/`](../../../endpoint-model-audit/).

The top-eight run took 20.09 seconds and 278 full evaluator calls. A
branch-model success used 16 evaluations: one base, ten max--min, and five
winning-gradient evaluations. A model stop used 66 because it additionally
triggered the 50-direction basis fallback.

## Reproduction

From the repository root:

```bash
uv run --script \
  experiments/dev-gradient-ascent/ascent-continuation/make_optimizer_endpoint_packet.py \
  experiments/dev-gradient-ascent/optimizer-runs/artifacts/candidate-history-f10-128-0d699aff5 \
  experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-quotient-ray/evaluations.jsonl \
  /tmp/top8-endpoint-input.json \
  --algorithm nonlinear-linearized-w3e-1-beta3e-1-h4-n2-d1e-1 \
  --top 8 \
  --accepted-step-cap 1 \
  --radii 1e-3,3e-4,1e-4,3e-5,1e-5

cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode debug \
  --states-json /tmp/top8-endpoint-input.json \
  --out-dir /tmp/top8-endpoint-run

# Separate diagnostic for both signs of the top endpoint's model direction.
uv run --script \
  experiments/dev-gradient-ascent/ascent-continuation/make_optimizer_endpoint_packet.py \
  experiments/dev-gradient-ascent/optimizer-runs/artifacts/candidate-history-f10-128-0d699aff5 \
  experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-quotient-ray/evaluations.jsonl \
  /tmp/top1-endpoint-input.json \
  --algorithm nonlinear-linearized-w3e-1-beta3e-1-h4-n2-d1e-1 \
  --top 1 \
  --accepted-step-cap 1 \
  --radii 1e-3,3e-4,1e-4,3e-5,1e-5

cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode debug \
  --states-json /tmp/top1-endpoint-input.json \
  --mirror-model-directions \
  --out-dir /tmp/top1-endpoint-mirror-run

uv run --script \
  experiments/dev-gradient-ascent/ascent-continuation/analyze_optimizer_endpoints.py \
  /tmp/top8-endpoint-run \
  /tmp/top8-endpoint-input.json \
  /tmp/top8-endpoint-analysis \
  --mirror-run /tmp/top1-endpoint-mirror-run
```

The tracked `input.json`, `raw/`, `raw-mirrored-top-endpoint/`, and `analysis/`
directories retain these runs. The producer, packet selector, analysis code,
and outputs are committed together.

## Interpretation boundary

The endpoints were selected by their outcomes on tuning data. A positive move
shows that one endpoint admitted a finite improvement under this evaluator. A
miss does not establish local maximality: the rotated-pentagon control
demonstrates that both a signed basis and sparse generic directions can miss a
thin improving set. The three stopped states are numerical local-max
candidates under this diagnostic, not certified local maxima.
