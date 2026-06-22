# Local Behavior Prediction Ledger

Purpose: current question ledger for source-stratified local/semi-local
`sys(a)` behavior. This is not thesis prose and not run output. Source truth
for computed observations is the current producer output plus the prepare and
analysis code in this packet.

## Session Goal

Understand local and semi-local behavior of `sys(a)` well enough to design and
evaluate a heuristic recentered first-order method. The intended method-level
claim is algorithmic: under a fixed implementation, thresholds, direction
generator, and recentering rule, a starting point returns an approximate
endpoint or endpoint class.

Do not turn this into a theorem-grade attractor claim. In fixed ordered
inequality coordinates

```text
K(a) = {x in R^4 : <a_i, x> <= 1, i = 1,...,F},
```

`sys` is continuous on the valid bounded full-dimensional chart domain, but it
is not generally `C^1`. The theorem-level local object is the active-germ /
direction-cell catalogue described in `research/sys-first-order-local-behavior.md`.

## Routing

The evidence pipeline for finite local/semi-local behavior lives here:

```text
produce:  experiments/sys-datascience/produce/local-behavior.rs
prepare:  experiments/sys-datascience/tables/prepare-local-behavior.py
analyze:  experiments/sys-datascience/methods/local-behavior-prediction/
```

`experiments/dev-gradient-ascent/branch-cartography/` is a prototype/reference
surface for optimizer design. It should not grow a parallel retained-evidence
pipeline for source-stratified local behavior. The deleted
`experiments/local-sys-methods/` package was an older overlapping smoke surface.

## Predicate Vocabulary

Use exact finite predicates from prepared rows instead of vague branch words:

- `same_min_branch_set`;
- `target_min_branches_all_in_base_near_active`;
- `target_min_branches_all_in_base_candidate_window`;
- `target_branch_status_at_base`;
- `min_branch_sets_intersect`;
- recomputed finite `observed_delta_sys`;
- `clarke_prediction_error` and `clarke_prediction_abs_error`.

Near-active and candidate-window claims are meaningful only with the stated
`branch_threshold_relative` and `action_window_relative`.

## Closed For This Packet

- HKO is a theorem/control stress case, not representative evidence for random
  starts.
- A single unqualified gradient is not available at non-generic points.
- Current finite diagnostics do not certify branch completeness or local
  maximality.
- Endpoint and basin language is algorithmic until a nonsmooth-flow definition
  is chosen and supported.
- Selected top/hash or hand-picked panels are not population estimates.

## Open Questions

- Over random starts, at which radii do target minimizers leave the base
  near-active set or candidate window?
- How do those rates differ by source stratum, direction family, and base
  `sys` range?
- Which branch-window policy trades target-minimizer coverage against
  prediction noise best enough for optimizer design?
- Are `t` and relative finite `sys` change useful predictors of whether a
  target point is still explained by base-point branch data?
- Which endpoint-stability experiment is worth running after source-stratified
  local behavior is measured?

## Next Target

Replace selected-panel observations with source-stratified estimates over
sampled starts. The statistical unit should be a provenance/start row, while
expensive point payloads can still be cached by `poly_id`.

First implementation target:

- sample from `polytope-provenance-table.jsonl`, initially
  `random_sample` and `random_product_sample`;
- keep failure rows in planned-attempt denominators;
- summarize by source stratum, radius, and direction family;
- report start counts, planned attempts, successful pairs, and
  direction-eligible starts;
- keep output run-local until a retained panel is intentionally chosen.
