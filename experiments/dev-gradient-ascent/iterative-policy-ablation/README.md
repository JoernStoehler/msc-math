# Iterative Step-Policy Ablation

Status: bounded retained optimizer-development packet; results pending the
first reviewed two-start run.

## Question and decision rule

Does guarded candidate-window direction ordering plus exact dyadic
expand/backtrack obtain more `sys` improvement per exact target-capacity
evaluation than the current fixed-step and first-boundary-scaled controls on
the smallest durable multi-start narrow-gap panel?

All variants start from the same deterministic rows, use the established
direction set, and differ only in step scheduling. Each start-policy pair gets
the same exact target-evaluation budget. Base-state orbit searches are recorded
separately and do not consume that proposal budget. Each iteration may spend at
most four proposals before accepting its best above-threshold move. Fixed uses
the configured step in candidate-window direction order. Dyadic doubles while
the exact improvement grows and halves after an unsuccessful initial proposal.
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
