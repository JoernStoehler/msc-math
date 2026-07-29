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
The cached HKO certificate-slope calibration under
`experiments/hko-local-maximum/empirical/certificate-slope-calibration/`
shows why slope magnitude is not a distance-to-maximum proxy for this sharp
nonsmooth control.

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

The same producer can consume an optimizer comparison's
`checkpoint-selection.json` with `--checkpoint-selection`, `--algorithm-id`,
and `--checkpoint-call`. In this mode it uses the optimizer evaluator's f64
geometry and volume route without rational-arithmetic fallback. The companion
`analyze_population.py` validates and summarizes an endpoint population. The
retained held-out F=10 branch-history result is under
`artifacts/heldout-f10-64-history-endpoints-19a8b4dfd{,-analysis}/`.
The same 16 starts after a five-second-ceiling history run are under
`artifacts/history-f10-16-compute-depth-endpoints-426ec7a7c{,-analysis}/`;
the latter report compares endpoint values, stopping reasons, and signed basis
slopes directly.

Reproduction commands and the Git LFS input boundary are in the generated
discussion report. Unit tests check the `sp(4)` Lie-algebra basis and the
expected six-facet and HKO quotient dimensions.
