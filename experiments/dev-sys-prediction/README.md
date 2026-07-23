# Dev Sys Prediction

Status: active development packet for semi-local prediction of `sys(a)`.

Current producer usage is in `produce/README.md`.
Current guidance back to optimizer work is in `OPTIMIZER-GUIDANCE.md`.
Facet-count radius and baseline-error calibration is in
`facet-scale-baseline-error/README.md`.

This package is separate from `experiments/dev-gradient-ascent/`. Gradient
ascent work asks how to choose steps that reach good local maxima. Work here
asks how well predictive local models forecast `sys(a0 + da)` on a useful
neighborhood of `a0`, before deciding which optimizer policy should consume
that forecast.

The current motivating problem is branch switching near lower-dimensional
cells. At a point `a0`, a branch can be inactive but close enough that a finite
step crosses into a region where it becomes the minimum. A purely active-branch
gradient model can miss this. The checked calibration rows currently use the
admissible candidate-window version of the semi-local lower-envelope model,
which keeps base branch gaps:

```text
delta_model(d)
  = min_sigma (sys_sigma(a0) - sys(a0) + <grad sys_sigma(a0), d>)
```

where the candidate sigma set is a finite window around the current minimum.
Raw `sysext_sigma` lower envelopes are related diagnostics, not the main
checked calibration surface in the current facet-scale packet.

## Thesis Role

The thesis-facing value is methodological clarity for hostile `sys` search:

```text
We can distinguish optimizer failures from local-model failures by measuring
how well branch lower-envelope models predict finite `sys` changes.
```

This packet is not thesis evidence by itself. Promote cleaned results into the
owning thesis-support or datascience packet only after the prediction question,
fixtures, and caveats are stable.

## Scope

Work kept in this package includes:

- semi-local prediction of `sys(a0 + da)` over finite direction/radius clouds;
- lower-envelope models with base branch gaps;
- candidate-window completeness diagnostics;
- branch-crossing distance estimates;
- `sysext` branch-domain diagnostics, including beta margins and raw KKT
  critical points;
- model-error measurement against recomputed actual `sys`;
- compact fixture panels and smoke producers for prediction diagnostics.

Outside this package:

- optimizer endpoint claims or local-maximum claims;
- random-start ascent policies;
- retained hostile-search reruns;
- reusable QP/KKT route development except for packet-local copy-edited
  instrumentation.

The local equality-set sampler and its retained nonregular pentagon-product
control live in `branch-equality-continuation/`. That control shows that normal
Newton correction can reliably hit a fixed pairwise branch-equality set, while
full capacity recomputation is still needed because many equality points are
undercut by a third branch. It is a method result, not a new high-`sys` result.

Route optimizer work back to `experiments/dev-gradient-ascent/`. Route reusable
KKT solver/API design back to `experiments/dev-quadratic-program/` or
`crates/symplectic/` once it is stable enough to stop being packet-local.

## Current Directory Inventory

This table covers every immediate child directory. It is a physical search
surface, not a claim that every directory contains retained evidence.

| Directory | What is there |
| --- | --- |
| `src/` | shared panel, schema, cache, prediction-cloud, and beta-boundary implementation |
| `produce/` | deterministic panel configs, producers, commands, and data-flow documentation |
| `facet-scale-baseline-error/` | retained facet-count/radius and lower-envelope error calibration |
| `error-model-smoke/` | three-row analyzer-development fixture; not evidence |
| `branch-equality-continuation/` | fixed pairwise branch-equality continuation and nonregular-product control |
| [`sysext-beta-boundary-scan/`](sysext-beta-boundary-scan/README.md) | standalone raw KKT beta-margin scan; its panel-producer counterpart is copy-edited rather than imported |
| [`sysext-sigma-line-probe/`](sysext-sigma-line-probe/README.md) | fixed-sigma raw-KKT branch behavior along finite lines; no retained canonical output |

## Current Model Families

### Active Smooth Branch

Use one branch gradient when `sigmaset(a0) = {sigma}` and no near branch is
capacity-relevant at the planned step radius.

This is the smooth-cell model. It is expected to fail near branch crossings.

### Active Maximin

Use gradients of currently minimizing branches:

```text
maximize_d min_{sigma in sigmaset(a0)} <grad sys_sigma(a0), d>
```

This is the local nonsmooth model at exact or numerical ties. It ignores
positive base gaps, so it is not the full semi-local model near a tie.

### Candidate-Window Lower Envelope

Use candidate branches with base gaps:

```text
delta_model(d)
  = min_sigma (sys_sigma(a0) - sys(a0) + <grad sys_sigma(a0), d>)
```

Choose directions inside a finite trust region and evaluate actual recomputed
`sys` on a step grid. This model predicts branch switching without trying to
stay inside the current cell.

The natural nonlinear fixed-window comparator is not a new model: evaluate the
same base candidate sigmas at the target and take
`min_sigma sys_sigma(a0 + d)`. The decomposition reports this as the
base-window exact envelope. Its remaining error against true `sys(a0 + d)` is
the window-miss error: the amount explained by a target minimizer outside the
base candidate window.

### Sysext Lower Envelope

Use raw KKT critical branches, including beta-invalid branches, when they are
near enough to beta-validity to be plausible near-future bottlenecks.

The unretained checked-run summary in
[`OPTIMIZER-GUIDANCE.md`](OPTIMIZER-GUIDANCE.md) records that including all
beta-invalid critical branches is destructive: far-invalid branches can
dominate value comparisons in irrelevant ways. Regenerate the
[fixed-sigma line probe](sysext-sigma-line-probe/README.md) before treating
that summary as evidence. A beta-margin filter or smooth beta gate is required
before `sysext` branches should constrain directions.

## First Experiments

### Direction Cloud Prediction

For selected basepoints, sample finite directions:

- policy directions from the optimizer packet;
- random unit directions;
- small angular perturbations of policy directions;
- combinations of policy directions;
- directions suggested by branch-crossing estimates.

For each direction `u` and step `t`, record:

```text
actual_delta_sys = sys(a0 + t u) - sys(a0)
predicted_delta_lower_envelope
target_best_sigma
target_best_sigma_visible_in_base_near_active_set
target_best_sigma_visible_in_base_candidate_window
predicted best-versus-second lower-envelope gap
model_error = predicted_delta_lower_envelope - actual_delta_sys
```

Beta margins for the predicted winner and target winner are still desired
future trace fields; they are not part of the current prediction-cloud row
schema.

In generated local-geometry rows, use
`candidate_window_predicted_delta_sys` for this model. The
`direction_model_predicted_delta_sys` field records whichever direction model
was used to propose the row; the older alias `predicted_delta_sys` is retained
for compatibility and should not be read as the candidate-window prediction.

This answers whether current optimizer failures are direction-limited or
step-radius-limited.

### Candidate Completeness

For each accepted finite step, check whether the target best sigma was already
visible in the base candidate window. If not, record the source:

- branch was beta-invalid but sysext-visible at base;
- branch was raw-KKT singular or numerically unstable at base;
- branch was outside the action window;
- branch was not visited by the candidate enumeration route.

Related empirical hypothesis:

```text
For random a0, random direction da, and random moderate scale t, the set
sigmalow(a0 + t da) = {sigma : action_sigma <= min_action * (1 + threshold)}
is usually small.
```

This is plausible for generic random samples because near action ties should be
rare, but it can fail at structured product-like points or optimizer endpoints.
It should be measured independently from raw `sysext_sigma` behavior: a small
returned low-action set can coexist with many beta-invalid raw KKT critical
branches.

### Beta-Domain Boundary

Measure whether raw `sysext` branches with small or negative beta margin become
actual admissible minimizers after finite steps.

The first useful axis is a beta-margin sweep:

```text
beta_margin >= 0
beta_margin >= -1e-4
beta_margin >= -1e-3
beta_margin >= -1e-2
beta_margin >= -1e-1
all raw sysext
```

The expected failure mode is that too-wide beta windows include irrelevant
far-invalid branches and destroy lower-envelope directions.

## Artifact Types

- **Prediction cloud:** rows for `(basepoint, direction, step)` with predicted
  and observed deltas.
- **Branch-window diagnostic:** base candidate sets, action gaps, beta margins,
  raw KKT statuses, and returned admissible branches.
- **Crossing estimate audit:** pairwise branch-crossing estimates along tested
  directions and whether observed target minimizers match.
- **Model-error report:** aggregate error by radius, degeneracy regime,
  candidate-window policy, and beta-margin policy.
- **Fixture panel:** compact deterministic basepoint table so prediction
  experiments do not need to parse the full retained datascience table.

## Evidence locations

The path formerly advertised as
`experiments/dev-gradient-ascent/syssmooth-sprint/SPRINT-RESULTS.md` is absent
from the current tree and all current Git refs. Do not search for it as the
authoritative source or cite it as durable evidence.

Current retained prediction evidence lives in
[`facet-scale-baseline-error/`](facet-scale-baseline-error/README.md) and
[`branch-equality-continuation/`](branch-equality-continuation/README.md).
[`OPTIMIZER-GUIDANCE.md`](OPTIMIZER-GUIDANCE.md) also preserves
optimizer-facing conclusions from earlier scratch runs, including the
fixed-sigma sysext line probe, but those claims are only independently
checkable where a retained packet supplies their underlying artifacts.

## Initial Hypotheses

- Candidate-window lower-envelope is better than the active box-LP-normalized
  maximin heuristic near high-degeneracy branch switching, but overconstrains
  large-gap points.
- Value-only soft-min can produce strong first jumps but can stall after branch
  set changes.
- Raw all-sysext lower-envelope is too conservative because far beta-invalid
  branches dominate the LP.
- Near-domain sysext filtering can improve lower-envelope directions when a
  beta-invalid branch is close enough to become relevant after a finite step.
- Optimizer step policies should be chosen after measuring finite direction
  clouds, not only by infinitesimal policy-direction scores.

## Smoke Boundary

Use the config-driven panel runner for smoke and larger local characterization.
Current producer commands and config fields are documented in
`produce/README.md`.

Fast analyzer-only smoke check:

```bash
python3 experiments/dev-sys-prediction/analyze_prediction_error_model.py \
  --prediction-cloud experiments/dev-sys-prediction/error-model-smoke/prediction-cloud-smoke.jsonl \
  --out-dir /tmp/dev-sys-prediction-error-model-smoke
```

The smoke fixture is for development coverage of the analysis script only. It
is not evidence for the prediction question.
