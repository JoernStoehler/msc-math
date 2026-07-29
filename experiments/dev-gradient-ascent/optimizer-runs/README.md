# Traced local-optimizer runner

Status: clean implementation foundation plus retained archive evidence. The
smoke proves plumbing only; the separately identified held-out packet below
supports the bounded optimizer comparison.

This package runs manifest-declared optimizers against one instrumented
`sys(a)` evaluator. Its purpose is to compare local step rules, trajectories,
and credible fixed-facet endpoints under the same accounting contract.

## Provenance and import boundary

The implementation was selectively reconstructed from the read-only archive
branch `sys-optimizer-completion` at
`073bb014428de60946a8ea1b744f4e8992042a83`. The original runner/comparison
pipeline entered that branch at `ab07fad91`.

Imported from the archive head:

- `optimizer-runs/src/`, `Cargo.toml`, and the schema-1 ask/tell runner;
- the small `manifests/smoke.json` shape, rewritten to use a checked-in smoke
  source and exact binary64 geometry;
- `../optimizer-comparison/analyze.py`, without retained datasets, plots, or
  specialized follow-up analyzers.

Initially not imported:

- archive `artifacts/` trees and production/tuning manifests;
- `optimizer-atoms/`, predictor replays, fitted triggers, endpoint
  classification, and trajectory-geometry analyses;
- the archive method ledger, because most of its numerical rows point to
  artifacts that are not present here;
- the archive-only heuristic f64 geometry export from
  `experiments/dev-quadratic-program/`.

The later closeout imports only the frozen F=10 held-out and matched
larger-budget datasets used by the thesis. Their provenance identifies the
archive executable, source commit, manifest, and evaluator contract. They are
historical schema-1 evidence, not outputs of the clean evaluator below:

- `artifacts/heldout-f10-64-finalists-19a8b4dfd/`;
- `artifacts/history-f10-16-compute-depth-426ec7a7c/`.

The corresponding frozen manifests are retained beside the smoke manifest.
Reproducing those values requires their recorded archive source and heuristic
f64 evaluator; running the manifest with this clean evaluator would be a new
experiment, not regeneration of the retained packet.

The clean runner instead uses the current
`exp_sys_landscape::SysLandscapePolytopeCache` exact geometry of the binary64
input coordinates. This keeps the imported foundation inside
`dev-gradient-ascent` and prevents an unavailable branch-predictor dependency
from silently defining the objective.

The archive `OPTIMIZER-METHODS.md` supplies planning context, not evidence
owned by this branch: branch history was the best tested one-second F10 method,
but its representative endpoints were still improvable. Gradient sampling,
full-oracle bundle/trust-region methods, full MADS, and nonsmooth BFGS had not
received decisive trials. This foundation preserves that distinction and does
not promote an archive winner to a final optimizer.

## Contracts

### Oracle

Every charged proposal is evaluated by `evaluator.rs`:

1. reconstruct exact geometry for the binary64 dual vertices;
2. compute volume in the manifest-selected exact or f64 mode;
3. run the complete legacy orbit-candidate search with minima-safe exact
   aggregation;
4. form the f64 systolic ratio and retain the winning orbit context.

The result is a high-confidence binary64 computation, not an exact real
number. `geometry_mode: "f64"` is intentionally rejected during manifest
validation because the archived heuristic implementation is not part of this
foundation. Near-threshold scientific use still needs a separate numerical
audit against the current production capacity route.

Structure-aware algorithms may perform named-branch KKT solves and surrogate
queries inside `ask`. Those are not counted as full-`sys` calls, but their wall
time is included in optimizer compute. A proposal affects the retained
best-so-far value only after a full evaluator call.

### Trajectory and locality

The ask/tell interface separates:

- the algorithm's current state and acceptance rule;
- the globally retained best fully evaluated state;
- proposals and their geometric reference;
- full evaluator observations.

Proposal rows report
`||a' - a_ref||_2 / ||a_ref||_2`. Local algorithms must state which evaluated
point supplies `a_ref`; a best-so-far curve alone does not identify the path or
show convergence. Invalid proposed polytopes remain charged observations.

### Symmetry

`quotient.rs` constructs a local Euclidean slice transverse to the 15
infinitesimal translation, common-scaling, and linear symplectic generators.
At a generic fixed-facet point the slice dimension is `4F - 15`, hence 25 for
`F=10`. This is a tangent-space gauge at a named anchor, not a global quotient
coordinate system. Methods must rebuild or explicitly transport the slice when
their state moves.

### Compute

- `budget` is a hard cap on charged full-`sys` calls.
- `compute_budget_ms`, when set, includes charged evaluator time plus measured
  optimizer `ask` and `tell` time.
- A proposal already in evaluation may overshoot the compute budget; the
  overshoot is recorded.
- Population batches may end after an evaluated prefix.
- Use `parallelism: 1` for equal-compute comparisons. Parallel wall time is not
  the serial-compute contract.

The contract supports comparisons by calls, evaluator time, and measured
evaluator-plus-optimizer time. It does not make different geometry/capacity
implementations comparable across archive and clean-runner datasets.

### Artifacts

The output directory must be new or empty. A run writes:

- `resolved-plan.json`;
- `run-provenance.json`;
- `evaluations.jsonl`;
- `proposals.jsonl`;
- `rounds.jsonl`;
- `runs.jsonl`, written last as completion coverage.

The analyzer checks schema versions, unique identifiers, table links, call and
round continuity, best-so-far reconstruction, completion coverage, and status
counts before producing summaries. Generated smoke or future experiment
outputs belong in fresh temporary or explicitly declared artifact directories;
do not hand-edit them.

## Current algorithm surface

All nine registered families compile and completed the two-call F6 smoke:

| Manifest kind | Actual method and role |
| --- | --- |
| `online_source` | independent online source draws; nonlocal allocation baseline |
| `iid_source` | independent draws from the declared source pool; nonlocal baseline |
| `direct_search` | signed coordinate poll in the local symmetry-transverse slice; not full MADS |
| `cma_es` | full-`sys` CMA-ES in a fixed local slice; population reference |
| `literal_gradient` | unconditional selected-winning-branch gradient step; explanatory baseline |
| `safeguarded_gradient` | normalized winning-branch gradient with accept/reject radius control |
| `gap_model` | affine finite-gap maximin model; `extension_mode: none` is the supported baseline |
| `nonlinear_candidate_cma` | CMA-ES over a repeatedly evaluated fixed named-branch envelope, followed by full validation |
| `nonlinear_candidate_relinearized` | fixed-candidate relinearization with optional recent branch history; this hosts the archive-selected branch-history configuration |

The transition-blocked extension and directional-transition switches remain
available only to preserve the coherent branch-history implementation. They
are exploratory branch-domain heuristics, not established oracle contracts and
should not enter the first missing-method comparison.

## Smoke and verification

From the repository root:

```bash
cargo test -p optimizer-runs

out="$(mktemp -d)/optimizer-runs"
cargo run -p optimizer-runs --release -- \
  --manifest experiments/dev-gradient-ascent/optimizer-runs/manifests/smoke.json \
  --out "$out"

uv run --script \
  experiments/dev-gradient-ascent/optimizer-comparison/analyze.py \
  --dataset "$out" \
  --out "$out-analysis" \
  --mode development
```

The smoke uses `fixtures/local-optimizer-source.jsonl`, one starting state, two
charged calls, and serial execution. It must not be cited as a performance
result.

## Recommended first missing-method batch

The rank below is by expected value for distinguishing local trajectory and
endpoint behavior, not by expected one-second performance. Engineering
estimates assume reuse of this runner and do not include a production
population.

| Rank | Missing method | Smallest credible implementation | F10 compute shape | Engineering estimate |
| ---: | --- | --- | --- | --- |
| 1 | gradient sampling | sample full-`sys` points in the 25-dimensional local slice, extract one valid branch gradient at each evaluated point, solve the minimum-norm convex-hull problem, then use a radius/line-search acceptance rule | 26 full evaluations per fresh canonical sample cloud, plus trial steps | 1–2 focused days |
| 2 | full-oracle nonconvex bundle/trust region | retain value/gradient jets only from fully evaluated local points, use locality-limited cuts and serious/null steps, and solve a proximal aggregate subproblem | usually one full trial evaluation per iteration; model solves should be cheap relative to the oracle | 2–4 focused days |
| 3 | full MADS | mesh and poll parameters, deterministic dense direction generation, opportunistic polling, and mesh refinement in a rebuilt local slice | up to 50 full evaluations for an unsuccessful positive-spanning F10 poll; often fewer with opportunistic stopping | 1–2 focused days |
| 4 | nonsmooth BFGS | selected-gradient updates with a nonsmooth-safe line search, curvature-update safeguards, and explicit restart/failure states | normally one gradient-bearing full evaluation per line-search trial | 0.5–1.5 focused days |

Implement ranks 1–3 as the first serious batch if development time allows.
Nonsmooth BFGS is a cheap additional trajectory comparator, but its use of one
selected gradient gives weaker endpoint evidence and the existing literal
gradient failures lower its prior. CMA-ES is already the zeroth-order
population reference.

The batch should first add shared atoms rather than four isolated drivers:

- an `EvaluatedJet` extracted only from a full evaluator result, carrying
  value, selected/near-active branch gradients, branch identities, eligibility
  status, and timing;
- a deterministic minimum-norm convex-combination solver, shared by gradient
  sampling and the bundle aggregate;
- a recentered quotient-coordinate adapter with recorded anchor and realized
  ambient displacement;
- common radius/mesh and line-search events, with current state kept separate
  from best-so-far;
- stationarity and failure fields that remain meaningful when a sample,
  gradient, trial point, or model solve is indeterminate.

Do not reuse unrestricted raw KKT tangents as global bundle cuts. Archive
replays showed that branch-domain failures can make such tangents control a
false model minimum. The first bundle implementation should use jets observed
at fully evaluated points and an explicit locality/trust-region contract.

## Remaining exploratory and nonmergeable work

- No production trajectory population has been run on this branch.
- The exact-geometry clean evaluator is not byte- or time-comparable to the
  archive's heuristic-f64 datasets.
- The analyzer is a general strict consumer, but only the smoke dataset has
  been checked in this branch.
- Gradient sampling, bundle/trust-region, full MADS, and nonsmooth BFGS are
  recommendations, not implemented algorithms.
- Endpoint stationarity, local-maxima classification, and trajectory
  invariants are owned elsewhere and are not inferred from these traces.
