# Local-optimizer comparison

This directory contains the strict schema-1 analyzer for datasets produced by
`../optimizer-runs/`. It was selectively imported from archive commit
`073bb014428de60946a8ea1b744f4e8992042a83`; no archive datasets, figures, or
specialized post-hoc analyzers were imported.

The analyzer validates completion and cross-table links before producing
best-so-far curves by charged call, evaluator time, and measured
evaluator-plus-optimizer time; paired-start comparisons; proposal behavior;
compute profiles; and checkpoint selections.

Use `--mode development` for the smoke and new method development. Tuning and
held-out modes require correspondingly declared manifest roles. A generated
summary is evidence only for its exact producer, source population, manifest,
and evaluator contract.

See [`../optimizer-runs/README.md`](../optimizer-runs/README.md) for the runner
contracts, current algorithm surface, smoke command, and ranked missing-method
batch.
