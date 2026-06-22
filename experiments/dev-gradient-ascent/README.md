# Dev Gradient Ascent

Status: active top-level development packet for a heuristic gradient-ascent
method for `sys(a)`.

Corrected current scope: the newest work in this packet studies behavior of
`sys(a)` and HK branch data across local, semi-local, and effectively global
perturbation scales. Gradient ascent is one downstream consumer of that
understanding, not the only current organizing question. The package name
`dev-gradient-ascent` is historical for this broader branch-behavior work.

Start with [CHARTER.md](CHARTER.md) before adding method code or interpreting
outputs. The charter defines the objective, definition of done, evidence
standard, and question families. This README is the operational entry point.
The current method candidate is summarized in
[METHOD-CANDIDATE.md](METHOD-CANDIDATE.md). The current promotion decision
packet is [PROMOTION-READINESS.md](PROMOTION-READINESS.md).

This package is not thesis evidence by itself. It is the development surface
for producing object-level diagnostic results that can later be promoted into
`experiments/sys-landscape/`, `experiments/performance/`,
`experiments/verification/`, `experiments/dev-quadratic-program/numerics-audit/`, or a final analysis
packet.

## Thesis Role

The thesis-facing target is a positive method claim:

```text
We developed a gradient-ascent method for the nonsmooth high-dimensional
function sys(a), and reproducible theory-informed diagnostics support that the
method reaches heuristic local maxima within reasonable compute budgets.
```

For generic non-HKO endpoints this package does not aim for theorem-grade
local-maximality certificates. The intended local-maximality status is
heuristic and must be reported as such. HKO theorem-strength local maximality
has its own certificate surface under `experiments/hko-local-maximum/`.

## Core Question Families

The package is organized around five question families.

| Family | Why it matters |
| --- | --- |
| Degeneracy regime | The main local-geometry axis is how many sigmas are close to the minimum action. |
| Near-active branch selection | The ascent direction is unreliable if the branch set omits relevant near-minimum sigmas or includes too much noise. |
| Ridge/cusp step behavior | Narrow-gap states need multi-branch ascent rather than single-branch bouncing or coarse line-search stalling. |
| Endpoint local-stability diagnostic | The method should stop because no practical common ascent direction remains under the chosen diagnostic, not because a coarse step failed. |
| Compute budget by regime | The method must be feasible for downstream datascience reruns, not only for isolated examples. |

## Question Map

The durable question map lives in [CHARTER.md](CHARTER.md). The shorter copy
below is a navigation aid.

### Questions Answerable By A Complete Model

These questions describe what a mature local model of the method would make
answerable. The thesis does not need all answers.

- What degeneracy regime is a point in?
- Which sigmas, branch domains, or branch germs are relevant near the point?
- Over what radius does a local model predict `sys(a0 + t d)`?
- When is single-branch ascent enough?
- When is near-active multi-branch ascent needed?
- What does a heuristic local maximum look like on the quotient/transversal
  slice?
- How does convergence behavior vary with degeneracy?
- Which apparent failures are optimizer bugs, diagnostic bugs, branch-domain
  issues, or genuine geometry?

### Questions Useful To Answer Now

These questions are valuable because their answers can change method design,
diagnostics, rerun planning, or thesis wording.

- What known failure mode makes current ascent endpoints non-local-maximal?
- Can near-active multi-branch ascent climb narrow-gap ridges better than
  single-branch ascent?
- What tolerance policy gives stable near-active branch sets?
- Which adaptive, tiny, or finite step rules avoid ridge and boundary stalling?
- Can random-start ascent reach high-degeneracy endpoint candidates within
  reasonable budgets?
- Do endpoints pass the chosen local-stability diagnostic?
- What trace fields are needed to debug failures and support thesis claims?
- Does rerunning fixed-`F` datascience with the fixed method change the
  hostile-landscape result?

### Prioritized Overlap

Most paths to thesis success should prioritize questions that are both useful
now and part of a complete local model:

- degeneracy regime;
- near-active branch selection;
- ridge/cusp step behavior;
- endpoint local-stability diagnostic;
- compute budget by regime;
- failure classification.

## Artifact Types

Use object-level artifact names rather than vague evidence labels.

- **Run trace:** per-iteration optimizer state, branch set, direction, step,
  and stop reason.
- **Branch-set diagnostic:** action gaps, tolerance windows, and near-active
  sigma counts.
- **Branch cartography:** paired `(a0, data(a0))` and nearby
  `(a, data(a), relation_to_a0)` records, including whether target best sigmas
  were visible at `a0` or appeared through branch-domain changes.
- **Local geometry probe:** sampled behavior of `sys(a0 + t d)` near selected
  points.
- **Endpoint diagnostic:** whether a produced endpoint passes the chosen
  local-stability checks.
- **Compute-budget report:** runtime, exact-evaluation counts, failures, and
  scale behavior.
- **Method comparison:** controlled comparison between retained variants or
  ablations after a method exists.
- **Thesis-support packet:** cleaned reproducible subset of the above with
  scoped claims and caveats.

## Current Smoke Command

This command only validates the package shape and artifact schema. It uses
synthetic action spectra to exercise the five artifact classes. It does not
compute `sys(a)` and must not be cited as a method result.

```bash
cargo run -p exp-dev-gradient-ascent --bin dev-gradient-ascent-smoke -- \
  --out-dir /tmp/dev-gradient-ascent-smoke
```

Without `--out-dir`, the command writes to a unique directory under `/tmp`.

It writes:

- `run-trace.jsonl`
- `branch-set-diagnostic.jsonl`
- `local-geometry-probe.jsonl`
- `endpoint-diagnostic.jsonl`
- `compute-budget-report.json`
- `summary.json`

## Branch Degeneracy Diagnostic

This command is the first real-data diagnostic in the package. It reads the
retained sys-landscape datascience tables, selects a small deterministic row
panel, recomputes real capacity/orbit data, and records how many admissible
returned sigmas are close to the minimum action under several relative action
windows. It widens the orbit aggregation window to the largest requested
relative threshold, then reports counts at every requested threshold.

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-branch-diagnostic -- \
  --out-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --max-rows 8
```

Without `--out-dir`, the command writes to a unique directory under `/tmp`.

It writes:

- `fixture-selection.jsonl`
- `branch-set-diagnostic.jsonl`
- `compute-budget-report.json`
- `summary.json`

Current checked smoke observation from the setup pass:

- command: `--max-rows 8`;
- successful recomputations: `8`;
- failures: `0`;
- total orbit-search iterations: `33755`;
- elapsed wall time: about `96s` in the local devcontainer;
- maximum retained action window: `1e-2` relative;
- degeneracy labels across the threshold sweep:
  `large_gap = 26`, `narrow_gap = 11`, `high_degeneracy = 3`.

Interpretation: the command is a real diagnostic surface. The small retained
table sample already exposes both generic large-gap behavior and near-active
multi-branch cases, including one random-product row with four near-active
branches at `1e-12` relative tolerance. The next useful step is to connect this
branch-set diagnostic to local geometry probes or ascent traces, not to treat
these counts as endpoint local-maximality evidence.

## Branch Cartography

Start with [branch-cartography/README.md](branch-cartography/README.md) before
changing or interpreting this command. It records the corrected scope,
perturbation-scale model, run defaults, and caveats.

This command consumes a branch diagnostic output directory, selects classified
basepoints, and records point/sample pairs:

```text
(a0, data(a0), [(a, data(a), relation_to_a0)])
```

It evaluates branch-derived directions and optional deterministic random unit
directions at finite radii. For every successful sample, it records whether the
target best sigma was already in the base near-active set, was at least inside
the wider base candidate window, was blocked by base transitions, or used a
transition that opened at the target.

By default it samples one layer around each selected fixture. Use `--layers N`
to expand improving samples for `N` finite-step layers. Non-improving target
points are still recorded as point records, but they are not expanded.

Defaults: `--steps 1e-4,1e-3`, `--layers 1`, `--random-directions 2`,
`--selection-threshold-relative 1e-3`, `--action-window-relative 1e-2`,
`--max-fixtures-per-label 1`, and all three degeneracy labels. Classification
uses the raw sign of `observed_delta_sys`; there is no positive-delta tolerance
yet.

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-branch-cartography -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --out-dir /tmp/dev-gradient-ascent-branch-cartography-check \
  --steps 1e-4 \
  --max-fixtures-per-label 1 \
  --random-directions 1
```

Use `--selection-threshold-relative 0.01 --degeneracy-labels high_degeneracy`
to select the wide-window high-degeneracy rows from the checked allsafe branch
diagnostic output.

It writes:

- `fixture-selection.jsonl`
- `branch-cartography-points.jsonl`
- `branch-cartography-samples.jsonl`
- `compute-budget-report.json`
- `summary.json`

Current checked observations:

- output: `/tmp/dev-gradient-ascent-branch-cartography-check`;
- canonical regeneration: run the branch degeneracy diagnostic first, then run
  the branch-cartography command above; the recorded output directories are
  intentionally ephemeral `/tmp` smoke outputs;
- selection threshold: `1e-3`;
- selected fixtures: `2` (`large_gap = 1`, `narrow_gap = 1`);
- sample rows: `7`;
- failures: `0`;
- classifications:
  `improving_visible_near_active_branch = 3`,
  `non_improving_visible_near_active_branch = 4`;
- elapsed wall time: about `85s` in the local devcontainer.

High-degeneracy check:

- output: `/tmp/dev-gradient-ascent-branch-cartography-highdeg-check`;
- selection threshold: `1e-2`;
- selected fixtures: `1` (`high_degeneracy = 1`);
- sample rows: `4`;
- base near-active count: `6`;
- classifications:
  `improving_visible_near_active_branch = 2`,
  `non_improving_visible_near_active_branch = 2`;
- elapsed wall time: about `49s` in the local devcontainer.

Layer-expansion smoke:

- output: `/tmp/dev-gradient-ascent-branch-cartography-layer-check`;
- command shape:
  `--degeneracy-labels large_gap --steps 1e-4 --random-directions 0 --layers 2`;
- selected fixtures: `1`;
- point records: `5`;
- sample rows: `4`;
- source-state counts:
  `selected_fixture = 1`, `sample_target_layer_1 = 2`,
  `sample_target_layer_2 = 2`;
- classifications:
  `improving_visible_near_active_branch = 2`,
  `non_improving_visible_near_active_branch = 2`.

Interpretation: these first small-radius runs did not expose missing target
branches or transition-opened samples. They did show raw-sign positive finite
samples for the wide-window high-degeneracy fixture, so this is not endpoint
local-maximality evidence. The current command is an ascent-biased reference
surface: it expands only raw-sign improving samples and its default directions
are gradient/maximin plus random probes. Use it as early branch-behavior
evidence or as code to adapt into a datascience-shaped producer, not as an
unbiased cartography of all perturbations.

## Local Geometry Probe

This command consumes a branch diagnostic output directory, selects
representative classified basepoints, and evaluates finite `sys(a0 + t d)`
steps along branch-derived directions. It uses the branch diagnostic threshold
to choose the near-active set for gradients and maximin directions. A wider
action window can still be used internally to collect candidate orbits before
filtering.

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --out-dir /tmp/dev-gradient-ascent-local-geometry-probe-check \
  --steps 1e-3,1e-4 \
  --max-fixtures-per-label 1 \
  --trace-iterations 4 \
  --min-observed-delta 1e-3 \
  --min-observed-relative-delta 0
```

The trace accepts a step only when the recomputed `sys` improvement is above
the effective stop threshold:

```text
max(min_observed_delta, min_observed_relative_delta * abs(base_sys)).
```

Each trace and endpoint row records the absolute threshold, relative threshold,
and effective threshold that was applied.

By default, endpoint direction scans use the same step list as the trace. Use
`--endpoint-steps` to scan a different finite-step grid at the final trace
state without changing the trace path.

Use `--skip-fixtures-per-label` together with `--max-fixtures-per-label 1` to
split expensive retained-sample checks into one complete output directory per
deterministic fixture rank. Fixture ranks are computed after filtering by
degeneracy label and branch-selection threshold, then sorting by degeneracy
label, descending input `sys`, and `poly_id`. Each `fixture-selection.jsonl`
row records `selection_rank_within_label`.

It writes:

- `fixture-selection.jsonl`
- `local-geometry-probe.jsonl`
- `run-trace.jsonl`
- `endpoint-diagnostic.jsonl`
- `endpoint-direction-scan.jsonl`
- `compute-budget-report.json`
- `summary.json`

Current checked smoke observation from the setup pass, using the branch
diagnostic output described above:

- selected fixtures: `2` (`large_gap = 1`, `narrow_gap = 1`);
- probe rows: `10`;
- run-trace rows: `4`;
- endpoint-diagnostic rows: `2`;
- failures: `0`;
- elapsed wall time: about `188s` in the local devcontainer;
- large-gap fixture: one near-active branch at threshold `1e-3`;
- narrow-gap fixture: two near-active branches at threshold `1e-3`;
- the narrow-gap near-active maximin direction predicted
  `+1.505655890076855e-4` at step `1e-4` and recomputation observed
  `+1.508299892882814e-4`;
- the iterative trace chose the best predicted positive direction and tried
  candidate steps in order;
- stop threshold: `min_observed_delta = 1e-3`;
- trace stop-reason counts:
  `accepted_observed_delta_above_threshold = 2`,
  `line_search_all_steps_below_min_observed_delta = 2`;
- trace line-search status counts:
  `accepted = 2`, `all_steps_below_min_observed_delta = 2`;
- trace orbit-search iterations: `1983` for base states and `584` for target
  states;
- post-stop endpoint diagnostic status:
  `post_stop_positive_below_threshold = 2`;
- after the threshold-stopped trace, both selected fixtures still had a
  positive post-stop finite probe below the configured stop threshold
  (`post_stop_improvement_found = true` and
  `post_stop_threshold_improvement_found = false`);
- both post-stop endpoint rows still had a positive predicted maximin direction:
  the large-gap row predicted `+9.269269259770647e-5` and observed
  `+9.269753087948196e-5`, while the narrow-gap row predicted
  `+7.377043198471886e-5` and observed `+7.377663452090566e-5`;
- endpoint line-search status counts:
  `all_steps_below_min_observed_delta = 2`;
- endpoint diagnostic orbit-search iterations: `584` for base states and `0`
  for accepted target states above the configured stop threshold.

Interpretation: this is a finite local behavior probe connected to the
branch-set diagnostic. It shows that the current artifact surface can compare
single-branch and near-active maximin directions on retained real data, and can
record a bounded prediction-selected trace with explicit line-search fields
plus a post-stop improvement check. With `min_observed_delta = 1e-3`, the trace
does stop for a method reason rather than the iteration cap, but the post-stop
diagnostic still finds smaller positive improvements. This is a useful
stop-threshold failure signal, not a local-maximality result.

Additional checked relative-threshold smoke:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --out-dir /tmp/dev-gradient-ascent-local-geometry-relative-threshold-check \
  --steps 1e-3,1e-4 \
  --max-fixtures-per-label 1 \
  --degeneracy-labels large_gap \
  --trace-iterations 2 \
  --min-observed-relative-delta 1e-3
```

This selected one large-gap fixture. The first trace step had
`base_sys = 0.9605700102775944`, effective threshold
`0.0009605700102775944`, and observed improvement
`0.0024845334751826265`, so it was accepted. The next trace row had
`base_sys = 0.963054543752777`, effective threshold
`0.000963054543752777`, predicted improvement
`0.00009269269259770647`, and observed improvement
`0.00009269753087948196`, so it stopped below the relative threshold. The
post-stop endpoint row had status `post_stop_positive_below_threshold`.

The same run now writes `endpoint-direction-scan.jsonl`, which evaluates every
available post-stop direction returned by the local direction generator. In the
checked one-fixture run, the scan had `6` rows and no failures. Its threshold
outcome counts were `above_threshold = 1`, `positive_below_threshold = 3`, and
`nonpositive = 2`. The above-threshold row was
`post_stop_single_near_active_gradient` at step `1e-3`: it predicted
`+3.216828608895297e-4` but recomputation observed
`+1.8526383133251612e-3`, and the target best sigma was not in the base
near-active set. The prediction-selected endpoint diagnostic had only tested
the maximin direction and therefore missed this above-threshold finite
improvement.

Interpretation: this is a concrete current failure mode for the trace policy.
Choosing only the best predicted maximin direction can miss a finite
single-branch step that changes the active branch set and improves `sys` above
the configured threshold.

Current checked observed multi-direction trace result:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --out-dir /tmp/dev-gradient-ascent-observed-multidir-large-gap-check \
  --steps 1e-3,1e-4 \
  --max-fixtures-per-label 1 \
  --degeneracy-labels large_gap \
  --trace-iterations 8 \
  --min-observed-relative-delta 1e-3
```

The trace now tries all generated directions in descending predicted derivative
order and accepts only after recomputed `sys` clears the effective threshold.
On the same large-gap fixture, it accepted four above-threshold steps and then
stopped on iteration `4`. At iterations `1` and `3`,
`near_active_maximin_direction` was tried first and rejected, then
`single_near_active_gradient` was accepted. The endpoint direction scan after
the stop had `above_threshold = 0`, `positive_below_threshold = 4`, and
`nonpositive = 2`.

Interpretation: this fixes the observed large-gap branch-switching miss for
this fixture. It is not endpoint local-maximality evidence yet: the result is
one large-gap fixture, and the endpoint scan still finds smaller positive
finite improvements below the relative threshold.

Additional current-method regime checks:

- Narrow-gap, default selection threshold `1e-3`, output
  `/tmp/dev-gradient-ascent-observed-multidir-narrow-gap-check`: one accepted
  step, then a method stop. Endpoint scan:
  `above_threshold = 0`, `positive_below_threshold = 3`, `nonpositive = 3`.
- High-degeneracy, selection threshold `0.01`, output
  `/tmp/dev-gradient-ascent-observed-multidir-high-degeneracy-check`: the trace
  rejected `near_active_maximin_direction`, then accepted
  `single_near_active_gradient` even though its predicted delta was negative
  (`-8.990368639757637e-4`) because recomputation observed
  `+1.4962061638912338e-3`. It then stopped. Endpoint scan:
  `above_threshold = 0`, `positive_below_threshold = 3`, `nonpositive = 3`.

Interpretation: the observed multi-direction policy fixed the checked
large-gap and high-degeneracy above-threshold misses in this fixture panel.
The remaining positive-below-threshold rows are still endpoint-condition
information, not local-maximality evidence.

Checked smaller-step endpoint scan panel:

```bash
--steps 1e-3,1e-4 --endpoint-steps 1e-3,1e-4,1e-5,1e-6
```

Outputs:

- `/tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check`
- `/tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check`
- `/tmp/dev-gradient-ascent-endpoint-smallsteps-high-degeneracy-check`

All three one-fixture runs still had `above_threshold = 0` in
`endpoint_direction_scan_threshold_counts`. With the four-step endpoint grid,
large-gap had `positive_below_threshold = 8`, `nonpositive = 4`; narrow-gap
and high-degeneracy each had `positive_below_threshold = 7`, `nonpositive = 5`.
The maximum positive observed deltas by endpoint step were:

| Regime | `1e-3` | `1e-4` | `1e-5` | `1e-6` |
| --- | ---: | ---: | ---: | ---: |
| large-gap | `5.629088200068688e-4` | `1.7021117537174835e-4` | `1.861537333058827e-5` | `1.8774892956985312e-6` |
| narrow-gap | `7.383249465420239e-4` | `1.7099324647518177e-4` | `1.7097519367736957e-5` | `1.7097338853577781e-6` |
| high-degeneracy | `6.160232262941712e-4` | `1.7099324647518177e-4` | `1.7097519367736957e-5` | `1.7097338853577781e-6` |

Interpretation: in this fixture panel, smaller endpoint steps do not reveal an
above-threshold improvement. The positive rows shrink with step size and remain
well below the relative stop threshold. This is still a finite direction/step
panel, not a certificate that the endpoints are local maxima.

Broadened retained-sample checks should currently be run as split fixture
commands rather than one large local chat-turn command. The selector supports
commands of the following shape:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --out-dir /tmp/dev-gradient-ascent-large-gap-rank-1-check \
  --steps 1e-3,1e-4 \
  --endpoint-steps 1e-3,1e-4,1e-5,1e-6 \
  --max-fixtures-per-label 1 \
  --skip-fixtures-per-label 1 \
  --degeneracy-labels large_gap \
  --trace-iterations 8 \
  --min-observed-relative-delta 1e-3
```

This command selects the second eligible `large_gap` fixture under the current
deterministic selector. The run is still expensive, but partial broadened
checks now survive as complete per-fixture artifact directories.

Split run summaries can be aggregated without replaying expensive geometry:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-aggregate-summaries -- \
  --out-dir /tmp/dev-gradient-ascent-broadened-aggregate \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check \
  /tmp/dev-gradient-ascent-large-gap-rank-1-check
```

It writes:

- `run-summary.jsonl`
- `budget-summary.jsonl`
- `aggregate-summary.json`

The aggregate command combines `summary.json` counts and, when present,
`compute-budget-report.json` runtime/orbit-iteration totals from complete run
directories. It does not inspect trace rows, rerun `sys(a)`, or certify
endpoint local maximality. Per-regime budget totals are attributed to a
degeneracy label only when the run summary has exactly one nonzero degeneracy
label; mixed runs are placed in `mixed_or_unknown`.

Current checked aggregate from the setup pass:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-aggregate-v3`;
- selected fixtures: `6`;
- degeneracy counts:
  `large_gap = 2`, `narrow_gap = 2`, `high_degeneracy = 2`;
- endpoint scan threshold counts:
  `above_threshold = 0`, `positive_below_threshold = 46`,
  `nonpositive = 26`;
- failed probe rows: `0`;
- failed endpoint scan rows: `0`;
- aggregate wall time from compute-budget reports: about `1679s`.

Interpretation: this is a small retained finite-diagnostic panel. The
endpoint-direction scan, not the prediction-selected endpoint diagnostic
status, is the artifact that supports the current finite endpoint condition at
the configured threshold. It is not a local-maximum certificate.

Endpoint scan magnitudes can be summarized with:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-endpoint-scan-report -- \
  --out-dir /tmp/dev-gradient-ascent-current-retained-panel-endpoint-scan-report \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check \
  /tmp/dev-gradient-ascent-full-large-gap-rank-1-check \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check \
  /tmp/dev-gradient-ascent-full-narrow-gap-rank-1-check \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-high-degeneracy-check \
  /tmp/dev-gradient-ascent-full-high-degeneracy-rank-1-check
```

It writes:

- `run-endpoint-scan-report.jsonl`
- `endpoint-scan-summary.json`

Unlike `dev-gradient-ascent-aggregate-summaries`, this command does inspect
`endpoint-direction-scan.jsonl`. It reports the largest remaining positive
finite endpoint improvement and the largest ratio to the effective endpoint
threshold.

Current checked endpoint scan report from the setup pass:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-endpoint-scan-report`;
- scanned runs: `6`;
- scanned endpoint rows: `72`;
- above-threshold rows: `0`;
- positive-below-threshold rows: `46`;
- nonpositive rows: `26`;
- largest positive observed endpoint delta:
  `7.383249465420239e-4`;
- largest ratio to the effective threshold: about `0.756`;
- row attaining both maxima: the narrow-gap run
  `/tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check`,
  direction `post_stop_near_active_maximin_direction`, step `1e-3`.

Run trace behavior can be summarized with:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-run-trace-report -- \
  --out-dir /tmp/dev-gradient-ascent-current-retained-panel-run-trace-report \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-large-gap-check \
  /tmp/dev-gradient-ascent-full-large-gap-rank-1-check \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-narrow-gap-check \
  /tmp/dev-gradient-ascent-full-narrow-gap-rank-1-check \
  /tmp/dev-gradient-ascent-endpoint-smallsteps-high-degeneracy-check \
  /tmp/dev-gradient-ascent-full-high-degeneracy-rank-1-check
```

It writes:

- `run-trace-report.jsonl`
- `run-trace-summary.json`

Current checked run trace report from the setup pass:

- output: `/tmp/dev-gradient-ascent-current-retained-panel-run-trace-report`;
- trace rows: `22`;
- accepted rows: `16`;
- method-stop rows: `6`;
- accepted rows with negative predicted delta: `5`;
- accepted rows after at least one earlier direction was rejected: `7`;
- accepted rows with positive observed delta but negative predicted delta:
  `5`;
- all accepted rows used `single_near_active_gradient`;
- per-regime accepted negative-predicted rows:
  `large_gap = 0`, `narrow_gap = 0`, `high_degeneracy = 5`.

Interpretation: in this panel, the rule that tests directions despite a
negative branch prediction is necessary for the checked high-degeneracy
improvements. The rule that keeps trying directions after an earlier direction
fails also changes trace behavior in the checked large-gap and high-degeneracy
runs.

## Trace Policy Sweep

This command consumes an existing `dev-gradient-ascent-local-geometry-probe`
output directory and reclassifies already-observed trace and endpoint deltas
under a grid of stop-threshold policies. It does not replay optimizer paths and
does not recompute `sys(a)`.

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-trace-policy-sweep -- \
  --geometry-dir /tmp/dev-gradient-ascent-local-geometry-predicted-stop-check \
  --out-dir /tmp/dev-gradient-ascent-trace-policy-sweep-check \
  --absolute-thresholds 0,1e-4,1e-3 \
  --relative-thresholds 0,1e-3
```

It writes:

- `trace-policy-sweep.jsonl`
- `summary.json`

Checked observation on the two-fixture geometry output above:

- policies: `6`;
- run-trace rows reclassified: `4`;
- endpoint rows reclassified: `2`;
- sweep rows: `24`;
- with zero threshold, both endpoint rows are above threshold because both
  post-stop deltas are positive;
- with every nonzero tested policy (`1e-4`, `1e-3`, relative `1e-3`, and
  combinations), both endpoint rows are `positive_below_threshold`.

Interpretation: this quickly separates threshold-choice questions from
expensive geometry recomputation. It is not a replacement for rerunning the
trace under a different policy, because a different accepted/rejected decision
would change later basepoints.

## Downstream Boundaries

- Promote stable reusable algorithm pieces into `exp-sys-landscape` helpers or
  durable crate code only after the development surface has identified the
  retained method.
- Rerun fixed-`F` datascience producers with the fixed method before thesis
  wording relies on ascent endpoint data.
- Keep active-development clutter out of the final thesis evidence packet.
- Put runtime-only measurements in `experiments/performance/`.
- Put correctness/regression checks in `experiments/verification/`.
- Put derivative and numerical-error work in `experiments/dev-quadratic-program/numerics-audit/`.

## Retired Nested Stub

The old `experiments/sys-landscape/gradient-ascent-dev/` placeholder surface
and its stub binaries were removed. New gradient-ascent and branch-behavior
method development starts in this package.
