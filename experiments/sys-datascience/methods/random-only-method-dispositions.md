# Random-Only Method Dispositions

Purpose: disposition ledger for active checklist families scoped to trusted
random/product rows. This is not a replacement for method packet READMEs or
artifacts.

Epistemic status: current control surface after removing the abandoned
ascent/continuation/local-behavior surfaces from the active datascience slice.
The full current-schema random/product prepare rerun and active method reruns
completed on 2026-06-25 as scratch artifacts under
`/tmp/sys-ds-random-only-full-current` and `/tmp/sys-ds-full-current/`. The
remaining closure gate is review and packet README integration.

## Disposition Vocabulary

- `run-pending-review`: method packet and current-schema scratch artifacts
  exist, but review and README integration are still required before thesis
  use.
- `defer`: plausible standard-method family, lower current value than closing
  and reviewing the active packet set.
- `reject`: not useful for this random/product table unless assumptions change.
- `out-of-scope`: not part of the current random-polytope slice.

## Baseline And EDA

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| target predicate scan | `run-pending-review` | `scan-sys-gt-1/` | Current-schema scratch scan found zero `sys > 1`; review and integrate packet wording. |
| target distribution, quantiles, tail summaries | `run-pending-review` | `random-tail-eda/` | Current-schema EDA exists; review tail-language limits. |
| source/facet/product summaries | `run-pending-review` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product diagnostics exist; decide which subgroup statements matter. |
| missingness, duplicates, schema, row-count checks | `run-pending-review` | `trusted-random-dataset/` | Current-schema scratch artifact has `14336` trusted rows, no duplicates, and zero excluded labels. |
| outlier and near-miss inspection | `run-pending-review` | `random-tail-eda/`, `projection-structure/` | Current top row has `sys = 0.86258589584944`; review near-miss language. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split and grouped validation | `run-pending-review` | `prediction-ranking/` | Current grouped holdout artifact exists; review adequacy for thesis use. |
| source/provenance leakage checks | `run-pending-review` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/factor tests, metadata-only baselines, and overlays exist. |
| provenance-only and metadata-only baselines | `run-pending-review` | `prediction-ranking/` | Current metadata-only baselines exist. |
| permutation and bootstrap checks | `run-pending-review` | `statistical-associations/`, `prediction-ranking/` | Current association permutation/bootstrap and ranking permutation artifacts exist. |

## Direct Random Sampling

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind generic random sampling | `run-pending-review` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `4096` generic rows and no positives. |
| blind random Lagrangian-product sampling | `run-pending-review` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `10240` product rows and no positives. |
| independent same-distribution reruns | `defer` | none | Useful for stability only after current-schema closure shows which facts need stability. |
| broader height intervals, facet ranges, product side ranges, or new distributions | `defer` | none | This is plausible next research work, but first close the current retained producer contract. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| ridge regression | `run-pending-review` | `prediction-ranking/` | Current geometry-only ridge model ran. |
| random forest regression | `run-pending-review` | `prediction-ranking/` | Current geometry-only RF model ran. |
| metadata-only prediction baselines | `run-pending-review` | `prediction-ranking/` | Current metadata-only baselines ran. |
| high-tail classification and classifier variants | `defer` | none | No positive class exists; near-tail classification mostly duplicates ranking until a generated-candidate follow-up is promoted. |
| generated-candidate proposer loop | `defer` | none | Reopen if rerun ranking evidence has enough expected thesis value to justify producing new candidates. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `run-pending-review` | `projection-structure/` | Current PCA artifact exists. |
| k-means clustering | `run-pending-review` | `projection-structure/` | Current k-means artifact exists. |
| isolation-style anomaly detection | `run-pending-review` | `projection-structure/` | Current isolation-style anomaly artifact exists. |
| hierarchical clustering, mixtures, density estimation, one-class classification | `defer` | none | Reopen only if the active projection/anomaly packet shows robust unexplained high-tail structure. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `run-pending-review` | `statistical-associations/` | Current scalar screen ran on `111` nonconstant covariates. |
| source/facet/product factor tests | `run-pending-review` | `statistical-associations/` | Current factor tests ran with current provenance fields. |
| omega and two-face scalar hypotheses | `run-pending-review` | `prepare/`, `statistical-associations/` | Current association screen includes omega and two-face features. |
| Kendall, partial correlation, FDR variants | `defer` | none | Add only if the active rerun promotes a scalar effect needing robustness checks. |

## Rejected Or Out Of Scope

| Checklist family | Current disposition | Reason and reopen trigger |
| --- | --- | --- |
| ascent, continuation, endpoint stability, attractors, basins, local-behavior panels | `out-of-scope` | Removed from active datascience slice; use a separate thesis slice only if Jörn explicitly reopens it. |
| forecasting/time-series | `reject` | No time-indexed forecasting target. |
| bandits or reinforcement learning | `reject` | No online environment or reward loop is defined. |
| MCMC/Bayesian posterior families | `reject` | No probabilistic model is currently part of the thesis claim. |
| multi-fidelity optimization | `reject` | No cheaper faithful proxy for `sys` is established. |

## Current Gaps Before Closure

1. Review active method packet interpretations and update README state.
2. Decide whether the full prepared table should be retained or regenerated on
   demand.
3. Do not claim closure over arbitrary random distributions unless a new
   distribution-design batch is run and reviewed.
