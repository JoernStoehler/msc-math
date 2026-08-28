# Data-Science Appendix Content Notes

Status: writer companion for `thesis/a-datascience-results.tex`. This file is a
source and boundary map, not source truth and not a duplicate result table.

## Current passage purpose

The appendix is the compact support surface for
`thesis/08-black-box-datascience.tex`. It explains enough of the frozen methods,
selections, controls, and caveats for a reader to audit the main bounded
conclusion. It is not an independent search chapter and should not become a
chronicle of method churn.

The active appendix now includes:

- the frozen seven-method optimizer steps and held-out comparison;
- one- and five-second endpoint polls and the oblique continuation counterexample
  to a basis-poll reading;
- the selected HKO perturbation calibration;
- the outcome-selected endpoint failure panel and three-proposal directional
  KKT audit;
- the retained random/product input and ordinary in-table diagnostics;
- the frozen generated-candidate scalar-filter packet; and
- mechanism and reference diagnostics.

Earlier inventory text that classified all optimizer, endpoint, continuation,
or local-behavior material as inactive is superseded for these named packets.
Other legacy `sys-landscape` ascent/local-behavior packets remain historical
context unless the active section explicitly reopens them.

Do not copy generated result tables into this companion. The active appendix
owns selected reader-facing values; generated artifacts and packet READMEs own
full metrics, thresholds, rankings, and per-case rows.

## Exact source map

### Optimizer implementation, held-out selection, and evaluator boundary

- `experiments/dev-gradient-ascent/optimizer-runs/README.md`: implementation
  contracts, historical archive provenance, compute accounting, and the fact
  that the retained trace field is a binary64 evaluator output rather than a
  certified mathematical capacity.
- `experiments/dev-gradient-ascent/optimizer-runs/manifests/heldout-f10-64-finalists.json`:
  frozen `F=10` start population, seven methods, hyperparameters, equal measured
  serial-compute budget, full-call cap, and historical evaluator.
- `experiments/dev-gradient-ascent/optimizer-comparison/README.md`: strict
  analyzer and finite-budget comparison boundary.
- `experiments/dev-gradient-ascent/optimizer-comparison/artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/SUMMARY.md`:
  generated outcome, paired, compute, and stopping summaries used by the TeX.

The held-out population is 64 matched previously unused random ten-facet
starts. The seven implementations and hyperparameters were frozen before those
outcomes. The ranking is only for this population, budget, and evaluator. Do
not describe it as an optimizer-family optimum or transfer it to certified
capacity.

### Endpoint depth and continuation

- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/README.md`:
  signed symmetry-transverse basis poll and finite-test limitation.
- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/heldout-f10-64-history-endpoints-19a8b4dfd-analysis/REPORT.md`:
  16 population-stratified held-out branch-history endpoints at the one-second
  comparison budget.
- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/history-f10-16-compute-depth-endpoints-426ec7a7c-analysis/REPORT.md`:
  the same starts after a five-second ceiling.
- `experiments/dev-gradient-ascent/ascent-continuation/README.md` and
  `experiments/dev-gradient-ascent/ascent-continuation/artifacts/four-state-full-20260729/analysis/REPORT.md`:
  three deliberately selected optimizer endpoints and HKO; one endpoint that
  passed the coordinate poll still has a sustained branch-informed oblique
  path.

The one- versus five-second comparison diagnoses ordinary insufficient depth
on named starts. The four-state continuation shows that a signed coordinate
poll can miss ascent between its axes. Neither packet estimates distance to an
eventual local maximum or supplies a convergence result.

### Selected controls and finite-step failure audit

- `experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/analysis/REPORT.md`:
  16 perturbations along four predeclared HKO development directions, with HKO
  itself as false-positive control.
- `experiments/dev-gradient-ascent/ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/analysis/REPORT.md`:
  the eight highest endpoints from a 128-start tuning population; this is
  outcome-selected discovery evidence, not held-out ranking evidence.
- `experiments/dev-gradient-ascent/endpoint-model-audit/README.md`: named KKT
  branch, action, volume, geometry, and evaluator trust boundaries.
- `experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/analysis/REPORT.md`:
  two failed proposals and one positive control, including finite differences,
  KKT spectral-scale comparison, and f64/exact-geometry checks.

Do not merge these selection roles. The HKO panel is development calibration;
the top eight are outcome-selected tuning endpoints; the KKT decomposition is
a three-proposal audit. They support selected mechanisms and observed finite
improvements, not population failure rates or a universal trust cutoff. Small
radius agreement diagnoses the implemented derivative's local limit; it does
not make that radius a useful optimizer step.

### Retained-table and generated-candidate evidence

- `experiments/polytope-datasets/README.md`: random and random-product producer
  contracts.
- `experiments/polytope-invariant-table/README.md`: active invariant and
  provenance table contract.
- `experiments/sys-datascience/methods/trusted-random-dataset/README.md`:
  trusted retained-row filter.
- `experiments/sys-datascience/methods/trusted-random-product-method-dispositions.md`:
  named ordinary retained-table methods and their dispositions.
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md`:
  frozen pre-target candidate selection and matched baseline evaluation.
- `experiments/sys-datascience/methods/ridge-mechanism-discriminator/README.md`:
  retained and generated ridge-area interpretation, including which
  concentration splits are post-target only.
- `experiments/sys-datascience/methods/hko-reference-coverage/README.md` and
  `experiments/sys-datascience/methods/hko-ridge-source-smoke/README.md`:
  bounded reference diagnostics; neither owns HKO local maximality.

Retained-table scan, association, projection, clustering, anomaly,
prediction, and rule packets operate after every row's target is known. They
may explain the table but do not validate a generated proposer. The generated
scalar-filter packet is stronger because selection precedes target evaluation,
yet it remains scoped to its generator, frozen rule set, and budget and did not
validate a threshold-crossing route.

## Claim and selection boundaries

- Use `\widehat{\operatorname{sys}}` for the historical optimizer trace field
  in prose that could otherwise be read as certified mathematical `sys`.
- “Fully reevaluated” in an optimizer packet means reevaluated by that packet's
  configured historical evaluator, not certified by a theorem-grade capacity
  computation.
- Across-start ranges are population variation, not uncertainty intervals for
  a median.
- The optimizer comparison shares a `1000 ms` start-new-work cutoff, not
  identical realized compute. Terminal values may lie to its right because
  atomic work begun below the cutoff is allowed to finish.
- The held-out optimizer ranking and outcome-selected diagnostic panels answer
  different questions; never use the latter to strengthen held-out frequency
  claims.
- Endpoint poll survival does not imply stationarity. A stopped continuation
  does not imply convergence. A finite KKT audit does not calibrate a general
  conditioning rule.
- HKO reference/control packets do not establish HKO local maximality; use
  `experiments/hko-local-maximum/` for that theorem.
- Retained-table diagnostics are not generated-candidate validation, and
  generated sub-threshold enrichment is not evidence for a general
  threshold-directed proposer.
- Keep detailed metrics and method-specific rows in generated artifacts unless
  a reader-facing claim in the active TeX actually needs them.

## Review state

`docs/project-status.md` records this data-science surface as an internally
reviewed integration candidate, while ordinary whole-PDF, Jörn, and Kai review
remains. Packet validation and internal review support the named evidence
transitions; they do not amount to stakeholder acceptance or a rendered-page
review. This companion repair updates routing and selection boundaries only.

## Reopen triggers

Reopen the appendix inventory when:

- the main section changes which packets it summarizes or changes the balance
  between main-text result and appendix audit detail;
- a frozen optimizer manifest, historical evaluator, held-out population, or
  generated analysis artifact is replaced;
- a new independent population changes the optimizer ranking or supports a
  population failure-rate claim;
- a certified evaluator, convergence/stationarity result, or calibrated KKT
  trust policy changes the present claim boundary;
- the retained random/product table, invariant schema, generator, or frozen
  candidate-selection contract changes; or
- reader or rendered review shows that the appendix duplicates the main text,
  obscures selection timing, or invites a certified-capacity/local-maximality
  reading.

A routine rerun of the same finite polls or another unselected method packet is
not by itself a reason to expand the appendix.
