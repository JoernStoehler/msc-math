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

The 1,000 ms value is a start-new-work cutoff, not a truncation of an active
ask/evaluate/tell round. The runner checks accumulated charged evaluator plus
optimizer time before starting another round and between members of a proposed
batch, then completes `tell` for the evaluated prefix. Consequently the
terminal `best_sys` in `runs.jsonl` and `final-summary.csv` includes a final
atomic round begun below 1,000 ms even when it finishes above the cutoff; there
is no separate post-loop terminal evaluation. A final `ask` that returns no
proposal can also add terminal optimizer time without changing `best_sys`.
In this packet, 310 of 448 terminal charged-compute totals exceed 1,000 ms
(maximum 1,067.525 ms); 305 contain an evaluated overrun round, and every such
round began below the cutoff. Runs can instead stop below the cutoff because
the optimizer finishes or the 128-call cap is reached. The comparison therefore
uses the same nominal ceiling and atomic stopping rule, not identical realized
milliseconds.

This is a finite-budget ranking for the named population, evaluator, and
budget. It is not a convergence or local-maximality result. The companion
trajectory-geometry directory describes matched paths, while
`../quotient-endpoint-diagnostic/` and `../ascent-continuation/` retain the
finite probes and continued improvements that rule out interpreting the
one-second plateaus as established local maxima.

In this historical packet, `sys` names the recorded heuristic evaluator field.
It is not a certified mathematical capacity value; the active thesis denotes
it by `hat(sys)`. The ranking and threshold counts are statements about that
field and evaluator contract.
