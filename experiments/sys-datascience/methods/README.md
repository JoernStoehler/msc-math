# Random-Polytope Datascience Methods

This directory owns current method packets for the random/product
sys-datascience method table.

Read first:

1. `../README.md`
2. `../prepare/README.md`
3. `trusted-random-product-closure-summary.md`
4. `trusted-random-product-method-dispositions.md`
5. the relevant method folder `README.md`

## Current Method Packets

- `trusted-random-dataset/`: shared input filter and row/provenance audit for
  trusted random/product rows.
- `scan-sys-gt-1/`: baseline target-predicate scan.
- `random-tail-eda/`: EDA over trusted random/product rows and source/parameter
  filters.
- `statistical-associations/`: scalar association screening against `sys`.
- `projection-structure/`: PCA, clustering, and anomaly diagnostics for
  active invariant scalar features.
- `prediction-ranking/`: supervised in-table prediction/ranking diagnostics.
- `tail-rule-mining/`: shallow decision-tree high-tail rule diagnostics with
  invariant-feature versus strata/provenance-control grouped-holdout comparison.
- `standard-baseline-p2/`: P2 retained-table missing baselines: lasso,
  elastic-net, gradient boosting, high-tail classification, and feature-family
  ablation under grouped holdout.
- `hko-reference-coverage/`: scores the known HKO reference/holdout row against
  retained random/product invariant-feature support without training on HKO.
- `hko-ridge-source-smoke/`: source-reproducible fixed-HKO ridge-area smoke
  packet used as sys-datascience mechanism/reference evidence. It does not own
  HKO local-maximality claims.
- `extreme-scalar-rejection-proposer/`: generated random-product proposer with
  the original 100k scalar boundary packet and a reviewed independent frozen
  ridge-concentration validation. The latter validates sub-threshold
  enrichment; its thesis terminology/role awaits Jörn.
- `ridge-mechanism-discriminator/`: compact diagnostic table combining retained
  tail-rule diagnostics and generated ridge/proposer summaries to distinguish
  ridge-magnitude, concentration, proxy, small-area, and Goodhart explanations.
- `tail-dependence-feasibility/`: existing-data identifiability and censoring
  audit that selected the bounded generic `F=10` transfer experiment and its
  stopping rule.
- `ridge-tail-source-comparison/`: frozen generic/product source comparison and
  product `5x5` versus `4x6` sensitivity evidence used to avoid a multi-bucket
  replication.
- `generic-ridge-tail-stage1/` and `generic-ridge-tail-stage1-target/`: reviewed
  target-free 10,000-candidate generic `F=10` selection followed by exactly 200
  frozen target evaluations. Low ridge transfers as a coarse filter but harder
  conditioning does not improve `sys`, so the packet stops at 10,000.
- `ridge-tail-anatomy/`: retained-data Euclidean-area/Kähler-angle decomposition
  for the generic ridge panel and frozen product `5x5` rho/ridge/control arms.
  Both components move with the coarse filter, but neither supplies a
  supported scalar ordering inside the favorable region.
- `residual-exemplar-seeds/`: target-free matched-pair construction plus
  post-target model-consistent residual inspection candidates for bounded
  geometry/HK-branch hypothesis formation; exploratory (`G`) only.
- `equal-budget-product-search/`: parked target-free S0 prototype for a fixed
  `5 x 5` IID/local/CEM comparison. It has no real-run artifacts or empirical
  result and must not be resumed without a new portfolio decision.
- `product-bounce-distribution/`: exact-`(k,m)` descriptive distribution and
  ridge-adjustment packet for 2- versus 3-bounce retained random products.

Exploratory fixed-bucket distribution-shape packets:

- `sys-distribution-broad-scan/`: broad SciPy distribution scan for fixed-bucket
  marginal laws of `sys(a)`, using logit-transformed families as proposal
  generation only.
- `sys-distribution-mle-likelihood-table/`: all-data MLE parameters and
  same-row log-likelihood table for fixed-bucket marginal transform/family
  model comparisons.
- `high-sys-tail-diagnostic/`: upper-tail-only diagnostics for fixed-bucket
  high-`sys` behavior, including excess-tail fits and endpoint estimates.
- `tail-survival-1m-posterior/`: survival summaries and zero-positive/tail-model
  sensitivity for deciding whether `1M` accepted samples is a rational
  current-generator scale-up.

These fixed-bucket packets are retained as exploratory side packets. Their
current README summaries describe scratch runs; they are not part of the
current invariant-feature method rerun gate until compact artifacts are
promoted into the packet directories.

- `distribution-sensitivity/` and `random-axis-diagnostic/`: exploratory
  variant-comparison scripts. They are not active method-table rows until two
  or more prepared random/product variants exist and are reviewed.

Target-free generator support:

- `conditioning-distortion/`: candidate-level proposal/rejection audit for the
  alternative planar laws. It reports bounded retry cost, separate terminal
  reasons, and accepted-versus-proposed feature shifts; it makes no target,
  `sys`, or capacity calls.

The old ascent endpoint, local-behavior, continuation, and perturbation packets
are not active in this slice. Use git history for archaeology only.

## Method Question

The global question is:

> Can a standard data-science method applied to the retained random/product
> sample help find a `sys > 1` polytope or a credible candidate-proposer?

Each method packet states its own reduction of that question and its current
disposition.

## Inputs

Ordinary methods read retained tables under `../prepare/` and build
method-specific matrices inside the method folder. Shared random/product input
filtering lives in `_shared/random_only.py`.

Current-schema full random/product prepare and method reruns have completed on
the retained invariant tables under `../prepare/`. Compact generated summaries
are tracked under each method's `artifacts/` directory when the README cites
current invariant-schema numbers; regenerate checked artifacts deliberately
before thesis-facing citation.

Active `polytope-table.jsonl` rows are invariant-only. Legacy method artifacts
that depend on raw Euclidean, omega-matrix, transition, capacity, or volume
fields are stale unless explicitly rerun against an older schema for archaeology.

## Packet Convention

One active method folder should support one method-table row or one explicitly
named row group. A method folder `README.md` should record:

- research question;
- method;
- inputs and feature construction;
- commands to rerun;
- retained artifacts;
- observations and interpretation with epistemic status;
- validity guards and leakage concerns;
- current disposition;
- remaining worthwhile questions;
- predicted stability under rerun;
- thesis use;
- reopen triggers.

Do not use `report.md` as durable method state. Extract current value into the
method `README.md`, then delete disposable reports.

Escalate before unrelated work continues if a method produces a trusted new
`sys > 1` candidate or source beyond the already-known HKO/rotated-pentagon
family and declared reference/control inputs, or a credible new
generated-candidate proposer. Encountering an expected known-positive row while
auditing or scoring references is not a discovery trigger.
