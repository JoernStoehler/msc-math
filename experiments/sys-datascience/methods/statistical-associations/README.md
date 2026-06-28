# statistical-associations

## Research Question

Which engineered scalar covariates are associated with the target scalar
`sys(a)` in the trusted random/product sample, and what obvious invariant
feature families are not yet represented?

This is the standard univariate association-screening question: screen one
scalar covariate at a time against the target scalar, then inspect the strongest
correlations and their scope limits.

The minimum local input is the facet-dual list `a_k`. Every scalar here,
including `sys(a)`, is computed from `a`. Therefore the relevant coverage
question is not "how many columns were screened?" but "does the engineered
feature map from `a_k` include the obvious invariant or geometrically meaningful
scalars?"

Conceptually the flow is:

```text
produce
  -> polytopes of interest plus cached expensive polytope computations
prepare/canonize
  -> transform representatives to quotient out symmetries of sys(a) as well as we can
prepare/features
  -> first-layer derived objects: a, omega(a_i,a_j), two-faces, graphs, metadata
  -> transforms/summaries: absolute value, restrictions, max/mean/quantiles/fractions/top-k
methods
  -> univariate association screening of prepared scalar features f_i(a) against sys(a)
```

This packet currently runs the last step on the engineered scalar features
already present in the retained table, and records gaps in the prepare stages.
In current HEAD, prepare-stage feature code lives under
`experiments/sys-datascience/prepare/features*.rs` and is run by the
`sys-datascience-prepare` binary.

The audit starts from first-layer derived objects, not final scalar column
names. A scalar like `allpair_abs_omega_max` is a composition: derive the
pairwise object `omega(a_i,a_j)`, normalize to the volume-one representative,
take absolute values, then summarize by `max`.

## Method

Screen eligible scalar retained-table covariates against `sys` with Pearson
and Spearman correlations. Use a permutation family-maximum check across all
tested association features for the strongest rank association, categorical
factor tests for source/provenance strata, and a bootstrap confidence interval
for the random-product minus generic-random mean `sys` difference.

Eligibility is defined by the shared random-only feature selector:

- include scalar numeric columns present as numeric values in at least `98%` of
  retained rows;
- exclude the target `sys`, prepared evaluation columns such as `capacity` and
  `volume`, non-scalar JSON columns, and sparse numeric columns;
- exclude two-face ordering diagnostics from clean covariate screens; exclude
  two-face symplectic-area summaries if any loaded row reports incomplete
  two-face ordering.

The screen is exhaustive only for the already-engineered scalar numeric
features stored by the shared prepare stage under this rule. It is not
feature-engineering closure from `a_k`.

Source and generator metadata are not treated as intrinsic geometric scalar
features. They are handled separately as categorical factors in
`source_factor_tests`, including source family, dataset label, facet count,
source-by-facet, product bucket, product bounce count, and height range when
those provenance fields are available.

Canonization convention: first normalize to the volume-one representative
before forming scale-sensitive geometric or symplectic summaries. This leaves
`Sp(4)` plus translations as the main invariance issue instead of also carrying
scalings. Further canonization, such as a translation convention based on a
Hausdorff-continuous center, is a prepare-stage design option but is not yet
implemented here.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`
- feature columns from the shared prepare stage, currently
  `../../prepare/features*.rs` and `../../prepare/polytope-table.jsonl`

## Command

```bash
uv run --script experiments/sys-datascience/methods/statistical-associations/analyze.py
```

## Generated Artifacts After Rerun

- `artifacts/summary.json`

The artifact records:

- `eligible_covariate_family_inventory`;
- `tested_covariate_family_inventory`;
- `obvious_covariate_audit`;
- `obvious_covariate_audit["first_layer_nodes"]`;
- `source_factor_tests`;
- `tested_scalar_covariates`;
- `skipped_constant_covariates`;
- `excluded_by_design`.

## Observation

The current artifact was regenerated against a full scoped random/product
scratch prepared table at `/tmp/sys-ds-random-only-full`, built with
`sys-dataset --random-only`. It therefore includes omega matrix/sign,
normalized-omega, two-face-tail, and explicit provenance columns for trusted
random/product rows.

Current full scoped random/product run:

- rows: `14336`;
- eligible scalar covariates: `122`;
- nonconstant scalar covariates tested: `111`;
- strongest absolute Spearman correlation: `0.9384368671850424`;
- family-maximum permutation p-value: `0.004975124378109453`;
- product-minus-generic mean `sys`: `0.04908416085647144`;
- bootstrap 95% interval for that mean difference:
  `[0.04252389852783871, 0.055886616744594816]`.
- source factor tests: `capacity_source`, `dataset_label`,
  `dataset_label_by_facet_count`, `facet_count`, `product_bucket`, and
  `product_bounces` were tested. `sample_height_range` is available but still
  has too few nonempty groups for a meaningful factor test in this retained
  random/product slice.
- strongest source/facet group mean spread in this artifact:
  `dataset_label_by_facet_count` max-minus-min group mean
  `0.3265780723303283`; `facet_count` spread `0.30691591817614594`;
  `product_bucket` spread `0.2750512503828982`;
  `product_bounces` spread `0.16092491442510856`;
  `capacity_source` spread `0.04908416085647144`.

The retained artifact's screened set is exhaustive relative to the scoped
random/product prepare schema and eligibility rule it was run against. The
current prepare schema covers several first-layer nodes. The prepared polytope
table is already in the volume-one representative, so column names do not
repeat that normalization.

- source object `a_k`: norms, centroid norm, coordinate standard
  deviations, pairwise Euclidean distances/cosines, centered singular values;
- pairwise object `omega(a_i,a_j)`: all-pair absolute omega summaries, zero
  fraction, ridge-restricted summaries, omega matrix summaries, omega-sign
  out-degree summaries, and normalized-omega summaries in the chosen Euclidean
  representative;
- two-face object `F_i cap F_j`: symplectic-area mean/std/min/max/sum/max-share,
  median, upper quantiles, top-k share, and zero/small-area fractions, with
  incidence-ordering diagnostics;
- incidence and transition graphs: counts, degrees, densities, adjacency and
  transition summaries;
- capacity outputs: post-evaluation explanatory summaries only.

Partially covered or missing first-layer-node work:

- source/product bucket metadata is now handled by EDA and categorical factor
  tests;
- generator rejection/attempt metadata is not available in the canonical
  retained random/product producer files; newer producer provenance can expose
  optional `sample_attempt`;
- explicit product-structure or symmetry scores are not implemented;
- local perturbation/sensitivity scalars are outside the current
  random/product table.

Raw coordinates of individual dual vertices are not a good substitute for this
audit: coordinate-level effects are not invariant under the relevant symmetries,
and flattened `a_k` arrays require an invariant featurization or an explicitly
equivariant model before they answer what high `sys` corresponds to.

Missing or separately handled families should mostly be added to shared
`prepare/canonize` or `prepare/features`, because projection,
prediction/ranking, clustering, anomaly checks, and other black-box methods
should reuse the same representatives and feature map.

The strongest associations are negative correlations between prepared ridge
symplectic-area, omega, and size features and `sys`.
The strongest current covariate is `ridge_symp_area_sum` with Spearman
correlation `-0.9384368671850424`. These are useful interpretation signals,
but this packet does not turn them into a generated-row candidate-proposer.

## Validity Guards

- Scalar associations are explanatory evidence unless they define a held-out or
  unevaluated candidate-proposer.
- Many correlated table columns are screened; the permutation check is a
  coarse guard, not a formal model-selection theorem.
- Capacity-derived columns are allowed here as post-evaluation explanatory
  features, not as pre-evaluation proposer features.
- Exhaustiveness is only prepare-schema-relative. Changing canonization or
  adding derived invariant geometric, combinatorial, or symplectic scalar
  summaries to the shared prepare stage would reopen this packet and the other
  black-box methods that consume prepared features.
- Categorical factors and engineered raw-dual-vertex features need their own
  encoding or feature-production step; the count of screened scalar columns is
  not evidence that those families were covered.

## Current Disposition

Use as association/interpretation evidence. Do not use as a standalone
candidate-proposer or as evidence that a generated-candidate route was
validated.

## Remaining Worthwhile Questions

- Build a small feature-map audit for invariant scalars derivable from `a_k`,
  then decide which missing families have enough value to implement in
  `prepare/features`.
- Record which symmetries `prepare/canonize` quotients out and which remain
  only partially handled.
- In particular, decide whether to add all-2-face symplectic-area summaries
  normalized by `vol^{1/2}`.
- Promote only associations that are strong, stable, and convertible into a
  pre-evaluation ranking rule.

## Predicted Stability Under Rerun

High on unchanged retained tables and unchanged prepare-stage feature columns.

## Thesis Use

Supports a statement about which current engineered invariant scalar features
co-vary with `sys`. It does not support a candidate-proposer claim, and it does
not claim that the obvious feature map from `a_k` is complete.

## Reopen Triggers

- prepared table columns change;
- prepare-stage canonization changes;
- an obvious derived scalar covariate is identified but not present in the
  shared prepare-stage schema;
- categorical source/product-bucket factors are promoted from EDA slices to a
  model or factor association test;
- a new random-only dataset is added;
- thesis wording promotes a scalar association to a stronger claim.
