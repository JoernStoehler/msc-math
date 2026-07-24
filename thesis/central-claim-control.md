# Central thesis claim propagation

Status: current thesis-wide navigation companion, reconciled 2026-07-23. This
file is not mathematical or empirical source truth.

Purpose: keep the central HKO, data-science, rotated-pentagon, finite-method,
and AI-use claims consistent where they propagate into the abstract,
introduction, result chapters, and conclusion.

Active thesis text, chapter companions, formal notes, experiment artifacts,
source papers, and accepted Jörn/Kai decisions overrule this view. Follow the
named source before changing a propagated claim.

Current writing state:

- `00-abstract.tex` is empty and `14-conclusion.tex` contains only its heading.
  Write them last from the settled result surfaces.
- `01-introduction.tex`, `07-hko-local-maximum*.tex`,
  `08-black-box-datascience*.tex`, and `09-rotated-regular-polygons*.tex`
  contain active drafts.
- `ai-use-disclosure.tex` is the separate Jörn-accepted factual disclosure.
  `13-use-of-ai.tex` is the numbered research-process discussion and remains
  provisional. Do not merge their support or status.

This is a claim-propagation view, not a task queue or whole-thesis inventory.

## Abstract and introduction

| Surface | Propagated claim | Source and support | Boundary and current action |
| --- | --- | --- | --- |
| `00-abstract.tex` | HKO2024 is locally maximal among nearby ten-facet polytopes modulo translations, positive scaling, and linear symplectic maps. | `07-hko-local-maximum*.tex`; `experiments/hko-local-maximum/README.md`; `experiments/hko-local-maximum/theorem/README.md`; `formal/hko-feasible-section-upper-branches.tex`. Exact feasible-section certificate plus the mathematical implication. | Do not state raw strict local maximality in ambient coordinates or changing-facet-count maximality. Draft from the active theorem wording; Jörn/Kai review still applies. |
| `00-abstract.tex` | The rotated regular-pentagon product has the exact named `sys > 1` profile. | `09-rotated-regular-polygons*.tex`; `thesis/rotated-regular-polygons-content.md`; `experiments/regular-products/README.md`; `experiments/regular-products/pentagon-rotation-formula-proof/README.md`; endpoint/symmetry sources. | Do not generalize beyond the checked pentagon-product family or classify all minimizers. |
| `00-abstract.tex` | The closed named data-science method table found no new source of `sys > 1` examples and no candidate-proposer for finding one beyond the already explained positive family and controlled relatives. | `docs/project-facts.md`, item 34.2; `08-black-box-datascience*.tex`; `experiments/sys-datascience/README.md`; `experiments/sys-datascience/coordination/current-question-map.md`; method packets. Bounded empirical support. | Do not say there are no positive examples, no proposer of any sub-threshold kind, exhaustive search, impossibility, or a population theorem. |
| `00-abstract.tex` | The thesis uses finite polytope computations and separates f64 search from exact/Sage verification where theorem strength requires it. | `03-generalized-reeb-orbits-polytopes.tex`; `04-haim-kislev-quadratic-program.tex`; `11-numerics.tex`; theorem packets and verification experiments. | Keep abstract-level. Do not promise a universally certified public solver or a finite reduction for every capacity problem. |
| `01-introduction.tex` | HKO2024 motivates the local ten-facet problem; the thesis probes it through local mathematics, finite computation, bounded search, and one exact structured family. | Active introduction; HKO, data-science, QP/flow-graph, and rotated-pentagon sources below. | The current introduction already states this architecture. Reassess it when a result's support or final thesis emphasis changes. |
| `01-introduction.tex` | The data-science result is a bounded negative benchmark, contrasted with known HKO/rotated-pentagon positives. | Active data-science chapter and packet sources. | Preserve finite populations, frozen-rule budgets, target-before/after distinctions, and the absence of global rarity or impossibility support. |
| `01-introduction.tex` | The method chain is generalized Reeb orbits, the Haim--Kislev QP, the project flow-graph route, first-order upper functions, numerics/exactness, and checked code/data. | Active method chapters; `formal/`; relevant crate and experiment entry points. | Keep the arbitrary-polytope first-order caveat and the exact hypotheses of the flow-graph theorem visible. |

## HKO local result

| Surface | Claim | Source and support | Boundary |
| --- | --- | --- | --- |
| `07-hko-local-maximum.tex` | The local question is ten-facet maximality modulo translations, positive scaling, and linear symplectic maps. | Active theorem statement; HKO experiment entry point and theorem packet. | Not strict local maximality in raw `R^40`, not changing-facet-count maximality. |
| `07-hko-local-maximum-sage-verifier.tex` | Verification uses exact dual coordinates over the ordered quartic field `Q(tan(pi/5))` and checks the theorem-facing predicates. | `experiments/hko-local-maximum/theorem/README.md` and `experiments/hko-local-maximum/theorem/verification-summary.json`. | Do not replace the field by `Q(sqrt(5))`; distinguish witness generation from verification. |
| `07-hko-local-maximum-exact-certificate.tex` | The certificate has 26 feasible-section derivative rows, row rank 25, symmetry tangent rank 15, and a strictly positive exact convex relation summing the rows to zero. | `experiments/hko-local-maximum/theorem/README.md` and `formal/hko-feasible-section-upper-branches.tex`. | Exact verification plus the mathematical upper-function implication supports the theorem; empirical checks are not proof inputs. |
| `07-hko-local-maximum-empirical-tests.tex` | First-/second-order numerics, neighborhood sampling, and eleven-facet ascent are supporting checks. | `experiments/hko-local-maximum/README.md` and empirical child packets. | State their finite scopes; they neither replace the exact certificate nor prove broader local maximality. |

## Bounded data-science search

| Surface | Claim | Source and support | Boundary |
| --- | --- | --- | --- |
| `08-black-box-datascience.tex` | The active method-table population is the retained random-polytopes/random-products table, followed by named frozen generated-candidate selector tests. | `experiments/polytope-datasets/README.md`; `experiments/polytope-invariant-table/README.md`; `experiments/sys-datascience/methods/README.md`; active chapter and appendix. | Old fixed-facet ascent, continuation, endpoint, and local-behavior work is separate legacy context routed through `experiments/sys-landscape/README.md`, not an active method-table population. |
| `08-black-box-datascience.tex` | The 14,336-row retained table contains no `sys > 1` row. In the 100,000-candidate scalar-filter packet, the frozen selected/baseline union of 1,675 evaluated rows also contains no `sys > 1` row. | Active chapter; trusted-table, scan, and `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md`. | Finite-table and configured-generator evidence only; no exhaustive-search, density, or nonexistence claim. |
| `08-black-box-datascience.tex` | In-table prediction/association/projection/rule packets recover structure; frozen generated-candidate rules provide bounded sub-threshold enrichment but no threshold-directed proposer. | `experiments/sys-datascience/methods/README.md`, current question map, and packet-local evidence. | Do not turn post-target diagnostics into prospective validation or describe sub-threshold enrichment as a proposer for finding `sys > 1`. |
| `08-black-box-datascience-local-maxima-check.tex` | The selected-body local screen is a separate theory-selected finite panel with its own controls. | `experiments/local-maxima-check/README.md`; active subsection. | The positive control demonstrates a false-negative mode; misses do not establish local maximality. |

## Conclusion

| Surface | Propagated content | Sources and boundary |
| --- | --- | --- |
| `14-conclusion.tex` | State the HKO local theorem at its exact ten-facet/symmetry scope. | Active HKO chapter and theorem packet; preserve the distinction between exact proof support and broader empirical checks. |
| `14-conclusion.tex` | Contrast the exact rotated-pentagon positive family with the bounded negative data-science search. | Active rotated-pentagon and data-science chapters; do not collapse “known positive family” into “new source found by the search.” |
| `14-conclusion.tex` | Summarize the generalized-orbit/QP/flow-graph/first-order/numerics/code-data contributions at their actual support strengths. | `03-*` through `06-*`, `10-*` through `13-*`, their companions, `formal/`, crates, experiments, and `docs/reproducibility.md`. Flow-graph and arbitrary-polytope first-order caveats remain material. |
| `14-conclusion.tex` | Separate established results, bounded empirical findings, limitations, and future work. | Do not move mandatory unfinished support into future work merely to make the current thesis look complete. Final emphasis and future-work selection are Jörn thesis-design decisions. |

## Update rule

When a source result or support boundary changes, search the active thesis for
every propagated occurrence and update this view only if it still prevents a
real cross-surface mismatch. Delete resolved task history rather than turning
this file back into a queue.
