# Sys-Landscape Toolbox Audit

## Purpose

- Make the hostile-landscape result legible below the headline level.
- Record, in one place, which "standard toolbox" methods were tried, which were not, and what each method actually licenses.
- Keep this file as the canonical method ledger for the empirical hostile-landscape claim; use `research/sys-landscape.md` for topic narrative and `RESULTS.md` for compressed thesis claim surface.

## Current Role

- This is a phase-1 scaffold. It defines the structure, vocabulary, and open slots for the detailed audit.
- Phase 2 should populate the ledger row by row from committed experiment packets and current notes.
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

- Which methods count as part of the thesis-relevant "standard toolbox" and must therefore appear in the ledger even if unused?
- Should phase 2 populate only methods already backed by committed artifacts, or also include clearly marked skipped rows for methods never implemented?
- Should phase 3 compress `RESULTS.md` further once this ledger exists, or keep one medium-detail hostile-landscape paragraph there?
