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
- `extreme-scalar-rejection-proposer/`: generated random-product scalar-filter
  proposer with a compact tracked 100k `promising-scalars` packet.
- `ridge-mechanism-discriminator/`: compact diagnostic table combining retained
  tail-rule diagnostics and generated ridge/proposer summaries to distinguish
  ridge-magnitude, concentration, proxy, small-area, and Goodhart explanations.
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

Escalate before unrelated work continues if a method records a trusted
`sys > 1` row or a credible generated-candidate proposer.
