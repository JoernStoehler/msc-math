# Random-Polytope Datascience Methods

This directory is the physical home of current method packets for the
random/product sys-datascience method table.

Read first:

1. `../README.md`
2. `topics/empirical-studies/polytope-invariant-table/README.md`
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
- `conditional-tail-atlas/`: exploratory figure options for within-bucket
  scalar percentiles, upper-`sys` behavior, and cross-bucket sign stability.
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
  packet used as sys-datascience mechanism/reference evidence. HKO
  local-maximality claims belong to the
  [HKO neighborhood](../../../variation-and-local-maxima/hko-local-maximum/README.md).
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
  result and must not be resumed without a new portfolio decision. The
  separately integrated standard fixed-population AMS core is reusable method
  code only; the bespoke adaptive comparison was never run.
- `product-bounce-distribution/`: exact-`(k,m)` descriptive distribution and
  ridge-adjustment packet for 2- versus 3-bounce retained random products.
- `product-bounce-class-degeneration/`: generator-local retained-row packet for
  common-support enrichment and the exact one-swap class-difference identity;
  not a physical or causal mechanism.
- `product-triangle-bounce-classification/`: exact falsifier/support packet for
  the agent-reviewed triangle-product capacity theorem. The authoritative
  proof draft is [the triangle classification](../../../products/product-triangle-bounce-classification.tex);
  empirical artifacts are support, not proof premises.
- `generator-orientation-smoke/` and
  `generator-orientation-target-pilot/`: frozen fixed-shape orientation witness
  with a retained 40-row orientation-only exact-feature snapshot, Haar-`U(2)`
  invariance control, and Haar-`SO(4)` alignment-changing arm. Retain the
  witness-level boundary and failed directional ridge-mediator gate; it
  supports no population, causal, proposer-transfer, or general-generator
  claim.
- `alternative-source-transfer/`: prospectively frozen `factorial-both`
  source, exact-feature selection, and one target exposure establishing bounded
  operational sub-threshold ridge/rho proposer transfer. It does not support a
  threshold-directed, population, mechanism, or rho-superiority claim.
- `canonical-vertex-covariance/`: post-target descriptive association between
  canonical extreme-vertex covariance eccentricity and retained `sys`; not
  frozen validation or a proposer.
- `product-bounce-mechanism/`: existing-row decomposition of the retained
  two-/three-bounce association into capacity, volume, generator, ridge, and
  class-balance terms.
- `product-bounce-width-shortcut/`: exact difference-body width formula for the
  two-bounce class minimum, with bounded retained checks; the authoritative
  proof draft is [the two-bounce class formula](../../../products/product-two-bounce-class.tex).
- `ridge-tail-pressure/`: stratified retained ridge-tail diagnostic and two
  frozen selection comparisons, with no universal pressure or endpoint claim.
- `ridge-endpoint-path/`: frozen eight-row endpoint evaluation along two
  explicit product-equality paths, including the q01 exact-minimizer record.
- `ridge-symmetry-completion/`: two-point continuation toward the regular
  triangle--hexagon endpoint; the path inequality is mathematical and further
  empirical densification is parked.

Later retained analysis/evaluation packets:

- `selection-mechanism/`: conditional attenuation, Gaussian-copula reference,
  fixed-normal interpretation and paired factor-covariance balancing.
- `paired-tangentialization/`: complete 64-target four-arm paired intervention.
- `orientation-allocation/`: four-block numerical orientation versus fresh-IID
  allocation comparison, with a negative result for this tactic.
- `diagonal-cem-pilot/`: completed supported-v2 three-seed `5x5` diagonal-CEM
  versus IID comparison. This is separate from the parked July full prototype;
  prior target exposure and historic executable-hash requirements are explicit.
- `current-body-evaluator/`: serial adapter with retained correctness/failure
  controls, not a pilot population result.
- `retrospective-interval-evaluator/`: adapter preserving capacity bounds when
  the scalar conversion is refused; numerical volume still limits ratio claims.

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

The broad-scan, MLE-table, and high-tail-diagnostic READMEs describe scratch
runs without retained compact artifacts. They are proposal/diagnostic code,
not independently recoverable thesis evidence. `tail-survival-1m-posterior/`
**does** retain compact TSV/JSON artifacts and a bounded interpretation of
prior/model sensitivity; it is not merely an unpromoted scratch packet.
These marginal analyses use `sys` and strata rather than the invariant-feature
matrix, so their evidence status is separate from the feature-schema rerun gate.

- `distribution-sensitivity/` and `random-axis-diagnostic/`: exploratory
  variant-comparison scripts. They are not active method-table rows until two
  or more prepared random/product variants exist and are reviewed.

The old ascent endpoint, local-behavior, continuation, and perturbation packets
are not active in this slice. Start at
`topics/empirical-studies/sys-landscape/README.md` for the preserved live legacy packets and
bounded observations; use Git history only for raw artifacts removed from the
current tree.

## Method Question

The global question is:

> Can a standard data-science method applied to the retained random/product
> sample help find a `sys > 1` polytope or a credible candidate-proposer?

Each method packet states its own reduction of that question and its current
disposition.

## Inputs

Ordinary methods read retained tables under `topics/empirical-studies/polytope-invariant-table/` and build
method-specific matrices inside the method folder. Shared random/product input
filtering lives in `_shared/random_only.py`.

Two prepared schemas occur in the retained method evidence. The registered
snapshot supplies 39 invariant numeric features; association, projection, and
prediction summaries retain those 39-feature results. The shared selector now
requires 45 features. P2, the feature-family quotient, and the recorded
current-schema tail-rule run used a separately rebuilt 45-feature table whose
hash is recorded in their READMEs/artifacts. These results must not be described
as a single simultaneous rerun of today's code on the registered snapshot.
See [the method account](../../methods.md) and
[the P2 input contract](standard-baseline-p2/README.md) before reproducing or
comparing them. Keep frozen results intact; regenerate deliberately in a fresh
output directory when an updated analysis is needed.

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
