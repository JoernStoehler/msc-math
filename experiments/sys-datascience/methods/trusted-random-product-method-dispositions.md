# Trusted Random/Product Method Dispositions

Purpose: disposition ledger for active checklist families scoped to trusted
random/product rows. This is not a replacement for method packet READMEs or
artifacts.

Coverage rule: every `open` family in `method-coverage-checklist.md` is named
below, either as its own row or in a joint row whose methods share the same
interface and concrete disposition reason. A joint row is not representative-
family sampling: every named family inherits the recorded status and reopen
trigger.

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
- `current-reviewed-packet`: a nearby generated-candidate or producer packet
  ran under its own reviewed contract rather than the retained invariant table.
- `covered-by-interface`: a named nearby method was not run because a current
  packet already answers the same retained-table question with no weaker
  claim-relevant interface.
- `park-low-value`: feasible, but skipped after an explicit low-promise or
  redundancy judgment under the current retained-table claim.
- `future-work`: applicable only after a named new candidate-generation,
  producer, probability-model, or thesis-claim interface is fixed.
- `reject`: not useful for this random/product table unless assumptions change.
- `out-of-scope`: not part of the current random-polytope slice.

## Baseline And EDA

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| target predicate scan | `current-invariant-run` | `scan-sys-gt-1/` | Current invariant full run found zero `sys > 1`. |
| target distribution, quantiles, tail summaries | `current-invariant-run` | `random-tail-eda/` | Current-schema EDA exists; review tail-language limits before thesis use. |
| source/facet/product/seed/height summaries | `current-invariant-run` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/facet/product diagnostics exist. The retained contract has one seed and one height interval, so seed/height contrasts require a future producer-axis research decision. |
| missingness, duplicates, schema, row-count checks | `current-invariant-run` | `trusted-random-dataset/` | Current invariant full run has `14336` trusted rows, no duplicates, and zero excluded labels. |
| outlier and near-miss inspection | `current-invariant-run` | `random-tail-eda/`, `projection-structure/` | Current top row has `sys = 0.86258589584944`; near-miss language still needs thesis-level judgment. |
| scatter/pairwise, histogram/density/box/tail, heatmap/matrix views | `park-low-value` | existing tabular artifacts | Feasible, but plotting the same retained summaries has low value until an exact thesis claim needs a figure; generated tables remain the metric source. |
| parallel coordinates and other multivariate views | `park-low-value` | `projection-structure/` | PCA/projection diagnostics already cover the current multivariate inspection question more legibly. |
| residual and model-diagnostic plots | `park-low-value` | `prediction-ranking/`, `standard-baseline-p2/` | Reopen only if a prediction result becomes thesis-facing; current in-table models do not support a proposer claim. |
| geometric/orbit inspection pictures | `park-low-value` | none | Requires a named geometric hypothesis and asset purpose; visual browsing alone does not close method coverage. |
| faceting, conditioning, overlays, annotations | `covered-by-interface` | `random-tail-eda/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Current packets already condition on source/facet/product groups in tables and tests; plot variants add no new retained claim. |

## Leakage, Validation, And Null Checks

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| train/test split, grouped cross-validation interface, and holdout validation | `current-invariant-run` | `prediction-ranking/`, `standard-baseline-p2/` | Current grouped holdout artifacts exist. Repeated grouped folds are redundant unless a model result is promoted beyond in-table diagnosis. |
| seed/source/facet/product leakage checks | `current-invariant-run` | `trusted-random-dataset/`, `statistical-associations/`, `prediction-ranking/`, `projection-structure/` | Source/factor tests, metadata-only baselines, and overlays exist. Seed is constant in the retained contract, so seed leakage needs a future independent-seed dataset. |
| provenance-only and metadata-only baselines | `current-invariant-run` | `prediction-ranking/` | Current metadata-only baselines exist. |
| permutation and bootstrap checks | `current-invariant-run` | `statistical-associations/`, `prediction-ranking/` | Current association permutation/bootstrap and ranking permutation artifacts exist. |
| p-values and test-statistic summaries | `current-invariant-run` | `statistical-associations/` | Current factor tests and the family-maximum permutation test record statistics and p-values. |
| confidence intervals or standard errors | `covered-by-interface` | `statistical-associations/` | A bootstrap interval exists for the current promoted source-family contrast; add more only for a promoted scalar/factor claim. |
| multiple-comparison or FDR treatment | `covered-by-interface` | `statistical-associations/` | The current scalar screen uses a family-maximum permutation check. Exact FDR variants are parked unless individual scalar discoveries are promoted. |

## Direct Random Sampling

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| blind generic random sampling | `current-invariant-run` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `4096` generic rows and no positives. |
| blind random Lagrangian-product sampling | `current-invariant-run` | `trusted-random-dataset/`, `random-tail-eda/`, `scan-sys-gt-1/` | Current full run included `10240` product rows and no positives. |
| producer rejection/acceptance calibration | `current-reviewed-packet` | `../produce/README.md`, producer metadata | Current producers record accepted samples and rejection attempts under their fixed contracts; broader calibration belongs to a new producer-axis question. |
| independent same-distribution reruns | `park-low-value` | none | Useful for stability only after a current fact or claim is named as needing it; another seed does not broaden the producer model. |
| broader height intervals, facet ranges, product side ranges, or new distributions | `future-work` | none | These could support stronger producer-axis wording only after a new research decision names the distribution and exact claim. |
| Latin-hypercube and other space-filling designs | `future-work` | `../produce/` | No continuous parameter domain, sampling measure, or claim is fixed. Reopen only through a new producer-axis research decision. |
| rejection sampling with cheap scalar predicates | `current-reviewed-packet` | `extreme-scalar-rejection-proposer/` | Frozen scalar rules were evaluated before `sys` on generated candidates; this is negative boundary evidence, not a positive proposer. |
| local pattern/generalized-pattern/MADS and Nelder--Mead search | `future-work` | none | These require a candidate parameterization and generated-candidate evaluation loop; they are not retrospective retained-table methods. |
| derivative-free trust-region, local interpolation/quadratic search | `future-work` | none | No continuous local candidate interface or validated cheap objective is fixed. |
| stochastic global, hybrid local/global, constrained black-box search | `future-work` | none | Requires a named candidate domain, constraints, budget, and selection-before-`sys` review gate. |
| Bayesian/surrogate-guided, batch, constrained, or high-dimensional optimization | `future-work` | none | No calibrated surrogate or generated-candidate interface exists; multi-fidelity variants also lack a faithful cheap proxy. |
| deterministic branch-and-bound black-box search | `park-low-value` | none | No bounding function or finite candidate partition is available, so the standard interface is absent. |

## Supervised Prediction And Rules

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| ordinary least squares / unshrunk linear regression | `covered-by-interface` | `prediction-ranking/`, `standard-baseline-p2/` | Ridge, lasso, and elastic net cover the linear retained-table interface more stably under correlated features; unshrunk OLS is unlikely to change the claim. |
| ridge regression | `current-invariant-run` | `prediction-ranking/` | Current invariant-feature ridge model ran under the active schema. |
| lasso and elastic-net regression | `current-invariant-run` | `standard-baseline-p2/` | P2 ran sparse/shrinkage linear baselines under grouped holdout. |
| random forest regression | `current-invariant-run` | `prediction-ranking/` | Current invariant-feature RF model ran under the active schema; it is in-table signal, not yet a generated-candidate proposer. |
| gradient boosting regression | `current-invariant-run` | `standard-baseline-p2/` | P2 ran histogram gradient boosting regression under grouped holdout. |
| polynomial/basis-expansion regression | `park-low-value` | none | Feasible but likely to duplicate nonlinear in-table structure already tested by trees/boosting; reopen for one frozen interaction hypothesis. |
| splines and generalized additive models | `park-low-value` | none | Feasible, but lower value than current boosted/tree models without a thesis-facing smooth scalar-response claim. |
| local regression and kernel smoothing | `park-low-value` | none | High-dimensional mixed-bucket smoothing has no current claim advantage; reopen for a named one- or two-feature diagnostic. |
| principal-components regression and partial least squares | `park-low-value` | `projection-structure/` | PCA structure and supervised linear/tree baselines already exist; PCR/PLS is unlikely to change the retained-table boundary. |
| subset/stepwise selection | `covered-by-interface` | `standard-baseline-p2/` | Lasso/elastic-net coefficient paths provide the current sparse-selection interface with better stability. |
| bagging and model averaging | `park-low-value` | `prediction-ranking/`, `standard-baseline-p2/` | Random forest already supplies a bagged-tree interface and boosting adds a distinct ensemble; another ensemble does not address proposer validity. |
| shallow high-tail decision-tree rules | `current-invariant-run` | `tail-rule-mining/` | Current invariant-feature tree run completed under the active schema; older geometry-family artifacts are stale. |
| logistic and high-tail classification | `current-invariant-run` | `standard-baseline-p2/` | P2 ran elastic-net logistic and histogram gradient boosting high-tail classifiers. Labels are retained-table high-tail labels, not positives or generated-candidate validation. |
| LDA and QDA | `park-low-value` | none | The high-tail label is an arbitrary retained-table quantile and P2 already covers linear and nonlinear classification interfaces. |
| naive Bayes | `park-low-value` | none | Conditional-independence modeling is low promise for correlated invariant features and cannot create a positive-class proposer claim. |
| k-nearest neighbors and prototype methods | `park-low-value` | none | Mixed bucket geometry and arbitrary feature scaling make this lower value than grouped tree/linear baselines without a named neighborhood hypothesis. |
| support-vector machines and kernel classifiers | `park-low-value` | none | Feasible but computationally and interpretively redundant with P2 for the arbitrary high-tail label; reopen only for a frozen nonlinear boundary claim. |
| probabilistic linear/logistic predictors | `covered-by-interface` | `standard-baseline-p2/` | Logistic scores cover the present classification-ranking interface; calibrated uncertainty is not justified by the arbitrary tail label. |
| Gaussian-process/Bayesian regression and posterior-predictive ranking | `future-work` | none | Requires a generated-candidate ranking/calibration design; retained-table posterior scores alone would not validate a proposer. |
| multilayer perceptrons, autoencoders, learned representations | `park-low-value` | none | The retained scalar table is small, correlated, and already modeled well enough for its bounded role; learned representations weaken interpretability without fixing proposer validation. |
| feature-family ablation | `current-invariant-run` | `standard-baseline-p2/` | P2 compared combinatorial-count and ridge symplectic-area feature families under grouped holdout. |
| metadata-only prediction baselines | `current-invariant-run` | `prediction-ranking/` | Current metadata-only baselines ran. |
| generated-candidate proposer loop | `current-reviewed-packet` | `extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/` | One frozen two-stage rule passed its independent sub-threshold enrichment criterion. It did not find `sys > 1`; Jörn must settle whether thesis vocabulary calls this a candidate-proposer or reserves that term for threshold-directed evidence. |

## Unsupervised, Density, And Anomaly Methods

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| PCA and low-dimensional projections | `current-invariant-run` | `projection-structure/` | Current PCA artifact exists. |
| k-means clustering | `current-invariant-run` | `projection-structure/` | Current k-means artifact exists. |
| isolation-style anomaly detection | `current-invariant-run` | `projection-structure/` | Current isolation-style anomaly artifact exists. |
| factor analysis / latent-factor models | `park-low-value` | `projection-structure/` | PCA already tests low-dimensional linear structure; no latent probabilistic factor claim is active. |
| hierarchical clustering | `park-low-value` | `projection-structure/` | K-means/projection diagnostics found no claim that requires a dendrogram or nested cluster interface. |
| Gaussian mixtures and soft clustering | `park-low-value` | `projection-structure/` | No stable cluster-based high-tail claim is active; mixture components would be dominated by source/bucket structure. |
| density estimation and density-based anomaly detection | `park-low-value` | `projection-structure/` | Current isolation-style anomaly diagnostics provide the applicable outlier interface; density claims would need stronger bucket-matched support. |
| one-class classification | `park-low-value` | `projection-structure/` | There is no trusted positive class or reference support set to define the one-class target. |
| normalizing flows / expressive density models | `park-low-value` | none | Data size, mixed buckets, and absent density claim do not justify a less interpretable density model. |

## Statistical Associations

| Checklist family | Current disposition | Packet/source | Reason and reopen trigger |
| --- | --- | --- | --- |
| Pearson/Spearman rank screening | `current-invariant-run` | `statistical-associations/` | Current scalar screen ran on `29` nonconstant active scalar covariates. |
| source/facet/product factor tests | `current-invariant-run` | `statistical-associations/` | Current factor tests ran with current provenance fields. |
| ridge symplectic-area scalar hypotheses | `current-invariant-run` | `../prepare/`, `statistical-associations/` | Active association screen includes invariant ridge symplectic-area features. |
| Kendall rank correlation | `park-low-value` | `statistical-associations/` | Spearman plus a family-maximum permutation check covers current monotone screening; add Kendall only for a promoted scalar effect. |
| partial correlation | `park-low-value` | `statistical-associations/`, `ridge-mechanism-discriminator/` | Current bucket/control diagnostics already show confounding; a promoted scalar mechanism claim would need a separately specified adjustment set. |
| t/Welch and Mann--Whitney comparisons | `covered-by-interface` | `statistical-associations/` | Current two-group questions are covered by ANOVA/Kruskal factor tests and a bootstrap mean-difference interval; exact pairwise variants would not change the bounded claim. |
| ANOVA and Kruskal--Wallis comparisons | `current-invariant-run` | `statistical-associations/` | Current categorical source/facet/product factor tests record both interfaces. |
| chi-square and Fisher exact association tests | `park-low-value` | none | `sys` is continuous and there is no positive class; dichotomizing at an arbitrary high-tail cutoff adds little beyond current classifiers/factor tests. |
| family-wise error / false-discovery-rate controls | `covered-by-interface` | `statistical-associations/` | A family-maximum permutation check controls the current scalar screen; exact FDR is needed only if individual discoveries are promoted. |

## Rejected Or Out Of Scope

| Checklist family | Current disposition | Reason and reopen trigger |
| --- | --- | --- |
| ascent, continuation, endpoint stability, attractors, basins, local-behavior panels | `out-of-scope` | Removed from active datascience slice; use a separate thesis slice only if Jörn explicitly reopens it. |
| forecasting/time-series | `reject` | No time-indexed forecasting target. |
| bandits or reinforcement learning | `reject` | No online environment or reward loop is defined. |
| MCMC/Bayesian posterior families | `reject` | No probabilistic model is currently part of the thesis claim. |
| multi-fidelity optimization | `reject` | No cheaper faithful proxy for `sys` is established. |
| distribution-variant sensitivity rows | `future-work` | `distribution-sensitivity/` and `random-axis-diagnostic/` need multiple prepared random/product variants; the active retained table has only the current producer contract. |
| symmetry, centering, scaling, normalization ablations | `current-invariant-run` | `../prepare/README.md`, invariant-feature checks | The active feature contract is tested under scale, translation, facet permutation, sampled symplectic maps, and their composition. |
| full symmetry-transverse or HKO positive-region sampling | `future-work` | none | Requires a thesis-facing local-volume or HKO-local claim and belongs to that owner, not retained-table closure. |

## Exploration Boundaries

1. Do not claim closure over arbitrary random distributions unless a new
   distribution-design batch is run and reviewed.
2. The reviewed ridge-concentration packet supports one exact independently
   generated sub-threshold enrichment rule. Do not promote it to a proposer for
   finding `sys > 1`, a mechanism, or transfer beyond its generator.
3. P2 closes the previously missing executor for the named retained-table
   standard repertoire. Reopen method execution only when a stronger exact
   claim makes a deferred family relevant; this does not close broader
   distributions or every possible method.
4. Use `../coordination/exploration-result.md` for the cross-method
   interpretation and the boundary between completed exploration and
   demonstration.
