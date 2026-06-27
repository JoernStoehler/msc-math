# Dev Sys Prediction Current Results

Status: producer smoke evidence plus optimizer-facing interpretation. This is
not a broad statistical study.

## Producer

Command:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-cloud-largegap-smoke \
  --selection-threshold-relative 0.001 \
  --degeneracy-labels large_gap \
  --max-fixtures-per-label 1 \
  --steps 1e-4 \
  --trace-iterations 1
```

Analogous high-degeneracy run:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-cloud-highdeg-smoke \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels high_degeneracy \
  --max-fixtures-per-label 1 \
  --steps 1e-4 \
  --trace-iterations 1
```

The compact fixture panel used for these smoke runs was:

```text
/tmp/dev-sys-prediction-fixture-panel.jsonl
```

It was extracted from the retained datascience polytope table. The producer can
read a compact panel through the existing `--polytope-table` argument, so smoke
runs do not need to parse the full retained table.

## Observations

Both smoke runs used the same base polytope:

```text
07455e997d624c62193180fd92026e2aba426e9b5bd1c3be4e8fe303ca4ffe5b
```

At threshold `0.001`, it is a `large_gap` fixture with one near-active branch.
At threshold `0.01`, it is a `high_degeneracy` fixture with ten near-active
branches.

Large-gap smoke:

- rows: 4;
- max absolute candidate-window prediction error: `5.334050949198921e-08`;
- candidate-window ranking matched observed ranking for all rows;
- elapsed time: `33.6s` for one fixture and four directions.

High-degeneracy smoke:

- rows: 5;
- max absolute candidate-window prediction error: `5.334050949198921e-08`;
- candidate-window ranking matched observed ranking for all rows;
- elapsed time: `33.9s` for one fixture and five directions.

Second high-degeneracy radius:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-cloud-highdeg-r1e-3 \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels high_degeneracy \
  --max-fixtures-per-label 1 \
  --steps 1e-3 \
  --trace-iterations 1
```

At radius `1e-3`:

- rows: 5;
- max absolute candidate-window prediction error: `5.343851513233289e-06`;
- candidate-window ranking matched observed ranking for all rows;
- elapsed time: `29.5s` for one fixture and five directions.

The meaningful high-degeneracy result is narrow: the tested first-order lower
envelope with base branch gaps,

```text
min_sigma (gap_sigma(a0) + t <grad sys_sigma(a0), u>)
```

predicted the observed finite-step ranking and deltas accurately on the tested
directions and two radii. The run also had copied debug columns for a no-gap
near-active calculation; those columns are not interpreted here because ignoring
the base gaps was not a serious proposed model.

The evidence supports separating:

- behavior of the returned low-action sigma set
  `action <= min_action(a0) * (1 + threshold)`;
- behavior of individual raw `sysext_sigma(a)` branches and beta-domain
  boundaries.

## Cost Interpretation

The smoke producer is too slow for direct use inside the optimizer loop in its
current form. It recomputes actual `sys(a0 + t u)` for every cloud row with the
AllSafe branch route and exact volume path. That is useful offline evidence,
but it is not yet a cheap step-selection primitive.

This does not falsify prediction as an optimizer aid. It says the optimizer
should not pay full prediction-cloud recomputation at every step. The plausible
split is:

- use branch-window lower-envelope information at `a0` for cheap model
  predictions and direction ranking;
- use sparse recomputed `sys` checks for validation and line search;
- use a separate `sysext_sigma` microprobe for beta-domain geometry, because a
  fixed sigma call should be microsecond-scale and avoids enumerating all
  branches.

## Sysext Sigma Line Probe

Command:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sysext-sigma-line-highdeg-smoke \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy \
  --steps -1e-3,-3e-4,-1e-4,0,1e-4,3e-4,1e-3
```

Output:

```text
/tmp/dev-sysext-sigma-line-highdeg-smoke/sysext-sigma-line-probe.jsonl
```

Result:

- rows: 70;
- sigmas: base best plus nine additional near-active sigmas;
- statuses: all `ok`;
- elapsed time: `298ms` excluding release compile;
- every sampled fixed sigma stayed beta-positive on the tested line;
- the smallest beta margins over the line were about `0.00206` for the two
  closest-to-boundary sigmas.

This supports treating fixed-sigma `sysext_sigma(a0 + t u)` as a cheap separate
object. It should not be merged conceptually with the low-action sigma-set
question:

- low-action set behavior asks how many returned branches are near the minimum
  and which one wins at target points;
- fixed-sigma sysext behavior asks whether an individual KKT branch has a
  stable raw critical point, how its action changes, and whether beta margins
  approach the domain boundary.

For this fixture and line, the low-action candidate-window model was highly
predictive and the fixed-sigma sysext branches behaved smoothly. That does not
yet test beta-invalid raw sysext branches.

Raw sysext bucket run:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sysext-sigma-line-highdeg-raw \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy \
  --steps -1e-3,-3e-4,-1e-4,0,1e-4,3e-4,1e-3 \
  --max-raw-sysext-per-bucket 3 \
  --raw-action-window-relative 0.05
```

Result:

- rows: 119;
- statuses: all `ok`;
- elapsed time: `327ms` excluding release compile;
- three `raw_invalid_near_boundary_*` sigmas had beta margins between about
  `-0.00421` and `-0.00187` over the line and stayed beta-invalid;
- three `raw_low_action_*` sigmas had tiny action values but beta margins
  around `-250` to `-400`, so they are numerically smooth but mathematically
  irrelevant for `sys`;
- this reproduces the scratch failure mode: raw all-sysext lower envelopes need
  beta gating, because far-invalid branches can dominate value comparisons.

For the checked line, a conservative optimizer-facing policy is:

```text
Use returned admissible low-action branches for prediction.
Use beta-invalid sysext branches only in offline diagnostics unless a local
line probe shows beta_margin approaching zero within the contemplated radius.
Do not let far-invalid raw branches constrain direction choice.
```

## Fixed-Sigma Prediction Error

The line probe now also separates fixed-sigma linearization error from full
lower-envelope/window error.

Cheap action-only run:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sysext-fixed-action-error-highdeg \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy \
  --steps -1e-2,-3e-3,-1e-3,-3e-4,-1e-4,0,1e-4,3e-4,1e-3,3e-3,1e-2 \
  --max-raw-sysext-per-bucket 3 \
  --raw-action-window-relative 0.05
```

This avoids target volume and target branch enumeration. It computes, for each
fixed sigma,

```text
predicted_action_sigma(a0 + t u)
  = action_sigma(a0) + t D action_sigma(a0)[u]

action_prediction_error
  = predicted_action_sigma(a0 + t u) - action_sigma(a0 + t u)
```

Result:

- rows: 187 total, 184 `ok`, 3 `nonpositive_q`;
- elapsed time: `315ms` excluding release compile;
- median absolute action error:
  - `1.83e-8` at `|t|=1e-4`;
  - `1.83e-6` at `|t|=1e-3`;
  - `1.8e-4` to `2.7e-4` at `|t|=1e-2`;
- max absolute action error:
  - about `1.0e-7` at `|t|=1e-4`;
  - about `1.0e-5` at `|t|=1e-3`;
  - about `1.0e-3` at `|t|=1e-2`.

This is the first clean evidence that fixed-branch KKT/action
linearization error is small and roughly second-order on this line. It does not
include target volume effects.

Tiny sys-value fixed-sigma run:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sysext-fixed-sys-error-highdeg-small \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy \
  --steps -1e-3,0,1e-3 \
  --max-raw-sysext-per-bucket 0 \
  --compute-target-sys-sigma
```

This computes target volume and therefore actual
`sys_sigma(a0 + t u)`. It is much more expensive: `30` rows took `29.2s`.

For the ten admissible low-action sigmas at `t = +/-1e-3`:

- median absolute fixed-sigma `sys` prediction error: about `1.6e-6`;
- max absolute fixed-sigma `sys` prediction error: about `5.34e-6`;
- max absolute action prediction error in the same rows: about `1.03e-5`.

So for this line and radius, the lower-envelope errors of order `5e-6` are
compatible with fixed-branch linearization error; this tiny sample does not
show a need to blame missing sigma-window effects.

## Three-Source Decomposition

The prediction-cloud producer now decomposes each lower-envelope prediction
error into:

```text
prediction_error
  = predicted_sys(a1) - actual_sys(a1)

linearization_error
  = base-window linear envelope at a1
    - base-window exact envelope at a1

sigma_set_error
  = base-window exact envelope at a1
    - true sys(a1)
```

The row also decomposes the predicted winning fixed branch into action and
volume pieces:

```text
action part:  sys(predicted_action, actual_volume) - sys(actual_action, actual_volume)
volume part:  sys(actual_action, predicted_volume) - sys(actual_action, actual_volume)
interaction: remaining nonlinear/cross term
```

Sanity check at `t = 1e-3`:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-decomp-highdeg-r1e-3 \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels high_degeneracy \
  --max-fixtures-per-label 1 \
  --steps 1e-3 \
  --trace-iterations 1
```

Result:

- rows: 5;
- max absolute total prediction error: `5.34e-6`;
- max absolute linearization error: `5.34e-6`;
- sigma-set error: `0` on all rows;
- sum residual: `0` on all rows.

Within the predicted winning branch:

- max absolute action part: `4.13e-6`;
- max absolute volume part: `4.57e-7`;
- max absolute interaction residual: `1.67e-6`.

So at `t=1e-3` on this basepoint, the observed lower-envelope error is explained
by fixed-branch linearization error, mostly action/capacity curvature rather
than volume curvature, with no observed sigma-window loss.

Sanity check at `t = 1e-2`:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-decomp-highdeg-r1e-2 \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels high_degeneracy \
  --max-fixtures-per-label 1 \
  --steps 1e-2 \
  --trace-iterations 1
```

Result:

- rows: 5;
- max absolute total prediction error: `1.33e-3`;
- max absolute linearization error: `5.23e-4`;
- max absolute sigma-set error: `1.38e-3`;
- sum residual: `0` on all rows.

The interesting row is the maximin direction:

```text
total prediction error:  +1.3319e-3
linearization error:     -4.3813e-5
sigma-set error:         +1.3757e-3
```

So at `t=1e-2`, the largest error in this small check is not fixed-branch
Taylor error; it is base-window/sigma-set error. The base-window exact envelope
is higher than true target `sys`, meaning a target winner outside the exact
base window became relevant.

## Sigmalow Count Audit

Cheap audit over the existing branch diagnostic:

```bash
jq -s 'map(select(.failure == null)) |
  {rows:length,
   by_threshold:(group_by(.threshold_relative) |
     map({threshold:.[0].threshold_relative,
          rows:length,
          min:(map(.near_active_count)|min),
          median:(map(.near_active_count)|sort|.[length/2|floor]),
          max:(map(.near_active_count)|max),
          avg:(map(.near_active_count)|add/length)}))}' \
  /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check/branch-set-diagnostic.jsonl
```

Result:

- threshold `1e-12`, `1e-9`, `1e-6`: median `1`, max `4`;
- threshold `1e-3`: median `2`, max `4`;
- threshold `1e-2`: median `4`, max `13`;
- random-sample rows specifically had average low-action count `1.2` and max
  `3`, while optimized/top-sys-style rows supplied the high-degeneracy examples.

This supports the current working hypothesis: generic random points usually
have small returned low-action sigma sets, while structured or optimized points
can have larger sets and are the important stress cases for optimizer design.

## Remaining Unknowns

The current packet is enough to guide the next optimizer iteration, but it is
not a broad statistical study.

Future prediction work, if reopened, should test:

- more fixtures and directions for the `sigmalow(a)` count hypothesis;
- target-best sigma sources directly in the sysext line probe;
- candidate-window prediction on optimizer trace endpoints where the optimizer
  actually stalls;
- beta-invalid sysext branches whose beta margin is much closer to zero than
  the checked `-0.002` to `-0.004` examples.

These are diagnostic-strengthening tasks, not blockers for returning to
optimizer design under the current layer-1 goal.
