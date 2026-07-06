# Tail And Rare-Event Modeling

Use / maintenance model: seed-level topic map for rare-event and scale-up
reasoning. Keep source-backed table facts separate from model-sensitive
inferences. Update when tail evidence changes compute or session-spawn
decisions.

Scope: empirical survival curves, extreme-value/tail models, zero-positive
posteriors, and compute-scale decisions for blind or focused large random runs.

Current belief: zero positives in the retained table is a stable fact, but
probability estimates for `sys > 1` under larger blind runs are model-dominated.
The clean `tail-survival-1m-posterior` packet is now the current entry point for
zero-positive and model-sensitivity evidence; it excludes the earlier tainted
HKO-distance/flank section.

Evidence sources:

- `../../methods/tail-survival-1m-posterior/`
- `../../methods/high-sys-tail-diagnostic/`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/`

Owner-readiness/status: parked topic; reopen only if scale-up wording, future
1M+ run design, or thesis-facing rare-event interpretation becomes relevant.

Adjacent topics to read:

- `generated-candidate-proposers.md`, for focused-proposer alternatives to
  blind scale-up.
- `hko-reference-and-local-geometry.md`, for HKO/reference boundaries.

Candidate hypotheses:

- Current retained random/product generators have positive but very small
  `sys > 1` probability near structured regions.
- Pooled tail fits are mostly mixture artifacts and should not drive scale-up
  decisions.
- Endpoint-like tail behavior is more plausible than exponential continuation
  beyond the observed record, but current evidence is not decisive.
- Focused generator-axis work dominates blind scale-up in expected value.

Cheap discriminators:

- Interpret `tail-survival-1m-posterior` against concrete candidate thesis
  sentences before launching more tail modeling.
- Run tail fits per fixed bucket and compare backtests, avoiding pooled claims.
- Compare blind large-run predictions against generated-candidate proposer
  selected tails.

Ready packet prompts:

- Tail survival thesis-use audit. `reviewer-ready`.
  Objective: decide which statements from
  `../../methods/tail-survival-1m-posterior/` are safe enough for thesis
  wording. Output should separate source-backed zero-positive facts,
  model-sensitive extrapolations, run-scale decisions, and statements that
  should remain out of thesis prose.

Needs topic-owner sharpening:

- Bucket-level tail backtest packet.
- Sequential stopping design for any future 1M+ blind probe.

Opportunity-cost notes: expensive blind runs should be framed as probes, not
calibrated hit searches, unless tail/generator evidence improves.
