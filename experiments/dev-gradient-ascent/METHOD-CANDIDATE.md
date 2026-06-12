# Candidate Observed Multi-Direction Ascent

Status: current development candidate, not promoted library code and not thesis
evidence by itself.

This note names the current method candidate so later work can review or change
it without reconstructing the algorithm and checked evidence from JSONL files.
The objective remains the one in [CHARTER.md](CHARTER.md).
The current promotion decision packet is
[PROMOTION-READINESS.md](PROMOTION-READINESS.md).

## Algorithm

At each trace state:

1. Recompute the active `sys` state and collect near-active HK sigma branches.
2. Build local directions from the current branch model:
   - `single_near_active_gradient`;
   - `negative_single_near_active_gradient`;
   - `near_active_maximin_direction` when more than one near-active branch is
     present.
3. Sort generated directions by predicted directional derivative, descending.
4. Try each generated direction and configured trace step.
5. Accept the first finite step whose recomputed `sys` improvement is above
   the effective threshold
   `max(min_observed_delta, min_observed_relative_delta * abs(base_sys))`.
6. Stop when all generated direction/step pairs fail that observed-improvement
   test, or when the trace iteration cap is reached.

This differs from the earlier predicted-positive policy: negative-predicted
directions are still tested because recomputation can reveal finite
branch-switching improvements.

## Current Parameters

Checked trace parameters:

- trace steps: `1e-3,1e-4`;
- endpoint scan steps: `1e-3,1e-4,1e-5,1e-6`;
- relative improvement threshold: `min_observed_relative_delta = 1e-3`;
- absolute improvement threshold: `min_observed_delta = 0`;
- branch selection threshold: `1e-3` for large-gap and narrow-gap checks;
- branch selection threshold: `0.01` for high-degeneracy checks because the
  available high-degeneracy fixture labels in the current branch diagnostic are
  at the wider threshold;
- trace iteration cap in checked regime runs: `8`.

These are development parameters, not approved thesis constants.

## Checked Evidence

Source branch diagnostic:

- `/tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check`
- selected rows: `8`;
- diagnostic rows: `40`;
- successful recomputations: `8`;
- degeneracy labels across threshold sweep:
  `large_gap = 26`, `narrow_gap = 11`, `high_degeneracy = 3`.

Current one-fixture regime panel:

| Regime | Output directory | Trace result | Endpoint scan threshold counts |
| --- | --- | --- | --- |
| large-gap | `/tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check` | four accepted above-threshold steps, then method stop | `above_threshold = 0`, `positive_below_threshold = 8`, `nonpositive = 4` |
| narrow-gap | `/tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check` | one accepted above-threshold step, then method stop | `above_threshold = 0`, `positive_below_threshold = 7`, `nonpositive = 5` |
| high-degeneracy | `/tmp/dev-gradient-ascent-endpoint-smallsteps-high-degeneracy-check` | one accepted above-threshold step, then method stop | `above_threshold = 0`, `positive_below_threshold = 7`, `nonpositive = 5` |

In all three checks, the endpoint scan found no above-threshold finite
improvement at the checked endpoint-step grid. Positive endpoint scan rows
remain, but their observed improvements shrink with step size and stay below
the relative threshold.

## Fixed Failure Modes

The candidate fixed two checked failure modes:

- Large-gap branch-switching miss: the previous maximin-only endpoint check
  missed an above-threshold `single_near_active_gradient` finite step. The
  observed multi-direction trace now accepts that step before stopping.
- High-degeneracy negative-prediction miss: a finite step with negative branch
  prediction had recomputed above-threshold `sys` improvement. The candidate
  tests all generated directions, so it accepts that finite improvement.

## Current Cost Boundary

A local broadened run with up to two `large_gap` and two `narrow_gap` fixtures
was attempted:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --out-dir /tmp/dev-gradient-ascent-broadened-large-narrow-check \
  --steps 1e-3,1e-4 \
  --endpoint-steps 1e-3,1e-4,1e-5,1e-6 \
  --max-fixtures-per-label 2 \
  --degeneracy-labels large_gap,narrow_gap \
  --trace-iterations 8 \
  --min-observed-relative-delta 1e-3
```

It was interrupted after exceeding the local-pass budget, before trace and
endpoint artifacts were written. The output directory only contains:

- `fixture-selection.jsonl`;
- `local-geometry-probe.jsonl`.

This means broad retained-sample runs with the current endpoint scan are too
expensive to keep running ad hoc in local chat turns. They need either cheaper
orchestration, a reduced diagnostic, or a cluster/local-batch handoff.

The local geometry runner now supports `--skip-fixtures-per-label`, records
`selection_rank_within_label` in `fixture-selection.jsonl`, and records the
run-level selector settings in `summary.json` and `compute-budget-report.json`.
This allows broadened checks to be split into complete one-fixture output
directories. It does not by itself supply the missing broader retained-sample
evidence.

The package also includes `dev-gradient-ascent-aggregate-summaries`, which
combines `summary.json` counts from complete split runs into
`run-summary.jsonl`, combines `compute-budget-report.json` rows into
`budget-summary.jsonl` when present, and writes `aggregate-summary.json`. It is
an accounting helper, not a diagnostic that inspects trace rows or recomputes
`sys(a)`.

Checked selector smoke:

- output: `/tmp/dev-gradient-ascent-offset-selector-smoke`;
- selector: `large_gap`, `--max-fixtures-per-label 1`,
  `--skip-fixtures-per-label 1`;
- selected fixture rank: `selection_rank_within_label = 1`;
- trace iterations: `0`;
- probe rows: `2`;
- endpoint scan rows: `2`;
- failures: `0`;
- elapsed wall time: about `43s`.

Checked full split runs:

- output: `/tmp/dev-gradient-ascent-full-large-gap-rank-1-check`;
- selector: `large_gap`, `--max-fixtures-per-label 1`,
  `--skip-fixtures-per-label 1`;
- selected fixture rank: `selection_rank_within_label = 1`;
- trace result: six accepted above-threshold steps, then method stop;
- endpoint scan: `above_threshold = 0`, `positive_below_threshold = 8`,
  `nonpositive = 4`;
- failures: `0`;
- elapsed wall time: about `232s`.
- output: `/tmp/dev-gradient-ascent-full-narrow-gap-rank-1-check`;
- selector: `narrow_gap`, `--max-fixtures-per-label 1`,
  `--skip-fixtures-per-label 1`;
- selected fixture rank: `selection_rank_within_label = 1`;
- trace result: no accepted above-threshold step; immediate method stop;
- endpoint scan: `above_threshold = 0`, `positive_below_threshold = 8`,
  `nonpositive = 4`;
- failures: `0`;
- elapsed wall time: about `259s`.
- output: `/tmp/dev-gradient-ascent-full-high-degeneracy-rank-1-check`;
- selector: `high_degeneracy`, `--selection-threshold-relative 0.01`,
  `--max-fixtures-per-label 1`, `--skip-fixtures-per-label 1`;
- selected fixture rank: `selection_rank_within_label = 1`;
- trace result: four accepted above-threshold steps, then method stop;
- endpoint scan: `above_threshold = 0`, `positive_below_threshold = 8`,
  `nonpositive = 4`;
- failures: `0`;
- elapsed wall time: about `378s`.

Current retained-panel aggregate:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-aggregate-v3`;
- included complete runs:
  `/tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check`,
  `/tmp/dev-gradient-ascent-full-large-gap-rank-1-check`,
  `/tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check`,
  `/tmp/dev-gradient-ascent-full-narrow-gap-rank-1-check`,
  `/tmp/dev-gradient-ascent-endpoint-smallsteps-high-degeneracy-check`,
  `/tmp/dev-gradient-ascent-full-high-degeneracy-rank-1-check`;
- selected fixtures: `6`;
- degeneracy counts: `large_gap = 2`, `narrow_gap = 2`,
  `high_degeneracy = 2`;
- run trace rows: `22`;
- accepted above-threshold trace steps: `16`;
- method-stop trace rows:
  `line_search_all_steps_below_min_observed_delta = 6`;
- prediction-selected endpoint diagnostic rows:
  `6` rows had no positive observed improvement in the prediction-selected
  post-stop row;
- endpoint scan threshold counts:
  `above_threshold = 0`, `positive_below_threshold = 46`,
  `nonpositive = 26`;
- failed probe rows: `0`;
- failed endpoint scan rows: `0`;
- aggregate wall time from compute-budget reports: about `1679s`;
- per-regime wall times from compute-budget reports:
  `large_gap = 569s`, `narrow_gap = 489s`, `high_degeneracy = 622s`.

Interpretation: this panel checks two fixtures in each degeneracy regime. The
endpoint-direction scan supports the current finite endpoint condition at the
configured relative threshold on a small retained panel. The
prediction-selected endpoint diagnostic is only a one-row post-stop summary and
must not be read as a scan over all generated directions. This does not certify
that the endpoints are true local maxima, and the positive-below-threshold
endpoint scan rows remain a caveat for thesis wording or further method
development.

Current endpoint-scan magnitude report:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-endpoint-scan-report`;
- scanned endpoint rows: `72`;
- above-threshold rows: `0`;
- positive-below-threshold rows: `46`;
- nonpositive rows: `26`;
- missing observed deltas or thresholds: `0`;
- largest positive observed endpoint delta:
  `7.383249465420239e-4`;
- largest ratio to the effective endpoint threshold: about `0.756`;
- row attaining both maxima: the narrow-gap run
  `/tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check`,
  direction `post_stop_near_active_maximin_direction`, step `1e-3`;
- per-regime largest ratios to threshold:
  `large_gap = 0.723`, `narrow_gap = 0.756`, `high_degeneracy = 0.631`.

Interpretation: the finite endpoint scan caveat is quantitatively nontrivial:
many positive rows remain. In this checked panel, however, none reaches the
configured relative endpoint threshold; the largest positive row is about
three quarters of that threshold.

Current run-trace behavior report:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-run-trace-report`;
- trace rows: `22`;
- accepted rows: `16`;
- method-stop rows: `6`;
- accepted rows with negative predicted delta: `5`;
- accepted rows after at least one earlier direction was rejected: `7`;
- accepted rows with positive observed delta but negative predicted delta:
  `5`;
- accepted rows by direction: `single_near_active_gradient = 16`;
- per-regime accepted negative-predicted rows:
  `large_gap = 0`, `narrow_gap = 0`, `high_degeneracy = 5`;
- per-regime accepted-after-rejected-direction rows:
  `large_gap = 2`, `narrow_gap = 0`, `high_degeneracy = 5`.

Interpretation: this is direct evidence for two retained method choices in the
observed multi-direction policy. The high-degeneracy improvements in this
panel require testing finite steps even when the local branch model predicts a
negative delta. The large-gap and high-degeneracy traces also show accepted
steps after a different generated direction failed the observed-improvement
test.

## Remaining Gaps

The candidate is not ready for promotion until at least these gaps are handled:

- Broader retained sample: current positive result is a small retained panel
  with two fixtures in each degeneracy regime.
- Endpoint condition: finite endpoint scans still find positive improvements
  below threshold. The current report bounds them in the checked panel, but the
  retained endpoint claim must still define whether those are acceptable,
  require adaptive continuation, or require a different endpoint diagnostic.
- Compute cost: the current small-step endpoint scan is expensive enough that
  broader local panels are not practical in this session shape.
- Downstream integration: no reusable algorithm pieces have been promoted to
  `exp-sys-landscape` or crate code.
- Thesis wording: no thesis-claim packet exists yet.

## Next Useful Work

High-value next steps:

- Decide whether the current endpoint condition should be retained, tightened,
  or replaced, using the endpoint-scan magnitude report; then state its
  tolerances, failure modes, and compute budget explicitly. The current
  decision framing is in [PROMOTION-READINESS.md](PROMOTION-READINESS.md).
- Add a batch or LICCA-ready runner if the retained sample should grow beyond
  the current local six-fixture panel.
