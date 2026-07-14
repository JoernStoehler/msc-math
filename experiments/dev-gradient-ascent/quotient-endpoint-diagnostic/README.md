# Quotient-Aware Endpoint Diagnostic

This packet tests whether frozen high `sys(a)` states have an observed ascent
direction after translations, scaling, and the identity-component linear
symplectic action are removed. Start with the generated
[`artifacts/DISCUSSION.md`](artifacts/DISCUSSION.md); it owns the interpretation,
direct control outcomes, limitations, evidence thresholds, and next decision.
The fresh readiness review and repair disposition are in
[`INDEPENDENT_REVIEW.md`](INDEPENDENT_REVIEW.md).

The producer uses a derivative-free signed-basis poll on the Euclidean
orthogonal complement of the 15 `sys`-symmetry tangent generators. This choice
avoids assuming that a base active-sigma list contains all right-active or
singular branch germs. It is a finite heuristic diagnostic, not a
local-maximality proof. HKO's exact theorem packet under
`experiments/hko-local-maximum/theorem/` remains the authority for that control.

Retained outputs:

- `artifacts/poll-directions.jsonl`: raw direction, geometry, capacity, and
  observed-improvement rows;
- `artifacts/states.jsonl`: source identity, recomputed target, facet status,
  and quotient diagnostics;
- `artifacts/radius-summaries.jsonl` and `artifacts/analysis.json`: generated
  compact views;
- `artifacts/run-provenance.json`: command, producer, selection-input hashes,
  and mathematical contracts;
- `figures/max-margin-by-radius.*` and `figures/directional-spread.*`:
  investigation displays generated from validated rows.

The producer's `--smoke` mode evaluates one signed quotient-basis pair at
relative radius `1e-4` for one negative control and HKO. Smoke output is
plumbing evidence only and is not retained.

Reproduction commands and the Git LFS input boundary are in the generated
discussion report. Unit tests check the `sp(4)` Lie-algebra basis and the
expected six-facet and HKO quotient dimensions.
