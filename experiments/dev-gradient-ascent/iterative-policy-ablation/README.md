# Iterative Step-Policy Ablation

Status: bounded retained optimizer-development packet; the two-start gate is
closed and no further optimizer run is currently warranted.

## Question and decision rule

Does guarded candidate-window direction ordering plus exact dyadic
expand/backtrack obtain more `sys` improvement per exact target-capacity
evaluation than the current fixed-step and first-boundary-scaled controls on
the smallest durable multi-start narrow-gap panel?

All variants start from the same deterministic rows, use the established
direction set, and differ only in step scheduling. Each start-policy pair gets
the same exact target-evaluation budget. Base-state orbit searches are recorded
separately and do not consume that proposal budget. Each iteration may spend at
most four proposals. It accepts the best tested step for the first
candidate-window-ordered direction yielding an above-threshold move. Fixed uses
the configured step in that direction order. Dyadic doubles while the exact
improvement grows and halves after an unsuccessful initial proposal.
Boundary-scaled checks the legacy ordered fractions of the first finite
combinatorial boundary.

Select dyadic for the next optimizer stage only if it improves exact-evaluation
efficiency on both retained starts without a new correctness or numerical
failure. Otherwise park it or retain the observed regime split. This is a
bounded method decision, not a default-policy, endpoint, or local-maximum
claim.

## Inputs and command

The panel is the two eligible `narrow_gap` rows at relative threshold `1e-3`
in the tracked branch diagnostic and polytope panel. The producer records input
and source hashes in `artifacts/run-provenance.json`.

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --out-dir experiments/dev-gradient-ascent/iterative-policy-ablation/artifacts \
  --steps 1e-3 \
  --max-fixtures-per-label 2 \
  --degeneracy-labels narrow_gap \
  --trace-iterations 8 \
  --direction-model candidate-window \
  --iterative-ablation-policies fixed,geometric,boundary-scaled \
  --iterative-exact-evaluation-budget 8 \
  --iterative-proposal-limit 4
```

Development smoke used one start and two exact evaluations per policy. Its
`/tmp` output checks plumbing only and is not retained evidence.

## Result and disposition

The source-qualified retained run is in [`artifacts/`](artifacts/). Read
`iterative-policy-outcomes.jsonl` for the six trajectory outcomes,
`iterative-policy-proposals.jsonl` for every charged exact evaluation, and
`compute-budget-report.json` plus `run-provenance.json` for costs and identity.

Dyadic expand/backtrack improved more than fixed step on both starts under the
common eight-evaluation cap. Boundary-scaled behavior split by start: it found
no improving move on the prior hard start and stopped after its four available
first-boundary proposals, but it slightly exceeded dyadic on the other start.
All 44 proposal evaluations completed without a recorded failure or nonfinite
observed delta. The artifact records target- and base-orbit search costs; the
second start was much more expensive than the first despite the same exact
proposal cap.

The predeclared strict selection rule therefore fails. Retain dyadic as a
robust improvement over the current fixed-step control on this two-start panel,
but do not select it as a general/default policy and do not claim dominance
over boundary scaling. This packet supports a start-dependent step-scale
decision only; it says nothing about endpoints or local maxima.

Park this line. Reopen with a larger frozen panel only if a downstream method
must choose one default scheduler, or if a concrete trajectory failure needs
to distinguish the dyadic and boundary regimes. That would be the next
expensive decision; the current packet does not justify running it merely to
strengthen presentation.
