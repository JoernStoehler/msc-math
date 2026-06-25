# Datascience Method Coverage Checklist

Status: recall checklist, not a taxonomy and not a result ledger.

Use this file to avoid forgetting methods, tactics, patterns, and concepts
when closing the random/product sys-landscape datascience method table. It is
intentionally redundant, mixed-granularity, and not ordered except for loose
grouping.

This file is not a result ledger. When an item becomes active, create or update
a method packet `README.md` with the method-specific question, inputs, leakage
guards, observations, caveats, current disposition, thesis use, and reopen
trigger.

Disposition words below are planning hints only:

- `open`: not currently closed on the retained tables.
- `used`: appears in the active random/product producer or method surfaces, but
  may still need a method packet README.
- `defer`: plausible but lower current thesis value.
- `reject`: not applicable unless a missing interface or data source appears.

## Baseline And EDA

- `used` target predicate scan: rows with `sys > 1`.
- `open` target distribution, quantiles, and tail summaries.
- `open` grouped summaries by source, facet count, product bucket, seed, and
  height range where available.
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
- `open` seed, source, facet-count, and product-bucket leakage checks.
- `open` provenance-only and metadata-only baselines.
- `reject` random-to-endpoint transfer checks for this slice.
- `reject` endpoint-only residual checks for this slice.
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
- `reject` fixed-F gradient ascent from random starts for this slice.
- `reject` fixed-F product gradient ascent from random starts for this slice.
- `reject` variable-F continuation for this slice.
- `defer` multistart and restart variants unless recast as pure random
  distribution changes.
- `reject` random perturbation followed by ascent for this slice.
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

- `reject` gradient ascent for this slice.
- `reject` line-search variants for this slice.
- `reject` trust-region variants for this slice.
- `reject` Newton, quasi-Newton, and nonlinear conjugate-gradient variants for
  this slice.
- `reject` projected-gradient or active-set variants for this slice.
- `reject` penalty, barrier, interior-point, and sequential-quadratic-programming
  variants.
- `reject` subgradient, bundle, and proximal-style variants for this slice.
- `reject` step-size and distance ablations for this slice.
- `reject` minimum-sigma versus multi-sigma gradient choices for this slice.
- `reject` zero-padded sigma handling for this slice.
- `reject` `D sys`, `D beta`, and related first-order directions for this
  slice.
- `reject` simple parameter continuation for this slice.
- `reject` predictor-corrector continuation for this slice.
- `reject` arc-length continuation for this slice.
- `reject` branch following and branch switching for this slice.
- `reject` witness-guided or parent-cache continuation for this slice.
- `reject` homotopy or solution continuation under changing problem data for
  this slice.

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
- `reject` endpoint-vs-random classification for this slice.
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

- `reject` trajectory summary statistics for this slice.
- `reject` ordered-sequence feature comparisons for this slice.
- `reject` Markov-chain style transition models for this slice.
- `reject` change-point or regime-switch checks for this slice.
- `reject` dynamic time warping or shape-based trajectory comparison for this
  slice.
- `defer` ARIMA, state-space, hidden-Markov, Kalman, recurrent, or
  reinforcement-learning-style sequence models unless a concrete trajectory
  question makes them relevant.

## Landscape And Attractor Probes

- `reject` endpoint duplicate and near-duplicate scan for this slice.
- `reject` endpoint clustering under selected metrics for this slice.
- `reject` perturbed-start ascent reruns for this slice.
- `reject` endpoint perturbation followed by ascent for this slice.
- `reject` same-attractor return probability for this slice.
- `reject` attractor-volume or basin-size estimates for this slice.
- `reject` local mutual-information checks between nearby attractors for this
  slice.
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
- the deleted historical `research/sys-landscape-toolbox-audit.md`;
- `experiments/sys-datascience/methods/future-method-ideas.md`;
- `/tmp/AGENDA.md`;
- standard textbook/common-practice method families from EDA, statistical
  learning, probabilistic machine learning, statistical inference, numerical
  optimization, derivative-free optimization, continuation methods, Bayesian
  optimization, and time-series/trajectory analysis.
