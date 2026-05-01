# Sys-Landscape Toolbox Audit

## Purpose

- Make the hostile-landscape result legible below the headline level.
- Record, in one place, which "standard toolbox" methods were tried, which were not, and what each method actually licenses.
- Keep this file as the canonical method ledger for the empirical hostile-landscape claim; use `research/sys-landscape.md` for topic narrative and `tasks/landscape.md` for thesis-facing obligations.

## Current Role

- This is a phase-1 scaffold. It defines the structure, vocabulary, and open slots for the detailed audit.
- Phase 2 should populate the ledger row by row from committed experiment packets and current notes.
- Jorn's 2026-04-25 scope choice for the next pass is: populate rows for
  committed artifact-backed methods and also add explicit skipped/deferred rows
  for key standard-toolbox families. Do not try to cover the broader data-science
  toolbox exhaustively.
- Phase 3 should tighten the shorter surfaces so they point here instead of paraphrasing the same story in multiple places.
- Frozen taxonomy anchors now live under `research/sys-landscape-datascience/`.

## Claim Boundary

Current repo-level claim target:

- Bounded empirical search found no new `sys > 1` example beyond the known pentagon-pentagon family.
- Local optimization methods improve nearby or low-`sys` states, but current evidence has not produced a transferable global-search heuristic.
- Current seed counts are still too small for a strong density claim about practical brute-force impossibility.

This file should keep separate:

- `observation`: what the artifact or run literally shows.
- `inference`: what that observation supports.
- `licensed thesis wording`: the strongest honest sentence the thesis may say.

## Rating Vocabulary

Use exactly one primary rating per method row:

- `attempted, negative vs random`
- `attempted, local optimization only`
- `inapplicable`
- `skipped: expensive to implement`
- `skipped: expensive to run`
- `inconclusive: experiment/design failure`

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
- `Validity guard`: leakage control, transfer test, provenance caveat, or reason no such guard exists.
- `Observation`: artifact-facing result, not interpretation.
- `Inference`: what the observation says about search usefulness.
- `Thesis use`: whether the method supports a main claim, only a caveat, only future work, or should stay out of the thesis.
- `Reopen condition`: concrete trigger for revisiting the method later.

## Phase-2 Audit Rows

| Method | Question | Search surface | Data / artifacts | Validity guard | Observation | Inference | Rating | Thesis use | Reopen condition |
|--------|----------|----------------|------------------|----------------|-------------|-----------|--------|------------|------------------|
| `M001` random generic sampling | Does blind generic sampling find a new `sys > 1` example? | random generic polytopes | `experiments/sys-landscape/random-sample/`; `research/sys-landscape.md` | Exact artifacts are treated as producer-owned; no density guard. | Current note records 70 rows, max `sys=0.739`, no `sys>1`. | Supports only a bounded negative baseline for the sampled generic regime. | `attempted, negative vs random` | Main hostile-landscape evidence with the explicit seed-count caveat. | Reopen if larger committed samples or changed generator semantics land. |
| `M002` random Lagrangian-product sampling | Does blind structured-product sampling find a new `sys > 1` example? | random Lagrangian products | `experiments/sys-landscape/random-product-sample/`; `research/sys-landscape.md` | Structured family is not a uniform model of all convex bodies. | Current note records 100 rows, max `sys=0.794`, no `sys>1`. | Adds a structured-family negative baseline without licensing a global density claim. | `attempted, negative vs random` | Main evidence for tested product sampling only. | Reopen if a refreshed product packet changes the max or samples a new product family. |
| `M003` rejection / acceptance calibration | Does the random baseline have basic generation support evidence? | random-sampling support packet | `experiments/sys-landscape/rejection-calibration/` | Calibration packet is not itself a search surface. | Ledger marks this as attempted supporting evidence. | Helps interpret random sampling logistics but does not add an independent negative search result. | `attempted, negative vs random` / `supporting evidence only` | Supporting caveat if random sampling mechanics are discussed. | Reopen if baseline generation changes. |
| `M004` rotated regular-product sweep | Is the known pentagon-pentagon family isolated among tested regular products? | explicit regular polygon product family | `experiments/sys-landscape/rotated-regular-products/`; `research/sys-landscape.md` | Family is explicit and low-dimensional; no extrapolation beyond tested pairs/angles. | Current note records confirmed `sys>1` at pentagon-pentagon `theta=18 deg`; no further tested regular-family violation is known. | Supports the "known family is special among tested regular products" phrasing, not a classification theorem. | `attempted, negative vs random` | Main evidence for the structured regular-family contrast. | Reopen if additional regular-pair sweeps or exact formula/CAS write-up land. |
| `M005`/`M006` fixed-`F` gradient ascent | Does local ascent from random starts find a transferable high-`sys` endpoint? | endpoint search over generic and product fixed-`F` polytopes | `experiments/sys-landscape/gradient-ascent-general/`; `experiments/sys-landscape/gradient-ascent-products/`; `research/sys-landscape.md` | Per-seed runs are local optimizer evidence; no guarantee of global optima. | Current note records general ascent: 10 seeds, max `sys=0.9030`, no `sys>1`; product ascent: 12 seeds, max `sys=0.8727`, no `sys>1`. | Local optimization improves endpoints but has not produced a new violation in current runs. | `attempted, local optimization only` | Main evidence for "local optimization did not find a second regime" with seed-count caveat. | Reopen if LICCA endpoint refresh or a changed optimizer produces new committed endpoints. |
| `M007` variable-`F` continuation | Does increasing `F` from local endpoints cross `sys=1`? | continuation from fixed-`F` endpoints into `F+1` states | `experiments/sys-landscape/variable-f-ascent/`; `research/sys-landscape.md` | Continuation starts from existing endpoint packets; it tests a local path, not arbitrary `F+1` states. | Current note records 90 trials including random-seed RQ2 and 10 RQ1 local-maxima starts; gains from `F=10` to `F=11` are common but still below `1`. | Continuation can improve local maxima, but current artifacts do not give a transferable global-search heuristic. | `attempted, local optimization only` | Main hostile-landscape evidence for continuation-as-local-search. | Reopen if witness-guided or other committed continuation replacement beats this benchmark. |
| `M008` HKO-local perturbation neighborhood | Is the known HKO-side neighborhood locally stable under perturbation? | HKO-local packet | `experiments/hko-local-maximum/perturbation-neighborhood/` | HKO-local evidence is not generic hostile-landscape evidence. | Ledger marks the packet attempted and supporting-only. | Supports local/HKO-neighborhood interpretation, not the generic-search headline by itself. | `attempted, local optimization only` / `supporting evidence only` | Supporting material only unless the thesis section discusses HKO-local stability. | Reopen if HKO-local rows become part of the main hostile-landscape claim. |
| `M009` omega scalar hypothesis test | Does a cheap scalar geometry hypothesis give a search heuristic? | scalar geometry heuristic on landscape packets | `experiments/combinatorial-cells/omega-hypothesis/`; `tasks/landscape.md` | Correlation-style check only; no causal or exhaustive guard. | Ledger marks the omega hypothesis correlation check as attempted and negative heuristic evidence. | Records one failed scalar heuristic rather than a search theorem. | `attempted, negative vs random` / `supporting evidence only` | Supporting failed-pattern evidence if scalar heuristics are mentioned. | Reopen if the omega feature is redefined or tied to a new committed candidate generator. |
| `M010` visual exploration | Did projection and picture inspection reveal a useful pattern? | exploratory geometry and dynamics views | `research/visualization.md`; `tasks/landscape.md` | Human visual inspection is exploratory and non-exhaustive. | Ledger records visual exploration as a negative exploratory result. | "Looking at pictures" produced no current search rule, but it is still research evidence about failed pattern discovery. | `attempted, negative vs random` / `supporting evidence only` | Supporting or standalone negative-exploration material, pending writing choice. | Reopen if figures become thesis-selected evidence or a visual pattern becomes an implemented heuristic. |
| `M011` feature-block regression | Do cheap feature blocks predict `sys` well enough to transfer from random samples to endpoints? | tabular random and endpoint regimes | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py`; feature JSONL/PNG artifacts in the same directory; `research/sys-landscape-datascience/idea-ledger.md` (`DS-I006`); `research/sys-landscape.md`; `tasks/landscape.md` | Grouped CV by lineage/source; transfer between random and endpoint regimes is the load-bearing guard; the sanity spike adds cheap permuted-label nulls. | DS-I006 found above-null within-random and within-endpoint pockets, but best random-to-endpoint transfer was still `face_symplectic` at `R^2 = -9.1481` despite beating a weak null p95 of `-14.6865`. | Supports the bounded claim that the current feature packet did not produce a reusable global-search heuristic; within-regime signal must not be read as transfer. | `attempted, negative vs random` | Main evidence for the data-science hostile-landscape paragraph. | Reopen if refreshed feature packets show positive random-to-endpoint transfer or add local-maxima-specific transfer evidence. |
| `M012` regime classification | Can standard classifiers separate endpoint from random regimes, and is the separator geometric rather than provenance-heavy? | grouped tabular regime-separation task | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py`; `regime_classification_bars.png`; `research/sys-landscape-datascience/idea-ledger.md` (`DS-I006`) | Grouped split; metadata/provenance fields are an explicit validity caveat; classification alone is not a search heuristic. | DS-I006's low-capacity classification separated endpoint from random rows: metadata balanced accuracy `1.0000`, and `all_non_metadata` also reached `1.0000` under grouped CV. | Shows strong regime separation in the current table, but the separator may reflect producer/regime structure and still does not say where to sample for new `sys > 1` rows. | `attempted, negative vs random` / `validity caveat` | Undecided; likely caveat/supporting-only unless Jörn wants the regime-separation story in the thesis. | Reopen after Jörn decides whether classification is claim-bearing, caveat-only, or omitted. |
| `M013` residualized endpoint regression | Do endpoint feature blocks add signal beyond metadata? | endpoint-only grouped tabular packet | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py`; `feature_pattern_search_residual.png` | Grouped endpoint CV; metadata-first residual design tests incremental feature signal. | Analyzer tests ridge and random forest residual models after subtracting metadata predictions. Ledger marks thesis use undecided. | Potentially refines whether endpoint geometry has feature signal, but current thesis role is not settled. | `attempted, negative vs random` / `validity caveat` | Undecided; needs Jörn decision before thesis-facing use. | Reopen after Jörn decides whether residual signal is claim-bearing, supporting, spike-only, or omitted. |
| PCA / global dimensionality reduction | Would a global low-dimensional embedding become a search heuristic? | bounded feature-table spike | `research/sys-landscape-datascience/idea-ledger.md` (`DS-I004`) | The spike excluded metadata/provenance fields and required a non-post-hoc generator-side rule, not only correlation with `sys`. | PCA found feature structure correlated with `sys`, but the strongest direction also carried regime/dataset structure and did not define a candidate generator. | Records a bounded negative PCA spike, not a theorem that no embedding can help. | `attempted, negative vs random` / `validity caveat` | Supporting failed-pattern evidence only. | Reopen if a generator-side rule for sampling toward a feature-space region is proposed without using `sys`, endpoint labels, dataset identity, or optimizer provenance. |
| Clustering / manifold learning | Would unsupervised grouping of landscape rows reveal a search-relevant regime? | bounded feature-table spike | `research/sys-landscape-datascience/idea-ledger.md` (`DS-I004`) | A valid result needed a cluster-to-search rule rather than rediscovery of producer families. | KMeans clusters were high-sys enriched, but the high-sys clusters were endpoint-heavy and dataset-heavy; anomaly detection was not high-sys enriched. | Current clustering/anomaly spike did not produce a new search-next rule. | `attempted, negative vs random` / `validity caveat` | Supporting failed-pattern evidence only. | Reopen if a non-post-hoc cluster rule can be turned into fresh candidate generation. |
| SVM / boosting / nearest-neighbor family | Would alternative supervised learners change the regression/classification verdicts? | deferred supervised alternatives on feature tables | No committed SVM, boosting, or nearest-neighbor method packet found in the ledger. | Would need the same grouped CV and random-to-endpoint transfer tests as `M011`/`M012`. | Ridge, logistic regression, and random forest are the committed supervised representatives; these alternatives are not attempted. | Their absence limits toolbox coverage but does not overturn the artifact-backed negative transfer observation. | `skipped: expensive to implement` | Explicit omitted-family caveat only; do not cite as failed. | Reopen if a thesis reader expects additional supervised baselines or if current analyzers are extended cheaply. |
| Neural-network methods | Would a flexible learned model extract a transferable high-`sys` signal? | deferred high-capacity supervised model family | No committed neural-network method packet found in the ledger. | Would need grouped splits, transfer tests, capacity/overfit controls, and enough data to make the fit meaningful. | Not attempted as a method program in the current ledger. | This is deferred because the current dataset size and closeout scope make overfit risk high relative to thesis value. | `skipped: expensive to implement` | Explicit omitted-family caveat only. | Reopen only for future work or if a pre-existing low-risk NN packet appears. |
| Bayesian optimization | Would surrogate-guided black-box search find better candidates than random/local methods? | deferred black-box search over candidate-generation space | No committed Bayesian-optimization method packet found in the ledger. | A valid attempt would need a parameterized candidate space, acquisition loop, exact-evaluation budget, and held-out reporting. | Not attempted as a method program in the current ledger. | This remains a plausible future search program, not evidence for or against the current negative result. | `skipped: expensive to run` | Future-work caveat only. | Reopen if a bounded candidate space and compute budget are approved. |

## Taxonomy Anchors

- `research/sys-landscape-datascience/taxonomy-islr.md`
- `research/sys-landscape-datascience/taxonomy-esl.md`
- `research/sys-landscape-datascience/taxonomy-murphy.md`
- `research/sys-landscape-datascience/taxonomy-dfo.md`
- `research/sys-landscape-datascience/taxonomy-numerical-optimization.md`
- `research/sys-landscape-datascience/taxonomy-continuation.md`
- `research/sys-landscape-datascience/taxonomy-bayesian-optimization.md`
- `research/sys-landscape-datascience/taxonomy-eda-visualization.md`
- `research/sys-landscape-datascience/taxonomy-statistical-inference.md`
- `research/sys-landscape-datascience/taxonomy-time-series.md`
- `research/sys-landscape-datascience/method-ledger.md`

Phase 2 should treat the taxonomy files as frozen external method universes, the method ledger as a cached repo-method index, and this audit as the place where repo-facing verdicts are organized.
Methods that lack taxonomy refs in the ledger should be treated as "not yet mapped to an external taxonomy", not as nonexistent.

## Method Buckets To Populate In Phase 2

### Search Families

- random generic sampling
- random Lagrangian-product sampling
- rotated regular-product sweeps
- fixed-`F` gradient ascent from random starts
- variable-`F` continuation
- HKO-local perturbation neighborhood

### Data-Science / Pattern-Search Methods

- ridge regression on feature blocks
- random-forest regression on feature blocks
- logistic regime classification
- random-forest regime classification
- endpoint residual analysis beyond metadata
- scalar correlation / hypothesis tests already used as search heuristics
- visual inspection / "look at pictures"

### Methods To Classify Explicitly As Unused Or Deferred

- PCA as a global-search heuristic
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

- The roadmap bundle now uses the current random-generic max `sys=0.739`
  rather than the stale `0.578`; see `tasks/landscape.md`.
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
