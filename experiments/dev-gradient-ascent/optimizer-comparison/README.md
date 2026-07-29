# Local-optimizer comparison

This directory contains the strict schema-1 analyzer for datasets produced by
`../optimizer-runs/`. It was selectively imported from archive commit
`073bb014428de60946a8ea1b744f4e8992042a83`.

The analyzer validates completion and cross-table links before producing
best-so-far curves by charged call, evaluator time, and measured
evaluator-plus-optimizer time; paired-start comparisons; proposal behavior;
compute profiles; and checkpoint selections.

Schema-1 packets written before current-state recording was added remain
supported as an explicit legacy case: the analyzer validates every check
supported by their recorded fields without inferring an algorithm state. New
packets must record both current-state fields on every round and a final state
on the run; those fields are validated for continuity, usable evaluation
references, and selected-state changes.

Use `--mode development` for the smoke and new method development. Tuning and
held-out modes require correspondingly declared manifest roles. A generated
summary is evidence only for its exact producer, source population, manifest,
and evaluator contract.

See [`../optimizer-runs/README.md`](../optimizer-runs/README.md) for the runner
contracts, current algorithm surface, smoke command, and ranked missing-method
batch.

## Retained F=10 comparison

The frozen held-out packet
`artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/` compares seven fixed
policies on 64 matched, previously unused random ten-facet starts. Each run had
a 1,000 ms measured serial evaluator-plus-optimizer ceiling and a 128-call cap.
The analyzer validated 448 runs. Four-anchor branch history had the largest
median best `sys`, `0.984783` (10--90% across starts
`0.957033`--`0.998515`); no run reached `sys >= 1`.

This is a finite-budget ranking for the named population, evaluator, and
budget. It is not a convergence or local-maximality result. The companion
trajectory-geometry directory describes matched paths, while
`../quotient-endpoint-diagnostic/` and `../ascent-continuation/` retain the
finite probes and continued improvements that rule out interpreting the
one-second plateaus as established local maxima.
