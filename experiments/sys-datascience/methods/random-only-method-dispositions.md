# Random-Only Method Dispositions

Purpose: disposition ledger for the checklist families in
`method-coverage-checklist.md`, scoped to trusted random/product rows and
bounded non-gradient perturbation panels. This file is not a replacement for
method packet READMEs or artifacts. It records why a checklist family is
currently considered run, pending rerun, deferred, rejected, or out of scope for
the random-only feature-space closure goal.

Epistemic status: current random-only control surface. A full scoped
random/product prepare rebuild and method rerun completed on 2026-06-22 using a
scratch table at `/tmp/sys-ds-random-only-full`: `4096` random rows, `10240`
random-product rows, zero ascent/continuation/computed-observation rows, max
`sys = 0.86258589584944`, and zero `sys > 1` rows. Since then, prepare grew
additional reusable omega/area summary columns and the local wrapup session did
not run another full prepare after CPU-budget warnings. The remaining closure
gate is a full scoped current-schema rerun followed by method/statistics review.

## Disposition Vocabulary

- `run-current-artifact`: current method packet has retained evidence from the
  current scoped random/product prepared table or checked-in artifacts.
- `run-pending-review`: code/packet surface and current artifacts exist, but
  method/statistics review has not yet accepted the interpretation for thesis
  use.
- `run-pending-retention-decision`: current artifacts were produced from a
  reproducible `/tmp` prepared table; decide whether to retain the prepared
  table in repo/LFS or keep it as regenerated input.
- `run-pending-rerun`: code/packet surface exists, but a
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
| target predicate scan | `run-pending-rerun` | `scan-sys-gt-1/`, `random-tail-eda/` | Previous full scoped random/product scan found zero `sys > 1`; refresh the scan with the current prepare schema before thesis use. |
| target distribution, quantiles, tail summaries | `run-pending-rerun` | `random-tail-eda/` | Previous full scoped EDA exists for random/product rows, but prepare-schema additions require a current-schema rerun before thesis use. Review should then check tail interpretation and sample-size language. |
| grouped source/facet/product summaries | `run-pending-rerun` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product EDA, factor tests, metadata baselines, and projection overlays exist from the previous full scoped rerun; rerun against the current prepare schema before deciding which subgroup statements are thesis-facing. |
| missingness, duplicate, schema, row-count checks | `run-pending-rerun` | `trusted-random-dataset/`, `random-tail-eda/` | Previous scoped build verified `4096 + 10240` rows and zero ascent/computed-observation rows; rerun and fingerprint the current prepare schema before deciding retention. |
| outlier and near-miss inspection | `run-pending-rerun` | `random-tail-eda/`, `projection-structure/` | Top-tail rows and anomaly rows were recorded from the previous scoped full rerun; refresh before using any near-miss claim. |
| histograms, tail plots, faceting, overlays | `run-pending-rerun` | `random-tail-eda/`, `projection-structure/` | Histogram/tail plots, PCA-by-sys plot, and source/facet/product overlays exist from the previous full scoped table; refresh them after the current-schema prepare rerun. |
| heatmaps, parallel coordinates, residual plots, geometric/orbit pictures | `defer` | none | Lower value than reviewing the current full scoped artifacts; reopen only if a model or association result needs a specific diagnostic view to support thesis wording. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split, holdout, grouped validation | `run-pending-rerun` | `prediction-ranking/` | Grouped holdout and metadata-only baselines ran on the previous scoped table; rerun before checking grouped split adequacy for thesis use. |
| source/provenance leakage checks | `run-pending-rerun` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Filter excludes ascent/continuation; source factor tests, metadata-only baselines, and overlays need current-schema refresh. |
| provenance-only and metadata-only baselines | `run-pending-rerun` | `prediction-ranking/` | Metadata-only ridge/random-forest baselines ran on previous scoped provenance columns; rerun with current tables. |
| permutation tests | `run-pending-rerun` | `statistical-associations/`, `prediction-ranking/` | Association family-max and prediction enrichment permutations ran previously; rerun before reviewing whether counts are enough for thesis-facing claims. |
| bootstrap intervals | `run-pending-rerun` | `statistical-associations/` | Product-minus-random mean bootstrap ran previously; rerun with current tables. |
| p-values, scalar test statistics, multiple-comparison caveat | `run-pending-rerun` | `statistical-associations/` | Pearson/Spearman p-values, family-max permutation, and source factor tests ran previously; rerun before reviewing family-wise interpretation. |
| random-to-endpoint transfer and endpoint residual checks | `out-of-scope` | ascent/local behavior packets | Requires endpoint/ascent trust model; excluded from clean random-only proposer claims. |

## Direct Search And Optimization

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind random generic sampling | `run-pending-rerun` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Previous scoped full rerun included `4096` generic random rows and no positives; refresh with the current prepare schema. |
| blind random Lagrangian-product sampling | `run-pending-rerun` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Previous scoped full rerun included `10240` product rows and no positives; refresh with the current prepare schema. |
| rejection/acceptance calibration | `run-current-artifact` | `produce/README.md`, `random-tail-eda/` generator contract | Producer contract records accepted samples and generator parameters; no full new producer run was part of the retained method rerun. |
| bounded non-gradient perturbation | `smoke-only` | `non-gradient-perturbation/` | Tiny hash-selected random-direction panel ran; larger panel deferred unless near-threshold/improvement evidence makes it high value. |
| Latin-hypercube, space-filling, pattern search, Nelder-Mead, derivative-free trust-region, surrogate/local/global wrappers | `defer` | none | Plausible direct-search families, but no cheap faithful parameterization currently has higher value than reviewing the full scoped random/product evidence. Reopen if random-only model suggests a concrete low-dimensional proposer target. |
| Bayesian/surrogate-guided optimization and branch-and-bound variants | `defer` | none | Higher setup cost and no current validated cheap acquisition/proxy; reopen only if supervised ranking produces a concrete generated-candidate loop. |
| gradient-ascent, continuation, multistart/restart ascent variants | `out-of-scope` | ascent-specific packets/session | Excluded from trusted random/product feature-space closure; needs separate trusted ascent/local-maximum data. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| linear/ridge regression | `run-pending-rerun` | `prediction-ranking/` | Ridge geometry-only model ran on the previous scoped full table; rerun with current features. |
| random forest regression | `run-pending-rerun` | `prediction-ranking/` | Geometry-only RF ran on the previous scoped full table with permutation enrichment; rerun with current features. |
| metadata-only prediction baselines | `run-pending-rerun` | `prediction-ranking/` | Metadata-only models ran on previous explicit provenance fields; rerun with current tables. |
| high-tail classification, logistic/LDA/QDA/naive Bayes/kNN/SVM/kernel classifiers | `defer` | none | No positive class exists and near-tail classification would mostly duplicate ranking/enrichment until a generated-candidate follow-up is promoted. |
| lasso/elastic net, splines/GAMs, local/kernel regression, PCR/PLS, subset/stepwise, bagging/boosting/model averaging | `defer` | none | Variants are standard but unlikely to change thesis claim before current ridge/RF evidence is reviewed; reopen if ridge/RF disagree sharply or a linear interpretable rule becomes thesis-relevant. |
| decision trees and interpretable tail rules | `defer` | none | Useful only if review of the current rerun reveals a stable high-tail split worth explaining. |
| Gaussian-process/Bayesian predictors and uncertainty ranking | `defer` | none | Higher cost; needs a candidate-generation loop or low-dimensional input surface. |
| neural predictors/autoencoders | `defer` | none | Lower interpretability and higher maintenance cost; no current evidence they improve thesis value. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `run-pending-rerun` | `projection-structure/` | PCA and metadata overlays ran on the previous scoped full table; rerun with current features. |
| k-means clustering | `run-pending-rerun` | `projection-structure/` | K-means summaries ran on the previous scoped full table; rerun with current features. |
| isolation-style anomaly detection | `run-pending-rerun` | `projection-structure/` | Isolation forest overlap ran on the previous scoped full table; rerun with current features. |
| hierarchical clustering, Gaussian mixtures, soft clustering, density estimation, one-class classification | `defer` | none | Would duplicate projection/anomaly questions unless PCA/k-means/isolation rerun shows a robust unexplained high-tail structure. |
| normalizing flows/expressive density models | `defer` | none | High setup/interpretation cost; no current thesis-facing need. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `run-pending-rerun` | `statistical-associations/` | Previous full scoped artifact exists; current-schema feature additions require rerun before reviewing which scalar effects survive interpretation. |
| Kendall, partial correlation | `defer` | none | Spearman and permutation family-max are enough unless review promotes a scalar effect needing robustness checks. |
| mean/rank group comparisons, ANOVA/Kruskal-Wallis | `run-pending-rerun` | `statistical-associations/` | Source/provenance factor tests ran previously; rerun with current explicit provenance fields. |
| chi-square/Fisher exact association tests | `defer` | none | No positive class; only useful if categorical high-tail event bins are promoted. |
| family-wise/FDR controls | `run-pending-rerun` | `statistical-associations/` | Family-max permutation ran previously for strongest rank association; rerun before deciding whether FDR is needed for thesis-facing scalar claims. |
| omega-style scalar geometry hypotheses | `run-pending-rerun` | `prepare/`, `statistical-associations/` | New omega matrix/sign/alignment features are implemented, but full scoped association rerun is pending. |
| source-wise, feature-group-wise, family-wise association scans | `run-pending-rerun` | `statistical-associations/` | Family inventory and source factor tests need a current-schema rerun. |

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

1. Regenerate the full scoped random/product prepared table against the current
   prepare schema.
2. Rerun the method packets that consume prepared-table columns, then perform
   method/statistics review of the artifacts and packet interpretations.
3. Decide whether the scratch prepared table used for the scoped rerun should
   be retained as a checked-in/LFS prepared table or remain a reproducible
   generated input.
4. Thesis wording must not use this ledger to claim closure over gradient
   ascent, local maxima, attractors, basin structure, or arbitrary random
   polytope distributions.
