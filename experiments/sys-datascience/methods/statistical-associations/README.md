# statistical-associations

## Research Question

Which engineered scalar covariates are associated with the target scalar
`sys(a)` in the trusted random/product sample, and what obvious invariant
feature families are not yet represented?

This is the standard univariate association-screening question: screen one
scalar covariate at a time against the target scalar, then inspect the strongest
correlations and their scope limits.

The active method-facing `polytope-table.jsonl` is invariant-only, with fields
defined by `prepare/rows.rs::PolytopeTableRow`. Legacy raw Euclidean,
omega-matrix, transition, `capacity`, and `volume` columns are not active
covariates.

Conceptually the flow is:

```text
prepare
  -> invariant scalar rows plus provenance metadata
methods
  -> univariate association screening of active invariant scalar features f_i(a) against sys(a)
```

This packet currently runs the last step on the invariant scalar features
already present in the retained table, and records gaps relative to the active
row schema.

## Method

Screen eligible scalar retained-table covariates against `sys` with Pearson
and Spearman correlations. Use a permutation family-maximum check across all
tested association features for the strongest rank association, categorical
factor tests for source/provenance strata, and a bootstrap confidence interval
for the random-product minus generic-random mean `sys` difference.

Eligibility is defined by the shared random-only feature selector:

- include scalar numeric columns present as numeric values in at least `98%` of
  retained rows;
- exclude the target `sys`, non-scalar JSON/string columns, sparse numeric
  columns, legacy `capacity`/`volume` columns, and non-active legacy feature
  prefixes if an older table is passed;
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

`capacity_source` remains metadata for grouping and source-factor tests. It is
not treated as an intrinsic scalar invariant feature.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`
- feature columns from `../../prepare/polytope-table.jsonl`

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

No current full retained-table interpretation is recorded here until the
invariant-only schema is rerun. Earlier numeric blocks were removed because they
predated the active schema and mixed deleted non-invariant feature families with
current invariant features.

Under the active invariant schema, current method-facing scalar families are:

- basic counts and simplicity summaries;
- vertex incidence and degree summaries;
- ridge-size summaries;
- facet vertex-count and neighbor-count summaries;
- ridge symplectic-area summaries normalized by `sqrt(volume)`;
- `capacity_source` and provenance fields only as grouping/factor metadata.

Partially covered or missing work under the active schema:

- generator rejection/attempt metadata is only available when provenance exposes
  optional `sample_attempt`;
- explicit product-structure or symmetry scores are not implemented;
- additional invariant scalar summaries would need to be added deliberately to
  `PolytopeTableRow` and its producers.

Missing or separately handled families should be added to
`prepare/rows.rs::PolytopeTableRow` or its producers only when they are intended
to be active invariant method-facing fields.

After rerun, interpret the strongest active scalar associations as explanatory
signals only. This packet does not turn them into a generated-row
candidate-proposer.

## Validity Guards

- Scalar associations are explanatory evidence unless they define a held-out or
  unevaluated candidate-proposer.
- Many correlated table columns are screened; the permutation check is a
  coarse guard, not a formal model-selection theorem.
- Capacity and volume columns are not active method-facing covariates.
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

- Build a small feature-map audit for additional invariant scalars derivable
  from the producer geometry, then decide which missing families have enough
  value to implement in `prepare/invariant_features.rs`.
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
