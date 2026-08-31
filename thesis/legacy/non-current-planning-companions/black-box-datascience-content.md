# Black-Box Data-Science Content Notes

Status: writer companion for `thesis/08-black-box-datascience.tex` and its two
included subsection files. This file routes current source truth; it is not an
experiment result or mathematical authority.

## Current passage purpose and selected shape

The section gives a controlled negative benchmark, not a nonexistence result.
It asks whether several bounded search interfaces found either a new body above
the threshold or a reproducible route worth using to seek one. Its current
shape has three distinct evidence strata:

1. the retained random/product table, its in-table diagnostics, and the frozen
   generated-candidate scalar-filter follow-up;
2. the held-out seven-optimizer comparison on random ten-facet starts, followed
   by endpoint, continuation, HKO-calibration, and finite-step failure
   diagnostics; and
3. the separate five-case, theory-selected fixed-facet local screen.

The optimizer and selected-body material is already active. Earlier planning
that treated ascent, continuation, endpoint behavior, or the local-maxima
screen as inactive pending another thesis-integration decision is superseded.
The old `experiments/sys-landscape/` ascent/continuation packets remain
historical context; they are not substitutes for the frozen optimizer packets
selected here.

Active prose routes:

- `thesis/08-black-box-datascience.tex` owns the section-level question,
  retained-table result types, and bounded synthesis;
- `thesis/08-black-box-datascience-finite-budget-optimization.tex` owns the
  held-out optimizer comparison and its concise diagnostics;
- `thesis/08-black-box-datascience-local-maxima-check.tex` owns the selected
  five-case local screen; and
- `thesis/a-datascience-results.tex` owns method detail and the optimizer
  steps, controls, and audits that would overload the main section.

Do not turn this companion into a second results appendix. Selected numbers
belong in the active TeX; full metric tables remain with the generated packet
artifacts.

## Source routes and relevance

### Retained and generated random/product search

- `experiments/polytope-datasets/README.md`: producer populations, seeds,
  bucket plans, and retained-row provenance.
- `experiments/polytope-invariant-table/README.md`: derived invariant-feature
  and provenance tables consumed by the ordinary data-science methods.
- `experiments/sys-datascience/README.md`: active random/product question and
  the boundary from historical `sys-landscape` work.
- `experiments/sys-datascience/coordination/current-question-map.md`: current
  routing and reopen state; it is a navigation view, so linked packet sources
  overrule it.
- `experiments/sys-datascience/methods/trusted-random-dataset/README.md`: exact
  trusted row filter and finite-table negative result.
- `experiments/sys-datascience/methods/trusted-random-product-method-dispositions.md`:
  named ordinary method surface and dispositions.
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md`:
  selection-before-target contract and bounded generated-candidate evidence.
- `experiments/sys-datascience/methods/ridge-mechanism-discriminator/README.md`:
  ridge-area interpretation and the boundary between pre-target rules and
  post-target diagnostics.

Use method-local READMEs and their cited generated artifacts for any method
claim. Do not infer a packet result from the coordination map alone.

### Held-out optimizer comparison and historical evaluator

- `experiments/dev-gradient-ascent/optimizer-runs/README.md`: runner, frozen
  dataset provenance, compute accounting, and the historical-evaluator trust
  boundary. The retained comparison is historical schema-1 evidence; running
  the current clean evaluator would be a new experiment.
- `experiments/dev-gradient-ascent/optimizer-runs/manifests/heldout-f10-64-finalists.json`:
  frozen population, methods, hyperparameters, budget, and evaluator contract.
- `experiments/dev-gradient-ascent/optimizer-comparison/README.md`: strict
  analysis contract and the exact population to which the ranking applies.
- `experiments/dev-gradient-ascent/optimizer-comparison/artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/SUMMARY.md`:
  generated held-out outcomes, paired comparisons, compute profiles, and
  threshold counts used by the TeX.

All optimizer values concern the JSON field produced by the common historical
heuristic binary64 orbit-search evaluator, denoted
`\widehat{\operatorname{sys}}` in the TeX. Candidate-family completeness is
untested. The comparison supports a ranking of the seven frozen
implementations on the matched random `F=10` starts and stated one-second
measured-compute budget. It does not certify mathematical capacity, transfer
the ranking to another population, facet count, budget, or evaluator, or show
convergence or local maximality.

### Endpoint and continuation diagnostics

- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/README.md`:
  finite signed-basis poll and its symmetry-transverse selection contract.
- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/heldout-f10-64-history-endpoints-19a8b4dfd-analysis/REPORT.md`:
  population-stratified one-second endpoint panel selected before its poll
  outcomes were computed.
- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/history-f10-16-compute-depth-endpoints-426ec7a7c-analysis/REPORT.md`:
  same-start larger-budget comparison.
- `experiments/dev-gradient-ascent/ascent-continuation/README.md`: repeated
  branch-informed continuation contract and its finite-direction limitations.
- `experiments/dev-gradient-ascent/ascent-continuation/artifacts/four-state-full-20260729/analysis/REPORT.md`:
  three deliberately selected optimizer endpoints plus HKO, including the
  oblique path missed by a signed-basis poll.
- `experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/analysis/REPORT.md`:
  four predeclared directions at four distances from HKO, with HKO as the
  false-positive control.
- `experiments/dev-gradient-ascent/ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/analysis/REPORT.md`:
  outcome-selected top-eight tuning endpoints and the systematic finite-step
  model failures that motivated the audit.
- `experiments/dev-gradient-ascent/endpoint-model-audit/README.md` and
  `experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/analysis/REPORT.md`:
  two selected failures and one positive control, localizing those failures to
  finite-distance nonlinearity of ill-conditioned named KKT branches.

Keep the selections distinct. The held-out ranking, population-stratified
endpoint poll, deliberately selected continuation states, development HKO
directions, outcome-selected tuning endpoints, and three-proposal KKT audit
are not one population. The diagnostics establish observed improvements and
failure mechanisms only on their named cases. They estimate neither a
population miss rate nor a universal KKT trust cutoff.

### Theory-selected local screen

- `experiments/local-maxima-check/README.md`: five evidence statuses, frozen
  probes, claim boundary, decision `LMC-D1`, and concrete reopen conditions.
- `experiments/local-maxima-check/artifacts/REPORT.md`: generated control and
  target outcomes. The rotated-pentagon structured probes succeed while the
  generic quotient/random probes miss that known thin improving family.
- `experiments/verification/ch2021-six-vertex/README.md`: exact value of the
  displayed CH2021 body, not a neighborhood theorem.
- `experiments/hko-local-maximum/README.md`: theorem-strength HKO local
  maximality; the finite screen is not its authority.
- `experiments/regular-products/pentagon-rotation-formula-proof/README.md`:
  exact rotated-pentagon profile and the known non-maximum control.

The target misses are fixed-facet, basis-, sample-, radius-, and
evaluator-dependent nominal-scalar diagnostics with broad capacity intervals.
They do not certify local maximality, exclude thin improving germs, or cover
facet additions. The conjectural and proved statuses in the five-case table
must remain visibly different.

## Claim and wording boundaries

- Preserve finite-table, generator, population, facet-count, budget,
  selection-timing, and evaluator scope wherever the prose summarizes a
  negative result.
- Retained-table association, prediction, and post-target splits explain
  already evaluated rows; they are not generated-candidate validation.
- A rule counts as a generated proposer only when selection is frozen before
  target evaluation. Sub-threshold enrichment is not validation of a route to
  `sys > 1`.
- Do not identify historical optimizer `sys` fields with certified
  mathematical systolic ratio or capacity.
- A fully reevaluated finite improvement is evidence of ascent for that state
  under that evaluator. A failed poll or stopped continuation is not
  stationarity, convergence, or local maximality evidence.
- The HKO calibration is a selected development panel, not an estimated
  false-negative rate. The KKT audit diagnoses three proposals, not a
  population conditioning law.
- Do not claim exhaustive search, random-model impossibility, absence of
  `sys > 1` bodies, or exhaustion of useful invariant features or optimizer
  families.

## Review state

`docs/project-status.md` classifies the data-science surface as an internally
reviewed integration candidate. The bounded random/product table, held-out
optimizer comparison, continuation diagnostics, and failure audits are
integrated; ordinary whole-PDF, Jörn, and Kai review remains. This companion
repair checks current source routing and stale planning only. It is not a new
scientific, mathematical, reader, or rendered-page review.

## Reopen triggers

Reopen this companion when:

- any of the four active TeX routes above changes its selected purpose or
  result surface;
- a retained producer/table is rebuilt, a frozen packet is replaced, or an
  evaluator contract changes in a way that affects a displayed claim;
- a new independently evaluated population changes the optimizer ranking or
  supports transfer beyond the current `F=10`/budget/evaluator contract;
- a stationarity, convergence, certified-capacity, or local-maximality result
  replaces one of the present empirical boundaries; or
- `LMC-D1` is reopened for one of its named reasons: a new independently
  motivated candidate, branch-complete or exact evidence, a stronger
  informative negative method, or a concrete thesis need that changes the
  value comparison.

More rows, another ordinary retained-table model, another routine basis poll,
or another seed alone is not a reopen reason under the current packet
decisions.
