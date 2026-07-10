# Trusted Random/Product Method Dispositions

Purpose: disposition ledger for active checklist families scoped to trusted
random/product rows. This is not a replacement for method packet READMEs or
artifacts.

Epistemic status: current control surface after removing non-invariant active
features and abandoned ascent/continuation/local-behavior surfaces from the
active datascience slice. The retained random/product tables and active method
artifacts have been regenerated under the invariant schema; method READMEs
record the current numbers. The remaining gate before thesis use is
human-level interpretation of whether these packets support a thesis-facing
claim, not schema repair.

## Disposition Vocabulary

- `current-invariant-run`: method packet has been rerun under the active
  invariant schema and its README records the current result.
- `defer`: plausible standard-method family, lower current value than closing
  and reviewing the active packet set.
- `reject`: not useful for this random/product table unless assumptions change.
- `out-of-scope`: not part of the current random-polytope slice.

## Baseline And EDA

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| target predicate scan | `current-invariant-run` | `scan-sys-gt-1/` | Current invariant full run found zero `sys > 1`. |
| target distribution, quantiles, tail summaries | `current-invariant-run` | `random-tail-eda/` | Current-schema EDA exists; review tail-language limits before thesis use. |
| source/facet/product summaries | `current-invariant-run` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product diagnostics exist; choose subgroup statements only when needed by thesis text. |
| missingness, duplicates, schema, row-count checks | `current-invariant-run` | `trusted-random-dataset/` | Current invariant full run has `14336` trusted rows, no duplicates, and zero excluded labels. |
| outlier and near-miss inspection | `current-invariant-run` | `random-tail-eda/`, `projection-structure/` | Current top row has `sys = 0.86258589584944`; near-miss language still needs thesis-level judgment. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split and grouped validation | `current-invariant-run` | `prediction-ranking/` | Current grouped holdout artifact exists; adequacy for thesis use depends on whether we promote a candidate-proposer claim. |
| source/provenance leakage checks | `current-invariant-run` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/factor tests, metadata-only baselines, and overlays exist. |
| provenance-only and metadata-only baselines | `current-invariant-run` | `prediction-ranking/` | Current metadata-only baselines exist. |
| permutation and bootstrap checks | `current-invariant-run` | `statistical-associations/`, `prediction-ranking/` | Current association permutation/bootstrap and ranking permutation artifacts exist. |

## Direct Random Sampling

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind generic random sampling | `current-invariant-run` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `4096` generic rows and no positives. |
| blind random Lagrangian-product sampling | `current-invariant-run` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `10240` product rows and no positives. |
| independent same-distribution reruns | `defer` | none | Useful for stability only after current-schema closure shows which facts need stability. |
| broader height intervals, facet ranges, product side ranges, or new distributions | `defer` | none | This is plausible next research work, but first close the current retained producer contract. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| ridge regression | `current-invariant-run` | `prediction-ranking/` | Current invariant-feature ridge model ran under the active schema. |
| lasso and elastic-net regression | `current-invariant-run` | `standard-baseline-p2/` | P2 ran sparse/shrinkage linear baselines under grouped holdout. |
| random forest regression | `current-invariant-run` | `prediction-ranking/` | Current invariant-feature RF model ran under the active schema; it is in-table signal, not yet a generated-candidate proposer. |
| gradient boosting regression | `current-invariant-run` | `standard-baseline-p2/` | P2 ran histogram gradient boosting regression under grouped holdout. |
| shallow high-tail decision-tree rules | `current-invariant-run` | `tail-rule-mining/` | Current invariant-feature tree run completed under the active schema; older geometry-family artifacts are stale. |
| high-tail classification and classifier variants | `current-invariant-run` | `standard-baseline-p2/` | P2 ran elastic-net logistic and histogram gradient boosting high-tail classifiers. Labels are retained-table high-tail labels, not positives or generated-candidate validation. |
| feature-family ablation | `current-invariant-run` | `standard-baseline-p2/` | P2 compared combinatorial-count and ridge symplectic-area feature families under grouped holdout. |
| metadata-only prediction baselines | `current-invariant-run` | `prediction-ranking/` | Current metadata-only baselines ran. |
| generated-candidate proposer loop | `defer` | none | Reopen if rerun ranking evidence has enough expected thesis value to justify producing new candidates. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `current-invariant-run` | `projection-structure/` | Current PCA artifact exists. |
| k-means clustering | `current-invariant-run` | `projection-structure/` | Current k-means artifact exists. |
| isolation-style anomaly detection | `current-invariant-run` | `projection-structure/` | Current isolation-style anomaly artifact exists. |
| hierarchical clustering, mixtures, density estimation, one-class classification | `defer` | none | Reopen only if the active projection/anomaly packet shows robust unexplained high-tail structure. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `current-invariant-run` | `statistical-associations/` | Current scalar screen ran on `29` nonconstant active scalar covariates. |
| source/facet/product factor tests | `current-invariant-run` | `statistical-associations/` | Current factor tests ran with current provenance fields. |
| ridge symplectic-area scalar hypotheses | `current-invariant-run` | `prepare/`, `statistical-associations/` | Active association screen includes invariant ridge symplectic-area features. |
| Kendall, partial correlation, FDR variants | `defer` | none | Add only if the active rerun promotes a scalar effect needing robustness checks. |

## Rejected Or Out Of Scope

| Checklist family | Current disposition | Reason and reopen trigger |
| --- | --- | --- |
| ascent, continuation, endpoint stability, attractors, basins, local-behavior panels | `out-of-scope` | Removed from active datascience slice; use a separate thesis slice only if Jörn explicitly reopens it. |
| forecasting/time-series | `reject` | No time-indexed forecasting target. |
| bandits or reinforcement learning | `reject` | No online environment or reward loop is defined. |
| MCMC/Bayesian posterior families | `reject` | No probabilistic model is currently part of the thesis claim. |
| multi-fidelity optimization | `reject` | No cheaper faithful proxy for `sys` is established. |
| distribution-variant sensitivity rows | `defer` | `distribution-sensitivity/` and `random-axis-diagnostic/` need multiple prepared random/product variants; the active retained table has only the current producer contract. |

## Current Gaps Before Closure

1. Do not claim closure over arbitrary random distributions unless a new
   distribution-design batch is run and reviewed.
2. Do not claim a generated-candidate proposer until an unevaluated-row ranking
   experiment is designed and run.
3. Decide whether P2 is enough for the intended broad retained-table
   standard-method wording; it does not close broader distributions.
4. Decide thesis wording separately from method-packet readiness.
