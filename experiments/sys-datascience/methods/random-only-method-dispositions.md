# Random-Only Method Dispositions

Purpose: disposition ledger for the checklist families in
`method-coverage-checklist.md`, scoped to trusted random/product rows and
bounded non-gradient perturbation panels. This file is not a replacement for
method packet READMEs or artifacts. It records why a checklist family is
currently considered run, pending rerun, deferred, rejected, or out of scope for
the random-only feature-space closure goal.

Epistemic status: current branch control surface. Full prepare-stage
retained-table rebuild and post-rebuild method reruns are still pending for new
prepare columns, so `run-pending-rerun` rows are not updated thesis evidence
for those columns.

## Disposition Vocabulary

- `run-current-artifact`: current method packet has retained evidence on the
  checked-in tables, though it may still need refresh after schema changes.
- `run-pending-rerun`: code/packet surface exists for this branch, but the
  prepare-stage rebuild or post-rebuild retained-table rerun is pending.
- `smoke-only`: route works on a tiny panel but is not broad method coverage.
- `defer`: plausible method family, lower current thesis value than
  post-rebuild retained rerun and review, with reopen trigger.
- `reject`: not useful for this search interface unless assumptions change.
- `out-of-scope`: belongs to ascent/continuation/attractor work or another
  thesis slice, not clean random/product feature-space closure.

## Baseline And EDA

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| target predicate scan | `run-current-artifact` | `scan-sys-gt-1/` | Direct random-only `sys > 1` scan exists; rerun after prepare rebuild. |
| target distribution, quantiles, tail summaries | `run-current-artifact` plus `run-pending-rerun` | `random-tail-eda/` | Current old-schema full-table EDA exists; prepare rebuild is needed before explicit provenance fields can appear. |
| grouped source/facet/product summaries | `run-pending-rerun` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product EDA, factor tests, metadata baselines, and projection overlays have old-schema full-table artifacts; prepare rebuild is needed for explicit provenance fields. |
| missingness, duplicate, schema, row-count checks | `run-current-artifact` plus `run-pending-rerun` | `trusted-random-dataset/`, `random-tail-eda/` | Existing filter audit and duplicate counts exist; explicit provenance fields need prepare rebuild. |
| outlier and near-miss inspection | `run-current-artifact` plus `run-pending-rerun` | `random-tail-eda/`, `projection-structure/` | Top-tail rows and anomaly rows are recorded; rerun after new prepare feature schema. |
| histograms, tail plots, faceting, overlays | `run-current-artifact` plus `run-pending-rerun` | `random-tail-eda/`, `projection-structure/` | Histogram/tail plots, PCA-by-sys plot, and source/facet/product overlays have old-schema full-table artifacts; rerun after new prepare feature schema. |
| heatmaps, parallel coordinates, residual plots, geometric/orbit pictures | `defer` | none | Lower value before post-rebuild reruns; reopen only if a model or association result needs a specific diagnostic view to support thesis wording. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split, holdout, grouped validation | `run-current-artifact` plus `run-pending-rerun` | `prediction-ranking/` | Grouped holdout and metadata-only baselines have old-schema full-table artifacts; rerun after new prepare feature schema. |
| source/provenance leakage checks | `run-pending-rerun` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Filter excludes ascent/continuation; source factor tests, metadata-only baselines, and overlays have old-schema full-table artifacts. |
| provenance-only and metadata-only baselines | `run-pending-rerun` | `prediction-ranking/` | Metadata-only ridge/random-forest baselines have old-schema full-table artifacts; rerun after explicit provenance fields exist. |
| permutation tests | `run-current-artifact` plus `run-pending-rerun` | `statistical-associations/`, `prediction-ranking/` | Association family-max and prediction enrichment permutations have old-schema full-table artifacts; rerun after new prepare feature schema. |
| bootstrap intervals | `run-current-artifact` | `statistical-associations/` | Product-minus-random mean bootstrap exists; rerun pending for updated source metadata. |
| p-values, scalar test statistics, multiple-comparison caveat | `run-current-artifact` plus `run-pending-rerun` | `statistical-associations/` | Pearson/Spearman p-values, family-max permutation, and source factor tests have old-schema full-table artifacts; family-wise interpretation remains caveated. |
| random-to-endpoint transfer and endpoint residual checks | `out-of-scope` | ascent/local behavior packets | Requires endpoint/ascent trust model; excluded from clean random-only proposer claims. |

## Direct Search And Optimization

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind random generic sampling | `run-current-artifact` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | `4096` retained generic random rows; no positives in old retained artifact. |
| blind random Lagrangian-product sampling | `run-current-artifact` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | `10240` retained product rows; no positives in old retained artifact. |
| rejection/acceptance calibration | `run-current-artifact` | `produce/README.md`, `random-tail-eda/` generator contract | Producer contract records accepted samples and generator parameters; no full new producer run in this branch. |
| bounded non-gradient perturbation | `smoke-only` | `non-gradient-perturbation/` | Tiny hash-selected random-direction panel ran; larger panel deferred unless near-threshold/improvement evidence makes it high value. |
| Latin-hypercube, space-filling, pattern search, Nelder-Mead, derivative-free trust-region, surrogate/local/global wrappers | `defer` | none | Plausible direct-search families, but no cheap faithful parameterization has higher value than post-rebuild method reruns. Reopen if random-only model suggests a concrete low-dimensional proposer target. |
| Bayesian/surrogate-guided optimization and branch-and-bound variants | `defer` | none | Higher setup cost and no current validated cheap acquisition/proxy; reopen only if supervised ranking produces a concrete generated-candidate loop. |
| gradient-ascent, continuation, multistart/restart ascent variants | `out-of-scope` | ascent-specific packets/session | Excluded from trusted random/product feature-space closure; needs separate trusted ascent/local-maximum data. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| linear/ridge regression | `run-current-artifact` plus `run-pending-rerun` | `prediction-ranking/` | Ridge geometry-only model has an old-schema full-table artifact; rerun after new prepare feature schema. |
| random forest regression | `run-current-artifact` plus `run-pending-rerun` | `prediction-ranking/` | Geometry-only RF has an old-schema full-table artifact with permutation enrichment; rerun after new prepare feature schema. |
| metadata-only prediction baselines | `run-pending-rerun` | `prediction-ranking/` | Old-schema full-table artifact exists; rerun after explicit provenance fields exist. |
| high-tail classification, logistic/LDA/QDA/naive Bayes/kNN/SVM/kernel classifiers | `defer` | none | No positive class exists and near-tail classification would mostly duplicate ranking/enrichment until a generated-candidate follow-up is promoted. |
| lasso/elastic net, splines/GAMs, local/kernel regression, PCR/PLS, subset/stepwise, bagging/boosting/model averaging | `defer` | none | Variants are standard but unlikely to change thesis claim before post-rebuild reruns; reopen if ridge/RF disagree sharply or a linear interpretable rule becomes thesis-relevant. |
| decision trees and interpretable tail rules | `defer` | none | Useful only if post-rebuild rerun reveals a stable high-tail split worth explaining. |
| Gaussian-process/Bayesian predictors and uncertainty ranking | `defer` | none | Higher cost; needs a candidate-generation loop or low-dimensional input surface. |
| neural predictors/autoencoders | `defer` | none | Lower interpretability and higher maintenance cost; no current evidence they improve thesis value. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `run-current-artifact` plus `run-pending-rerun` | `projection-structure/` | PCA and metadata overlays have old-schema full-table artifacts; rerun after new prepare feature schema. |
| k-means clustering | `run-current-artifact` plus `run-pending-rerun` | `projection-structure/` | K-means summaries have old-schema full-table artifacts; rerun after new prepare feature schema. |
| isolation-style anomaly detection | `run-current-artifact` plus `run-pending-rerun` | `projection-structure/` | Isolation forest overlap has an old-schema full-table artifact; rerun after new prepare feature schema. |
| hierarchical clustering, Gaussian mixtures, soft clustering, density estimation, one-class classification | `defer` | none | Would duplicate projection/anomaly questions unless PCA/k-means/isolation rerun shows a robust unexplained high-tail structure. |
| normalizing flows/expressive density models | `defer` | none | High setup/interpretation cost; no current thesis-facing need. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `run-current-artifact` plus `run-pending-rerun` | `statistical-associations/` | Old-schema full-table artifact exists; rerun pending with new prepare feature schema. |
| Kendall, partial correlation | `defer` | none | Spearman and permutation family-max are enough unless post-rebuild rerun promotes a scalar effect needing robustness checks. |
| mean/rank group comparisons, ANOVA/Kruskal-Wallis | `run-pending-rerun` | `statistical-associations/` | Source/provenance factor tests have an old-schema full-table artifact; rerun after explicit provenance fields exist. |
| chi-square/Fisher exact association tests | `defer` | none | No positive class; only useful if categorical high-tail event bins are promoted. |
| family-wise/FDR controls | `run-current-artifact` plus `run-pending-rerun` | `statistical-associations/` | Family-max permutation has an old-schema full-table artifact for strongest rank association; add FDR only if many individual scalar claims are thesis-facing. |
| omega-style scalar geometry hypotheses | `run-pending-rerun` | `prepare/`, `statistical-associations/` | New omega matrix/sign/alignment features implemented and smoke-tested; prepare rebuild and post-rebuild rerun pending. |
| source-wise, feature-group-wise, family-wise association scans | `run-pending-rerun` | `statistical-associations/` | Family inventory and source factor tests have old-schema full-table artifacts; post-rebuild rerun pending for new feature schema. |

## Sequence, Trajectory, Landscape, And Attractor Probes

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| sequence and trajectory methods | `out-of-scope` | none for clean random-only slice | Trusted random/product rows are unordered samples; trajectory methods need perturbation/ascent traces or a separate panel. |
| endpoint duplicate/clustering/perturbation/attractor/basin probes | `out-of-scope` | ascent/local-max work | Explicitly excluded from clean random-only feature-space closure until trusted ascent/local-maximum data exists. |
| symmetry, centering, scaling, normalization ablations | `defer` | feature ledger translation-canonized robustness row | Volume normalization is implemented; translation canonization and broader symmetry ablations are deferred unless support-representative-sensitive findings become thesis-critical. |
| HKO positive-region random walk | `out-of-scope` | HKO-local thesis slice | Not random/product datascience closure; reopen only if thesis wording asks to compare against HKO-local evidence. |

## Usually Rejected Families

| Checklist family | Current disposition | Reason and reopen trigger |
| --- | --- | --- |
| forecasting/time-series as forecasting | `reject` | No time-indexed forecasting target in the random/product table. |
| bandits or reinforcement learning | `reject` | No online environment or reward loop is defined. |
| MCMC, variational inference, Laplace/local posterior approximations, hierarchical Bayes | `reject` | No probabilistic model is currently part of the thesis claim; reopen only if a probabilistic generator model becomes thesis-facing. |
| multi-fidelity optimization | `reject` | No cheaper faithful proxy for `sys` is currently established. |

## Current Gaps Before Closure

1. Prepare-stage retained-table rebuild and post-rebuild method reruns are
   still pending for new prepare columns.
2. Method/statistics review must run after post-rebuild reruns and after final
   relevant code changes.
3. Thesis wording must not use this ledger to claim closure over gradient
   ascent, local maxima, attractors, basin structure, or arbitrary random
   polytope distributions.
