# Sys-Landscape Toolbox Audit

## Purpose

- Make the hostile-landscape result legible below the headline level.
- Record, in one place, which "standard toolbox" methods were tried, which were not, and what each method actually licenses.
- This is no longer the ordinary entry point for the datascience slice. Use
  `experiments/sys-landscape/datascience/README.md` and
  `experiments/sys-landscape/datascience/methods/README.md` first.
- Method reports and source code overrule this audit for current row-level
  evidence status. Update this audit only when cross-thesis wording still uses
  it.

## Current Role

- This is a historical/cross-thesis audit. It defines structure, vocabulary,
  and open slots that were used before the datascience README system became
  the local control surface.
- Phase 2 should populate the ledger row by row from committed experiment packets and current notes.
- Jorn's 2026-04-25 scope choice for the next pass is: populate rows for
  committed artifact-backed methods and also add explicit skipped/deferred rows
  for key standard-toolbox families. Do not try to cover the broader data-science
  toolbox exhaustively.
- Phase 3 should tighten the shorter surfaces so they point here instead of paraphrasing the same story in multiple places.
- Frozen taxonomy anchors now live under
  `experiments/sys-landscape/datascience/methods/taxonomies/`.

## Claim Boundary

Current repo-level claim target:

- The closed method table records no new source of `sys > 1` examples and no
  candidate-proposer for finding one, beyond examples that are already explained
  by the HKO2024 construction and its symplectic images or controlled
  perturbations.
- Local optimization methods improve nearby or low-`sys` states in current
  artifacts, but no current artifact gives a candidate-proposer for finding a
  new source of `sys > 1` examples.
- Current seed counts are still too small for a strong density claim about practical brute-force impossibility.

This file should keep separate:

- `observation`: what the artifact or run literally shows.
- `inference`: what that observation supports.
- `licensed thesis wording`: the strongest honest sentence the thesis may say.

## Rating Vocabulary

Use exactly one primary rating per method row:

- `ran with no candidate-proposer and no new validated row`
- `ran as local optimization only`
- `not applicable to this data/search interface`
- `not run within the stated implementation bound`
- `not run within the stated compute/data bound`
- `implementation bug; no method verdict`

Allow one optional secondary tag only when it prevents ambiguity:

- `supporting evidence only`
- `validity caveat`
- `future reopen trigger`

## Ledger Columns

Each method row should answer these questions:

| Method | Question | Search surface | Data / artifacts | Validity guard | Observation | Inference | Rating | Thesis use | Reopen condition |
|--------|----------|----------------|------------------|----------------|-------------|-----------|--------|------------|------------------|

Interpret the columns strictly:

- `Question`: what problem the method was supposed to solve.
- `Search surface`: random regime, endpoint regime, HKO-local regime, structured family, or other explicit domain.
- `Data / artifacts`: the committed files or experiment directories that carry the evidence.
- `Validity guard`: leakage control, random-to-endpoint prediction test,
  provenance caveat, or reason no such guard exists.
- `Observation`: artifact-facing result, not interpretation.
- `Inference`: whether the observation gives a candidate-proposer, a new
  validated row, supporting evidence only, or no search output.
- `Thesis use`: whether the method supports a main claim, only a caveat, only future work, or should stay out of the thesis.
- `Reopen condition`: concrete trigger for revisiting the method later.

## Phase-2 Audit Rows

| Method | Question | Search surface | Data / artifacts | Validity guard | Observation | Inference | Rating | Thesis use | Reopen condition |
|--------|----------|----------------|------------------|----------------|-------------|-----------|--------|------------|------------------|
| `random-generic-sampling` random generic sampling | Does blind generic sampling find a new `sys > 1` example? | random generic polytopes | `experiments/sys-landscape/random-sample/`; `research/sys-landscape.md` | Exact artifacts are treated as producer-owned; no density guard. | Current note records 70 rows, max `sys=0.739`, no `sys>1`. | Supports only a bounded no-new-row baseline for the sampled generic rows. | `ran with no candidate-proposer and no new validated row` | Main hostile-landscape evidence with the explicit seed-count caveat. | Reopen if larger committed samples or changed generator semantics land. |
| `random-product-sampling` random Lagrangian-product sampling | Does blind structured-product sampling find a new `sys > 1` example? | random Lagrangian products | `experiments/sys-landscape/random-product-sample/`; `research/sys-landscape.md` | Structured family is not a uniform model of all convex bodies. | Current note records 100 rows, max `sys=0.794`, no `sys>1`. | Adds a structured-family no-new-row baseline without licensing a global density claim. | `ran with no candidate-proposer and no new validated row` | Main evidence for tested product sampling only. | Reopen if a refreshed product packet changes the max or samples a new product family. |
| `rejection-calibration` rejection / acceptance calibration | Does the random baseline have basic generation support evidence? | random-sampling support packet | `experiments/sys-landscape/rejection-calibration/` | Calibration packet is not itself a search surface. | Ledger marks this as attempted supporting evidence. | Helps interpret random sampling logistics but does not add an independent no-new-row result. | `ran with no candidate-proposer and no new validated row` / `supporting evidence only` | Supporting caveat if random sampling mechanics are discussed. | Reopen if baseline generation changes. |
| `rotated-regular-product-sweep` rotated regular-product sweep | Is the known pentagon-pentagon family isolated among tested regular products? | explicit regular polygon product family | `experiments/regular-products/rotated-regular-products/`; `research/sys-landscape.md` | Family is explicit and low-dimensional; no extrapolation beyond tested pairs/angles. | Current note records confirmed `sys>1` at pentagon-pentagon `theta=18 deg`; no further tested regular-family violation is known. | Supports the "known family is special among tested regular products" phrasing, not a classification theorem. | `ran with no candidate-proposer and no new validated row` | Main evidence for the structured regular-family contrast. | Reopen if additional regular-pair sweeps or exact formula/CAS write-up land. |
| `fixed-f-gradient-ascent` fixed-`F` gradient ascent | Does local ascent from random starts find a new high-`sys` endpoint outside the known HKO2024-derived source? | endpoint search over generic and product fixed-`F` polytopes | `experiments/sys-landscape/gradient-ascent-general/`; `experiments/sys-landscape/gradient-ascent-products/`; `experiments/sys-landscape/datascience/tables/README.md`; `research/sys-landscape.md` | Per-seed runs are local optimizer evidence; no guarantee of global optima. | Current LICCA dataset records `4096` general ascent endpoints with max `sys=0.9202772093964651` and `4089` product ascent endpoints with max `sys=0.9750768559799221`; no retained row has `sys>1`. | Local optimization improves endpoints but has not produced a new validated row in current retained runs. | `ran as local optimization only` | Main evidence for "local optimization did not find a new source" with seed-count caveat replaced by the LICCA retained-dataset caveat. | Reopen if a changed optimizer, new producer cache, or post-LICCA endpoint refresh produces new committed endpoints. |
| `variable-f-continuation` variable-`F` continuation | Does increasing `F` from local endpoints cross `sys=1`? | continuation from fixed-`F` endpoints into `F+1` states | `experiments/sys-landscape/variable-f-ascent/`; `research/sys-landscape.md` | Continuation starts from existing endpoint packets; it tests a local path, not arbitrary `F+1` states. | Current note records 90 trials including random-seed RQ2 and 10 RQ1 local-maxima starts; gains from `F=10` to `F=11` are common but still below `1`. | Continuation can improve local maxima, but current artifacts do not give a candidate-proposer for finding a new source of `sys > 1` examples. | `ran as local optimization only` | Main hostile-landscape evidence for continuation-as-local-search. | Reopen if witness-guided or other committed continuation replacement beats this benchmark. |
| `hko-local-perturbation-neighborhood` HKO-local perturbation neighborhood | Is the known HKO-side neighborhood locally stable under perturbation? | HKO-local packet | `experiments/hko-local-maximum/empirical/neighborhood-sampling/m10/` | HKO-local evidence is not generic hostile-landscape evidence. | Ledger marks the packet attempted and supporting-only. | Supports local/HKO-neighborhood interpretation, not the generic-search headline by itself. | `ran as local optimization only` / `supporting evidence only` | Supporting material only unless the thesis section discusses HKO-local stability. | Reopen if HKO-local rows become part of the main hostile-landscape claim. |
| `omega-scalar-hypothesis` omega scalar hypothesis test | Does a cheap scalar geometry hypothesis give a candidate-proposer? | scalar geometry check on landscape packets | `experiments/combinatorial-cells/omega-hypothesis/`; `tasks/planning-notes.md` | Correlation-style check only; no causal or exhaustive guard. | Ledger marks the omega hypothesis correlation check as attempted. | Records one scalar check that did not become a candidate-proposer, not a search theorem. | `ran with no candidate-proposer and no new validated row` / `supporting evidence only` | Supporting evidence if scalar hypotheses are mentioned. | Reopen if the omega feature is redefined or tied to a new committed candidate-proposer. |
| `visual-exploration` visual exploration | Did projection and picture inspection produce a candidate-proposer? | exploratory geometry and dynamics views | `research/visualization.md`; `tasks/planning-notes.md` | Human visual inspection is exploratory and non-exhaustive. | Ledger records visual exploration as an attempted exploratory check. | Picture inspection produced no recorded rule mapping visible features to candidate rows that were then validated for `sys > 1`. | `ran with no candidate-proposer and no new validated row` / `supporting evidence only` | Supporting or standalone exploration material, pending writing choice. | Reopen if figures become thesis-selected evidence or a visible feature becomes an implemented candidate-proposer. |
| `table-column-regression` table-column regression | Do retained table-column groups predict `sys` on endpoint rows after fitting on random rows? | tabular random and endpoint rows | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md`; `research/sys-landscape.md`; `tasks/planning-notes.md` | Grouped CV by lineage/source; random-to-endpoint prediction is the load-bearing guard. | Old current-looking artifacts were deleted during the table-API reset because they were stale or cheaper to rerun cleanly. | No current LICCA verdict yet for this row. | `not run within the stated compute/data bound` | Redo before thesis use as main data-science evidence, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| `regime-classification` endpoint-vs-random classification | Can standard classifiers separate endpoint rows from random rows, and is the separator geometric rather than provenance-heavy? | grouped tabular endpoint-vs-random task | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md` | Grouped split; metadata/provenance fields are an explicit validity caveat; classification alone is not a candidate-proposer. | Old current-looking classification artifacts were deleted during the table-API reset because they were stale or cheaper to rerun cleanly. | No current LICCA verdict yet for this supporting/caveat row. | `not run within the stated compute/data bound` | Redo before thesis use as an endpoint-vs-random caveat, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| `endpoint-residualized-regression` endpoint residualized regression | Do endpoint table-column groups add endpoint-only grouped-CV association beyond metadata? | endpoint-only grouped tabular packet | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md` | Grouped endpoint CV after metadata-first residualization. | The pre-LICCA disposition/report was deleted after its current fact was migrated: this row needs current retained-dataset evidence or terminal abandonment before thesis use. | No current LICCA verdict yet for this supporting/caveat row. | `not run within the stated compute/data bound` | Redo before thesis use as an endpoint-side association caveat, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| PCA / global dimensionality reduction | Would a global low-dimensional embedding become a candidate-proposer? | bounded retained-table-column spike | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md` | The packet should exclude metadata/provenance fields unless it explicitly studies source labels, and it needs a rule specified before inspecting `sys`, not only correlation with `sys`. | Old current-looking PCA artifacts were deleted during the table-API reset because they were stale and failed the current interpretation standard. | No current LICCA verdict yet for this toolbox row. | `not run within the stated compute/data bound` | Redo before thesis use as PCA coverage, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| Clustering / manifold learning | Would unsupervised grouping of landscape rows define a candidate-proposer? | bounded retained-table-column spike | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md` | A valid result needs a cluster-to-search rule rather than rediscovery of producer families. | Old current-looking clustering/anomaly artifacts were deleted during the table-API reset because they were stale or cheaper to rerun cleanly. | No current LICCA verdict yet for this toolbox row. | `not run within the stated compute/data bound` | Redo before thesis use as clustering/manifold coverage, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| SVM family | Would support-vector classifiers/regressors change the supervised table-column verdicts? | deferred supervised alternatives on retained dataset tables | No committed SVM method packet found in the active method README. | Would need the same grouped CV and random-to-endpoint prediction tests as `table-column-regression` and endpoint-vs-random classification. | SVMs are not attempted in the current before-submission evidence set. | Their absence is a coverage caveat, not evidence for or against the random-to-endpoint prediction observation. | `not run within the stated implementation bound` | Explicit omitted-family caveat only; do not cite as failed. | Reopen if a thesis reader expects SVM baselines or if current analyzers are extended cheaply. |
| Boosting / nearest-neighbor supervised alternatives | Would boosting or nearest-neighbor models change the regression/classification verdicts? | supervised alternatives on retained dataset tables | `experiments/sys-landscape/datascience/methods/README.md`; `experiments/sys-landscape/datascience/methods/STATUS.md`; `experiments/sys-landscape/datascience/tables/README.md` | Same grouped-CV and random-to-endpoint prediction framing as the supervised table-column checks. | Old current-looking supervised alternative artifacts were deleted during the table-API reset because they were stale or cheaper to rerun cleanly. | No current LICCA verdict yet for this supporting/caveat row. | `not run within the stated compute/data bound` | Redo before thesis use as supervised-alternatives coverage, or record a terminal abandoned/not-applicable state. | Reopen by running or reclassifying this row on the current retained dataset. |
| Neural-network methods | Would a flexible learned model give a candidate-proposer for high-`sys` rows? | deferred high-capacity supervised model family | No committed neural-network method packet found in the ledger. | Would need grouped splits, random-to-endpoint prediction tests, capacity/overfit controls, and enough data to make the fit meaningful. | Not attempted as a method program in the current ledger. | This remains unclassified for the current LICCA dataset until value, implementation cost, and review cost are reassessed. | `not run within the stated implementation bound` | Explicit omitted-family or abandoned-for-cost row only after current-dataset reassessment. | Reopen if the current method-table coverage requires a neural-network terminal state before thesis closeout. |
| Bayesian optimization | Would surrogate-guided black-box search find better candidates than random/local methods? | deferred black-box search over candidate-generation space | No committed Bayesian-optimization method packet found in the ledger. | A valid attempt would need a parameterized candidate space, acquisition loop, exact-evaluation budget, and held-out reporting. | Not attempted as a method program in the current ledger. | This remains a plausible future search program, not evidence for or against the current no-candidate-proposer result. | `not run within the stated compute/data bound` | Future-work caveat only. | Reopen if a bounded candidate space and compute budget are approved. |

## Taxonomy Anchors

- `experiments/sys-landscape/datascience/methods/taxonomies/`
- `experiments/sys-landscape/datascience/methods/README.md`

Phase 2 should treat the taxonomy files as frozen external method universes,
the method README as the active repo-method index, and this audit as the place
where thesis-facing verdicts are organized.

## Method Buckets To Populate In Phase 2

### Search Families

- random generic sampling
- random Lagrangian-product sampling
- rotated regular-product sweeps
- fixed-`F` gradient ascent from random starts
- variable-`F` continuation
- HKO-local perturbation neighborhood

### Data-Science / Pattern-Search Methods

- ridge regression on retained table-column groups
- random-forest regression on retained table-column groups
- logistic endpoint-vs-random classification
- random-forest endpoint-vs-random classification
- endpoint residual analysis beyond metadata
- scalar correlation / hypothesis tests already used as search heuristics
- visual inspection / "look at pictures"

### Methods To Classify Explicitly As Unused Or Deferred

- PCA as a source for a candidate-proposer
- clustering / manifold learning
- SVM / boosting / nearest-neighbor methods
- neural-network methods
- any other method Jörn wants counted as part of the "standard toolbox of a datascientist"

## Validity And Failure-Mode Notes To Record

Phase 2 should record these items explicitly instead of leaving them implicit:

- why grouped CV or lineage-grouped splits are the anti-leakage guard for the feature packet
- why transfer between random and endpoint regimes is the load-bearing test for global-search usefulness
- why metadata-heavy regime separation does not count as a geometry-based search heuristic
- whether any committed experiment should be marked `inconclusive: experiment/design failure`
- whether any method was skipped for cost reasons rather than scientific reasons

## Known Cleanup Facts Already Settled

- The task progress files now use the current random-generic max `sys=0.739`
  rather than the stale `0.578`; see `tasks/current-state.md`.
- The feature-pattern packet has refreshed plots, but the repo did not yet have a durable markdown method ledger for that packet.
- `research/sys-landscape.md` is now explicitly narrative-only for this surface; this file is the intended canonical tool-by-tool ledger.

## Phase-1.5 Discussion Questions

- Resolved for the next pass: include artifact-backed methods and clearly marked
  skipped/deferred rows for key standard-toolbox families, but do not attempt an
  exhaustive toolbox survey.
- Still open: which skipped families beyond PCA, clustering/manifold learning,
  SVM/boosting/nearest-neighbor methods, neural networks, and Bayesian
  optimization must be named for thesis-reader expectations?
- Should phase 3 compress `research/sys-landscape.md` further once this ledger exists, or keep one medium-detail hostile-landscape paragraph there?
