# Sys-Datascience First-Wave P1/P3 Results, 2026-07-08

Use: synthesis of the first two design packets launched from
`first-wave-design-2026-07-08.md`. This is coordination evidence, not method
evidence and not thesis prose. P2 later executed and superseded the "P2 next"
state here; see `p2-synthesis-2026-07-08.md` for the current continuation.

Execution mode: read-only design packets. No experiments, models, or producers
were run.

Subagents:

- P1 standard-repertoire disposition audit:
  `019f4269-eeb4-7901-953f-a3bb8d51babd`;
- P3 broader producer/distribution design scout:
  `019f426a-178a-7010-a956-ca699539cf81`.

Parent source-checks:

- inspected `produce/README.md`, `prepare/README.md`,
  `produce/plans/two-face-control-licca-smoke.json`,
  `produce/plans/two-face-control-replication.json`, and relevant producer
  code surfaces;
- confirmed the high-complexity producer plan is an existing named plan over
  generic `F=10,11,12` and product `4x6,5x5,5x6,6x6`;
- did not run the plan.

## Parent Synthesis

P1 and P3 successfully reduced the first-wave heterogeneity:

- P1 says the retained-table standard-method surface still has one compact
  missing executor: P2 tiny retained-table baseline. It should run if broad
  retained-table standard-method wording remains live.
- P3 says no broad or blind producer run should launch now. The only concrete
  producer launch candidate is the existing high-complexity bucket extension
  plan, with product side range as the strongest axis and facet range as the
  paired secondary axis.

Default next action at the time:

1. Launch P2 tiny retained-table missing baseline before thesis closeout prose.
2. Keep the high-complexity producer extension as a compute packet candidate,
   not as an automatic run.
3. After P2, synthesize again before choosing between P4 proposer closure,
   P5 mechanism/tail wording audit, or a high-complexity producer packet.

No Jörn input was needed for the next local step. P2 has since run. A Jörn crux
may become useful only if the parent is about to spend substantial compute on
the high-complexity producer packet and local evidence still leaves
depth-versus-diversity ambiguous.

## P1 Standard-Repertoire Disposition

P1 conclusion: P2 should run if broad retained-table wording remains live.
FACTSHEET item 31 says not to weaken standard-repertoire coverage to merely
representative families where feasible, and skipped applicable methods need
concrete reasons.

| Method family | Disposition | Concrete reason |
| --- | --- | --- |
| target scan, trusted rows, schema, duplicates, row counts | already covered | Current invariant retained table has row/provenance audit and zero trusted `sys > 1` rows. |
| EDA, tail summaries, source/facet/product summaries, near-miss inspection | already covered | Enough for retained-table descriptive wording; tail-language still needs claim-boundary review. |
| plot variants, residual plots, inspection views | park-low-value | Visualization should follow a named claim; it should not drive method closure. |
| grouped validation, leakage checks, metadata/provenance baselines, permutation/bootstrap | already covered | Current packets include grouped holdout, metadata-only baselines, source/provenance checks, and permutation/bootstrap checks. |
| confidence intervals, FDR, Kendall/partial-correlation variants | park-low-value | Useful only if a scalar effect becomes thesis-facing mechanism evidence. Current scalar screens are not proposer validation. |
| blind generic/product random sampling under current retained producer | already covered | Current retained producer contract has generic and product rows with no positives. |
| independent same-distribution reruns, height/facet/product-side variants, Latin-hypercube/space-filling designs, alternative random models | future-work | These affect broader producer/distribution claims, not retained-table standard-method coverage; use P3-style design before runs. |
| rejection sampling with cheap predicates / scalar generated-candidate filters | already covered for boundary evidence; future-work for positive proposer | Existing scalar generated-candidate packet is boundary evidence only. Positive proposer wording still needs frozen independent validation. |
| ridge regression, random forest regression, shallow tail decision trees | already covered | Current in-table ranking/rule packets exist with grouped/source controls. They do not validate generated-candidate proposers. |
| lasso/elastic-net, gradient boosting, high-tail classification, feature-family ablation | needs executor | Cheap, standard, applicable, and currently not run. This is P2. |
| OLS / linear regression without shrinkage | already covered enough | Ridge covers the linear-regression interface more stably on correlated scalar features. Lasso/elastic-net remains useful because feature selection differs. |
| logistic/LDA/QDA/naive Bayes/prototype/kNN/SVM/kernel classification | park-low-value | No positive class exists; near-tail labels are arbitrary. P2 high-tail classification covers the classification interface first. |
| polynomial/basis expansion, splines/GAM, local regression/kernel smoothing, PCR/PLS, subset/stepwise, bagging/model averaging | park-low-value | Applicable but lower value than P2; likely duplicate in-table structure unless P2 reveals a new interaction. |
| Gaussian-process/Bayesian regression and posterior predictive ranking | future-work | Potentially applicable as ranking/uncertainty, but needs generated-candidate interface and calibration story. |
| neural predictors, autoencoders, learned latent representations | park-low-value | Small retained scalar-feature table, weak interpretability, and no proposer-validation interface. |
| PCA, low-dimensional projections, k-means, isolation-style anomaly | already covered | Active projection/anomaly packet exists. |
| factor analysis, hierarchical clustering, mixtures, density estimation, density anomaly, one-class classification, normalizing flows | park-low-value | Reopen only if projection/anomaly or P2 shows robust unexplained high-tail structure. |
| t/Welch, Mann-Whitney, ANOVA/Kruskal-Wallis, chi-square/Fisher, family/source scans | already covered enough for current factors; future-work for promoted claims | Current source/facet/product factor tests exist. More tests are robustness work only if a factor claim is promoted. |
| derivative-free local/global search, Nelder-Mead, pattern search, trust-region surrogate, Bayesian optimization, constrained black-box optimization | future-work / reject-interface for retained table | These are not retrospective retained-table methods. They require candidate-generation/evaluation interface and generated-candidate wording. |
| gradient ascent, continuation, endpoint/ascent/local-behavior panels | reject-interface | Removed from active retained random/product datascience slice; route elsewhere if reopened. |
| time-series, sequence, trajectory, forecasting, bandits, reinforcement learning, online optimization | reject-interface | No time-indexed target, trajectory object, online environment, or reward loop exists. |
| MCMC, variational inference, Laplace/posterior approximations, hierarchical Bayes | reject-interface for current wording; future-work only with model | No probabilistic model is currently part of the retained-table thesis claim. |

P1 coordination updates accepted:

- P1 stop condition should cover every checklist family or checklist group,
  with relevance interpreted under FACTSHEET 31.1.
- `topics/supervised-and-representation-methods.md` should say P2 is the
  minimum retained-table executor for broad ordinary-method wording, not
  optional cleanup.
- Coordination should warn that representative-family coverage is insufficient
  for broad standard-method wording.

## P3 Producer/Distribution Design

P3 conclusion: do not launch broad producer work blindly. If producer work is
promoted, the only concrete launch candidate currently identified is the
high-complexity bucket extension plan.

| Axis | Disposition | Rough cost | Thesis sentence affected | Stop rule |
| --- | --- | --- | --- | --- |
| independent seed, same retained contract | park | low-medium at retained size, low information | only strengthens finite-sample precision for retained contract | stop after one fixed same-contract rerun only if needed for reproducibility/regression |
| height interval | park | low for targeted plan, medium for serious per-bucket sampling | robustness under named height-spread variants | reopen only after stronger bucket/family coverage; use fixed intervals and effect-size summaries |
| generic facet range | launch candidate | medium-high | no new source under tested high-complexity generic buckets | fixed buckets only; scan `sys > 1`, summarize high tail and ridge stability |
| product side range | strongest launch candidate | medium-high | no new source under tested high-complexity random Lagrangian-product buckets | fixed high-product buckets only; positive row escalates, no positive yields bounded bucket-extension evidence |
| alternative random model | park / reject for first wave | high | claims about random models beyond current normals/heights and random polygons | do not launch until model is named with mathematical reason and source contract |
| space-filling parameter design | park | medium-high plus analysis design | broader named parameter-design probe | reopen after parameters and objective are fixed; no adaptive browsing |

Existing candidate plan:

```text
experiments/sys-datascience/produce/plans/two-face-control-replication.json
```

Plan contents:

- generic random rows: `F=10` with `8192` rows, `F=11` with `4096` rows,
  `F=12` with `4096` rows;
- random product rows: `4x6`, `5x5`, `5x6`, `6x6`, each with `4096` rows;
- default height interval `[0.8,1.2]` by producer plan defaults.

P3 wording boundary:

- retained-contract wording: "For the retained random/product producer
  contract, the closed method table records no new source of `sys > 1` examples
  and no validated candidate-proposer."
- broader-distribution wording: "Additional producer variants were treated as
  separate probes of named generator axes, not evidence about arbitrary random
  polytope distributions."

Do not write "random polytopes do not produce counterexamples" or "standard
random models found none" unless every model is named.

## Updated Packet Ranking

1. P2 tiny retained-table missing baseline: completed after this synthesis; see
   `p2-synthesis-2026-07-08.md`.
2. High-complexity bucket-extension compute packet: candidate after P2 or after
   a separate compute-budget decision. It should not launch without a packet
   card naming cost, output artifacts, review standard, and LICCA/local route.
3. P4 generated-candidate proposer closure/rescue design: run if proposer
   wording remains important after P2, or if P2 finds an interaction that
   suggests a new candidate rule.
4. P5 mechanism/tail thesis-use audit: run when the parent needs wording
   boundaries for the actual thesis chapter, preferably after P2 clarifies the
   method story.

## Default Continuation

The next autonomous parent should use `p2-synthesis-2026-07-08.md`, not this
older continuation, to choose the next packet.

Before any high-complexity producer run, use the LICCA skill and write a
separate compute packet. The local crux is not permission; it is whether the
thesis value of more depth in high-tail retained-family buckets beats broader
distribution diversity after P2.
