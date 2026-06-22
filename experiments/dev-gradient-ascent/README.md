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

Inspect `summary.json` and `branch-set-diagnostic.jsonl` after regeneration for
successful recomputation counts, orbit-search iterations, and degeneracy labels.
These counts are branch-set diagnostics, not endpoint local-maximality evidence.

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

Regenerate branch-cartography outputs before quoting selected fixture counts,
sample counts, classifications, or wall time. The command is an ascent-biased
reference
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

Use `--direction-model near-active` for the original near-active first-order
score. Use `--direction-model candidate-window` to order generated directions
by the best predicted finite step under the minimum over base candidate-window
branch models with base branch gaps included; the runner still tests the
configured step list in input order for the chosen direction. Add
`--include-candidate-window-directions` only for the candidate-window maximin
direction experiment; those step-indexed directions are tested only at the
finite step used to generate them.

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --out-dir /tmp/dev-gradient-ascent-local-geometry-probe-check \
  --steps 1e-3,1e-4 \
  --max-fixtures-per-label 1 \
  --trace-iterations 4 \
  --min-observed-delta 1e-3 \
  --min-observed-relative-delta 0 \
  --direction-model near-active
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

With `--write-step-ranking-audit`, it also writes
`step-ranking-audit.jsonl`. Use that flag only for small hard panels. The audit
exhaustively recomputes every generated direction/step pair at each traced
state and ranks the same finite moves by near-active and candidate-window
predictions. Candidate-window rows include the branch/orbit witness attaining
the lower-envelope prediction. This is for deciding whether a first-order model
guides observed moves; it is not a normal broad-panel option and it is not the
same as the runner's direction-first line search.

Regenerate this probe before quoting fixture counts, prediction/observation
pairs, trace stop reasons, endpoint statuses, or compute cost. The artifact
surface can compare single-branch and near-active maximin directions, record a
bounded prediction-selected trace, and write post-stop endpoint checks. Those
checks are finite diagnostics, not local-maximality certificates.

Relative-threshold smoke command:

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

This path writes `endpoint-direction-scan.jsonl`, which evaluates every
available post-stop direction returned by the local direction generator. Inspect
that file before claiming a prediction-selected endpoint diagnostic missed a
finite improvement.

Observed multi-direction trace runs can be regenerated with:

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

The trace tries generated directions in descending predicted derivative order
and accepts only after recomputed `sys` clears the effective threshold. Regenerate
and retain the output before claiming that this fixes a branch-switching miss or
changes behavior in a degeneracy regime.

Smaller-step endpoint scan panels can be regenerated by adding endpoint steps:

```bash
--steps 1e-3,1e-4 --endpoint-steps 1e-3,1e-4,1e-5,1e-6
```

Inspect `endpoint_direction_scan_threshold_counts` and the maximum positive
observed deltas by endpoint step in the regenerated output. A finite direction
and step panel is not a certificate that endpoints are local maxima.

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

No retained aggregate output is checked in here. Regenerate this report into an
intentionally retained artifact location before citing aggregate counts,
endpoint-scan threshold counts, or compute-budget totals.

Endpoint scan magnitudes can be summarized with:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-endpoint-scan-report -- \
  --out-dir /tmp/dev-gradient-ascent-endpoint-scan-report-check \
  /tmp/dev-gradient-ascent-local-geometry-run-1 \
  /tmp/dev-gradient-ascent-local-geometry-run-2
```

It writes:

- `run-endpoint-scan-report.jsonl`
- `endpoint-scan-summary.json`

Unlike `dev-gradient-ascent-aggregate-summaries`, this command does inspect
`endpoint-direction-scan.jsonl`. It reports the largest remaining positive
finite endpoint improvement and the largest ratio to the effective endpoint
threshold.

The endpoint scan report is the artifact to inspect before claiming an endpoint
condition. A scratch report is not retained evidence.

Run trace behavior can be summarized with:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-run-trace-report -- \
  --out-dir /tmp/dev-gradient-ascent-run-trace-report-check \
  /tmp/dev-gradient-ascent-local-geometry-run-1 \
  /tmp/dev-gradient-ascent-local-geometry-run-2
```

It writes:

- `run-trace-report.jsonl`
- `run-trace-summary.json`

The run-trace report is the artifact to inspect before claiming that a trace
policy choice mattered on a retained panel.

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

Use this report to separate threshold-choice questions from expensive geometry
recomputation. It is not a replacement for rerunning the trace under a different
policy, because a different accepted/rejected decision would change later
basepoints.

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
