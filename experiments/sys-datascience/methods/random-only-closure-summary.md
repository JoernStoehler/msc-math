# Random-Only Method Closure Summary

Purpose: current cross-method dashboard for the trusted random/product
`sys` datascience slice. This file is a navigation and disposition summary, not
an additional method result. Source truth remains each method packet README,
the method artifacts, the retained table fingerprint, and the feature-space
coverage ledger.

For checklist-family dispositions, read
`random-only-method-dispositions.md`. The checklist itself remains a recall aid,
not a result ledger.

Epistemic status: implementation and smoke coverage exist, and the main
random-only method packets were rerun against a reproducible scratch prepared
table at `/tmp/sys-ds-random-only-full`, built with `sys-dataset
--random-only`. Since that rerun, prepare grew additional reusable omega/area
summary columns. The checked-in method artifacts have therefore not yet been
rerun against a full scoped prepared table with the latest schema. Do not use
this file to claim final thesis closure until the full scoped current-schema
rerun, method/statistics review, and deferred-family review are recorded.

## Current Data Slice

Trusted random/product rows are defined by
`trusted-random-dataset/` and shared helper code in `_shared/random_only.py`.

Current retained-table random-only fingerprint recorded by method packets:

| dataset | rows | `sys > 1` rows |
| --- | ---: | ---: |
| `random_sample` | `4096` | `0` |
| `random_product_sample` | `10240` | `0` |
| trusted random/product total | `14336` | `0` |

Prepared-input caveat: the prepared input used for these artifacts was a
scratch table at `/tmp/sys-ds-random-only-full`, not a retained checked-in
table. It is reproducible with
`experiments/sys-datascience/prepare/build-random-only-slice.sh full` using
hydrated canonical producer files. Regenerate it before relying on current
prepare-schema feature columns.

## Method Summary

| Method group | Packet | Data slice | Test performed | Current result | Caveat | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| trusted input filter | `trusted-random-dataset/` | scoped random/product rows | shared provenance filter, row counts, duplicate/exclusion audit | `14336` trusted rows, `0` duplicates, no excluded-label hits | prepared input currently `/tmp`, not retained checked-in table | input contract |
| target predicate scan | `scan-sys-gt-1/` | retained random/product rows and broader retained table when not using `--random-only` | direct `sys > 1` scan | random-only scoped run: `0` positives in `14336` rows | table-scoped absence only; does not validate capacity/sys computations | baseline negative scan |
| EDA and tail summaries | `random-tail-eda/` | scoped random/product rows and overlapping source/facet/product filters | marginal distribution, quantiles, top tail, generator contract, source-parameter availability | max `sys = 0.86258589584944`, no positive row; explicit height/product fields available where canonical producers provide them | tail extrapolations are unstable | descriptive EDA, not a proposer |
| scalar association screening | `statistical-associations/` | scoped random/product rows | Pearson/Spearman screens, family-max permutation, bootstrap source mean difference, source factor tests | `110` eligible scalar covariates, `99` nonconstant tested, strongest Spearman `0.9384368671850424`, family-max p `0.004975124378109453` | explanatory screening; multiple comparisons handled only at family-max level | clean proposer evidence only after separate generated-row test |
| projections and structure | `projection-structure/` | scoped random/product geometry features | PCA, k-means summaries, isolation-forest anomaly overlap, source/facet/product metadata overlays | `109` geometry features, no anomaly/top-tail overlap, metadata overlays present | in-table exploratory structure only | exploratory structure only |
| supervised ranking | `prediction-ranking/` | scoped random/product geometry features, plus metadata-only diagnostics | grouped holdout ridge/random forest, enrichment permutation, metadata-only source/facet/product baselines | geometry RF `R^2 = 0.9266078877149259`, metadata-only RF `R^2 = 0.0019535588595060993` | held-out rows already have `sys` | in-table signal, no validated candidate-proposer |
| non-gradient perturbation | `non-gradient-perturbation/` | tiny hash-selected trusted random/product basepoint panel | fixed random directions and fixed radii, no gradient/ascent semantics | smoke panel ran, no `sys > 1`, max increase about `7.9e-4` | deliberately tiny; not broad perturbation coverage | route smoke only |

## Checklist Mapping

This mapping is deliberately coarse. Use
`method-coverage-checklist.md` for recall,
`random-only-method-dispositions.md` for family-level run/defer/reject/out-of-scope
decisions, and the packet READMEs for evidence.

| Checklist family | Current handling in random-only slice | Remaining gap |
| --- | --- | --- |
| baseline scans and target predicate | `scan-sys-gt-1/`, `trusted-random-dataset/` | method/statistics review |
| distribution, quantiles, source/facet/product EDA | `random-tail-eda/` | method/statistics review; independent generator reruns if thesis wording needs stability |
| missingness, duplicates, provenance/filter audit | `trusted-random-dataset/`, `random-tail-eda` availability diagnostics | retain or reproduce scoped prepared input |
| scalar statistical associations | `statistical-associations/` | method/statistics review |
| grouped validation, leakage, null checks | prediction grouped split, association permutation/bootstrap, metadata-only baselines | method/statistics review |
| projections, clustering, density/anomaly checks | `projection-structure/` | method/statistics review |
| supervised prediction/ranking | `prediction-ranking/` | generated-candidate follow-up only if thesis value justifies it |
| bounded non-gradient perturbation search | `non-gradient-perturbation/` | larger panel only if promoted by value-of-information |
| gradient/ascent/continuation/attractor methods | out of clean random-only scope for this feature-space goal | separate ascent/local-max data needed before claims |
| broad derivative-free optimization families | not closed by the current tiny perturbation smoke | defer or create explicit non-gradient panels only if they remain high value after retained reruns |
| post-capacity orbit/KKT interpretation | kept out of clean proposer features by shared selector; available only for interpretation | audit only if thesis wording needs post-capacity explanation |

## Current Blocking Gate

The next evidence gate is a full scoped random/product rerun against the
current prepare schema, method/statistics review of the resulting artifacts, and
a decision about whether the scratch prepared input should be promoted or
treated as reproducible generated data. Do not use the all-source retained-table
rebuild for this goal by default.

## Thesis Claim Status

Currently supported by scoped full random/product artifacts:

- the retained random/product table slice used an explicit trusted filter;
- the scoped random/product table slice had no recorded `sys > 1` rows;
- EDA/model artifacts found in-table structure but no validated generated
  candidate-proposer.

Not yet supported:

- a final random-only method-table closure claim;
- any ascent endpoint, local-maximum, attractor, basin, or continuation claim.

## Reopen Triggers

- retained tables are rebuilt or producer data changes;
- new random/product or non-gradient perturbation rows become trusted inputs;
- a method reports `sys > 1`, near-threshold behavior requiring escalation, or
  a candidate-proposer that ranks unevaluated rows before `sys` computation;
- thesis wording asks for broader random distributions, broader optimization
  families, or ascent/attractor structure.
