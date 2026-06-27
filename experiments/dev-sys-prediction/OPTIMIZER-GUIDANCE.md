# Optimizer Guidance From Dev Sys Prediction

Status: current guidance from the first prediction-cloud and sysext-line
evidence. This is enough to steer the next optimizer pass, not a theorem about
all polytopes.

## Classification

For the current layer-1 optimizer goal, prediction should be used as:

```text
offline diagnostics plus lightweight optimizer instrumentation
```

Do not put the full prediction cloud inside the optimizer loop. Recomputing
actual `sys(a0 + t u)` for each sampled direction costs about `30s` for one
fixture and five directions in the current AllSafe/exact-volume route.

Do put candidate-window branch predictions into optimizer traces, because they
are cheap once the base branch window and gradients are already computed.

## What Worked

Candidate-window lower-envelope prediction:

```text
min_sigma (base_gap_sigma + <grad sys_sigma(a0), t u>)
```

worked on the checked high-degeneracy fixture at both `t = 1e-4` and
`t = 1e-3`. It matched observed direction ranking on the tested policy, random,
and angled directions. Max absolute error was about `5.3e-8` at `1e-4` and
`5.3e-6` at `1e-3`.

This is the useful model to carry into optimizer instrumentation.

## What Not To Infer

The prediction-cloud producer still emits copied debug columns for a no-gap
near-active calculation over a thresholded branch window. Ignore those columns
for research interpretation. Nobody proposed ignoring base gaps, and their
failure should not be counted as evidence for the prediction question.

Raw all-sysext lower envelopes should not constrain optimizer directions.
Fixed-sigma probes found raw branches with tiny action values and beta margins
around `-250` to `-400`; these are smooth raw critical points but irrelevant to
the beta-positive `sys` minimum.

## Direction Policy

The optimizer should test at least:

- first active gradient direction;
- near-active maximin direction when several actual minimizers are tied;
- candidate-window lower-envelope direction when branch gaps are small;
- one or more angled perturbations of the best predicted direction;
- occasional random directions as a diagnostic, not necessarily as the main
  policy.

The prediction-cloud evidence did not show random/angled directions beating the
best candidate-window-ranked direction on the checked fixture. The current
evidence only supports using those directions as a finite cloud to test and
rank with the base-gap lower envelope.

## Step Policy

Use candidate-window predictions to rank a small radius grid, then verify with
actual recomputed `sys` line search. Do not trust the prediction as acceptance
proof.

The current decomposition supports this trust-region policy:

- at `t = 1e-3` on the checked high-degeneracy fixture, the full prediction
  error was explained by fixed-branch linearization error; the base candidate
  window still contained the target minimizer;
- at `t = 1e-2`, one tested direction had a much larger error dominated by
  sigma-window loss: the exact envelope over the base candidate window was
  higher than the true target `sys`.

So the optimizer should distinguish "Taylor error inside the known window" from
"target winner outside the known window". The former argues for smaller steps
or second-order correction; the latter argues for recomputing/expanding the
branch window and treating the current radius as outside the single-anchor
model's reliable region.

Large-step stress checks at `t = 1e-1` and `t = 1e0` reinforce this. By
`t = 1e-1`, one tested direction already failed target polytope construction
and all valid targets decreased `sys`. By `t = 1e0`, total prediction errors
were order one and the fixed winning branch action/volume split was unavailable
on most valid rows. These radii are useful as rejection/trust-region evidence,
not as normal local prediction samples.

Trace fields to record:

- base best sigma;
- candidate-window branch count;
- predicted lower-envelope delta for every attempted direction/radius;
- observed delta after recomputed `sys`;
- target best sigma;
- whether target best sigma was visible in the base candidate window;
- near-active count at base and target;
- min beta margin in returned branches.
- when running offline audits, the decomposition fields
  `decomposition_linearization_error`, `decomposition_sigma_set_error`, and
  `decomposition_sum_residual`.

## Sysext Policy

Use raw `sysext_sigma` only as an offline diagnostic or as a cheap line probe
for a small selected set of sigmas.

For cheap source-error decomposition, first use fixed-sigma action prediction
error:

```text
action_sigma(a0) + t D action_sigma(a0)[u] - action_sigma(a0 + t u)
```

This avoids target branch enumeration and target volume. Computing actual
`sys_sigma(a0 + t u)` additionally needs target volume; in the current exact
volume path that is much slower and should be sampled sparsely.

Default beta policy for optimizer-facing direction constraints:

```text
admissible returned low-action branches only
```

Optional diagnostic policy:

```text
include beta-invalid raw branches only if a fixed-sigma line probe shows
beta_margin approaching zero inside the contemplated step radius
```

For the checked high-degeneracy line, near-boundary invalid branches stayed
invalid over `[-1e-3, 1e-3]`, while far-invalid low-action raw branches were
obviously irrelevant. This argues against negative beta-margin sysext branches
inside the default optimizer loop.

## Single-Anchor Versus History

Single-anchor candidate-window prediction is useful enough to instrument next.
History-aware or point-cloud surrogates should be deferred.

Reason: the candidate-window single-anchor model already predicted the finite
direction ranking correctly on the checked hard fixture, while the cost
bottleneck came from recomputing actual `sys` for validation.

Reopen history/point-cloud models only if optimizer traces show repeated cases
where:

- target best sigma was not visible in the base candidate window;
- candidate-window prediction ranks directions incorrectly at useful radii;
- or finite accepted steps systematically exceed the local branch-linear trust
  region.

## Current Closure Decision

Return to `experiments/dev-gradient-ascent/` with this design:

```text
Use candidate-window lower-envelope prediction as optimizer instrumentation and
direction/radius ranking aid. Keep prediction-cloud and sysext-line probes as
offline diagnostics. Do not make full prediction-cloud recomputation part of
the hot optimizer loop.
```

Remaining prediction work is future diagnostic strengthening, not a blocker for
the next optimizer iteration.
