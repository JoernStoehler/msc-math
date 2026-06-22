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
prepare:  experiments/sys-datascience/prepare/prepare-local-behavior.py
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
- `near_active_prediction_error` and `near_active_prediction_abs_error`.

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
- The local-behavior producer and prepare stage can now select provenance
  rows from source datasets and keep planned-attempt denominators by source
  stratum, radius, and direction family.

## Open Questions

- Over random starts, at which radii do target minimizers leave the base
  near-active set or candidate window?
- How do those rates differ by source stratum, direction family, and base
  `sys` range?
- Does local branch predictivity persist in high-`sys`, top-tail, and
  ascent-generated endpoint or near-endpoint regions, or is random-start
  behavior misleading for optimizer claims?
- Which branch-window policy trades target-minimizer coverage against
  prediction noise best enough for optimizer design?
- When near-active first-order prediction fails at ascent endpoints, do wider
  base candidate-window branches explain the finite `Delta sys`, or is the
  failure due to branches outside the window or nonlinear drift?
- Are `t` and relative finite `sys` change useful predictors of whether a
  target point is still explained by base-point branch data?
- Which endpoint-stability experiment is worth running after source-stratified
  local behavior is measured?

## Next Target

Use the source-stratified path to estimate local branch coverage over sampled
starts. The statistical unit is a provenance/start row, while expensive point
payloads can still be cached by `poly_id`.

First evidence-building target:

- sample from `polytope-provenance-table.jsonl`, initially
  `random_sample` and `random_product_sample`;
- keep failure rows in planned-attempt denominators;
- summarize by source stratum, radius, and direction family;
- report start counts, planned attempts, successful pairs, and
  direction-eligible starts;
- keep output run-local until a retained panel is intentionally chosen;
- decide whether the next panel should spend compute on more starts, more
  random directions per start, wider radii, or branch-window sweeps.

When judging random-start panels, use `local-behavior-start-breakdown.csv`
before quoting rates. Pair rows are correlated within a start, and starts with
more active branches generate more gradient-family rows.

Local current-code attempts to run larger random-source panels were stopped
before retained artifacts were written. This does not answer the random-start
distribution question. It means the next distribution estimate should either run
as a batch job or first make the producer cheaper/incremental enough to write
partial retained artifacts.

Current producer support for that next estimate:

- `sys-local-behavior-produce` has `--basepoint-start` and
  `--basepoint-limit` for complete selected-basepoint shards;
- shard runs preserve global `base_####` ids;
- `experiments/sys-datascience/prepare/combine-local-behavior-shards.py`
  combines non-overlapping completed shards into one marked combined producer
  directory and rejects overlapping ranges or duplicate joined-row ids.

The combined smoke path verifies prepare/analyze on both random source strata,
but a smoke with only a few starts is not a distribution estimate. Before
quoting rates from any source-stratified run, inspect
`prepared/local-behavior-source-radius-summary.csv`,
`prepared/local-behavior-start-breakdown.csv`,
`prepared/local-behavior-candidate-window-summary.csv`, and
`prepared/local-behavior-candidate-gradient-summary.csv`.

For endpoint panels, inspect `local-behavior-candidate-window-summary.csv` and
`local-behavior-candidate-window-predictions.jsonl` before blaming missing
candidate coverage. Those files compare finite target values over the base
candidate window; they are not analytic candidate-window derivative rows.

Also inspect `local-behavior-candidate-gradient-summary.csv` and
`local-behavior-candidate-gradient-predictions.jsonl`. These rows test the
analytic base candidate-window model with base branch gaps included and with
each candidate branch differentiated at its own action. The current method
implication is: near-active-only first-order prediction is too optimistic near
ascent/top endpoints, but the first-order route remains live as a
candidate-window-aware recentered method. This is a plausible repair path, not
a validated method.

Do not lean on the finite candidate-window rows as independent evidence when
the target minimizer is already inside the base candidate window. The analytic
candidate-gradient rows are the relevant evidence for whether the repair path
is first-order rather than only retrospective.

The candidate-window-aware dev-gradient runner now exists for comparison.
The scoring-only mode changes finite-step scoring over the existing generated
direction family. An opt-in candidate-window maximin direction generator also
exists. Treat `action_window_relative = 0.01` as a working parameter to test, not
as an evidence-backed thesis constant. Neither variant is validated by this
ledger without retained outputs.

The next high-value crux is whether candidate-window scoring ranks observed
good moves better than the near-active model on hard trace states. Test that by
exhaustive replay/ranking of the generated direction/step pairs. The current
audit code records the branch/orbit witness attaining the lower-envelope
prediction; use that witness to investigate numerical derivative pathologies,
branch-domain/model invalidity, and ordinary pessimism from using a wide
candidate window.
