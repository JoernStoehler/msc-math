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

First retained-evidence target:

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
before artifacts were written:

- `/tmp/sys-local-behavior-random-source-n50-fixed`;
- `/tmp/sys-local-behavior-random-source-n20-fixed`.

Both were CPU-bound for too long for an interactive local checkpoint. This
does not answer the random-start distribution question. It means the next
distribution estimate should either run as a batch job or first make the
producer cheaper/incremental enough to write partial artifacts.

Current producer support for that next estimate:

- `sys-local-behavior-produce` has `--basepoint-start` and
  `--basepoint-limit` for complete selected-basepoint shards;
- shard runs preserve global `base_####` ids;
- `experiments/sys-datascience/tables/combine-local-behavior-shards.py`
  combines non-overlapping completed shards into one marked combined producer
  directory and rejects overlapping ranges or duplicate joined-row ids.

Smoke paths:

- `/tmp/sys-local-behavior-random-source-shard-smoke`;
- `/tmp/sys-local-behavior-random-source-shard-smoke-random`;
- `/tmp/sys-local-behavior-random-source-shard-combined-smoke`.

The combined smoke verifies prepare/analyze on both random source strata, but
it has only four starts and is not a distribution estimate.

First sharded estimate:

- `/tmp/sys-local-behavior-random-source-estimate-40starts`.

This output covers all starts selected by the current source-stratified run
with `starts-per-source=20`. Inspect
`prepared/local-behavior-source-radius-summary.csv`,
`prepared/local-behavior-start-breakdown.csv`,
`prepared/local-behavior-candidate-window-summary.csv`, and
`prepared/local-behavior-candidate-gradient-summary.csv` before quoting rates.
Current qualitative read: target-minimizer coverage failures are concentrated
at the larger checked radii, while candidate-gradient sign predictions do not
break on successful pairs in this panel. This is a first estimate for the
distribution question, not a final population estimate.

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

The candidate-window-aware dev-gradient runner now exists for comparison. The
scoring-only mode changes finite-step scoring over the existing generated
direction family. An opt-in candidate-window maximin direction generator also
exists, but the first overlap check below does not support pursuing that
generator without a new reason.

Current endpoint/top window-sweep scratch outputs:

Superseded for candidate-gradient interpretation:

- `/tmp/sys-local-behavior-ascent-top-window-003`;
- `/tmp/sys-local-behavior-ascent-top-candidate-gradient`;
- `/tmp/sys-local-behavior-ascent-top-window-03`.

Current corrected outputs:

- `/tmp/sys-local-behavior-ascent-top-window-003-fixed`;
- `/tmp/sys-local-behavior-ascent-top-candidate-gradient-fixed`;
- `/tmp/sys-local-behavior-ascent-top-window-03-fixed`.

The `*-fixed` outputs supersede the three earlier paths for candidate-gradient
interpretation because candidate-branch derivatives were corrected to use each
branch's own action. Current qualitative read from the fixed outputs: the
`0.003` action window is too narrow on the checked endpoint/top panel, while
`0.03` adds branch count and local-state cost without improving the
source-stratified candidate-gradient sign result over `0.01`. Treat
`action_window_relative = 0.01` as the current plausible setting for a
candidate-window-aware method variant, not as a final thesis constant.

Candidate-window dev-gradient overlap outputs:

- `/tmp/dev-gradient-ascent-candidate-window-overlap-panel-aggregate`;
- `/tmp/dev-gradient-ascent-candidate-window-overlap-panel-endpoint-scan-report`;
- `/tmp/dev-gradient-ascent-candidate-window-overlap-panel-run-trace-report`;
- `/tmp/dev-gradient-ascent-candidate-window-directions-overlap-panel-aggregate`;
- `/tmp/dev-gradient-ascent-candidate-window-directions-overlap-panel-endpoint-scan-report`;
- `/tmp/dev-gradient-ascent-candidate-window-directions-overlap-panel-run-trace-report`.
- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-aggregate`;
- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-endpoint-scan-report`;
- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-run-trace-report`.

These outputs cover the five retained fixtures available through the current
polytope table. They are not an exact six-fixture replacement for the older
near-active retained panel because one old large-gap random-product fixture
came from a table path that is not present in this worktree. Current read:
candidate-window scoring removes the accepted negative-prediction trace rows
on this overlap panel and keeps the finite endpoint scan below the configured
relative threshold, but it does not close endpoint local maximality and it does
not test the opt-in candidate-window direction generator.

The first candidate-window direction-generator test adds finite-step-indexed
candidate-window maximin directions and tests each such direction only at the
step used to generate it. On the same five-fixture overlap, those new
directions are not selected in the trace, add endpoint scan rows and runtime,
and do not improve the endpoint-scan threshold-ratio diagnostic. Current
method implication: keep candidate-window scoring as the live repair path, but
do not pursue this candidate-window maximin generator without a new reason.

The lower-threshold candidate-window replay tests whether the positive
below-threshold endpoint rows are mostly a too-large stopping threshold. On the
same five-fixture overlap, reducing the relative acceptance threshold to
`1e-4` makes traces continue to the iteration cap, but endpoint scans still
find improvements above the lowered threshold and the overlap panel again
contains a candidate-window prediction failure. Current method implication:
threshold-only repair is weakened. The next high-value crux is whether
candidate-window scoring actually ranks observed good moves better than the
near-active model on hard trace states. Test that by exhaustive replay/ranking
of the generated direction/step pairs, not by another broad random-source panel.

First exhaustive replay/ranking audit outputs:

- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-large-gap-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-1-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-high-degeneracy-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-high-degeneracy-rank-1-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-summary.json`.

Current read from the summary scratch: across 40 traced states, the
candidate-window scoring rule ranked the best recomputed improving checked move
first in 39 states; the older near-active rule did so in 25 states. The
candidate-window rule is therefore a live ranking repair candidate on this
small selected audit.

The same audit also has one state where that rule is not safe as a rejection
gate. In
`/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-0-check`,
iteration 2, the generated move `near_active_maximin_direction` at step
`0.001` has recomputed `Delta sys = +0.0007425813825379102`, above that row's
acceptance cutoff `0.00009773304353618649`. The candidate-window rule predicts
`Delta sys = -15189216053351.336` for the same move and ranks it last among the
six checked moves, while the near-active rule predicts
`+0.0007413626661731293` and ranks it first. This rules out a pure
prediction-gated first-order method without a fallback or explicit
failure-mode guard.

The retained audit output does not record which candidate branch caused the
large negative prediction. Current audit code records the branch/orbit witness
attaining the lower-envelope minimum; before spending more broad local compute,
rerun only this hard state and inspect that witness. That gives the branch-level
handle needed to investigate numerical derivative pathologies,
branch-domain/model invalidity, and ordinary pessimism from using a wide
candidate window; it does not by itself classify the failure.
