# Datascience Method Coverage Checklist

Status: recall checklist, not a taxonomy and not a result ledger.

Use this file to avoid forgetting methods, tactics, patterns, and concepts
when closing the sys-landscape datascience method table. It is intentionally
redundant, mixed-granularity, and not ordered except for loose grouping.

This file is not a result ledger. When an item becomes active, create or update
a method packet `README.md` with the method-specific question, inputs, leakage
guards, observations, caveats, current disposition, thesis use, and reopen
trigger.

Disposition words below are planning hints only:

- `open`: not currently closed on the retained tables.
- `used`: appears in current producer/table surfaces, but may still need a
  method packet README.
- `defer`: plausible but lower current thesis value.
- `reject`: not applicable unless a missing interface or data source appears.

## Baseline And EDA

- `used` target predicate scan: rows with `sys > 1`.
- `open` target distribution, quantiles, and tail summaries.
- `open` grouped summaries by source, family, optimizer, lineage, and role.
- `open` missingness, duplicate, schema, and row-count checks.
- `open` outlier and near-miss inspection.
- `open` scatter plots and pairwise relation views.
- `open` histograms, density views, boxplots, and tail plots.
- `open` heatmaps and matrix views.
- `open` parallel coordinates and other multivariate views.
- `open` residual and diagnostic plots.
- `open` visual inspection of geometric or orbit pictures.
- `open` faceting, conditioning, overlays, and annotation choices for
  exploratory plots.

## Leakage, Validation, And Null Checks

- `open` train/test split and grouped cross-validation.
- `open` holdout validation.
- `open` lineage, root-group, seed, and source leakage checks.
- `open` provenance-only and metadata-only baselines.
- `open` random-to-endpoint transfer checks.
- `open` endpoint-only residual checks beyond metadata.
- `open` permutation tests.
- `open` bootstrap checks.
- `open` p-values and test-statistic summaries when a scalar test is reported.
- `open` simple confidence intervals or standard errors where they clarify a
  table result.
- `open` multiple-comparison or false-discovery caveat if many scalar tests are
  reported.

## Direct Search And Optimization

- `used` blind random generic sampling.
- `used` blind random Lagrangian-product sampling.
- `open` Latin-hypercube or other space-filling random designs.
- `used` rejection or acceptance calibration for random producers.
- `used` fixed-F gradient ascent from random starts.
- `used` fixed-F product gradient ascent from random starts.
- `used` variable-F continuation.
- `open` multistart and restart variants.
- `open` random perturbation followed by ascent.
- `open` rejection sampling with cheap predicates.
- `open` local pattern search.
- `open` generalized pattern search and mesh adaptive direct search.
- `open` Nelder-Mead or simplex-style search.
- `open` derivative-free trust-region or surrogate-assisted local search.
- `open` local quadratic or interpolation-model search.
- `open` stochastic global-search wrappers.
- `open` hybrid local/global black-box search.
- `open` constrained black-box search.
- `defer` deterministic branch-and-bound style black-box search.
- `defer` Bayesian or surrogate-guided optimization.
- `defer` batch, constrained, multi-fidelity, or high-dimensional Bayesian
  optimization variants.

## Gradient And Continuation Tactics

- `used` gradient ascent.
- `open` line-search variants.
- `open` trust-region variants.
- `open` Newton, quasi-Newton, and nonlinear conjugate-gradient variants.
- `open` projected-gradient or active-set variants.
- `open` penalty, barrier, interior-point, and sequential-quadratic-programming
  variants.
- `open` subgradient, bundle, and proximal-style variants.
- `open` step-size and distance ablations.
- `open` minimum-sigma versus multi-sigma gradient choices.
- `open` zero-padded sigma handling.
- `open` `D sys`, `D beta`, and related first-order directions.
- `open` simple parameter continuation.
- `open` predictor-corrector continuation.
- `open` arc-length continuation.
- `open` branch following and branch switching.
- `open` witness-guided or parent-cache continuation.
- `open` homotopy or solution continuation under changing problem data.

## Supervised Prediction And Rules

- `open` linear regression.
- `open` ridge, lasso, elastic net, and shrinkage models.
- `open` polynomial or basis-expansion regression.
- `open` splines and generalized additive models.
- `open` local regression and kernel smoothing.
- `open` principal-components regression and partial least squares.
- `open` subset selection and stepwise selection.
- `open` random forest regression.
- `open` bagging.
- `open` boosting regression.
- `open` model averaging.
- `open` decision trees and interpretable tail rules.
- `open` logistic classification.
- `open` endpoint-vs-random classification.
- `open` high-tail or near-miss classification.
- `open` linear discriminant and quadratic discriminant analysis.
- `open` naive Bayes.
- `open` k-nearest neighbors.
- `open` prototype-based methods.
- `open` support vector machines.
- `open` kernel methods.
- `open` probabilistic linear/logistic models.
- `open` Gaussian-process or Bayesian-regression predictors.
- `open` posterior predictive uncertainty as a candidate-ranking signal.
- `defer` multilayer perceptrons and other neural predictors.
- `defer` autoencoders and learned latent representations.

## Unsupervised, Density, And Anomaly Methods

- `open` PCA.
- `open` low-dimensional projections.
- `open` factor analysis and latent factors.
- `open` k-means.
- `open` hierarchical clustering.
- `open` Gaussian mixtures and soft clustering.
- `open` density estimation.
- `open` density-based anomaly detection.
- `open` one-class classification.
- `open` isolation-style anomaly methods.
- `defer` normalizing flows or expressive density models.

## Statistical Associations

- `open` Pearson correlation.
- `open` Spearman and Kendall rank correlation.
- `open` partial correlation.
- `open` t-tests and Welch-style mean comparisons.
- `open` Mann-Whitney or rank-sum tests.
- `open` ANOVA-style and Kruskal-Wallis group comparisons.
- `open` chi-square and Fisher exact association tests.
- `open` family-wise error or false-discovery-rate controls when many tests
  are promoted.
- `open` scalar geometry hypotheses such as omega-style checks.
- `open` source-wise, feature-group-wise, and family-wise association scans.

## Sequence And Trajectory Methods

- `open` trajectory summary statistics.
- `open` ordered-sequence feature comparisons.
- `open` Markov-chain style transition models.
- `open` change-point or regime-switch checks.
- `open` dynamic time warping or shape-based trajectory comparison.
- `defer` ARIMA, state-space, hidden-Markov, Kalman, recurrent, or
  reinforcement-learning-style sequence models unless a concrete trajectory
  question makes them relevant.

## Landscape And Attractor Probes

- `open` endpoint duplicate and near-duplicate scan.
- `open` endpoint clustering under selected metrics.
- `open` perturbed-start ascent reruns.
- `open` endpoint perturbation followed by ascent.
- `open` same-attractor return probability.
- `open` attractor-volume or basin-size estimates.
- `open` local mutual-information checks between nearby attractors.
- `open` symmetry, centering, scaling, and normalization ablations.
- `defer` full symmetry-transverse sampling unless a thesis-facing local-volume
  claim needs it.
- `defer` HKO positive-region random walk unless promoted as HKO-local
  evidence.

## Methods Usually Rejected Unless A New Interface Appears

- `reject` time-series forecasting as forecasting.
- `reject` bandits unless there is an online candidate-selection loop.
- `reject` reinforcement learning unless there is a defined environment and
  reward.
- `reject` MCMC or variational inference unless there is a probabilistic model
  worth sampling.
- `reject` Laplace or local posterior approximations unless there is a
  probabilistic model worth approximating.
- `reject` hierarchical Bayes unless partial pooling answers a concrete
  source/family question.
- `reject` multi-fidelity optimization unless there is a cheaper faithful
  proxy for `sys`.

## Sources Used For Recall

This checklist is not source truth. Use it as a local memory aid for thesis
method-table coverage. The broader source truth is the literature, standard
practice, and current expert judgment about data science, statistics,
optimization, visualization, and numerical search.

Items above were extracted from:

- the deleted local taxonomy snapshots formerly under `methods/taxonomies/`;
- `research/sys-landscape-toolbox-audit.md`;
- `experiments/sys-landscape/datascience/methods/future-method-ideas.md`;
- `/tmp/AGENDA.md`;
- standard textbook/common-practice method families from EDA, statistical
  learning, probabilistic machine learning, statistical inference, numerical
  optimization, derivative-free optimization, continuation methods, Bayesian
  optimization, and time-series/trajectory analysis.
