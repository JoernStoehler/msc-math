# Local Behavior Prediction

Method packet for local and semi-local `sys(a)` prediction diagnostics.

Status: exploratory. This packet reads run-local prepared outputs from
`experiments/sys-datascience/tables/prepare-local-behavior.py`; it does not
own capacity search or reusable joins. It is an interpretation and navigation
surface, not thesis evidence by itself.

Current question routing, predicate vocabulary, and open/closed research
questions live in [LEDGER.md](LEDGER.md). Keep regenerated run outputs as the
source for numerical observations.

## Research Question

This packet asks how far local branch data at a base point `a0` predicts
`sys(a0 + t d)` at finite radii.

The downstream purpose is optimizer design and thesis wording. A gradient
ascent method should use this packet to decide plausible step sizes, direction
families, branch-window policies, and failure classifications before claiming
that an optimizer reaches local-max-like endpoints.

This packet is not a theorem route for arbitrary endpoint local maximality.
The exact generic object is the active-germ / semialgebraic model in
`research/sys-first-order-local-behavior.md`.

## Why This Slice Exists

The thesis-success chain for this packet is:

```text
thesis success
-> hostile sys data-science story must close
-> the method table includes optimization, endpoint, and attractor-style rows
-> those rows need meaningful perturbation, rerun, and local-neighborhood data
-> current retained ascent data has endpoints and traces, not certified
   local maxima or attractors
-> first understand local and semi-local behavior of sys(a0 + t d)
```

Do not skip this step by assuming that an optimizer endpoint is a local
maximum, an attractor, or a stable basin representative. Current endpoint
diagnostics in `../endpoint-local-max-diagnostic/` show the opposite for the
sampled retained endpoints: they still admit quotient ascent directions and
tiny improving probes under that diagnostic.

The point of this packet is therefore to determine perturbation scales,
direction families, and branch-window regimes where finite changes
`a0 -> a0 + t d` are locally interpretable. That information is upstream of
later choices about perturb-and-rerun experiments, endpoint clustering,
same-attractor return tests, basin-size estimates, and optimizer endpoint
claims. It is not local-neighborhood analysis for its own sake.

## Artifact Map

Run-local producers write raw point and sample data under the requested
output directory. The prepare and analyze stages derive the following files:

| Artifact | What to inspect |
| --- | --- |
| `local-behavior-pairs.jsonl` | Pair rows for `(a0, d, t, a0 + t d)`, including target-minimizer status at `a0`, observed `Delta sys`, branch-gradient prediction, and prediction error. |
| `local-behavior-radius-summary.csv` | Radius and direction-family summaries. Start here for local-to-semilocal behavior and step-size planning. |
| `local-behavior-branch-facts.jsonl` | Per-point branch facts derived from producer rows. Use this when a pair summary needs branch-level explanation. |
| `local-behavior-branch-variation.jsonl` | Per-branch value variation between base and target. Use this for branch-function stability questions. |
| `local-behavior-gradient-projections.jsonl` | Per-gradient projection diagnostics. Use this for prediction-quality and direction-alignment questions. |
| `branch-stability-by-radius.png` | Visual summary of target-minimizer status at the base point by radius and direction family. |
| `gradient-prediction-vs-observed.png` | Visual check of branch-gradient prediction quality against recomputed finite changes. |
| `target-branch-status-at-base.png` | Status mix for how target minimizing branches relate to base minimizer, near-active, and candidate-window sets. |

Generated files are source-adjacent evidence for a run. The durable current
interpretation belongs in this README because interpretation can become stale
without the producer or analysis code becoming stale.

## Interpretation Guards

Treat these diagnostics as sampled finite-scale evidence, not as exact
first-order branch coverage. The prepared rows track minimizer branch sets,
candidate/near-active windows, KKT status, beta positivity diagnostics, and
branch gradients. Compare those sets and statuses; the scalar `best_sigma`
value alone is not enough.

Important failure modes include zero-beta boundary germs, singular KKT systems,
active continua, and repeated or redundant listed rows. In particular, a branch
with `beta_i = 0` at the base point can have a right-hand germ with
`beta_i(t) > 0`; dropping zero-beta coordinates can preserve the base HK value
while losing a different first-order slope nearby.

Near-active branch data is not automatically enough. Some finite-step target
minimizers may belong only to the wider base candidate/action window. The
candidate window is also a noise source, so branch-window policy is a design
variable that should be measured rather than assumed.

## Prior Scratch Inputs

The following `/tmp` panels are recovery guidance only. They can inform
parameter choices, but do not cite them as evidence without regenerating a
current retained panel and pointing to that output.

| Path | Role |
| --- | --- |
| `/tmp/sys-local-behavior-panel` | Larger local-neighborhood panel used to choose follow-up radii and status predicates. |
| `/tmp/sys-local-behavior-current-rerun-smoke` | Current-code smoke run from 2026-06-20. |
| `/tmp/sys-random-pair-radii-panel` | Random-pair cross-check for strict minimizing branch sets and cross-evaluation action gaps. |

Reusable qualitative guidance from those scratch panels:

- radii around `1e-4,1e-3,1e-2` are worth checking first;
- group by explicit target-minimizer status predicates, not by informal regime
  labels;
- candidate-window membership depends on the configured action window and is
  not a branch-completeness certificate;
- strict minimizing branch-set equality is brittle at degenerate basepoints.

Typical local flow from repo root:

```bash
cargo run --release -p exp-sys-landscape --bin sys-local-behavior-produce -- \
  --out-dir /tmp/sys-local-behavior-smoke \
  --max-top-basepoints 1 --max-hash-basepoints 0 \
  --random-directions 1 \
  --radii 1e-6,1e-3

uv run --script experiments/sys-datascience/tables/prepare-local-behavior.py \
  /tmp/sys-local-behavior-smoke

uv run --script experiments/sys-datascience/methods/local-behavior-prediction/analyze.py \
  /tmp/sys-local-behavior-smoke/prepared
```

The report and figures are written under
`/tmp/sys-local-behavior-smoke/prepared/local-behavior-prediction/` by default.

## Current Disposition

Use this packet as optimizer-design and thesis-wording guidance. It can
support statements about what a regenerated finite panel observed under stated
radii, direction families, and branch-window settings.

Do not use this packet to claim true local maximality, theorem-level branch
coverage, or that a gradient ascent method reaches local maxima on the
quotient. Those claims require separate evidence.

## Remaining Worthwhile Questions

- Which radii give useful finite-step improvements without losing local branch
  predictivity?
- Which direction families remain predictive in same-min-branch,
  near-active, candidate-window-only, and missing-candidate rows?
- How wide should the branch/action window be before added branch coverage is
  outweighed by overprediction and noise?
- Are optimizer endpoint failures explained by same-branch smooth tails,
  near-active ridge behavior, candidate-window-only branches, missing branches,
  or target construction/domain failures?

## Predicted Stability Under Rerun

High for the smoke command if retained tables and local-behavior code are
unchanged. Larger panels should preserve the qualitative radius questions but
may change counts because selected basepoints, random directions, branch
windows, and target-construction failures affect the status mix.

## Reopen Triggers

- retained tables are rebuilt;
- `sys-local-behavior-produce`, `prepare-local-behavior.py`, or `analyze.py`
  changes;
- branch-window, KKT/status, or minimizer-set semantics change;
- thesis wording asks for optimizer endpoint stability or local maximality.
