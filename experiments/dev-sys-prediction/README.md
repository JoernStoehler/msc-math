# Dev Sys Prediction

Status: active development packet for semi-local prediction of `sys(a)`.

This packet is separate from `experiments/dev-gradient-ascent/`. The gradient
ascent packet owns optimizer behavior: how to choose steps that reach good
local maxima. This packet owns predictive local models: how well we can
forecast `sys(a0 + da)` on a useful neighborhood of `a0`, before deciding which
optimizer policy should consume that forecast.

The current motivating problem is branch switching near lower-dimensional
cells. At a point `a0`, a branch can be inactive but close enough that a finite
step crosses into a region where it becomes the minimum. A purely active-branch
gradient model can miss this. A semi-local lower-envelope model keeps base
branch gaps:

```text
sys(a0 + da) ~= min_sigma (sysext_sigma(a0) + <grad sysext_sigma(a0), da>)
```

where the candidate sigma set is a finite window around the current minimum.

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

This packet owns:

- semi-local prediction of `sys(a0 + da)` over finite direction/radius clouds;
- lower-envelope models with base branch gaps;
- candidate-window completeness diagnostics;
- branch-crossing distance estimates;
- `sysext` branch-domain diagnostics, including beta margins and raw KKT
  critical points;
- model-error measurement against recomputed actual `sys`;
- compact fixture panels and smoke producers for prediction diagnostics.

This packet does not own:

- optimizer endpoint claims or local-maximum claims;
- random-start ascent policies;
- retained hostile-search reruns;
- reusable QP/KKT route development except for packet-local copy-edited
  instrumentation.

Route optimizer work back to `experiments/dev-gradient-ascent/`. Route reusable
KKT solver/API design back to `experiments/dev-quadratic-program/` or
`crates/symplectic/` once it is stable enough to stop being packet-local.

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

### Sysext Lower Envelope

Use raw KKT critical branches, including beta-invalid branches, when they are
near enough to beta-validity to be plausible near-future bottlenecks.

Scratch evidence from `experiments/dev-gradient-ascent/syssmooth-sprint/`
showed that including all beta-invalid critical branches is destructive:
far-invalid branches can dominate the LP in irrelevant ways. A beta-margin
filter or smooth beta gate is required before `sysext` branches should constrain
directions.

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
target_best_sigma_visible_at_base
base/target beta margins
model_error = actual_delta_sys - predicted_delta_lower_envelope
```

This answers whether current optimizer failures are direction-limited or
step-radius-limited.

### Candidate Completeness

For each accepted finite step, check whether the target best sigma was already
visible in the base candidate window. If not, record the source:

- branch was beta-invalid but sysext-visible at base;
- branch was raw-KKT singular or numerically unstable at base;
- branch was outside the action window;
- branch was not visited by the candidate enumeration route.

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

## Current Scratch Inputs

The exploratory evidence that motivated this packet currently lives in the
scratch worktree branch `syssmooth-sprint`, especially:

```text
experiments/dev-gradient-ascent/syssmooth-sprint/SPRINT-RESULTS.md
```

Do not cite that scratch packet as durable evidence. Its role is to seed this
packet with failure modes and initial hypotheses.

## Initial Hypotheses

- Candidate-window lower-envelope is better than active maximin near
  high-degeneracy branch switching, but overconstrains large-gap points.
- Value-only soft-min can produce strong first jumps but can stall after branch
  set changes.
- Raw all-sysext lower-envelope is too conservative because far beta-invalid
  branches dominate the LP.
- Near-domain sysext filtering can improve lower-envelope directions when a
  beta-invalid branch is close enough to become relevant after a finite step.
- Optimizer step policies should be chosen after measuring finite direction
  clouds, not only by infinitesimal policy-direction scores.

## Smoke Boundary

A first mergeable producer should be small:

- one compact fixture panel;
- one direction cloud around each fixture;
- one or two radius grids;
- JSONL outputs under `/tmp` by default;
- no LICCA dependency.

Broader retained-sample or LICCA runs should wait until the schema and
interpretation of the smoke output are stable.
