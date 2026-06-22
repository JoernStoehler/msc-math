# Random-Only Method Dispositions

Purpose: disposition ledger for the checklist families in
`method-coverage-checklist.md`, scoped to trusted random/product rows and
bounded non-gradient perturbation panels. This file is not a replacement for
method packet READMEs or artifacts. It records why a checklist family is
currently considered run, pending rerun, deferred, rejected, or out of scope for
the random-only feature-space closure goal.

Epistemic status: current branch control surface. A full scoped random/product
prepare rebuild and method rerun completed on 2026-06-22 using
`/tmp/sys-ds-random-only-full`: `4096` random rows, `10240` random-product rows,
zero ascent/continuation/computed-observation rows, max `sys =
0.86258589584944`, and zero `sys > 1` rows. The prepared table is reproducible
from `prepare/build-random-only-slice.sh full` but is not currently retained as
a checked-in table artifact. The remaining closure gate is method/statistics
review, not another schema-refresh rerun.

## Disposition Vocabulary

- `run-current-artifact`: current method packet has retained evidence from the
  current scoped random/product prepared table or checked-in artifacts.
- `run-pending-review`: code/packet surface and current artifacts exist, but
  method/statistics review has not yet accepted the interpretation for thesis
  use.
- `run-pending-retention-decision`: current artifacts were produced from a
  reproducible `/tmp` prepared table; decide whether to retain the prepared
  table in repo/LFS or keep it as regenerated input.
- `run-pending-rerun`: code/packet surface exists for this branch, but a
  required rerun is still pending.
- `smoke-only`: route works on a tiny panel but is not broad method coverage.
- `defer`: plausible method family, lower current thesis value than reviewing
  and integrating the completed full scoped reruns, with reopen trigger.
- `reject`: not useful for this search interface unless assumptions change.
- `out-of-scope`: belongs to ascent/continuation/attractor work or another
  thesis slice, not clean random/product feature-space closure.

## Baseline And EDA

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| target predicate scan | `run-current-artifact` plus `run-pending-review` | `scan-sys-gt-1/`, `random-tail-eda/` | Full scoped random/product scan found zero `sys > 1`; review still needs to check whether thesis wording uses the result with the correct random-distribution caveat. |
| target distribution, quantiles, tail summaries | `run-current-artifact` plus `run-pending-review` | `random-tail-eda/` | Full scoped current-schema EDA exists for random/product rows; review should check tail interpretation and sample-size language. |
| grouped source/facet/product summaries | `run-current-artifact` plus `run-pending-review` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product EDA, factor tests, metadata baselines, and projection overlays reran on current scoped tables; review should check which subgroup statements are thesis-facing. |
| missingness, duplicate, schema, row-count checks | `run-current-artifact` plus `run-pending-review` | `trusted-random-dataset/`, `random-tail-eda/` | Current scoped build verifies `4096 + 10240` rows and zero ascent/computed-observation rows; review should decide whether the `/tmp` prepared table must be retained. |
| outlier and near-miss inspection | `run-current-artifact` plus `run-pending-review` | `random-tail-eda/`, `projection-structure/` | Top-tail rows and anomaly rows are recorded from the scoped full rerun; review should check whether any near-miss claim is justified. |
| histograms, tail plots, faceting, overlays | `run-current-artifact` plus `run-pending-review` | `random-tail-eda/`, `projection-structure/` | Histogram/tail plots, PCA-by-sys plot, and source/facet/product overlays reran on the scoped full table. |
| heatmaps, parallel coordinates, residual plots, geometric/orbit pictures | `defer` | none | Lower value than reviewing the current full scoped artifacts; reopen only if a model or association result needs a specific diagnostic view to support thesis wording. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split, holdout, grouped validation | `run-current-artifact` plus `run-pending-review` | `prediction-ranking/` | Grouped holdout and metadata-only baselines reran on current scoped tables; review should check grouped split adequacy. |
| source/provenance leakage checks | `run-current-artifact` plus `run-pending-review` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Filter excludes ascent/continuation; source factor tests, metadata-only baselines, and overlays reran on the scoped table. |
| provenance-only and metadata-only baselines | `run-current-artifact` plus `run-pending-review` | `prediction-ranking/` | Metadata-only ridge/random-forest baselines reran on current scoped provenance columns. |
| permutation tests | `run-current-artifact` plus `run-pending-review` | `statistical-associations/`, `prediction-ranking/` | Association family-max and prediction enrichment permutations reran; review should check whether permutation counts are enough for thesis-facing claims. |
| bootstrap intervals | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Product-minus-random mean bootstrap reran on current source metadata. |
| p-values, scalar test statistics, multiple-comparison caveat | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Pearson/Spearman p-values, family-max permutation, and source factor tests reran; family-wise interpretation remains caveated. |
| random-to-endpoint transfer and endpoint residual checks | `out-of-scope` | ascent/local behavior packets | Requires endpoint/ascent trust model; excluded from clean random-only proposer claims. |

## Direct Search And Optimization

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind random generic sampling | `run-current-artifact` plus `run-pending-review` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | `4096` generic random rows in the scoped full rerun; no positives. |
| blind random Lagrangian-product sampling | `run-current-artifact` plus `run-pending-review` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | `10240` product rows in the scoped full rerun; no positives. |
| rejection/acceptance calibration | `run-current-artifact` | `produce/README.md`, `random-tail-eda/` generator contract | Producer contract records accepted samples and generator parameters; no full new producer run in this branch. |
| bounded non-gradient perturbation | `smoke-only` | `non-gradient-perturbation/` | Tiny hash-selected random-direction panel ran; larger panel deferred unless near-threshold/improvement evidence makes it high value. |
| Latin-hypercube, space-filling, pattern search, Nelder-Mead, derivative-free trust-region, surrogate/local/global wrappers | `defer` | none | Plausible direct-search families, but no cheap faithful parameterization currently has higher value than reviewing the full scoped random/product evidence. Reopen if random-only model suggests a concrete low-dimensional proposer target. |
| Bayesian/surrogate-guided optimization and branch-and-bound variants | `defer` | none | Higher setup cost and no current validated cheap acquisition/proxy; reopen only if supervised ranking produces a concrete generated-candidate loop. |
| gradient-ascent, continuation, multistart/restart ascent variants | `out-of-scope` | ascent-specific packets/session | Excluded from trusted random/product feature-space closure; needs separate trusted ascent/local-maximum data. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| linear/ridge regression | `run-current-artifact` plus `run-pending-review` | `prediction-ranking/` | Ridge geometry-only model reran on the scoped full table. |
| random forest regression | `run-current-artifact` plus `run-pending-review` | `prediction-ranking/` | Geometry-only RF reran on the scoped full table with permutation enrichment. |
| metadata-only prediction baselines | `run-current-artifact` plus `run-pending-review` | `prediction-ranking/` | Metadata-only models reran on current explicit provenance fields. |
| high-tail classification, logistic/LDA/QDA/naive Bayes/kNN/SVM/kernel classifiers | `defer` | none | No positive class exists and near-tail classification would mostly duplicate ranking/enrichment until a generated-candidate follow-up is promoted. |
| lasso/elastic net, splines/GAMs, local/kernel regression, PCR/PLS, subset/stepwise, bagging/boosting/model averaging | `defer` | none | Variants are standard but unlikely to change thesis claim before current ridge/RF evidence is reviewed; reopen if ridge/RF disagree sharply or a linear interpretable rule becomes thesis-relevant. |
| decision trees and interpretable tail rules | `defer` | none | Useful only if review of the current rerun reveals a stable high-tail split worth explaining. |
| Gaussian-process/Bayesian predictors and uncertainty ranking | `defer` | none | Higher cost; needs a candidate-generation loop or low-dimensional input surface. |
| neural predictors/autoencoders | `defer` | none | Lower interpretability and higher maintenance cost; no current evidence they improve thesis value. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `run-current-artifact` plus `run-pending-review` | `projection-structure/` | PCA and metadata overlays reran on the scoped full table. |
| k-means clustering | `run-current-artifact` plus `run-pending-review` | `projection-structure/` | K-means summaries reran on the scoped full table. |
| isolation-style anomaly detection | `run-current-artifact` plus `run-pending-review` | `projection-structure/` | Isolation forest overlap reran on the scoped full table. |
| hierarchical clustering, Gaussian mixtures, soft clustering, density estimation, one-class classification | `defer` | none | Would duplicate projection/anomaly questions unless PCA/k-means/isolation rerun shows a robust unexplained high-tail structure. |
| normalizing flows/expressive density models | `defer` | none | High setup/interpretation cost; no current thesis-facing need. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Full scoped current-schema artifact exists; review should check which scalar effects survive interpretation. |
| Kendall, partial correlation | `defer` | none | Spearman and permutation family-max are enough unless review promotes a scalar effect needing robustness checks. |
| mean/rank group comparisons, ANOVA/Kruskal-Wallis | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Source/provenance factor tests reran on current explicit provenance fields. |
| chi-square/Fisher exact association tests | `defer` | none | No positive class; only useful if categorical high-tail event bins are promoted. |
| family-wise/FDR controls | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Family-max permutation reran for strongest rank association; add FDR only if many individual scalar claims are thesis-facing. |
| omega-style scalar geometry hypotheses | `run-current-artifact` plus `run-pending-review` | `prepare/`, `statistical-associations/` | New omega matrix/sign/alignment features are in the scoped full table and association rerun. |
| source-wise, feature-group-wise, family-wise association scans | `run-current-artifact` plus `run-pending-review` | `statistical-associations/` | Family inventory and source factor tests reran on current feature schema. |

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

1. Method/statistics review must check the full scoped rerun artifacts and the
   interpretation in each packet.
2. Decide whether `/tmp/sys-ds-random-only-full` should be retained as a
   checked-in/LFS prepared table or remain a reproducible generated input.
3. Thesis wording must not use this ledger to claim closure over gradient
   ascent, local maxima, attractors, basin structure, or arbitrary random
   polytope distributions.
