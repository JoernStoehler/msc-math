# Fixed-Sigma Sysext Line Probe

Status: offline development diagnostic. This directory retains its producer
but no canonical output.

## Question

Along a finite line `a0 + t u`, how do selected raw KKT branches change in
action, linearization error, and beta margin without enumerating all target
branches at every sample?

This is different from predicting the minimum `sys` branch set. It follows
fixed sigmas selected at the base point: the best and near-active admissible
branches, plus bounded buckets of raw near-window, near-beta-boundary, and
low-action branches.

## Inputs and command

The probe consumes:

- `branch-set-diagnostic.jsonl` from
  `experiments/dev-gradient-ascent/branch-diagnostic/`;
- the corresponding producer-generated polytope table, defaulting to
  `experiments/polytope-invariant-table/polytope-table.jsonl`.

```bash
cargo run -p exp-dev-sys-prediction --release \
  --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --polytope-table experiments/polytope-invariant-table/polytope-table.jsonl \
  --out-dir /tmp/dev-sysext-sigma-line-check \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy
```

The default line samples seven `t` values from `-1e-3` through `1e-3`.
`--compute-target-sys-sigma` additionally computes the fixed branch's target
`sys_sigma`, including target volume; it still does not recompute the minimum
over all target branches.

Outputs:

- `fixture-selection.jsonl`;
- `sysext-sigma-line-probe.jsonl`;
- `summary.json`.

## Current evidence status

[`../OPTIMIZER-GUIDANCE.md`](../OPTIMIZER-GUIDANCE.md) preserves the summary
of an earlier scratch run: on one checked high-degeneracy line, selected
near-boundary invalid branches remained invalid, while far-invalid raw
low-action branches had very negative beta margins and were irrelevant to the
admissible `sys` minimum. The underlying scratch output is not retained in the
current repository, so that summary is guidance and a regeneration target,
not independently checkable evidence.

The current optimizer-facing policy is therefore conservative: use returned
admissible low-action branches for prediction, and use beta-invalid raw
branches only as offline diagnostics unless a regenerated line probe shows a
margin approaching zero inside the contemplated radius.

## Claim and maintenance boundaries

The probe follows a finite selected branch set along one direction from one
selected fixture. It does not measure target candidate-set completeness,
recompute true target `sys`, or establish prediction quality for other rows or
directions.

Its raw KKT solve is a packet-local instrumented implementation built from the
shared QP assembly surface. Changes to KKT assembly, residual correction, beta
classification, or fixed-branch derivatives should trigger comparison with
the corresponding implementations found by repository search; similar code
here is not a shared API.
